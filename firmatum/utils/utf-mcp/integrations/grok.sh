#!/usr/bin/env bash
# grok -p with the Unicode tools and no other tools.
#
# Isolated HOME (auth.json symlinked, the glyph-sequence harness's pattern) whose
# user-scope config.toml declares only this server; that also stops grok
# importing MCP servers, skills and hooks from ~/.claude*, and avoids the
# project-trust gate that keeps ./.grok/config.toml servers from starting.
# grok offers MCP tools through search_tool/use_tool only; --tools keeps exactly
# those two, and --allow pre-approves calls to this server (headless mode cancels
# unapproved use_tool calls). Pitfalls observed 2026-10-05 (grok 1.0.44):
# --tools "" and --tools none are ignored (full default tool set, shell
# included); --disallowed-tools run_terminal_command did not remove the shell.
#
# Verified: against a local fake model endpoint (calls reach the server only
# with a matching --allow rule; rules for other names leave them cancelled), and
# live under the user-home variant (grok-4.7-build found and called the tools).
# This exact isolated-home script has not had a live model run.
CLI=grok DEFAULT_MODEL=""
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

H="$WORK/grokhome"; mkdir -p "$H/.grok" "$WORK/cwd"
[[ -e "$H/.grok/auth.json" ]] || ln -s "$HOME/.grok/auth.json" "$H/.grok/auth.json"
cat > "$H/.grok/config.toml" <<EOF
[cli]
auto_update = false

[mcp_servers.unicode]
command = "$UTF_MCP"
args = ["--log", "$LOG", "--tag", "$TAG"]
enabled = true
EOF
summary
cd "$WORK/cwd"
MODEL_ARGS=(); [[ -n "$MODEL" ]] && MODEL_ARGS=(-m "$MODEL")
HOME="$H" exec "${GROK_BIN:-$HOME/.grok/bin/grok}" -p "$PROMPT" ${MODEL_ARGS[@]+"${MODEL_ARGS[@]}"} \
  --tools search_tool,use_tool --allow 'mcp__unicode__*' \
  --no-subagents --disable-web-search --no-plan \
  --output-format streaming-json \
  ${EXTRA[@]+"${EXTRA[@]}"} < /dev/null
