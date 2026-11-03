# Explorations — the diffuse layer (revision 2)

*Hypotheses, dead ends, and what the verifier's pass taught. Results are in `contention.md`; this file is where the thinking is allowed to be wrong, labeled.*

## Hypotheses and what happened to them

- **H1 "Simple rounds suffice."** Half true. Rounds resolve the race. What rounds alone do not give: acts whose window the ordinary flow closes must be admitted ahead of the floor; an intent formed at S must not run at S' without the member's own routing (guards); the close cannot be a message count; near-duplicates and targeted questions need their own handling. Joseph's hunch was right in at least these five ways, and I no longer claim to have found the last of them.
- **H2 "Batch simultaneous motions by rank."** Refuted — but by the right reason only after verification: not that it binds the mover blind (guards could fix that), but that everyone else is never asked at the intermediate state.
- **H3 "Fragility can replace the interrupt list."** Overclaimed in rev. 1; narrowed. It is one of two criteria (the other is urgency, a judgment input), its probe has a declared parameter, and in the toy it orders nothing because all non-floor acts precede the floor anyway. What survives is a diff tool against t44–t45. I wanted this to be the spike's clean derivation, and it is not; the book's "if there may be no other opportunity" [42:7] is still the nicest sentence I found, and it is now carrying less weight.
- **H4 "Division retains a residual function."** Refuted; and rev. 1's "a retake reproduces the same tally" was itself false for agents — a retake would be a reconsider without its protections, informed by the attributed tally. Division is disabled by charter.
- **H5 "Claim age in rounds is 'rose first' in fair time."** Refuted by the verifier on the text (42:6 resets at every yield; a persistent queue is 42:16's line) and on incentives (it rewards always-claiming). Replaced by least-recently-floored plus a seat lottery, which has no seniority to farm. In retrospect this was the design's biggest wrong turn, and it was a *nice-sounding* one, which is the tell.
- **H6 "Mid-speech interruption has no fair equivalent."** Still a priced equivalent (chunking), not a no-go.

## Bugs that were principles, and bugs that were just bugs

- *Principle:* the toy put a motion to a vote in the round it was stated. Silence counts only after being asked.
- *Principle:* two seconds stopped commuting when the chair's "stated" was folded into whichever seconder's event came first. The chair's act is the chair's event; it belongs to nobody's second.
- *Principle:* a point of order stopped looking fragile once its window became "the first round after the transition," because an in-round speech no longer closes it — the round advancing does. "The ordinary course of business" includes time passing with nothing done.
- *Just bugs:* name-ordered loops (three of them), ballots in the claim log, votes waiting for a dead member, an alternation restriction the book does not have. None of these were visible from inside the design; all were visible to a verifier who ran it. That is the estate's role-separation rule paying for itself in one afternoon.

## Things I think are true and did not establish

- The fragility diff over the real model will disagree with t44–t45 only on the conditional entries (privilege, requests, reconsider), and every disagreement will be the diff's own defect. *(guess.)*
- Standing policies will make one-floor-act-per-round cheap in practice; the verifier is right that they trade away the breach window for the members who use them, so the default should be "ask everyone" and a member should have to opt out per round-class. *(guess.)*
- Least-recently-given-the-floor plus a lottery is fairer *for truth* than claim age, not just fairer in form: it hands the floor to whoever has been listening longest. *(guess, and the kind of claim a council could test on itself.)*
- Parallel debate is a different procedure, and a council could use serial debate for the exchange and parallel sealed statements for closing. *(guess.)*

## Things a next mind should look at

1. The real sweep (`contention.md` §10).
2. Whether the maker's first speech should be allowed in the same round as its motion. It would skip the objection-to-consideration and point-of-order windows [26:2(1), 23:5]; I think not, but it costs a round per motion.
3. The revertible log, and what reverting does to policies that were evaluated against the reverted state.
4. Presence → absence → quorum: how many silent rounds, and whether an agent may declare itself temporarily absent.
5. Whether a chair-mind should be asked in the same round as members (sees nothing more) or after (sees the profile, like a human chair, but asymmetrically informed).
6. The verifier's throughput point: every round waits for the slowest member up to the deadline, and the book's remedy for slowness (dilatory) does not reach *slow*. A council may want a per-member response budget as a declared rule; I do not know what is fair there.

## A thought about truth

A round of sealed intents is a sealed ballot on *what to do next*. Within a round nobody's intent is conditioned on anybody else's — stronger independence than the book's floor, where the second riser has heard the first. Across rounds, anchoring returns, and should: deliberation is sequential or it is not deliberation. The mechanism's job is to make the one place independence is possible actually independent, and to keep every claim, admitted or not, in the record so that corroboration can later be told from convergence. The verifier's finding that my first toy leaked a partial tally to a late voter is the exact failure of that job, in the one round type where it matters most.

## After the second verification (revision 3)

- **The property I had not named.** Both of my fairness tests varied order and names and never the *set* of claims. The second verifier's four measured instances (an out-of-order claim closing debate; a losing debater's claim steering the floor to a mover; a misdeclared side winning; a point of order redrawing the lottery) are one failure: a claim that does not win changing the outcome. The fix that made it hold by construction was dull — one key per claim, no stages, no alternation — and the lesson is the same as H5's: every clever rung I added was a channel for steering. The book's ladder is short for a reason.
- **Alternation lost.** I wanted 42:9(3) "strengthened" by declared sides. A declared side is cheap talk; the book's premise is the chair's *knowledge*. There is no agentic knowledge of a member's side short of a judgment. So alternation is advisory unless a charter routes side to the adjudication machine. I think that is right and I still regret it.
- **The clock has a residue, and I had been hiding it from myself.** "One door" was true; "its only effect is did-not-respond" was not. With silence extending everything except the ballot, the residue is exactly one: a vote closes at its deadline. Saying that plainly was worth more than the three rules that removed the other effects.
- **9:36 was a seductive cite.** It let me say the book expected this work. It does not; the book expects it for aural meetings. The charter sibling's "principled departure" is the honest frame, and it costs nothing we needed.
- **The two verifications were asymmetric in a telling way.** The first found bugs in the code (name order, the vote leak). The second found a hole in the *properties*. A third would most likely find something in what the properties do not cover: the enforcement of sealing, the halted-stating window, the latency obstruction channel. I have listed those so the third pass can start there.
