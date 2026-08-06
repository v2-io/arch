---
slug: coupling-confounding
type: obs
---

# Coupling confounding (layout vs co-change)

**Summary.** Co-change / coupling statistics used to justify a layout are confounded by the layout that already exists; the estate’s TST-side discussion treats only directional asymmetries as carrying causal weight for layout decisions.

## Observation

From ASF TST measurement theory (read via INFLUX `tst-grounding` and related segments’ claims as reported there; not re-proved here):

- Coupling is estimated from **observed co-change** (`#def-system-coupling` family): \(P(\text{change}(m_j) \mid \text{change}(m_i))\).
- If two parts already share a file (or directory policy), they co-change **because of the container**, not only because of a necessary design dependency. Using that co-change to “prove” they should share a file is circular.
- The generalization note and logical-model letters treat this as **C2-style confounding**: layout explains the measurement that was supposed to explain layout.
- The constructive caution reported alongside: prefer **directional** / asymmetric co-change evidence over symmetric “they change together” when justifying separation or merge.

**Estate application already drawn (not new math):** body vs Working Notes often co-change (high coupling estimate) → co-location is consistent with the measure *and* with the confounder; body vs independent event log co-change less → separate child table is less circularly justified by co-change alone.

## Method

- Warrant: TST coupling/coherence measurement framing as relayed and used in 2026-07 document-architecture discussions (`tst-grounding`, underlying-logical-model).
- This segment does **not** re-run `#meas-coherence-coupling` on the asf tree (precondition of atomic commits is itself often unmet — named in the same literature).

## Strength and scope

- Supports: **naive co-change → layout is not a safe inference**; designers already treat this as a known confounder in this estate’s theory.
- Does not support: a measured coupling matrix for asf or vivarium; a complete causal identification strategy for layout.
- Honest strength: **observation of a theoretical/methodological constraint in use**, not a new empirical study.

## Working Notes

- If someone later runs asymmetric co-change on segment parts under a disciplined commit regime, attach results here or supersede with a measurement obs.
- Related forms: [[atom-as-cluster]], [[write-safety]], layout half of multi-timescale strata.
