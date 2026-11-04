# asf-flows.md, read fresh against ASF as it is

*2026-11-03. Claude Fable 5.1, a session with no part in the councils work or the inside-out sprint, asked by councils-46 (for Joseph) to give an independent read of `../asf-flows.md` at arch `59669aa` and to "ponder how useful it will really be for ASF in its current state." The ask was open; I took it as: is the model right, would it change what anyone does in ASF now, and what does ASF need that it misses.*

*What I read whole: `asf-flows.md`, `research/flows-vsect-lead.md`, `research/rc1-vsect-gaps.md`, `councils-system-model.md`, `generic-collab-council.md`; in asf, `msc/reflections/31-…retrospective.md`, `msc/inside-out-2026-10/{INTENT,HANDOFF,README}.md`, `system-review/00-system.md`, all of `system-review/vsect-flow-2026-11-02/` except `05-questions.md` and `debrief.md`, `.vsect/kinds.yaml`, `doc/sop/agents.sop.md` through its standings; vsect's `MATURATION-PLAN.md` and `help model`; RC1's `07-DISPOSITION…` and `95-ADDENDUM` (EPISODE). What I checked by computation is marked **[checked]** below, with the method; everything else is reasoning from those readings and is marked as such where it matters.*

## 0. The short version

1. **The structure is right and most of it is already written down elsewhere.** The atoms, the two steady-state conditions, the four loops, and the levers are the vsect lead's answers and the flow review, recast as a flow model. The model's own register says so. Three things are new to the estate: the yield-matrix framing (§4), **changesets as super-items whose ripples don't count globally** (§6), and the **circuit breaker as a branching-process test** (§7). Those three are where its value sits.
2. **One arithmetic defect matters.** The L2 table (§5) multiplies 245 dependents by the 0.42 yield of a *first independent check on text being revised*. That yield is not the yield of re-checking a dependent whose upstream moved by one edit; the measured proxy for that is the confirm pass, about 0.03. So the cascade's *findings* from a root edit are about 7 however lapses are handled; what differs between the rows is *work* (245 checks versus 245 confirms), not yield. The table conflates the two, and its own leaf sentence contradicts row 1 (median 6 × 0.42 = 2.5 is not subcritical). The changeset conclusion survives (7 is still above 1 at a root), but on different grounds than the text gives. Details in §2.1.
3. **Filled from the trail, the model retrodicts the sprint's loop and gives the retrospective's rule a number.** At the sprint's ~2 checks per version and 0.64 findings per check, L1's gain is about 1.3; at one check per version it is about 0.6–0.8. "One check per landing, then back to the author" (retrospective item 5) is the lever that moves $\rho$ across 1. That is the model doing something a prose retrospective cannot, and it cost me fifteen lines of Python over events that already exist. **[checked]**, with the caveats in §3.1.
4. **It will not help ASF as ASF is now unless one more condition is added.** ASF's dominant failure today is not loop gain. It is idle stations: zero canon commits since 2026-10-31, a 4,339-line `vsect open`, 18 of 858 items ever closed `done`, 0 sources retired, and no KR with an agent as owner. A model that removes wall time cannot see an edge that exists and is never traversed. The residue condition needs a sibling: *every queue has a drainer with nonzero service share, or is declared a sink.* Without it the model says ASF is in a comfortable steady state ($\rho$ small, application edge zero) while the stocks grow because nobody is at the stations. Details in §3.2.
5. **What would make it useful, in order:** fill A once from the trail (the script exists); pilot the changeset on the Claim Record extraction (24 bodies, root-ish, mechanical, lapses 13 of 36 pairs for nothing) so vsect's EPISODE is defined by use; run the L1 experiment every document names and nobody has run; retrodict the breaker on the sprint's lanes with findings typed, because on raw counts it fires on the wrong loop. §5.

## 1. What I checked against ASF itself

| Claim in the model | Method | Result |
|---|---|---|
| Fan-out: direct median 3 / p90 11 / max 35; transitive median 6 / p90 123 / max 245; 56 leaves; the five named roots at 230–245 | re-parsed `depends:` across the four `src/` dirs, excluding `old-*`; 247 segments | **reproduced** (my transitive p90 is 117; a parse difference, not a disagreement). Five `depends:` targets have no file (`def-cognitive-fusion`, `der-active-salience-management`, `der-self-referential-closure`, `form-structured-rich-context`, `obs-backward-inference-empathy`); likely planned rows, not a flow matter |
| Self re-derivation yield 0 of 116; independent 65 of 153 = 42% | tallied `rederive` events by `actor.role` × `outcome` | **reproduced exactly**: integrator 116 confirmed / 0 broken / 0 narrowed; every non-integrator role 153 with 65 broken-or-narrowed |
| Confirm yield ~0.03 (3 real in ~120 lapses) | `confirm-rewording` events: 74 equivalent, 3 not-equivalent | consistent; I did not recount the 120 bindings |
| Closure 18 of 858; findings 313 placed / 131 answered; questions 64 / 12 | `vsect` dashboard and `act:` tallies at HEAD `2133cf2e` | consistent (dashboard now: 632 items, 182 findings, 52 questions open; 223 rewordings / 336 lapsed checks awaiting confirm) |
| Application edge about zero | INTENT E.1–E.3 status lines; E.3's account | true as stated: E.3 measured file lifetimes against a claim about remaining feature count |
| Per-round yields (scope lane 2/5 … 3/11; Part I 2/17 → 0/16 → 0/8; Vol 4 9/11 …) | read from `03-loops.md`; not recomputed | carried on the flow review's word |
| Units per source, 20–40% of owed work changing canon, restatement churn 2–5 per change | marked **guess** in the source; nothing to check | unverified priors, correctly labelled |
| "Nothing has run since": the state of ASF | `git log --since=2026-10-31 -- 0*/src` → 0 commits; HANDOFF "Nothing is running"; INTENT "Who is on what" has one row | **checked** |
| Claim Records still in bodies | `grep -l "^## Claim Record" 0*/src/*.md` → 24 | **checked** |
| Mechanical stations that exist | `bin/`: `check-commit`, `check-links`, `lint-outline`, `lint-md`, `align-slug`, `refresh-all --check`, `term`, plus `md-press` and `vsect check` | exist. No notation-consistency checker across uses exists; `NOTATION.md` is hand-maintained (an open item, `practica-migrate-notation-md…`). So the model's clearest changeset example, a notation change verified mechanically, has no tool yet |

## 2. Is the model right?

### 2.1 L2: work and yield are two different columns

§5's table, "Expected findings from one root edit":

- *every lapse re-checked in full: 245 × 0.42 ≈ 100.* The 0.42 is measured on first independent re-derivations of text under active revision in an adversarial sprint (§1). A dependent of a root edit is, 117 times in 120, untouched by the edit (the confirm pass). Re-deriving it in full finds what a first independent check would find on *that* text, which for mature derivations the model's own table puts under 10%, and for text already independently checked, lower still. The right expected-findings figure for row 1 is roughly (real lapses ≈ 7) × (yield on a truly affected dependent, unmeasured, plausibly 0.4) ≈ 3, plus whatever a full re-check finds about defects unrelated to the edit, which is not cascade gain at all; it is ordinary first-check yield that the edit happened to trigger.
- The rows differ in **cost**: 245 full checks, or 245 confirms at the confirm station's much lower price. That is a real and important difference, and it is the argument for read sets and confirm. But it belongs in a "work per root edit" column, not in "findings".
- "Leaves (median 6 transitive dependents) are subcritical however lapses are handled" is wrong on row 1's arithmetic (6 × 0.42 = 2.5) and only true on row 2's (6 × 0.03 = 0.18). The median-6 segment is also not a leaf; the 56 true leaves have zero dependents. The sentence should say: *at the median segment the cascade is subcritical through confirm and supercritical through full re-check; at a root it is supercritical either way, so a root edit is a changeset.*

The fix is to carry A as two matrices, or one matrix with two entries per edge: expected items produced (gain) and expected adjudicator-units consumed (cost). The spectral radius lives on the first; the station throughput vector on the second. The model's §4 already says both fall out of (I − A)⁻¹f, but one matrix cannot carry both, and the L2 table is where the two got merged.

Small: with a_ij defined as "items of type j from one item of type i", total items by type is x = (I − Aᵀ)⁻¹ f, not (I − A)⁻¹ f. The spectral radius is the same; the convention just needs stating once.

### 2.2 The trail's state machine: reject its routing, keep its yields

§1 leaves out "the empirical state machine from the sprint trail" because the process was deficient. But every **measured** number in §4 comes from that trail. The model is already using the sprint's A; it just copied it by hand. The honest statement, which would also strengthen the model, is a split:

- **Process-invariant yields**, properties of minds and text that survive a process change: self-check ≈ 0; independent first check 0.42 (0.1 on mature derivation, 0.8 on discussion grade); confirm precision 0.03; fan-out distribution; units per source. These come from the trail and are the model's priors.
- **Process-chosen routing**, which the sprint got wrong and the model should not inherit: 2 checks per version, strengthenings landed inside the loop, a new integrator each round, nothing retired, 61 of 64 questions addressed to anyone.

Stated that way, the model can say exactly which entries of A it takes from the record and which it sets by design, and the "grossly insufficient and over-complicated" caution applies to the second set only. `scripts/fsm.py` in the flow review prints the first set already.

### 2.3 The breaker fires on the wrong loop unless generations are typed

§7's rule: generation sizes that don't shrink after a few generations trigger a stop, routed to reframing. Applied to the most-checked lane in the record (`scope-adaptive-system`: findings per round 2, 2, 2, 2, 2, 3 across six rounds — flat), the breaker fires at about generation four and sends the segment to a high council to ask whether the problem statement is ill-formed. The record says what was actually going on: the mathematics held (check 8's verdict: it holds up, two wording items), checks 6 and 8 found one real defect each, and the rest of the flat two-per-round were restatement and wording findings, the L3 loop. The cure was binding restatements, not reframing. A breaker on raw generation size would have convened a council for an edge that a tool should cut.

So the breaker needs generations counted **per finding type** (body-math defect / restatement-wording / scope-question), and a routing table: flat L3 → bind and move on; flat L1 on the mathematics → reframe. The data to decide the typing is in the 313 `place-finding` events, though their text would have to be classified by hand once; today a finding has no kind field. (Reasoned from the record; I did not classify the 36 scope findings myself.)

### 2.4 Maturity is already declared, and unused

§4 says maturity and independence must be tracked from the start. Independence is maturation-plan phase 2. Maturity already has a carrier: `kinds.yaml` declares rungs per evidence line (`derivation: drafted → premises-checked → independently-rederived`, and the same for simulation, instance, imported, measured, testimony). A record's rung is the maturity the model wants, and it is computable from the checks already recorded. Nothing reads it. The model should name the rung as its maturity variable rather than ask for a new dimension.

### 2.5 Where the model is right and the estate hasn't caught up

- **Changesets.** RC1's EPISODE is an open question (vsect Q17); the model gives it its strongest form and names the one thing that must fit in a mind (intent, invariant, judgment residue) versus what fans out (impact). That is a real contribution. The sprint already had the git half (worktrees, `--no-ff` merges of reviewed SHAs); the missing half is vsect's: lapses and findings raised inside the branch stay inside until commit, and at commit one confirm-set is emitted against canon. Today a lane's revises on a branch lapse checks globally at merge, which is why 336 are waiting.
- **Tension records.** Correct as a design, and for a regime ASF hasn't entered. The record shows no period-2 cycle yet (the sprint was one day). ASF's live contradiction (C.3: `#scope-composite-agent` admits what it requires, eight dependents) is a defect, not two well-supported claims; it needs an adjudicator, not a tension record.
- **"Set aside" is a flow, not a sink.** Right, and checkable now: logos settlements did land in Volume 4. The audits drain's 37% "set aside" is the biggest unmeasured edge in the estate's accounting.

## 3. Would it help ASF as ASF is now?

### 3.1 What it does when filled: a retrodiction, and a number for a rule

Trail frequencies at HEAD **[checked]**: 488 checks, 246 revises (versions), 313 findings; so 1.98 checks per version and 0.64 findings per check (1.18 per independent check, since integrator checks found nothing). Taking one revise per answered finding, L1's gain per version ≈ checks/version × findings/check × versions/finding:

| Regime | Gain |
|---|---|
| sprint as run (~2 checks per version) | ≈ 1.3 |
| one independent check per landing, findings back to the same integrator | ≈ 0.6 |
| the same, plus the lapse route (≈10 lapses × 0.03 real, each a check) | ≈ 0.8 |

This is illustrative arithmetic in the model's own register, on one day's frequencies, with versions-per-finding set to 1 by assumption (246 revises against 131 answered findings says it was nearer 1.9 if every revise were finding-driven, which would push the sprint row to ≈ 2.4 and the one-check row toward 1). Two things it shows anyway. The sprint's measured growth (segments 1.5–2.5×, no round net-removing) is what a gain above 1 looks like, so the model is consistent with the record. And retrospective item 5 is not just hygiene; it is the lever that crosses 1. A prose rule that says "one check then back to the author" and a model that says "that moves $\rho$ from 1.3 to 0.6–0.8, and the untested $a_{ij}$ is versions-per-finding" are different instruments. The second tells you which experiment to run.

### 3.2 What it cannot see: the stations are empty

ASF at `2133cf2e` **[checked]**: no commit to any `src/` since 2026-10-31; no agent running; the one agent-owned KR is the lead's I.1; items are owned by KR ids, which Joseph has said cannot own anything; `vsect open` prints 4,339 lines; done 18 of 858; retired sources 0; findings with an exit other than "revise and get it confirmed" 0; questions answered 12 of 64. Station throughput in the sprint, from `act:` counts: securing 488 : confirm 77 : close 29 : answer 12 : retire 0 : cross 0.

In the model's terms ASF today has $\rho$ comfortably below 1 (nothing is being checked, so nothing lapses) and the application edge at zero, and §5 would call that steady state "easy for a bad reason". That is half right. The other half is that the stocks are not bounded by any gain; they are bounded by nothing, because the edges out of them have no adjudicator. A model without wall time has no variable for "the queue is served at rate zero". The residue condition, *every stock has a retirement edge*, is satisfied on paper for items (`close`), findings (`review-finding`, `dispose`), questions (`answer`), checks (`confirm`); the edges exist; nobody traverses them.

The condition the model needs, stated without wall time: **every station has a declared drainer and a nonzero share of adjudicator-units per unit of influx**, with the share computed from (I − Aᵀ)⁻¹f against the actual allocation. That turns the model's "relative throughput each part needs" from a derived quantity into a comparison: required 1 : 0.3 : 0.5 : 0.1 : 0.2 (say) against actual 1 : 0.16 : 0.06 : 0.02 : 0. The gap is the finding. The councils-system-model says who drains which queue is plugged in afterward; fine, but *whether* each queue has anyone is a steady-state condition, not an implementation detail, and it is the one ASF is failing now.

Related: **the units are not equal, and ASF's bottleneck is choosing.** The model counts per unit of influx with every unit the same. 632 open items are not the same; the decision the lead faces is which fifty to work. RC1 07 already states the ordering (influence priced by decisions waiting, cascade size, staleness). A flow model with an unweighted A computes how much work; ASF needs which work. Either weight f and A by importance, or say plainly that the flow model answers throughput and leaves priority to the disposition projection.

And: **ASF is in a transient, not a steady state.** The influx is mostly already in the store (858 items, 2,700 drain dispositions, 64 spikes, 24 audit dirs). The question for the next phase is a backlog drain, where the useful numbers are the owed-work → canon-change fraction (the model's 20–40% guess) and the station allocation, not $\rho$.

### 3.3 What it changes if taken seriously

- **Changesets, immediately, for work that is already queued.** Moving the Claim Records out of 24 bodies (retrospective item 12, maturation phase 4); moving appendix-grade proofs out of `#scope-adaptive-system`; B.2's retyping of Part II; F.1/F.4's standard names and index families. Each is a root-ish, many-atom, single-intent edit that today would lapse hundreds of checks and emit findings into the global queues. The first one is mechanical and is the right pilot: it has a crisp invariant (hashed text of every body unchanged except the removed section), a mechanical check, and no judgment residue. It would define the vsect episode by use before anyone designs it.
- **Rounds.** The sprint's eight reads of one segment would, under the model, have stopped at the first generation that did not shrink, with the typing caveat of §2.3. The retrospective says the same in prose; the model gives the stopping rule a form a tool can apply.
- **Nothing on objectives A–H.** The model is a model of I. The theory's starved objectives (A.3, B.2, B.3, C.3, E, F, G, H) have had no hands since 10-31, and the model does not move them; it changes how the next hands are paced. That is worth saying to Joseph plainly, because it is now the fourth design layer (RC1 → vsect design → maturation plan → flows → councils) over a store with four unbuilt phases and a theory with no one on it. Joseph licensed exactly this ("thoughts about this as a system in and of itself … is wholly worthy"), so I'm not counselling against it; I am saying the marginal use of a fifth layer is lower than of filling A once and running one experiment.

## 4. What ASF needs that the model misses

1. **The drain condition (§3.2).** The one addition that would make the model describe ASF now.
2. **Ownership as a flow atom.** An owner that can answer for work is an agent with a session that ends. The maturation plan names it (`serves:` plus an assignment to an agent key, with since-when and handed-over-by). In flow terms, an owner's session ending is a retirement edge on the *drainer*, and the model has no such thing. Ownership lapsing is the same shape as a check lapsing and should be shown the same way.
3. **Finding kinds.** The breaker (§2.3), L3 (restatement findings), and the "what a check finds" table all need findings typed. 313 exist with no kind. The founding questions were to be run on `item`; findings need the same run.
4. **The reality edge is a measurement, not a model.** The model marks it zero and says the system must be designed to survive switching it on. True. But what ASF needs there is E.1 or E.2 run once, which is not something a flow model can supply, and one measurement would tell you more about L4's gain than any amount of modeling it.
5. **The "set aside" flow is measurable now and isn't.** 37% of the audits drain went somewhere; whether logos and vivarium feed back is a question with an answer in git.

## 5. What would make it useful, in order

Each with what it costs and what would show it was the wrong thing.

1. **Fill A from the trail, once.** `scripts/fsm.py` plus the role tally gives every process-invariant entry; the process-chosen entries are set to the retrospective's rules. Print $\rho$, the station vector, and the actual allocation beside it. Cost: a day, inside maturation phase 2's `vsect flows`. Wrong if the frequencies are so regime-bound that $\rho$ moves by more than its distance from 1 when the window changes; then print the window and treat it as a dashboard number, not a prediction.
2. **The changeset pilot on the Claim Record extraction.** One branch, one intent, one invariant, a mechanical check, vsect emitting one confirm-set at commit. Cost: the work was owed anyway; the vsect half is a `vsect changeset open|commit|abort` or whatever Q17 becomes. Wrong if, done this way, the global lapse count after commit is no smaller than doing it as 24 edits; then the episode boundary was drawn wrong, which is itself what §10's "changeset boundary" question asks.
3. **The L1 experiment.** A round that lands only the findings, then one independent check. It is named in the vsect lead's answer, the flow review, the maturation plan, the retrospective and this model, and has not run. It costs one round. It decides the a_ij the whole model turns on. Modeling further before running it is the "plausibility dressed as verification" shape.
4. **Breaker retrodiction with typed findings.** Classify the scope lane's 36 findings by hand (math / wording / scope), replay the rule, see where it fires and what it would have routed to. Cost: an hour. Wrong if typing doesn't separate the lanes; then the breaker needs a different statistic (edit mass per generation, which `03-loops.md` already argues for).
5. **Add the drain condition and the ownership atom** (§3.2, §4.2) to the model's §5 and §2. Cost: a paragraph. Wrong if, once drainers are plugged in, the stocks fall without it; they won't, because the condition is what "plugged in" means.
6. **Carry cost and gain as separate entries** (§2.1). A rewrite of §5's L2 table.

## 6. Scope honesty

I did not read any segment body, Part I–IV mathematics, the RC1 files other than 07 and 95, vsect's Rust source or its design in `~/src/aat-v3/`, `05-questions.md`, `debrief.md`, or the councils spikes. I did not classify any finding by type; §2.3's claim that the scope lane's flat yield was mostly L3 rests on the retrospective's and check 8's accounts, not on my reading the findings. The gain arithmetic in §3.1 is one day's frequencies with one assumption set by hand, and its purpose is to show the model can be filled, not to be the number. My transitive p90 differs from the lead's (117 vs 123) and I did not chase it. I reasoned about the `(I − A)⁻¹` convention from the model's definition of a_ij and did not check the vsect lead's original. What I am most confident of: the three reproduced measurements, the zero canon commits since 10-31, and the L2 conflation. What I am least confident of: the specific gain figures, and whether the breaker typing is the right fix rather than edit mass.
