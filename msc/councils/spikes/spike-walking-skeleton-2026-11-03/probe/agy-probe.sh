#!/usr/bin/env bash
# agy -p with the probe server as a workspace plugin; the MCP allow rule passed
# with --settings (session-only, never saved -- agy 1.4.12). Nothing global.
#   agy-probe.sh RUNDIR "PROMPT" [extra agy args...]
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUN="$1"; shift; PROMPT="$1"; shift
P="$RUN/cwd/.agents/plugins/probe"; mkdir -p "$P"; RUN="$(cd "$RUN" && pwd)"; P="$RUN/cwd/.agents/plugins/probe"
printf '{ "name": "probe", "description": "harness probe" }\n' > "$P/plugin.json"
printf '{ "mcpServers": { "probe": { "command": "/usr/bin/python3", "args": ["-I", "%s/probe_mcp.py", "--log", "%s/probe.jsonl", "--tag", "%s"] } } }\n' "$HERE" "$RUN" "$(basename $RUN)" > "$P/mcp_config.json"
cd "$RUN/cwd"
agy -p "$PROMPT" --model "${MODEL:-gemini-3.6-flash-low}" --output-format stream-json --disable-slash-commands \
  --settings "${AGY_SETTINGS:-{\"permissions\":{\"allow\":[\"mcp(probe_probe/*)\"]}}}" "$@" < /dev/null > "$RUN/stream.jsonl" 2> "$RUN/stderr.txt"
echo "exit=$?" >> "$RUN/stderr.txt"
