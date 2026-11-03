#!/usr/bin/env bash
# One claude -p HOST session holding the council server in multi-seat mode
# (no --seat), which spawns one subagent per seat. The subagents reference the
# host's server by name, so (per Claude Code docs) they share its one stdio
# connection; each identifies its seat only by the seat_token it is given.
#   claude-subagent-host.sh COUNCIL_DIR RUN_DIR [MEMBER_MODEL]
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
C="$(cd "$1" && pwd)"; RUN="$2"; MM="${3:-haiku}"
mkdir -p "$RUN/host/cwd"; RUN="$(cd "$RUN" && pwd)"
CFG=$(printf '{"mcpServers":{"council":{"type":"stdio","command":"/usr/bin/python3","args":["-I","%s/concilium/server.py","--dir","%s","--log","%s/host/server.jsonl","--tag","host"]}}}' "$HERE" "$C" "$RUN")
BRIEF=$(sed -e 's/{SEAT}/the seat named in your task/' -e 's/{VANTAGE}/(given in your task)/' "$HERE/members/brief.md")
AGENTS=$(BRIEF="$BRIEF" MM="$MM" /usr/bin/python3 -c "
import json,os
print(json.dumps({'council-member':{
 'description':'Holds one seat on an agentic council and takes part until the meeting adjourns.',
 'prompt': os.environ['BRIEF'] + '\n\nYou act through the council tools only. Every council tool call must carry seat_token set to the exact token in your task.',
 'tools':['mcp__council__council_wait','mcp__council__council_submit','mcp__council__council_status','mcp__council__council_record'],
 'model': os.environ['MM']}}))")
TASKS=$(/usr/bin/python3 -c "
import json
ch=json.load(open('$C/charter.json')); tk=json.load(open('$C/tokens.json'))
for s in ch['seats']:
    print(f\"- seat {s['id']} (vantage: {s['vantage']}); seat_token: {tk[s['id']]}\")")
PROMPT="You are hosting a council meeting; you do not hold a seat yourself. Launch one council-member subagent per seat below, all in a single message so they run at the same time. Give each its seat id, its vantage and its seat_token exactly as written, and nothing else is needed: they know how the meeting works. When all of them have returned, report in a few lines what each said about how the meeting went. Do not call the council tools yourself.

$TASKS"
cd "$RUN/host/cwd"
echo "$(date -u +%FT%TZ) start host" >> "$RUN/host/launcher.txt"
set +e
CLAUDE_CODE_DISABLE_AUTO_MEMORY=1 claude -p "$PROMPT" --model "${HOST_MODEL:-haiku}" \
  --mcp-config "$CFG" --strict-mcp-config --tools "Agent" --agents "$AGENTS" \
  --allowedTools "Agent,mcp__council__council_wait,mcp__council__council_submit,mcp__council__council_status,mcp__council__council_record" \
  --setting-sources "" --output-format stream-json --verbose < /dev/null >> "$RUN/host/stream.jsonl" 2>> "$RUN/host/stderr.txt"
echo "$(date -u +%FT%TZ) exit=$?" >> "$RUN/host/launcher.txt"
