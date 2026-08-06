---
slug: orientation-scaffold
type: form
depends:
  - orientation-gate
  - priming-discipline
---

# Orientation is a scaffold with declared slots, and it feeds the corpus rather than only guarding it

*The general part of orientation — a front door, a declared read order, an importance-ranked free-read set, a proof of contact, and a disposition — is portable; what changes per instance is which SOPs drop into the slots.*

## The claim

Total turnover means every session begins un-oriented, and a corpus that treats orientation as "read the docs" gets whatever reading the agent's own priors produce. What generalizes is not a reading list but a **scaffold with named slots**:

- **A front door that is the single entry point**, from which every other read is reachable — so that being oriented is a definite state rather than a feeling.
- **A declared read order with a priming split.** Some material orients and some material carries verdicts, and they are read in that order for reasons that survive good intentions ([[priming-discipline]]).
- **A bounded high-importance set** rather than the whole corpus. Free-reading everything does not scale past a few dozen records, so the scaffold needs an importance annotation to draw from — which is a view-local ranking, computed ([[hotness-methodology]]).
- **A proof of contact**, cheap and checkable, where stakes warrant it ([[orientation-gate]]).
- **A disposition layer** — how work is done here, not what is true here — which is the slot instance-specific SOPs actually fill.

Three properties are what make the scaffold worth building rather than describing.

**The importance annotation and the onboarding curriculum are the same data.** The set a new mind is asked to free-read *is* the set the corpus considers currently load-bearing. When both are drawn from one ranking, a change to what the corpus considers important silently changes what every incoming agent learns — which is the desired coupling, but it is a coupling, and an instance that maintains its curriculum by hand alongside a ranking has two things that will drift.

**Proof-of-contact addressing is provenance addressing run forwards.** An orientation check asks *quote the rest of this unit, addressed by record + section + opening words*; a provenance checker asks *is this quoted span still located in its source*. Both need exactly one identity-anchored addressing scheme, and neither works with paths or line numbers ([[identities-over-locations]]). An instance that builds either gets the other nearly free — which is the strongest argument for building the addressing before either consumer exists.

**Grading should be graduated, and the misses should convert into work.** Binary pass/fail wastes the most informative outcome. A near-pass that grants limited access against a promise to read the sections that were missed converts the failure into targeted reading; and friction encountered during the free-read — stale records, ambiguous wording, a unit that was unquotable because it was mid-clause — converts into a standing repair backlog homed on a named record, so the *first legitimate uses* of newly granted access are the repairs the orientation made obvious. This is what turns orientation from a tax into an intake: the scaffold feeds the corpus instead of only guarding it.

**The costs are real and belong in the design, not in a footnote.** A single sealed check per working tree means parallel agents on one tree clobber each other ([[partition-isolation]]'s hazard in the orientation namespace). "Answerable from memory" pressure shapes which units are quotable at all, and therefore shapes the corpus in the direction of the check — a small, real feedback from instrument to content. And a bypass has to exist, owned by a person, or the emergency case teaches agents to route around the whole thing. The mechanism detail is worth carrying because it explains both costs at once: the seal is a file-permission change over the record directory, which is why exactly one check can be open per working tree and why an emergency unseal has to ship alongside it as a named command. A gate implemented in the filesystem inherits the filesystem's granularity, and that granularity — one tree, one lock — is what makes it hostile to parallel agents.

## Strength & grounds

**Heuristic, from one live implementation plus one soft one.** The graduated grading, the misses-into-reading conversion, the repairs-first norm, the single-sealed-test cost, and the bypass rule are all stated in `~/src/arch/vivarium/ORIENT.md` (read first-hand 2026-08-06), whose grades run 5/5 unlimited-for-this-compaction-generation, 3–4/5 one-time-plus-promise, below 3 re-orient — and which distinguishes soft rejects that do not burn an attempt from real graded failures that do. The stars-are-the-curriculum coupling is that file's own statement that most quiz mass is drawn from the outline's starred rows, which `bin/orient-rank --mark-outline` maintains. The declared-read-order and priming-split slots are ASF's `doc/sop/audit.sop/de-novo.sop.md` §4.1, which is the same scaffold enforced by norm rather than by file permissions.

One instance has actually built the gate; the generalization to a slotted scaffold is this segment's, made from two instances and the shape they share. **No measurement exists** that oriented sessions produce better work than un-oriented ones — the whole family rests on the recognized failure in [[outline-skipping-failure]], which establishes that skipping happens, not that gating repairs it.

## Working Notes

- Discharges TODO entries N20, N21, N22, and N11's ch. 5 half (the curriculum/ranking coupling; N11's ch. 2 half — that the ranking is simultaneously view-local metadata — is owed to the hotness gap row, not carried here).
- The role-differentiated half of onboarding — that a de-novo auditor, a harvester, an integrator, and a reviewer need materially different reads and different priming rules — is the standing ch. 5 gap and is deliberately not attempted here. Two of its inputs are already registered (R41's rescinded-candidates deliverable for auditors, R45's budget-gate-as-permission) and a third is R55's harvester-brief genre.
- Untested and cheap: whether the disposition slot can be inherited by reference rather than restated per instance. The estate has a live rule that restating a definition forks it; disposition text is exactly the material that gets restated most.
- The instrument-shapes-the-content effect above is worth watching rather than fixing. It is the only observed case in this corpus of a *check* changing what gets written, and it is not obviously bad.
