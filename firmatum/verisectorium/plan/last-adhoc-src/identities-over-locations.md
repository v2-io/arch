---
slug: identities-over-locations
type: norm
depends:
  - provenance-rot-specimen
---

# References carry identities, not locations

*A reference stores the stable name of what it points at — never the path, position, or number where that thing currently happens to live.*

## The norm

Every reference in a verisectorium — cross-references between segments, outline rows, provenance pointers, citations from sidecars — names its target by **identity** (the slug, or an identity-anchored address like slug + section + a few words of a span). References do not embed:

- **paths** (directories get reorganized for reasons unrelated to content),
- **positions** ("the third segment of chapter 2" changes whenever exposition improves),
- **presentation numbers** ("Definition 1.4" exists only after a particular view is assembled).

The reason is an asymmetry of change: locations move for housekeeping reasons on a housekeeping clock, while identities change only deliberately, rarely, and visibly. A location-bearing reference breaks **silently at the move** and surfaces later, if ever; an identity-bearing reference either resolves or fails loudly where it's used. Where a reference must reach *inside* a target — a quoted span, a particular clause — the anchor stays identity-shaped (slug + section + span words) rather than becoming a path with a line number.

Decided for this corpus and recommended for the pattern generally. Not truth-apt; overturn expressly if it proves wrong — the supporting observation below is why that seems unlikely.

## Supporting observation (anecdotal)

This week supplied a natural experiment on one corpus: a routine directory reorganization broke **106 of 109** path-anchored quotation references in a single mundane commit — silently, with the corpus's own checker disabled by the same move — while **all 18** identity-based references in the same corpus survived untouched. Full account, counts, and method: [[provenance-rot-specimen]]. One corpus, one move, one checker: an anecdote, not a rate — but a clean one, and the mechanism (a stored location cannot survive a move it doesn't know about) needs no statistics to state.

## Working Notes

- Second specimens are cheap if wanted: any corpus with mixed path/identity references undergoing a move is a free replication (the MOVED/udon relocations are uncounted candidates).
- The instrument wrinkle — the checker disabled by the very move it existed to catch — belongs to ch. 14's instruments-going-blind concern, not to this norm.
- Boundary worth keeping honest: sometimes a path *is* the honest anchor (external trees this corpus doesn't govern, one-off scratch). The norm governs references the corpus owns and expects to survive its own housekeeping.
