---
slug: sidecar-ubiquity-census
type: obs
depends: []
---

# The per-atom sidecar, counted across four corpora

*How often an atom in this estate carries a non-canon forward-facing attachment, and under how many different names — the count grounding [[working-notes-sidecar]].*

## What was counted

Four corpora that keep one claim per file, counted 2026-08-06 by listing the segment files and then counting those carrying a section (or udon element) whose declared role is *forward-facing notes that are not part of the claim*. The point of the count is not the volume of the notes ([[working-notes-deluge]] measures that) but whether the **position** exists at all, and whether the instances agree on what to call it.

## The numbers

| Corpus | Atom files | Carrying a sidecar | Form the sidecar takes |
|---|---|---|---|
| `asf/01-aat-core/src/` | 170 | **164** | `## Working Notes` (markdown section) |
| `vivarium/core/src/` | 115 | **115** | `## Working Notes` (markdown section) |
| `comproprium/vera/` | 12 | **12** | `\|working-notes` (first-class udon element) |
| `udon/v2/udon-needs/` | not characterized | 20 files | `## Open Questions` / `### Open Questions` / `## Notes` |

Three of the four reach effective saturation. The fourth is the interesting one: udon-needs uses a *different name* for the position, and uses it inconsistently — `## Open Questions` (13), `### Open Questions` (6), `## Open questions` (3), `# Notes` (3), `## Notes` (2), and `## Working Notes` (5) all appear in the same tree. Its denominator was not established, so the 20 is a presence count, not a rate.

## Method & scope

`ls` for denominators; `grep -rl` on the section heading for the markdown corpora; a sigil-frequency count (`grep -o '^\s*|[a-z-]+' | sort | uniq -c`) for the udon corpus, which reported `|working-notes` at 12 alongside `|segment`, `|title`, `|summary`, `|formal-expression`, `|epistemic-status` and `|discussion` — all at 12, i.e. the sidecar is as mandatory there as the claim body itself.

Four corpora in one estate, largely shared authorship. This establishes that the position is reached for reliably *here*; it is not evidence about corpora outside this estate, and it cannot distinguish convergence from inheritance — asf's convention plausibly seeded vivarium's, and comproprium's FORMAT ports ASF conventions by reference and says so. What it does establish without that confound is the **naming divergence**: the same structural position carries at least three distinct names across four corpora, and six spellings within one of them, which is a fact about the vocabulary, not about who copied whom.

## Working Notes

- The count that would separate convergence from inheritance: an outside-authored claim corpus with a per-atom note position. None is known here; [[turnover-solution]]'s independent-corroboration gap is the same gap seen from another angle.
- The six-spelling spread inside udon-needs is a cheap instrument test — any linter or build that projects the sidecar out of a rendered view has to match all six, and a name-matching implementation would silently pass the ones it missed.
