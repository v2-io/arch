# Proposed integration — suggestions for the council server, with reasons

*From the contention spike, 2026-11-03. These are suggestions; the integrator decides. Each carries its reason so it can be rejected on the reason. A do-not-inherit list is at the end.*

## What I would carry into the server's core

1. **`resolve(S, profile) → (S', minutes, claims)` as a pure function with no clock and no mind.** Reason: F1 is then a property of a function, testable by shuffling (the walking-skeleton's `selftest` already does this), not a property of a process.

2. **Members respond with policies — prioritized (guard, act) lists — not single intents.** Reason: it is the routing Joseph suspected rounds would need, written by the member who knows its own contingencies; it dissolves the staleness problem (a stale act is a failed guard); and standing policies let the server skip asking members whose policy covers the new state, which is what makes one-floor-act-per-round affordable. The guard language can be the model's own expression language, restricted to round-open state plus the structural effects of this round's admitted acts; never this round's sealed contents.

3. **A fixed admission order inside a round:** fragile acts (derived) → commuting non-floor acts (derived) → a point of order halts the floor → exactly one floor act by ladder → the chair's duties. Reason for each is in `contention.md` §4–5; the two derivations should run against `actions.json` with the model's evaluator once stage 5 exists, and until then against a hand list *marked as the stand-in*.

4. **The ladder: maker's first speech → alternation (debaters only) → claim age in rounds → rotation, with 42:9(2) as a filter on the second speech.** Reason: that is what 42:6–13 says once the race is factored out, and the specimen showed what goes wrong when a rung is added the book does not have.

5. **One floor act per round; in-round-triggered floor acts defer to the next round.** Reason: I1 (informed acts) and 42:6 (no claim by rising early). Do not batch floor acts by rank; `contention.md` §5.2 is the construction that shows why.

6. **Every transition that opens a window is followed by a round; silence counts as exhaustion only in a round after the one in which the question was stated.** Reason: the `stated_round` bug.

7. **Close condition = every asked member finalized, or the deadline.** Replacement allowed only before a member's own finalization. Reason: the walking-skeleton measurement (4 distinct logs under replaceable-until-everyone-is-in).

8. **Two record layers:** minutes (admitted acts with the model's justifications; announcements) and the claim log (every submitted entry with guard/admissibility outcome; "silent" and "late" marks). Reason: 42:9(2) and 43:13 need to know who was seeking the floor; the late/silent marks are the only arrival-dependent thing F1 permits and should be visibly quarantined.

9. **The presence protocol as the only clock,** sized in minutes, with consecutive silences → absent → quorum recomputed → the model's inquorate action set. Reason: F4; and it is RONR's own door (a member who left the room), so the model needs nothing new.

10. **The server's mechanical acts are appealable like a chair's, and the log can revert a transition.** Reason: 42:15; a bug in `resolve` must be correctable in-band, not by an operator editing the log.

11. **Votes: sealed, simultaneous, attributed; no vote-change window by default; division, the inconclusive-vote ladder, and tellers disabled in the committee charter.** Reason: `contention.md` §7 rows; the charter sibling's §5 reached the same no-go independently.

## What I would leave to the charter spike

The chair question (server-as-clerk with judgment submitted to the council vs. a chair-mind, and rotation of that role); what a charter must declare (the 9:36 list the charter sibling names); the maker's modify-before-stating window as a declared choice; serial vs. parallel debate as a declared choice.

## Do not inherit

These are my words, not the material's. If they survive into the server's docs they should be re-derived there, not copied.

- **"Fragile"** for the derived interrupt class. The book says "in order when another has the floor" and "if there may be no other opportunity." My word is a mnemonic for the test; it is not a RONR term and should not read as one.
- **"Informed act" / I1.** My name for the property RONR's serial floor gives silently. The book has no name for it.
- **"Claim age."** My translation of "rose first after the floor was yielded" into rounds. The translation is derived; the term is mine.
- **The five-rung ladder as written in the sim** (maker, alternation, claim age, rotation, then act key as a final deterministic tie-break). The last rung — breaking a residual tie by act key — is a toy artifact; a real server should use the rotation pointer or a recorded lottery and never a property of the act.
- **"Vacuous"** as a status word. It means "the purpose dissolved, not the form suppressed." Say that, not the word.
- **The counts in `contention.md` §7** (17 survive / 6 change / 5 vacuous / 1 conditional). They are over the 29 rows I chose; another reader would cut the rows differently. The table is the content; the count is a reading aid.
- **Any sentence in `contention.md` beginning "RONR's X *is* Y."** Several of these are strong readings I believe (the race was never the fairness; fragility is the interrupt class), and a verifier should test them against the paragraphs rather than inherit the confidence.
- **The toy's act set and phases.** Nine acts, four phases. The real server reads the model; it must not inherit the toy's simplifications (no secondary amendments, no series, no privileged-motion yields, no appeal-yield matrix, one point of order at a time).
