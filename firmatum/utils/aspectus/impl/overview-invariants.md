# overview-invariants — finish note

*Landed. Source: `src/overview.rs` (stamp/root), `src/columns.rs::render`
(header shape). Tests: `tests/overview.rs` (real binary) plus
`overview::tests` for the UTC stamp.*

Header (simple-header steer, 2026-08-14, folded into Wave D): up to three
lines — the UTC stamp, then the root's own facts on their own line **only
when it has any** (heat·age, `[git: …]`, `[has: …]`, quiet-spoken
columns), then the **bare** absolute path directly above its children
(facts-*before*-path kept: the path line stays adjacent to the tree, per
Joseph's earlier vote). Path from `std::path::absolute` (logical, not
realpath); a symlinked root keeps its `-> target` decoration — it
completes the name. Stamp is UTC second resolution, when this print ran.

Budget: the header's line(s) are charged against `--lines` before the
tree's share (`--explain-budget` names the split). Recorded corner: when
a tiny budget makes the *allocator* fold the root's children into a
census, that census lands on the facts line post-charge — a one-line
overshoot at `--lines 2` with a fact-bearing root. Flag if it bites.
