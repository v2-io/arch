# Territory sweep: Attention-level and mechanistic salience

Probe date 2026-08-28. Everything below is from live web search (WebSearch/WebFetch)
this session unless flagged "[prior knowledge, unverified this session]". Several
arXiv IDs returned by search (e.g. 2606.xxxxx, 2601.xxxxx, 2603.xxxxx) imply
mid/late-2026 postings — consistent with "today is 2026-08-28" — so treat
numbering as legitimate rather than a hallucination artifact, but I could not
independently confirm arXiv's ID-year convention still holds; flagged where a
fetch actually rendered content vs. only search-snippet.

**Coverage caveat up front:** WebFetch on two of three deep-dive targets returned
model-paraphrased summaries with fairly generic methodology language ("multiple
controls," "rigorous," "statistically significant") rather than the paper's own
sentences — one fetch (OpenReview) failed outright (login wall). So the two
RoPE/attention-basin summaries below should be read as directionally reliable but
NOT as verbatim-verified methodology; a synthesis agent that wants exact numbers
should re-fetch the arXiv HTML/abstract pages directly, not OpenReview.

---

## 1. Retrieval heads (Wu et al. 2024) — current standing and follow-ups

Original claim [prior knowledge, consistent with search]: Wu et al. identified a
sparse, universal-ish subset of attention heads ("retrieval heads") responsible
for copy-paste behavior in needle-in-a-haystack-style long-context retrieval —
intrinsic (emerge early in pretraining, persist through further training),
sparse (~5% of heads), and causally important (masking them collapses
long-context retrieval/factuality).

Follow-ups found this session, roughly newest framing first:

- **"Retrieval Heads are Dynamic"** (ACL 2026, aclanthology.org/2026.acl-long.715)
  — directly challenges/extends the "intrinsic and static" framing: heads doing
  copy-paste retrieval are not a fixed set but shift with context/task, found by
  aggregating attention patterns across many probes rather than one static head
  map. **This is a notable update on my prior framing** — if Wu et al.'s original
  claim was "same ~5% of heads regardless of task," this paper's title alone
  signals that claim doesn't fully hold up under broader probing. Worth Joseph
  re-reading directly; I only have the title + abstract-level search snippet, not
  the full method.

- **Query-Focused Retrieval Heads (QRHead)** — Zhang et al., EMNLP 2025
  (aclanthology.org/2025.emnlp-main.1214, arXiv:2506.09944). Methodology
  signal: instead of Wu et al.'s copy-paste/NIAH signal for identifying retrieval
  heads, this paper uses a *query-outputs-contrast* method — heads selected for
  actually differentiating between relevant/irrelevant content under a real query,
  not just repeating literal strings. Built "QRRetriever" — use these heads'
  attention scores directly as a retrieval/reranking signal, reporting >10% gain
  over full-context baselines on multi-hop reasoning. This is directly relevant
  to Joseph's application (2) directed-separation and (4) SASM: it's evidence
  that retrieval-head attention scores are usable online as a cheap relevance
  signal, not just a post-hoc interpretability artifact.

- **"Does RoPE Prevent or Degrade Retrieval Heads? A Mechanistic Analysis Across
  Model Families"** (arXiv:2606.21249). Methodology (per WebFetch summary,
  moderate confidence): cross-family analysis spanning Llama 2/3, Qwen, DeepSeek,
  OLMo. Central claim: RoPE's rotational position encoding actively degrades
  formation/stability of retrieval-head attention patterns at long distances —
  i.e., the positional encoding scheme itself is a causal factor in *how well*
  retrieval heads work, not just a neutral background. Controls reportedly
  included ablations isolating RoPE from other architecture components and
  Benjamini-Hochberg correction for multiple comparisons across many
  head/layer tests. **This is directly load-bearing for Joseph's salience work**
  if true: it means a heatmap/salience measurement built on attention weights
  will behave differently (weaker signal, more noise) at longer relative
  distances purely as an artifact of RoPE, independent of true content
  relevance — a confound to control for explicitly. I could not verify exact
  numbers; recommend a direct read of the arXiv HTML before relying on it.

- **"From Interpretability to Performance: Optimizing Retrieval Heads for
  Long-Context Language Models"** (arXiv:2601.11020) — title suggests using
  retrieval-head identification as an engineering lever (e.g. weighting/boosting
  those heads or protecting them under quantization/pruning) rather than pure
  interpretability. Not fetched in depth; flag as a lead for "principled memory
  consolidation" (application 3) — if certain heads are known load-bearing for
  retrieval, that's a natural place to anchor compaction/eviction decisions
  (protect KV entries these heads attend to; summarize/evict what they don't).

- **SEAL** (arXiv:2501.15225, "Scaling to Emphasize Attention for Long-Context
  Retrieval") and **LycheeDecode** (arXiv:2602.04541, hybrid-head sparse
  decoding) — both treat the retrieval-vs-"streaming"/local-attention head
  distinction as an engineering handle: identify which heads are doing
  long-range retrieval vs. local/streaming processing, then allocate KV-cache
  budget or compute asymmetrically. This retrieval/streaming head bifurcation
  recurs across multiple 2025-26 papers as a stable, reusable category —
  seems more load-bearing empirically than any single head-identification
  method.

**Cross-model layer findings (from search synthesis, moderate confidence,
sourced against a table-understanding paper — arXiv:2603.15402 — that reused
retrieval-head identification as a diagnostic):**
Retrieval heads cluster in **middle-to-late layers**, but exact layer indices are
model-specific, not universal: **Qwen2.5-7B ~ layers 19–27**; **Llama-3.1-8B ~
layers 14–15 and 26–28** (two separate clusters, not one contiguous band). Only
~10–15 heads carry most of the retrieval signal; re-ranking accuracy peaks
around the top-10 heads and degrades adding more. The paper also reports that
retrieval-head location correlates with a proxy "attention norm intensity"
measure that persists even after task-specific fine-tuning — i.e. fine-tuning
seems to reuse pre-existing retrieval circuitry rather than growing new heads.
**This is a genuinely useful, fairly specific empirical anchor** for Joseph's
work if he wants to instrument a live open-weights Llama/Qwen model: it gives
concrete starting layer ranges to look at rather than scanning the whole model
blind. I have not independently verified these exact layer numbers against the
primary source — flagged for direct verification before being treated as fact
in a methods section.

---

## 2. Attention sinks

Good coverage found; this territory looks more mature/converged than retrieval
heads.

- **Survey: "Attention Sink in Transformers: A Survey on Utilization,
  Interpretation, and Mitigation"** (arXiv:2604.10098, also on HF papers) —
  useful as an entry point; frames the field's timeline as roughly: 2024 =
  discovery + causes; 2025 = mechanistic explanation; 2025-26 = strategic
  mitigation (structural fixes rather than just working around the phenomenon).

- **Mechanistic account, GPT-2 scale**: "A Mechanistic Account of Attention
  Sinks in GPT-2: One Circuit, Broader Implications for Mitigation"
  (arXiv:2604.14722) — claims a single identifiable circuit is responsible,
  which is a stronger/cleaner claim than the earlier "it's just softmax needing
  somewhere to dump unneeded attention mass" folk explanation. Not fetched in
  full; worth a direct read if Joseph wants circuit-level grounding rather than
  phenomenological description.

- **"Catch, tag, release" framing** (OpenReview, r8UWp9JeJi) — sinks
  functionally act as a mechanism where the model "catches" attention onto
  low-semantic tokens (BOS, punctuation), effectively parking/buffering
  representational capacity there, then "releases" it later — proposed as
  explaining why sinks are causally necessary for good performance under KV
  caching/compression rather than being a harmless artifact.

- **Mechanism decomposition** (search synthesis): sinks traced to (a)
  active-dormant attention-head switching and (b) mutual reinforcement between
  attention logits and value-state suppression — i.e., two interacting
  phenomena, not one. **This nuance (two interacting mechanisms, not a single
  cause) is worth flagging** since a simpler "it's just an artifact of softmax
  normalization" story is common in less rigorous treatments and apparently
  now considered incomplete.

- **Structural/architectural framing**: "The Structural Origin of Attention
  Sink: Variance Discrepancy, Super Neurons, and Dimension Disparity"
  (arXiv:2605.06611) and "The Spike, the Sparse and the Sink: Anatomy of
  Massive Activations and Attention Sinks" (arXiv:2603.05498) — both tie
  attention sinks to a *separate* well-known phenomenon, "massive
  activations"/outlier feature dimensions in residual streams. This is a
  connection worth Joseph knowing about even outside the sink literature
  narrowly: if his salience heatmap is built on raw attention weights, massive-
  activation dimensions may distort the signal independent of true relevance —
  another confound alongside the RoPE-distance one above.

- Relevance to Joseph's SASM idea (application 4): sinks being "necessary
  under KV-cache/compression" is directly relevant — any compaction/eviction
  scheme has to preserve sink tokens or risk collapse, a constraint largely
  orthogonal to but interacting with salience-driven eviction.

---

## 3. Lost-in-the-middle / positional salience, 2025-2026 state

This subfield has moved noticeably past the original "U-shaped curve, middle is
bad" finding — the newer work complicates rather than confirms the simple
story. Flagging this as a genuine **surprise relative to the framing I'd
default to**: "lost in the middle" is often cited as settled folk wisdom, but
current papers are actively contesting *when* and *why* it holds.

- **Attention Basin** (Yi et al., ACL 2026, arXiv:2508.05128,
  aclanthology.org/2026.acl-long.1198) — reframes the phenomenon specifically
  for *structured* sequences of discrete items (e.g., retrieved documents,
  few-shot examples), not raw prose context: attention weight, plotted against
  item position, forms a literal "basin" shape — high at the edges, low in the
  middle — and this shape is reportedly most pronounced at *shallow* layers,
  with the paper's causal claim being that shallow-layer attention allocation
  is what determines which positions matter downstream (i.e. the effect may be
  set early in the layer stack, not accumulated). My WebFetch summary of the
  PDF was fairly generic ("early/middle/late layer patterns") rather than
  giving exact layer indices or the quantitative basin depth — recommend
  re-fetching the arXiv HTML abstract+intro directly for numbers before citing
  specifics. **Directly relevant to Joseph's application (1)**: if "healthy"
  salience decay in his Strategy-DAG framing is supposed to look like
  monotonic/graceful forgetting of completed subgoals, the attention-basin
  literature suggests real models don't decay smoothly at all — they have a
  U-shaped/basin structure driven by shallow layers, which is a different
  shape than "decay." Worth distinguishing "healthy forgetting" from "basin
  artifact" empirically rather than assuming they're the same curve.

- **Context-window-size dependence** (Veseli et al. 2025a, per search
  synthesis, exact venue/arXiv id not captured in search snippets — recommend
  a follow-up search "Veseli 2025 primacy recency context window" to pin down):
  reported finding that recency vs. primacy dominance *flips* depending on what
  fraction of the context window the relevant span occupies — primacy
  dominates when relevant content is ≤50% of the window, recency dominates as
  it grows. If accurate, this means "lost in the middle" is not a fixed
  phenomenon but interacts with span-length-relative-to-window, which is a
  methodologically important control variable Joseph would want in any of his
  own salience experiments (report relevant-span-as-fraction-of-window, not
  just absolute position).

- **Mitigation attempts**: "Found in the Middle: Calibrating Positional
  Attention Bias" (arXiv:2406.16008, note: this is a 2024 paper, older than
  the "2025-26" ask but the throughline into calibration approaches);
  "Pause-Tuning for Long-Context Comprehension" (2025, lightweight attention
  recalibration via inserted pause tokens); Multi-scale Positional Encoding
  (Ms-PoE). Search-level consensus as of 2026: **no production model has fully
  eliminated position bias** — mitigations reduce but don't remove it without
  retraining.

- **New variant, not the classic shape**: "Lost at the End: Primacy Bias in
  Multimodal Retrieval-Augmented QA" (arXiv:2606.16494) — reports a *primacy*
  bias (favoring beginning, not end) specifically for multimodal RAG, i.e. the
  direction of the bias itself is not universal across modalities/task setups.
  Worth noting as a caution against assuming "recency wins" or "primacy wins"
  as a fixed law — it's task/modality contingent.

- Cross-cutting methodological point worth surfacing to Joseph explicitly:
  multiple of these papers (attention basin, primacy/recency window-fraction
  paper) are measuring position bias via *black-box behavioral* probes
  (does accuracy degrade by position) rather than *attention-weight* probes
  directly — i.e., "lost in the middle" as originally stated is a behavioral/
  output phenomenon, and only some of the follow-ups connect it mechanistically
  to attention weights at all. That gap (behavioral vs. mechanistic evidence)
  is exactly the kind of thing Joseph's proposed salience heatmap could help
  close, and it doesn't look fully closed by anyone yet in what I found.

---

## 4. "Attention is not explanation" — current state of the debate

This is the most settled/least-moving of the four territories — genuinely a
dry-ish well for *new* argument, though there's new nuance worth reporting.

- Core positions unchanged from the 2019 exchange: Jain & Wallace 2019
  ("Attention is not Explanation," NAACL) argued attention weights don't
  reliably indicate feature importance (adversarially swappable attention
  patterns preserving output); Wiegreffe & Pinter 2019 ("Attention is not
  *not* Explanation," EMNLP) rebutted that some of Jain & Wallace's tests were
  themselves flawed and attention can carry real signal in certain diagnostic
  setups. [prior knowledge, confirmed still the reference frame via search]

- A 2025 HuggingFace blog post ("Is Attention Interpretable in
  Transformer-Based LLMs? Let's Unpack the Hype") is described in search
  results as landing on "attention matters, it's just not the full story" —
  i.e. the field seems to have settled into a middle position rather than
  either pole, treating attention as one signal among several (alongside
  gradients, ablations, causal tracing) rather than either dismissing it or
  treating it as sufficient alone.

- Most relevant methodological find for Joseph specifically: **"The Fragile
  Truth of Saliency: Improving LLM Input Attribution via Attention Bias
  Optimization"** (OpenReview, Sept 2025; I could not get past OpenReview's
  login wall via WebFetch, so this is search-snippet only, not verified
  primary-source). Its stress-test design directly targets the kind of
  heatmap Joseph wants to build: a needle-in-a-haystack-based adversarial
  test showing that *existing* input-saliency methods (raw attention,
  presumably also gradient-based ones) **consistently misattribute importance
  to irrelevant context, and get worse as input length grows** — i.e. the
  naive "heatmap of prior context contribution" approach has a known failure
  mode that gets *worse* exactly in the long-context regime Joseph cares about.
  Their fix, "Attention Bias Optimization" (ABO), reportedly works by directly
  optimizing an additive bias term on attention (rather than reading raw
  attention weights) to make the resulting bias's magnitude causally track
  actual influence on the target token — closer to a causal/interventional
  attribution method than a purely observational one. **This paper is a strong
  candidate for the synthesis agent to flag as required reading** given
  Joseph's explicit goal of a salience heatmap — it's the most on-target single
  methodology find in this whole sweep, but I was unable to verify it beyond
  the search snippet; strongly recommend Joseph or a follow-up fetch get the
  actual arXiv/OpenReview PDF (try arXiv directly rather than OpenReview,
  which walled the fetch).

- General 2025-2026 trend confirmed across multiple hits: **causal/
  interventional methods (ablation, patching, contrastive perturbation) are
  increasingly preferred over raw-attention-weight reading** as the credible
  standard, which directly validates the framing in Joseph's application (2)
  ("directed separation via contrastive perturbation and probing") — that's
  the methodologically favored direction in the current literature, not a
  dated approach.

---

## 5. Attention rollout / attention flow — current standing

Lower priority territory but covered per the brief.

- Still actively used and extended, primarily in **vision transformer**
  interpretability (GMAR — Gradient-Driven Multi-Head Attention Rollout,
  arXiv:2504.19414; RFEM with class-specific filtering, Oct 2025) more than in
  LLM/language interpretability specifically — most hits skew vision-domain.
  I did not find strong evidence of rollout/flow being a live, preferred
  method in long-context LLM salience work specifically; it looks more like a
  legacy method being incrementally patched (gradient-weighted variants) than
  a growth area.

- A 2026 preprint, **"Interior interpretability with attention rollout:
  contraction and propagation profiles in Transformers"** (arXiv:2607.22367)
  is the most LLM/transformer-general hit, but I did not fetch it in depth —
  flagging as a lead only.

- Standing criticism (consistent across sources, matches prior knowledge):
  rollout only combines attention weights layer-to-layer and ignores value
  projections, output projections, LayerNorm, and MLP transformations — so it
  is fundamentally an incomplete picture of information flow, and known to
  encode a "combinatorial shortcut bias" where jointly-trained attention
  weights carry information exploited by downstream heads in ways not
  captured by the rollout math itself.

- **Takeaway for Joseph**: rollout/flow does not look like the modern
  foundation to build a salience heatmap on — the field has moved toward (a)
  identifying specific functional heads (retrieval heads, sink heads) rather
  than aggregating all heads via rollout, and (b) causal/interventional
  attribution (ABO-style, ablation, ordinary probing) over pure attention-
  weight propagation. This is a genuine, mildly surprising directional
  finding: the "heatmap of contribution" intuition maps much more naturally
  onto the causal-attribution literature (§4) and the retrieval-head
  literature (§1) than onto rollout/flow, which I'd have expected to be the
  more obvious starting point given its name literally describes flow/heatmap
  visualization.

---

## Cross-cutting synthesis notes (my own read, not sourced from any single paper)

1. **Two confounds recur across sections 1 and 2** that any salience-heatmap
   design should control for explicitly: (a) RoPE-induced distance decay of
   retrieval-head attention quality (§1, arXiv:2606.21249, moderate
   confidence), and (b) massive-activation/outlier-dimension distortion of raw
   attention weights tied to sink formation (§2). Both mean "raw attention
   weight at position i" is not a clean salience signal on its own — this
   matches and reinforces the "Fragile Truth of Saliency" paper's stress-test
   finding in §4 from an independent angle (mechanistic vs. behavioral-testing
   evidence converging on the same conclusion).

2. **Retrieval-head identification and streaming/local-head identification
   (§1) is the most mature, reusable, and quantitatively specific handle**
   found in this sweep — concrete layer ranges for Llama-3.1-8B and
   Qwen2.5-7B, a stable retrieval-vs-streaming bifurcation across multiple
   independent 2025-26 papers, and at least one paper (QRHead) showing these
   heads' attention scores are usable online as a live relevance signal, not
   just post-hoc. This looks like the strongest existing scaffold to build
   Joseph's SASM idea (application 4) on top of, rather than starting from
   generic attention-weight heatmaps.

3. **"Lost in the middle" as a fixed universal law does not survive 2025-26
   scrutiny** — direction (primacy vs recency), magnitude, and even which
   layers cause it are now reported as contingent on window-fill-fraction,
   modality, and shallow-vs-deep layer, per §3. If Joseph's Strategy-DAG
   application treats "healthy decay of the forgotten middle" as analogous to
   the classic lost-in-the-middle curve, that analogy needs to be checked
   against the newer, more contingent picture rather than the original 2023
   framing.

4. Genuinely unexpected finding for the whole sweep: the causal/
   interventional-attribution direction (§4, ABO) and the recently-contested
   "static vs dynamic retrieval heads" question (§1, ACL 2026 paper) both
   suggest the field's center of gravity has shifted *away* from "read
   attention weights and call it salience" toward "intervene and measure
   effect" as the credible standard — which is good validation for Joseph's
   stated plan (contrastive perturbation, application 2) but means any
   *observational* heatmap component of his SASM/consolidation work should be
   framed explicitly as a cheaper proxy to be validated against interventional
   ground truth, not as ground truth itself.

## What I did NOT cover / could not verify

- Did not get primary-source, page-and-number-level detail on: Attention
  Basin's exact quantitative basin depth/shape; the RoPE mechanistic paper's
  exact effect sizes; the Fragile Truth of Saliency paper's ABO method
  internals (blocked by OpenReview login wall — a direct arXiv search was not
  attempted before time ran out on this sweep, and should be the first
  follow-up).
  arXiv search for "Fragile Truth of Saliency" arXiv id was not run — worth a
  quick follow-up.
- Did not search DeepSeek's own technical reports / Qwen3 technical report for
  self-reported retrieval-head or attention-sink findings (model-vendor
  primary sources vs. third-party interpretability papers) — this is a real
  gap; vendor technical reports sometimes contain their own internal
  ablations not otherwise published.
- Did not chase the "Veseli et al. 2025a" citation to a real venue/arXiv id —
  flagged inline above, worth 5 minutes of follow-up search.
- No hands-on verification of any of this against an actual open-weights model
  — everything here is literature-only, consistent with the "probe" framing of
  this task.
