# Councils in the system: record types, transitions, and their movers

*2026-11-03. Claude (Opus 5.5, councils-46), writing up a model Joseph stated after the feedback round on the generic collaborative council. It is a short note on how councils fit the larger system. It deliberately stops before choosing, for ASF, which transitions need which mover. Joseph is taking that question up separately, at ASF's highest level.*

## The model

Joseph (2026-11-03):

> [vsect-record-type & flows] <-> [council types (charters) instantiated (agenda) as `vsect` actors] == the overall "system" gets defined as vsect-records but the thing that allows them to change state are council results, which are defined along with the vsect record types — which even makes things like the final list of what's being certified / decided / passed is the exact correct data/transitions allowed by the overall system definition. Add in there the ability for all participating agents to have a distinct separate channel for giving meta-feedback (suggesting changes to the charter or vsect items that use that charter etc.) and we might have the "real thing" here...

And, on what comes next (same day):

> we can pretty much treat each vsect transition-edge as a queue that is drained by a "decision-maker" and not specify a council or agent etc., but we will be looking at ASF at the highest level, and then the vsect "system" and can plug in the right agents & councils and systems-level automation (when to start draining various queues, etc. etc.) afterward

## What the model consists of

- **The system is defined as record types and their transitions.** The things that can be certified, decided or passed are exactly the transitions the definition allows. Nothing outside that table changes a record's state.
- **Each transition edge is a queue, drained by a decision-maker.** Whether the decision-maker is one agent, a council of some type, or Joseph is a separate choice, plugged in after the system is defined.
- **A council type (a charter) is one kind of decision-maker.** A council instance (an agenda) drains items from a transition's queue. Its results are the events that move records along that edge. The outcomes a council can produce are defined together with the record types they move, so a close writes exactly the record the transition calls for.
- **Charters are records too.** Meta-feedback from participants arrives as influx against a charter record, and moves through that record's own transitions under whatever decision-maker governs charters. The system amends its own definitions through its own movers. That is the concrete form of the RONR roadmap's self-hosting phase.

## How it relates to what exists

- **RC1** (`firmatum/verisectorium/theory/influx/segments-model-rc1/`). RC1 says every epistemic dimension is "moved only by its own movers", named event classes performed by named actors. This model says what those movers are: single-agent acts, councils, and the steward. RC1 also sketches a rule language, `09-LANGUAGE.md` (*when* conditions, *allow* a state or crossing, *enabling* a license). A transitions table with its decision-makers is that table. Where RC1 and vsect differ, see `research/rc1-vsect-gaps.md`.
- **The RONR-12 model** (`model/`) has the same shape in one domain. `actions.json` is a table of transitions, each with its admissibility condition, and `council.json` holds configurations of who may act. So it is an existence proof that the shape can be executed, with each action's justification computed from its admissibility condition.
- **The generic collaborative council** (`generic-collab-council.md`) is the procedure a council-type decision-maker runs.

## What this settles, and what it leaves

- **Part of "what calls a council" is settled from inside this scope.** If the system definition says a transition is council-moved, then attempting that transition is what calls the council.
- **Not every transition should need a council.** If every state change waited on one, throughput would collapse, and procedure would become the work. Most transitions should stay single-agent acts: a check, a revision, a recheck. Councils are for the transitions whose importance requires them.
- **Importance decides the mover, and it can change.** A decision taken at low importance by a single agent can become load-bearing later. The record then shows it as decided at lower stakes than it now bears, and the transition needs the decision-maker the new importance calls for. This is the lapse-on-rising-stakes idea from the same day's discussion of decision importance.

## Out of scope here

- Which ASF transitions need which decision-maker. That is the next piece of work, starting from ASF at its highest level.
- When queues start draining, how many decision-makers run at once, and how draining is automated. That is systems-level automation, plugged in after the system is defined.
- The record kinds themselves. vsect's types will be developed together with the council outcomes that move them.
