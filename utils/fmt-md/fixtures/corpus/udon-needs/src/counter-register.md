---
slug: counter-register
type: counterposition
evidence: [T2, T3, T5]
status: standing-register (each row carries its own weight)
stage: drafted
consumers: both
depends: [method-evidence-tiers]
sources:
  - ../../01-ideation/02-provenanced/syntheses/CONVERGENCES.md  # singletons
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # Part D
  - ../../01-ideation/02-provenanced/syntheses/external-landscape-2026-07.md  # findings 3, 5, 6
  - ../../01-ideation/STEWARD-CALLS.md  # #6
---

# The counter-register: evidence against our own theses

Kept as a first-class segment so the monograph cannot quietly firm up its
own caveats. Each row names the thesis it qualifies and where the full
context lives. **A downstream decision that touches one of these theses
should cite the counter-row, not just the supporting segment.**

| # | Counter-evidence | Qualifies | Weight |
|---|---|---|---|
| 1 | **SAR alignment-speed non-reproduction.** The sar2 experiment (2025) measured agent comprehension of the same content in aligned/structured notation vs conventional prose: the headline internal result was 100% vs 60% immediate comprehension favoring the structured form — but the effect **failed to reproduce on 1 of 4 model families tested**, and the same run produced an honest latency counter-result (the structured form was *slower* to process in that family). Joseph's ruling on it: carry as important evidence; confounding factors suspected in the original; feeds the eventual house-style discussion and, via what it missed, experiment methodology. | Any "aligned/structured notation improves agent comprehension" claim | Single experiment, but the only *measured* internal test of the thesis in this report's evidence — outranks any amount of design conviction |
| 2 | **Obsidian's deliberate anti-nesting Properties stance** — no nesting, no markdown-in-properties, *intentionally*: "properties are meant for small, atomic bits… human and machine readable." A widely-deployed, considered counter-position to attribute-values-as-nodes. | UDON's edge-may-terminate-at-a-node model | An argument to answer (their constraint serves a UI/simplicity goal UDON doesn't share — but say why, don't wave it off) |
| 3 | **BFCL: structured modes ≠ fewer errors** — more incorrect calls than free-text in the multiple-call category; error *profile* changes. | Naive "structure improves reliability" | Medium confidence (2-1, one category) — scoping discipline, not refutation of structure |
| 4 | **aider's tool-call editing abandonment** — the codebase's own `RuntimeError("Deprecated")` marks where JSON-function editing was tried and killed after models mangled structured arguments. | Any plan to route edits through JSON tool-call arguments | Family/era-scoped by row 11: strong *within* the Claude/OpenAI-lineage ecosystem this corpus samples; not a universal law |
| 5 | **Fail-plausible + tests-don't-predict** — ~70% of silent production failures caught by a human *using* the product, ~none by 4,286 tests ex-ante ("audits are regression engines, not prediction engines"); errors become confident false output. | Any claim that schema/validation layers *catch* agent failure (they catch malformation, not plausible wrongness); also qualifies #tools-are-observation-infrastructure's A≈0 story — validity ≠ truth | Medium (single-system case study) but echoed contemporaneously; the harness's human-verification surfaces exist because of exactly this |
| 6 | **kimi-code: AGENTS.md is untrusted data** — injection-precedence rules vs the ecosystem's trusted-instruction consensus. | The instruction-file trust model (#tool-definition-anatomy) | Live disagreement, unresolved; security-relevant |
| 7 | **Non-composing sub-skills / non-linear compounding** (external survey spanning 27 papers): scaffolding does not uniformly help; tool discipline is model-family-specific, not scale-explained. | Any "better tooling fixes reliability" framing; also model-agnostic tool-contract assumptions (#edit-representation-landscape's routing finding is the shipped echo) | High confidence, preprint-era caveats noted |
| 8 | **Self-chunking is unmeasured** for UDON specifically — an adjacent code-retrieval experiment (sar3) supports parsing-based chunking generally; UDON's own claim has no measurement. Claim-or-kill experiment specified in #self-chunking-status. | The README-level self-chunking pitch | Absence-of-evidence row: the thesis is live, the *claim* is currently over-dressed |
| 9 | **One-shot tool constraint** — tools can't call back mid-execution (a limit both recorded design discussions and an agent's own self-correction account ran into): interactive-confirmation and mid-edit-clarification designs must survive this reality or state their transport assumption. | Any design assuming conversational tools | Realism check; transport-dependent |

| 10 | **Prefix-parseable ≠ streaming transport** (codex review, 2026-07-22 -- cross-substrate practitioner dissent): a format whose partial payloads parse honestly does not thereby provide framing, sequencing, cancellation, or multiplexing; NDJSON solves transport, prefix-parsing solves payload validation, and neither replaces the other absent a protocol experiment. | Any "UDON can serve the NDJSON role natively" claim (#headless-io-contract, #streaming-and-partial-documents) | Practitioner-architecture reasoning, no measurement either way; adopted as the working framing |
| 11 | **Tool-call JSON editing is not universally abandoned** (agy/Gemini-ecosystem review, 2026-07-22 -- cross-substrate dissent): structured tool-call editing with schema validation at the tool layer is the default, successful, primary edit modality in the Gemini/Antigravity ecosystem; the aider abandonment is best read as an artifact of specific model families and eras, not a law. | #edit-representation-landscape's abandonment framing | Materially scopes a Tier-2 conclusion; first genuinely cross-substrate counter-weight in the register |

Rows 1–2 and 8 are UDON-facing; 3–7 and 9–11 harness-facing (10–11 carry
cross-substrate attribution — independent counter-weight from outside the
model family that produced most of this report's other evidence); all
travel with both consumers.
