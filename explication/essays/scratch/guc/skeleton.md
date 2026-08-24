# FAST paper skeleton — PROVISIONAL SKETCH

> [!warn] **This is a provisional sketch, not an outline of record.** It exists to give the outline work a concrete strawman. It is waiting on, and expected to change under, all four of:
>
> - **(a) Adjacent/prerequisite ASF concept needs** — which upstream definitions (complete agent state, orient cascade, event/observation machinery) the paper must carry inline vs. compress vs. cite, and whether any concept needs new work before it can stand alone outside the corpus.
> - **(b) Full ASF-segment outline substrate** — a real pass over the segments themselves (`der-directed-separation`, `der-class-coercion-via-wrapping`, `der-class-coercion-in-composition`, `hyp-directed-separation-under-composition`, `disc-partial-coupling-pathways`, `disc-w1-structural-bound-boundary`, `disc-dynamic-regime-axis`, 03-llm-core's `def-coupled-update-dynamics` / `der-logogenic-as-wrapping`, NeurIPS Paper 3 `#lem-attention-coupled`) rather than the one-segment read this sketch is built from.
> - **(c) Deep dive of currently known & fetched literature** — relata now holds `pant-2026-inseparability`, `debenedetti-2025-defeating`, `abdelnabi-2026-agents` (and `hafez-2026-informational-cost`, markdown re-converting for mangled math); none read whole yet. Sweep-agent report had six primaries fetched, rest snippet-grade.
> - **(d) Fuller literature survey** — the 2026-08 sweep was a couple-of-agents pass, not Undermind-grade; the classical `[^cat-2026-05-22]` citations (Wonham, Witsenhausen, Bar-Shalom & Tse, Derpich & Yüksel) are still primary-unverified.
>
> Section titles, emphasis allocation, and even the claim-set may move. Nothing below is drafted prose; nothing below is committed scope.

**Target:** FAST @ NeurIPS 2026 (Paris), full paper, ≤7pp excl. refs, NeurIPS template, double-blind, due Aug 29 AoE. Non-archival; under-submission-elsewhere explicitly allowed.

**Working title candidates** (all provisional): "Directed Separation: When Can an Agent's Beliefs Be Insulated From Its Goals?" / "Goal-Update Coupling in LLM Agents: A Classification and Its Fifty-Year Prehistory" / shorter, for the short-paper fallback: "LLM Agents Are Coupled Systems."

---

## §1 Introduction (~1pp)

- Hook: one mechanism, many names — sycophancy, motivated reasoning, prompt injection becoming belief, hallucination-as-level-confusion — all are the same structural fact: nothing in the architecture keeps the goal out of the belief-update.
- The distinction that organizes everything: goals may select *which* events arrive (legitimate, unavoidable); the question is whether they shape *how* a realized event is processed. Selection ≠ processing.
- Thesis: (i) this admits a discrete architectural classification, (ii) transformers fail separation by construction, (iii) control theory mapped the policy-level version of this for 50 years and the AI discourse imported neither the vocabulary nor the no-go results, (iv) scaffolds can restore separation at the composite level, with a precise account of certificate vs. promise and the cost paid.
- ⚠ (b): how much of the hook can lean on Paper 3's lemma vs. restate.

## §2 Directed separation and the three classes (~1.5pp)

- Minimal formal core: state split $(M_t, G_t)$, update maps $f_M$ (no $G$ argument) / $f_G$ / policy $\pi$; the conditional-independence statement $M_{\tau^+} \perp G_t \mid (M_{\tau^-}, e_\tau)$.
- GUC Class 1 Separated / 2 Partial / 3 Coupled; classes as positions of one coupling distinguished by *what is certifiable*, not virtue ranking. κ_processing as the Partial-case diagnostic + the behavioral estimator κ̂ (same-event/different-goal probing).
- The two boundaries are different kinds: certifiability boundary (1↔2) vs. behavioral boundary (2↔3). This is the paper's most reusable conceptual tool.
- Pearl-blanket positioning, one paragraph (adopt the conditional-independence content, decline the metaphysics; Bruineberg et al. as the honest-scope model).
- ⚠ (a): how much of `form-complete-agent-state` must ride along; whether κ's denominator-support convention (open Working-Note item) must be settled before publishing the definition.

## §3 Transformers are Class 3 by construction (~0.5–1pp)

- The structural claim, at the strength Paper 3 established: attention processes goals and observations together; robust to normalization/masking/efficiency variants; extends to SSMs/linear attention under non-degeneracy.
- ⚠ (b)+(d): double-blind handling of the Paper 3 citation; check Paper 3's live status. Contrast with pant-2026-inseparability (provenance axis) goes here or §5.

## §4 The fifty-year prehistory, and why it doesn't cover this (~1pp)

- Compressed chronology (from `chronology.md`): Feldbaum's dual control → Wonham/Witsenhausen separation → Bar-Shalom & Tse dual effect → (optionally) Derpich & Yüksel 2023 correction as evidence the classical story is still live.
- The sharpened blind-spot claim: classical coupling is action→information (policy level, estimator clean by construction); the modern failure is goal→processing (substrate level). The old results are the *right ancestors* and the *wrong coverage* — which is why the vocabulary never transferred.
- ⚠ (d): all four classical anchors need primary verification before this section makes dated claims.

## §5 What the field keeps rediscovering (~0.5–1pp)

- The 2026 near-miss cluster as evidence of ripeness: pant-2026-inseparability (impossibility on the provenance axis), CaMeL/debenedetti-2025-defeating (an engineered wrapper with "provable security" and an unpriced utility tax), abdelnabi-2026-agents, sycophancy mech-interp probes, strict-mediation belief-state work. Each is one face; none has the classification, the lineage, or the diagnostic.
- ⚠ (c): every sentence here is gated on reading those primaries whole.

## §6 Restoring separation by construction: wrapping (~1–1.5pp)

- Class coercion: W₀/W₁/W₂; Class-1-by-structure vs Class-1-by-behavior (does the belief-update query carry the goal?); the leakage bounds (structural for W₁ via DPI, behavioral-only for W₂); the tempo cost (more component calls per macro-step).
- CaMeL slotted as a found W₁-instance: its measured utility tax is the tempo cost, unnamed; its "provable security" is a certificate-class claim the hierarchy makes precise.
- Hafez IDT sidecar as the monitoring-side instance (system-vs-component separation) — ⚠ (c): gated on the re-OCR'd primary confirming 89%/44% and the (S,A,S′) characterization.

## §7 The lift to populations (~0.5–1pp) — *the FAST-audience section*

- Composite-level inheritance: what flips a composite's class is routing goal-dependence and shared substrate — *not* goal misalignment (Cournot witness; the withdrawn "organizations are natively Class 3" claim stays withdrawn — regression guard from segment Working Notes).
- Architectural class vs. dynamic regime as separate axes; consequences for observability/steerability of agent populations (CFP topic hit).
- ⚠ (b): this compresses `hyp-directed-separation-under-composition` + `disc-dynamic-regime-axis`; needs its own substrate pass.

## §8 Discussion & limits (~0.5pp)

- Honest scope: κ not computable in closed form (diagnostic, not measurement); behavioral compliance adversarially fragile; bounded-signaling assumption named; where goal-directedness is *correct* (model-space exploration) so the condition isn't read as "goals bad."
- Feedback asks, stated as such: the classification's edge cases, the κ̂ protocol, what the community would want measured first.

## Appendix candidates (if full-paper)

- κ̂ estimator protocol details + confounds (the $M_{\tau^-}$-held-fixed requirement).
- Extended chronology table.
- Inheritance table in full.

---

## Claims-with-provenance ledger (to be completed at (b))

| Claim | Source | Status in canon | Paper-side need |
|---|---|---|---|
| Directed-separation conditional claim | `der-directed-separation` | exact (conditional) | restate |
| Class 1/2/3 partition + two-boundary-kinds | same | robust qualitative | restate |
| Transformers Class 3 | Paper 3 `#lem-attention-coupled` | lemma-grade | cite (double-blind handling TBD) |
| W₁/W₂ theorems + DPI bound | `der-class-coercion-via-wrapping` | conditional (C1–C3) | cite + state conditions honestly |
| Tempo cost | `der-class-coercion-in-composition` | conditional | cite |
| Composite inheritance table | `der-directed-separation` §Discussion + `hyp-…-under-composition` | robust qualitative / hypothesis — check | ⚠ verify tier before stating |
| Classical anchors (4) | ref catalogs | `[^cat]` unverified | primary reads (d) |
| Near-miss cluster (5+) | 2026-08 sweep | snippet-to-primary mixed | primary reads (c) |
| Hafez IDT numbers | relata pdf (re-converting) | unverified | primary read (c) |
