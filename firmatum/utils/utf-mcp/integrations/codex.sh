#!/usr/bin/env bash
# codex exec with the Unicode tools and as few other tools as codex allows.
#
# Isolated CODEX_HOME (auth.json symlinked, the pattern the glyph-sequence
# harness already uses), the server added with -c overrides, and every built-in
# that has a feature switch disabled. What remains depends on the model:
#
#  - gpt-5.5 and older ("classic" tools): list/read_mcp_resource(s),
#    request_user_input, apply_patch (refused by the read-only sandbox), and
#    tool_search. The unicode tools are deferred: the model finds them with
#    tool_search and calls them as namespace mcp__unicode.
#  - gpt-5.6-* / gpt-6-* (model catalog tool_mode "code_mode_only"): one `exec`
#    tool that runs JavaScript in a V8 isolate (no fs/network). The unicode tools
#    are tools.mcp__unicode__identify(...) etc., listed only in ALL_TOOLS; the
#    model is not told they exist. The isolate itself can do code-point
#    arithmetic and \p{...} property regexes without the server.
#
# Verified 2026-10-05 (codex-cli 0.157.1) against a local fake model endpoint
# (probes/fake_responses_server.py), not a live model: the account's codex quota
# was exhausted until 2026-11-02. Rollouts are kept (no --ephemeral) because
# they are the only record of code-mode JavaScript; codex --json does not show it.
CLI=codex DEFAULT_MODEL=gpt-5.5
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

HOMEDIR="$WORK/codexhome"; mkdir -p "$HOMEDIR"
[[ -e "$HOMEDIR/auth.json" ]] || ln -s "$HOME/.codex/auth.json" "$HOMEDIR/auth.json"
summary; echo "utf-mcp[$CLI]: rollouts under $HOMEDIR/sessions" >&2
cd "$WORK"
CODEX_HOME="$HOMEDIR" exec codex exec --skip-git-repo-check --ignore-rules --json -s read-only -m "$MODEL" \
  -c "mcp_servers.unicode.command=\"$UTF_MCP\"" \
  -c "mcp_servers.unicode.args=[\"--log\",\"$LOG\",\"--tag\",\"$TAG\"]" \
  -c 'web_search="disabled"' \
  --disable shell_tool --disable unified_exec --disable multi_agent --disable browser_use \
  --disable computer_use --disable image_generation --disable apps --disable goals \
  --disable view_image --disable skill_search --disable tool_suggest --disable sleep_tool \
  ${EXTRA[@]+"${EXTRA[@]}"} "$PROMPT" < /dev/null
