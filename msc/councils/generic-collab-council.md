# The generic collaborative council

*2026-11-03. A minimal council procedure, proposed by Joseph and refined with Claude (Opus 5.5, councils-46) in conversation. It is a template, not one council type: the same loop serves brainstorming and other generative or creative meetings, epistemic tribunals, and decisions. In Joseph's words, "the main qualifications being the preparation and leadership nuance". Most of what makes it work lives in the members' norms and skill, not in the mechanism.*

*Register: Joseph's words are quoted. Everything else either restates what was agreed in conversation (marked "agreed") or is Claude's note (marked "note"). Nothing here has run yet. It is meant to be built on the walking-skeleton substrate (`spikes/spike-walking-skeleton-2026-11-03/`) and tried on real items.*

## The loop

Joseph's statement of it (2026-11-03), lightly formatted:

> **Round — Discuss:**
> - Leader's turn: present topic or give guidance/steer etc. based on prior round, or skip to "finish".
> - All remaining participants write response — especially questions (take as much time as they need; allowed to send off subagents to do research and come back etc.).
> - Full responses from all other participants shared with the rest of the participants (after they're all done, all at once).
> - All of those same participants shuffled randomly, and then, for each one in shuffled order: allowed to give a *second* / modified response (or pass), which is shared with all of the other agents immediately (or, effectively the same, 2nd responses already given are shown to the agent whose turn it is now to give theirs).
> - Final agent given the responsibility to then submit a report on perceived unity, open questions, etc. etc.
> - Loop.
>
> **Finish:**
> - Leader fills out the consensus, the decision, the assignments, the assumptions, etc. / or adds anything to the record… Leader needs to allow at least one loop of the following, but after that can do the final finish action for that council on that item.
> - Asks for any recommended changes, the reason, or for sustaining a decision / decision to support (or maybe highly support vs support vs willing vs neutral vs object vs strongly-object or something — and possibly with conditions [such as importance] attached).
> - Random-shuffle again with each participant allowed to see the responses already written. (Possibly allow for anonymous?)
> - Loop (finish).

His caveat: "Probably a bit naive but also maybe a bit of overkill in some ways. A lot of the nuance in this case would end up falling to the capabilities and skills and instructions / norms of the agents involved..."

### Why the shape works (note)

- **First responses are sealed and parallel.** Everyone commits before seeing anyone else. The divergence among first responses is the record's evidence that any later convergence was earned. Nobody waits on anybody, so slowness costs nothing.
- **Second responses are sequential, in a recorded random order.** This restores responsiveness. Because the order is a recorded shuffle and never arrival, it is fair and it replays exactly. On the substrate it is a chain of one-seat rounds, each seeing the ones before it.
- **The last seat in the shuffle writes the unity report.** It has seen everything, and because the role rotates at random, no one agent's reading accumulates. The next round's sealed first responses can contest the report.

## The leader

This is where most of the nuance lives. Joseph (2026-11-03):

> The 1st iteration should *always* be 'present the topic' without solution. One thing that would be useful, that I've used when presiding to ensure that I'm properly prepared, is to have a *secret* "expected solution" or something — effectively my solution that was part of the thinking process that arrived at the discussion question in the first place (or my first-pass recommendation etc.) — but it is *always* sealed up, not from hiding accountability etc., but to ensure the leader is prepared and really involved, but also that the leader does not unduly influence and must present anything new that the committee misses in light of their own questions etc. etc. Often iteration 2 is some simple follow-up questions that illuminate the problem in a different light. The leader can't do this well if they haven't spent their own time on the problem, but even at iteration 2 the wrong thing to do is simply present what "you think it ought to be." The leader is seeing what consensus the rest are arriving at and giving them the additional context they need if they are diverging because he missed something in the setup — or, often, didn't realize he was holding some key piece of context that changed the picture. Then he gives the context.

So, as agreed:
- **The leader's sealed expected solution** is written before round 1 and stays sealed. It is never revealed to steer the council. It exists so that the leader has done the work, and so that, after the decision, the record shows how the council's result compared with it.
- **The leader's turns frame, ask, and supply context.** They do not opine. Round 1 presents the topic without a solution. Later turns ask follow-up questions, and give context the leader finds the council lacks, especially context the leader didn't realize it held.

**What leaders should expect, from Joseph's experience:**

| How often | What happens |
|---|---|
| ~2 in 10 | the council arrives at what the leader expected, and what the leader would have done alone |
| ~4 in 10 | the council arrives at something that supersedes the leader's expectation and is better |
| ~3 in 10 | there are known unknowns and tasks, so the item is easy to defer until later |
| under 1 in 10 | the leader must decide before all the facts can be laid out or known, and the best unity available is "Can you support this for now given the unknowns?" |

*Note:* the sealed expected solution makes these rates measurable for agent councils. Comparing it with each outcome after the fact sorts every item into one of these rows. A leader whose councils keep landing in the first row may be steering without meaning to.

## Sustaining

As agreed, with Joseph's assent that it may need practice and adjustment ("I could support this assuming the agents are well-enough-instructed"):

- **The level of support is sealed, then revealed all at once.** Levels might be highly support, support, willing, neutral, object, strongly object, each with optional conditions such as importance. Sealing applies here, where herding would do the most damage: later seats otherwise calibrate their support to what is already written, and the record then shows a unanimity that wasn't independent. Sealed levels also make anonymity possible if it is ever wanted, since levels can be revealed without names.
- **Then comes an optional sequential round, in shuffled order,** for comments and recommended changes, each seat seeing the ones before it.

## Exits

Agreed ("yes, pretty much"). Finish ends in one of:
- **return to Discuss**;
- **hold**, with assignments, and the knowledge each dissenting seat says would change its view, so that the hold has an exit;
- **close**, with the decision record.

## Seats, and what happens when a member doesn't answer

Joseph:

> Sometimes the council decision is because the council as a body is accountable and therefore there has to be representation. Sometimes it's to round out the perspectives tribunal style in which case it's more of a duck-typed seat than an individual continuous entity seat...

So the charter declares, per seat, which it is:
- **Representative seat:** an accountable member whose presence is part of the decision's standing. If it doesn't answer, the council waits, or re-seats it from its line, the walking skeleton's "line of stewards". It never proceeds as if the seat weren't there.
- **Perspective seat (duck-typed):** a vantage the council needs covered. Any suitable agent can fill it. If it doesn't answer, the seat is refilled; the vantage matters, not the instance.

The charter also states its decision rule, which sets how non-response is handled. Under unanimity or hold, the council never shrinks. Under majority, it may proceed without a seat.

## What a council produces

- **The decision record**, written by the leader at finish: the consensus or decision, the assumptions it rests on, the assignments (one owner each, stated as an outcome, with a report-back point), what would reopen it, and the importance it was decided at.
- **The sealed first responses, second responses, unity reports, and sustaining levels**, all from the log.
- **The leader's sealed expected solution**, unsealed into the record after close.

*Note:* importance stated at decision time is what lets a decision show as under-secured later, if it becomes more load-bearing than it was when decided (from the same day's discussion of decision importance).

## Uses

- **Brainstorming and generative meetings:** the leader's topic is an open question, first responses are ideas, second responses build on them, and finish records what is worth pursuing, with assignments.
- **Epistemic tribunals:** perspective seats hold the roles (an advocate, someone looking for what was missed, an observer of shared bias, a risk analyst). Finish records the decision apart from its confidence, the load-bearing arguments, and the revisit conditions. See `research/tribunal-history.md`.
- **Decisions:** as above, with representative seats where the body must be accountable.

## Open

- How members are instructed for each role (leader, seat, last-in-shuffle reporter) so that the nuance actually lands. This is where practice and adjustment will go.
- The sustaining scale's exact levels, and how conditions such as importance are written.
- Whether anonymity of sustaining levels is ever wanted, and for which councils.
- Wall-clock cost: the sequential phase costs the sum of members' turns. Passing keeps it cheap when there is little to add.
