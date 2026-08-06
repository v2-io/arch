---
slug: lifecycle-events
type: form
depends:
  - observable-crossings
---

# Tagging, checkpointing, and freeze-and-supersede are first-class lifecycle events

*A living corpus still has moments that deserve a name: a tagged version others can build against, a checkpoint a release was cut from, and — rarest and most consequential — the declaration that a body of material is frozen history, superseded at a named successor.*

## The claim

State flags track where each atom is; lifecycle events mark moments of the *collection*. Three kinds recur, in increasing weight:

- **Tags/versions** — a named, immutable coordinate ("core-v0.3", a submission's manifest at send time) that external consumers can cite and reproduce against. The macro-clock instrument: canon ticks per tag while the working layer ticks continuously ( [[layer-speeds]] ).
- **Checkpoints** — internal snapshots taken *because something is about to happen* (a migration, a big integration, a submission), so before/after is diffable. Cheap under git; the discipline is taking them deliberately and naming why.
- **Freeze-and-supersede** — the heavyweight: an entire outline's worth of material declared no longer alive *here* — "this snapshot is frozen history, superseded at ⟨successor⟩" — with references redirected and the frozen copy retained as archaeology. This is the event [[integration-metabolism]]'s delete-test presupposes at collection scale, the mechanism whole-outline migration (ch. 4 gap) needs, and the estate has performed it repeatedly without a common form: retired name lineages kept literal in frozen archaeology, `.super-archive/` graduation, non-authoritative router files whose own sections redirect, `old-*` prefix conventions.

What makes each an *event* and not a state: it happens once, at a moment, on someone's adjudication, and downstream behavior changes at the boundary — which is why each wants recording with its reasons ( [[observable-crossings]] ), and why a freeze without a recorded successor pointer is how estates get two half-alive twins.

## Strength & grounds

**Heuristic on repeated estate practice, unnamed until now.** The freeze-and-supersede instances above are live and were read first-hand (asf's frozen LOG/`_obs/` conventions and naming-lineage rules; vivarium's `.super-archive/` with manifest; the dissolved router file); versioned-tag practice is universal git culture plus the estate's compliance-group tagging. The synthesis — that these are one family of collection-scale events wanting one recorded form — is this segment's proposal, exercised nowhere as a unified mechanism.

## Working Notes

- The twin-tree hazard is this segment's motivating negative: two copies of the udon tree coexisted mid-move with no frozen/live declaration, and a downstream register cited the stale one. A freeze event with successor pointer is the repair shape.
- Ch. 4's whole-outline-migration gap = freeze-and-supersede + [[integration-metabolism]] at the source + birth at the destination; when that gap is worked, this segment supplies the event vocabulary.
