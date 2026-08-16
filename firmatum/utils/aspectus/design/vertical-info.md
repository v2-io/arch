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

## Steward asks (inbox, 2026-08-14 — verbatim, routed 2026-08-15)

Three inbox entries land here; together they decide the shape this row's design starts from: **the fact-columns hold every fact of their kind, and a line's non-column material may spill to owned sub-lines rather than displace the columns.**

**1. Mass lines go in the `lines` column.** Joseph, seeing:

```
│   │   ├── build/                                     0.00 · 8.8d ago  [md×1]  ≈101 lines
│   │   ├── synthesis/                                 0.00 · 8.8d ago  [md×4]  ≈681 lines
│   │   ├── reflections-coord-2026-08-10.md        25  0.00 · 4.1d ago
```

> would rather have the ~= 681 lines in the lines column. etc.:

```
│   │   ├── build/                               ≈101  0.00 · 8.8d ago  [md×1]
│   │   ├── synthesis/                           ≈681  0.00 · 8.8d ago  [md×4]
│   │   ├── reflections-coord-2026-08-10.md        25  0.00 · 4.1d ago
```

(This supersedes [[mass|Mass]]'s shipped call "the subtree's text lines follow the census" — the deep line total is the `lines` fact's deep-agg office and sits in that column, marks `≈`/`~`/`≥` kept.)

**2. Wrap a line's description so columns line up; `--lines` is a logical count.** Joseph, seeing symlink targets shove the whole far-right block:

```
│   │   │   ├── claim-naming-criteria.md -> ../../../theory/src/claim-naming-criteria.md     52               0.00 · 5.2d ago
```

> Recommend:

```
│   │   │   ├── claim-naming-criteria.md           52               0.00 · 5.2d ago
                │ -> ../../../theory/src/
                ╰    claim-naming-criteria.md
│   │   │   ├── claim-dispatch-compounds.md        53               0.00 · 8.1d ago
                │ -> ../../../theory/src/
                ╰    claim-dispatch-compounds.md
```

> Or, in other words, allow the wrapping of the file description so that columns properly line up. I would even go as far as to say don't worry about counting this as extra lines against the line count --lines, which can be essentially a logical count, instead of exact count. Because the secondary and tertiary lines (etc.) no longer need numbers in the columns after that, they can overflow into the columns without it messing up vertical flow too much.

Decided by this ask: sub-lines exist; they carry no column cells; **`--lines` counts logical lines** (a node with sub-lines is one line of budget). Which material spills (symlink target, `[git: …]`, `[has: …]`, censuses?) and when (always, or only past a width) is the design work of this row.

**3.** The third inbox entry — the header names every non-default effective setting — is routed to [[overview-invariants|Overview invariants]].
