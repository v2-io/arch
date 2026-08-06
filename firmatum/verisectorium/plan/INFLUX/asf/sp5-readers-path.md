<!--
  Verisectorium notes gather — extract, not full-file authority.
  Provenance: arch/asf/_obs/architectural-proposals-2026-04-22.md § SP-5 (lines 842–858)
  Copied: 2026-08-05
  Later reclassified under PROPOSALS §H.5 as a filterable segment layer.
  Do not edit here expecting to update the live original.
-->

# Extract: SP-5 — Two-tier "Reader's Path" presentation

*From the 2026-04-22 architectural proposals portfolio (now `_obs/`).
PROPOSALS §H.5 later reopens this as one filterable layer under outline-as-view.*

---

### SP-5 — Two-tier "Reader's Path" presentation

**Source:** Opus Big Picture §4 (2026-04-23 audit).

**Thesis.** AAD's honesty discipline (equation tags, scope conditions, epistemic-status, derivation-audit tables) produces heavy reading load. A reader trying to *understand the framework's shape* must process substantial qualification before reaching load-bearing content. Proposal: each segment carries a 1–2 sentence "Reader's Path" preamble that states load-bearing content without qualification, with the formal apparatus following.

This does not replace the honesty discipline — only adds an entry ramp. The cost is mild redundancy; the benefit is teachability.

**Merits.** Approachability (very high); concision (low — adds redundancy); correctness (neutral — doesn't change content); beauty (depends on execution — could be crisp or could feel padded).

**Scope.** FORMAT.md convention addition; incremental per-segment application. Pairs naturally with O-BP14 (derivation-audit table) as the "entry-level" and "exit-level" counterparts of the segment-reading experience — O-BP14 tables are the summary *after* the formal content; SP-5 Reader's Paths are the orientation *before*.

**Effort shape.** 30 min convention + ~5 min per segment (~40 segments → ~4 sessions total if comprehensive; can be incremental as segments are next visited).

**Risks.** Pressure to write a tight Reader's Path may push authors toward overclaiming-for-concision, against the scope-honesty discipline. Convention must explicitly require "load-bearing but not overclaiming" in the Reader's Path.

**Status:** unexamined. TST's calibration-laboratory preamble already demonstrates the pattern (Phase 4, commit `d0373fc`).
