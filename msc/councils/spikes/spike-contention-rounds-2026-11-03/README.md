# Spike: contention rounds — sharing a floor among agents without a clock

*2026-11-03. Spiker: Claude (Fable 5.1), commissioned by councils-46 for Joseph. Status: spiked; two independent de novo verifications received and answered (`de-novo-feedback-1.md` → `response-to-feedback-1.md`; `de-novo-feedback-2.md` → `response-to-feedback-2.md`); revision 3 of the result and the sim incorporate both sets of repairs. Not re-verified since the second report.*

**The question.** RONR settles who speaks next by a human chair seeing who rose first. For agents answering through APIs, "first" is latency. What mechanism collects members' concurrent intents and resolves them so the outcome never depends on arrival order, and is still as fair as RONR means — and how much of RONR's contention machinery survives it?

**The short answer, revision 3.** The book says its contention layer presupposes simultaneous hearing [1:1, 9:34]; an agentic council is a principled departure from that letter, with the charter as its authority and the book's devices as precedent. Replace the race with rounds: every present member is sent every state and answers with a sealed, persisting *policy* (guarded acts in priority order); a pure resolver admits points of order first (breach-specific, one per member per transition, each ruled), then other non-floor acts, then exactly one floor act by **one key per claim** — the maker's first speech, then least-recently-given-the-floor, then a lottery committed in advance and keyed by floor grants and seat. No persistent queue, no alternation rung (a declared side is cheap talk), nothing a losing claim can steer. Arrival order and names are never read; silence extends a question and never closes it; a vote round closes at its deadline, which is the one place a slow member still pays, declared as such. Seven properties (F1–F4, I1, IND, S), five of them tested on the toy (F2 and S are derived). Of 35 RONR contention devices: 22 survive, 8 change with a price, 3 are vacuous, 1 conditional, 1 (bylaw-required secret ballot) has no equivalent.

**Read in this order.**

1. `contention.md` — the result (rev. 3), retractions in the body.
2. `response-to-feedback-2.md`, then `response-to-feedback-1.md` — what each verifier found and what was done, finding by finding.
3. `de-novo-feedback-2.md`, `de-novo-feedback-1.md` — the verifiers' reports, unedited.
4. `sim/rounds.py`, `sim/test_rounds.py` — the executable toy and its checks (`cd sim && PYTHONPATH=. python3 test_rounds.py`; output in `sim-output.txt`). The checks encode both verifiers' probes.
5. `explorations.md` — hypotheses, dead ends, what two verifications taught.
6. `proposed-integration-plan.md` — suggestions for the server, with reasons, and a do-not-inherit list.
7. `debrief.md` — for Joseph; opens with what the previous revision got wrong.

**Siblings.** `../spike-charter-agenda-2026-11-03/` owns the charter side and the departure framing this revision adopts; `../spike-walking-skeleton-2026-11-03/` measured the close rule on a running server and found sealing to be honor-system, which became property S here.
