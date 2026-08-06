---
slug: ladder-never-fired
type: obs
---

# The ladder that never finished

*The full count of stage occupancy across vivarium and ASF on 2026-08-05 — the specimen grounding [[state-flags-not-gates]].*

## What happened

Nearly every mature claim corpus in the estate documents a **promotion ladder**: segments are supposed to climb through named stages (draft → deps-verified → claims-verified → format-clean → candidate, with Gate 4 requiring Working Notes empty at the top). The ladder is real prose — FORMAT, SOPs, audit instructions. Agents and humans talk about “promoting” and “gates.”

On 2026-08-05, as part of checking whether that process was *lived*, the stage field was counted on disk rather than assumed from the documentation.

**Vivarium** is the most actively worked claim surface in the sample: 115 files under `core/src/`, all touched in recent weeks, FORMATs and gates fully articulated. Every one of them carries `stage: draft`. Not mostly draft — **all 115**. Nothing has left the first flag.

**ASF** (all four components’ live `src/*.md`, excluding `old-*` archaeology) is the corpus with the richest ladder vocabulary in use:

| `stage:` value | Count (2026-08-05) |
|---|---|
| draft | 200 |
| deps-verified | 23 |
| claims-verified | 18 |
| exploratory | 2 |
| format-clean | **0** |
| candidate | **0** |

So ASF *does* use mid-ladder flags. What it does **not** do is finish the documented trajectory: the terminus stages that Gate 4 was written for never appear. There is no population of segments sitting at “ready for external challenge with Working Notes gone.”

The practical consequence is visible without a second instrument: work that *feels* most advanced still carries heavy Working Notes, because nothing ever gets the “empty the notes” gate as a real destination. Landing and refining continue; the ladder’s top rungs do not.

## The numbers

| Measure | Value | As of |
|---|---|---|
| vivarium segments | 115 | 2026-08-05 |
| vivarium `stage: draft` | **115 / 115** | same |
| asf live segments with a `stage:` line | 243 | same (sum of table) |
| asf at `format-clean` or `candidate` | **0** | same |
| asf at mid-ladder (deps- or claims-verified) | 41 | same |

## Method & scope

Shell count of `^stage:` lines under `vivarium/core/src/` and `asf/{01,02,03,04}-*/src/*.md` (excluding `old-*`), then `uniq -c`. One calendar day, two trees. This is **stage-field occupancy**, not an archaeology of every audit that ever ran Gate 1 by hand without updating frontmatter.

It shows: the full designed ladder is not how completion is recorded today; flags that *do* get written are mid-course, not terminal. It does **not** show that nobody ever checks dependencies or claims — only that the documented end-state is unused as a stage value on these trees.

## Working Notes

- Keep the table dated; a re-run that finds `candidate` segments collides productively with this obs.
- Related undrafted: instruments that ignore `old-*` (taught blindness) live near [[corpus-instruments]], not here.
- Unintegrated influx (do not cite as warrant): live-state field-report headline that first noticed vivarium’s 100% draft; the warrant is the grep above.
