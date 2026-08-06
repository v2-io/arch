---
slug: navigation-relocation-specimen
type: obs
depends: []
---

# The origin document's prediction, and what the corpus actually grew

*The pattern's earliest surviving argument for one-claim-per-file argued it on navigation grounds and expected the navigation layer to become unnecessary. The corpus that resulted carries a richer exposition layer than the monolith it replaced — relocated, not removed. The specimen grounding [[outline-as-organizing-principle]].*

## What the document says

`~/src/arch/asf/_obs/old-tf-scratch-09-restructure-plan.md` ("TFT Restructure Plan — Session 7") is the estate's earliest surviving argument for splitting a theory corpus into one-result-per-file. It predates the outline layer entirely — there is no `OUTLINE.md` anywhere in the plan.

Its case is made on navigation. Among the problems it lists with the monolithic documents:

> Navigational scaffolding (dependency DAGs, results indexes, notation appendix) was needed BECAUSE the document structure doesn't self-document

and the model it holds up:

> In TST, each file IS a theorem. The document structure IS the navigation. No index needed to find where theorems live.

That is a substantive prediction about what atomization buys: that the scaffolding was compensation for monolithic structure, and would not be needed once each claim had its own file.

The same document is also, quietly, the first counter-evidence to its own prediction. Two sections later, describing what happens to the notation appendix, it reassigns the removed apparatus rather than dropping it:

> README gets: TOC, Epistemic Status Legend, Dependency Chain, Formal Results Index

— and it then writes out a full dependency chain and cross-link list by hand. Under the plan's own scheme, identity is positional (`TF-01` … `TF-11`, with an `Appendix A`), so the dependency chain it draws is a graph over positions.

## What the corpus grew instead

The corpus that resulted did not shed its exposition layer; it developed one considerably richer than a results index, and made it authored rather than derived:

- Each component carries an `OUTLINE.md` — for the AAT core, 401 lines of ordered rows with framing prose over 171 segment files. It is not a membership list: it carries part and chapter structure, per-row summaries, reading-path preambles, and `--GAP--` rows naming what is absent.
- The ordering it declares is checked by a tool (`bin/lint-outline`) against the segments' own `depends:` graph, and deliberate ordering exceptions live in a per-component `OUTLINE-accepted.md` whitelist keyed by slug-pair.
- Indexes did not disappear either — they became *generated*: `FINDINGS.md`, `LEXICON.md` and the public `README.md` are auto-generated surfaces, regenerable from the segments.
- The positional identity of the plan (`TF-01`…) was itself abandoned twice over on the way ([[identity-regime-archaeology]]).

So the pre-atomization prediction was half right in a way that is more informative than being simply wrong. What atomization removed was the *derived* apparatus — the index that answers "where does theorem X live," which a slug-named file now answers by itself, and which the corpus regenerates mechanically where it still wants it. What atomization did **not** remove, and what grew instead, was the *authored* layer: order, framing, importance, and marked absence, none of which is recoverable from the segments.

## Method & scope

Quotations read first-hand from `~/src/arch/asf/_obs/old-tf-scratch-09-restructure-plan.md` on 2026-08-06 (§The Problem, §The TST Pattern, §What happens to TF-00?, §Dependency Chain). Corpus state counted the same day: `01-aat-core/OUTLINE.md` at 401 lines; 171 files in `01-aat-core/src/`; `OUTLINE-accepted.md`, `bin/lint-outline` and the generated-surface conventions verified present (the latter as described in `~/src/arch/asf/doc/sop/agents.sop.md` §File Organization, which marks `README.md`, `FINDINGS.md` and `LEXICON.md` auto-generated).

**Scope.** One corpus, one transition, and a document that is scratch-grade by its own filename — it is a working plan, not a considered position paper, and it should be read as an author's expectation at the time rather than as a claim the estate defended. It shows that this corpus expected the exposition layer to shrink and it did not; it does not establish that atomization always regrows one, and no comparable transition elsewhere was examined. The document's date was not established; what the specimen needs is only that it predates the outline layer, which is established by its content (it proposes the post-split structure and no outline appears in it) and by its filing under the pre-AAT `old-tf-*` scratch series.

## Working Notes

- The sharper unasked question this specimen raises: whether the plan's *"the document structure IS the navigation"* is true at all under slugs. Discovery-by-filename works for a reader who already knows the claim's name; it is exactly the reader who does not — the arriving agent — that the outline exists for ([[turnover-solution]]).
- Registered as A1 in `plan/TODO.md`, which also routes it to [[atom]]; the atom half (why one claim per file at all) is carried there and not here.
- Not examined: whether the other three ASF components' outlines have the same authored richness, and whether the TST component — the one the plan was emulating — ever had an index of its own.
