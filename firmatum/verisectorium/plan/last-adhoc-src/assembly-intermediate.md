---
slug: assembly-intermediate
type: form
depends:
  - build-forced-commitments
  - selection-and-projection
---

# Render to a durable intermediate before any final form

*A build goes source atoms → assembled, addressable intermediate (normalized markdown) → final renderings; the intermediate, not the PDF, is the citable artifact — and every final form is one rendering of it.*

## The claim

A pipeline that renders atoms directly to a final form (PDF, site, submission package) welds three decisions together: which content, in what assembled shape, in which presentation. Interposing a **durable intermediate** — assembled markdown, plus optionally one normalized chunk per atom with a manifest — pulls them apart, and each separation pays:

- **The intermediate is the citable artifact.** A final form is one rendering among several; new renderers (HTML, EPUB, JSON-for-LLM, a search index) attach at the intermediate without touching authoring or assembly. Citations and reviews aimed at the intermediate survive a change of typesetter.
- **The intermediate is where authoring discipline becomes machine-readable.** Per-atom chunks with a stated metadata contract are the enforceable boundary: what a chunk must contain is checkable; what a PDF contains is not.
- **Incremental build falls out** rather than being bolted on: a manifest recording a source hash per chunk regenerates only what changed — with the honest exception that genuinely position-dependent transforms (header level depends on where a view places the atom) tie a chunk to its placement and regenerate on moves.
- **Projection gets a visible seam.** What each audience receives (Working Notes in or out, formal apparatus or brief) is decided between intermediate and rendering — the place [[selection-and-projection]] says the decision should be *declared* rather than buried in a renderer flag.

The rule in one line: **nothing renders to a final form except from an intermediate someone could read, diff, and cite.**

## Strength & grounds

**Heuristic, from one mature implementation plus convergent partial forms.** ASF's three-stage pipeline is the full instance (ingest → chunks + index → assembled markdown → typeset; all stages live since 2026-05); neurips' manifests assemble to a `.tex` intermediate per manifest; the digest builder emits markdown as its final *and* intermediate form. Specimen detail and quotations: [[build-forced-commitments]]; the chunk-contract and incremental-build particulars are registered as residue A12 pending fuller treatment. Single estate, shared authorship; the external prior (every serious document toolchain from LaTeX's `.aux` era to pandoc's AST interposes intermediates) is weak-but-real convergence that the shape answers something general.

## Working Notes

- Residue A12 carries the chunk-grammar detail (metadata block contract, cross-refs resolved at assembly, header-bump position-dependence) — absorb here or into a ch. 12 companion when the generator (ch. 11) needs the mechanics.
- Open: whether the intermediate should be *committed* (ASF places an assembled snapshot at the root for discoverability; neurips gitignores `.build/` and tracks only extracted artifacts whose names declare derivedness) — this is [[derived-vs-authored]]'s question applied to the intermediate layer, and the estate answers it both ways.
