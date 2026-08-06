---
slug: multiple-views
type: form
depends:
  - build-forced-commitments
---

# Many views, one set of atoms

*A view is cheap and a claim atom is expensive, so when a corpus needs to say the same things to a new audience the honest first move is a new view — not an edit to the atoms.*

## The claim

Once ordering and membership live in a view rather than in the records themselves, a second view costs about one new file plus its framing prose. Editing a record costs much more: every other view that includes it inherits the change, everything that depends on it may need re-checking, and the edit is irreversible in the sense that the earlier wording is gone from the present-truth layer. The asymmetry is large — roughly a file against a re-verification cascade — and it is stable, because it follows from where identity and order live rather than from any particular toolchain.

Two consequences follow, and both are practical rather than philosophical.

**Before scoping work as a rewrite, ask whether it is a view.** A great many requests that arrive shaped like "restructure the corpus for X" are really "assemble a reading path for X." A pedagogical entry path, an audience-specific cut, a shorter version for a venue, a thematic tour — each of these can be a selection over existing atoms with new connective prose, leaving every atom untouched and every other view intact.

**When a view will not fit, omit rather than fork.** The failure mode that destroys the whole arrangement is editing a record so it fits one particular view, or copying it into a variant. Either move ends the property that made views cheap: after it, the same claim exists in two places at two truths, and the corpus has silently acquired a collision it cannot see. Omission keeps one truth per claim and lets the view be honestly incomplete.

The economics also explain why multiple views tend to arrive at *different rigor levels* rather than as mere reorderings. Since a view chooses membership, the cheapest way to serve a reader who needs less apparatus is to include fewer, differently-framed atoms — not to soften the atoms.

## Strength & grounds

**A design formulation with converging live practice, not a measured result.** Four pipelines in one estate were surveyed on 2026-08-05 ([[build-forced-commitments]]); three put ordering in a view file and two run more than one view over one substrate today. The sharpest independent statement comes from the neurips workspace under venue pressure — a hard nine-page budget produced the rule that trimming is manifest selection rather than segment editing, with the stated reason that a tightened proof then propagates to every manifest at no cost. ASF reached the same formulation from the opposite direction, as an audit convention: *"Outlines are cheap; segments are expensive"* (`~/src/arch/asf/PROPOSALS.md`, §H item 4), with a list of proposals reclassified from segment-rewrites to new outlines.

The honest caution is shared authorship: these instances were built by the same steward and closely related agents, so their agreement is better read as one practitioner group meeting the same pressure repeatedly than as independent corroboration. The cost asymmetry itself has not been measured — no one has timed a view against the rewrite it replaced. What is genuinely observed is the *choice*: given both options, this estate has repeatedly chosen the view, and the paper projects that never needed a second view also never built the manifest layer.

## Working Notes

- The unmeasured quantity worth measuring: for one real case, the actual cost of a new view versus the segment rewrite it displaced. A retrospective on the §H item-4 reclassifications would supply it cheaply.
- Where a view's order must respect atom dependencies, the tension is [[dependency-order-tension]]; what a view stores about its own membership is [[view-edge-metadata]]; the two operations a view performs are [[selection-and-projection]].
- Open: whether "omit rather than fork" needs a recorded absence (this view deliberately excludes atom X) so that omission is distinguishable from oversight. [[absence-as-structure]] is where that would land.
