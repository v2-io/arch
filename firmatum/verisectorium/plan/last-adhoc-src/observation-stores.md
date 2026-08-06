---
slug: observation-stores
type: form
depends:
  - observable-crossings
---

# Record the process wide before you know how to weigh it

*Anything about the corpus's own operation that is deterministically knowable at the moment it happens should be written down then, append-only, without waiting for a model that can use it — because the model is a consumer of the record and cannot be built from a record that was never kept.*

## The claim

A corpus that runs for years generates a continuous stream of facts about its own process: which gate a record crossed and with what outcome, how a working note was disposed of, which records change together, what kind of defect was found and where, how long a record sat in a state before moving. Each is cheap to record at the instant it occurs and effectively unrecoverable later — the actor, the criterion, and the context that made the act legible are all gone by the time anyone wants the data.

The instinct that suppresses recording is *"we don't know what to do with this yet."* That instinct has the dependency backwards. A weighting model is downstream of the observations; recording waits on nothing. The discipline is therefore asymmetric and deliberately so: **wide on observation, careful on the model.** Record every deterministically-knowable feature; commit to no interpretation of it. Adding an observation later costs nothing but the missing history; adding the history later is impossible.

**Append-only, per-event, is the form the discipline requires.** A mutable status field answers *what is true now* and destroys *who set it, when, and against what criterion* — which is the entire content of a process observation. Timestamped per-actor event files also make concurrent writers non-colliding by construction, so the store is safe under exactly the parallel work that generates the most events ([[write-safety]], [[partition-isolation]]).

**This is the same machinery as recorded crossings, at a different altitude.** [[observable-crossings]] argues that a layer crossing must be an event so that dispositions are auditable and backlog is countable; this segment argues that the same events, accumulated, are *data* — and that the second use is worth recording for even where the first is already satisfied. The two motives are independent, which is a reason to expect the structure to be durable.

**What it makes possible, stated honestly as potential.** With such a store, claims about a corpus's process that are currently metaphorical become measurable: whether layers genuinely run at different rates ([[layer-speeds]]), whether a gate is firing at all ([[state-flags-not-gates]]), whether retirement backlog is growing or shrinking, whether a confidence procedure is calibrated ([[verbal-label-calibration]] wants exactly this shape of data and reports that nothing here has it). None of these has been measured. The store is the precondition, not the result.

**And the honest counter-evidence: building the store is not the hard part, filling it is.** In the one instance in this estate that shipped a labelled-decision store, the schema and its event classes existed well before the store held anything — it stayed empty until the surface that writes to it shipped, months later. Nothing accumulates from a store's existence; observations accumulate from an *act that writes one*, which is the same moment-versus-stance asymmetry [[role-activation]] describes. A recording discipline with no act attached to it records nothing, and the emptiness is invisible because a store that is empty and a store that is quiet look identical.

## Strength & grounds

**Heuristic, one live instance read first-hand 2026-08-06.** `~/src/arch/firmatum/relata` ships the shape argued for here: an append-only labelled-decision store written by every decision act, alongside append-only verification-event and fetch-attempt stores, with the stated design reason that *"mutable status fields lose that history silently"* and that per-actor timestamped filenames mean concurrent writers never collide. Its README states the intent in the same terms used above — the factor weights are to be refit from the labelled outcome trail, and the automatic tier expands only as the data earns it — and records the counter-evidence directly: the store *"was empty until 2026-07-10"* and began filling the day the confirm surface shipped. The store lives in an external data tree, so its current fill was not inspected here; the emptiness claim is the source's own, dated.

The generalization from one bibliographic store to corpus process metrology is this segment's. One instance, one estate; the list of features worth recording is a considered guess and has not been derived from any question anyone is actually trying to answer, which is both the point of recording wide and the weakest thing about it.

## Working Notes

- Discharges TODO entry R52.
- The uncomfortable version of the claim, worth keeping: *record wide* has no natural stopping rule, and "deterministically knowable" is a large set. Nothing here says what makes a feature not worth recording, and an instance that takes the discipline literally will generate more events than it will ever read.
- Cheapest first landing in this estate: gate and disposition outcomes. Both are already *decided* acts that leave no trace (a working note is resolved, deferred, or promoted and then simply disappears), so emitting them costs one write at a moment that already exists.
- Left to ch. 8: what a consumer of this store actually does with it. Calibration of confidence labels is the named downstream, and the connection is currently an aspiration in both directions — this segment says the data would enable it, and that segment says no such data exists.
