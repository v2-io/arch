# RONR-12 Model — Stage 2: Outline + Interface

*Stages: (1) full text → (2) this outline + interface → (3) independent-but-coherent models → (4) unified models (structured data) → (5) execution engine(s). See `../RONR-12-MODEL.md` for the decomposition rationale.*

Stage-3 rule: each component is internally rigorous in whatever internal scheme suits it, but everything it expresses *outward* (IDs, events, guards, citations, cross-component references) follows the conventions below, consistently, so stage-4 normalization by a fresh agent is a mechanical join.

## Files (stage 3)

| File | Components | Owns |
|---|---|---|
| `01-catalog.md` (+ data) | 1 Catalog, 2 Precedence | canonical motion registry; SDC table; precedence order + guards; ballot-counting decision table |
| `03-engine.md` | 3 Stack machine | frame types; push-legality rule; serialization/restore semantics |
| `04-lifecycle.md` | 4 Question lifecycle | statechart states, transitions, timers, history |
| `05-protocol.md` | 5 Floor & dialogue | roles, turn-taking, recognition preference, speech counters |
| `06-07-rules-scheduler.md` | 6 Rule hierarchy, 7 Scheduler | rule-class priority/defeasibility; suspension; orders-of-the-day queues |

Prose spec in markdown; structured data as fenced JSON (or TSV for wide tables) inside the same file or as sibling `NN-*.json` files — component's choice, declared at top of its file.

## Conventions (binding on all external expressions)

1. **Citations.** Every normative claim cites RONR §:¶ in the form `[12:7(2)]`, `[41:53–55]`, footnotes as `[56:49n1]`. Express the rules in your own words — cite, don't transcribe passages.
2. **Motion IDs.** Kebab-case of RONR's canonical motion name: `main-motion`, `postpone-indefinitely`, `amend`, `commit`, `postpone-to-certain-time`, `limit-extend-debate`, `previous-question`, `lay-on-table`, `call-for-orders-of-the-day`, `raise-question-of-privilege`, `recess`, `adjourn`, `fix-time-to-which-to-adjourn`, `point-of-order`, `appeal`, `suspend-rules`, `objection-to-consideration`, `division-of-question`, `consideration-by-paragraph`, `division-of-assembly`, `take-from-table`, `rescind-amend-adopted`, `discharge-committee`, `reconsider`, `reconsider-enter-minutes`, … The catalog's registry (in `01-catalog`) is the authoritative superset; other components derive IDs by the same kebab-case rule and stage 4 reconciles against the registry.
3. **Qualified external references.** Cross-component references use `component:kind/id` — e.g. `catalog:motion/previous-question`, `catalog:guard/no-question-pending`, `lifecycle:state/tabled`, `protocol:event/stated`, `scheduler:queue/special-orders`, `rules:class/bylaws`. Internal references within a component are unconstrained.
4. **Event vocabulary (core, global).** `made`, `seconded`, `stated`, `debate-opened`, `put`, `adopted`, `rejected`, `withdrawn`, `ruled-out-of-order`, `referred`, `tabled`, `taken-from-table`, `postponed`, `called-up`, `reconsidered`, `session-ended`, `meeting-ended`, `recessed`, `resumed`. Components may add events, namespaced as in (3); prefer reusing core events.
5. **Guards.** Named predicates, kebab-case, declared wherever first used with signature `guard-id: description [citation]`; evaluated against a context (pending stack, session clock, member state, rule state). Components consuming another component's guard reference it qualified per (3).
6. **Classes and thresholds.** Motion classes: `M`, `S`, `P`, `I`, `B`, `M/B` (RONR Table II key). Vote thresholds: `majority`, `two-thirds`, `majority-entire-membership`, `two-thirds-neg` (e.g. objection-to-consideration), `chair-rules`, `single-member-demand`, plus compound forms `notice+majority`, `notice+two-thirds` expressed as alternatives lists.
7. **Time.** Named temporal anchors: `same-day`, `next-business-day`, `end-of-session`, `next-regular-session`, `quarterly-interval` `[9:7]`. Timers reference these anchors, never raw durations.
8. **Honesty markers.** Where RONR is ambiguous or the mapping is a modeling judgment call, mark it `NOTE(judgment):` inline rather than silently resolving.

## Stage-3 acceptance

A component is done when: internally consistent; every normative statement cited; all external expressions follow the conventions above; and it names (in a closing section) the Form-and-Example passages usable as test vectors against it (e.g. `[10:34]` eight-motion stack; `[12:83–89]` substitute walk; `[16:26–27]` competing previous-question forms; `[37:27]` reconsider mid-series) — these become the stage-4/5 acceptance traces.
