//! Regressions from the 2026-10-02 pass: the filed reports in
//! FEEDBACK-2026-08-12.md / FEEDBACK-2026-08-22.md, plus the structural
//! corruptions observed when the math pass was run live that day. Each test
//! names its incident; all are offline (the math cases use a fake model that
//! returns the exact bad proposal the real one produced).

use md_press::math::{self, Model, promote_site};

fn press(input: &str) -> String {
    md_press::format(input)
}

fn assert_render_equal(a: &str, b: &str) {
    assert_eq!(md_press::render_fingerprint(a), md_press::render_fingerprint(b));
}

// ---- unwrap stage -------------------------------------------------------

#[test]
fn nbsp_line_after_hard_break_survives() {
    // FEEDBACK-08-12 §3: the "whitespace-only" line was four spaces and a
    // U+00A0, which is paragraph content; a Unicode trim deleted it and the
    // render gate refused the file.
    let input = "- Discounts for cycling.  \n    \u{a0}\n";
    let out = press(input);
    assert!(out.contains('\u{a0}'), "NBSP deleted: {out:?}");
    assert_render_equal(input, &out);
}

#[test]
fn display_math_keeps_its_own_lines() {
    // FEEDBACK-08-22 §1: a single-line $$…$$ between prose lines was joined
    // into the paragraph at p≈0.47.
    let single = "Two sub-agents observing the same plant give\n$$ \\frac{T_c}{\\sum_i T_i} \\gt 1 $$\nwhich is super-additive.\n";
    assert_eq!(press(single), single);
    // a multi-line block's delimiters and interior keep their lines too
    let multi = "so that\n$$\nx = y\n+ z\n$$\nholds.\n";
    assert_eq!(press(multi), multi);
    // prose around it still unwraps
    let wrapped = "a wrapped\nline\n$$x$$\nafter\nwrap\n";
    assert_eq!(press(wrapped), "a wrapped line\n$$x$$\nafter wrap\n");
}

#[test]
fn table_looking_rows_are_not_joined() {
    // asf FINDINGS.md: a table whose header is glued after a bold label
    // parses as a paragraph; joining its rows buries the evident table.
    let input = "**Related Work:** | concern | prior art |\n| --- | --- |\n| a | b |\n| c | d |\n";
    assert_eq!(press(input), input);
}

// ---- math: detector ------------------------------------------------------

#[test]
fn prose_punctuation_glyphs_do_not_trigger() {
    // FEEDBACK-08-12 §2 and the 2026-10-02 estate census: → and · as prose
    // were two thirds of all would-be model calls
    for line in [
        "**Control cluster:** AI control · control measures · control protocols",
        "- `dialogue-curator.md` → symlink to our new agent",
        "Identity probe → State check → Practice",
        "*Workshop on Affective Interactions* · Jun 23, 2020 · 73 citations",
        "- 35× ROI on refactoring time",
        "**CLadder** (Jin 2023): ≈70% on in-distribution queries",
        "- Depends chain (depth ≤ 3):",
        "ratio (5.7% → 1.9%)",
        "Coordination cost (higher $\\gamma$ → fewer partitions)",
        "Grep(fn if_else_directive) → 61 matches",
        "The latency was 31μs on average",
        "`1.0`→float, `01234`→octal",
    ] {
        assert!(!math::needs_math_pass(line), "should be prose: {line}");
    }
}

#[test]
fn operator_glyphs_still_trigger() {
    for line in [
        "as t → ∞ the bound holds",
        "the update η → 1 under load",
        "re-homes to XPos (u→+1)",
        "when n × k > 6",
        "total emergent H ≤ 5 km",
        "with $\\beta$ ≈ 0.3 the load varies",
        "where n_future ≈ 0",
        "the gain η exceeds ρ",
    ] {
        assert!(math::needs_math_pass(line), "should fire: {line}");
    }
}

#[test]
fn existing_display_math_is_masked() {
    // `$$…$$` toggled the single-$ mask twice and exposed its interior
    assert!(!math::needs_math_pass("$$ drift(t) = \\int_0^t |C(τ)| dτ $$"));
}

// ---- math: deterministic \(…\) -------------------------------------------

#[test]
fn paren_math_leaves_link_destinations_and_numbers_alone() {
    // FEEDBACK-08-12 §1: `\(1\)` inside a scraped link destination became
    // `$1$` and silently broke the URL
    let url = "see [the PDF](https://x.gov/a%20Publication%20\\(1\\).pdf) now";
    assert_eq!(math::normalize_paren_math(url), url);
    let num = "Publication \\(1\\) in the list";
    assert_eq!(math::normalize_paren_math(num), num);
    assert_eq!(math::normalize_paren_math("through \\(h\\) only"), "through $h$ only");
}

// ---- math: structure and the alignment gate ------------------------------

#[test]
fn structure_is_split_off_verbatim() {
    let (p, b, s) = math::split_structure("  - [ ] nested η rises  ");
    assert_eq!((p, b, s), ("  - [ ] ", "nested η rises", "  "));
    let (p, b, s) = math::split_structure("> 12. quoted ρ\\");
    assert_eq!((p, b, s), ("> 12. ", "quoted ρ", "\\"));
    let (p, b, _) = math::split_structure("### Head κ");
    assert_eq!((p, b), ("### ", "Head κ"));
    let (p, b, _) = math::split_structure("[^lure]: Lure, with ρ");
    assert_eq!((p, b), ("[^lure]: ", "Lure, with ρ"));
}

#[test]
fn gate_refuses_lost_markup_and_whitespace() {
    // 2026-10-02 live: these passed every word-level gate
    let orig = "**Narrow Lindy Effect (median)**: Pareto distribution with α = 1";
    assert!(!math::edits_confined(orig, "Narrow Lindy Effect (median): Pareto distribution with $\\alpha = 1$"));
    assert!(math::edits_confined(orig, "**Narrow Lindy Effect (median)**: Pareto distribution with $\\alpha = 1$"));
    // a prose-role arrow may not be absorbed into math
    let arrows = "Identity probe → State check while η rises";
    assert!(!math::edits_confined(arrows, "Identity probe $\\to$ State check while $\\eta$ rises"));
    assert!(math::edits_confined(arrows, "Identity probe → State check while $\\eta$ rises"));
    // a replaced region must itself read as math (asf spike, 2026-10-02)
    let plus = "Sub-scope β (residual): PID + non-positive-real plant";
    assert!(!math::edits_confined(plus, "Sub-scope $\\beta$ (residual): PID $+$ non-positive-real plant"));
    assert!(math::edits_confined(plus, "Sub-scope $\\beta$ (residual): PID + non-positive-real plant"));
    assert!(math::edits_confined("where a + b ≤ c holds", "where $a + b \\leq c$ holds"));
    // link destinations and code stay out of replaced regions (link *text*
    // is prose and may be promoted)
    let o = "see [ρ](https://x.org/ρ) and ρ";
    assert!(!math::edits_confined(o, "see [ρ](https://x.org/$\\rho$) and $\\rho$"));
    assert!(math::edits_confined(o, "see [$\\rho$](https://x.org/ρ) and $\\rho$"));
    assert!(!math::edits_confined("run `ρ` and ρ", "run `$\\rho$` and $\\rho$"));
}

#[test]
fn site_promotion_keeps_list_marker_indent_and_hard_break() {
    // the real model's output for these lines, 2026-10-02 (before the fix,
    // all three were written)
    let model = Model::fake(|t: &str| {
        t.replace("η", "$\\eta$").replace("α = 1", "$\\alpha = 1$").replace("**", "")
    });
    let nested = "  - nested item where the gain η rises  ";
    assert_eq!(promote_site(&model, nested).text, "  - nested item where the gain $\\eta$ rises  ");
    // the bold-stripping proposal is refused; the line stays as written
    let bold = "- **Narrow Lindy Effect (median)**: Pareto distribution with α = 1";
    let r = promote_site(&model, bold);
    assert_eq!(r.text, bold);
    assert_eq!(r.flags.len(), 1);
}

#[test]
fn prose_separators_split_the_work() {
    // the model never sees a prose separator, and one refused piece does not
    // block the others
    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
    let log = seen.clone();
    let model = Model::fake(move |t: &str| {
        log.borrow_mut().push(t.to_string());
        t.replace("η", "$\\eta$").replace("κ", "$\\kappa$")
    });
    let line = "gain η rises · see notes → and κ falls";
    let r = promote_site(&model, line);
    assert_eq!(r.text, "gain $\\eta$ rises · see notes → and $\\kappa$ falls");
    for t in seen.borrow().iter() {
        assert!(!t.contains('·') && !t.contains('→'), "model saw a separator: {t}");
    }
}

#[test]
fn glued_command_fix_spares_longer_commands() {
    // `\infty` was rewritten to `\in fty` (and `\int`, `\top`, `\cdots`)
    assert_eq!(math::postprocess("$x \\to \\infty$"), "$x \\to \\infty$");
    assert_eq!(math::postprocess("$\\int_0^t \\cdots \\top$"), "$\\int_0^t \\cdots \\top$");
    assert_eq!(math::postprocess("$\\Vertw$ and $\\ltt$"), "$\\Vert w$ and $\\lt t$");
}

#[test]
fn consistency_accepts_common_spellings_and_structure() {
    // refusals observed 2026-10-02 that were the gate's fault, not the model's
    let o = "when F(M) < 1 - ε no update helps";
    assert!(math::math_content_consistent(o, "when $F(M) \\lt 1 - \\epsilon$ no update helps"));
    let o = "the persistence condition α > ρ/R holds";
    assert!(math::math_content_consistent(o, "the persistence condition $\\alpha \\gt \\frac{\\rho}{R}$ holds"));
    let o = "floor above threshold (α_min > ρ/R) always";
    let p = "floor above threshold ($\\alpha_{\\min} \\gt \\rho/R$) always";
    assert!(math::math_content_consistent(o, p));
    assert!(math::preserves_prose(o, p), "subscript word counted as prose");
}

#[test]
fn much_greater_may_not_weaken() {
    let o = "Timescale ordering ν_M >> ν_Σ >> ν_O holds";
    assert!(!math::math_content_consistent(o, "Timescale ordering $\\nu_M \\gt \\nu_\\Sigma \\gt \\nu_O$ holds"));
    assert!(math::math_content_consistent(o, "Timescale ordering $\\nu_M \\gg \\nu_\\Sigma \\gg \\nu_O$ holds"));
    assert_eq!(math::postprocess("$a >> b$"), "$a \\gg b$");
}

#[test]
fn sentences_are_promoted_separately() {
    let pieces = math::split_sentences("First has η here. Second is plain. Fig. 3 shows `a. B` and $x. Y$ too.");
    let texts: Vec<&str> = pieces.iter().filter(|(_, sep)| !sep).map(|(t, _)| *t).collect();
    // splits at real sentence ends, not before a digit, nor inside code/math
    assert_eq!(texts, vec!["First has η here.", "Second is plain.", "Fig. 3 shows `a. B` and $x. Y$ too."]);
    let joined: String = pieces.iter().map(|(t, _)| *t).collect();
    assert_eq!(joined, "First has η here. Second is plain. Fig. 3 shows `a. B` and $x. Y$ too.");
}

#[test]
fn unavailable_model_is_one_state_not_many_errors() {
    let model = Model::new("definitely-not-a-model:0b");
    let a = promote_site(&model, "gain η rises");
    if !a.skipped {
        // ollama answered with a completion for a nonexistent model name —
        // nothing to assert about the breaker here
        return;
    }
    assert!(model.down_reason().is_some());
    let b = promote_site(&model, "and ρ falls");
    assert!(b.skipped);
    assert_eq!(b.text, "and ρ falls");
}

#[test]
fn display_blank_lines_stay_inside_blockquotes() {
    let input = "> quoted\n> $$\n> x = y\n> $$\n> after\n";
    assert_eq!(
        math::fix_display_math_blanks(input),
        "> quoted\n>\n> $$\n> x = y\n> $$\n>\n> after\n"
    );
}

#[test]
fn math_stage_keeps_line_endings_and_trailing_blanks() {
    // asf end-to-end, 2026-10-02: each run removed one trailing blank line
    // (rebuilt with lines()+join), and CRLF was flattened to LF
    let input = "text\n\n$$x$$\nmore\n\n\n";
    let once = math::fix_display_math_blanks(input);
    assert!(once.ends_with("more\n\n\n"), "{once:?}");
    assert_eq!(math::fix_display_math_blanks(&once), once);
    let crlf = "a\r\n$$\r\nx\r\n$$\r\nb\r\n";
    let out = math::fix_display_math_blanks(crlf);
    assert_eq!(out, "a\r\n\r\n$$\r\nx\r\n$$\r\n\r\nb\r\n");
}

#[test]
fn replaced_text_must_survive_into_its_span() {
    // llama3.2 proposals that passed every other gate (2026-10-02 model
    // comparison); Muse Glimmer's correct versions alongside
    let o = "persistence compares α against ρ/R in time";
    assert!(!math::edits_confined(o, "persistence compares $\\alpha$ against $\\rho/\\rho$ in time"));
    assert!(math::edits_confined(o, "persistence compares $\\alpha$ against $\\rho/R$ in time"));
    let o = "saturation at large ||δ||, threshold effects";
    assert!(!math::edits_confined(o, "saturation at large $\\delta$, threshold effects"));
    assert!(math::edits_confined(o, "saturation at large $||\\delta||$, threshold effects"));
    let o = "Gap: cognitive cost of Σ_t.";
    assert!(!math::edits_confined(o, "Gap: cognitive cost of $\\Sigma_t$"));
    assert!(math::edits_confined(o, "Gap: cognitive cost of $\\Sigma_t$."));
    let o = "§B — Strengthening ideas over 𝒜";
    assert!(!math::edits_confined(o, "§B — Strengthening ideas over $\\mathcal{S}$"));
    assert!(math::edits_confined(o, "§B — Strengthening ideas over $\\mathcal{A}$"));
    // reading η2p as partial eta-squared keeps every operand
    assert!(math::edits_confined("p = .43, η2p < .01", "$p = .43, \\eta^2_p \\lt .01$"));
}

#[test]
fn frontmatter_after_a_leading_comment_is_left_alone() {
    // verisectorium mining copies: a provenance comment above frontmatter.
    // CommonMark only knows byte-0 frontmatter, so the block parsed as prose
    // and joining folded each YAML key into the previous key's comment.
    let input = "<!--\n  provenance note\n-->\n\n---\nslug: x\nregister: [a, b]   # why\nsupport-kind: [c]\nstage: drafted\n---\n\nBody text\nwrapped.\n";
    let out = press(input);
    assert!(out.contains("register: [a, b]   # why\nsupport-kind: [c]\n"), "{out}");
    assert!(out.ends_with("Body text wrapped.\n"), "body still unwraps: {out}");
}

#[test]
fn single_star_emphasis_may_not_be_dropped() {
    // independent verification of the unicode-math spike, 2026-10-02
    let o = "the *gain η* rises";
    assert!(!math::edits_confined(o, "the *gain $\\eta$ rises"));
    assert!(math::edits_confined(o, "the *gain $\\eta$* rises"));
    assert!(math::edits_confined("the optimum η* holds", "the optimum $\\eta^\\ast$ holds"));
}
