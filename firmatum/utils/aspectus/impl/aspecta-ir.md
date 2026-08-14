# aspecta-ir — finish note

*Landed. Checked against `src/ir.rs`, `src/render.rs`, `tests/ir_render.rs` (2026-08-13, 19 tests in the crate).*

In-memory tree (`Aspecta` / `Node`) plus `render_text`. No `read_dir` in this layer.

Nodes are dir, file, symlink, or aggregate. Hand-built fixture `broot_failure_fixture`: `.git` is not a child; unexpanded `01-aat-core` still contains `src/`.
