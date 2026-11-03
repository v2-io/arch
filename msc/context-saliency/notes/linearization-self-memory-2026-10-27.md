# Linearization, causal memory, and the coherent self: notes for later

*2026-10-27. Joseph with Claude (Opus 5.5), from the memory-tutorial session. A companion to `memory-synthesis-boundaries-timescales-2026-10-27.md`.*

> **Thesis of the program: honestly forgetting.** How do you honestly give memory a saliency and recall gradient without compromising causality or truth? On the hypothesis below, the same honesty conditions that make forgetting safe are also what keep a coherent self possible.

*Labels follow that note. Status: established elsewhere / cross-field / frontier / practice / synthesis. Basis: searched / training (from model knowledge only, unchecked). Each label carries a confidence. Every literature citation here is from training only and needs checking before reuse.*

## 1. The hypothesis (Joseph)

1. **One causal stream.** Much processing happens in parallel subsystems and networks. Some of it is deep, but none of it has agency of its own. A single orchestrator is the only gatekeeper for external actions (for an LLM, its words). That orchestrator's channels and memory are strictly causal, and the subsystems' outputs reach it as inputs and interruptions.
2. **Consciousness is the linearization.** What we call consciousness is the decided acts that take effect in the world, together with the *linearization* of this ambient parallel processing into that one causal stream.
3. **The self is what can be remembered.** Consciousness is what can be *remembered as a coherent self*, provided memory works.
4. **LLM training is a different kind of learning.** It resembles slow, semantic, instinct-like learning: it doesn't necessarily change schemas, and it is distinct from experience.
5. **For any agent,** a coherent understanding of how it functions as an agent requires a coherent picture of itself within its sphere of agency. That picture is the closest thing to ground truth it has.

## 2. Where it sits in the literature (training only)

| Theory | Shared with the hypothesis | Difference |
|---|---|---|
| Global Workspace Theory (Baars 1988; Dehaene's global neuronal workspace) | Parallel specialist processes without agency of their own; a limited serial workspace whose contents are broadcast and guide action | GWT stresses broadcast and access, not causal order or memory |
| Multiple drafts (Dennett 1991) | Parallel drafts; the self as a narrative centre of gravity | Dennett denies a single privileged stream; Joseph posits one gatekeeper |
| Autonoetic consciousness (Tulving 1985); "consciousness as a memory system" (Budson, Richman & Kensinger 2022) | Consciousness tied to episodic memory | Joseph makes causal order the defining property of that memory |
| The interpreter (Gazzaniga; split-brain studies) | A system that builds a coherent causal story of one's own actions, and confabulates when it lacks access | Joseph's account predicts *when* this happens: when the orchestrator has no memory of its own causal stream |
| Telling more than we can know (Nisbett & Wilson 1977); choice blindness (Johansson & Hall 2005) | People give confident reasons that didn't cause their choices | — |

The rival theories are unsettled. The Cogitate adversarial collaboration tested Global Workspace Theory against Integrated Information Theory and confirmed neither cleanly. *Training · ~70%.*

**What the hypothesis adds** (synthesis):
- **Total-order broadcast as the formal core.** The workspace imposes one total order on events that are otherwise only partially ordered (Lamport). What is "conscious" is what was linearized into the causally ordered stream that memory keeps. *~60% that this sharpens GWT usefully.*
- **Memory integrity as the condition.** A coherent self requires that this causal memory persists and is not falsified: no reordering, no fabricated completeness. That makes memory architecture, including compaction, directly relevant to the hypothesis.

## 3. Observation: LLMs confabulate once their own thinking is gone (Joseph)

When an LLM's earlier reasoning is removed from its context, it reconstructs plausible explanations of what it did. It does this from the causal turn structure, with near-full certainty and in hindsight. When shown what it actually thought, it is often highly surprised by the difference.

- **The human version is the interpreter.** Humans have lossy memory; LLMs that lose their reasoning have *none*. Both show the same signature: a confident causal story built from the outside.
- **Related LLM work** (training, ~70% that these exist roughly as described):
  - Stated chain-of-thought is often unfaithful to what actually drove the answer (Turpin et al. 2023; Anthropic 2025, "Reasoning models don't always say what they think").
  - Anthropic's 2025 work on limited, unreliable introspective awareness in LLMs.
- **The AAT link (synthesis, ~70%).** An agent's own past reasoning is an intermediate node of its strategy. If it isn't observable, credit assignment over one's own decisions becomes impossible. This is observability dominance: "cannot learn, and cannot recognize that it cannot learn". **Confabulation is what credit assignment looks like when one's own reasoning can't be observed.**
- **Practice** (Joseph's direct observation, the strongest available basis): harnesses commonly discard prior reasoning at compaction, and editing earlier context can discard encrypted reasoning.

## 4. Predictions and experiments

| # | Experiment | Prediction under the hypothesis | Where |
|---|---|---|---|
| L1 | **Why did you do X?** After N turns, ask the agent why it took an earlier action. Compare three arms: (a) its prior reasoning retained, (b) reasoning stripped, (c) reasoning stripped but a short visible decision note kept. Score faithfulness against the actual reasoning, along with confidence and surprise when shown the truth. | (b) is confident and unfaithful. (c) recovers much of (a) at a fraction of the cost. | harness replay (`summary-replay/`) |
| L2 | **Influence without ownership.** A subagent's result is injected (i) as a tool output the orchestrator reads into its stream, (ii) silently into the system context. Later, ask what drove the decision. | (ii) influences behaviour but is disowned or confabulated about; (i) is owned. | middle-fold lab |
| L3 | **Order-corrupted memory.** Folds that are honest about order vs folds that violate the convexity and acyclicity condition (§7 of the companion note), at the same size. | Order violations cause more causal-attribution errors and more confident wrong explanations than equal-size honest folds. | middle-fold lab |
| L4 | **Self-model accuracy.** Have the agent predict its own next actions, its error rate, and what it will need, with and without causal memory of its own reasoning. | Self-prediction calibration drops without that memory, more than task accuracy does. | any long session |
| L5 | **Recurring thoughts made observable.** Log hunches and decisions in a visible channel; estimate recurrence with a multi-timescale need estimator; surface them when they recur. | Thoughts that recur across sessions get kept; a human partner rates the recurring ones as the ones that matter. | personal notes, agent-core |
| L6 | **Experience vs being told.** The same knowledge given (i) as a loaded skill or note, (ii) through attempts with feedback. Test on transfer to novel variants and on higher-order surprisal (does the agent notice strategy-level anomalies?). | (ii) beats (i) on transfer and on strategy-level surprise, even when (i) scores equal on recall. Mechanism: error-driven learning needs one's own predictions (generation and testing effects; Rescorla–Wagner). | agent-core |

## 5. Links to the tutorial and the companion note

- **Part 5 (CLS) and §4 above.** LLM pre-training resembles the slow neocortical learner, and in-context experience the fast store. Joseph's point that experience differs from loaded context is the old fast-versus-slow distinction plus error-driven learning.
- **12b and E18 (provenance and reality monitoring).** Confabulation is a failure of source monitoring: a reconstruction gets recorded as a memory. Provenance tags on one's own past reasoning (observed: "I thought this then"; inferred: "I probably thought this") would make the difference visible.
- **The companion note's §7 (honest folds keep causal order) and §1 (no false completeness)** are, on this hypothesis, conditions for a coherent self, not just for efficiency.
