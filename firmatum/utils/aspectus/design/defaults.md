# Shipped defaults — the grid, stated as data

*Opened 2026-08-22 (Joseph: "I think we need to ship with a default config… the default config should have some of the columns we've been playing with, like git-status and time and size/mass on the far left side"). A **prefactor** row: it puts in place the declarative home that [[grid-cleanup|Grid cleanup]] and [[lattice-2|Lattice 2]] need — which facts, where, in what order — so the renderer reads the grid instead of embodying it.*

## Story

A user (or an agent's harness) can **read** the tool's defaults as a file, copy it, delete what they don't want to change, and have a valid config. `aspectus config` shows where every effective value came from; `aspectus config defaults` prints the shipped file verbatim to stdout. The furniture map, the kinds map, and the important-files set are **data in that file**, not Rust tables.

## Decisions (Joseph, 2026-08-22) and leanings (coordinator; correct on contact)

- **Embedded at build time** (`include_str!("../defaults.toml")`, parsed at startup), not installed as a file — zero dependencies stay zero, no install step, no "file stale vs binary" failure class (the stale-install scar). The user copies it out with `aspectus config defaults`. *(Leaning, proposed as the call.)*
- **It carries everything that is a default today** — the `won:` keys and the three maps. What stays in code: the fact inventory itself (which facts exist, their offices — lattice-2's rows), glyph vocabulary (interface, not preference), magic-byte/shebang tables (facts about bytes), parsers and semantics.
- **Layout is stated as ordered lists per position** — membership = shown, order = left-to-right. Positions are lattice-2's: `far-left`, `near-right`, `supplement`, `far-right` (name-location, name-suffix, after-name are structural and not listable). Sorting is a separate key (`sort` orders rows, not columns).
- **`columns.X = on/off` collapses into membership** (off = not listed); **quiet** survives as a per-fact law: a listed fact that speaks only on surprise. Spelling: a `quiet = [...]` list beside the layout lists, plus the existing per-fact sensitivity keys.
- **First far-left tenants** (Joseph): git-status, time (compact age), size/mass — far-left cells are positional (blank when silent) and use compact forms (one-cell glyph; `13.6d`; the count cell without its subject glyph since the heading names it). Exact far-left default membership is his to set in the file; the mechanism ships with the file.
- **Caller overlays** (`caller-<key>.toml`) are the same format; an agent-default overlay (un-hiding `.claude`, the harness-cap budget) can ship later as data.

## The file (shape — the shipped one is the primary; this is the grammar)

```toml
# aspectus defaults — copy to ~/.config/aspectus/aspectus.toml and keep only what you change.
depth = 2
lines = 80
walk = 10000
reads = 67108864
sort = "recency"
recency-source = "mtime"
readme-title = false
globify = true
globify.min = 5
one-fs = true
quiet.sensitivity = 1.0
heat.half-life = 7

[layout]
far-left   = ["git-status", "mtime", "bytes"]     # Joseph's first tenants; compact forms
near-right = ["census", "has", "marks"]
supplement = ["facets"]                            # spills below the row when it doesn't fit
far-right  = ["lines", "heat"]
quiet      = ["mtime", "bytes", "permissions", "owner", "filekind"]

important = ["README*", "AGENTS.md", "CLAUDE.md"]

[format]
mtime = "relative"
bytes = "human"
lines = "physical"
owner = "name"
initial-sha = "short"
latest-sha = "short"

[furniture]            # PATTERN = "KINDS[:FATE]"  — the shipped map (impl/furniture.md table)
".git"  = "git"
"target/" = "build"
"node_modules/" = "build+js"
".DS_Store" = ":omit"
"Cargo.toml" = "rust:mark"
# …

[kinds]                # SUFFIX or NAME = "MAJOR/MINOR" (filetype.md); text|binary still accepted
"md" = "text/markdown"
"udon" = "text/udon"
"png" = "image/png"
# …

```

*(Shape note, from the step-2 landing: TOML table scope is sticky — a bare `important = [...]` after `[format]` would read as `format.important` — so in the shipped file `important` sits with the top-level keys, before any table. The example above is illustrative; `defaults.toml` is the primary.)*

Keys keep today's spellings where they exist (`columns.size` → the `bytes` fact is the one rename, per lattice-2; `columns.*` state keys are accepted for one release and warned as superseded by `[layout]`). The exact shipped contents are the file itself.

## Parser

`config.rs` parses a TOML subset today (bare `key = value`, strings, ints, floats, bools). This row extends it to what the file needs and no more: `[table]` headers, dotted keys, **arrays of strings**, quoted keys. Still zero dependencies. (UDON emit/parse is its own later row; TOML is what config already speaks and what users expect to copy.)

## `aspectus config` after this row

- Layers as today, with `defaults  present  built-in (defaults.toml, embedded)`.
- `won:` as today, every value with its source.
- **The effective layout** (each position's list, with per-entry source when an overlay changed it).
- **The effective furniture / kinds / important maps**, rows marked `default` / `user-home` / `caller` / `env` — the question that was open since 2026-08-15.
- `aspectus config defaults` → the embedded file verbatim, stdout, exit 0.

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Embedded | The binary runs with no files anywhere and has every default. | Clean XDG home: look renders; `config` names the embedded layer. |
| 2 | Round-trip | `aspectus config defaults > f; aspectus --config f` renders byte-identical to no-config. | Diff. |
| 3 | Maps as data | Removing a furniture row in a user file changes the look exactly as `!PATTERN` did; adding one works. | Fixture. |
| 4 | Layout lists | Moving `heat` from far-right to far-left in user config moves the column. | Snapshot. |
| 5 | Membership = shown | A fact absent from every list renders nothing, reserves nothing. | Snapshot. |
| 6 | Quiet list | A fact in `quiet` speaks only on surprise (existing quiet tests hold). | Existing suite. |
| 7 | Effective map printed | `config` shows every furniture row with its source. | Scrape. |
| 8 | Old keys | `columns.size = on` still works for one release, with a stderr note naming `[layout]`. | Fixture. |

## Foundations

[[config|Config]] (the caller stack; nothing from the locus) · [[../../../principles/src/norm-caller-tunes-the-channel|caller-tunes]] · [[../../../principles/src/norm-overlays-are-config|overlays]] · [[lattice-2|Lattice 2]] (positions, facts) · [[grid-cleanup|Grid cleanup]] (the renderer that reads this).

## Open

- Far-left default membership and the compact forms' exact spellings — Joseph sets them in the file.
- Whether `quiet` is a list or stays per-fact keys (`quiet.mtime = true`) — leaning list.
- Name of the embedded file (`defaults.toml` at crate root) — one constant.

## Open — column-set stability across looks (2026-08-22 cold reads, both substrates)

*Why open:* quiet facts add whole columns (`perms`, `mtime`, `bytes`) when any row surprises, so the far-right set differs between two looks with no per-look signal that it changed — Sonnet named it the one usability risk ("silent misreads"); Grok: "the tool grows extra clocks when git is present" (`audit/hallway-2026-08-22.md` #2). The quiet law is right; the *announcement* is missing. Candidates: a fixed column order always (so a column's absence never shifts its neighbors' meaning); a visible mark on the heading of a quiet-spoken column; reserving width for every `[layout]` far-right entry regardless of speakers (costs width on quiet looks). Joseph's trade.
