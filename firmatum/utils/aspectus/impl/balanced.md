# balanced / leaf-census — finish note

*Landed. Source: `src/n_level.rs` `apply_budget`. Tests: `tests/balanced.rs`.*

`--lines N` (config `lines`, default 80, `0` unlimited). Siblings share; omitted siblings print as `[+N: …]`. `--explain-budget` on stderr.
