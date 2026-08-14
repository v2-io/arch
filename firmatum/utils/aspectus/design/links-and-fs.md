# Links and one filesystem

A symlink shows its target, and a symlinked **directory** is followed and recursed into for depth — the look treats the linked place as part of this place, because for the reader it is. (Origin, verbatim: *"shows symbolic link targets and still recurse into it for depth"*; and among broot's named failures: *"doesn't follow symlinked directories"*, *"doesn't show size of symlinked files"*. The old pipeline row text — targets shown, no recursion promise — under-asked; this design restores the origin.)

Three duties:

1. **Show the target.** `link -> ../real/place` (INFO office, per the lattice's symlink-target row: default ON, `target*` format). A broken link says so — never rendered as a plain name.
2. **Recurse.** A symlinked directory expands like a real one, spending depth and budget the same way. Facts on a symlinked *file* (size, lines) are the target's facts; the link mark plus target string is what says how it got here.
3. **Never hang.** A cycle (link back to an ancestor, or two links to the same place) is caught by a visited set of `(st_dev, st_ino)` of directories already being expanded on this path. The second encounter prints the target and a cycle mark instead of recursing — same honesty family as [[denied|Denied]]: a fact about the look, never a silent stop and never a hang.

## One filesystem

The walk stays on the starting filesystem by default; `--no-one-fs` follows mounts. There is no flag to turn the default *on* (origin: the `-x` "turn on what's always on" flag was refused). A mount point under the default is not silently pruned — its line shows with a mark (spelling open) so the reader knows a filesystem boundary, not an empty directory, is what stopped the walk. Summarization's law: never a silent cut.

## Foundations (clauses)

| Clause | Where |
|---|---|
| Unbounded walks take explicit bounds; loops must not hang | [[../../../principles/influx/cli-conventions/performance-and-resources\|Resources]] |
| A stop the walk chose is stated, never silent | [[summarization\|Summarization]] |
| The mark hangs on the name (INFO), same office as denied | [[denied\|Denied]] · [[aspect-lattice\|Aspect lattice]] (symlink-target row) |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Target shown | A symlink prints `name -> target` (target as recorded, not resolved). | Fixture with relative and absolute links; both render verbatim. |
| 2 | Broken link | Target missing → the line says so (`-> target [broken]` or similar), exit still 0. | Dangling link fixture. |
| 3 | Dir recursion | A symlinked dir's children print, spending depth from the link's position. | Link to a sibling dir; its children appear under the link line at depth. |
| 4 | File facts | A symlinked file carries the target's size/line facts once those columns exist. | Link to a known file; size matches the target's. |
| 5 | Cycle | A link to an ancestor prints target + cycle mark, does not recurse, exits 0 in bounded time. | `a/loop -> ../..`; run completes; mark present. |
| 6 | Diamond | Two links to the same dir: no hang; behavior per the Open question below, but deterministic. | Two links to one target dir; byte-identical across runs. |
| 7 | One-fs default | A mount point is not descended; its line carries the mark. | Needs a mount fixture (macOS: a mounted dmg or `/Volumes` case); may be a manual/dev-only test. |
| 8 | `--no-one-fs` | Mounts are followed; the mark disappears. | Same fixture with the flag. |
| 9 | Census honesty | An unexpanded symlinked dir still gets a dir census like a real dir. | Link at the depth cutoff shows `[N: …]`. |

## Open

- **Cycle-mark and mount-mark spelling** (`[cycle -> path]`? `[other fs]`?) — Joseph ratifies; one constant each.
- **Diamond (non-cycle revisit):** two links reaching the same directory on *different* paths. Expanding both is honest but can double-spend budget and double-count [[mass|Mass]]; expanding the first and marking the second (`[seen above]`?) is cheap but asymmetric. Leaning: expand both in the look, but mass/deep aggregates count each `(dev,ino)` once. Not decided.
- **Symlink whose target is on another filesystem:** does the one-fs default stop it? The default's purpose (don't wander into `/Volumes`, network mounts) argues yes for *recursion*, target string still shown. Not decided; whichever way, marked, never silent.
- Whether `--no-one-fs` rides the caller stack like other look-shape settings (leaning yes, like `depth`/`lines`/`walk`).

## Not in this row

Hard-link `linkcount` (lattice, OFF). Furniture fates of well-known linked names. What mass counts (that row holds the count-once rule once ratified here).
