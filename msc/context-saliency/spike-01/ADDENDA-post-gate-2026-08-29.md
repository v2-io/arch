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

## Inversion / construction

See `INVERSION-T1-SPATIAL-2026-08-29.md` — the load-bearing note. Formulaic discriminator did not dissolve the inversion at n=10.
