# PRACTICA

Live efforts only. Pipeline: [`ASPECTUS.outline.md`](ASPECTUS.outline.md) Part I. Do not grow a second DAG here.

Start at Part I. Follow Foundation for a why. Seeds: [`IMPLEMENTATION-NOTES.md`](IMPLEMENTATION-NOTES.md), [`FEATURE-PIPELINE.md`](FEATURE-PIPELINE.md). `design/` / `impl/` only when there is something to put in them.

## Now

**The glance** (Part I). Order: [`design/build-order.md`](design/build-order.md).

The name table and the line-sharing can be written together. The walk needs the name table so it does not enter `.git` or `target`. The command that prints needs those three. Do not add config, JSON, git-on-the-line, or extra columns until the default print (a) does not list `.git` or `target` as children and (b) still names what it did not expand.

- [x] IR + text renderer, broot-failure fixture
- [x] parent-as-state + absorb four fates
- [x] sibling-share allocator + `--explain-budget`
- [x] bounded walk (never enter absorbed names)
- [x] first snapshot — md-press shows `src/`; `~/src/arch` does not dump `.git`

## Standing

- An `impl/` finish note is fine. Do not draft a `design/` file to unlock a checkbox.
- `future` stays on the outline.
