---
slug: tribunal-record
type: form
depends:
  - tribunal-strand-survey
  - rationale-capture-survey
  - decision-records
---

# Typed voices, and the record as the product

*Structured disagreement is worth having because the voices fail in different directions. It is worth **keeping** because the deliberation — not the verdict — is what a later finding needs to attack.*

## The claim

A tribunal, in the sense this pattern uses, is a deliberation run by voices with **deliberately different failure modes**:

- an **advocate**, whose failure mode is motivated construction;
- a **red-team**, whose failure mode is motivated destruction;
- a **neutral observer**, watching for the biases the first two *share* — the single-author caution given a seat;
- a **risk analyzer**, naming failure shapes orthogonal to the pro/con axis entirely;
- an **adjudicator**, which receives all of them and produces the record.

The role-set is not arbitrary and not merely "more perspectives." Agreement between two channels is worth something only if the channels can fail independently — so this is that requirement institutionalized as an org chart, and the adjudicator can weigh these inputs *precisely because* it knows how each is biased. Two of the five are the ones the wider practice consistently omits — the observer and the risk analyzer — and they are exactly the two that a lone author cannot supply for themselves, since the biases they exist to catch are the author's own.

**The second half is the one that is actually scarce.** Multi-agent debate is commonplace; what remains rare is treating the deliberation itself as the durable artifact. The debate and LLM-as-judge lineage optimizes *answer accuracy* and discards the argument once the answer lands — and so do most working harnesses, which run degenerate tribunals (extract, refute, vote, report) with no observer, no risk node, no revisit criteria, and no surviving deliberation. The verdict is a summary; the record is the asset. What the record buys is stated in [[decision-records]]: a decision that knows its legs absorbs a refutation locally instead of defending itself totally.

There is a structural reason the two halves belong in one segment. The typed roles are what make the typed record *affordable*: the deliberation emits the structure as exhaust, so nobody has to annotate anything afterward. That is the only proposed escape from the capture problem that has killed better-designed systems than this one ([[rationale-capture-survey]]) — and it is a lean, not a result.

## Register-collapse is the failure mode

Everything the record is for depends on the voices staying typed rather than blending. A con folded into decision prose loses its provenance and stops being attackable. A confidence stated in the decision's own voice *becomes* the decision. This is the same discipline that keeps status banners, pre-validation marks, and history layers separate elsewhere in the pattern — blended registers rot — except generated structurally, one element per voice, instead of maintained by hand.

## The shape scales down

The full council is expensive, and the same job runs at much smaller grain: subjecting an interpretive claim to typed challenge before anyone inherits it costs one reviewer at the paragraph level and a council at the governance level. The paragraph-level form is in force in ASF today and has its own segment — [[discussion-probes]]. Its existence is the cheap evidence that the role-typing idea is not tied to the expensive apparatus. Do not confuse the rungs when citing: the forms share a name and a motive, not a design ([[tribunal-strand-survey]] keeps the four apart).

## Strength & grounds

**Formulation, from a design in use plus estate practice; the durability half is argued, the capture-escape is unproven, and none of it has outside-authorship corroboration.** The role-set and the record-as-product framing are Joseph's, from a design he built and later glossed. The independent-failure-modes rationale is analytic. That the two extra voices are missing from the surveyed prior art is inherited at one remove and scoped to what was walked ([[rationale-capture-survey]]).

What is *not* established: that a full five-voice council outperforms cheaper structures, that the record is worth its cost at scale, or that process-generated capture escapes the capture problem. Two supporting legs exist for that last one and both are this estate, this month.

## Working Notes

- The estate's council already writes record embryos unprompted; the delta from the fuller shape is close to just the adjudicator's distinctive columns ([[decision-records]]).
- The historical strands share the name and diverge on product and subject; a unification would be an argument to make, not a merge to perform. See [[tribunal-strand-survey]].
- Unhomed here: the *internalized* form — the same four voices run as one mind's self-interrogation before output, at graded depths. It is real practice, not this project's material, and it is described in [[tribunal-strand-survey]] strand B.
- Do not cite influx copies under `plan/INFLUX/tribunal/` as warrant. Live sources: `~/src/arch/firmatum/udon/v2/theory/to-integrate/refine-more/epistemic-tribunal-revisited.md` (roles, record-as-product), `~/.claude/memory/epistemic-discipline/gate2-probes-discussion.md` (the probes and the worked example), `~/src/_ref/epistemic_tribunal/` (the built system).
