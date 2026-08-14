# Shorthand — placement classes and glyph columns

*Seeded 2026-08-14 from Joseph's direction, verbatim below. Not yet a row; the columns wave absorbs the placement vocabulary now, the glyph vocabulary matures with use.*

> "there are probably lots of 'columns' that could/should be either decoration on the name (e.g., '/' and symlink targets etc.), a 'real actual column' aligned either far left pre-hierarchy, immediate-left indented w children, immediate-right like overflow etc. is now, and far-right-aligned full columns — OR a sort of symbolic/glyph column/columns similar to the displays for permissions, and keeping in mind that we have all of unicode glyphs to give affordances with etc. (and, like the time-delta, you can decide ★★★ means one thing while ✫ means something else while ✭ means a third thing, etc. etc. — I'm sure there are a *lot* of glyphs that will give affordances and that are pipe safe etc. etc. that more than make up for lack of colors)"

## Placement classes (supersedes the lattice's column/INFO binary)

| Class | Where | Natural tenants |
|---|---|---|
| **decoration** | fused to the name | `/` on dirs, `-> target`, `"README title"` |
| **far-left** | before the hierarchy indent | glyph blocks (see below), heat color-analog marks |
| **near-left** | indented with the children, before names | line numbers-style facts, per-level marks |
| **near-right** | after the name at the computed tab-stop | today's `[kind: …]`, facets, censuses, marks |
| **far-right** | right-aligned full columns | `score · age` clusters, sizes, counts |
| **glyph-block** | a dense fixed-width symbolic column (permissions-style) | one glyph per fact, position = meaning |

## The glyph-block idea

Like `rwxr-xr-x`: a compact block where *position* carries which fact and *glyph* carries its value — learnable once, then read at a glance. SIGNA proved the mechanism for time; this generalizes it: distinct-but-related glyphs (`★★★` vs `✫` vs `✭`) can carry graded or categorically different meanings, and the pipe-safe Unicode range is large enough that glyph affordances "more than make up for lack of colors" in the no-TTY look. Candidate tenants: heat grade, git letter, kind class, molten/working-surface mark, denied/bound marks. Vocabulary must be: deterministic, taught in `--help` (the law channel), stable across releases once learned (a learned glyph is an interface), and always absent-not-faked when the fact is unobtainable.

## Open

- Which facts earn glyph-block slots first, and the block's position (far-left leading candidate — it survives narrow terminals and reads columnar like `ls -l`).
- Glyph vocabulary itself — pipe-safe survey needed; SIGNA's glyphs are the precedent family; Joseph ratifies meanings before they ossify (a learned glyph is expensive to change).
- Machine formats never get glyphs (JSON carries the facts as fields — same rule as SIGNA).
