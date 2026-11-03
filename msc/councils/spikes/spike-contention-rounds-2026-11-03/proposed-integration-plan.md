# Proposed integration — suggestions for the council server, with reasons (revision 3)

*From the contention spike after `de-novo-feedback-1.md` and `de-novo-feedback-2.md`. Suggestions, with reasons so they can be rejected on the reason. The do-not-inherit list is at the end.*

## Carry into the server's core

1. **`resolve(S, profile) → (S', minutes, claims)` pure, no clock, no mind.** F1 and F3 are then properties of a function, tested by shuffling arrivals *and by relabeling members* (the walking skeleton's `selftest` does the first; the second is `test_f3_relabeling` here).

2. **Policies, not single intents.** Prioritized (guard, act); guards over the round-open state plus structural in-round effects, never sealed contents. An unguarded floor act carries the implicit snapshot guard (top frame, stated-ness, phase, pending points), enforced in code.

3. **Admission order inside a round:** non-floor acts admissible at the round-open state, applied in a fixed kind order with set-valued effects (never re-checked against siblings, never ordered by name) → a point of order halts the floor → one floor act → the chair's duties. The non-floor class comes from the registry's interrupt attribute and its adjudicated urgency condition; the fragility probe is a diff tool against t44–t45 whose disagreements are its own bugs.

4. **The ladder is ONE key per claim:** maker's first speech → least-recently-given-the-floor → a lottery keyed by (committed seed, floor-grant count, seat). No stages, no alternation rung (a declared side is cheap talk; a charter may route side to a judgment predicate and rank on the adjudged side). The second-speech bar [43:13] is an admissibility filter over *admissible speech claims*, and a deferred speech still counts as a claim for debate. Property: independence of losing claims, tested. The seed is committed before a session and revealed after each draw; never derived from round content.

5. **One floor act per round; in-round-triggered floor acts defer.** F2 / 42:6 is the reason.

6. **Every transition that opens a window is followed by a round;** silence counts only in a round after the question was stated.

7. **Close = every asked member finalized, or the deadline. No early close.** Replacement only before a member's own finalization.

8. **Vote rounds are the poll:** they close at the deadline, non-voters recorded, thresholds over votes cast, ballots never in the claim log before announcement, late ballots never counted.

9. **Points of order are breach-specific:** they name the transition, are timely only in the next round, one per member per transition, each ruled in turn; a point landing in a round defers every other non-floor act to after the ruling; a point against the put or a stating may be raised in the poll round and voids the ballots; the announcement is a challengeable transition. Dilatory stays a judgment. Known gap: a stating that lands in a round halted by an unrelated point needs its window keyed to "first round in which its challengers were admissible," not to round arithmetic.

10. **Two record layers:** minutes with the model's justifications; the claim log with every entry's guard and admissibility outcome, "silent" and "late" marks, ballots as "(sealed)."

11. **The presence protocol as the only clock, with its effects declared:** silence never puts a question, never kills an unseconded motion, never rejects an empty poll (it repeats); a vote round closes at its deadline and a member slower than that loses its ballot — the one residue, to be governed by the charter (deadline length; declared absence; extension cap). Consecutive silences → absent → quorum recomputed.

11a. **Sealing is an enforcement requirement of the store and launchers** (property S): no member may read another's same-round response. The walking skeleton found it honor-system today; the server is not fair until it is enforced. Guards must see a claim-free view of the round-open state.

11b. **Duplicate submissions are refused** (first is final) and logged; one ballot per policy.

11c. **Policies persist** until replaced; every member is still sent every state. A persisting policy forgoes the breach window for rounds it is not refreshed in; say so in the member's interface.

12. **Server acts appealable like a chair's; the log can revert a transition.**

13. **Targeted rounds** (maker's modify window; a merge question to two makers; a ruling from a chair-mind) are allowed iff revealed with the round and no member's rights change at a state it was not asked at.

14. **Seat assignment is the charter's,** fixed at seating, never by name or join order (F3's third device).

15. **Record the committee divergence:** keeping the two-speech cap [43:12] and the 43:13 bar in a committee departs from 49:21(3)/50:25; the charter should declare it.

16. **A charter bound by a secret-ballot bylaw [45:20] needs a sealed-unattributed vote method** this design lacks; flag it rather than silently attributing.

## Leave to the charter spike

The chair question; the 9:36 declarations; the maker's modify window as a declared choice; serial vs. parallel debate; a per-member response budget, if any, for the throughput problem.

## Do not inherit

My words, not the material's. Re-derive before reusing.

- **"Fragile."** A mnemonic for a probe with a declared parameter; not a RONR term; not the interrupt class.
- **"Informed act" / I1.** My name for what the serial floor gives silently.
- **"Claim age."** Retracted entirely; if it appears in a doc, it is from rev. 1.
- **"Alternation, strengthened by declared sides."** Retracted in rev. 3; alternation is advisory.
- **"The book names the slot this spike fills" / 9:36 as license.** Retracted in rev. 3.
- **"Vote-changing is vacuous by 45:8."** Retracted in rev. 3; it is dropped and priced (45:50).
- **"One door, whose only effect is did-not-respond."** Retracted; the effects are listed and the ballot residue is declared.
- **"Vacuous."** Means "the purpose dissolved"; say that.
- **The table counts** (23/5/4/1 over 33 rows). A reading aid over rows I chose.
- **Every sentence of the form "RONR's X *is* Y."** Two such sentences failed verification (fragility = interrupt class; claim age = rose first). The remaining ones (the race was never the fairness; 45:8 exempts sealed voting) should be tested, not inherited.
- **The toy's act set, phases, and the "(sealed)" log convention.** The real server reads the model.
- **The lottery seed as a string constant.** A real server records a seed per council and per session; the toy's constant is a stand-in.
