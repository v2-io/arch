# utf-mcp

A small MCP server that answers factual questions about Unicode characters, for language models that should be able to look characters up without having a shell. It reads nothing at run time: the Unicode Character Database (UCD) 18.0.0 files are compiled into the binary. It has no file-system or network tools, and its only side effect is an append-only log of every call, written to a path chosen by whoever launches it. Tool arguments cannot choose that path.

It was built for the glyph-sequence perception study (`asf/empirica/glyph-sequence-perception`). There, models answering questions about glyphs through `claude -p`, `agy`, `grok` and `codex exec` had been reaching for Python to look characters up. In 172 denied attempts by Gemini, 151 identified the glyphs it was shown (name and code point), 19 generated code-chart neighbours by code-point arithmetic, and 2 asked only for `ord()`. Those three shapes became the first two tools.

## Quick start

```sh
./fetch-ucd                  # download the pinned UCD 18.0.0 files, verify SHA-256 (data/SHA256SUMS-18.0.0)
cargo build --release        # ~20 MB self-contained binary: target/release/utf-mcp
target/release/utf-mcp query identify '{"text":"㏳㏴㏵","detail":"brief"}'
```

`mise run build` / `mise run test` / `mise run crosscheck` do the same through mise (the directory's `mise.toml` must be trusted first). The data files are not committed; `fetch-ucd` re-creates them byte-for-byte from unicode.org against the committed checksums, and extracts a field subset of Unihan into `unihan-subset.txt`.

## Tools

All four are read-only, declared `readOnlyHint: true`, and report `"unicode_version": "18.0.0"` in every result. Results are compact JSON, one array element per line. Bad input comes back as a tool error the model can read (`isError: true`) rather than as a protocol failure.

| tool | what it answers | main arguments |
|---|---|---|
| `identify` | what given glyphs, code points or names are | `text` (any string; each distinct code point, in order of first appearance), `codepoints` (`"U+2460"`, `"0x2460"`, `"\\u2460"`, `"&#x2460;"`, bare hex of 2+ digits, or one glyph), `names` (exact names or aliases, UAX #44 loose matching, including algorithmic names like `CJK UNIFIED IDEOGRAPH-4E01`, named sequences and emoji names like `flag: Japan`), `detail` = `brief` / `standard` (default) / `full` |
| `range` | a character's neighbours in the code charts | `around` + `before`/`after`, or `start` + `end`; at most 512 code points; runs of unassigned / private-use / surrogate / noncharacter code points collapse into one row |
| `search` | characters whose name, formal alias or informal alias contains the query words | `query`, `mode` (`prefix` default, `word`, `substring`), filters `block`, `script`, `category` (`So`, `Nd`, or `S` for all symbols), `han_definitions` (also match Unihan English definitions), `limit` (default 50, max 500). Also reports named/emoji sequences and blocks whose names match (`tai xuan jing` finds the block whose characters are named `TETRAGRAM FOR …`) |
| `blocks` | Unicode blocks, and how the code charts subdivide one | no `query`: all 353 blocks with ranges; `query` = block name (or part), glyph or code point: the block, its assigned count, and its chart headings, each with range, glyphs and Unicode's notes |

What `identify` returns per character, by detail level:

- **brief:** code point, glyph, name (or a code point label such as `<control-0000>` / `<reserved-0378>`), general category, block, numeric value if any.
- **standard:** adds the code-chart heading the character sits under (from NamesList), script and script extensions, age (version first assigned), numeric type and value, decomposition (with algorithmic Hangul), simple case mappings, formal and informal aliases, Unicode's own notes and cross-references to related characters (`see_also`), emoji properties, and for Han ideographs the Unihan definition, Mandarin reading, stroke count, radical (e.g. `⽔ KANGXI RADICAL WATER (#85) + 6 strokes`), simplified/traditional variants and Unihan numeric values.
- **full:** adds bidi class and mirroring, combining class, East Asian width, decimal value, UTF-8 and UTF-16 code units, standardized variation sequences, and more Unihan readings (Cantonese, Japanese, Korean, Vietnamese).

Per call, `standard` returns at most 128 characters and `full` at most 32; the rest are reported as truncated. Text is limited to 4000 code points and requests to 1 MiB.

Sources, all pinned to 18.0.0: UnicodeData, DerivedName, NameAliases, NamesList, Blocks, Scripts, ScriptExtensions, DerivedAge, EastAsianWidth, DerivedNumericType/Values, PropertyValueAliases, NamedSequences, emoji-data, emoji-sequences, emoji-zwj-sequences, and Unihan (fields listed in `fetch-ucd`). Nothing comes from a font or renderer: the server knows what a code point *is*, not what it looks like. Glyph-shape measurements live next door in `../utf/`.

## The call log

Every request is one JSON line, appended: `start` (pid, parent pid and parent command line, cwd, load time), `initialize` (client name and version, protocol), each `call` (tool, exact arguments, the client's `_meta`, the exact text returned to the model, error flag, microseconds), rejected calls, unimplemented methods (with their params, so harness probes are visible), and `exit`. Every record carries a per-process `session` id, a sequence number, and the optional `--tag`.

```sh
utf-mcp --log PATH --tag RUN-ID     # append to PATH (directories created)
utf-mcp --log -                     # records on stderr
utf-mcp --no-log
# default: $UTF_MCP_LOG, else $XDG_STATE_HOME/utf-mcp/calls.jsonl (~/.local/state/utf-mcp/calls.jsonl)
```

Joining a call to the harness transcript: Claude Code puts its tool-use id in `meta["claudecode/toolUseId"]`, and codex puts its call id in `meta["callId"]`. grok's `_meta` has only a progress token, so join by the tag and time. For agy, the `agy-hook` records (below) carry agy's conversation id and step.

## Wiring it into the four CLIs

`integrations/<cli>.sh [-m MODEL] [-l LOG] [-t TAG] [-w WORKDIR] [-x ARG]... PROMPT` runs one prompt with the Unicode tools as the only tools the CLI offers, to the extent each CLI allows. Each script builds its configuration in a per-run directory and changes nothing under `~/.claude`, `~/.codex`, `~/.grok` or `~/.gemini`. Tested state as of 2026-10-05:

| CLI | how | other tools left | tested |
|---|---|---|---|
| `claude -p` 2.1.289 | `--mcp-config` JSON + `--strict-mcp-config`, `--tools ""`, `--allowedTools mcp__unicode__*` (four names), `--setting-sources ""` | none | live, 2 calls (Haiku), one through `integrations/claude.sh`: init listed only the four tools, the model used them, and the log joined to the transcript |
| `codex exec` 0.157.1 | isolated `CODEX_HOME`, server via `-c mcp_servers.unicode.*`, built-ins disabled with `--disable …` | gpt-5.5: `tool_search`, MCP-resource readers, `request_user_input`, `apply_patch` (read-only sandbox). gpt-5.6+/6: a JavaScript `exec` tool (V8, no fs/network), `wait`, `request_user_input`, and a sub-agent `collaboration` namespace that `--disable multi_agent` did not remove | against a local fake model endpoint only, both tool modes; the account's codex quota is exhausted until 2026-11-02 |
| `grok -p` 1.0.44 | isolated `HOME` whose `config.toml` declares only this server; `--tools search_tool,use_tool --allow 'mcp__unicode__*'` | `search_tool`/`use_tool` (the only way grok exposes MCP tools) | live, 2 calls. The first used `--tools ""`, which grok ignores, so the model had a shell and used it (Python `unicodedata`, in a scratch directory). The second (grok-4.7-build) used `--tools search_tool,use_tool` with the server in a project config and `GROK_FOLDER_TRUST=0`; the model found and used the tools. `integrations/grok.sh` (isolated home) tested against the fake endpoint only |
| `agy -p` 1.2.17 | workspace plugin `.agents/plugins/unicode/` with `mcp_config.json` and a `PreToolUse` hook (`utf-mcp agy-hook`) | the hook denies every other tool | live, 3 calls (Gemini 3.6 Flash Low): the server starts and the model calls it, but **print mode auto-denies the call** without an allow-rule in `~/.gemini/antigravity-cli/settings.json`. Not unblocked; see `proposed-integration.md` |

Things that differ by CLI and matter for anyone reading results:

- **Discovery.** Claude gets the four tools directly. codex and grok defer MCP tools behind a search step (`tool_search`, `search_tool`), and codex's code-mode models are not told the server exists unless the prompt says so. In the captured requests, codex (classic mode) and grok show the model the server's `initialize` instructions; for Claude and agy I did not inspect the model-side request.
- **What else is available.** codex's code-mode `exec` is a general JavaScript interpreter. It can do code-point arithmetic (`String.fromCodePoint(cp + 1)`) and Unicode property regexes (`/\p{Script=Han}/u`) with no server call, and those uses appear only in codex's rollout file, not in `--json` output or in this log. grok's `use_tool` accepts `tool_input_file`, so it can read a local JSON file to use as arguments (untested beyond reading the schema).
- **Pitfalls found.** grok ignores `--tools ""` and `--tools none` and keeps its full default set, shell included. `--disallowed-tools run_terminal_command` did not remove grok's shell. grok also imports MCP servers, skills and hooks from `~/.claude*` unless `HOME` is isolated or `GROK_CLAUDE_*_ENABLED=0` is set. grok does not start MCP servers from a project config (`./.grok/config.toml`) in an untrusted folder unless `GROK_FOLDER_TRUST=0`; agy did start the workspace plugin's server in an untrusted folder. Claude Code and agy send an MCP `server/discover` request before `initialize`; the server answers "method not found" and both fall back normally.

## How it was checked

- `cargo test --release`: 9 unit tests (parsing, names and aliases, algorithmic names, properties, each tool) and 4 stdio tests (handshake, calls, error shapes, version fallback, batches, oversized-line refusal, the call log, the agy hook policy).
- `checks/crosscheck_unicodedata.py`: every code point's name, category, combining class, bidi class, decomposition, mirroring, East Asian width and numeric value, compared with Python 3.14's `unicodedata` (Unicode 16.0), an independent implementation, over the 294,645 code points assigned by 16.0. All differences are explained: properties Unicode changed after 16.0 (U+0295 → Lo; U+1B3A/1B3C/1B42 bidi → L; U+5146 兆 now with two Unihan numeric values; eight cuneiform signs given numeric values), and 6,145 Tangut names that CPython does not generate.
- `checks/crosscheck_perl.pl`: block, script and age compared with Perl's `Unicode::UCD` (13.0) over 283,506 code points. Block and age agree everywhere; script differs on two (U+16FE2/16FE3, which became Han in 14.0). Slow: about 20 minutes.
- `probes/fake_responses_server.py`: a stand-in for the OpenAI Responses API that records each request and replays scripted tool calls. Pointed at by codex (`-c model_providers…`) or grok (`GROK_CLI_CHAT_PROXY_BASE_URL`), it shows the exact tool list a CLI would send a model and drives the full call path with no model quota. The codex and grok rows above were established with it; the commands are in `debrief.md`.

## Layout

```
src/ucd.rs        UCD parsing and lookups (all strings borrow from the embedded files)
src/tools.rs      the four tools: schemas, argument parsing, result shapes, unit tests
src/main.rs       MCP stdio loop (JSON-RPC 2.0), CLI modes: serve, query, tools, dump, agy-hook
src/log.rs        the call log
src/hook.rs       agy PreToolUse hook policy
tests/protocol.rs stdio protocol tests
checks/           cross-checks against Python and Perl UCD implementations
probes/           fake model endpoint for testing CLI wiring without quota
integrations/     per-CLI launch scripts
fetch-ucd         data download + checksum verification + Unihan subset
data/             SHA256SUMS-18.0.0 (committed); data/18.0.0/ (fetched, ignored)
```

Moving to a later Unicode version means running `UTF_MCP_ACCEPT_NEW=1 ./fetch-ucd <version>`, changing the version in the `ucd_file!` macro and `UCD_VERSION` in `src/ucd.rs`, rebuilding, and re-running the cross-checks.
