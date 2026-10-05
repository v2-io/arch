#!/usr/bin/env bash
# agy -p with the Unicode tools, via a workspace-scoped plugin.
#
# The working directory gets .agents/plugins/unicode/ holding the MCP server
# (mcp_config.json) and a PreToolUse hook (hooks.json -> `utf-mcp agy-hook`) that
# allows only this server's calls and agy's turn-control steps and denies every
# other tool, logging each decision. Nothing under ~/.gemini is touched, and
# HOME is not changed (doing so makes macOS offer to reset the keychain).
#
# STATUS (agy 1.2.17, 2026-10-05): the server starts and the model calls it,
# but print mode auto-denies the call ("a tool required the "mcp" permission
# that headless mode cannot prompt for") unless an allow-rule exists in
# ~/.gemini/antigravity-cli/settings.json. Neither the hook's "allow" nor its
# permissionOverrides satisfy that check. See proposed-integration.md for the
# one-line settings change this needs (not made; it is Joseph's config).
#
# The agy server name is "<plugin>_<server>" = unicode_unicode.
CLI=agy DEFAULT_MODEL=gemini-3.6-flash-low
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

P="$WORK/.agents/plugins/unicode"; mkdir -p "$P"
printf '{ "name": "unicode", "description": "Read-only Unicode Character Database lookup (utf-mcp)." }\n' > "$P/plugin.json"
printf '{ "mcpServers": { "unicode": { "command": "%s", "args": ["--log", "%s", "--tag", "%s"] } } }\n' \
  "$UTF_MCP" "$LOG" "$TAG" > "$P/mcp_config.json"
cat > "$P/hooks.json" <<EOF
{
  "unicode-only": {
    "PreToolUse": [
      { "matcher": "*", "hooks": [ { "type": "command", "timeout": 10,
        "command": "'$UTF_MCP' --log '$LOG' --tag '$TAG' agy-hook --allow-server unicode_unicode" } ] }
    ]
  }
}
EOF
summary
cd "$WORK"
exec agy -p "$PROMPT" --model "$MODEL" --output-format json --disable-slash-commands \
  ${EXTRA[@]+"${EXTRA[@]}"} < /dev/null
