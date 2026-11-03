#!/usr/bin/env bash
# codex exec member. Isolated CODEX_HOME (auth symlinked); shell/exec tools disabled;
# MCP approval granted per server (codex 0.163: default_tools_approval_mode="approve").
DEFAULT_MODEL=gpt-5.6-luna DEFAULT_MAX_WAIT=${CODEX_MAX_WAIT:-600}
source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"
mkdir -p "$RUN/$SEAT/codexhome"; [[ -e "$RUN/$SEAT/codexhome/auth.json" ]] || ln -s "$HOME/.codex/auth.json" "$RUN/$SEAT/codexhome/auth.json"
PROMPT="$PROMPT

(Harness note: the council tools are on the MCP server named council; in code mode they are tools.mcp__council__council_wait, tools.mcp__council__council_submit, tools.mcp__council__council_status, tools.mcp__council__council_record.)"
cd "$RUN/$SEAT/cwd"; log_start codex; set +e
CODEX_HOME="$RUN/$SEAT/codexhome" codex exec --skip-git-repo-check --json -s read-only -m "$MODEL" \
  -c 'mcp_servers.council.command="/usr/bin/python3"' -c "mcp_servers.council.args=$(json_args)" \
  -c "mcp_servers.council.env={CONCILIUM_TOKEN=\"$TOKEN\"}" \
  -c 'mcp_servers.council.default_tools_approval_mode="approve"' -c "mcp_servers.council.tool_timeout_sec=${CODEX_TOOL_TIMEOUT:-900}" \
  -c 'web_search="disabled"' --disable shell_tool --disable unified_exec --disable multi_agent \
  "$PROMPT" < /dev/null >> "$RUN/$SEAT/stream.jsonl" 2>> "$RUN/$SEAT/stderr.txt"
log_exit $?
