# Contention without a clock: how agents can share a floor fairly

*Spike result, 2026-11-03, revision 2 after `de-novo-feedback-1.md` (what changed and why: `response-to-feedback-1.md`). Claude (Fable 5.1), spiker. Labels: **derived** = argued from the model or the book with the cite; **checked** = exercised in `sim/`; **guess** = my judgment. RONR cites are §:¶ into `../../RONR-12/body.md`, read for every cite below. Statements retracted since revision 1 are marked* **retracted** *in the body, because a reader of the first version may be carrying them.*

## 1. The question, and the question under it

The question as posed: RONR settles who speaks next by a human chair's perception of who rose first; for agents responding through APIs, "first" is latency. What mechanism collects, combines, sequences and resolves members' concurrent intents so that the outcome does not depend on arrival order and is still as fair as RONR means it to be — and how much of RONR's contention machinery survives it?

The question under it, found by asking what "rose first" was *doing*: RONR's contention layer presupposes **simultaneous aural communication**, and the book says so. A deliberative assembly "meets in a single room or area or under equivalent conditions of opportunity for simultaneous aural communication among all participants" [1:1]; an electronic meeting keeps its deliberative character only if it provides that [9:31]; a group that deliberates in writing "does not constitute a deliberative assembly … it is foreign to the deliberative process as understood under parliamentary law" [9:34]. The book's own list of what electronic-meeting rules must supply includes "methods for seeking recognition and obtaining the floor" [9:36]. So the agentic medium is outside the book by the book's statement, and the book names the slot this spike fills. Simultaneous aural communication did three jobs:

1. **Common state.** Everyone perceives the same state at the same moment.
2. **A real-time interrupt channel.** "Point of order!" reaches everyone at once, so a window "at the time of the breach" [23:5] can be a breath wide.
3. **Contention resolved by perception.** The chair sees the claimants and, after the preference rules, the race.

(The charter-agenda sibling spike reached this foundation independently, with 1:1 n.1, 49:16 and 51:2 besides, and named the round's property the *common-state condition*.)

The server supplies (1) by construction, (2) as a window one round wide — a discretization, not the real-time channel, and §8 says what that costs — and replaces (3) with a rule. Section 7 is the table of what that does to each of the book's devices.

## 2. What "fair" means here

councils-46 proposed: *the record is identical under any permutation of when members' responses arrive.* I keep it and add to it, because roster-order tie-breaking satisfies it and is unequal. Each property below is one the revised toy has, not one the prose asserts:

- **F1 — order-independence.** Within a round, the transition and the record are a function of the *set* of (member, response) pairs received before the close, never of their order. *(checked: `test_f1_invariance`, 200 random states × 18 permutations; the verifier confirmed the test detects an arrival-order mutant on its first trial.)*
- **F2 — universal opportunity at round-open states.** Every present member is asked at every round-open state, and nothing *inside* a round is a state anyone holds rights at. A member may instruct the server not to ask it while a stated condition holds (a standing policy); that instruction is the member's own, is revealed, and gives up that member's breach window for those rounds — so it should not be the default for rights-critical acts. A round addressed to a subset (a targeted round, §6) is fair under F2 iff it is revealed to all and no member's rights change at a state it was not asked at. *(derived; the toy never skips anyone.)*
- **F3 — equivariance under relabeling.** The record does not change when a member's name changes, except through two declared and recorded devices: least-recently-given-the-floor, and a lottery keyed by the council seed, the round, and the member's roster slot (the book's "numbered microphones" [42:16]). Arrival is never read. *(checked: `test_f3_relabeling`, 600 random states, one member renamed to sort first or last, records identical after mapping back. The first toy failed this in 347 of 1,500 of the verifier's runs through three name-order channels; all three are closed.)*
- **F4 — wall-clock enters through one door.** The presence deadline per round, sized in minutes. Its only effect is "did not respond this round." A late response is logged as supplementary and never applied; a late ballot is never counted and never seen; a vote round closes when the round closes, with non-voters recorded as such. *(checked: `test_f4_deadline_and_vote_leak`.)* **Retracted:** revision 1 allowed "early close on determined rounds"; that is a second clock door (whoever arrives after the determining response becomes "late," and their other acts in that round are lost). Removed.
- **I1 — informed floor acts.** Every floor act fires against a state that includes every act admitted before it, in earlier rounds and in this one: an unguarded floor act carries the implicit guard "top frame, its stated-ness, the phase, and the pending points are as I saw them," and if this round's non-floor acts changed any of those, all floor claims defer to the next round with a recorded reason. Non-floor acts in one round are blind to each other by design — they are the class the book lets a member call from the seat without the floor [4:9, 23:2(3)]. *(checked: `test_i1_implicit_guard`.)*

Joseph's constraint — no agent penalized because its API was slower — is F1 plus F4.

## 3. What RONR's race decides, so we know what to replace

From 3:30–34 and 42:1–22 read whole, floor assignment has three layers; only the last is wall-clock.

1. **Acts in order without the floor** [42:18–19; t44–t45]: admitted for a closed list of purposes *and* only when urgency justifies — two gates. Points of order, questions of privilege, calls for the orders of the day, division, appeal, objection to consideration, requests needing an immediate response, and "if there may be no other opportunity," notice and the making of a reconsider [42:7, 42:19].
2. **Preference in recognition** among those who rose before anyone was recognized [42:7]: the maker's first speech [42:9(1)]; no second speech while an unspoken member seeks the floor [42:9(2), 43:13]; alternation between sides [42:9(3)]; the chair's first word on an appeal [42:10]; the retake mover after a result [42:8]; the mover of a reconsider-for-amendment [42:11–12]; the no-question-pending ladders [42:13].
3. **The race**: "the member who rose and addressed the chair first *after the floor was yielded*" [42:6], reset at every yield, with the rule that rising before the yield establishes no claim [3:32, 42:6]; chair in doubt → the assembly votes [42:14]; wrong assignment → point of order, appealable [42:15].

Layer 1 is registry data plus one judgment predicate (`in-order-when-other-has-floor` routes conditional cases to `interruption-urgency`). Layer 2 is computable from state once members *declare* their side when claiming, which the book suggests with cards and for/against microphones [42:9(3)]. Layer 3 the book treats as a residue it allows large halls to replace with microphones, lights and lists [42:16–17], and names as a thing electronic-meeting rules must supply [9:36]. So replacing the race is something the book expects. The race was never the fairness; the preference rules and the equal right to be heard were.

## 4. The mechanism

The server keeps the model's state `S` and runs rounds. A round is one state transition's worth of contention.

**Ask.** Every present member is sent `S` (as a delta), the record since it last looked, and its `available_actions` with justifications — the model's function. (A member with a standing policy covering `S` may have asked not to be asked; see F2.)

**Respond.** A member returns a **policy**: a prioritized list of (guard, act). A guard is an expression over the round-open state — the model's expression language will do — restricted to that state plus the *structural* effects of acts admitted this round, never this round's sealed contents (so "I vote with the majority" is not expressible). An unguarded floor act carries the implicit snapshot guard (I1). A policy may carry several non-floor acts and at most one floor act. *Pass* is a real response. One response per round is final; if a server allows replacement, a member must finalize explicitly and the round closes only on finalizations (next paragraph).

Why policies: Joseph's intuition was that "what arrives from members in one step may call for routing that depends on what arrived." Guards are that routing, written by the member. A policy of "second any postponement; if nothing is pending, move X; if an amendment to X is on top, speak against it" does the right thing in whichever state obtains, and a stale act is one whose guard failed — it does not fire.

**Close.** The round closes when every asked member has responded (or finalized), or when the presence deadline passes. Nothing else closes it. The walking-skeleton spike measured why: over 50 shuffled arrival orders its server produced one log when submissions were final and four when a seat could replace its submission, because a replacement races an "everyone is in" close. *The close condition must be a function of each member's own declared-final state, never of a count of messages.* *(derived; measured by the sibling's `steward.py selftest`.)*

**Resolve** — `resolve(S, profile) → (S', record, claims)`, pure, in this order:

1. *Who is seeking the floor* this round is read off the profile (every member with a live floor entry), so the second-speech bar [43:13] is evaluated against the round-open profile and not against anything mutated during resolution.
2. *Live acts*, all evaluated against the round-open state `S0`: every entry whose guard holds and whose act is admissible at `S0`. Every entry goes to the claim log whether or not it fires — the equivalent of "rose but was not recognized."
3. *A vote round is the poll.* If a vote is in progress only ballots are admissible [45:6]; they are sealed, the round's close is the poll's close, non-voters are recorded, and the result is announced in the same round.
4. *Non-floor acts*, each admitted iff admissible at `S0`, applied in a fixed order by kind (points of order, then objections, then seconds), never re-checked against one another. Points, seconds and objections are set-valued in the state, so order within a kind cannot matter. Two points of order in one round are both admitted and both ruled on. *(This is the sibling's reading of 23:5: any point submitted in the first round addressed to a state is "at the time of the breach.")*
5. *A point of order halts the floor* this round; it is disposed of first [23:2(1)], by the chair-mind if there is one, else submitted to the council [23:18–21]. A point names the transition it challenges, is timely only in the first round after it, and cannot be raised against a transition already challenged or ruled — so halting the floor costs the obstructor one ruling round per transition and is bounded. Whether a point is *frivolous* is the model's `dilatory` predicate, a judgment input the contention layer routes and does not compute. *(checked: `test_point_of_order_bounded`.)*
6. *One floor act*, only if the snapshot guard holds (I1), chosen in two stages: debaters are ranked among themselves (the maker's first speech [42:9(1)]; then alternation by declared side [42:9(3)]; then least-recently-given-the-floor; then the slot lottery), movers among themselves (least-recently-floored; lottery); the maker's first speech beats everyone; otherwise the top debater and top mover are compared by the neutral key alone, since between a debater and a mover the book has only the race [42:6]. The second-speech bar is an admissibility filter [43:13], not a rung, so it cannot rank a debater above a mover. *(checked: `test_f3_ladder`, `test_mover_claim_does_not_flip_debater`.)* **Retracted:** revision 1 ranked claimants by *persistent claim age* ("rose first," in rounds). The verifier showed it was wiped by every vote or ruling round (so a point of order stole turns), and that as a design it is 42:16's microphone line, not 42:6 — and it rewards always-claiming, the disposition councils-46 named as the hazard. 42:6 resets at every yield; the revised ladder does too.
7. *The chair's duties.* Where exactly one chair act is due (state; put an undebatable question; announce; "no second"), the server does it, as `actions.json` already says ("the list is also the obligation").

**Reveal.** `S'`, the admitted acts with justifications, and the full claim log go to everyone. Next round.

Two rules that fell out of making this faithful:

- **Every transition that opens a window is followed by a round.** A motion stated this round is not put this round even if nobody claimed debate, because nobody was asked at a state where it was pending. Silence is exhaustion only after being asked. (A toy bug found this; `stated_round`.)
- **A guard that becomes true only because of an act admitted this round does not fire a floor act this round** — it is a claim next round, competing with members freshly asked. That is 42:6's "rising before the floor is yielded establishes no claim." Non-floor acts (a second, an objection, a point) do land in the round they are made, against the round-open state. A second therefore lands the round *after* its motion is moved, which is when members first see the motion. **Retracted:** revision 1 said a second "fires in-round" with the motion; it cannot, and should not.

## 5. The two derivations, as they stand after verification

### 5.1 Fragility: one of two admission criteria, and a diff tool

**Retracted:** revision 1 claimed "RONR's interrupt class *is* the fragile set, computed." Two things were wrong. First, 42:18 has two gates — a closed list of purposes, *and* urgency — and the model already routes the conditional cases to the judgment predicate `interruption-urgency`; no simulation derives a judgment input. Second, the fragility probe's verdict depends on what "the ordinary course of business" is taken to include, which is a declared choice, so "derived, not curated" overstated what was done.

What stands: define an act **fragile** at `S` if it is admissible at `S` and inadmissible after the ordinary course proceeds without it — another member (who can) speaking when the pending question is debatable, the chair's next duty that no claimant blocks, or the round advancing with nothing done. The exclusion of the chair putting a *debatable* question is a declared choice with a reason: the chair does not put while a member seeks the floor [42:22, 4:34], so for a claimant that put is not part of the ordinary course. Under that probe the toy classifies second, point of order, and objection to consideration as fragile and amend, previous question and a speech as not, and the classification is stable under relabeling members. *(checked: `test_d1_classification_and_d2`.)* The book states a fragility test in its own words at the one place it makes interruption conditional — "if there may be no other opportunity" [42:7, 42:19] — so fragility is *a* reason the book recognizes for admission ahead of the floor. Urgency is the other. **The admission class is fragile ∪ urgent**, with urgent coming from the registry's interrupt attribute and its adjudicated condition.

What the toy cannot show: in it, every non-floor act is admitted before the floor act regardless, so "fragile first" does no ordering work (the verifier replaced `is_fragile` with a constant and nothing changed). The work the interrupt class does in the book is deciding *whether an act needs the floor at all*, and the toy hard-codes that partition. The use of the derivation in a real server is as a **diff tool**: run it over `actions.json` against t44–t45 and 42:18–19, and treat every disagreement as a defect in the sweep (the model's rule: where model and source disagree, the source is right). **Owed,** as before, since the model's evaluator does not exist yet.

### 5.2 One floor act per round is forced

Could the server admit several floor acts in one round, ordered by rank? It would satisfy F1 and F3. What forbids it is F2 / 42:6: members who would react to the first admitted act were never asked at the state it produced, so the second act is a claim made before the floor was yielded. **Retracted:** revision 1 said the forbidding premise was that batching binds the *mover* to a blind act; the verifier pointed out that re-evaluating guards against the intermediate state would route that mover correctly, so the blindness that matters is everyone else's. The construction in `test_i1_implicit_guard` still shows the guarded routing working (C postpones only if the main alone is pending, else speaks against the amendment), and the conclusion — one floor act per round, with policies and no batching — is unchanged.

Corollary: **precedence among pending motions is untouched.** The stack machine, the yield matrices, the series-halting adoptions govern what the state does once an act is admitted; contention touches only admission.

## 6. Adversarial cases, including the verifier's

- **Two primary amendments in one round.** Do not commute [12:13]; one wins by the ladder; the other is re-asked with the first pending and may move a secondary amendment or speak. *(checked.)*
- **Point of order and a motion in one round.** The point goes first and halts the floor; the motion is re-asked after the ruling. The mover loses no standing, because there is no standing to lose (per-round claims). *(checked in `specimen()`.)*
- **One member raises a point every round.** Bounded to one halt per transition it can name; ben still spoke twice in twenty rounds against an obstructor. The rest is the dilatory predicate. *(checked.)*
- **A third member's throwaway motion claim changes which debater speaks.** It cannot: debaters are ranked among themselves first. *(checked.)*
- **A member misses a vote round's deadline.** Recorded as not voting; the tally it would have seen was never revealed; a late ballot is not counted. *(checked.)*
- **A member is renamed.** Nothing changes except through the two declared devices. *(checked.)*
- **A member pre-commits "if anyone moves to close debate, I move to adjourn."** The guard is evaluated at round open; it becomes true only after the trigger is admitted; the adjourn is a next-round claim among freshly asked members. *(structural in the toy: guards see `S0` only.)*
- **"I vote with the majority."** Not expressible: guards cannot reference sealed contents. *(derived; a constraint the server's guard language must enforce.)*
- **A member always claims to build seniority.** There is no seniority: claims are per round. Least-recently-given-the-floor favors whoever has had the floor *least*, which is the direction 42:9(2) points. *(derived.)*
- **A member that never responds.** Silent for the round; after a council-configured number of consecutive silences, absent, quorum recomputed, the chair's duty on noticing fires [40:11–12]. *(derived; not in the toy.)*
- **The server misassigns the floor (a bug).** Its mechanical acts are appealable like a chair's [42:15]: a point of order against the server's act; if sustained, the server reverts to the pre-act state. Needs a revertible log. *(derived; not in the toy.)*

**Joseph's own examples, which revision 1 did not address.** He gave three kinds of routing-that-depends-on-what-arrived:

- *Near-duplicate motions in one round.* Both are floor acts; one wins. Whether the other is "substantially the same" is the model's adjudicated predicate (`substantially-same`, 10:26(3–5)) and cannot be computed; the server can flag byte-identical text and otherwise re-asks the loser with the winner pending, where its options include seconding or joining the winner's motion, withdrawing in its favor, or moving its own text as an amendment. In committee no second is needed, so "counts as a second" is moot; "which text is pending" is the live question, and the ladder answers it with a recorded device rather than a race. councils-46's lean — only byte-identical text merges automatically — is right. *(derived.)*
- *Intents that did not win.* Standing with their guards; re-asked when no guard covers the new state. *(derived; the sim's guarded routing.)*
- *Targeted follow-up questions to a subset.* A targeted round (the maker asked whether it will modify or withdraw [4:19–23]; two makers asked whether their motions are one; a chair-mind asked for a ruling) is fair under F2 iff it is revealed to everyone with the round and no member's rights change at a state it was not asked at. The maker's modify-before-stating window in §8 is a targeted round. *(derived; not in the toy.)*

## 7. What survives, what changes, what is vacuous

| RONR contention device | Its job | Under rounds | Status |
|---|---|---|---|
| Rising and being recognized [3:30–31, 42:2] | mutual exclusion on speech | one floor act per round, chosen by ladder | **survives**, race replaced (§3) |
| "Rose first after the floor was yielded" [42:6] | the tie-break, reset at each yield | per-round contest; least-recently-floored, then recorded slot lottery | **changed**: same reset structure, declared device |
| "Rising before yield gives no claim" [3:32, 42:6] | no queue-jumping | in-round-triggered floor acts defer to the next round | **survives** (§4) |
| Maker's first speech [42:9(1)] | the proposer is heard first | first rung; beats movers too | **survives** |
| No second speech while an unspoken member seeks the floor [42:9(2), 43:13] | equal hearing | admissibility filter against the round-open profile | **survives** |
| Alternation between sides [42:9(3)] | balance | among debaters, by declared side | **survives, strengthened** (declared, not guessed) |
| Chair speaks first on an appeal [42:10, 24:3(5)] | the ruling defends itself | only with a chair-mind | **conditional** |
| Reconsider-for-amendment preference [42:11–12] | the announced purpose is reachable | computable from the reconsider's recorded purpose | **survives** (derived; not in toy) |
| Ladders when nothing is pending [42:13] | series, notice, reconsider | computable from state | **survives** (derived; not in toy) |
| Chair in doubt → vote on the floor [42:14] | last-resort neutrality | the device is never in doubt; a council may adopt a vote | **changed**, adoptable |
| Appeal from a wrong assignment [42:15] | correct the chair | point of order against the server's act; revert | **survives** (needs revertible log) |
| One floor grant, several purposes [42:5] | debate then move in one turn | a speech and a motion are two floor acts, two rounds | **changed**: priced in rounds; a policy can carry both, the second deferring to the next round where the speaker's least-recently-floored standing is now worst |
| Large-hall devices [42:16–17] | replace the race | this design is one | **survives** — the book's license |
| Stand aside unless germane [42:16] | the queue follows the question | per-round claims make it automatic: a debate claim is re-formed against the new question each round | **survives** by construction (claims do not persist) |
| Acts in order without the floor [42:18–19, t44–t45] | timely rights | fragile ∪ urgent, admitted ahead of the floor against the round-open state | **survives**; the class is registry + judgment, with fragility as a diff tool (§5.1) |
| Interrupting mid-speech [42:18] | stop harm in progress | a speech is atomic; the window is the next round; chunked speeches with a round per chunk are the fair real-time form | **changed**, priced (§8) |
| Speaker keeps the floor through interruption [42:20] | continuity | a floor-holder flag across the ruling round | **survives** (derived; not in toy) |
| "Question!" calls [42:22] | pressure | no channel | **vacuous** |
| Seconding [4:9–14] | screen one-person motions | not required in committee [49:21(2), 50:25]; where required, a non-floor act landing the round after the motion | **survives** |
| Second immaterial once debate begins [4:13] | timeliness | a fragile act's window | **survives** |
| Modify/withdraw before stating [4:19–23] | the maker's property | nil if the server states at once; one targeted round between made and stated restores it | **changed**, a declared choice (§8) |
| Unanimous consent and objection [4:58–59] | skip the vote when no minority | a consent round; one objection settles it; everyone asked | **survives, strengthened** (derived; not in toy) |
| "Reserving the right to object" [4:60] | consult before objecting | a *reserve* response inserts one inquiry round | **survives** (derived; not in toy) |
| Precedence among pending motions [5–6] | what may be moved while what pends | untouched | **survives verbatim** |
| Point of order timeliness [23:5] | challenge at the moment | the first round after the transition; everyone gets it; breach-specific | **survives, strengthened** |
| Reserving a point of order [23:5] | hear the maker first | a reserve response holds the point open one round | **survives** (derived; not in toy) |
| Appeal timeliness [24:8] | before business intervenes | the round after the ruling | **survives, strengthened** |
| Division of the assembly [29, 4:52] | verify a doubtful uncounted vote | every vote is counted and attributed; a retake would be a reconsider without its protections, informed by the tally | **vacuous** — disabled by charter (derived) |
| The inconclusive-vote ladder, "seem to have it" [4:50–51] | make an uncounted vote conclusive | no uncounted votes | **vacuous** |
| Change one's vote before announcement [45:8] | correct a misheard vote | 45:8 itself exempts "some other method that provides secrecy"; sealed-until-announced is such a method | **vacuous by the book's own exception** |
| Vote-challenge window [45:9] | one last look | the round after announcement | **survives** |
| Prevailing-side eligibility [37:10(a)] | honor-system in voice votes | every vote attributed; enforceable | **strengthened** |
| Stand at ease [8:2(4)] | a pause | a round with no acts; any objection ends it | **survives** (derived; not in toy) |

Thirty-three rows: 23 survive (5 strengthened), 5 change with a priced or declared equivalent, 4 are vacuous, 1 is conditional. The count is a reading aid over rows I chose; the rows are the content. Seven rows are marked "derived; not in toy" and rest on the book and the model, not on the sim.

## 8. The priced equivalents, and the no-go underneath them

**Mid-speech interruption.** Streaming a speech and letting interrupts land between sentences is unfair under F1 — where the interrupt lands depends on who read faster. The only order-independent form makes each chunk a transition followed by a round. Fair, faithful, and paid per chunk. Default: whole speeches with the next-round window. *(derived.)*

**The maker's modify/withdraw window.** One targeted round between "made" and "stated" in which the maker and informal suggestions [4:20–23] are live. Spend it when the charter asks. *(guess.)*

**One floor grant for several purposes [42:5].** A member who debates and then moves the previous question in one turn now needs two rounds, and after the speech it is the member least entitled by the neutral key. A policy can carry both acts; the server will fire the second when the member next wins the floor. The book's single-grant efficiency is lost; the fairness of the second act's timing is gained. *(derived.)*

**The no-go, stated once.** Any mechanism in which a member's act lands between two other members' acts within a round decides something by arrival order. The round boundary is the only fair place to sequence; everything in §7 that "changed" changed because of that.

## 9. What this does to the server's center

- `resolve` is small and pure: `available_actions`, the registry's interrupt class plus the fragility diff, a commutation probe, the two-stage ladder, least-recently-floored, a seeded slot lottery. No clock, no mind.
- Members send policies; the guard language is the model's expression language restricted as in §4.
- Two record layers: minutes, and a claim log carrying every submitted entry with guard and admissibility outcomes, "silent" and "late" marks, and ballots only as "(sealed)" until announcement.
- One clock door, the presence deadline, a council rule.
- Mechanical chair duties are the server's; judgment (dilatory, urgency, germaneness, substantially-same) is a chair-mind's or the council's. A council can run without a chair-mind by always submitting [23:18–21].

## 10. What this spike did not do, and what it costs

- **The real sweep** over `actions.json` (no evaluator yet). The fragility classification is checked on a nine-act toy as a classification; it has not been shown to order anything.
- **Throughput.** The specimen takes 11 rounds for three speeches, one amendment, one point of order and two votes; each round can wait up to the presence deadline for the slowest member. Slowness is everyone's cost, bounded by the deadline; standing policies reduce who is asked, not how long a round waits. Not measured on a real council.
- **Parallel debate** (councils-46's default): sealed statements revealed together remove the floor from debate entirely, leaving only motions to contend; 42:9(1) and the 42:13 ladders are what it drops. Fair under F1–F4; a different procedure. *(guess: serial is the faithful default; parallel a council rule.)*
- **Presence → absence → quorum** arithmetic; the chair question; the revertible log; targeted rounds and consent rounds in the toy (derived rows in §7 say "not in toy").

## 11. Where councils-46's drafts are narrower than the material, corrected

- `mechanics.md`'s exclusive-win ladder (standing intent → least-recently-floored → seeded) is close to what this revision lands on; its *least-recently-floored* is adopted here. What it drops, under its parallel-debate default, is the maker's first-speech preference [42:9(1)] against a mover and the 42:13 ladders; 42:9(2)–(3) have nothing to order when statements are additive.
- Conditional intents, left open there for staleness, are resolved by guards (§4), which also answer what happens to losing intents.
- The "preemptive" class is curated there; the registry's interrupt attribute plus the adjudicated urgency condition *is* the curated list the model carries, and fragility is only a diff against it (§5.1) — so the draft is less wrong here than revision 1 said.
- Division, the inconclusive-vote ladder, and vote-changing are not addressed there; they are vacuous here, and 45:8 switches itself off for sealed methods.
- The draft's debate rounds come after a consent round, so its "everyone passes → move on" is already silence-after-being-asked; the `stated_round` principle was a toy artifact, not a gap in the draft.
- A point of order should halt the floor for the round; under per-round claims nobody loses standing by it, so the draft's "each opening its own ruling round" needs only the breach-specific timeliness and re-raise bar of §4 step 5 to be obstruction-bounded.
