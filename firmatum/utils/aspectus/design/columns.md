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
| 2 | Per-fact override | Config can turn a fact `ON`/`OFF`; a flag overrides config. | `columns.size = on` in user-home shows size; `--no-size` (or the compose equivalent) wins over it. |
| 3 | Own flags per lattice | `--size` exists; a lattice `NO` fact has no flag and the unrecognized-option refusal names the config path. | `aspectus --owner` → exit 2, refusal mentions config. |
| 4 | Format override | `format.size = bytes` renders bytes; default renders human. | Fixture with a known file size. |
| 5 | One line | Any column set yields the same line *count* for the same tree and budget. | Same fixture, `--lines 20`, with and without size: identical shape, differing only within lines. |
| 6 | Determinism | Two runs, same tree, same config: byte-identical stdout, wide or narrow terminal, piped or not (color aside). | Diff two runs under different `COLUMNS`/pty. |
| 7 | Inventory shown | The discoverability surface lists every lattice fact with its current state and format. | Snapshot; adding a fact to the lattice without the surface updating fails the test. |

## Open

- **The compose surface.** Does `--columns=size,mtime` exist, or are `COMPOSE` facts named only in config? (Lattice's own open question.) One decision, shared with [[sort|Sort]] — if a compose grammar exists, the two asks should speak it identically. Not decided; Joseph ratifies.
- ~~**Alignment.**~~ **Decided (steward, 2026-08-14): align.** Joseph's empirical finding: *"all modern agents, including locally run small ones, LOVE column alignment"* — so the non-name material (columns, INFO, furniture facets, censuses, overflow marks) lands at **computed pseudo-tab-stops**: a good column position calculated per look from the content (deterministic — a pure function of tree + caller state, never terminal width), scattered-to-the-right raggedness refused. The diff-noise cost is accepted knowingly: structural diffing (JSON, the after-image) is the real diff channel; the text look optimizes for the reader it has. git-heat's shipped look is the visual prior art: name left, a compact right-aligned fact cluster (`0.50 · 13.6d ago`) at one consistent column — separable at a glance because position carries meaning.
- **Side of the name.** `ls -l` puts columns left of the name; the origin sketches hang everything right. Not decided; one constant.
- **Sorting by a hidden fact** (asked for order by a fact whose column is `OFF`) — does it imply the column `ON`? Lives in [[sort|Sort]]'s Open; noted here because the answer changes selection.

## Not in this row

When a quiet fact speaks ([[quiet-columns|Quiet columns]]). Order ([[sort|Sort]]). Any individual fact's obtain/cache story (its own row). INFO furniture content ([[furniture|Furniture]]). A width budget.
