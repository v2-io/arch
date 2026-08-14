# summarization / dir-census — finish note

*Landed for the depth cutoff. Source: `src/n_level.rs` `census_dir`. Tests: `tests/n_level.rs` `depth_1_is_children_only`.*

Stopping at `--depth` no longer prints a bare `a/` when `a/` has children. The line carries `[N: k .ext, …]`. Leaf census (leftover files under a line budget) is still open.
