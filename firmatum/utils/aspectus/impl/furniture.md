# furniture — finish note

*Landed. Source: `src/furniture.rs` (map, glob matcher, config parsing), wired in `src/n_level.rs::gather_at`. Tests: `tests/furniture.rs`.*

The map is glob → kinds + fate (`hide` / `omit`; `mark` rows are the Labels feed, see `labels.md`). First match wins; a trailing `/` restricts a pattern to directories; `*` and `?` only — no regex engine. Config key `furniture` on the caller stack (`ASPECTUS_FURNITURE` env too): `PATTERN[:KINDS[:FATE]]`, comma-separated, kinds `+`-separated; config rules match before the defaults; `!PATTERN` drops a default row.

Partition happens right after `enumerate()`, before any census or stat — so hidden names never join a census silently (the kind spot is what says they are here), never spend walk budget, and the mechanism works identically at the depth cutoff. `--explain-budget` confesses the total hidden count on stderr.

`--show-all` neutralizes every hide/omit; `--inspect KIND` (repeatable) neutralizes one kind's rows; either way the walk enters what it lists, and the kind claims stay printed (they are still true).

**The `build` vs `rust` question (impl/parent-state-not-children.md) — decided:** `target/` stays `build`. The name alone claims build output, not an ecosystem — maven writes `target/` too, and a label is a claim, not a guess. Where the name *is* an unambiguous ecosystem claim the rule claims both: `__pycache__/` → `build`+`python`, `node_modules/` → `build`+`js`, `.ruby-lsp/` → `build`+`ruby`. `rust` arrives with `Cargo.toml` (a mark row).

Also decided (was open in the outline): `.github` is its own kind (`github`), not folded into `git` — it has its own plugin and its own facts.

**Alignment (steward decision, arrived mid-landing):** all non-name material — censuses, facets, the kind spot, look-marks — lands at a computed pseudo-tab-stop per `design/columns.md`: max name column in the look + 2, capped at 48 (`n_level.rs` `STOP_CAP`), a pure function of tree content, never terminal width; an over-cap name goes ragged on its own line only. The machinery (`Row` / `paint` in `src/n_level.rs`) is minimal on purpose — the Columns row owns the full mechanism when it lands and should absorb this.
