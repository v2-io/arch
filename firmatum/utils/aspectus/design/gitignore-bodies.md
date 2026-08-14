# Ignored bodies

Inside a git work tree, gitignored **contents** stay out of the look and out of the numbers; the ignored thing's **presence** still shows. The repo has already declared "this is not the project" — build products, venvs, logs. An agent's picture of the place should honor that declaration without ever hiding that the declaration was applied (Summarization's law: never a silent cut).

Two effects, one per direction of honesty:

1. **Out of the look and out of mass.** An ignored directory is not expanded, gets no dir census of its innards, and contributes nothing to [[mass|Mass]] or line-count aggregates (mass already promises this: *"hidden/omitted furniture and gitignored bodies stay out"* — else every Rust crate is a mountain of `.fingerprint`). Ignored files are not listed and not counted in aggregates.
2. **Presence still shows.** An ignored *directory* keeps its line, with an ignored mark (spelling open) in the same INFO office as `[denied]` — a fact about the look, not the inode. Ignored *files* do not get lines; they appear as a typed remainder alongside the censuses (e.g. a `3 ignored` bucket — spelling open) so the level never pretends they don't exist.

## Mechanism

- Git's own semantics, not a reimplementation-by-vibes: nested `.gitignore` files, negations, repo `info/exclude` — whatever the chosen gitignore library honors is the contract, and the design goal is "agrees with `git check-ignore`". Global `core.excludesFile`: see Open.
- Applies **only inside a git work tree** (and submodules, which are the same thing per [[furniture/git|Furniture/Git]]). Outside a repo nothing changes; a `.gitignore` lying around a non-repo tree is just a file.
- **Tracked beats ignored:** a path that is ignored by pattern but actually tracked in the index is not ignored (git's own rule) — it is part of the project and shows normally.
- **Order with furniture:** furniture fates ([[furniture|Furniture]] map: hide/omit) apply first — `target/` is usually furniture before it is ignored, and furniture's rendering wins. This row catches everything the map does not know: the ad-hoc `scratch-output/`, `*.log`, the project-specific junk. Two mechanisms, one law (nothing silent), no double marks.
- **`--show-all` restores everything**, ignored contents included — the same ask that opens furniture. Restored contents still carry the mark, and see Open on whether restored bodies join mass.
- This row is about *ignored paths*. `.git/` itself is furniture, and the git **status letter** on tracked files is [[furniture/git|Furniture/Git]]'s — no overlap.

## Foundations (clauses)

| Clause | Where |
|---|---|
| Never a silent cut; leftover is typed | [[summarization\|Summarization]] |
| Mass excludes ignored bodies | [[mass\|Mass]] |
| The mark's office; a fact about the look | [[denied\|Denied]] · [[aspect-lattice\|Aspect lattice]] |
| Furniture fates and `--show-all` | [[furniture\|Furniture]] |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Dir presence | An ignored dir's line shows with the mark; its children do not print; no dir census of its innards. | Repo fixture with ignored `logs/` full of files. |
| 2 | File remainder | Ignored files at a level appear only in the typed remainder, with exact count. | Fixture with `*.log` ignored. |
| 3 | Out of mass | Mass/line aggregates over a subtree exclude ignored bodies; identical to the same tree with the ignored parts deleted. | Compare two fixtures. |
| 4 | Tracked beats ignored | A tracked file matching an ignore pattern lists normally. | `git add -f` fixture. |
| 5 | Nested + negation | A nested `.gitignore` and a `!keep.log` negation behave as `git check-ignore` says. | Fixture; cross-check against git. |
| 6 | Submodule | Ignores inside a submodule apply from the submodule's own files. | Submodule fixture. |
| 7 | Non-repo | Outside a work tree, a `.gitignore` changes nothing. | Plain-dir fixture with a decoy `.gitignore`. |
| 8 | `--show-all` | Ignored contents list; marks remain. | Same fixture as 1. |
| 9 | Walk cost | The walk does not recurse into ignored dirs at all (their innards cost no stat budget) — the honesty is from presence + declaration, not from walking them. | Bounded-walk fixture: budget not drained by a huge ignored dir. |
| 10 | JSON | `ignored` is a field; the remainder bucket is data ([[json\|JSON]]). | Parse fixture output. |

## Steward asks (2026-08-14, fold into this row's landing)

Joseph: *"has: git vs gitignored vs .gitignore"* — the has-vocabulary distinguishes three different claims that currently blur: **`git`** (repo machinery — a verified `.git`, facet territory), **`.gitignore`** (the place *has* an ignore file — a contents claim like any map row), and **`gitignored`** (this entry *is itself ignored* — a per-node status, not a parent-contents claim). Three words, three truths; the map and this row split them.

And: *"gitignored files/directories dim color for human and sigil/glyph for non-color"* — ignored entries render **dimmed on a TTY** and carry a **glyph in the no-color look** (the mark this design already wanted, now with its two-channel rendering decided in principle — glyph spelling still interface vocabulary for ratification, [[shorthand|Shorthand]]'s stability law applies).

## Open

- **Mark and remainder spelling** (`[ignored]`? the census bucket word?) — Joseph ratifies; one constant each.
- **Global `core.excludesFile` / `~/.gitignore_global`:** honoring it makes the look agree with the user's git; but it makes the *look* depend on per-user state outside the caller stack — two agents on one machine still agree, two machines may not. Leaning honor-it (git-agreement is the least surprising truth); flag for ratification.
- **Under `--show-all`, do restored bodies join mass?** Leaning yes (show-all means *show all*), but then two looks disagree on a number — may deserve a `≈`-style mark. Not decided.
- Whether an ignored dir's line may carry a *cheap* census (one readdir of its top level only) instead of nothing — costs almost nothing, says a bit more; against it: it walks into what the repo disclaimed. Not decided.

## Not in this row

Furniture mapping and its config ([[furniture|Furniture]]). Git status letters and parent-line git facts ([[furniture/git|Furniture/Git]]). Other ignore dialects (`.ignore`, `.rgignore` — nobody asked; git's declaration is the one with authority about the project).
