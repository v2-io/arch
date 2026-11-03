#!/usr/bin/env bash
# agy -p member. Workspace plugin carries the server; the MCP allow rule is passed
# session-only with --settings (agy 1.4.12); utf-mcp's tested agy-hook denies every
# tool but this server's calls and agy's turn-control steps, and logs each decision.
DEFAULT_MODEL=gemini-3.8-flash-low DEFAULT_MAX_WAIT=${AGY_MAX_WAIT:-600}
source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"
UTF=/Users/josephwecker-v2/src/arch/firmatum/utils/utf-mcp/target/release/utf-mcp
P="$RUN/$SEAT/cwd/.agents/plugins/council"; mkdir -p "$P"
printf '{ "name": "council", "description": "agentic council member tools (concilium)" }\n' > "$P/plugin.json"
/usr/bin/python3 -c "
import json,sys
json.dump({'mcpServers':{'council':{'command':'/usr/bin/python3','args':json.loads(sys.argv[1]),'env':{'CONCILIUM_TOKEN':sys.argv[2]}}}},open('$P/mcp_config.json','w'))
json.dump({'council-only':{'PreToolUse':[{'matcher':'*','hooks':[{'type':'command','timeout':10,
  'command':\"'$UTF' --log '$RUN/$SEAT/agy-hook.jsonl' --tag $SEAT agy-hook --allow-server council_council\"}]}]}},open('$P/hooks.json','w'))
" "$(json_args)" "$TOKEN"
cd "$RUN/$SEAT/cwd"; log_start agy; set +e
agy -p "$PROMPT" --model "$MODEL" --output-format stream-json --disable-slash-commands \
  --settings '{"permissions":{"allow":["mcp(council_council/*)"]}}' < /dev/null >> "$RUN/$SEAT/stream.jsonl" 2>> "$RUN/$SEAT/stderr.txt"
log_exit $?
