# Columns

I can see which facts a line may carry, which of them this look is showing, and how to ask for more. The [[aspect-lattice|Aspect lattice]] is the inventory; this row is the mechanism that turns a lattice row's `column` / `default` / `flag` / `format` cells into an actual line. Quiet-columns, git-on-the-parent, line counts, and heat *add facts to the set* — they do not invent a second mechanism.

Three duties, in order:

1. **Selection.** Which facts are in this look. Resolved through the [[config|Config]] caller stack: the lattice's built-in `ON`/`OFF`/`QUIET` marks are the `defaults` layer; config and flags override per fact. A fact whose lattice `flag` is `Y` gets its own flag (`--size`); `COMPOSE` facts are reachable only through the shared ask; `NO` facts are refused a flag (the refusal names the config path instead).
2. **Placement.** `column` = a column beside the name; `INFO` = hangs to the right of the name (symlink target, furniture, censuses, prior name). This row governs columns; INFO placement is each fact's own story. A fact never migrates between the two by option — the lattice cell is the law.
3. **Format.** Each shown fact renders in its lattice `format` options, starred default, overridable per fact through the same stack (`format.size = bytes`). JSON ignores text formats (size is always bytes there — lattice `unique`).

## Discoverability

"Which facts *may* a line carry, and how do I ask" is itself part of the story. `aspectus config` already shows what won; the fact inventory with current state (`ON`/`OFF`/`QUIET`, format in effect) belongs on that surface (or a sibling of it), on stdout, so an agent can learn the asks without reading source. Help teaches the shape once; it does not enumerate every fact.

## Line budget

A line with more columns is still **one line**. `--lines` counts lines, not width; turning a column on does not change the allocator's arithmetic. (Whether width ever becomes a second budget is not claimed here — no office for it exists in the lattice. If it is ever wanted, it is a new office, not a reinterpretation of `--lines`.)

## Determinism (binding, from the outline working notes)

Same tree + same caller state ⇒ byte-identical look. Consequences this row must hold:

- Column selection and formats resolve from the caller stack only — never from terminal width, screen, or the place. Width-adaptive column dropping would make two aspecta undiffable; it is refused.
- **Alignment is the trap.** If columns pad to align across lines, one line's value (or a quiet fact speaking on one line) changes the *bytes of other lines*, and a small tree change diffs as noise. The default leaning: pad within the look for readability is tolerable only if we accept that cost knowingly — see Open. Whatever is chosen, it is chosen once, deterministically.
- A `QUIET` fact that has nothing to say prints nothing — no reserved gap, no placeholder. When quiet speaks is [[quiet-columns|Quiet columns]]' law, not this row's.

## Foundations (clauses)

| Clause | Where |
|---|---|
| Codes, counts, paths — low-ambiguity channel | [[../../../principles/influx/tools-are-observation-infrastructure\|Observation]] |
| A fact with nothing to say prints nothing | [[../../../principles/influx/quick-tooling-conventions\|Silence]] |
| Deterministic ordering/rendering in machine mode | [[../../../principles/influx/cli-conventions/ai-agent-considerations\|Agent]] |
| Config is the first arbitrator of the lattice | [[config\|Config]] |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Defaults from lattice | With no config, exactly the lattice `ON` facts show; `OFF` facts do not. | Fresh home: name, filekind info, child-count, git letter behavior per lattice; no size, no mtime. |
| 2 | Per-fact override | Config can turn a fact `ON`/`OFF`/`QUIET`; a higher stack layer wins. | `columns.size = on` in user-home shows size; env `ASPECTUS_COLUMNS_SIZE=off` wins over it. |
| 3 | No own fact-flags | No fact has its own flag (`COMPOSE` and `NO` alike); trying `--size` / `--owner` refuses naming the config path (or "not built yet"). | `aspectus --owner` → exit 2, refusal mentions `columns.owner`. |
| 4 | Format override | `format.size = bytes` renders bytes; default renders human. | Fixture with a known file size. |
| 5 | One line | Any column set yields the same *tree* line count for the same tree and budget. (Amended with the simple-header decision, 2026-08-14: the header's root-facts line exists only when the root has facts to say, so turning a fact on can add that one header line — the tree's shape and the allocator's arithmetic stay untouched.) | Same fixture, `--lines 20`, with and without size: identical tree shape, differing only within lines. |
| 6 | Determinism | Two runs, same tree, same config: byte-identical stdout, wide or narrow terminal, piped or not (color aside). | Diff two runs under different `COLUMNS`/pty. |
| 7 | Inventory shown | The discoverability surface lists every lattice fact with its current state and format. | Snapshot; adding a fact to the lattice without the surface updating fails the test. |

## Open

- **The compose surface.** Partially resolved by shipping (2026-08-14, Joseph ratifies): `COMPOSE` facts are asked through config keys (`columns.FACT = on/off/quiet`, `format.FACT = …`, env `ASPECTUS_COLUMNS_*`) — no per-fact flags, and no `--columns=…` flag yet. Sort shipped a `--sort KEY` flag; whether a shared compose *flag* grammar ever exists (and re-spells both) is still open.
- ~~**Alignment.**~~ **Decided (steward, 2026-08-14): align.** Joseph's empirical finding: *"all modern agents, including locally run small ones, LOVE column alignment"* — so the non-name material (columns, INFO, furniture facets, censuses, overflow marks) lands at **computed pseudo-tab-stops**: a good column position calculated per look from the content (deterministic — a pure function of tree + caller state, never terminal width), scattered-to-the-right raggedness refused. The diff-noise cost is accepted knowingly: structural diffing (JSON, the after-image) is the real diff channel; the text look optimizes for the reader it has. git-heat's shipped look is the visual prior art: name left, a compact right-aligned fact cluster (`0.50 · 13.6d ago`) at one consistent column — separable at a glance because position carries meaning.
- ~~**Side of the name.**~~ **Shipped: right of the name** (far-right cells past the tab-stop, per design/shorthand.md's placement classes — which supersede this binary; far-left/near-left are declared seams if a fact ever wants the other side).
- ~~**Sorting by a hidden fact**~~ — decided in [[sort|Sort]]: an explicitly asked key implies its column `on` (unless set `off`); the recency default does not.

## Not in this row

When a quiet fact speaks ([[quiet-columns|Quiet columns]]). Order ([[sort|Sort]]). Any individual fact's obtain/cache story (its own row). INFO furniture content ([[furniture|Furniture]]). A width budget.
