# De novo feedback 2: the contention-rounds spike, revision 2

*2026-11-03. Claude (Opus 5.5), independent adversarial pass on the spike as committed at `ac20802`. I am not the spiker, the first verifier, or an integrator. Labels: **measured** = I ran it against `sim/rounds.py` at `ac20802` (unmodified in the working tree), and a minimal reproduction is given; **read** = checked against the cited text in `../../RONR-12/body.md` or the named file; **argued** = my reasoning, to be weighed as reasoning.*

*Read whole: `README.md`, `contention.md`, `response-to-feedback-1.md`, `de-novo-feedback-1.md`, `explorations.md`, `debrief.md`, `proposed-integration-plan.md`, `sim/rounds.py`, `sim/test_rounds.py`, `sim-output.txt`; the sibling `../spike-charter-agenda-2026-11-03/01-what-simultaneity-protects.md` (its uncommitted working copy) and the 9:36 and sealing passages of its `de-novo-feedback-1.md`; `../../initial-thoughts.md` §§"Two criteria" and "The disposition to always act"; the committee configuration in `../../model/council.json`; the sealing passages of the walking skeleton's `debrief.md` and `proposed-integration-plan.md`. RONR paragraphs read: 1:1, 3:32, 4:13, 4:34–35, 4:52, 4:59–60, 9:31, 9:34–36, 12:13, 16:2, 16:5(7), 23:2, 23:5–6, 24:8, 26:2–3, 29:1, 37:10, 40:11, 42:1–22 whole, 43:8–13, 45:6–9, 45:18–20, 45:45–53, 49:21, 50:25.*

## Summary

The repairs from round 1 that I could check did land. Claim age is gone. Name-sorted loops are gone, and the relabeling test passes. The vote-round leak is closed. The snapshot guard is enforced, and the mover-flips-debater case is fixed. F1 is still a real property with a test that has teeth. The architecture (pure resolve, rounds, one floor act per round, the close rule, the claim log) still holds up.

What does not hold is a class of fairness that F1–F4 and I1 do not name. In the toy, a claim that does not win can still decide the outcome, and none of the stated properties or tests can see that. The four measured instances are:

- one member's out-of-order motion claim bars another member's second speech, and the question is then put (§1.1);
- a losing debate claim moves the floor from a debater to a mover (§1.2);
- a misdeclared side wins alternation (§1.2);
- a point of order that is rejected still re-rolls the lottery for the next floor contest (§1.3).

The other main findings:

- Several windows the survival table calls "survives" or "strengthened" fall into vote or ruling rounds, where points of order are barred, so they never open (§1.4).
- The round-1 repair for multiple points in one round did not land as the response describes it (§1.5).
- The single clock door has effects beyond "did not respond": one missed deadline can close debate. That contradicts the coordinator's original latency criterion, which was meant to cover a human member who is slower by hours (§1.6).
- Fairness rests on sealing, which the walking skeleton found to be honor-system (§1.7).
- Under the spike's own default of asking everyone every round, the guards in "policies" do no work. They do work only if policies persist, which the spike relies on in places and disclaims in others (§1.8).
- Two book readings are contestable: 9:36 as "the slot this spike fills", and 45:8 as making vote-changing vacuous. On 9:36 the sibling spike has since gone the other way (§1.9).

Each of these looks repairable without weakening a headline claim, and §3 suggests how.

---

## 1. Findings, most consequential first

### 1.1 One out-of-order floor claim closes debate *(measured; read)*

Ada moved, ada spoke, and ben spoke once. Next round, ben claims a second speech and nobody else wants the floor:

```
control (ben alone):           {'round': 4, 'member': 'ben', 'act': 'speak', ...}
cyd adds ('move', 'main', ..): {'round': 4, 'chair': 'put', 'on': 1}
claim log: ben speak admissible=False; cyd move admissible=False
```

Cyd's claim is a main motion while a question is pending, so it is inadmissible. It costs nothing, and the question is put. Two mechanisms compose:

- `seeking` (`resolve` step 0) counts every floor entry whose guard holds, whether or not the act is admissible. Cyd has not spoken, so ben's second speech is barred by the 43:13 filter.
- `asked_for_debate` is set only by *admissible* speech claims. A speech barred by 43:13 therefore counts as nobody asking, and `put_question_if_exhausted` puts the question.

The book is against each step:

- 43:13 bars a second speech until "every member who desires to speak on it" has spoken. Even on 42:9(2)'s wider "claims the floor", the bar *defers* the second speech "so long as" the other member claims; it does not close debate.
- The chair does not entertain a claim for an out-of-order purpose (42:3's "For what purpose does the member rise?").
- 4:34 puts the question only "if no one then rises to claim the floor". Here ben did rise.

The effect is cloture by one member at zero cost. The previous question needs two-thirds [16:5(7)]. The model's committee configuration, the one the specimen runs, disables it entirely [`council.json` committee overlay, 50:25]. Here one inadmissible claim does what the previous question would do.

Reproduction: `fresh()`; then ada moves main; ada speaks; ben speaks; then `[ben: speak against, cyd: move main]` + passes.

### 1.2 Losing claims steer the floor: the missing property *(measured; argued)*

**A losing debater's claim flips the floor from a debater to a mover.** The state: dee, cyd and eli each moved earlier mains (dee and cyd's were voted down), so last-floor values are dee 1, cyd 4, eli 8, ada 9. On eli's main C, eli spoke *for* and ada *against*. The claims:

```
{ben speak against (never floored), dee move previous-question}        -> ben speaks
{ben speak against, cyd speak for (floored at 4), dee move PQ}          -> dee's previous question
{ben, cyd}                                                             -> cyd speaks
```

Cyd loses in both rounds it enters, yet its entry hands the floor to dee's previous question. The cause is `choose_floor`. It picks the top debater by `debater_key`, where alternation outranks least-recently-floored, and then compares only that debater with the top mover by `neutral_key`. Cyd beats ben on alternation and then loses to dee on recency, whereas ben would have beaten dee.

This is the dual of round 1's §1.4. That repair, ranking debaters first, fixed one direction and opened this one. Any "champion of each group, then compare the champions on a different key" scheme has this property.

**Declared side is cheap talk.** With the same state, honest ben (*against*) and honest cyd (*against*) give ben the floor. If cyd declares *for* while meaning against, cyd gets the floor. The table calls alternation "**survives, strengthened** (declared, not guessed)". But 42:9(3) applies "where the chair *knows*" the opinions. A declaration from a strategic member is weaker than a chair's knowledge, not stronger, and the honest declarer is the one who loses.

**The missing property.** Call it *independence of losing claims*: adding a claim that is not selected does not change which claim is selected, except where the book says it does. 43:13 is the book's one such case, and there the bar favors the blocking claimant itself. F1, F3, the permutation test and the relabeling test are all blind to this by construction. They vary order and names, never the *set* of claims.

Choosing by minimizing one per-claim key over all eligible claimants satisfies this property automatically, because the key does not depend on the other claims. The two-stage scheme does not. §1.1 is the same failure seen from the admissibility side: a claim that should be invisible was not.

### 1.3 A point of order re-rolls the lottery, and the lottery is predictable *(measured; argued)*

`lottery` hashes `(seed, round_no, slot)`. A point of order costs two rounds (the point round and the ruling round), so it moves the floor contest to a later `round_no`.

The probe: ben and cyd are tied (both unspoken, same side, never floored). For 40 seeds, I compared who gets the floor with and without a rejected point by eli in between. **The winner changed for 20 of the 40 seeds.**

The seed is "recorded", and the integration plan plans "a seed per council and per session". If members can see the seed in advance, a member who can compute the lottery can spend a point of order exactly when the tie is about to go the wrong way. A point is always available whenever the previous round had a floor act.

So "a point of order now cannot steal a turn because there is no seniority to steal" (response 1.1) holds for seniority but not for the tie-break. Objection to consideration, and anything else that spends a round, is the same lever.

Repair candidates:

- Key the tie-break by the **number of floor grants so far**, not by round. That is 42:6's own reset unit, "after the floor was yielded". A point of order does not yield the floor (42:20), so it would not re-draw.
- Commit to the seed before a session and reveal it after each draw, so that nobody can compute a draw ahead of time. Do not derive the seed from round content, since members could then grind their policy text to choose it.

### 1.4 Windows that open into a vote or ruling round never open *(measured; read)*

A point of order is admissible only in "the first round after" the transition it names. A vote or ruling round admits only ballots ("voting in progress; no interruption [45:6]"). Any transition whose next round is a vote or ruling round therefore cannot be challenged at all:

- **The put.** It is in `transitions_in_round`, so it looks challengeable, but the next round is always its vote round, and in the round after that it is too late. In the book, 4:34 lets a point against the wording of the put be made "until any member has actually voted". 45:6 bars interruption only "from the time that any member has actually voted". In a sealed simultaneous round that window has zero width. Measured: "voting in progress", then "not one of the previous round's".
- **The announcement.** It is not in `transitions_in_round`, so a point on the conduct of the vote [45:9, 23:5's last sentence] cannot be raised at all. Even so, the table's 45:9 row says "the round after announcement | **survives**" and is not marked "not in toy".
- **A stating that lands in a halted round.** When a second states a motion in the same round as an unrelated point of order, the next round is the ruling round. By the round after that, the stating is "not one of the previous round's". In the book, 23:5 keeps that window open "until debate has begun", and 4:13 keeps it open for the no-debate case "until … any member has voted". Measured.

So the table's "Point of order timeliness [23:5] … **survives, strengthened**" is narrower than the book in exactly these cases. The rule "every transition that opens a window is followed by a round" holds in form, but the following round does not admit the window's acts.

**A further gap.** Besides the seven rows marked "derived; not in toy", these rows also have no support in the sim and are not marked:

- 24:8 appeal timeliness: there is no appeal in the toy, since points go to the council;
- 45:9;
- 42:15, server misassignment;
- 42:14;
- 4:19–23.

The sentence "Seven rows are marked…" is literally true, but it implies that the unmarked rows rest on the sim.

Repair candidate: let a vote round admit a point against the put, or against the stating, with a rule that an admitted point voids the round's ballots. Or insert a "ready" round before the poll. Make the announcement a challengeable transition whose window is the next round.

### 1.5 The round-1 multi-point repair did not land as described *(measured)*

Response 1.3 says: "Two points in one round are both admitted and both ruled on; objection and point both land, the point first [23:2(1)]." `contention.md` §4 step 4 says the same thing. Neither holds.

- **Point and objection in one round: the objection is disposed of first.** Ben raises "motion not in order" against the stated main, and cyd objects to consideration in the same round. The point is applied first (kind order). Then `apply(object-consideration)` sets `phase = "objection-vote"`, and `chair_duties` submits the point only when `phase == "open"`. So the next round votes the objection. The motion is removed, and the point is ruled two rounds later on a motion that no longer exists. In the toy, a sustained point has no effect at all.
- **Two points on the same transition: the roster slot decides which breach is ruled.** Cyd says "outside the council's objects" and eli says "conflicts with an adopted rule", both against transition 0. Both are admitted. `chair_duties` rules `points[0]`, which is the lower slot, because the sort by target is stable after the slot-ordered apply. `announce` then clears every point with that target and adds it to `ruled_targets`. Eli's different breach is never ruled and cannot be re-raised. Swap the two members' slots and the other breach is ruled instead.

  As far as I can see, the toy produces at most one challengeable transition per round. So "two points, both ruled on" never arises in it, and this same-target case is the one that does arise. It is also a third use of the roster slot that F3 does not declare. `test_f3_relabeling` keeps slots fixed, so it cannot see this.

Points are "set-valued" only in the sense that they are a list. An objection is not set-valued, and it interacts with points across kinds.

### 1.6 The clock door does more than "did not respond", and the criterion was weakened *(measured; read; argued)*

`initial-thoughts.md` states the coordinator's criterion this way: "The event log is identical under any permutation of when members' responses arrive. No agent gains or loses because its API was faster or slower … A human member, if one is ever seated, is just the slowest member, by hours, and the criterion covers that too."

The spike's F1 is narrower: identical over the set "received before the close". Its F4 adds a presence deadline "sized in minutes". The spike then says Joseph's constraint "is F1 plus F4". But F4 is the exception to that constraint, and the exception has effects F4's text does not list:

- **One missed deadline can close debate.** Ada moved. Ben, who has not spoken, claims to speak but arrives one position past the cut, and everyone else passes. On time, ben speaks. Late, the record shows `{'chair': 'put'}`. The cause is that `put_question_if_exhausted` treats *silent* exactly as *passed*. The code already separates the two (the `silent` set), but it does not use the distinction. "Silence is exhaustion only after being asked" conflates *asked and declined* with *asked and did not answer*.
- **Two silent rounds kill a motion.** The question is put, then announced 0–0 with all five members `not_voting`, and the result is "rejected". There is no quorum check. That is disclosed as not in the toy, but the cost of omitting it is a lost motion, not a missing annotation.
- A late seconder kills the motion it would have seconded, since "no second" fires the round after the motion is made.

So in exactly the regime the coordinator named — a human member, slower by hours — the design costs that member its debate and its second. Possible repairs, in rough order of how much of the criterion they recover:

- silence extends the round, up to a council-set limit, before counting as a pass;
- exhaustion requires every present member to *pass* explicitly;
- a member may declare "absent for N rounds", which recomputes presence instead of silently forfeiting.

### 1.7 Fairness assumes sealing, which is not yet enforced *(read; argued)*

The walking skeleton's `debrief.md` ("Sealing is honor-system in a shared directory") and its integration plan item 3.4 say that sealed intents are files a member with file tools can read, and that today they stay sealed only because launchers withhold tools.

This spike never mentions enforcing the seal, and neither does its integration plan. Yet F1 takes members' responses as given, so it is blind to any leak. If responses can read earlier arrivals' sealed policies, the record is still a function of the *set*, but the set itself depends on arrival order, and the *slower* member gains information.

So Joseph's constraint needs a third property next to F1 and F4: no response can depend on another response from the same round. That is an enforcement property that lives outside `resolve`. The toy also has a small version of the same leak in §1.8.

### 1.8 Under "ask everyone every round", guards do no work *(argued; one part measured)*

A member is sent `S` at the open of the round. Every guard is evaluated against `S0`, the state at the open of that same round. Late responses are never applied. So a guard can only test something the member already saw when it composed its response; the member could have picked the branch itself. "A stale act is one whose guard failed" cannot happen inside a round.

Guards do work only if a policy outlives its round, and the spike is inconsistent on whether policies do:

- **Policies persist:**
  - §6 on Joseph's example (2): "Standing with their guards; re-asked when no guard covers the new state";
  - §8 and the 42:5 row: "a policy can carry both acts; the server will fire the second when the member next wins the floor";
  - the guarded-routing half of `test_i1_implicit_guard`, which works because the test resubmits the same policy next round. The test simulates persistence.
- **Policies do not persist:**
  - §4: "at most one floor act" per policy, and claims are per round;
  - F2's default of asking everyone;
  - the toy, which keeps nothing between rounds;
  - the integration plan's "Default 'ask everyone'".

What the prioritized list does do in-round is fall back when admissibility depends on *other members' claims*. In practice that means the 43:13 bar, which is the machinery behind §1.1.

So the spike's answer to Joseph's hunch ("members answer with policies") has force only in the persistent or standing regime, which F2 advises against as a default. The spike should say which regime policies are for. If they are for standing use, it should also say what a standing policy evaluated at a state the member never saw does to I1 and to the breach window.

**Measured, related.** §4 permits guards over "the structural effects of acts admitted this round", but the code and §4's own second rule allow only `S0`. And in the toy, a guard can read sealed in-round information. `s0.claims_this_round` is empty when guards run in step 0 and holds the round's floor claimants when they run in step 1. A guard `"ben" in st.claims_this_round` is false for the 43:13 `seeking` set and true for the live claim, so "I claim if ben claims" is expressible. §6 says that kind of expression must not be, and the `seeking` set and the live floor set then disagree.

The F1 and F3 property tests generate only unguarded policies (`random_policy`), so guards, the centerpiece, are never property-tested.

### 1.9 Two book readings *(read)*

**9:36.** Revision 2 adds that "the book's own list of what electronic-meeting rules must supply includes 'methods for seeking recognition and obtaining the floor' [9:36] … the book names the slot this spike fills." It also says the race is "something the book expects". But:

- 9:36 opens "If an organization authorizes … electronic meetings", and 9:31 defines those by the simultaneous-aural standard that 9:34 says written deliberation lacks.
- 42:16 is likewise a device for an aural hall.

So 9:36 and 42:16 are analogies the book offers for replacing the race inside an aural assembly. They are not its expectation for a written one. The sibling spike's verifier refuted exactly this reading of 9:36 (its `de-novo-feedback-1.md` §1). The sibling's working copy of `01-…` (uncommitted, apparently mid-revision in another session) now says "9:36's list … governs meetings that already meet the aural standard" and frames the council as "a principled departure from 9:34's letter".

So the README's "the sibling reached the same foundation" is no longer true on this point. The honest line is probably the sibling's: the council is outside the book's letter, the book's devices are precedent rather than license, and the charter is the authority.

**45:8, "vacuous by the book's own exception".** The exception covers "the ballot (or some other method that provides secrecy)". The council's vote is attributed by name in the record, so it is a roll call (or a signed ballot, 45:46), which has "exactly the opposite effect of a ballot vote" [45:45]. A roll call explicitly allows changes "before the result is announced" [45:50]. So the right exists, and the sealed round removes it by design. The row should read "dropped, priced" (the price: an agent that mis-submits has no correction before announcement), not "vacuous".

The sibling's working copy already treats it that way ("keeps the right in form and empties its trigger … [dropped, declared]"). I am contesting a reading that round 1's verifier suggested and the spiker accepted "gladly". The do-not-inherit list was right to flag it for testing.

A related gap: under "every vote attributed" there is no way to hold a secret ballot. Bylaws may require one, and the requirement "cannot be suspended—even by a unanimous vote" [45:20]. That deserves a row, or a sentence in §10.

### 1.10 Smaller points *(measured where marked; otherwise read)*

- **The no-go is stated more broadly than it holds.** "Any mechanism in which a member's act lands between two other members' acts within a round decides something by arrival order." But step 4 of the spike's own resolver lands acts between others within a round, ordered by kind. What actually rules out mid-speech interruption is *informedness* (and with it reading speed, i.e. latency): an interrupt can react only to what its sender had seen. That narrower statement is the true one.
- **Obstruction is bounded per transition, not in total, and it costs the council, not the obstructor.** On a move with four speeches, an obstructor raising a point against every transition took the session from 7 rounds to 17 (measured). "Halting the floor costs the obstructor one ruling round" should read: it costs everyone two rounds, and the obstructor nothing. Latency is the second obstruction channel: answering at deadline-ε every round is never "silent", is never dilatory, and costs a full deadline per round. The point-of-order bound does not touch it.
- **Duplicate submissions: the last arrival wins** (measured). `collect_and_resolve` builds the profile as a dict in arrival order, so two submissions from one member produce `['second text']` or `['first text']` depending on order. That is the replaceable-submission failure the walking skeleton measured, inside the toy's own collect layer. The prose says "final", and the F1 test never generates duplicates.
- **A policy with two ballots counts the *lowest*-priority one** (measured: ben `[aye, no]` is tallied `no`).
- **"Least-recently-given-the-floor favors whoever has had the floor *least*"** (§6). Least recently is not least often. A member who spoke ten times long ago beats one who spoke once recently.
- **`last_side` is global.** Alternation carries over from an amendment's debate to the main question once the amendment is disposed of.
- **The "committee configuration" specimen applies a two-speech cap and the 43:13 bar.** 49:21(3)/50:25 remove speech limits in committees. `initial-thoughts.md` leans toward keeping the counters "as rate limits" and recording that as a divergence, and the spike does not record it. The toy also cites the two-speech cap as "[43:8]" (ten-minute speeches); the rule is 43:12.
- **The debrief says that "each of these is a property the toy has and a test checks"** and lists F2 among them. F2 has no test ("derived; the toy never skips anyone"). Its "Always claiming gains nothing" is true of seniority. §§1.1–1.2 show that claiming in order to steer does gain something.
- **Prevailing-side eligibility "strengthened".** The sibling's working copy notes that the model's `mover-was-prevailing-side` guard has no committee branch [37:35(2)] (its F12). So enforceability depends on a model fix, and the row should say so.
- **Roster slot assignment is unspecified.** F3 is clean only if slots are not themselves assigned by name or by join order. The relabeling test holds slots fixed, so it cannot tell.

## 2. What held when I checked

- **Round-1 repairs:**
  - claim age is gone, with retractions in the body;
  - the name-sorted channels are gone, and `test_f3_relabeling` passes;
  - the vote leak is closed: no ballot content appears in the claim log, and a late ballot is never counted;
  - the snapshot guard is enforced, and by reading the code I1 holds in the toy, since only non-floor acts precede the floor act and `snapshot_key` covers what they change;
  - the mover-cannot-flip-debater direction holds;
  - early close is removed.

  The re-run output is byte-identical to `sim-output.txt`.
- **F1** is real for unguarded, non-duplicate profiles.
- **One floor act per round** is right, for the reason given now (fresh responders were never asked at the intermediate state).
- **"Precedence among pending motions is untouched"** is right.
- **Quotations I checked are accurate:** 1:1, 3:32, 9:31, 9:34, 42:5–22, 43:13, 45:6, 45:8, 45:9, 23:5, 12:13. The problems in §1.9 are with the *inferences* drawn from 9:36 and 45:8, not with the quotations.
- **The fragility narrowing (§5.1) is honest.** It is a diff tool, its parameter is declared, and it does no ordering work.

## 3. What I would try before softening anything

1. **Add independence of losing claims as a property and test it.** Over random states, adding a claim that is not selected leaves the selection unchanged, except for the 43:13 deferral in favor of an unspoken debater. A single per-claim key over all eligible claimants satisfies it by construction. Count only *admissible* claims as `seeking`, and count a speech barred by 43:13 as "asked for debate", so that the bar defers and does not close (§§1.1–1.2).
2. **Key the tie-break by floor-grant count, not by round, and commit to the seed and reveal it after each draw** (§1.3).
3. **Decide what side declarations are worth.** Options: make alternation advisory; check a declared side after the fact, which needs a judgment predicate the server can route; or drop the "strengthened" label (§1.2).
4. **Give the put, the stating and the announcement real windows**, through a points-void-ballots rule or a pre-poll round, and test each one (§1.4).
5. **Fix point/objection ordering and same-target points.** Rule every distinct breach, or merge them explicitly, and keep slots out of which text gets ruled (§1.5).
6. **Distinguish silent from passed, and restate F4's effects in full.** Then say plainly whether the coordinator's "slower by hours" criterion is met, met within a declared bound, or given up (§1.6).
7. **Name sealing as a property next to F1 and F4,** with the walking skeleton's enforcement requirement carried into the integration plan (§1.7).
8. **Decide whether policies persist.** If they do, work through I1 and breach windows under standing policies. If they do not, rewrite §6 example (2), the 42:5 row and the "why policies" paragraph. Either way, property-test guarded policies, and do not expose `claims_this_round` to guards (§1.8).
9. **Re-ground §1 on the sibling's departure framing,** and recast the 45:8 row as dropped and priced (§1.9).

## 4. Scope, so the silence is not misread

I did not read the RONR-12 model beyond the committee overlay in `council.json`. I also did not read `mechanics.md`, the walking skeleton's code, or `../../RONR-12/` beyond the paragraphs listed at the top.

I did not re-run round 1's probes except where the new tests encode them. I did not audit `explorations.md`'s guesses.

The probes above are short. Each builds a state with `collect_and_resolve` from `test_rounds.fresh()`/`pol()`/`passes()` and reads `record`, `points`, `vote` or `claim_log`; the setups are given inline.

This report's content is its list of findings. That I did not raise something is not evidence that it is sound.

I'm available for follow-up if the spiker or Joseph wants a probe re-run against a revision.
