# absorb — finish note

> [!note] **History, not the binary.** This describes the retired first-snapshot code; none of it is in the current crate (`impl/README.md`). "Landed" below is past-tense of that snapshot.


*Landed. Source: `src/absorb.rs` `RULES`. Tests: `tests/absorb.rs`, `raw_is_the_only_way_into_git`.*

Static name table. First match wins. No config file. Each row is a basename, exact or `README` prefix, and one outcome: do not list (and stamp a label on the directory), list (and maybe stamp a label), drop (`.DS_Store`), or list with a tag (`(archive)`, `(trash)`).

`--show-all` / `--inspect` / `--inspect KIND` turn “do not list” names into ordinary children so the walk will enter them.

| Name | What happens |
|---|---|
| `.git` `.gitignore` `.gitmodules` `.gitattributes` `.github` | not listed; directory gets `git` |
| `target` `node_modules` `__pycache__` `.build` `.ruby-lsp` | not listed; directory gets `build` |
| `.obsidian` `.obsidian.vimrc` | not listed; `obsidian-vault` |
| `.claude` | not listed; `agents` |
| `.mise.toml` | not listed; `mise` |
| `.DS_Store` | dropped |
| `Cargo.toml` `Cargo.lock` | listed; also `rust` |
| `pyproject.toml` `Gemfile` `mise.toml` `package.json` `AGENTS.md` `CLAUDE.md` `GEMINI.md` | listed; also `python` / `ruby` / `mise` / `js` / `agents` |
| `README*` | listed; no extra label |
| `.archive` `.super-archive` `archive` | listed as `(archive)` |
| `.trash` | listed as `(trash)` |
| anything else | listed, no tag |
