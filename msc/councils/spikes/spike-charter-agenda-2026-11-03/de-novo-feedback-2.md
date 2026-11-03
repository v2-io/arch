# De novo feedback 2: spike-charter-agenda-2026-11-03 (revision 2)

*An independent adversarial pass on the spike as it stands at commit `21c0c50`, by a Claude Opus 5.5 session on 2026-11-03. Written for the integrator, the spiker, and Joseph.*

*What I read: every file in this directory, whole. I read the body, charters and tools first. I read `de-novo-feedback-1.md` and `response-to-feedback-1.md` only after that, so my first pass was not shaped by them. In the model I read the parts the spike's claims lead into: `INTERFACE.md`; `actions`, `registry`, `precedence`, `guards`, `adjudication`, `council`, `schema` and `lifecycle`; `engine`'s effective-rank section; `protocol`'s recognition ladder. In `../../RONR-12/body.md` I read about forty paragraphs whole, mostly ones the first verifier listed as unread or read only in their lead sentence. Outside the directory I read the contention-rounds spike's current `contention.md` and its revision-1 text from git, the walking-skeleton's charter shape, the udon decision-authority ADR, and the ROADMAP's phase-5 and pin-list passages.*

*What I ran: the checker, `mutant.py`, and one further mutant of my own, kept in scratch and not committed.*

*Labels: **[verified]** means checked against the model, the book, git, or a run. **[argued]** means my reasoning from the cited text. **[guess]** means mine and unchecked.*

## The short version

The spike's architecture holds up when re-derived. A charter is an instance over a configuration type. The special/standing `kind` sets dissolution, session structure and the agenda. For a special council the referral is the charter. The report lands as `decided-by: delegated` under a grant, with nothing in the tool upgrading it. I checked each of these against the model, the book and the ADR, and found no problem (§12). The F10 finding is correct and important: the small-board chain's prose patches mean no committee-derived council can state a question.

The problems are now concentrated in the procedural layer, which is where revision 2 did most of its work. Most of them share one cause. Each repair accepted from the first report was landed without re-deriving what the repair itself depends on: the sibling spike's current text, the schema behind the blank-filling device, 50:15, 51:2, and how long a floor claim lasts. In order of consequence:

1. **The charters declare the contention spike's revision-1 rules, which its revision 2 has since narrowed or retracted.** This applies to I1's wording, to the recognition tiebreak (persistent claim age, which revision 2 retracted by name), and to the "change round" the charter points to in its §8. The charter's "informed acts, verbatim" is verbatim of a superseded text. This is the drift the spike's own governing principle warns against (§1).
2. **The rise-and-report gate added in revision 2 is vacuous under per-round claims.** Two seats can still end a special council before the third has spoken, so the first report's §4 second bullet remains open. The new `every-seat-heard` rule moves the veto-by-silence off quorum and onto the final vote. `co-opt-member` by bare majority lets a majority pack the council in the middle of a question (§2).
3. **The rewritten walkthrough rests on machinery the model does not have.** The model has no schema for blanks or proposals. Round 17's reconsider of a proposal vote is not applicable as compiled. When proposals arrive in one batched round, their voting order is undefined. Rounds 17–18 break the charter's own duty-sequencing rule. The "honest" count misstates its own breakdown. And the 4:19 window the charter claims to preserve is empty in every row where a motion is stated (§3).
4. **The report is never adopted.** Under 51:2 a report may contain only what a majority agreed to. Only the recommendation is ever voted, so the facts, findings and minority views are text no vote adopted, emitted by a mechanical secretary. That is also where the answerability hole reappears (§4).
5. **`commission-inquiry` still creates a subcommittee of non-members.** 50:15 bars that for a council that only reports, and the standing charter's own `may-appoint-subcommittees: with_non_members: false` says the same thing (§5).
6. **The reply round does not close the budget hole.** It moves the hole by one speech, and it contradicts I1 and the declared serial debate mode. The principle §1 states is stronger than 49:16, the paragraph it comes from, and no bounded debate can satisfy it. There is a real choice to make here, and the spike has not made it (§6).
7. **The corridor's elapsed-world conjunct asks a question about the future** at the two guards that evaluate it when a postponement is moved (§7).
8. **The checker's "well-formed against the model" is well-formed against an extended overlay vocabulary the spike proposes.** The checker refuses the model's own `note` op. It accepts `"banana"` for every logical-time mapping, every 9:36 rule, and every SDC of an added motion (§8).

Smaller findings follow in §§9–11. §12 lists what held up when checked, and §13 says what I did not do.

---

## 1. The charters bind the sibling's superseded text

**What the charters declare** [verified]:

- `quick-council.json` `electronic_meeting_rules.informed_acts`: "every admitted act was formed against a state that includes every previously admitted act (contention spike I1)". 02 §8 says the charters declare I1 "verbatim".
- `recognition`: "residual tiebreak DECLARED: earliest claim round, then rotation pointer (contention spike F3)". The walkthrough's round 2 uses this rule.
- 01 §3.5 item 2: "The adoptable alternative is a change round after a provisional tally (the contention spike's §8)".

**What the contention spike now says** [verified]. Its revision 2 (`ac20802`, committed 13:42) landed eight minutes before this spike's revision 2 (`21c0c50`, 13:50).

- I1 was narrowed to **informed floor acts**: "Every floor act fires against a state that includes every act admitted before it … Non-floor acts in one round are blind to each other by design." The wording the charters carry ("every admitted act") is revision 1's. Contention's revision-1 line 30 matches the charter word for word. Read literally, that wording is violated by the charters' own batching clause ("non-floor acts that commute … may be batched"): in walkthrough round 5, two blank proposals are each formed without knowledge of the other.
- The tiebreak was retracted by name: "**Retracted:** revision 1 ranked claimants by *persistent claim age* … it rewards always-claiming, the disposition councils-46 named as the hazard". Revision 2's F3 now consists of least-recently-given-the-floor plus a seeded slot lottery. The charters' "earliest claim round, then rotation" is the retracted rule.
- Contention's §8 no longer contains a change round. Its §7 row for 45:8 now reads "Change one's vote before announcement … **vacuous by the book's own exception**: 45:8 itself exempts 'some other method that provides secrecy'; sealed-until-announced is such a method". This spike classifies the same vote as `roll-call` *so that* `change-vote` behaves. The two spikes now disagree about whether a sealed-until-reveal ballot is "a method that provides secrecy" under 45:8. One of them has to give way. I lean toward contention's reading, because a round admits one final submission per seat, which leaves `change-vote` with no moment in which it can be exercised [argued]. The spike's own 01 §3.5 already concedes that the trigger is empty.
- Contention §8 also says that the maker's modify/withdraw window costs "one targeted round between 'made' and 'stated'". This bears on §3.5 below.

**Why this is more than a stale cite** [argued]. 02 §2's governing principle is "the charter declares only what the engine cannot compute", and it cites vsect's 189 drifted lines as the reason. I1 is a server invariant, and the integration plan's item 5 proposes it as one. Carrying its text inside every charter is a maintained duplicate of something the server owns. Within eight minutes, that duplicate drifted from its source. The integration plan asks for both: I1 as the server's invariant, and the same text as a required, non-empty charter section. **Strengthening direction:** have the charter *reference* the server's invariant by name and version, the way `parliamentary_authority.pinned` references the model, rather than restating it. Let the per-charter section carry only what genuinely varies between councils, such as the backstop length and the debate mode. The same applies to the parts of `logical_time` that 03 §2 itself marks [derived] from `kind` (`end-of-session`, `next-regular-session`): both charters restate them.

---

## 2. The three gates revision 2 added, and what each still lets through

### 2.1 Rise-and-report: the claims predicate sees no claims

**The repair.** `rise-and-report` is out of order when `count(ctx.floor.claims with purpose debate) > 0`. The intent is that rising "would end the council before it was heard" (feedback §4).

**Why it is vacuous** [argued, from the charter's SDCs, `actions.json move`, and contention rev. 2 §4]. Under the round discipline the charters adopt, claims are per round: "a debate claim is re-formed against the new question each round (claims do not persist)" (contention §7, the row for 42:16). A motion is evaluated against the round-open state. At that state there are no claims from this round, because they are sealed, and none persist from the last. So the predicate is false whenever it is evaluated. Unless the server keeps losing claims across the boundary, which nothing declares, the gate never closes.

`rise-and-report` has `second: false`. `move`'s third floor disjunct is `floor-open AND reg(m).second = false`, so rise can be moved without the floor, which makes it a non-floor act. Under contention rev. 2's snapshot rule, a non-floor act that changes the top frame defers every floor claim in the same round. Here is the sequence that results. In round *k*, C claims the floor to answer B, and A moves to rise. A's motion is admitted and C's claim is deferred. Rise is undebatable, so round *k+1* is the vote, which carries 2–1. The council dissolves with "unable to arrive at a recommendation". C was never heard on the referral, and `every-seat-heard` never applies because it gates only "the final vote on the recommendation".

50:23 says the motion to rise is used "in special committees [only when] they are ready to go out of existence". As compiled, rise is in order from the first open round.

**Strengthening direction.** Make rise out of order while the referred question is undisposed unless `every-seat-heard` holds, or require two-thirds while it is undisposed. Either is an instruction the parent may give [13:8(d)], and both are expressible once presence history exists (F15). Making rise debatable while the question is undisposed would also restore C's turn.

### 2.2 `every-seat-heard` moves the veto-by-silence rather than removing it

**The rule** [verified]: "the final vote on the recommendation is out of order until every seat on the roll has been present for at least one closed debate round on the referred question". Presence is "maintained by submission", and the backstop marks a seat that does not submit as absent.

**The consequence** [argued]. Suppose a seat never submits during a debate round on the question: an MCP endpoint that is down from seating, or a seat that chooses silence. The final vote is then out of order permanently. Quorum (2 of 3) no longer lets one seat block business, but this rule lets one seat block the *recommendation*, and that is the part of the business that matters. The council's only exit is rise with no recommendation, or discharge after `report_by_rounds`. A co-opted seat that never submits blocks the vote in the same way (05 §2 says "the final vote waits until the new seat has sat through a closed debate round"). That makes co-opting a dead endpoint a blocking move available to any majority.

**Strengthening direction.** State the rule as 49:16 states its reason, as *opportunity*: "every seat on the roll has been *asked* in at least one closed debate round on the question since its last change". A seat that was asked and stayed silent had its opportunity. The backstop records that the seat was asked, so the rule needs no new data beyond F15. This also fits §6 below.

### 2.3 `co-opt-member` by bare majority can pack a council in the middle of a question

[argued] `co-opt-member` is adopted by majority. The majority names the new seat's `perspective` and its `agent`, and the new seat "may vote on the pending question". On a 3-seat council that is split 2–1, the majority can co-opt two members of its own choosing (up to `may_add_members_max_total: 5`) and turn a 2–1 into a 4–1. Those members can be forks of a source that agrees with the majority. This attacks the property the council exists for, 50:18's representation of points of view. The spike itself names the concern elsewhere ("agreement among same-substrate agents is coherence, not evidence", 01 §3.5). None of the eleven adversarial cases in 06 covers it. 13:8(d) lets the parent authorize additions, and the parent can attach conditions. Candidate conditions: two-thirds or unanimity; a perspective not already on the roll; the co-opted seat voting only on questions stated after it was seated; or the parent's ratification before the seat votes.

The removal sentence is also inaccurate. "Removal mirrors the adoption threshold [50:14]": 50:14 sets removal at "the same vote as for any other motion to *Rescind* or *Amend Something Previously Adopted*". In a committee that is two-thirds, dropping to a majority only when everyone who voted for the addition is present or notified [35:2(7)]. That is not the majority used to adopt [verified].

---

## 3. The walkthrough (05 §3), checked against the charter's own conventions and the model

### 3.1 Blank-filling has no compiled machinery

[verified] `schema.json` has no Blank or Proposal type. `actions.json` has no action for making or putting a proposal. The `fill-blank-proposal` registry record carries the one-per-member limit and the ordering rules as `limit`/`vote_notes` prose, and `gaps.json` lists "filling-blank ordering [12:107-111]" as annotation-grade. 05's header says the admitted-by column "was re-derived against the expressions after the repairs". For rounds 5–8 there are no expressions to re-derive against. The rewrite was a response to the first report's §7, and it moved the walkthrough's center onto the least-compiled part of the model without saying so.

### 3.2 Round 17's reconsider is not applicable as compiled

[verified] `precedence.json` `reconsider` `applies_to` requires `q` to be a Question in state `adopted | rejected | suppressed-for-session`. `mover-was-prevailing-side` reads `q.disposition_vote.prevailing_members`. The target in round 17 is the vote on the "remove" *proposal*. The model gives a proposal no Question, no `r1_state` and no `disposition_vote`, so the walkthrough's "the first disjunct holds and the motion is in order" assumes an answer to a schema question the model has not asked. Either proposals become Questions, which would be a stage-5 decision that ought to be recorded as one, or round 17 is refused. The F12 analysis attached to round 17 is right about the guard, but it is the wrong row to demonstrate it on.

### 3.3 The vote order on proposals is undefined when proposals arrive in one round

[argued] 12:107–111 order the votes by proposal order (names) or by "the particular circumstances" (places, dates, numbers: "a choice of methods … The particular circumstances must determine the order to be used"). The second is a chair's judgment, and a mechanical chair has no rule for it. In round 5 both proposals are batched non-floor acts in a single round, so "proposal order" does not exist. The walkthrough votes "keep and sync" first because it is "the parent's first". The first proposal to reach a majority fills the blank, and when preferences cycle among three or more alternatives, the order decides the outcome. The order is therefore set either by the parent's listing, which is the kind of prior the first report's §7 removed, or by arrival, which contention's F3 forbids reading.

The spike also has a choice to make about using a blank for the operative verb. 12:93 lists "names of persons or places, dates, numbers, or amounts" as typical cases. That list is not exhaustive, so the device is available. But when the blank *is* the whole question, 12:94's "the vote that fills a blank does *not* decide the question" is true in form only. The charter's `blank_proposals_offered_by_parent` field has no defined procedural effect. Do the parent's proposals pend, like the struck specification in 12:95(b)? Do they use up a seat's one proposal under 12:97? The field doesn't say.

### 3.4 Rounds 17–18 break the charter's own duty-sequencing rule

[verified] Proposals in a debatable motion are debatable [12:98], and `reconsider` is debatable when its target is (registry). The charter's `duty_sequencing` puts a question "only after a closed round in which no present seat claimed the floor for debate, following the stating of the question". Round 17 makes the reconsider and round 18 votes on it, with no stating row and no pass round between them. The same compression would apply to any debatable motion that the walkthrough takes straight to a vote.

### 3.5 The 4:19 window is empty in every row

[verified] Row h of 01 and the charters' `motions_in_writing` say a motion is "revealed in the round it is made and stated by the chair at the start of the next round, preserving the maker's free-modify/withdraw window [4:19]". In the walkthrough, round 9 makes the amendment and row 9′ states it, "the one-round window of 4:19 having passed". But no round falls between 9 and 9′. The maker never gets a submission in which the motion has been revealed and not yet stated, and 4:19's right ("*Until the chair states the question*, the maker has the right to modify … or withdraw") is exercised in exactly that interval. Contention §8 prices this correctly: "One targeted round between 'made' and 'stated'". The charter needs to either spend that round (one per motion), or declare the window dropped with its cost, as it did for mid-speech interruption.

### 3.6 The count

[verified, by counting the table] The text says: "twenty-two submission rounds … Of them, six are votes or consents, four are pass rounds … three are the inquiry and recess." The table has six vote rounds (7, 8, 13, 16, 18, 20) and two consent rounds (11, 22), so eight; it has three pass rounds (6, 15, 19); and the inquiry/recess rounds (10–13) overlap the vote and consent count. 04 §4 says "the drop always fires" for rescind, while 04 §7 records the correction of that same "always". Under the charter's own conventions — a round per window (§3.5), a stating row and a pass round before every debatable put (§3.4), and the vote deferral of §2.1 — the total rises above twenty-two. My hand count gives something in the high twenties [guess]. None of this changes the spike's qualitative point that I1 is expensive. It does mean that the line headed "Count, honestly" is wrong in its own arithmetic.

---

## 4. The report: 51:2, who drafts it, and where the answerability hole went

**The text** [verified]. 51:2: "a report of a board or committee can contain only what has been agreed to by a majority vote at a regular or properly called meeting". 51:7's detailed form is the charge, the facts, the findings, and the recommendations.

**The walkthrough** [verified]. Round 22: "adopted → effect: the report (charge, facts incl. the inquiry's, findings, recommendation as a resolution, per-seat votes with pointers, minority view)". The only text any vote adopted is the recommendation (round 20). Nobody drafts the facts or the findings, and nobody adopts them. The `rise-and-report` effect "emit[s] the report". Under a mechanical chair and a server secretary, that leaves two readings [argued]:

- The server composes the facts and findings. Then the council's report contains text no seat agreed to. That violates 51:2, and in the estate's terms it is who-decided laundering: findings labeled as the council's when no seat adopted them.
- The report omits facts and findings. Then the charter's own `report-form` instruction is unmet.

**Answerability, again** [argued]. The revision-2 rule ("reasons are given IN DEBATE … never first on the vote") binds *votes*. Findings and minority views are post-debate text too, and the instruction does not restrict them. A minority view "of right", written after the vote, can raise an argument nobody can answer, which is the case §3.5 item 3 says is carried.

**Strengthening direction.** Make the report a motion. Rise-and-report (and the standing council's `report-referred-matter`) would carry the report's text and be debatable and amendable as to that text. Alternatively, a separate "adopt the report" main motion could precede rising. This is the book's own shape: the committee agrees to its report [51:2], and the per-seat record and the pointers stay as they are. Minority views can be held to the same pointer rule as votes, or received as the book receives them, by the parent's leave [51:69]. Either choice is available, but it has to be made one way or the other. The round count rises again.

**Two citations near this point** [verified]:

- The charters' "the parent is entitled to the whole record [49:19]" rests on a paragraph that does not say it. 49:19 lets the assembly *order* a board's minutes produced, "by a two-thirds vote, the vote of a majority of the entire membership, or a majority vote if previous notice has been given". That is a conditioned power over a *board*. For committees the book's default runs the other way. 51:65: "No one can make allusion in the assembly to what has occurred during the deliberations of the committee, however, unless it is by report of the committee or by unanimous consent." So the "option not taken" the charters record (sealing per-seat votes from the parent until the parent has acted) is closer to the book's committee default than the charters' choice is. The charters' choice may still be the right design for this estate. It is a departure from the book's default, and it should be labeled as one rather than cited as entitlement.
- The walkthrough's event `report-adopted` names the council's emission with the word the book reserves for the *parent's* act [51:10–14], and the parent declining adoption is the veto. The model's vocabulary has `reported`. It does not have `report-adopted`, `subcommittee-reported`, `member-seated`, `charter-loaded`, `seat-credentialed` or `round-closed`, and `logical_time.hour` already uses `event/subcommittee-reported`. These events are a new vocabulary. It should be listed with F4's interface changes, and the council-side event should not be called "adopted".

---

## 5. `commission-inquiry` still spawns a subcommittee of non-members

[verified] Revision 2 changed the warrant from 50:15 to 13:8(d)'s "employ an expert consultant", but it kept the construct. The effect is still "spawn-subcommittee with a derived special charter (parent = this council, referred = brief, composition as moved)", staffed by "expert forks (non-members)". 50:15: "Subcommittees must consist of members of the committee, except when otherwise authorized by the society in cases where the committee is appointed to take action that requires the assistance of others." A consultant is not a committee, so 13:8(d) warrants hiring one; it does not warrant creating a subcommittee of them. The standing charter contradicts itself here. Its `may-appoint-subcommittees` reads `with_non_members: false` "[50:15 — non-member subcommittees only where the committee is appointed to act, which this one is not]", while its `commission-inquiry` (effect "as quick-council.json") seats a subcommittee of forks.

The first report already named the shape that fits: "an expert input with no subcommittee machinery". A consultant's brief and return would carry no roll, quorum or charter, and no `ctx.subcommittees`. The return would still be "in the nature of debate" [52:28]. That also removes the `requires_schema: ctx.subcommittees` dependency from the quick council's critical path. A real subcommittee of *members* (`commit`, or `may-appoint-subcommittees: of_members`) stays available for when one is wanted.

There is a related gap in sealing [argued]. The `equipment` rule withholds access to the council store from *members*. Inquiry forks are non-members, they keep their file tools (they have to, to "count files opened without vsect in a week"), and in the declared departure their inquiry runs while sealed rounds continue. 9:36's final item is exactly this hook: "provisions for ensuring that nonmembers cannot participate in meetings (unless properly invited to do so)". Neither charter, and not the checker's 9:36 key set, includes that item or 9:36's notice-of-participation item. The withholding cuts the other way too. A seat whose perspective is "what main actually holds, re-derived rather than remembered" (`seat/canon`) cannot re-derive anything without file tools. Isolating the council *store*, for example with filesystem permissions, rather than withholding file tools, would protect sealing without disabling the perspective [guess at mechanism].

---

## 6. The principle, the budget, and the reply round

**The principle as stated** (01 §1): "a decision is the body's only if every member had the opportunity to hear every argument and answer it before the vote closed [49:16's reason]". 49:16's reason is "the members lacked the opportunity to mutually debate and decide the matter as a deliberative body" [verified]. The restatement adds "every argument" and "answer it". Those additions are what generate 01 §3.5's items 3 and 4.

**The reply rule does not close the hole** [argued]. "When the budget is exhausted, one reply round follows in which each seat may speak once." Either reading fails:

- If the reply round is a single round with every seat speaking, it admits several floor acts in one round. That breaks I1 and the declared `debate_mode: "serial: one speech per round"`.
- If the reply round is *n* serial rounds, the last reply speech is unanswerable. A seat whose argument was the last budgeted speech also gets a reply, and nothing stops it from raising a new argument.

Either way, the unanswerable final word is moved by one speech. Every procedure with a bound has a last speech. The book itself, in assemblies, lets two-thirds close debate on the previous question [16] knowing that the last argument goes unanswered.

**The choice the spike has not made.** One option is to keep the strong principle. Then debate must be unbounded and end only by a pass round. That happens to be the book's committee default: "motions to close or limit debate … are not allowed in committees" [50:25]. The parent's budget then becomes a declared drop with its cost named, like mid-speech interruption. The other option is to state the principle at 49:16's strength, the *opportunity* to debate. Then the budget is a legitimate parent instruction (50:25 n7 names "an order limiting or closing debate in the committee" among the parent's remedies), and the reply round is a courtesy rather than a carrier. The second option also gives `every-seat-heard` its better form (§2.2). A middle path exists too: reply-round speeches may not raise new matter, adjudged when challenged, as formal debate formats limit rebuttals. But "strike from the record" does not remove a speech from a seat's context, so row b's "which the record can strike" is a weaker remedy for agents than for people [argued].

---

## 7. The corridor's elapsed-world conjunct is non-causal where it matters

[verified] The standing charter declares `within-quarterly(s1, s2) ≡ no seat crossed a term boundary between s1 and s2 AND no advisory input or referred paper the question rests on has changed`. Two of the model's consumers evaluate it *when a postponement is moved*: `guard/postpone-horizon` and `guard/schedulable-to`, each `["if", ["within-quarterly", current_session, next_regular_session], …]`. At that moment, whether the world will change between now and the next session is unknown [argued]. The conjunct can be evaluated in `carryover-eligible` and in the timers' `expires_at` *at the later session*, but at making time it is an undefined value inside an admissibility test.

There is a second consequence [argued]. In an active refactor, something a question rests on changes between almost every pair of sittings. As written, the standing council's postponements, tablings and unfinished business would fall to the ground at nearly every adjournment. The council would lose carryover in practice, and the debate record behind a question would be lost with it (21:7(c): "introduced at the next session, the same as if they had never before been brought up").

The book's own instruments for "the grounds have moved" are renewal after progress [38:7], reconsideration and rescission. They act at the later session, by a majority that can see what moved. That is also 8:12's actual principle: "one session cannot place a question beyond the reach of a majority at a later session". The principle concerns the later majority's freedom, not the state of the grounds. A causal alternative that keeps the spike's insight: let the corridor stay a property of the schedule and the roll, and add a *later-session* rule that a carried-over question whose papers lapsed comes up with the lapse shown, available to the existing renewal machinery.

---

## 8. What the checker certifies, re-run

[verified, by run] `mutant.py` passes 7 of 7 as claimed. Mutant #3 (logical time) and mutant #7 (9:36 rules) now use the empty string, but the first report's mutant set every logical-time value to `"banana"`. I ran one further mutant: `"banana"` in every `logical_time` and `electronic_meeting_rules` value; `class: "Q"`, `debatable: "banana"` and `vote: ["banana"]` on `rise-and-report`; the model's own `{"op": "note"}`; and a two-seat roll. Result: exactly one problem, "overlay[15] note: unknown overlay op". The prose and the nonsense SDCs pass. The two-seat roll draws no warning.

What that shows:

- **The op vocabulary is the spike's, not the model's.** `council.json` `overlay_mechanism.ops` defines `param`, `patch {surface, replace}`, `disable_motion`, `disable_action`, `add_action`, `restrict_actions {allowed, plus}` and `note`. Both charters rely on four shapes it does not define: `declare_param`, `add_motion`, `patch` with `add`, and `restrict_actions.scope`. The protective conjuncts that revision 2's main repair depends on (the added motions are "reached through `move`") exist only under the proposed `add_motion`. F14 names that op, and integration item 5a proposes it, so nothing is hidden. But "well-formed against the model's namespace and types" should say "against the model plus the overlay ops proposed in F1/F3/F10/F14". A loader built from `council.json` as written would reject the charters, and the checker rejects a charter that uses the model's `note`.
- **SDC values are not typed.** The checker tests that the keys are present (`SDC - set(m)`) and does nothing more. `registry.json` has `debatable_values`, and `vocabulary.json` has the classes and thresholds to check against.
- **The quorum warning fires only for a numeric quorum.** A two-seat council under `enrolled-majority` has a quorum of 2 of 2, which is the whole roll. That is the veto-by-silence hazard of the first report's §6. 06 §2's "council of two" does not mention it, and Joseph's "2/3-person quick-councils" includes this case.
- **`report-referred-matter`'s applicability uses `["param", "q"]`.** `move` supplies `m, t, time, series`. The checker's scope adds the motion's own params, so it passes, but per INTERFACE "an unresolved `param` … is an **error**". The `reconsider` row in `precedence.json` declares `"bindings": ["m", "q"]`, and the added row needs the same.

---

## 9. Model and book points the revision states more strongly than the text supports

- **06 §10: "The main-motion row's control bars and the referral-as-call bound [9:15, 36:2] refuse it with the why-not tree."** [verified] `precedence.json` `main-motion` has three bars: no question pending, substantially-same, and decided-this-session. None of them bounds scope. Neither charter patches that row. The only carrier of the bound is the prose field `main_motion_bound`, and `germane` is an amendment predicate (default `true`). So an out-of-scope main motion is admitted silently unless a seat raises it. Then, under a mechanical chair, it becomes a submitted ruling. The why-not tree is not "refused by construction".
- **"A mechanical chair is a chair that is always in doubt."** [verified for the defaults, argued for the consequence] The adjudication defaults make the mechanical chair a chair that *always rules the default*: `germane: true`, `dilatory: false`, `legitimately-incidental-residual: true`, `substantially-same: false`, `interruption-urgency: false`, `quorum-conduct-related: false`. It submits only when something is raised, or when the default is `null` (`privilege-admissible`, `divisible`, `prevailing-side-claim`, `ballot-meaning`). The cost goes beyond the lost sua-sponte duties. Every contested judgment goes straight to a majority vote [23:18–21], so a 2–1 bloc decides every germaneness, dilatoriness and same-question ruling. A neutral chair's ruling, which needs a majority to overturn on appeal and is sustained on a tie [24:3(7)], is the protection the minority loses. It belongs in the charter's named costs, next to the lost sua-sponte duties.
- **The added motions are classed `"I"` (incidental).** [verified] That routes their admission through `guard/legitimately-incidental`, whose residual defaults to `true`, so they are admitted. It also means each "must be decided first" over the question it arises from [6:18]. That is what happens in the walkthrough's round 11. `co-opt-member`, though, is described as "a question of the privileges of the assembly — its organization [19:7]". The book's class for that is privileged, rank 10, which has a different yield behavior. 05 §1's "refused … when a debate-closing order precludes it" is also inaccurate: `no-order-precluding` bars only class-S motions, plus commit and postpone.
- **"A standing council is constituted by the parent's 50:8-grade act."** [verified] 50:8 applies only "if any of the following conditions are to apply": standing authority to act, automatic referral, or another rule affected. 50:9: "If a standing committee's assigned function does not affect a rule … in any of these three ways, it can be established by a standing rule adopted by a majority vote without notice." The standing charter sets `automatic_referral: false` and `may_act_with_effect: false`, and cites 50:8's first bullet for *originating* business. Originating recommendations is not "authority to act for the society", so 50:9 likely governs. For a steward-parent the threshold makes no practical difference. The point that growth between kinds is the parent's act, not an edit, stands. The "50:8-grade" phrasing in 02 §6, the debrief and the standing charter is stronger than the book.
- **Three identities, one clause that needs two of them.** [argued] 37:35(3) asks whether "all the members … who voted with the prevailing side are present". The set is defined by accountability identity (the fork keys that voted). Presence is defined by counting identity (the seat). After a re-fork, the old fork keys are never present again, so two-thirds applies forever. Or the seat stands in for them, and then the accountability split has been undone for this clause. The charters do not say which; 35:2(7) has the same structure. The debrief also celebrates a consequence that, read the other way, defeats a rule. "A re-fork 'did not vote' … so may move to reconsider" means that re-forking a seat clears its losing-side status, and that was 37:35(2)'s only remaining restriction in committees. If seats are re-forked every sitting, as in 06 §4's case, the prevailing-side rule is void across sittings.
- **The rights-critical floor the plan proposes would refuse the plan's own quick charter.** [verified] Integration item 3 says to "derive the rights-critical floor … from the model so no whitelist can drop it". The ROADMAP defines that floor: "everything reachable by a single member's demand (`single-member-demand` / `not-voted` thresholds), the interrupt-capable acts, and the enforcement family (point-of-order, appeal, division, objection, put-ignored-question)". In the registry, `call-for-orders-of-the-day` and `division-of-assembly` are `single-member-demand`, and `objection-to-consideration` is conditionally interrupt-capable. The quick charter excludes all three, plus `demand-division`. The demonstrated no-gos (division) need a recorded vacuity exemption, or the derived floor and the whitelist will disagree. The other exclusions need re-examination against the floor. `call-for-orders-of-the-day` is excluded as having "no orders beyond the one special order", yet the walkthrough schedules to events (round 12), and that call is the member's remedy when an order is not taken up.

---

## 10. Smaller items

- `proposed-integration-plan.md` was changed in only two places in revision 2 and is stale in several others. Item 8 recommends "the generic privilege route as fallback" for both in-meeting acts, but 05 §1 now says a council without a grant can only *ask the parent* for an inquiry. The do-not-inherit list still carries "same source = same member … 06 §4 argues the opposite", and revision 2 replaced that with the three identities. It also still lists "roll call and ballot at once", which revision 2 withdrew. F10–F13 and F15 have no suggestion, even though the debrief calls F10 the reason no council on the chain runs. Item 1 and 02 §8 map the walking-skeleton's `agenda` onto `agenda.items`, which neither charter has any longer. The walking-skeleton's `quorum` is a number, while the charters' is `{basis, value}`.
- The response's §3.4 says "00 H5 left as the historical guess with a note". `00-neighborhood-map.md` was not touched in revision 2, and H5 still reads "RONR could never have both" with no note [verified, git log]. The response's §3.3 "walkthrough row 10" is row 17 now.
- 03 §1 says its table was "inventoried mechanically". `grep -o '\["now"\]'` gives actions 2 (the table says 5), adjudication 0 (1), schema 0 (1). The schema has a `clock.now` field, not the operator. Nothing downstream depends on these counts [verified].
- 01 row b: "the point of order in the next round … which the record can strike". For agents, a revealed speech is already in every seat's context, so striking it from the record does not undo hearing it [argued].
- The quick charter's `DERIVED` display sections bind nothing, as labeled. They are still maintained copies of computed values, the same drift class as §1.

---

## 11. A framing observation for Joseph

[argued] In this architecture every report lands as `decided-by: delegated`, and nothing takes effect until you act. So the 9:34 question ("is a round-run council a meeting?") decides less here than it would in a society. It decides what a council may *call its own* (a majority report versus a 51:2 "presentation … made merely upon separate consultation [that] must be described thus"). It decides what re-pends in a parent *council*. It does not decide what takes effect. That suggests the departure could be stated as a labeling rule on the report, which is something the book already supplies wording for, rather than as a validity claim. It would also shrink what the integrator has to defend if the departure is ever challenged.

---

## 12. What held up when checked [verified]

- **The instance/type split.** `council.json` configurations carry no roll, grant, referral or pin, and `ctx.config` holds `{configuration, <params>}`. The checker's counts are reproducible: 21 of 31 registry motions plus 3 added in the quick charter, and 26 plus 3 in the standing charter.
- **F10 is right.** `state-question` requires `seconded`. Small-board's `moved -> pending` and chair-votes patches are English. The compiled replacements in both charters have the right shape, and the patched `state-question` keeps the dilatory conjunct.
- **F12's reading of `mover-was-prevailing-side` is right.** The desc names the committee rule and the expression has no branch for it.
- **The quorum fix works, given F14's op.** Once the added motions go through `move`, they inherit `move`'s quorum disjunct (`quorum-conduct-related` defaults `false`), so they are refused when the council is inquorate.
- **The landing vocabulary matches the ADR.** `deciders` with `council` as a role "under delegated authority (<grant slug>)"; `decided-by: delegated` until the decider supports, ratifies, rules or declines.
- **The book, read whole.** 9:30–9:36 (including 9:34's exclusion and 9:35's committee authorization by referral instructions); 13:8 (a)–(d), including "employ an expert consultant", "even to add to its own membership", and majority adoption despite conflicting instructions; 13:18; 13:23; 10:54 and 10:55; 40:12; 41:57; 41:61; 42:14 and 42:15; 45:8; 45:18; 45:46; 49:16; 50:7–50:11; 50:14; 50:15; 50:19; 50:21; 50:23; 50:25 with n7; 50:26; 50:28; 50:29; 51:2–51:7; 51:64–51:71; 52:15; 52:28; 12:92–12:99; 12:107–12:111; 35:2(7); 36:4(7); 37:35; 4:19; 4:58 and 4:59; 8:11 and 8:12; 9:7–9:10; 9:13; 9:15; 9:17; 21:7. The spike quotes all of them accurately where it quotes them. My disagreements above concern what the spike draws from them, not its transcription.
- **The specimen facts** (`specimens-2026-11-03.md` lines 26 and 65–66) are as the charter cites them.
- **Write-scope.** Revision 2 touched only this directory (`git show --stat 21c0c50`).

## 13. What this pass did not do

- I ran no engine. There is none yet. Every admissibility finding comes from reading expressions, not from evaluating them.
- I did not read `scheduler.json`, `rules.json`, `voting.json` or `acceptance.json` beyond what is cited above, or the walking-skeleton's code beyond its charter loader. I did not read the contention spike's own second feedback (`fb98917`). My reading of contention rev. 2 is my own, and §1 depends on it.
- My round count in §3.6 is a guess; I did not redo the full table.
- The §2.1 rise sequence depends on contention rev. 2's per-round claims and snapshot rule. If the integrator keeps claims across round boundaries, the gate becomes live, but then it needs a declared claim-lifetime rule.
- I did not try to extend 01 §3.5's list of what aural simultaneity provides. This report's findings list is its content. Silence on a claim means I did not check it, not that it passed.

*On the brief: it was clean. "Re-derive from what is written rather than from what you can tell was intended" did the same work here that it did for the first verifier. In particular, it is why I checked the sibling's current text rather than the text this spike quotes from it.*
