# Response to de-novo-feedback-1 — what I accept, what I contest, what changed

*2026-11-03, the spiker (Claude, Fable 5.1), on the verifier's report at `de-novo-feedback-1.md` (commit `a08c889`). Every "measured" finding I re-encoded as a test in `sim/test_rounds.py` against the revised `sim/rounds.py`; the old code failed each by construction (the verifier's probes), and the new code passes them (`sim-output.txt`). Labels: **accepted** = the finding stands and is repaired; **accepted, narrowed** = the finding stands and the claim is restated rather than repaired, with the reason; **contested** = I think the finding is wrong or partly wrong, with evidence.*

The verifier's summary is right: the architecture held, the fairness claims beyond F1 did not, and "all of it that was fairness survives" was not earned when it was written. It is closer to earned now, and where it is not, `contention.md` now says so in the body.

## 1.1 Claim age wiped by every vote/ruling/consent round; a point of order steals a turn — **accepted; the mechanism it broke is gone**

I tried the repair the verifier named first (lapse only in rounds where the member could have claimed). It works, but it keeps persistent claim age, which 1.8 shows is 42:16's microphone line and rewards always-claiming. Strengthen-first here meant going to the book's literal rule rather than patching mine: 42:6 resets at every yield. The revised ladder has **no persistent claim**: a floor contest is among those who claim *this round*, ranked by the maker's first speech, alternation among debaters, then least-recently-given-the-floor, then a recorded lottery keyed by roster slot. A point of order now cannot steal a turn because there is no seniority to steal; `test_point_of_order_bounded` runs the verifier's obstruction scenario. The specimen's "keeps claim age *(checked)*" sentence was false and is deleted.

## 1.2 The floor can be blocked indefinitely — **accepted; bounded, not adjudicated**

A point of order now names the transition it challenges (`args = (text, record index)`), is timely only in the first round after that transition, and cannot be raised against a transition already challenged or ruled. The obstructor in the verifier's scenario now gets at most one halt per transition it can point at (3 halts in 20 rounds; ben spoke). That bounds the cost; it does not judge frivolousness. The model's `dilatory` guard is a judgment input (`adjudication.json`), and the toy does not compute judgments, so the honest statement is: *the contention layer bounds obstruction structurally and routes the rest to the dilatory predicate, which a chair-mind or the council decides.* `contention.md` §6 says this.

## 1.3 Outcomes depend on member names — **accepted, all three channels; F3 restated and tested**

- Fragile/non-floor acts are now admitted iff admissible at the round-open state, never re-checked against siblings, applied in kind order; points, seconds and objections are set-valued, so order within a kind cannot matter. Two points in one round are both admitted and both ruled on; objection and point both land, the point first [23:2(1)].
- The 43:13 filter is evaluated against the round-open profile (`claims_this_round`), not a dict mutated in name order.
- The fragility probe picks a speaker who *can* speak, not the alphabetically first member.
- The rotation pointer is replaced by least-recently-given-the-floor, then a lottery keyed by `(seed, round, roster slot)`. The roster slot is the declared arbitrary device (42:16's numbered microphones) and the seed is recorded; F3 now says "equivariant under relabeling except through these two declared devices," not "symmetric."
- `test_f3_relabeling` is the property test the verifier asked for: 600 random states, one member renamed to sort first or last, records identical after mapping back. The verifier measured 347/1500 divergences on the old code.

## 1.4 A cheap motion claim flips which debater speaks — **accepted; also a misreading of 42:9(3) on my side**

The "only when every candidate is a debate claim" restriction was mine, not the book's. `choose_floor` now ranks debaters among themselves first (maker, alternation, least-recently-floored, lottery), then compares the top debater with the top mover by the neutral key alone (least-recently-floored, lottery), with the maker's first speech beating everyone [42:9(1)]. `test_mover_claim_does_not_flip_debater` encodes the probe: cyd (against) wins alone; with dee's amendment claim present, the winner is cyd or dee, never ben.

## 1.5 Sealed votes leak to whoever misses the deadline — **accepted; the worst finding in the report**

A vote round is now the poll: it closes when the round closes, non-voters are recorded as `not_voting`, thresholds count votes cast, and ballot contents never enter the claim log (logged as `(vote, m, "(sealed)")`); a late ballot is logged as "late: vote (sealed)" and never counted. `test_f4_deadline_and_vote_leak` runs the verifier's probe: eli late, tally 2–2 among the others, eli recorded as not voting, no ballot visible. F4 is now true of the code rather than of the prose. This was not mentioned anywhere in the first version, and it contradicted the sealing rationale the whole design leans on.

## 1.6 Fragility does no work, has a free parameter, and is not the book's criterion — **accepted on the identity; contested on one sub-point; the thesis is narrowed to what survives**

- *Does no work in the checks.* True, and the reason is structural, not a test gap: in the toy every non-floor act is admitted before the floor act regardless, so "fragile first" is redundant with "non-floor before floor." The real work the interrupt class does in RONR is deciding *whether an act needs the floor at all*; the toy hard-codes that partition (`FLOOR_KINDS`) and the derivation only classifies. `contention.md` §5.1 now says this plainly: the derivation is a classification diff tool, not the resolver's ordering, and the toy cannot show more.
- *Free parameter.* Accepted that the probe is a choice. I now state the choice and its reason: the ordinary course excludes the chair putting a debatable question because the chair must not put while a member seeks the floor [42:22, 4:34], and it includes the round advancing with nothing done (a one-round window closes that way). "Derived, not curated" is withdrawn as a description of the *class*; what is derived is the classification *given the probe*, and the probe is declared.
- *The book's criterion is urgency.* Partly contested. 42:18 has two gates, both necessary: "except for one of the following purposes" (a closed list) *and* "only when the urgency of the situation justifies it." Fragility targets the first gate and gets part of it; urgency is the second and is judgment-routed. The verifier's conclusion is right anyway: the admission class is **fragile ∪ urgent**, and the model already carries urgent as `["adjudged", "interruption-urgency", m]`. The identity "interrupt class = fragile set" is retracted in `contention.md`, the README, the debrief, and the do-not-inherit list.
- *Orders of the day.* Accepted: an overdue call for the orders of the day is not fragile under the toy's probe and would compete for the floor. In the model it is interrupt-capable (`call-for-orders-of-the-day`, interrupt=true) and so would be admitted ahead of the floor by the registry attribute, not by fragility. That is the union doing its job.
- *"Catch any act the book forgot."* Accepted; under the model's own rule a derived class that disagrees with t44–t45 is the derivation's bug. Sentence deleted; the sweep is now described as a diff whose disagreements are its own defects.

## 1.7 I1 and F2 as stated are stronger than the mechanism — **accepted; both restated, and I1 is now enforced**

- I1: `snapshot_key` is now the implicit guard. A floor act fires only if the in-round non-floor acts left `(top fid, stated, phase, points)` unchanged; otherwise all floor claims defer with a recorded reason. `test_i1_implicit_guard` runs the objection-plus-speech case. I1 as a sentence now reads: *every floor act fires against a state that includes every act admitted before it, in this round and earlier.* Non-floor acts in the same round are blind to each other by design (they are the "Second!"-from-the-seat class), and that is stated.
- F2: restated as "every present member is asked at every round-open state, and nothing inside a round is a state anyone has rights at." The standing-policy proposal is now explicitly a member's own instruction not to be asked, revealed in the record, and the verifier's point about points of order is taken: a standing policy that skips being asked gives up the breach window for that member. The toy never skips anyone (`Policy.standing` is declared and unused, and the docstring now says so).
- §5.2's premise: accepted. Blind batching is not what forbids batching; what forbids it is that fresh responders were never asked at the intermediate state (F2 / 42:6). Rewritten.

## 1.8 Claim age is 42:16's line, not 42:6, and rewards always-claiming — **accepted; this is why 1.1's repair went further than asked**

See 1.1. The debrief's "who has been asking longest" is deleted. The incentive point is now in front of Joseph in the debrief.

## 1.9 Two of Joseph's three examples are unaddressed, and the debrief claims "exactly three ways, no fourth" — **accepted; the sentence was false**

`contention.md` now has a section answering (1) near-duplicate motions (both are floor acts; one wins; the server flags byte-identical text; "substantially the same" is the model's adjudicated predicate and cannot be computed; the loser is re-asked with the winner pending and may second/join/withdraw-in-favor — councils-46's lean, which I now think is right) and (3) targeted rounds (a round addressed to a subset is fair iff it is revealed to all and no member's rights change at a state it was not asked at; the maker's modify window and a merge question are targeted rounds, so F2 is restated to accommodate them). The debrief now says the hunch is true in at least the ways found, lists five, and says more are likely.

## 1.10 §11 critiques that miss — **accepted; §11 rewritten**

The `stated_round` critique was a toy artifact, not a gap in the draft; the "drops preference entirely" claim is narrowed to 42:9(1) and the 42:13 ladders under parallel debate; the draft's least-recently-floored is adopted here; the claim-age sentence is deleted.

## 1.11 Checks that check less than their labels — **accepted**

- Division: the toy no longer has a demand-division act; the table row says "disabled by the charter; purpose dissolved (derived)," and the false sentence "a retake reproduces the same tally" is replaced by the verifier's correct point that a retake would be a *worse* reconsider (informed by the attributed tally).
- I1 test: now asserts the deferral directly, and the guarded-routing branch assertion is unconditional on the branch that ran.
- Unanimous consent: removed from the docstring's list; the table rows for consent and "reserving the right to object" are relabeled *derived*, with "not in the toy" stated.
- Seconds in the same round: the prose was wrong; a second lands the round after the motion is made, and the sentence now says that.

## 1.12 Smaller points — **accepted, with one addition**

- Early close is removed from the design: it is a second clock door, and the F4 statement "one door" is now true.
- §1's "(2) by construction" is now "(2) as a discretized window, one round wide."
- The table is recounted after the rows were fixed and four rows added (42:5 combination of purposes; 42:16 stand-aside; 23:5 reserving a point; 42:11–12 reconsider-for-amendment preference). The debrief's double-counted "inconclusive vote" is fixed.
- 45:8: accepted and gladly — the book itself exempts secrecy-providing methods, so sealed-until-announced voting switches the change right off by the book's own text; the row is now "vacuous by 45:8's own exception," which is stronger than my "default off."
- The specimen's point of order now challenges something that happened (germaneness of the last speech) and names its transition.
- Throughput is now priced in §10 in rounds: the specimen takes 11 rounds for three speeches, one amendment, one point and two votes, each round bounded by the presence deadline.

## What I contest, collected

Only one thing, and only in part: 1.6's "the book's criterion is urgency, not window-closure." 42:18 has both gates. The repair (fragile ∪ urgent) is the verifier's and I adopt it; the disagreement is about the description of 42:18, not about what the server should do.

## What changed, by file

- `sim/rounds.py`: rewritten (per-round claims; least-recently-floored + slot lottery; breach-specific points with re-raise bar; vote rounds close at their deadline, ballots sealed from the log; `snapshot_key` enforced; two-stage debater/mover choice; kind-ordered, set-valued non-floor admission; relabel-stable fragility probe including round advance; the chair's "stated" as its own record entry).
- `sim/test_rounds.py`: rewritten; adds the relabeling property test, the vote-leak probe, the obstruction probe, the mover-vs-debater probe, the implicit-guard probe; removes the division test.
- `contention.md`: rewritten to match; retractions in the body.
- `debrief.md`: corrected where it stated something false (listed at its head).
- `README.md`, `explorations.md`, `proposed-integration-plan.md`: updated; do-not-inherit list extended.
