# two-level — finish note

*Landed. Source: `src/two_level.rs`. Tests: `tests/two_level.rs` (real binary).*

Default `aspectus` / `aspectus PATH` prints the place and its immediate children. No grandchildren. `.` / `..` are not children. Dirs marked `/`. Hidden names, including `.git/`, are listed. Order: directories first, then name. Success is quiet. Missing path: class `not found`, exit 2, not a help menu.

This picture does **not** hide furniture. `--show-all` / `--inspect` / `--explain-budget` still use the older deep walk.

Help Examples now include `aspectus` and `aspectus PATH`.
