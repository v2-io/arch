# two-level — finish note

*Landed. Source: `src/two_level.rs`. Tests: `tests/two_level.rs` (real binary).*

Default `aspectus` / `aspectus PATH` prints the place and its immediate children. No grandchildren. `.` / `..` are not children. Dirs marked `/`. Hidden names, including `.git/`, are listed. Order: directories first, then name. Success is quiet. Missing path: class `not found`, exit 2, not a help menu.

This is the only look the binary prints. The first-snapshot walk/absorb/budget code is not in the crate.

Help Examples now include `aspectus` and `aspectus PATH`.
