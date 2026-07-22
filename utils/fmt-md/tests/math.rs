//! Math-pass tests. Everything here is offline except the last test, which
//! talks to a local ollama model and runs only with FMT_MD_OLLAMA=1 set
//! (model output is nondeterministic-ish and slow; the verification gates,
//! not the model, are what the offline tests pin down).

use fmt_md::math;

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
    if std::env::var("FMT_MD_OLLAMA").as_deref() != Ok("1") {
        eprintln!("skipped (set FMT_MD_OLLAMA=1 to run against local ollama)");
        return;
    }
    let model = std::env::var("FMT_MD_MODEL").unwrap_or_else(|_| "llama3.2:3b".into());
    let line = "Under mismatch ‖δ‖ ≤ R the update M_t stays bounded, and α > 0 holds.";
    match math::promote_line(&model, line) {
        math::MathOutcome::Converted(l) => {
            assert!(!math::needs_math_pass(&l), "residual math in: {l}");
            assert!(math::preserves_prose(line, &l));
            eprintln!("model produced: {l}");
        }
        math::MathOutcome::Flagged(why) => {
            // an honest flag is an acceptable outcome; a panic is not
            eprintln!("model flagged: {why}");
        }
        math::MathOutcome::Unchanged => panic!("detector should have fired"),
    }
}
