# balanced / leaf-census — finish note

*Landed. Source: `src/n_level.rs` `apply_budget`. Tests: `tests/balanced.rs`.*

`--lines N` (config `lines`, default 80, `0` unlimited). Siblings share. A dir that only has its own line gets a dir-census on that line, not a child `[+]`. `[+N: …]` is leftover siblings after some were listed. `--explain-budget` on stderr.

**2026-08-14 — capacity-aware redistribution** (audit finding 9). `apply_budget` extra-share loop rewritten: round-robin capped by per-child `capacity()` (fully-rendered subtree lines), with the 1→3 step for dirs of 2+ children (a share of 2 is a dead value there — the fold spends 1). Unspent budget past the tree's capacity is named on stderr (`unspent N (tree exhausted)`). asf 80-line look: 62 → 80 lines spent; `01-aat-core/` shows `src/ [172: 170 .md, …]`. Tests: `tests/balanced.rs` `share_a_child_cannot_use_flows_to_one_that_can`, `budget_beyond_the_tree_is_named_unspent_not_lost`, `redistribution_is_deterministic`. Single-entry census question flagged open in [[../design/dir-census.md|dir-census]], not folded here.
