# config — finish note

*Landed. Source: `src/config.rs`. Tests: `tests/config_show.rs` (real binary, isolated `XDG_CONFIG_HOME`).*

Stack: defaults < global (`/etc/aspectus/aspectus.toml`) < user-home (`$XDG_CONFIG_HOME/aspectus/aspectus.toml`) < agent-type (`caller-<key>.toml` when `--caller` is set) < env (`ASPECTUS_LINES`) < flags (`--lines`). `--config=PATH` substitutes for user-home. No file is read from the locus.

`aspectus config` prints layers (consulted / existed / won) on stdout, exit 0. Missing layers are absent, not an error.

Constants (one each, not a zoo): `USER_HOME_FILENAME`, `GLOBAL_PATH`, `CALLER_FLAG`.
