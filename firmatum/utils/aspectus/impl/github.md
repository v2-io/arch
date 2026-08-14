# github — finish note

*Landed. Source: `src/github.rs`. Tests: `tests/github_furniture.rs`.*

`.github/` is not a child. With workflows (`*.yml` / `*.yaml` under `.github/workflows/`): facet `[github: N workflows]`. Without: the `github` kind alone carries it. No network, no fetch, no `(private)`. `--inspect github` lists it (the walk then enters it). Decided here: `.github` is its own kind, not `git` (the outline's open question).
