---
slug: partition-isolation
type: form
depends:
  - write-safety
  - lost-update-hazard
---

# Isolation comes from the layout, not from locks

*Concurrent writers in a corpus are kept apart by giving them disjoint files rather than by serializing them — which makes the layout itself the concurrency design, and makes a file's record-mapping something tooling needs declared rather than inferred.*

## The claim

**(A) Partition, not locking.** Where one record is one filesystem object named by its identity, two writers working on different records never meet — no lock, no coordinator, no protocol. Where events are written as separate files under collision-free names, many writers append to one logical stream without contending at all. This is the whole isolation mechanism in the estate's shipped stores, and it is a property of *how the records were laid out*, not of any runtime.

**(B) Same-key contention is deliberately left unserialized.** Two writers rewriting the same record produce a content disagreement that surfaces for judgment rather than being silently resolved. That is a feature: the arbitration is a decision, and decisions want a human and a recorded reason. What is not acceptable is contention that surfaces as *nothing*.

**(C) The reassurance that fails.** "Concurrent edits show up as a conflict" is sound for one-record-per-file and structurally false for multi-record files: a lost record is an absence, absences do not appear in diffs, and no history command can show what never arrived ([[lost-update-hazard]]). So a layout is either safe for concurrent writers or it is not, and this is a fact about the layout that tooling and agents cannot infer at the moment of writing — it has to be **declared**. A declared record-mapping is what tells a tool that the ordinary conflict net does not cover this file.

**(D) Where a shared file is still wanted, build a membrane.** The mature answer everywhere this has been solved is not better coordination but a single writer: an intake queue with one drainer, an append-only protocol the filesystem itself enforces, a log that only one hand promotes into. And one inheritable primitive detail — a create-if-absent that *fails loudly on collision* expresses "this key should not already exist," which an atomic replace cannot say; the classic mail-storage repair used exactly that, and per-key writes without it can silently clobber.

**(E) Scope limit.** This does not settle same-key policy, cross-file transactions, or durability across a crash (visibility and durability are different guarantees, and directory-level flushing is a real omission in the naive form of atomic replace). Those are named as adjacent, not answered here.

## Strength & grounds

**Heuristic / engineering.** The mechanisms in (A) and (D) are shipped estate practice — per-key entry files, per-key verification event directories with timestamped collision-free names, and a single-drainer intake spool, all read first-hand in `~/src/arch/firmatum/relata/` on 2026-08-05. (C) is a standard concurrency result rather than an estate discovery, with a live specimen of the dangerous shape already accounted for at [[lost-update-hazard]] — vivarium's multi-record root decision log, re-checked present on 2026-08-05. No dual-writer race was simulated and no count of actually lost records is claimed — the observation is the shape plus the mechanism. The declaration proposal in (C) is a design position from the udon file-roles correspondence, unimplemented anywhere.

## Working Notes

- The cheapest test of (C): take any live multi-record file and ask what tells a fresh agent it is one. Today the answer is generally "nothing, and reading the file will not tell you either."
- (A) and the strata design are the same decision seen twice — disjoint placements are also what give each layer its own write clock ([[layer-speeds]]).
- Unclaimed and worth someone's time: whether per-part write rules on a shared file are expressible at all without declared record boundaries. If they are not, that is a genuine substrate-conditional principle and [[substrate-independence]] narrows.
- Unintegrated influx behind this segment (do not cite as warrant): `plan/INFLUX/udon-analysis/underlying-logical-model.md` §2 and §5. Live original under `~/src/arch/firmatum/udon/v2/theory/to-integrate/primary/`.
