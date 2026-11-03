# Spike: the executable charter and agenda for agentic councils (2026-11-03)

*Commissioned by councils-46 for Joseph; spiked by a Claude (Fable 5.1) session. Role separation: this directory is the spike only — verification and integration are other minds' work. Write-scope was this directory; nothing outside it was changed.*

**The question.** What are an agentic committee's charter and agenda as artifacts a server loads: what they must declare, how a council comes into being, how it carries a question to a recorded decision and reports, how it grows, and which of RONR's machinery still means something when the members are agents and time is logical.

**The answer, in one line.** A charter is an *instance* over the model's `committee` configuration — roll, grant, referral, instructions, pins, the round as the unit of simultaneity, and a logical-time mapping; for a quick council the charter and agenda are one artifact (the referral), for a standing council two; and the two routes Joseph named are the book's own special/standing distinction on one engine.

## Read in this order

| File | What it holds |
|---|---|
| `debrief.md` | For Joseph first: the problem, the attempts, the discoveries, the outcomes, the tangents, in pre-spike vocabulary. |
| `02-charter-and-agenda.md` | **The main result.** Declared vs derived; the three ways a council comes into being; question → decision → report → parent; the chair; growth and whether it is "widening the list"; the sibling interfaces. |
| `01-what-simultaneity-protects.md` | The hardest part, first: the principle under the book's simultaneity requirement, the ten mechanisms it carries, what a round must carry instead, a demonstrated no-go, and two gains. |
| `03-logical-time.md` | Mechanical inventory of every clock word in the model and the small mapping that makes it run on rounds. |
| `04-machinery-survey.md` | Family by family: keeps / translates / instruction / no-go, with the walking-skeleton's `amend` evidence. |
| `05-in-meeting-acts.md` | The research subcommittee and the co-opted member as event streams; a three-seat council walked end to end on a real specimen. |
| `06-adversarial-cases.md` | Eleven cases outside the motivating family, each with what held, what bent, and what to push on. |
| `charters/quick-council.json`, `charters/standing-council.json` | The two executable instances, in the model's own vocabulary. |
| `tools/check-charter.py` | Resolves every reference a charter makes against the model and prints the resulting action space. `python3 tools/check-charter.py charters/*.json` — both are well-formed as of this commit. |
| `model-defects.md` | Nine things in the model the work ran into (F1–F2 are real defects in the committee-of-the-whole overlay). |
| `proposed-integration-plan.md` | Suggestions with reasons, and a do-not-inherit list of language that is mine. |
| `00-neighborhood-map.md` | My map of the ground, written before reading the coordinator's drafts. |

## Completion state, honestly

- **Strengthened past the claim:** the charter/agenda artifact exists, loads against the model, and the "narrow configuration" hypothesis holds on the action-space axis with two defects found that must be fixed first.
- **Demonstrated no-gos:** division of the assembly, the vote ladder, tellers, stand-at-ease under a mechanical chair, call of the house, reconsider-and-enter-on-the-minutes (by vacuity).
- **Not done, and said so:** no engine ran (stage 5 is pending); the walkthrough is hand-derived; the checker tests load-time closure, not admissibility; the "same source = same member" rule is asserted, not derived.
