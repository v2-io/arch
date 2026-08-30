# git-heat

Walkable **commit-decay heatmap** for any git repository or subdirectory. If the path is **not** a git work tree, ranks by **filesystem mtime** instead (no half-life, no blame).

Each path gets a heat score from how recently (in *commits*, not wall time) it
was touched. Interactive HTML shows a collapsible tree, a focus slider
(historical ↔ recent), a syntax-highlighted file viewer, and a left **git-blame
age gutter** (hot = line last changed recently; hover for SHA / author /
summary). Interactive viewer: `git-heat --serve` (HTML held in memory; no file
unless you also pass `--html`). Blame API: `/__git-heat__/blame`.

## Install

```sh
# one-time (or after moves):
ln -sfn ~/src/arch/firmatum/utils/git-heat/git-heat-decay/git-heat ~/.local/bin/git-heat
# ensure ~/.local/bin is on PATH
```

Requires: Python 3.10+. `git` on PATH for the commit-decay path (mtime ranking does not need it). HTML viewer uses highlight.js from CDN.

## Usage

```sh
git-heat --help

git-heat                              # CLI top paths for cwd's repo
git-heat --serve                      # heatmap from memory (no file write)
git-heat --html                       # write git-heat.html only
git-heat --html --serve               # write file AND serve
git-heat ~/src/arch/asf --serve       # whole repository
git-heat crates/codegen --serve       # only that subdirectory
git-heat --half-life 3 --top 25 .     # short memory, ranking only
git-heat --noise Cargo.toml,Cargo.lock
git-heat /tmp/scratch                 # not a git repo → mtime ranking
```

`path` may be any directory (or file) inside a git work tree. Heat is computed
from the whole history, then **scoped** to that subdirectory; paths in the
output are relative to the scope so the file viewer can fetch them when the
HTML is served from that directory.

If `path` is **not** inside a git work tree, git-heat walks regular files
(skipping hidden names and build/cache dirs), ranks **newest mtime first**,
and prints ages. `--half-life` is ignored. HTML `--serve` still works (slider
and blame hidden; color is recency across the tree, newest = 2).

## Heat model (brief)

- Age unit: commits behind HEAD (HEAD touch = 0). Initial commit excluded.
- `raw = Σ exp(-age / τ)`, `τ = half_life / ln(2)`
- Scale-invariant: `heat = 2 · (1 − e^{−1/τ}) · raw`  
  → touched every commit forever → **~2** at any half-life
- Directory heat = max of non-noise leaves
- Noise basenames (`Cargo.toml`, `SOURCE_REV` by default): heat 0, ignored for
  parent max (override/extra with `--noise`)

HTML focus slider: left = longer half-life (smoother / historical), right =
shorter (hottest recent). Slider magnitude expansion (more log steps) is a
planned follow-up.

## Layout

```
git-heat-decay/
  git-heat      # executable (python3)
  README.md     # this file
```

Parent index: [`../README.md`](../README.md).
