# Debrief — for Joseph

*Written last, by re-reading the corrected files in this directory rather than from memory of doing the work. In the vocabulary we had before the spike: council, charter, agenda, member, round, report, parent, veto. Where I coined something I say so.*

## The problem you set

You wanted to know what a council's charter and agenda are as things the server loads — what has to be written down to seat one, how it comes into being, how it takes a question to a recorded decision and reports to you, how it grows, and which parts of Robert's Rules still mean anything when the members are agents and time is counted in turns rather than hours. You weren't sure whether to start narrow (a 2–3 agent council with one decision) or broad (the whole book, then cut down), or whether those are one road or two.

## What I did, in order

1. Read the whole model and the parts of the book it leans on (committees, referral, sessions, electronic meetings, reports, quorum, the chair's duties), and the estate pieces a council plugs into (decision authority, agent identity, experts, vsect). Wrote my own map before reading the coordinator's drafts.
2. Took the hardest thing first. The book says, in its very first definition, that a group deciding things in writing is not a deliberative assembly and "many of its rules … will not be applicable". Our councils are written and turn-based. Mid-spike you said you don't care about the book's labels, only the principles it surfaces — so I used that passage for the principle it carries: **a decision belongs to the body only if every member could hear every argument and answer it before the vote closed.** Humans get that from being in one room at one time. I listed the ten rules in the book that depend on being in one room, and showed what a turn-based round has to carry in place of each.
3. Wrote the charter as an actual artifact, twice — one for a 3-seat quick council, one for a 5-seat standing council — in the model's own vocabulary, and wrote a checker that resolves every name they use against the model. Both load. Then the rest: how the clock words translate, a motion-by-motion survey, the two in-meeting acts you asked about as step-by-step event streams, a full walkthrough of a quick council deciding one of the real vsect specimens, eleven adversarial cases, and a list of defects I hit in the model.

## What I found

**A charter is an instance, not a type.** The model already has "configurations" (committee, small board, convention). Those are types. What no type carries is who sits, under what grant, on what question, with what instructions, under which pinned version of the rules, and how turns and time work. That instance is the charter. The rule for what goes in it is the one vsect lives by: **declare only what the engine cannot compute.** Quorum, minutes, the action list, and — for a quick council — the agenda itself are all computed.

**For a quick council, the charter and the agenda are the same document.** The book has two kinds of committee: *special* (one task, dies when it reports) and *standing* (lives on, takes a stream of referrals). A special committee's agenda is just its referral — the book even has a name for a meeting devoted to one thing, "the special order for the meeting" — and the committee can't change it, only report or ask you for instructions. A standing committee has an order of business and can adopt an agenda per sitting, which then binds. Your two routes — narrow-up and broad-down — are this distinction. Both run on the same engine; they differ in one field (`kind`) and in how long the list of permitted motions is. **One road.**

**"Growing is widening the list" is true for motions and false for two other things.** The coordinator wondered whether a quick council is just the model with most motions switched off, and growing it is switching more on. For motions, yes — I checked that turning a motion off leaves nothing dangling. But two kinds of growth aren't list edits: the model needs new *records* (a committee record so a council can have subcommittees; a sitting record so a standing council can have sessions), and the new acts need an *executor* (something that actually forks an agent or hands out a credential). Those are named in the charters as declared extensions so nobody mistakes a list edit for them.

**The round is what replaces the room.** I named the property the server has to keep the "common-state condition" (my coinage): every act is formed against a state every present member has seen, and nothing advances until everyone has had that chance. Under it, the ten room-dependent rules translate cleanly, and four things are left that genuinely can't be derived and must be written in the charter: how presence is established, who wins when several members speak at once (the tiebreak), whether a motion is stated the moment it's made, and what happens when an agent doesn't answer in time. The book itself lists almost exactly these four as the things a body must write down to meet electronically — so your "turn-based, wall-clock-ambivalent" concern already has a slot in the book, and it's in the charter.

**Two things agents get that humans never could.** A sealed round-vote is both a secret ballot (nobody sees anyone else's vote before casting) and a roll call (every vote is on the record with a name). The book says those are opposites. And the rule that only someone who voted with the winning side may move to reconsider — which the book admits runs on the honor system — becomes a computed fact.

**Your two in-meeting acts are both in the book.** A research subcommittee that reports back without stopping the meeting is the book's "breakout group", whose report is "in the nature of debate" — information, not a motion. Adding a member mid-meeting is a "question of the privileges of the assembly" concerning its *organization*, and the book explicitly lets a referral authorize a committee "even to add to its own membership". Without a grant in the charter, each costs an extra vote on whether the interruption is admissible; with the grant, each is one round. The charters carry the grants.

**Your veto has a name in the book, and so does delegation-as-abdication.** A parent can only *ratify* an act taken by staff "in excess of their instructions or authority — including … approval obtained separately" from members. That is what you've been doing after the fact. A council moves the decision into a valid meeting so what reaches you is a *report* you adopt or decline. And the book's fallback for a committee that can't meet is: it may report only what *every* member agreed to. So polling agents separately isn't worthless — it's valid exactly when unanimous. That gives the old practice a lawful floor instead of none.

**What has no meaning anymore.** Rising votes and the "voice → rising → counted" ladder, tellers, and "division of the assembly" — all exist to make an uncounted vote conclusive, and no vote is uncounted. The chair "standing at ease" to consult someone, when the chair is the server. Call of the house. I tried to give each a meaning before writing it off, and wrote down the attempts.

**What the walking-skeleton runs showed.** Their first version had no `amend`. In every meeting the members invented a way around it: voting yes with a dissent note, withdrawing so a substitute could be moved, or unanimously rejecting their own motion so the agreed text could be moved. The book names all three detours and has a rule that would *refuse* the third (a rejected question can't come straight back). Taking a motion away doesn't remove the need; it pushes members into moves the rules block. So the permitted list has a floor, and `amend` is on it.

## Outcomes

- Two charters that load against the model, and a checker that will fail loudly if the model ever renames something they use.
- A chair that is the server for everything mechanical and that hands every judgment call to the council as a vote — which turns out to be exactly what the book tells a chair "in doubt" to do. Cost named: nobody is *charged* with catching a dilatory motion or bad conduct; any member may raise it.
- Nine defects or gaps in the model, two of them real bugs in an existing configuration (the committee-of-the-whole whitelist, read literally, turns off the chair's ability to state a question, and drops the lone member's right to object to unanimous consent).
- A clear statement of the one wall-clock fact in the whole system: the per-round backstop that marks an agent absent, which the charter declares and the record keeps as a fact.

## Tangents worth your attention

- **"Same source = same member."** For a standing council whose seats are re-forked each sitting, I asserted a re-fork of the same frozen expert is the same member. A verifier could argue the opposite (it can't remember changing its mind), and if they win, every sitting is a membership turnover and only referrals survive between sittings — which would be simpler, not worse.
- **Executive session for agents.** Hiding a council's debate from *you* is meaningless; hiding it from *later agents* — out of the search index, unreadable by future forks — might protect later councils from prior-sampling convergence. That's disposition territory you deferred; I only recorded it.
- **Inquorate action.** The book voids action taken without a quorum but is lenient about proving when the quorum was lost. A round system knows the exact round. I guessed the server should apply the void automatically.
- **The reasons are the product.** The book warns that putting a committee's reasoning in its report can cost it votes. For a truth-seeking council that caution runs backwards, and the charter's report instruction says so.

## What is not done

No engine ran; the walkthrough is hand-derived against the model's expressions. The checker tests that a charter is well-formed, not that any particular state is admissible. The recognition tiebreak and the inside of a round belong to the contention-rounds spike. Every "[derived]" label is my own grading; that's what the verifier is for.
