#!/usr/bin/env bash
# grok -p with the probe server; isolated HOME (auth symlinked), nothing global.
#   grok-probe.sh RUNDIR "PROMPT" [extra grok args...]
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUN="$1"; shift; PROMPT="$1"; shift
mkdir -p "$RUN/grokhome/.grok" "$RUN/cwd"; RUN="$(cd "$RUN" && pwd)"
[[ -e "$RUN/grokhome/.grok/auth.json" ]] || ln -s "$HOME/.grok/auth.json" "$RUN/grokhome/.grok/auth.json"
cat > "$RUN/grokhome/.grok/config.toml" <<TOML
[cli]
auto_update = false

[mcp_servers.probe]
command = "/usr/bin/python3"
args = ["-I", "$HERE/probe_mcp.py", "--log", "$RUN/probe.jsonl", "--tag", "$(basename $RUN)"]
enabled = true
TOML
cd "$RUN/cwd"
HOME="$RUN/grokhome" "$HOME/.grok/bin/grok" -p "$PROMPT" ${MODEL:+-m "$MODEL"} \
  --tools search_tool,use_tool --allow 'mcp__probe__*' --no-subagents --disable-web-search --no-plan \
  --output-format streaming-json "$@" < /dev/null > "$RUN/stream.jsonl" 2> "$RUN/stderr.txt"
echo "exit=$?" >> "$RUN/stderr.txt"
