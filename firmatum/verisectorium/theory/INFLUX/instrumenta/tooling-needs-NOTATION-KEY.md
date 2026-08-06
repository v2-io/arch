<!--
  Verisectorium theory INFLUX — instrumenta mining, verbatim copy.
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/udon-needs/02-tooling-needs/NOTATION-KEY.md
  Copied: 2026-08-06 (mining agent, first outside reader of the theory corpus).
  This is a copy, not authority — do not edit here; segments draft from live sources.
  NOTE: the MOVED/ tree is mid-reorg; verify the source path before drafting from it.
-->

# Notation key — vocabulary and apparatus

Two jobs: the theory vocabulary that capability cards and theory passages name their quantities in (taught in plain words at first use in chapters; collected here for reference), and the auditor apparatus — source-index codes that appear only in metadata and source notes, never in body prose. If body prose ever leans on a code from the second half, that is a defect — flag it.

## Theory quantities (ASF/AAT — full treatment reproduced in [[theory-of-agentic-tooling| the theory report]])

| Symbol | One-line meaning |
|---|---|
| **κ** (kappa) | Architectural coupling: how much the agent's goals are wired into its processing of observations. For LLMs κ ≈ 1 *by construction* (one forward pass does both) — not improvable by design. |
| **A** | Observation ambiguity: the fraction of an observation's meaning left open to goal-bendable interpretation. A test pass/fail has A ≈ 0; "does this look right?" has high A. The designer's knob. |
| **κ×A law** | Bias in the agent's beliefs is bounded by κ times A (a conditional theorem with named premises — theory report §2.1). Since κ is stuck, reduce A. |
| **Level-1 / Level-2** | Pearl's causal rungs: correlational data ("what co-occurs") vs interventional data ("what happens when I *do* X"). A chat window is L1; a tool loop is L2 — if the gates hold. |
| **do()** | Pearl's intervention operator — the formal way of saying an action *sets* a variable rather than observes it. Every tool call is a do(). |
| **C3 (the known-mechanism gate)** | The condition that upgrades tool-loop data to genuine Level-2: the agent must know the map from decision to executed action. An interface property (precise semantics + teaching refusals). |
| **T, ν, η\*** | Adaptive tempo = event rate (ν) × update quality (η\*), summed over channels. η\* collapses under channel noise: you can't outrun a bad channel by looping faster. |
| **ρ / persistence condition** | Environmental drift rate; an agent persists only while T > ρ/‖δ_critical‖ — tempo must beat drift. Below it, the agent doesn't slow down, it stops being viable. |
| **DL budget** | The context window as a joint description-length budget: strategy + world-model + task spec share one capacity (theory report, context-turnover treatment). Why compactness is structural, not cosmetic. |
| **W₁ / W₂** | The two wrapping regimes for making a coupled LLM behave like a separated agent: W₁ = separate goal-blind calls (structural guarantee), W₂ = one call with typed response fields routing belief vs strategy (behavioral guarantee). Typed response schemas *are* the W₂ mechanism. |
| **Reinjection channel** | The externalize-then-reload path (files, tracking docs, summaries) that is provably the *only* way information survives session turnover (theory report §4.1). |

## Corpus and ledger indices

| Prefix | What it indexes |
|---|---|
| **T1–T5** | The five evidence tiers (see [[method-evidence-tiers| #method-evidence-tiers]]): ideology / shipped practice / lived testimony / formal theory / external research. |
| **C1–C16** | Cross-tool convergence clusters in the shipping-practice examination ([[shipping-practice| shipping practice]]) — C1 str-replace, C2 fuzzy ladder, C7 deferred loading, C16 headless contract, …. Auditor apparatus; body prose does not use these codes. |
| **Cluster #1–18** | The cross-evidence agreement clusters in the source apparatus (CONVERGENCES) — a different numbering than C1–C16. Auditor apparatus only. |
| **P-A … P-H** | Provisional proposals from the [agent-utility exploration](../01-ideation/02-provenanced/commentary/spikes/agent-utility-NOTES.md) §8 (stage products, verdict channel, edit binding, …). Proposals, not law. |
| **D1 … D9** | Provisional boundary demands from the [addressing exploration](../01-ideation/02-provenanced/commentary/spikes/paths-NOTES.md) §8. |
| **S1 … S12** | The twelve gathered situations in [needs-map.md](../01-ideation/needs-map.md) (S1 agent-edit-under-schema-guard … S12 mid-stream reconfiguration). |
| **R1 …, W0/W1d, L0 …, C5/C6, S3/S12/S14 …, ML, PATH-1** | Ruled rows in [`v2/DECISIONS.md`](../../DECISIONS.md) and live questions in [`v2/OPEN.md`](../../OPEN.md) — the UDON language ledger. W0 = sufficiency/no-reachback at product boundaries; W1d = self-delimiting value extents; L0 = error-means-loss severity; C6 = recognition-verdict fixtures; ML = the (possibly dissolved) multi-line question. |
| **SC#N** | Steward calls in [STEWARD-CALLS.md](../01-ideation/STEWARD-CALLS.md) — questions surfaced to Joseph during gathering, with his rulings inline. |
| **"the night-spine lesson"** | The 2026-07-21 incident recorded in [pipeline-discussion.md](../pipeline-discussion.md): an autonomous session built a polished supply-side architecture ahead of demand understanding and it had to be archived wholesale. Cited here whenever a segment warns against pinning design from spikes. |

## If you're coming from the harness, not UDON

You can read this report without any UDON background. The UDON-specific vocabulary (recognition/assembly products, envelopes `<…>`, dialects, `$partial-key`, keep-everything) appears mainly in Parts III–V, and each segment's "What it generates" splits your consumer out explicitly — the harness-facing half never depends on UDON internals. The theory shorthand above is the only prerequisite layer, and the dossier it points into was written to be read standalone. Start with Part I, then VI, then II; take III–V as the notation consumer's deep-dive that you can skim for the demand statements in bold.
