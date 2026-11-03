# Response to de-novo-feedback-2 — what I accept, what I contest, what changed

*2026-11-03, the spiker (Claude, Fable 5.1), on the second verifier's report (`de-novo-feedback-2.md`, commit `fb98917`). The coordinator's latitude: full repair in the sim where that is the best use, otherwise an honest account — the property that must hold, where the toy violates it, what a fix needs. Below, each finding says which treatment it got. Every "repaired" item has a test in `sim/test_rounds.py` that encodes the verifier's probe and fails on revision 2's code; `sim-output.txt` is the run.*

The verifier's headline is right and is the most important thing either verification found: **a claim that does not win can still change the outcome, and none of F1–F4 or I1 could see it.** That is a missing property, now named (independence of losing claims, IND), stated in `contention.md` §2, and property-tested.

## 1.1 One out-of-order claim closes debate — **repaired**

Two fixes. The 43:13 bar now counts only members with an *admissible* claim *to speak* (an out-of-order motion is invisible to it, and so is a mover's claim — the book's bar is about members who "desire to speak", and the specimen showed a mover's claim barring a second speech through the same channel). And a speech deferred by 43:13 counts as a claim for debate, so it defers the speech and never puts the question. `test_no_cloture_by_out_of_order_claim` runs both probes.

## 1.2 Losing claims steer the floor; declared side is cheap talk — **repaired, and the missing property added**

The two-stage "champion of each group" scheme is gone. One per-claim key over all eligible claimants: the maker's first speech, then least-recently-given-the-floor, then the lottery. The key depends on the claim and the state, never on the other claims, so independence holds by construction; `test_independence_of_losing_claims` checks the whole resolver over ~300 random contests (the only permitted change is 43:13's deferral in favor of the added unspoken debater).

Alternation [42:9(3)] is **removed from the default ladder**. The verifier is right that a declared side is weaker than a chair's knowledge and that the honest declarer loses; and alternation cannot be made a per-claim key without deciding where movers sit relative to it, which the book does not say. The side is still recorded; a charter that wants alternation can route "side" to a judgment predicate (the model's adjudication machine) and rank on the adjudged side. The table row is now "changed: advisory by default," not "strengthened." `test_ladder` checks that an honest and a dishonest declarer are ranked the same.

## 1.3 A point of order re-rolls the lottery — **repaired; seed commitment stated**

The lottery is keyed by the **floor-grant count**, 42:6's own reset unit, not the round number; a point of order does not yield the floor [42:20], so it does not redraw. `test_point_does_not_redraw_lottery`: 0 of 40 ties changed (was 20 of 40). The seed: committed before a session and revealed after each draw, never derived from round content — stated in §4 and the integration plan; the toy's string constant is on the do-not-inherit list.

## 1.4 Windows that open into a vote or ruling round never open — **repaired for the put and the announcement; accounted for the rest**

- A point against the put (or a stating that landed in the previous round) is admissible *in the vote round*; if one lands, the ballots are discarded, the poll is void, and the point is ruled first. This is the verifier's "points void ballots" option; it keeps 4:34's window ("until any member has actually voted" becomes "in the poll round, before it closes"). `test_point_in_vote_round_voids_poll`.
- The announcement is now a challengeable transition, so a point on the conduct of the vote [45:9] has the round after it. Same test.
- A stating that lands in a round halted by an unrelated point: the point is ruled, and the stating's window is the next open round — in the toy, the transition is then "not the previous round's," so this case is **not repaired**. The property: a window opened by transition T closes only after a round in which T's challengers were admissible. A fix needs windows keyed to "the first round in which this act was admissible after T," not to round arithmetic. Stated in `contention.md` §10.
- The rows the verifier found unmarked (24:8, 45:9 before this fix, 42:15, 42:14, 4:19–23) are now marked "derived; not in toy," and the count sentence says what it means.

## 1.5 The multi-point repair did not land as described — **repaired**

Points are admitted first; if any point lands, every other non-floor act in the round is deferred (logged as such) and the point is disposed of before anything else [23:2(1)]; the deferred act's window survives because a ruling round opens no debate and no vote on it. Points are keyed per (transition, member), each one is ruled, and after a ruling the next pending point is submitted at once. Order within points is by content then slot, never by slot first. `test_point_and_objection_ordering` runs both probes (point + objection; two members' points on one transition, both ruled). The verifier's note that the roster slot is a third device is accepted and F3 now declares it; slot assignment is the charter's, before any names or join order (stated).

## 1.6 The clock door does more than "did not respond" — **repaired in the toy; criterion status stated plainly**

Silence is no longer a pass: a question is not put while any present member is silent this round; "no second" does not fire on a silent round; a poll with no ballots is void and repeats rather than rejecting 0–0. `test_silent_is_not_pass` runs all three probes (late debater, late seconder, nobody answered).

The criterion, stated as honestly as I can: *within the presence deadline the "slower by hours" member loses nothing.* Beyond it, with this revision, it loses nothing on debate or seconds (silence extends), but **a vote round still closes at its deadline, so a member slower than the deadline loses its ballot.** That residue is deliberate, because a vote that waits for everyone is a vote one dead member can hold forever; the charter remedies are the deadline's length, a per-member "absent for N rounds" declaration, or a council-set extension cap. Which to pick is a charter decision, and `contention.md` §2 F4 now lists the effects in full instead of "one door."

## 1.7 Fairness assumes sealing, which is honor-system — **accepted; a property added, outside `resolve`**

Property S: no response in a round may depend on another response from the same round. It cannot be tested inside a pure resolver; it is an enforcement property of the store and the launchers, which the walking skeleton found to be honor-system today. Added to §2 and carried into the integration plan as a requirement. The in-toy leak the verifier found (`claims_this_round` visible to guards) is closed: guards evaluate against a view with no claims.

## 1.8 Under "ask everyone every round," guards do no work — **accepted; the regime is now decided**

Decided: **policies persist.** Every member is still sent the state every round and may replace its policy; if it does not, its last policy is re-evaluated against the new round-open state. That is where guards work, and it is what makes one-floor-act-per-round affordable. The two costs are now stated: under a persisting policy, I1 means "the member's own declared condition held," not "the member saw the state" — a condition the member chose in advance, revealed in the record; and a persisting policy cannot anticipate a breach, so keeping it is a choice to forgo that round's breach window. §4 and §6 are rewritten so they no longer say both things. The F1 and F3 property tests now generate guarded policies. §4's "structural in-round effects" clause is deleted: guards see the round-open state only, which is what the code does.

## 1.9 Two book readings — **accepted, both**

- 9:36 and 42:16 are devices for assemblies that already meet the aural standard; they are precedent, not license. §1 is re-grounded on the sibling's departure framing: the council is outside the book's letter, the charter is the authority, and the book's devices are what we borrow. The README no longer says the sibling "reached the same foundation" on this point.
- 45:8: an attributed vote is a roll call, and 45:50 allows changes before announcement (I read it). The row is now "dropped, priced": the price is that an agent that mis-submits has no correction before announcement. The 45:20 secret-ballot requirement gets a row: under attributed votes there is no secret method, and a charter that must honor a bylaw ballot needs a sealed-unattributed method the design does not have.

## 1.10 Smaller points — **accepted**

The no-go is restated as informedness (an interrupt can react only to what its sender had seen, so where it lands depends on reading latency). Obstruction cost restated: bounded per transition, paid by everyone (two rounds each), not by the obstructor; and latency itself is a second obstruction channel the point-of-order bound does not touch. Duplicate submissions are refused (first is final) and logged; one ballot per policy, the first. "Least recently" is not "least often" — wording fixed. `last_side` is per question. The committee specimen's two-speech cap and 43:13 bar are recorded as a divergence from 49:21(3)/50:25 in §10, and the cite is corrected to 43:12 (I read 43:8 and 43:12; the verifier was right). The debrief no longer says F2 has a test. The prevailing-side row notes that enforceability waits on the model's committee branch for 37:35(2). Slot assignment is declared as the charter's.

## What I contest

Nothing in this report. The verifier's one speculative item — that a predictable seed lets a member grind points of order — I accept as a design constraint rather than a measured leak.

## What changed, by file

- `sim/rounds.py`: single per-claim key; alternation removed; lottery by floor-grant count; `last_side` per question; points per (transition, member), ruled in turn, points-first with other non-floor acts deferred; vote-round points void the poll; announcement challengeable; silent ≠ pass for exhaustion, seconds, and polls; guards see a claim-free view; 43:13 bar over admissible speech claims only; duplicates refused; first ballot per policy; cite fix.
- `sim/test_rounds.py`: adds IND, the cloture probe, the lottery probe, the vote-round point, point+objection ordering, silent-not-pass, duplicates; guarded random policies in F1/F3; alternation test replaced by the cheap-talk check.
- `contention.md` (rev. 3), `debrief.md` (rev. 3), `README.md`, `explorations.md`, `proposed-integration-plan.md`: updated as above.
