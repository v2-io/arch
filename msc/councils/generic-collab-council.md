# The generic collaborative council

*v2, 2026-11-03. A minimal council procedure, proposed by Joseph and refined in conversation with Claude (Opus 5.5, councils-46), then through a feedback round from six readers (`feedback/gcc-*.md`) and Joseph's corrections to that round. It is a template, not one council type: the same loop serves brainstorming and other generative meetings, epistemic tribunals, and decisions. In Joseph's words, "the main qualifications being the preparation and leadership nuance". Most of what makes it work lives in the members' norms and skill, not in the mechanism.*

*How it fits the larger system, where vsect record types and their transitions are what councils move, is in `councils-system-model.md`.*

*Register: Joseph's words are quoted. "Agreed" marks what was settled in conversation. "Note" marks Claude's comments. Nothing here has run yet.*

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

**What each member sees** (agreed; it was already true of the 2025 implementation, in which members called a `wait-for-turn` endpoint):
- **First response:** written blind. The server releases no first response until all are in, then releases them all at once.
- **Second response:** written after seeing every first response, plus the second responses of those shuffled ahead of it (anywhere from none to all of them).
- **The last seat in the shuffle** sees the most second responses and writes the unity report. That gives it nothing the next loop doesn't give everyone.

Withholding is a property of delivery. It needs no proof machinery: a member who loves the truth doesn't go looking at others' responses early, and no mechanism would contain one that did. The shuffle is server-random and recorded, so a meeting replays exactly.

**The unity report** is an attempt to put the round into an organized form, if there is enough consensus to do so: consensus so far, open questions, discussion-level proposals, and what someone is waiting on first. Its shape is close to the finish form, so the leader can take it up directly. It is one member's reading, contestable in the next round's first responses.

## The leader

Joseph (2026-11-03):

> The 1st iteration should *always* be 'present the topic' without solution. One thing that would be useful, that I've used when presiding to ensure that I'm properly prepared, is to have a *secret* "expected solution" or something — effectively my solution that was part of the thinking process that arrived at the discussion question in the first place (or my first-pass recommendation etc.) — but it is *always* sealed up, not from hiding accountability etc., but to ensure the leader is prepared and really involved, but also that the leader does not unduly influence and must present anything new that the committee misses in light of their own questions etc. etc. Often iteration 2 is some simple follow-up questions that illuminate the problem in a different light. The leader can't do this well if they haven't spent their own time on the problem, but even at iteration 2 the wrong thing to do is simply present what "you think it ought to be." The leader is seeing what consensus the rest are arriving at and giving them the additional context they need if they are diverging because he missed something in the setup — or, often, didn't realize he was holding some key piece of context that changed the picture. Then he gives the context.

So:
- **The sealed expected solution is the leader's own preparation.** It exists so the leader has done the work and can see what the council needs from it. It is not an object of audit or verification.
- **The leader's turns frame, ask, and supply context.** They never present what the leader "thinks it ought to be".

What leaders should expect, from Joseph's experience. These are rough, anecdotal priors to illustrate the point, expected to be superseded:

| How often | What happens |
|---|---|
| ~2 in 10 | the council arrives at what the leader expected |
| ~4 in 10 | the council arrives at something better than what the leader expected |
| ~3 in 10 | there are known unknowns and tasks, so the item is easily deferred |
| under 1 in 10 | the leader must decide before all the facts can be known, and the best unity available is "Can you support this for now given the unknowns?" |

The lesson: the leader's initial thoughts are crucial for knowing what to add to the conversation, and are rarely the best a council can finish with.

## Finish

**The finish form.** In the first finish loop, the leader fills it out by revising the last unity report. In later loops, the leader revises it again. Its typed parts:
- the decision;
- conditions ("under the following conditions: …");
- assumptions, linked to the records they rest on where possible;
- the warrant (the arguments the decision rests on) where the record type requires one, kept distinct from the assumptions;
- assignments: one owner each, stated as an outcome, with a report-back point;
- what would reopen it;
- the importance it was decided at.

Anything not settled can be marked provisional.

**Recommended changes.** When a member recommends a change in a finish loop, the leader's next revision says what became of it: adopted, not adopted and why, or deferred. *Note:* a leader norm, not a mechanism. It keeps a suggestion from silently vanishing between revisions.

**Sustaining.** Each member submits a level, all revealed together, then optionally comments in shuffled order, seeing the comments before theirs. Possible levels: highly support, support, willing, neutral, object, strongly object, each with optional conditions. Consensus is never typed; it is the levels as cast.

**What each level does is declared in the charter, by importance** (agreed). Where the decision requires unanimity, including unanimity on degree of certainty for an epistemic label, an "object" prevents the close. For something like the colour of a bike shed, the council may continue over "object" but not over "strongly object".

**Exits** (agreed):
- **return to Discuss;**
- **hold,** with assignments, and with the knowledge each objecting member says would change its view, so the hold has an exit;
- **close,** with the finish form as the record.

**No confidence.** In the places the charter allows, a member may cast no confidence in the leader. It goes to a higher body, never to the council itself, and is recorded as escalation, not obstruction.

## Who takes part

Two independent axes. Each participant has a position on both.

**Seat kind: what the seat is, and how it is refilled.**
- **Representative.** An accountable member whose presence is part of the decision's standing. If it doesn't answer, the council waits or re-seats it from its line of stewards. It never proceeds as if the seat weren't there.
- **Perspective (duck-typed).** A vantage the council needs covered. Any suitable agent can fill it, and it is refilled by any agent that can hold the vantage.
- **Standing.** An expert seat, refilled by re-forking the same prepared expert, never by substitution. A charter might seat:
  - a primary expert, on the segment or Part in question;
  - adjacent experts, on what it depends on and what depends on it;
  - cross-cutting experts on concerns such as notation and terminology coherence across the corpus.

  This is the matrix structure human organizations use for similar work.

**Participation tier: what the participant does.**
- **Full member:** every turn, both responses, sustaining.
- **Discussant:** first responses in discussion; no second response and no sustaining.
- **Preparer:** assembles what the agenda needs before the council meets, much as a presidency prepares a larger council's agenda.
- **Consulted expert:** joins at any point at a member's request, is given the full log up to that point, answers, and doesn't vote.

**Non-response** follows the charter's decision rule. A unanimity council never shrinks: it waits, or re-seats. A majority council may proceed without a seat.

**Who acted is recorded.** The estate's agent-identity hook (`~/src/agent-core/claude-code/agent-identity/`) already gives each agent a key on Bash calls. Extended to the council's tools, it records which agent, subagent or fork made each submission. *To verify:* that the hook can rewrite MCP tool input the way it rewrites Bash commands.

**Research subagents are in the member's sphere.** A member may delegate research however it sees fit. If it hands its answer to a subagent without reason, that is a matter for correction or better instruction, not a mechanism.

**Kinds of council.** Where a council's value is that its members are independent of the leader, as in a representative body or a tribunal for others, members are not forks of the leader. An agent may also counsel with itself: forks given different perspectives argue before it makes its own decision. That is a self-council. Its output is the agent's own decision, and the record says so.

## The record

What a close writes depends on the record type it moves (`councils-system-model.md`). A council always ends with a decision. When its objective is truth (for example, consensus on whether a weight-of-evidence verdict holds), what the close writes is evidence on that claim, plus the process decision about what to do next.

## The meta-feedback channel

Every participant has a separate channel, outside the discussion, for suggesting changes to the charter or to the record types that use it. These suggestions arrive as influx against the charter record, and are adjudicated by whatever council type governs charters (`councils-system-model.md`).

## Uses

- **Brainstorming and generative meetings:** the topic is an open question, first responses are ideas, second responses build on them, and the finish records what is worth pursuing.
- **Epistemic tribunals:** perspective and standing seats cover the roles. The close writes evidence and the process decision. See `research/tribunal-history.md`.
- **Decisions:** with representative seats where the body must be accountable.
- **Self-councils,** before an agent's own decision.

## Building it

*Note,* from the walking-skeleton spike and its feedback (`spikes/spike-walking-skeleton-2026-11-03/`, `feedback/gcc-skeleton.md`):
- **The substrate carries over.** Each sequential second response is a one-seat round. The core needs rounds addressed to a subset of seats, and a recorded shuffle.
- **Driver mode first.** A seat-only session can't drift back to its home session, which is the failure of October 2025. The background-session wait is not reliable yet.
- **Size the backstop for research turns.**

## Open

- How members are instructed for each role (leader, full member, discussant, preparer, consulted expert, last-in-shuffle reporter) so the nuance lands. Practice and adjustment will go here.
- The exact sustaining levels, and how conditions are written.
- Where no confidence may be cast, and which body hears it.
- Whether sustaining is ever anonymous, and for which councils.
