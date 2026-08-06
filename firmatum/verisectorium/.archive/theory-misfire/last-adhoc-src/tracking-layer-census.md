---
slug: tracking-layer-census
type: obs
depends: []
---

# The ASF tracking layer, measured

*What one mature corpus's non-claim surfaces actually look like when you count them: how many there are, how long each is, and which carry a declared audience — the specimen grounding [[tracking-altitudes]] and part of [[priming-discipline]].*

## The ladder, with sizes

ASF's own orientation file declares a layered model — *navigator → items → architectural moves*, with history split from all three. Measured on 2026-08-06 against the live tree:

| Surface | Declared role | Lines |
|---|---|---|
| `PRACTICA.md` | navigator: current areas of work with priority markers | **132** |
| `TODO.md` | tactical: items within those areas | **537** |
| `PROPOSALS.md` | portfolio: structural moves cutting across areas | **442** |
| `JOSEPH-TODO.md` | steward queue: pointers only, work stays in home trackers | **27** |
| `CHANGELOG.md` | forward-going history, 2026-04-24 onward | **1,417** |
| `LOG.md` | frozen pre-2026-04-24 archaeology | **221** |
| `audits/STATUS.md` | routing tracker: where each audit stands | **44** |

The shape is the finding. The surface an agent is told to read *first* is the smallest of the working set, and the tactical layer beneath it is four times its size. The steward's own queue is the smallest thing in the corpus — by design, since it holds pointers rather than work. And the history layer is larger than every working tracker combined, which is what a history layer looks like when the replacement discipline is actually running: the working surfaces stay proportional to open work, and everything that has stopped being open accumulates elsewhere.

## Partitions, and the direction of the affix

The root carries nine tracker-class files. Four of them are partitions of the tactical layer by subject, and all four put the qualifier *before* the base name: `BIBLIOGRAPHY-TODO.md`, `FORMAT-TODO.md`, `INTEGRATION-CLEANUP-TODO.md`, `TERMINOLOGY-TODO.md` — alongside a fifth, `TODO-big-picture.md`, which puts it after. The same estate's udon tree partitions the other way throughout (`TODO-META.md`, `TODO-PUBLISHING.md`, `TODO-UTILS.md`). Both directions are live in one estate, and they read differently under a union convention: qualifier-last names a partition of the *qualifier*, qualifier-first names a partition of the *base*. Nothing enforces either.

## Audience declarations

The same orientation file marks surfaces for audience, and the markings are countable: **6** files are marked *auditor-hidden*, **3** surfaces are marked *auditor-safe*, and **4** carry a *priming-heavy* marking (one of them self-declared). The hidden set is exactly the working set above minus the navigator — tactical items, structural proposals, the steward queue, a big-picture correction file, an ideas substrate, and a long-form positioning document. The safe set is the navigator plus the two auditor-facing generated surfaces.

So the altitude ladder and the priming partition are **not the same cut**, and they are close enough to look like one: the navigator is safe and everything below it is hidden, but the *reason* differs — `PROPOSALS.md` is hidden because it is priming, `JOSEPH-TODO.md` because it names decisions in flight. A reader who collapses the two cuts will assume altitude predicts safety, which holds here by coincidence of this corpus's contents rather than by construction.

## Method & scope

`wc -l` on the live tree; `ls` for the tracker inventory; `grep -c` on the orientation file for each audience marker. One corpus, one day, and the line counts are a proxy for size that a heavily-tabular file would distort. The *declared* roles are quoted from the corpus's own orientation file, not inferred from contents — whether each file's contents match its declared role was not checked, and that check is the obvious next measurement.

## Working Notes

- Not measured and worth measuring: time-in-state on the tactical layer, and what fraction of `TODO.md`'s 537 lines are items versus framing. The 2026-07-22 prune commit reports ~103 done-or-decided items deleted across 13 forward trackers for −1,165 net lines, which suggests the working surfaces drift upward between prunes rather than staying level.
- The affix-direction split is a live ambiguity, not a stated convention on either side; it belongs to [[sidecar-conventions]]'s territory.
