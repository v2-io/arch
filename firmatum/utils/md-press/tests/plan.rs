//! Plans: every decision gets a code, flipping by code changes exactly that
//! decision, the file remembers flipped break decisions, and codes go stale
//! the moment anything they were derived from changes.

use md_press::math::Model;
use md_press::plan::{Detail, Plan, Tally, plan_file};

fn plan(input: &str, given: &[&str], model: Option<&Model>) -> Plan {
    plan_at("a.md", input, given, model)
}

fn plan_at(key: &str, input: &str, given: &[&str], model: Option<&Model>) -> Plan {
    let given: Vec<String> = given.iter().map(|s| s.to_string()).collect();
    let mut tally = Tally::default();
    plan_file("a.md", key, input.to_string(), "test-run", None, model, &given, false, &mut tally).unwrap()
}

fn code_where(p: &Plan, pred: impl Fn(&Detail) -> bool) -> String {
    p.items.iter().find(|i| pred(&i.detail)).expect("item").code.clone()
}

const WRAPPED: &str = "A paragraph that was\nwrapped by a column\nwrapper.\n\nAnother\nparagraph.\n";

#[test]
fn a_paragraph_flips_as_one_and_the_file_remembers() {
    let p = plan(WRAPPED, &[], None);
    assert_eq!(p.output, "A paragraph that was wrapped by a column wrapper.\n\nAnother paragraph.\n");
    assert_eq!(p.items.len(), 2, "one item per paragraph");
    let first = code_where(&p, |d| matches!(d, Detail::Para { first: 1, .. }));

    let q = plan(WRAPPED, &[&first], None);
    assert_eq!(q.matched, vec![first.clone()]);
    assert_eq!(q.output, "A paragraph that was  \nwrapped by a column  \nwrapper.\n\nAnother paragraph.\n");

    // the marks are the memory: nothing left to decide in that paragraph
    let again = plan(&q.output, &[], None);
    assert_eq!(again.output, q.output);
    assert!(again.items.is_empty());
}

#[test]
fn codes_are_stable_and_bound_to_content_and_path() {
    let a = plan(WRAPPED, &[], None);
    let b = plan(WRAPPED, &[], None);
    let codes = |p: &Plan| p.items.iter().map(|i| i.code.clone()).collect::<Vec<_>>();
    assert_eq!(codes(&a), codes(&b));
    let elsewhere = plan_at("b.md", WRAPPED, &[], None);
    assert_ne!(codes(&a), codes(&elsewhere));
    // a code from before an edit names nothing after it
    let stale = &a.items[0].code;
    let edited = plan(&WRAPPED.replace("Another", "One more"), &[stale], None);
    assert!(edited.matched.is_empty());
}

#[test]
fn math_changes_are_items_and_a_flip_restores_only_that_piece() {
    let model = Model::fake(|t| t.replace('η', "$\\eta$").replace('κ', "$\\kappa$"));
    let input = "The gain η rises. Then κ falls.\n";
    let p = plan(input, &[], Some(&model));
    assert_eq!(p.output, "The gain $\\eta$ rises. Then $\\kappa$ falls.\n");
    let maths: Vec<_> = p.items.iter().filter(|i| matches!(i.detail, Detail::Math { .. })).collect();
    assert_eq!(maths.len(), 2, "one item per converted sentence");
    let eta = code_where(&p, |d| matches!(d, Detail::Math { removed, .. } if removed == "η"));
    let q = plan(input, &[&eta], Some(&model));
    assert_eq!(q.output, "The gain η rises. Then $\\kappa$ falls.\n");
    // not remembered: a declined math change is proposed again
    let again = plan(&q.output, &[], Some(&model));
    assert!(again.items.iter().any(|i| matches!(&i.detail, Detail::Math { removed, .. } if removed == "η")));
}

#[test]
fn display_blank_lines_are_items() {
    let model = Model::fake(|t| t.to_string());
    let input = "Text before.\n$$\nx = 1\n$$\nText after.\n";
    let p = plan(input, &[], Some(&model));
    assert_eq!(p.output, "Text before.\n\n$$\nx = 1\n$$\n\nText after.\n");
    let before = code_where(&p, |d| matches!(d, Detail::Blank { after: false, .. }));
    let q = plan(input, &[&before], Some(&model));
    assert_eq!(q.output, "Text before.\n$$\nx = 1\n$$\n\nText after.\n");
}

#[test]
fn unknown_codes_match_nothing() {
    let p = plan(WRAPPED, &["zzzzz"], None);
    assert!(p.matched.is_empty());
    assert_eq!(p.output, plan(WRAPPED, &[], None).output);
}
