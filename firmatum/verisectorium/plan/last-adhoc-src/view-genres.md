---
slug: view-genres
type: survey
---

# View genres

**Summary.** Across the estate, “outline-like” artifacts are not one thing: pedagogical orderings, assembly manifests, narrative chains, indexes over not-yet-existing segments, nested outlines, and generated projections all appear as views over populations of records.

## Survey (instances observed)

| Genre | What it orders / projects | Specimen (live path or role) |
|---|---|---|
| **Pedagogical / topological outline** | Claim reading order, often with gaps; may track deps | asf component `OUTLINE.md`; vivarium `core/OUTLINE.md` |
| **Assembly manifest** | Which segments enter *this* build; can inject structure | neurips `OUT.*.md` (full vs 9-page); ASF outline as mono ingest input |
| **Digest / filter view** | Subset of segments or sections (e.g. strip Working Notes) | asf `bin/build-markdown` recipes; `--public` monograph variant |
| **Generated projection** | Regenerable surface over records | `LEXICON.md` from terminology; relata emit `.bib`; FINDINGS extracts |
| **Narrative-order outline** | Story flow with connective prose | comproprium `the-chain.md` (connective prose is not the claim store) |
| **View over not-yet-existing segments** | Future work as structure | neurips `REVISION-OUTLINE.md`; OUTLINE `--GAP--` / missing rows |
| **Adjudication / changelog outline** | Post-verification map of claims + residue | neurips `adjudicated/OUTLINE.md` |
| **Trigger / harvest index** | Entry by failure mode or trigger | comproprium `by-trigger.outline.udon`; `GATHERING.md` as harvester brief |
| **Starred / importance annotation** | Subset of outline rows for orientation | vivarium ★ rows + `orient-rank` (see `ORIENT.md`) |
| **Paper-section sequence** | Fixed venue skeleton | behavioral-floor / causal-language `src/NN-*.md` (order in filenames; no separate outline file) |

## Method

- Synthesis from known instances as of 2026-08-05 — **not** a fresh exhaustive walk of every outline-shaped file on disk.
- Genres are **descriptive buckets**; a single file can mix genres (e.g. asf OUTLINE is pedagogical and assembly input).

## Strength and scope

- Supports: **multiple view genres are live**; “outline” is overloaded in ordinary speech.
- Does not support: a closed taxonomy of all possible views; that every genre is equally load-bearing.
- Honest strength: **survey of known instances**, open to new genres.

## Working Notes

- Cross-ref: [[multiple-views]], [[view-edge-metadata]], [[outline-as-organizing-principle]].
- Unintegrated influx (do not cite as warrant): `plan/INFLUX/build/build-issues-survey-2026-08-05.md` (neurips multi-manifest detail); `plan/INFLUX/synthesis/live-state-field-reports-2026-08-05.md` (family-member roster). Specimens above should be checked live if a genre is load-bearing for a decision.
