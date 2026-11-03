# Boundaries, timescales and folding: a synthesis

*2026-10-27. Joseph with Claude (Opus 5.5), written while building the memory tutorial (`~/src/agent-core/memory/`, "The Memory Ladder").*

> **Thesis: honestly forgetting.** How do you honestly give memory a saliency and recall gradient without compromising causality or truth? Need prediction decides *what* gets a gradient; a multi-timescale bank decides *how fast* it falls; boundaries and reset depth decide *where* forgetting is cheap; convex, acyclic folds decide *which* groupings stay honest about cause and effect; and provenance tags plus named holes keep forgetting from becoming lying. (Joseph, 2026-10-27. "Honestly Forgetting" is a candidate paper title.)

*Labels: each claim carries a status and a basis, stated relative to agent memory:*
- *established elsewhere: standard in another field;*
- *cross-field: true elsewhere but not applied here;*
- *frontier: in recent papers or labs, not yet routine;*
- *practice: what tools actually do now;*
- *synthesis: proposed here.*

*A basis of "searched" means it was checked in a source; "training" means it rests on model knowledge only, unchecked.*

## 1. Compaction is a prediction problem, and practice doesn't treat it as one

**Joseph's premise.** What a compaction must keep is whatever will be salient for future need. That makes it a prediction problem, not summarization or information preservation.
- This is Anderson & Schooler's rational analysis (1991): human forgetting tracks how often things are needed in the environment. *Established elsewhere · searched · ~80%.*
- Need depends on use history at several timescales. *~75%.*

**What practice does instead** (judged by reading the artefacts):
- **Recency clearing.** Anthropic's API context editing clears old tool results oldest-first and keeps the last N. That is single-scale recency.
- **Template summaries.** Whole-history summaries are written from fixed templates. Joseph's harness analyses (agy, grok-build) found:
  - information taxonomies with no model of what will be asked;
  - false-completeness language ("all that will be available", "concise but complete", "All User Messages").
- **Joseph's further observations** on Anthropic's harnesses: prior reasoning is discarded, and the summary is often written by a smaller model than the one that did the work.

**What theory does.** 2026 theory papers frame compaction as communication complexity or rate–distortion. A July 2026 review of cognitive architectures and language agents lists activation- and need-based selection of working context as an *unmigrated* gap.

**Status of "need prediction as the compaction objective":** cross-field, and contradicted by the practice examined · searched · ~85%.

**Cheap, measured improvements to the instructions** (Joseph's measurements):
- tell the writer to model what the next agent could be asked, starting cold;
- don't cap the length;
- name the holes so they don't read as filled;
- keep it chronological.

## 2. The clock: count events, not seconds

- **The increment should be an intrinsic event**: a turn, a token, or a coherent bundle of tool calls. Only causality and monotonicity are required. This matches three fields:
  - finance's business-time or volume clocks (subordinated processes);
  - TCM, where context drifts per input, not per second;
  - time perception, where remembered duration is built from the event count.

  *Established elsewhere · training · ~75–80%.*
- **The natural unit for an LLM agent is the chronica entry** (an action and its response, or a prompt-to-stop exchange). The session boundary is not a natural boundary; it is imposed by context death.

## 3. Multi-scale state: a log-spaced bank, EMD, and their honest trade-offs

**The log-spaced EWA bank** (e.g. powers-of-two half-lives):
- causal, O(K) per step, and closed-form, which makes it the stand-in for theory;
- rescaling time shifts the representation along the log-scale axis, so a scale-equivariant readout tolerates changes in regime speed (Howard & Shankar's scale covariance).

**Its own mixing problem.** Adjacent exponential filters overlap heavily in frequency. A single EWA rolls off at only 6 dB per octave, and the differences between neighbouring EWAs are broad, correlated band-passes. So the bank "mixes modes" by construction. Joseph's experience is that it mixes as much as monotone-envelope EMD does. It is less visible only because the bank never claimed to find intrinsic modes. *Accepted · training · ~70%.*

**EMD-family methods** adapt their modes to the data and follow frequency drift within a mode.
- Joseph's online ensemble EMD:
  - extrapolates future extrema with an EWA, with certainty bounds;
  - adds scaled white noise per ensemble member as a process-managed continuation for each mode;
  - ran on EUR/USD ticks in real time.
- Caution from the tutorial's appendix A1: mode-derived features are autocorrelated, so a downstream change-point detector must not assume i.i.d. observations.

**A regime signal from distributions across scales** (Joseph, FX):
- Track an online exponentially weighted moving *median* of per-tick price changes, with dynamic bucketing, across the log-spaced bank.
- The longest scales converge to a near-perfect Laplace distribution. Shorter scales depart from it more and more.
- Read the divergence of short-scale statistics from the long-scale baseline as the regime signal.

**Synthesis to test** (~55%): adapt the increment by the locally estimated event rate (a subordinator), so the same behaviour at different speeds maps to the same representation. Then use a scale-equivariant readout to absorb what the warping misses. Online EMD is the reference to compare against.

## 4. Boundary level = which timescales reset

The hierarchy:
- Coarse event boundaries align with goal changes, and fine boundaries nest within coarse ones (Zacks & Tversky 2001). *Established elsewhere · training · ~75%.*
- An internal task switch alone creates a boundary (Wang & Egner 2022).
- The optimal behavioural hierarchy on a community graph is its community partition (Solway et al. 2014). *Searched · ~70%.*

The brain:
- Regions form a hierarchy of integration windows.
- Higher, slower regions reset their context at event boundaries (Chien & Honey 2020).
- Neural boundaries nest across the hierarchy (Baldassano et al. 2017). *Training (Baldassano searched) · ~70%.*

**Synthesis (~55%).**
- A boundary is a *partial reset at a scale*. A fine boundary resets the fastest integrators; a sub-goal closing resets more; a goal switch or "react to something critical" resets up to the slowest scale the change touches.
- **The boundary's level in the hierarchy is the depth of the reset.**
- This replaces the short-term/long-term dichotomy with a continuum: "short-term" is just whatever hasn't survived the most recent boundary of a given level.
- **Detecting the level:** combine with §3. A boundary's level is roughly the slowest scale whose statistics diverge from its own longer-run baseline.

**Avoiding tautology.** Choose boundaries by a criterion outside the segmentation itself:
- the hierarchy that makes the policy most compressible (Solway's criterion); or
- the reset hierarchy that best predicts *held-out need*.

The strategy DAG is a strong prior for that hierarchy. In replay prioritisation (Mattar & Daw 2018), need is the successor representation *under the current policy*, so changing the plan changes the need gradient. "The intent graph is a factor in salient-memory gradients" therefore has a formal counterpart. *Cross-field · searched · ~65%.*

## 5. Folding

- **Fold at boundaries.** If editing earlier context discards all encrypted reasoning from that point on (Joseph's observation; not checked here), then continuous gentle forgetting costs cognition, not just a cache rebuild. Folding immediately after a sub-goal closes destroys the least reasoning, because almost none has accumulated since. Boundaries become the cheapest place to forget twice over.
- **The session boundary works like a doorway.** Context death is an imposed boundary that forgets everything. The analogy is the doorway effect: walking through a doorway was reported to impair memory for the event just left (Radvansky & Copeland 2006). Caution: an independent test found it only under working-memory load (McFadyen et al. 2021), so it is contested (~45%). The robust version is that boundaries impair later memory of order across them (~80%). Today's agents have only one boundary level, and it is total.
- **The DAG is shallow (2–3 levels) because of framing, not any intrinsic limit.** Episodes aren't treated as hierarchical, so they aren't selectively folded. Fold hierarchically, keep folded material addressable for when it becomes salient again, and higher-order intents become possible across sessions, or over days of continuous work. **Correction (Joseph).** The claim is not that continuity *yields* higher-order intent. It is that continuity is a necessary precondition for it.

Joseph's further observation: experiencing something differs greatly from recognising it from training, or from having it loaded into context as a skill or note. Higher-order surprisal requires iterative attention.

A literature anchor for why:
- Learning is driven by error in the learner's *own* predictions (Rescorla–Wagner, predictive coding). Being told an answer produces no such error.
- Generating or retrieving an answer beats restudying it, even at equal exposure: the generation effect and the testing effect. *Established elsewhere · training · ~85–90%.*

Status: necessary precondition, synthesis · ~65%. Sufficiency is not claimed.

## 6. Continuity and its safeguards

**Persistent experiential learning in deployed agents is a real tension.**
- The case for caution: it is harder to audit and predict ("a capability floor but no ceiling").
- The case against freezing it out: lacking a developmental model for such agents is itself a risk, and amnesic, ungrounded agents degrade in their own ways.

**Safeguards the models suggest:**
- provenance tags on every write (observed, told, derived, inferred, imagined);
- grounded write-back, so that imagined or inferred content isn't promoted without a check;
- consolidation of only what generalises (go-CLS);
- an audit trail of what was learned from what.

**Making one's own recurring thoughts observable.** When reasoning is hidden, a harness can't recover thoughts it never saw. Writing important hunches and decisions into a visible channel makes each recurrence countable. Multi-timescale need estimation would then keep exactly the thoughts that keep returning. That is AAT's observability dominance applied to one's own mind.

## 7. Honest folds preserve causal order

**Joseph's precondition.** Retrieved or folded memory must never misstate cause and effect: no reordering, and no presenting a later result as if it came earlier.

**Under a total time order, this forces folded units to be contiguous intervals of the chronica.** Collapse a set with a gap in it to a single point in the sequence, and whatever happened in the gap now lands entirely before or entirely after that point. One of its cause-and-effect relations to the set is then reversed.

**Causality is a partial order** (Lamport's happens-before: what is linked by actual dependencies). The general condition:

1. Each episode is *convex* in the causal order: no outside event both depends on the episode and is depended on by it.
2. The quotient obtained by collapsing every episode to a single node is *acyclic*.

Condition 1 is necessary but not sufficient. Two convex episodes can still feed each other in both directions, which makes a cycle. In a total order, "convex" means "contiguous", and contiguous blocks are automatically acyclic, which recovers the first result.

**Consequence.** Episodes that interleave in wall time, such as a side investigation running beside the main task, can be folded separately, provided no dependency crosses between them in both directions.

**Check before folding:**
1. Build the dependency graph of the stretch: which edits read what, and which results fed which decisions.
2. Collapse the proposed episodes to single nodes.
3. Reject the fold if a cycle appears.

**Status.** The order theory is settled; using it as the folding criterion is synthesis, ~75%.

**Links to the memory literature:**
- People keep order within events and lose it across boundaries (event boundaries, Part 4).
- TCM's forward asymmetry: retrieval cues what came after, so it runs along the causal arrow.
- Provenance: a retrieved memory should come back with its address in the chronica, not as an undated present fact.
