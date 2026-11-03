#!/usr/bin/env bash
# codex exec with the probe server; isolated CODEX_HOME (auth symlinked), nothing global.
#   codex-probe.sh RUNDIR "PROMPT" [extra codex args...]
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUN="$1"; shift; PROMPT="$1"; shift
mkdir -p "$RUN/codexhome" "$RUN/cwd"; RUN="$(cd "$RUN" && pwd)"
[[ -e "$RUN/codexhome/auth.json" ]] || ln -s "$HOME/.codex/auth.json" "$RUN/codexhome/auth.json"
cd "$RUN/cwd"
CODEX_HOME="$RUN/codexhome" codex exec --skip-git-repo-check --json -s read-only -m "${MODEL:-gpt-5.5}" \
  -c "mcp_servers.probe.command=\"/usr/bin/python3\"" \
  -c "mcp_servers.probe.args=[\"-I\",\"$HERE/probe_mcp.py\",\"--log\",\"$RUN/probe.jsonl\",\"--tag\",\"$(basename $RUN)\"]" \
  -c "mcp_servers.probe.default_tools_approval_mode=\"approve\"" -c "mcp_servers.probe.tool_timeout_sec=${TOOL_TIMEOUT:-60}" -c 'web_search="disabled"' --disable shell_tool --disable unified_exec --disable multi_agent \
  "$@" "$PROMPT" < /dev/null > "$RUN/stream.jsonl" 2> "$RUN/stderr.txt"
echo "exit=$?" >> "$RUN/stderr.txt"
