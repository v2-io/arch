---
slug: ref-verisectorium-tools
form: reference
status: proposed
max: current
state: [template]
depends: []
---

# Verisectorium tools — what the kit ships and makes available

Owned and maintained upstream by verisectorium (state: `template`); copied at deployment. This file tracks the tools every instance can rely on, honestly split between **built** and **planned** — a planned tool listed as available is a label lying about status, so each entry states which it is. When the upstream roster changes, the change arrives through `sop/influx/` like any template update. Instance-specific tools and their normative usage belong in [[dir-essential-tools]], not here.

## Built (usable today)

| Tool | What it does | Source of truth |
|---|---|---|
| `md-press` | Markdown canonicalizer (estate-wide, on PATH): unwraps hard-wrapped prose, leaves tables/code/frontmatter/math alone; render-equality gate makes clean runs safe by construction. Praxis here: `md-press --check` every touched `.md` before reporting it done. It does not handle udon files — hand-edit those with corresponding care. | `md-press --help` |

## Planned (do not rely on; existence here is a roadmap claim only)

| Tool | Intent | Status |
|---|---|---|
| def→LEXICON assembler | Generates `LEXICON.md` from `def/` entries (the clobber-guarded generated view `LEXICON.md`'s header promises) | Not built; first item queued for `bin/` |
| Observing verbs (`ls-segments`, `check-state`) | M₀ corpus verbs: canon-ordered segment listing with orphans/flags/counts; outline-row ↔ frontmatter agreement check | Spike planned in the theory instance; not built |

## Working Notes

- Upstream maintenance rule: verisectorium updates this file when a tool ships or changes contract; deployments adopt/adapt/decline through their `sop/influx/`. A deployment noticing drift between this file and reality should treat that as feedback worth routing upstream — this file is a proxy, and the tool's own `--help` wins on conflict.
