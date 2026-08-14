# balanced / leaf-census — finish note

*Landed. Source: `src/n_level.rs` `apply_budget`. Tests: `tests/balanced.rs`.*

`--lines N` (config `lines`, default 80, `0` unlimited). Siblings share. A dir that only has its own line gets a dir-census on that line, not a child `[+]`. `[+N: …]` is leftover siblings after some were listed. `--explain-budget` on stderr.
