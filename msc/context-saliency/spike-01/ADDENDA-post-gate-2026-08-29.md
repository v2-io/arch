# Post-gate cells — Qwen3 reuse, 20-room length, reversed-placement, 2026-08-29

*After the v4 gate. v3 `out/instr_*.json` and the 12-trace v4 mint were not overwritten.*

## Qwen3-8B reuse contrast (thinking off)

Four traces, 4-room chrono, seeds 7 and 11, all **base-correct**. T2-allhead reuse-line final-quarter delayed/screened:

| seed | Qwen3-8B | Qwen2.5-7B v4 (same seeds) |
|---|---|---|
| 7 | **0.99** | 2.20 |
| 11 | 1.63 | 1.84 |

The spike’s most robust 7B instrumented finding (query-conditional elevation of the reuse line) **does not travel cleanly**. Seed 7 on Qwen3 is a null. T2-allhead is still pre-filter; this is a cross-model map, not a calibrated measurement.

## 20-room length cell (~2266–2296 n_ctx) — measure the floor, don't mourn the sweep

Eight 7B traces (`out/len2k_20r_*`). Base-correct **1/8** (chrono delayed_reuse seed 11). Failures are digit-level retrieval and dash-collapse, not placement-only. T2-vs-length and T1-vs-length are **not scorable** on this (model × family) pairing: the retrieval floor swallows the cell. That is a program fact, not a failed experiment.

Capability curve, 3-digit codes, greedy 7B, walk-task, unmodified prompts:

| cell | n_ctx | n_codes | base-correct |
|---|---|---|---|
| spike v3 (FINDINGS) | ~550 | 4 | 7/12 (5/12 fail) |
| v4 mint | 539–587 | 4 | **7/12** |
| 20-room | 2266–2296 | 20 | **1/8** |

The GOAL file already named this curve as worth having. Length-sweep of salience on 7B Plan-NIAH with 3-digit codes cannot run until the floor recedes (2-digit codes, or a stronger model). The measurement this regime gives for free is the floor itself.

## Reversed placement (v4 mint) — capability, not just confound

Recorded in the gate (seeds 7, 13 delayed_reuse walked *transcript* order). Placement cells are not difficulty-matched to chrono. Walk-task copy now says room-number order regardless of transcript order (`tasks.py`); that fix applies to later cells (formulaic, 20-room), not retroactively to v4.

## T1-temporal boundary depression

v4: 0/6 predicted sign, **6/6 opposite** (near-boundary Δ depressed). Pre-registered for the next mint: “boundary depression replicates.” Not claimed as a result at n=6.

## Qwen3-8B 20-room — floor receded; T3 at ~2.3K is live

Moved the 20-room cell to Qwen3-8B (thinking off) instead of changing the code alphabet. Chrono, seeds 7 and 11, both variants: **4/4 base-correct** (`out/len2k_qwen3_20r_chrono_*.json`, n_ctx 2242–2277). 7B on the same construction was 1/8. The length sweep as planned can run on this model × family pairing.

T3, gated on those four (`out/occlusion_qwen3_20r.json`, 0 skipped):

| condition | flipped |
|---|---|
| `interior_body` (header preserved) | **0/4** |
| `header_ctrl` | **0/4** |
| `narrative_ctrl` (header+body+count of one room) | **0/4** |
| `terminal_ctrl` | **4/4** (first-code substitution) |
| `count_matched` (screened) | **0/2** |
| `reuse_line` | **1/2** (seed 11: count `4`→`9`, silent; seed 7 held) |

The 2×2’s behavioral axis **survives at ~2.3K** on this model: screened interiors are droppable; terminals are not. Headers are not load-bearing here (capability-graded, same direction as Muse-30B at 4-room). Reuse-as-silent-drop is mixed at n=2.

T2-allhead (pre-filter) terminal/body across 80 rooms: **min/median/max = 0.43 / 6.73 / 10.26**. Median still looks like “terminals hotter”; the worst-case room is *colder* than body. That is the Fragility lesson at length — do not headline the median. Reuse-line delayed/screened at final quarter: **0.94 / 0.73** — the 4-room Qwen3 elevation is gone at 20 rooms.

7B 20-room files were not overwritten.

## Inversion / construction

See `INVERSION-T1-SPATIAL-2026-08-29.md` — the load-bearing note. Formulaic discriminator did not dissolve the inversion at n=10.
