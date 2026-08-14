# help-verb — finish note

*Landed. Source: `src/main.rs` (help page, parse, refusals). Tests: `tests/help_verb.rs` (real binary).*

`help` / `-h` / `--help` → stdout, exit 0. `version` / `-v` / `--version` → one line `aspectus <semver>` or `aspectus <semver>+<sha>` (SHA from `build.rs`, not a runtime git call). Unknown option and unknown verb are distinct classes on stderr, exit 2, next action `aspectus help`. `--` ends flags.

Help is generated from the same verb/flag list the parser accepts. The page leads with the version line and the defs (faculty, locus, aspecta), not a two-line slogan. Examples include `aspectus` and `aspectus PATH`.
