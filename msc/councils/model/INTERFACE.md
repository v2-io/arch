# RONR-12 Unified Model — Interface

*The binding interface for the unified model (stage 4) and the contract stage 5 builds against. The model is structured data an engine loads: multiple parts, one namespace, one guard registry, one vocabulary, one expression language. Decomposition rationale: `.priors/RONR-12-MODEL.md` (stage-2, kept for provenance). Source: `../RONR-12/body.md` (§:¶ citations); the source wins over the model wherever they disagree.*

## Files

| File | Part | Owns |
|---|---|---|
| `schema.json` | state schema | the typed context object every expression is evaluated against |
| `vocabulary.json` | shared vocabularies | reference kinds, events, thresholds, classes, temporal anchors, closed enums |
| `guards.json` | guard registry | every named predicate, each with a compiled expression or judgment routing |
| `adjudication.json` | adjudication machine | the reusable judgment sub-machine + per-predicate parameterization |
| `registry.json` | catalog | motion records with Standard Descriptive Characteristics |
| `precedence.json` | precedence | rank order, rank modifications, incidental admission, appeal-yield matrix, applicability table |
| `engine.json` | stack machine | push legality, unwind, serialization/restore, context stack, operations |
| `lifecycle.json` | question lifecycle | states, transitions, timers, history erasures, effect/control |
| `protocol.json` | floor & dialogue | roles, dialogue cycle, recognition preference, interrupts, counters, decorum, voting exchange |
| `rules.json` | rule hierarchy | classes, authority order, interpretation, suspension, change thresholds, breach grounds, order objects |
| `scheduler.json` | scheduler | temporal frame, queues, dispatch, preemption, carryover, agenda overlay |
| `voting.json` | vote arithmetic | ballot-counting decision table, election rules |
| `actions.json` | action space | the root table: every action kind, its role, its admissibility expression — what `available_actions` enumerates |
| `council.json` | configuration layer | the axes instantiating the one machine into a particular context (small board, committee, convention, …); ordinary-society is the named default configuration |
| `acceptance.json` | acceptance suite | script-replay tests, the three sweeps, verified fixed points |
| `gaps.json` | honesty layer | every condition not compiled: schema gaps vs. genuine judgment-inputs |

## Execution model — the success criterion

The whole model reduces to one function, recomputed on every event:

```
available_actions(member, role, state) → [(action, justification)]
```

`actions.json` is its table: each action kind carries a role and an admissibility expression over `schema.json`. The parts are inputs to this function, not ends in themselves: registry/precedence/guards decide what is movable; protocol decides who may speak or interrupt now; lifecycle + scheduler decide what is timely; rules decides thresholds; the adjudication machine supplies the judgment-shaped entries. **Justification is by construction**: the admissibility expression's satisfied conjuncts, each carrying its §:¶ citation — no separate justification text exists or is needed. As events fire, the state changes and every member's list is recomputed. The end.

## Conventions (binding)

1. **Citations.** Every normative datum cites RONR 12th ed. §:¶ as strings: `"12:7(2)"`, `"41:53-55"`, footnotes `"56:49n1"`, plates `"t4"`. Rules are expressed in the model's own words — cite, don't transcribe.
2. **One namespace.** References are `kind/id` (kinds closed, listed in `vocabulary.json reference_kinds`): `motion/amend`, `guard/no-question-pending`, `state/tabled`, `event/stated`, `queue/general-orders-current`, `class/bylaws`, `order/<id>`, `timer/table-expiry`, `table/appeal-yield-matrix`, `role/chair`, `question/<uuid>`.
3. **Executable without interpretation.** Every condition an engine must evaluate — guards, applicability, transition guards, dispatch eligibility, preemption, threshold selection — is an expression in the language below, over `schema.json`. Prose plus citation survives only as annotation *on* an expression (`desc`/`note`/`cite` fields), never as the condition itself. A condition that cannot be compiled is flagged in `gaps.json`, not left as prose.
4. **Judgment is routed, not computed.** Predicates RONR leaves to human judgment (germaneness, dilatoriness, decorum, …) enter through the single primitive `["adjudged", pred, subject…]`, whose value is produced by the adjudication sub-machine (`adjudication.json`) from input events — like a vote. Everything *around* the judgment (who declares, contest window, appeal, threshold, finality) is decidable structure.
5. **Guards.** Named predicates, kebab-case, positive polarity: named for what is true when they hold; never a negated twin — negate at the use site. All declarations live in `guards.json` only.
6. **Honesty markers.** Modeling judgment calls are marked `NOTE(judgment)` where they occur; they are the honest-uncertainty layer and survive all refactors. Distinct from `["adjudged", …]`, which marks *RONR's own* judgment points.
7. **Time.** Timers reference named temporal anchors (`vocabulary.json temporal_anchors`), never raw durations. Timer definitions are data (`start_event`, `expires` expression); `["timer-open", id, q]` is how conditions consult them.

## The expression language

Expressions are JSON arrays in operator-first form; atoms are JSON strings (ids/enum values), numbers, `true`/`false`/`null`. Strings in operand position that match a quantifier-bound variable are variable references; otherwise they are literals (ids/enums). Evaluation is against the context object (`schema.json`) plus bindings supplied by the decision point (conventionally `m` = the motion id being moved, `t` = its target frame or null, `q` = the question object, `member` = the acting member).

### Core forms

| Form | Meaning |
|---|---|
| `["and", e…]` / `["or", e…]` / `["not", e]` | boolean connectives |
| `["if", c, a, b]` | conditional |
| `["=", a, b]` `["!=", a, b]` `["<", …]` `["<=", …]` `[">", …]` `[">=", …]` | comparison |
| `["in", x, ["list", …]]` | membership; `["list", …]` is the literal-list constructor |
| `["+", a, b]` / `["*", a, b]` | integer arithmetic (threshold evaluation) |
| `["exists", "v", L, p]` / `["forall", "v", L, p]` | quantify variable `v` over list expression `L` with predicate `p` |
| `["count", "v", L, p]` | number of elements satisfying `p` |
| `["param", name]` | a binding supplied at the decision point |
| `["var", name]` | explicit reference to a quantifier-bound variable — the normative form. A bare string matching an in-scope binder is still read as a variable (legacy tolerance), but new expressions use `["var", …]`; the collision hazard is why. |

### Accessors (over `schema.json`)

| Form | Meaning |
|---|---|
| `["ctx", path…]` | walk the context object by field names |
| `["attr", obj, field…]` | field access on any typed object |
| `["reg", m, field…]` | registry record lookup for motion id `m` |
| `["frames"]` | frames of the active pending stack, bottom→top |
| `["top"]` / `["bottom"]` | immediately pending frame / base frame (null if empty) |
| `["motion", f]` / `["class", f]` / `["erank", f]` | a frame's motion id / class / effective rank (number: position in `precedence.json rank_order`, positionally modified per `engine.json effective_rank`) |
| `["applied-to", f]` / `["adheres-to", f]` | frame links (frame or null) |
| `["question", f]` | the frame's lifecycle question object |
| `["state", q]` | question's R1 disposition state |

### Derived builtins (evaluator-provided; defining rule fixed here)

| Form | Meaning |
|---|---|
| `["debatable", f]` | boolean: resolves the registry `debatable` value for the frame's motion, following `if-target-debatable` through `applied-to`, and treating `yes`, `yes-restricted`, `yes-opens-question` as true, per-record exception conditions applied `[12:7(5), 37:9(5)]` |
| `["amendable", f]` | boolean, analogous, from the registry `amendable` value |
| `["guard", id, args…]` | the named guard's expression, inlined with bindings |
| `["adjudged", pred, subject…]` | current adjudicated value of a judgment predicate for a subject: the predicate's default until an adjudication instance finalizes otherwise (`adjudication.json`) |
| `["timer-open", id, q]` | the named timer (`lifecycle.json timers`), instantiated for question `q`, has started and not expired |
| `["event-occurred", event, scope…]` | the event has occurred within the scope (a question, a question's series, the session, the day) — evaluated over `ctx.history` |
| `["rank-admissible", m, t]` | the ranked-motion class test: m's effective rank (positionally computed per `engine.json effective_rank`, with t as its application target) exceeds the effective rank of every frame in the active stack `[5:8-9]`; vacuously true on an empty stack for main-class motions |
| `["applicable", m, t]` | `precedence.json table/applicability` row lookup for m: the row's `applies_to` expression holds and no `out_of_order_when` disjunct does; motions without a row route through `guard/legitimately-incidental` plus their registry conditions |
| `["same-day", a, b]` / `["before", a, b]` | calendar predicates over times/anchors |
| `["within-quarterly", s1, s2]` | the two sessions are within a quarterly interval `[9:7]` |
| `["anchor", name, session?]` | resolve a temporal anchor to a time in context |
| `["now"]` | current time |

`["attr", obj, k…]` keys may themselves be expressions (e.g. `["ctx","clock","current_day"]` as a counter key).

The operator set is **closed**: an engine implements exactly these forms. Needing a new form is a schema/interface change, recorded here and in `CHANGELOG.md`.

### Evaluator strictness (semantics for partial data)

- `["attr", x, …]` on a non-record (including `null`, `true`, `false`) yields `null` — never an error. This is load-bearing: e.g. the interrupt attribute is `false | true | {conditional…}`, and `["attr", …, "conditional"]` on `false` must yield `null`.
- Comparisons where either operand is `null` are `false`, **except** `["=", x, null]` / `["!=", x, null]`, which test null-ness itself.
- An unresolved `["param", name]` (binding not supplied at the decision point) is an **error**, not `null` — decision points declare their bindings; a missing one is a model bug, not missing data.
- Unknown guard/predicate/timer/event ids are errors.

### Lint rules (enforced by `tools/sweep.py`, part of acceptance)

- No comparison whose operands are both literals (an always-true/false conjunct is prose wearing an expression costume).
- Every `guard`/`adjudged`/`timer-open` id and every `event-occurred` event name resolves to its registry.
- Operator closure over every expression position.
- No negated-twin guard names.

## The adjudication sub-machine

One reusable machine, parameterized per judgment predicate (`adjudication.json`). Shape, with the recurring citations:

```
raised(pred, subject)                        — by the chair sua sponte, or via point-of-order [23:1-5]
  → declared(value)                          — chair rules [23:2(7)]; judgment content arrives as an
                                               input event, like a vote
  | submitted                                — chair in doubt submits to assembly [23:18-21];
                                               vote decides directly (debatability per appeal rules)
declared → appeal-window                     — any two members (mover + seconder) [24:2]
  → appealed → assembly decides              — debatability per appeal SDC [24:3(5)];
                                               threshold/tie-or-majority-sustains-chair [24:3(7)]
  | window lapses → final
final: binding for the session [24:1];       — recorded ruling + reasons = persuasive precedent
                                               thereafter [23:10-11]
```

Per-predicate parameters: `default` (the presumed value absent challenge), `subject_type`, `raisable_by`, `timeliness` (guard expression; continuing-breach exceptions `[23:6]`), `appealable` (false only where no two reasonable opinions are possible `[24:2]`), and notes. `["adjudged", pred, subject]` reads the current value: default, unless an instance has finalized a ruling for this subject (session-scoped).

## Acceptance (stage 4→5)

The definitive test is **script replay**: each Form-and-Example passage named in `acceptance.json` is an event stream; at every step, the scripted actor's action must appear in `available_actions(actor, role, state)` with the right justification, and every action the book declares out of order at that moment must be absent from everyone's list. Three sweeps supplement the replays — (a) Chart I right-column diff generated from `precedence.json table/applicability`; (b) Table II re-expansion diff from `registry.json` (the collapse-is-mechanical claim, tested); (c) `[24:9-13]` and `[23:12-16]` replayed as *engine* streams exercising `table/appeal-yield-matrix`. A verification pass, ideally by an agent outside this authoring line, executes them.
