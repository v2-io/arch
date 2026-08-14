# aspecta-ir — finish note

> [!note] **History, not the binary.** This describes the retired first-snapshot code; none of it is in the current crate (`impl/README.md`). "Landed" below is past-tense of that snapshot.


*Landed. Checked against `src/ir.rs`, `src/render.rs`, `tests/ir_render.rs` (2026-08-13, 19 tests in the crate).*

In-memory tree (`Aspecta` / `Node`) plus `render_text`. No `read_dir` in this layer.

Nodes are dir, file, symlink, or aggregate. Hand-built fixture `broot_failure_fixture`: `.git` is not a child; unexpanded `01-aat-core` still contains `src/`.
