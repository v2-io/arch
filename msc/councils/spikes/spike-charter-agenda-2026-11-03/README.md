# Spike: the executable charter and agenda for agentic councils (2026-11-03)

*Commissioned by councils-46 for Joseph; spiked by a Claude (Fable 5.1) session; verified de novo by an Opus 5.5 session (`de-novo-feedback-1.md`), and again by a second Opus 5.5 session (`de-novo-feedback-2.md`); both sets of findings came back to the spiker and are answered in `response-to-feedback-1.md` and `response-to-feedback-2.md`. Everything below is revision 3 unless marked; items marked PARKED are serial-floor findings the coordinator parked after the higher-council principles arrived. Write-scope was this directory; nothing outside it was changed.*

**The question.** What are an agentic committee's charter and agenda as artifacts a server loads: what they must declare, how a council comes into being, how it carries a question to a recorded decision and reports, how it grows, and which of RONR's machinery still means something when the members are agents and time is logical.

**The answer, in one line.** A charter is an *instance* over the model's `committee` configuration — roll, grant, referral, instructions, pins, the round as the unit of simultaneity, and a logical-time mapping; for a quick council the charter and agenda are one artifact (the referral), for a standing council two; and the two routes Joseph named are the book's own special/standing distinction on one engine.

## Read in this order

| File | What it holds |
|---|---|
| `debrief.md` | For Joseph first: the problem, the attempts, the discoveries, the outcomes, the tangents, in pre-spike vocabulary — including what verification overturned. |
| `response-to-feedback-2.md`, `response-to-feedback-1.md` | Finding by finding: accepted / contested / PARKED, what changed, where. |
| `reaction-higher-councils.md` | Where the higher-council principles (`../../high-councils/`) and this spike meet: the unanimity tier coincides with the book's letter; the report-as-motion repair is the proposition-last shape; `discuss-informally` is the hinge; the presider is a third chair mode. |
| `de-novo-feedback-2.md`, `de-novo-feedback-1.md` | The two independent verifiers' reports (other minds; not edited by the spiker). |
| `02-charter-and-agenda.md` | **The main result.** Declared vs derived; the three ways a council comes into being; question → decision → report → parent; the chair; growth and whether it is "widening the list"; the sibling interfaces. |
| `01-what-simultaneity-protects.md` | The hardest part, first: the principle under the book's simultaneity requirement (and 9:34's explicit exclusion of written process), the ten mechanisms it carries plus four more the verifier found, what a round must carry instead under the contention spike's informed-acts property, two declared drops with their costs, a demonstrated no-go. |
| `03-logical-time.md` | Mechanical inventory of every clock word in the model and the small mapping that makes it run on rounds. |
| `04-machinery-survey.md` | Family by family: keeps / translates / instruction / no-go, with the walking-skeleton's `amend` evidence. |
| `05-in-meeting-acts.md` | The research subcommittee and the co-opted member as event streams; a three-seat council walked end to end on a real specimen. |
| `06-adversarial-cases.md` | Eleven cases outside the motivating family, each with what held, what bent, and what to push on. |
| `charters/quick-council.json`, `charters/standing-council.json` | The two executable instances, in the model's own vocabulary. |
| `tools/check-charter.py`, `tools/mutant.py` | Checks a charter against the model's namespace and types (references, parameter enums/types, patch surfaces, allowed∩disabled, declared params, the 9:36 keys, the clock fields) and prints the resulting action space. `mutant.py` runs both verifiers' mutant sets (fifteen checks: twelve caught, the model's `note` op accepted on purpose, two prose cases named as uncatchable). The result line says what it certifies: the model plus the four overlay ops this spike proposes; prose is not typed; nothing is evaluated. |
| `model-defects.md` | Nine things in the model the work ran into (F1–F2 are real defects in the committee-of-the-whole overlay). |
| `proposed-integration-plan.md` | Suggestions with reasons, and a do-not-inherit list of language that is mine. |
| `00-neighborhood-map.md` | My map of the ground, written before reading the coordinator's drafts. |

## Completion state, honestly

- **Strengthened to the claim:** the charter/agenda artifact exists and is well-formed against the model plus four proposed overlay ops; the "narrow configuration" hypothesis holds on the action-space axis once the model defects F1–F3 and F10 are fixed; the two in-meeting acts are motions reached through `move` (a consultant, not a subcommittee; co-opting at two-thirds with conditions); the report is text the council adopts [51:2].
- **PARKED (serial-floor findings, per the coordinator, 2026-11-03):** the walkthrough's procedure rows and count, the 4:19 window, the reply round and debate budget, and the exact form of the gate on rising.
- **A principled departure, declared:** by 9:34's letter a written council is consultative and may report only unanimity (51:2); the meeting tier is claimed by principle, with the four extra loads the verifier named carried or deliberately dropped (01 §3.5).
- **Demonstrated no-gos:** division of the assembly, the vote ladder, tellers, stand-at-ease under a mechanical chair, call of the house, reconsider-and-enter-on-the-minutes (by vacuity).
- **Not done, and said so:** no engine ran (stage 5 is pending); the walkthrough is hand-derived; the checker tests well-formedness, not admissibility; the corridor's elapsed-world conjunct is a guess at form; the 01 §3.5 load list is not proven complete.
