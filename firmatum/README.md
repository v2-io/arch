# firmatum — tooling belt

Program-level tools that **firm, confirm, and establish** work across the estate — not the lived register, and **not** the old PROPRIUM ontology tree (that lives under `proprium/INGEST/old-firmatum`).

**Name.** *Firmare*: to strengthen / confirm / establish. Coined here for the auxiliary tooling belt after the predecessor research repo of the same name was parked for ingest.

**Naming rule for tools inside this belt (Joseph, 2026-07-10):** utilities say what they do, plainly. Latin names are reserved for fundamental things that deserve a new name (memorata earned its name as a concrete PROPRIUM memory-mechanism experiment; a file-mover does not). So: `mv-src-repo`, `fmt-md` — not ornamental Latin.

## Layout

```text
firmatum/
  utils/              # programme-internal / pre-public tools only
    md-press/         # markdown canonicalizer (Rust; still listed as fmt-md in older notes)
    aspectus/         # the look of a locus — budgeted tree snapshot (Rust)
    git-heat/         # git-heat — per-path commit-decay heatmap (inside one repo)
    repo-heat/        # repo-heat — per-repo commits/day EMA heatmap (across repos)
  udon/               # sm v2-io/udon  (independent product)
  ato/                # sm v2-io/ato   (private)
  relata/             # sm v2-io/relata
  practica/           # sm v2-io/practica
  verisectorium/      # outline+segments pattern (working name); theory founding in progress — see its README
  # descent/          # deferred — still nested under udon/tools/descent
```

**Split:** top-level `firmatum/<project>/` = independently applicable projects. `firmatum/utils/` = Archema-local or not-yet-extracted tools (and, later, shared `bin/` harvest).

## Built / mounted

### [`utils/aspectus/`](utils/aspectus/)

The look of a locus (Rust). Print-and-quit budgeted snapshot for agents; well-known furniture (`.git`, `target/`, …) is parent state, not a child listing. Faculty name **aspectus**; one snapshot is an **aspecta**. Glance pipeline is live — [`utils/aspectus/ASPECTUS.outline.md`](utils/aspectus/ASPECTUS.outline.md).

```sh
cargo install --path firmatum/utils/aspectus
# → ~/.cargo/bin/aspectus
```

### [`utils/fmt-md/`](utils/fmt-md/)

Markdown canonicalizer (Rust; house standards today; general engine). In-tree.

**Install (global):**

```sh
cargo install --path firmatum/utils/fmt-md
# → ~/.cargo/bin/fmt-md  (requires ~/.cargo/bin on PATH)
```

Queued (not done): rename, public crate/repo readiness, re-intern as `firmatum/<newname>/` submodule and empty-or-repurpose `utils/` for internal migrators.

### [`utils/git-heat/`](utils/git-heat/)

Commit-decay heatmap for any git repo or subdirectory — CLI ranking or interactive HTML tree + file viewer. Index: [`utils/git-heat/README.md`](utils/git-heat/README.md). Ages in *commits behind HEAD*, inside one repo. A path that is not a git work tree is ranked by filesystem mtime instead (no half-life).

```sh
ln -sfn ~/src/arch/firmatum/utils/git-heat/git-heat-decay/git-heat ~/.local/bin/git-heat
git-heat --help
git-heat --html --serve
```

### [`utils/repo-heat/`](utils/repo-heat/)

Per-repository commits/day heatmap across the josephwecker + v2-io GitHub catalog (plus local-only extras such as `~/src/AISI-responses`). Exponentially weighted wall-clock half-life (default 7 days). Ages in *days*, across repos — the sibling of git-heat. Index: [`utils/repo-heat/README.md`](utils/repo-heat/README.md).

```sh
ln -sfn ~/src/arch/firmatum/utils/repo-heat/repo-heat ~/.local/bin/repo-heat
repo-heat
repo-heat --half-life 3 --top 20
```

### [`udon/`](udon/)

UDON language / tooling (submodule `v2-io/udon`). See that tree's README.

### [`ato/`](ato/)

ATO tooling (submodule `v2-io/ato`, private). See that tree's README / CLAUDE.

### [`relata/`](relata/)

Cross-project bibliography CLI (submodule `v2-io/relata`). **Code-only repo**; bibliography data is external.

| Concern | Fact |
|---|---|
| **Data** | `$RELATA_DATA_DIR` (default `~/.local/share/relata/`) — **already external**; no “move the bibliography” when mounting under firmatum. Same store whether you run from `~/src/relata` or `firmatum/relata`. |
| **Global command** | Gem install under mise Ruby (`…/gems/relata-*`). Not a path into any checkout. |
| **Developing from this mount** | Edit `firmatum/relata`, then **`gem build` + `gem install` from that directory** so PATH `relata` picks up the new code. Submodule alone does not replace the gem. |
| **Local overlay (this machine, 2026-08-02)** | Obsidian vault overlay recreated under `firmatum/relata/` (shared-config symlinks + vault-local plugins/json), same class of gitignored local as practica. `mise trust` applied on `firmatum/relata/mise.toml`. |
| **Not moved** | `_emitted/`, `.yardoc/`, `.ruby-lsp/` — generated/local IDE; recreate as needed. |
| **Tests** | Some tests assume `~/src/_ref/…` assets; optional. |

After clone of arch: `git submodule update`, `mise trust firmatum/relata` if needed, reinstall gem from `firmatum/relata` for live code; data dir keeps working without that.

### [`practica/`](practica/)

Practica notes/docs (submodule `v2-io/practica`).

**Local-only overlay (gitignored; must be recreated per machine):** `.obsidian/` (shared-config symlinks + vault-local plugins/json) and `ref/Art-of-Action` → `~/src/_ref/books/…`. Fresh `git submodule update` does **not** bring these — they live only in a working tree that had them (e.g. historical `~/src/practica`). On this machine they were **physically recreated under `firmatum/practica/`** (2026-08-02) so the programme mount is usable as an Obsidian vault without depending on `~/src/practica`. No install binary.

### descent — deferred re-home

**Do not add `firmatum/descent` yet.** Descent already lives as `firmatum/udon/tools/descent` → `v2-io/descent`. Leave it until Joseph removes that nest from within udon and re-homes at `firmatum/descent/`.

## Planned / tracked

- **`mv-src-repo`** — general `~/src/` relocator (spec in [`../MIGRATION.md`](../MIGRATION.md) §5); after hand migrations are harvested.
- **fmt-md** rename + public extraction + submodule re-intern (see above).
- **Harvest** cross-member `bin/` tools that belong in `firmatum/utils/`.
- **`utils/descent`** — only after udon drops `tools/descent`.

## Conventions

Ruby for scripts; Rust where the tool is durable content-parsing (see asf `agents.sop.md`). Each tool gets a `--help` worth reading and refuses destructive work without a journal.

See `notes/TREE-REORG-PLAN.md` for programme tree context.
