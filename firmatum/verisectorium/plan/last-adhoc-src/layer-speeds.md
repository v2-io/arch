---
slug: layer-speeds
type: form
depends:
  - first-principles-grounding
---

# Fast strata feed slow canon, and the membrane is the dial

*A corpus with an exploration layer and a canon layer is a stacked system whose stability depends on the two changing at genuinely different rates and on canon not being too sensitive to unsettled output — two conditions that fail separately, in ways practitioners routinely describe as one problem.*

## The claim

**(A) Two failure modes, not one.** The complaint that "canon churns" and the complaint that "changelog voice and diff voice keep leaking into canon" are usually voiced as the same pathology. They are not. The first is a **rate** failure — canon revising at exploration tempo, so the fast layer is forever chasing a moving target. The second is a **sensitivity** failure — canon absorbing transients that had not settled — and it is *directional*, fast-to-slow, not a symmetric mixing of two fluids. The repairs differ: slow the slow layer for the first; tighten what may cross for the second.

**(B) The membrane is where sensitivity is set.** Everything the estate already builds at the boundary — an intake queue that validates before promoting, a ratification step, a required disposition on a working note before it disappears — is a permeability setting. It is a dial, not a wall, and stating it as a dial is what makes it adjustable rather than moral.

**(C) Early promotion is priced, not forbidden.** Integrating output that has not settled is not a violation; it is a purchase, paid in the slow layer's spare capacity to absorb being wrong — which in this register is review capacity. A membrane can charge that price explicitly instead of pretending the transaction did not happen.

**(D) Separation can be bought architecturally.** Rather than hoping the two layers happen to run at different rates, a deployment can *make* them: batch promotion on a fixed cadence (a release tag, a review cycle, a frozen fixture group) so canon ticks once per many exploration ticks. That converts an assumption into a design guarantee, and it is cheap.

**(E) The sharp negative.** Slowing canon helps only against the rate failure. Where the exploration layer has no settled output to offer — genuinely open questions, positions still cycling — a moratorium buys nothing. It is worth knowing which of the two problems a proposed slowdown is aimed at before paying for it.

## Strength & grounds

Held at **heuristic**, deliberately below the tier of the theory it borrows structure from, and the gap is the point. AAT's `#der-multi-timescale-stability` is an `exact` result giving a closed-form separation threshold and two conditions whose separate violation is exactly the (A) split; the two-pathologies-are-one-theorem reading, the pricing in (C), and the fixed-cadence construction in (D) are that segment's own content, restated here for documents. **The theorem's premises assume continuous dynamics and document edits are jumps**, a gap AAT carries on its own open list — so only the qualitative structure transfers, and exporting the threshold formula to document layers would be a false claim. The mapping from theorem objects to document strata is a proposal made in the 2026-07-28 grounding reading, not a result; whether a jump-process version of the theorem yields the same two conditions is genuinely unknown. Citation discipline for this import is [[first-principles-grounding]]; tiers are as-of 2026-08-05 and should be re-checked before load-bearing use.

## Working Notes

- (E) is the checkable one and the cheapest to falsify: find a case where canon was slowed, the exploration layer had nothing settled, and the slowdown helped anyway.
- Not carried here on purpose: any dimensionless group, growth rate, or fluid-dynamical quantity. The apparatus AAT holds has its own parameters; borrowing a second vocabulary on top would be costume.
- What makes strata separable at all is disjoint placement — each layer needs its own write clock, which is why this and [[partition-isolation]] are two readings of one design decision.
- Unintegrated influx behind this segment (do not cite as warrant): `plan/INFLUX/udon-analysis/tst-grounding.md` §5 and `plan/INFLUX/udon-analysis/underlying-logical-model.md` §3. The live authorities are the ASF segments themselves under `~/src/arch/asf/`.
