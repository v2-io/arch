# Sourced by the member launchers.  Usage of a launcher:
#   <harness>-member.sh COUNCIL_DIR SEAT RUN_DIR [MODEL]
# Env: CONCILIUM_MAX_WAIT (seconds a council_wait may block; default per harness)
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
C="$(cd "$1" && pwd)"; SEAT="$2"; RUN="$3"; MODEL="${4:-$DEFAULT_MODEL}"
mkdir -p "$RUN/$SEAT/cwd"; RUN="$(cd "$RUN" && pwd)"
TOKEN=$(/usr/bin/python3 -c "import json;print(json.load(open('$C/tokens.json'))['$SEAT'])")
VANTAGE=$(/usr/bin/python3 -c "import json;print(next(s['vantage'] for s in json.load(open('$C/charter.json'))['seats'] if s['id']=='$SEAT'))")
PROMPT=$(SEAT="$SEAT" VANTAGE="$VANTAGE" /usr/bin/python3 -c "import os;print(open('$HERE/members/brief.md').read().replace('{SEAT}',os.environ['SEAT']).replace('{VANTAGE}',os.environ['VANTAGE']))")
SERVER_ARGS=("-I" "$HERE/concilium/server.py" "--dir" "$C" "--seat" "$SEAT" "--log" "$RUN/$SEAT/server.jsonl" "--tag" "$SEAT" "--max-wait" "${CONCILIUM_MAX_WAIT:-$DEFAULT_MAX_WAIT}")
json_args() { /usr/bin/python3 -c 'import json,sys;print(json.dumps(sys.argv[1:]))' "${SERVER_ARGS[@]}"; }
log_start() { echo "$(date -u +%FT%TZ) start seat=$SEAT harness=$1 model=$MODEL pid=$$" >> "$RUN/$SEAT/launcher.txt"; }
log_exit() { echo "$(date -u +%FT%TZ) exit=$1" >> "$RUN/$SEAT/launcher.txt"; }
