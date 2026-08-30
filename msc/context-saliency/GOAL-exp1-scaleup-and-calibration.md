# Goal orientation: Exp-1 scale-up + the T1 calibration gate

*For an agent taking this as a goal (written 2026-08-29). You're picking up a live research line with a working rig; this file is the map, not the work order — the judgment calls are yours, and the documents below carry the why at whatever depth you want it.*

## Orient (in this order)

1. `REFLECTIONS-2026-08-28.md` — what this research line is and how it thinks (the founding dialog distilled, with all addenda).
2. `PLAN-experiments-draft-2026-08-28.md` — the experiment pipeline; you are executing **Phase 0's calibration gate + Exp 1 at scale** (§ "Phase 0" and § "Experiment 1").
3. `spike-01/FINDINGS-2026-08-29.md` — what the proof-of-concept already found, its method lessons (each cost a broken run), and what it deliberately left undone.
4. `spike-01/` code — `tasks.py` (generator, v3), `rig.py` (instrumented decode-row capture), `run_experiment.py`, `analyze.py`, `embed_diff_pg.py`. It all runs; the spike's debugged path is your starting point, not a constraint.
5. `dossier-2026-08-28.md` §6 for the methodology the plan inherits (worst-case-over-time reporting, occlusion protocol, the five conditions observational signals must earn).

## The goal, in one paragraph

The spike captured T1-temporal (`g` arrays) and never analyzed them; T1-spatial was not captured at all. The gate is two calibrations (plan, 2026-08-29 correction): **T1-temporal** against event alignment (phase boundaries, reuse moment via matched-pair, damage-detection); **T1-spatial** (prefill hidden-diff + KV variance) against T3 occlusion labels — Spearman + top/bottom-20% masking, worst-case bands, never averages. All-head T2 is the pre-filter measurement (Fragile Truth family); supervised-head T2 is unwired. Sequence: extend the rig (done in `spike-01/` v4) → smoke the new capture → mint the larger set (more seeds; more rooms; context lengths toward 2K–8K; **position-varied subgoal placement**; both task shapes). The bar is unchanged: signals beat random and masking deltas separate — a clean failure is a publishable result. Do not overwrite the four v3 `out/instr_*.json` runs; v4 writes `out/v4*`.

## What's already known that will save you pain

- Models on hand: Qwen2.5-7B-Instruct (**bf16 — fp16 overflows at 7B**), Qwen3-8B (downloaded, unrun — a cross-model replication of the reuse contrast would be genuinely valuable; the rig has `enable_thinking=False` handling ready), Qwen2.5-0.5B (debug only), Muse-Glimmer-30B via llama-server (behavioral judge; thinking model — read `reasoning_content`; ~48GB machine, don't run it concurrently with a 7B/8B instrumented pass plus anything heavy).
- The occlusion filler must be epistemically neutral AND locally uniform (both lessons in FINDINGS); base-correct gating is mandatory (7B misreads codes at ~40% of instances — that base-rate curve vs length is itself worth capturing as data).
- 7B occlusion inference needs base-correct instances only; Muse gives clean baselines but ~2min/run in thinking mode — budget it for the cells where the 7B's noise floor bites.
- pgvector db `context_saliency_spike` exists (psql-18) if the differential-recall line tempts you; it's explicitly optional here.
- Report trajectories with worst-case bands; detrend T1 with the causal rolling z-score before correlating (EpiKV recipe, bands are per-regime — don't inherit 7–13/18–25).

## Deliverables (coordination defaults, not constraints)

Results + updated findings into `spike-01/` (or a sibling `exp1/` if the scale-up outgrows the spike dir — your call), slimmed data in `data/`, and the calibration verdict stated at whatever strength it earns, committed as you go with honest completion language. If something upstream looks wrong — a plan assumption, a task-design flaw, a finding that doesn't replicate — surfacing that loudly is worth more than any completed sweep.

## Proposed `/goal` objective (stretch, 2026-08-29 evening)

Paste-ready. Completion is evidence-reviewable; a clean gate failure is completion, not a miss.

```
Exp-1 v4: land the split-T1 rig on Qwen2.5-7B, mint a first scaled Plan-NIAH cell, and state the calibration-gate verdict at whatever strength it earns.

Do not overwrite spike-01/out/instr_*.json (v3). Write v4 to out/v4instr_* and out/occlusion_v4.json.

Done when all of:
1. 7B instrumented pair exists with t1_spatial (prefill hidden-diff + KV var) and per-step entropy/logprob; arrays finite; roles include header and count. 0.5B smoke is not this evidence (it masked dtype once already).
2. Position-varied subgoal placement is in the generator (the basin confound the plan named).
3. A mint larger than the spike's n=4: at least 12 instrumented traces spanning ≥2 context lengths or ≥2 placements, walk-task, matched screened/delayed pairs, base-correct gated for any occlusion inference.
4. Calibration written into spike-01/ (LOOK or FINDINGS addendum): T1-temporal event-alignment on matched pairs; T1-spatial vs T3 (Spearman and/or top-bottom masking) OR an explicit no-go ("signal at floor / too few base-correct / beat-random fails"). All-head T2 labeled pre-filter. Worst-case bands, never averages as the headline.
5. Muse stays resident unless a 7B/8B pass needs the 20GB; do not run Muse thinking-sweeps concurrent with instrumented 7B.

Stretch if runway: same pairs on Qwen3-8B (reuse contrast); a length cell toward 2K. Not required for done.
Not in this goal: AGMR supervised-head T2 (separate construction), Exp 2/3, ALFWorld.
```

## The double duty (be aware, don't perform for it)

The session running this goal is itself a live test of the harness's new honest-compaction build: it's long enough to cross compaction seams. Nothing to do about it except work naturally — but if you notice seam effects (things you reach for and find missing, holes declared or undeclared, the summary's claims vs your actual state), note them in a `seam-notes.md` as you go; those observations feed the compaction workstream (`COMPACTION-DESIGN-THEORY-2026-08-29.md`) as first interlocutor-side field data.
