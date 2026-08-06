---
slug: working-notes-deluge
type: obs
depends: []
---

# The working-notes deluge

*What a per-atom sidecar with no drain looks like after two years, measured on the ASF corpus — the specimen grounding [[working-note-lifecycle]].*

## What happened

ASF's segment format has carried a `## Working Notes` section from early on, with an explicit and well-written discipline: a note earns its place *only* if it assists future work, in one of three kinds — a **forward pointer** (an open follow-on or unresolved question), a **regression guard** (a disconfirmed prediction or a deliberately-corrected form, recorded so it is not re-attempted), or a **dead-end warning**. Past-work narration is named and excluded: *"vanity-changelog — pure past-work narration ('previously carried X,' 'the audit recommended a soften'); that is `CHANGELOG.md`'s job."* The rule sits in the live format SOP and other documents point to it as authoritative.

The format also specifies the drain. Working Notes are process artifacts, not canon, and the promotion ladder's last gate is a **notes disposition**: at the final stage every note must be explicitly resolved, deferred, or promoted, and the section emptied — *"A segment with unresolved Working Notes is not a candidate."*

That gate has never fired. As of 2026-08-05 the corpus has **0 segments** at either of the two terminal stages the gate leads to (the fuller stage-occupancy picture is [[ladder-never-fired]]). Everything the discipline admits accumulates; nothing the discipline defines as the exit has ever been taken.

A second process poured into the same container. During 2026-05, a sweep harvested "incidental gold" — worthwhile observations made in passing during de-novo audits — and lifted it per-segment into Working Notes under an `### Incidental audit gold` heading. The sweep covered one of four components and stopped in motion at the end of May; the onward leg that was supposed to promote that material into the segments' own prose never ran. What was designed as a transit lounge became the destination.

## The numbers

Counted first-hand on 2026-08-05 against the live tree (`~/src/arch/asf`), excluding `old-*` archaeology:

| Measure | Value |
|---|---|
| Live segments (`0*/src/*.md`, minus `old-*`) | 243 |
| Segments carrying a `## Working Notes` section | **232** (95.5%) |
| Total words in Working Notes sections | **175,122** |
| Words in segment bodies before Working Notes | 465,953 |
| Working Notes share of total corpus word volume | **27.3%** |
| Segments carrying an `### Incidental audit gold` block | **122** — all in one component (`01-aat-core`) |
| Words inside those gold blocks | **101,440** — **57.9%** of all Working-Notes words |
| Segments at a terminal stage (`format-clean` or `candidate`) | **0** |

A single well-developed segment shows the shape at record grain: `01-aat-core/src/def-chronica.md` is 106 lines, of which **67 (63%) are Working Notes** — and it is among the corpus's more advanced segments, not one of its neglected ones.

An earlier internal review of the same corpus on 2026-07-07 reported 235 segments, the same 122 gold blocks, and a comparable gold share of working-notes volume, so the state has been stable rather than momentary; the count above supersedes it and is not independent of it.

## What the specimen does and does not show

It shows that a per-atom sidecar with an admission rule but no *executed* drain grows to roughly a quarter of a corpus's prose, and that more than half of that volume can arrive from a single unfinished transit process rather than from ordinary authoring. It shows the drain being *designed and documented* is not the same as the drain running.

It does not show that the material is worthless — the gold blocks were harvested precisely because they were judged valuable, and some of it is load-bearing theory-correctness content. Nor does it show that the admission rule failed: the rule is unenforced by any linter, so the corpus is evidence about a documented-and-unenforced discipline, not about a discipline that was tried and beaten. And it is one corpus, one steward, one day.

## Method & scope

Word and file counts by script over `~/src/arch/asf/0*/src/*.md` excluding paths containing `/old-`: a segment's Working Notes are taken as everything from the `## Working Notes` heading to end of file, and a gold block as everything from `### Incidental audit gold` to the next `###` heading. Stage occupancy from `^stage:` frontmatter lines. The format rules and the notes-disposition gate were read in `~/src/arch/asf/doc/sop/format.sop.md` (to which `FORMAT.md` is a symlink) on the same day; the quoted phrases locate there character-for-character.

## Working Notes

- Keep the counts dated. A re-run that finds the volume falling is the interesting collision.
- The end-of-file convention for measuring notes volume is generous where a segment places another section after Working Notes; spot-checking suggested this is rare, but the number is an upper bound.
- Not measured here and worth measuring: how much of the 175K words is *forward-pointing* (still doing the job the rule describes) versus retrospective. That ratio, not the total, is what would say whether the deluge is debt or inventory.
