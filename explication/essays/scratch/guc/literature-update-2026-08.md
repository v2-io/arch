# Literature update — directed separation / GUC bridge (2026-08-24)

*Fast-turnaround sweep for the FAST @ NeurIPS 2026 paper decision. Base assumed known and not re-plowed: `asf/ref/prior-art-analysis/05-directed-separation.md`, `ref/Prior_art_for_AAT_directed_separation.md` (Undermind ~2026-05), and the 2026-08-22 landscape sweep. Focus: 2025–2026, the specific bridge claim.*

**Bottom line: the bridge is unclaimed.** Nothing found that (a) classifies LLM-agent architectures by goal↔belief causal coupling, (b) connects the Feldbaum/Wonham/Witsenhausen estimation–control separation lineage to LLM/transformer substrates, or (c) derives wrapper-level separation guarantees with structural-vs-behavioral leakage distinction. But 2026 has produced a **converging cluster of near-misses** — three independent lines (prompt-injection impossibility, secure-by-design wrappers, epistemic-architecture critique) each rediscovering a facet without the control-theory frame or the taxonomy. That convergence strengthens the paper's timeliness and its "phenomenological rediscovery" thesis, and it raises the bar on speed: the security community is one framing move away from part of this.

Legend for "read": **P** = primary read (full page / abstract page fetched and summarized), **S** = search snippet only.

## Closest near-misses (would-be partial competitors; each becomes a citation + contrast point)

1. **Pant, Lohani & Kumar, "On the Inseparability of Instructions and Data in Shared-Embedding Sequence Models," arXiv:2606.27567 (Jun 25 2026, cs.CR).** [P]
   Three impossibility results: shared representations make trusted/untrusted content statistically inseparable (TV-distance bound on provenance recovery); untrusted tokens flow through the same attention pathway that determines control-authoritative outputs; finite training can't certify invariance over semantic-equivalence classes. Formalizes "Prompted Action Models" + "Semantic-Faithful Control."
   **Relationship: nearest single competitor to the "Class 3 by construction" half — and it's instruction/data, not goal/belief.** No control-theory lineage (verified: no Feldbaum/Wonham/Witsenhausen), no architecture taxonomy, no wrapper constructions or leakage bounds. Its axis is *provenance* (trusted vs untrusted content); ours is *teleology* (goal-conditioning upstream of epistemic update). Strong citable support for the substrate-level claim; contrast: they prove one coupling channel is unremovable, we classify the whole space and price the escape routes.

2. **Debenedetti et al. (Google DeepMind), "Defeating Prompt Injections by Design" (CaMeL), arXiv:2503.18813 (2025); plus "CaMeLs Can Use Computers Too," arXiv:2601.09923 (2026).** [S — abstract-level via multiple sources; well-known system]
   Wrapper that splits a privileged LLM (sees only trusted instructions, plans control/data flow) from a quarantined LLM (processes untrusted content, can never influence control flow), with capability metadata and an enforcing interpreter; "provable security" on AgentDojo (77% task completion vs 84% undefended — an empirical tempo/capability tax).
   **Relationship: the closest existing *wrapper construction* — effectively a security-motivated W₁ instance, engineered rather than theorized.** No class taxonomy, no leakage-rate formalism, no κ diagnostic, no estimation/control lineage; the separated axis is again instruction-provenance, not goal/belief. Ideal exhibit: "the industry is already building W₁ wrappers without a theory of what they guarantee" — and its utility loss is a measured instance of the Brooks's-Law tempo cost.

3. **Abdelnabi & Bagdasarian, "AI Agents May Always Fall for Prompt Injections," arXiv:2605.17634 (May 2026).** [P]
   Impossibility argument via Contextual Integrity: an adversary can always construct a context making a blocked flow appear legitimate; tightening norms blocks legitimate flows. Explicitly critiques data-instruction separation as a defense paradigm.
   **Relationship: near-miss / citable support.** Phenomenological-normative rather than architectural; no control theory, no classification. Useful as evidence the security discourse is groping for a structural no-go it lacks vocabulary for.

4. **Romanchuk & Bondar, "Semantic Laundering in AI Agent Architectures," arXiv:2601.08333 (Jan 2026).** [P]
   Formalizes propositions acquiring unwarranted epistemic status by crossing tool boundaries; "Theorem of Inevitable Self-Licensing" (circular epistemic justification under standard scaffold assumptions). Calls for explicit "epistemic architecture" (which components observe vs generate); type system left as future work.
   **Relationship: near-miss, kindred spirit.** Verified: no goal→belief causal classification, no control theory, no leakage bounds. Cite as the epistemology-side rediscovery that scaffolds need a theory of epistemic structure — it names the need our framework fills.

5. **Gao et al., "Textual Belief States for World Models: Identifiable Representation Learning Under Strict Mediation," arXiv:2606.27681 (Jun 2026).** [P — abstract]
   "Strict mediation" = predictions must depend only on latent state + action, vs "history bypass" architectures; shows mediation makes representation quality testable and leaky architectures break that connection.
   **Relationship: near-miss.** A separation-style architectural condition on the *belief side* of language agents, with a testability payoff — structurally analogous to Class-1-by-structure — but the mediated variable is history, not goal; no goal-coupling axis, no control lineage.

## Citable support / contrast points

6. **"The Separation Principle and the Dual–Certainty-Equivalence Gap in Model Predictive Control," arXiv:2604.06045 (Apr 2026).** [S]
   Current control-theory statement of exactly the classical line: separation holds in LQG, breaks under model uncertainty, necessitating dual control; information-weighted dual MPC quantifies the policy's dependence on uncertainty.
   **Relationship: citable support** — proof the classical community is still actively refining *policy-level* separation in 2026 while saying nothing about AI substrates. Reinforces the "50 years, wrong level" positioning alongside Derpich & Yüksel 2023.

7. **Sycophancy mechanistic-interpretability cluster (empirical Class-3 evidence):** Genadi et al., "Sycophancy Hides Linearly in the Attention Heads," arXiv:2601.16644 (Jan 2026) [P — abstract: correct→incorrect sycophancy linearly separable in mid-layer attention heads; steering works; heads attend to user-doubt expressions; explicitly stops short of architectural/causal claims]; "Dissociating the Internal Representations of Sycophancy in LLMs," arXiv:2607.07003 (Jul 2026) [S]; "Sycophancy Is Not One Thing: Causal Separation of Sycophantic Behaviors," arXiv:2509.21305 [S]; **BASIL: Bayesian Assessment of Sycophancy in LLMs, arXiv:2508.16846** [S — Bayesian framework explicitly separating sycophantic belief shift from rational updating — the empirical cousin of a κ-style diagnostic, behavioral not information-theoretic]; "It's Not Always Sycophancy: LLM Conformity as a Function of Epistemic Uncertainty," arXiv:2605.27288 [S].
   **Relationship: citable support** — the empirical literature measuring goal→belief contamination one behavior at a time, without a structural frame; exactly the "rediscovers the coupling phenomenologically" thesis.

8. **Shai et al. line — "Constrained Belief Updates Explain Geometric Structures in Transformer Representations," arXiv:2502.01954 (ICML 2025).** [S]
   Transformers implement constrained (parallelized, architecture-shaped) Bayesian belief updating over HMM generators.
   **Relationship: citable support** — gives "the transformer computes belief states" rigorous footing, which is the premise the Class-3 claim contaminates; nothing on goal-conditioning of the update.

9. **Persona-assigned LLMs exhibit human-like motivated reasoning, arXiv:2506.20020 (2025).** [S]
   Goal/identity conditioning degrades veracity discernment across 8 LLMs.
   **Relationship: citable support** — direct behavioral demonstration that goal-context is causally upstream of epistemic performance.

10. **Structural-separation-for-safety architectures (engineering rediscoveries, no theory of the guarantee):** "Parallax: Why AI Agents That Think Must Never Act" (Cognitive-Executive Separation), arXiv:2604.12986 [S]; "Structural Enforcement of Goal Integrity via Separation-of-Powers Architecture" (PEA: intent/authorization/execution layers, capability tokens), arXiv:2604.23646 [S].
    **Relationship: near-miss/citable** — 2026 safety papers proposing separation architectures on the *action* side; neither classifies belief↔goal coupling nor bounds leakage.

11. **Banu, "Harness Engineering as Categorical Architecture," arXiv:2605.12239 (May 2026).** [P — abstract]
    Category-theoretic (G, Know, Φ) formalization of harnesses; "structural guarantees are harness-level properties," certificates preserved under compilation to five frameworks. **Relationship: near-miss on the meta-claim** ("scaffold structure is where guarantees live") with entirely different machinery; no goal/belief axis, no information theory. Worth a sentence as independent convergence on wrapper-level guarantees.

12. **"Inside the Scaffold: A Source-Code Taxonomy of Coding Agent Architectures," arXiv:2604.03515 (2026).** [S]
    12-dimension empirical taxonomy of 13 scaffolds (control architecture / tool interface / resource management). **Relationship: citable contrast** — taxonomy by code structure, not by causal information structure; the natural "descriptive vs structural classification" foil.

13. **Active-inference-for-LLM-agents cluster (2025–26):** Orchestrator (arXiv:2509.05651), Active Inference for Self-Organizing Multi-LLM Systems (arXiv:2412.10425), Language-Mediated Active Inference safety framework (arXiv:2508.05766). [all S]
    **Relationship: citable** — the coupled-by-philosophy camp now building LLM scaffolds; extends the Baltieri/Buckley contrast into the LLM era. None classifies or bounds anything.

## Honest dry wells (searches returning nothing on-target)

- "dual control theory LLM agents … separation principle 2025" — only exploration/exploitation-flavored LLM work (token-budget multi-agent "dual-dial," SEREN-style decoupled objectives); nobody imports Feldbaum's sense, let alone at substrate level.
- ""separation principle" OR Feldbaum OR "dual control" + "language model"" — search engine itself concluded no 2025–26 work combines the classical lineage with LLM agents; the hits are pure control theory (item 6) or pre-LLM Bayesian RL (Klenske & Hennig 2016).
- "conditional mutual information diagnostic goal belief LLM" — no information-theoretic goal-contamination diagnostic found; nearest are behavioral/Bayesian sycophancy metrics (BASIL) and CMI-from-hidden-states work for decoding, not for goal-leakage. κ appears unanticipated.
- "belief/desire disentanglement probing in LLMs" — ToM belief-representation work and knowledge-editing disentanglement exist; no goal-vs-belief architectural separation result.
- No hit anywhere on a three-way (or any-way) *classification* of agent architectures by goal↔belief coupling, and no impossibility result specifically for goal→belief separation in attention models (2606.27567 is the instruction/data analog — the closest thing in existence).

## Read-status caveat

Primaries actually fetched: 2606.27567, 2605.17634, 2601.08333, 2605.12239, 2606.27681, 2601.16644 (abstract pages / HTML — not full PDFs cover-to-cover). Everything marked [S] is snippet-grade; before *citing* any [S] item load-bearingly, pull the primary (generative-citation risk). CaMeL in particular deserves a full read before the paper leans on it — its exact guarantee statement determines how sharply the "W₁ without a theory" framing can be put.
