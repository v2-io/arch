# Links and one filesystem — finish note (2026-08-14, Wave C)

- Symlinks expand as what they point at: `fs::metadata` follows, facts are
  the target's, `-> target` (verbatim, as recorded) decorates the name;
  broken links say `[broken]`, exit 0. Symlinked dirs recurse, spend depth
  and budget like real dirs, and census at cutoffs.
- **Cycle guard:** a `(st_dev, st_ino)` stack of dirs being expanded on
  the current path; a second encounter prints `[cycle]` (constant, awaits
  ratification) and does not recurse. Bounded time verified by test.
- **Diamond:** both links expand in the look; mass counts the target once
  (design leaning, shipped — see impl/mass.md).
- **One-fs:** default on (config `one-fs`, flag `--no-one-fs` → off; rides
  the caller stack per the design leaning). A dir on another device prints
  with `[other fs]` (constant, awaits ratification) and is not entered —
  no census of its innards (reading names across the boundary is the thing
  the default refuses); an ancestor's mass floors (`≥`). A symlink whose
  *target* is on another filesystem is stopped the same way — target
  string still shown (the design's leaning).
- The `[other fs]` path has no portable fixture (needs a mount); the flag
  path and the marks are covered by `tests/links_fs.rs` (8); the mount
  behavior was exercised manually against /System/Volumes cases.
