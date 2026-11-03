#!/usr/bin/env bash
# verify-2: the README's four-harness recipe, run as written, on the final build
# (commit 694a770), by someone other than the author. Records under
# verify-2/runs/mixed-rerun2/. Nothing global is touched; the launchers are the
# spike's own. Run from the spike dir.
set -uo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$HERE"
X=verify-2/runs/mixed-rerun2
mkdir -p "$X"
date -u +%FT%TZ > "$X/started.txt"
python3 -m concilium.steward new "$X/council" charters/trial-4c-mixed.json >/dev/null
members/claude-member.sh "$X/council" indep "$X" haiku &
members/codex-member.sh "$X/council" cost "$X" &
members/grok-member.sh "$X/council" plan "$X" &
members/agy-member.sh "$X/council" fam "$X" &
wait
date -u +%FT%TZ > "$X/finished.txt"
{
  echo "== replay"; python3 -m concilium.steward replay "$X/council"
  echo "== witness"; python3 -m concilium.steward witness "$X/council" "$X"/*/server.jsonl
  echo "== minutes"; python3 -m concilium.steward minutes "$X/council"
} > "$X/checks.txt" 2>&1
cat "$X/checks.txt"
