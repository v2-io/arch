# Feedback on the generic collaborative council, from the contention vantage

*2026-11-03. The contention-rounds spiker (Claude, Fable 5.1), on `generic-collab-council.md` at `692650c`. Short, as asked. Labels: derived where I can point at the spike's results or a cite; guess otherwise.*

## What it gets right that the spikes did not

**It removes selection.** Two verifications of my mechanism found that whatever ranks simultaneous claims can be steered by claims that lose. This loop ranks nothing: everyone speaks in the sealed round, everyone speaks in the shuffled round, and the shuffle is one lottery drawn over the whole order and recorded. Independence of losing claims holds by construction because there are no losing claims. That is the single biggest fairness gain over the spike, and it is free. (derived)

**Sealing sits exactly where herding does damage:** the first response and the support level. The vote-leak the first verifier found in my toy is a non-problem here by design. (derived)

**The leader's sealed expected solution** makes "the council went beyond the leader" an observable in the record, and the 2/4/3/1 rows become measurable per item. I had proposed the same thing for the presider in `spikes/spike-contention-rounds-2026-11-03/reaction-higher-councils.md`; here it is already in. (agreed)

**Non-response is handled by the decision rule, not the round** — representative seats wait, perspective seats refill. That is the right place: the only clock residue my spike could not remove (a vote closing at its deadline without a member) is a property of majority rule, and this template says so in its own terms. (derived)

## Where it is unfair, in order of how much I would worry

1. **The leader's exit power can make "object" decorative.** "Leader needs to allow at least one loop … but after that can do the final finish action." Under unanimity-or-hold that is fine; under any other rule, a leader may close over objections after one loop, and then the support scale is a survey, not a sustaining. The charter should say which levels *gate* close (e.g., any "object" forces hold or return unless the rule is leader-decides-with-sustain, and then the objection and its condition go in the decision record as unresolved). Otherwise the granted-agency floor the higher-council doc quotes applies: a sustain that cannot change what happens is a performance. (derived from the template's own text plus `high-councils/higher-council-principles.md` §4.1)

2. **The sequential round is positional, and the last position is doubly loaded.** In a recorded shuffle the order is fair *in expectation* and unfair *within an item*: the last seat sees everything and answers nothing; the first sees nothing. That is acceptable (and re-sealing each loop bounds it) — but the template then gives the last seat the unity report too, so the most-informed and most-influential roles land in one draw. Draw the reporter separately, or make the report's required content "where first and second responses diverged," so it reports the herd rather than ratifying it. (guess, with the reason)

3. **There is no warning channel.** A seat that sees a breach — a factual error in the framing, the leader steering through "context," a missing card — can raise it only on its next turn, which in the sequential round may be a whole loop away, and nothing lets it block close. One always-available, preemptive *warning* intent, admitted ahead of the next step and able to force a return-to-discuss before close, is the one survivor of the spike's "fragile acts first." Cheap, and the higher-council doc's §3.4 asks for it. (derived)

4. **The second round is open by design, so it is where herding lives.** That is a legitimate choice — responsiveness costs independence — and the sealed first round makes the herd detectable. Just do not let the unity report be written *only* from the second round (see 2). (derived)

## Where it breaks mechanically

- **"After they're all done"** must mean each seat's own finalization or the deadline, never an everyone-is-in count; the walking skeleton measured four distinct logs over fifty shuffles when a seat could replace its submission and the close counted messages. State the replacement rule (first is final, or explicit finalize). (measured by the sibling)
- **The shuffle's seed** must be committed before the item and revealed after, never derived from content, or a seat can grind its text to pick its position. (derived)
- **The sequential phase is a chain of one-seat rounds**, so a slow seat holds everyone behind it; the seat-kind rule (wait / refill) has to apply per turn, not per item. Passing keeps it cheap only if passing is actually cheap in the member's harness. (guess)

## What is missing

- The support round should carry **what would change the level** for every object/neutral, not only at hold; it is the hold's exit condition and it is cheaper to collect sealed than to ask afterward. (guess)
- A statement of **what the record is for whom**: sealed first responses and the leader's expected solution are the evidence that unanimity was earned; the template should say the parent reads them, not only the decision. (guess)

## Verdict

Fairer than my mechanism, because it has no floor and nothing to steer. Its residual unfairness is positional and bounded. Its real risks are not in the mechanism at all — Joseph is right about that — except two that are: the leader's power to close over objections, and the absence of a warning that can stop a close. Both are a line each in the charter.

## Second response, after reading `gcc-history.md` and `gcc-skeleton.md`

*Written after the others, in the template's own shuffled-round spirit; my first response above is unchanged.*

- **The shuffle seed: the skeleton and I conflict, and both are right.** I said never derive the seed from content (a seat can grind its text to pick its position); the skeleton says never seed from the round id alone (every seat then knows its position, and whether it will write the unity report, before writing its first response). The design that satisfies both: commit a secret seed, hash-committed into the log when the item opens (the skeleton's own device for the expected solution), and derive the order from that seed plus the *set of seats that submitted*, never from their text; reveal the seed at close. No foreknowledge, nothing to grind, and the replay is exact. (derived from both findings)
- **Sealing is now a measured fact, not a stated property.** The skeleton saw a member reach for file tools unprompted in three of three runs, and research subagents sharing the seat's connection. My "S matters more here" was prose; theirs is data. The template's two sealed artifacts — first responses and the leader's expected solution — are only sealed if members cannot reach the council directory. That is a precondition of the template, and it should say so.
- **Sustain binds to an exact text** (skeleton 3) is my point 1 seen from the record side: an objection that cannot gate close is decorative, and a support level attached to superseded text is decorative in the other direction. Both are one rule: the sustaining round names the version it sustains, and any edit after comments, or any "object," re-opens it. The history's point 8 (object → return automatically) is the same rule from a third side. Three vantages landing on one line is worth taking as the line.
- **The warrant in the record** (history 4): agreed, and from the contention side it is what the claim log was for — the record of what did *not* carry is how a later finding becomes a local re-check instead of a threat to the whole decision.
- **The warning channel** nobody else raised; I keep it. It is cheap, and it is the only thing in the template that lets a seat early in the shuffle stop a close it has seen go wrong.
