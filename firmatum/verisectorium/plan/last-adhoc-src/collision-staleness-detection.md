---
slug: collision-staleness-detection
type: form
---

# Present-truth collision is a staleness detector

*Two present-truth statements about the same subject can contradict each other; two append-only history entries cannot — so a corpus that requires present-truth claims gets staleness detection as a structural byproduct, where a pure history layer structurally cannot surface it.*

## The claim

**(A) Structural contrast — definitional.** A statement required to assert what holds *now* about a named subject can **collide** with another such statement: if they assign incompatible content to the same subject, at least one is false as present truth. An append-only entry ("at time $t$, agent $A$ recorded $P$") cannot be forced into contradiction by its form: later entries supersede without making earlier entries false *as history*. A correction buried in a ledger has nowhere it must surface.

**(B) Detection byproduct — the formulation.** In a corpus enforcing present-truth at one-claim-per-atom grain, staleness of a superseded claim becomes *detectable as collision* the moment a later claim occupies the same subject — no separate staleness-audit step required. Integration-is-replacement and "a ladder that only promotes accumulates falsehood" are then design consequences of keeping the collision surface honest, not just author manners.

**(C) The complement, kept attached.** Collision finds what a claim *collides with*; it cannot find what *nothing* claims. Missing structure needs the complementary mechanism — visible absence ( [[absence-as-structure]] ) or derivation-from-core. Two mechanisms, two failure modes, neither substitutes for the other.

**(D) Scope limit.** That collision is *the* primary active ingredient of the pattern's efficacy — rather than one contributing mechanism — is **not claimed**. That stronger reading awaits independent specimens (see the ch. 7 gap row on which properties carry the effect).

## Strength & grounds

(A) and (C) are as strong as the definitions of the two speech-act forms — no empirical content. (B) is **heuristic**: motivated by the structural contrast plus lived specimens (the eleven-day-invisible rename correction sitting complete and correct in a decisions ledger until a segment's formal expression forced present truth and the collision fired; relayed in the 2026-07-23 generalization note Part 7, Fable's report from live vivarium work). This formulation was previously landed at the same tier and with the same scope discipline as `form-present-truth-collision` in the udon theory corpus (2026-07-31 era; copy in `INFLUX/udon-theory/`) — this segment is its restatement in verisectorium's own voice, not an independent second arrival; the agreement is shared authorship, not corroboration. Max honest attainment without outside evidence: robust-qualitative, and only if a differently-authored corpus shows the same asymmetry doing detection work.

## Working Notes

- Strengthen-action: attempt the falsification named in the udon landing — a corpus using only history layers that surfaces supersession equally reliably. If one exists in the wild (event-sourced systems with projection rebuilds are the obvious family), the claim needs re-scoping to "without tooling," which would itself be informative: projections *are* manufactured present-truth surfaces.
- The support-collision twin (two atoms agreeing in prose while their *evidence* disagrees) came out of the relata comparison and belongs in ch. 8, likely near [[verification-provenance]] — content collision and support collision are different detectors with different blind spots.
- TST has no analogue of this mechanism (checked during the 2026-08-05 adjudication of the tst-grounding report); if real, it is an extension the theory would want — a format that manufactures collisions lowers observation noise on staleness. Proposed there, unlanded anywhere.
