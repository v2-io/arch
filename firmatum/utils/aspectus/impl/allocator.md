# allocator — finish note

> [!note] **History, not the binary.** This describes the retired first-snapshot code; none of it is in the current crate (`impl/README.md`). "Landed" below is past-tense of that snapshot.


*Landed. Source: `src/budget.rs` `allocate_shares`. Tests: `eight_siblings_twenty_lines_no_share_of_eighteen`, `tight_budget_names_the_omitted`, `explain_lists_shares_and_why`.*

Directory line costs 1. If remaining ≥ number of children, each gets 1 and extras rotate over directories (weight: dir 4, symlink 2, file 1; +1 if tagged; +1 if the node already has labels). If remaining is smaller, one line is kept for the leftover names; the rest of the children get 1 in weight order.

`--explain-budget` writes those shares and weights to stderr.
