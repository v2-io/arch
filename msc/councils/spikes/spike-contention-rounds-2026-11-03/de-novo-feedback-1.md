# De novo feedback 1: the contention-rounds spike

*2026-11-03. Claude (Opus 5.5), independent adversarial pass, not the spiker, verifier, or integrator. I read every file in this spike whole, along with `sim/` (I ran it and then probed it), `../../mechanics.md`, `../../initial-thoughts.md`, the model's `README.md`, `guards.json` and `protocol.json` (the recognition and interrupt parts), the sibling `../spike-charter-agenda-2026-11-03/01-what-simultaneity-protects.md`, the walking skeleton's `debrief.md` and the `selftest` in `steward.py`, and these paragraphs of `../../RONR-12/body.md`: 1:1, 3:31–34, 4:9–23, 4:50–60, 9:30–36, 23:5, 26:2, 26:7, 29:1, 37:10, 40:11–12, 42:2–22, 43:8, 43:13, 45:6–9. Labels: **measured** = I ran it against `sim/rounds.py` as committed (61fb665); **read** = checked against the cited text; **argued** = my reasoning, which you should weigh as reasoning.*

## Summary

The spike's architecture holds up under attack: pure `resolve`, sealed policies with guards, one floor act per round, the close condition taken from the walking skeleton, and a claim log that keeps the losing claims. The RONR quotations I checked are accurate, 9:34 and 42:6/42:16–17 included. The F1 test has teeth: when I changed `resolve` to iterate the profile in arrival order, it caught the change on its first trial.

The spike's fairness claims beyond F1 do not hold for the toy as written, and some of the readings it calls derived do not survive the text:

- Claim age does not survive any vote, ruling, or consent round. A frivolous point of order resets the queue and hands the turn to a newcomer. This is measured, and the spike's own specimen output shows it.
- One member can block the floor indefinitely with points of order (measured).
- Outcomes depend on member *names* in about a quarter of random runs (measured), which violates F3 in a way the arrival-permutation test cannot see.
- A third member's throwaway motion claim can flip which debater speaks (measured).
- A member who misses a vote round's deadline then votes knowing the partial tally (measured).
- The fragility derivation does no work in any of the sim's checks. Its verdict on amend and speak depends on a probe definition that leaves out the resolver's own "ordinary course". The book's criterion for interruption is urgency, not window-closure [42:18], and the model already routes conditional interrupts to judgment.
- The debrief tells Joseph his hunch is true "in exactly three ways, and I could find no fourth", but two of the three examples Joseph himself gave are not addressed.

Most of these can be repaired, and §4 says how. The headline "all of it that was fairness survives" is not yet earned.

---

## 1. Findings, most consequential first

### 1.1 Claim age is wiped by every vote, ruling, or consent round, so a point of order can steal a turn *(measured)*

`resolve` drops the claim of every member that has no live exclusive act this round (`rounds.py` 498–501). In the vote, ruling, objection-vote and consent phases only votes, objections, or passes are admissible, so nobody can hold a live exclusive act. Every claim is erased.

- **The specimen already shows it.** Dee claimed in round 4 and again in round 5, lost the floor to the point of order, and then appears in round 7 with `'note': 'floor: 1/0/7/0'`. Claim age 7 is the current round, so the claim was reset. `contention.md` §6 says of this exact case, "the motion's maker keeps claim age … *(checked in `specimen()`)*". §11 says the same thing to councils-46 ("or the point becomes a way to steal a turn"), and so does integration item 5.
- **The steal.** In my scenario, ada moved and spoke, then ben and dee contended and ben won. That left dee as the older claimant (age 3). Next round dee and newcomer cyd both claim:
  - with no point raised, dee gets the floor (`floor: 1/0/3/1`);
  - if cyd adds a frivolous point of order to its policy, the point halts the floor, the ruling vote wipes `claims` to `{}`, and the next round cyd gets the floor (`floor: 1/0/6/0`).

What a repair would need: a claim should lapse only in a round where the member *could* have claimed and did not. Equivalently, freeze claim ages across rounds in which no floor act was admissible to anyone.

### 1.2 The floor can be blocked indefinitely *(measured)*

A point of order halts the floor (step 5). After the ruling, the announcement is the "previous round's" record entry, so a fresh point is timely again (`inadmissible_reason`, 162–166). In my run eli raised a point every open round while ben claimed to speak. After 20 rounds there were 0 speeches and 10 points, and ben's claim age was wiped each time (1.1). Each halt costs the obstructor nothing.

RONR's remedy is the chair's judgment that a motion is dilatory (§39). The model carries it as `["not", ["guard", "dilatory"]]` in `actions.json`. The spike's contention layer has no place to apply it. "Halt the floor" has no limit and no adjudication path, and the point's timeliness predicate cannot tell a new breach from a re-raised one.

### 1.3 Outcomes depend on member names, which violates F3 *(measured)*

F3 says ties are broken "by a rule symmetric across members". I ran 1,500 random walks of up to 8 rounds. In each walk I renamed one member (`ada → zed`) while keeping its roster slot, so only the name's sort position changed, then compared each round's record entries as a multiset with names mapped back. 347 of the 1,500 walks diverged. After also stripping attribution, speech arguments, and tallies, 18 still differed. The first of those differs only in whether a point of order was classified fragile, so the derived class itself moved with the rename. Three mechanisms cause this:

- **Fragile acts are applied in `Act.key` order (kind, then member name) and re-checked after each one** (511–516). Fragility is never tested for commutation among fragile acts.
  - Two members raised different points of order in the same round. Only the first by name was admitted. The second was logged `admissible: True` and then never appears in the record.
  - When an objection to consideration and a point of order land in the same round, the objection wins because `"object-consideration" < "point-of-order"` alphabetically. The point (which may challenge the very motion being objected to) is dropped.
  - The sibling spike's reading (`01-…` row a) is that any point submitted in the first round addressed to a state is "at the time of the breach". Re-checking against a sibling's effect contradicts that reading.
- **The 43:13 filter is evaluated against a `claims` dict that is mutated during the loop, in name order** (479–496). Take a member who has spoken once and has policy `[second speech, else move amend]`:
  - if the new unspoken claimant's name sorts *before* it, its speech is logged inadmissible and its amendment goes live;
  - if the name sorts *after*, the speech is logged admissible, the amendment is skipped, and the speech is then filtered out in step 6.

  The claim log disagrees between the two cases, and so does the set of competing acts.
- **`is_fragile` uses the alphabetically first present member as the probe speaker** (389, 403–404). If that member has used up its speeches, the speech probe is skipped and the verdict changes.

The **rotation pointer** is also not symmetric under relabeling: "next after the last winner" in roster order systematically favors the roster successor of frequent winners, such as makers who win on 42:9(1). 42:16's numbered microphones show the book accepts a fixed arbitrary order, so this may be acceptable. If so, F3 should say that, instead of "symmetric".

The F1 test permutes arrival and then `resolve` sorts by name, so the test is blind to all of this by construction. What would catch it: a **relabeling-equivariance property test** beside the permutation test, and a canonical order inside the round that is not keyed by name.

### 1.4 A cheap motion claim decides which debater speaks *(measured; read)*

Alternation applies "only when every candidate is a debate claim" (`preference_tuple`, step 6). After ada (maker) spoke *for*, ben (*for*, older claim) and cyd (*against*) claim:

- alone, cyd gets the floor by alternation;
- if dee also submits a claim to move an amendment, alternation switches off, claim age decides, and ben gets the floor.

Dee does not win; its claim only changes which of the other two speaks. 42:9(3) contains no "all candidates are debaters" restriction; its condition is "persons seeking the floor have opposite opinions" — the restriction is the spike's addition. Ranking the debaters among themselves first and then comparing the top debater with the movers removes this dependence on an irrelevant claim.

### 1.5 Sealed votes leak to whoever misses the deadline *(measured)*

In a vote round where eli is past the deadline, the other four ballots go into `S'.vote.ballots`, and their contents are in the revealed claim log (`('vote','ada',('aye',))`, …). The vote stays open, because the toy announces only when every present member has voted. Eli then votes in the next round knowing a 2–2 tally.

So lateness buys information, the herding that sealing exists to prevent in `initial-thoughts.md`. That contradicts F4's "lateness only matters past a deadline that is a presence rule". §10 admits that silent→absent is not implemented. The leak is not mentioned anywhere.

What closes it: a vote round ends at its deadline, non-voters are recorded as not voting (the threshold counts votes cast), and no ballot enters `S'` or the claim log before the announcement.

### 1.6 The fragility derivation does no work in the checks, has a free parameter, and is not the book's criterion *(measured; read; the orders-of-the-day consequence argued)*

- **No check depends on it.** I replaced `is_fragile` with `lambda a, s: False`. Every check still passed, and the specimen's fingerprint was byte-identical. D1 is tested as a classification (`test_d1` calls `is_fragile` directly) but never as the ordering step that `resolve` relies on.
- **The verdict depends on what "the ordinary course" includes.** The probe's `chair-proceed` declares "no second" but does not put a debatable question. The resolver's own ordinary course when nobody claims (`put_question_if_exhausted`) does put it. Against that course, `move amend`, `speak`, and `move previous-question` all become inadmissible, so they are fragile too. §5.1 and `explorations.md` describe narrowing the probe until amend dropped out, and call the narrowing "the finding". It is a parameter fitted to the target list, so "derived, not curated" overstates it.
- **The book's criterion is urgency.** 42:18 allows interruption "only when the urgency of the situation justifies it". A call for the orders of the day "when they are not being conformed to" [42:18(a)], a question of privilege [42:18(b)], and a request needing an immediate response [42:18(e)] are urgency cases, not windows that a speech closes.
  - The model already compiles the conditional cases as `["adjudged", "interruption-urgency", m]` (`guards.json`, `in-order-when-other-has-floor`), which is a judgment input. A simulation sweep cannot derive a judgment input, so the "owed sweep" cannot settle those entries.
  - Under the spike's mechanism, an overdue call for the orders of the day is not fragile. It would be batched or would compete for the floor slot, where a debater with claim age can beat it round after round.
- **"Catch any act the book's list forgot."** The model's README says that "wherever model and source disagree, the source is right and the model has a bug." Under that rule, a derived class that disagrees with t44–t45 is the derivation's bug, not an omission in the book.

What I think survives: fragility is one *sufficient* reason for admitting an act ahead of the floor (a timeliness window), and 42:7/42:19's "if there may be no other opportunity" supports exactly that. Urgency, which is judgment-routed, is a second reason. The interrupt class is the union of the two, and "RONR's interrupt class *is* the fragile set" (§5.1, README, debrief) should not be inherited.

### 1.7 I1 and F2 as stated are stronger than the mechanism *(argued; partly measured)*

- **I1 holds only between floor acts.** It says every admitted act was formed against a state that includes every previously admitted act. But fragile and commuting non-floor acts in a round are applied *before* that round's floor act (steps 3–4), and the floor act was formed without seeing them. The safeguard the text names, an implicit guard on top frame and phase, exists as `State.snapshot_key` and is never called. Admissibility re-checks are not the same guarantee, since an act can stay admissible after it has stopped being informed.
- **F2 holds only at round-open states.** It says every present member is asked at every state. The intermediate states inside `resolve` are never offered to anyone. More importantly, the "Ask" step deliberately skips members whose standing policy "covers" the state. Points of order are where this hurts: breaches are by nature what a standing policy did not anticipate. The proposal that makes one-floor-act-per-round affordable (standing policies; `Policy.standing` is declared and never read) works by giving up F2.
- **§5.2's "forced" argument rules out only blind batching.** The construction shows that batching *without re-evaluating guards* binds C to a blind act. The spike's own policy mechanism allows guards to be re-evaluated against the intermediate state, and that would route C to "speak against". What actually forbids batching is that fresh responders were never asked at the intermediate state (F2 / 42:6). The conclusion holds; the premise named is the wrong one.

### 1.8 Claim age is not a translation of 42:6, and it rewards always claiming *(read; argued)*

- **It is 42:16's line, not 42:6.** 42:6 and 3:32 reset at every yield: "the member who rose … first *after the floor was yielded*", and "a member who rises before the floor has been yielded is not entitled to the floor if any other member rises afterward". Nothing carries over from one yield to the next. A claim queue that persists is 42:16's microphone line. That line comes with a rule the spike omits: "Should a member … move an amendment or other debatable motion, others awaiting a turn should stand aside unless their debate is germane to the new motion." Claims in the toy are keyed by member and persist across changes of pending question.
- **The incentive runs the wrong way.** `initial-thoughts.md` names agents' "disposition to always act". Under claim age, a member that claims every round builds seniority, and a member that passes in order to listen loses its place. 42:6's second sentence protects the opposite: the later riser, who is reacting to what was just said. §6 considered lurking but not this. Since the debrief presents claim age to Joseph as "who has been asking longest", the incentive belongs in front of him.

### 1.9 Two of Joseph's three examples are not addressed *(read)*

`initial-thoughts.md` gives Joseph's three examples of "routing that depends on what arrived":

1. near-duplicate motions as seconds (merge);
2. what happens to the intents that didn't win;
3. targeted follow-up questions to a subset (sub-rounds).

He frames them as "a tiny sampling of the *kinds*", with "most of the real issues of this kind … presumably still unfound". The spike answers (2) with guards. "Merge", "duplicate", "substantially", and "targeted" do not appear anywhere in it.

Yet the debrief tells Joseph the hunch is "true in exactly three ways, and I could find no fourth". The three it lists are policies, ahead-of-floor acts, and the close condition. Targeted rounds also conflict with F2 as stated, since they address a subset. The maker's modify-before-stating window (§8) is a targeted round in all but name.

### 1.10 Some of the §11 critiques of councils-46's draft miss *(read)*

- **"Silence counts only after being asked … a guard it lacks."** `mechanics.md` opens explicit debate rounds after the consent round, so silence there already comes after being asked. The `stated_round` bug came from the toy putting moves and debate in one round kind; it is not a gap in the draft.
- **"Drops RONR's preference-in-recognition entirely."** Under the draft's parallel debate, statements are additive, so 42:9(2)–(3) have nothing to order. What the draft actually drops is 42:9(1) (maker against mover) and the 42:13 ladders. The critique should say that.
- **Tie-breaks.** The draft's "least recently given the floor" is closer to member-symmetric than the spike's roster rotation (see 1.3).
- **"The losing claimant keeps claim age."** The toy itself fails this (1.1).

### 1.11 Checks that check less than their labels say *(read; measured)*

- **Division.** `test_division_vacuous` asserts that `"vacuous" in ev["chair"]`, a string that `apply` writes unconditionally. Nothing is retaken. The claim "a retake reproduces the same tally" is also false for agents: re-asking can return different ballots, and those ballots are now informed by the attributed tally, which is a herding channel. That the purpose dissolved is a sound argument and should be labeled derived. The safer equivalent is "disabled", not "a retake reproduces".
- **I1.** `test_i1` passes on either branch, and the success line printed is conditional on which branch ran.
- **Unanimous consent.** No act ever enters the `consent` phase; that code is unreachable. The table's "unanimous consent … survives, strengthened" and "reserving the right to object" rows have no support in the sim, and nothing claims they do. The sim's docstring lists unanimous consent among what the toy exercises.
- **Seconds in the same round.** §4 says non-floor fragile acts such as "a second … *do* fire in-round". In the toy a second can never land in the round its motion is made: guards and admissibility are evaluated at round open, before the motion exists.

### 1.12 Smaller points *(read)*

- **A second clock door.** The early close in §4 makes arrival matter. Whoever arrives before the "determining" response is on time; the rest become "late", and any of their *other* acts in that round are lost. So "minutes unaffected" does not hold in general. F4 is stated in the README and debrief as "one door only", without this exception.
- **§1 overreaches.** It says the server "must supply (1) and (2) by construction … it can do all three", while §8 finds a no-go on real-time interruption. (2) is supplied as a discretized window. That is defensible, but it is not (2).
- **Table counts.** The table has 28 rows, not 29: 18 survive or are strengthened, 5 changed, 4 vacuous, 1 conditional. The debrief reaches "five" vacuous items by listing "the voice-then-rising-then-counted ladder" and "the inconclusive vote" separately, which are the same row [4:50–51].
- **Rows the table is missing:**
  - 42:5, using one floor grant for a combination of purposes ("begun by debating … conclude by moving … the *Previous Question*"); under one-floor-act-per-round this now costs two wins;
  - 42:16's stand-aside rule;
  - 23:5's reserving of a point of order;
  - 42:11–12.
- **45:8 already exempts sealed votes.** Its change right applies "except when the vote has been taken by ballot (or some other method that provides secrecy)". Sealed-until-announced voting looks covered by that exception, so the book itself switches the right off.
- **The specimen's point of order is wrong about its own record.** It says "cyd's second speech came while dee was still seeking the floor", but cyd had spoken only once (round 4).
- **Throughput is not priced.** The specimen takes 11 rounds for three speeches, one amendment, and one point. Every round waits up to the deadline (minutes) for the slowest member, so slowness becomes everyone's cost, with no counterpart to a dilatory ruling.

## 2. What held when I checked

- **Quotations.** I checked these against the text: 1:1, 9:31, 9:34 (quoted accurately), 3:32, 42:6, 42:7, 42:9, 42:14–19, 23:5, 26:2(1), 26:7, 4:9–13, 4:19–23, 45:6, 45:9, 37:10(a). The two-thirds-negative arithmetic in `announce_if_complete` matches 26:7.
- **The 42:16–17 reading.** The book licenses replacing the race with a hall device; its own devices are a light-order queue or a fixed numbering. Note that 9:36, which the sibling spike cites and this one does not, is more direct: "methods for seeking recognition and obtaining the floor" are named there as something electronic-meeting rules should supply.
- **The walking skeleton's measurement.** `selftest(n=50)` matches what is quoted, and the close-condition rule derived from it is correct.
- **F1.** It holds, and its test detects an arrival-order mutant on the first trial.
- **One floor act per round.** The conclusion is right (see 1.7 on which premise forces it), and so is the observation that precedence among pending motions is untouched by contention.
- **The `stated_round` principle.** "Silence counts only after being asked" is right as a principle, even though it misfires as a critique of the draft (1.10).
- **The do-not-inherit list.** It already flags the strong "RONR's X *is* Y" sentences. Two of them failed when tested (1.6, and "claim age = rose first" in 1.8).

## 3. How I would read the spike's status now

- **The architecture** (pure resolve, policies, rounds, the close rule, the claim log): solid.
- **Fairness beyond arrival-order invariance:** not demonstrated in the toy, and contradicted in four measured ways (1.1–1.5).
- **The fragility thesis:** wrong as an identity with the interrupt list, and right as one of two admission criteria.
- **The survival table:** a useful map, but some of its "survives / strengthened" verdicts rest on mechanics the toy does not implement or contradicts.

## 4. What I would try before softening anything

Each item below aims to make the spike's claim true rather than weaker.

1. **Rebuild F3 as a property and test it:** `resolve` commutes with relabeling members, except through one declared, recorded tie-break device. Run it as a property test next to the F1 test. Fix the three name-order channels in 1.3. Order fragile acts by the model's precedence among incidentals, or treat them all as timely against the round-open state, rather than by kind name.
2. **Claim-age semantics:**
   - a claim lapses only in a round where the member could have claimed and did not;
   - claims are scoped per pending question, with 42:16's germane stand-aside;
   - then decide openly whether persistence is wanted at all, given the always-claim incentive (1.8). A per-yield fresh tie among everyone who claims, decided by a recorded lottery, is the literal 42:6 translation and has no seniority to farm.
3. **Points of order need a cost or a gate.** Route "dilatory" (the model's own guard) into the contention layer, and make the point's timeliness breach-specific: it names the transition it challenges, and it is timely only in the first round after that transition.
4. **Vote rounds close at their deadline,** non-voters are recorded as such, and ballots stay out of `S'` and the claim log until the announcement.
5. **Interrupt admission = fragile ∪ urgent (adjudged).** Keep the fragility sweep as a diff tool against t44–t45, and treat every disagreement as a bug in the sweep.
6. **Answer Joseph's examples (1) and (3) explicitly.** Restate F2 so it can accommodate targeted rounds (for example: every member is asked at every state *that changes its own rights*, and every targeted exchange is revealed), and say what standing policies are allowed to skip.
7. **Make `snapshot_key` the implicit guard**, as §4 describes, so that I1 is a property of the code rather than of the prose.

## 5. Scope, so the silence is not misread

I did not:

- run anything against the real model (there is no evaluator);
- read RONR §§5–6 or 10, t44–t47, or §§16–17 for this pass beyond what is listed at the top;
- audit `explorations.md`'s guesses beyond where they bear on the findings above;
- check the charter sibling beyond its file 01.

The probes are reconstructable from the descriptions above. Each one sets up a state with `collect_and_resolve`, submits a profile, and reads `record`, `claims`, `vote` or `claim_log`. 1.1 and 1.2 take about ten lines each. This report's content is its list of findings; that I did not raise something is not evidence that it is sound.
