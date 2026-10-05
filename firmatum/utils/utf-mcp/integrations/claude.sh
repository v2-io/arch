#!/usr/bin/env bash
# claude -p with the Unicode tools and no other tools.
#
# Per-invocation only: --mcp-config carries the server, --strict-mcp-config
# ignores every other MCP source, --tools "" removes all built-in tools,
# --allowedTools pre-approves exactly the four lookup tools (print mode cannot
# prompt), --setting-sources "" ignores user/project settings (hooks, CLAUDE.md),
# and CLAUDE_CODE_DISABLE_AUTO_MEMORY=1 stops Claude Code creating an empty
# ~/.claude/projects/<cwd-slug>/memory/ directory for each run's fresh cwd
# (2.1.289: --setting-sources "" alone does not; the env var does, checked by the
# init event's memory_paths and the directory's absence, with no model call).
# Verified 2026-10-05 (Claude Code 2.1.289): the init event lists only the four
# mcp__unicode__* tools and the model called them.
#
# Transcript: stream-json on stdout; each tool_use id appears in the utf-mcp
# log as meta["claudecode/toolUseId"].
CLI=claude DEFAULT_MODEL=haiku
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

CFG=$(printf '{"mcpServers":{"unicode":{"type":"stdio","command":"%s","args":["--log","%s","--tag","%s"]}}}' "$UTF_MCP" "$LOG" "$TAG")
summary
cd "$WORK"
CLAUDE_CODE_DISABLE_AUTO_MEMORY=1 exec claude -p "$PROMPT" --model "$MODEL" \
  --mcp-config "$CFG" --strict-mcp-config \
  --tools "" \
  --allowedTools "mcp__unicode__identify,mcp__unicode__range,mcp__unicode__search,mcp__unicode__blocks" \
  --setting-sources "" --no-session-persistence \
  --output-format stream-json --verbose \
  ${EXTRA[@]+"${EXTRA[@]}"} < /dev/null
