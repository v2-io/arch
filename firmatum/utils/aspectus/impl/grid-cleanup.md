# Grid cleanup — implementation plan (2026-08-22, coordinator; code follows design/grid-cleanup.md + design/lattice-2.md, never leads)

Staged so the look never moves while the code does, and so each slice is one agent's job with one review seam.

1. **Prefactor, output-identical** — restructure `columns.rs` (and the render half of `n_level.rs`) into the pipeline the design names: ready-facts (per-node formatters → `{text, place, office, marks}`) → cells (a row grammar with named slots, in a fixed order) → rows (a node may own sub-rows; none yet) → paint (stops, right edge, color). **The rendered bytes do not change** in this slice; snapshot tests over fixture trees pin that (and `~/src/arch/asf` / this crate dogfooded by diff). `facts.rs` becomes the inventory lattice-2 describes (fact, derived-from, position, display, sort, width, formats) and stops lying about `place`. No new facts, no new glyphs.
2. **Shipped defaults** (design/defaults.md) — embedded `defaults.toml`; `[layout]` lists feed the cell order; maps as data; `config` prints the effective grid.
3. **Count cell** — far-right `lines`/`bytes` rendered in the decided 12-cell grammar (design/grid-cleanup.md §The count cell). Mass lines join the `lines` column (Joseph's inbox ask). The census's *internal* form is **not** changed here (subgroup-subject form undecided).
4. **Filetype** (design/filetype.md) — `kind.rs` → `filetype.rs`; consumers keep today's output until a census grain is asked for.
5. **git-status far-left glyph** — first far-left tenant via `[layout]`; `⊘` moves into it; the marks-column `⊘` retires.

Not in this plan (undecided): subgroup-subject form · census delimiter · mass/size placement beyond step 3 · `[denied]` glyph · empty-dir mark · the header designation · sub-row spill rules · `-q`.
