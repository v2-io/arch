# furniture — finish note

*Landed. Source: `src/furniture.rs` (map, glob matcher, config parsing), wired in `src/n_level.rs::gather_at`. Tests: `tests/furniture.rs`.*

The map is glob → kinds + fate (`hide` / `omit`; `mark` rows are the Labels feed, see `labels.md`). First match wins; a trailing `/` restricts a pattern to directories; `*` and `?` only — no regex engine. Config key `furniture` on the caller stack (`ASPECTUS_FURNITURE` env too): `PATTERN[:KINDS[:FATE]]`, comma-separated, kinds `+`-separated; config rules match before the defaults; `!PATTERN` drops a default row.

Partition happens right after `enumerate()`, before any census or stat — so hidden names never join a census silently (the kind spot is what says they are here), never spend walk budget, and the mechanism works identically at the depth cutoff. `--explain-budget` confesses the total hidden count on stderr.

`--show-all` neutralizes every hide/omit; `--inspect KIND` (repeatable) neutralizes one kind's rows; either way the walk enters what it lists, and the kind claims stay printed (they are still true).

**The `build` vs `rust` question (impl/parent-state-not-children.md) — decided:** `target/` stays `build`. The name alone claims build output, not an ecosystem — maven writes `target/` too, and a label is a claim, not a guess. Where the name *is* an unambiguous ecosystem claim the rule claims both: `__pycache__/` → `build`+`python`, `node_modules/` → `build`+`js`, `.ruby-lsp/` → `build`+`ruby`. `rust` arrives with `Cargo.toml` (a mark row).

Also decided (was open in the outline): `.github` is its own kind (`github`), not folded into `git` — it has its own plugin and its own facts.

## The shipped map (verified against `src/furniture.rs::default_rules`, 2026-08-15)

First match wins; config rules are prepended. Trailing `/` = dirs only.

| pattern | kinds | fate |
|---|---|---|
| `.git` | git | hide (plugin: `src/git.rs`) |
| `.github` | github | hide (plugin: `src/github.rs`) |
| `.gitignore` · `.gitmodules` · `.gitattributes` | gitignore · gitmodules · gitattributes (each its own word, never `git`) | hide |
| `target/` | build | hide |
| `node_modules/` | build+js | hide |
| `__pycache__/` · `*.egg-info/` · `.pytest_cache/` · `.mypy_cache/` · `.ruff_cache/` · `.tox/` | build+python | hide |
| `.build/` | build | hide |
| `.ruby-lsp/` | build+ruby | hide |
| `.obsidian/` · `.obsidian.vimrc` | obsidian-vault | hide |
| `.claude/` | agents | hide |
| `.mise.toml` | mise | hide |
| `.archive/` | archive | hide |
| `.trash/` | trash | hide |
| `.DS_Store` | — | omit |
| `Cargo.toml` · `Cargo.lock` | rust | mark |
| `pyproject.toml` | python | mark |
| `Gemfile` | ruby | mark |
| `mise.toml` | mise | mark |
| `package.json` | js | mark |
| `AGENTS.md` · `CLAUDE.md` · `GEMINI.md` | agents | mark |

**Subsumption (in code, not in the map — `read_names`):** when a level claims `git` (a real `.git`), the words `gitignore` / `gitmodules` / `gitattributes` are dropped from that level's `has:` — a repo root says `[has: git, …]`, not `[has: git, gitignore]`. Hidden *dirs* whose kind does not speak for itself (`speaks_for_itself`: git/github excluded) get the readdir-only file count folded onto the kind word (`archive ≈127f`).

**Sibling keys, same grammar family:** `kinds = "SUFFIX:text|binary, !SUFFIX"` (`src/kind.rs`, feeds line counts and the kind word) and `important = "GLOB, …, !GLOB"` (`src/important.rs`).

**Not in the mechanism (as of 0.1.8):** kind identity beyond a word (no glyph/tag/priority — `has:` sorts alphabetically); no split between kind-of-place (rust, python, vault) and furniture-class (build, archive, trash); no `--caller`-keyed default map (the agent-caller "don't hide `.claude`" ask stays open); no plugin registry beyond the two wired kinds; a rule cannot say what it counts as for mass beyond the hidden-dir file count.

**Alignment (steward decision, arrived mid-landing):** all non-name material — censuses, facets, the kind spot, look-marks — lands at a computed pseudo-tab-stop per `design/columns.md`: max name column in the look + 2, capped at 48 (`n_level.rs` `STOP_CAP`), a pure function of tree content, never terminal width; an over-cap name goes ragged on its own line only. The machinery (`Row` / `paint`, since moved to `src/columns.rs`) is minimal on purpose — the Columns row owns the full mechanism when it lands and should absorb this. *(It did — impl/columns.md.)*
