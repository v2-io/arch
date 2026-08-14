# Two-level look

I run `aspectus` in a directory (or name a path) and get two levels: the place itself, and its children. No grandchildren. Then it exits.

This is the first picture. It is not yet a line budget, and it is not yet furniture. `.git` will show as a child if it is there. That is honest for this row.

## Foundations (clauses)

| Clause | Where |
|---|---|
| Stdout is the picture | [[../../../principles/src/norm-stdout-is-data\|norm-stdout-is-data]] |
| Success is quiet on stderr | [[../../../principles/src/norm-success-is-quiet\|norm-success-is-quiet]] |
| Glance before focus | [[../../../principles/src/norm-glance-before-focus\|norm-glance-before-focus]] |
| A picture of the place, not an essay | [[../../../principles/src/claim-query-for-files\|claim-query-for-files]] |
| No prompts | [[../../../principles/src/norm-no-prompts\|norm-no-prompts]] |
| Explicit paths and the default locus are CWD-relative | [[../../../principles/src/norm-paths-relative-to-cwd\|norm-paths-relative-to-cwd]] |
| `--` already from Help | [[../../../principles/src/form-shared-flags\|form-shared-flags]] |
| Help Examples must gain `aspectus` / `aspectus PATH` | stays in this design (story constraint) |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Default locus | No path → look at CWD. | In a fixture dir, `aspectus` lists that dir’s name and its children. |
| 2 | Named locus | `aspectus PATH` looks at PATH. Relative to CWD. | `aspectus sub/` from the fixture parent lists `sub/` and *its* children, not the parent’s. |
| 3 | Two levels | The locus line, then each child. A child directory is **not** expanded. | A child dir that contains files does not print those files. |
| 4 | `.` and `..` | Not listed as children. `.` is the locus itself (the root line). `..` is outside the look. | Fixture contains only `a/` and `f`; stdout has no `./` child line and no `../`. |
| 5 | Dirs marked | Directory names end in `/`. | `a/` in stdout; file `f` has no slash. |
| 6 | Hidden names | Listed. No furniture rule yet. | `.git/` appears as a child if present. |
| 7 | Order | Uses [[design/sort\|Sort]]’s default (dirs first, then name). This row does not invent sort. | Stable across runs; same in a pipe. |
| 8 | Success is quiet | stderr empty, exit 0. | `aspectus` in the fixture. |
| 9 | Missing path | Path does not exist → stderr, exit 2, class: not found. Next action is not `help` unless the path looks like a flag. | `aspectus /no/such`. |
| 10 | Install | After `cargo install --path firmatum/utils/aspectus` from `arch/`, `aspectus` on PATH works from any CWD. | Invoke the installed binary from a temp dir that is not the crate. |
| 11 | Help examples | Add to help in the same commit: `aspectus` and `aspectus PATH`. | Help snapshot updated. |

## Picture (this row)

```
fixture/
├── a/
├── .hidden
└── f
```

No sizes, no dates, no git letters. Those wait.

## Not in this row

Nth-level tree, Summarization, Balanced summarization, furniture, visit cap, JSON, [[color\|Color]].
