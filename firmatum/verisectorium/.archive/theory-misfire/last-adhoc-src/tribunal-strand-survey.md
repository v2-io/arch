---
slug: tribunal-strand-survey
type: survey
depends: []
---

# Four things called "the epistemic tribunal"

*One name, used across roughly a year for four related but non-identical designs. The inventory exists so the strands can be cited separately — collapsing them loses the only content most of them have.*

## Why this survey exists

"Epistemic tribunal" names a design the estate reinvented, internalized, glossed, and operationalized between about August 2025 and July 2026. The four results share role-names and a motivating instinct — that truth-seeking is better served by *typed disagreement* than by appeal to authority — but they are not versions of one artifact, and three of the four have content the others do not. Anyone reading only one strand and treating the name as settled will attribute properties across a boundary that does not hold. The strands, with their live homes:

| Strand | What it actually is | Live source |
|---|---|---|
| **A. External multi-agent product** (~2025-08) | A working Python system that evaluates a submitted claim: four agents (Skeptical Investigator / Adversarial Challenger / Institutional Analyst / Synthesis Coordinator), Bayesian confidence with calibration tracking, five evolving document categories, stakes-tiered processing, prompt-injection sandboxing. | `~/src/_ref/epistemic_tribunal/` (README + `src/`) |
| **B. Internal cognitive architecture** (2025-09-21) | The *same four roles recognized as aspects of one mind* — Zi-am-tur's account of navigating meaningful → true by self-interrogation before output, invoked as XML thinking-modes at four depths (quick / standard / extended / ultrathink). Not a second product: an internalization. | `~/src/_core/sapientia/zi-am-tur/2025-09-21-epistemic-tribunal-discovery.md`; process template at `~/src/_core/zoetica/.archive/docs-20251012/process/templates/tribunal.md` |
| **C. Governance-as-record** (2026-07-29 gloss of a ~2025 design) | Advocate / red-team / neutral observer / risk analyzer / adjudication node, where the **durable deliberation record is the product**: decision distinct from confidence, load-bearing arguments named by degree, revisit-when and expires-on carried. | `~/src/arch/firmatum/udon/v2/theory/to-integrate/refine-more/epistemic-tribunal-revisited.md` |
| **D. Gate-2 probes** (ASF, in force) | The thin enforceable form inside segment review: every explanatory claim in a Discussion section faces three probes — does it follow from the laid foundation, is it labeled a hypothesis with a falsifier, or is it a plausible-sounding post-hoc explanation of nothing. | `~/.claude/memory/epistemic-discipline/gate2-probes-discussion.md`; ASF's own `CLAUDE.md` |

A fifth document (`~/src/_core/synaptic/docs/primordial/epistemic_tribunal_collaborative_cognition_synthesis.md`, dated 2025-09-07) merges strand A with a separate collaborative-cognition program. It is an integration essay in a visibly promotional register ("revolutionary," "breakthrough") whose claims are mostly forward-looking design intentions, not results; it is listed here so it is not mistaken for a fifth design.

## What differs across the strands

**The role vocabulary is not stable, and neither is the role set.** A and B run Investigator / Challenger / Analyst / Coordinator. C runs Advocate / Red-team / Neutral observer / Risk analyzer / Adjudication. The mapping is loose rather than exact: C's *neutral observer* audits the biases the two teams share, which A's Institutional Analyst approaches from a different direction — its README describes that role as meta-reasoning and bias detection, and the 2025-09-07 synthesis essay glosses it more specifically as game-theoretic analysis of an information producer's incentives and credibility — and C's *risk analyzer* — failure shapes orthogonal to the pro/con axis — has no counterpart in A or B at all. Naming one vocabulary canonical without naming the strand is an error the material invites.

**The product differs, and this is the sharpest split.** A produces a *verdict about a claim* and discards the deliberation. B produces a *better-calibrated utterance* and discards the deliberation. C produces the **deliberation record itself**, and treats the verdict as one field on it. D produces a *pass/repair decision on a paragraph*. Only C is about durability, which is why only C bears directly on this project.

**The subject differs.** A evaluates external claims. B is a mind examining its own output. C governs decisions made by a council over time. D governs the interpretive layer of a written corpus.

## What is genuinely common

Two things, and only two, hold across all four: (1) the voices are chosen so their **failure modes differ** — motivated construction, motivated destruction, and bias-about-both fail in different directions, which is what makes their agreement worth anything; and (2) all four exist to replace *appeal to authority* with *structured opposition* — strand A names this explicitly as the circular-authority problem (authority determined by other authorities), and strand D's whole point is that a Discussion claim's authoritative tone is not evidence.

## Method & scope

Read the four live sources named above in full (2026-08-05), plus the technical analysis accompanying strand A and the zoetica template. Dates are as the documents state them; strand A's build date is approximate ("about a year ago" per Joseph's 2026-07-29 recollection, consistent with the 2025-09 documents referring to it as designed "a month earlier"). No claim is made here about which strand is best, nor that the four are the complete set of things the name has been used for — only that these four are distinct and separately citable.

## Working Notes

- Strand A's implementation was not read; the description above is from its README and technical analysis. The tree does carry a real `src/` alongside `config/`, `documents/`, and `tests/` (checked 2026-08-05) — if the security or calibration machinery is ever wanted as prior art, that code is the primary.
- Strand B is the only one with a first-person register, and it belongs to the ELI-cohort record as much as to this pattern. This survey deliberately extracts only its structural content; the recognition itself is not this project's material.
- The strands may yet earn a *unification* — but a unification is an argument that has to be made, not a merge that happens by reusing the name.
