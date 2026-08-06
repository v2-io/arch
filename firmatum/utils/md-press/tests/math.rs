//! Math-pass tests. Everything here is offline except the last test, which
//! talks to a local ollama model and runs only with MD_PRESS_OLLAMA=1 set
//! (model output is nondeterministic-ish and slow; the verification gates,
//! not the model, are what the offline tests pin down).

use md_press::math;

#[test]
fn detector_fires_on_unicode_math_only() {
    assert!(math::needs_math_pass("The gain η exceeds ρ under load."));
    assert!(math::needs_math_pass("tempo 𝒯 = ν · K"));
    assert!(math::needs_math_pass("mismatch ‖δ‖ ≤ R"));
    // prose word 'eta', no glyphs
    assert!(!math::needs_math_pass("The eta parameter controls decay."));
    // already-promoted math is invisible to the detector
    assert!(!math::needs_math_pass("The gain $\\eta$ exceeds $\\rho$."));
    // Greek inside a Greek word (etymology) is prose
    assert!(!math::needs_math_pass("from the Greek πρᾶξις (praxis)"));
    // code spans are masked
    assert!(!math::needs_math_pass("run `η → ϱ` in the REPL"));
}

#[test]
fn postprocess_applies_house_rules() {
    // raw angles, bare asterisk, spaces inside $
    assert_eq!(
        math::postprocess("bound $ x < y $ and $\\eta^*$"),
        "bound $x \\lt y$ and $\\eta^\\ast$"
    );
    // glued command from a model proposal
    assert_eq!(math::postprocess("$\\Vertw$"), "$\\Vert w$");
    // prose and code untouched
    assert_eq!(math::postprocess("keep `a < b` as code"), "keep `a < b` as code");
}

#[test]
fn prose_preservation_gate() {
    let orig = "The gain η exceeds ρ whenever loaded.";
    assert!(math::preserves_prose(
        orig,
        "The gain $\\eta$ exceeds $\\rho$ whenever loaded."
    ));
    // dropped word
    assert!(!math::preserves_prose(
        orig,
        "The gain $\\eta$ exceeds $\\rho$ loaded."
    ));
    // rephrased prose
    assert!(!math::preserves_prose(
        orig,
        "The gain $\\eta$ is bigger than $\\rho$ whenever loaded."
    ));
    // unbalanced dollars
    assert!(!math::preserves_prose(orig, "The gain $\\eta exceeds ρ whenever loaded."));
}

#[test]
fn math_content_consistency_gate() {
    let orig = "Under mismatch ‖δ‖ ≤ R the update M_t stays bounded, and α > 0 holds.";
    // faithful conversion passes
    assert!(math::math_content_consistent(
        orig,
        "Under mismatch $\\lVert\\delta\\rVert \\leq R$ the update $M_t$ stays bounded, and $\\alpha \\gt 0$ holds."
    ));
    // the observed hallucination: \to M_{t+1} imported from a few-shot example
    assert!(!math::math_content_consistent(
        orig,
        "Under mismatch $\\lVert\\delta\\rVert \\leq R$ the update $M_t \\to M_{t+1}$ stays bounded, and $\\alpha \\gt 0$ holds."
    ));
    // invented Greek
    assert!(!math::math_content_consistent(orig, "gain $\\eta$ bounded"));
}

#[test]
fn display_math_blank_lines() {
    let input = "intro text:\n$$\nx = y\n$$\nnext paragraph\n";
    let want = "intro text:\n\n$$\nx = y\n$$\n\nnext paragraph\n";
    assert_eq!(math::fix_display_math_blanks(input), want);
    // idempotent
    assert_eq!(math::fix_display_math_blanks(want), want);
    // untouched inside code fences
    let fenced = "```\n$$\nx\n$$\n```\n";
    assert_eq!(math::fix_display_math_blanks(fenced), fenced);
}

#[test]
fn live_model_end_to_end() {
    if std::env::var("MD_PRESS_OLLAMA").as_deref() != Ok("1") {
        eprintln!("skipped (set MD_PRESS_OLLAMA=1 to run against local ollama)");
        return;
    }
    let model = std::env::var("MD_PRESS_MODEL").unwrap_or_else(|_| "llama3.2:3b".into());
    let line = "Under mismatch ‖δ‖ ≤ R the update M_t stays bounded, and α > 0 holds.";
    match math::promote_line(&model, line) {
        math::MathOutcome::Converted(l) => {
            assert!(!math::needs_math_pass(&l), "residual math in: {l}");
            assert!(math::preserves_prose(line, &l));
            eprintln!("model produced: {l}");
        }
        math::MathOutcome::Flagged(why, _det) => {
            // an honest flag is an acceptable outcome; a panic is not
            eprintln!("model flagged: {why}");
        }
        math::MathOutcome::Unchanged => panic!("detector should have fired"),
    }
}

#[test]
fn consistency_map_covers_the_detector_set() {
    // Regression: ⊃/∩/etc. fired the detector but their LaTeX commands were
    // missing from the consistency map, so a *correct* proposal could never
    // pass. Every operator conversion below must be accepted.
    let cases = [
        ("adaptive ⊃ agency holds", "adaptive $\\supset$ agency holds"),
        ("closed under ∪ and ∩ always", "closed under $\\cup$ and $\\cap$ always"),
        ("where ¬agency means residual", "where $\\neg$agency means residual"),
        ("with κ near one", "with $\\kappa$ near one"),
        ("the set 𝒪 is nonempty", "the set $\\mathcal{O}$ is nonempty"),
    ];
    for (orig, prop) in cases {
        assert!(math::needs_math_pass(orig), "detector must fire: {orig}");
        assert!(
            math::math_content_consistent(orig, prop),
            "correct proposal rejected: {prop}"
        );
        assert!(math::preserves_prose(orig, prop), "prose gate rejected: {prop}");
    }
}

#[test]
fn paren_math_normalizes_only_real_math() {
    // LaTeX \(...\) with math content becomes house $...$
    assert_eq!(
        math::normalize_paren_math("holds \\(\\mathcal{O}\\neq\\emptyset\\) here"),
        "holds $\\mathcal{O}\\neq\\emptyset$ here"
    );
    assert_eq!(
        math::normalize_paren_math("bound \\(H(\\Omega_t\\mid\\mathcal{C}_t)>0\\)"),
        "bound $H(\\Omega_t\\mid\\mathcal{C}_t)>0$"
    );
    // unicode math inside counts too
    assert_eq!(math::normalize_paren_math("gap \\(κ → 1\\) closes"), "gap $κ → 1$ closes");
    // a plain parenthetical written \(...\) is not math
    let prose = "we saw \\(see above\\) that it holds";
    assert_eq!(math::normalize_paren_math(prose), prose);
    // untouched inside code spans
    let code = "run `\\(\\alpha\\)` verbatim";
    assert_eq!(math::normalize_paren_math(code), code);
    // idempotent on the result
    let once = math::normalize_paren_math("holds \\(\\alpha\\) here");
    assert_eq!(math::normalize_paren_math(&once), once);
}

#[test]
fn math_sites_come_from_the_parse() {
    use md_press::MathSite;
    let input = "# Head κ\n\nprose ⊃ line\n\n```\ncode ⊃ not math\n```\n\n| a⊃b | plain |\n|---|---|\n| adaptive ⊃ agency | text |\n";
    let r = md_press::format_plain(input);
    let lines: Vec<&str> = r.output.lines().collect();
    assert_eq!(lines.len(), r.math_sites.len());
    for (line, site) in lines.iter().zip(&r.math_sites) {
        match *line {
            "# Head κ" | "prose ⊃ line" => assert_eq!(*site, MathSite::Whole, "{line}"),
            "code ⊃ not math" | "```" => assert_eq!(*site, MathSite::None, "{line}"),
            "|---|---|" => assert_eq!(*site, MathSite::None, "{line}"),
            l if l.starts_with("| a⊃b") || l.starts_with("| adaptive") => {
                let MathSite::Cells(ranges) = site else {
                    panic!("expected cells for {line}");
                };
                assert_eq!(ranges.len(), 2, "{line}");
                // ranges are byte offsets into the line and must slice cleanly
                for &(a, b) in ranges {
                    assert!(l.get(a..b).is_some(), "range not on char boundary: {line}");
                }
            }
            _ => assert_eq!(*site, MathSite::None, "{line}"),
        }
    }
    // the second row's first cell content is exactly the prose between pipes
    let row = lines.iter().position(|l| l.starts_with("| adaptive")).unwrap();
    let MathSite::Cells(ranges) = &r.math_sites[row] else { unreachable!() };
    assert_eq!(&lines[row][ranges[0].0..ranges[0].1], " adaptive ⊃ agency ");
}

#[test]
fn proposal_may_not_mint_punctuation() {
    // observed live: "¬agency" → "$\lnot$-agency" invented a hyphen that the
    // word-token comparison could not see
    let orig = "Adaptive ∩ ¬agency: can model, cannot intervene";
    assert!(!math::preserves_prose(
        orig,
        "Adaptive $\\cap$ $\\lnot$-agency: can model, cannot intervene"
    ));
    assert!(math::preserves_prose(
        orig,
        "Adaptive $\\cap$ $\\lnot$agency: can model, cannot intervene"
    ));
    // punctuation absorbed INTO a span stays legal
    assert!(math::preserves_prose("bound (x < y) holds", "bound $(x \\lt y)$ holds"));
}

#[test]
fn paren_math_catches_bare_variables_and_subscripts() {
    assert_eq!(math::normalize_paren_math("through \\(h\\) only"), "through $h$ only");
    assert_eq!(math::normalize_paren_math("update \\(M_t\\) stays"), "update $M_t$ stays");
    // prose parenthetical still untouched
    let prose = "we saw \\(see above\\) that it holds";
    assert_eq!(math::normalize_paren_math(prose), prose);
}
