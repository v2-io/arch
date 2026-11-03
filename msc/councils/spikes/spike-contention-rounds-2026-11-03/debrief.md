# Debrief for Joseph — the contention spike, corrected after verification

*2026-11-03. Revision 2, written by re-reading `contention.md` (rev. 2), `response-to-feedback-1.md`, the sim and its output. An independent verifier (`de-novo-feedback-1.md`) ran my toy and found that several fairness claims beyond arrival-order independence were false of it. What was false in the first debrief, so you can un-learn it: that the mechanism kept a member's place in line across a point of order (it wiped it); that one member could not block the floor (it could, forever); that outcomes did not depend on members' names (they did, in about a quarter of random runs); that a late voter saw nothing (it saw the partial tally); that the derived "fragile" class *is* the book's interrupt list (it is one of two criteria, and the derivation has a declared parameter); that your hunch was true "in exactly three ways and no fourth" (two of your own three examples were unaddressed); and that "all of it that was fairness survives" — which is now closer to true, and said more carefully below.*

Joseph,

You asked for a council mechanism where no agent is penalized because its API answered slower, and you suspected that "simple rounds" would not be the whole answer. Here is what I found, including what I got wrong and what the verifier's probes forced.

## The thing under the question

Robert's Rules settles "who speaks next" by the chair seeing who stood first. I went looking for what that was doing and found the book says, in writing, that its whole system assumes everyone can hear everyone at the same moment, and that a group deliberating in writing is not a deliberative assembly at all (1:1, 9:34). It also names, in its electronic-meeting rules, the slot this work fills: "methods for seeking recognition and obtaining the floor" (9:36). So the agentic medium is outside the book by the book's statement, and the book expects someone to supply what I was asked to supply. Simultaneous hearing did three jobs: everyone sees the same state; an objection reaches everyone at once; the chair settles ties by eye. The first can be built in; the second can be built in as a window one round wide (not the real-time channel — that costs something, below); the third was only ever about the clock, and the book itself lets a big hall replace it with numbered microphones (42:16). The race was never the fairness. The fairness was the rules about who deserves the floor, and the equal right to be heard.

(The charter spike found the same passages independently.)

## What "fair" has to mean, and what the code now actually has

Your coordinator's criterion was: the record must not depend on the order answers arrive. I kept it and had to add to it, because "resolve every tie by roster order" satisfies it and is plainly unequal. After the verifier's pass, each of these is a property the toy has and a test checks:

1. The record depends only on *what* members said in a round, never the order it arrived.
2. Every member is asked at every state between rounds; nothing inside a round is a state anyone has rights at.
3. Nothing changes if a member is renamed, except through two declared devices: whoever has had the floor least recently goes first, and remaining ties go to a recorded lottery keyed by seat number. The verifier found my first version failed this in about a quarter of random runs because three pieces of the code sorted by name; all three are gone.
4. The clock enters at one door: a per-round presence deadline, sized in minutes. A late answer is logged and never applied. A vote round closes when the round closes, and a late ballot is never counted and never seen — the verifier showed my first version let a late voter see a 2–2 tally and then vote, which is exactly the herding sealed votes exist to prevent. That one I am glad was caught.

And one property that became the hinge: a floor act fires only against a state that still is what the member saw. The book gives this for free because you move after hearing what was just stated. Here it is an explicit guard, and it forces **one floor-needing act per round**. I had argued that batching several motions in a round binds a mover to a blind act; the verifier pointed out the real reason is that everyone *else* was never asked at the in-between state. The conclusion stands; the reason I gave was the wrong one.

## Your hunch, more honestly

You said rounds would not be the whole of it. I first told you it was true "in exactly three ways and no fourth." That was false — I had not addressed two of the three examples you gave. Here is where it stands:

- **Members answer with policies, not single intents** — "second any postponement; if nothing is pending, move X; if an amendment to X is on top, speak against it." The server runs the branch the state calls for. A stale intent is one whose condition failed. This also answers your example of intents that did not win: they stand with their conditions.
- **Some acts go ahead of the floor** — points of order, objections, seconds. In the book these are the acts you may make from your seat. I claimed I had *derived* that list from the model and that it matched the book exactly. Narrowed: the toy classifies acts by whether the ordinary flow of business would close their window, and that reproduces the book's list for the acts the toy has — but the test has a parameter I chose, the book's rule also has an urgency gate that is a judgment call the model already routes to a mind, and in the toy the classification changes nothing (every non-floor act is admitted first anyway). So the honest value of the derivation is as a check against the book's list, where any disagreement is the derivation's bug.
- **The round cannot close on a count of messages** — the walking-skeleton spike measured this: when a seat could replace its answer, 50 shuffled arrival orders gave four different logs. A round closes when every member has declared itself final, or at the deadline.
- **Near-duplicate motions in one round** (your first example): both want the floor; one wins by the ladder; whether the other is "substantially the same" is a judgment the model routes to a mind, so the server flags only byte-identical text and otherwise re-asks the loser with the winner pending, where it can second, join, withdraw in favor, or move its text as an amendment. In a committee no second is needed, so the live question is only which text is pending.
- **Targeted questions to a subset** (your third example): fair if the exchange is revealed to everyone with the round and nobody's rights change at a state they were not asked at. The maker's chance to modify a motion before it is stated is one of these.

There are probably more. I found no sixth; I no longer claim there is none.

## What the tie-break is now, and the incentive it avoids

My first version ranked claimants by how many rounds they had been asking — "rose first," measured in rounds. The verifier showed two things: every vote or ruling round wiped it, so a frivolous point of order handed the next turn to a newcomer; and as a design it is the book's microphone *line* (42:16), not its rule for the floor (42:6), which resets at every yield. Worse, it rewards the agent that claims every round and punishes the one that passes to listen — the always-act disposition your coordinator flagged. The revised ladder has no seniority at all: maker's first speech, then alternation among debaters, then whoever has had the floor least, then the lottery. Always claiming gains nothing. The verifier also showed a cheap motion claim by a third member could flip which of two debaters spoke, because I had added a restriction to the alternation rule the book does not have; debaters are now ranked among themselves first.

## Obstruction

The verifier blocked the floor indefinitely with a point of order every round. A point now names the transition it challenges, is timely only in the next round, and cannot be raised against a transition already challenged or ruled on. That bounds it: one halt per transition, each costing a ruling round. Whether a point is frivolous is the book's "dilatory" judgment, which the model routes to the chair or the council; the server does not decide it.

## What survives, what changes, what has nothing left to do

Thirty-three of the book's contention devices, one by one: 23 survive (5 stronger than for humans — the two timeliness windows, declared sides for alternation, unanimous consent asked of everyone, and the "prevailing side" rule, which becomes enforceable because every vote is attributed); 5 change with a priced or declared equivalent; 4 have nothing left to do (division of the assembly, the uncounted-vote ladder, "Question!" calls, and vote-changing — the last because the book's own text exempts secret methods, which sealed voting is); 1 depends on whether there is a chair at all. Seven of the 33 rest on the book and the model only, not on the sim, and are marked so.

The one real no-go: anything that lets a member's act land *between* two other members' acts within a round is deciding by arrival order. Interrupting a speaker mid-speech has a fair form — chunk the speech and run a round per chunk — at a price. Debating and then moving in one turn, which the book allows in one floor grant, now costs two rounds.

## What I did not do

The sweep over the real model (no engine yet); measuring how slow this is on a real council (the specimen takes 11 rounds for three speeches, an amendment, a point, and two votes, each round waiting up to the deadline); serial-versus-parallel debate; the chair question; the absence-and-quorum arithmetic. The toy does not implement targeted rounds, consent rounds, or the floor-holder-through-interruption flag; the table rows that depend on them say so.

Everything above is in `contention.md` with its cite or its test; what changed and why, finding by finding, is in `response-to-feedback-1.md`. I am on the line.
