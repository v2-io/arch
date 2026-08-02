# firmatum — tooling belt

Program-level tools that **firm, confirm, and establish** work across the estate — not the lived register, and **not** the old PROPRIUM ontology tree (that lives under `proprium/INGEST/old-firmatum`).

**Name.** *Firmare*: to strengthen / confirm / establish. Coined here for the auxiliary tooling belt after the predecessor research repo of the same name was parked for ingest.

**Naming rule for tools inside this belt (Joseph, 2026-07-10):** utilities say what they do, plainly. Latin names are reserved for fundamental things that deserve a new name (memorata earned its name as a concrete PROPRIUM memory-mechanism experiment; a file-mover does not). So: `mv-src-repo`, `fmt-md` — not ornamental Latin.

## Layout

```text
firmatum/
  utils/           # plain-named utilities (in-tree or submodules)
    fmt-md/        # markdown canonicalizer (Rust; in-tree)
    udon/          # submodule v2-io/udon
    # ato/, descent/ — expected as submodules when remotes are ready
  # relata/, practica/, … — expected as submodules when remotes are ready
```

## Built

### [`utils/fmt-md/`](utils/fmt-md/)

Markdown canonicalizer for the house standards: removes manual word-wrapping (structurally, via a CommonMark parse — no heuristics, no flag-triage pile) and optionally promotes Unicode math to `$LaTeX$` with a local ollama model behind deterministic verification. Rust.

**Install (global):** from this repo, re-run after clone or path moves:

```sh
cargo install --path firmatum/utils/fmt-md
# binary: ~/.cargo/bin/fmt-md  (requires ~/.cargo/bin on PATH)
```

`cargo install --path .` from inside `firmatum/utils/fmt-md` is equivalent. Re-run to upgrade; `cargo uninstall fmt-md` to remove. The installed binary does not hard-code the source path — only reinstall needs the new location.

Rationale and research: [`PROBLEM.md`](utils/fmt-md/PROBLEM.md) / [`PLAN.md`](utils/fmt-md/PLAN.md) / [`research/`](utils/fmt-md/research/). Capability and limits: [`STATUS.md`](utils/fmt-md/STATUS.md). Usage: [`README.md`](utils/fmt-md/README.md) and `fmt-md --help`.

### [`utils/udon/`](utils/udon/)

UDON language / tooling (submodule `v2-io/udon`). See that tree's own README and docs for install and CLI.

## Planned / tracked

- **`mv-src-repo`** — the general `~/src/` project relocator (spec in [`../MIGRATION.md`](../MIGRATION.md) §5): config-driven (safe-sweep repo set, protected paths, Claude memory roots, memorata/relata adapters), dry-run `plan`, journaled actions with inverses, `rollback`, verification. **Build it *after* the first migration is done by hand** — harvest the hand-kept journal in MIGRATION.md into the implementation; shakedown run = the gated `archema-io` → `archema` rename. Ruby, per the script-language convention.
- **Submodules** for other belt citizens (relata, practica, ato, udon, descent, …) as their remotes are confirmed clean.

## Conventions

Ruby for scripts; Rust where the tool is durable content-parsing (see language note in asf's `agents.sop.md`). Each tool gets a `--help` worth reading and refuses destructive work without a journal.

See `notes/TREE-REORG-PLAN.md` for programme tree context.
