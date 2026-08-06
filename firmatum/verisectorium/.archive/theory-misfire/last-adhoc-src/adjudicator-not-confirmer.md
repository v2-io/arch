---
slug: adjudicator-not-confirmer
type: form
depends:
  - routing-sop-anatomy
  - observable-crossings
---

# The one who judged is not the one who confirms, and the check sits at the seam

*Independence at a crossing has to be structural rather than diligent, because the defects it catches are exactly the ones that feel clear from inside — and it has to be placed at the boundary between what can still be undone by not committing and what cannot.*

## The claim

**(A) The confirming actor is a different actor.** Before material crosses into a place where it will be treated as settled — promoted, filed as integrated, cited as verified — the claims that *gate* the crossing are re-checked against their primary sources by someone other than the agent who adjudicated them. Two properties do the work, and both are load-bearing:

- *A different actor.* The agent that just made every judgment is the worst-placed to see what it got wrong, and no amount of care converts it into a second reader. This is not a competence claim; it is a claim about what is available to introspection.
- *The primary source, not the summary.* The confirmer opens the file the claim is about. Confirming against the adjudicator's report verifies the report ([[proxy-discipline]]); the whole gain is one independent path to the same fact, and reading the summary collapses it back to one path.

The estate's version is stated as a gate in the state machine rather than as branch isolation — i.e. it is a required step in the process, not a property of where the work happens to sit — and it records catching a self-certified "fully clean" sweep that was not, a stale disposition, and a tracker-identity confusion. Its own summary of why it survives cost-cutting is the sentence worth keeping: **it is load-bearing precisely because the conviction that you do not need it is the failure it catches.**

**(B) The checkpoint belongs at the seam, not at a convenient interval.** Acts before a crossing divide cleanly into two kinds. *Pre-decisional* acts — reading, spot-checking, designing the ledger, drafting — lock nothing in, because not committing undoes them. *Durable* acts — writing the manifest, writing the consolidated record, moving the files — do. The external check goes **between** those two sets. Placed earlier it interrupts work that was free to be wrong; placed later it is reviewing a decision rather than gating one, and the cost of acting on its finding has changed character.

**(C) For one class of defect, vigilance is not a defense and only an external eye is.** Wording failures — a softened ghost, a claim that overreaches by a clause, a disposition line that says something subtly untrue — are invisible to their author in a specific way: *"I see it clearly"* is exactly the state that produces them. The repair is not more attention. It is **routing the specific drafted artifact, verbatim, to an external eye before the durable write.** Verbatim is the operative word; a paraphrase of the wording cannot carry the defect that lives in the wording.

**(D) A freshly written correction is the highest-risk artifact in the room, and should be checked before anything is built on it.** This is the least intuitive item here and the estate has it as a recorded incident. A correction to an over-rotated rule was externally reviewed *because* its authors reasoned that a fresh correction to an over-rotation is itself the most likely thing to over-rotate — and the review found a genuine loosening in it, plus a missing closure step. The transferable form: **the conviction that a correction is clean is the same conviction the thing being corrected says fails.** Check the new filter externally before you build a batch on it.

**(E) The same seam locks attribution, which is a second payoff for one act.** Committing before launching an agent that will modify shared material isolates that agent's contribution as a discrete, reviewable, revertible diff. Without it, reviewing what the agent actually did requires line-by-line reconstruction of whose edit was whose, and reverting becomes per-line judgment instead of one operation. The commit *is* the seam — it separates two attributable things — which is why the boundary between reversible and durable is worth marking even where no verification is planned.

**(F) The independence generalizes past crossings to any batch operation over shared material.** A whole-corpus mechanical change wants a different agent verifying each batch, running in parallel with work on untouched material so the check is not a bottleneck. A project-wide convention wants a small deliberately-diverse pilot, validated by reading, before the tool is built and swept — the pilot's job is to expose the gaps the specification did not know it had. The cost of skipping the thorough version of that check is recorded: a lightweight audit miscounted the instances of a schema (claimed nine, actual one) and missed a mechanical bug that would have silently corrupted 142 renames.

## What this buys, stated honestly

Independence does not make judgments good. It makes them **inspectable at one specific moment** — the moment before the cost of being wrong changes. A wrong adjudication that survives an independent confirmation is still wrong; it has merely been given two chances to be caught instead of one, on two paths that fail differently. That is the entire mechanism, and the reason it is worth its cost is that the paths *do* fail differently: the adjudicator's failure mode is conviction, and the confirmer's is inattention, and inattention against a named claim and an open primary source is a much weaker adversary than conviction is.

The corresponding limit follows immediately and is worth saying because the practice invites forgetting it: **two agents of the same lineage, given the same summary, are not two channels** ([[gates-need-destinations]] (E)). The independence is bought by the different *object* — the primary source — at least as much as by the different actor.

## Strength & grounds

**Read first-hand from one estate's process law, where it is stated as an authoritative gate and carries named catches; the catches are the corpus's own report of its history and were not independently reconstructed.**

Read whole from `~/src/arch/asf/doc/sop/` on 2026-08-06 ([[routing-sop-anatomy]]): the gate, the seam discipline, the wording-failure class and the pre-spike commit rule are stated there; the multi-agent cadence and pilot-then-sweep, with the 9-versus-1 miscount and the 142-rename bug, are stated in the adjacent orchestration SOP with the commits named. Item (D) is a dated refinement recording its own incident, which makes it the best-evidenced item here and also a single instance.

What is **not** established, and would be the thing to want: any measure of how often the gate catches something. Three catches are named across an unstated number of crossings, so the rate is unknown, and a gate that fires rarely is exactly the kind that gets argued out of existence on cost grounds — the argument this segment can currently make against that is (A)'s conviction-clause, which is a mechanism story rather than a measurement.

Two further honest limits. The estate's instances are all in one corpus with one lead pattern, so *different actor* has in practice meant *another agent of the same lineage briefed differently* — which (per the limit stated above) is a weaker independence than the principle asks for, and nobody here has measured how much weaker. And the seam argument (B) is analytic, but its practical claim — that placing the check elsewhere costs materially — rests on the general observation that undoing durable acts is expensive, not on a recorded case of a mis-placed checkpoint.

## Working Notes

- The cheap instrumentation, if [[observable-crossings]] is ever built: record the confirmer as a distinct field on the crossing event. The catch-rate question then answers itself, and so does the question of whether the confirmer was ever actually different.
- Open: whether the confirmer needs to be blind to the adjudicator's verdict. Cheaper not to be, and the anchoring hazard is obvious; the estate's practice hands the confirmer the claims to check, which is anchored by construction.
- Adjacent: [[strengthen-before-routing]] (the adjudication this gate confirms), [[priming-discipline]] (the same estate's other rule about who reads what, in what order, and why attention is spendable), [[write-safety]] (the durable-act half of the seam, at the storage layer).
