# globify — finish note

*Wave E, 2026-08-14. `src/globify.rs` (new); pipeline slot in `main.rs`; census fold-back in `n_level.rs::census_nodes`; rendering in `columns.rs`; structured node in `json.rs`. 10 tests (`tests/globify.rs`).*

## What shipped

- Candidate = names identical except **exactly one maximal decimal-digit run** (same prefix, same suffix, same width, same kind); guards exactly as designed: min count (config `globify.min`, default 5 — floor-clamped to 2), width honesty (mixed widths never fuse), files-with-files / dirs-with-dirs, symlinks never, **important files exempt** (the rest still collapse if they clear the threshold without them), and only *plain* nodes join (anything carrying kinds, facets, marks, denial, or an ignored flag stays listed by name — every guard leans against false collapse).
- Renders `output-[001-047].bak  (44 files)` — range-only spelling (the design's leaning over `[NN]` placeholders), true count; count < span is the gap signal.
- **Pipeline position:** pre-budget, after importance, **before quiet** — the one listee joins its level's sibling norms as a single member with aggregate facts (size = sum, mtime/git-ts = newest, heat = max, lines = sum only when every member's count is known). Collapsed dirs are never expanded and carry no aggregates beyond count/recency.
- **Budget honesty:** the group costs exactly one line; when a fold pushes it into a census it folds back as its *members* (`dat×30`, dirs by their count) — never one listee. Dir-group masses were never walked, so a parent census's `dir_files` floors (`≥`) rather than lies.
- Determinism: groups form biggest-first then key-order; a name joins at most one group; groups re-check the threshold after losses.
- `globify = off` and `--show-all` restore names (globify simply doesn't run under show-all). JSON: `"glob": {min, max, width, count, kind}` beside the pattern name — members recoverable in principle.

## Leanings surfaced (design Open)

- **Unpadded multi-width series stay listed** (`1..47` without zero-padding spans widths 1–2): width honesty forbids the fuse per the design's own law. Common in the wild; if it grates in dogfood the law is the thing to revisit, not the code.
- Aggregate marks on the collapsed line: count always; size/lines join the columns when those are on; no `≈`-style mark claimed yet (lattice-office question, still open).
- Threshold default 5 and key spellings `globify` / `globify.min` — ratify.
- Hex runs don't count (decimal only), per the leaning.

## Cross-row interference, recorded

Four existing test harnesses (`balanced`, `important`, `quiet`, one `json_format` case) used numbered-series fixtures that globify now legitimately collapses; each pins `ASPECTUS_GLOBIFY=off` with a dated comment — their subject is the allocator/norms, not this row.
