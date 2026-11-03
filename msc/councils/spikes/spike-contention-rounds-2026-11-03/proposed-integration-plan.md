# Proposed integration — suggestions for the council server, with reasons (revision 2)

*From the contention spike after `de-novo-feedback-1.md`. Suggestions, with reasons so they can be rejected on the reason. The do-not-inherit list is at the end.*

## Carry into the server's core

1. **`resolve(S, profile) → (S', minutes, claims)` pure, no clock, no mind.** F1 and F3 are then properties of a function, tested by shuffling arrivals *and by relabeling members* (the walking skeleton's `selftest` does the first; the second is `test_f3_relabeling` here).

2. **Policies, not single intents.** Prioritized (guard, act); guards over the round-open state plus structural in-round effects, never sealed contents. An unguarded floor act carries the implicit snapshot guard (top frame, stated-ness, phase, pending points), enforced in code.

3. **Admission order inside a round:** non-floor acts admissible at the round-open state, applied in a fixed kind order with set-valued effects (never re-checked against siblings, never ordered by name) → a point of order halts the floor → one floor act → the chair's duties. The non-floor class comes from the registry's interrupt attribute and its adjudicated urgency condition; the fragility probe is a diff tool against t44–t45 whose disagreements are its own bugs.

4. **The ladder:** maker's first speech → alternation among debaters by declared side → least-recently-given-the-floor → a lottery keyed by (council seed, round, seat). Debaters ranked among themselves first, then the top debater against the top mover by the neutral key. The second-speech bar [43:13] is an admissibility filter against the round-open profile, never a rung. **No persistent claim queue.**

5. **One floor act per round; in-round-triggered floor acts defer.** F2 / 42:6 is the reason.

6. **Every transition that opens a window is followed by a round;** silence counts only in a round after the question was stated.

7. **Close = every asked member finalized, or the deadline. No early close.** Replacement only before a member's own finalization.

8. **Vote rounds are the poll:** they close at the deadline, non-voters recorded, thresholds over votes cast, ballots never in the claim log before announcement, late ballots never counted.

9. **Points of order are breach-specific:** they name the transition, are timely only in the next round, and cannot be re-raised against a ruled transition. Dilatory stays a judgment, routed to chair-mind or council.

10. **Two record layers:** minutes with the model's justifications; the claim log with every entry's guard and admissibility outcome, "silent" and "late" marks, ballots as "(sealed)."

11. **The presence protocol as the only clock,** with consecutive silences → absent → quorum recomputed → the model's inquorate action set.

12. **Server acts appealable like a chair's; the log can revert a transition.**

13. **Targeted rounds** (maker's modify window; a merge question to two makers; a ruling from a chair-mind) are allowed iff revealed with the round and no member's rights change at a state it was not asked at.

14. **Default "ask everyone";** a standing policy is a per-member opt-out that gives up that member's breach window and is revealed as such.

## Leave to the charter spike

The chair question; the 9:36 declarations; the maker's modify window as a declared choice; serial vs. parallel debate; a per-member response budget, if any, for the throughput problem.

## Do not inherit

My words, not the material's. Re-derive before reusing.

- **"Fragile."** A mnemonic for a probe with a declared parameter; not a RONR term; not the interrupt class.
- **"Informed act" / I1.** My name for what the serial floor gives silently.
- **"Claim age."** Retracted entirely; if it appears in a doc, it is from rev. 1.
- **"Vacuous."** Means "the purpose dissolved"; say that.
- **The table counts** (23/5/4/1 over 33 rows). A reading aid over rows I chose.
- **Every sentence of the form "RONR's X *is* Y."** Two such sentences failed verification (fragility = interrupt class; claim age = rose first). The remaining ones (the race was never the fairness; 45:8 exempts sealed voting) should be tested, not inherited.
- **The toy's act set, phases, and the "(sealed)" log convention.** The real server reads the model.
- **The lottery seed as a string constant.** A real server records a seed per council and per session; the toy's constant is a stand-in.
