//! The deterministic converter as the math engine, and the in-file memory of
//! declined math decisions: a declined passage gets a
//! `<!--md-press:not-math:KEY-->` marker at the end of its site, and is never
//! offered again, so a decided file reports nothing on every later run.

use md_press::math::{CONVERTER, Model, fix_inline_angles, not_math_marker, split_not_math};
use md_press::plan::{Detail, Plan, Tally, plan_file};

fn plan(input: &str, given: &[&str]) -> Plan {
    let model = Model::new(CONVERTER);
    let given: Vec<String> = given.iter().map(|s| s.to_string()).collect();
    let mut tally = Tally::default();
    plan_file("a.md", "a.md", input.to_string(), "test-run", None, Some(&model), &given, false, &mut tally).unwrap()
}

fn math_code(p: &Plan, removed: &str) -> String {
    p.items
        .iter()
        .find(|i| matches!(&i.detail, Detail::Math { removed: r, .. } if r == removed))
        .unwrap_or_else(|| panic!("no math item removing «{removed}»"))
        .code
        .clone()
}

#[test]
fn the_converter_is_the_default_engine() {
    assert_eq!(md_press::math::DEFAULT_MODEL, CONVERTER);
    assert!(Model::new(CONVERTER).is_converter());
    assert!(!Model::new("llama3.2:3b").is_converter());
}

#[test]
fn converts_unicode_math_and_is_idempotent() {
    let p = plan("The gain η sets the rate, and ‖δ‖ ≤ R holds.\n", &[]);
    assert_eq!(p.output, "The gain $\\eta$ sets the rate, and $\\lVert \\delta\\rVert \\leq R$ holds.\n");
    let again = plan(&p.output, &[]);
    assert_eq!(again.output, p.output);
    assert!(again.items.is_empty());
}

#[test]
fn a_declined_passage_is_remembered_and_never_offered_again() {
    let input = "The regime η is a label here, and κ×A is the bias.\n";
    let p = plan(input, &[]);
    let code = math_code(&p, "η");
    let q = plan(input, &[&code]);
    assert_eq!(
        q.output,
        "The regime η is a label here, and $\\kappa \\times A$ is the bias.<!--md-press:not-math:η-->\n"
    );
    // the decided file: nothing to report, and a plain --fix leaves it alone
    let again = plan(&q.output, &[]);
    assert!(again.items.is_empty(), "{:?}", again.items.iter().map(|i| i.line).collect::<Vec<_>>());
    assert_eq!(again.output, q.output);
    // still remembered after an unrelated edit to the same line
    let edited = q.output.replace("is a label here", "is only a label here");
    let after = plan(&edited, &[]);
    assert!(after.items.is_empty());
    assert_eq!(after.output, edited);
}

#[test]
fn a_new_passage_on_a_marked_line_is_still_offered() {
    let marked = "The regime η is a label.<!--md-press:not-math:η-->\n";
    assert!(plan(marked, &[]).items.is_empty());
    let added = "The regime η is a label, and ‖δ‖ ≤ R holds.<!--md-press:not-math:η-->\n";
    let p = plan(added, &[]);
    assert_eq!(p.items.len(), 1);
    assert_eq!(p.output, "The regime η is a label, and $\\lVert \\delta\\rVert \\leq R$ holds.<!--md-press:not-math:η-->\n");
}

#[test]
fn a_declined_passage_in_a_table_cell_is_remembered_in_that_cell() {
    let input = "| a | b |\n|---|---|\n| η is a label | x ≤ 3 |\n";
    let p = plan(input, &[]);
    let code = math_code(&p, "η");
    let q = plan(input, &[&code]);
    assert_eq!(q.output, "| a | b |\n|---|---|\n| η is a label<!--md-press:not-math:η--> | $x \\leq 3$ |\n");
    assert!(plan(&q.output, &[]).items.is_empty());
}

#[test]
fn two_declines_on_one_line_get_two_markers() {
    let input = "Both η and ρ are labels in this line.\n";
    let p = plan(input, &[]);
    let (a, b) = (math_code(&p, "η"), math_code(&p, "ρ"));
    let q = plan(input, &[&a, &b]);
    assert_eq!(q.output, "Both η and ρ are labels in this line.<!--md-press:not-math:η--><!--md-press:not-math:ρ-->\n");
    assert!(plan(&q.output, &[]).items.is_empty());
}

#[test]
fn raw_angles_in_inline_math_are_fixed_and_currency_is_not_math() {
    assert_eq!(fix_inline_angles("when $a<b$ holds"), "when $a\\lt b$ holds");
    assert_eq!(fix_inline_angles("$x<=y$ and $p>>q$"), "$x\\leq y$ and $p\\gg q$");
    assert_eq!(fix_inline_angles("costs $5 < $10 now"), "costs $5 < $10 now");
    assert_eq!(fix_inline_angles("display $$a<b$$ stays"), "display $$a<b$$ stays");
    assert_eq!(fix_inline_angles("`$a<b$` is code"), "`$a<b$` is code");
    let p = plan("The bound $\\rho<R$ holds.\n", &[]);
    assert_eq!(p.output, "The bound $\\rho\\lt R$ holds.\n");
    let code = math_code(&p, "<");
    let q = plan("The bound $\\rho<R$ holds.\n", &[&code]);
    assert_eq!(q.output, "The bound $\\rho<R$ holds.<!--md-press:not-math:$\\rho<R$-->\n");
    assert!(plan(&q.output, &[]).items.is_empty());
}

#[test]
fn marker_grammar() {
    assert_eq!(not_math_marker("η").as_deref(), Some("<!--md-press:not-math:η-->"));
    assert_eq!(not_math_marker("a--b"), None);
    assert_eq!(not_math_marker("x-"), None);
    assert_eq!(not_math_marker(""), None);
    let (t, m, k) = split_not_math("text η.<!--md-press:not-math:η--> <!--md-press:not-math:ρ-->");
    assert_eq!(t, "text η.");
    assert_eq!(m, "<!--md-press:not-math:η--> <!--md-press:not-math:ρ-->");
    assert_eq!(k, vec!["η".to_string(), "ρ".to_string()]);
    let (t, m, k) = split_not_math("no markers <!-- a comment -->");
    assert_eq!((t, m, k.len()), ("no markers <!-- a comment -->", "", 0));
}

#[test]
fn a_heading_and_a_list_item_keep_their_structure() {
    let p = plan("## The gain η\n\n- **Item**: where ‖δ‖ ≤ R\n", &[]);
    assert_eq!(p.output, "## The gain $\\eta$\n\n- **Item**: where $\\lVert \\delta\\rVert \\leq R$\n");
}

#[test]
fn regions_align_when_prose_text_also_occurs_inside_a_span() {
    // `, ` occurs inside `(0, 1)` and again between the spans
    let input = "For $K_c > 1$ and any $\\lambda \\in (0, 1)$, $\\lambda^{K_c} < \\lambda$ here.\n";
    let p = plan(input, &[]);
    assert_eq!(p.output, "For $K_c \\gt 1$ and any $\\lambda \\in (0, 1)$, $\\lambda^{K_c} \\lt \\lambda$ here.\n");
    assert_eq!(p.items.len(), 2);
    // declining either one leaves the other intact, with no spliced text
    let first = p.items[0].code.clone();
    let q = plan(input, &[&first]);
    assert!(q.output.contains("$\\lambda \\in (0, 1)$, $\\lambda^{K_c}"), "{}", q.output);
    assert!(plan(&q.output, &[]).items.is_empty());
}
