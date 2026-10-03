# Dot-only directory — finish note (2026-10-03)

Landed per [[../design/dot-only|design]]. A directory whose every readdir name starts with `.` says `[dot-only]`.

**Code**

- **`src/n_level.rs`:** `Node.dot_only`, set in `gather_dir` beside `empty` from the same raw `enumerate` result (`!entries.is_empty() && !iter_err && all names start with '.'`), before any filter. Unread nodes (`⊘`, denied, cycle, other fs) never reach it.
- **`src/ready.rs`:** `look_marks` appends `[dot-only]` right after `[empty]`'s slot (the two are exclusive).
- **`src/json.rs`:** `"dot_only": true` (present-when-true, additive within schema 1, not truncation).
- **`src/facts.rs`:** new `dot-only` row (✓, near-right, mark, always, `[dot-only]*`).
- **Help:** the honesty paragraph names it and lists examples.

**Tests:** `tests/dot_only.rs` (3), covering the rule across omitted/listed/hidden-furniture/symlink dot children, mixed and empty dirs, has-spot coexistence, the root, JSON, and help. No golden moved. Suite 335 green.

**Dogfood** (release vs installed v0.1.20). arch `--depth 4` changed 2 lines (`src/ [.gitkeep]  [dot-only]` ×2). aat, asf, `~/src --depth 1`, vivarium, this crate, and grok are byte-identical. `~/src --depth 2` / `--depth 3` show 4 marks, among them `audits/ [.integrated/ ≈13f]  [dot-only]`.

**Calls made**

- The spelling `[dot-only]` (proposed at landing; ratified by Joseph the same day, *"(I'm happy with 'dot-only', fwiw)"*; the runner-up `[only .*]` is in the design).
- Kept beside a one-name census even where redundant (`[.gitkeep]  [dot-only]`; design Open).
- Mark order: after `[empty]`'s slot, last among the marks.
- `design/empty-dir.md`'s `.DS_Store` Open is closed by this row (struck through with the resolution), and its "not `[empty]`" table row now points here.
