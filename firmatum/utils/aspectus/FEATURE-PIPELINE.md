# Feature pipeline

> **Seed, not tracker.** The living pipeline is [`ASPECTUS.outline.md`](ASPECTUS.outline.md) Part I. This file is influx for `design/` segments. Do not add new tracking here. When a feature row is drafted, the corresponding section here is disposable (delete-test).

*Original design record: [`IMPLEMENTATION-NOTES.md`](IMPLEMENTATION-NOTES.md). Live effort: [`PRACTICA.md`](PRACTICA.md).*

Status marks: **decided** (Joseph, this founding conversation, 2026-08-13) · **proposed** (agent, unratified) · **open**.

A feature does not jump the pipeline because it is interesting. Annotations hang on a node. Collapse operators change what a node *is*. The allocator is the invention. Features that need the allocator and the absorb-partition are blocked on those, not on each other.

---

## P0 — named and holdable

| Feature | Status | Notes |
|---|---|---|
| Name: faculty `aspectus`, snapshot `aspecta` | decided | Locus-aspectus in PROPRIUM talk; binary is the faculty |
| Print-and-quit; no TUI | decided | broot’s original sin for this job |
| Rust crate in `firmatum/utils/aspectus/` | decided | Same belt as md-press; extract later if it earns a repo |
| These three navigator files | decided | PRACTICA is the DAG |

---

## P1 — the thing without which it is not the tool

Without this stage, extra columns are just a worse `eza -T`.

| Feature | Status | What it is |
|---|---|---|
| Line budget (`--lines N`) as the first-class constraint | proposed (default 80–120 for agents) | Depth is a consequence, not the knob |
| Sibling-sharing allocator | proposed | Each child gets a share; interesting children more; remainder is a typed summary. Opposite of broot height-fill BFS and of `tree -L` |
| Absorb / witness / omit / child partition | decided | Well-known names are parent *state*, not children. Only unmapped hidden names remain hidden children |
| Parent facets (`git:`, `rust:`, `mise:`, …) + `kind` labels | decided (shape); mappings proposed | See absorb table in IMPLEMENTATION-NOTES |
| Collapsed-dir census by suffix/type | decided (intent) | `[128: 90 .rs, 20 .md, 18 other \| 2.1M]` — never a bare `…` or `N unlisted` |
| Leaf aggregates the same way | decided (intent) | `[+47: 31 .bak, 12 .log, 4 other]` |
| Dirs marked `/`; git-repo mark on the directory, not a 28-byte gitlink in the file list | decided | |
| Symlink targets; recurse with `dev+ino` cycle guard | decided (intent) | |
| Important-name boost (witness files) | proposed list | `README*`, `AGENTS.md`, `CLAUDE.md`, `Cargo.toml`, `package.json`, … |
| `--dotfiles-first` (implies dirs-first) | proposed | Only the *unabsorbed* hidden names |
| `--raw` / `--inspect <kind>` escape | proposed | The only path to `.git/objects`. Default view must not be able to stumble there |
| Text render of an `Aspecta` | proposed | Sparse; columns only for surprise |
| `--explain-budget` | proposed | Why each node got lines. We will need it while building |

**Not in P1:** walking `.git/objects` to produce a census. Absorb means do not enter.

---

## P2 — quiet metadata, local git facet, config

| Feature | Status | What it is |
|---|---|---|
| Layered config (tool defaults + user + repo) | proposed | Adding `.fmt-mdignore → md-press` is a config line, not a release |
| Size on dirs; on files when large vs siblings | proposed | |
| mtime when recent or asked | proposed | |
| Mode only if not usual (`644` file / `755` dir) | proposed | |
| Git letter only if dirty | proposed | |
| `git:` facet from local data | proposed | short remote, branch, porcelain (`M3 ?2`), short HEAD. **Not** private/public in P2 |
| `mise:` pins when `mise.toml` / `.mise.toml` present | proposed | Local, cheap |
| Role dirs stay children | decided | `.archive/`, `.super-archive/`, `.trash/` — tagged, not absorbed |
| Kinds are claims | proposed | A container with only `README.md` does not grow an empty `[kind:]` |
| `-x` / one-filesystem default | proposed | Do not follow mounts |
| JSON emit of the same `Aspecta` | proposed | Agents parse this. Text is a view |

---

## P3 — collapse that looks like recognition

| Feature | Status | What it is |
|---|---|---|
| Suffix buckets (already in P1 census) | — | 80% of globification |
| Sequence globification | proposed | `output-[001-047].bak (47)` |
| Line counts for text files | proposed | Optional; cache by ino+mtime+size |
| Birthtime (macOS getattrlist) | proposed | |
| Honor gitignore for file *bodies*; still show ignored-dir *presence* as collapsed/absorbed | proposed | `target/` does not vanish from reality; it becomes rust-build state |

---

## P4 — focus is a weight, not a filter

| Feature | Status | What it is |
|---|---|---|
| `--focus PATH` | proposed | Extra budget on that subtree; parents and siblings stay as locating summaries |
| Stdin paths (`rg -l … \| aspectus`) | proposed | Matches highlighted in place; non-matches become `...` slots |
| `--rg PATTERN` / `--glob` | proposed | Same IR. Do not drop surroundings |

Erlang analog: keep the tuple, write `...` in the boring slots. `tree \| rg` throws the tree away. broot-filter throws the non-matches away.

---

## P5 — git archaeology (optional, cached)

Do not `git log` per file. Reuse [`../code/git-heat-decay/`](../code/git-heat-decay/) or its model.

| Feature | Status | What it is |
|---|---|---|
| Heat (commit-decay, not wall-clock) | proposed | `raw = Σ exp(-age/τ)` as in git-heat |
| Introducing commit as `H~N` | decided as wanted | Different fact from last-touch |
| Last-touch SHA | proposed | Cheaper than introducing-commit |
| Prior name if recently renamed | proposed | Only on the visible/focused set |

---

## P6 — other heads, cache, later kinds

| Feature | Status | What it is |
|---|---|---|
| UDON emit of an `Aspecta` | proposed | After the IR is stable. JSON is not gated on this |
| Cache for derived metrics | proposed | Line counts, heat. Not the walk itself, first |
| `(private)` on remotes | open | Network or `gh` cache. Do not guess from `git@` vs https |
| Estate overlay mappings | open | `.orient`, `.fmt-mdignore`, … — user/repo config, not baked-in world-law |
| `--kind rust` filter | proposed | Kinds exist to be queried once they exist |

---

## Explicitly not a feature

- A TUI, a `br` replacement, interactive filtering.
- Being `ls`.
- Expanding infrastructure dirs in the default view “just in case.”
- A flag museum for every column. Quiet text + complete `Aspecta` is the pair.
