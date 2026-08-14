# First efflux — from the three aspectus designs

> [!note] 2026-08-14: three atoms in this record — [[form-config-precedence]], [[norm-config-walks-from-locus]], [[norm-overlays-are-config]] — were rewritten after [[norm-caller-tunes-the-channel]] was carved (the original carve had inherited "repo file / locus walk" from the convention pack against the decided caller-stack design). The tables below describe the atoms *as first carved*; the atoms themselves are current truth.

`src/` did not exist until this. Demand set: the Foundations tables in [[../../utils/aspectus/design/help-verb|help-verb]], [[../../utils/aspectus/design/two-level|two-level]], [[../../utils/aspectus/design/config|config]], plus the sibling clause those rows already use in the same breath.

Influx stays gather. Status on every atom is the influx/outline type (`formulation` / `normative` / `demand`) and `state: influx`. Landing here is not a promotion. The *carve* (what we bind vs leftover axis) is the one already decided in [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.

Cite the atom, not the gather file. Appendix A of that outline already said carved `src/` segments supersede the pointer when they exist.

## Grain

Split when a design row was still two ideas (class vs menu; path resolution vs locus-walk; stream assignment vs color vs no-prompts). Merged when two design rows were one clause (stdout-is-data also covers "teaching is not stdout" and "TTY does not change which stream"). One atom when two headings were one idea (version display + semver; precedence + XDG).

Not taken: man-page section list; `--help --format=json` / `--list-flags`; the rest of `cli-conventions/` (i18n, load-testing, MCP, signals, dry-run-as-plan, format family, truncation verdict, …). Those wait on a row that actually cites them.

Aspectus-only stays in the crate: `-v` is version (not verbose); config filename still open; help Examples must gain `aspectus` / `aspectus PATH` in the same commit.

## Retarget — `design/help-verb.md`

| Design clause | Atom |
|---|---|
| Stdout is pipeable data; diagnostics on stderr | [[norm-stdout-is-data]] |
| Teaching / “Done!” / progress are not stdout | [[norm-stdout-is-data]] (same clause) |
| Usage errors exit 2; success 0 | [[form-exit-vocabulary]] |
| `-h`/`--help`, `--version`, `--` end-of-flags | [[form-shared-flags]] |
| `-v` is version on this tool, not verbose | *stays in the design* (aspectus overlay of the `-v` axis) |
| Help is the law channel *before* a refusal | [[norm-help-is-law-channel]] |
| A refusal names the class and offers a next action | [[norm-refusal-names-class]] · [[norm-refusal-offers-next]] |
| Name + semver; commit when we have a release story | [[norm-version-line]] |
| Help shape: usage, what it is, commands, options, examples | [[form-help-shape]] |
| Man page sections when we grow a page | *not effluxed* — still [[../influx/cli-conventions/documentation-standards#Man Page Sections|docs · man]] |
| TTY changes color/prompts, not which stream. No prompts. | [[norm-stdout-is-data]] (stream) · [[norm-color-follows-tty]] · [[norm-no-prompts]] |
| Do not require `ASPECTUS_AGENT_MODE=1` | [[norm-machine-path-is-detected]] |
| `--help --format=json` / `--list-flags` are a later axis | *not effluxed* — still [[../influx/cli-conventions/ai-agent-considerations#Help for Agents|agent · machine help]] |

## Retarget — `design/two-level.md`

| Design clause | Atom |
|---|---|
| Stdout is the picture; success is quiet on stderr | [[norm-stdout-is-data]] · [[norm-success-is-quiet]] |
| Glance before focus; context window is a design constraint | [[norm-glance-before-focus]] |
| A picture of the place, not an essay | [[claim-query-for-files]] |
| `--color=auto` follows the TTY | [[norm-color-follows-tty]] (flag spelling: [[form-shared-flags]]) |
| No prompts | [[norm-no-prompts]] |
| Explicit paths are relative to CWD; default locus is CWD | [[norm-paths-relative-to-cwd]] |
| `--` already from Help | [[form-shared-flags]] |
| Help Examples must gain `aspectus` / `aspectus PATH` | *stays in the design* (story constraint) |

## Retarget — `design/config.md`

| Design clause | Atom |
|---|---|
| Flags → env → repo file → user XDG → defaults. No `/etc` until we install that way. | [[form-config-precedence]] |
| Discovery walks up from the locus. `--config=PATH` overrides. | [[norm-config-walks-from-locus]] |
| Asked-for “what did you load?” is data → stdout | [[norm-stdout-is-data]] |
| Estate overlays stay out of baked world-law | [[norm-overlays-are-config]] |
| Secrets never as argv | [[norm-secrets-never-argv]] |

## Retarget — outline Part I Foundation column

Whole-file pointers become the atoms the design actually uses:

- **Help & Version:** [[norm-stdout-is-data]] · [[form-exit-vocabulary]] · [[form-shared-flags]] · [[norm-help-is-law-channel]] · [[norm-refusal-names-class]] · [[norm-refusal-offers-next]] · [[norm-version-line]] · [[form-help-shape]] · [[norm-no-prompts]] · [[norm-machine-path-is-detected]]
- **Two-level look:** [[norm-stdout-is-data]] · [[norm-success-is-quiet]] · [[norm-glance-before-focus]] · [[claim-query-for-files]] · [[norm-color-follows-tty]] · [[norm-no-prompts]] · [[norm-paths-relative-to-cwd]]
- **Config:** [[form-config-precedence]] · [[norm-config-walks-from-locus]] · [[norm-stdout-is-data]] · [[norm-overlays-are-config]] · [[norm-secrets-never-argv]]

Part II rows still point at influx. Same carve, heavier pages. Retarget those Tags when you want the outline to cite atoms as the girders; do not copy leftover axis into `src/` just to make every Part II link resolve here.

## Atoms in this efflux

| Atom | One line |
|---|---|
| [[norm-stdout-is-data]] | stdout is the result; teaching/progress/diagnostics on stderr |
| [[norm-success-is-quiet]] | success is silent unless teaching or protecting |
| [[form-exit-vocabulary]] | `0` / `1` / `2` / `130` — not the sysexits zoo |
| [[form-shared-flags]] | `-h`/`--help`, `--version`, `--`, `-`; verbose `-v` is not floor |
| [[norm-color-follows-tty]] | `--color=auto` follows stdout TTY |
| [[norm-help-is-law-channel]] | help teaches law before the first refusal |
| [[norm-refusal-names-class]] | name the failure class |
| [[norm-refusal-offers-next]] | error-as-menu |
| [[norm-version-line]] | name + semver; +sha when untagged |
| [[form-help-shape]] | usage, what it is, commands, options, examples |
| [[norm-no-prompts]] | never block on stdin for confirmation when not a TTY |
| [[norm-machine-path-is-detected]] | detect; do not require `*_AGENT_MODE=1` |
| [[norm-glance-before-focus]] | glance/skeleton before focus/full |
| [[claim-query-for-files]] | paths (and a picture), not answers |
| [[norm-paths-relative-to-cwd]] | explicit paths, and the default locus, are CWD-relative |
| [[form-config-precedence]] | flags → env → repo → XDG → defaults; no `/etc` until installed |
| [[norm-config-walks-from-locus]] | walk up from the path argument; `--config=PATH` overrides |
| [[norm-overlays-are-config]] | estate overlays are the repo-file layer, not a release |
| [[norm-secrets-never-argv]] | file / env / stdin; fail closed |
