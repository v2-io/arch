# Overview invariants

Every look states the facts that locate it, on the header, before the children.

1. **Perspective root** — the absolute path of the locus, not `./`, not only the basename. `aspectus` and `aspectus PATH` both print the same kind of root: `std::path::absolute` of the path that was looked at (logical absolute, not necessarily `realpath`).
2. **Time of the look** — the system datetime when this aspecta was made. Default format is ISO-8601 / RFC-3339 UTC (`YYYY-MM-DDTHH:MM:SSZ`). Other formats wait on the lattice `format` office.

The header is one line: absolute path (directory `/`, painted if Color is on), then the stamp.

```
/Users/joseph/src/arch/firmatum/utils/aspectus/  2026-08-14T06:12:03Z
├── design/
└── src/
```

No `./` child. The stamp is when *this* print ran, not mtime of the directory.

## Foundations

| Clause | Where |
|---|---|
| Stdout is the picture | [[../../../principles/src/norm-stdout-is-data\|norm-stdout-is-data]] |
| Success is quiet | [[../../../principles/src/norm-success-is-quiet\|norm-success-is-quiet]] |
| Paths named on the command line are CWD-relative; the *printed* root is absolute | [[../../../principles/src/norm-paths-relative-to-cwd\|norm-paths-relative-to-cwd]] (invoke) |

## Not in this row

Canonicalize / realpath. Local timezone offset. Extra header fields (uid, hostname) until someone names them.
