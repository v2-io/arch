#!/usr/bin/env bash
# One council member = one claude -p session with only the council tools.
#   claude-member.sh COUNCIL_DIR SEAT RUN_DIR [MODEL] [extra claude args...]
# Per-run config only: --mcp-config/--strict-mcp-config, --tools "", --setting-sources "".
# Session persistence is ON (so a member can be resumed after a crash); its
# transcript lands under ~/.claude/projects/<cwd-slug>/ like any session.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
C="$(cd "$1" && pwd)"; SEAT="$2"; RUN="$3"; MODEL="${4:-haiku}"; shift 4 || shift $#
mkdir -p "$RUN/$SEAT"; RUN="$(cd "$RUN" && pwd)"
TOKEN=$(/usr/bin/python3 -c "import json,sys;print(json.load(open('$C/tokens.json'))['$SEAT'])")
VANTAGE=$(/usr/bin/python3 -c "import json;print(next(s['vantage'] for s in json.load(open('$C/charter.json'))['seats'] if s['id']=='$SEAT'))")
PROMPT=$(SEAT="$SEAT" VANTAGE="$VANTAGE" /usr/bin/python3 -c "import os;print(open('$HERE/members/brief.md').read().replace('{SEAT}',os.environ['SEAT']).replace('{VANTAGE}',os.environ['VANTAGE']))")
CFG=$(printf '{"mcpServers":{"council":{"type":"stdio","command":"/usr/bin/python3","args":["-I","%s/concilium/server.py","--dir","%s","--seat","%s","--log","%s/%s/server.jsonl","--tag","%s"],"env":{"CONCILIUM_TOKEN":"%s"}}}}' "$HERE" "$C" "$SEAT" "$RUN" "$SEAT" "$SEAT" "$TOKEN")
mkdir -p "$RUN/$SEAT/cwd"; cd "$RUN/$SEAT/cwd"
echo "$(date -u +%FT%TZ) start seat=$SEAT model=$MODEL pid=$$" >> "$RUN/$SEAT/launcher.txt"
set +e
CLAUDE_CODE_DISABLE_AUTO_MEMORY=1 claude -p "$PROMPT" --model "$MODEL" \
  --mcp-config "$CFG" --strict-mcp-config --tools "" \
  --allowedTools "mcp__council__council_wait,mcp__council__council_submit,mcp__council__council_status,mcp__council__council_record" \
  --setting-sources "" --output-format stream-json --verbose "$@" < /dev/null >> "$RUN/$SEAT/stream.jsonl" 2>> "$RUN/$SEAT/stderr.txt"
rc=$?
echo "$(date -u +%FT%TZ) exit=$rc" >> "$RUN/$SEAT/launcher.txt"
