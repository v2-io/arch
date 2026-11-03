#!/usr/bin/env bash
# gemini -p with the probe server via a project-scoped .gemini/settings.json in the run cwd.
#   gemini-probe.sh RUNDIR "PROMPT" [extra gemini args...]
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUN="$1"; shift; PROMPT="$1"; shift
mkdir -p "$RUN/cwd/.gemini"; RUN="$(cd "$RUN" && pwd)"
cat > "$RUN/cwd/.gemini/settings.json" <<JSON
{"mcpServers": {"probe": {"command": "/usr/bin/python3",
  "args": ["-I", "$HERE/probe_mcp.py", "--log", "$RUN/probe.jsonl", "--tag", "$(basename $RUN)"],
  "trust": true, "timeout": ${GEMINI_TIMEOUT_MS:-600000}}}}
JSON
cd "$RUN/cwd"
gemini -p "$PROMPT" ${MODEL:+-m "$MODEL"} --allowed-mcp-server-names probe -o stream-json "$@" < /dev/null > "$RUN/stream.jsonl" 2> "$RUN/stderr.txt"
echo "exit=$?" >> "$RUN/stderr.txt"
