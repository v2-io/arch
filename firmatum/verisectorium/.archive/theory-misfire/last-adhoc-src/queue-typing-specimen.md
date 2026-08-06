---
slug: queue-typing-specimen
type: obs
depends: []
---

# Two ingestion queues, side by side

*A typed queue and an untyped one in the same estate, measured on the same day — the specimen grounding [[influx-queues]].*

## The typed queue: relata's ingest membrane

Relata's canonical entry store may not be written by anything except `relata ingest`. External writers — agents, scripts, Joseph — drop files into a spool at `$RELATA_DATA_DIR/ingest/`, and only the ingest verb validates and promotes. Its README (`~/src/arch/firmatum/relata/README.md`, §"The write-membrane") names the reason: an unguarded entry store is the corruption vector where a hallucinated YAML file silently becomes canonical truth.

What makes it a *typed* queue is that a drop which does not get promoted lands in one of three distinguishable states, each with a sidecar explaining itself:

- **`.rejected`** — the *submitter* erred: malformed, schema-invalid, deny-listed, colliding. Fix and re-drop.
- **`.needs-review`** — the submission is fine; the *system* is honestly uncertain. The sidecar carries ranked candidates with their evidence ledgers.
- **`.skipped-nonbib`** — the drop was not bibliographic material at all.

The README states the reason for keeping the first two apart in exactly the terms that matter here: conflating them "would mislabel system-uncertainty as user-error." Two further properties travel with the typing: `relata pending` is a runnable report of everything awaiting a decision, with each item's stage, headline reason, and "the exact command to decide it" — the bullet closes *"Nothing waits invisibly"*; and a sidecar reports relata's own gap by name when relata is the thing that fell short — a DOI it cannot resolve because it has no rung for that registration agency is reported as relata's missing rung, never as a defect in the submitted file.

Live state, 2026-08-05: 300 files in the ingest spool, 15 recorded calibration events, and a `quarantine/` directory holding one dated case (`2026-07-13-false-pdf-registrations`).

## The untyped queue: comproprium's INGEST

Comproprium is a precept corpus in the same estate, of comparable youth. Its intake is a single directory, `INGEST/`, with no disposition vocabulary, no sidecars, and no report verb. Measured 2026-08-05 from the live tree at `~/src/arch/proprium/comproprium/`:

| Measure | Value |
|---|---|
| Segments in the live store (`vera/` + `praxes/` + `exempla/`) | 57 (12 + 10 + 35) |
| Harvest segments sitting unintegrated under `INGEST/harvest-a` + `harvest-b` | **51** (43 + 8) |
| Loose files at `INGEST/` top level | 13 |
| Files in `.integrated/` | **0** (the directory exists and is empty) |

So roughly half the corpus's segment-shaped material is outside the store it belongs to, and the destination that would record metabolism has never received anything. Nothing in the layout distinguishes "not yet looked at" from "looked at and declined" — the two are the same absence.

The queue's contents are also lopsided against the store's own type vocabulary. Counting `:type` declarations:

| Type | Live store | INGEST harvest |
|---|---|---|
| `quotation` | 20 | 49 |
| `probe` | 8 | 0 |
| `principle` | 4 | 0 |
| `practice` | 1 | 0 |
| everything else | 24 | 2 |

Quotations are the cheapest kind to harvest and the least adjudicated; the kinds that require judgment barely appear in the queue at all. One live segment additionally carries `:type pattern`, which is in none of the three vocabularies its FORMAT declares — an undeclared drift that no instrument reported.

## Method & scope

Directory listings and `grep -rhoE ':type [a-z-]+'` over the live comproprium tree, 2026-08-05; `ls` counts under `~/.local/share/relata/`; the relata queue semantics read from its live README. Two instances in one estate, both authored by the same steward and agents — this contrasts two designs, it does not measure a population. Counts are dated and a re-run supersedes them.

## Working Notes

- The comproprium numbers were also reported by an earlier survey this week; the figures above are a first-hand re-run, not an inheritance.
- A second, independent measurement of the same asymmetry would be worth having from a corpus outside this estate — none is available.
- Related but not this observation: of 87 `:from` provenance fields across the 57 live comproprium segments, **79** point into a `.to-integrate/` directory that no longer exists — it is the `INGEST/` tree above. That is the provenance-rot mechanism ([[provenance-rot-specimen]]), observed here again from the queue side: the reorganization that created this queue is the same one that broke the pointers.
