---
slug: appendix-placement
type: form
---

# Appendices go at the bottom of the outline — even when they are dependencies in the DAG

*Supporting material (specimens, raw observations, worked detail, method) is placed at the end of the exposition even though the claims that cite it depend on it — because a reader's path and a verifier's path through the same corpus run in opposite directions.*

## The claim

An outline serves two journeys at once. The **reader** wants the argument: claims in an order that builds understanding, each one readable without descending into evidence detail. The **verifier** wants the ground: for any claim, the specimen, the counts, the method — everything needed to check it. These pull opposite ways: the ground is *upstream* in the dependency DAG (the claim depends on the specimen, never the reverse), but putting it upstream in the *exposition* buries the argument under its own footnotes.

The resolution is the appendix convention: supporting segments live at the bottom of the outline, and the claims that stand on them cite them by slug. The dependency DAG still records the truth (the claim `depends` on the appendix segment); the outline records the *reading order*; and the two disagreeing here is not a violation to fix but the design working — it is the sharpest recurring instance of [[dependency-order-tension]], resolved the same way every time and therefore worth its own name.

Concretely, in this corpus: chapter-level segments are readable claims (`emp`, `form`) that a front-to-back reader can follow whole; the raw-material segments (`obs`, `survey`) sit in the Appendices part, cited from the claims they ground. A reader who trusts us never descends; a verifier descends exactly once per claim, to a named place.

Because placement is a *view* decision, **the segment itself never announces it**: no "Appendix:" in a title, no "(appendix)" in a citation. Appendix-ness is written only in the outline (asf's form: `## *Appendices* <name>` chapter headings over ordinary content-titled segments), so the same segment can sit main-line in one view and appendix in another without editing it.

Two disciplines keep the convention honest. The appendix segment must actually carry the verification load (the claim's numbers trace to it, not to prose in the claim), and the claim must actually be readable without it (if understanding — not just checking — requires the appendix, the split is wrong and the material belongs in the claim).

## Strength & grounds

**Heuristic, on repeated estate practice.** The convention is live in every mature instance: asf's monograph appendices (demonstration segments kept off the critical path but on the DAG), the neurips papers (appendix segments after the bibliography row, with the manifest injecting the boundary — and the 9-page main text readable without them by venue requirement), and the accepted-ordering-exception machinery that exists precisely because outline order legitimately diverges from dependency order. Same caution as all estate-internal formulations: shared authorship; but this one has an external leg — the appendix is a centuries-old convention of technical writing generally, which is weak evidence it answers something real about expository order versus logical order, not just our habits.

## Working Notes

- Open: whether appendix membership is a *placement* (an outline fact — the same segment could be main-line in another view) or a *kind* (a segment fact). Current lean: placement — a pedagogical view for tool-builders might promote [[lost-update-hazard]] to its main line. That reading makes "appendix" an edge property, consistent with [[view-edge-metadata]].
- The ordering-exception machinery (relation-keyed accepted-violations) is how a linter tolerates this convention without going blind to *unintended* order/dependency inversions; when this corpus gets a lint, appendix rows need that treatment from day one.
