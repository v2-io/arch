//! Sibling-share allocator. A cutoff is not a summary.

use aspectus::{allocate_shares, explain, Node};

fn eight_dirs() -> Vec<Node> {
    (0..8).map(|i| Node::dir(format!("d{i}"))).collect()
}

#[test]
fn eight_siblings_twenty_lines_no_share_of_eighteen() {
    let kids = eight_dirs();
    let alloc = allocate_shares(&kids, 20);
    assert_eq!(alloc.shares.len(), 8);
    assert!(
        alloc.shares.iter().all(|&s| s < 18),
        "no sibling may get a height-fill share of 18: {:?}",
        alloc.shares
    );
    // When everyone fits, nobody is silently dropped.
    assert!(alloc.aggregated.is_empty());
    assert!(alloc.shares.iter().all(|&s| s >= 1));
    // First child must not swallow the remainder.
    let remain = 20 - alloc.self_lines;
    assert!(
        alloc.shares[0] <= remain - (8 - 1),
        "first child share {} ate the remainder {remain}",
        alloc.shares[0]
    );
}

#[test]
fn tight_budget_names_the_omitted() {
    let kids = eight_dirs();
    let alloc = allocate_shares(&kids, 5); // self=1, remain=4 → 3 listed + aggregate
    assert_eq!(alloc.aggregated.len(), 8 - 3);
    assert_eq!(alloc.shares.iter().filter(|&&s| s > 0).count(), 3);
    for &i in &alloc.aggregated {
        assert_eq!(alloc.shares[i], 0);
    }
}

#[test]
fn explain_lists_shares_and_why() {
    let kids = eight_dirs();
    let alloc = allocate_shares(&kids, 20);
    let text = explain(&kids, &alloc, 20);
    assert!(text.contains("budget 20"), "{text}");
    assert!(text.contains("share="), "{text}");
    assert!(text.contains("weight="), "{text}");
    assert!(text.contains("d0/"), "{text}");
}
