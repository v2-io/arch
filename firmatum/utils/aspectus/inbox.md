# aspectus inbox — feedback, anomalies, issues, confusion

*Solicited by the tool itself (footer on every look, added 2026-08-14 at Joseph's direction). Append at the bottom: what you saw, the command, and your cwd. Raw and unpolished is perfect — this is an inbox, not a report. Routed periodically into the pipeline/audit flow by the coordinating session.*

## Routed (ledger — the entries themselves now live verbatim at their destinations)

- 2026-08-22 — Joseph's multi-path ask → [`design/focus.md`](design/focus.md) §Multiple paths. Fable instance's `~/src` mid-tree cut (exit 1, silent tail) → PRACTICA as a bug to reproduce; its heat-threshold wish → noted in focus.md §Multiple paths (weight/threshold, undesigned). **Resolved 2026-08-22:** the mid-tree cut was the *harness's* ~30 KB tool-output cap (look = 42 KB; cap at line 212, `umi/` at 230), not aspectus — exit 0 and clean tail on the exact command; SIGPIPE paths all exit 0. Hazard recorded in PRACTICA (header-states-its-own-size proposal held for the overview-invariants discussion).

- 2026-08-15 — three entries by Joseph (2026-08-14): mass lines in the `lines` column and description-wrap-to-sub-lines / logical `--lines` → [`design/vertical-info.md`](design/vertical-info.md) §Steward asks; config-drift in the header → [`design/overview-invariants.md`](design/overview-invariants.md) §Config drift. Pass record: [`audit/usability-aesthetics-2026-08-15.md`](audit/usability-aesthetics-2026-08-15.md).

- 2026-10-03 — two entries (Opus 5.5 instances, 2026-09-24 and 2026-09-30), routed with Joseph's decisions the same day — entries + decisions verbatim in [`audit/inbox-2026-10-03.md`](audit/inbox-2026-10-03.md): (a) **ignored dirs carry no mass** (`target/` 110G, `models/` 36G shown bare) → new pipeline row *Ignored-dir bytes* — a walk-bounded byte total on the `⊘` line itself (`≥` when cut), never folded into the parent's mass; (b) **`lines` column drifts right on long names** → stop computed from the widest name in the look (no cap) (design home: `design/grid-cleanup.md`); the trailing `.` is the decided anchor (second outside arrival of "loud on small numbers"); (c) **empty dir renders bare** (4th arrival) → `[empty]` (design home: `design/grid-cleanup.md` / lattice-2's empty-dir row). Design + dev delegated 2026-10-03.

---

---


## 2026-10-03 — blank `lines` cell on larger files, no mark (independent-reading agent, clean room)
- cmd: `aspectus --lines 300 --depth 4 .`  cwd: `/private/tmp/claude-505/-Users-josephwecker-v2-src-arch-firmatum-utils-aspectus/fcc2ed5a-ac92-4900-9048-7c0adcf7b4bf/scratchpad/gmp-cleanroom/`
- Files of roughly 256 KB or more show an empty `lines` cell with no marker. Examples: `data/stimuli-v1/triads.jsonl` (279 KB, 1800 lines), `pilot/results2-llama3.2_3b.jsonl` (269 KB, 1512 lines), `data/surveys-v1/extracted/sonnet5-1.jsonl` (476 KB, 429 lines). A 240 KB sibling (`fable-1.jsonl`) shows 236. If this is a deliberate size cap, the cell looks the same as "no lines" and doesn't confess the cut the way `≥`/`[walk bound]` do elsewhere. On first read I took these for empty or binary files.
- Otherwise the look was what I needed. `triads-perp-single-gptoss20b/ … [spec.json]` told me at a glance that gpt-oss had no ledger, a fact that mattered to the analysis.

## 2026-10-04 — `ASPECTUS_COLUMNS_HEAT=off` now draws a migration notice; the global CLAUDE.md still recommends it (glyph-study design agent, Opus 5.5)
- cmd: `ASPECTUS_COLUMNS_HEAT=off aspectus --lines 300 --depth 3 .`  cwd: `~/src/arch/asf/empirica/glyph-magnitude-perception/`
- First output line: `aspectus: columns.heat accepted for this release; membership is now [layout] (see 'aspectus config defaults')`. `~/.claude/CLAUDE.md`'s aspectus entry still tells de novo auditors to use `ASPECTUS_COLUMNS_HEAT=off`, so every agent following it will hit this notice and has to guess the new spelling. I didn't look up the [layout] form; the look itself worked (heat was off).
- The look was useful: `analysis/views/` and `independent-reading-2026-10-03/` showed up at the top by recency, which told me the newest interpretive layers before I'd read anything.
