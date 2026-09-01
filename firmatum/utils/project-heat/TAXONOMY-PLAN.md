# Plan: multi-axis taxonomy, replacing `disposition`

*Drafted 2026-09-01 (Claude, session with Joseph) from the TODO block at the top of `census-details.txt`; revised same day after Joseph's in-session answers (undeclared default, group shorthand, edges-as-census-file, active-ref mapping). Status: **ratified in the large** — the axis inventory is his; the provenance carves and §2/§7 shapes got his read; the phasing is trusted-not-reviewed (his words), and he notes the current disposition values were evolved by a grok agent, not core principles — so mapping fidelity is best-effort, not sacred.*

## What problem this solves

`disposition` is one field carrying at least three questions (is it alive? what kind of life? why is it here?), and it's already sprouting compound values (`active-ref`, `active-fork`, `inert-exp`) — each new combination would need a new token. The census header's seven axes let each question have its own answer. A fourth question surfaced in-session 2026-09-01 (see `inbox.md`): lineage — which projects fed which — is currently invisible; it rides along here as §7 because the migration touches the same file format.

## 1. The axes, with provenance carves

The key design question per axis is **who answers it** — detection, census defaults, or human declaration. Getting this right keeps `--refresh` honest (derived may be rewritten; frozen never).

| axis | values | provenance | how |
|---|---|---|---|
| `git-rel` | gitroot · subgit · submodule · no-git · group | **detected** | Already computed as booleans (`git`/`subgit`/`submodule`); collapse to one enum. New: `group` = not itself git-rooted but has census children that are (arch pre-submodule style dirs, org clone dirs). |
| `git-vis` | local · private · public · external · group | **detected default, freezable** | no remote → `local`; remote org in `internal-git-orgs` (the commented census line — uncomment it) → `private` as the safe default (public needs `gh`/network to verify — don't; freeze `public` per-project when known); other org → `external`; group dirs → `group`. |
| `proj-rel` | root · mid · leaf · isolated | **derived** | Already computed (pass2 parent detection + has-children); pure rename/derivation, never stored frozen. |
| `lifecycle` | nascent · experiment · pre-poc · pre-mvp · pre-release · ongoing · maintenance · reference | **declared** (census-blob defaults + frozen overrides) | Human judgment. Census defaults by glob (e.g. `src/_exp/**` → experiment, `src-ext/**` → reference). The three `pre-*` stages (added 2026-09-01) mark active development toward a named milestone, and a project can re-enter them several times — lifecycle is not a one-way ladder. `reference` = could definitely continue to influence things; whether it's *used* is the local-use axis's business, not lifecycle's. |
| `activity` | active · paused · watch · archived | **declared** (same mechanism) | Intent, not measurement — last-action can *suggest* ("active but idle 60d") but never rewrite; auto-demoting activity would make the census lie about aspiration, which is what this axis records ("aspires to regular active pushing"). |
| `local-use` | undeclared · pending · pilot · transition · secondary · primary | **declared** | `undeclared` is the non-frozen default (Joseph 2026-09-01) — the field is present and honest about not having been decided, distinct from a decided value. |
| `public-use` | undeclared · unlikely · peers-only · undecided · planned · preprint · submitted · accepted · published · early · broad | **declared** | Same `undeclared` default. Papers/books stages ride the same field. (`undecided` = considered and genuinely open; `undeclared` = never considered.) |

**Value-collision check** (matters for the `set` bare-token shorthand): every value is unique to its axis except `group` (git-rel ∩ git-vis) — and that collision is meaningful, not accidental: a group usually *is* both. Decided (Joseph 2026-09-01): bare `group` sets **both** axes; `rel-group` / `vis-group` set them independently. All other bare tokens resolve axis-free.

## 2. Mapping the current dispositions

| today | activity | lifecycle | notes |
|---|---|---|---|
| `active` | active | ongoing | the default blob |
| `active-maintenance` | active | maintenance | |
| `active-exp` | active | experiment | |
| `active-ref` | active | reference | + usually `git-vis=external` (detected). Decided: active and reference are separate axes now, so this compound dissolves cleanly. |
| `active-fork` | active | ongoing | + `git-vis=external` (grok-build fork); the "fork" fact is git-vis, not activity |
| `inactive` | paused | *(keep existing/unset)* | |
| `inert` | archived | *(unset)* | |
| `inert-exp` | archived | experiment | |
| `ref-only` | watch | reference | usually also `git-vis=external` (detected) |
| `historical` | archived | *(unset)* | |

Frozen `disposition` → the mapped fields land **frozen**; blob-derived → mapped fields stay derived. **Heatmap membership** becomes `activity == active` (matches today's `active*` exactly under this table; `watch` rows stay off the heatmap, as `ref-only` does today). Table `DISP` column → two columns or one composite; suggestion: show `activity` (the question the table answers) with lifecycle appended only when set and ≠ ongoing (`paused·exp`); full facts in `--format=json` regardless.

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

## 7. Edges — `census-edges.txt` (decided in shape, Joseph 2026-09-01)

Lineage/relations do **not** go through `set` or a CLI edge verb, and do not live per-project in `.practica`. Joseph's vote: a hand-typeable file beside the census, `census-edges.txt`, same comment/blank conventions, maybe globs. Line shape (his sketch, generalized edges, not just lineage):

```
# FROM TO RELATION [INVERSE-RELATION]
#   FROM/TO: comma-separated names (globs allowed, expanding against census names)
#   RELATION/INVERSE: comma-lists pairing positionally with FROM when FROM is a list
#   INVERSE omitted → derived from the known-inverses table; '-' → explicitly none
rowan,verisectorium arch/udon influence,waiting-for influenced-by,-
verisectorium arch/udon depends-on-critically
chiridion arch/udon influence
```

- **Either side may declare** — `influenced-by` on the successor is as valid as `influence` on the predecessor; the reader collapses both-side declarations into one directional edge set (dedupe on from/to/relation after inverse-normalization).
- **Relation vocabulary is open**, not an enum — `influence`, `influenced-by`, `depends-on-critically`, `waiting-for`, `predecessor-of`, … A small growable known-inverses table (`influence ↔ influenced-by`, `predecessor-of ↔ successor-of`, `depends-on-critically ↔ critically-depended-on-by`) drives derivation; an unknown relation with no stated inverse simply has none derived (absent, never faked).
- **Resurrectability** is not an edge kind and not a new field — `activity=paused` + `lifecycle=reference` is the typical combination for internal projects that are resurrectable (Joseph 2026-09-01). The axes stay independent: paused already kind of implies not-in-use (or it would be maintenance), and usage proper is the local-use axis's business. (Refines the earlier in-session carve, where it rode the edge type; the inbox.md entry stands as the original record.)
- `.practica`'s `related` field stays as-is (untouched, still optional free-form) until the edges file exists; then JSON output exposes each project's collapsed in/out edges computed from the file.
- Rendering (a lineage thread from a fading heatmap row to its successor) stays future work; this format just mustn't block it.

## 8. Display: verbosity ladder + 7-axis glyph block (Joseph, 2026-09-01)

The current table columns were guessed by the grok agent, never iterated. New model — `-v` becomes counting verbosity (we now have a story for it, so form-shared-flags is satisfied):

- *(no flag)* — just names (full `name`), like today's `-n`.
- `-v` — a **7-character axis block before the name** (one column per axis, fixed order: git-rel · git-vis · lifecycle · activity · proj-rel · local-use · public-use; position = which axis, glyph = value, `ls -l`-permission-block style; `·` = undeclared/unset) plus the directory.
- `-vv` — adds `desc`.
- `-vvv`+ — adds the rest (last-action, git remote, …); exact allocation is implementation latitude.

Glyph vocabulary is **provisional v0** — taught in `--help` (law channel), expected to be iterated by Joseph on sight before it ossifies. Old `-n`/`-p` stay for scripting. Sorting flags unchanged. Longer term Joseph expects the primary output to become udon; this ladder is the interim shape.

## 9. Edges file — decided

`census-edges.txt` (created 2026-09-01, beside the census): carries `inverse: A <-> B` declarations and `alias: NAME -> REL` shorthands in-file (Joseph's spellings: predecessor-of↔successor-of, depends-on-critically↔critically-needed-by, informed↔was-influenced-by, influences↔is-influenced-by; aliases requires/informs/became), plus edge lines per §7. Seeded with the session's known edges.

## 10. Go decision (Joseph, 2026-09-01)

Implement now, in one pass — *"The project is only 1 day old and only used by me… doesn't need to be that safe."* The five phases collapse into a single working landing: detection axes + declared axes + census `default` lines + consumer flip + `.practica` migration on first run + **immediate** disposition switch (old tokens translate on `set` and print what they applied; no deprecation window).
