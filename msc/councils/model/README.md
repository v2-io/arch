# RONR-12 Unified Model

An executable model of parliamentary procedure per Robert's Rules of Order Newly Revised, 12th edition: structured data an engine loads, not prose an engineer re-transcribes. One generic machine, instantiated into particular contexts — a small council, a convention, a committee — by the configuration layer (`council.json`): many councils, one engine. The whole model reduces to one function, recomputed on every event:

```
available_actions(member, role, state) → [(action, justification)]
```

Every condition an engine must evaluate is a machine-evaluable expression over a typed state schema; every normative datum carries its §:¶ citation, so justification is by construction — the satisfied cited conjuncts. Judgment calls RONR leaves to humans (germaneness, dilatoriness, decorum, …) are not computed but *routed*: their content arrives as input events, like votes, through a reusable adjudication sub-machine.

## Where truth lives

- **The source wins.** `../RONR-12/body.md` is RONR itself (with `front.md` TOC, `index.md`, Tables II/VIII, Chart I). Wherever model and source disagree, the source is right and the model has a bug. The source is copyrighted, local-only.
- **`gaps.json` is the honesty map** — every condition not fully compiled, classified: judgment-inputs (correct by design), schema gaps (owed), annotation-grade content (preserved but not yet evaluator-consumed). Read it before trusting completeness anywhere.
- **`NOTE(judgment)` markers** flag *modeling* choices where RONR lacks the vocabulary; `["adjudged", …]` expressions flag *RONR's own* judgment points. They are different things.
- **`.priors/` is archaeology, not authoritative** — superseded stage-2/3 material kept inspectable. `CHANGELOG.md` is the history layer; working files state present truth only.

## Reading order

1. **`INTERFACE.md`** — the contract: namespace, expression grammar (closed operator set), the adjudication machine, the execution model. Nothing else parses correctly without it.
2. **`schema.json`** — the typed context object expressions evaluate against.
3. **`actions.json`** — the root table: the action space `available_actions` enumerates.
4. Then the parts as the action expressions lead into them (registry → precedence → guards is the densest path).

## File map

| File | Carries |
|---|---|
| `INTERFACE.md` | conventions, expression language, adjudication machine, execution model |
| `schema.json` | typed state schema; `input:true` marks world-facts supplied with events |
| `vocabulary.json` | reference kinds, events, vote thresholds, motion classes, temporal anchors |
| `actions.json` | every action kind: roles + admissibility expression |
| `council.json` | the configuration layer: seven axes (assembly type, size regime, rule stack, membership determinacy, temporal structure, authority relation, public-law overlay) instantiating the one machine into a particular council — ordinary-society named as the default; small-board and committee compiled as proof configurations |
| `registry.json` | motion records with Standard Descriptive Characteristics (Table II, collapsed + aliases) |
| `precedence.json` | rank order, rank modifications, incidental admission, appeal-yield matrix, per-motion applicability (Chart I) |
| `guards.json` | the single guard registry — every named predicate, compiled or judgment-routed |
| `adjudication.json` | the judgment sub-machine + 13 predicate parameterizations |
| `engine.json` | pending-question stack machine: push legality, unwind, bundles, context stack |
| `lifecycle.json` | question statechart: states, transitions, timers, history erasures, effect/control |
| `protocol.json` | floor & dialogue: cycle, recognition preference, interrupts, speech counters, decorum, voting exchange |
| `rules.json` | rule classes, authority order, interpretation principles, suspension, change thresholds, order objects |
| `scheduler.json` | orders of the day: queues, dispatch, preemption, carryover, agenda overlay |
| `voting.json` | ballot-counting table (Table VIII), elections, threshold arithmetic |
| `acceptance.json` | script-replay suite, three sweeps, independently verified fixed points |
| `gaps.json` | the flagged non-compiled remainder |

## Status

Stage 4 of 5 (full text → outline+interface → component models → **unified model** → execution engine). The engine is pending. Three independent audits have been integrated (see `CHANGELOG.md`); further de novo audits are a standing expectation — their reports land beside the model and are absorbed on integration. Acceptance is script replay: each Form-and-Example passage in `acceptance.json` is an event stream the engine must reproduce — the scripted action present with the right justification, the book's out-of-order actions absent.
