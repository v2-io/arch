# De novo feedback 1: spike-charter-agenda-2026-11-03

*An independent adversarial pass by a Claude Opus 5.5 session on 2026-11-03, written for the verifier and integrator and for Joseph. I read every file in this directory whole, ran the checker, and checked the spike's claims against `../../model/*.json` and `../../RONR-12/body.md`, quoting the primaries below. I also read parts of the sibling spikes where this one makes claims about them: `contention.md` §§2–5 and the walking-skeleton `debrief.md`. I did not read any other verification of this spike (an untracked `verify-1/` sits in the walking-skeleton directory; I left it alone). Labels: **[verified]** means I checked it against the model, the book, or a run; **[argued]** means it is my reasoning from the cited text; **[guess]** means it is mine and unchecked.*

## The short version

The spike's map of the ground is good. Its citations of the committee chapters are accurate, its labels are mostly honest, and its do-not-inherit list is the right instinct. The trouble is in the four places where it claims the most:

1. **The book explicitly rules out the slot the spike says it left open.** 9:34 says that "regardless of the technology used, the opportunity for simultaneous aural communication is essential," and that a deliberative process conducted in writing "may achieve a consultative character, but it is foreign to the deliberative process." The spike cites 9:30, 9:31, 9:35 and 9:36. It never cites 9:34, which sits between them. Read by the letter, the book puts the council in the tier where a report must be unanimous, the reverse of the spike's "[derived]" result (§1).
2. **The common-state condition is weaker than what the walkthrough and the sibling spike need.** As written it permits several acts per round. The contention-rounds spike shows that this breaks informed action and that one floor act per round is forced. The walkthrough puts two speeches in a single round, which the model's own `speak-in-debate` conjuncts refuse. And the interruption row cannot be "exact" when the speech being interrupted is sealed (§2).
3. **The "admitted by" column, which the spike names as its checkable content, fails against the model.** `state-question` requires state `seconded`, which no small-board-derived configuration ever reaches. `put-question` becomes admissible, and so obligatory for a mechanical chair, the moment a question is stated, before any debate round. `mover-was-prevailing-side` has no committee branch and keeps its assertion path even when votes are recorded. The vote-method enum forces "roll call or ballot" (§3).
4. **The three acts the charter adds bypass every protective conjunct of `move`.** They skip quorum, the floor, rank, dilatoriness, and the governing rules. As written, an inquorate council can co-opt members, spawn inquiries, and rise and report (§4).
5. **The checker's "well-formed" is much weaker than "loads against the model."** A mutant with seven deliberate breakages passes with exit 0. Among them: an out-of-enum parameter, a wrong-typed parameter, a patch to a nonexistent surface, `amend` both allowed and disabled, and prose in place of every logical-time mapping (§5).
6. **Quorum 3 of 3, combined with the backstop rule, gives every seat a veto by silence**, and the claim that the backstop "decides nothing" is false (§6).
7. **The quick-council shape works only when the parent has already drafted one side of the question as a resolution.** The real specimen is "keep or remove". The book's own device for that shape, filling blanks, is whitelisted but unused (§7).

Smaller findings follow in §§8–12. §13 lists what held up when checked. §14 says what I did not check.

---

## 1. 9:34 and the validity tier

**The spike's claim.** 01:55 says 9:36 "is not an agentic invention bolted onto RONR; it is a slot RONR left open and labeled." 01:63 says "**rounds → majority is valid; polling → unanimity only.** [derived]"

**What the book says** [verified]. 9:34, in full:

> It is important to understand that, regardless of the technology used, the opportunity for simultaneous aural communication is essential to the deliberative character of the meeting. Therefore, a group that attempts to conduct the deliberative process in writing (such as by postal mail, e-mail, "chat rooms," or fax)—which is not recommended—does not constitute a deliberative assembly. Any such effort may achieve a consultative character, but it is foreign to the deliberative process as understood under parliamentary law.

9:36 opens "If an organization authorizes … electronic meetings", and 9:31 defines those by the aural standard. On the book's own definitions, then, 9:36's list governs meetings that already meet that standard, and it is not open to a written process [argued, from 9:31/9:34/9:36 read together]. 51:2 grants majority-validity only to a report "agreed to by a majority vote at a regular or properly called meeting". For a committee that cannot meet, it allows a report only of "what has been agreed to by every one of its members". By the book's letter, a written turn-based council is the 9:34 "consultative" case. That puts it in 51:2's unanimity tier, and the spike's two-tier result is inverted.

**Why this matters more than a missed cite** [argued]. 01:9 sets the label question aside ("Joseph, mid-spike: 'I only care about the book inasmuch as it surfaces underlying foundational principles'"), and that is a legitimate move. But §4 of the same file then takes the label's consequence (majority validity, which the book grants only to a *meeting*) and grades it [derived]. The spike can't both decline the book's test for what a meeting is and claim the book's validity rule for meetings. As written, the result is an argued departure from 9:34, presented as a derivation from 51:2.

**The strengthening direction** [argued]. The principle-level argument can be made, and it would be stronger than the current one. 9:34 gives its reason: simultaneous *aural* communication is "essential to the deliberative character". The work is to name what aural simultaneity provides that the common-state condition does not, and show that each item is either carried or deliberately dropped. My candidates for what it provides that the ten-row table misses:
- **Interruption of a speech in progress.** 42:18 applies to a member who "has begun to speak". See §2.
- **Hearing votes in sequence.** In a roll call or voice vote, members hear earlier votes and may change their own until the announcement (45:8). A sealed vote keeps the right in form and empties it of its trigger.
- **Answerability of the final word.** Under the charter, each vote carries "its stated reasons" (`quick-council.json:165`, `report-form`). Those reasons are formed after debate has closed and no other seat can answer them. Yet 02 §4 makes them "the product." A reason no one could answer is the thing the principle at 01:9 says a body's decision must not rest on.
- **The parent's debate budget.** A token budget that runs out can close debate one speech after a new argument has been made. The book allows this for the parent (13:8(d)), but it puts a hole in the principle as 01:9 states it, and the spike does not name the hole.

If the CSC can be shown to carry or deliberately drop each of these, the claim becomes: "9:34's stated reason is satisfied functionally by the round discipline, so the 51:2 meeting tier applies by principle though not by letter." That is a real result. As it stands, the label should be **[argued, against 9:34]**, not [derived].

---

## 2. The common-state condition: what it guarantees, and what the walkthrough assumes

**The definition, read literally** [verified, internal]. 01:34 says: "Every act applied at state $s_k$ was formed by a member who held $s_k$; and every present member held $s_k$ before any act at $s_k$ was applied," where $s_k$ is "the event log through event $k$." If a round admits more than one act, the second act is applied at $s_{k+1}$, not $s_k$. So either the CSC limits a round to one applied act, or the indexing has to be read as "per round". The walkthrough takes the second reading: in 05:46, round 2 carries two speeches.

**The batch reading is the one the sibling spike refutes** [verified, cross-spike]. `contention.md` I1 says: "Every admitted act was formed against a state that includes every previously admitted act … A round mechanism keeps it only if it admits at most one *floor-needing* act per round (§5.2)." Its §5.2 construction: amend and postpone are moved in the same round, postpone is admitted first, and amend is dropped because it was formed against a state that no longer holds. This spike's 02 §8 gives the CSC to contention-rounds as "the property the charter declares and the server must keep." The CSC is strictly weaker than I1, though, and the charter declares the weaker one. The two spikes also disagree about debate. This spike, following the coordinator's `mechanics.md:93` ("debate rounds (parallel): statements revealed together"), allows parallel speeches. Contention-rounds forces one floor act per round. The integrator will have to choose, and the choice changes rows b, c, g and i of 01 §3 and every round count in 05.

**The walkthrough's round 2 is inadmissible under the model** [verified]. `speak-in-debate` requires `["=", ["ctx","floor","holder"], ["param","member"]]`. Only one member holds the floor, and `recognize` is a chair duty that assigns it. Two seats cannot both speak from one state. 01 §3 c (ladders choose among a round's claims) agrees with this, and 05:46 contradicts it.

**Row b (interruption) is not "exact"** [argued]. 42:18 protects a member who "has been assigned the floor and has begun to speak". Under sealing, an interrupt-class intent submitted in the same round as a speech was formed without knowledge of that speech, so it cannot be *about* it. An objection to a speech, such as indecorum or a speech that has strayed from the question, can be raised only in the next round, after the whole speech has been delivered and revealed. What survives is a point of order raised after the fact. That may well be acceptable, but it is a loss, not an exact translation. Row b's text ("the resolver applies the interrupt first … the speech's author 'retains the floor'") describes a case that can only be a coincidence of timing.

**Row g and `put-question`.** See §3.2. The CSC does not say which derived chair events may follow one another without a round, and that gap is where row g breaks.

**The round count** [verified, my count]. 05:66 says "Thirteen rounds of three sealed submissions each". Counting the walkthrough's own rows under its own conventions (a separate round for each vote and consent, a round between revealing a motion and stating it, a pass round before each put), I get about 20–22 submission rounds. That is before applying contention-rounds' one-floor-act rule, which adds rounds for rows 2 and 5. The 13 counts row labels, not rounds. The round numbers are flagged as illustrative (do-not-inherit list), but 05:66 states the total as a cost.

---

## 3. The "admitted by" column against the model as compiled

The do-not-inherit list says "the *admitting conjuncts* column is the checkable content." I checked it.

### 3.1 No small-board-derived council can state a question [verified]

`actions.json state-question`: `["and", ["=", ["state", q], "seconded"], ["not", ["adjudged", "dilatory", …]]]`. The `seconded` state is reachable only through the `seconded` event (lifecycle `moved → seconded`). `small-board` disables `second`, and its fix for the lifecycle is a **prose** patch: `"surface": "lifecycle.transitions: moved -> pending", "replace": "on stated directly; the seconded state is skipped …"`. No compiled path leads from `moved` to anything `state-question` accepts, and no compiled path exists for motions that never need a second (e.g. a point of order) either. 02:73 says "The small-board/committee overlays already supply the one structural patch a narrow council needs (no `seconded` state)". They supply it only in prose. This is F1's class of defect (the configuration cannot state a question) on the chain both charters actually use, which F1 does not: F1 is about committee-of-the-whole, which neither charter loads. The same holds for rows 1′ and 3′ of the walkthrough.

The same chain has another prose patch: small-board's "chair-member … votes on all questions" is a prose patch to `protocol.decorum`. Meanwhile `vote`'s compiled expression still limits the chair to votes that affect the result. Under sealed simultaneous voting that condition cannot be evaluated, because the tally is unknown when votes are cast. This bears on the standing council's elected member-chair (`standing-council.json`: "the member-chair votes with the members"). The quick council's mechanical chair also inherits `chair_participates: true` from small-board, and the charter does not override it.

### 3.2 A mechanical chair puts every question the moment it is stated [verified + argued]

`put-question`'s third disjunct is `["=", ["count","c",["ctx","floor","claims"], …debate…], 0]`. Right after a question is stated, before any round, there are no claims, so the disjunct holds. `chair_duty_note` says "where exactly one chair action is admissible … the list is also the obligation," and 02:64 has the server perform duties "as derived events with no round." Together, a mechanical chair is obliged to put the question immediately. 04 §7's check ("A round in which no present seat claims the floor for debate satisfies it literally ✔") is true and beside the point: the expression is also satisfied with *no* round. The `put-question` note says "no *timely* floor claim," but nothing in the expression encodes timeliness. Row g's "the round makes the predicate literal" needs an added conjunct (e.g. "a round has closed since the last stating or speech with zero debate claims"). That is a model change, not the model's own rule made literal.

### 3.3 The prevailing-side "gain" requires a guard change [verified]

`guards.json mover-was-prevailing-side`:
`["or", ["in", member, disposition_vote.prevailing_members], ["=", method, "unanimous-consent"], ["adjudged", "prevailing-side-claim", member]]`.

- The description says "(committee: anyone not on the losing side [37:35])", but the expression has no such branch. Under 37:35(2), a seat that abstained, or that the backstop marked absent, may move to reconsider ("one who did not vote at all, or even was absent"). The expression sends such a seat to the adjudicated assertion path, and `prevailing-side-claim` defaults to `null`, which means a ruling round.
- In walkthrough row 10, the expression does not yield the "refused with the why-not" the spike displays. The cold-reader is not in `prevailing_members`, the method is not unanimous consent, and the third disjunct is undetermined. Nothing short-circuits it to "recorded on the losing side, therefore false." So 01 §6's "`guard/mover-was-prevailing-side` is computed, not adjudicated, and the `prevailing-side-claim` predicate is unused" is a proposed change to the guard. It is not something rounds deliver. (The observation itself is the coordinator's, from `RONR-OBSERVATIONS.md` §3: "for the first time in its history".)
- `schema.json`: `prevailing_members` is "recorded only for roll-call". So the gain requires the engine to classify the sealed vote as `roll-call`. See §3.4.

**Strengthening direction.** Add the committee disjunct (`member ∉ losing_members`). Make the recorded-vote case exclusive of the assertion path. Record presence per event, which `gaps.json presence-history` already names as missing. After those changes the claim holds, and it holds on every recorded vote rather than only on roll-call votes.

### 3.4 "Roll call and ballot at once" is the book's signed ballot [verified]

01:82 (and 00:45, H5: "RONR could never have both") say a sealed round-vote is "the ballot's epistemic property" (independence) together with the roll call's record. The book does not frame the ballot that way. 45:18: "used when **secrecy** of the members' votes is desired … without revealing how individual members have voted." The ballot's property is secrecy, and a sealed-then-published vote does not deliver it. The combination the spike describes, written, cast without seeing others' votes, and recorded per member, is the book's **signed ballot** (45:46): "the voter writes 'yes' or 'no' on the ballot and signs it. The votes can be recorded in the minutes just as a roll call would be." The standing charter cites this paragraph itself (`standing-council.json:95`: "a 'signed ballot' [45:46] is already what every vote is"). So it is not something the book could never have. The engine has to pick a value from `method ∈ {voice, rising, counted-rising, ballot, roll-call, unanimous-consent}`, and the two candidate values are incompatible:
- `ballot`: `change-vote` becomes inadmissible (its expression requires `method != ballot`), which contradicts the charter's "vote change until the reveal". 45:8 agrees with the expression, since it excepts ballots from vote change. `prevailing_members` is also not recorded.
- `roll-call`: the gains hold, but nothing secret remains, and the member-chair rule in `vote` assumes sequential casting.

**One thing worth keeping** [argued]. The ballot's *purpose* has an agentic meaning the spike set aside: voting without regard to how the principal will react. For agents whose per-seat record the steward reads, that is a sycophancy channel. 04 §6 dismisses secrecy from Joseph as meaningless ("the parent is always entitled to the record"). That may be the right design, but it is a design choice with a cost to name, not a vacuity.

---

## 4. The charter-added acts bypass `move` [verified]

`commission-inquiry`, `co-opt-member` and `move-rise-and-report` are `add_action` records with their own expressions (`quick-council.json:130, 139, 149`). Every other motion is reached through `action/move`, whose expression includes `quorum-present` (or the inquorate list), `rank-admissible`, `applicable`, `no-order-precluding`, `act-in-order-under-governing-rules`, `not dilatory`, and floor possession. None of the three added expressions contains any of these. Consequences:

- **No quorum.** With quorum 3 and one seat absent, the remaining two can adopt `co-opt-member` and seat a fourth member. 40:6–7 permit no substantive business without a quorum. `move-rise-and-report` can likewise adopt a report while inquorate. This is the path 06 §7 actually takes: there, "the council re-forks the same source into the seat by its own act". That also conflates *adding* a member (13:8(d), the grant) with *filling a vacancy* (13:23, the appointing power's), and an absent seat is not a vacancy anyway.
- **No floor, rank, or dilatoriness check.** `move-rise-and-report` is in order on round 1, before anyone has spoken, and is undebatable. Two of three seats can end the council with "unable to arrive at a recommendation" before the third has spoken. That conflicts with the principle at 01:9 (the same pattern is in the book for committee-of-the-whole's rise, but there the assembly is still sitting to hear the matter).
- **The note and the expression disagree.** `commission-inquiry`'s note says it is "in order while any question is pending", but its expression has no pending-question conjunct.

**Strengthening direction.** F3 asks for SDC fields on `add_action`. The stronger fix is to make these registry motions reached through `move` (or to make `add_action` records that declare `vote`/`debatable` route through it), so that they inherit every protective conjunct by construction.

---

## 5. What the checker certifies [verified, by run]

I wrote a mutant of `quick-council.json` in my scratchpad: `session_scope: "no-such-scope"` (outside the enum), `speech_limit: "banana"` (wrong type), every `logical_time` value set to `"banana"`, a `patch` to `actions.no-such-action.expr`, a `disable_motion amend` while `amend` stays in `allowed`, `may-add-members.max_total: 99` while the expression still says `< 5`, and every `electronic_meeting_rules` value set to `""`. Result: `RESULT: well-formed`, exit 0. The printed action space even lists `amend` as allowed and as disabled at once.

So `check-charter.py` checks that references resolve and that keys exist. That is useful, and its docstring says something close to this ("load-time closure only"). But "Two charters that load against the model" (commit message), "both load" (debrief), and F9's proposal to adopt it "as acceptance" all claim more than it checks. Unchecked:
- parameter values against `council.json` types and enums. The charters' own `speech_limit {max_speeches, max_tokens}` does not match the model's `{max_speeches, max_minutes}`.
- `patch` ops of any kind. The quick charter replaces `actions.report.expr` with an English sentence (`quick-council.json:154`). That is the kind of prose expression F3 says the INTERFACE forbids.
- allowed ∩ disabled. The quick charter allows `object-to-standing-at-ease` (`:105`) and disables `stand-at-ease` (`:125`).
- consistency between instructions and expressions. See §8.
- logical-time coverage beyond `vocabulary.json` anchors plus `now`, `hour` and `minutes`. See §9.

A further structural point [argued]. 02 §1 and the debrief cite "the checker shows both charters load against the same configuration chain `small-board -> committee`" as evidence for "one road". But both charters *declare* `"configuration": "committee"`, so the checker is reporting back what it was given. That is not evidence about roads.

---

## 6. Quorum, the backstop, and veto by silence [argued, from the charter text]

The quick charter sets quorum to 3 of 3 seats (`quick-council.json:52`, "because every seat was chosen for a perspective the decision needs"). Its malfunction rule (`:166`) says: "a seat that has not submitted at the backstop is absent for that round — never passed, never penalized; quorum is recomputed; the inquorate set governs." Taken together:

- One slow API response in a vote round makes the council inquorate, so no vote can be taken that round.
- A seat that wants to block a decision can simply not submit. That is indistinguishable from latency, "never penalized" by design, and repeatable every round until `report-by` (80 rounds) runs out and the parent discharges the council. Each seat holds a unilateral veto.
- 03:38 says the backstop "marks absence, decides nothing." But absence feeds quorum, the threshold drops of 35:2(7) and 37:35(3) (which 04 §7 itself computes "from presence"), and whether a polled result is unanimous. Wall-clock latency past the backstop does change outcomes. Contention-rounds puts it more carefully: "lateness only matters past a deadline that is a presence rule". The latency-invariance property in 01 §8 holds only for latency inside the backstop.

That quorum choice confuses representation (50:18, *who sits*) with quorum (40:5, *how many must be present to act*). The book's default of a majority (2 of 3) exists so that one absence does not stop the body. If every perspective must be heard before a decision, the honest way to write that is a rule about the decision, for example "the final recommendation requires every seat to have had an open debate round", not a quorum.

---

## 7. The referral as a pre-drafted resolution [verified + argued]

The specimen is a choice (`specimens-2026-11-03.md:26`, "Keep computed lines … or remove them?"; `:65`, "Should vsect sync them … or should they be removed?"). The quick charter casts it as one side already drafted: "Resolved, That vsect keep the computed frontmatter lines … and sync them". It also excludes `main-motion` ("the referred question pends without a motion"). As a result:

- The council can reach "remove" only by an amendment that reverses the resolution. The walkthrough then has to spend a germaneness fight on that amendment (rows 4–4′).
- A referral of a **subject** cannot work at all. With main motions excluded, there is no way to put any resolution before the council. The book expects subject referrals (50:19: "When a subject or item of business is referred to the committee"), expects the committee to draft (51:4: "A board or committee is usually best fitted to prepare resolutions to carry out its recommendations, and it should never leave this responsibility to others"), and has rules for resolutions that originate in committee (52:8). 04:9's "special council: out of order" for main motions is correct only for a resolution-referral, and only if a substitute (an amendment) is the sole route to the alternative.
- The book's device for "which of these alternatives" is **filling blanks** (12:92–113): "Resolved, That vsect ___ the computed lines", with keep-and-sync and remove as proposals. Neither side is given the advantage of being the drafted text. The quick charter whitelists `create-blank` and `fill-blank-proposal`, and the walkthrough never uses them.

The charter has a second place where a prior can enter: seat `perspective` strings are written as positions. For example, `seat/tool-design`: "the tool: state is computed from the trail, never typed". That is the conclusion of one side of the question, and in the walkthrough that seat argues exactly its string. Whoever writes the referral and the perspectives has largely set the outcome. Given the program's own concerns about priming, it may be worth a format rule that a perspective names what the seat looks at, not what it concludes [guess at the fix].

---

## 8. Smaller defects in the charters

- **"Grounded" objection** [verified]. `quick-council.json:165`: "unanimous consent by a consent round in which any **grounded** objection defeats consent [4:59]". 4:59 says "if any member objects". It sets no ground requirement, and the spike's own 01 §3 d says "any objection." The word comes from the coordinator's `mechanics.md:90–92` ("object-with-ground … any grounded objection → debate"). If "grounded" only means "carries a stated reason," then it adds a requirement to the lone objection, which ROADMAP's pin list and F2 treat as rights-critical. If someone judges whether an objection is grounded, it narrows the right. Either way, it is a declared departure from 4:59 and should not cite 4:59 as authority.
- **The charter repeats things the engine already knows.** 02 §2's governing principle is "the charter declares only what the engine cannot compute," and it cites vsect's 189 drifted lines. Yet the charter writes out fields the same table marks derived: `agenda.items` (derived for a special council), `decided_by_on_report`, `dissolution`, and `record`. It also states the instruction bounds twice: `may-add-members.max_total: 5` (`:77`) and the literal `5` in `co-opt-member`'s expression (`:149`), and likewise `max_concurrent: 2` against the literal `2`. My mutant (§5) shows that these can drift apart silently.
- **51:40 applied to the wrong body** [verified]. 02 §3 and walkthrough row 1′ have the council state the referred question "without any motion [51:40]". 51:40 describes the *assembly* when a report comes back ("When the presentation of the report is concluded … the resolution or main question becomes pending automatically"). It says nothing about how a committee takes up its charge. As an analogy the point is fine, but as a citation it is wrong.
- **52:28 assumes a recess, so "without interrupting" has no basis in the book** [verified]. The breakout groups of 52:28 report "after it has been reconvened and the matter under consideration is again pending". The groups "often … meet during a recess or adjournment of the assembly." So the book's breakout is the recess pattern, which is what the walkthrough does at row 9. Joseph's "research subcommittee without interrupting", where the inquiry runs while debate continues, has no precedent in 52:28. The debrief's "is the book's 'breakout group'" (debrief:27) overstates it. The *report in the nature of debate* part does hold.
- **Route A cannot staff with non-members** [verified]. 05:9 says the inquiry is "staffed by non-members only if the charter authorizes [50:15]". 05:11 then calls Route A, "a subcommittee of one, a fork of expert E", available "to any council … no charter grant." A fork of an expert is a non-member, so the two statements contradict each other. 50:15 also limits its exception to "cases where the committee is appointed to take action that requires the assistance of others", and a council that only reports is not appointed to act. 13:8(d)'s "employ an expert consultant" is the better warrant. A consultant is not a subcommittee, which points to a third shape: an expert input with no subcommittee machinery. Separately, whether creating an inquiry counts as "organization or existence" under 19:7 is a stretch. Adding a member fits that phrase more naturally.
- **Sealing is not enforceable for the roll the charter declares** [verified, cross-spike]. Walking-skeleton `debrief.md`: "Sealed intents are files in the council directory. Today they stay sealed only because every launcher withholds file tools from members … Prepared experts keep their file tools." The quick charter's `seat/tool-design` is a `claude-code-fork` of a prepared expert, and `commission-inquiry` spawns expert forks. The CSC depends on sealing, and the charter's 9:36 section has no rule about what tools a member may hold during a sitting. 9:36's first bullet ("the type of equipment or computer software required for participation") is where that rule belongs.

---

## 9. The logical-time inventory is incomplete [verified]

03 §1 is described as "Every temporal construct in the model, inventoried mechanically." I found these gaps:

- **Seven timers, not six.** `lifecycle.json` has `reconsider-make-window`, `reconsider-callup-window`, `reconsider-enter-minutes-callup`, `table-expiry`, `vote-challenge-window`, `renewal-unblock`, and `order-exhaustion`. 03:21 says "six timers," and 04:90's adversarial self-check marks "six timers … ✔". The missing timer is calendar-bound (`opens_at` uses `clock.last_business_day_of_session` and `next-business-day`). It is excluded from both charters, so nothing breaks, but the self-check passed a miscount.
- **Clock fields missing from the inventory and from both `logical_time` maps:** `clock.last_business_day_of_session` (actions 1, lifecycle 1), `clock.adjourned_meeting_set` (lifecycle, precedence, scheduler), and `clock.current_session` (guards 5, lifecycle 4). The checker cannot notice, because it checks only `vocabulary.json` anchors plus `now`, `hour` and `minutes`.
- **The window scope tokens are not in the inventory at all.** `gaps.json event-scope-sugar` lists `just-now`, `since-ruling`, `open-now`, `pending-consent` and others as informal. These are the scopes that implement "promptly" and "at the time" (in `raise-point-of-order`, `appeal`, and `object-to-unanimous-consent`), which are exactly the simultaneity loads that 01 §3 a/d/e grade "exact." Rounds give them a meaning, which is useful, but it is a *definition* the charter proposes, filling a gap the model already names. It is not a translation of existing rules.
- **`now` is not monotone across sittings.** 03:29 maps it to "the round counter within the sitting (monotone, total)". In a standing council, `hour` for a postponement into a later sitting, and `postpone-horizon`'s comparison against `end-of-session(next_regular_session)`, need values that can be compared across sittings. A per-sitting counter doesn't provide that. `(sitting, round)` in lexicographic order would.
- **The redefinitions are changes to the interface, and only one is flagged.** F4 flags `before(now, event-ref)` as an interface change. But `within-quarterly` already has fixed semantics in the closed operator set (INTERFACE: "month-boundary arithmetic"), and the charters redefine it per council (`≡ false`, `≡ not term_boundary`). The same applies to `same-day`, `anchor`, and `now`. These are all per-charter operator semantics and belong in the same CHANGELOG entry as F4.
- **The purpose 03 §3 assigns to the quarterly interval is not what the cited text says.** 03:42 says the quarterly interval's purpose is "continuity of the deliberating membership [derived from 8:11–12, 9:7–10]". 8:11–12 give a different principle: "the freedom of each new session … one session cannot place a question beyond the reach of a majority at a later session." That principle is about not binding a later sitting whose judgment may have moved, and for agents what moves is the *world* (main has moved, the specimen's facts have changed) more than memory. "Agents have no calendar decay" may be true of a frozen source and false of what it deliberates about. Reducing `within-quarterly` to `not term_boundary` drops the elapsed-world part on a premise the cited text doesn't give.

---

## 10. model-defects.md: what holds and what is missing [verified]

- **F1 holds as a literal reading.** `restrict_actions`: "every motion/action not listed is disabled". COTW's `plus` contains no chair or secretary acts.
- **F2 holds for `object-to-unanimous-consent`, and misses the largest omission: `action/move`.** COTW's `plus` lacks `move`, the action that offers every motion. Read with F1's literalism, COTW members cannot move the `amend` their whitelist allows. Both charters put `action/move` in `plus`, so the spike evidently knew it was needed.
- **F7.** `gaps.json committee-identity` already lists `Committee {id, kind, roll, charter_ref, parent}`. The spike adds `state` and `Sitting`, which is less new than "this spike adds the field list" suggests.
- **Missing from the list:** the `seconded` and prose-patch problems (§3.1), `put-question` with no round (§3.2), the guard's missing committee branch (§3.3), the method enum (§3.4), add_actions bypassing `move` (§4), and `presence-history` (`gaps.json`), which 04 §7's "computed from presence" and 06 §7's automatic voiding both need.
- **The fix count disagrees across files.** The README says "two defects found that must be fixed first"; 02:78 says "Three defects must be fixed (F1–F3)" but describes only F1 and F2; `model-defects.md` says "F1–F2 are real defects."
- **The engine change is stated two ways.** 02 §4 says "The engine needs no change" for the recommendation vote. F6 says `Question.effect` needs a new `effect.kind` flag. The second is right.

---

## 11. Framing and structure

- **How much of the "executable" charter is executable** [verified, rough count]. I counted leaf values with four or more words, ignoring cite and note fields. The overlay is nearly all structured (122 structured to 16 prose in the quick charter). `electronic_meeting_rules`, which carries the CSC, the four residuals and the backstop, is entirely prose in the quick charter (0 to 10). `logical_time`, `dissolution` and `record` are prose too. The parts the spike presents as its contribution (the unit of simultaneity, the residuals, logical time) are the parts no engine can read. "Executable" fits the whitelist plus the add_action expressions, and those have the problems in §4.
- **"One road"** [argued]. The debrief's "**One road.**" rests on both charters declaring the same configuration (§5) and differing in "one field (`kind`) and two lists" (02:80). They also differ in chair mode, quorum basis, `session_scope`, the logical-time mappings, agenda structure, and the meaning of `move-rise-and-report` ("the effect differs by kind"). More basically, the book gives no transition by which a special committee becomes a standing one. A standing committee is constituted by a 50:8-grade act. So what the spike shows is that *one format expresses both*, which is useful. That is not the same as *growing a quick council to a standing one by editing it*.
- **50:10 links the two routes** [verified + argued]. "A special committee may not be appointed to perform a task that falls within the assigned function of an existing standing committee." The standing charter's `class_of_business` ("decisions in the ASF in-place refactor …", sourced from the same specimens file) plausibly covers the vsect question the quick charter takes up. Once a standing council exists, the book bars special councils inside its class, and the charter format has nowhere to record that check.
- **Mechanical rulings are appealable after all.** 02:67 says a compiled refusal is "not appealable inside the tool because an appeal against a compiled conjunct is a claim that the model is wrong." Contention-rounds §5 makes the counter-case: when the server assigns the floor wrongly, the *state* is wrong, not the model, and "its mechanical acts must be appealable like a chair's [42:15] … the server reverts." The same applies to a compiled refusal computed from a wrong record, such as a backstop false-absent feeding a threshold. The suspend/erratum/ratify route does not cover state errors.
- **Inquorate voiding needs data the model lacks.** 06 §7's guess (void inquorate action automatically) depends on per-round presence history, which `gaps.json presence-history` says is not recorded. Separately, if the server refuses inquorate acts as they are submitted, the case it guards should not arise. The add_actions of §4 are the exception.

---

## 12. On "same source = same member"

The spike invites push here. One thing it does not separate [argued]: RONR uses a single notion of *person* for three jobs that a forked seat splits apart:
- **Counting:** quorum, majorities, "present or notified." Seat identity serves this well.
- **Accountability for one's own past act:** the prevailing-side rule, and changing a vote. A re-fork never cast the earlier vote, so seat identity is a legal fiction here, which can be acceptable as long as it is declared.
- **Continuity of judgment:** the corridor. Neither answer fits, because a fork carries the source's reading and none of the sittings' deliberation.

The charter could declare each of the three separately instead of settling on one identity rule. That would also dissolve the binary 06 §4 poses.

---

## 13. What held up when checked [verified]

These are listed so the integrator knows which claims were checked and found right. Each was read against the primary text.

- 13:8(d), including "employ an expert consultant", "even to add to its own membership", and that a majority suffices even for instructions that conflict with rules. Also 13:15, 13:19, 13:20, 13:22, 50:15, 50:21, 50:23, 50:25, 50:26, 50:30, 41:57, 41:63, and 40:5.
- 37:35(1)–(3) and the committee clause of 35:2(7). 04 §7's correction (the threshold drop is computed per vote from presence, not "always") is right and was well caught.
- 51:2's committee-unanimity sentence, quoted accurately. 49:16 and the 1:1 footnote, quoted accurately. 45:45's "exactly the opposite effect."
- Sample Rules Scenario A rules 4 and 5, quoted accurately. Note that rule 4 says "audible roll call", which the charter's roll round adapts.
- 23:5, 4:19, 33:12, 42:7, 42:20, 4:59 (the text, as distinct from the charter's use of it), 43:13, and 44:12.
- The registry has 31 motions, and the checker's counts (20 of 31 allowed, 26 of 31) match its output.
- `privilege-admissible` defaults to `null` (F8). `floor.interrupted_holder` exists in the schema. `put-question`'s third disjunct is as quoted.
- The walking-skeleton's three amend detours are reported as described. The argument that a council recommending text cannot exclude `amend` stands. One qualification: in the quick charter, detour (c) is blocked by the exclusion of `main-motion` before any same-question bar is reached, and whether the bar would refuse it is adjudged ("likely", 06 §11), not certain as 04:10 and the debrief state it.
- The spike stayed in its write-scope: commit `4a9131e` touches only this directory.

---

## 14. What this pass did not do

- I ran no engine (there is none yet). Every finding about admissibility comes from reading expressions, not from evaluating them.
- I did not audit `protocol.json`, `precedence.json`, `scheduler.json` or `engine.json` beyond the entries the spike cites. Rows 1′ and 3′ may have stage-5 orchestration notes I didn't see that address the `seconded` path.
- Of the book, I read about seventy of the cited paragraphs. Most I read in full. For some (10:54, 42:18, 21:7, 49:21, 50:8, 9:36) I read only the lead sentence, not the enumerated list that follows. 9:4, 9:13, 9:17, 41:61, 48:5 and 53:9 are unread.
- I did not check the ROADMAP beyond the pin-list and pinned-version passages.
- The tenth-row/eleventh-load hunt (§1) is not exhaustive. I offer four candidates. I did not prove there are no others.
- This report's list of findings is its content. Silence on a claim means I did not check it, not that it passed.

*Feedback on the brief itself: it was clean. "Re-derive from what is written rather than from what you can tell was intended" did real work. It is why §3 checks the "admitted by" column against the expressions instead of taking the spike's word that those conjuncts admit.*
