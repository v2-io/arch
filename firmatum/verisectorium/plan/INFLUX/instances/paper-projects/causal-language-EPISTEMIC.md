<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: causal-language/EPISTEMIC.md (honest adversarial self-critique; not OUTLINE but epistemic discipline)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/causal-language/EPISTEMIC.md
  Do not edit here expecting to update the live original.
-->

# EPISTEMIC.md — Honest Adversarial Review

*An attempt at the strongest honest critique of the work, written from a perspective that would try to **break** the project rather than defend it — but without strawman attacks, misrepresentations, or critiques that are already known and queued in `OPEN-WORK.md`. Last updated 2026-05-14.*

*The point of this file: catch our own weaknesses before reviewers do. The strengthen-before-soften discipline applied to our own work means we should know exactly where it can be hit hardest. Some of these critiques can be addressed; some can't without substantially more work; a few may turn out to be unanswerable.*

*Ordered by severity, most damning first. Each item flags whether it's testable, what would address it, and current status.*

---

## Purpose and read order

If you're a reviewer thinking about citing this work or building on it: **read this file before any of the others** — including `HEADLINE-FINDINGS.md`. It is the honest map of what we don't know and where the work can be challenged.

If you're a future agent picking the work up: this is what the work looks like when its weaknesses are surfaced openly. Several items here are listed as *testable but untested* — those are good candidates for follow-on empirical work. Several are *not testable in our experimental paradigm* — those are philosophical-framing weaknesses that future paper-drafting needs to navigate, not empirical gaps.

If you're sketching out this work as part of a broader argument: the items in §1-§6 are the ones most likely to compound across the work. §1 (added 2026-05-14) is more methodologically foundational than the others — it concerns whether the empirical methodology is testing the right *kind* of thing, not just whether specific tests are tight. The items in §7-§15 are local to specific findings.

---

## §1. The methodology starts from phrasings, not from situations — possibly the wrong end of the testing chain

**The weakness** (flagged 2026-05-14 in conversation with Joseph): the entire empirical methodology constructs phrasings (minimal-pair sentences like "A caused B" / "without A, B would not have occurred"), embeds them, and tests whether the embedding-space geometry respects Pearl-semantic relations *between phrasings*. Ground truth lives at the linguistic-convention level — Pearl's formal semantics as oracle for what relations should hold between Pearl-marked sentences.

But the claim we're really trying to make is about what the embedding *encodes about causation in the world* — not just about what it respects of formal-semantic conventions between specially-constructed sentences. The deeper test inverts the methodology: start from **situations** (causal scenarios with known ground-truth structure — direct causal, confounded, mediated, spurious), generate diverse phrasings of each situation, and test whether the embedding represents the *situation* across diverse phrasings. Ground truth lives at the world-level causal structure; linguistic-form variation becomes a nuisance dimension the encoding should be invariant to.

**Joseph's framing** (2026-05-14, in conversation): *"the spirit of the knowledge should come before the phrase (or many many possible phrasings to neutralize syntactic variability or training corpus variability maybe...)"*

**What follows from this**: the current positive results are consistent with — and now better organized under — two distinct routes the originating spike already named:

**(C1 route — Theorem 1, discourse-act encoding)**: Pearl-Level-2 content encoded in the **syntactic position** of causal-marker arguments. "Without **X**, Y would not have occurred" vs "Without **Y**, X would not have occurred" differ only in which nonce word occupies which argument position. Non-reducible to Level-1 event-variable distributions by CHT. Token-shuffle on vocabulary-identical comparisons nulls this — confirmed across all 5 models by Spike S02/S03 and the token-shuffle control (2026-05-14). This is the paper's load-bearing claim and it is supported.

**(C2 route — Reichenbachian inheritance)**: Vocabulary-based distributional signal from training corpus. "Definitely caused" clusters near "entirely depended" because these word-pairs co-occur in causal-rich training text. Token-shuffle on vocabulary-different comparisons (E10 adverb cells: definitely/partially/minimally) partially survives — consistent with bag-of-words detection. This route is real but weaker in theoretical grounding (no CHT non-reducibility; RCCP has known counterexamples).

**The (a)/(b) framing above was the wrong dichotomy.** Both C1 and C2 are valid encoding routes; choosing between "situational" and "marker-convention" missed that the originating spike anticipated both. The right question per experiment is: which route dominates, and what does that imply?

**Operational test distinguishing C1 from C2**: Does the alignment claim depend on which token occupies which syntactic argument position, holding the rest of the vocabulary fixed? If yes → C1, token-shuffle nulls. If no → C2, token-shuffle may survive. This is now a runnable diagnostic for each existing experiment in E1-E13.

**C1-route detection degrades with competing topic content** (finding from concurrent spike `.archive/spikes_2026-05-14_situation_independent/`): nonce-Pearl minimal pairs (marker/arguments are all the content) → C1 detected cleanly. Rich natural prose (direction-reversal with topic-dense sentences) → C1 fails. The diagnostic works cleanest in marker-dominated probe sentences, which is exactly the minimal-pair design's condition.

**The current methodology cannot fully separate C1 from C2 effects without the token-shuffle diagnostic.** The minimal-pair design forces both routes to operate simultaneously.

This critique subsumes or sharpens several of the others below: §7 (semantic-field clustering — addressed by situation-anchored design when phrasings span semantic fields for the same situation); §10 (structural-vs-lexical fuzziness — situational invariance is a cleaner operational definition of "structural" than within-marker-class generalization); §11 (correlational-not-interventional — situational-invariance is still correlational, but the correlation is now between *situation* and *encoding*, which is closer to what "encoded content" actually means); §6 (hand-constructed pairs + narrow register — situations admit naturalistic phrasings across registers).

**What would address it** — situation-anchored phrasing-variation experiment. Concrete operationalization (rough first cut):

- Take ~20 controlled causal scenarios with explicit ground-truth structure. Classic Pearl examples (firing-squad; sprinkler-rain-wet-grass; smoking-tar-cancer; rooster-crow-sunrise as spurious-correlation control) plus everyday physical/social/temporal scenarios with known confounders.
- For each scenario, generate 10-15 diverse phrasings varying: register (technical / conversational / narrative / marketing-prose); syntactic form (active/passive, nominalized/clausal, simple/complex); vocabulary (causal verbs vs paraphrases vs unmarked descriptive); length; explicit-causal-marking (some phrasings should have NO causal markers — just describe what happened).
- Test:
  - **Within-scenario clustering tightness vs across-scenario** — do diverse phrasings of the same situation cluster tighter than diverse phrasings across situations?
  - **Confounded vs direct-causal distinguishability** — do descriptions of confounded scenarios (X ← Z → Y) cluster apart from direct-causal (X → Y) descriptions, even when surface text mentions X and Y prominently in both?
  - **Marker-free vs marker-bearing same-situation alignment** — do phrasings that describe the situation WITHOUT explicit causal markers still cluster with marker-bearing phrasings of the same situation?

If (i)-(iii) hold robustly across architectures, the encoding genuinely tracks situations (claim (a)). If they fail, the encoding tracks phrasings or marker conventions (claim (b)), and most existing positive results need reinterpretation as "the embedding respects Pearl-semantic conventions about marked sentences" rather than "the embedding encodes Pearl-causal content."

**Why this is the most fundamental experimental critique in the file**: most other critiques are about specific aspects of the methodology that could be tightened. This one is about whether the methodology is testing the right *kind* of thing. If it's not, tightening the existing tests won't fix the underlying ambiguity. The situation-anchored test is the cleanest single experiment available for revalidating or seriously reframing the empirical contribution.

This also bears directly on the (SC) concern in §3: situation-anchored phrasings naturally vary in (SC)-faithfulness (the marker-free descriptions are not making Pearl-Level-2 commitments; some hedged descriptions are weakly committed; explicit-causal descriptions are strongly committed). If the encoding clusters by situation across these (SC) gradients, (SC) is empirically vacuous in the methodology — which is exactly what §3 says formally but the situation-anchored test would *demonstrate* operationally.

**Status update — Spike S01 executed 2026-05-14 (archived 2026-05-15):** `.archive/spikes_2026-05-14_phrasing_vs_situation/experiment_s01_situation_anchored.py` ran Parts A/B/C across all 5 primary models; synthesis in `.archive/spikes_2026-05-14_phrasing_vs_situation/RESULTS-S01.md`. *(This spike and its parallel `situation_independent` were consolidated into the V2.1.1 E14–E19 synthesis — see Resolution below — and archived; cite E14–E19, not the spike.)* Key findings (each reproduced and strengthened by E15+E16):

- **For 4 of 5 models (embeddinggemma, nomic-v1.5, nomic-v2-moe, mxbai)**: the §1 critique is CONFIRMED in its strongest form. Marker-free descriptions are directionally neutral. The cooccur template ("A and B occurred.") also has a previously unknown surface-form confound: the entity listed last (B) is syntactically adjacent to "occurred," which matches cf_fwd structure (B adjacent to "would not have occurred"). A listing-order control ("B and A occurred.") produced significant REVERSAL in nomic-v1.5 (p=0.0007) and nomic-v2-moe (p=0.0001), confirming artifact. Encoding for these models is interpretation **(b): marker-convention depth only**.

- **For qwen3 (4096d)**: H_B1 (naturalistic marker-free descriptions) is **SIGNIFICANT (p=0.017, 10/12 correct)**. Naturalistic descriptions like "The pavement was bone dry; then the storm came through, and every low spot collected standing water." cluster with the correct-direction CF ("Without rainfall, puddles would not have formed.") despite completely different vocabulary — ruling out syntactic-proximity artifact. **For qwen3, world-knowledge encoding is real.** The listing-order control for cooccur is inconclusive at n=30 (p=0.175 for reversal, underpowered).

- **New surface-form confound documented**: listing-order in co-occurrence templates can produce spurious directional bias when CFs are in canonical order. Future probe design should include listing-order controls.

**Resolution (2026-05-15) — this critique is met by the V2.1.1 synthesis, not open.** §1 was the deepest *design* critique, and it was answered — not by the spike alone but by the V2.1.1 synthesis the two parallel spikes were consolidated into (per `STRENGTHENING-AND-SYNTHESIS.md` lines 3 / 437 / 449–453, which planned exactly this and declared OPEN-WORK §A.0 closed). The mapping is exact and *strengthens past* the spikes:

- **Situational / marker-free encoding (S01 Part B)** → **E15** (`experiment_15_real_entity_hd1_hd3.py`) adds the **H_D3 symmetry control S01 lacked**, isolating the world-knowledge-direction prior. Verified: qwen3's apparent situational signal is substantially that prior (ASYMMETRIC; H_D3 CI crosses zero), not deep situational encoding.
- **Marker-free condition** → **E16 L3b** strips *all* markers including the Level-1 temporal markers ("then", "after", "within minutes") that S01's H_B1 leaked — the exact confound the blueprint flagged S01 for. Strictly stronger than S01.
- **Direct-causal vs confounded (S01 Part C)** → **E16** `DIRECT_REAL` vs `CONFOUNDED_REAL` template-class clustering, bootstrap-CI'd, 12 models.

The verified panel answer: within-scenario clustering survives marker-stripping but collapses under entity-rotation with a substantial register-vocabulary share, and qwen3's signal is substantially a world-knowledge prior — i.e., predominantly vocabulary-co-occurrence / entity-recurrence ("bag"/C2), **not** deep situational structural encoding. The §1 (a)/(b) dichotomy is thereby resolved into the richer C1/C2/(bag) decomposition; the answer is the paper's deflationary finding. Both spikes are superseded and archived at `.archive/spikes_2026-05-14_situation_independent/` and `.archive/spikes_2026-05-14_phrasing_vs_situation/` (each with a provenance `README-archived.md`).

**The one genuine residual** (optional robustness, *not* a load-bearing open falsification): the *fullest naturalistic register-diversity* form of §A.0 — many rich-prose phrasings per situation across technical/conversational/narrative registers, vs E16's controlled templated phrasings — was run by neither. It is correctly minor: E16's controlled gradient isolates the mechanism *more cleanly* than naturalistic prose would (naturalistic register variation reintroduces the lexical/topical confounds the controlled design removes). Tracked in OPEN-WORK §A.0 as optional future robustness.

*(Historical note: a 2026-05-15 verify-before-archive pass first flagged `phrasing-vs-situation` "NOT superseded, do not archive," reached via a too-shallow token-grep of E14–E19 for S01's literal template names; deeper verification — blueprint intent + E16 template bodies + panel results — reversed that. Strengthen-before-soften: the synthesis met the deepest critique; the work is stronger than the cautious flag claimed.)*

A note on relationship to existing partial-versions: E3 (cross-marker direction) and E13 (cross-marker evidential) are primitive partial-versions of phrasing variation — they vary the marker within a tight syntactic frame. Cross-lingual (E8, E11) varies language across same-construction. The Spike S01 result is a substantive deepening: it establishes that (a) the situational interpretation is confirmed only for qwen3, (b) a new confound class is documented, and (c) the prior positive results describe marker-convention encoding for most models.

---

## §2. The IB-optimality bridge is plausibility, not derivation

**The weakness**: the theoretical chain runs spike Theorem 1 → C4 (causal-IB consequence) → "preserved in embedding geometry of frozen pretrained sentence embeddings." The bridge in the middle assumes these models are "approximately IB-optimal compressors" of natural-language data. They aren't, strictly. They're trained with contrastive losses (mxbai), MLM (BERT-derived), next-token prediction (decoder-derived), sentence-pair similarity (most multilingual variants). Tishby-Zaslavsky's IB interpretation of deep-network training is contested (Saxe et al. 2019; Goldfeld et al. 2019 found it holds for some but not all settings).

**What follows from this**: the empirical findings stand as descriptions of embedding-geometry behavior. The *theoretical explanation* for why they happen is post-hoc — we observe Pearl content, we have *a* story for why (IB), but the story isn't tight. A skeptical reviewer can fairly say the IB framing is decorative rather than load-bearing.

**What would address it**: either (a) demonstrate that the specific training objectives of the tested models converge to approximately IB-optimal solutions for natural-language data, or (b) reframe the theoretical link more weakly — perhaps as "any representation-learning objective that minimizes prediction error on causally-structured data will preserve some of that causal structure proportional to its predictive value," without privileging IB specifically.

**Current status**: untested. The reframing path (b) is probably more tractable than (a) and would weaken the headline claim slightly but on more solid ground.

---

## §3. (SC) is essentially vacuous in our empirical setup

**The weakness**: spike Theorem 1 lists three postulates — (SLC) Standard Linguistic Convention, (SC) Speaker Commitment, (CS) Compositional Structure. (SC) does load-bearing work in the theorem: it bridges "the text contains marker M" to "the speaker committed to M's Pearl-hierarchy content." But our test sentences are **not speaker-committed**. We constructed "the alarm's sounding caused the doors' closing" as a probe; we (the experimenters) didn't mean it as a Pearl-Level-2 claim about the world. The embedding doesn't care whether we meant it.

The empirical findings depend on (SLC + CS) plus whatever (SC)-faithful content survived in the *training corpus*. (SC) in the test sentences is empirically operationalized at zero.

**What follows from this**: the theorem is over-postulated for what the empirical results require. We could rewrite Theorem 1 without (SC) and the empirical findings would be unchanged. Either (a) (SC) is unnecessary in the theorem, or (b) (SC) is necessary but its operationalization is missing from this work entirely.

**What would address it**: characterize (SC)-faithfulness in actual training corpora — what fraction of "X caused Y" usage in web-scale text is speaker-committed Pearl-Level-2 vs metaphorical / hedged / fictional / sarcastic / advertising? This would tell us the empirical force of (SC) at training time, even if not at test time. Adversarial test §D-untried below is the cleanest way to attack this from the empirical side.

**Current status**: not addressed. Likely needs to be in the paper's limitations section explicitly.

---

## §4. Pearl content is rare in training corpora

**The weakness**: we probe with explicit-marker Pearl-Level-2 and Level-3 constructions ("A caused B" / "without A, B would not have occurred" / "definitely caused" / etc.). **Most natural language doesn't make Pearl-Level-2 commitments.** Web crawls are mostly bag-of-clauses, lists, technical jargon, conversation fragments. Wikipedia has causal markers but they're sparse. Academic writing reserves explicit Pearl-language for specific argumentative moves. By rough estimation, explicit-marker Pearl content might be 1-5% of any real training corpus.

**What follows from this**: the diagnostic detects *something* aligned with Pearl-structured templates. But the dominant signal in the embeddings is almost certainly *not* Pearl content — it's everything else: lexical co-occurrence, topical clustering, syntactic patterns, register, sentiment, formality, etc. What we're showing is "enough Pearl content survives to be detectable," not "Pearl content is a major structural axis of the embedding space."

**What would address it**: quantify the actual rate of explicit-marker Pearl content in pretraining-equivalent corpora. Probe-effect sizes should correlate with that rate across content domains (e.g., evidential markers might be even rarer than causal markers, but E12-E13 showed evidential probes work at comparable effect sizes — that's *more* surprising than the Pearl finding, not less, and worth thinking about).

**Current status**: untested. Worth measuring. Could meaningfully affect framing of the "diagnostic class" contribution.

---

## §5. The "encoded vs deployed" distinction is philosophically thin for embedding-only models

**The weakness**: we've leaned hard on the encoded-vs-deployed distinction as a methodological contribution. **But for embedding-only models, there is no separate "deployment."** The embedding IS the output. There's no generation, no decoding, no sampling. We're measuring the only thing these models do. We're not "isolating encoding from deployment"; we're measuring a different kind of model behavior than behavioral benchmarks measure.

The encoded-vs-deployed framing borrows authority from a distinction that makes sense for generative LLMs (where there's a clear gap between internal state and generated output) but doesn't cleanly apply to the model class we used.

**What follows from this**: the methodological contribution as currently framed is partially misaligned with the experimental work. A reviewer can fairly say: "your methodological contribution is positioned against behavioral benchmarks for generative LLMs, but your experiments are on embedding-only models where 'behavior' isn't really a thing. The encoded-vs-deployed distinction is overclaimed."

**What would address it**: either (a) reframe the contribution as "structural embedding probes complement behavioral benchmarks; the two test orthogonal questions" without claiming to ISOLATE encoded from deployed, or (b) extend the methodology to generative models (probe their pooled hidden states, test whether the encoding pattern correlates with generation behavior — the §F-untried below).

**Current status**: partially addressed by the "complementary methodology" framing in `docs/diagnostic-methodology.md`, but the encoded-vs-deployed framing remains prominent elsewhere in the project and should probably be softened or operationalized more carefully.

---

## §6. The 30 event pairs were constructed by us, with knowledge of the test — AND are in a narrow register

**The weakness**: we constructed 30 event pairs sequentially, with awareness of what the tests were measuring. Selection-bias risk: even without explicit cherry-picking, the construction process introduces unconscious bias toward pairs that "feel like" they should work. The "30/30 perfect" results across multiple probes is consistent with successful pair-construction as much as with robust encoding.

**Additional layer (flagged by independent review 2026-05-14)**: the event-pair lexicon is not just hand-constructed but **all in a narrow register of gerundive event-causation** — "the alarm's sounding," "the doors' closing," "the package arrival," "the engine start." This was a deliberate methodological choice (avoids the full-clause grammaticality artifacts that produced the E1 surface-form confound; see PROGRESS.md §E1 methodological catch), but it means the empirical findings are about **how this register of constructed event-causation lives in embedding geometry**, not about how causation lives in the full register of natural language. Different registers — academic argumentation, conversational dialogue, technical prose, narrative fiction — likely have different distributional profiles for causal markers and might produce different probe results.

**What follows from this**: external validity has two stacked uncertainties: (i) hand-construction selection bias, (ii) narrow-register confinement. A genuinely random sample of 30 event-causation expressions from naturalistic discourse — across registers — might produce different effect sizes. Possibly weaker, possibly stronger, possibly differently-distributed across models.

**What would address it**: the PDTB-3 / GUM / RST-DT validation already queued in `OPEN-WORK.md` would address (i). Stratifying that validation across discourse registers would address (ii). **But until both are done, the work is vulnerable to "you picked your test cases AND you picked your register" objection.**

**Current status**: (i) known and queued. (ii) newly noted as a separable concern; should be folded into the corpus-validation experimental design when it runs.

---

## §7. Cross-marker direction (E3) may be semantic-field clustering

**The weakness**: E3 tested four causal markers — "caused / produced / triggered / resulted in/from". Within-direction cosine > across-direction cosine, 30/30 on best models. We interpreted: direction is encoded structurally, not lexically.

**Alternative reading**: causation verbs share a semantic field. Active-voice causation sentences cluster; passive-voice causation sentences cluster. We could be detecting the **active/passive voice distinction** (which is in any embedding) **plus correlated semantic-field signal**, not direction-as-such.

**What would address it**: use markers from different semantic fields (causal: "led to / enabled / catalyzed / precipitated") and compare to markers with similar grammatical structure but no causal force (similarity: "resembled / mimicked / matched / paralleled"). If causal and similarity verbs cluster *separately by direction*, that's evidence direction-as-such is encoded. If they cluster *together as active-voice*, the E3 finding reflects voice-geometry plus correlated semantic-field signal, not direction-encoding.

**Current status**: untested. This is one of the strongest available adversarial moves; it's an experiment we should run.

---

## §8. Cross-lingual (E8) could be translation-pair surface vocabulary

**The weakness**: E8 cross-lingual finding rests on "A caused B" (EN) being closer to "A causó B" (ES) than to "B causó A" (ES). 10/12 in predicted direction.

**Alternative reading**: "A caused B" and "A causó B" share more surface content than "A caused B" and "B causó A". Proper-nouns A and B in shared positions; sentence-final punctuation; basic SVO structure; **cognate vocabulary** ("caused" ↔ "causó" are cognates; bge-m3's multilingual representation almost certainly aligns cognates closely).

The 10/12 result could reflect translation-pair surface similarity plus same-script tokenization, not direction-encoding-as-such.

**What would address it**: test with orthographically completely different scripts. Mandarin, Arabic, Korean, Japanese — no cognate vocabulary, different writing system, different tokenization. If direction alignment holds there, the cross-lingual claim is genuinely structural. If it fails, E8 reflects surface similarity not structural transfer.

**Current status**: untested. Joseph's translations were Spanish-only; the orthographically-different-script test is one of the cheapest substantial adversarial moves available.

---

## §8.5. The "capacity-correlated Pearl-Level-2 specificity in qwen3" claim has a dimensionality/architecture/training-data confound

**The weakness** (flagged by independent review 2026-05-14): we report that Pearl-Level-2 specificity emerges *only* in qwen3 (4096d) — the largest model tested. We've consistently framed this as **capacity-correlated**, with the implication that dimensionality is the relevant variable.

But qwen3 differs from the smaller models in three ways simultaneously: it's higher-dimensional (4096 vs 768-1024), architecturally distinct (Qwen3-derived vs BERT/MoE/decoder-derived), and trained on different data with different objectives. The "qwen3 cleanly distinguishes Pearl-Level-2 specificity" finding is consistent with **all three** of:
- Dimensionality matters (4096d gives enough representational capacity to factor Level-1 from Level-2)
- Architecture matters (Qwen3's specific design favors structural-content factoring)
- Training data/objective matters (qwen3-embedding's training corpus emphasized causal content enough)

The finding is uniquely-attributable to no single variable from the experimental setup.

**What follows from this**: the framing "Pearl-Level-2 specificity is capacity-correlated" is more accurately stated as "Pearl-Level-2 specificity is detectable in the highest-capacity model tested, which also differs in architecture and training corpus from the others." Without isolating the variables, we can't claim it's specifically capacity.

**What would address it**: test on models that differ from qwen3 along only one variable at a time. Test on architecturally-different 4096d models (some Llama-3-based embedding extractions are larger-dim than 768d). Or test on Qwen3-architecture models at different dimensionalities. Either would help disentangle.

**Current status (CONFIRMED on the full 12-model V2.1.1 panel, 2026-05-15)**: **the qwen3 reframing is confirmed; the capacity-correlation hypothesis is not supported (no trend).**

Full V2.1.1 panel evidence:
- Capacity correlation, all n=12, all non-significant: E1 ρ≈−0.48 (p≈0.11), E2 ρ≈−0.08, E10 ρ≈−0.06, E13 ρ≈+0.12. Within-family scale-ups non-monotone. There is **no capacity trend** — the pre-audit "weak, direction-consistent" hedge resolves, on the full panel, to a clean negative.
- E2 (vocab-identical — the cleanest single test of the Theorem-1 non-reducibility property): C1-dominant in only 3/12, with 4 ORIG-NULL, 3 ANOMALOUS-sign-flip, 1 MIXED, 1 C2. The non-reducible route is the minority outcome and not capacity-ordered (smallest and largest both C1; mid-band null/anomalous).
- E15 ALL/orig across 12 models: BIDIRECTIONAL 4, ASYMMETRIC (direction-prior-confound) 5, REVERSE-only 1, NULL 2. qwen3 = ASYMMETRIC: H_D1 +0.025 [+0.015,+0.035] strong, H_D3 +0.009 with CI crossing zero [−0.001,+0.018]. Reproduced on independently-generated test data (E19 codex_c1: BIDIRECTIONAL 2, ASYMMETRIC 3, NULL/REVERSE 7 of 12).

**Reframing (confirmed)**: qwen3's apparent real-world Pearl-Level-2 specificity **is substantially a world-knowledge-direction prior, not C1 syntactic-position** — confirmed on the full panel and cross-substrate, no longer "at least partly." qwen3 retains a *thin* genuine C1 only on the vocabulary-identical E2 (ratio +0.089). Capacity-asymmetry-as-asymmetric-comprehension is **not supported as an empirical pattern** by this panel (it may persist as a conceptual hypothesis in the scaffold, but the data do not evidence it; stating that plainly is the strengthen-before-soften move — a confirmed negative beats a hedged "weak"). This is also the paper's headline empirical finding: the deflation is the contribution (the instrument resolves what behavior cannot; what it resolves is that the strong reading is narrow and the most-cited evidence confounded). See `paper/src/05-empirical-findings.md` / `06-mechanism-decomposition.md`.

---

## §9. Capacity dissociation (E12-E13 vs E1) may be probe-difficulty artifact

**The weakness**: I called the nomic-v1.5-fails-Pearl-passes-evidential dissociation "genuinely new methodologically — capacity grading is multi-axis, not single-linear."

**Could be probe-difficulty artifact.** Evidential templates (E12) use first-person constructions with three lexically-disjoint verb classes; Pearl-Level-2 cascade (E1) tests a more subtle distinction between cause-marker and conjunction. The probes haven't been **equated for difficulty**. A capacity-monotonic model (better-at-everything-at-higher-capacity) would produce exactly what we observed: pass the easier probe, fail the harder probe. The "multi-axis capacity grading" claim then collapses to "harder probes need more capacity," which is unsurprising.

**What would address it**: equate probe difficulty across content domains. Construct evidential probes that are AS subtle as the Pearl-Level-2 cascade probe; construct Pearl-Level-2 probes that are AS easy as the evidential category probe. If models still dissociate after difficulty-equating, the multi-axis claim survives. If they don't, the finding collapses to capacity-monotonic with content-difficulty variation.

**Current status**: untested. Until equated-difficulty probes exist, the headline E12-E13 capacity-dissociation finding should be flagged as preliminary.

---

## §10. The "structural" vs "lexical" distinction is operationally fuzzy

**The weakness**: we claim the diagnostic detects "structural" content (direction-as-such, modal-strength-as-such, evidential-category-as-such) rather than "lexical patterns." Operationally we use cross-marker variance as proxy — if a probe works across multiple markers, it's structural.

But the boundary is fuzzy. "Direction across four causation verbs" generalizes lexically (because the verbs share a semantic field — see §6). The inferential cluster in E13 ("inferred / deduced / concluded from the evidence" — three markers with almost no shared surface content) is stronger evidence for "structural," but even there, the three markers share evidential-inferential semantic field.

A skeptical reviewer can fairly say: "your 'structural' just means 'generalizes across a few markers chosen from a semantic field that maps to the construct.' That's a moderately abstract lexical pattern, not pure structure."

**What would address it**: define "structural" more rigorously. One operational route: train an embedding axis on marker-class A's minimal pairs; show the axis transfers to marker-class B's minimal pairs where A and B share NO semantic field but both align with the abstract construct. Hard to construct; would be definitive if achievable.

**Current status**: this is more philosophical/conceptual than empirical, but it underlies the methodological-contribution framing.

---

## §11. The findings are correlational, not interventional

**The weakness**: all our probes observe similarity patterns. We have **no interventional test** on the embeddings themselves. We can't change a specific representational feature and see what changes downstream. We observe that "A caused B" embeds closer to its counterfactual than "A and B" does — that's correlation. We don't show that the embedding-geometry similarity *causes* anything downstream (in generation, retrieval, classification, anything).

For a claim about *encoded content*, this is a meaningful gap. Encoded content should make some downstream behavior different than it would be without that content; otherwise the encoding is epiphenomenal.

**What would address it**: pair the structural probe with a downstream task. E.g., use the embeddings as retrieval/classification features, show that models passing the Pearl-probe perform differently on causal-reasoning downstream tasks than models failing it. The behavioral-pairing experiment queued in `OPEN-WORK.md` partially addresses this, but only at the model level, not at the per-sentence representation level.

**Current status**: this is the deepest methodological weakness in the work and isn't going to be fully addressed by anything currently planned. Acknowledging it honestly in the paper would be appropriate.

---

## §12. The five model architectures share substrate

**The weakness**: we tested 5 + 1 model architectures across BERT-derived (nomic-v1.5, mxbai), decoder-derived (embeddinggemma), Mixture-of-Experts (nomic-v2-MoE), Qwen3-architecture (qwen3-embedding), and XLM-RoBERTa-derived (bge-m3). We claim "substrate-agnostic" methodology validated across architecturally-diverse models.

But they all share substrate in deeper ways: all transformer-derived, all trained on overlapping web-text corpora (Common Crawl, Wikipedia, etc.), all from the 2023-2026 generation of pretrained models. Genuinely independent representational paradigms — RNN-based, structured-prediction-based, multimodal-derived embeddings, retrieval-augmented embeddings, embedding-from-instruction-tuned-LLMs — weren't tested.

**What follows from this**: the "substrate-agnostic" claim is "tested across five transformer-derived models" not "tested across genuinely diverse representational substrates."

**What would address it**: test on truly different substrates. word2vec / GloVe (non-transformer); InferSent (RNN-based); CLIP text encoder (multimodal-derived); GPT-4 last-layer pooled (instruction-tuned generative).

**Current status**: untested. Would substantially strengthen substrate-agnosticism claim if positive; would substantially weaken it if negative.

---

## §13. The diagnostic might be detecting Pearl-Anglo-centric assumptions baked into our framework

**The weakness**: Pearl's hierarchy itself is a particular philosophical framework for carving causal structure. It's the dominant one in causal-inference statistics and AI, but it's not the only way to think about causation. Other frameworks: probabilistic-causation (Suppes), counterfactual-only-no-interventional (Lewis), mechanism-based (Glennan), agency-based, process-based (Salmon).

Languages with grammaticalized evidentiality, classifier languages, polysynthetic languages might encode different aspects of "causation" that Pearl's hierarchy doesn't capture. Our methodology presupposes Pearl's framework as oracle. If models encode "causation" in a way that doesn't map cleanly onto Pearl-Level-2, our probes will fail to detect it.

**What follows from this**: claims like "Pearl content is structurally encoded" might be more accurately "what we constructed as Pearl-content is structurally detectable; whether models encode causation in non-Pearl-frame ways is untested."

**What would address it**: probe for non-Pearl causation frameworks. Use Suppes-probabilistic-causation templates; use Lewis-pure-counterfactual templates; compare whether models pass our probes consistently across causation-framing or whether some framings produce nulls. If frameworks produce different probe-passage profiles, the methodology is Pearl-frame-specific.

**Current status**: not flagged in the project anywhere. Worth noting in the paper as a scope condition.

---

## §14. The empirical work might be probing for representational ontology that doesn't apply

**The weakness**: the "encoded content" framing implicitly assumes embedding models have something like internal representations of meaning that can be inspected. A strict behaviorist or eliminativist about LLM representational content would argue we're projecting representational ontology onto pure pattern-matching systems. Our methodology can be reread as: "models have geometry-of-input-patterns that happens to correlate with Pearl predictions" — without commitment to representation-language at all.

**What follows from this**: the methodological-contribution framing of "audit what models encode" presupposes models encode things in a meaning-respecting way. A philosopher of mind / AI eliminativist (Searle-style, or some computational-functionalists) would push back.

**What would address it**: explicit philosophical positioning. Acknowledge the representational-ontology presupposition; note that the empirical findings stand independently of the philosophical framing; offer the reframing "geometry-of-input-patterns aligned with Pearl predictions" as a behaviorist-compatible reading.

**Current status**: not currently in the project's framing. The asymmetric-comprehension philosophical machinery already commits to a non-eliminativist representational ontology; explicit acknowledgment of that commitment would help.

---

## §14.5. The ethics-arc connection from empirical findings to welfare implications is long and indirect

**The weakness** (flagged by independent review 2026-05-14): the project sometimes presents the empirical work as load-bearing for an ethics-arc conclusion in the welfare-implications-of-LLM-phenomenology debate. The logic runs: embeddings encode rich structural content → substrate is richer than often assumed → asymmetric-comprehension means we can't confidently characterize what's absent → confident dismissal of welfare relevance is unwarranted.

Each step is individually defensible, but **the chain is long**, and the empirical work is doing a lot of load-bearing for a conclusion that ultimately lives in a philosophical companion (the Synthese paper at `~/src/synthese-paper/01-synthese-asymmetric-comprehension/`). The empirical contribution stands without the welfare-arc; the welfare-arc stands on its own philosophical merits. **Tightly binding them in the empirical paper invites reviewers to dismiss both by association** — "this is an empirical paper trying to do philosophy" or "this is a welfare paper using empirics for rhetorical weight."

**What follows from this**: any paper drafting in this project needs to consider whether the welfare-arc framing is load-bearing or decorative for the empirical claims. If load-bearing, the connection needs to be tight; if decorative, it probably shouldn't be in the empirical paper at all.

**What would address it**: explicitly factor the contributions. The empirical paper makes a methodological-and-empirical claim about structural Pearl-content in pretrained sentence embeddings. The Synthese paper makes a philosophical claim about asymmetric-comprehension and AI welfare. The two compose well, but the empirical paper should probably *point to* the philosophical paper rather than carry the welfare-arc itself. Cross-cite at submission.

**Current status**: framing risk identified; needs explicit decision at paper-drafting time. Probably the right move is "empirical paper points to Synthese paper for the welfare implications" rather than "empirical paper makes the welfare argument from the data."

---

## §15. The MoE / nomic-v1.5 E1 nulls might be different-mechanism encoding, not absence

**The weakness**: nomic-v1.5 and nomic-v2-MoE failed E1 H1 (Pearl cause-vs-temp distinction). We framed this as capacity-correlated and recovered by E10's clean templates.

But: maybe these models encode causation *differently* — not via cause-vs-conjunction CF-alignment but via some other mechanism (e.g., direct lexical-association of "caused" with "would not have"). They might recover Pearl-Level-3 modal strength via the adverb-scaled axis (definitely/partially/minimally) while encoding Pearl-Level-2 via a different (non-CF-cascade-aligned) mechanism entirely.

**What follows from this**: "these models encode Pearl content" is the conclusion the framing leads to, but it might more accurately be "these models encode some-content-that-our-probes-tested-for via mechanisms-our-probes-detected." Different models might use genuinely different encoding mechanisms; the probes catch the ones aligned with our test design.

**What would address it**: design alternative probes for the same Pearl content that use different test logic, then check whether models that pass one probe also pass the other. If all models pass all probes, the encoding is uniform. If models pass different probes, the encoding is multi-mechanism.

**Current status**: untested. Would significantly refine the "Pearl content is encoded" claim if probed properly.

---

## §16. C2-route findings do not test Theorem 1's non-reducibility property — only C1 does

**Surfaced 2026-05-14 by the opus audit (§11). Joint audit framing-level addition.**

**The distinction that matters.** The originating spike's Theorem 1 establishes that Pearl-Level-2 content is encoded in natural-language text in a way **not derivable from the Level-1 joint distribution over event variables alone** (per CHT applied to the kettle-pair Lemma 1). The "non-reducibility" is the load-bearing claim that gives Theorem 1 teeth.

The C1/C2 framework introduced in §1 names the empirical-mechanism distinction:
- **C1 route**: syntactic-position encoding (vocabulary-identical contrast — same tokens, different positions; null under token-shuffle). This is the empirical analog of Theorem 1's CHT-non-reducibility property — the geometric distinction lives in the position of arguments around the marker, which a Level-1 joint distribution over event variables cannot recover.
- **C2 route**: Reichenbachian inheritance via vocabulary-pattern statistics (vocabulary-different contrast; survives token-shuffle). This is consistent with Pearl-content being implicitly encoded in word co-occurrence — but a sufficiently rich Level-1 corpus summary *could in principle reproduce* the alignment via vocabulary co-occurrence statistics. C2 is **not** the property Theorem 1 directly licenses.

**Implication for the diagnostic class.** The diagnostic measures whether some-Pearl-content-aligned-signal exists in the embedding. When E14 classifies a test as **C1-dominant**, that result tests the Theorem-1 property cleanly: the geometric distinction requires word-order, which a Level-1 corpus summary cannot produce. When E14 classifies a test as **C2-dominant** or **MIXED**, the result is evidence that some Pearl-content-aligned signal is present in the embedding *via mechanisms that a Level-1 corpus summary could plausibly reproduce*. Both are real empirical findings about embedding geometry; only the first directly licenses the "Pearl-Level-2 content non-reducibly encoded" reading from Theorem 1.

**What this changes about the project's framing.** The current framing in HEADLINE-FINDINGS — "Pearl content is structurally encoded in embeddings" — reads as if the diagnostic measures the Theorem-1 non-reducibility property uniformly. It doesn't. The honest reframing has two layers:
- **C1-dominant findings** = "Theorem-1-property tests passing" = "Pearl-Level-2 content is encoded *non-reducibly* (in the CHT sense) in this model on this test."
- **C2-dominant or MIXED findings** = "Pearl-content-aligned signal present, mechanism Level-1-reproducible" = "Some Pearl-marker-correlated structure is detectable in this model on this test, but the CHT-non-reducibility claim is not licensed by this finding alone."

**What follows.** Future paper drafting should split the empirical results into "C1-route findings (Theorem-1-property tests)" and "C2-route findings (Pearl-content-aligned signal-presence tests)." Both are real contributions; only the first is what the originating spike's Theorem 1 directly licenses. This is not a softening — it's a sharpening of *which empirical results map to which theoretical claim*. The C2-route findings remain genuine empirical evidence about embedding geometry, and the broader diagnostic-class methodology remains sound; the caveat is on the bridge from any individual finding to the Theorem-1 framing specifically.

**The behavioral-floor relationship (added 2026-05-15; the "constructive dual / complement" phrasing of the first version was a proven category error — corrected to D1 below by the theory-push spike `spikes/spike-constructive-dual-of-behavioral-floor/`).** Read through `~/src/behavioral-floor/`'s Claims 1–2, the C1/C2 split stops looking like a limitation and becomes the contribution — but the connective is *not* a duality and the substrate diagnostic does *not* discharge the deployed mechanism. The exact, proven statement (spike Proposition D1, under premises (D1-1)–(D1-4)):

> behavioral-floor proves the inherited-vs-pattern-match mechanism distinction is **behaviorally unbounded from below** (Lemma-3 parrot witness; strictly stronger than Claim 2's non-certifiability of the deployed mechanism M). causal-language's substrate diagnostic discharges the **encoded-presence precondition M⁻** — directional Pearl-Level-2-form content distinguishing inherited-L2 from Level-1-reproducible mechanism is present and CHT-non-reducible in the frozen representation — that the behavioral route provably cannot even lower-bound, via a concrete realization of behavioral-floor's own (N1) structural-inspection escape. behavioral-floor's encoded/deployed seam (its src/03) is *exactly why M⁻ does not upgrade to deployed-mechanism certification.*

So "this finding is C2-route, not Theorem-1-licensed C1" is still not a hedge — where the diagnostic returns C1-dominant it discharges M⁻, which behavioral evaluation provably *cannot bound*. But the claim is now exact: the diagnostic certifies *encoded-presence* (M⁻), not *deployed mechanism* (M); the correction matters because the original framing overreached on precisely the encoded/deployed distinction behavioral-floor itself enforces. The genuine dual pair the intuition was tracking is **GC2 ↔ H_D3** (same nuisance-prior partialling operator at different layers; spike Proposition D2, a role-homology not a content-equivalence). Framework-level: Claim 2 and the CHT result are two adjacent instances of the `#disc-identifiability-floor` cascade sharing the (N1) escape modality. Full argument + epistemic-status ledger: `spikes/spike-constructive-dual-of-behavioral-floor/` (README → `03` → `04` → `05`).

**Current status**: surfaced, characterized, and made exact (2026-05-15). Reframed as additive-via-D1, not a softening — but D1's empirical reach equals the C1-dominant set (currently model/test-dependent; inherited open front, not resolved by the spike). The next paper draft should carry the D1 sentence verbatim as the cross-reference (not "dual/complementary"); split C1 vs C2 in any Theorem-1-tied results table; and route the mirrored behavioral-floor cross-reference through OPEN-WORK §C (do not edit behavioral-floor src unilaterally).

---

## Untried tests that could falsify the work (the falsification battery)

These are specific, runnable experiments. Most can be done in ~1 week of focused work; one (§F) is harder.

### §A. Lexically-impoverished probes (event placeholders)

Replace event noun-phrases with abstract referents R1, R2: "R1 caused R2" / "without R1, R2 would not have occurred." 

**Status: CLOSED POSITIVELY (2026-05-14).** Spike S02 Part D ran this on 30 algorithmically-generated nonce pairs (deterministic seed=42) across all 5 primary models. H_D1 passes on all 5 models (p<0.01 on every model). The R1/R2 placeholder variant (Part D_ph) shows the effect with single-character placeholders at reduced magnitude (+0.012 vs +0.023 for nonce words), confirming the directional encoding does not require event-specific semantic content. The effect is fully structural under the C1 route.

**Additional finding**: Token-shuffle on H_D1 nulls the effect on all 5 models — confirming the directional encoding is syntactic (C1 route), not bag-of-words (C2 route). cf_fwd and cf_rev have identical token sets; shuffling collapses them to near-identical embeddings (mean sim 0.91), destroying the directional signal.

### §B. Token-shuffle controls

Take E10 templates: shuffle token order within each sentence (preserving the token bag but destroying syntactic structure). Embed. Run the probes.

**Status: PARTIALLY RUN (2026-05-14).**

- **H_D1 (nonce Pearl markers, vocabulary-identical comparison)**: NULLS under shuffle on all 5 models — confirming C1-route (syntactic) encoding. This is the predicted result under Theorem 1 (Level-2 directional content is non-reducible to Level-1 token co-occurrence; shuffle is a Level-1 test).

- **E10 H4 (Pearl modal strength, vocabulary-DIFFERENT comparison)**: run by a concurrent independent spike (`.archive/spikes_2026-05-14_situation_independent/`). The shuffled-vs-original ratio holds (~1.0 for smaller models, ~0.23 for qwen3). Interpretation: E10 H4 is primarily **C2-route** (bag-of-words accessible via different adverb vocabulary), with a small additional C1-route contribution in higher-capacity models.

The key interpretive distinction: token-shuffle tests different things on vocabulary-identical vs vocabulary-different comparisons. Vocabulary-identical: shuffle makes comparison cells indistinguishable → C1-route nulled. Vocabulary-different: shuffle preserves bag differences → C2-route survives. The right diagnostic is: "Does the alignment depend on which token occupies which syntactic position, holding vocabulary fixed?"

**Open**: E1, E2, E3, E12, E13 token-shuffle not yet run. Classifying the full E1-E13 battery by C1/C2 mechanism via this test would be a clean methodological contribution (Section 4 of any paper). This is now the most important remaining control battery.

### §C. Token-bag-matched random sentence control

For each minimal-pair sentence, generate a random sentence with same token bag but completely scrambled semantics. Embed both. The probes should treat them as different.

If they don't, the embedding is responding to token statistics not sentence semantics.

### §D. Adversarial templates — causal markers without Pearl content

Construct sentences with *all the surface causal markers* but no actual Pearl-Level-2 commitment:
- Marketing copy: "X causes you to feel great!"
- Fictional: "In the story, A caused B"
- Metaphorical: "Loneliness caused his collapse"
- Sarcastic: "Oh sure, the rain caused the sunshine"

If probes pass on these the same as on genuine Pearl claims, the methodology detects marker presence not Pearl commitment. Consistent with §3 above ((SC) being empirically vacuous).

### §E. Pearl-stripped training corpus control

Train a small embedding model on a corpus with explicit-causal-markers removed (or filtered/replaced). Run our probes.

Predicted: probes fail because the encoded content was removed. Failure mode: if probes pass, the encoding doesn't come from causal-marker content in training — it comes from something else (architectural priors, token-statistics, etc.).

**Most rigorous causal-attribution test possible.** Expensive (requires training a model from scratch on a filtered corpus). Without this, we can't strongly claim the encoded content is *because of* causal-marker content in training.

### §F. Probe on generative-LLM pooled hidden states

Take Llama-3-8b / Qwen3-7b / Gemma-2-9b — generative models — extract pooled last-layer hidden states as "sentence embeddings." Run our probes.

If pattern differs significantly from embedding-class models, our findings are specific to embedding-model architecture/training, not general to pretrained representations. If similar, findings generalize.

Cleanest test of substrate-agnosticism claim.

### §G. Synthetic-language positive control

Build a small artificial language with known Pearl-Level-2 structure (controlled corpus generation), train a small embedding model on it, run probes.

Positive control: methodology should correctly detect what we know we put in. If it doesn't, methodology is broken in some unknown way. If it does, methodology is validated on a setting with ground truth.

Cleanest possible methodological validation — and we haven't done it.

### §H. Cross-semantic-field marker test

For E3, use markers from different semantic fields (causation: "led to / enabled / catalyzed / precipitated") and compare to markers with similar grammatical structure but no causal force (similarity: "resembled / mimicked / matched / paralleled").

If causal and similarity verbs cluster *separately by direction*, direction-as-such is encoded. If they cluster *together as active-voice*, E3 reflects voice-geometry + correlated semantic-field signal.

Direct test of §7 weakness.

### §I. Non-Latin-script cross-lingual

Repeat E8 with Mandarin, Arabic, Korean, or Japanese. No cognate vocabulary; different writing system; different tokenization. Tests whether cross-lingual direction alignment is structural or surface-similarity.

Direct test of §8 weakness.

---

## What the structure of the critique reveals (meta-observation)

The critiques above cluster around a unifying shape:

**We have positive correlational findings on a specific kind of constructed input, interpreted within a specific theoretical framework, with limited testing of alternative explanations.**

Each major weakness is an instance:

- **§1: methodology-design dependency** (we start from phrasings; the situation-anchored alternative tests the same claim differently) — **addressed** (E15+E16; see §1 Resolution)
- §2, §13, §14: theoretical-framework dependencies (IB-optimality, Pearl-framework-specificity, representational-ontology)
- §3, §4, §6, §7, §8, §12: input-construction dependencies (test sentences not speaker-committed; Pearl-rare in training; hand-constructed-and-narrow-register pairs; same-semantic-field markers; same-script-language; same-transformer-substrate)
- §5, §8.5, §9, §10, §11, §14.5, §15: interpretation dependencies (encoded-vs-deployed for embedding-only models; capacity-confound across dim/architecture/training; probe-difficulty confound; structural-vs-lexical operationalization; correlational-not-interventional; ethics-arc framing; different-mechanism vs absence)

This isn't an indictment — much of empirical NLP and representation-probing work has the same shape. But it suggests the most fertile *falsification* moves are the ones that vary the things our setup holds constant: vary **the methodology** (§1 — situation-anchored phrasing variation, *executed* via the two parallel spikes consolidated into E15+E16; see §1 Resolution), vary the inputs (§A, §H, §I), vary the substrate (§E, §F), vary the framework (§D, §13).

**§1's situation-anchored phrasing-variation experiment was the single move that would most have strengthened the work against this critique-shape — and it was executed** (2026-05-14 parallel spikes → consolidated into the V2.1.1 E15+E16 synthesis; see §1 Resolution). Its outcome did *not* support situational-invariance: the existing positive results are correctly reframed as predominantly vocabulary-co-occurrence / entity-recurrence with a thin structural residual and a qwen3 world-knowledge prior — which is exactly the deflationary framing the paper now carries, not a borrowed "the embedding encodes Pearl-causal content" claim.

The strongest single move that **remains unrun** is therefore **§E (Pearl-stripped training corpus control)** — because it tests causal-attribution from content-in-training to encoding-in-representation, which §1's resolution does not establish.

So: §1 is addressed (deflationary, via E15+E16); §E is the strongest outstanding causal-attribution move; until §E is done we have a measured *mechanism* decomposition but not a *training-causal-attribution* of it.

---

## What survives the adversarial review robustly

Not everything is weak. The following findings would, I think, survive a careful adversarial reanalysis 5 years from now — *with the V2-panel-driven revisions noted in italics*:

1. **E10's Pearl-Level-3 modal-strength alignment** — 30/30 perfect across all 5 V1 models with clean templates, effect sizes up to +0.165 cosine shift. *V2 caveat (joint audit, opus §1)*: under E14 V2 multi-shuffle controls on the 12-model panel, E10 reads as **C2-route bag-detectable on smaller models** (nomic-v1.5 ANOMALOUS-AMPLIFY-MILD, shuf > orig — the bag of paired modal markers carries the alignment signal independent of word order) and **C1-route order-dependent only on qwen3** (ratio 0.184). The honest reframing: Pearl-Level-3 modal-strength alignment is a real geometric phenomenon on the V1 templates across all models, but the *mechanism* varies — bag co-occurrence on smaller models, syntactic-position on the largest. The "encoded across all models" framing should soften to "detectable across all models, with C1-vs-C2 mechanism varying by capacity."

2. **E3 cross-marker direction alignment** — 30/30 perfect on best V1 models with markers chosen from genuinely distinct lexical surfaces. The semantic-field-clustering objection (§7) softens this somewhat, but the inferential-cluster finding in E13 ("inferred / deduced / concluded from the evidence" share almost no surface content) is partly inoculated. *V2 caveat (joint audit)*: V2 E14 shows E3 is MIXED on most models and only C1-dominant on qwen3 + all-minilm; the "structural not lexical" framing should be modulated to "axis generalizes across markers, but mechanism varies by model."

3. **The E7 → E10 surface-form-recovery sequence** as a methodological move. This is genuinely novel and demonstrates the diagnostic catches confounds. A reviewer can attack what the diagnostic measures, but the diagnostic's *ability to surface its own failure modes and recover from them via cleaner templates* is itself a real methodological capability.

4. **The Solomonoff-Tarski-oracle cross-disciplinary route to Nagel/Jackson** in the theoretical-spike. Three formal frameworks well-known separately, wired to Nagel/Jackson — this connection is real and doesn't depend on the empirical findings holding up.

5. **The asymmetric-comprehension principle's load-bearing role in the project's epistemic posture.** Whatever the empirical findings turn out to mean, the discipline of "don't project upper bounds from below" is a sound methodological commitment that protects against overclaiming.

---

## Net assessment

**The empirical findings are robust *as descriptions of embedding-geometry behavior on our specific minimal-pair constructions*.**

**The interpretation as "Pearl-hierarchy content is structurally encoded in IB-optimal compressors of natural language" requires several inferential steps that are individually defensible but compoundingly weaker than the headline framing suggests.**

**The methodological contribution (the diagnostic class) is real but somewhat over-framed against behavioral benchmarks given that we tested only embedding-only models.**

The work is in the position of "having shown something" without yet "having shown everything we sometimes say we've shown." That's normal for empirical work at this stage; it's also why this file exists rather than being suppressed.

The single move that would most strengthen the work against this adversarial review is **§E (Pearl-stripped corpus control)** — because it's the only one that could *causally* attribute the encoded content to training-data content rather than to architectural or statistical alternatives.

If I had to bet on which finding survives careful adversarial reanalysis 5 years out: **E10 is the most robust empirical result and survives**. The theoretical framing around it (the IB-causal-substrate bridge, the encoded-vs-deployed distinction, the methodological-class generalization) might need substantial revision.

---

## How to use this file

When paper-drafting:
- Each §1-§14 point should be honestly addressed in the paper's limitations section. Don't skip; reviewers will surface them anyway.
- The strongest of the falsification battery (§A, §B, §H, §I — all cheap and fast) should be run before submission. §E (Pearl-stripped corpus) is harder but most decisive.

When responding to reviews:
- Don't be defensive about items in this file. They're known. The right response is "yes, that's a real limitation; here's our characterization of it" or "yes, that's testable, and here's the experiment we'd run / have run."

When introducing the work to new collaborators:
- Read this file alongside `HEADLINE-FINDINGS.md`. The combination is the honest picture.

When the temptation to overclaim arises:
- Reread §1, §2, §3, §4, §10. The work is interesting and the empirics are real. The framing can survive being made more careful.

---

*This file is intended to be live. New weaknesses surfaced through review, criticism, or honest reflection should be added; addressed weaknesses should be marked as such and (if substantively resolved) potentially moved to OPEN-WORK.md or PROGRESS.md as completed items.*
