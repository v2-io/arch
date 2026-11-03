#!/usr/bin/env bash
# grok -p member. Isolated HOME (auth symlinked) whose config declares only the council server.
DEFAULT_MODEL="" DEFAULT_MAX_WAIT=${GROK_MAX_WAIT:-600}
source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"
H="$RUN/$SEAT/grokhome"; mkdir -p "$H/.grok"; [[ -e "$H/.grok/auth.json" ]] || ln -s "$HOME/.grok/auth.json" "$H/.grok/auth.json"
cat > "$H/.grok/config.toml" <<TOML
[cli]
auto_update = false

[mcp_servers.council]
command = "/usr/bin/python3"
args = $(json_args)
env = { CONCILIUM_TOKEN = "$TOKEN" }
enabled = true
TOML
cd "$RUN/$SEAT/cwd"; log_start grok; set +e
HOME="$H" "$HOME/.grok/bin/grok" -p "$PROMPT" ${MODEL:+-m "$MODEL"} \
  --tools search_tool,use_tool --allow 'mcp__council__*' --no-subagents --disable-web-search --no-plan \
  --output-format streaming-json < /dev/null >> "$RUN/$SEAT/stream.jsonl" 2>> "$RUN/$SEAT/stderr.txt"
log_exit $?
