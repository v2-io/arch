# utf-mcp: de novo critical pass 1

*2026-10-05. Independent adversarial read of `firmatum/utils/utf-mcp/` at commit `0420485`, by a Claude Opus 5.5 instance that did not build it. I re-derived claims from the code, the data files and the builder's run artifacts, not from the README's account of them. Nothing in the tool directory was changed except adding this file. No model quota was spent: the two CLI runs below used the repo's own fake endpoint (`probes/fake_responses_server.py`). No CLI configuration was touched, and `agy` was not run.*

## What I ran

- `cargo test --release`: 9 unit and 4 stdio tests pass (rustc 1.96.0).
- `utf-mcp dump` with `checks/crosscheck_unicodedata.py` under Homebrew Python 3.14.7 (Unicode 16.0): output identical to the README's account (3 bidi, 1 gc, 9 numeric, 6,145 Tangut name differences).
- A **new** cross-check against Node 26.10 / ICU 78 (Unicode **17.0**): general category and script for every code point assigned through 17.0 (297,400, surrogates excluded), including the 4,803 added in 17.0, which no existing check covers. **0 mismatches.** The script is at the end of this report.
- The crosscheck's selection of code points: it filters by utf-mcp's own `age`, which could hide errors. Python's count of assigned code points (294,579) plus the 66 noncharacters (which have an Age) equals the 294,645 compared, so no code point was silently dropped.
- About 40 direct `utf-mcp query` probes of argument handling, the stdio loop and the agy hook. The headers of the fetched files and unicode.org's `ReadMe.txt` confirm 18.0.0 is the **final** release data, not beta.
- Two `integrations/grok.sh` runs against the fake endpoint to test `use_tool`'s `tool_input_file` (finding 3).
- The builder's run artifacts, which are still in this session's scratchpad, `/private/tmp/claude-505/…/scratchpad/` (finding 9), and the study's `METHODOLOGY.md` and `core/judges.py`.

## Overall

The data layer holds up. Parsing is clean and borrows from the embedded files, the `@missing` handling follows UAX #44, and the property cross-checks were reproduced and extended to Unicode 17 without a mismatch. The problems sit at the edges:

- The `search` and argument surface returns silent zeros and silent misreads.
- The containment claims for grok, agy and Claude are each weaker than stated.
- The study-facing argument in `proposed-integration.md` §1 is partly contradicted by the builder's own live evidence, and partly argues against a decision `METHODOLOGY.md` already made.
- The un-cross-checked data areas (NamesList, Unihan) have small fidelity losses.
- None of the "tested" evidence is durable.

Findings are ordered by how much I think they matter for the study's use of the tool.

## Findings

### 1. `search` returns a confident zero for hyphenated words, and the study's names are hyphenated

`search` splits the query only on whitespace, but splits names on spaces, hyphens and punctuation (`is_word_sep`). So any query word containing a hyphen can never match in the default `prefix` mode or in `word` mode:

```
search {"query":"day twenty-one"}        -> matches_total: 0
search {"query":"left-pointing angle"}   -> matches_total: 0   (substring mode: 18)
```

The character exists: U+33F4 IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWENTY-ONE, one of the glyphs in the README's own quick-start example. The study's glyph families are full of hyphenated ordinals (`TWENTY-ONE`, `TRUMP-11`, `LEFT-POINTING`), and a mind extending a sequence will search for exactly these. The answer gives no hint about why it is empty, and a model will reasonably read `matches_total: 0` as "no such character exists". For a tool that exists to answer factual questions, a confident false negative is the worst failure shape.

The same silent-zero pattern appears in the filters:

- `category: "so"` returns 0, because the short-code comparison is case-sensitive.
- `script: "Latn"` returns 0, because only long script names match.

Splitting query words on the same separators as names would fix the first. The filters could normalise case, accept the PropertyValueAliases short forms, or at least say in the result that the filter value is unknown.

### 2. `proposed-integration.md` §1: the suggested mitigation does not close the channel, and the methodology already settled the use

Two separate points.

**(a) `identify`-only does not remove the adjacency.** §1 proposes offering `identify` alone, at `brief` detail, as "closer to 'what is this glyph'". The builder's own live transcripts show why that fails:

- `it-claude`: Haiku called `identify {"text":"🃮 🃭 🃬"}`, computed the next code point itself, then called `identify {"codepoints":["U+1F0EB"]}`. It never used `range`.
- `claude-live-1` was the case that used `range`.

Any tool that accepts arbitrary code points is a `range` tool with a stride of one. Beyond that, the brief detail level returns names, and the names carry the ordinal structure: `PLAYING CARD TRUMP-14 / -13 / -12` and `… DAY TWENTY / TWENTY-ONE` state the series and its direction outright. A "what is this glyph" surface without the allocation scaffold would have to accept only the glyphs already in the prompt (`text`, with no `codepoints`, `names` or `search`). Even that leaves names that state the sequence. Whether a name counts as perception is the study's question, but §1's mitigation menu doesn't actually offer a scaffold-free option. The live evidence shows this, and the document cites that evidence without drawing the conclusion.

**(b) The use case was already decided.** `refactored/METHODOLOGY.md` line 111 was committed at 12:28 on 2026-10-05 (`5526f6f9`), before the tool's build finished. It already says:

- the tool is for a separate **reasoned pass**, "where reasoning is fine";
- Joseph expected it to be "probably especially useful for extending a sequence";
- reasoned answers are "their own tagged kind, never pooled with first impressions";
- first-impression passes are tool-free: Claude and Grok are offered no tools, and a Gemini tool attempt is recorded as its own outcome and re-asked as `no-lookup`.

So "if the aim is only to stop judges stalling on a denied shell" describes an aim the methodology rejects. Adjacency in the reasoned pass is what that pass is for. The debrief's "nothing in the brief said so" may be true of the brief, but the study's own front door said it. The open question I'd keep from §1 is narrower: within the reasoned pass, tag each answer by which information channel produced it (`range`, `identify` by code point, `search`, or none). The call log already supports that, and it is where the tool can add real value.

**(c) The motivating statistic does not reproduce as stated.** The README says that of 172 denied Gemini attempts, "151 identified …, 19 generated code-chart neighbours …, 2 asked only for `ord()`". The total reproduces: 172 conversations under `refactored/data/agy/brain/` have a `run_command`. The split does not. A naive classifier over the first command per conversation finds about 32 that generate neighbours by code-point arithmetic, e.g. `[chr(0x33FC - i) for i in range(1, 25)]`, `chr(ord('㏨') + i)`, `range(0x3369, 0x3371)`. Many of those also call `unicodedata.name`, so a mutually exclusive split depends on the counting rule. The rule isn't stated, and these numbers were inherited from mid-task statistics. Roughly a fifth of Gemini's attempts were doing the sequence-extension-by-chart move, not one ninth. That strengthens point (a): the arithmetic shape is common, and `identify` by code point serves it.

### 3. grok: `use_tool` reads arbitrary local JSON files, headless, under `integrations/grok.sh`

The README flagged `tool_input_file` as "untested beyond reading the schema". I tested it against the fake endpoint, using `integrations/grok.sh` unchanged (isolated `HOME`, `--tools search_tool,use_tool --allow 'mcp__unicode__*'`). The scripted model called:

```
use_tool {"tool_name":"unicode__identify","tool_input_file":"<scratchpad>/audit/secret.json"}
```

grok read the file, which lies outside the run's workdir, and passed its contents to the server as arguments. The call log records `"arguments":{"text":"SECRET-OUTSIDE-FILE ⑧","note":"…"}`, and `identify` echoed the text back to the model. No Read permission was granted beyond what the script sets.

- With a non-JSON file, grok returns `Invalid MCP arguments JSON at line 1, column 1`. So the channel also reveals whether a file exists and whether it is valid JSON.
- Any JSON object with a `text`, `query`, `names` or `codepoints` key is echoed in full. `search` echoes `query`, and `identify` lists `text_codepoints`.

For the study the practical risk is low: a judge would have to guess paths. But the README's "only tools" framing for grok is not accurate. grok judges have a file-read side channel. Every use of it lands in the utf-mcp log with the file's contents, so the log should be checked for arguments that don't look model-typed. The run artifacts are in the scratchpad under `audit/w1` and `audit/w2`.

### 4. agy hook: the allow decision keys on model-supplied arguments, not on the tool

`hook.rs` allows any tool call whose `args` contain `ServerName`, `serverName`, `server_name` or `server` equal to an allowed server, whatever the tool's `name` is:

```
{"toolCall":{"name":"run_command","args":{"CommandLine":"rm -rf ~/x","ServerName":"unicode_unicode"}}}
  -> {"decision":"allow","allowTool":true,"permissionOverrides":[…]}
{"toolCall":{"name":"view_file","args":{"AbsolutePath":"/etc/passwd","server":"unicode_unicode"}}}
  -> allow
```

The arguments are written by the model. The builder's own live hook record shows agy's MCP tool is named `call_mcp_tool`, so the allow branch could require `name == "call_mcp_tool"` and the server match together. I don't know whether agy strips undeclared arguments before calling the hook. If it does, this is latent rather than live, but the policy should not rely on that.

Three smaller hook points:

- The `permissionOverrides` also grant `mcp(unicode/*)`, derived by splitting `unicode_unicode` at `_`. That is broader than the "scoped to this server only" comment says. It is moot only while agy ignores overrides.
- The module doc (lines 4–8) still says "A hook in a workspace plugin can decide instead", which the same file's later comment and the README report as observed to be false.
- `--allow-tool` is undocumented.

What agy does when the hook times out or the binary is missing (open or closed) is unknown. That matters for option 2 in `proposed-integration.md` §3, which already notes the fail-open risk.

### 5. `codepoints`: integers are decimal and strings are hex, silently

```
identify {"codepoints":[2460]}      -> U+099C BENGALI LETTER JA
identify {"codepoints":["2460"]}    -> U+2460 CIRCLED DIGIT ONE
```

The schema says `items: string`, but `parse_cp_value` accepts JSON numbers as decimal, and nothing in the description mentions numbers. A model that sends `2460` meaning U+2460 gets a different character with no warning. The usual fixes are to reject numbers or to report the reading back (`"input": 2460, "read_as": "decimal"`).

Two near neighbours:

- `"U+33F3 U+33F4"` or `"U+33F3,U+33F4"` in one string gives `bad digits`. That fails loudly, which is fine, but splitting on whitespace and commas would accept what models naturally send.
- `tools/call` with `arguments` as a JSON-encoded string, which some harnesses produce, is treated as `{}` and answered "give at least one of `text`, `codepoints`, `names`". That message misdiagnoses the problem.

### 6. NamesList and Unihan: small fidelity losses in the areas the README says are spot-checked only

These are in exactly the areas the debrief honestly says have no independent check:

- **84 block-level notices are dropped.** An `@@` block header resets `cur_sub`, so the `@+` lines that follow it attach to nothing. Examples: Greek and Coptic, "Casing relationships between Greek letters are complex…"; Hangul Jamo, "The aliases in this block represent the Jamo short names"; Thai, "The encoding order is based on TIS 620-2533". The 16 `@@+` lines and the 35 double-tab `\t\tx` cross-references under notices are also skipped. `blocks` advertises "Unicode's notes" and returns subheader notes only.
- **Trailing group notices go to the last character.** At U+050F the notice "These letters are for the obsolete Molodtsov alphabet…" ends the Komi-letters group. utf-mcp shows it as a note on U+050F alone, and not on U+0500–U+050E. Some char-following `@+` lines really are about one character (U+0418's), so position alone can't decide. I'd mark these as positional rather than present them as the character's own note.
- **The radical glyph is wrong for simplified radicals.** 纟 U+7E9F has `kRSUnicode 120'.0` and is rendered `⽷ KANGXI RADICAL SILK (#120, simplified form)`. That shows the traditional radical's glyph next to the word "simplified". `''` (non-Chinese simplified, 53 entries) and `'''` (1) get the same label.
- **LM2 edge:** `"hangul_jungseong_o-e"` resolves to U+116C, not U+1180, because the O-E exception is tested after whitespace-joining but before underscores are dropped.
- **Emoji names:** "emoji names like `flag: Japan` accepted" holds only for multi-code-point sequences. `"red heart"` resolves only because it is a `2764 FE0F` Basic_Emoji sequence. Single-code-point CLDR names (`"grinning face with big eyes"`) fail, because emoji-test.txt is not loaded.

### 7. Claude: the "changes nothing under `~/.claude`" claim is false

Each `integrations/claude.sh` run leaves a new empty directory, `~/.claude/projects/<slug-of-the-mktemp-workdir>/memory/`. Two are there now, from the builder's 12:46 and 13:25 runs. The stream's `init` event shows `memory_paths.auto` pointing there: `--setting-sources ""` does not turn auto-memory off. Since every run uses a fresh `mktemp` workdir, a study round leaves one directory per call in Joseph's daily config tree.

It is harmless in content: the judges have no write tools, so the directories stay empty and nothing is loaded from them. But it is a direct claim in the README and in each script's header, and it is unbounded. A fixed cwd per mind, or a setting that disables auto-memory, would bound it. I did not test which setting does that.

On the contamination question: the 47 KB `~/.claude/CLAUDE.md` would be roughly 12k tokens, and `it-claude`'s cached prefix is about 9.8k tokens in total. That is consistent with it not being loaded. This is an inference from sizes, not a verification.

### 8. Integration scripts interpolate unescaped values into JSON, TOML and a shell command

`claude.sh`, `agy.sh` and `codex.sh` build JSON or TOML with `printf '%s'`, and `grok.sh` with a heredoc. `agy.sh` writes `"command": "'$UTF_MCP' --log '$LOG' --tag '$TAG' agy-hook …"`, which agy later runs through a shell. A tag or path containing `'` or `"` breaks the config. In the hook string it becomes command execution under agy.

The inputs are operator-controlled, so this is low. But `proposed-integration.md` §4 recommends passing ledger ids as `--tag`, so tag hygiene now depends on the id format. JSON-encoding via `jq` or Python, or validating tags against `[A-Za-z0-9._-]`, would close it.

Related, and unverified: `codex.sh` and `grok.sh` symlink `~/.codex/auth.json` and `~/.grok/auth.json` into the isolated home. This pattern is inherited from `judges.py`. If either CLI refreshes an OAuth token by write-then-rename, the isolated home gets a new file. If the provider rotates refresh tokens, the copy in Joseph's real home then goes stale, so a judge run could quietly log out his daily CLI. I did not test it. It is worth one look at a real refresh.

### 9. The "tested" evidence is not durable

Every live and fake-endpoint result in the README's "Tested state" table rests on files in the session scratchpad: `claude-test.jsonl`, `it-claude.out`, `grok-live2.jsonl`, `agy-test.jsonl`, `hooktest.jsonl`, `fake-req.jsonl` and others. That directory is temporary and not referenced from the repo. Once it is cleaned, the table's claims can't be traced.

Committing a small `evidence/2026-10-05/` with the utf-mcp call logs, the `init` events and the captured tool lists would make them traceable. These are a few tens of KB, minus anything private: the call logs' `parent_command` holds the full prompt. This audit was only possible because the scratchpad happened to be shared.

Also, the docstrings in `checks/crosscheck_unicodedata.py`, `checks/crosscheck_perl.pl` and `probes/fake_responses_server.py` give `tests/…` paths. So do the builder's `gprobe.sh` runs, so the files were moved after those probes ran.

### 10. Smaller things

- `mise run crosscheck` runs `python3`. In this directory that resolves to mise's 3.11 (Unicode 14.0), as the debrief noticed, not the 3.14 / 16.0 the README reports. The task still works, because it filters by age, but it checks less than documented. Pinning `/opt/homebrew/bin/python3.14`, or printing the version prominently, would close the gap.
- `blocks {"query":"ab"}` returns Latin-1 Supplement, because two-to-six-character hex-looking queries parse as code points before block names are tried.
- An empty JSON-RPC batch `[]` gets no reply; JSON-RPC 2.0 requires an error. MCP clients don't send batches since 2025-06-18, so this is cosmetic.
- `fetch-ucd` verifies the downloads, but the derived `unihan-subset.txt`, which is what gets compiled in, has no recorded checksum. A different `unzip`/`grep` could change the binary silently. Unicode publishes no hashes, so the committed SUMS are trust-on-first-fetch. That is fine, but worth saying in the README.
- `Cargo.toml` has no `rust-version`, though the code uses edition 2024 and `str::floor_char_boundary`. I believe that was stabilized only recently, in 1.91. Not verified.
- The log claims each record is "single `write` calls of one line each". That holds in practice with `O_APPEND` on a local disk, but `write_all` may loop on a partial write, and a 512-row `range` result is about 60 KB. It is fine for this use. I'd soften "single" only if the claim is ever relied on.
- README nit: "the 294,645 code points assigned by 16.0" includes the 66 noncharacters, which are designated but not assigned characters.

## What held up

These README and debrief claims I checked directly and found accurate:

- the CLI versions (claude 2.1.289, codex 0.157.1, grok 1.0.44, agy 1.2.17);
- "353 blocks", "~20 MB", the test counts, and the startup cost (about 65 ms for a full load);
- the Python crosscheck output and its explanation of every difference;
- Claude's tool set: the `init` event lists exactly the four tools;
- agy: auto-denial in print mode, the `server/discover` probe before `initialize`, and the `unicode_unicode` naming;
- `call_mcp_tool` carrying `ServerName` and `ToolName`;
- the grok `--allow` gating;
- the debrief's account of its own grok mistake, which `grok-test.stream.jsonl` shows (19 `run_terminal_command` mentions).

Beyond what was claimed, gc and script agree with ICU's Unicode 17.0 across all 297,400 code points assigned through 17.0. The protocol loop, the oversized-line refusal and the parse-error ids behave as documented.

## Not checked

- The Perl cross-check (about 20 minutes; not run).
- Case mappings, Script_Extensions, emoji properties and named sequences, beyond spot checks.
- Anything new in 18.0, which has no independent implementation on this machine.
- Codex's code-mode path.
- Whether agy validates tool arguments before calling the hook.
- Token-refresh behaviour through the auth symlinks.

## The Node/ICU cross-check, for reuse

```js
// node xcheck.mjs dump.tsv   — gc and script vs ICU (process.versions.unicode)
import fs from 'fs';
const lines = fs.readFileSync(process.argv[2], 'utf8').split('\n'); const h = lines[0].split('\t');
const I = n => h.indexOf(n); const cache = new Map();
const re = (k, v) => { const key = k + '=' + v; if (!cache.has(key)) { let r; try { r = new RegExp(`^\\p{${k}=${v}}$`, 'u'); } catch (e) { r = e.message; } cache.set(key, r); } return cache.get(key); };
const le17 = a => { const [x, y] = a.split('.').map(Number); return x < 17 || (x === 17 && y === 0); };
const bad = { gc: [], sc: [] }; let n = 0;
for (const l of lines.slice(1)) { if (!l) continue; const r = l.split('\t'); const age = r[I('age')];
  if (age === 'Unassigned' || !le17(age)) continue; const cp = parseInt(r[0], 16); if (cp >= 0xD800 && cp <= 0xDFFF) continue;
  const ch = String.fromCodePoint(cp); n++;
  for (const [k, col, b] of [['General_Category', 'gc', 'gc'], ['Script', 'script', 'sc']]) { const x = re(k, r[I(col)]); if (typeof x === 'string' || !x.test(ch)) bad[b].push(r[0]); } }
console.log('unicode', process.versions.unicode, 'compared', n, 'gc', bad.gc.length, 'sc', bad.sc.length);
```

It could sit beside the Python and Perl checks: it is the newest independent implementation on this machine, it runs in a few seconds, and it extends to Script_Extensions through `\p{scx=…}` if the dump gains that column.
