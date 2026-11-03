# Spike: contention rounds — sharing a floor among agents without a clock

*2026-11-03. Spiker: Claude (Fable 5.1), commissioned by councils-46 for Joseph. Status: spiked; one independent de novo verification received (`de-novo-feedback-1.md`, Claude Opus 5.5) and answered (`response-to-feedback-1.md`); revision 2 of the result and the sim incorporate its repairs. Not yet re-verified.*

**The question.** RONR settles who speaks next by a human chair seeing who rose first. For agents answering through APIs, "first" is latency. What mechanism collects members' concurrent intents and resolves them so the outcome never depends on arrival order, and is still as fair as RONR means — and how much of RONR's contention machinery survives it?

**The short answer, revision 2.** The book itself says its contention layer presupposes simultaneous hearing and names "methods for seeking recognition and obtaining the floor" as something electronic-meeting rules must supply [1:1, 9:34, 9:36]; its "rose first" is a device it lets large halls replace [42:16]. Replace it with rounds: every present member is asked at every state and responds with a sealed *policy* (guarded acts in priority order); a pure resolver admits non-floor acts first against the round-open state (points of order, objections, seconds — the class the book lets members call from their seats, which is registry data plus an urgency judgment, with a "fragility" probe as a diff against it), then exactly one floor act chosen by the book's preference rules, then least-recently-given-the-floor, then a recorded lottery keyed by seat. No persistent queue, so there is no seniority to farm and no turn for a point of order to steal. Arrival order is never read; a member's name is never read; the clock enters at one door, a presence deadline, and a vote round closes at it with ballots sealed until announcement. Points of order are breach-specific and not re-raisable, which bounds obstruction. Twenty-three of thirty-three RONR contention devices survive, five change with a price, four have nothing left to do, one depends on whether there is a chair.

**Read in this order.**

1. `contention.md` — the result (rev. 2): fairness properties F1–F4 and I1 as the code has them; the mechanism; the two derivations as they stand after verification, with retractions in the body; adversarial cases including the verifier's and Joseph's own examples; the survival table; priced equivalents and the one no-go; what was not done.
2. `response-to-feedback-1.md` — finding by finding: accepted, narrowed, or contested, and what changed.
3. `de-novo-feedback-1.md` — the verifier's report, unedited.
4. `sim/rounds.py`, `sim/test_rounds.py` — the executable toy and its checks (`cd sim && PYTHONPATH=. python3 test_rounds.py`; output in `sim-output.txt`). The checks now include the verifier's probes as tests: relabeling equivariance, the vote leak, the obstructor, the mover-flips-debater case, the implicit guard.
5. `explorations.md` — hypotheses, dead ends, what the verifier's pass taught.
6. `proposed-integration-plan.md` — suggestions for the server, with reasons, and a do-not-inherit list.
7. `debrief.md` — for Joseph; rev. 2 opens with what rev. 1 got wrong.

**Siblings.** `../spike-charter-agenda-2026-11-03/01-what-simultaneity-protects.md` reached the same foundation independently and owns the charter side; `../spike-walking-skeleton-2026-11-03/` measured on a running server that invariance holds when submissions are final and fails when they are replaceable, which fixed this spike's close rule.
