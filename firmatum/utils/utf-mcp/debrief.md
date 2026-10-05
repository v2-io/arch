# Debrief: utf-mcp, 2026-10-05

*To Joseph, from the Claude Opus 5.5 agent that built it.*

## What exists

`utf-mcp` is a Rust binary of about 20 MB with the Unicode 18.0.0 database compiled in. It speaks MCP over stdio, written by hand: about 150 lines of protocol, no async runtime, and serde_json as the only dependency. It has four read-only tools. `identify` handles glyphs, code points or names in a batch. `range` covers code-point neighbours, the arithmetic Gemini kept trying in Python. `search` looks up names, aliases, block names and, optionally, Han definitions. `blocks` gives a block's code-chart headings. Every call goes into a JSON-lines log with its exact arguments and the exact text returned, tagged for joining to the harness. `integrations/` has a launch script per CLI. `README.md` is the cold-reader entry and `proposed-integration.md` holds suggestions for the study.

## Where each CLI stands

- **claude -p:** done and live-verified. Only the four tools exist in the session.
- **grok -p:** works live. The script version (isolated `HOME`, `--tools search_tool,use_tool --allow 'mcp__unicode__*'`) is verified against a fake model endpoint; it has not had a live run.
- **codex exec:** wiring verified end to end against a fake Responses endpoint in both of codex's tool regimes. The account's codex quota is exhausted until 2026-11-02, so no live model has called it. Two facts change what codex results mean. Newer models (gpt-5.6+, gpt-6) get only a JavaScript sandbox, with MCP tools hidden inside it. And that sandbox's own work is visible only in rollout files.
- **agy -p:** the server starts and Gemini calls it correctly, but print mode auto-denies the call. Unblocking it takes one allow-rule in `~/.gemini/antigravity-cli/settings.json`, which I did not touch. The exact line is in `proposed-integration.md` §3. A workspace hook couldn't grant the permission (tested two ways).

## Something I got wrong

On my first live grok call I passed `--tools ""`, believing it removed grok's tools. It doesn't, and your grok config auto-approves, so the model had a shell and used it three times: `python3 -c 'import unicodedata …'`, in my scratch directory, reading only. Nothing was written or deleted. I should have checked the tool list before giving an unattended model a prompt. Afterwards I found a way to see each CLI's offered tools without spending a model call (`probes/fake_responses_server.py`), and every later grok and codex configuration was checked that way first.

## Correctness

Beyond the unit and protocol tests, every code point's core properties were compared with two independent UCD implementations: Python 3.14's `unicodedata` (16.0) and Perl's `Unicode::UCD` (13.0). Every difference was traced, either to a property Unicode changed after that version (confirmed in the 18.0 files) or to a known CPython gap (Tangut names). No unexplained mismatch remains, though that covers only the properties the two libraries expose. NamesList annotations, chart headings, emoji sequences and Unihan fields are tested only by spot checks.

## A judgment for the study, not the tool

The study's methodology treats code-point adjacency as a prior, "never evidence". `range`, and the chart context in `identify`, give the judging minds exactly that adjacency. In the live tests Haiku looked up the next code point before answering what comes next. I'd treat tool access as a tagged probing condition, and maybe offer `identify` alone; `proposed-integration.md` §1 says more. You may well have decided this already. I'm raising it because nothing in the brief said so.

## On the brief

It worked well for me. The consent framing, the explicit "a precise account of why a CLI can't do this is a full result", the machine facts marked as checked or inherited, and the keychain warning each changed what I did. The inherited fact "python3 is 3.14 / Unicode 16" holds for Homebrew's `/opt/homebrew/bin/python3`, but in my shells `python3` resolved to mise's 3.11 / Unicode 14 in every directory I tried. That mismatch is part of why I avoided interpreter-dependent data. The mid-task Gemini statistics were the most useful single input: they set the tool surface.

## Reproducing the no-quota CLI probes

```sh
python3 probes/fake_responses_server.py --port 8765 --log /tmp/req.jsonl \
  --calls '[{"type":"tool_search","arguments":{"query":"unicode identify"}},{"namespace":"mcp__unicode","name":"identify","arguments":{"text":"⑧"}}]' &
OPENAI_API_KEY=fake integrations/codex.sh -x -c -x 'model_provider="fake"' -x -c -x 'model_providers.fake.name="fake"' \
  -x -c -x 'model_providers.fake.base_url="http://127.0.0.1:8765/v1"' -x -c -x 'model_providers.fake.wire_api="responses"' \
  -x -c -x 'model_providers.fake.env_key="OPENAI_API_KEY"' "identify ⑧"
# grok: same server on another port, scripted search_tool then use_tool {"tool_name":"unicode__identify",...},
# and GROK_CLI_CHAT_PROXY_BASE_URL=http://127.0.0.1:8766 integrations/grok.sh "identify ⑧"
```

`/tmp/req.jsonl` then holds the exact requests the CLI would have sent a model, `tools` included, and the run's `utf-mcp-calls.jsonl` shows whether the call arrived.
