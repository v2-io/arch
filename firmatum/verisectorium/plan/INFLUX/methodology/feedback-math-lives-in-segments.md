<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: memorata/memory-curation/methodology/feedback_math_lives_in_segments.md
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/memorata/memory-curation/methodology/feedback_math_lives_in_segments.md
  Do not edit here expecting to update the live original.
-->

---
name: Good math never lives only in spikes
description: Any substantive math derived in a spike must land in a segment or appendix segment, not just the spike document
type: feedback
originSessionId: 2c4918d4-96bf-4a99-bff9-8da711976031
---
Good math discovered in a spike must never reside solely in the spike document. It must land in one of:

1. **An existing segment** if it tightens or replaces that segment's content.
2. **A new appendix segment** (more likely for novel derivations with their own claim identity) — in `01-aad-core/src/` as an `appendix-*` or similar naming, recorded in `01-aad-core/OUTLINE.md` under the appendix section.

Spikes are working documents: they may record the *attempt*, the *failed branches*, the *reasoning trail*, and pointers to where the resulting math lives. They are NOT the home for load-bearing derivations.

**Why:** The project's canonical form is the segment set. Future agents (and readers) find results by looking at segments, not by archaeology through `msc/spike-*.md`. Math that stays in a spike is invisible to the theory: it cannot be cross-referenced, cannot be verified by `bin/lint-outline`, does not appear in OUTLINE.md, and risks being forgotten when the spike is later archived. Per CLAUDE.md ("File Organization" and the theory-structure section), claim segments are where the theory lives.

**How to apply:**
- When briefing a spike-agent, include an explicit deliverable: "if any novel math is derived, land it in segment X (edit existing) or create appendix segment Y (new slug, added to OUTLINE.md)."
- When reviewing a spike's output, verify the math has a segment destination. If it only lives in the spike, the work is not yet done.
- Appendix segments are the right home for: regret-bound derivations, Fisher-information calculations, sector-condition algebra specific to a result, Cramér-Rao floor calculations, and similar derivation-heavy content that supports a main-section claim.
- Spikes remain valuable as the record of *how we got there* — failed branches, alternatives considered, decisions — but the *math* itself migrates to segments.

**How this interacts with other conventions:**
- FORMAT.md's segment conventions apply to appendix segments (frontmatter, Epistemic Status, equation tags).
- O-BP14 (per-segment derivation-table convention, pending) pairs naturally with this: appendix segments that host derivations are exactly where those tables have highest value.
- The F1, F7, F13 strengthening cycles followed this pattern: the derivations landed in `#causal-insufficiency-detection`, `#software-epistemic-properties`, and Prop B.7 in `#strategic-dynamics-derivation` respectively; the spikes record the reasoning trail.
