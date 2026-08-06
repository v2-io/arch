# Synthesis — ASF level-2 notes and the build-illuminated model

*Gather session 2026-08-05. Focus: notes/plans that illuminate the **verisectorium model** (not pipeline runbooks), especially material written because ASF had to assemble monographs/PDFs.*

Paths below are relative to `verisectorium/notes/`.

---

## 1. Load-bearing “why” documents (build-adjacent)

| Local copy | Live source | What it illuminates |
|---|---|---|
| `asf/proposals-H-outline-as-view.md` | `asf/PROPOSALS.md` §H.4–5 | **Cleanest ASF statement:** outlines cheap / segments expensive; substrate presentation-neutral; outline = **view**; multiple outlines; segments **evergreen**; filter layers; papers = new outlines; don’t delete WN to promote |
| `asf/markdown-first-pipeline-design.md` | `msc/markdown-first-pipeline.md` | Eight architectural commitments: authoring surface unchanged; chunks by slug; **index = assembly manifest**; cross-refs at assembly; header bump container-dependent; PDF one rendering of assembled markdown; chunk contract as machine-readable boundary |
| `asf/build-markdown-design.md` | `msc/build-markdown-design.md` | Digest recipes as data (filter + Liquid projection); digests as views; `--public` = exclude Working Notes |
| `asf/format-segment-conventions.md` | `FORMAT.md` | **Three formats:** Sources / Intermediates (`mono/*.md`) / Finals (PDF); slug = identity; ordering in OUTLINE |
| `asf/format-todo-evergreen-hierarchy.md` | `FORMAT-TODO.md` | Evergreen cross-ref principle; Book/Part/Chapter/Segment/Field/Atom vocabulary |
| `asf/build-latex-plan-early.md` | `_obs/build-latex-plan.md` | Early forks: WN strip, slug→label, type→theorem env |
| `asf/tft-restructure-one-result-per-file.md` | `_obs/old-tf-scratch-09-…` | **Fossil of why segments exist:** one result per file; structure IS navigation |
| `asf/sp5-readers-path.md` | architectural-proposals SP-5 | Extra *layer* inside segment; later reclassed as filterable under §H.5 |
| `asf/theory-content-lifecycle-findings.md` | meta-process 01 | Stage denorm, WN as overloaded queue, promotion ladder aspirational vs landing-driven |
| `asf/format.sop.md` | `doc/sop/format.sop.md` | SOP twin of FORMAT (segment-set, WN gates) |
| `asf/agents-sop-theory-structure-extract.md` | agents.sop extract | Theory structure + file org + OUTLINE preamble pedagogy |

---

## 2. Model insights the build forced open

1. **Identity ≠ order ≠ presentation number** — slug in file; order in outline/index; “Definition 1.4” only after assembly.  
2. **Outline is a view; index is the view materialised** — PROPOSALS §H in English; `index.md` as machine form.  
3. **Segments are evergreen multi-layer records; views project** — WN / Formal Expression / Reader’s Path / status filters.  
4. **PDF is not privileged** — assembled markdown is the citable intermediate.  
5. **Position in a view is not free** — same atom can produce different chunks under different outline placements.  
6. **Outline framing can escape segment discipline** — OUTLINE prefaces outside epistemic gates (CHANGELOG 2026-05-12; not copied).  
7. **Dual-homed `stage` is a known denormalization** — OUTLINE row vs frontmatter (`theory-content-lifecycle-findings`; later edge-vs-record in `udon-analysis/underlying-logical-model.md`).

---

## 3. Reading order (model strand)

1. `asf/proposals-H-outline-as-view.md`  
2. `asf/format-segment-conventions.md` (Audiences section)  
3. `asf/markdown-first-pipeline-design.md` — Why + commitments + Why this matters  
4. `asf/build-markdown-design.md` — filter/recipe  
5. `asf/tft-restructure-one-result-per-file.md`  
6. `asf/format-todo-evergreen-hierarchy.md`  
7. `asf/sp5-readers-path.md` + H.5 reclassification  

---

## 4. Relation to other layers

| Layer | Path | Role |
|---|---|---|
| Cross-corpus generalization | `primary/cross-corpus-generalization-2026-07-23.md` | Across instances; collision; strata clocks |
| **This ASF build-adjacent set** | `asf/*` | Where ASF named presentation-neutrality, multi-outline views, evergreen layers |
| UDON distillations | `udon-theory/form-*.udon` | Claim-grade landings of the same structure |

PROPOSALS §H + markdown-first commitments are the **missing middle** between “we have FORMAT rules” and the 2026-07 generalization.
