# Help & Version

Aspectus describes itself. That is the data of this invocation.

## Story

`help`, `-h`, and `--help` print the description. An unrecognized verb or option refuses, names the class, and points at `help`. `version`, `-v`, and `--version` print the version (and the commit SHA when this build is not a tagged release).

## Foundations (clauses, not whole files)

| Clause | Where |
|---|---|
| Stdout is pipeable data; diagnostics, teaching, progress on stderr | [[../../../principles/src/norm-stdout-is-data\|norm-stdout-is-data]] |
| Usage errors exit 2; success 0 | [[../../../principles/src/form-exit-vocabulary\|form-exit-vocabulary]] |
| `-h`/`--help`, `--version`, `--` end-of-flags | [[../../../principles/src/form-shared-flags\|form-shared-flags]] |
| `-v` is **version** on this tool, not verbose | stays here (aspectus overlay; shared flags unbind verbose-`-v` as floor) |
| Help is the law channel *before* a refusal | [[../../../principles/src/norm-help-is-law-channel\|norm-help-is-law-channel]] |
| A refusal names the class | [[../../../principles/src/norm-refusal-names-class\|norm-refusal-names-class]] |
| A refusal offers a next action | [[../../../principles/src/norm-refusal-offers-next\|norm-refusal-offers-next]] |
| Name + semver; +sha when untagged | [[../../../principles/src/norm-version-line\|norm-version-line]] |
| Help shape: usage, what it is, commands, options, examples | [[../../../principles/src/form-help-shape\|form-help-shape]] |
| Man page sections when we grow a page | still [[../../principles/influx/cli-conventions/documentation-standards#Man Page Sections\|docs · man]] (not effluxed) |
| TTY does not change which stream; color follows TTY; no prompts | [[../../../principles/src/norm-stdout-is-data\|norm-stdout-is-data]] · [[../../../principles/src/norm-color-follows-tty\|norm-color-follows-tty]] · [[../../../principles/src/norm-no-prompts\|norm-no-prompts]] |
| Do not require `ASPECTUS_AGENT_MODE=1` | [[../../../principles/src/norm-machine-path-is-detected\|norm-machine-path-is-detected]] |
| `--help --format=json` / `--list-flags` are a later axis | still [[../../principles/influx/cli-conventions/ai-agent-considerations#Help for Agents\|agent · machine help]] (not effluxed) |

## Subfeatures

| # | Sub | Behavior | Test | Help / version text |
|---|---|---|---|---|
| 1 | Asked-for help | `help`, `-h`, `--help` write the help page to **stdout**, exit **0**. | `aspectus help`; `aspectus -h`; `aspectus --help`. Assert stdout matches the page below; stderr empty; exit 0. `aspectus --help \| wc -l` is nonzero. | The page in §Help page. |
| 2 | Asked-for version | `version`, `-v`, `--version` write one version line to **stdout**, exit **0**. | Same three spellings. Tagged release: `aspectus 0.1.0\n`. Untagged: `aspectus 0.1.0+<sha>\n` (short SHA). No date, no rustc, no OS. | That one line. Also named on the help page. |
| 3 | Unknown option | e.g. `--nope` → **stderr**, exit **2**. Class: unknown option. Next action: `aspectus help`. Full help page is **not** reprinted. | `aspectus --nope`. stderr contains `unknown option` and `--nope` and `aspectus help`; stdout empty; exit 2. | Refusal only. |
| 4 | Unknown verb | e.g. `frob` → **stderr**, exit **2**. Class: unknown verb. Distinct from (3). Next action: `aspectus help`. | `aspectus frob`. stderr contains `unknown verb` and `frob`; stdout empty; exit 2. | Refusal only. |
| 5 | End of flags | `--` stops flag parse. | `aspectus -- --help` treats `--help` as a path/verb, not help. (Once glance exists: it is a locus named `--help`. Until then: unknown verb `--help`.) | Mention `--` under options. |
| 6 | Help names the tool | The page uses the words **aspectus** and **aspecta** as defined. | stdout of `--help` contains both words in the sense of the defs (faculty / one look). | First lines of §Help page. |
| 7 | Examples teach the forms | The Examples block is the agent’s first lesson. **Rewrite this block whenever a new verb or flag ships.** A stale example is a failed test. | Snapshot test of the Examples section. When glance lands, add `aspectus` and `aspectus PATH` here in the same commit. | §Help page · Examples. |
| 8 | One source | Parser, help page, and (later) man page take flag/verb names from one list. Adding a flag without updating that list fails a test. | Test: every flag the parser accepts appears in help; every flag help lists is accepted or documented as not-yet. | Generated from that list. |

## Help page

Help is the law channel. The “what it is” block is the def prose (aspectus, locus, aspecta, the neighbors it is not) — not a paraphrase and not a slogan. First line is the version line. Shape: usage → what it is → commands → options → examples.

## Version line

```
aspectus <semver>            # tagged
aspectus <semver>+<sha>      # otherwise
```

From `Cargo.toml` version. SHA from the build, not from a live `git` call at runtime if we can avoid it.

## Not in this row

`--help --format=json`, `--list-flags`, completions, man(1) file, self-update, `--verbose`. Man page sections in the docs convention are the shape to follow *when* a page is written; they do not gate this story.

Color belongs with the first picture, not with help.

## Open — help's shape (2026-08-22 cold reads, both substrates; usability pass item 9)

*Why open:* both readers said the first picture taught more than the essay, that `aspectus config` is the real documentation, and that the orientation recipe they arrived at — `--depth 1 PATH`, then `--lines 0 --depth 1 PATH`, then enter — is not what the examples lead with (`audit/hallway-2026-08-22.md` #7). Help is the law channel, so the content stays; the *order* is the question: a one-screen usage + recipe table first, the defining prose and the glyph tables after, under headings. Deferred until the header designation and the subgroup form settle, so the examples teach the final grammar once.
