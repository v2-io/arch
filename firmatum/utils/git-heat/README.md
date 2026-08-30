# firmatum/utils/git-heat

Commit-decay heatmap for any git repo or subdirectory — CLI ranking or interactive HTML tree + file viewer. Non-git trees rank by filesystem mtime (no half-life).

The executable lives in [`git-heat-decay/`](git-heat-decay/). See that README for the heat model and usage.

## Install

```sh
ln -sfn ~/src/arch/firmatum/utils/git-heat/git-heat-decay/git-heat ~/.local/bin/git-heat
# ensure ~/.local/bin is on PATH
```

## Related (elsewhere under firmatum/utils)

| Command | Path | Notes |
|---------|------|--------|
| **`md-press`** | [`../md-press/`](../md-press/) | Markdown unwrap / canonicalize (Rust; `cargo install --path …`) |
| **`aspectus`** | [`../aspectus/`](../aspectus/) | Budgeted locus snapshot (Rust) |
| **`repo-heat`** | [`../repo-heat/`](../repo-heat/) | Per-repo commits/day EMA (calendar half-life; ranks repos against each other) |
