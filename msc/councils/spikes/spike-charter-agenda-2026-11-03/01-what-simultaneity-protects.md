# What simultaneity protects, and what a round must carry instead

*The hardest part, taken first. Every paragraph cited was read whole in `../../RONR-12/body.md`. Labels: **[derived]** = follows from the cited text plus the stated definition; **[declared]** = a rule the charter must supply because the book says it cannot be derived; **[no-go]** = a mechanism shown to have no function under the definition; **[guess]** = mine.*

## 1. The book's objection

RONR's first definition makes simultaneity constitutive. A deliberative assembly "meets in a single room or area or under equivalent conditions of opportunity for simultaneous aural communication among all participants" [1:1]. Its footnote is blunt: "A group that attempts to conduct the deliberative process in writing—such as by postal mail, electronic mail (e-mail), or facsimile transmission (fax)—does not constitute a deliberative assembly. When making decisions by such means, many situations unprecedented in parliamentary law will arise, and many of its rules and customs will not be applicable" [1:1 n.1]. The reason is given for boards: approval "obtained separately … even from every member … is not the approval of the board, since the members lacked the opportunity to mutually debate and decide the matter as a deliberative body" [49:16]. Electronic meetings are admitted only "so long as the meetings provide, at a minimum, conditions of opportunity for simultaneous aural communication among all participating members equivalent to those of meetings held in one room" [9:31].

Joseph's council is written and turn-based by design. Whether the book would *call* it an assembly is not the interesting question (Joseph, mid-spike: "I only care about the book inasmuch as it surfaces underlying foundational principles"). The passage matters because it names a principle the whole system rests on: **a decision is the body's only if every member had the opportunity to hear every argument and answer it before the vote closed.** Simultaneity is the human implementation of that principle. So the work here is to find every mechanism that implementation carries, and show what a round discipline must carry in its place — because any of those that a server drops silently is a fairness hole with a 150-year case history behind it.

## 2. What simultaneity does in the mechanics

Reading the rules that would be "not applicable" under the footnote, simultaneity carries exactly these loads:

| # | Rule | What simultaneity supplies | Cite |
|---|---|---|---|
| a | Timeliness of a point of order: "promptly at the time the breach occurs"; after debate begins it is too late | a shared *moment* in which the breach is perceivable by everyone | 23:5 |
| b | Interruption of a speaker for listed purposes; the interrupted speaker retains the floor and resumes | the speaker and the interrupter share the moment of speaking | 42:18–20 |
| c | Preference in recognition "among members who rose before anyone was recognized"; otherwise first to rise | a race whose order is perceivable by the chair | 42:7, 3:30–32 |
| d | Unanimous consent: the chair "pauses, and if no member calls out, 'I object'" | a pause everyone experiences as the same window | 4:59 |
| e | Vote windows: division "from the moment the negative is cast until immediately after announcement"; vote change "until the result is announced"; challenges "until debate or business intervenes" | shared perception of announcement and of what intervened | 29:4(1), 45:8, 45:9 |
| f | Presence and quorum: continued quorum presumed; absence noticed by seeing the room | the room | 40:11–12 |
| g | Debate as exchange: no second speech while one who has not spoken seeks the floor; alternate sides; "Are you ready for the question?" | perceivable claims and a perceivable silence | 43:13, 42:9, 4:34 |
| h | A motion is the maker's property until the chair states it: free to modify or withdraw | a gap between making and stating that the maker can act in | 4:19, 33:12 |
| i | Yielding the floor by resuming one's seat | a perceivable end of a speech | 3:31 |
| j | Voice and rising votes; the inconclusive-vote ladder (voice → "seem to have it" → rising → counted) | shared perception of an uncounted vote | 4:35, 4:50–51 |

Nothing else in the floor, lifecycle, scheduler, or rules layers depends on simultaneity; they depend on *order* (which event came before which) and on *windows* (open from one event, closed by another), and the model already expresses them that way (`event-occurred`, timers with `expires_on_event`).

## 3. The replacement: the common-state condition

Define a **round** as the server's unit of simultaneity: at state $s_k$ (the event log through event $k$), every present member receives $s_k$ and may submit intents formed against $s_k$; no event $k+1$ is applied until every present member has had that opportunity (or the charter's liveness backstop has marked a member absent for the round — the one wall-clock fact, recorded as a fact). Intents submitted in a round are sealed until the round closes and are revealed together.

**Common-state condition (CSC).** Every act applied at state $s_k$ was formed by a member who held $s_k$; and every present member held $s_k$ before any act at $s_k$ was applied.

Under the CSC the ten loads translate: [derived]

| # | Under the common-state condition | Status |
|---|---|---|
| a | A breach becomes visible at the state in which it occurred; a point of order submitted in the first round addressed to that state is "at the time of the breach"; in any later round it is late (continuing breaches excepted, 23:6). The human "promptly" becomes an exact predicate. | exact, sharpened |
| b | An interrupt-class intent and the speech it would interrupt arrive in the same round; the resolver applies the interrupt first (the registry's `interrupt` attribute, via `guard/in-order-when-other-has-floor`); the speech's author "retains the floor" as the model's `floor.interrupted_holder` and is re-addressed first after the interrupting matter is disposed [42:20]. | exact, given a resolver rule |
| c | All claims in a round are "at about the same time" [42:7], so the preference ladders (`protocol.json preference_in_recognition`) decide everything they decide in the book, and the first-to-rise residual is replaced by a **declared** tiebreak. The book itself puts that residual in the electronic-meeting rules: "methods for seeking recognition and obtaining the floor" [9:36]. | ladders exact; residual **[declared]** |
| d | The pause is one consent round; any objection in it defeats the consent and the regular cycle runs [4:59]. | exact |
| e | "Until announced" = until the vote-round's reveal event is applied; "until business intervenes" = until the first non-challenge act after the reveal is applied; both are history queries the model already makes (`timer/vote-challenge-window`, `guard/result-window-open`). | exact |
| f | Presence is declared at call to order and maintained by participation in rounds; a member that does not submit before the liveness backstop is `absent` for that round (not `passed`), quorum is recomputed, and the inquorate action set [40:7] governs if it falls below. The book's own sample rules do this ("the continued presence of a quorum shall be determined by the online list of participating members" — Sample Rules A §4). | exact; backstop **[declared]** |
| g | A round in which no present member claims the floor for debate is "no one rising"; the model's `put-question` already derives exhaustion as "no timely floor claim for debate" — the round makes the predicate literal. The second-speech bar and alternation are resolver rules over the round's claims. | exact |
| h | With a chair that states instantly the gap vanishes. Either the server reveals a motion one round before stating it (restoring the maker's free-modify window), or the charter declares immediate stating and routes modification through the consent/vote path of 33:11–19. Both are legitimate; the first keeps 4:19 intact. | **[declared]** choice |
| i | A speech is one submitted statement; the floor is yielded at reveal. A member cannot hold the floor across rounds unless the charter says so. | translation |
| j | Every vote is cast as a recorded intent and counted exactly. The ladder exists only to make an uncounted vote conclusive; with no uncounted votes it has no step to take. | **[no-go]** by vacuity (§5) |

So every load simultaneity carries has either an exact counterpart under the CSC, a counterpart with a residual the book itself assigns to adopted electronic-meeting rules [9:36], or no function left. The "many situations unprecedented in parliamentary law" of the footnote turn out, under the CSC, to be the four **[declared]** items — and the book's own list of what electronic-meeting rules should cover is nearly that list:

> rules relating to: the type of equipment or computer software required for participation … and contingencies for technical difficulties or malfunctions; methods for determining the presence of a quorum; the conditions under which a member may raise a point of order doubting the presence of a quorum …; methods for seeking recognition and obtaining the floor; means by which motions may be submitted in writing during a meeting; and methods for taking and verifying votes. [9:36]

That paragraph is the charter section Joseph's "turn-based, wall-clock ambivalent" concern lands in. It is not an agentic invention bolted onto RONR; it is a slot RONR left open and labeled.

## 4. Where the authorization comes from

An assembly or board may hold electronic meetings only if the bylaws authorize it [9:30]. A committee may be authorized "by a standing rule of the parent body or organization, by the motion establishing the particular committee, or by instructions included in a motion referring an individual matter to the committee" [9:35]. So for a council (a committee), the authorization and the electronic-meeting rules belong in the **establishing act** — the charter — and the parent (Joseph, or a parent council) is the right author. [derived]

Two consequences worth holding:

1. **The book's own fallback for asynchronous committees is unanimity.** "In the case of a committee, however, if it is impractical to bring its members together for a meeting, the report of the committee can contain what has been agreed to by every one of its members" [51:2]. A set of agents polled separately, with no round discipline, can still produce a valid *committee* report — but only of what every one of them agreed. Majority decisions require a meeting, i.e., the CSC. This gives a two-tier result the charter can state: **rounds → majority is valid; polling → unanimity only.** [derived]
2. **Joseph's veto has a name in the book.** Action "taken by officers, committees, delegates, subordinate bodies, or staff in excess of their instructions or authority—including action to carry out decisions made without a valid meeting, such as by approval obtained separately from all board members" can be cured only by the parent's *ratification* [10:54]. Delegation-as-abdication is, in these terms, staff action the steward has been ratifying after the fact. A council moves the decision into a valid meeting so that what reaches the parent is a report to adopt or decline [51:10–14], not an act to ratify. The veto outside the tool is the parent's power to decline adoption; ratification remains available for a council that acted beyond its charter.

## 5. The no-go, demonstrated

**Claim: division of the assembly [29], the inconclusive-vote ladder [4:50–51], and tellers [45:26–30] have no function in a council whose every vote is a recorded intent.**

Attempt 1 — read division as "demand a recount." A recount of recorded intents is deterministic; a discrepancy could only be a server defect, which is a point of order on the conduct of the vote [23:5 last sentence, 45:9], already in the action space.

Attempt 2 — read division as "demand a re-vote." Division compels an uncounted *rising retake* of the same vote [29:5]; it changes the result only because the first count was unreliable. Votes may not be changed after announcement [45:8]; a second ballot after seeing the tally would be a reconsideration without reconsider's protections (prevailing-side mover, 37:10(a)). So a re-vote reading is not division; it is a worse reconsider.

Attempt 3 — read tellers as the server. The server counts, but "teller" in the model is a *role* whose content is adjudicating ballot meaning (`adjudication.json ballot-meaning`, `raisable_by` tellers first). A structured intent has no ambiguous meaning to adjudicate. The role is empty.

Conclusion: these three have no function; the charter disables them. Rights-critical status (the phase-7 pin list names division) is discharged by the stronger property that every vote is published per seat. **The adversarial case outside the motivating family:** a council that *chooses* a secret method (if an estate ever allowed one) would re-create an uncounted-by-seat vote; division would then regain a purpose only if the server's count were contestable — which it is not, because the sealed intents are still recorded. So the no-go holds for any method the server records, secret or not. [derived]

## 6. What is gained, not just preserved

Under the CSC two things the book treats as mutually exclusive coincide. [derived]

- **Roll call and ballot at once.** A sealed round-vote is independent (no member sees another's vote before casting — the ballot's epistemic property, 45:18ff.) and recorded per seat (the roll call's accountability property, 45:45). The book says roll call "has exactly the opposite effect of a ballot vote" [45:45]; here they are one act. This is what the coordinator's `initial-thoughts.md` calls sealing-as-epistemics; the point to add is that it is not a divergence from RONR but the removal of a constraint the book had only because humans cannot be blind and on the record at the same time.
- **The prevailing-side rule becomes enforceable.** `../../RONR-OBSERVATIONS.md` §3 notes the honor-system character of 37:10(a). With every vote recorded per seat, `guard/mover-was-prevailing-side` is computed, not adjudicated, and the `prevailing-side-claim` predicate is unused.

## 7. What this settles

The claim is checkable and narrow: **every mechanism in the book that rests on simultaneity has, under the common-state condition, a defined counterpart or a demonstrated vacuity, and the residuals the charter must declare are the ones 9:36 already names.** The principle underneath — mutual deliberation before decision — is carried by the round, not by the clock; the four declared residuals are exactly where the server's fairness is a design choice rather than a derivation. The rest of the model applies unchanged.

The common-state condition is the interface to the *contention-rounds* sibling spike: that spike owns how a round's intents are ordered; this one owns the fact that the charter must declare the round as the unit of simultaneity and declare the residuals. The CSC is also the property a verifier can test on a running server: shuffle arrival order within rounds and the log must not change (the coordinator's latency-invariance criterion is the CSC's observable consequence).
