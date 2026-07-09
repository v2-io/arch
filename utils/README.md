# archema-io/utils

Small, program-level operational tools. **Naming rule (Joseph, 2026-07-10): utilities say what they do, plainly** — Latin names are reserved for fundamental things that deserve a new name (memorata earned its name as a concrete PROPRIUM memory-mechanism experiment; a file-mover does not). So: `mv-src-repo`, not anything grander.

Planned / tracked:

- **`mv-src-repo`** — the general `~/src/` project relocator (spec in [`../MIGRATION.md`](../MIGRATION.md) §5): config-driven (safe-sweep repo set, protected paths, Claude memory roots, memorata/relata adapters), dry-run `plan`, journaled actions with inverses, `rollback`, verification. **Build it *after* the first migration is done by hand** — harvest the hand-kept journal in MIGRATION.md into the implementation; shakedown run = the gated `archema-io` → `archema` rename. Ruby, per the script-language convention.

Conventions: Ruby for scripts; each tool gets a `--help` worth reading and refuses destructive work without a journal.
