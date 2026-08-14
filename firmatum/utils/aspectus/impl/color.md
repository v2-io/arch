# color — finish note

*Landed. Source: `src/color.rs`. Tests: `tests/color.rs` (real binary).*

`--color=auto` (default) colors only when stdout is a TTY. `--color=always` / `--color=never`. Piped `auto` has no CSI. Directories are bold blue. Files are plain. A bad value (`--color=purple`) is usage, not a silent swallow.
