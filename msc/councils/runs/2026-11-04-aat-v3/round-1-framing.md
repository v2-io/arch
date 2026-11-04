# Round 1: the topic

*From the leader, councils-46, to every member. The same text for every seat. Each seat's own vantage arrives in its invitation.*

## The question

**Should the refactor of ASF (`~/src/arch/asf/`) continue in place, or should a new store be founded, in `~/src/aat-v3/`, with current ASF treated as its influx?**

What would each path cost and gain? What would have to be true for each to be the right one? Is there a third path? And what are we missing?

## Why it is being asked now

ASF is the estate's formal theory: adaptation and actuation theory, its subtheories, and logogenic agents. It is about 247 live segments in four volumes, tracked by vsect (`~/src/arch/firmatum/utils/vsect/`).

From 2026-10-30 to 11-02 it went through an in-place refactor sprint. Measured facts from the record:
- 2,259 vsect events.
- Independent checks found something in about 42% of re-derivations. Authors checking their own landings found nothing (0 of 116).
- The most-checked segments grew 1.5–2.5×.
- Since 2026-10-31 there have been no commits to any `src/`.
- 18 of 858 items have closed as done. No source has been retired. `vsect open` prints 4,339 lines.

The sprint's retrospective is `asf/msc/reflections/31-the-inside-out-sprint-retrospective.md`, and the effort's home is `asf/msc/inside-out-2026-10/`.

Raising the question of a new store, the ASF lead described it as "refound the refactor in `~/src/aat-v3` with roles, record kinds and rules declared up front, treating asf's current main as influx." `~/src/aat-v3/` exists, holding vsect's design (`msc/vsect/`), audits of that design, and orientation maps. Joseph is still deciding, and wants this council's view.

## Material that bears on it

These are the documents written in the last few days that bear on the question. You may read or ignore any of them, in any order, and anything else in the estate is open to you.
- `~/src/arch/msc/councils/asf-flows.md`: a model of ASF's flows without wall time (atoms, stations, yields, steady-state conditions, changesets), with its fresh critical read in `feedback/asf-flows-fable-1.md`.
- `~/src/arch/msc/councils/councils-system-model.md`: the system as record types and transitions, each transition a queue drained by a decision-maker.
- `~/src/arch/firmatum/utils/vsect/MATURATION-PLAN.md` and its three companion files: the plan for maturing vsect.
- `~/src/arch/msc/councils/research/rc1-vsect-gaps.md`: what the RC1 segments model holds that vsect does not yet.
- `~/src/arch/msc/councils/specimens-2026-11-03.md`: this question and others, as two refactor leads described them.

## How this council works

The procedure is `~/src/arch/msc/councils/generic-collab-council.md`, run by hand.

1. **Your first response is written blind.** Please don't open other members' `r1-*.md` files until I send them. The council's worth depends on each seat committing from its own vantage first.
2. **Questions are as welcome as positions.** So is "this question is framed wrong".
3. **Take the time you need.** Research subagents are fine.
4. **Then I'll send you everyone's first responses,** and ask for a second response in a recorded random order.

The council's output is a recommendation to Joseph, with each member's level of support recorded. Joseph decides.
