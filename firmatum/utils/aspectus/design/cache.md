# Cache — and the purity restatement

Coming back to the same place is cheap. Derived facts (line counts, deep mass, heat, the after-image behind [[last-look|Last look]] and personalized [[quiet-columns|Quiet]]) are keyed and reusable.

## Purity, restated with scope

Part II carries "read-only tools are pure: no writes, `--read-only` is the nature not a flag." A cache makes aspectus a writer, so the nature claim gets its honest scope now, before the row lands:

- **The look never mutates the locus.** Nothing is ever written at or under the path being looked at. That half stays the nature, not a flag.
- **The tool may keep its own memory, elsewhere and documented.** Cache and after-image live under the tool's own state dir (XDG cache/state home), keyed by caller and locus. The influx clause already allowed this ("no implicit cache writes *unless documented*" — this file is the documentation).
- `--no-cache` (ignore, neither read nor write) and `--clear-cache` exist the moment a cache does. A cache failure degrades to a slower look, never to a wrong or absent one.

Two stores, one discipline: the **cache** (recomputable derived facts — losing it costs time only) and the **after-image** (what this caller saw last — losing it costs [[last-look|Last look]] and Quiet personalization their memory, not their honesty: cold state means "no basis for surprise," which renders as the unpersonalized default, never as a guess).

## Freshness / invalidation — open, not designed

How a cached fact knows it is stale is **not decided**. Candidate keys, each with a known hole:

| Candidate | Holes |
|---|---|
| `ino + mtime + size` per file | mtime granularity; mtime forgeable/preserved by some tools; ino reuse |
| content hash | costs the read it was meant to avoid (fine for line-count, wrong for stat-cheap facts) |
| dir mtime for membership | catches add/remove in *that* dir only, not grandchildren |
| git `HEAD` + porcelain-clean | only inside repos; dirty trees fall back |
| whole-locus generation counter (FSEvents / fswatch) | daemon-shaped; not print-and-quit |

Likely shape: per-fact keys (a line count keyed by `ino+mtime+size`; a deep mass keyed by the subtree's dir mtimes, honestly marked `≈` when the walk was bounded) *[2026-10-03, surfaced during the `≈`-retirement slice: this `≈` is a third meaning: neither exact-grouped (now retired, so exact is unmarked) nor the floor `≥` that a bounded walk already earns. A cached value that may have drifted is a staleness claim, which this row still has to spell. `≈` is free again but sits in `~`'s confusables family (design/glyphs.md), so reusing it would revive the confusion just retired. Decision open, Joseph's.]*, never one global validity bit. And "identity of a look" for the after-image (uid? ino+mtime? cache key?) is the same open question the [[aspect-lattice|lattice]] `last-look` row already carries. Decide when the first cached fact lands.

Related law: a census or mass computed under a [[walk-bound|walk bound]] carries `≥` into the cache too — staleness machinery must not launder a bounded count into an exact one.
