# Shipped defaults — finish note (2026-08-22, step 2 of impl/grid-cleanup.md)

*Source: `defaults.toml` (embedded), `src/config.rs` (parser + stack + `config` show), maps consumed in `src/furniture.rs` / `src/kind.rs` / `src/important.rs`. Tests: `tests/config_show.rs`, `tests/furniture.rs` table overlay, parser unit tests in `config.rs`. Snapshots in `tests/grid_snapshot.rs` unchanged.*

## What the binary does now

The shipped file is the `defaults` layer. `include_str!("../defaults.toml")`, parsed at startup, no files required anywhere. `aspectus config defaults` prints it verbatim to stdout, exit 0 (no footer — stdout is the file).

`aspectus config` still prints layers and `won:`, and now also:

- the effective `[layout]` lists, with per-list source, and `(unbuilt position)` on `far-left` and `supplement`
- the effective furniture / kinds / important maps, every row tagged `default` / `user-home` / `caller` / `env` / `global` / `flags`

The furniture map, the kinds map, and the important set no longer live as Rust tables. `default_rules()` / `kind::Map::shipped()` / `important::Set::shipped()` read the parsed file. Legacy comma keys (`furniture = "…"`, `kinds = "…"`, `important = "…"`) still overlay.

Column state is derived from `[layout]` + the `quiet` list (`bytes` in quiet → `columns.size = quiet`, `lines` in far-right → `columns.line-count = on`, unlisted sha facts → off). That derivation matches today's defaults, so the rendered look does not move. `columns.X = on/off/quiet` still wins for this release; stderr names `[layout]`.

Help documents `config defaults` and `[layout]`.

## Calls made (all reversible)

- **Parser is a TOML subset, not a TOML crate.** `[table]` headers, dotted keys, arrays of strings, quoted keys, `#` comments outside strings. That is what the file uses and no more.
- **`important` moved above the first `[table]` in `defaults.toml`.** After `[format]` it is `format.important` under real TOML (and under this parser). The design-doc *shape* example still shows it after the tables — flagged, not edited (design/ is the coordinator's).
- **Bools `true`/`false` and `on`/`off` both accepted** (`one-fs`, `globify`, `readme-title`, `dotfiles-first`). The file uses TOML bools; the rest of the stack still speaks on/off.
- **`format.bytes` aliases `format.size`; `format.lines` aliases `format.line-count`.** The file uses the lattice names; today's code and tests still ask the old keys. Higher layer wins when both names are set.
- **Serialization `format = text` is not in the file** (`[format]` is fact-formats). It stays an implicit code default so the two `format`s don't collide.
- **Equal values in a file layer do not promote the source.** A copied `defaults.toml` restating `sort = "recency"` stays `(defaults)`, so it does not lift mtime the way `--sort recency` does. Needed for round-trip identity (design subfeature 2) without breaking `tests/sort.rs::explicit_sort_key_implies_its_column`.
- **Renderer does not yet restable paint from the lists.** Far-right order and near-right part order stay the code order. The shipped lists don't disagree with that order among *built* positions, so the look is unchanged. A user list that would move `heat` into `far-left` is parsed and shown as `(unbuilt position)`; heat still paints far-right (step 5 is the paint).
- **Kinds this slice: `text/*`, `data/*`, `log/*`, `image/svg` → text; bare `text`/`binary` still work; everything else → binary.** Full ladder is design/filetype.md.

## Awkward grammar (the feedback asked for)

These are the places the file's spelling was more interesting than the parser:

1. **Table scope is sticky.** A key after `[format]` belongs to `[format]`. `important = [...]` had to live with the other top-level keys. Quoted keys (`".git"`, `"target/"`, `"7z"`) are required TOML for leading dots, slashes, and a leading digit — that's fine, just dense in `[furniture]`/`[kinds]`.
2. **Two `format`s.** Top-level `format = "text"|"json"` vs `[format] mtime = "relative"`. The file only has the table; the serialization key stayed in code. Worth a one-line note in the file later if it keeps biting.
3. **`true`/`false` vs `on`/`off`.** Same keys, two vocabularies. Accepting both is cheap; picking one in the file would be cheaper to read.
4. **`[layout].quiet` is not a position.** It sits next to `far-left` / `far-right` and the comment says "listed-or-not". That is load-bearing: quiet facts still claim today's cells even when they aren't in a built-position list. Membership-strict (`a fact in no list renders nothing`) would have dropped permissions/owner/filekind from the look on surprise.
5. **Drop spelling is three things:** `"PATTERN" = "!"` (drop a shipped row), `":omit"` (empty kinds + omit fate), and the legacy `!PATTERN` comma form. All three work; they don't look like one grammar.
6. **Inline comments after arrays** carry quoted English (`# Joseph, … "git-status and time…"`). The parser has to strip `#` only outside strings; a naive `split('#')` would have eaten the array.

## Not in this slice (flagged, not acted on)

- **Subfeature 4 — moving `heat` to far-left actually moves the column.** Far-left has no paint (step 5). Honoring it would either invent the paint or drop heat from the look. Config tells the truth (`(unbuilt position)`); the renderer keeps today's far-right cell.
- **Subfeature 5 — membership = shown, strictly.** Applied to the *sha* facts (unlisted → off, already off). Not applied to quiet-only facts, because that would hide surprise cells. The file's own comment ("listed-or-not") is the reading that keeps the look.
- **Layout list order is not yet paint order.** Reordering `far-right = ["heat", "lines"]` is parsed and printed; the cells stay code order so this slice cannot move the look by accident.
- **`jsonl` (and a few other suffixes in the file but not in the old Rust tables) now count as text.** Snapshots don't have them. An A/B on a tree full of `.jsonl` would show line counts that the previous binary sniffed.

## Verified

264 tests green (254 before this slice, +10: parser units, `config defaults`, embedded-layer scrape, old-key warning, furniture table overlay, round-trip look). Grid snapshots byte-identical. Round-trip: `aspectus config defaults > f; aspectus --config f` matches no-config below the stamp.
