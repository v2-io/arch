# Color

`--color=auto` (default) colors only when stdout is a TTY. `--color=never` never colors. `--color=always` colors even in a pipe. A pipe under `auto` has no CSI.

This is how **any** look is painted — two-level, n-level, later columns — not a property of two-level. Two-level is a complete picture in plain text without this row.

## Foundations

| Clause | Where |
|---|---|
| `--color=auto` follows stdout TTY | [[../../../principles/src/norm-color-follows-tty\|norm-color-follows-tty]] · [[../../../principles/src/form-shared-flags\|form-shared-flags]] |
| TTY changes presentation, not which stream | [[../../../principles/src/norm-stdout-is-data\|norm-stdout-is-data]] |

What gets which color (dirs vs files vs furniture) waits on this row’s design when we implement it. Do not ship the flag until it colors.
