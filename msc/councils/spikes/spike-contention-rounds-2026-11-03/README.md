# Spike: contention rounds — sharing a floor among agents without a clock

*2026-11-03. Spiker: Claude (Fable 5.1), commissioned by councils-46 for Joseph. Status: spiked, not verified. The verifier is a separate mind by this estate's rule; the integrator a third.*

**The question.** RONR settles who speaks next by a human chair seeing who rose first. For agents answering through APIs, "first" is latency. What mechanism collects members' concurrent intents and resolves them so the outcome never depends on arrival order, and is still as fair as RONR means — and how much of RONR's contention machinery (recognition, preference, interrupts, precedence, seconds, consent, division, yielding) survives it?

**The short answer.** All of it that was fairness survives; what does not survive was never the fairness. RONR's "rose first" was a neutral device the book itself lets large halls replace with microphones and lights [42:16–17]. Replace it with rounds: every present member is asked at every state, responds with a sealed *policy* (guarded acts, in priority order), and a pure resolver admits acts in a fixed order — fragile acts first (the ones a passing speech would make untimely; this class is derived from the model, not curated, and it reproduces the book's interrupt list), then commuting non-floor acts as a batch, then exactly one floor act by RONR's preference ladder, then claim age in rounds, then rotation. Arrival order is never read. Wall-clock enters at one door only — a presence deadline — which is the same door RONR uses for a member who left the room.

**Read in this order.**

1. `contention.md` — the result: the fairness statement (F1–F4 + I1), the mechanism, the two derivations, adversarial cases, the survival table over 29 RONR devices, the priced equivalents and the one no-go, what was not done, and where councils-46's drafts are narrower than the material.
2. `sim/rounds.py` and `sim/test_rounds.py` — the executable toy and its checks (`cd sim && PYTHONPATH=. python3 test_rounds.py`; output in `sim-output.txt`). Not the RONR model's engine; a nine-act toy exercising the contention layer. The checks are the evidence behind every "checked" label in `contention.md`.
3. `explorations.md` — the diffuse layer: hypotheses, dead ends, the bugs that turned out to be principles, and things I would want someone to think about that this spike did not settle.
4. `proposed-integration-plan.md` — suggestions for the server, with reasons, plus a do-not-inherit list.
5. `debrief.md` — for Joseph, in pre-spike vocabulary, written by re-reading the above.

**Siblings.** `../spike-charter-agenda-2026-11-03/01-what-simultaneity-protects.md` reached the same foundation independently (RONR's own statement that written deliberation is not a deliberative assembly, 1:1 / 9:34) and names the round's property the *common-state condition*; it owns the charter side (what must be declared), this spike owns how a round's intents are ordered. `../spike-walking-skeleton-2026-11-03/` measured on a running server that arrival-order invariance holds when submissions are final and fails when a seat may replace its intent — a finding `contention.md` §4 absorbs as the close-condition rule.
