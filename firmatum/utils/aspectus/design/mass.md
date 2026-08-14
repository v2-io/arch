# Mass — the size of the unseen

*The short-term point of the whole tool, named. Seeded 2026-08-14.*

Joseph:

> "Right now all agents are very subpar at situational awareness about where they are. They spot and they focus and they discount and never realize where the holes in their understanding are. … I can't tell you how many times I've heard — 'Oh, I didn't realize AAT alone has over 300 segments…' after reading 4 or 5 and reporting that they understand AAT. So the biggest surprisal we are trying to fix in the short term is 'SURPRISE! This "directory" is far more than you thought it was!'"

[[dir-census|Dir census]] is one level deep: `01-aat-core/ [7: 4 dir, 2 .md, 1 other]` tells an agent almost nothing about what those 4 dirs hold. Mass is the cumulative weight of an unexpanded subtree, carried on its line:

```
├── 01-aat-core/  [7: 4 dir, …]  ≈340 files, ≈61k lines
```

so a glance at `asf/` *calibrates the agent's sense of how much it has not seen* before it spends a single read. An agent that says "oriented" after 5 of 300 segments failed because nothing in its glance carried the 300.

## Shipped (2026-08-14, Wave C)

Weight = descendant files + text lines (the mass unit), computed bottom-up on every dir and by a dedicated deep walk at cutoffs — furniture excluded, `(dev,ino)` counted once, cycle-guarded, one-fs. Rendered *through the census* (design/dir-census.md, same-day rework): the dir bucket carries deep files (`[dir×4 ≈338f · md×2]`), the subtree's text lines follow it (`≈61k lines`) — the deep file total is computable from the census so it does not print twice. `≥` under any bound (walk cut, denied, mount stop, mass name-cap 500k, unreadable text); read-budget overflow estimates lines from size, still `≈`. Gitignored bodies still count until [[gitignore-bodies|that row]] lands — the furniture map catches the big offenders (`target/`, …) today. Details: impl/mass.md.

## Mass-mark distinction (proposed 2026-08-14, hardening pass — Joseph ratifies the glyphs)

The hallway testers caught `≈` doing two jobs: rounding an *exact* count for the census channel (`61,234` → `≈61k`) and flagging a *budget-estimate* whose value genuinely depends on this walk's read budget ("directory line-totals unstable across flags… trust the file, distrust the directory total" — grok). One glyph for both meant an exact count and a walk-relative guess wore the same face, and the guess's instability discredited the exact ones. Implemented leaning, three marks by precedence:

- `≥` — a floor (walk bound, denied, mount stop, name cap): unchanged.
- `~` — **estimated**: some lines under this aggregate were inferred from size (constant 64 B/line) because the read budget ran out. Walk-relative by nature; the mark now confesses it. The constant is calibrated against measured estate trees (close audit 2026-08-14; the first-shipped 32 overestimated prose 2–3×): asf ≈100 B/line, vivarium ≈110, firmatum ≈66, memorata ≈58 — md-press-unwrapped markdown runs ≈110–125, dense code ≈40–60; 64 lands exact on the mixed trees and the `~` register owns the residual. Recalibrate here when the estate's texture shifts.
- `≈` — exact count, grouped for the eye. Stable across flags for the same tree.

Two stabilizers landed with it: the estimator's bytes-per-line is a constant (the old look-observed ratio made a dir's total depend on what else the walk read first), and each depth-cutoff subtree gets a deterministic *share* of the read budget in the parallel deep phase, so `--inspect git` spending reads inside `.git` no longer starves a sibling's total.

## Shape (as first sketched)

- **What counts as weight:** descendant file count first; total lines of non-binary files when [[linecount|Line counts]] exist; maybe bytes. Which of these prints, and when (always on dirs at a cutoff? quiet when small?), is lattice work — `size`'s "which number" nuance applies.
- **Honesty under bounds:** mass wants a deep walk; the [[walk-bound|walk bound]] refuses one. A mass computed under a bound prints `≥`; an unbounded mass is exact until the tree changes. This is the row where [[cache|Cache]] pays first — deep mass is the expensive derived fact that makes revisits cheap.
- **Furniture does not count:** `target/`'s 10k build objects are not the mass of a crate. Mass counts what an agent could be expected to comprehend; hidden/omitted furniture and gitignored bodies stay out (or print separately), else every Rust crate looks like a mountain of `.fingerprint`.
- **Not quietable at cutoffs.** Like [[denied|Denied]], mass at an unexpanded dir is existence-information, governed by [[summarization|Summarization]]'s law, not by Quiet.
