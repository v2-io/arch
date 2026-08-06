---
slug: path-provenance-rots
type: emp
---

# Path-carrying provenance rots; slug identity survives

*If a reference stores a file path, an ordinary directory reorganization silently breaks it; if it stores only a stable name, the same reorganization leaves it untouched. We have watched both happen to the same corpus in the same week.*

## The claim

A reference can carry its target's *name* or its target's *location*. Locations change for reasons that have nothing to do with the content — someone tidies a directory, renames a queue, splits a folder — and every reference that embedded the old location breaks at that moment, **without any error at the time of the move**. The breakage surfaces later, if ever, when something finally tries to follow the reference. Names, held apart from location, pass through the same reorganization unharmed.

This is why the pattern puts identity in a slug and *never* in a path or a position: not as a style preference, but because the two rot at completely different rates. It is also why a checker that verifies references is only as good as its resistance to this failure — a checker whose own lookups embed paths goes red (or worse, silently wrong) at the same moment the references do.

The clean specimen: comproprium's reorganization this week broke **106 of 109** path-anchored quotation references in one commit, while **all 18** slug references in the same corpus survived untouched — and the corpus's own verification tool was disabled by the very move it existed to catch. Full account with counts, cause, and method: [[provenance-rot-specimen]].

Design pressure this exerts: where provenance must locate something *inside* a target (a quoted span, a section), prefer layout-independent anchors — slug + section + a few words of the span — over paths; the estate already uses exactly that address shape successfully elsewhere (vivarium's orientation quiz).

## Strength & grounds

**Empirical, one clear specimen** — the mechanism is near-structural (a stored location cannot survive a move it doesn't know about), but the measured evidence is a single corpus, single reorg, single checker. It supports "paths rot silently under ordinary moves and slugs don't," not "path provenance is always wrong" — sometimes the path *is* the cheapest honest anchor, and the tradeoff is real ( [[provenance-rot-specimen]] Working Notes hold the repair options).

## Working Notes

- Second legs would raise this: any other estate corpus with mixed path/slug references undergoing a move gives a free replication. The MOVED/udon relocations are candidates nobody has counted.
- The checker-disabled-by-the-move wrinkle deserves its own eventual treatment in ch. 14 (instruments that go blind exactly when needed).
