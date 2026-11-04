# ASF's primary flows: a model without wall time

> [!note] **How to read this now (2026-11-04).** Joseph has decided that ASF will not be iterated on in place: "There is no good reason to iterate on ASF under this repo and in its current regime… that one sprint showed us exactly what it needed to for us to now do this right." The new store is being founded in `~/src/aat-v3/`, and current ASF is ordinary influx to it. So this model describes what the sprint showed, and serves as a design input to the new store. Its levers (§5) and its recommended order (§10) are founding rules for that store, not repairs to ASF.

*2026-11-03. Claude (Opus 5.5, councils-46), with Joseph, written up from a day's conversation. It builds on:*
- *the RC1 segments model (`firmatum/verisectorium/theory/influx/segments-model-rc1/`), with the gaps noted in `research/rc1-vsect-gaps.md`;*
- *the councils work in this directory (`councils-system-model.md`, `generic-collab-council.md`);*
- *the vsect lead's answers and numbers (`research/flows-vsect-lead.md`, session 0903e35e).*

*Register: this is a model to reason with, and nothing in it has run. Every number is copied from `research/flows-vsect-lead.md` with its mark there (**measured**, **estimate** or **guess**). The arithmetic below on those numbers is illustrative, not a prediction. The claims about structure are my synthesis, shaped by Joseph's corrections in conversation; where an idea is his, it's quoted.*

## 1. What this models, and what it leaves out

Joseph's picture of ASF at the highest level:

```
external |                         |                  Models &          Mental Models/
influx   +--------+--------------->| Theory    ----->  Theory    -----> Projections/
         |     ---+----            |(working/          (canon)           Pedagogy/
               internal            |  provisional)                       Implications...
               influx (spikes, audits, experiments)
```

"with most of the main work and complexity happening between flux and working-theory or maybe flux & canon..."

**The aim** is to get the primary flows down, so that we know the relative throughput each part of the system needs, and whether it can reach a steady state at all.

**Left out on purpose:**
- **Wall time.** It is only a proxy here. Everything below is counted per unit of influx, not per hour.
- **Who adjudicates.** Each station is one generic adjudicator: an agent, a council, a human, or a deterministic tool. Joseph: "we can pretty much treat each vsect transition-edge as a queue that is drained by a 'decision-maker' and not specify a council or agent etc." Which decision-maker drains which queue, and when draining starts, is plugged in afterward.
- **The sprint trail's routing.** The empirical state machine records a process known to be deficient. Joseph's caution: it would be "both grossly insufficient and seriously over-complicated at the same time". The trail's *yields* are a different matter (found by the fresh read, `feedback/asf-flows-fable-1.md` §2.2). Some yields are properties of minds and text that survive a change of process: self-check ≈ 0, first independent check 0.42, confirm precision ≈ 0.03, the fan-out distribution, units per source. Those come from the trail and are this model's priors. The sprint's routing choices are what the model sets by design instead: 2 checks per version, strengthenings landed inside the loop, nothing retired.

## 2. The atoms

These are the first-class atoms, as corrected by the vsect lead:

| Atom | What it is | Why first-class |
|---|---|---|
| **influx area** (source) | an external paper, a spike, an audit dir, an experiment, a Working Notes block | the input. It retires only when every unit in it is terminal (the delete-test) |
| **unit** | one thing an influx area says that may matter | its record must persist, or its source can never be shown retirable |
| **question** | what adjudication produces when the evidence doesn't decide | a hold is a stock, and waiting must be visible |
| **finding** | a defect report about an existing claim | the feedback edges produce findings, not claims, and a finding costs a revision plus an independent check |
| **claim** | a node in a graph with precursors (`depends:`) and its own epistemology | the primary output |
| **claim version** | each landed text of a claim | checks bind to versions, and the loop's gain sits on versions |
| **check** | a recorded securing or refuting act on a version | its yield is the loop's gain |
| **explanatory atom** | outline cells, Briefs, front-door sentences, teaching prose; each restates specific claims | its churn is a feedback source |
| **application atom** | a model, implication or measurement that uses claims against something real | the only edge to reality |
| **changeset** (§6) | a coherent multi-atom change with one intent, open until its ripples settle | where the hard dynamics live |
| **tension record** (§8) | a standing contradiction between two well-supported claims | a stable state instead of a loop |

**Where items end:**
- **true sinks:** duplicate, superseded, stale, not work, already in canon, rejected;
- **"set aside":** often not a sink. It is a flow into another system's influx (logos, vivarium, pedagogy). Whether that system feeds back decides whether it is a sink at all.

## 3. The stations

Each station is a generic adjudicator with a finite set of outcomes.

| Station | Consumes | Possible outcomes |
|---|---|---|
| **dispose** | a unit | owed work (a new claim, a finding, a question) · already in canon · set aside to another system · duplicate / superseded / stale / not work |
| **adjudicate** | a finding or an owed unit | a new claim version · a new claim · a question · evidence on a claim |
| **check** | a claim version | holds · finding · refutation (which cascades) · **strengthening** (new internal influx) |
| **confirm** | a check lapsed by an edit | re-bound (the change didn't touch what it read) · ended (redo the check) |
| **restate** | a stale explanatory atom | a restatement bound to the current claim version · finding |
| **apply** | a claim used against something real | evidence · finding · refutation · new influx (something the theory doesn't cover) |
| **mechanical** | anything with a checkable invariant | pass · fail, with the place named |

**Mechanical stations are deterministic adjudicators.** They check what can be checked without judgment: notation consistency across every use, schema validity, link resolution, replay of the trail. The estate already states the principle: "logogenic proposes, deterministic disposes". Where a check can be mechanical, it should be. Such a check costs no context and doesn't tire, and its yield is exact.

**One attribute matters at every judging station: did the adjudicator author what it judges?** A generic adjudicator that ignores this averages a station that finds things with one that finds nothing (§4).

## 4. Yields

Each edge carries two quantities, and they must not be merged:
- **gain** A: a_ij is the expected number of items of type j produced by processing one item of type i;
- **cost** C: c_i is the adjudicator-units consumed in processing one item of type i, at the station that handles it.

Then:
- **items by type per unit of influx** are x = (I − Aᵀ)⁻¹ f, where f is the influx vector;
- **each station's required share** of adjudicator-units comes from x weighted by C;
- **the spectral radius** that decides whether work is finite lives on A alone.

A full re-check and a confirm can have the same gain (they find the same real changes) at very different cost. That is why the two are carried separately.

**The matrix's shape comes from RC1** (my synthesis, untested). Its coupling map says which entries are nonzero, and its substitution law says which must stay zero. For example, authority never moves evidence, and a beautiful explanation never moves support.

The priors that matter, from `research/flows-vsect-lead.md`:

**Units per source:**
- trackers: about 26 each (**measured**: 371 from 14);
- an audit dir: 5–20 (**guess**);
- a spike: 2–8 (**guess**);
- a long influx file: 10–40 (**guess**);
- a Working Notes block: 1–5 (**guess**).

**What happens to a unit:** about half become owed work and half sink at once (**measured** on trackers and the audits drain). Of the owed work, the share that changes canon text when worked is unknown: only 18 of 858 items have closed as done. Prior: 20–40% (**guess**).

**What a check finds depends on the state of what it checks:**

| | Found something |
|---|---|
| independent re-derivation | 42% (**measured**, 65 of 153) |
| author re-deriving its own landing | 0% (**measured**, 0 of 116) |
| mature mathematics, by later rounds | under 10% (**measured**, Part I: 2/17 → 0/16 → 0/8) |
| discussion grade | ~80% (**measured**, Volume 4: 9/11, 9/11, 10/15) |
| text revised by rounds that also added material | stays near its earlier rate (**measured**, the scope segment: 17–40% over six rounds, no trend) |

So **maturity and independence must be tracked from the start**: they change the most important yields by an order of magnitude. This is the general rule: track a dimension when it changes a guard or a yield.
- **Independence** is maturation-plan phase 2.
- **Maturity already has a carrier:** the rungs `kinds.yaml` declares per evidence line, such as `derivation: drafted → premises-checked → independently-rederived`. A record's rung is computable from the checks already recorded, and nothing reads it yet.

**Fan-out is heavy-tailed** (**measured**, `depends:` graph, 247 live segments):

| Dependents | Median | 90th percentile | Max |
|---|---|---|---|
| direct | 3 | 11 | 35 |
| transitive | 6 | 123 | 245 |

The roots (`def-agent-environment`, `def-action-transition`, `def-observation-function`, `def-chronica`, `scope-adaptive-system`) each reach 230–245 segments, nearly the whole theory. They are also the most-edited and most-checked segments.

**Lapses:** any edit lapses every check downstream, 5–17 per revise in the sprint. One confirm pass found about 3 real changes in 120 lapses, a confirm yield near 0.03 (**measured**).

**Restatement churn:**
- At the pre-sprint tag, 16 of 28 front-door sentences overstated or mismatched their bodies (**measured**).
- Each substantive claim change leaves 2–5 restatements stale, and about half become findings if anyone checks (**guess**).
- Binding restatements to claim versions should drive this edge toward zero.

**The application edge is about zero today** (**measured**): Objective E is unmet, and the one measurement run measured a proxy.

## 5. Steady state: three conditions

1. **Work is finite** when the spectral radius of A's feedback part is below 1. Loop gains near or above 1 mean work grows without bound, however many adjudicators run.
2. **Residue is bounded** when every stock has a retirement edge. Spectral radius below 1 bounds the work. It does not bound the residue: sources never retired, restatements not tied to their claim, Working Notes, unanswered questions. I.10 measured residue growing while claims converged. Each stock needs its exit:
   - a source retires when all its units are terminal;
   - a restatement retires or re-binds when its claim changes;
   - a question closes, or is held with its exit stated.
3. **Every edge is drained.** Each station has a declared drainer and a nonzero share of adjudicator-units per unit of influx, or the station is declared a sink. A model without wall time otherwise can't see an edge that exists and is never traversed. The useful form is a comparison: the required shares, from x and C, against the actual allocation. The gap between them is the finding.

**The drainer itself can lapse.** An owner who can answer for work is an agent whose session ends. In flow terms, that is a retirement edge on the drainer: ownership ends visibly unless it is handed over, the same shape as a lapsed check.

**ASF now** (checked by the fresh read at `2133cf2e`, `feedback/asf-flows-fable-1.md` §3.2):
- no commit to any `src/` since 2026-10-31;
- 18 of 858 items closed as done;
- 0 sources retired;
- 12 of 64 questions answered;
- no KR with an agent as owner.

Securing ran at 488 acts against 77 confirms, 29 closes, 12 answers and 0 retirements. ASF fails the third condition, not the first. It is also in a **transient**, not a steady state: most of its influx is already in the store, so the next phase is a backlog drain. For that, the numbers that matter are the share of owed work that changes canon and the allocation of stations, more than ρ.

**Priority is a separate question.** The flow model counts how much work there is, with every unit equal. Which work comes first is priced by what is waiting on it, by cascade size and by staleness: RC1's disposition ordering (`07-DISPOSITION-AND-GAP-ECONOMICS.md`), a projection over this model, not part of it.

### The loops

**L1, the version loop.** Revise → checks lapse → re-check → finding → revise. Its gain per version is (checks per version) × (findings per check) × (versions per finding). The fresh read filled it from the trail at HEAD: 488 checks, 246 revises, 313 findings, so 1.98 checks per version and 0.64 findings per check. Taking one revise per answered finding:

| Regime | L1 gain |
|---|---|
| the sprint as run, about 2 checks per version | ≈ 1.3 |
| one independent check per landing, findings back to the same integrator (retrospective item 5) | ≈ 0.6 |
| the same, plus the lapse route | ≈ 0.8 |

These figures are illustrative, from one day's frequencies. Versions per finding is set to 1 by assumption, and the trail allows nearer 1.9, which would put the sprint's gain near 2.4 and the one-check regime near 1. Two things hold either way:
- the sprint's measured growth (segments 1.5–2.5×, no round removing more text than it added) is what a gain above 1 looks like;
- "one check per landing, then back to the author" is not only hygiene. It is the lever that moves the gain across 1.

**L2, the cascade, decided at the roots.** An edit at a root lapses checks on up to about 245 dependents. How lapses are handled changes the cost of a root edit much more than what it finds. Of 120 lapses in the one confirm pass, about 3 were real (yield ≈ 0.03). So:

| How lapses are handled | Work per root edit | Expected real changes to address |
|---|---|---|
| every lapse re-checked in full | 245 full checks | ≈ 7 |
| every lapse confirmed | 245 confirms, each much cheaper | ≈ 7 |
| read sets: only checks that read the changed text lapse | only those checks | ≈ 7 |

A full re-check also finds defects unrelated to the edit, at the dependent's own first-check rate. That is ordinary checking the edit happened to trigger, not cascade gain. At a root, about 7 real changes, each needing work, is above 1 however lapses are handled. **So a root edit can't be handled as a single edit. It has to be a changeset (§6).** At the median segment (6 transitive dependents, about 0.2 real changes), the cascade is subcritical. Read sets and confirmation are what keep its cost proportionate.

**L3, restatement churn.** Claim change → stale restatements → findings → claim revisions. This is what kept Part I producing findings after its mathematics stopped yielding. Binding restatements to versions cuts this edge.

**L4, reality.** Application → evidence or refutation → claim revision. Today about zero, so steady state is currently easy for a bad reason. Expect the feedback part's spectral radius to approach 1 once this edge is switched on. It is the edge that makes the theory honest, and the one the system has to be designed to survive.

**Endogenous influx.** Audits, spikes and experiments are triggered by activity and by canon changes, and checks emit strengthenings as new internal influx. Some of what looks like exogenous f belongs in A. Strengthenings are a gain the model should want. They must route through spiker ≠ verifier ≠ integrator, or they feed L1.

### The levers

| Lever | What it does |
|---|---|
| don't count self-checks as evidence | they cost work and find nothing |
| route lapses through confirm, with read sets | stops a whole-graph re-check |
| undercut versus rebut | weakening a claim degrades its dependents' support without a hard cascade |
| bind restatements to claim versions | cuts L3 |
| make root edits changesets | contains L2 |
| run the one untested experiment: a round that only lands findings, then one more independent check | decides L1's real gain |

## 6. Changesets

Joseph's statement of the problem, which is the most important complication in the model:

> the tension between what can be bit off by minds as currently constituted and what constitutes an "atomic" changeset. The tension is between what can be adjudicated and comprehended in a context window when in fact the impact radius is likely larger. Certain mutations — for example, a notational change on something used throughout the theory, would cascade into an "atomic changeset" (if we define such a thing as a coherent changeset that ostensibly [or provably] makes the theory *more* true than it was [with coherence being a subfactor of truth]) that involves many many atoms.

**A changeset is a super-item with an internal loop.**
- **Open.** It opens with an intent and a scope: the atoms it expects to touch, which can grow.
- **Intermediate state.** While open, it may hold its part of the theory in an intermediate, incomplete state. That state lives on a branch or overlay, never on canon itself.
- **Internal ripples.** Ripples circulate inside the changeset instead of entering the global queues.
- **Close.** It ends one of two ways:
  - **commit:** atomically, when its ripples have died away and the result is shown to be more true, coherence included;
  - **abort:** canon is left as it was.
- **What the rest of the system sees:** one item entering and one leaving. A cause's ripples stop counting against the global loop gain.
- **One owner** from open to commit, which is integrator continuity.

This is RC1's EPISODE (vsect design question Q17) in its strongest form. It is the same pattern as a database transaction or a long-running saga.

**It drains its neighborhood when it commits.** A changeset "pulls into its changeset all of the memories of other related/adjacent/historical things done in that area to make sure it really is an improvement" (Joseph): the auditors with the same finding, the spike that already tried something, the influx papers that touch it. Those units attach to the changeset and are disposed of when it commits. That retires several sources at once and satisfies the residue condition in batches. Gathering them can be a separate specialist's job.

**What has to fit in one mind is the intent, not the impact.** The impact radius can reach 245 atoms. What one mind must hold:
- the changeset's intent;
- its invariant (what must be true everywhere afterward);
- the judgment residue (the places where applying it takes thought).

Execution fans out to other workers. Coherence is checked by mechanical stations wherever it can be. The clearest case is a notation change: a tool verifies the new notation is consistent across every use, while a mind handles only the intent and the hard cases. **The context-window limit is why mechanical stations matter.** No checker for notation consistency across uses exists yet; `NOTATION.md` is maintained by hand.

**How vsect would carry it, as measured** by Line A on 2026-11-04, extracting the Claim Records from 24 segments (`asf/msc/inside-out-2026-10/system-review/claim-record-extraction-2026-11-04.md`, on branch `worktree-agent-a25fdf864ac319cc9`):
- **Grouping alone saves nothing.** In vsect 0.1.0, one changeset and 24 separate edits leave the same lapses: 234 to 242 pending pairs either way, identical pair by pair. A verdict depends only on the end state.
- **The changeset's value lies in confirmation, and it needs two things vsect lacks:**
  - a **transform vsect can verify**, such as "this section moved, the rest is byte-identical", so most confirmations need no judgment;
  - **read sets at span grain.** Of 15 confirmations still owed, 5 or 6 can't be blanket-equivalent, because those checks judged the moved record itself.

  This is vsect's design question Q17 (episodes) meeting Q2 (partial reads).
- **Moved text strands its evidence** unless the design's split act carries checks and findings along with it. 25 checks and up to 21 findings were stranded this way.
- **Exact undo is free.** Where text returned to an earlier hash, its bindings came back.
- **The new store should hold Claim Records as separate records from the start.** Reviews of the records broke or narrowed 21 of 31 checks, so records fail on their own schedule. vsect 0.1.0 handled a claim-record kind with a `describes` edge without any code change.

**A pilot that is already owed** (from the fresh read): moving the Claim Records out of the 24 segment bodies where they were written. The changeset has:
- one intent;
- a crisp invariant: every body's hashed text is unchanged except for the removed section;
- a mechanical check of that invariant;
- no judgment residue.

It would define vsect's episode by use, before anyone designs it in the abstract. The pilot is wrongly drawn if the global lapse count after commit is no smaller than doing the same work as 24 separate edits.

## 7. The circuit breaker

Joseph: "one way or another we have to be able to allow a minor perturbation to amplify into all sorts of wild ripples and stuff before either stability is regained or we hit some kind of stop-limit where we take a step back and say 'what in the world are we missing here?' (usually some kind of ill-formed problem statement or conflation of ideas that need to be teased apart before the theory can even hope to settle now…)".

**This stop-limit can be computed.** Ripples inside a changeset come in generations, which makes it a branching process:
- **settling:** generation sizes shrink;
- **supercritical:** after a few generations they haven't shrunk.

Supercritical triggers the stop, but **generations must be counted per kind of finding**, or the breaker fires on the wrong loop. The fresh read retrodicted it on the sprint's most-checked segment, `scope-adaptive-system`: its findings ran 2, 2, 2, 2, 2, 3 across six rounds, flat. Raw counts would fire at about round four and send it to reframing. The record says the mathematics held, and most of the flat yield was restatement and wording churn (L3), whose cure is binding restatements, not reframing. So:

| What doesn't shrink | Route |
|---|---|
| restatement and wording findings (L3) | bind restatements to versions, and continue |
| findings on the mathematics itself (L1) | **reframing** |

Reframing doesn't send the work back to execution. It asks whether the problem statement is ill-formed, or two ideas are conflated. A high-council or a de novo look fits it well. In RC1's terms, a conflation is a fused dimension or a mixed composite, and the cure is to split them.

Findings have no kind field today: the 313 recorded have no type. The same founding-questions pass planned for `item` would give them one.

If typing doesn't separate the loops, the breaker needs a different statistic, such as edit mass per generation. The circuit breaker's limits (how many generations, how much growth) are set by importance, like everything else a charter declares.

## 8. Tension records

Joseph: "two seemingly mutually exclusive claims both have a lot of evidence at the moment so that individually they are shown to be 'true' — e.g. the crisis in cosmology / hubble tension — which definitely requires some deep and serious thought — but shouldn't mean burning up cycles and LLM councils in an endless loop alternating between two states."

A standing contradiction is a **state**, not a loop. A tension record owns the conflict:
- **both claims stand** at their evidence;
- **their dependents are conditioned on the tension**, with results under A and under B, which is Joseph's conditional-theorem practice. Overall confidence never exceeds that of the weakest part of the chain;
- **re-evaluation happens only when evidence arrives that bears on the tension**, through its falsifier triggers, never continuously.

**Detection.** Oscillation shows as a period-2 cycle: the same pair of changes reverting each other across rounds. Detecting one converts the loop into a tension record.

ASF hasn't entered this regime yet: the record shows no period-2 cycle. Its one live contradiction (C.3: `#scope-composite-agent` admits what it requires) is a defect needing an adjudicator, not two well-supported claims.

**Seeds in the verisectorium OUTLINE:**
- "a standing contradiction *expands* work, never shrinks it" (the relata transfer);
- the tension typology under `claim-dialectical-synthesis`: context mismatch, boundary condition, flawed foundation.

## 9. How the complete model and the simple one meet

RC1's epistemic dimensions are state carried by atoms. The flow model is edges between atom types. Most of RC1 enters the flow model as guards (which mutations are allowed) and as yields, not as new nodes:

| RC1 dimension | Its edge in the flow model |
|---|---|
| Evidence | check and refute work, and the cascade |
| Backing | the decision path, where councils sit |
| Freshness | claim change → stale restatements |
| Comprehension | explanatory atoms → confusion → findings |
| Efficacy, Conviction | application → evidence |
| Disposition | the routing itself |
| Fidelity | not a flow: the trail every flow writes to |

**Tracking rule:** track a dimension when it changes a guard or a yield. By that rule, the vsect lead's numbers make **independence** and **maturity** the first two to track. The complete model supplies candidates; the flow model admits them when they change what moves or how much.

## 10. What to do with it, and what is open

**What to do, in order** (from the fresh read, `feedback/asf-flows-fable-1.md` §5):
1. **Fill A and C from the trail, once.** The flow review's `scripts/fsm.py` plus a tally by role gives every process-invariant entry. Set the process-chosen entries to the retrospective's rules. Print ρ, the required station shares, and the actual allocation beside them. This belongs in maturation phase 2's `vsect flows`.
2. **Pilot the changeset on the Claim Record extraction** (§6).
3. **Run the L1 experiment** (below). It decides the yield the whole model turns on, and it costs one round.
4. **Retrodict the breaker with typed findings,** by classifying the scope segment's 36 findings by hand.

A model is a reason to run these, not a substitute for running them. The fresh read's caution: this is the fourth design layer (RC1, vsect's design, the maturation plan, these flows) over a theory that has had no hands since 2026-10-31. Filling A once and running one experiment is worth more than a fifth layer.

**Open:**
- **L1's real gain:** a round that only lands findings, followed by one more independent check. If that check still finds problems at about a quarter of the rate, the defects were there all along. If it finds almost nothing, rounds were creating their own work. Versions per finding is the entry this would measure.
- **The cascade with read sets:** recompute L2 at the roots once checks declare what they read.
- **The reality edge:** what switching on application does to the feedback part's spectral radius, and what the system needs in order to survive it.
- **Endogenous influx:** how much of f belongs in A.
- **"Set aside":** whether the other systems feed back into ASF, which decides whether it is a sink.
- **The changeset boundary:** how a changeset's scope is drawn at open, and how it grows.
- **Which decision-maker drains which queue,** and when draining starts. Out of scope here by design.
