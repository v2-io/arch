# Contention without a clock: how agents can share a floor fairly

*Spike result, 2026-11-03. Claude (Fable 5.1), spiker. Not verified by anyone else yet; the verifier is councils-46's to commission. Labels: **derived** = argued from the model or the book with the cite; **checked** = exercised in `sim/`; **guess** = my judgment, marked so you can discount it. RONR cites are §:¶ into `../../RONR-12/body.md`, which I read for every cite below.*

## 1. The question, and the question under it

The question as posed: RONR settles who speaks next by a human chair's perception of who rose first; for agents answering through APIs, "first" is latency. What mechanism collects, combines, sequences and resolves members' concurrent intents so that the outcome does not depend on arrival order and is still as fair as RONR means it to be — and how much of RONR's contention machinery survives it?

The question under it, which I found by asking what RONR's "rose first" was *doing*: RONR's whole contention layer presupposes **simultaneous aural communication**. The book is explicit that this is constitutive, not incidental. A deliberative assembly "meets in a single room or area or under equivalent conditions of opportunity for simultaneous aural communication among all participants" [1:1]; an electronic meeting keeps its deliberative character only if it provides that [9:31]; and a group that tries to deliberate in writing "does not constitute a deliberative assembly … it is foreign to the deliberative process as understood under parliamentary law" [9:34]. So the book itself says the agentic medium — sealed responses, delivered in writing, at different times — is outside its scope. That is not a defeat; it tells us exactly what has to be rebuilt. Simultaneous aural communication did three jobs:

1. **Common state.** Everyone perceives the same state at the same moment, so a claim is made against a state everyone shares.
2. **A real-time interrupt channel.** "Point of order!" reaches everyone the instant it is uttered, so a timeliness window ("at the time of the breach" [23:5]) can be as narrow as a breath.
3. **Contention resolved by perception.** The chair sees the claimants and, after the preference rules, the race.

(The charter-agenda sibling spike reached this same foundation independently and from further cites — 1:1 n.1, 49:16 on separately-obtained board approval, 51:2 on the unanimity fallback for committees that cannot meet — and named the round's property the *common-state condition*; I take the convergence as evidence the foundation is the book's, not either spike's.)

The agentic server must supply (1) and (2) by construction and replace (3) by a rule. The finding of this spike is that it can do all three, that the result is *more* protective than RONR in several places, and that the places where it is less are few, named, and each have a priced equivalent. Section 7 is the table.

## 2. What "fair" means here

councils-46 proposed: *the record of a meeting is identical under any permutation of when members' responses arrive.* I keep it and sharpen it into four parts, because permutation-invariance alone is satisfied by rules RONR would never accept (resolve every tie in roster order: invariant, and systematically unequal).

- **F1 — order-independence.** Within a round, the transition and the record are a function of the *set* of (member, response) pairs received, never of their order. *(checked: `test_f1_invariance`, 200 random states × 18 permutations each, identical fingerprints.)*
- **F2 — universal opportunity.** Every present member is asked at every state. No act that is in order at a state can go out of order for a member before that member has been asked at that state. RONR does not give this: a slow riser misses the window. *(derived: it is the round structure itself, §4.)*
- **F3 — neutrality.** When RONR's preference rules do not decide, the tie is broken by a rule symmetric across members — claim age measured in rounds, then a rotation pointer — never by arrival, and recorded so it can be audited. *(checked: `test_f3_claim_age`.)*
- **F4 — wall-clock enters only through presence.** The only way time touches the record is a per-round presence deadline, sized in minutes rather than seconds, whose effect is "this member did not respond this round" — the same fact RONR records when a member has left the room. A late response is logged as supplementary, never silently dropped. *(checked: `test_f4_deadline`.)*

And one property that is not about arrival at all but turned out to be the hinge of the design:

- **I1 — informed acts.** Every admitted act was formed against a state that includes every previously admitted act. RONR's serial floor gives this for free: you make your motion after hearing what was just stated. A round mechanism keeps it only if it admits at most one *floor-needing* act per round (§5.2). *(checked: `test_i1_informed_acts`.)*

Joseph's constraint "an agent shouldn't be penalized because its API took longer" is F1 plus F4: order never matters, and lateness only matters past a deadline that is a presence rule, not a race.

## 3. What RONR's race actually decides, so we know what to replace

Reading 3:30–34 and 42:1–22 whole, the floor-assignment rule has three layers, and only the bottom one is wall-clock:

1. **Interrupt-capable acts** jump the queue entirely: they are in order while another has the floor [42:18–19; t44–t45]. Points of order, questions of privilege, calls for the orders of the day, division, appeal, objection to consideration, requests needing immediate response, and — "if there may be no other opportunity" — giving notice and making (not taking up) a reconsider [42:7, 42:19].
2. **Preference in recognition** among those who rose before anyone was recognized [42:7]: the maker's first speech [42:9(1)], not-yet-spoken over already-spoken [42:9(2)], alternation between sides [42:9(3)], the chair's first word on an appeal [42:10], the retake-the-vote mover just after a result [42:8], and the no-question-pending ladders [42:13].
3. **The race**: "the member who rose and addressed the chair first *after the floor was yielded*" [42:6], with the explicit anti-queue-jumping rule that rising before the floor is yielded establishes no claim [3:32, 42:6]; chair in doubt → the assembly votes and plurality takes the floor [42:14]; wrong assignment → point of order, appealable [42:15].

Layer 1 is computable from state and the registry (`guard/in-order-when-other-has-floor`). Layer 2 is computable from state (`protocol.json preference_in_recognition` already compiles it; alternation needs members to *declare* their side when claiming, which the book itself suggests with colored cards and for/against microphones [42:9(3)]). Only layer 3 is perception, and the book treats it as the arbitrary residue: it allows large halls to replace it with numbered microphones, lights, and lists [42:16–17] — i.e. with *whatever neutral ordering device the hall affords*. So replacing the race with a neutral device is not a departure from RONR; it is something RONR expressly contemplates. That lowers the bar for "as fair as RONR means": the race was never the fairness; the preference rules and the equal right to be heard were.

## 4. The mechanism

The server keeps the model's state `S` and runs rounds. A round is one state transition's worth of contention.

**Ask.** Every present member whose standing policy does not already cover `S` is sent `S` (as a delta), the record since it last looked, and its `available_actions` with justifications — the model's own function. A member who need not be asked is one whose last response said, in effect, "pass unless one of these things happens," and none has.

**Respond.** A member returns a **policy**: a prioritized list of (guard, act). A guard is an expression over the state — the model's own expression language will do (`["=", ["top"], q]`) — and an unguarded act carries the implicit guard "the state I saw is still the state in the respects this act depends on" (top frame, phase). A policy may carry several non-floor acts and at most one floor act, and may be marked *standing* (keep consulting it in later rounds without asking me). *Pass* is a real response.

Why policies rather than single intents: Joseph's intuition was that "what arrives from members in one step may call for routing that depends on what arrived." Guards are the routing, written by the member rather than guessed by the server. A member can say "second any motion to postpone; if nothing is pending, move X; if an amendment to X is pending, speak against it" and the server does the right thing in whichever of those states obtains, without re-asking and without ever executing something the member did not decide. The staleness worry councils-46 raised about conditional intents dissolves once the condition is explicit: a stale act is one whose guard has failed, and it is simply not fired.

**Close.** The round closes when every asked member has responded, or when the presence deadline passes (F4). For rounds whose outcome is already determined by the responses in hand (one objection to unanimous consent settles it; a point of order halts the floor), the server may close early and keep logging the stragglers as supplementary; the minutes are unaffected, and the claim log carries the "late" mark. *(guess: worth it for consent and objection rounds; never for floor rounds, since an unarrived member might hold preference; never for votes, §7.)*

One rule about closing that a sibling spike found by measurement rather than argument: the walking-skeleton server, run over 50 shuffled arrival orders, produced one canonical log when a seat's first submission was final and **four distinct logs when a seat could replace its submission**, because a replacement races the "everyone is in" close. The derivation from F1 is immediate once seen: *the close condition must be a function of each member's own declared-final state, never of a count of messages received.* So either a response is final on arrival (one policy per round, which is how this spike frames it — the policy carries the member's contingencies, so it has little reason to replace), or replacement is allowed but a member must explicitly finalize, the round closes only when every asked member has finalized or the deadline passes, and a replacement after one's own finalization is refused. Both make the record a function of the set of final responses. "Replaceable until everyone is in" is neither, and the measurement is the no-go for it. *(derived; measured by `../spike-walking-skeleton-2026-11-03/concilium/steward.py selftest`.)*

**Resolve** — the pure function `resolve(S, profile) → (S', record, claims)`, in this order:

1. *Live acts.* For each member, every entry whose guard holds and whose act is admissible at `S`; all its live non-floor acts, and its first live floor act. Every entry is written to the claim log whether or not it fires — the equivalent of "rose but was not recognized," which 42:9(2) and the second-speech bar [43:13] need to know.
2. *Votes.* If a vote is in progress, only votes are admissible [45:6]; they are recorded sealed and the result is announced when every present member has voted.
3. *Fragile acts first.* An act is **fragile** at `S` if it is admissible now and would become inadmissible after the ordinary course of business — another member's speech, or the chair proceeding to the next duty. Fragile acts are admitted ahead of the floor, in canonical order, each re-checked. This class is *derived by simulation from the model*, not curated (§5.1).
4. *Commuting non-floor acts.* Of the remaining non-floor acts, those that pairwise commute (both orders admissible, same resulting state — checked by applying them both ways) are admitted as a batch in canonical order. Any that do not commute compete for the floor slot.
5. *A point of order halts the floor.* If one was admitted, no floor act this round; the point is disposed of first [23:2(1)] — by the chair-mind if the council has one, else submitted to the council [23:18–21].
6. *One floor act.* Among the floor-needing candidates, exactly one is admitted, chosen by: the maker's first speech on the immediately pending question [42:9(1)] → alternation by declared side [42:9(3)], applied only when every candidate is a debate claim → earliest claim round (RONR's "rose first," measured in rounds) → rotation pointer. The losers keep their claim age. The rotation pointer advances past the winner. Two things the ladder deliberately does *not* do: it does not rank an unspoken debater above a mover (the book ranks debaters among debaters at 42:9(2) and says nothing about a mover versus a debater — that is the race), and it carries 42:9(2) as an admissibility filter on the *second speech* [43:13] rather than as a rung, so it can never promote a debater over a mover either. My first version got both wrong, and the specimen run exposed it: a member claiming to speak was beating a member claiming to move an amendment on a "not yet spoken" rung the book does not have for that pair. *(checked after the fix: the specimen's round 4 now resolves on claim age, which is the right reason.)*
7. *The chair's duties.* Where the model says exactly one chair act is due (state a motion; put an undebatable question; announce; the no-objection adoption; "no second"), the server does it, because `actions.json` already says that where one chair action is admissible "the list is also the obligation."

**Reveal.** `S'`, the admitted acts with their justifications, and the full claim log (admitted or not) go to everyone. Then the next round.

Two rules that fell out of making this faithful:

- **Every transition that opens a window is followed by a round.** A motion stated this round is not put this round, even if nobody claimed debate, because nobody was yet asked at a state where it was pending. (The sim's first version got this wrong and put a motion in the round it was seconded; `stated_round` fixed it. That bug is the general principle showing up: exhaustion of debate is derived from *silence after being asked*, never from silence before.)
- **A guard that becomes true only because of an act admitted this round does not fire a floor act this round.** It becomes a claim next round, competing fairly with members freshly asked. This is RONR's "rising before the floor is yielded establishes no claim" [42:6], translated: a pre-committed policy must not queue-jump members who are reacting to what just happened. Non-floor fragile acts (a second, an objection) *do* fire in-round, which is calling out "Second!" from one's seat [4:9].

## 5. The two derivations

### 5.1 Fragility is RONR's interrupt list, computed

RONR's interrupt-capable acts are hand-listed [42:18–19] and encoded per motion in the registry. Define instead: *an act is fragile at `S` if admissible at `S` and inadmissible after the ordinary course of business proceeds without it.* In the toy this reproduced the book's list for every act the toy has: second (the chair declares "no second" [4:10]), point of order (breach no longer "at the time" [23:5]), objection to consideration (debate begins [26:2(1)]), division (business intervenes [45:9]); and *not* amend, not previous question, not a debate speech. *(checked: `test_d1_fragile_set`.)*

The first version of the probe was too wide: it counted a competing higher-ranked motion as "the ordinary course," which made *amend* fragile (a previous question would preclude it). That is wrong by RONR's own lights — being outranked is contention, resolved by the ladder, not a timeliness window — and narrowing the probe to "a speech, or the chair proceeding" fixed it. The narrowing is itself the finding: **RONR's interrupt class is the set of acts whose window the ordinary flow of business closes**, and the book says this in its own words at the one place it makes interruption conditional — notice and reconsider may interrupt "if there may be no other opportunity" [42:7, 42:19]. That is a fragility test stated in English.

What this buys the server: the class is computed from `actions.json` + the applicability table by trying the transition, so it cannot rot when an action is added, and it will reproduce RONR's list *and* catch any act the book's list forgot. **Owed:** the sweep over the real model (every action in `actions.json`, at each state class, against `speak-in-debate` and the chair's duties) and a diff against t44–t45 and 42:18–19. I could not run it — the model's evaluator is stage 5 and does not exist yet — so the claim stands at the toy's scale, labeled.

### 5.2 One floor act per round is forced, not chosen

Could the server admit several floor acts in one round, ordered by precedence rank, to save rounds? It would be order-independent (F1) and neutral between members (F3). It fails I1. Construct: A moves to amend, C moves to postpone, same round, both in order. Batch-by-rank admits postpone first, then re-checks amend: not in order while postpone is pending; dropped; A is re-asked. Batch-by-rotation admits amend first, then postpone: in order, admitted — but C's postponement was formed without seeing the amendment, and C, had it seen it, might have preferred to speak against the amendment or to amend the amendment. In RONR, C would have risen again *after* hearing "it is moved and seconded to amend…" and chosen then. Under batching, C is bound to a blind act. With one floor act per round and a guarded policy, C's own routing takes over: `test_i1_informed_acts` has C's policy say "postpone if only the main is pending; if an amendment is on top, speak against it," and when A's amendment wins the round, C's next-round act is the speech, not the postponement. Nothing ran that C had not decided for the state it ran in.

So the cost of fairness here is one floor act per round of polling, and the mitigation is standing policies (members with a covering policy are not asked) and early close for determined rounds — not batching.

Corollary worth naming: **precedence among pending motions is untouched.** The stack machine, the yield matrices, the series-halting adoptions — none of that is contention; it is what the state does once an act is admitted. Simultaneity only touches *admission*, and admission is now one act per round under the ladder. What changes is only the arbitrary element the race supplied, and §3 argued the race was never the fairness.

## 6. Adversarial cases, run on purpose

- **Two primary amendments in one round.** Do not commute (only one primary may pend [12:13]); one wins by ladder/claim-age/rotation; the other keeps its claim and is re-asked with the first amendment pending, where its act is now a secondary amendment or a speech — its choice. *(checked: `test_d2_commutation`.)*
- **Point of order and a motion in one round.** The point is fragile, goes first, and halts the floor; the motion's maker keeps claim age; the point is disposed of (chair or council) and the motion is re-asked. This is 23:2(1) exactly. The maker is not deprived: the breach-window had to come first or it would have closed. *(checked in `specimen()`.)*
- **Five members all want to speak, same side.** Maker first [42:9(1)]; then each unspoken member before anyone speaks twice [42:9(2)]; sides declared alternate [42:9(3)]; ties by claim age then rotation. Arrival never read. *(checked: `test_f3_claim_age`.)*
- **A member pre-commits "if anyone moves to close debate, I move to adjourn."** Faithful handling: the guard becomes true mid-round; the adjourn does not fire this round; next round it is one claim among others, including members reacting fresh. Without that rule a policy is a way to rise before the floor is yielded. *(derived, §4; not separately tested — the sim's guards are evaluated at round open only, which enforces it structurally.)*
- **"I vote with the majority."** A vote guard on the tally would be a different voting rule (sequential, bandwagon-enabled). Guards may reference the state at round open and the structural effects of acts admitted this round, never the sealed contents of this round. *(derived; a constraint on the guard language the server must enforce.)*
- **A member lurks until the rotation favors it.** Claim age is first-claim-round among continuous claimants, so a member that stops claiming resets its age; lurking gains nothing over a member that has been claiming. *(derived from the aging rule; the sim resets claim age when a member does not claim in a round.)*
- **A member that never responds.** After the presence deadline it is silent for the round and logged so; after a council-configured number of consecutive silences it is absent, quorum is recomputed, and the chair's duty on noticing the absence of a quorum fires [40:11–12]. The one wall-clock door, and it is RONR's own: a member who has left the room.
- **The server misassigns the floor (a bug).** Its mechanical acts must be appealable like a chair's [42:15]: a point of order against the server's act, ruled by chair-mind or council; if sustained, the server reverts to the pre-act state. This needs the event log to support reverting a transition, which an append-only log with a "reverted" event gives.

## 7. What survives, what changes, what is vacuous, what has no fair equivalent

| RONR contention device | Its job | Under rounds | Status |
|---|---|---|---|
| Rising and being recognized [3:30–31, 42:2] | mutual exclusion on speech | one floor act per round, chosen by ladder | **survives**, with the race replaced (§3) |
| "Rose first after the floor was yielded" [42:6] | the tie-break | earliest claim *round*, then rotation | **changed**: same structure, fair clock |
| "Rising before yield gives no claim" [3:32, 42:6] | no queue-jumping | claims count from the round after the floor opens; in-round-triggered floor acts defer | **survives** (translated to policies, §4) |
| Preference: maker, unspoken, alternation [42:9] | equal hearing | computed from state; side is declared on claiming | **survives, strengthened** (no chair guesswork on sides) |
| Preference ladders when nothing is pending [42:13] | series, notice, reconsider | computed from state (series-declared-open, notice registry) | **survives** |
| Chair speaks first on an appeal [42:10, 24:3(5)] | the ruling defends itself | only if the council has a chair-mind; vacuous under submit-to-council | **conditional** |
| Chair in doubt → vote on the floor [42:14] | last-resort neutrality | the ladder+rotation is never in doubt; a council may adopt a vote instead | **changed**: the device is no longer needed, but is adoptable |
| Appeal from a wrong assignment [42:15] | correct the chair | point of order against the server's act; revert | **survives** (needs revertible log) |
| Large-hall adaptations [42:16–17] | replace the race with a device | this whole design is one | **survives** — the book's own license |
| Interrupt-capable acts [42:18–19, t44–t45] | timely rights without the floor | the fragile class, derived (§5.1), admitted ahead of the floor | **survives, derivable** |
| Interrupting *mid-speech* [42:18] | stop harm in progress | a speech is atomic; the window is the round after it. Fair real-time equivalent: chunked speeches with a round between chunks, each chunk paid in polling | **changed**; real-time form has a priced equivalent, not a free one (§8) |
| Speaker retains the floor through interruption [42:20] | continuity | floor-holder flag persists across the ruling round | **survives** |
| "Question!" calls, calling out adjourn/table [42:22] | pressure the chair must resist | no channel exists for it | **vacuous** |
| Seconding [4:9–14] | screen one-person motions | not required in committee [49:21(2), 50:25]; where required, a non-floor fragile act, batched | **survives** (committee: moot) |
| Second immaterial once debate begins [4:13] | timeliness | derived as fragility | **survives** |
| Modify/withdraw before stating [4:19–23] | maker's property | with the server stating at once, the window is the round in which the motion was made — i.e. nil. Keep it by not auto-stating: one round between made and stated | **changed**; a council choice (§8) |
| Unanimous consent and objection [4:58–59] | skip the vote when no minority | a consent round; one objection settles it; everyone is asked | **survives, strengthened** |
| "Reserving the right to object" [4:60] | consult before objecting | a *reserve* response defers the consent round by one inquiry round | **survives** |
| Precedence among pending motions [5–6] | what may be moved while what pends | untouched; it governs admissibility, not contention | **survives verbatim** |
| Point of order timeliness [23:5] | challenge at the moment | the round after each transition *is* the moment; everyone gets it | **survives, strengthened** |
| Appeal timeliness [24:8] | before business intervenes | same | **survives, strengthened** |
| Division of the assembly [29, 4:52] | verify a doubtful voice vote | every vote is an exact, attributed, sealed count; a retake reproduces it | **vacuous** — its purpose dissolved, not suppressed |
| The inconclusive-vote ladder [4:50–51] | voice → rising → counted | every vote is counted | **vacuous** |
| Change one's vote before announcement [45:8] | correct a misheard vote | sealed simultaneous votes have no "before announcement" with knowledge of others; a change-window after a provisional tally is possible but is a new procedure that invites bandwagon | **changed**: default off; adoptable (§8) |
| Vote-challenge window [45:9] | one last look | the round after announcement | **survives** |
| Prevailing-side eligibility for reconsider [37:10(a)] | honor-system in voice votes | every vote is attributed; the rule is enforceable for the first time (as `RONR-OBSERVATIONS.md` 3 predicted) | **strengthened** |
| Chair's "seem to have it" pause [4:50] | invite a division | nothing to invite | **vacuous** |
| Stand at ease [8:2(4)] | a pause without recess | a round with no acts; any objection ends it | **survives** |

Count, for honesty rather than as a score: of 29 rows, 17 survive (6 of them strengthened), 6 change with a priced or adoptable equivalent, 5 are vacuous because their purpose dissolved, and 1 (the chair's appeal preference) is conditional on a chair-mind. None landed as "no fair equivalent" after a full attempt. The two I expected to land there did not: mid-speech interruption has a fair equivalent (chunking, §8), and the race has a fairer one.

## 8. The priced equivalents, and the no-go underneath them

**Mid-speech interruption.** A full attempt at a real-time equivalent: stream the speech; let interrupt-class acts land between sentences. This is unfair under F1 — *where* the interrupt lands depends on who read and responded faster. The only order-independent form is to make each chunk a transition followed by a round, so every member is asked at every chunk boundary. That is fair and faithful; it costs one round per chunk, and a speaker's length becomes a cost to the council in polling. So: **no-go for unchunked real-time interruption; priced equivalent by chunking; default to atomic speeches with the post-speech window** (what is lost is only the power to stop the remainder from being read, and the record can strike it). *(derived.)*

**The maker's window to modify or withdraw before stating.** The server states instantly, so the window is nil. Restoring it costs one round between "made" and "stated" in which only the maker (modify/withdraw) and informal suggestions [4:20–23] are live. In committee, where there is no second to wait for, I would spend that round only when the council's rules ask for it. *(guess.)*

**Vote change before announcement.** Possible as "provisional tally revealed, one change round, then announcement." It respects 45:8 and enables the herding councils-46 wants sealed votes to prevent. Default off. *(guess, with the reason given.)*

**The real no-go, stated once.** Any mechanism in which a member's act can land *between* two other members' acts within the same round is a mechanism in which arrival order decides something. The round boundary is the only place sequencing can be fair, and the price of fairness is that nothing happens between rounds. Everything in §7 that "changed" changed because of this one fact.

## 9. What this does to the server's center

- `resolve` is small and pure: the model's `available_actions`, a fragility probe, a commutation probe, the preference ladder, claim age, a rotation pointer. It needs no clock and no mind.
- Members send policies, not single intents; the guard language is the model's expression language restricted to round-open state plus structural in-round effects.
- The record has two layers: the minutes (admitted acts with justifications, announcements) and the claim log (everything submitted, fired or not, including "late" and "silent" marks). The minutes are F1-invariant; the claim log is F1-invariant except for the supplementary labeling, which carries no rights.
- The presence protocol is the only clock, and it is a council rule.
- Mechanical chair duties are the server's; judgment is a chair-mind's or the council's. The council can run without a chair-mind at all by always submitting [23:18–21]; the only RONR devices that go vacuous then are the chair's appeal preference and the chair's sua-sponte noticing of breaches (members' points of order carry the load).

## 10. What this spike did not do

- **The real sweep.** §5.1's fragility derivation and §5.2's commutation partition were run on a toy with nine act kinds, not on `actions.json` with the model's evaluator. The claim that the derived class equals t44–t45 / 42:18–19 is checked for {second, point of order, objection to consideration, division} and refuted for nothing, which is far from a proof over the book. The sweep is owed and is the first thing a verifier should want.
- **Parallel debate.** councils-46's draft defaults to sealed parallel statements revealed together. Under parallel debate the only floor-needing acts left are motions; the preference ladder for *speech* goes unused and alternation becomes moot. I did not analyze which is better for truth; I note only that it is a different procedure (RONR's debate is serial by construction) and that both are fair under F1–F4. *(guess: serial is the faithful default; parallel is a council rule.)*
- **The chair question** beyond §9 — whose it is to rule, and rotation of the role — belongs to the charter spike.
- **Latency cost.** One floor act per round is slow. I did not measure it or design the standing-policy UI that makes it bearable.
- **Quorum arithmetic under the presence protocol**, beyond saying it is RONR's own. The toy's vote rounds wait for every present member; the "silent → absent → quorum recomputed" path that would let a vote close without a dead member is described (§6) but not implemented.

## 11. Where councils-46's drafts are narrower than the material, as asked

- `mechanics.md`'s exclusive-win ladder (standing intent → least-recently-floored → seeded) drops RONR's preference-in-recognition entirely: the maker's first speech [42:9(1)], unspoken-before-spoken [42:9(2)], alternation [42:9(3)], and the 42:13 ladders. Those are the fairness, not ornaments; the race is what to replace. Put the ladder above claim age.
- Conditional and bundled intents are left open there for fear of staleness. Guards resolve it (§4), and they also answer "what happens to the intents that didn't win": they stay standing with their guards, and the member is re-asked only when no guard covers the new state.
- The "interrupt/preemptive" class is curated there; it can be derived (§5.1), and should be, for the reason the roadmap gives for the phase-7 pin list: a curated list rots.
- Division, the inconclusive-vote ladder, the "seem to have it" pause, and vote-changing are not addressed there; they are vacuous or default-off here (§7).
- The draft's "a round in which every seat passes moves the question on" is right, with one guard it lacks: silence counts only in a round *after* the one in which the question was stated (§4, the `stated_round` bug).
- The draft's "preemptive … each opening its own ruling round" is right; add that the point halts the floor for that round and the losing claimant keeps claim age, or the point becomes a way to steal a turn.
