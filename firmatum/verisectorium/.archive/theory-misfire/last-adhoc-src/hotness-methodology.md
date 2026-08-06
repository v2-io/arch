---
slug: hotness-methodology
type: form
depends: []
---

# Importance is computed from several weak signals, and its consumers make it consequential

*No single observable says which records matter now; a defensible ranking combines several that fail differently, publishes why each record scored as it did, and is honest that its weights are a first guess.*

## The claim

Several things a corpus needs are the same question wearing different clothes: which records a new mind should read, which a study order should foreground, which deserve attention this cycle. All of them ask *what is important now* — a property no record can declare about itself, because importance is relational and moves.

**Compute it, from signals with independent failure modes.** The available observables each measure something real and each is defeasible alone: recency of change; how often a record's file has been touched in a recent window of history; how often it is *mentioned* in that history; how many other records transitively depend on it; whether it carries live working residue; and a link-graph centrality over soft mentions. Structural signals (dependents, centrality) are stable and blind to what is happening now; activity signals (churn, mentions, recency) are current and blind to what is foundational. Neither family alone produces a usable list, which is the argument for combining rather than picking.

**Normalize before combining, and choose the combination deliberately.** Raw factors are incommensurable — a timestamp and a dependent-count share no scale — so each is mapped to a common range by rank rather than by value. The aggregation then encodes a real judgment: a *geometric* mean makes the weakest axis dominate, so a record must be at least somewhat interesting on every axis to rank high; an arithmetic mean lets one spectacular axis carry a record that is dead on all the others. Bottleneck behavior is the better default for a reading list, where a record that is merely fashionable is exactly what wastes a newcomer's budget.

**Priors ride outside the aggregate.** A per-type multiplier — definitions and scope statements up, sketches and discussion down — belongs *after* the combination, not as a factor inside it, so that a type prior cannot be traded against evidence. That separation is what makes the prior arguable rather than buried.

**Recency gets a fiat override, and the honesty is in naming it as one.** A ranking that is purely a smooth function of the factors will bury a record that changed an hour ago beneath structurally central ones, which is wrong for the actual consumer. Pinning a small set of just-touched records to the top is a defensible override; presenting it as an emergent property of the scoring is not. It is fiat, it should be labelled fiat, and its window should be a stated knob.

**Two consumers with different needs, one ranking.** A human or agent reading a hotlist wants an order; a sampler choosing what to check wants a *distribution*, with a temperature that trades peakiness against coverage. One score serves both, but only if the same order is visible in each — otherwise the free-read set and the tested set diverge, and the check stops measuring the reading.

**The consequential part: the ranking is not only a view.** Where an orientation curriculum is drawn from it ([[orientation-scaffold]]), the ranking *is* what every incoming mind learns first, and a change to importance silently changes onboarding. That coupling is desirable and should be deliberate; it also means the ranking's errors are inherited by every new session, which raises the standard for publishing the per-record explanation rather than only the score.

**Say that the weights are a guess.** Equal weighting over normalized factors is a starting point, defensible because it is neutral and because moving with it beats waiting for evidence that only running it can produce. What makes it honest is the standing statement that it is imperfect and rebalances when data arrives — and the data that would rebalance it is exactly the kind [[observation-stores]] describes recording before anyone knows how to weigh it.

## Strength & grounds

**Heuristic, from one live implementation read first-hand 2026-08-06.** Every design move above is running in `~/src/arch/vivarium/bin/orient-rank` and documented in its own module docstring: six rank-CDF-normalized factors (freshness, path churn and slug mentions over a rolling commit window, transitive dependents, working-surface residue, mention-graph centrality), geometric-mean aggregation chosen explicitly for bottleneck behavior, a type prior applied after the mean, a labelled tip fiat pinning recently-touched records to the top ranks, and a softmax temperature exported for the sampler. Its own stated posture — *"deliberately imperfect… good enough to move with confidence; rebalance when we have evidence"* — is quoted rather than softened. The consumer coupling is that instance's own arrangement, stated in `ORIENT.md`.

One implementation, one estate, unvalidated: **nothing has measured whether this ranking picks better records than any other ordering, including alphabetical.** The factor set is a considered guess, the weights are explicitly a first pass, and no outcome data has yet been fed back. What the segment establishes is the *shape* a defensible ranking takes and which choices inside it are load-bearing — not that this ranking is right.

## Working Notes

- TODO entry N11 (the ranking-is-also-the-curriculum coupling) is discharged by [[orientation-scaffold]], not here; this segment restates the coupling from the producing side. The still-owed half of N11 is ch. 2's: that the ranking is simultaneously view-local metadata, an attribute of the membership edge per [[view-edge-metadata]].
- The cheapest validation available and unrun: the sampler already records which records were checked and how they scored. That is labelled outcome data against the ranking's own predictions, sitting unused.
- Genuinely open: whether "importance" is one quantity at all. A record important to *read first* and a record important to *get right* may not rank together, and one score serving both consumers assumes they do.
- Left for the ch. 2 half: nothing here says where the score lives — recomputed on demand, cached, or written back into the view as annotations. This instance does the last (stars marked into the outline), which makes the ranking a derived artifact inside an authored file, exactly the hazard [[derived-vs-authored]] describes.
