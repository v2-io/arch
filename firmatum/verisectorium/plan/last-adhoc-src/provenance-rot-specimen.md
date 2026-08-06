---
slug: provenance-rot-specimen
type: obs
---

# Appendix: the comproprium provenance-rot specimen

*The full account of the week the comproprium corpus's quotation-provenance broke en masse while its slug references survived — the specimen behind [[path-provenance-rots]].*

## What happened

Comproprium is a young precept corpus whose exempla quote Joseph and agents verbatim. Every quotation segment carries a `:from` field pointing at the primary source file, and the corpus ships a checker (`bin/check-corpus`) whose job is to confirm each quoted span still locates, character-for-character, in its primary — that is the corpus's falsifiability mechanism, and its FORMAT argues explicitly that spans must stay unnormalized *so the checker can find them*.

On 2026-08-01, a routine tidying commit (`a40825f`, "Reorganize various ingest queues") moved the primaries: `.to-integrate/` became `.integrated/` plus two `INGEST/harvest-*` directories. Nobody rewrote the `:from` paths, and nobody taught the checker the new layout. Nothing failed at the time — the move was clean, the commit message mundane.

Four days later (2026-08-05, this project's founding survey) someone ran the checker: **106 of 109 quoted spans could no longer be located**. The corpus's one guarantee had been off for four days, silently, and the instrument that should have caught it was the thing the move had disabled.

The same survey checked the corpus's *other* reference class: the 18 slug references in its outline (`:see #slug` style, no paths). **All 18 still resolved.** Same corpus, same move, two reference designs, opposite outcomes.

## The numbers

| Measure | Value | As of |
|---|---|---|
| Segments in corpus | 57 | 2026-08-05, re-run at drafting |
| Quoted spans locating in a primary | **3 / 109** | same run |
| Span failures | **106** | same run |
| Outline slug references surviving the move | **18 / 18** | first-hand survey, same day |
| Breaking commit | `a40825f`, 2026-08-01 | comproprium git history |

## Method & scope

Re-run `comproprium/bin/check-corpus` from the corpus root; read the breaking commit with `git show a40825f`. One corpus, one reorganization, one checker — a clear specimen, not a rate. It does not show that path provenance is always wrong or that slugs solve every integrity problem; it shows the two designs failing and surviving *the same ordinary event*.

## Working Notes

- Repair options (none chosen yet, deliberately — the choice is precedent-setting for the whole pattern): repoint the paths · teach the checker the layouts · make provenance layout-independent (slug + section + word anchors, the address shape vivarium's orientation quiz already uses).
- Keep the counts dated; a re-run supersedes them and the collision is the point.
