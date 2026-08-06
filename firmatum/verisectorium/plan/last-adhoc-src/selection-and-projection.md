---
slug: selection-and-projection
type: form
depends:
  - multiple-views
  - build-forced-commitments
---

# A view does two different things: it selects records and it projects parts

*Choosing which records appear and choosing which parts of a record appear are separate operations with separate consequences, and a corpus that runs them through one mechanism ends up hardcoding the second.*

## The claim

Assembling any view over an atom corpus performs two operations that feel like one.

**Selection** is a filter over the record set: which atoms are members of this view, in what order. It is the operation everyone notices, because it is what an outline or manifest visibly *is* — a list of rows.

**Projection** is a filter over the parts *within* each selected record: does this reading get the Working Notes, the formal apparatus, the epistemic-status paragraph, the plain-language brief? It is the operation everyone implements accidentally, because it usually arrives as a single concrete need — "the public version should not show development notes" — and gets satisfied by one line of code rather than by a declaration.

Keeping them distinct matters for a specific reason: **projection is what makes a record evergreen**. If a view can take only whole records, then serving a second audience means either an atom that is written for everyone at once (and serves no one well) or a second atom that duplicates the first. If a view can project parts, an atom may carry every layer any view might want — the mental-model gloss, the formal statement, the qualification, the development residue — and each view takes the layers its reader needs. The authoring discipline that follows is small but real: keep section names consistent enough for a filter to name them, keep each layer self-contained enough to be read without the layers a view drops, and prefer adding a layer to compressing a new purpose into an existing one.

There is a third thing a view does that belongs to neither: it can tell the assembler *how* to treat a member — start the appendix here, break the page there. That is a directive attached to a membership rather than an operation over content; it lives with the rest of the edge data ([[view-edge-metadata]]).

The practical test for whether a corpus has made the distinction: ask where it is written down that Working Notes do not appear in the published form. If the answer is a flag in a build script, projection is happening but is not declared — which means no second view can vary it, and no reader of the view can see what was dropped.

## Strength & grounds

**A design formulation, well-instantiated on the selection side and thinly on the projection side.** From the 2026-08-05 survey ([[build-forced-commitments]]): ASF's digest builder is the one place both operations are declared in the same artifact — a recipe's frontmatter carries the `filter` (selection, over frontmatter fields and section presence) and its Liquid body chooses which parsed sections to emit (projection), one file, no drift possible between spec and template. Everywhere else in the estate, selection is declared and projection is not: the ASF monograph pipeline strips Working Notes through a `--public` variant flag handled inside the ingest stage, and the proposal to give outlines header-name and status filters *within* segments (`~/src/arch/asf/PROPOSALS.md`, §H item 5) is written down and unimplemented. The neurips manifests take whole segments only.

So the claim that the two operations are distinct is well-supported by one shipping implementation that separates them cleanly; the stronger claim — that declared projection outperforms hardcoded projection — is **not** demonstrated, because the corpora with hardcoded projection have not yet needed to vary it. The argument for it is structural rather than measured, and it is one steward's estate throughout.

One consequence of that proposal is worth flagging as a claim it makes rather than a fact it establishes: that under filterable layers, Working Notes need not be deleted to promote a segment — they can stay and simply be projected out of mature views. That would change what a promotion gate means, and nothing in the estate has tested it.

## Working Notes

- The sharpest open question the survey left: where do the directives and filters live — build script, view row, or a separate recipe? Three shipping answers exist in one estate and none has been compared against the others.
- If projection becomes declarative, the declaration is also a *disclosure*: a reader could be shown what a view dropped. Nothing in the estate does this, and it may be the more valuable half.
- Related and undrafted: [[derived-vs-authored]] (a projection's output is derived by construction), [[semantic-indexing]] (an index is a projection with different needs than a reader's view).
