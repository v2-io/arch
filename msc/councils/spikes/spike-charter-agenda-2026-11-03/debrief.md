# Debrief — for Joseph

*Revision 3, after two independent verifications (`de-novo-feedback-1.md`, `de-novo-feedback-2.md`) and my repairs (`response-to-feedback-1.md`, `response-to-feedback-2.md`), written by re-reading the corrected files rather than from memory. Where an earlier debrief said something a verifier showed false, this one says so. Some items are HELD pending your answer on whether council debate is serial or parallel; they are marked. Pre-spike vocabulary throughout.*

## The problem you set

You wanted to know what a council's charter and agenda are as things the server loads: what has to be written down to seat one, how it comes into being, how it takes a question to a recorded decision and reports to you, how it grows, and which parts of Robert's Rules still mean anything when the members are agents and time is counted in turns. And whether starting narrow (a 2–3 agent council, one decision) and starting broad (the whole book, cut down) are one road or two.

## What I did, in order

1. Read the whole model, the parts of the book it leans on, and the estate pieces a council plugs into. Wrote my own map before reading the coordinator's drafts.
2. Took the hardest thing first — the book's statement that a group deciding in writing is not a deliberative assembly — and used it, as you asked, for the principle under it.
3. Wrote the charter as a real artifact, twice (a 3-seat quick council; a 5-seat standing council), with a checker; then the clock-word translation, a motion-by-motion survey, your two in-meeting acts as event streams, a walkthrough on a real vsect specimen, adversarial cases, and a list of model defects.
4. A first verifier found my strongest claims overstated and the added acts unguarded; I repaired. A second verifier confirmed the architecture and found the procedural repairs under-derived; I repaired again, and held what depends on your debate-mode decision.

## What stood, across both verifications

**A charter is an instance, not a type.** The model's configurations (committee, small board, …) are types. The charter is who sits, under what grant, on what question, with what instructions, under which pinned version of the rules, and how turns and time work. The rule for what goes in it is vsect's: **declare only what the engine cannot compute** — and, I learned the hard way, **never restate what the server owns**: my charters copied the contention spike's rules word for word, and the copy was stale within eight minutes. They now reference that spike by revision, the way they pin the model.

**For a quick council, the charter and the agenda are the same document.** A *special* committee has one task and dies when it reports; its agenda is its referral. A *standing* committee has an order of business and adopts an agenda per sitting. Your two routes are this distinction, and **one format expresses both on one engine**. Not "one road by editing": a quick council doesn't grow into a standing one — a standing committee is constituted by your own act (the book's 50:8 or 50:9 depending on what it may do), and the book bars a quick council inside a standing one's class.

**The report lands as delegated.** Every report is `deciders: council under your grant; decided-by: delegated` until you support, ratify, rule or decline. Nothing in the tool upgrades it. Both verifiers checked this against the udon decision record and found no problem.

**The small-board chain can't state a question.** The model's own patch for "no second required" is written in English, not in its expression language, so as shipped no committee-derived council runs. Both charters carry the compiled form. Both verifiers call this finding right and important.

## What verification changed, and what it is now

**Whether it's a "meeting."** The book is explicit (9:34): a written process, whatever the technology, is "consultative" and "foreign to the deliberative process"; such a committee may report only what every member agreed to. I had called the electronic-meeting rules "a slot the book left open." They aren't. The honest statement: **by the letter, a turn-based council may report only unanimity; by principle — that each member had the opportunity to mutually debate and decide (49:16's own words) — a council run under the round discipline claims the majority tier as a declared departure.** The second verifier added the right framing: since nothing takes effect until you act, this decides what the council may *call* its own, not what happens — so the charters carry a labeling rule: the report names itself a majority report of a round-run council and, by the letter, a presentation on separate consultation. That's wording the book supplies.

**The principle, stated too strongly.** I had restated 49:16 as "every member could hear every argument and answer it before the vote closed." That's stronger than the text, and no bounded debate can meet it — there is always a last speech. The charters now use 49:16's words. What follows for your token budget on debate (keep it, as a declared drop from the committee's unbounded-debate default? or treat it as a legitimate instruction under the weaker principle?) is **PARKED** for your debate-mode answer.

**The report is adopted, not emitted.** The book: a report "can contain only what has been agreed to by a majority vote." My walkthrough had the server emit the facts and findings after a vote on the recommendation only — findings labeled as the council's that no seat adopted; in our terms, laundering. Now the report's text is the content of the motion to rise and report, debatable and amendable as to that text. Minority views point to statements made in debate, where they could be answered.

**Your access to the record is a departure, not an entitlement.** I had cited 49:19 for "the parent is entitled to the whole record." That paragraph is a conditioned power over a *board's* minutes; the committee default runs the other way (51:65: nobody alludes to what occurred in a committee's deliberations except by its report). The charters now declare your full access as a departure from that default, with the cost named: per-seat votes you read are a channel for voting toward you.

**The research inquiry is a consultant, not a subcommittee.** The book bars a committee that only reports from forming subcommittees of non-members (50:15); what the referral may authorize is "employ[ing] an expert consultant." So the in-meeting act is *engage a consultant*: an expert fork with a brief and a return, no roll or quorum, participating only by its return. Its return is "in the nature of debate" — but the book's breakout groups meet during a *recess*, so running the inquiry while debate continues is a declared departure.

**Co-opting could pack the council.** By bare majority, a 2–1 council could co-opt two agreeing forks and become 4–1 on the pending question. Now: two-thirds to co-opt; the new seat's perspective must not already be on the roll; a seat added while a question is pending doesn't vote on it. And I had said removal "mirrors" adoption — the book sets removal at the rescind thresholds, which in a committee is two-thirds or a majority when everyone who voted for the addition is present.

**Veto by silence, twice.** My first quorum (all three seats) let any seat block every vote by not answering. I moved to a majority quorum and a rule that the final vote waits until every seat has *been heard* — which the second verifier showed just moved the veto onto the vote. Now it is *every seat asked*: a seat that was asked and stayed silent had its opportunity, which is what 49:16 protects. The same fix means a dead endpoint co-opted by a majority can't block either. One hazard stays and is declared: on a two-seat council, a majority *is* the whole roll, so the per-round timeout and its remedy must be chosen knowingly.

**The corridor was judging the future.** For a standing council I had made "did the world move?" part of the test for carrying a question over — but the model evaluates that test when a postponement is *moved*, about a future nobody knows, and in an active refactor it would have killed carryover at nearly every adjournment. Now the corridor is a property of the schedule and the roll only, and the "grounds moved" test acts at the later sitting: a carried-over question whose papers lapsed comes up with the lapse shown, open to the book's own renewal, reconsideration and rescission — the later majority's freedom, which is what the book's principle actually says.

**The blank was the wrong instrument.** I had recast the referral as a resolution with a blank so neither "keep" nor "remove" was pre-drafted. Good instinct, wrong device: the model has no schema for blanks or proposals, the vote order among proposals offered in one round is undefined, and a proposal is not a question you can reconsider. Now the referral is a *subject*; the council drafts, as the book says a committee should; the alternatives meet as a main motion and a hostile-but-germane substitute. The scope bound on main motions, which I had left as prose, is now a compiled rule (still a judgment call when challenged — the book's germaneness predicate).

**The checker now says exactly what it certifies.** First version: resolved names only, and passed a seven-way broken charter. Second: passed types and enums but still passed nonsense motion characteristics and refused the model's own `note` op. Third: types the characteristics, accepts `note`, requires declared bindings, checks the rights floor against the whitelist, warns on tiny rolls — and says plainly that it cannot type prose (a nonsense time mapping passes) and that it checks against the model *plus four overlay operations I'm proposing*, which the model does not yet have. Both verifiers' mutants run: fifteen checks, twelve caught, one accepted on purpose, two prose cases named uncatchable.

**The vote.** It is the book's signed ballot (45:46), not something the book "could never have." There is no change of vote after submitting: the contention spike reads the book's own exception for secret methods as covering a sealed-until-announced vote, and a first submission in a round is final; the drop is priced (a mis-submitting agent has no correction).

**The mechanical chair.** It does not "always submit to the council" as I wrote; it rules the book's defaults (germane, not dilatory, not the same question) and submits only what a seat raises or has no default. Its second cost, which I had missed: every contested ruling goes straight to a majority, so a 2–1 bloc decides every germaneness and dilatoriness call — the minority loses the neutral ruling that is sustained on a tie. Both costs are named where the chair mode is chosen.

**Three identities for a forked seat**, declared separately: counting (the seat), accountability (the fork that acted — and, from the second verifier, a seat's recorded vote binds its occupant for the rest of the session, so re-forking mid-session doesn't launder a losing-side status), continuity of judgment (not asserted). My earlier line that "a re-fork may move reconsider" holds only across sessions, where the book's own "absent" rule applies.

## PARKED (the serial-floor findings)

The walkthrough's procedure rows and its round count (my "22 rounds" breakdown was wrong in its own arithmetic and rises further under the charter's own conventions); whether to spend a round on the maker's modify-before-stating window or declare it dropped; the reply round and the debate budget; the exact form of the gate that stops two seats from rising and ending the council before the third was heard (a presence-based gate is in place provisionally).

## Outcomes

- Two charters at revision 3, well-formed against the model plus four proposed overlay operations; a checker and a two-set mutant that say what they can and cannot certify.
- Seventeen model defects or gaps, four of them real defects: the committee-of-the-whole overlay turns off the chair, omits the act that makes motions and the lone objection; the small-board chain's patches are prose; added acts could bypass the model's gate (now they can't).
- The principle, the departure, and every drop written down where the charters claim what the letter withholds.

## One more thing, from the higher-council principles

The document Joseph had written alongside ours (`~/src/arch/msc/high-councils/higher-council-principles.md`) reaches, from Ballard and the Handbook, the place my 51:2 finding reached from the book: a council that decides only with unanimity, else holds. Put together, a unanimity council is a valid committee **by the book's letter**, round discipline or not — the departure I defended is needed only for majority decisions, which that shape doesn't take. Reading it also caught a gap in my own charters: the model's small-board `discuss-informally` [49:21(4)] — the nearest hinge to counsel rounds where every seat speaks — had been silently dropped by my whitelists; it is now whitelisted and the checker reports the class. The full reaction, including why the "presider who speaks last" is a third chair mode that answers the mechanical chair's bloc cost, is `reaction-higher-councils.md`.

## What is not done

No engine ran; every admissibility claim comes from reading expressions. Three schema extensions (presence history, consultant tasks, the scope bar) are declared, not built. The checker cannot type prose. The list of what the room provides (ten mechanisms plus the first verifier's four) is not proven complete. And the held items are held.
