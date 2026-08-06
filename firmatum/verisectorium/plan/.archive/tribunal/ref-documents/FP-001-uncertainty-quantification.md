<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: _ref/epistemic_tribunal/documents/.../FP-001-uncertainty-quantification.md
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/_ref/epistemic_tribunal/documents/first_principles/FP-001-uncertainty-quantification.md
  Do not edit here expecting to update the live original.
-->

---
id: FP-001
domain: meta-epistemic
category: first_principles
confidence: 0.95
confidence_interval: [0.92, 0.97]
decay_rate: 0.01
empirical_support: 127
contradictions: 3
last_updated: "2024-01-15T10:30:00"
parent_principles: []
derived_principles: ["DP-001", "DP-002", "DP-003"]
---

# FP-001: Uncertainty Quantification Principle

## Statement
All knowledge claims must be expressed with calibrated confidence intervals rather than binary truth values.

## Formal Expression
∀ claim C: assess(C) → (μ, [CI_lower, CI_upper]) where CI represents 95% credible interval

## Justification
- **Empirical**: Binary classifications lead to overconfidence in 89% of tested cases (n=127)
- **Theoretical**: Bayesian reasoning requires probability distributions for proper updating
- **Practical**: Decision-making requires understanding uncertainty magnitude for risk assessment

## Application Conditions
- **Always Apply**: When evaluating any factual claim
- **Strengthen**: In high-stakes decisions or novel domains
- **Exception**: Only when formal mathematical proof exists

## Derived Principles
- DP-001: Confidence Calibration Tracking
- DP-002: Temporal Confidence Decay  
- DP-003: Evidence Quality Weighting

## Empirical Tracking
- Applications: 130
- Success Rate: 97.7%
- Last Validated: 2024-01-15
- Next Review: 2024-07-15