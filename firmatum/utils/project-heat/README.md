# "Project-Heat": Project Activity & Status

- `projects` (which utilizes cdpath and the `census-details.txt` narrowing) populates `.practica` as it goes and, without any flags, prints the table in **canonical** order (cdpath, then lexical). `-g` adds cdpath-parent headings on that order; `-c` sorts by last-action-time; `-a` alphabetically. The same flags apply to `projects heatmap`. `.practica` is the cache: later runs reuse git/session/last-action unless `--refresh` or HEAD/index/session dirs are newer.
- `projects heatmap` — one inferno row per `active*` project, 17 full weeks plus the current week through today (newest at the left; grows through Saturday, then the oldest week drops). `--double` / `--triple` stack the previous 17-week band(s) below, shifted so weeks line up, with a full-width rule between bands; the scale bar is only after the last band. Month ticks are `<- Aug |` at the start of each month (older/right edge: day 1); omitted if the span is too short to fit. Older bands omit any project whose first-commit is after that band's newest day (not yet born during those 17 weeks); born mid-window still shows, with the tail wash. The current band still shows every `active*` row. Names right-aligned and elided at 29. Between the month labels and the first row, each full week is labeled `| 25·254 |` (space after the pipe, unpadded nproj·ncontrib, spaces until the next pipe): distinct projects with commits that week, or a birth onramp / idle-cooldown offramp (not 0-commit white, pre-birth wash, or stale) | total heatmap counts that week. Color is log1p-normalized across the whole grid and mapped continuously through matplotlib's 256-stop inferno LUT (truecolor per cell, not a bin); the scale bar samples that ramp at 120 stops. Inferno is for a dark terminal (black→purple→red→yellow); on a light appearance each RGB channel is inverted (`1 - rgb`), so 0 is near-white and heat goes through ice-blue/teal toward dark. Idle/pre-birth use the heat scale: pre-birth walks t=0.125→0.5 over 4 days (wash `#e6e6f1` beyond that); post-activity idle walks t from the last non-zero day down to 0. Own-git and subgit rows are exclusive commits (plus dirty mtimes as uncached blips). Projects with no git at all use each file's birthtime and mtime as the change-count proxy — same walk yields first-commit (tail onramp), last-action, and per-day counts. Empty trees fall back to the directory's birth for first-commit so the tail still has somewhere to fade from.
- Parent stats (last-action and heatmap) are residual: git pathspecs exclude immediate census children (`:!child`). Submodules fall out naturally (child commits live in the child repo; parent only sees gitlink bumps unless we exclude them too, which we do). Subfolders are the same exclude, just in the shared object store. Cache key `stat-kids` / `heatmap-kids` rebuilds when the child set changes — which is also the hook for promoting a folder to its own submodule: the relative path often stays, new commits stop appearing in the parent log, and a path *move* changes the kids list and rebuilds.
- The projects can be (and often are) nested-- e.g., arch -> asf -> 01-aat-core
- It assumes that cdpath (for zsh) is set-- and uses that as the basis for traversal of projects (applying census-details, which has drop-globs, per-parent whitelists, and disposition-default blobs)
- Nested identity is a second pass over the census, closest-to-`~` first: a project has a parent iff another census project's root prefixes it (longest prefix wins). `base-name` (dirname, or frozen override) → `parent-name` (parent's `name`) → `name`. `subproject?` is just whether that parent exists.
- Populates and Utilizes `.practica` within the project directories, which should
  also be in the global gitignore so it can be dropped in external repos etc.
  (that are reference only).
- `projects` will allow for manually setting / overriding fields specified below. `projects set NAME field=value` freezes the field; a bare token (`projects set NAME historical`) is `disposition=<token>`. Heatmap rows are `active*` only, so `historical` (like `inactive` / `inert` / `ref-only`) stays in the table and drops off the heatmap.
- Unless a field specifically asks for `base-name`, `name` is the identifier everywhere (`related`, heat, session join, CLI).

Below, a key ending with '?' means boolean. A key *starting* with '? ' is just
an indication that it is optional. '?(predicate) ' before the key is just
shorthand for when it is needed vs not needed/optional).

## Data/Info

### Field provenance in `.practica`

Each stored field is either **explicit** (frozen — a human/CLI override; refresh must not touch it) or **derived** (default / detected / cached; may be rewritten whenever).

On disk: `.practica` is JSON. Values are top-level keys. A `frozen` list names the fields that are explicit. `projects set <name-or-path> field=value` writes the value *and* adds the field to `frozen`. Detection and census defaults write the value and do not add it to `frozen`.

`base-name` default is the final directory name (`01-aat-core`). Overriding it to `aat` is a frozen `base-name`; `name` then becomes `arch/asf/aat`. That is the whole rename design — no hidden transform.

### `.practica`

*Within .practica we need the following  (some detected and cached, others set through CLI or editing .practica file; explicit stays frozen, derived can be refreshed)*


| field | required | default | desc |
| ---- | ----------- | ------- | ---- |
| base-name | true | final directory name | project base name |
| project-root | true | given | directory of the project root |
| categories | false | (none) | list of tags that categorize it |
| disposition | true | from `census-details.txt` blobs (see below) | {active, active-ref, active-fork, inactive, inert, ref-only, historical} |
| related    | false | (none) | list of related projects by `name` |
| git?       | true   | detected | whether or not the project is also its own git repository |
| subproject? | true  | false    | true if another census project's root is a prefix of this one |
| subgit?    | true   | detected | whether or not the project is git-tracked but from a parent directory |
| submodule? | if(git?) | detected | whether or not the project is a submodule of a parent directory git repository |
| remote?    | if(git?) | detected | whether or not the git repo is remotely tracked |
| remote     | if(remote?) | detected | remote repo url |
| org        | if(remote?) | remote's org | but, like the others, can be overridden or set even without it having a remote |
| extra-noise | false | (none) | additional basenames passed to `git-heat --noise` (even if not a git repo). See below |
| highlight | false | 1 | 0 = gray name; 1 = normal; 2–10 = name toward red; 8–10 underline the row; 10 also bold+italic. `projects set NAME highlight=10` |


### Derived / lazy (can be cached)

| field          | constructed | desc |
| -------------- | ----------- | ---- |
| parent-name    | parent? no: '', yes: immediate parent's `name` | recursively parent name |
| name           | (parent-name /) + base-name | actual full name for the project, e.g., 'arch/asf/aat' when base-name is overridden to `aat` (default would be `arch/asf/01-aat-core`) |
| last-action-time | most recent of (git-log, any dirty mtime) (non-noise) | |
| last-action    | (still a bit undetermined) | something like last PRACTICA entry or git-log commit summary or something |
| last-session   | (see desc) | last claude or grok session originating in this project's root, or if none, parent's. **Noisy by default:** the first session of a project often originates in a sibling or an altogether different directory; cwd-join is best-effort, not proof of ownership. Claude keys `~/.claude/projects/` by cwd slug; grok-build keys `~/.grok/sessions/` by percent-encoded cwd. |
|    -time (ago) | mtime or final timestamp within jsonl | when the last session originating in this project concluded |
|    -id         | from jsonl basename | key used for restoring/resuming session |
|    -model      | inferred + from jsonl | e.g., claude-fable-5, grok-4.6, ... |
| ongoing-efforts | (undetermined) | |
| ... | ... (more to come!) ... | ... |

### `census-details.txt` disposition blobs

Drop-globs / `only` decide *membership*. Disposition blobs default `disposition` for survivors. First matching glob wins; list specific globs before a catch-all.

```
disposition ref-only: src-ext src-ext/** vaults vaults/**
disposition inactive: src/_exp src/_exp/**
disposition active: **
```

A frozen `disposition` in `.practica` wins over these blobs (e.g. `src-ext/grok-build` as `active-fork`).

### git-heat noise (what `extra-noise` adds to)

git-heat already treats these as noise — do not duplicate them into every `.practica`:

- **git heat:** basenames `Cargo.toml` and `SOURCE_REV` (heat 0; ignored for parent max-heat and the color scale). `--noise a,b` adds more basenames for that run.
- **always, in `projects`:** `.practica` itself. It is written by this tool; counting it (or the directory mtime that write updates) ranked the census by overlay-write order. Do not put it in per-project `extra-noise` — it is automatic.
- **non-git (mtime ranking):** skips hidden names (leading `.`) and directory names `node_modules`, `__pycache__`, `venv`, `target`, `dist`, `build`, `vendor`, `.tox`, `.mypy_cache`, `.pytest_cache`, `.eggs`, `.direnv`, `.terraform`, `.venv`, `.cache`, `.git`, `.hg`, `.svn`, `.bzr`.
