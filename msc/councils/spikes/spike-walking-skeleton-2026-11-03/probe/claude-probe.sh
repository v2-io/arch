#!/usr/bin/env bash
# claude -p with only the probe server's tools. Per-run config, nothing global.
#   claude-probe.sh RUNDIR "PROMPT" [extra claude args...]
# Env passes through, so per-process knobs (MCP_TOOL_TIMEOUT,
# CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT, CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS) can be
# set on the command line for one run.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUN="$1"; shift; PROMPT="$1"; shift
mkdir -p "$RUN/cwd"; RUN="$(cd "$RUN" && pwd)"
SERVER_TIMEOUT="${PROBE_SERVER_TIMEOUT:-}"
if [[ -n "$SERVER_TIMEOUT" ]]; then
  CFG=$(printf '{"mcpServers":{"probe":{"type":"stdio","command":"/usr/bin/python3","args":["-I","%s/probe_mcp.py","--log","%s/probe.jsonl","--tag","%s"],"timeout":%s}}}' "$HERE" "$RUN" "$(basename "$RUN")" "$SERVER_TIMEOUT")
else
  CFG=$(printf '{"mcpServers":{"probe":{"type":"stdio","command":"/usr/bin/python3","args":["-I","%s/probe_mcp.py","--log","%s/probe.jsonl","--tag","%s"]}}}' "$HERE" "$RUN" "$(basename "$RUN")")
fi
echo "$CFG" > "$RUN/mcp-config.json"
cd "$RUN/cwd"
CLAUDE_CODE_DISABLE_AUTO_MEMORY=1 claude -p "$PROMPT" --model "${MODEL:-haiku}" \
  --mcp-config "$CFG" --strict-mcp-config \
  --tools "" --allowedTools "mcp__probe__probe_echo,mcp__probe__probe_sleep" \
  --setting-sources "" --no-session-persistence \
  --output-format stream-json --verbose "$@" < /dev/null > "$RUN/stream.jsonl" 2> "$RUN/stderr.txt"
echo "exit=$?" >> "$RUN/stderr.txt"
