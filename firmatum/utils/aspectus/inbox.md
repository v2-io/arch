# aspectus inbox — feedback, anomalies, issues, confusion

*Solicited by the tool itself (footer on every look, added 2026-08-14 at Joseph's direction). Append at the bottom: what you saw, the command, and your cwd. Raw and unpolished is perfect — this is an inbox, not a report. Routed periodically into the pipeline/audit flow by the coordinating session.*

## Routed (ledger — the entries themselves now live verbatim at their destinations)

- 2026-08-22 — Joseph's multi-path ask → [`design/focus.md`](design/focus.md) §Multiple paths. Fable instance's `~/src` mid-tree cut (exit 1, silent tail) → PRACTICA as a bug to reproduce; its heat-threshold wish → noted in focus.md §Multiple paths (weight/threshold, undesigned). Both entries kept below until the bug is reproduced.

- 2026-08-15 — three entries by Joseph (2026-08-14): mass lines in the `lines` column and description-wrap-to-sub-lines / logical `--lines` → [`design/vertical-info.md`](design/vertical-info.md) §Steward asks; config-drift in the header → [`design/overview-invariants.md`](design/overview-invariants.md) §Config drift. Pass record: [`audit/usability-aesthetics-2026-08-15.md`](audit/usability-aesthetics-2026-08-15.md).

---


## 2026-08-15 — Fable instance, cwd ~/src/AISI-responses (orienting for the AISI response)
- `aspectus --lines 300 --depth 3 ~/src` overflowed the budget and the cut landed *mid-tree* (output truncated inside `umi/`), exit code 1. Redirected to a file to see it whole. A footer at the cut ("[+ N subtrees not shown under budget]") would tell me what I'm not seeing; right now the missing tail is silent.
- The thing I most wanted for orientation and couldn't express: filter by heat ("only dirs with heat > 0.2") — recency ordering gets close, but a threshold would let me see the live corners of a 40-project estate in one screen.
- Positive: recency-first + heat + `[has: …]` census told me in one look which corners of ~/src/arch were alive this week (verisectorium influx, comproprium, firmatum/principles) — that was the whole value, and it worked.


---

I need to be able to do this:

aspectus --lines 200 --depth 4 ~/src/arch/asf/{01-aat-core,02-tst-core,03-llm-core,04-eli-core}

---
