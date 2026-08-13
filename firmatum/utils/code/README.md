# firmatum/utils/code

Small, installable code utilities that live in the firmatum belt (not the
research register). Each tool has its own directory; install is usually a
symlink into `~/.local/bin`.

## Utilities

| Command | Path | What it does |
|---------|------|----------------|
| **`git-heat`** | [`git-heat-decay/`](git-heat-decay/) | Commit-decay heatmap for any git repo or subdirectory — CLI ranking or interactive HTML tree + file viewer |

See each tool’s README for install and usage.

## Related (elsewhere under firmatum/utils)

| Command | Path | Notes |
|---------|------|--------|
| **`md-press`** | [`../md-press/`](../md-press/) | Markdown unwrap / canonicalize (Rust; `cargo install --path …`) |
| **`aspectus`** | [`../aspectus/`](../aspectus/) | Budgeted locus snapshot (Rust) — see `ASPECTUS.outline.md` |
