# Aspect lattice

How a **fact** may enter the one look (`aspectus` / `aspectus PATH`). Not a verb table. Verbs stay `help`, `version`, implicit **show** (and later only a new *act*: `config` already, maybe `snapshot` / `invalidate-cache`).

This grid is **very provisional**. It is **orthogonal to implementation priorities** — order of work lives on the Part I pipeline, not here. It is **not a replacement for more detailed design**: a cell is a claim that an office exists, not the story, fixtures, or law of that office. Size is the specimen. A blank or `—` means we have not claimed that office for that fact. `flag` is here so we can **refuse** own flags.

[[config|Config]] is the first arbitrator of these cells (which facts are `ON` / `OFF` / `QUIET`, which format is starred). Caller stack, not a file in the project — so a given agent’s look does not drift when the tree does.

## Offices (columns of the grid)

| Office | In the look |
|---|---|
| **column** | Where the fact *sits* on the line: `Y` = a column; `INFO` = to the right of the name (symlink target, furniture, prior name, aggregates, …). |
| **default** | In the default look: `ON` · `OFF` · `QUIET` (in the look only when it surprises). |
| **quiet** | This fact *can* appear only when it surprises (the old Quiet-columns, per fact). |
| **sort** | May be the order key. |
| **flag** | `Y` = own flag (`--size`). `COMPOSE` = only via a shared ask (`--columns`, `--sort`). `NO` = we do not want one. |
| **format** | Rendering options for this fact. `*` marks the default. `pattern` means a user/config strftime (or equivalent). |
| **dir-census** | An unexpanded directory reports this about *itself* (how many children, of what kinds). |
| **leaf-census** | Leftover *files* at a level report this (`[+47: 31 .bak]`). |
| **filter** | May *cut* the look. Dangerous for a glance; Focus is weight, not cut. |
| **weight** | May reweight the allocator (Focus, heat, maybe size). Not sort. |
| **unique** | What only this fact has — which quantity, absence, obtain cost, … |

Marks: `Y` in scope · `INFO` hangs on the name · `Y + INFO` both · `ON` / `OFF` / `QUIET` default · `COMPOSE` shared ask only · `NO` refused · `*` format default · `—` not this fact.

## Lattice

| Fact | column | default | quiet | sort | flag | format | dir-census | leaf-census | filter | weight | unique |
|---|---|---|---|---|---|---|---|---|---|---|---|
| name | Y | ON | — | Y | NO | basename* / relative / absolute | — | — | — | — | Always on. Default sort. The name itself, not INFO. Perspective root is an [[overview-invariants\|overview invariant]], not this cell. |
| size | Y | OFF | Y | Y | Y | human* / bytes / log | Y | Y | NO | Y | **Which:** file `st_size` ≠ allocated (sparse / APFS clone) ≠ recursive du. Dir-census size is a walk; furniture / gitignored bodies must not silently join it. Leaf-census is the leftover files’ `\| 2.1M`. JSON is always bytes. |
| modified (=mtime) | Y | OFF | Y | Y | Y | iso-8601* / rfc-3339 / rfc-2822 / odbc / epoch / pattern | — | — | NO | — | Quiet = recent vs siblings / now. |
| created (btime / crtime) | Y | OFF | Y | Y | Y | iso-8601* / rfc-3339 / rfc-2822 / odbc / epoch / pattern | — | — | NO | — | **Absence:** omit on Linux (and anywhere cheap getattrlist is missing). Never fake as mtime. |
| line-count | Y | ON | Y | Y | COMPOSE | physical* / non-blank | Y | Y | NO | — | Non-binary only (text, md, udon, source, …). Obtain is a read; cache by ino+mtime+size. Binary: omit, never `0`. |
| filetype (suffix) | Y | OFF | — | Y | NO | suffix* / bare | — | Y | Y | Y | Remainder census by suffix. Globify hangs here. Not MIME-from-magic. Suffix is already in the name; a type *column* is OFF until asked. |
| filekind (text / image / binary / dir / (other) file / link / …) | Y + INFO | ON | — | Y | NO | word* / slash / both | — | Y | — | — | Kind from a suffix map in config (not magic). INFO is the `/` on directories. Column is the word (`text`, `image`, …). Dirs-first lives on **sort**. |
| dir-furniture | INFO | ON | — | — | NO | facet* / first-line | — | — | Y | — | Parent state, not a column. Plugins: [[furniture/git\|Git]] / [[furniture/github\|GitHub]]. The rest is the furniture map. Filter is `--kind`, not a cut of children. |
| permissions | Y | QUIET | Y | — | NO | octal* / symbolic / octal+flags | — | — | — | — | Quiet = not usual (`644` file / `755` dir). Flags = append-only / immutable / uchg when set. |
| owner | Y | QUIET | Y | — | NO | name* / uid | — | — | — | — | Quiet = not you / not usual. |
| group | Y | OFF | Y | — | NO | name* / gid | — | — | — | — | Same as owner. |
| cloud | INFO | QUIET | Y | — | NO | evicted* / hydrated | — | — | — | — | Evicted / not hydrated. Omit where the FS has no such bit. |
| linkcount | Y | OFF | Y | — | NO | n* | — | — | — | — | Quiet = nlink > 1. |
| filesystem | — | OFF | — | — | COMPOSE | — | — | — | NO | — | Already `--no-one-fs` on [[links-and-fs\|Links]] (default is one filesystem). Not a column. |
| child-count | INFO | ON | — | Y | NO | n* / n+shown | Y | Y | — | — | How many children, shown or not. Broot’s gap. Distinct from size. Critical differentiating feature. Hangs on the directory name. |
| git letter | Y | ON | Y | — | NO | letter* / porcelain | — | — | — | — | Dirty only. Clean prints nothing. |
| initial-sha | Y | OFF | — | Y | NO | short* / H~N / full | — | — | NO | — | Introducing commit. Outside a repo: omit. Lives with [[../ASPECTUS.outline#heat\|Heat]], not a second universe. |
| latest-sha | Y | OFF | — | Y | NO | short* / full | — | — | NO | — | Last-touch sha. Same obtain family as heat. Omit outside git. |
| heat | Y | OFF | Y | Y | COMPOSE | score* / bar | Y | — | NO | Y | Commit-decay, visible set only. Weight is the office it earns first. |
| prior name | INFO | QUIET | Y | — | NO | was* | — | — | — | — | Only if recently renamed. |
| symlink target | INFO | ON | — | — | NO | target* / target+type | — | — | — | — | Plus broken. Cycle does not hang. [[links-and-fs\|Links]]. |
| denied | INFO | ON | — | — | NO | — | — | — | — | — | Could not stat. A fact about the look, not the inode. |
| last-look | INFO | OFF | Y | Y | COMPOSE | marker* / delta | — | — | NO | Y | Delta of other facts (size changed, mtime newer), not a new inode field. Identity of a look is still open (uid / ino+mtime / cache key). |

## What this is for

- **Columns**, **Sort**, and **Quiet-columns** in Part I are views of this grid, not three separate inventions.
- A new fact starts as a row with most offices `—` or `NO`. It earns cells; it does not get a flag by existing.
- `unique` is where “size is not one number” lives, so we do not invent a new office for every nuance.
- `default=QUIET` is the default *mode* of a fact that also has `quiet=Y`. `default=ON` + `quiet=Y` means it is in the default look, but still hushes when there is nothing to say (git letter).

## Open (play)

- Is `weight` an office or only unique-to-heat/focus?
- Does `--columns` / `--sort` exist as the *compose* surface, or do we name facts only in config?
- Child-count vs dir-census vs leaf-census: three names, or is child-count just the `n` of dir-census?
- Move the `*` on size (`human*` vs `bytes*`) if the text look should default to bytes. JSON is bytes either way.
