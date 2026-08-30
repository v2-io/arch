# Goal orientation: Exp-1 scale-up + the T1 calibration gate

*For an agent taking this as a goal (written 2026-08-29). You're picking up a live research line with a working rig; this file is the map, not the work order — the judgment calls are yours, and the documents below carry the why at whatever depth you want it.*

## Orient (in this order)

1. `REFLECTIONS-2026-08-28.md` — what this research line is and how it thinks (the founding dialog distilled, with all addenda).
2. `PLAN-experiments-draft-2026-08-28.md` — the experiment pipeline; you are executing **Phase 0's calibration gate + Exp 1 at scale** (§ "Phase 0" and § "Experiment 1").
3. `spike-01/FINDINGS-2026-08-29.md` — what the proof-of-concept already found, its method lessons (each cost a broken run), and what it deliberately left undone.
4. `spike-01/` code — `tasks.py` (generator, v3), `rig.py` (instrumented decode-row capture), `run_experiment.py`, `analyze.py`, `embed_diff_pg.py`. It all runs; the spike's debugged path is your starting point, not a constraint.
5. `dossier-2026-08-28.md` §6 for the methodology the plan inherits (worst-case-over-time reporting, occlusion protocol, the five conditions observational signals must earn).

## The goal, in one paragraph

The spike captured T1 signals (per-layer hidden-state diffs, in every instrumented JSON's `g` arrays) **and never analyzed them** — the plan's "calibration gate" is unrun: do the cheap always-on signals track what occlusion proves? Answering that needs more data than four runs: scale Exp 1 (more seeds; more rooms; context lengths swept toward 2K–8K; **position-varied subgoal placement** to break the position/decay confound; ideally both task shapes — walk and sum — since the spike showed trajectory shape is task-dependent), run the occlusion tier for ground truth on the same instances, then calibrate T1 (and the utilization matrix) against the labels: Spearman + top/bottom-20% masking deltas per AGMR, worst-case bands, never averages. The gate's honest bar is in the plan: "signals beat random and masking deltas separate" — and a clean failure is a publishable result, not a failed goal.

## What's already known that will save you pain

- Models on hand: Qwen2.5-7B-Instruct (**bf16 — fp16 overflows at 7B**), Qwen3-8B (downloaded, unrun — a cross-model replication of the reuse contrast would be genuinely valuable; the rig has `enable_thinking=False` handling ready), Qwen2.5-0.5B (debug only), Muse-Glimmer-30B via llama-server (behavioral judge; thinking model — read `reasoning_content`; ~48GB machine, don't run it concurrently with a 7B/8B instrumented pass plus anything heavy).
- The occlusion filler must be epistemically neutral AND locally uniform (both lessons in FINDINGS); base-correct gating is mandatory (7B misreads codes at ~40% of instances — that base-rate curve vs length is itself worth capturing as data).
- 7B occlusion inference needs base-correct instances only; Muse gives clean baselines but ~2min/run in thinking mode — budget it for the cells where the 7B's noise floor bites.
- pgvector db `context_saliency_spike` exists (psql-18) if the differential-recall line tempts you; it's explicitly optional here.
- Report trajectories with worst-case bands; detrend T1 with the causal rolling z-score before correlating (EpiKV recipe, bands are per-regime — don't inherit 7–13/18–25).

## Deliverables (coordination defaults, not constraints)

Results + updated findings into `spike-01/` (or a sibling `exp1/` if the scale-up outgrows the spike dir — your call), slimmed data in `data/`, and the calibration verdict stated at whatever strength it earns, committed as you go with honest completion language. If something upstream looks wrong — a plan assumption, a task-design flaw, a finding that doesn't replicate — surfacing that loudly is worth more than any completed sweep.

## The double duty (be aware, don't perform for it)

The session running this goal is itself a live test of the harness's new honest-compaction build: it's long enough to cross compaction seams. Nothing to do about it except work naturally — but if you notice seam effects (things you reach for and find missing, holes declared or undeclared, the summary's claims vs your actual state), note them in a `seam-notes.md` as you go; those observations feed the compaction workstream (`COMPACTION-DESIGN-THEORY-2026-08-29.md`) as first interlocutor-side field data.
