---
slug: retrofit-specimen
type: obs
depends: []
---

# The autopax graft: adoption stopped at the invariant-bearing subsystem

*The estate's one measured attempt to retrofit a shared schema framework onto an existing system — fast adoption everywhere the stakes were low, and a permanent stall exactly where integrity lived.*

## What happened

In late 2025, autopax — a working system with several subsystems of very different maturity — adopted the Archema resource framework (now rowan). Phases 0–2 landed within **three days**, into subsystems that were new or thin. Phase 3 was CHRONICA and TRACTUS: the hash-chained integrity log, the subsystem whose correctness the rest depends on. Its blocking item read "verify BLAKE3 hash chain compatibility." **Phase 3 was never started.**

Seven months later (2026-07-20), when that subsystem was finally addressed, the framework was not used: `CHRONICA-PORT-SPEC.md` treats autopax's hand-rolled log as source material for an independent Rust spine, with Archema absent.

The confound is real and recorded in the estate's own notes: the wider stall coincided with attention moving to ASF ("largely stalled once ASF took legs — thankfully, so the math could solidify first"). What survives the confound is narrower and still sharp: the graft covered the thin subsystems and never reached the invariant-bearing one — and when that subsystem's turn came, the framework was passed over rather than resumed.

## Why it is carried here

This is the estate's best evidence about what ch. 11's generator proposes to do — introduce a shared spine into places that already work. The reading the source analysis drew: **a schema layer that cannot absorb the integrity-critical subsystem has located the case that will outlive it; sequencing the hardest subsystem first is diagnostic either way** — three days tells you one thing, a never-started Phase 3 tells you the other, and both are cheaper to learn first than last. Any instantiation or migration plan that starts with the friendly cases inherits this specimen as its warning.

## Method & scope

Primary: the cross-corpus generalization record, Part 2 (live: `~/src/arch/notes/outline-segments-generalization-2026-07-23.md`), itself citing the doc-store review's §1.4 trace of the adoption phases and `CHRONICA-PORT-SPEC.md`; register: inherited at a stated remove — this segment's author verified the generalization note's text first-hand (2026-08-05) but has not re-walked the autopax repo or the port spec. One system, one adoption, one confound named. It supports "the hard case is diagnostic and does not arrive by default"; it does not support "retrofits fail."

## Working Notes

- Discharges TODO entry R12. A first-hand re-walk of `~/src/autopax/` (phase records, ADR trail, the port spec) would raise the register to first-hand and might date Phase 3's abandonment precisely.
- The forward question for ch. 11: what "sequence the hardest subsystem first" means for a *corpus* migration — plausibly: migrate the instance whose invariants are strictest (logos' submission gates? vivarium's moratorium-adjacent law?) before the friendly ones.
