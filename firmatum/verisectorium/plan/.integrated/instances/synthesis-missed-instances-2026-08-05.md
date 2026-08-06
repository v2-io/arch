# Synthesis — missed instances and planned verisectoria

*Gather extension 2026-08-05 after Joseph named terminology, relata, behavioral-floor, causal-language, embeddings, and neurips-reviews processing-flow.*

---

## What “instance” means on a spectrum

| Tier | Criteria | Examples |
|------|----------|----------|
| **Full verisectorium** | slug identity; separate outline view; typed claims + epistemic status; optional lint/build | asf, vivarium, udon-needs, comproprium, neurips `adjudicated/` |
| **Cousin store** | dir-as-table; slug=file; append-only events; generated view; **no** pedagogical outline | terminology, neurips/refs, logos/refs, relata |
| **Paper-section lite** | one file per section; order in **filename numbers**; build concat; little/no status ladder | behavioral-floor, causal-language paper/src |
| **Pre-pattern** | docs/experiments without segment+outline discipline | embeddings (as surveyed) |

Cousin stores were already treated as siblings in the doc-store report; they belong in the verisectorium **family** even when they are not OUTLINE+segments.

---

## ASF terminology

- **Why it is an instance:** multi-agent-safe term records; `entries/<slug>.md`; `decisions/<slug>/` append-only; LEXICON is a **generated view** (clobber-guarded render). Explicit sqlite-vs-files trade study.  
- **vs claim segments:** atom is a *term*, not a *claim*; “outline” is tag/seq ordering in render, not a chapter OUTLINE.  
- **Copy:** `cousin-stores/asf-terminology-README.md` (+ sample entry `exact`).

## Relata

- **Why it is an instance (and more):** epistemic-state-primary bibliography; designator resolution ladder; write-membrane; verification/calibration event stores; emit as view. Data externalized from code repo.  
- **Lineage:** fourth generation of the refs/terminology pattern.  
- **Copy:** `cousin-stores/relata-README.md` (concepts; TODO-ingest remains live design-of-record at source — not fully copied, ~280K).

## neurips/refs and logos/refs

- Ancestors/siblings of terminology and relata.  
- logos **gates** lint (anonymization) — enforcement-profile specimen.  
- **Copies:** both READMEs under `cousin-stores/`.

## behavioral-floor

- AIES paper: `src/NN-section.md` ordered by number; pandoc + relata build.  
- **Not** full ASF: numbering *in* filenames; no type/status/stage ladder; no separate OUTLINE table.  
- Still a deliberate modular paper substrate (identity of *section*, not claim).  
- **Copy:** `paper-projects/behavioral-floor-CLAUDE.md`.

## causal-language

- Companion empirical paper; same `paper/src/NN-*.md` pattern + large experiment/spike tree.  
- Epistemic discipline lives in `EPISTEMIC.md` (project-level honesty), not per-segment status.  
- **Copies:** README + EPISTEMIC.

## embeddings

- TACL epistemic-hedging geometry work; experiment scripts + `docs/` notes.  
- **No OUTLINE+segments found** in a shallow survey — predates or sidesteps that discipline for the paper shape. Kept as a **negative instance** so the gather doesn’t re-ask forever.  
- **Copy:** README only.

## Planned: neurips-reviews-responses processing-flow

- Explicit plan: per paper, **adjudicated claim segments + residue + discussion segments + outline/changelog**, ASF-like diligence, then package for AAT backport; then rebuttal strategies.  
- **Already partially live** under `~/src/neurips/<paper>/adjudicated/` (OUTLINE + claims/ + discussion/), not only aspirational.  
- **Copies:** processing-flow.md; sample OUTLINEs for paper 01 and adjudicated-common.

---

## Design lessons (portable)

1. **Cousin stores prove the spine without pedagogy** — identity/events/generated views can exist without chapter OUTLINEs.  
2. **Paper-section lite reintroduces order-in-filename** — cheaper for fixed venue skeletons; loses free reordering without renames.  
3. **Enforcement profile travels** — logos gates, terminology render never blocks: same family, different stakes.  
4. **Adjudication-as-verisectorium** is the intended pattern for review-response science packages — processing-flow is the brief; `adjudicated/` is the proof of concept.
