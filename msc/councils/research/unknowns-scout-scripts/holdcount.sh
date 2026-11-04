#!/bin/bash
# Count holding check marks over the 17 segments that hold checks in ASF at 2133cf2e.
# Run inside a store. Arg: strict (only ✓) or all (✓ and ✓~). Default all.
pat='^✓'; [ "$1" = strict ] && pat='^✓ '
n=0
for s in def-adaptive-tempo def-action-transition def-model-sufficiency def-observation-function der-causal-insufficiency-detection der-causal-hierarchy-requirement der-directed-separation der-turnover-information-recursion disc-continuity-stance disc-constructive-impossibility-posture form-composition-closure example-kalman result-adversarial-tempo-advantage result-coupled-diagnostic-framework schema-strategy-persistence scope-developer-agent scope-adaptive-system; do
  c=$(vsect read segment:$s 2>/dev/null | grep -cE "$pat"); n=$((n+c)); done
echo "holding marks (${1:-all}) over the 17: $n"
