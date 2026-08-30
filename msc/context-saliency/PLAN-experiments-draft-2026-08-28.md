# Context-saliency experiment pipeline — provisional draft, 2026-08-28

*Status: **proposed** (agent-drafted, unratified — Joseph decides what proceeds). Built from the founding dialog ([`REFLECTIONS-2026-08-28.md`](REFLECTIONS-2026-08-28.md)), the literature dossier ([`dossier-2026-08-28.md`](dossier-2026-08-28.md)), and five whole-paper reads ([`notes/paper-readings-2026-08-28.md`](notes/paper-readings-2026-08-28.md)). Designed for the minimal setup: one open-weights model on Joseph's hardware, batch-size-1, text-only environments, everything scriptable. The trunk is deliberately the piece whose instrumentation every branch reuses.*

## Design commitments (each traceable to a read source)

1. **Ground truth by construction, then occlusion, then behavior** — never a saliency score as its own evidence. (Fragile Truth: the best instrument recovers <45% of causal mass; EpiKV occlusion protocol; DefensiveKV behavioral validation.)
2. **No full attention matrices** — eager attention OOMs an 80GB A100 at ~8K context. Instrument stack: **T1** hidden-state/KV signals (free, every step, any length) → **T2** named retrieval-head recomputation (cheap, segment-level) → **T3** occlusion + behavioral delta (offline ground truth). (EpiKV §4.4, DefensiveKV fn. 2.)
3. **Observational signals earn use through five conditions**: supervised head selection, above-token aggregation, causal detrending, masking spot-checks, worst-case-over-time reporting. (Synthesis note 2 in the readings file.)
4. **Report trajectories with worst-case bands, never averages.** (DefensiveKV; Fragility's avg-0.92/worst-0.34 lesson.)
5. **Run query-known and query-unknown conditions where consolidation is involved** — the delta is the H(Q) penalty, measured. (Rate-distortion survey via DefensiveKV's eval protocol.)
6. Salience scores are **(context, query)-conditional** — recompute per probe; a one-shot heatmap is not a defined object. (Pitfalls; dossier §4.)

## Phase 0 — the rig (shared by everything)

- **Models**: Qwen2.5-7B-Instruct as primary (AGMR's supervised retrieval heads already published: (17,18),(17,19),(19,15),(19,17),(19,22); HIPIF trains on the same family, opening a trained-vs-base comparison later). Llama-3.1-8B-Instruct as the replication model (heads: (13,6),(14,13),(14,29),(14,31),(13,1)). DeepSeek-R1-Distill-Llama-8B only if/when a reasoning-trace regime is added (EpiKV's bands are calibrated there).
- **Serving**: HF transformers with FlashAttention for T1 runs; hooks on residual stream (per-layer hidden states are exposed by standard interfaces). T2 = recompute QK for the 5 named heads only, from cached hidden states — never `output_attentions`.
- **T1 signals logged every step**: per-layer hidden-state L2 diffs g_l(t) + causal rolling-64 z-scores (EpiKV recipe, bands to be calibrated per task-regime — do not inherit 7-13/18-25 blindly, the bands move with difficulty); KV key/value variance; next-token entropy/logprob.
- **T2 signal**: AGMR-style context utilization matrix — per-segment, per-decision-step, observation-tokens excluded, prompt-length rescaled, top-5 heads averaged. Head sets re-derived on our tasks via AGMR's supervision protocol (mine fail-without-X vs succeed-with-X transition cases) rather than trusted blindly — the published lists are the starting hypothesis and a cross-check.
- **T3 harness**: segment-occlusion (pad-replace at equal length, regenerate greedily, record answer/action change — EpiKV Appendix C adapted to segment boundaries) and full-vs-modified-context behavioral delta.
- **Calibration gate before any experiment reports**: on ~50 traces, check T1/T2 rankings against T3 occlusion labels (Spearman, and top-/bottom-20% masking deltas per AGMR). Expect modest correlations (EpiKV saw |ρ| ≈ 0.1–0.25 per layer); the gate is "signals beat random and masking deltas separate," not "signals are good." If they fail even that on plan traces, that is itself a publishable negative and the experiments proceed on T3 alone, slower.

## Experiment 1 — the trunk: native salience structure over hierarchical plans ("Plan-NIAH")

*The Strategy-DAG measurement. Its rig is a strict subset of every branch's.*

**Task design — ground truth by construction.** Synthetic multi-subgoal tasks, script-generated, in two matched families:

- **Screened family**: each subgoal's interior is causally screened by construction — e.g. combination-lock chains: subgoal k requires exploring/working to obtain code c_k; only c_k (the terminal observation) is ever needed again; the *how* is irrelevant post-completion, provably. Interiors are the plan-analog of NIAH distractors *after* discharge while being load-bearing *during* execution — the ground-truth salience trajectory is a step function known in advance.
- **Delayed-reuse family**: identical surface structure, but one interior detail from subgoal j is silently required again at subgoal j+m (the Governance-Decay analog: dormant-but-binding). Which detail, and the lag m, are generator parameters.

Populate the 2×2 by construction: screened interiors (low-salience-correct = healthy release), delayed-reuse interiors (low-salience-wrong = silent drop), terminal observations (high-salience-correct = live working set). Environments: pure-text, deterministic, ~10–40 subgoals, context lengths swept 2K–32K. ALFWorld PICK2-style tasks as the ecological-validity check after the synthetic results, not before (synthetic first — Joseph's invariant-control experimental posture).

**Measurements & predictions** (each falsifiable, each from the founding dialog or ASF):

- **P1 (healthy decay)**: T1/T2 salience of screened interiors decays after subgoal discharge, faster and deeper than branch-open/close segments. Shape and rate are the measurement; ASF Strategy-DAG bounds are the comparison curve (deferred to when the theory side supplies the idealized distribution — the instrument comes first).
- **P2 (turnover at boundaries)**: heavy-hitter set turnover (top-K by T1 score, Jaccard step-to-step) spikes at subgoal boundaries and is quiet inside subgoals — task-topological stability vs the "recency plus fixed anchors" null.
- **P3 (the capability gap)**: in the delayed-reuse family, does the to-be-reused interior *retain* salience before its reuse (anticipatory retention would be remarkable), *recover* it at reuse (retrieval), or fail (the lost-middle pathology)? Failure rate vs lag m and context length is a capability curve no benchmark currently measures.
- **P4 (behavioral closure)**: occluding decayed screened interiors → null behavioral delta; occluding delayed-reuse interiors → damage. This closes the 2×2 empirically and is the validation that P1's decay is *healthy* rather than merely present.

**Confound controls** (all from the readings): position-detrend every T1 signal; report window-fill fraction, not just absolute position; check basin/edge shape (attention-basin literature says structured item sequences get U-shaped attention — distinguish decay-after-discharge from middle-position artifact by varying *where* in context a discharged subgoal sits, which the generator makes free); sinks excluded from scoring; per-probe query conditioning.

**Discoverables**: whether native healthy decay exists at all in base instruct models (HIPIF's evidence suggests weakly — untrained folding fails, and retained middles measurably interfere); the turnover signature; the delayed-reuse capability curve; and calibration data on how well cheap signals track plan-level ground truth (valuable either way, per the gate).

## Experiment 2 — branch: salience-gated consolidation (application 3)

Same tasks, run long enough to need compaction. Compare policies at matched token budgets:

(a) LLM summarizer (the industry default, "summarize for your successor"); (b) structural fold — HIPIF's [g_k, o_k^end] rule applied by harness, no training; (c) **salience-gated verbatim retention** — keep segments by measured worst-case-over-observed T1/T2 score with a per-head prior floor (DefensiveKV's aggregation, transplanted to consolidation), summarize only the residue; (d) full context (ceiling); (e) recency window (floor); (f) **prospective compaction** (Joseph's hypothesis, 2026-08-29 — see REFLECTIONS addendum): the compactor is prompted to *predict what remains undone and likely-to-be-asked*, preserve by P(needed) x reconstruction-cost, emit **signposts** (pointers back to what was cut), and — leg 3 — state **explicitly where the holes now are**, defeating the false-completeness implicature. Measurable per capability tier: a hole-aware summary should eliminate silent confabulation in successors below the absence-detection threshold (the flag is supplied) and eliminate rumination-spiral cost in successors above it (detection gets routing + a stopping license); the spike's 7B/Muse-30B contrast is the instrument for both. Framing note that governs the whole experiment: measured salience is retrospective and query-bound; "salience overall" is a category error — every policy here is a *bet* about the future query distribution, and (c) formalizes the bet that past attention predicts future need, which the delayed-reuse family falsifies by construction. The interesting result is the *pattern* of which policies fail which task families, not a single ranking.

Planted dormant constraints (Governance-Decay protocol: establish rule → compact → re-issue identical prohibited request → grade *deterministically by effects*, judge-model only secondary) ride along in every condition.

**Discoverables**: does measured salience beat the structural rule (if not, the structure *is* the salience and cheap harnesses win); do dormant constraints survive salience gating — prediction: **they don't** (they score cold), which would demonstrate empirically that salience alone cannot be the retention criterion and the screening-off counterfactual (or pinning) is irreducible — the founding dialog's claim, falsifiably staged; the measured H(Q) delta between query-known and query-unknown compaction.

## Experiment 3 — branch: SASM-pidgin, timing isolated (application 4)

The minimal harness-level test of SASM's core claim — that *when* a reminding arrives matters, not only *what* arrives.

- Memory corpus: prior traces/facts relevant to the current task family (generator-controlled so relevance is known).
- **Trigger conditions**: (i) T1 epiphany-signal spike (causal z threshold); (ii) next-token-entropy spike; (iii) fixed-interval injection; (iv) random-timing injection; (v) no injection. Same retrieval, same terse-fragment format (8–12 words, Kellogg's design instinct), same forward-only frontier append (cache-preserving by construction).
- **Measures**: downstream-span logprob/perplexity delta, task success, and — the SASM-specific one — *uptake*: does the generation actually use the injected fragment (occlude it post-hoc and check)?
- **The hypothesis in miniature**: (i)/(ii) beat (iii)/(iv) at matched injection budgets. If timing confers nothing over random, token-level SASM is not worth a training run and the involuntary framing needs rethinking; if it does, conditions (i)/(ii) *generate the training signal* for a learned trigger (uncertainty-resolved-by-retrieval events, as designed in the founding dialog), and the next stage is a trained trigger head/token — which is where PIC cache-splicing (MiniPIC) enters, not before.

## Experiment 4 — branch: directed separation, first contrast (application 2)

Deferred-but-designed: the same generator emits **goal-contrast pairs** (world fixed, goal perturbed) and **world-contrast pairs** (goal fixed, world perturbed). Measure, per generated token: output-distribution movement (KL) under each contrast; per context segment: salience shift under each contrast. First coupling estimate = overlap mass (tokens/segments responsive to *both*) with the goal-directedness paper's cautions applied (multi-layer patching only, nonlinear probes beside linear, explicit OOD split across task families, conformal intervals on probe outputs). This runs on the trunk rig plus contrast-pair generation — no new instrumentation — but is listed last because interpretation leans on Exp 1's calibration.

## Sequencing and minimality

Phase 0 → Exp 1 is the critical path; 2 and 3 branch off the same rig cheaply; 4 reuses everything. The deliberately-unhypothesized instrument from the founding dialog (run real estate transcripts through the instrumented model and *watch*) becomes nearly free once Phase 0 exists, and belongs interleaved with Exp 1, not after it — the anomalies are where the next hypotheses come from.

Single GPU (46–80GB class) suffices throughout: 7-8B models, T1 signals are forward-pass residue, T2 is five heads, T3 is offline batch work — the batch-size-1 idle-compute regime this program is structurally advantaged in. No training runs anywhere in Phase 0–3; the first training decision (SASM trigger) is gated on Exp 3's result.

## What this plan does not cover (named, not written around)

- The ASF-side derivation of idealized salience-decay distributions for P1's comparison — theory work, different desk, and the instrument doesn't wait on it.
- Cross-architecture replication beyond the two named models (a later robustness pass).
- Latent-reasoning models (Coconut-family) — the measurement problem is strictly harder there; noted in the dossier as a field-level tension, out of scope here.
- Any claim about frozen-weights vs trained comparisons beyond the HIPIF-vs-base opportunity noted in the readings — that comparison needs HIPIF's checkpoints or a retrain, and is flagged as valuable, unscoped.
