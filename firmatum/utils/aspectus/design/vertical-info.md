# Vertical info organization

*Row opened by Joseph, 2026-08-14, with the repro that shows the problem: `aspectus --lines 200 --depth 1 ~/src/` (and depth 2–3, lines 100/300) — furniture-heavy multi-repo roots. Not designed; the problem statement and the material to design from.*

## The problem, from the look itself

On `~/src` every child line carries up to five fact-groups — heat·age cluster, census, mass, `[git: …]`, `[has: …]` — of wildly varying width. Each is individually honest and aligned to its tab-stop, but the *composition* fails at this density: no fact-group has a stable column across lines (the census ends where it ends and git begins there), so the eye cannot scan any single fact vertically, and the richest lines are the least readable. Horizontal alignment machinery solved the sparse case; the dense case needs **vertical** organization — position-carries-fact across the whole look, not just within a line.

## Material to design from

- [[shorthand|Shorthand]]'s placement classes — the dense case may be where **near-left**, **far-left**, and **glyph-block** earn their existence (compact glyphs instead of wordy brackets), and per-kind edge offsets return.
- A line owning **sub-lines**: a repo child could spend two lines (name + facts-line) the way the *root* header now does — the simple-header decision recursively applied to heavyweight children. Budget-honest (two lines cost two lines); maybe only when a line's facts exceed a width.
- Facet columns: `[git: …]` at one stable column for every line that has it; `[has: …]` at another — column-per-fact-group rather than flow layout.
- The eyes doc's channels: vertical position is the *cheap* channel; the dense look is currently spending the expensive one (width) on what position could carry.

## Acceptance

Joseph's gauntlet reads clean at a glance: `~/src` at depth 1/2/3 × lines 100/200/300. "Clean" = any fact-group findable by eye-column across all lines; no line an unscannable jumble; determinism and budget-honesty unchanged.
