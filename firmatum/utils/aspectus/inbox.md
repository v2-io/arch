# aspectus inbox — feedback, anomalies, issues, confusion

*Solicited by the tool itself (footer on every look, added 2026-08-14 at Joseph's direction). Append at the bottom: what you saw, the command, and your cwd. Raw and unpolished is perfect — this is an inbox, not a report. Routed periodically into the pipeline/audit flow by the coordinating session.*

## Routed (ledger — the entries themselves now live verbatim at their destinations)

- 2026-08-22 — Joseph's multi-path ask → [`design/focus.md`](design/focus.md) §Multiple paths. Fable instance's `~/src` mid-tree cut (exit 1, silent tail) → PRACTICA as a bug to reproduce; its heat-threshold wish → noted in focus.md §Multiple paths (weight/threshold, undesigned). **Resolved 2026-08-22:** the mid-tree cut was the *harness's* ~30 KB tool-output cap (look = 42 KB; cap at line 212, `umi/` at 230), not aspectus — exit 0 and clean tail on the exact command; SIGPIPE paths all exit 0. Hazard recorded in PRACTICA (header-states-its-own-size proposal held for the overview-invariants discussion).

- 2026-08-15 — three entries by Joseph (2026-08-14): mass lines in the `lines` column and description-wrap-to-sub-lines / logical `--lines` → [`design/vertical-info.md`](design/vertical-info.md) §Steward asks; config-drift in the header → [`design/overview-invariants.md`](design/overview-invariants.md) §Config drift. Pass record: [`audit/usability-aesthetics-2026-08-15.md`](audit/usability-aesthetics-2026-08-15.md).

---

---


- 2026-09-24 — Opus 5.5 instance, disk-usage hunt for Joseph. **Ignored dirs carry no mass, and that's exactly where disk goes.** Twice in one session the thing I was looking for sat inside an ignored dir, shown with no size: (1) `aspectus --lines 300 --depth 4 .` in `~/src-ext/grok-build-compaction-fork/` — header said `[ignored×1]` and the tree never showed `target/`, which was 110G of a 116G checkout (`du` found it); (2) `aspectus --lines 120 --depth 3 .` in `~/src/_self/memoryllm-eval/` — `⊘ models/` with an empty lines column, and it was 36G of safetensors. Both were disclosed (`[ignored×1]`, `⊘`), so nothing was hidden by omission, but "why is this dir huge" is a natural question to bring to a look, and the look points away from the answer. Wish: a cheap byte mass on ignored/⊘ entries (even `≥` or approximate), or at least `bytes` on the root line. cwd: as above.
