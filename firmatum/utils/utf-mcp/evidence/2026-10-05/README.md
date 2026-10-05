# Evidence, 2026-10-05

The files behind the "tested" claims in `../../README.md`, cut from the session scratchpad by `extract.py`. The scratchpad was temporary; the de novo verification (`../../de-novo-feedback-1.md` §9) found these claims untraceable without it. Call logs are verbatim. CLI streams and captured model requests are reduced to the tools offered, the calls made, their results (truncated) and the final text; full system prompts are left out.

| claim | files |
|---|---|
| claude: only the four tools in the session; the model used them; log ↔ transcript join | `claude-live-1.summary.json`, `claude-live-2-script.summary.json` (`init.tools`, `tool_use` ids), `call-logs/claude-live-*.jsonl` (`meta["claudecode/toolUseId"]`) |
| claude: `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` stops the per-cwd memory directory | `claude-memprobe-default.init.json` (has `memory_paths.auto`) vs `claude-memprobe-disable-auto-memory.init.json` (none); both runs used a non-existent model, so no model call was made |
| codex: the live call reached the server, then failed on quota | `call-logs/codex-live-quota-exhausted.jsonl` (initialize and tools/list, no call) |
| codex: tool regimes, both verified against the fake endpoint | `codex-fake-script-gpt55.requests.json` (classic tools, `tool_search`, then the `mcp__unicode` namespace), `codex-fake-script-codemode.requests.json` (`exec`/`wait`/`collaboration` only), matching `call-logs/codex-fake-*.jsonl` |
| grok: `--tools ""` is ignored and the first live model used the shell | `grok-live-1-tools-flag-ignored.summary.json` (`tools_offered` includes `run_terminal_command`; the calls) |
| grok: live run with `--tools search_tool,use_tool` found and used the tools | `grok-live-2.summary.json`. This run's server log was overwritten by a later probe; the stream's tool calls and results are the record |
| grok: `integrations/grok.sh` against the fake endpoint | `grok-fake-script.requests.json`, `grok-fake-script.summary.json`, `call-logs/grok-fake-script.jsonl` |
| grok: `use_tool` reads a local file via `tool_input_file`; `--deny Read` and `--sandbox strict` do not stop it; after the repairs the server refuses the extra key and the script flags the run | `grok-fake-tool-input-file-{baseline,deny-Read,sandbox-strict,after-repairs}.summary.json` |
| agy: print mode auto-denies the MCP call, with and without the hook | `agy-live-{1,2,3}.result.json` (`denied_actions`, stderr); `call-logs/agy-live-3-hook-overrides.jsonl` (hook allowed; no call reached the server). Runs 1 and 2 shared that log path and their server logs were overwritten |
| agy hook policy, offline | `call-logs/agy-hook-offline.jsonl` (the first, pre-repair policy; the current one is covered by `tests/protocol.rs`) |
| Gemini's shell attempts in the study: 172 conversations, 149 names, 31 arithmetic (12 both) | `gemini-shell-attempts.py` (counting rule in its docstring; reads the study's harvested agy transcripts) |

Not here: the Python and Perl cross-check outputs (reproducible with `checks/`), and the first grok live run's server log, which never existed because the server was never started (project config in an untrusted folder).
