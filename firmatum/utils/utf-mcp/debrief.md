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

## The tools and the study's reasoned pass

The study's methodology had already put tool use in a separate reasoned pass, where lookups are expected and answers are tagged apart from first impressions. I had read only the first sections of `METHODOLOGY.md` and missed it, so my first version of `proposed-integration.md` §1 argued against a decision you had made. It also proposed `identify`-only as a way to keep code-chart order out, which can't work: looking up an arbitrary code point is a stride-one `range`, and Haiku did exactly that. §1 now proposes what the tool is good for there instead: tagging each reasoned answer by the information channels it used, all of which are recoverable from the call log.

## On the brief

It worked well for me. The consent framing, the explicit "a precise account of why a CLI can't do this is a full result", the machine facts marked as checked or inherited, and the keychain warning each changed what I did. The inherited fact "python3 is 3.14 / Unicode 16" holds for Homebrew's `/opt/homebrew/bin/python3`, but in my shells `python3` resolved to mise's 3.11 / Unicode 14 in every directory I tried. That mismatch is part of why I avoided interpreter-dependent data. The mid-task Gemini statistics were the most useful single input: they set the tool surface.

## After the first independent verification

`de-novo-feedback-1.md` (an independent Claude Opus 5.5 pass, commit `8d59ac2`) found the data sound, adding a Node/ICU cross-check on Unicode 17.0 with 0 mismatches, and found the problems at the edges. What I did with each finding:

| # | finding | done |
|---|---|---|
| 1 | `search` returned 0 for hyphenated words (`day twenty-one`), lowercase category codes, script codes | Query words split exactly like names; category and script filters accept codes or names in any case (via PropertyValueAliases, groups like `S` and `LC` included); unknown filter values are errors; an empty result carries a `note` saying what was searched and what else to try |
| 2a | `identify`-only does not remove chart adjacency | Agreed; §1 rewritten (above) |
| 2b | the methodology had already settled the use | Agreed; §1 rewritten, README states the reasoned-pass purpose |
| 2c | the 151/19/2 split doesn't reproduce | Recounted with a stated, overlapping rule (`evidence/2026-10-05/gemini-shell-attempts.py`): 172 conversations, 149 names, 31 arithmetic (12 both), 3 `ord`/`hex` only, 1 built neighbours from literal code points |
| 3 | grok's `use_tool` reads local JSON files (`tool_input_file`) | Confirmed; `--deny Read` (four forms) and `--sandbox` (`read-only`, `workspace`, `strict`) do not stop it. Not closable from the server. The server now refuses argument keys outside its schema without echoing their values; `grok.sh` flags any run whose stream shows a file argument (stderr warning, `harness_warning` log record, exit 3) (this flagging made `grok.sh` orphan grok when killed; it moved to `grok-check.py` after the second verification, A1 below); README states the channel |
| 4 | agy hook allowed any tool carrying a model-written `ServerName` | Allow now requires agy's `call_mcp_tool` step; overrides scoped to the one server; module doc corrected; `--allow-tool` documented; tests added for the spoofing cases. Hook-failure behaviour remains untested |
| 5 | JSON numbers read as decimal; several code points in one string refused; string-encoded `arguments` misdiagnosed | Numbers refused with a message giving both readings; strings split on spaces and commas; JSON-encoded `arguments` decoded |
| 6 | NamesList/Unihan fidelity | Block-level notices (84) and notice cross-references now kept and returned by `blocks`; a notice printed after a character is returned as `chart_notice_following` with its positional scope stated; simplified radicals shown with their own glyph from CJKRadicals.txt (UAX #38 forms named); the LM2 O-E exception now survives underscores; single-code-point CLDR emoji names (emoji-test.txt) resolve and are searchable |
| 7 | `claude.sh` leaves a memory directory under `~/.claude/projects/` per run | `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` stops it (verified with no model call); README claim corrected. Three empty directories from my runs remain under `~/.claude/projects/` (names ending `scratchpad-cl`, `scratchpad-it-claude`, `scratchpad-memprobe-base`). The last comes from the probe that established the fix. I left them for you to delete or keep |
| 8 | unescaped values in generated JSON/TOML/shell; auth symlink refresh | Scripts refuse tags outside `[A-Za-z0-9._:+-]` and paths needing escaping. Auth: the grok judge home's token refresh at 08:23 today wrote through the symlink to `~/.grok/auth.json` and left the link intact (circumstantial: its lock file and the real file share the timestamp). Codex refresh is unobserved |
| 9 | "tested" evidence lived only in the scratchpad | `evidence/2026-10-05/`: call logs verbatim plus reduced streams and tool lists, with the extraction script; some server logs had already been overwritten by later runs, and its README says which. Stale `tests/…` paths in docstrings fixed |
| 10 | smaller | `mise run crosscheck` uses Homebrew `python3.14` when present and runs the ICU check; `blocks` tries names before bare hex; empty batch answered with an error; derived `unihan-subset.txt` checksummed; trust-on-first-fetch stated; `rust-version = "1.85"`, built with that toolchain (installed via rustup for the check); the append-atomicity claim softened; "assigned" corrected to "designated" |

Also adopted from the verifier: `checks/crosscheck_icu.mjs`. Tests are now 10 unit and 5 protocol. Machine changes outside the tool directory: the Rust 1.85.0 toolchain in `~/.rustup`, the empty directory above, and grok session records from the probe runs.

## After the second verification

`de-novo-feedback-2.md` found all ten first-round repairs holding, and one regression I had introduced: to scan grok's stream after a run, `grok.sh` stopped `exec`-ing grok, so a harness timeout that killed the script left grok, `tee` and the server running. What I did:

| # | finding | done |
|---|---|---|
| A1 | killing `grok.sh` orphans grok, tee and the server | `grok.sh` `exec`s grok again, with the stream copied by `tee` through process substitution. Tested the verifier's way (a socket that never replies, `kill -9` of the script after 6 s): the script's PID was grok, and two seconds after the kill no grok, `tee` or `utf-mcp` process remained. The file-argument scan moved to a separate step, `integrations/grok-check.py WORKDIR`, which the harness runs after the call |
| A2 | the agy hook interpolated a model-written `ToolName` into its permission grant | The grant is now only `mcp(<server>/*)`, built from the operator's server name; a `ToolName` that is not a plain identifier is denied. Test added for the verifier's crafted string |
| A3 | no post-repair evidence for most CLIs | Claude: cited the study's live post-repair calls (by path and commit, not copied). grok: post-repair call logs for the inline, `tool_input_file` and `file` forms added. codex and grok schema acceptance: the verifier's second pass. agy: still no post-repair run (blocked as before). The evidence README now says which files predate the repairs |
| A4 | hyphen splitting ranked scattered-word matches alongside phrases | A middle tier: names containing the query as a verbatim phrase (hyphen and space alike) rank after exact matches and before scattered ones. `twenty-one` now lists CIRCLED NUMBER TWENTY ONE and the two TWENTY-ONE telegraph symbols before the musical symbols |
| A5 | small inaccuracies | Non-integer and negative numbers get a plain message. The README now says `harness_warning` records come from the checker and lack `session`/sequence. The channel table says its rows overlap and suggests tagging requested vs returned. Detection is now a JSON parse that accepts `rawInput` as an object or a JSON string and exits 4 when the stream is unreadable, rather than a grep that would fail open on a format change. The `file` form is tested. `gemini-shell-attempts.py` prints its usage instead of crashing without an argument |
| — | "sun" resolves to U+2609, not ☀ | Kept as a deliberate choice (character names win) and stated in the README |

Something the verifier didn't raise, found while checking A3: the README said every session ends with an `exit` record. That holds only when the client closes the server's stdin (agy). Claude Code, codex and grok end the server with a signal, so their logs stop at the last call. The README now says so; analyses should not treat a missing `exit` as a crash.

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
