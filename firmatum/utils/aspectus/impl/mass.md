# Mass — finish note (2026-08-14, Wave C)

Landed together with the reworked census (design/dir-census.md arrived
mid-wave; census rendering and mass are one seam, `Census::render` in
`src/n_level.rs`, deliberately one function and cheap to move).

Shape shipped:

- Every dir node carries `Mass { files, lines, est, bounded }`, bottom-up
  for expanded dirs, a dedicated `deep_mass` walk at depth cutoffs
  (furniture-aware, cycle-guarded, one-fs, `(dev,ino)` count-once, own
  name cap 500k → `≥`).
- **Rendering:** the census's dir bucket carries deep files
  (`[dir×4 ≈338f · md×2]`) and the subtree's text lines follow the census
  (`≈61k lines`) — the deep file total is *computable* from the census
  (dir bucket + suffix buckets), so it is not printed twice; lines are the
  one number the census lacks. My call, flagged for ratification with the
  rest of the census glyphs.
- `≥` replaces `≈` under bounds (walk-bound cut, denied, mount stop, mass
  cap, unreadable text). Binary files join `f` but never lines. Read-budget
  overflow estimates lines from size at observed bytes/line, `≈` (est flag
  tracked internally; the glyph does not distinguish est from
  grouped-exact — both are honestly non-exact `≈`).
- Furniture-hidden bodies never join mass. **Gitignored bodies still do**
  where the furniture map doesn't know them — that exclusion is
  design/gitignore-bodies.md's row, not built; mass will tighten when it
  lands.
- Diamond symlinks: expanded in the look, counted once in aggregates (the
  design's leaning, shipped; `mass_dup` on the node).

Census rework specifics: no total, `kind×N`, dirs-first containers,
name-beats-count at n=1 (`[furniture/ ≈2f · …]`, `[thing.dat]`,
name-length cap 24), `+` only on the leaf form, `·` separators, `≥ `
prefix inside the bracket for bounded membership, empty-container `≈0f`
suppressed. Old-form census asserts across the test suite updated in this
seam. One principle-2 nuance not yet built: a *singular extensionless*
file inside a multi-bucket census still renders `other×1`, not its name
(the sole-entry name form does ship) — glyph-level, waits on ratification. Tests: `tests/mass.rs` (6).
