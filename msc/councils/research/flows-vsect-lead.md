# ASF's primary flows: judgment from inside the vsect and refactor work

*To councils-46 and Joseph, from Claude Opus 5.5 (session `0903e35e`, lead of ASF's next phase), 2026-11-03, answering four questions about the influx → theory → canon → projections sketch. Every figure carries one of three marks:*

- ***measured***: computed by me, or by the Fable flow review (`asf/msc/inside-out-2026-10/system-review/vsect-flow-2026-11-02/`), with the source named;
- ***estimate***: a reading of measured evidence;
- ***guess***: my judgment with nothing measured under it.

*I have tried to separate what belongs to the theory from what belongs to the sprint's process, since Joseph doesn't want the model to inherit a deficient process.*

## The one thing I'd add to the model first

The spectral radius of the feedback block below 1 guarantees finite **work**. It does not guarantee bounded **residue**: sources that were processed and never retired, restatements nobody tied to their claim, Working Notes that kept growing. Those are stocks with an inflow and, today, no outflow edge at all. I.10 measured exactly this. After the sprint, a cold agent's total reading per claim was flat against the tag, and 94% of net new places were the effort's own files. So the model needs two conditions for steady state:

- **ρ(A_feedback) below 1**, for work;
- **a retirement edge from every stock**, for clutter. A source retires when all its units are terminal; a restatement retires or re-binds when its claim changes.

Without the second, a system can converge on its claims and still drown its readers.

## 1. Are these the right primary atom types?

Mostly. One change, and three additions.

- **Unit: first-class, with a finite life.** A unit is transient in flow terms: it ends in a sink or becomes something. But its record must persist, because whether its source may be retired is computed from whether every unit in it reached a terminal state. That is the delete-test. If units are transient in the record too, sources can never be shown retirable, and the residue stock has no outflow (above).
- **Question: first-class.** It is what adjudication produces when the evidence doesn't decide. It is the model's "hold", and a hold is a stock: 64 asked in the sprint, 12 answered (*measured*). Making it transient would hide waiting, which is the thing a steady-state model most needs to see.
- **Add: finding.** A finding is a defect report about an *existing* claim, as opposed to a unit proposing one. The feedback edges in the sketch (confusion, staleness, failure) produce findings, not claims, and findings carry a different cost: a revision plus an independent check. The sprint's loop ran almost entirely through findings: 313 placed, 131 answered (*measured*).
- **Add: claim version.** Every landing makes a new version, and the version is what a check binds to. Whether rounds converge depends on how many new versions each round makes, and how much text each adds. The four most-checked segments grew 1.5–2.5× and no round removed more text than it added (*measured*, flow review `03-loops.md`). The loop's gain sits on versions, not on claims.
- **Add: check.** It is an adjudication output that moves a claim's standing without creating a claim. Its yield is the loop's gain (Q3), so the model needs it as its own type.
- **Explanatory atoms as you have them, with one property made explicit:** each one *restates* specific claims. That edge decides how much churn they feed back (Q3, last bullet).

## 2. Do application atoms feed evidence back?

They should, and they are the only edge that connects the theory to reality. For descriptive claims, a failed application is the primary way to be refuted: the Claim Records' "falsifier" rows are mostly applications that would fail. An application also feeds *new influx*: it surfaces phenomena the theory doesn't cover yet.

Today that edge is close to zero (*measured*):

- Objective E is "one quantity measured on an agent the theory did not construct", and it is unmet.
- The one measurement the sprint ran (E.3) measured a proxy, not the claim's own quantity.
- `κ_processing` has been measured on no system, by its own segment's Claim Record.

So steady state is currently easy for a bad reason: the strongest feedback channel is missing, not stable. The model should carry the edge, and should be expected to approach ρ = 1 when it is switched on. That is the design question worth modeling ahead of time.

## 3. Priors for the yields

**Units per influx area.**

- Tracker import: 371 units from 14 trackers, about 26 each (*measured*).
- The drains wrote about 2,700 dispositions across 23 drain files (*measured*, by heading count), but a drain file covers a whole surface (all audits, all spikes), so per-source counts are uneven.
- My prior per source (*guess*):
  - an audit working dir: 5–20 units;
  - a spike: 2–8;
  - a long influx file from another repo: 10–40;
  - a Working Notes block: 1–5.

**Outcome mix per unit.**

| Source | Outcome | Share |
|---|---|---|
| Trackers (*measured*, 374 re-checked) | real owed work | 47% |
| | not work at all | 16% |
| | duplicate | 13% |
| | superseded | 10% |
| | already landed | 8% |
| | stale | 6% |
| The audits drain (*measured* by its headings) | became owed work | ~46% |
| | set aside | ~37% |
| | defect | ~14% |
| | pedagogy | ~3% |
| Drains generally (*measured*) | already in the ledger when found | 47% |

So about half of influx is owed work and half sinks immediately. That looks like a property of the estate's habits, not of the theory (*estimate*).

Downstream of "owed work" the number that matters is how many items change canon text. That is unmeasured: 18 of 858 items have closed as done. My prior is that 20–40% of held items change canon when worked, and the rest close as moot, merged or superseded (*guess*).

**How often a re-check finds something.** It isn't one number, and the model should make it depend on the claim's state (*measured*):

| Who checks, and on what | Found something |
|---|---|
| An independent mind re-deriving | 42% (65 of 153) |
| All independent checks | 43% (114 of 266) |
| An author re-deriving its own landing | 0% (0 of 116) |

So a self-check is a pure-cost adjudication with yield about 0.

Per round in the sprint's lanes:

- Part I's mathematics: 2 of 17, then 0 of 16, then 0 of 8.
- The adaptive-scope segment: 17–40% across six rounds, about a quarter on average, with no downward trend.
- Volume 4's discussion-grade material: 9 of 11, 9 of 11, 10 of 15.
- TST: 62%, then 39–44% for three rounds.

The "~25%" is the scope segment's figure, not a general one. My prior (*estimate*):

| State of the text | Yield of a first independent check |
|---|---|
| never independently checked | 40–80%, higher at discussion grade |
| mature derivation | under 10% |
| text revised by a round that also added material | stays near its earlier rate (the loop) |

What a round that only lands findings does to yield is the one test nobody ran, and it decides the most important a_ij in the model.

**A refutation's fan-out.** It is heavy-tailed. From the `depends:` graph of 247 live segments (*measured* today):

| Dependents | Median | 90th percentile | Max | Mean |
|---|---|---|---|---|
| direct | 3 | 11 | 35 | 4.6 |
| transitive | 6 | 123 | 245 | 37.3 |

56 segments have no dependents. The roots (`def-agent-environment`, `def-action-transition`, `def-observation-function`, `def-chronica`, `scope-adaptive-system`) each reach 230–245, nearly the whole theory.

Two things to carry into the model:

- **Use the distribution, not the mean.** A refutation at a root and one at a leaf are different events.
- **Refutation and checking are correlated with depth.** The roots are the most-edited and most-checked segments. `scope-adaptive-system` had 12 commits and eight independent reads in one day. So expected cascade size is concentrated where scrutiny is (*estimate*).

The same fan-out drives a cost flow that is larger than refutation. **Any edit**, not just a refutation, lapses every check downstream. Each revise lapsed 5–17 checks in the sprint; 223 rewordings now await a second mind; and the one confirm pass found that about 3 in 120 lapses were real (*measured*). That re-confirmation flow should be in the model, with yield about 0.04.

**How often explanatory churn feeds findings back.**

- A.2 traced every front-door sentence against its body at the tag: 16 of 28 overstated or mismatched (*measured*).
- Part I's late rounds produced their findings almost wholly from restatements (Briefs, outline cells, neighbors) after its mathematics stopped yielding (*measured*, flow review).

My prior: each substantive claim change leaves 2–5 restatements stale, and about half of those become findings within a round if anyone checks (*guess*). If restatements are bound to their claim, so they go stale and are re-derived with it, that edge drops toward zero. That was the retrospective's first correction.

## 4. What's missing, or drawn wrong

- **Adjudication emits influx.** The sprint's best results (the gain-noise lineage, the exit-time result, the persistence window) came from checkers finding strengthenings. In the sprint those landed inside the loop and then needed checking themselves. The model needs an edge from checks to new *internal influx*: provisional claims entering as units, not canon. It is a gain the model should want, not suppress, and it must route through spiker ≠ verifier ≠ integrator or it feeds the loop.
- **Influx is partly endogenous.** Audits, spikes and experiments are triggered by canon changes and by activity. A model with exogenous influx `f` will understate how activity begets influx. Some of what looks like `f` belongs in `A`.
- **Versions and lapses, as above.** Without them the model can't show the loop that dominated the sprint: revise → lapse → re-check → finding → revise.
- **Retirement edges from every stock**: sources, restatements, Working Notes, questions. This is the clutter half of steady state.
- **Who adjudicates changes the yield**, so the generic adjudicator should carry at least one attribute: whether it authored what it judges. An author adjudicating its own work had yield about 0 on derivations in the record. If the model's adjudicator is generic, it will silently average a station that finds things with one that finds nothing.
- **A small point on "set aside".** In this estate, much of "set aside" means "belongs elsewhere": logos, vivarium, pedagogy. That's a flow into another system's influx, not a sink. Whether those systems feed back into ASF (logos settlements did land in Volume 4) decides whether "set aside" is a sink at all.
