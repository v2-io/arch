# What simultaneity protects, and what a round must carry instead

*Revision 2, after `de-novo-feedback-1.md` §§1–3. The hardest part, taken first. Every paragraph cited was read whole in `../../RONR-12/body.md`. Labels: **[derived]** = follows from the cited text plus the stated definition; **[declared]** = a rule the charter must supply because it cannot be derived; **[dropped]** = a thing the book provides that this design deliberately does not, with the cost named; **[no-go]** = a mechanism shown to have no function; **[argued]** = my reasoning from the text, not the text's own statement; **[guess]** = mine.*

## 1. The book's position, read whole

RONR makes simultaneity constitutive. A deliberative assembly "meets in a single room or area or under equivalent conditions of opportunity for simultaneous aural communication among all participants" [1:1]; a group that deliberates in writing "does not constitute a deliberative assembly" [1:1 n.1]. The electronic-meeting section is explicit that technology does not change this: "regardless of the technology used, the opportunity for simultaneous aural communication is essential to the deliberative character of the meeting. Therefore, a group that attempts to conduct the deliberative process in writing (such as by postal mail, e-mail, 'chat rooms,' or fax)—which is not recommended—does not constitute a deliberative assembly. Any such effort may achieve a consultative character, but it is foreign to the deliberative process" [9:34]. The reason is given for boards: separately obtained approval is not the board's "since the members lacked the opportunity to mutually debate and decide the matter as a deliberative body" [49:16]. And 9:36's list of rules for electronic meetings governs meetings that already meet the aural standard [9:31, 9:36 "If an organization authorizes … electronic meetings"].

So by the letter, a written, turn-based council is the 9:34 consultative case, and the book's only validity tier for it is 51:2's: a committee that cannot meet "can contain [in its report] what has been agreed to by every one of its members" — unanimity. Revision 1 of this file called 9:36 "a slot RONR left open" and graded majority validity [derived]; both were wrong, and the verifier caught them.

Joseph's instruction stands: the book matters for the principle it surfaces. The principle here is **a decision is the body's only if every member had the opportunity to hear every argument and answer it before the vote closed** [49:16's reason]. Simultaneous aural communication is the human implementation of that principle. So the work is: find everything that implementation carries, and show what a round discipline carries in its place, dropping nothing silently. The result is a **principled departure from 9:34's letter**, and the charters say so where they claim the meeting tier.

## 2. What simultaneity does in the mechanics

| # | Rule | What simultaneity supplies | Cite |
|---|---|---|---|
| a | Timeliness of a point of order: "promptly at the time the breach occurs"; after debate begins it is too late | a shared moment in which the breach is perceivable | 23:5 |
| b | Interruption of a member who "has begun to speak", for listed purposes; the speaker retains the floor | the speaker and interrupter share the moment of speaking | 42:18–20 |
| c | Preference in recognition among those who rose "at about the same time"; otherwise first to rise | a perceivable race | 42:7, 3:30–32 |
| d | Unanimous consent: the chair "pauses, and if no member calls out, 'I object'" | a pause everyone experiences as one window | 4:59 |
| e | Vote windows: division until announcement; vote change until announced; challenges until business intervenes | shared perception of announcement and of what intervened | 29:4(1), 45:8, 45:9 |
| f | Presence and quorum presumed; absence noticed | the room | 40:11–12 |
| g | Debate as exchange: no second speech while one who has not spoken seeks the floor; alternation; "Are you ready for the question?" | perceivable claims and a perceivable silence *after being asked* | 43:13, 42:9, 4:34 |
| h | A motion is the maker's until stated | a gap between making and stating | 4:19, 33:12 |
| i | Yielding the floor by resuming one's seat | a perceivable end of a speech | 3:31 |
| j | Voice and rising votes; the inconclusive-vote ladder | shared perception of an uncounted vote | 4:35, 4:50–51 |

## 3. The replacement: informed acts, and one floor act per round

Revision 1 defined a *common-state condition* over "the event log through event k". The contention-rounds spike showed that formulation admits several acts per round, and that batching breaks informed action (`contention.md` §5.2: a postponement moved blind to an amendment admitted in the same round). Its property is strictly stronger and is adopted here as what the charter declares:

**I1 — informed acts.** Every admitted act was formed against a state that includes every previously admitted act. A round mechanism keeps I1 only if it admits **at most one floor-needing act per round**; non-floor acts that commute (a second, an objection, a vote) may be batched. And **every transition that opens a window is followed by a round** before any chair duty closes it — exhaustion is silence *after being asked*, never silence before. [derived by the contention spike; adopted]

Under I1 the ten loads translate: [derived unless marked]

| # | Under informed acts, one floor act per round | Status |
|---|---|---|
| a | A breach is visible at the state in which it occurred; a point of order in the first round addressed to that state is "at the time of the breach"; later rounds are late (continuing breaches excepted, 23:6). The window tokens the model leaves informal (`gaps.json event-scope-sugar`: "just-now", "since-ruling") get a **definition** here, not a translation. | exact, by definition |
| b | A speech is one submitted statement, revealed whole. Nothing can interrupt it *in progress*, because nothing is in progress: interrupt-class acts in the same round were formed without knowledge of it. What survives is the point of order in the next round, after the whole speech is on the record, which the record can strike. The contention spike's chunked speeches (a round per chunk) are the priced real-time alternative. | **[dropped]**: mid-speech interruption; cost: the remainder of a bad speech is read |
| c | All claims in a round are "at about the same time" [42:7]; the preference ladders decide what they decide in the book; the first-to-rise residual is replaced by a declared tiebreak (earliest claim round, then a rotation pointer — the contention spike's F3). | ladders exact; residual **[declared]** |
| d | The pause is one consent round; any objection defeats consent and the regular cycle runs [4:59]. No ground is required. | exact |
| e | "Until announced" = until the vote-round's reveal is applied; "until business intervenes" = until the first admitted non-challenge act after it. | exact |
| f | Presence is declared at call to order and maintained by submission; a seat that has not submitted at the backstop is absent for the round. **The backstop decides presence**, and presence feeds quorum and the threshold drops of 35:2(7)/37:35(3); latency invariance holds inside the backstop (contention F4). | exact; backstop **[declared]**, and it does decide something |
| g | "Nobody rises" = a closed round in which no present seat claimed the floor for debate, *after* the question was stated or a speech was given. The model's `put-question` third disjunct is satisfied with no round at all (feedback §3.2), so the charter adds a **duty-sequencing rule**: derived chair events fire only at round boundaries, and `put-question` only after such a closed round. | exact given the declared sequencing; model owes a timeliness conjunct (F11) |
| h | A motion is revealed in the round it is made and stated at the start of the next, keeping the maker's free window [4:19]. | **[declared]** choice (the contention spike would spend that round only when a council's rules ask) |
| i | The floor is yielded at reveal; a speech is atomic. | translation |
| j | Every vote is a recorded intent, counted exactly; the ladder has no step to take. | **[no-go]** (§5) |

## 3.5 What aural simultaneity provides beyond the ten rows

The verifier offered four loads the table missed. Each is taken up; the hunt is not proven exhaustive.

1. **Interruption of a speech in progress.** Row b: dropped and declared; the priced alternative exists.
2. **Hearing votes in sequence.** In a roll call members hear earlier votes and may change their own until the announcement [45:8]. A sealed signed ballot keeps 45:8's right in form and empties its trigger. **[dropped, declared]**: for independence — the estate's standing concern that agreement among same-substrate agents is coherence, not evidence. The adoptable alternative is a change round after a provisional tally (the contention spike's §8); default off, because it re-opens the herding channel the sealing exists to close.
3. **Answerability of the final word.** If each vote carries its reasons, those reasons are formed after debate has closed and nobody can answer them — exactly what the principle in §1 forbids. **[carried]**: the charters' `report-form` instruction requires that reasons be given *in debate*; a vote carries only a pointer to the seat's own statements. The report still shows every seat's reasons, because they are on the record where they were answerable.
4. **The parent's debate budget.** A token budget can run out one speech after a new argument. The book allows the parent to impose it [13:8(d)], and it puts a hole in the principle. **[carried]**: the budget instruction carries a reply rule — when the budget is exhausted, one reply round follows in which each seat may speak once (the one-speech rule of 24:3(5), borrowed), so no argument made in the last budgeted speech goes unanswerable.

## 4. Validity tiers, stated honestly

- **By the letter** [derived from 9:34, 51:2]: a written turn-based council is consultative; its report may contain only what every seat agreed to.
- **By principle** [argued, departing from 9:34's letter]: under I1 with the declared residuals and the two declared drops, every mechanism that aural simultaneity carries is either carried or dropped with its cost named; on that basis the charters claim the meeting tier (majority decisions as the council's). Whether the book's authors would accept it is not a question the text answers, and the charters record the departure where they make the claim.
- The two tiers remain useful as a design fact: a council that cannot run rounds (plain polling) is in the letter's tier and may report only unanimity; a council that runs rounds claims the principled tier. Joseph's "veto outside the tool" is the parent's power to decline a report [51:10–14] and, for a council that acted beyond its charter, to ratify or not [10:54–55]. Delegation-as-abdication is, in these terms, staff action the steward has been ratifying after the fact. [derived]

## 5. The no-go, demonstrated

**Claim: division of the assembly [29], the inconclusive-vote ladder [4:50–51], and tellers [45:26–30] have no function in a council whose every vote is a recorded intent.**

Attempt 1 — division as a recount: a recount of recorded intents is deterministic; a discrepancy is a server defect, which is a point of order on the conduct of the vote [23:5, 45:9], already in the action space. Attempt 2 — division as a re-vote: division compels an uncounted *retake* [29:5] and changes a result only because the first count was unreliable; a second vote after seeing the tally would be a reconsideration without 37:10(a)'s protection. Attempt 3 — tellers as the server: the role's content is adjudicating ballot meaning (`adjudication.json ballot-meaning`); a structured intent has none. Conclusion: no function; the charters disable them. Adversarial case: a secret method would re-create an unattributed vote, but the sealed intents are still recorded, so the server's count stays uncontestable; the no-go holds for any method the server records. [derived]

## 6. What is gained, and what only becomes possible

- **The signed ballot.** A sealed round-vote, recorded per seat and revealed together, is the book's **signed ballot** [45:46]: "the voter writes 'yes' or 'no' on the ballot and signs it. The votes can be recorded in the minutes just as a roll call would be." Revision 1 said the book "could never have both"; it has this one, and the standing charter had cited it. The ballot's *secrecy* purpose [45:18] is not delivered and is dropped by design: per-seat votes read by the steward are a channel for voting toward the principal, a cost the charters name (and an untaken option: seal per-seat votes from the parent until the parent has acted). The engine classifies the method as `roll-call` so that `change-vote` (admissible until the reveal, 45:8) and `prevailing_members` (recorded for roll-call only) behave; the enum wants a value of its own (F13).
- **The prevailing-side rule can become enforceable** (`RONR-OBSERVATIONS.md` §3 predicted it). It is not enforceable by rounds alone: `guard/mover-was-prevailing-side` as compiled has no committee branch (37:35(2): anyone *not on the losing side*, including one who did not vote) and keeps the adjudicated assertion path even when votes are recorded. With the committee disjunct added and the recorded case made exclusive (F12), the rule is computed on every recorded vote. Available to the model; not yet in it.

## 7. What this settles

Every mechanism in the book that rests on simultaneity has, under informed acts with one floor act per round, a defined counterpart, a declared residual, a declared drop with its cost, or a demonstrated vacuity. The principle — mutual deliberation before decision — is carried by the round, not by the clock. The departure from 9:34's letter is real and is written down where the charters claim what the letter withholds.
