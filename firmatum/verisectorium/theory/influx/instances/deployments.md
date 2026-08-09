# Named verisectoria and cousin deployments

*Updated 2026-08-05. Paths are live sources. Design-doc copies live under `cousin-stores/`, `paper-projects/`, `planned/`.*

**Spectrum (not a single type).** Full claim-verisectoria (ASF OUTLINE+segments) sit at one end; **directory-as-table** stores (terminology, refs, relata) share the population/serving split without a pedagogical outline; **paper section files** use ordered `src/NN-*.md` without epistemic ladders; some projects are **pre-pattern** or only partially migrated.

---

## A. Full claim / precept verisectoria

| Deployment | Atom | Outline / view | Rules | Scale (approx) | Notes copy |
|---|---|---|---|---|---|
| **asf** ×4 components | claim | per-component `OUTLINE.md` | `FORMAT.md` + SOPs | ~170+72+23+22 | `../asf/` |
| **vivarium** `core/` | nomos / claim | `core/OUTLINE.md` | `vivarium/FORMAT.md` | ~115 | `vivarium-format.md` |
| **udon-needs** tooling | demand / finding | `OUTLINE.md` | shape; local type vocab | ~30 | `udon-needs-tooling-OUTLINE.md` |
| **udon theory** | formulation / norm | `*.outline.udon` | theory FORMAT | process segments | `../udon-theory/` |
| **comproprium** | precept / practice / exemplum | `by-trigger.outline.udon` | divergences FORMAT | many `.udon` | `comproprium-format-divergences.md` · `../vera/` |
| **neurips adjudicated** ×3 papers + common | claim / residue / discussion | `adjudicated/OUTLINE.md` | ASF-like epistemology (post-review) | paper 01 ~19 md | `planned/neurips-01-adjudicated-OUTLINE.md` |

---

## B. Cousin directory-as-table stores (same family, not full OUTLINE+segments)

Doc-store report §12.1/lineage; **identity = filename/slug**, **events append-only**, **generated views** (LEXICON, `.bib`).

| Store | Atom | Layout | Generated view | Gate profile | Notes copy |
|---|---|---|---|---|---|
| **asf terminology** | term | `entries/<slug>.md` + `decisions/<slug>/` | `LEXICON.md` via `bin/term` | careful (lint ≠ block render) | `cousin-stores/asf-terminology-README.md` |
| **neurips/refs** | bibliographic entry | `entries/<bibkey>.yml` + `verifications/` | emit `.bib` | careful → critical near submit | `cousin-stores/neurips-refs-README.md` |
| **logos/refs** | same lineage | same layout | emit | **gates** anonymization lint | `cousin-stores/logos-refs-README.md` |
| **relata** | work + epistemic state | data tree external (`$RELATA_DATA_DIR`); code in firmatum | emit / pending / memos | write-membrane + critical paths | `cousin-stores/relata-README.md` |

**Lineage:** neurips/refs → asf/terminology → logos/refs → relata (documented; fourth generation).

---

## C. Paper projects — ordered sections, partial / non-ASF

| Project | Shape | Outline? | Epistemic ladder? | Notes copy |
|---|---|---|---|---|
| **behavioral-floor** (`~/src/behavioral-floor`) | `src/00-meta.md` … `09-conclusion.md`; pandoc concat + relata | **Order in filenames**, not separate OUTLINE | No FORMAT status ladder | `paper-projects/behavioral-floor-CLAUDE.md` |
| **causal-language** (`~/src/causal-language`) | `paper/src/01-…10-…md` same pattern; heavy experiment/spikes tree | Filename order | Strong **process** discipline (`EPISTEMIC.md`) not segment-status | `paper-projects/causal-language-*.md` |
| **embeddings** (`~/src/embeddings`) | experiment scripts + `docs/` notes; TACL submission | **No** OUTLINE+segments observed | Paper-level epistemic framing | `paper-projects/embeddings-README.md` |
| **logos** papers | per-paper `src/` | paper-shaped; not full asf stack | venue scaffolds | (see asf/logos notes elsewhere) |
| **neurips** main papers | `src/re/` segments + adjudicated overlay | yes (paper + adjudicated) | yes on adjudicated | `planned/` |

**Judgment:** BF and causal-language are **paper-section verisectoria lite** (stable section files, ordered assembly, identity partly in `NN-` prefixes — *violates pure slug-without-number* rule). embeddings is **pre-pattern** / experimental notebooks relative to this shape.

---

## D. Planned / in-flight

| Plan | Source | What it wants |
|---|---|---|
| **NeurIPS reviews adjudication package** | `neurips-reviews-responses/processing-flow.md` | Per paper: claim segments + residue + discussion segments + **changelog-style OUTLINE**; ASF-segment epistemology; then backport package to AAT | `planned/neurips-reviews-processing-flow.md` |
| **Status (paper 01, as of flow note)** | same + live tree | Adjudication largely filled under `neurips/.../adjudicated/`; rebuttal strategy still in flux | sample OUTLINEs in `planned/` |

---

## E. Build projections (asf mature)

| Artifact | Role |
|---|---|
| `bin/build-monograph` | OUTLINE + segments → chunks → assembled md → PDF |
| `bin/build-markdown` | digest recipes over segments |
| `bin/lint-outline` | deps / orphans / stage warnings |
| `bin/term` / `relata` / `bin/refs` | cousin-store CLIs |

---

## F. Not a verisectorium (listed so they aren’t rediscovered as “missing”)

| Thing | Why listed out |
|---|---|
| Full asf `src/` corpora | Instance, not notes gather |
| relata data tree (`~/.local/share/relata/`) | Live corpus; code+README only here |
| embeddings experiment suite | Not outline-segment structured |
| `_ref/epistemic_tribunal` product code | Under `../tribunal/` notes |
