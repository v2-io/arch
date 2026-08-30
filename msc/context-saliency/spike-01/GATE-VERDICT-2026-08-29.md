# Calibration-gate verdict — Exp-1 v4 mint, 2026-08-29

*Status: gate result, n small, one instrumented model (Qwen2.5-7B-Instruct bf16 MPS). Numbers from `python analyze.py` on the minted JSON (`{SCRATCH}/analyze.txt` in the goal run). A clean failure of a cheap signal is a pass of this goal, not a miss.*

## What was minted

12 walk-task instrumented traces: seeds `{7, 11, 13}` × placements `{chrono, reversed}` × variants `{screened, delayed_reuse}`. Matched pairs: 6. `n_ctx` 539–587. v4 paths `out/v4instr_{placement}_{variant}_{seed}.json`. v3 `out/instr_*.json` hashes unchanged.

Base-correct (got = constructed answer): **7/12**. Failures: seed 13 systematically read `105` as `115` (all four traces); reversed delayed_reuse on seeds 7 and 13 emitted codes in *transcript* order rather than room-index order. Occlusion inference gated on the 7 base-correct traces only (`out/occlusion_v4.json`; 5 skipped).

T2-allhead is the **pre-filter** measurement (all-head, all-layer decode-row mass; Fragile Truth family; AGMR heads unwired).

## T1-temporal — event alignment — **no-go on phase boundaries**

Matched-pair Δ = delayed − screened at the same step, per-layer z-scored `g`, mean over transformer layers (no inherited 7–13/18–25 bands; traces are 54–56 steps so rolling-64 cannot run).

From the shipped analyzer, 6 pairs:

| Δ(near-boundary − elsewhere) band | final-quarter Δ band | sign(near > far) |
|---|---|---|
| min/median/max = **−0.3549 / −0.0363 / −0.0041** | **−0.0508 / +0.0822 / +0.1037** | **0/6** (random ≈ 50%) |

The predicted phase-boundary elevation is not present: every pair has near-boundary Δ *below* elsewhere. That fails “beat random” for the predicted sign. Final-quarter (reuse-element region) is weakly positive on the three chrono pairs and mixed on reversed — not a gate pass, not used as one.

## T1-spatial vs T3 — **no-go** (wrong sign; masking inverted)

Raw prefill hidden-diff (`g_prefill` layer-mean, **not** position-detrended in the analyzer) vs occlusion-flip labels on 7 base-correct rows, n=35 paired scores:

- Spearman **ρ = −0.242**
- top-20% flip-rate **0.429** vs bottom-20% **1.000** (k=7)

Higher spatial score does not predict damage when occluded; the bottom quintile flipped *more*. Fails “signals beat random and masking deltas separate” in the predicted direction.

A follow-up linear detrend of `g_prefill` against segment index (not in `analyze.py`) made this worse (ρ = −0.301; top 0.286 vs bottom 1.000). The no-go is not an undetrended artifact.

**This no-go is for causal ranking on this generator, not a claim that the signal is empty.** The inversion is signed and stable. Plan-NIAH puts causality in the *formulaic* `CODE-k: NNN` line by construction (v1 interiors were not screened; we moved every recompute-sufficient token onto the terminal) and puts screened mass in *varied* narrative. Prefill hidden-diff is an ingestion-novelty instrument. On this task family, novelty and causality are **anti-correlated by design** — the same construction that makes T3 labels decidable. Full characterization, two sentences that must not be collapsed, and the formulaic-surface discriminator: `INVERSION-T1-SPATIAL-2026-08-29.md`.

## T3 construction (behavioral) — holds on the base-correct subset

Flip counts among inferred cells (condition, flipped?):

- `terminal_ctrl` **7/7 flipped** (terminals are causal)
- `interior_body` (header preserved) **1/7 flipped**
- `header_ctrl` **1/7 flipped** (7B base-correct is less header-fragile than the spike’s mixed-seed full-interior wipe)
- `count_matched` (screened) **0/4 flipped**
- `reuse_line` **2/3 flipped**
- `narrative_ctrl` (header+body+count) **5/7 flipped**

The 2×2 is still a behavioral instrument. The cheap T1-spatial ranking does not track it.

## T2-allhead (pre-filter) — terminals still hotter than body

Per-room terminal/body attention ratio, 48 rooms across 12 traces: **min/median/max = 4.60 / 7.78 / 9.62**. Worst-case still well above 1. Chrono matched-pair reuse-line ratios (final quarter, pre-filter): 1.84, 2.06, 2.20. This is **not** a calibrated salience measurement; it is the same all-head family Fragile Truth breaks at length, still separating terminals from body at ~550 tokens.

## Headline

At this scale, on this model, **T1-temporal does not fire at phase boundaries** and **T1-spatial (prefill hidden-diff) does not rank causal spans against occlusion**. Experiments that need a span ranking proceed on **T3 (occlusion)**, with T2-allhead only as a pre-filter observational map. That is an actionable gate answer, not a failed mint.

7B smoke (criterion 1) is in the goal scratch `7b-smoke/` (two launches, both exit 0, seed 7 chrono). Generator placement tests: `python -m unittest test_tasks`. Muse (pid 73729) stayed up and was not used as a thinking sweep during instrumented 7B passes.
