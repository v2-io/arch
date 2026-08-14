# Important files

README and the other important files stay visible when the budget is tight (origin: *"Automatic highlighting or early listing of 'important' files -- e.g., README*, and other things defined in config"*). They are still children — ordinary lines in the ordinary order. This row is an allocator **weight** and the definition of the **config set**; it owns no column, no glyph, and no order (the lattice's `important` row: office `(weight)`, no format, no flag; the pipeline row: *"Sort is Sort's; this row does not own order"*).

Why a weight and nothing more (the eyes doc's option walk): the fact serves the calibrating duty — a look that dropped the README has hidden the one file that says what the place *is* — but its presence carries no surprisal glyph-by-glyph. Its whole value is *being there*, so it claims the free channel (survival) and refuses the costly ones. A cold reader cannot tell an important file survived by weight rather than by luck, and does not need to.

## The set

One **ordered** config list of name globs, on the caller stack like everything else ([[config|Config]]):

- Config key: `important` (shipped; spelling Joseph ratifies), a comma-separated list. Shipped semantics follow the furniture grammar: config entries go **in front** of the defaults (they outrank in list order), `!PATTERN` drops a row.
- Built-in default (shipped as the leaning; ratify): `README*` + `AGENTS.md` + `CLAUDE.md`. `README*` is the only entry the origin names; the reader of this tool is usually an agent, which argues for the other two. The seeds' longer list (`Cargo.toml`, `package.json`, …) is agent interpolation — those names already have a home as furniture **mark** rows feeding [[labels|Labels]], which is a different office (kind-claim, not survival).
- **Order matters** because [[readme-title|README title]] borrows this exact set and breaks ties by list order (*"the config list's order picks one"*). Defining the set here, once, is what keeps the two rows from drifting — readme-title says so explicitly.
- Matching is on the basename, per level. A glob set, not regex — same grammar family as the furniture map.

## The weight

Under a tight budget the allocator decides which children at a level get listed; that ranking is **key-within-weights** ([[sort|Sort]] §survival: weight tier first, then the sort key, then name). This row adds a tier:

```
dirs  >  important files  >  plain files      (survival tiers)
key-within-tier, name tiebreak                 (unchanged)
```

- **Survival only, not position.** Display order is untouched: an important file sorts by recency like any sibling. Only the choice of *who falls into the leaf census* changes.
- **Per level, among siblings.** Importance does not force a parent open, widen the walk, add lines to the budget, or lend weight to ancestors. A README inside an unexpanded directory is simply part of that directory's census/mass — rescuing it would be Focus-shaped behavior this row does not have.
- If a level has more important files than surviving slots, they compete among themselves by the same key — and the leftover falls into the typed census like anything else (never a silent cut; [[summarization|Summarization]]).
- Where the lattice's weight-office algebra lands someday (heat · Focus · importance composing — the lattice Open, orient-rank prior art in [[heat|Heat]]), importance joins as a factor; until then the tier is the whole mechanism. Focus, when asked for, outranks it (the caller's explicit ask beats a standing default).

## Foundations (clauses)

| Clause | Where |
|---|---|
| Survival follows key-within-weights | [[sort\|Sort]] |
| Weight is a lattice office, distinct from sort and filter | [[aspect-lattice\|Aspect lattice]] |
| The set rides the caller stack, never the place | [[config\|Config]] |
| Leftover is typed; no silent cut | [[summarization\|Summarization]] |
| The same set lends dir titles | [[readme-title\|README title]] (borrower) |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Survives the squeeze | A level of 30 files, budget for 4: README is listed even when 25 siblings out-recency it; the census absorbs the rest. | Fixture with README mtime oldest. |
| 2 | Order untouched | With budget to spare, README sits wherever recency puts it — no early listing. | Same fixture, `--lines 0`; diff against a no-importance build. |
| 3 | Config set | Adding `DESIGN.md` to `important` in user-home makes it survive; removing `README*` demotes it. | Override fixtures. |
| 4 | No ancestor rescue | An unexpanded dir containing only a README renders exactly as without this row. | Budgeted fixture. |
| 5 | Too many importants | Five importants, three slots: key-within-tier picks; leftover counted in the census. | Fixture with set mtimes. |
| 6 | Determinism | Same tree, same config: byte-identical, budget tight or loose. | Diff runs. |
| 7 | JSON | No `important` field is claimed unless we decide it is a fact worth exporting — see Open. | — |

*Shipped 2026-08-14 (Wave D) — [[../impl/important-files.md|finish note]].*

## Open

- **Default set membership** — shipped `README*` + `AGENTS.md` + `CLAUDE.md` (the leaning). Joseph ratifies.
- **Config key spelling** — shipped `important`. One constant.
- ~~**Does JSON carry `important: true`?**~~ **Shipped: no** — caller-config echo, not a fact of the tree (the JSON look should not vary in *fields* with one caller's weights). Revisit if an agent post-processing JSON asks why a node survived.
- Origin says *"highlighting or early listing"*. Early-listing lost to Sort's law; **highlighting** (a decoration glyph on important lines) is unclaimed — leaning against by default (survival is the visibility; a glyph would spend default width restating config), possible later as a quiet/compose rendering. Ratify the leaning.

## Not in this row

Order ([[sort|Sort]]). Titles from READMEs ([[readme-title|README title]]). Kind claims from well-known names (furniture mark rows → [[labels|Labels]]). Focus weighting ([[focus|Focus]]). The allocator's share arithmetic ([[balanced|Balanced summarization]]).
