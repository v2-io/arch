# Plan: multi-axis taxonomy, replacing `disposition`

*Drafted 2026-09-01 (Claude, session with Joseph) from the TODO block at the top of `census-details.txt`. Status: **proposed** — the axis/value inventory is Joseph's (from that block, verbatim); the provenance carves, mapping table, and phasing below are this plan's contribution and want his read before phase 2. Register: "may want to" throughout; nothing here is decided beyond what the census header already decided.*

## What problem this solves

`disposition` is one field carrying at least three questions (is it alive? what kind of life? why is it here?), and it's already sprouting compound values (`active-ref`, `active-fork`, `inert-exp`) — each new combination would need a new token. The census header's seven axes let each question have its own answer. A fourth question surfaced in-session 2026-09-01 (see `inbox.md`): lineage — which projects fed which — is currently invisible; it rides along here as §7 because the migration touches the same file format.

## 1. The axes, with provenance carves

The key design question per axis is **who answers it** — detection, census defaults, or human declaration. Getting this right keeps `--refresh` honest (derived may be rewritten; frozen never).

| axis | values | provenance | how |
|---|---|---|---|
| `git-rel` | gitroot · subgit · submodule · no-git · group | **detected** | Already computed as booleans (`git`/`subgit`/`submodule`); collapse to one enum. New: `group` = not itself git-rooted but has census children that are (arch pre-submodule style dirs, org clone dirs). |
| `git-vis` | local · private · public · external · group | **detected default, freezable** | no remote → `local`; remote org in `internal-git-orgs` (the commented census line — uncomment it) → `private` as the safe default (public needs `gh`/network to verify — don't; freeze `public` per-project when known); other org → `external`; group dirs → `group`. |
| `proj-rel` | root · mid · leaf · isolated | **derived** | Already computed (pass2 parent detection + has-children); pure rename/derivation, never stored frozen. |
| `lifecycle` | nascent · experiment · ongoing · maintenance · reference | **declared** (census-blob defaults + frozen overrides) | Human judgment. Census defaults by glob (e.g. `src/_exp/**` → experiment, `src-ext/**` → reference). |
| `activity` | active · paused · watch · archived | **declared** (same mechanism) | Intent, not measurement — last-action can *suggest* ("active but idle 60d") but never rewrite; auto-demoting activity would make the census lie about aspiration, which is what this axis records ("aspires to regular active pushing"). |
| `local-use` | pending · pilot · transition · secondary · primary | **declared** | No detection, no census default beyond unset; unset prints nothing (absent, never faked). |
| `public-use` | unlikely · peers-only · undecided · planned · preprint · submitted · accepted · published · early · broad | **declared** | Same. Papers/books axes ride the same field. |

**Value-collision check** (matters for the `set` bare-token shorthand): every value is unique to its axis except `group` (git-rel ∩ git-vis). So `projects set NAME experiment` can keep working axis-free; `group` alone is refused with a menu (`git-rel=group` / `git-vis=group`) — and both are detected anyway, so it should rarely come up.

## 2. Mapping the current dispositions

| today | activity | lifecycle | notes |
|---|---|---|---|
| `active` | active | ongoing | the default blob |
| `active-maintenance` | active | maintenance | |
| `active-exp` | active | experiment | |
| `active-ref` | watch *or* active | reference | few rows; confirm each with Joseph at migration |
| `active-fork` | active | ongoing | + `git-vis=external` (grok-build fork); the "fork" fact is git-vis, not activity |
| `inactive` | paused | *(keep existing/unset)* | |
| `inert` | archived | *(unset)* | |
| `inert-exp` | archived | experiment | |
| `ref-only` | watch | reference | usually also `git-vis=external` (detected) |
| `historical` | archived | *(unset)* | |

Frozen `disposition` → the mapped fields land **frozen**; blob-derived → mapped fields stay derived. **Heatmap membership** becomes `activity == active` (matches today's `active*` exactly under this table, modulo the `active-ref` rows to confirm). Table `DISP` column → two columns or one composite; suggestion: show `activity` (the question the table answers) with lifecycle appended only when set and ≠ ongoing (`paused·exp`); full facts in `--format=json` regardless.

## 3. Census file format

Generalize the disposition blob to `default <axis>=<value>: glob [glob…]` (same first-match-wins, same glob dialect):

```
default activity=watch:      src-ext src-ext/**
default lifecycle=reference: src-ext src-ext/**
default lifecycle=experiment: src/_exp src/_exp/**
default activity=paused:     src/_exp src/_exp/**
default activity=active:     **
internal-git-orgs: v2-io, josephwecker
```

`disposition <value>: globs` keeps parsing during the transition as sugar that expands through §2's table (warn on stderr once per run: teaching, not stdout).

## 4. CLI and `.practica`

- `.practica` gains the seven keys (in `RECORD_KEY_ORDER` after `disposition`); `git-rel`/`proj-rel` replace the four booleans **in output** — read both spellings for one release so stale files parse.
- `projects set NAME lifecycle=experiment activity=paused` — already works structurally; extend `SET_FIELDS` + validation per-axis. Bare token: resolve across all axis vocabularies (unique everywhere but `group`); `disposition` bare tokens (e.g. `set NAME historical`) keep working via §2 during transition.
- `--format=json` carries all axes as plain fields from day one (facts, not affordances — this is where the new taxonomy pays off first, before any display work).

## 5. Phasing (each lands green; commit is the seam)

1. **Additive detection** — compute+store `git-rel`, `git-vis`, `proj-rel` alongside existing fields; JSON exposes them; nothing consumes them yet. Pure additive, zero behavior change.
2. **Declared axes + census `default` lines** — parse the new blob form; store `lifecycle`/`activity` per §2 mapping derived from disposition where not explicitly set. `disposition` still authoritative for consumers.
3. **Consumer flip** — heatmap filter and table column read `activity`/`lifecycle`; `disposition` becomes write-through legacy (setting it maps; reading it reconstructs). Confirm the `active-ref` rows with Joseph here.
4. **Migration sweep** — one `--refresh` run rewrites every `.practica` with mapped fields (frozen-ness preserved per §2); census file rewritten to `default` lines. `disposition` kept in files but no longer written fresh.
5. **Deprecate** — after a comfortable window: drop `disposition` writing entirely; `set NAME <old-token>` prints the mapping it applied (error-as-menu style) and applies it.

Rollback at any phase = revert the commit; `.practica` files never lose information (old fields retained through phase 4).

## 6. Not doing (named so they're decisions)

- **No network** for `git-vis` (no `gh` calls; `public` is a frozen human claim).
- **No auto-demotion of `activity`** from last-action (see §1 — the axis records aspiration; staleness is already visible in LAST and the heatmap).
- **No new sort/group semantics** in this transition — `-c`/`-a`/`-g` untouched.

## 7. Lineage (adjacent, riding the same migration — from `inbox.md` 2026-09-01)

Evolve `related` (unused today) into typed edges, one line per relation in `.practica`:

```json
"related": [
  {"name": "udon", "kind": "influenced"},
  {"name": "verisectorium", "kind": "influenced"}
]
```

Kinds (from Joseph's in-session carve): `predecessor` (direct lineal — successor absorbed it), `influenced` (critical influence; project stayed itself), `influenced-resurrectable` (influence + defunct only because the need lapsed — the ruby→rust cases). Plain strings in `related` keep parsing as `{"kind": "related"}` (untyped). Edges are declared on the *predecessor* side (the dying row knows its heirs; cheap to invert at read time). Rendering (a lineage thread in the heatmap from a fading row to its successor) is future work — the data model just shouldn't block it.

## Open questions for Joseph

1. `active-ref` rows: watch or active, per row?
2. Table `DISP` display: composite `activity·lifecycle` as suggested, or two columns?
3. Should `watch` rows appear on the heatmap (today's `active-ref` does, `ref-only` doesn't — the mapping has to pick)?
4. Lineage kinds' spelling — happy with `predecessor` / `influenced` / `influenced-resurrectable`?
