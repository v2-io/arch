# utf-mcp

A small MCP server that answers factual questions about Unicode characters, for language models that should be able to look characters up without having a shell. It reads nothing at run time: the Unicode Character Database (UCD) 18.0.0 files are compiled into the binary. It has no file-system or network tools, and its only side effect is an append-only log of every call, written to a path chosen by whoever launches it. Tool arguments cannot choose that path.

It was built for the glyph-sequence perception study (`asf/empirica/glyph-sequence-perception`), for its *reasoned* pass: a separately tagged pass in which minds may look things up and reason, kept apart from first-impression answers. Before this tool, models answering questions about glyphs had been reaching for Python. Gemini made shell attempts in 172 conversations. Counted with overlapping categories (rule and script in `evidence/2026-10-05/gemini-shell-attempts.py`), 149 asked for the names of the glyphs it was shown, 31 generated code-chart neighbours by code-point arithmetic (12 of them also asked for names), and 3 asked only for `ord()`/`hex()`. Those shapes became `identify` and `range`.

## Quick start

```sh
./fetch-ucd                  # download the pinned UCD 18.0.0 files, verify SHA-256 (data/SHA256SUMS-18.0.0)
cargo build --release        # ~20 MB self-contained binary: target/release/utf-mcp
target/release/utf-mcp query identify '{"text":"㏳㏴㏵","detail":"brief"}'
```

`mise run build` / `mise run test` / `mise run crosscheck` do the same through mise (the directory's `mise.toml` must be trusted first). The data files are not committed; `fetch-ucd` re-creates them byte-for-byte from unicode.org against the committed checksums, and extracts a field subset of Unihan into `unihan-subset.txt`.

## Tools

All four are read-only, declared `readOnlyHint: true`, accept only the argument keys in their schemas (`additionalProperties: false`; unknown keys are refused by name, without echoing their values), and report `"unicode_version": "18.0.0"` in every result. Results are compact JSON, one array element per line. Bad input comes back as a tool error the model can read (`isError: true`) rather than as a protocol failure.

| tool | what it answers | main arguments |
|---|---|---|
| `identify` | what given glyphs, code points or names are | `text` (any string; each distinct code point, in order of first appearance), `codepoints` (strings: `"U+2460"`, `"0x2460"`, `"\\u2460"`, `"&#x2460;"`, `"&#9312;"`, bare hex of 2+ digits, or one glyph; several per string separated by spaces or commas; JSON numbers are refused as ambiguous between hex and decimal), `names` (exact names or aliases, UAX #44 loose matching, including algorithmic names like `CJK UNIFIED IDEOGRAPH-4E01`, named sequences, and CLDR emoji names, single (`grinning face with big eyes`) or sequence (`flag: Japan`)). Where a character name and a CLDR emoji name are the same words, the character name wins: `sun` is U+2609 SUN, not ☀ U+2600, whose CLDR name is "sun"), `detail` = `brief` / `standard` (default) / `full` |
| `range` | a character's neighbours in the code charts | `around` + `before`/`after`, or `start` + `end`; at most 512 code points; runs of unassigned / private-use / surrogate / noncharacter code points collapse into one row |
| `search` | characters whose name, formal alias, informal alias or CLDR emoji name contains the query words | `query` (split into words the way names are, so `twenty-one` matches `TWENTY-ONE`), `mode` (`prefix` default, `word`, `substring`), filters `block`, `script` (name or code: `Latin`, `Latn`), `category` (code or name, any case: `So`, `nd`, `S` for all symbols, `Other_Symbol`; an unknown filter value is an error, not an empty result), `han_definitions` (also match Unihan English definitions), `limit` (default 50, max 500). Also reports named/emoji sequences and blocks whose names match (`tai xuan jing` finds the block whose characters are named `TETRAGRAM FOR …`). Ranking: exact name or alias matches first, then names containing the query as a verbatim phrase (hyphen and space treated alike), then names where the words are scattered, each tier in code point order. A result with no matches carries a `note` saying so and suggesting other routes, so an empty answer is not mistaken for "no such character" |
| `blocks` | Unicode blocks, and how the code charts subdivide one | no `query`: all 353 blocks with ranges; `query` = block name (or part; tried first), glyph, or code point (`U+…` forces a code point): the block, its assigned count, Unicode's block-level notes, and its chart headings, each with range, glyphs, notes and cross-references |

What `identify` returns per character, by detail level:

- **brief:** code point, glyph, name (or a code point label such as `<control-0000>` / `<reserved-0378>`), general category, block, numeric value if any.
- **standard:** adds the code-chart heading the character sits under (from NamesList), script and script extensions, age (version first assigned), numeric type and value, decomposition (with algorithmic Hangul), simple case mappings, formal and informal aliases, Unicode's own notes and cross-references to related characters (`see_also`), a NamesList notice printed after the character (`chart_notice_following`, labelled as positional because such a notice can concern the character or the whole group it ends), emoji properties and CLDR emoji name, and for Han ideographs the Unihan definition, Mandarin reading, stroke count, radical with the glyph of the radical form actually used (e.g. `⽔ KANGXI RADICAL WATER (#85) + 6 strokes`, `⺰ CJK RADICAL C-SIMPLIFIED SILK (#120, Chinese simplified form) + 0 strokes`), simplified/traditional variants and Unihan numeric values.
- **full:** adds bidi class and mirroring, combining class, East Asian width, decimal value, UTF-8 and UTF-16 code units, standardized variation sequences, and more Unihan readings (Cantonese, Japanese, Korean, Vietnamese).

Per call, `standard` returns at most 128 characters and `full` at most 32; the rest are reported as truncated. Text is limited to 4000 code points and requests to 1 MiB.

Sources, all pinned to 18.0.0: UnicodeData, DerivedName, NameAliases, NamesList, Blocks, Scripts, ScriptExtensions, DerivedAge, EastAsianWidth, DerivedNumericType/Values, PropertyValueAliases, NamedSequences, CJKRadicals, emoji-data, emoji-sequences, emoji-zwj-sequences, emoji-test, and Unihan (fields listed in `fetch-ucd`). The committed source checksums are trust-on-first-fetch (unicode.org publishes no hashes): they record what was downloaded on 2026-10-05. The derived `unihan-subset.txt`, which is what gets compiled in, has its own checksum in `data/SHA256SUMS-18.0.0.derived`. Nothing comes from a font or renderer: the server knows what a code point *is*, not what it looks like. Glyph-shape measurements live next door in `../utf/`.

## The call log

Every request is one JSON line, appended: `start` (pid, parent pid and parent command line, cwd, load time), `initialize` (client name and version, protocol), each `call` (tool, exact arguments, the client's `_meta`, the exact text returned to the model, error flag, microseconds), rejected calls, unimplemented methods (with their params, so harness probes are visible), `hook` decisions from `agy-hook`, and `exit`. Every record the server writes carries a per-process `session` id, a sequence number, and the optional `--tag`. `exit` is written only when the client closes the server's stdin: agy does, but Claude Code, codex and grok end the server with a signal, so their sessions simply stop after the last record. `integrations/grok-check.py` may append a `harness_warning` record (see below); it is written by the checker, not the server, so it has `ts` and `tag` but no `session` or sequence number.

```sh
utf-mcp --log PATH --tag RUN-ID     # append to PATH (directories created)
utf-mcp --log -                     # records on stderr
utf-mcp --no-log
# default: $UTF_MCP_LOG, else $XDG_STATE_HOME/utf-mcp/calls.jsonl (~/.local/state/utf-mcp/calls.jsonl)
```

Joining a call to the harness transcript: Claude Code puts its tool-use id in `meta["claudecode/toolUseId"]`, and codex puts its call id in `meta["callId"]`. grok's `_meta` has only a progress token, so join by the tag and time. For agy, the `agy-hook` records (below) carry agy's conversation id and step.

## Wiring it into the four CLIs

`integrations/<cli>.sh [-m MODEL] [-l LOG] [-t TAG] [-w WORKDIR] [-x ARG]... PROMPT` runs one prompt with the Unicode tools as the only tools the CLI offers, to the extent each CLI allows. Each script builds its configuration in a per-run directory and changes no configuration under `~/.claude`, `~/.codex`, `~/.grok` or `~/.gemini`. Two side effects in those trees remain: the CLIs' own session/conversation records (agy, grok; codex rollouts stay in the run's isolated home), and token refreshes written through the symlinked `auth.json` (the grok judge home's refresh on 2026-10-05 08:23 updated the real file and left the symlink intact; codex refresh is unobserved). Before the 2026-10-05 repair, each `claude.sh` run also left an empty `~/.claude/projects/<cwd-slug>/memory/` directory. Tags must match `[A-Za-z0-9._:+-]`, and paths may not contain quotes, spaces or other characters the scripts would have to escape; the scripts refuse such values rather than escape them for JSON, TOML and a shell line. Tested state as of 2026-10-05 (evidence in `evidence/2026-10-05/`):

| CLI | how | other tools left | tested |
|---|---|---|---|
| `claude -p` 2.1.289 | `--mcp-config` JSON + `--strict-mcp-config`, `--tools ""`, `--allowedTools mcp__unicode__*` (four names), `--setting-sources ""`, `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` (without it every run creates an empty memory directory under `~/.claude/projects/`) | none | live, 2 calls (Haiku), one through `integrations/claude.sh`: init listed only the four tools, the model used them, and the log joined to the transcript. Post-repair: the study's reasoned pass has run the repaired binary live (two Sonnet calls, two successful `range` calls; `evidence/2026-10-05/README.md`) |
| `codex exec` 0.157.1 | isolated `CODEX_HOME`, server via `-c mcp_servers.unicode.*`, built-ins disabled with `--disable …` | gpt-5.5: `tool_search`, MCP-resource readers, `request_user_input`, `apply_patch` (read-only sandbox). gpt-5.6+/6: a JavaScript `exec` tool (V8, no fs/network), `wait`, `request_user_input`, and a sub-agent `collaboration` namespace that `--disable multi_agent` did not remove | against a local fake model endpoint only, both tool modes; the account's codex quota is exhausted until 2026-11-02 |
| `grok -p` 1.0.44 | isolated `HOME` whose `config.toml` declares only this server; `--tools search_tool,use_tool --allow 'mcp__unicode__*'`; then `grok-check.py WORKDIR` | `search_tool`/`use_tool` (the only way grok exposes MCP tools). `use_tool` can also make grok read a local JSON file as arguments (below) | live, 2 calls. The first used `--tools ""`, which grok ignores, so the model had a shell and used it (Python `unicodedata`, in a scratch directory). The second (grok-4.7-build) used `--tools search_tool,use_tool` with the server in a project config and `GROK_FOLDER_TRUST=0`; the model found and used the tools. `integrations/grok.sh` (isolated home) tested against the fake endpoint only |
| `agy -p` 1.2.17 | workspace plugin `.agents/plugins/unicode/` with `mcp_config.json` and a `PreToolUse` hook (`utf-mcp agy-hook`) | the hook denies every tool except agy's `call_mcp_tool` to `unicode_unicode` and agy's turn-control steps; behaviour if the hook fails to run is untested | live, 3 calls (Gemini 3.6 Flash Low): the server starts and the model calls it, but **print mode auto-denies the call** without an allow-rule in `~/.gemini/antigravity-cli/settings.json`. Not unblocked; see `proposed-integration.md` |

Things that differ by CLI and matter for anyone reading results:

- **Discovery.** Claude gets the four tools directly. codex and grok defer MCP tools behind a search step (`tool_search`, `search_tool`), and codex's code-mode models are not told the server exists unless the prompt says so. In the captured requests, codex (classic mode) and grok show the model the server's `initialize` instructions; for Claude and agy I did not inspect the model-side request.
- **What else is available.** codex's code-mode `exec` is a general JavaScript interpreter. It can do code-point arithmetic (`String.fromCodePoint(cp + 1)`) and Unicode property regexes (`/\p{Script=Han}/u`) with no server call, and those uses appear only in codex's rollout file, not in `--json` output or in this log. grok's `use_tool` accepts `tool_input_file` (and `file`): grok then reads any local JSON file the model names and sends its contents as the arguments. The verifier showed this reading a file outside the run directory; no `--deny Read` rule and no `--sandbox` profile (`read-only`, `workspace`, `strict`) stopped it. The server cannot tell such arguments from typed ones, but it refuses keys outside its schema without echoing values, so only a file shaped like valid arguments (e.g. `{"text": …}`) is echoed. After a run, `integrations/grok-check.py WORKDIR` parses the stream `grok.sh` saved and, if any `use_tool` call named a file (either form), prints the calls, appends a `harness_warning` record to the call log, and exits 3. It exits 4 if the stream has no recognisable grok events, so a future change in grok's output format fails loudly rather than passing. The call log always holds the arguments grok sent, so file contents forwarded this way end up in the log too; treat the log as possibly holding local file contents. The check is a separate step because `grok.sh` exec's grok: an earlier version that ran grok as a child so it could scan afterwards left grok, `tee` and the server running when a harness timeout killed the script (found by the second verification).
- **Pitfalls found.** grok ignores `--tools ""` and `--tools none` and keeps its full default set, shell included. `--disallowed-tools run_terminal_command` did not remove grok's shell. grok also imports MCP servers, skills and hooks from `~/.claude*` unless `HOME` is isolated or `GROK_CLAUDE_*_ENABLED=0` is set. grok does not start MCP servers from a project config (`./.grok/config.toml`) in an untrusted folder unless `GROK_FOLDER_TRUST=0`; agy did start the workspace plugin's server in an untrusted folder. Claude Code and agy send an MCP `server/discover` request before `initialize`; the server answers "method not found" and both fall back normally.

## How it was checked

- `cargo test --release`: 10 unit tests (parsing, names and aliases, algorithmic names, properties, each tool, and one covering each repair from the first verification) and 5 stdio tests (handshake, calls, error shapes, version fallback, batches and empty batch, oversized-line refusal, the call log, the agy hook policy including spoofed `ServerName`). Builds with Rust 1.85.0 (`rust-version`), checked with that toolchain.
- `checks/crosscheck_unicodedata.py`: every code point's name, category, combining class, bidi class, decomposition, mirroring, East Asian width and numeric value, compared with Python 3.14's `unicodedata` (Unicode 16.0; `/opt/homebrew/bin/python3.14` here, since `python3` resolves to mise's 3.11 / Unicode 14.0), an independent implementation, over the 294,645 code points designated by 16.0 (assigned characters plus the 66 noncharacters). All differences are explained: properties Unicode changed after 16.0 (U+0295 → Lo; U+1B3A/1B3C/1B42 bidi → L; U+5146 兆 now with two Unihan numeric values; eight cuneiform signs given numeric values), and 6,145 Tangut names that CPython does not generate.
- `checks/crosscheck_icu.mjs` (written by the verifier): general category and script compared with Node's ICU (Unicode 17.0) over all 297,400 code points assigned through 17.0, surrogates excluded: 0 mismatches.
- `checks/crosscheck_perl.pl`: block, script and age compared with Perl's `Unicode::UCD` (13.0) over 283,506 code points. Block and age agree everywhere; script differs on two (U+16FE2/16FE3, which became Han in 14.0). Slow: about 20 minutes.
- Not independently checked: NamesList annotations, chart headings, emoji sequences, CLDR emoji names and Unihan fields (spot checks and unit tests only), and anything new in 18.0.
- `probes/fake_responses_server.py`: a stand-in for the OpenAI Responses API that records each request and replays scripted tool calls. Pointed at by codex (`-c model_providers…`) or grok (`GROK_CLI_CHAT_PROXY_BASE_URL`), it shows the exact tool list a CLI would send a model and drives the full call path with no model quota. The codex and grok rows above were established with it; the commands are in `debrief.md`.

## Layout

```
src/ucd.rs        UCD parsing and lookups (all strings borrow from the embedded files)
src/tools.rs      the four tools: schemas, argument parsing, result shapes, unit tests
src/main.rs       MCP stdio loop (JSON-RPC 2.0), CLI modes: serve, query, tools, dump, agy-hook
src/log.rs        the call log
src/hook.rs       agy PreToolUse hook policy
tests/protocol.rs stdio protocol tests
checks/           cross-checks against Python, Node/ICU and Perl UCD implementations
evidence/         run records behind the "tested" claims, with the extraction script
probes/           fake model endpoint for testing CLI wiring without quota
integrations/     per-CLI launch scripts
fetch-ucd         data download + checksum verification + Unihan subset
data/             SHA256SUMS-18.0.0 and .derived (committed); data/18.0.0/ (fetched, ignored)
de-novo-feedback-1.md   the first independent verification; repairs are listed in debrief.md
```

Moving to a later Unicode version means running `UTF_MCP_ACCEPT_NEW=1 ./fetch-ucd <version>`, changing the version in the `ucd_file!` macro and `UCD_VERSION` in `src/ucd.rs`, rebuilding, and re-running the cross-checks.
