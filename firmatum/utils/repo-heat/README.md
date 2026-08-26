# repo-heat

Per-**repository** commits/day heatmap — exponentially weighted, wall-clock
half-life (default **7 days**).

Sibling of [`../git-heat/`](../git-heat/), which scores *paths inside one
repo* by commits-behind-HEAD. This one ranks *repos against each other*.

## Install

```sh
ln -sfn ~/src/arch/firmatum/utils/repo-heat/repo-heat ~/.local/bin/repo-heat
# ensure ~/.local/bin is on PATH
```

Requires: Python 3.10+, `git`, `gh` (authenticated for private v2-io / josephwecker repos).

## Usage

```sh
repo-heat
repo-heat --half-life 3 --top 20
repo-heat --format json
repo-heat --zeros                  # include the cold catalog tail
repo-heat --forks --archived       # otherwise omitted
repo-heat --local ~/src/AISI-responses
repo-heat --self-test
```

Catalog is `gh repo list` for `--owners` (default `josephwecker,v2-io`), plus
local-only trees: `--local PATH` and, by default, top-level no-remote git
repos under `--src` (`~/src/AISI-responses`, `~/src/vestigia`, …).

Local clones are found by walking `--src` (skips `_ref`, `MOVED`, and other
archive-ish names unless `--deep`). Multiple worktrees sharing a
git-common-dir are one object store. Unpushed work on local branches counts;
a fetch of a fork's upstream does not (`git log --branches`).

## Heat model

```
κ     = ln(2) / half_life_days
score = κ · Σ  2^(-age_days / half_life)     over unique SHAs
```

Units: **commits/day**. A steady 1 commit/day forever converges to 1.0. A
commit one half-life ago contributes half of one right now.

Commits older than `12 × half-life` (override with `--window DAYS`) are
dropped: at 12 half-lives the weight is 2⁻¹² ≈ 0.00024.

`LAST` dates prefixed with `~` are GitHub `pushedAt`, not a commit timestamp.

Cache: `~/.cache/repo-heat/` (catalog TTL 1h; GitHub commit pages keyed by
nwo+pushedAt). `--refresh` / `--clear-cache`.

Parent index: [`../../README.md`](../../README.md).
