# utf-mcp: verification of the repairs

*2026-10-05. Second pass by the same Claude Opus 5.5 instance that wrote `de-novo-feedback-1.md`. It checks the repairs in `c869e66` against those findings, working from the code and its behaviour rather than from `debrief.md`'s table, and looks for anything the repairs broke. Nothing was changed except adding this file. No model quota was spent: the CLI runs below used `probes/fake_responses_server.py` or a local socket that never replies. I did not run `agy`, and I touched nothing under `~/.claude`, `~/.codex`, `~/.grok` or `~/.gemini`.*

## Method

- Read the full diff of `c869e66` for `src/`, `integrations/`, `fetch-ucd`, `mise.toml`, the README and `proposed-integration.md`.
- Re-ran `./fetch-ucd`, which verifies both the source checksums and the new derived checksum, then `cargo test --release` (10 unit and 5 protocol tests pass) and `cargo +1.85.0 check` (passes).
- Re-ran the Python and ICU cross-checks on a fresh `dump`. The dump is byte-identical to the pre-repair one, so the core properties were not disturbed. The ICU check gives 0/0 and the Python check gives the same explained differences as before.
- Probed each repaired behaviour directly with `utf-mcp query` and the stdio loop.
- Ran three CLI checks against local stand-ins:
  - `integrations/grok.sh` with a `tool_input_file` pointing at two different files;
  - `integrations/codex.sh` (gpt-5.5, classic tools) end to end;
  - `integrations/grok.sh` against a socket that never replies, which I then killed the way a harness timeout does.
- Re-ran `evidence/2026-10-05/gemini-shell-attempts.py` against the study's transcripts.

## Finding by finding

| # | finding (report 1) | state after `c869e66` |
|---|---|---|
| 1 | `search` silent zeros | **Repaired.** `day twenty-one` gives U+33F4 in prefix and word mode. `left-pointing angle` gives 11. Lowercase `so` and code `Latn` are accepted. Unknown script, category or block values are errors, and an empty result carries a `note`. Side effect: see A4. |
| 2a | `identify`-only doesn't remove adjacency | **Repaired** in `proposed-integration.md` §1, which now says so plainly. |
| 2b | the methodology already settled the use | **Repaired.** §1 and the README now name the reasoned pass and quote Joseph. |
| 2c | 151/19/2 split | **Repaired.** The script reproduces exactly: 172 conversations; 149 names; 31 arithmetic, 12 of them also names; 4 neither. The counting rule is in its docstring. (It exits with an `IndexError` if run without the path argument the docstring shows.) |
| 3 | grok `tool_input_file` read channel | **Narrowed and flagged, as the debrief says; not closed.** Confirmed post-repair against the fake endpoint. A file with an extra key gets `error: unknown argument […]`, which names the keys but echoes no values. A file shaped like valid arguments (`{"text":"ONLY-TEXT-KEY-FILE"}`) is still echoed. In both cases `grok.sh` printed the warning, appended a `harness_warning` record and exited 3. The call log still records the file's full contents under `arguments`, which is right for audit but means the log may hold local file contents. |
| 4 | agy hook keyed on model-written args | **Repaired** for the reported case: `run_command` + `ServerName:"unicode_unicode"` is now denied, and so is `call_mcp_tool` with conflicting `ServerName`/`serverName`. One latent issue remains: see A2. |
| 5 | number/string code points, multi-code-point strings, string-encoded `arguments` | **Repaired.** `2460` is refused with both readings offered. `"① ②"`, `"U+33F3 U+33F4"` and `"U+1F0A1,U+1F0A2"` split. A JSON-string `arguments` is decoded both in `query` mode and over stdio, and a non-object is refused clearly. The error message misfires for non-integer numbers: see A5. |
| 6 | NamesList/Unihan fidelity | **Repaired.** The Greek and Coptic block note and the IPA notice cross-references are returned. U+050F/U+0418/U+0269 notices come back as `chart_notice_following` with a stated positional scope. 纟 shows ⺰ (C-SIMPLIFIED SILK). Every primed radical used as a first kRSUnicode value has a CJKRadicals entry, so the fallback branch is currently unreachable. The `hangul_jungseong_o-e` and `grinning face with big eyes` lookups resolve correctly. **Correction to my report 1:** the 16 `@@+` lines I listed as dropped are empty markers with no text, so nothing was lost there. |
| 7 | `claude.sh` creates `~/.claude/projects/…/memory/` | **Repaired per the builder's evidence; not re-run by me.** The two committed init events differ exactly as claimed (`memory_paths.auto` present without the variable, absent with it, both against a non-existent model). I did not reproduce it, because if the variable failed the probe would itself create a directory under `~/.claude`. |
| 8 | unescaped interpolation; auth symlink refresh | **Repaired by validation.** Tags must match `[A-Za-z0-9._:+-]{1,128}`, and paths are limited to a safe set. The auth-refresh observation is reported as circumstantial, which matches what was observed. |
| 9 | evidence not durable | **Repaired.** `evidence/2026-10-05/` (148 KB) holds the call logs verbatim and reduced streams. I scanned it for tokens, keys and e-mail addresses; only the env-var *name* `OPENAI_API_KEY` appears. Caveat in A3. |
| 10 | smaller items | **All repaired as described.** Empty batch gives a `-32600` error. `blocks "ab"` now lists blocks whose names contain AB, and `U+00AB` forces the code point. The derived checksum is verified by `fetch-ucd`. `rust-version = "1.85"` checks under 1.85.0. `mise run crosscheck` uses `python3.14` when present and runs the ICU check. |

## What the repairs broke or introduced

### A1. `grok.sh` no longer `exec`s grok, so killing the script orphans grok (demonstrated)

To scan the stream after the run, `grok.sh` now runs `grok … | tee …` as a child instead of `exec`-ing it. Python's `subprocess.run(timeout=…)` and most harness timeouts kill only the direct child. To reproduce:

1. Point `GROK_CLI_CHAT_PROXY_BASE_URL` at a local socket that accepts connections and never replies.
2. Start `integrations/grok.sh`, wait 6 s, then `kill -9` the script.
3. `grok-1.0.44`, `tee` and the `utf-mcp` server are all still running.

I cleaned them up afterwards and confirmed none remained. In a live run, an orphaned grok would keep its model session going after the harness had moved on, spending quota and appending to the round's log under a tag the harness considers finished. Before the repair, `exec` made the timeout kill grok itself.

Options I can see:

- The harness starts the script in its own session (`start_new_session=True`) and kills the process group on timeout.
- The script traps `TERM`/`INT` and forwards them. This does nothing for `SIGKILL`.
- Restore `exec` and move the post-run scan into the adapter.

The study's current `grok_call` doesn't use `grok.sh`. But `proposed-integration.md` §2 offers the scripts as recipes to copy, so this shape could spread.

### A2. The hook interpolates a model-written `ToolName` into the permission grant (latent)

```
{"toolCall":{"name":"call_mcp_tool","args":{"ServerName":"unicode_unicode","ToolName":"x)/*, mcp(chrome_devtools"}}}
  -> "permissionOverrides":["mcp(unicode_unicode/*)","mcp(unicode_unicode/x)/*, mcp(chrome_devtools)"]
```

agy 1.2.17 ignores `permissionOverrides` (observed by the builder), so this has no effect today. The overrides are kept "in case a later agy honours them", and that is exactly when a parser that splits on `, ` or `)` would turn this into a wider grant. Validating `ToolName` against the four tool names, or keeping only the `mcp(unicode_unicode/*)` grant, removes it.

### A3. The schema tightening has no committed post-repair evidence; I filled two of the gaps

Every CLI record in `evidence/2026-10-05/call-logs/` predates the repair commit. The latest is 19:25 UTC, and the commit is 20:05 UTC. The exception is the grok `tool_input_file` after-repairs summary. The repair changed what every client sees: `additionalProperties: false` on all four schemas, and hard refusal of unknown keys.

Post-repair I confirmed:

- **codex (gpt-5.5, classic):** `tool_search` returns the new schema (`additionalProperties: false`, `strict: false`), and the namespaced `identify` call reaches the server and succeeds.
- **grok:** `search_tool` and `use_tool` work with the new schema.

Not exercised post-repair:

- **Claude.** I'd expect no problem, but it was not run.
- **agy.** It passes arguments through `call_mcp_tool`, so the schema is unlikely to be a native function declaration there; not checked.

Two smaller points:

- The evidence README presents the CLI files as backing the current README's claims without saying they record the pre-repair binary.
- Behaviour change: a model that adds a harmless extra key (e.g. `limit` on `identify`) now gets an error instead of an answer. That is the intended trade, with a readable message. It will show up in the reasoned pass as extra failed calls.

### A4. Hyphen splitting trades precision for recall

`search "twenty-one"` now returns 7 results. Besides CIRCLED NUMBER TWENTY ONE and the two TWENTY-ONE telegraph symbols, it includes MUSICAL SYMBOL ONE HUNDRED TWENTY-EIGHTH NOTE/REST and SQUARED ONE HUNDRED TWENTY P, because the words may match anywhere and in any order. Exact matches still rank first, but nothing ranks the adjacent-phrase matches above scattered ones. That is better than the old silent zero. A small fix would be to rank a hyphenated query word that appears verbatim in the name ahead of the rest.

### A5. Small inaccuracies introduced

- **Non-integer numbers.** The number-refusal message is built for integers. `2460.0` gets the advice `"U+2460.0"` / `"&#2460.0;"`, and `-1` gets `"U+-1"` / `"&#-1;"`. Neither suggestion parses.
- **`harness_warning` records.** The README says every log record carries a per-process `session` id and a sequence number. The `harness_warning` record that `grok.sh` appends has neither (it has `ts`, `event`, `tag`, `harness`, `warning`, `stream`). Tag plus timestamp still joins it; only the claim is wrong.
- **Default detail and the channel table.** The new channel table in `proposed-integration.md` §1 counts "`identify` at `standard`/`full`" as the *chart structure* channel. `standard` is the default, so almost every `identify` call will carry that tag unless the judge asks for `brief`. That may be intended, since the chart headings really are in the answer, but the table reads as if it separates channels that will in practice nearly always co-occur.
- **Detection format.** The `grok.sh` detection greps for an unescaped `"tool_input_file":` inside a `"toolName":"use_tool"` line. That matches grok 1.0.44's stream format; I confirmed it fires. If a later grok JSON-encodes `rawInput` as a string, the grep stops matching and the check fails open, silently. A run with no `use_tool` lines at all is indistinguishable from that.

## Not checked

- `CLAUDE_CODE_DISABLE_AUTO_MEMORY` (reason above).
- A live Claude or agy call after the repair.
- The `file` form (as opposed to `tool_input_file`) of grok's `use_tool` through the new detector.
- The Perl cross-check.
- Whether the new CLDR emoji names collide in a way that matters. Character names win the loose-name index, so `"sun"` resolves to U+2609 SUN rather than ☀ U+2600, which is CLDR "sun". That is defensible, but it is a choice.

## Summary

The ten findings are repaired as the debrief describes, or narrowed where the debrief says so (finding 3), and the repairs did not disturb the core property data. Three things are new:

- the orphaned-grok regression in `grok.sh` (A1), which matters if the script shape is copied into a harness with timeouts;
- a latent grant-string injection in the agy hook (A2);
- post-repair evidence that is thinner than the evidence README implies (A3; I covered codex and grok, not Claude).

My own report 1 overstated one point: the `@@+` lines carry no text.

Run artifacts for this pass are in this session's scratchpad under `audit2/`; they are not durable.
