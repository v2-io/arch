# columns — finish note

*Landed. Source: `src/columns.rs` (selection/format/paint — it absorbed the minimal `Row`/`paint` machinery from `n_level.rs` as the furniture note asked), `src/facts.rs` (the lattice inventory + placement classes + fact-flag refusals). Tests: `tests/columns.rs`.*

**Selection** rides the caller stack as config keys: `columns.size` / `columns.mtime` = `on` / `off` / `quiet` (lattice defaults: both `quiet`), with env spellings `ASPECTUS_COLUMNS_SIZE` etc. No per-fact flags exist (lattice: almost no own flags); trying one (`--size`, `--owner`) is an exit-2 refusal that names the config path, or says "not built yet" for facts whose obtain hasn't landed. **Quiet today = renders nothing unless asked `on`** — the quiet *hook* exists; the surprisal law (cold baseline, warm after-image) is Quiet columns' wave.

**Formats**: `format.size` = `human`* / `bytes`; `format.mtime` = `iso-8601`* / `epoch`. Unknown values are refused with the menu, unbuilt lattice formats (`log`, `rfc-3339`, `pattern`, `signa`) named as such. JSON's always-bytes rule waits on JSON.

**Placement** is now an enum (`facts::Placement`): decoration / far-left / near-left / near-right / far-right / glyph-block, per `design/shorthand.md`. Rendering today: **decoration** (dir `/`, symlink `-> target` fused to the name, `[broken]` when dangling), **near-right** (censuses, facets, kind spot, look-marks at the computed tab-stop), **far-right** (size/mtime cells, right-aligned per column across the look). The other classes are declared seams, no tenants.

**Alignment**: single computed tab-stop (max name column + 2, capped 48) for near-right; far-right cells additionally right-edge-aligned per column, empty cells padded so silent facts don't shift their neighbors, trailing whitespace always trimmed. All pure functions of tree + caller state; `COLUMNS`/pty never consulted (tested). **Not done: per-kind edge offset** (design/shorthand.md, the git-heat pulled-in right edge). The far-right cluster today is left-anchored at the tab-stop, so a naive per-kind stop would put dir/file edges in content-dependent order and muddy the kind channel rather than carry it — it wants the cluster right-anchored first. Deliberately left for the wave that does that (heat/phenom-format is the natural one).

**Discoverability**: `aspectus config` now ends with the fact inventory — every lattice fact the machinery knows, its placement word, current state (from the stack, `unbuilt` when the obtain is missing), the config ask, and formats with the in-effect one starred. Guarded by `tests/columns.rs::config_shows_fact_inventory`.

**Header** (steward decisions, mid-wave): two lines, stamp then root; the stamp line costs 1 of `--lines` (the root line stays inside the tree's budget as before) and `--explain-budget` says so.

**One line stays one line**: turning columns on never changes the line count (tested).

Decided-by-implementation, Joseph ratifies: config-key spelling (`columns.FACT`, `format.FACT`); columns have no compose *flag* (config/env only — `--sort` is the only new flag, see `impl/sort.md`); columns sit right of the name.

## Heading–cluster alignment fix (Wave E, 2026-08-14; steward repro `~/src/arch/vivarium`)

The `score · age` cluster was one right-aligned blob, so a row's score sat
well left of the heading's `heat` word (the heading, being narrower than
the widest cluster, hugged the right edge). Now the cluster aligns as
**two sub-columns inside the same paint pass that positions all cells**:
score right-aligned under `heat`, age right-aligned under `age`, the `·`
at one char position on every row — heading included, since it flows
through the identical split — so heading and values cannot drift apart
structurally. A score-only cell (no mtime) pads the age subfield.
Pinned by `tests/columns.rs::heading_sits_over_the_cluster`.

The second reported defect (a `size` heading over apparent silence)
was confirmed **not a defect**: the heading rule is already
spoke-at-least-once (paint drops columns whose value-width is zero; the
vivarium look's speaker was `DECISIONS.decision-log.udon  543K`, below
the first screen). Pinned anyway by
`quiet_column_without_speakers_has_no_heading`.

Known ragged case, by design: a row whose name passes the tab-stop cap
(`STOP_CAP`) shifts its cells right ("a name past the cap goes ragged on
its own line only") — its `·` is off-grid. Revisit only if the cap law
changes.
