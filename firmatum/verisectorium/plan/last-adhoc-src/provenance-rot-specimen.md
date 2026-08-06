---
slug: provenance-rot-specimen
type: obs
---

# Provenance rot specimen

**Summary.** After a directory reorg on comproprium, outline slug references stayed intact while path-based quote provenance failed almost completely — a natural experiment for identity-without-path.

## Observation

On 2026-08-05, `comproprium/bin/check-corpus` reported (re-run same day by this drafting session):

- **57** segments in the corpus
- **3 / 109** quoted spans located in a primary
- **106** fails (span not found)
- **5** forward references (expected by FORMAT, naming segments that should exist)

The live-state field report (2026-08-05, first-hand ✔) attributes the mass failure to commit `a40825f` (“Reorganize various ingest queues”), which moved material from `.to-integrate/` into `.integrated/` and `INGEST/harvest-a|b/` **without** rewriting `:from` paths or teaching the checker new layouts.

The same report records that **all 18** outline `#slug` references **survived** that move (slug identity, not path).

Re-run confirmation: the 3/109 · 106-fail totals still hold at drafting time; the mechanism (path-tied provenance vs slug identity) is what the specimen is about, not a one-day flake.

## Method

- Tool: `bin/check-corpus` under `proprium/comproprium/`
- Contrast: outline cross-refs by slug vs quote spans keyed to primary paths
- Cause: git history / field report on the reorg; not re-derived line-by-line here

## Strength and scope

- **What this supports:** path-as-provenance **can** fail silently under ordinary directory moves while slug identity does not — observed on one corpus, one reorg, one checker.
- **What it does not support:** that all path provenance is always bad; that slug-only addressing solves every integrity problem; estate-wide rates outside comproprium.
- Honest strength: **one clear specimen**, useful as design pressure (prefer layout-independent anchors: slug + section + span), not a universal law.

## Working Notes

- Repair options named in field report (not chosen here): repoint paths · teach checker layouts · make provenance layout-independent (slug/anchor-shaped, as orientation quiz already does).
- Forward: when drafting [[slug-identity]], this obs is the primary specimen; keep the 3/109 numbers dated so re-runs can supersede them.
