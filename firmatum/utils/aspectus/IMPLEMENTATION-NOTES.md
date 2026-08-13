# Implementation notes

> **Seed, not tracker.** Pipeline and progress: [`ASPECTUS.outline.md`](ASPECTUS.outline.md). Live effort: [`PRACTICA.md`](PRACTICA.md). This file is founding-conversation residue for `design/` segments. Do not add new decisions here — put them on the outline row or in the segment.

*Founding conversation, 2026-08-13. Not a spec to comply with while inventing — a record of what was seen and decided, so a later pass does not reconstruct from vibe.*

Where a claim is Joseph’s it is marked **steward**. Where it is this founding instance’s inference it is marked **proposed**. Unmarked sentences are descriptive of the conversation or of artifacts that were actually read.

---

## What this is

A print-and-quit snapshot of a filesystem **locus** (PROPRIUM: the place of action — project, sandbox, machine, domain). Agents are the primary reader. The job is a *picture of the system at a glance*, under a hard output budget, in the philosophy of Erlang prettyprinting: spend the paper so the whole term still has a shape. Not Erlang parity — not `io_lib_pretty` ported to inodes.

**steward:** well-known furniture is *state on the parent*, not a child you decline to expand. `.git/ (vcs, 509M)` still spends a child slot on implementation guts. The parent line should already say what the place *is*.

```text
asf/  636M  [git: v2-io/agentic-systems  br<main>  M] [kind: git, obsidian-vault, agents]
md-press/  903M  [kind: rust] [rust: target ~890M]
```

---

## Names

**steward** accepted the duality:

| Word | Job |
|---|---|
| **aspectus** | the faculty; the binary; “the look” |
| **aspecta** | one snapshot — the seen-things (house morphology: memorata, percepta, operata) |
| **locus-aspectus** | the concept, when speaking PROPRIUM |

The compound is the concept. The command is the half that is not already the path: you run `aspectus` *on* a locus. Putting both roots in the binary would be naming a function `function_argument`.

Neighbors that must not collapse:

| Name | Job |
|---|---|
| **carta** | who this place *is* (standing identity) |
| **aspectus** | how it *looks right now* |
| **conspectus** | what a *mind* is shown (assembled context — already a PROPRIUM crate/concept) |
| **percepta** | ongoing status/health |

`spectus` was an earlier candidate. It remains a fair English-facing nickname. It is not the binary, once the placement is “aspect of a locus.” `conspectus` is the more exact Latin for “survey of the whole” and is exactly why this tool must not be called that.

Firmatum rule (Joseph, 2026-07-10): Latin is for a thing that needs a name, not a file-mover in a toga. This earned the coinage because the object is a kind of seeing, not a tree clone.

Holding path: `firmatum/utils/aspectus/`, same pattern as md-press, until it deserves its own repo.

---

## Why not broot (measured, not vibed)

Joseph’s closest existing shot, on `~/src/arch`, 2026-08-13:

```text
broot -g -d --cmd ":set_max_depth 5;:print_tree" -s \
  --sort-by-type-dirs-first --max-depth 5 --show-root-fs --height 300 -h
```

Captured export (TTY on stdin, file on stdout, `--color no`): `/tmp/tree-util-spike/broot-export.txt`, 298 lines. That dump is the specimen.

What it actually did with a 300-line budget, hidden-and-dirs-first, height-fill BFS:

- Lines 5–34: `.git/` internals (hook samples, `objects/05`, `COMMIT_EDITMSG`).
- Then ~90 lines of every root-level file under `asf/`.
- `firmatum/utils/md-press` (903M, the working tree of that day) → `md-press …`.
- `proprium/comproprium` → `14 unlisted`.
- `01-aat-core` → two files + `6 unlisted` (those six include `src/`).
- `vivarium` (35G) → a shallow BFS slice.
- `CHARTER-DRAFT.md` and `AGENTIC-DELEGATION.md` at line 290.

`N unlisted` / `name …` do not say *what* was omitted. A collapsed directory has no child census. Symlink targets clip mid-path. Git status is one `M` on `asf`.

broot is a good existence proof that “tree + size + date + a few collapses” is ~40%, and a good existence proof that **height-fill BFS cannot be the allocator**.

`tree -L` and `tree --filelimit` are cutoffs. A cutoff is not a summary.

---

## The allocator (**proposed**)

First-class constraint: `--lines N`. Every directory that exists appears, either expanded or as a typed summary. Siblings *share* remaining budget.

Sketch:

```text
weights = importance(child)   # dir, match/focus, witness name, kind-rich; absorbed names have weight 0 because they are not children
shares  = largest-remainder(remain, weights)
render each child with its share
one last line is the typed aggregate of share==0
```

Importance is a claim and will be wrong at first. `--explain-budget` exists so we can see the wrongness.

Focus (`--focus`, stdin paths from `rg -l`) is a **weight**, not a filter. Matches get lines; everything else stays as locating summaries.

---

## Four fates of a name (**steward** shape, table **proposed**)

Read the directory. Partition through config. Fold absorb/omit into parent annotations. *Then* hand leftover children (plus witnesses) to the allocator. Hidden is not a display toggle. It is a recognition problem.

Grounding sample, 2026-08-13, of hidden names at real roots under `~/src/arch`:

| Root | Hidden that would currently absorb | Hidden that would currently remain children (until mapped) |
|---|---|---|
| `arch/` | `.git/`, `.gitignore`, `.gitmodules`, `.obsidian/`, `.obsidian.vimrc` | `.archive/` (role child), `.orient/` |
| `asf/` | `.git` gitlink, `.gitignore`, `.obsidian*`, `.claude/`, `.build/`, `.ruby-lsp/` | `.check-links-ignore`, `.zenodo.json` |
| `md-press/` | `.gitignore` | — |
| `udon/` | `.git` gitlink, `.gitignore`, `.gitmodules`, `.gitattributes`, `.github/`, `.claude/` | `.fmt-mdignore` |
| `vivarium/` | `.git` gitlink, `.gitignore`, `.obsidian*`, `.claude/` | `.archive/`, `.super-archive/`, `.trash/`, `.orient/` (role / unmapped) |
| `relata/` | `.git` gitlink, `.gitignore`, `.obsidian*`, `.ruby-lsp/` | `.dx-use-local`, `.rubocop.yml` |
| `ato/` | `.git` gitlink, `.gitignore`, `.mise.toml` | — |
| `firmatum/`, `proprium/` | (none) | — |

**Absorb** (never listed; become parent state):

| name | facet / kind |
|---|---|
| `.git/` or `.git` gitlink | `git:` |
| `.gitignore` `.gitmodules` `.gitattributes` | `git:` |
| `.github/` | `git:` or thin `github:` (workflow count, not the dir) |
| `.obsidian/` `.obsidian.vimrc` | `obsidian-vault` |
| `.claude/` | `agents` |
| `target/` `node_modules/` `__pycache__/` `.build/` `.ruby-lsp/` | toolchain/build; size may hang on the language facet |
| `.mise.toml` | `mise` |
| `.DS_Store` | omit entirely — not a kind, not a child |

**Witness** (contribute to `kind`, still listed if they survive importance/budget): `Cargo.toml`, `pyproject.toml`, `Gemfile`, `mise.toml`, `AGENTS.md`, `CLAUDE.md`, `README*`, `package.json`. These are documents someone might open.

**Omit:** `.DS_Store`.

**Child:** everything else, including hidden names the config does not know. Role directories with real contents (`.archive/`, `.super-archive/`, `.trash/`, this estate’s `archive/`) stay children. You might look in them. Tag the role; do not vanish them into `[kind: archive]` the way `target/` vanishes into rust.

Detection is the union of witnesses. `Cargo.toml` + absorbed `target/` → `rust`. A lone `target/` without `Cargo.toml` is still absorbed (do not list it) but rust is a weak/guess kind and must be marked as such. A directory with no witnesses does not get a fake `[kind:]`.

Config is layered: tool defaults + user + repo. Estate names (`.orient`, `.fmt-mdignore`) belong in an overlay, not world-law.

---

## Walk cost

A truthful census of a collapsed directory needs a walk. `~/src/arch` was 38G that day; `vivarium` 35G. You cannot stat `.git/objects` to tell an agent what `arch/` is.

**proposed** prune, as part of truth-telling, not as a shortcut:

- Do not enter absorbed names, even *to count*.
- For those: the parent facet carries name, role, and a size if cheap.
- Ordinary dirs: walk, with a visit budget and an honest `≥` if we stop.
- Honor gitignore for hiding file bodies; still show an ignored directory’s presence as absorbed/collapsed state.
- Cache derived metrics (line counts by ino+mtime+size; heat by `HEAD`+path). Do not cache the walk first.
- Default `-x` (one filesystem).

CI must not walk `~/src/arch`. Allocator tests run on a tiny fixture tree under `fixtures/`.

---

## Output

Two views of one `Aspecta`:

- **Text** — sparse. Columns appear when they carry surprise. Agent profile is the default: no icons, no alternate screen, stable shape.
- **JSON** — the snapshot complete enough to parse. **proposed** as P2.
- **UDON** — after the IR is stable. Not a gate.

`--explain-budget` is a third view, for us.

---

## Git facet, heat, introducing-commit

Local and cheap (**proposed** P2): short remote (`github:v2-io/asf`), branch, porcelain counts, short HEAD.

**steward** wants `(private)` on the remote. That is not local. **proposed:** show the remote always; show private/public only when already known. Do not guess from `git@` vs `https://`.

Heat and `H~N` (introducing commit) are real and wanted. They are P5. Reuse `git-heat`’s commit-decay model; do not invent a second one. Introducing-commit and rename-follow run only on the visible/focused set.

---

## Language and shape of the crate

Rust. **steward:** “a rust crate I'm assuming unless you would like to do something different.” No reason to differ: agents will invoke this constantly; walk+stat+ignore wants `ignore` / `walkdir` / `git2` or carefully bounded `git` subprocesses; md-press is the sibling durable-tool pattern.

Help is hand-rolled (md-press style), not a clap novel, until the flag surface exists.

Suggested modules when P1 starts (proposed, not a wall):

```text
src/
  lib.rs          Aspecta
  main.rs         CLI
  absorb.rs       config mappings; partition
  walk.rs         bounded walk; never enters absorbed names
  budget.rs       sibling-share allocator
  render/
    text.rs
    json.rs
  kinds.rs        witness → kind / facet extractors
```

No TUI crate. No ratatui. No crossterm except if a test needs a PTY, which it should not.

---

## Comparison points (do not wrap these; steal the gap)

| Tool | What it gives | What it cannot |
|---|---|---|
| broot `:print_tree` | size, date, some `N unlisted`, git letter | sibling-shared budget; typed census; absorb; line counts; follow-symlink-dirs honestly |
| `tree -L` / `--filelimit` | cutoff | summary |
| eza `--tree --git` | columns, git status | budget allocation; parent-as-kind |
| git-heat | commit-decay heat | a locus picture |
| `dust` / `dua` | size | shape |

---

## Open questions worth not closing early

- Default `--lines` number. 80 vs 120 vs “terminal height if a TTY, 100 otherwise.”
- Whether `mise.toml` (non-dot) is witness (listed) and `.mise.toml` is absorb — current table says yes.
- Whether `.github/` is its own thin facet or part of `git:`.
- Config filename (`.aspectus.toml` vs `aspectus.toml` vs XDG).
- How weak/guess kinds are spelled in text so they cannot be mistaken for claims.
