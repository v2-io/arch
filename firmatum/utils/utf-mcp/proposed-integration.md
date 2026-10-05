# Proposed integration: utf-mcp in the glyph-sequence study

*2026-10-05, from the agent that built utf-mcp. These are suggestions with reasons; nothing outside `firmatum/utils/utf-mcp/` was changed. The study's code is `asf/empirica/glyph-sequence-perception/refactored/`, mainly `core/judges.py`.*

## 1. Labelling answers in the reasoned pass

*Rewritten after the first verification. The earlier version of this section argued that the tools leak code-chart order into answers the study treats as perception, and suggested offering `identify` alone at `brief` detail. Two things were wrong with that.* First, the study had already decided the question: `refactored/METHODOLOGY.md` (line 111, committed before this tool was finished; I had read only its first sections) puts tool use in a separate **reasoned pass**, where looking things up and reasoning are expected, tagged as their own kind of answer and never pooled with first impressions. Joseph: *"where the tool works and you can have them give their reasoned one as well (probably especially useful for extending a sequence etc. — where reasoning is fine — verification/recognition != generation (more complex))"*. First-impression passes stay tool-free. Second, the suggested mitigation does not work: any tool that accepts arbitrary code points is `range` with a stride of one. In the live test Haiku identified 🃮 🃭 🃬 and then looked up U+1F0EB by code point, without `range`. Names also state the series outright (`PLAYING CARD TRUMP-14 / -13 / -12`).

So code-chart order surfacing through the tools is expected in the reasoned pass. What the tool can add is a precise label for *how* each reasoned answer was informed. Every call is in the log with its exact arguments, so each answer can be tagged by the information channels it used:

| channel | from the log |
|---|---|
| chart neighbours | any `range` call; `identify` on a code point or name that was not among the glyphs shown |
| identification | `identify` whose `text` (or code points) are only the glyphs shown |
| name search | `search` |
| chart structure | `blocks`, or `identify` at `standard`/`full` (chart headings, `see_also`) |
| none | no calls |

Outside the log: codex code-mode models can do code-point arithmetic in JavaScript (only in codex's rollout files), and grok runs carrying a `harness_warning` had a model point `use_tool` at a local file. I have not built the classifier; it is a short script over the log and the ledger's prompts, and I can write it if wanted.

## 2. Per-CLI adapter changes

Each `integrations/<cli>.sh` is a working recipe to copy from; `README.md` has the tested state.

**Claude (`claude_call`).** The adapter already passes `--tools "" --strict-mcp-config --setting-sources ""`. Changes: replace `--mcp-config '{"mcpServers":{}}'` with the server's config, add `--allowedTools` naming the offered tools (print mode cannot prompt, so unlisted MCP tools are refused), and set `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` in the environment. With `integrations/claude.sh`, every run without it created an empty `~/.claude/projects/<cwd-slug>/memory/` directory for its fresh working directory. I found no such directory for the adapter's own `claude-cwd`, so I don't know whether the adapter's current flags or environment already avoid it; setting the variable is cheap either way. The current `--output-format json` returns only the final result. Tool calls are in the utf-mcp log, joinable through `meta["claudecode/toolUseId"]`, or in `--output-format stream-json --verbose`.

**codex (`codex_call`).** The adapter's isolated `CODEX_HOME` pattern works unchanged; add the `-c mcp_servers.unicode.*` overrides and the `--disable …` list from `integrations/codex.sh`. Two decisions belong to the study:
- *Model family sets the tool regime.* gpt-5.5 gets function tools, with the unicode tools found through `tool_search`. gpt-5.6-* and gpt-6-* get only a JavaScript `exec` sandbox (`tool_mode: code_mode_only` in codex's model catalog) and are not told the server exists. Telling them in the prompt (e.g. "Unicode lookup is available as `tools.mcp__unicode__identify`") is probably needed for them to use it, and the sandbox can do code-point arithmetic and `\p{…}` regexes with no lookup.
- *Keep rollouts* (drop `--ephemeral`): the JavaScript a code-mode model ran appears only in the rollout file (`response_item` / `custom_tool_call` `input`), not in `codex exec --json` and not in the utf-mcp log.

None of the codex path has met a live model (quota exhausted until 2026-11-02). It is verified against `probes/fake_responses_server.py`.

**grok (`grok_call`).** The adapter's isolated `HOME` is the right base. Write `[mcp_servers.unicode]` into that home's `config.toml`, and replace the current all-tools-removed flags (`--tools todo_write --disallowed-tools …search_tool,use_tool…`) with `--tools search_tool,use_tool --allow 'mcp__unicode__*'`. Without `--allow`, headless grok in the isolated home cancels the call (observed); my reading is that the isolated home has the default permission mode. Joseph's own `~/.grok/config.toml` sets `permission_mode = "always-approve"`, which is why a non-isolated run needs no `--allow`, and why a non-isolated run with the wrong `--tools` gave a model a working shell.

grok's `use_tool` can also be pointed at a local JSON file (`tool_input_file`), which grok reads and forwards as arguments. Neither `--deny Read` nor any `--sandbox` profile I tried stopped it. In practice a judge would have to guess paths, the server refuses unexpected keys, and the forwarded arguments always land in the log. `integrations/grok.sh` flags such runs (exit 3 plus a `harness_warning` record); an adapter could treat that the same way it treats Gemini's tool attempts.

**agy (`agy_call`).** Workspace plugin plus hook, as in `integrations/agy.sh`, with `cwd` = the plugin's workspace. The blocker is in §3.

## 3. agy needs one decision from Joseph

agy 1.2.17 in print mode auto-denies MCP calls ("a tool required the "mcp" permission that headless mode cannot prompt for") unless `permissions.allow` in `~/.gemini/antigravity-cli/settings.json` has a matching rule. I tested whether the workspace hook could grant it instead: `{"decision": "allow"}` was ignored, and so was `{"decision": "allow", "allowTool": true, "permissionOverrides": ["mcp(unicode_unicode/*)", …]}`. I found no flag or environment variable that points agy at another settings file, and changing `HOME` breaks the keychain. The options I see:

1. **Add one allow-rule** to `~/.gemini/antigravity-cli/settings.json`:
   ```json
   "permissions": { "allow": [ "...existing rules...", "mcp(unicode_unicode/*)" ] }
   ```
   `unicode_unicode` is agy's name for a plugin's server (`<plugin>_<server>`, seen in agy's transcript). The `mcp(<server>/*)` form is inferred from strings in the agy binary (`mcp(chrome_devtools/*)`) and from the error's own hint, `mcp(<target>)`; it has not been tested. It matches only a server named `unicode_unicode`, so it grants nothing in sessions without the plugin. This is Joseph's daily config, so I did not make the change.
2. **`--dangerously-skip-permissions` plus the deny-all hook.** The hook runs before the permission check (observed), so it may hard-deny everything but the unicode server even when permissions are skipped. That is untested, and so is what agy does when the hook times out or fails to start. It also fails open: if the plugin did not load, the model would have every tool, shell included. I would not use it for unattended runs without a check that the hook is active (for example, require a `hook` record in the log before trusting a result). After the first verification the hook allows only agy's `call_mcp_tool` step for `unicode_unicode`; it no longer trusts a server name written into another tool's arguments.

A related observation, for Joseph rather than for this tool: the same `permissions.allow` list already holds `command(grep)`, `command(head)`, `command(ls)`, `command(diff)` and `command(wc)` from earlier interactive work. Those apply to unattended print-mode runs too, so an agy judge could run those commands without a prompt. I did not test whether any judge has.

## 4. Smaller things

- **Tags.** Pass the ledger's presentation or sheet id as `--tag`. Each server process is one CLI call, so the log's `session` already groups a call; the tag joins it to the ledger.
- **Log placement.** One log per round, e.g. `data/rounds/<round>/utf-mcp.jsonl`. Each record is written with one `write_all` to a file opened for append. On a local disk that keeps parallel judges' lines whole in practice, but a large record (a 512-row `range` result is about 60 KB) could in principle be split by a partial write. If a strict guarantee matters, give each judge its own log.
- **Binary location.** The scripts default to `target/release/utf-mcp` in this directory; `UTF_MCP=/path` overrides that. `cargo install --path .` would put it on `PATH` beside `aspectus` and `md-press`. I did not do that, since it changes `~/.cargo/bin`.
- **Unicode version.** The study's other Unicode data (`utf/axes`, `bmp-tables.md`) is Unicode 14.0 (from mise's Python 3.11); this server is 18.0. Names are stable across versions, but properties drift (the cross-check lists real changes after 16.0), and characters added after 14.0 exist only here.
