# Goal-Update Coupling — chronology, sources, and fields

*Staging scratch (2026-08-24). Assembled from asf primaries + prior-art catalogs, with gaps filled from model training (marked). Purpose: ground the "GUC is a central blind spot in AI/agency discourse — but has deep control-theory precedent" thread that Joseph recalled from an earlier conversation, ahead of possible essay/paper work.*

## Provenance tiers used below

- **[asf]** — carried in asf canon or `ref/` prior-art catalogs with DOI (Undermind-sourced; segment marks these `[^cat-2026-05-22]`, primary-source verification still queued in the BG2 cluster — the catalogs have search support but nobody here has read most of the PDFs).
- **[verified]** — checked against the web today.
- **[training]** — from my training, plausible-tier; verify before citing in anything durable.

## What ASF already holds (the primaries I read)

`01-aat-core/src/der-directed-separation.md` is the backbone: directed separation = the epistemic update $f_M$ is goal-blind *conditional on the realized event* (selection of events may be goal-driven; processing may not be). GUC Classes: **1 Separated** (holds by construction — Kalman+LQR), **2 Partial** (κ_processing diagnostic), **3 Coupled** (fails by construction — transformer LLMs), with the 2026-05-09 renumbering warning. The segment's Related Work table already carries the classical anchors (Wonham, Witsenhausen, Bar-Shalom & Tse, Baltieri & Buckley) and the Pearl-vs-Friston-blanket positioning. `ref/prior-art-analysis/05-directed-separation.md` is the four-pillar analysis; `ref/Prior_art_for_AAT_directed_separation.md` is the raw Undermind sweep (66+ entries with abstracts).

## Chronology

### 1960s — the coupling is named, and the clean case is proven

| Year | Work | Field | Why it matters for GUC |
|---|---|---|---|
| 1960–61 | **Feldbaum, "Theory of Dual Control" I–IV**, *Avtomatika i Telemekhanika* 21(9), 21(11), 22(1), 22(3); English in *Automation and Remote Control* **[verified]** | Soviet control theory | The founding statement: control actions serve two coupled purposes — steering the state *and* probing to learn it. Optimal control must trade regulation against information gain; the two provably do not decompose in general. This is goal→update coupling named 65 years ago. |
| 1965 | Åström, "Optimal control of Markov processes with incomplete state information," *JMAA* **[asf]**; Aoki 1965 **[asf]** | Stochastic control | Belief-state (information-state) formulation — the goal-blind estimator as an object. Root of the POMDP. |
| 1968 | **Wonham, "On the Separation Theorem of Stochastic Control,"** *SIAM J. Control* 6 **[asf]** | Stochastic control | The clean case: for LQG, estimation and control factorize exactly — Kalman filter (goal-blind by construction) + LQR on its output. The mathematical anchor of GUC Class 1. |
| 1968 | Witsenhausen's counterexample (*SIAM J. Control*) **[training]** | Decentralized control | Nonclassical information patterns break everything even in linear-quadratic settings — a two-controller problem where control is also signaling. Relevant to *composite-level* GUC (routing/signaling channels), not just within-agent. |

### 1970s — exactly when separation holds, and when it can't

| Year | Work | Field | Why it matters |
|---|---|---|---|
| 1971 | **Witsenhausen, "Separation of estimation and control for discrete time systems,"** *Proc. IEEE* 59 **[asf]** | Control | General conditions for the decoupling; foundational survey-grade statement. |
| 1971 | Athans, LQG survey, *IEEE TAC* **[asf]**; Tse, "On the optimal control of stochastic linear systems" **[asf]** | Control | LQG as the design paradigm — Class 1 as the *default engineering worldview* of the era. |
| 1974 | **Bar-Shalom & Tse, "Dual effect, certainty equivalence, and separation in stochastic control,"** *IEEE TAC* 19 **[asf]** | Control | The crucial 70s finding Joseph half-remembered, almost certainly: names the **dual effect** (actions affect future observation quality) and ties certainty-equivalence / separation / no-dual-effect together. When the dual effect is present, separation *provably fails* — the classical name for the Class 3 boundary. |
| 1973–75 | Åström & Wittenmark, self-tuning regulators (*Automatica* 1973) **[training]**; variable-forgetting STR is in the adaptive-tempo catalog (Fortescue et al. 1981 **[asf]**) | Adaptive control | The engineering response: certainty-equivalent adaptive control that deliberately *ignores* the dual effect because optimal dual control is intractable ("curse of dimensionality" — flagged in Tse's abstracts). The field knew it was approximating away the coupling. |

### 1980s–2000s — the coupling goes underground

- Dual control becomes a niche: acknowledged intractable, handled by heuristics (probing signals, cautious control). Wittenmark's surveys **[training]**.
- Cognitive science builds the "classical sandwich" (perception → cognition → action) — the Class 1 worldview imported wholesale, mostly without the control-theoretic caveats.
- RL inherits the coupling as **exploration–exploitation**, but flattens it to action-selection; Bayes-adaptive MDPs (Duff 2002 **[training]**) are the honest dual-control formulation and are likewise intractable. Meanwhile motivated reasoning (Kunda 1990 **[training]**) documents the *human* κ>0 case in psychology, with no formal bridge to the control literature.

### 2010s–2020s — coupled architectures return, mostly without the old vocabulary

| Year | Work | Field | Why it matters |
|---|---|---|---|
| 2010–13 | Friston, active inference / free-energy principle **[asf]** | Comp. neuroscience | Deliberately *coupled* formulation: perception and action minimize one objective; goals absorbed into priors. Class 3 embraced as a feature. |
| 2015–16 | Tanaka, Esfahani & Mitter, "LQG Control with Minimum Directed Information" **[asf]**; Fox & Tishby, minimum-information LQG **[asf]** | Info-constrained control | Prices the estimator↔controller channel in bits — nearest formal ancestor of ASF's wrapper-leakage κ bounds. |
| 2018 | **Baltieri & Buckley, "The modularity of action and perception revisited…"** *Frontiers in Psych.* 9 **[asf]** | Control ↔ cognition | Explicitly maps modularity ↔ separation principle ↔ active inference's non-modularity. Closest single antecedent for the Class 1/3 partition. |
| ~2018 | Mesbah, dual-control revival in stochastic MPC (survey) **[training]** | Control | Dual control returns under "active uncertainty learning." |
| 2022 | Bruineberg et al., "The Emperor's New Markov Blankets," *BBS* 45 **[asf]** | Philosophy of cog-sci | Pearl-blanket vs Friston-blanket; ASF adopts the technical reading and the scope honesty. |
| 2022–23 | ReAct, Reflexion, MemGPT, Voyager, Generative Agents **[asf]** | LLM agents | The entire industry builds W₂-style wrappers around Class-3 components with *no structural theory of what the wrapping guarantees* — the blind spot in its current form. |
| 2023 | **Derpich & Yüksel, "Dual Effect, Certainty Equivalence, and Separation Revisited,"** *IEEE TAC* (DOI 10.1109/TAC.2022.3151189) **[asf]** | Control | A gem already sitting in the catalog: finds a *subtle error in Bar-Shalom & Tse's 1974 proof* that CE⇒NDE, gives a counterexample, and proposes a relaxed "dual freeness" sufficient for separation. The 50-year-old cornerstone is still being corrected — evidence the territory is live, not settled. |

## The blind-spot thesis, sharpened

The half-century of precedent is real — but note *what* it covers and what it doesn't:

1. **Classical dual control couples action→information** (my choices affect what I'll learn). It largely does **not** treat goal→*processing* coupling — Feldbaum's controller still has a goal-blind estimator; the coupling is in the *policy*. ASF's directed-separation condition is about the processing channel ($G_t \to f_M$), which the classical literature mostly assumed away by construction because its architectures *were* Class 1.
2. So the honest positioning (already in the segment's Novelty Claim) is: control theory spent 50+ years mapping when estimation/control *separate at the policy level*; the AI discourse now runs on architectures where separation fails *at the substrate level* (attention processes goals and observations together), and imported neither the vocabulary nor the no-go results. Motivated reasoning / sycophancy / hallucination discourse re-discovers κ>0 phenomenologically without the structural frame.
3. The Derpich–Yüksel correction shows even the classical policy-level story wasn't fully nailed — a nice hook for "this was never a solved problem that AI can safely ignore."

## Open verification queue (before anything durable cites these)

- All `[^cat-2026-05-22]` entries: primary-source reads still queued (segment Working Notes, BG2 cluster).
- My `[training]` entries: Witsenhausen 1968 counterexample citation; Åström & Wittenmark 1973; Mesbah survey; Duff 2002; Kunda 1990.
- Whether anyone has *already* written the "AI agents ignore dual control" bridge paper — candidates to search: recent dual-control-for-RL and "exploration as dual control" literature; `ref/prior-art-analysis/agentic-systems-landscape-2026-08-22.md` may already touch it.
