---
slug: outline-as-organizing-principle
type: form
depends:
  - atom
  - navigation-relocation-specimen
---

# The outline is the organizing principle, not an index

*Once claims are atoms, the corpus needs somewhere to keep everything the atoms cannot hold — order, emphasis, connective argument, and marked absence. That place is the outline, and it is authored content rather than a generated table of contents: losing it loses work no regeneration can recover.*

## The claim

**An index is derived; an outline is authored.** A generated index — a lexicon, a results list, an emitted bibliography — is a projection of what the records already say, regenerable at will, and losing one loses nothing but the time to rebuild it ([[derived-vs-authored]]). An outline holds things that exist nowhere else: the order the argument runs in, the framing prose between rows, the per-row gloss written for this reading path, the importance marks, and the declared gaps. None of that is recoverable from the segments, because none of it is *about* any one segment ([[view-edge-metadata]]).

**Atomization does not remove the exposition layer; it relocates it.** This is the pattern's most reliably surprising consequence, and the estate's own founding argument got it wrong in a useful way. The pre-atomization plan for the first corpus argued the split on navigation grounds — *"the document structure IS the navigation. No index needed to find where theorems live"* — and expected the scaffolding to become unnecessary. What the corpus grew instead was a per-component outline larger and richer than the index it replaced, plus a linter to check it and an exception store to record its deliberate violations; the parts that *did* disappear were exactly the derived ones, which came back as generated files ([[navigation-relocation-specimen]]). The lesson generalizes: slug-named atoms answer *where does X live*, which is the index's question. They do not answer *what is this corpus, what matters in it, and in what order should I meet it* — and that question gets asked by every arriving mind, which under total turnover is every session ([[turnover-solution]]).

**So the outline is the front door.** It is the surface an arriving agent reads first and the one a steward reads instead of the corpus ([[steward-surfaces]]). That imposes obligations an index does not have: it leads with a mental model before precise structure ([[pedagogy-layering]]), it carries enough per-row summary that a reader can decide what to open, and it marks what is missing so absence is legible rather than inferred ([[absence-as-structure]]).

**It carries the glue.** Connective argument — why this chapter follows that one, what the part is for, what the reader should now be able to do — is order-dependent by nature and therefore cannot live in an order-independent atom without falsifying it. The outline's framing prose is where it goes, and a corpus that treats its outline as a table of contents has nowhere to put its own argument, so the argument either disappears or is smeared across segments that then stop being movable.

**Being an organizing principle does not make it the only view.** The outline is a view — the canonical one, but structurally the same kind of thing as any other selection-and-ordering over the atoms, and cheap to have more of ([[multiple-views]], [[selection-and-projection]]). What distinguishes the canonical outline is not its mechanism but its role: it is the one that is *maintained*, that new work is placed into, and that the corpus's instruments check against. A deployment can have many views and still needs exactly one front door.

**What follows, practically.** Placement is an outline decision, so a segment never states where in the outline it sits ([[appendix-placement]]). Reordering costs nothing at the atom layer, so exposition improvements are cheap and should be made freely ([[slug-identity]]). And the outline is authored content, which means it is edited by the same discipline as a segment body — present truth, honest gaps, no accumulation — rather than being regenerated or left to drift.

## Strength & grounds

**Heuristic, with one specimen doing most of the work.** The relocation claim is grounded in [[navigation-relocation-specimen]] — a first-hand reading of the estate's earliest one-claim-per-file argument against the corpus that resulted (quotes and counts re-verified 2026-08-06). It is a single transition in a single corpus, and it establishes that *this* corpus expected the exposition layer to shrink and instead grew one; it does not establish that atomization always regrows one.

The authored/derived distinction is better supported: the estate runs both kinds side by side and treats them differently in practice — generated surfaces carry clobber guards and banners, outlines are hand-edited and lint-checked — and the 2026-08-05 survey of four live pipelines found ordering in a view file in three of them ([[build-forced-commitments]], [[view-genres]]). The strongest independent-of-this-project statement is ASF's own audit convention, *"Outlines are cheap; segments are expensive,"* which reclassifies proposals from segment-rewrites to new outlines and gives the cost reason (`~/src/arch/asf/PROPOSALS.md` §H item 4, read first-hand 2026-08-06) — though that is an argument about view economics, which is [[multiple-views]]' territory, and it is the same estate again.

The claim that the outline must carry the glue is the least tested part: it is argued from what an order-independent atom cannot hold, not from any instance that tried the alternative and reported the result. What would raise the whole: a corpus that atomized and deliberately kept only a generated index, and a report of what its arriving readers actually did.

## Working Notes

- Registered in `plan/TODO.md` as A1 (the origin document; landed here and in [[navigation-relocation-specimen]]) and R24's second half (the pure-graph-without-outline competing formulation, stated in [[slug-identity]] where the identity/order split is argued).
- The seam with ch. 6/12 is deliberate and stated in the outline's own open questions: this segment owns *which content in what order with what view-local meaning*; turning a chosen view into an artifact is a publishing concern.
- Not carried here and genuinely open: whether a corpus can have more than one *maintained* outline without one of them silently becoming stale — the estate has several outlines per corpus but, as far as this pass found, only ever one that new work is placed into.
