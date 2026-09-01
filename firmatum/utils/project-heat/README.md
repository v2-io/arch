# "Project-Heat": Project Activity & Status

- `pd <name>` prints a project's root directory (`aspectus $(pd arch)`); it is a symlink to `projects` (argv[0] dispatch), also reachable as `projects pd <name>`. `cpd <name>` cd's there — a zsh function (`cpd.zsh`, symlinked into `~/.config/zsh/interactive/25-cpd.zsh`) since a child process can't move the parent shell.
- `--color=auto|always|never` (default auto): a TTY gets truecolor; a pipe or agent gets the same look in glyphs with no ANSI. Heatmap glyph vocabulary — heat is the lower-eighths ramp ` ▁▂▃▄▅▆▇█` (U+258N is N/8 fill by design, so magnitude is computable from the codepoint and rows read as sparklines; same log1p t as the color ramp, onramp/offramp/birth included), `·` pre-birth wash, zero days and stale blank (matching the color design's prominence order — stale is the dimmest fill there too). The shade/texture alphabet is deliberately distinct from the magnitude alphabet; chosen 2026-09-01 from the arch/utf axes data (designed-fill MAE 0.015–0.03 vs rasters; tokenization + glyph-comprehension survey).
- CLI conventions (arch/principles pass, 2026-09-01): exit vocabulary 0/1/2/130 (SIGINT clean, broken pipe quiet); `--version` = name + semver + sha while untagged; `--format=json` on the listing (census records: facts, ISO times, plus edges) and on `heatmap` (per-project day counts + scale); refusals name the class and offer a next action; `projects feedback <text…>` appends provenanced friction reports to `inbox.md` here; help carries commands/examples, the axis glyph vocabulary, and teaches that the tool writes `.practica` into project roots.

## The seven axes (2026-09-01; TAXONOMY-PLAN.md)

The retired single `disposition` field is replaced by seven independent axes. **No migration happened and none will**: the axes exist because disposition wasn't working, so old values are ignored — stored `disposition` keys are silently dropped, census `disposition:` lines refuse with the new syntax, and `projects set NAME <old-token>` refuses with a menu naming the axes it splits into (e.g. `historical` → "say what you mean: activity=archived"). Joseph declares axis values fresh, going forward.

| axis | values | provenance |
|---|---|---|
| git-rel | gitroot · subgit · submodule · no-git · group | **detected** (group = no own git but census children with git) |
| git-vis | local · private · public · external · group | **measured, freezable** — an `internal-git-orgs` repo's public/private comes from `--refresh`'s `gh repo view` (cached as `gh-visibility`; the primary source — real network answers, overridable by freezing git-vis); `private` fallback until first measured. Other org → `external`; no remote → `local`; a subgit inherits its census parent's git-vis. Normal runs never touch the network. |
| lifecycle | nascent · experiment · pre-poc · pre-mvp · pre-release · ongoing · maintenance · reference | **declared** (pre-\* are re-enterable milestones; reference = still influence-bearing; usage is local-use's business) |
| activity | active · paused · watch · archived | **declared** — aspiration/directive, never auto-demoted from staleness |
| proj-rel | root · mid · leaf · isolated | **derived** from the census parent graph |
| local-use | undeclared · pending · pilot · transition · secondary · primary | **declared**; `undeclared` non-frozen default |
| public-use | undeclared · unlikely · peers-only · undecided · planned · preprint · submitted · accepted · published · early · broad | **declared**; `undeclared` non-frozen default |

Declared-axis defaults come from `census-details.txt` `default <axis>=<value>: glob [glob…]` lines (first match wins, per axis); per-project values are frozen with `projects set`. Heatmap membership is `activity == active`; everything else stays in the listing.

## Listing: counting verbosity + axis glyph block

- *(no flag)* — names only.
- `-v` — a 7-character **AXES** block (one column per axis, order as the table above; position = which axis, glyph = value, `·` = undeclared/unknown) + names + paths. Vocabulary is provisional v0, taught in `--help`.
- `-vv` — adds `desc`. `-vvv` — adds last-action + git.
- `projects set NAME experiment paused` — bare tokens resolve to their axes (every value is unique across axes except `group`, which sets both git axes; `rel-group` / `vis-group` pick one). `projects set NAME desc="…"` for the one-line description.
- `projects unset NAME field [field…]` unfreezes and clears; detection / census defaults refill on the same run.
- `--refresh` also caches `gh-stars` / `gh-forks` / `gh-watchers` per github remote (same single `gh repo view` call as visibility; absent when unmeasured). Shown at `-vvv` as a `★·⑂·◉` column — temporarily in desc's slot while Joseph gauges whether that's the right meaningfulness level (desc stays at `-vv`). **Derived public-use floor:** measured stars+forks+watchers > 50 → `public-use=published` (derived; declared/frozen values win).
- The axis glyph vocabulary lives in `axis-glyphs.json` beside the script — edit it to experiment; `--help`'s AXES section renders from the loaded table so changes show up in usage automatically. Per-axis keys: `glyph-<value>` (one char), `fg-<value>` / `bg-<value>` (`#rrggbb` for a light terminal, `""` = none), optional `fg-dark-<value>` / `bg-dark-<value>` (dark terminals otherwise channel-invert the light hex, the heatmap's convention); `<value>` colors also accept the pseudo-value `undeclared`. Colors are presentation only — applied on the TTY/`--color` path with targeted resets (row underline survives); **pipes and agents always get bare glyphs, never ANSI**. Missing file falls back to built-ins; unknown axes/values, bad hexes, and empty glyphs are skipped with a stderr note; glyphs may be any width — each axis column pads to its widest loaded glyph.

## Edges — `census-edges.txt`

Typed relations between projects, hand-typed beside the census: `inverse: A <-> B` pairs, `alias: NAME -> REL` shorthands, and edge lines in **sentence order** `FROM REL TO [INV]` — comma-lists (REL/INV pair positionally with FROM), globs against census names, `[brackets]` guard spaces, `-` = explicitly no inverse. Either side may declare; readers collapse both-side declarations into one canonical directional set. Exposed per-project as `edges-out` / `edges-in` in `--format=json`; rendering (lineage threads on the heatmap) is future work.

## Mechanics

- `projects` (which utilizes cdpath and the `census-details.txt` narrowing) populates `.practica` as it goes and prints in **canonical** order (cdpath, then lexical). `-g` adds cdpath-parent headings; `-c` sorts by last-action-time; `-a` alphabetically. The same flags apply to `projects heatmap`. `.practica` is the cache: later runs reuse git/session/last-action unless `--refresh` or HEAD/index/session dirs are newer.
- `projects heatmap` — one inferno row per `activity=active` project, 17 full weeks plus the current week through today (newest at the left; grows through Saturday, then the oldest week drops). `--double` / `--triple` stack the previous 17-week band(s) below, shifted so weeks line up, with a full-width rule between bands; the scale bar is only after the last band. Month ticks are `<- Aug |` at the start of each month (older/right edge: day 1); omitted if the span is too short to fit. Older bands omit any project whose first-commit is after that band's newest day (not yet born during those 17 weeks); born mid-window still shows, with the tail wash. The current band still shows every active row. Names right-aligned and elided at 29. Between the month labels and the first row, each full week is labeled `| 25·254 |` (space after the pipe, unpadded nproj·ncontrib, spaces until the next pipe): distinct projects with commits that week, or a birth onramp / idle-cooldown offramp (not 0-commit white, pre-birth wash, or stale) | total heatmap counts that week. Color is log1p-normalized across the whole grid and mapped continuously through matplotlib's 256-stop inferno LUT (truecolor per cell, not a bin); the scale bar samples that ramp at 120 stops. Inferno is for a dark terminal (black→purple→red→yellow); on a light appearance each RGB channel is inverted (`1 - rgb`), so 0 is near-white and heat goes through ice-blue/teal toward dark. Idle/pre-birth use the heat scale: pre-birth walks t=0.125→0.5 over 4 days (wash `#e6e6f1` beyond that); post-activity idle walks t from the last non-zero day down to 0. Own-git and subgit rows are exclusive commits (plus dirty mtimes as uncached blips). Projects with no git at all use each file's birthtime and mtime as the change-count proxy — same walk yields first-commit (tail onramp), last-action, and per-day counts. Empty trees fall back to the directory's birth for first-commit so the tail still has somewhere to fade from.
- Parent stats (last-action and heatmap) are residual: git pathspecs exclude immediate census children (`:!child`). Submodules fall out naturally (child commits live in the child repo; parent only sees gitlink bumps unless we exclude them too, which we do). Subfolders are the same exclude, just in the shared object store. Cache key `stat-kids` / `heatmap-kids` rebuilds when the child set changes — which is also the hook for promoting a folder to its own submodule: the relative path often stays, new commits stop appearing in the parent log, and a path *move* changes the kids list and rebuilds.
- The projects can be (and often are) nested-- e.g., arch -> asf -> 01-aat-core
- It assumes that cdpath (for zsh) is set-- and uses that as the basis for traversal of projects (applying census-details, which has drop-globs, per-parent whitelists, and per-axis default blobs)
- Nested identity is a second pass over the census, closest-to-`~` first: a project has a parent iff another census project's root prefixes it (longest prefix wins). `base-name` (dirname, or frozen override) → `parent-name` (parent's `name`) → `name`. `proj-rel` falls out of the same graph.
- Populates and Utilizes `.practica` within the project directories, which should also be in the global gitignore so it can be dropped in external repos etc. (that are reference only).
- Unless a field specifically asks for `base-name`, `name` is the identifier everywhere (`related`, heat, session join, CLI).

## Data/Info

### Field provenance in `.practica`

Each stored field is either **explicit** (frozen — a human/CLI override; refresh must not touch it) or **derived** (default / detected / cached; may be rewritten whenever).

On disk: `.practica` is JSON. Values are top-level keys. A `frozen` list names the fields that are explicit. `projects set <name-or-path> field=value` writes the value *and* adds the field to `frozen`. Detection and census defaults write the value and do not add it to `frozen`.

`base-name` default is the final directory name (`01-aat-core`). Overriding it to `aat` is a frozen `base-name`; `name` then becomes `arch/asf/aat`. That is the whole rename design — no hidden transform.

The old git booleans (`git?`/`subgit?`/`submodule?`/`subproject?`) are no longer stored — `git-rel` and `proj-rel` carry those facts; the booleans are hydrated in memory for internal use only.

### `.practica`

| field | required | default | desc |
| ---- | ----------- | ------- | ---- |
| base-name | true | final directory name | project base name |
| desc | false | (none) | one-line description, set via `projects set NAME desc="…"` |
| project-root | true | given | directory of the project root |
| git-rel | true | detected | {gitroot, subgit, submodule, no-git, group} |
| git-vis | true | detected default | {local, private, public, external, group}; freezable (esp. `public`) |
| lifecycle | false | census `default` lines | see axes table |
| activity | true | census `default` lines (`active` floor) | see axes table |
| proj-rel | true | derived | {root, mid, leaf, isolated} |
| local-use | true | `undeclared` | see axes table |
| public-use | true | `undeclared` | see axes table |
| categories | false | (none) | list of tags that categorize it |
| related | false | (none) | free-form list; typed relations live in `census-edges.txt` |
| has-remote | if(git) | detected | whether the git repo is remotely tracked |
| remote | if(has-remote) | detected | remote repo url |
| org | if(has-remote) | remote's org | but, like the others, can be overridden or set even without it having a remote |
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

### `census-details.txt` axis-default blobs

Drop-globs / `only` decide *membership*. `default` blobs set declared-axis values for survivors. First matching glob wins, per axis; list specific globs before a catch-all. `internal-git-orgs:` names the orgs whose remotes read as `private`.

```
internal-git-orgs: v2-io, josephwecker

default activity=watch:       src-ext src-ext/**
default lifecycle=reference:  src-ext src-ext/**
default activity=active:      **
default lifecycle=ongoing:    **
```

A frozen axis value in `.practica` wins over these blobs.

### git-heat noise (what `extra-noise` adds to)

git-heat already treats these as noise — do not duplicate them into every `.practica`:

- **git heat:** basenames `Cargo.toml` and `SOURCE_REV` (heat 0; ignored for parent max-heat and the color scale). `--noise a,b` adds more basenames for that run.
- **always, in `projects`:** `.practica` itself. It is written by this tool; counting it (or the directory mtime that write updates) ranked the census by overlay-write order. Do not put it in per-project `extra-noise` — it is automatic.
- **non-git (mtime ranking):** skips hidden names (leading `.`) and directory names `node_modules`, `__pycache__`, `venv`, `target`, `dist`, `build`, `vendor`, `.tox`, `.mypy_cache`, `.pytest_cache`, `.eggs`, `.direnv`, `.terraform`, `.venv`, `.cache`, `.git`, `.hg`, `.svn`, `.bzr`.
