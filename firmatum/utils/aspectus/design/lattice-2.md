# Aspect Lattice Refactored
| Column | Values | Desc   |
| -------------------- | ---------------- | -------------------------------------------- |
| **stat** | ␠ | Provisional  |
| | ✓ | Settled Design & Implemented to Spec |
| | ↬ |           Settled Design & Needs Implementation/Reimplementation/Refactor |
| | ⇥ | Provisionally Designed & Currently Deferred|
| | | |
| **fact** | ‹slug› |          The fact about that row / file |
| | | |
|                     **derived-from** | ‹slugs-and-desc› | Ideally other columns, but also underlying system commands/utilities, and special remarks |
| | census-agg(...)| A special census aggregator with variable buckets on the argument(s) |
| | |           |
| **default position** | far-left | Multiple stackable columns to the left of the hierarchy and filenames - usually known fixed width |
|          | level-location | Variable column where the hierarchy is indented and lines are potentially drawn  |
| | name-location| Where the file/directory basename and suffixes are written - usually shouldn't be truncated |
|               | name-suffix| Decoration appended to the name-location text |
| | after-name | Cell that is to the right of the name+suffix, or wrapped to its own line below the name |
| | near-right                               | Fixed columns (usually with headers) aligned (independent of indent) to the right of the name/after-name |
| | supplement | Additional info cell between near and far-right columns or under columns on next line if it doesn't fit|
| | far-right|                                                                                Fixed columns (usually with headers) that after a pseudo-flex after near-right, align to the right of the display |
| | | |
| **default-display**| always | A fact that is *always* displayed / turned on |
| | on | On (at default position) by default |
| | off | Off by default  |
| | | |
| **sort** | (pos) | Put in this position relative to its siblings |
| | ‹fact-shorthand› | Common name for that thing in that sort value. Note several facts can end up being sorted together (e.g. glob & name) |
| | ‹fact-shorthand› | Common name for that thing in that sort value. Note several facts can end up being sorted together (e.g. glob & name) |
| | — | Not considered for sorting |
| | | |
| **width** | ‹val› | Might not be useful, but calculation of the width in text columns |
| **formats or examples** | ‹format1›\* / ‹format2› / ... | Default format name/label (marked with \*, and other formats available. (Probably described below later) |
|  | `lit` / ... | Exact literal or example literal for the value |
|  | {abc} | Set of unicode glyphs for single column or repeating (like permissions) |

| Column               | Values           | Desc                                         |
| -------------------- | ---------------- | -------------------------------------------- |
| **stat**             | ␠                | Provisional                                  |
|                      | ✓                | Settled Design & Implemented to Spec         |
|                      | ↬                | Settled Design & Needs Implementation/Reimplementation/Refactor |
|                      | ⇥                | Provisionally Designed & Currently Deferred  |
| | | |
| **fact**             | ‹slug›           | The fact about that row / file               |
| | | |
| **derived-from**     | ‹slugs-and-desc› | Ideally other columns, but also underlying system commands/utilities, and special remarks |
|                      | census-agg(...)  | A special census aggregator with variable buckets on the argument(s)                      |
| | | |
| **default position** | far-left         | Multiple stackable columns to the left of the hierarchy and filenames - usually known fixed width |
|                      | level-location   | Variable column where the hierarchy is indented and lines are potentially drawn                   |
|                      | name-location    | Where the file/directory basename and suffixes are written - usually shouldn't be truncated       |
|                      | name-suffix      | Decoration appended to the name-location text |
|                      | after-name       | Cell that is to the right of the name+suffix, or wrapped to its own line below the name                  |
|                      | near-right       | Fixed columns (usually with headers) aligned (independent of indent) to the right of the name/after-name |
|                      | supplement       | Additional info cell between near and far-right columns or under columns on next line if it doesn't fit  |
|                      | far-right        | Fixed columns (usually with headers) that after a pseudo-flex after near-right, align to the right of the display |
| | | |
| **default-display**  | always           | A fact that is *always* displayed / turned on |
|                      | on               | On (at default position) by default           |
|                      | off              | Off by default                                |
| | | |
| **sort**             | (pos)            | Put in this position relative to its siblings |
|                      | ‹fact-shorthand› | Common name for that thing in that sort value. Note several facts can end up being sorted together (e.g. glob & name) |
|                      | ‹fact-shorthand› | Common name for that thing in that sort value. Note several facts can end up being sorted together (e.g. glob & name) |
|                      | —                | Not considered for sorting |
| | | |
| **width**               | ‹val› | Might not be useful, but calculation of the width in text columns             |
| **formats or examples** | ‹format1›\* / ‹format2› / ... | Default format name/label (marked with \*, and other formats available. (Probably described below later) |
|                         | `lit` / ... | Exact literal or example literal for the value                          |
|                         | {abc}       | Set of unicode glyphs for single column or repeating (like permissions) |

## Initial

| stat | fact               | derived-from                          | default position             | default<br>display | sort      | width                   | formats or example                          |
| ---- | ------------------ | ------------------------------------- | ---------------------------- | ------------------ | --------- | ----------------------- | -------------------------------- |
| ✓    | level-decorator    | depth-in-tree                         | level-location\*             | on                 | —         | 4×depth                 | box-light\* / ascii / indent     |
| ✓    | filename           | readdir                               | name-location\*              | always             | name      | var                     | basename\* / relative / absolute |
| ↬    | leaf-census        | census-agg(parent's unlisted children)| name-location\*              | on                 | (last)    | (var (≥1 cc per bucket) | `+`+census-style                 |
| ✓    | directory-glyph    | stat = dir                            | name-suffix                  | on                 | —         | 1                       | `/`                              |
|      | directory-stat     | (various)                             | far-left                     | on                 | —         | 2                       | { H}{ S~}  (hidden, (normal, system, or home)) |
|      | fifo-glyph         | stat = named-pipe-or-fifo             | name-suffix                  | on                 | —         | 1                       | `\|`                              |
|      | block-special      | stat =                                | name-suffix                  | on                 | —         | 1                       | ?                              |
|      | character-special  | stat =                                | name-suffix                  | on                 | —         | 1                       | ?                              |
|      | socket             | stat =                                | name-suffix                  | on                 | —         | 1                       | ?                              |
| ↬    | symlink-target     | lstat + readlink                      | after-name                   | on                 | —         | var                     | `→ `+target\* / `→ `+target+`[broken]`     |
| ↬    | rename-from-target | git-status                            | after-name                   | on                 | —         | var                     | `← `+short-target |
| ↬    | git-status         | git-status                            | far-left                     | on                 | git       | 1                       | {⊘ M A ⁇ R U D C T} |

## Special Summaries

| stat | fact              | derived-from                            | default position | default<br>display | sort      | width     | formats                          |
| ---- | ----------------- | --------------------------------------- | ---------------- | ------------------ | --------- | --------- | -------------------------------- |
| ✓    | glob-template     | names at the level (a real series)      | name-location\*  | off                | (as name) | var       | range\* (`out-[001-047].bak`)    |
| ↬    | glob-count        | glob-template (members)                 | after-name       | off                | —         | cc        | count cell (`● 44. 𝓃`)          |
| ⇥    | focus-match       | the focus set ([[focus\|Focus]])        | far-left         | on when focused    | (first)   | 1         | mark (spelling open)             |



## Derived information


**FileType**

- Raw filetype: *Directory*
(NOT type (for our purposes): symlink target, cloud hydration status, permission / access, empty, git-ignored, owner, etc.) 
- [(special)] =>
    - [project].  (We need to survey other kinds of "top-level" project conventions such as VisualStudio, etc.)
        - Specialized "Project"  Supplement area
        - Automatic focus (if there isn't one)
    - [furniture] (map) (can be used by project heuristic code
    - 


| stat | fact              | derived-from                                                                             | default position                                                                          | default display | sort      | width                   | formats                          |
| ---- | ----------------- | ---------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- | --------------- | --------- | ----------------------- | -------------------------------- |
| ↬    | filetype          | stat type → magic bytes → shebang → **suffix as tie-breaker** → null-byte sniff; `major/minor` (+ trait) — [[filetype\|Filetype]] | (consumed by line-count, census buckets, kind-word, name-suffix; not rendered itself) | on | type | — | `text/markdown` · `image/svg` · `exe/script` · `dir` · `link` · `special/fifo` … |
| ↬    | readme-title      | head-peek (4 KB) of the first important-files match                                      | after-name **or** near-right — *open*                                                     | off             | —         | ≤60                     | quoted\*                         |





# Working Notes / Scratch

*Opened 2026-08-22 by Joseph, on finding the first lattice both unwieldy and incomplete ("the fact that the lattice is unwieldy AND incomplete gives me the impression that we're doing something wrong here"). This is the design-first refactor of the facet / furniture / type / kind / mass / extra-info / dir-census / leaf-census / deep-agg cluster — **no code until this is nailed**. Seven fields, his; the unique-notes / weight / filter / flag columns of [[aspect-lattice|lattice 1]] are deliberately dropped. Rows marked `*` are structural (always present, not configurable). The claims cluster (has / facet / kind / furniture / labels) is at the bottom on purpose — it is the next thing to refound.*

**Field meanings.** *derived-from*: the raw facts or other rows this is computed from (a dir's value from its subtree is stated here, which is what makes a separate "deep-agg" row unnecessary). *default position*: one of `level-location*` (the tree/indent cell) · `name-location*` (the slot a name stands in — filename **or** glob-template **or** leaf-census, mutually exclusive) · `after-name` (decoration fused to the name) · `far-left` (positional block before the tree) · `near-right` (a stable stop after the name; may spill to a sub-row) · `far-right` (right-aligned columns) · `in-cell` (a mark inside another fact's cell). *default display*: `always` · `on` · `quiet` (speaks on surprise) · `off`. *width*: cells; `cc` = the 12-cell count cell (`g ␠ m T · NNN . f s u`, [[grid-cleanup|grid-cleanup]] §count cell; 10 under a heading that names subject+unit). *formats*: `*` = default. *stat*: `✓` built and settled as-is · `↬` built, but to be reimplemented against this table · `⇥` deferred to a later stage (demand is not in question).


## Quantities (every one renders as a count cell; a dir's value is derived from its subtree — this is what replaces "deep-agg")

| stat | fact        | derived-from                                                                                       | default position                                                     | default display           | sort             | width | formats                                |
| --- | ----------- | -------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ------------------------- | ---------------- | ----- | -------------------------------------- |
| ↬ | lines       | file: read (physical \| non-blank); dir: Σ descendant text files (read budget → `~`; bounds → `≥`) | far-right `lines`                                                    | on                        | Y (`line-count`) | cc    | count cell, unit `𝓁`                  |
| ↬ | bytes       | file: `st_size`; dir: Σ descendants (furniture/ignored excluded)                                   | far-right `bytes`                                                    | quiet (magnitude outlier) | Y (`size`)       | cc    | count cell, unit `B` (1024)            |
| ↬ | files       | dir: # descendant files (furniture/ignored excluded; `≥` under bounds)                             | near-right (inside the census today) — *open: own far-right column?* | on                        | Y?               | cc    | count cell, `● … 𝓃`                   |
| ↬ | dirs        | dir: # descendant dirs                                                                             | near-right (inside the census)                                       | on                        | —                | cc    | count cell, `□ … 𝓃`                   |
| ⇥ | tokens    | bytes × per-kind prior (`~`); later a tokenizer / per-substrate history                            | far-right `tokens`                                                   | off                       | Y                | cc    | count cell, unit `𝓉`                  |
| ↬ | dir-census  | an **unexpanded** dir's own children: dirs (with subtree files), files by suffix, ignored          | near-right (spills to a sub-row when long)                           | on (not quietable)        | —                | var   | count cells; delimiter *open* (`⟨ ⟩`?) |
| — | child-count | = the census's totals (no separate fact)                                                           | —                                                                    | —                         | Y?               | —     | —                                      |

## Time and aliveness

| stat | fact | derived-from | default position | default display | sort | width | formats |
| --- |---|---|---|---|---|---|---|
| ↬ | mtime / age | file: stat; dir: inode mtime today — *newest-beneath proposed* | far-right (paired in the heat cluster); far-left compact — *open* | quiet (recent vs now); on inside the cluster | Y (**default key**, newest first) | 9 (`13.6d ago`) / 5 compact (`13.6d`) | relative\* / iso-8601 / epoch / signa |
| ⇥ | created (btime) | getattrlist / statx; absent elsewhere | far-right | off | Y | 9 | relative / iso-8601\* / epoch |
| ↬ | heat | git log, commit-decay (half-life 7 commits); dir: max of non-noise leaves; repo-relative | **far-left, two-cell density block before git-status** (Joseph, 2026-08-23); the age stays far-right | on in-repo | Y | 2 (`░▒`) | density\* (`  ░ ░░ ░▒ ▒▒ ▒▓ ▓▓ ▓█ ██`) / score |
| ↬ | git-status | `git status --porcelain` (worktree wins); `⊘` = gitignored | far-left (one-cell glyph block) | on (blank when clean) | — | 1 | glyph\* `⊘ M A ⁇ R U D` (`C T` available) |
| ✓ | initial-sha / latest-sha | git log `--name-status` (never past the log window) | far-right | off | Y | 7 / `H~N` | short\* / h~n / full |
| ⇥ | prior-name | git log rename-follow | after-name or sub-row | quiet (recently renamed) | — | var | was\* |
| ⇥ | last-look-delta | the caller's after-image ([[last-look\|Last look]]) | far-left glyph or near-right | quiet (unchanged prints nothing) | Y | 1 | `+ Δ −`\* / delta / signa |
| ⇥ | working-surface | live working markers (Working Notes, TODO/OPEN — config patterns) × recency (orient-rank "molten") | near-right mark; census form `N molten` | quiet | Y | 1 | mark |
| ⇥ | mentions | inbound references to this name across the locus (grep-scale) | far-right | off | Y | cc | count cell |
| ⇥ | preeminence | transitive inbound dependents (per-kind link extractors) | far-right | off | Y | cc | count cell |

## Identity and anomaly (quiet by nature)

| stat | fact | derived-from | default position | default display | sort | width | formats |
| --- |---|---|---|---|---|---|---|
| ✓ | permissions | stat mode; speaks when odd for level; special bits always | far-right (or far-left block) | quiet | — | 3–4 / 9 block | octal\* / rwx-block |
| ✓ | owner / group | stat uid/gid; speaks when not you and not the level's majority | far-right | quiet | — | var | name\* / id |
| ⇥ | linkcount | stat nlink | far-right | off | — | 2 | n |
| ⇥ | cloud | FS attribute (evicted / not hydrated) | after-name mark | quiet | — | 1 | evicted |
| ✓ | filekind-word | suffix map vs the level's plurality (the binary among the .md) | near-right | quiet | — | var | word\* |

## Honesty marks (facts about the *look*, loud by default)

| stat | fact | derived-from | default position | default display | sort | width | formats |
| --- |---|---|---|---|---|---|---|
| ↬ | count-marks | the walk/read/bounds state of the set an aggregate describes | in-cell (`m` slot of every count cell) | always | — | 1 | `≈` exact-grouped · `≥` floor · `~` estimated |
| ✓ | denied | readdir/stat EACCES/EPERM | near-right marks | always | — | 8 | `[denied]`\* (the word beats any glyph — both witnesses) / `[unreadable: io]` |
| ✓ | walk-bound | the stat budget ran out here | near-right marks (and header) | always | — | 12 | `[walk bound]` |
| ✓ | cycle / other-fs | `(dev,ino)` seen on this path / mount boundary | near-right marks | always | — | 7–10 | `[cycle]` / `[other fs]` |
| ↬ | ignored-remainder | gitignored files at an expanded level | near-right (or a census bucket) | always | — | cc | `ignored` subject + count cell (subgroup form open) |
| ⇥ | empty-dir | readdir = ∅ | near-right marks | always — *open* | — | var | `[empty]`? (not `∅` — kept apart from `⊘`) |

## Claims about a place — the cluster to refound next (kept at the bottom on purpose)

*Today these are five words for ~one thing: a claim about what a place **is** or **holds**, graded by how it was known. Rows as they ship, stated plainly; the refounding happens here, not in [[furniture|furniture]]/[[labels|labels]] piecemeal.*

| stat | fact | derived-from | default position | default display | sort | width | formats |
| --- |---|---|---|---|---|---|---|
| ↬ | has | the furniture map: a **glob-matched name** claims kind words (`git`, `rust`, `agents`, `build`, `archive` …); hidden dirs add a readdir-only file count (`archive ≈127f`); `git` subsumes the ignore-file words (code rule) | near-right `[has: …]` → sub-row | on | — | var | words\* / two-letter tags (candidate) / kind-glyph-block (rejected by both witnesses) |
| ↬ | facet: git | the `.git` plugin — **verified**: remote (local config), branch, short HEAD, dirty count (one porcelain subprocess) | near-right `[git: …]` → sub-row | on | — | var | `remote<host/path> br<x> @sha dirty N` |
| ↬ | facet: github | the `.github` plugin — verified: workflow count | near-right `[github: …]` | on | — | var | `N workflows` |
| ✓ | furniture (fate) | the map's **hide / omit / mark** verdict on a name — not a rendered fact; it decides child-slot vs parent-state | — | — | — | — | — (`--show-all` / `--inspect KIND` restore) |
| ↬ | kind / labels | the *rendering* of has-words on a line (sorted, deduped) — not a separate fact | — | — | — | — | — |
| ✓ | filetype (suffix) | the name's suffix; the census's bucket key; globify's field | (census buckets) | on | Y | — | suffix\* / bare |

## What this table drops from lattice 1, and why

- **deep-agg** as an office: every quantity row says in *derived-from* how a dir gets its value from its subtree. One row per fact, two derivations.
- **child-count** as a fact: it is the census's totals.
- **column / INFO** binary, **flag**, **filter**, **weight**, **unique**: not fact properties. Flags are refused anyway (compose via config); filter is Focus's weight law; weight/survival is the allocator's ([[sort|Sort]] §survival); the unique notes moved into *derived-from* or into the rows' own designs.
- **dir-census / leaf-census** are kept as rows because *composition by subject* is not derivable from any single fact — but they are now the same grammar (count cells over a set), differing only in *which set* and *where it stands*. Whether they collapse further once the subgroup-subject form is decided is open.

## Open on this table (Joseph)

- readme-title: after-name vs near-right. focus-match: mark and place. files/dirs: own far-right columns or census-only. mtime at far-left compact. The empty-dir mark. The subgroup-subject form (which decides how dir-census / leaf-census / ignored / has-masses are written). The claims cluster's refounding — next.
