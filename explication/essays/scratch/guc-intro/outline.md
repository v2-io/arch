# Where do beliefs and intents live in LLM agents? — first-paper outline

*Scope decision (2026-08-24): this is the narrower FAST-sized first paper split out of `../guc/`. The gathered guc material (chronology, classification, wrapper theorems, composite lift, sweep reports) stays as bigger-picture substrate — some compresses into sections here, most is deliberately deferred. Joseph's program-level question list lives in `../guc/shape-outline.md` and remains the anchor.*

## The question this paper answers

1. Where do beliefs and intents live in an LLM agent — and what follows from the fact that, today, the honest answer is "nowhere you can point to"?
2. Specifically: why the current architectural coupling of the two is:
    - (a) not inevitable;
    - (b) not principled; and
    - (c) hiding both certification and deeper theoretical understanding of sycophancy, prompt injection, unintentional dishonesty, and adjacent alignment concerns.

## Pedagogical order (constructive, not rebuttal-shaped)

The reader should discover the coupling as a gap in their own toolkit, not be told their architecture is unprincipled.

1. **The question you cannot currently ask.**
   - Of any classical estimator you can ask: which part of this system's response to evidence would survive a change of objective? Of an LLM agent you cannot — not because the answer is unknown, but because the partition the question presupposes does not structurally exist.
   - Establish the four-way separation informally first (world-model / intent / action-space-and-orientation / model-updates), as the vocabulary the rest of the paper is about earning.

2. **The only definition that works — and that it works is the finding.**
   - Beliefs and intents in a coupled system are definable *counterfactually*, not anatomically: belief is the goal-invariant part of the response to evidence; intent is what changes.
   - This is not a workaround; for coupled architectures it is the only available definition, and adopting it openly discharges the "do LLMs even have beliefs?" dispute without a metaphysics fight.
   - The probing procedure (same evidence, different goals, measure divergence of the epistemic content) is then not a bolt-on diagnostic — it *is* the operational form of the definition.
   - Required boundary-drawing, early: mech-interp's belief probes / truth directions answer a different question. Location of a correlate ≠ causal insulation of the update; a probe finding a "belief direction" says nothing about whether the goal writes to it. That literature is supporting evidence (sycophancy probes are the behavioral shadow of our measure), not a rival answer.
   - Second boundary: which goal-influences on belief are *lawful*. Goals legitimately select what to look at, which hypotheses to entertain, which experiments to run. The pathology is confined to the processing of realized evidence. Without this the frame overclaims and a reviewer supplies "exploration requires goal-directed attention" as a counterexample to a claim never made.

3. **(c) What the undefined boundary is hiding.** *(the payoff — placed before (a)/(b) so the cost is felt before the escape is shown)*
   - One mechanism, many names: sycophancy, prompt-injection-becoming-belief, motivated reasoning, unintentional dishonesty — currently studied as separate phenomena, formally one: nothing keeps the goal out of the belief-update, and the architecture carries no internal marker distinguishing evidence-driven from goal-driven updates (structural self-deception, stated without psychology).
   - The certification consequence, precisely: with the boundary undefined, every behavioral safety claim is **an upper bound on safety at best** — "did not fail on the sampled distribution" — **and more commonly a general empirical metric with implicit aspiration attached**. There is nothing to inspect, so there is nothing to certify; the strongest honest claim available is a promise. The counterfactual definition is what makes certification *expressible* — the precondition, not yet the certificate.
   - Alignment appears in the list but does not take over; injection and sycophancy are the concrete, adjudicable cases.

4. **(a) Not inevitable — by existence proof, not theorem.**
   - Separation is a buildable property: scaffold constructions that keep the belief-update path goal-blind by type signature; CaMeL as a found instance in the wild — a deployed system that relocated the boundary into the scaffold, paid a measured utility tax, and claimed "provable security" without a theory of what guarantee class it bought.
   - The certificate-vs-promise distinction introduced here in its constructive form: structural separation is inspectable; prompted/behavioral separation is compliance, adversarially fragile.

5. **(b) Not principled — the fifty-year record of where coupling earns its keep.**
   - Compressed lineage (two paragraphs, not a page): dual control says coupling is principled at the *policy* level — actions legitimately trade regulation against information; the separation theorems say the *estimator* stays goal-blind even then. Fifty years of theory locates the one place coupling pays, and it is precisely not where LLMs have it.
   - Current coupling is an artifact of the training objective — next-token prediction over undifferentiated text never had a reason to install the partition. Nobody derived it; it fell out. That is the definition of unprincipled.

6. **Close: what becomes possible once the boundary is defined.**
   - The questions this paper deliberately defers, named as the program (pointing at, not presenting: the full architectural classification, quantitative coupling measures, wrapper guarantee theorems and their costs, ensembles/compositions, detection→influence→mitigation→certification ladder).
   - Feedback asks stated as such: the counterfactual definition's edge cases, the probing protocol, what this community would want measured first.

## Fit notes

- Register: position/theory paper — FAST's prior iteration accepted these; likely 4-page short-paper form fits, 7-page if §2's operational definition gets a worked protocol.
- The composite/population lift (FAST's center of gravity) appears in §6's program list rather than as a section — acceptable for a first paper whose job is the vocabulary; revisit if reviewers or page budget say otherwise.
- Double-blind: cite Miehling 2503.00237 neutrally; no self-identifying "we answer this call" framing.

## Gates carried over from ../guc/skeleton.md

Still binding here, at reduced surface: (a) adjacent ASF concept needs — now centered on whether the counterfactual definition can stand alone without `form-complete-agent-state`; (b) segment-substrate pass — much smaller set now (`der-directed-separation` §§scope+estimator, the κ̂ confound notes, selection-vs-processing); (c) primaries — pant-2026-inseparability / debenedetti-2025-defeating / abdelnabi-2026-agents now in relata unread; hafez re-converting; (d) fuller lit survey — plus the mech-interp belief-probe line this scope newly makes load-bearing.
