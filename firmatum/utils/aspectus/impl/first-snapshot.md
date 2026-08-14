# first-snapshot — finish note

*Landed. Source: `src/main.rs`, `Cargo.toml`. Test: `walk_render_names_src_not_target_as_child`. Also ran the binary on md-press, this crate, and `~/src/arch`.*

`aspectus [PATH]` walks, applies the table, shares lines, prints text, exits 0.

Flags: `--lines N` (80), `--visit N` (400), `--explain-budget`, `--raw`, `--inspect [KIND]`, `-x` / `--no-one-fs`, `--help`, `--version`. Stdout is the picture; diagnostics and `--explain-budget` go to stderr. Usage/IO errors exit 2. Help names `--raw` / `--inspect` as DANGEROUS.

Live checks: md-press shows `src/`, not `target/` as a child; `~/src/arch` does not dump `.git` internals.

**Install.** `cargo install --path firmatum/utils/aspectus` from `arch/`. `publish = false`. No installer script.

**Version.** `0.1.0` in `Cargo.toml`. `--version` prints `aspectus 0.1.0`. No bump tool, no crate changelog, no release tags.

**Config.** None. No file, no `ASPECTUS_*` env, no `--config`.
