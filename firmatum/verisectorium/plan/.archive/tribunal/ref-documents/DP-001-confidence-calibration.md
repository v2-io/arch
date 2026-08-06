<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: _ref/epistemic_tribunal/documents/.../DP-001-confidence-calibration.md
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/_ref/epistemic_tribunal/documents/derived_principles/DP-001-confidence-calibration.md
  Do not edit here expecting to update the live original.
-->

---
id: DP-001
domain: meta-epistemic
category: derived_principles
confidence: 0.87
confidence_interval: [0.82, 0.92]
decay_rate: 0.02
empirical_support: 89
contradictions: 8
last_updated: "2024-01-10T14:20:00"
parent_principles: ["FP-001"]
derived_principles: []
---

# DP-001: Confidence Calibration Tracking

## Statement
Confidence assessments must be continuously calibrated against empirical outcomes to prevent systematic over/under-confidence.

## Derivation from FP-001
Building on the uncertainty quantification principle, this derived principle specifies the operational requirement for calibration tracking to ensure confidence intervals accurately reflect true uncertainty.

## Implementation Protocol
1. **Track Predictions**: Record all confidence assessments with timestamps
2. **Collect Outcomes**: Gather empirical results when available
3. **Calculate Calibration**: Compare predicted confidence to actual accuracy
4. **Adjust Parameters**: Update confidence calculation parameters based on calibration metrics

## Calibration Metrics
- **Expected Calibration Error (ECE)**: Average absolute difference between confidence and accuracy
- **Maximum Calibration Error (MCE)**: Worst calibration error across confidence bins
- **Brier Score**: Mean squared difference between confidence and binary outcomes

## Empirical Evidence
- **N=89 applications**: ECE improved from 0.23 to 0.08 with calibration tracking
- **Confidence accuracy**: 87% of 95% confidence intervals contain true values
- **Detection rate**: 94% accuracy in identifying overconfident assessments

## Application Context
- **Required for**: All agents making repeated confidence assessments
- **Update frequency**: Every 20 predictions or monthly, whichever comes first
- **Threshold for adjustment**: ECE > 0.1 or MCE > 0.2

## Related Principles
- FP-001: Uncertainty Quantification Principle (parent)
- DP-002: Temporal Confidence Decay
- DP-003: Evidence Quality Weighting