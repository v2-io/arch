# Contention without a clock: how agents can share a floor fairly

*Spike result, 2026-11-03, revision 3 after two independent verifications (`de-novo-feedback-1.md`, `de-novo-feedback-2.md`; my responses beside them). Claude (Fable 5.1), spiker. Labels: **derived** = argued from the model or the book with the cite; **checked** = exercised in `sim/`; **guess** = my judgment. Retractions since earlier revisions are marked* **retracted** *in the body.*

## 1. The question, and the question under it

As posed: RONR settles who speaks next by a human chair's perception of who rose first; for agents responding through APIs, "first" is latency. What mechanism collects, combines, sequences and resolves members' concurrent intents so that the outcome never depends on arrival order and is still as fair as RONR means — and how much of RONR's contention machinery survives it?

Under it: RONR's contention layer presupposes **simultaneous aural communication**, and the book says so. A deliberative assembly "meets in a single room or area or under equivalent conditions of opportunity for simultaneous aural communication among all participants" [1:1]; an electronic meeting keeps its deliberative character only if it provides that [9:31]; a group that deliberates in writing "does not constitute a deliberative assembly … it is foreign to the deliberative process as understood under parliamentary law" [9:34]. **Retracted** (rev. 2): that 9:36's "methods for seeking recognition and obtaining the floor" names the slot this spike fills, and that the book "expects" the race to be replaced. 9:36 governs meetings that already meet the aural standard, and 42:16's microphone queues are devices for an aural hall. They are precedent we borrow, not license. The honest framing is the charter sibling's: an agentic council is a principled departure from 9:34's letter, the charter is its authority, and the book's devices are what the charter borrows. Simultaneous hearing did three jobs — common state; an instant interrupt channel; contention resolved by perception — and the server supplies the first by construction, the second as a window one round wide (a discretization, §8), and replaces the third with a rule.

## 2. What "fair" means here — seven properties

councils-46's criterion: *the record is identical under any permutation of when members' responses arrive, so no agent gains or loses because its API was faster or slower; a human member slower by hours is just the slowest member.* The first verifier showed that permutation-invariance alone admits unequal rules; the second showed that a class of unfairness — a claim that does not win still steering the outcome — is invisible to every property I had stated. Seven now, each with its status:

- **F1 — order-independence.** The transition and the record are a function of the *set* of responses received before the close. *(checked, with guarded policies: `test_f1_invariance`.)*
- **F2 — universal opportunity at round-open states.** Every present member is sent every round-open state; nothing inside a round is a state anyone holds rights at. A member may keep its previous policy instead of answering (§4), which is a revealed choice to forgo that round's breach window. Targeted rounds (§6) are fair under F2 iff revealed to all and no member's rights change at a state it was not sent. *(derived; no test — the toy never skips anyone.)*
- **F3 — equivariance under relabeling,** except through three declared devices: least-recently-given-the-floor; a lottery keyed by a committed seed, the floor-grant count, and the roster slot; and the roster slot itself as the tie-break among otherwise identical acts. Slots are assigned by the charter at seating, not by name or join order. *(checked: `test_f3_relabeling`.)*
- **F4 — wall-clock enters through one door, and these are its effects.** The presence deadline per round. A response after it is logged and never applied. With this revision: silence never puts a question, never kills an unseconded motion, and never rejects a poll nobody answered (silence extends). **A vote round still closes at its deadline, and a member slower than the deadline loses its ballot.** So the criterion is met *within* the deadline, and beyond it everywhere except the ballot; that residue is deliberate (a vote that waits for everyone can be held forever by one dead member) and the charter chooses among the remedies: the deadline's length, a member's declared "absent for N rounds," or an extension cap. *(checked: `test_silent_is_not_pass`, `test_f4_deadline_and_vote_leak`.)* **Retracted** (rev. 2): "its only effect is 'did not respond this round'."
- **I1 — informed floor acts.** A floor act fires only if the state it was formed against is still the state in the respects it depends on (an implicit snapshot guard on top frame, its stated-ness, phase, pending points); otherwise all floor claims defer with a recorded reason. Under a persisting policy (§4) this means the member's own declared condition held, which the member chose in advance and the record shows. *(checked: `test_i1_implicit_guard`.)*
- **IND — independence of losing claims.** Adding a claim that is not selected does not change which claim is selected, except where the book says it does — 43:13's deferral of a second speech in favor of an unspoken member who claims to speak. A single per-claim key over all eligible claimants gives this by construction; the test checks the whole resolver. *(checked: `test_independence_of_losing_claims`.)* **Retracted** (rev. 2): the two-stage "top debater vs top mover" ladder, under which a losing debater's claim could hand the floor to a mover, and a misdeclared side could win alternation.
- **S — sealing.** No response in a round may depend on another response from the same round. This is an enforcement property of the store and the launchers, not of `resolve`; the walking-skeleton spike found it honor-system today (sealed intents are files a member with file tools could read). Without S, F1 holds vacuously and the slower member gains information. *(derived; the toy's one internal leak — guards could read this round's claims — is closed; guards see a claim-free view.)*

## 3. What RONR's race decides, so we know what to replace

From 3:30–34 and 42:1–22: three layers; only the last is wall-clock. (1) Acts in order without the floor [42:18–19; t44–t45] — a closed list of purposes *and* an urgency gate. (2) Preference in recognition [42:7–13]: the maker's first speech; no second speech while an unspoken member desires to speak [42:9(2), 43:13]; alternation where the chair *knows* the sides [42:9(3)]; the chair's first word on an appeal; the retake mover; the reconsider-for-amendment mover; the no-question-pending ladders. (3) The race, reset at every yield [42:6, 3:32], with chair-in-doubt → vote [42:14] and wrong assignment → point of order [42:15]. Layer 1 is registry data plus a judgment input. Layer 2 is computable from state except alternation, whose premise — the chair's *knowledge* of sides — has no agentic equivalent short of a judgment (a declared side is cheap talk). Layer 3 is what we replace. The race was never the fairness; the preference rules and the equal right to be heard were.

## 4. The mechanism

**Ask.** Every present member is sent the round-open state `S` (as a delta), the record since it last looked, and its `available_actions` with justifications.

**Respond.** A member returns a **policy**: a prioritized list of (guard, act). Guards are expressions over the round-open state as the member sees it — no claims of the current round, no sealed contents. An unguarded floor act carries the implicit snapshot guard (I1). Several non-floor acts, at most one floor act, at most one ballot (the first). *Pass* is a response. **Policies persist:** a member that does not replace its policy has it re-evaluated against each new round-open state, which is where guards do their work (a stale act is one whose guard fails at the new state); the cost is that a persisting policy cannot anticipate a breach. A member's first submission in a round is final; a replacement is refused and logged.

Why policies: Joseph's intuition was that "what arrives from members in one step may call for routing that depends on what arrived." Guards are the routing, written by the member, applied across rounds. **Retracted** (rev. 2): that guards see "structural effects of acts admitted this round"; they see the round-open state only.

**Close.** When every asked member has responded, or at the deadline. Nothing else. (Walking-skeleton measurement: replaceable submissions under an "everyone is in" close gave four logs over fifty shuffles. The close must be a function of each member's own final state.)

**Resolve** — `resolve(S, profile) → (S', record, claims)`, pure:

1. *Seeking to speak:* members with an admissible claim to speak, judged before the 43:13 bar, from the profile alone. Movers and out-of-order claims are not in it.
2. *Live acts* against `S0`. A speech barred by 43:13 still counts as asking for debate (it defers; it never closes).
3. *A vote round is the poll.* Ballots sealed; the round's close is the poll's close; non-voters recorded; thresholds over votes cast; a poll with no ballots is void and repeats. A point of order against the put or a stating, raised in the poll round, voids the ballots and is ruled first [4:34]. The announcement is a challengeable transition [45:9].
4. *Points first.* Points admissible at `S0` are admitted (ordered by content, then slot). If any lands, every other non-floor act is deferred to the round after the ruling, logged as deferred; their windows survive, since a ruling round opens no debate and no vote on them. Points are keyed per (transition, member), each is ruled, and the next pending point is submitted as soon as one is ruled. A point is timely only in the first round after its transition and cannot be re-raised by the same member against it; whether it is *frivolous* is the model's `dilatory` judgment, routed to a chair-mind or the council.
5. *Other non-floor acts* (objections, seconds), admitted iff admissible at `S0`, in kind order with set-valued effects.
6. *One floor act*, if the snapshot guard holds, by **one per-claim key**: the maker's first speech [42:9(1)] → least-recently-given-the-floor → the lottery. Alternation is not a rung (§3); the declared side is recorded. **Retracted** (rev. 1 and 2): persistent claim age; the two-stage ladder; alternation as a rung.
7. *The chair's duties* — stating; putting an undebatable question; announcing; "no second" — fire only on the strength of answers, never of silence.

**Reveal.** `S'`, the admitted acts with justifications, and the full claim log (fired, not fired, deferred, late, silent, duplicate; ballots as "(sealed)" until announced).

Two rules that fell out of faithfulness, unchanged: every window-opening transition is followed by a round, and silence is exhaustion only after being asked — now with "asked" meaning *answered* (F4).

## 5. The two derivations

**5.1 Fragility** is one of two admission criteria and a diff tool. The admission class — acts in order without the floor — is registry data (the interrupt attribute) plus the adjudicated urgency condition. Define an act *fragile* at `S` if admissible at `S` and inadmissible after the ordinary course proceeds without it: another member who can speak speaking, the chair's next duty that no claimant blocks, or the round advancing with nothing done. The exclusion of the chair putting a *debatable* question is a declared choice [42:22, 4:34]. Under that probe the toy classifies second, point of order and objection to consideration as fragile and amend, previous question and a speech as not, stably under relabeling. *(checked.)* The book states a fragility test at 42:7/42:19 ("if there may be no other opportunity"). In the toy the classification orders nothing (all non-floor acts precede the floor anyway). Its use is as a diff against t44–t45 over `actions.json`, where every disagreement is the diff's own defect. **Owed.**

**5.2 One floor act per round is forced** by F2 / 42:6: members who would react to the first admitted act were never sent the state it produced. Precedence among pending motions is untouched.

## 6. Adversarial cases, including both verifiers'

- *Two primary amendments in one round* — one wins, the other re-asked with it pending. *(checked.)*
- *Point of order and a motion in one round* — point first; the mover loses nothing (no standing to lose). *(checked.)*
- *One member raises a point every round* — bounded to one per transition it can name; everyone pays two rounds per point, the obstructor nothing beyond its own. The rest is `dilatory`. **Latency is a second obstruction channel:** answering at deadline-ε every round is never silent and never dilatory, and the point-of-order bound does not touch it; a per-member response budget would be a charter rule. *(checked for the bound; the latency channel is derived.)*
- *An out-of-order claim closes debate* — it cannot: invisible to the 43:13 bar. *(checked.)*
- *A losing debater's claim hands the floor to a mover; a misdeclared side wins* — cannot: one key, no alternation rung. *(checked.)*
- *A point of order between two contests re-rolls the lottery* — cannot: keyed by floor grants. *(checked.)*
- *A point and an objection in one round* — point ruled first, objection deferred with its window intact. *(checked.)*
- *Two members' points on one transition* — both ruled. *(checked.)*
- *A late debater, a late seconder, a round nobody answered* — silence extends. *(checked.)*
- *A member misses a vote deadline* — recorded as not voting; sees nothing; ballot not counted. *(checked.)*
- *A member is renamed* — nothing changes except the declared devices. *(checked.)*
- *"I vote with the majority" / "I claim if ben claims"* — not expressible: guards see no sealed contents and no current-round claims. *(structural.)*
- *A member always claims* — gains nothing: no seniority; least-recently-floored favors whoever has been *listening*. (Not "least often": a member who spoke ten times long ago ranks above one who spoke once just now; that is 42:6's spirit, and a charter could add a per-question count.) *(derived.)*
- *A stating lands in a round halted by an unrelated point* — the stating's challengers are admissible only in "the previous round," which was the ruling round. **Not repaired.** Property: a window opened by T closes only after a round in which T's challengers were admissible. Fix: key windows to the first round in which the act was admissible after T. *(derived; §10.)*
- *The server misassigns the floor* — point of order against the server's act; revert. *(derived; not in toy.)*

**Joseph's examples.** (1) *Near-duplicate motions in one round:* both floor acts; one wins; "substantially the same" is the model's adjudicated predicate; the server flags byte-identical text and otherwise re-asks the loser with the winner pending, where it may second, join, withdraw in favor, or amend. (2) *Intents that did not win:* their policies persist with their guards. (3) *Targeted rounds* (the maker's modify window; a merge question; a ruling from a chair-mind): fair under F2 iff revealed and no rights change at an unsent state. *(derived; not in toy.)*

## 7. What survives, what changes, what is vacuous

| RONR contention device | Its job | Under rounds | Status |
|---|---|---|---|
| Rising and being recognized [3:30–31, 42:2] | mutual exclusion on speech | one floor act per round, by one key | **survives** |
| "Rose first after the floor was yielded" [42:6] | the tie-break, reset at each yield | least-recently-floored, then a lottery keyed by floor grants and slot | **changed**: same reset unit, declared devices |
| "Rising before yield gives no claim" [3:32, 42:6] | no queue-jumping | guards see the round-open state only; in-round effects never fire a floor act | **survives** |
| Maker's first speech [42:9(1)] | the proposer is heard first | first rung | **survives** (checked) |
| No second speech while an unspoken member desires to speak [42:9(2), 43:13] | equal hearing | admissibility filter over admissible speech claims; defers, never closes | **survives** (checked) |
| Alternation between sides [42:9(3)] | balance where the chair *knows* the sides | a declared side is cheap talk; recorded, advisory; a charter may route side to a judgment and rank on it | **changed**: advisory by default |
| Chair speaks first on an appeal [42:10] | the ruling defends itself | only with a chair-mind | **conditional** |
| Reconsider-for-amendment preference [42:11–12] | the announced purpose is reachable | computable from the recorded purpose | **survives** (derived; not in toy) |
| Ladders when nothing is pending [42:13] | series, notice, reconsider | computable from state | **survives** (derived; not in toy) |
| Chair in doubt → vote [42:14] | last-resort neutrality | never in doubt; adoptable | **changed** (derived; not in toy) |
| Appeal from a wrong assignment [42:15] | correct the chair | point against the server's act; revert | **survives** (derived; not in toy) |
| One floor grant, several purposes [42:5] | debate then move in one turn | two rounds; a persisting policy carries both | **changed**, priced |
| Large-hall devices [42:16–17] | replace the race inside an aural hall | precedent for the declared devices | **borrowed**, not licensed |
| Stand aside unless germane [42:16] | the queue follows the question | no queue persists | **survives** by construction |
| Acts in order without the floor [42:18–19] | timely rights | registry + urgency judgment; fragility as a diff | **survives** |
| Interrupting mid-speech [42:18] | stop harm in progress | the next-round window; chunking is the fair real-time form | **changed**, priced (§8) |
| Speaker keeps the floor through interruption [42:20] | continuity | a floor-holder flag | **survives** (derived; not in toy) |
| "Question!" calls [42:22] | pressure | no channel | **vacuous** |
| Seconding [4:9–14] | screen one-person motions | not required in committee; where required, a non-floor act the round after the motion | **survives** |
| Second immaterial once debate begins [4:13] | timeliness | fragile window | **survives** |
| Modify/withdraw before stating [4:19–23] | the maker's property | one targeted round if the charter wants it | **changed** (derived; not in toy) |
| Point against the put, before any vote [4:34, 45:6] | the wording is right | a point in the poll round voids the poll | **survives** (checked) |
| Unanimous consent and objection [4:58–59] | skip the vote when no minority | a consent round | **survives** (derived; not in toy) |
| "Reserving the right to object" [4:60] | consult first | one inquiry round | **survives** (derived; not in toy) |
| Precedence among pending motions [5–6] | what may be moved while what pends | untouched | **survives verbatim** |
| Point of order timeliness [23:5] | challenge at the moment | the first round after the transition, breach-specific; one gap (stating into a halted round) | **survives**, with the gap named (§6) |
| Reserving a point of order [23:5] | hear the maker first | a reserve response | **survives** (derived; not in toy) |
| Appeal timeliness [24:8] | before business intervenes | the round after the ruling | **survives** (derived; not in toy) |
| Division of the assembly [29, 4:52] | verify a doubtful uncounted vote | no uncounted votes; a retake would be an informed reconsider | **vacuous** — disabled by charter (derived) |
| Inconclusive-vote ladder [4:50–51] | make an uncounted vote conclusive | none to make | **vacuous** |
| Change one's vote before announcement [45:8, 45:50] | correct a mis-cast vote | an attributed vote is a roll call, which allows changes [45:50]; the sealed round removes it | **dropped, priced**: a mis-submitting agent has no correction |
| Vote-challenge window [45:9] | one last look | the round after the announcement, which is a challengeable transition | **survives** (checked) |
| Bylaw-required secret ballot [45:20] | secrecy that cannot be suspended | every vote here is attributed; no secret method exists | **no equivalent** in this design; a charter bound by such a bylaw needs a sealed-unattributed method |
| Prevailing-side eligibility [37:10(a)] | honor-system in voice votes | every vote attributed; enforceable once the model's committee branch for 37:35(2) exists | **strengthened**, conditional on a model fix |
| Stand at ease [8:2(4)] | a pause | a round with no acts | **survives** (derived; not in toy) |

Thirty-five rows: 22 survive (1 strengthened, conditional on a model fix), 8 change (6 priced or declared, 1 borrowed, 1 dropped), 3 vacuous, 1 conditional, 1 with no equivalent. Eleven rows rest on the book and the model only ("derived; not in toy"); the others on the sim's checks or on the model's own data. The count is a reading aid over rows I chose.

## 8. The priced equivalents, and the no-go

Mid-speech interruption: chunk the speech, a round per chunk. The maker's window: one targeted round. 42:5: two rounds. Vote change: dropped, priced. **The no-go, restated:** an interrupt can react only to what its sender had seen, so where it lands within a speech depends on reading latency; anything that lets one member's act land mid-way through another's *as a reaction to it* decides by latency. **Retracted** (rev. 2): "any act landing between two others within a round" — the resolver's own kind-ordering lands acts between others, harmlessly, because none of them reacts to another.

## 9. What this does to the server's center

A pure `resolve` with one per-claim key, a committed lottery seed keyed by floor grants, breach-specific points, poll-voiding points, silence that extends; persisting policies over a claim-free view; a store that enforces sealing; a two-layer record; one clock door with its effects declared; mechanical chair duties the server's, judgment a chair-mind's or the council's.

## 10. What this spike did not do, and what it costs

- The real sweep over `actions.json`.
- The halted-stating window (§6); targeted rounds, consent rounds, the floor-holder flag, the revertible log, presence → absence → quorum (the toy's void poll repeats forever if nobody ever answers).
- **Throughput:** the specimen takes 12 rounds for four speeches, one amendment, one point and two votes; an obstructor raising a point against every transition took a seven-round session to seventeen (verifier's measurement). Every round can wait the full deadline for the slowest member.
- **A divergence recorded:** the committee specimen keeps the two-speech cap [43:12] and the 43:13 bar, which 49:21(3)/50:25 remove in committees. councils-46's draft keeps them as rate limits; so does the toy; the charter should say so.
- Sealing enforcement (S) is a store-and-launcher problem this spike only names.
- Parallel debate; the chair question; a per-member response budget.

## 11. Where councils-46's drafts stand against this

Its least-recently-floored tie-break is adopted; its parallel-debate default drops 42:9(1) against a mover and the 42:13 ladders, nothing more; its curated preemptive class is the registry's own; its "everyone passes → move on" is right with silence excluded; its sealing rationale is now a named property with an enforcement requirement.
