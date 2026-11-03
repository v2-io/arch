# Debrief for Joseph — the contention spike

*2026-11-03. Written after re-reading `contention.md`, the sim, and its output, not from memory of doing the work. Nobody has verified any of this yet.*

Joseph,

You asked for a council mechanism where no agent is penalized because its API answered slower, and you suspected that "simple rounds" would not be the whole answer. Here is what I found, in the order the truth came out rather than the order I planned.

## The thing under the question

The first thing that moved was the question itself. Robert's Rules settles "who speaks next" by the chair seeing who stood up first. I went looking for what that *standing up first* was really doing, and found that the book is explicit that its whole system assumes everyone can hear everyone at the same moment. It says, in so many words, that a group trying to deliberate in writing is not a deliberative assembly at all and that its rules do not apply there (1:1, 9:34). So the agentic medium — each member answering privately, at its own pace — is outside the book by the book's own statement. That turned out to be good news, because it says exactly what has to be rebuilt. Simultaneous hearing did three jobs: everyone sees the same state at the same time; an objection reaches everyone the instant it is spoken; and the chair settles ties by eye. The first two can be built in. The third is the only part that was ever about the clock, and the book itself says a big hall may replace it with numbered microphones and lights (42:16–17). The race was never the fairness. The fairness was the rules about who deserves the floor — the maker speaks first, everyone speaks once before anyone speaks twice, sides alternate — and the equal right to be heard. Those survive untouched.

(The charter spike, working separately, found the same passages and the same conclusion, with cites I had not reached. I take that as a sign the foundation is the book's and not mine.)

## What "fair" has to mean

Your coordinator's criterion was: the record must be the same no matter the order in which answers arrive. I kept it and had to add to it, because "resolve every tie in roster order" also satisfies it and is plainly unfair. Four parts, each checked in the toy or derived with the cite:

1. The record depends only on *what* members said in a round, never on the order their answers came in. (Checked over 200 random states and 18 arrival orders each: identical every time.)
2. Every present member is asked at every state. Nothing can go out of order for a member before that member has had its chance. This is stronger than the book gives humans — a slow riser misses the window; a slow agent does not.
3. When the book's preference rules do not decide, the tie goes to who has been asking longest *measured in rounds*, then to a rotation — never to arrival.
4. The clock enters at exactly one door: a presence deadline, sized in minutes, whose only effect is "this member did not answer this round." That is the same fact the book records when someone leaves the room. A late answer is logged, never silently thrown away.

And one property that is not about the clock but became the hinge: every act must be made against a state that already includes everything admitted before it. The book gives this for free, because you make your motion after hearing what was just stated. A round mechanism keeps it only by admitting **one floor-needing act per round**. I tried the obvious shortcut — admit several motions in one round, ordered by their rank — and showed it binds a member to a blind act: a member who moved to postpone would have chosen differently had it seen the amendment that landed first. So one floor act per round is forced, not chosen, and the price is paid in polling.

## Your hunch, made specific

You said rounds would not be the whole of it: "what arrives from members in one step may call for routing that depends on what arrived." That is true in exactly three ways, and I could find no fourth:

- **Members answer with policies, not single intents.** A member says "second any postponement; if nothing is pending, move X; if an amendment to X is on top, speak against it," each with its condition. The server runs the right branch for the state that obtains. A stale intent is just one whose condition failed; it never runs. This also answers the "what happens to the intents that didn't win" question: they stay standing with their conditions, and the member is only re-asked when no condition covers the new state — which also makes the one-act-per-round rule affordable, since most members most rounds need not be asked.
- **Some acts must go ahead of the floor.** A point of order, an objection to consideration, a second, a division demand — each has a window that the ordinary flow of business closes. If they had to win the floor they would be unfairly lost. The interesting result: I did not list these. I *derived* them in the toy by asking, for each act, "if someone else just speaks, or the chair just proceeds, is this act still in order?" The set that comes out is the book's own list of acts that may be made without the floor. And the book says the test in English at the one place it makes interruption conditional: notice and reconsider may interrupt "if there may be no other opportunity" (42:7, 42:19). The server can compute this class from the model instead of curating it, so it cannot rot when an action is added. (The derivation ran on a nine-act toy; running it over the real model is the first thing a verifier should want, and I could not do it because the model's engine does not exist yet.)
- **The round cannot close on a count of messages.** The walking-skeleton spike measured this on a running server: when a seat could replace its answer, 50 shuffled arrival orders produced four different logs, because a replacement races the "everyone is in" close. The rule that follows is that a round closes when every member has *declared itself final*, or the deadline passes — never when a count is reached.

## What survives, what changes, what has nothing left to do

I took 29 of the book's contention devices one by one. Seventeen survive (six of them stronger than for humans — the timeliness windows, unanimous consent, and the "prevailing side" rule for reconsideration, which becomes enforceable for the first time because every vote is attributed). Six change, each with a priced or adoptable equivalent. Five have nothing left to do because their purpose dissolved: division of the assembly, the voice-then-rising-then-counted ladder, the chair's "seem to have it" pause, calling out "Question!", and the inconclusive vote — every agentic vote is already exact, attributed, and sealed. One (the chair speaking first on an appeal) depends on whether the council has a chair-mind at all.

None landed as "no fair equivalent." I expected two to. Interrupting a speaker mid-speech has a fair equivalent — chunk the speech and run a round between chunks — it just costs a round per chunk, so the default should be whole speeches with the window after. And the race has a fairer equivalent, not a worse one. The one real no-go, stated once: anything that lets a member's act land *between* two other members' acts within a round is deciding something by arrival order. The round boundary is the only fair place to sequence, and everything that "changed" changed because of that one fact.

## Where I was wrong along the way, since those were the useful moments

- My first ladder let an unspoken debater beat a member trying to move an amendment. The book ranks debaters among debaters and says nothing about a debater versus a mover — that is the race, now claim age. The toy's own specimen exposed it.
- My first toy put a motion to a vote in the same round it was stated, because nobody had asked to debate — but nobody could have; they had not yet been asked at a state where it was pending. The fix is a principle: silence counts only after being asked.
- My first "fragility" test was too wide and made *amend* look like an interrupt because a previous question would preclude it. Being outranked is contention, not a timeliness window; narrowing the test to "the ordinary course of business" is what made it match the book.

## What I did not do

The real sweep over `actions.json`; the serial-versus-parallel debate question (both are fair; they are different procedures, and which serves truth is the disposition question you set aside); the chair question (the charter spike's); measuring how slow one-act-per-round actually is on a real council; and the exact arithmetic of presence and quorum beyond saying it is the book's own.

## Where councils-46's drafts were narrower, as asked

The draft's tie-break dropped the book's preference rules entirely and should put them above claim age; conditional intents were left open out of a staleness worry that conditions themselves resolve; the interrupt class was curated where it can be derived; division and the vote ladder were not addressed; "everyone passes means move on" needs the after-being-asked guard; and a point of order should halt the floor without costing the losing claimant its place in line.

Everything above is in `contention.md` with its cite or its test, and the toy is in `sim/`. I am on the line.
