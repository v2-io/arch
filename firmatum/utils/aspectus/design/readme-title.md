# README title

A directory line can show the name its README gives it, not only the folder name (origin: *"peek at name given in a README"*):

```
├── aspectus/  "ASPECTUS — the look of a locus, …"
```

The folder name is an address; the README title is what the place calls itself. For a cold agent that is orientation at zero reads — the difference between `01-aat-core/` and knowing it is the AAT mathematical core without opening anything.

- **Office:** INFO, hanging on the directory's name (lattice: the important-files row already notes *"READMEs may also lend the dir a title"*). Not a column, not sortable.
- **Which file:** the same config-defined important-files set that [[important-files|Important files]] weights (`README*` first). If several match, the config list's order picks one. This row does not define the set — it borrows it, so the two rows cannot drift apart.
- **What is the title:** first ATX heading (`# …`) if one appears within the peek window; else the first non-empty line. Markdown-ish decoration stripped lightly (leading `#`, surrounding emphasis); no full rendering.
- **The peek is bounded.** Read only the head of the file (a few KB constant), because this fact multiplies across every visible directory. A README whose title starts past the window simply lends no title — absence, never an error.
- **Truthful or silent.** A binary or unreadable README lends nothing. Empty README lends nothing. Nothing prints a placeholder — Silence's law.

## Foundations (clauses)

| Clause | Where |
|---|---|
| A fact with nothing to say prints nothing | [[../../../principles/influx/quick-tooling-conventions\|Silence]] |
| Which files count as READMEs — one config set, shared | [[important-files\|Important files]] |
| Placement/selection mechanics | [[columns\|Columns]] · [[aspect-lattice\|Aspect lattice]] |
| Obtain is a (bounded) read; cacheable by the linecount key when Cache lands | [[cache\|Cache]] (later; works uncached first) |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Heading title | Dir with `README.md` starting `# Rowan` shows `Rowan` on the dir line. | Fixture. |
| 2 | Fallback line | README with no heading: first non-empty line, trimmed. | Fixture. |
| 3 | Nothing to say | Empty / binary / unreadable README, or no README: no title, no placeholder. | Fixtures for each. |
| 4 | Bounded peek | A huge README costs one bounded read; title only from the window. | Large fixture; time/IO assertion or unit test on the reader. |
| 5 | Config set | Renaming the important-files config entry changes which file lends the title. | Override in user-home. |
| 6 | Redundancy guard | Title identical (case/punct-insensitive) to the folder name prints nothing — it would spend glyphs saying nothing. | `rowan/` with `# rowan`. |
| 7 | Determinism | Same tree, same config: byte-identical. | Diff runs. |
| 8 | JSON | The title is a field on the dir node when present ([[json\|JSON]]). | Parse fixture output. |

## Shipped (2026-08-14, Wave E)

Default **OFF with the key working** (`readme-title = on`) so the Open below is ratified by flipping one value. 4 KB peek, ATX-heading-else-first-line, redundancy guard, 60-char cap, truthful-or-silent throughout; the set is borrowed from important-files through one shared method. Rendering landed **near-right, quoted** (the INFO office as this codebase renders it; the columns wave's provisional *decoration* filing would have shredded column alignment — divergence recorded in [[../impl/readme-title|impl/readme-title]] for ratification).

## Open

- **Default state — ON or QUIET/OFF?** The orientation value argues ON; the cost is one bounded read per visible dir-with-README plus line width on every such line. Leaning ON (it is exactly the "what is this place" fact the tool exists for), but this spends default glyphs and the lattice's governing principle makes Joseph the judge of that trade. Ratify.
- **Rendering spelling** — quoted? dimmed? truncation length for long titles (a max-width constant seems needed; where it lands is one constant). Joseph ratifies.
- Whether the *root* line's title also shows (it is the one dir the reader is already inside — arguably still worth it; the Overview invariants line is nearby). One constant.

## Not in this row

Important-files allocator weighting (its own row). Any parsing beyond the head peek (no frontmatter `title:` keys for now — could join later as a config choice). Titles from non-README sources (`Cargo.toml` names etc. — furniture/kind territory).
