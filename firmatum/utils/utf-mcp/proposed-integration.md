# Proposed integration: utf-mcp in the glyph-sequence study

*2026-10-05, from the agent that built utf-mcp. These are suggestions with reasons; nothing outside `firmatum/utils/utf-mcp/` was changed. The study's code is `asf/empirica/glyph-sequence-perception/refactored/`, mainly `core/judges.py`.*

## 1. The tool hands the minds the scaffold the study treats as non-evidence

`refactored/METHODOLOGY.md` §1: before any mind answers, the only edges are code-point neighbours, and *"that scaffold reflects Unicode's allocation, not anyone's perception, so it is a prior and never evidence."* A judge that can call `range`, or read `identify`'s `chart_heading` and `see_also`, is reading that allocation. Its continuation may then report the chart rather than its perception. The live tests show this happening: asked what follows ㏳ ㏴ ㏵, Haiku called `identify` and then `range(after=1)`; asked about 🃮 🃭 🃬, it identified them and then looked up U+1F0EB.

So I would treat tool access as a probing condition (concern 1 in §2a) rather than an adapter detail:

- Tag every answer with which tools were offered (`none` / `identify-only` / `all`) and which were called. The call log has the latter exactly, keyed by `--tag`.
- If the aim is only to stop judges stalling on a denied shell, `identify` alone covers 151 of the 172 observed Gemini attempts. Each CLI can offer a subset: Claude by listing fewer names in `--allowedTools`, grok by a narrower `--allow` (rules are per tool, verified), codex probably by `mcp_servers.unicode.enabled_tools` (the key exists in codex 0.157.1; untested).
- `identify`'s `standard` detail includes `chart_heading` and `see_also`, which are chart structure. `detail: "brief"` (name, category, block) is closer to "what is this glyph" if that is the intent; the tool could also gain a server flag that caps detail per run. That isn't built; say if you want it.

Whether chart knowledge should count as perception is the study's question, not mine. The mechanism makes it a choice, and the log makes it auditable.

## 2. Per-CLI adapter changes

Each `integrations/<cli>.sh` is a working recipe to copy from; `README.md` has the tested state.

**Claude (`claude_call`).** The adapter already passes `--tools "" --strict-mcp-config --setting-sources ""`. Changes: replace `--mcp-config '{"mcpServers":{}}'` with the server's config, and add `--allowedTools` naming the offered tools (print mode cannot prompt, so unlisted MCP tools are refused). The current `--output-format json` returns only the final result. Tool calls are in the utf-mcp log, joinable through `meta["claudecode/toolUseId"]`, or in `--output-format stream-json --verbose`.

**codex (`codex_call`).** The adapter's isolated `CODEX_HOME` pattern works unchanged; add the `-c mcp_servers.unicode.*` overrides and the `--disable …` list from `integrations/codex.sh`. Two decisions belong to the study:
- *Model family sets the tool regime.* gpt-5.5 gets function tools, with the unicode tools found through `tool_search`. gpt-5.6-* and gpt-6-* get only a JavaScript `exec` sandbox (`tool_mode: code_mode_only` in codex's model catalog) and are not told the server exists. Telling them in the prompt (e.g. "Unicode lookup is available as `tools.mcp__unicode__identify`") is probably needed for them to use it, and the sandbox can do code-point arithmetic and `\p{…}` regexes with no lookup.
- *Keep rollouts* (drop `--ephemeral`): the JavaScript a code-mode model ran appears only in the rollout file (`response_item` / `custom_tool_call` `input`), not in `codex exec --json` and not in the utf-mcp log.

None of the codex path has met a live model (quota exhausted until 2026-11-02). It is verified against `probes/fake_responses_server.py`.

**grok (`grok_call`).** The adapter's isolated `HOME` is the right base. Write `[mcp_servers.unicode]` into that home's `config.toml`, and replace the current all-tools-removed flags (`--tools todo_write --disallowed-tools …search_tool,use_tool…`) with `--tools search_tool,use_tool --allow 'mcp__unicode__*'`. Without `--allow`, headless grok in the isolated home cancels the call (observed); my reading is that the isolated home has the default permission mode. Joseph's own `~/.grok/config.toml` sets `permission_mode = "always-approve"`, which is why a non-isolated run needs no `--allow`, and why a non-isolated run with the wrong `--tools` gave a model a working shell.

**agy (`agy_call`).** Workspace plugin plus hook, as in `integrations/agy.sh`, with `cwd` = the plugin's workspace. The blocker is in §3.

## 3. agy needs one decision from Joseph

agy 1.2.17 in print mode auto-denies MCP calls ("a tool required the "mcp" permission that headless mode cannot prompt for") unless `permissions.allow` in `~/.gemini/antigravity-cli/settings.json` has a matching rule. I tested whether the workspace hook could grant it instead: `{"decision": "allow"}` was ignored, and so was `{"decision": "allow", "allowTool": true, "permissionOverrides": ["mcp(unicode_unicode/*)", …]}`. I found no flag or environment variable that points agy at another settings file, and changing `HOME` breaks the keychain. The options I see:

1. **Add one allow-rule** to `~/.gemini/antigravity-cli/settings.json`:
   ```json
   "permissions": { "allow": [ "...existing rules...", "mcp(unicode_unicode/*)" ] }
   ```
   `unicode_unicode` is agy's name for a plugin's server (`<plugin>_<server>`, seen in agy's transcript). The `mcp(<server>/*)` form is inferred from strings in the agy binary (`mcp(chrome_devtools/*)`) and from the error's own hint, `mcp(<target>)`; it has not been tested. It matches only a server named `unicode_unicode`, so it grants nothing in sessions without the plugin. This is Joseph's daily config, so I did not make the change.
2. **`--dangerously-skip-permissions` plus the deny-all hook.** The hook runs before the permission check (observed), so it may hard-deny everything but the unicode server even when permissions are skipped. That is untested. It also fails open: if the plugin did not load, the model would have every tool, shell included. I would not use it for unattended runs without a check that the hook is active (for example, require a `hook` record in the log before trusting a result).

A related observation, for Joseph rather than for this tool: the same `permissions.allow` list already holds `command(grep)`, `command(head)`, `command(ls)`, `command(diff)` and `command(wc)` from earlier interactive work. Those apply to unattended print-mode runs too, so an agy judge could run those commands without a prompt. I did not test whether any judge has.

## 4. Smaller things

- **Tags.** Pass the ledger's presentation or sheet id as `--tag`. Each server process is one CLI call, so the log's `session` already groups a call; the tag joins it to the ledger.
- **Log placement.** One log per round, e.g. `data/rounds/<round>/utf-mcp.jsonl`. Appends from parallel judges are single `write` calls of one line each.
- **Binary location.** The scripts default to `target/release/utf-mcp` in this directory; `UTF_MCP=/path` overrides that. `cargo install --path .` would put it on `PATH` beside `aspectus` and `md-press`. I did not do that, since it changes `~/.cargo/bin`.
- **Unicode version.** The study's other Unicode data (`utf/axes`, `bmp-tables.md`) is Unicode 14.0 (from mise's Python 3.11); this server is 18.0. Names are stable across versions, but properties drift (the cross-check lists real changes after 16.0), and characters added after 14.0 exist only here.
