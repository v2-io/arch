# RONR-12 model — changelog

*History layer. Working files state present truth only; how they got there lives here.*

## 2026-08-27 — stages 1–3, verification, integration

- **Stage 1.** Source split: `ronr-12c.md` → `ronr-12c-front.md` (TOC/frontmatter, lines 1–1130), `ronr-12c-main.md` (Chapter I onward), `ronr-12c-index.md` (book index). Joseph later added `table-ii.md` and `table-viii.md` extracted from the plates in `images/`.
- **Stage 2.** `RONR-12-MODEL.md` (seven-component decomposition) and `model/INTERFACE.md` written by the coordinating session.
- **Stage 3.** Five fork agents authored the component files in parallel against INTERFACE.md; committed at `8df9606` with `STAGE-4-BRIEF.md` carrying their handoff reports and four self-flagged seams.
- **De novo audit.** A fresh agent outside the authoring line verified ~120 citations first-hand plus Table II sampling and Chart I (report: `VERIFICATION.md`, since absorbed and deleted — see below). Verdict: substantially faithful; no fabricated citations, no invented rules, no inversions. Findings F1–F8:
  - **F1 (structural):** SDC-2 applicability had no data backing; concrete wrong admissions (commit over pending reconsider [13:7(2)]; postpone/commit over an undebatable adhering appeal [24:3(1)]; postpone's undebatable-pending exception set [14:4]).
  - **F2:** both 01 and 03 stated the incidental-yield rule [6:19] without its division-of-assembly exception.
  - **F3:** 01 §5.1's series-halting list contradicted [10:34] and 03 (omitted recess, added adjourn).
  - **F4:** committee-context thresholds missing (reconsider two-thirds [37:35(3)]; rescind/amend committee rule [35:2(7)]).
  - **F5:** `sched:` vs `scheduler:` prefix collision + queue-id granularity mismatch (unnamed fifth seam).
  - **F6:** guard-registry collisions — duplicate `no-question-pending`, sense-inverted `dilatory`/`not-dilatory` pairs (unnamed sixth seam).
  - **F7:** smaller verified omissions — take-from-table intervening-business precondition [34:2(2)]; CFOTD yields to reconsider [18:4(1)]; imprecise exhaustion cites in 03 §3.4.3; rescind/amend notice qualifier [35:2(7)]; reconsider-enter-minutes applicability compressed vs [37:47(3)].
  - **F8:** affirmations — the subtle SDC attributes, ballot table, split-rank slotting, dispatch synthesis, timers, interpretation principles, and all `NOTE(judgment)` markers verified correct. (Preserved in STAGE-4-BRIEF "Verification status".)
- **Integration (this change).** F2, F3, F4, and most of F7 fixed directly in `01-catalog.md`, `03-engine.md`, `06-07-rules-scheduler.md`; F1's three concrete cases closed via new `catalog:table/appeal-yield-matrix` (01 §5.6) and per-record conditions, with the full applies-to/does-not-yield-to extraction assigned to stage 4; F5's prefix half fixed by renaming `sched:` → `scheduler:` in 06-07 (queue-id half assigned to stage 4); F6 addressed by adding the closed-prefix rule and guard polarity convention to INTERFACE §3/§5 and rewiring 03's guard declarations to consume the catalog registry (single registry assigned to stage 4). F8, the [34:6] flattening caution, the acceptance-sweep additions (Chart I column diff, Table II re-expansion diff, appeal passages as engine vectors), and the independent-checks framing for future auditors moved into STAGE-4-BRIEF. `VERIFICATION.md` then deleted as fully absorbed (integration-is-replacement).
- **Register cleanup (same change).** Evolution narration ("added after verification", fix-log framing, F-number references) stripped from INTERFACE.md, STAGE-4-BRIEF.md, and 03-engine.md; working files now state present truth, with this file as the history carrier.
