# Ghostty-face packed / roomy rasters

*2026-08-24. Second-tier capture. Do not overwrite `bmp-metrics.tsv` (that is unconstrained CoreText cascade — wrong face/size for “here”).*

## Goal

PNGs and stats that approximate **what Ghostty paints**, not native fallback outlines.

Joseph’s observation: many `wcwidth=1` glyphs still **overflow** after Ghostty’s resize (aspect preserved; size fitted; **placement not fitted**). Packed vs with space on the right only changes width a little — not “twice as wide.”

## Algorithm (per measured TSV row)

1. **Face** = `ghostty` column from `bmp-metrics.tsv` (`+show-face`).
   - If that name is the primary family (VictorMono Nerd Font Mono), use the configured style (`VictorMono NFM SemiBold`).
   - Skip `ghostty-sprite` (no font paint), `not-found`, LastResort.
2. Cell grid still from the **primary** font at `font-size × 4` (same 3×2 canvas, red guides, origin at left of origin cell, primary baseline).
3. Build a CTLine at full raster size in the Ghostty face. Ink bbox = `CTLineGetImageBounds`.
4. **Ghostty constraint** — clone `~/src-ext/ghostty` @ `600a86dcfd70ac6a16db199367ee6aad337b99cc`
   - `src/renderer/generic.zig` `addGlyph` ~3302: `getConstraint(cp) orelse (isSymbol(cp) ? .{ .size = .fit } : .none)` — **not** nerd `center1`. Number Forms (`Nl`/`No`, U+2150–218B) are **not** `isSymbol`, so they stay native size (Ghostty does not shrink `ⅸ`).
   - `isSymbol`: Co/PUA, or blocks Arrows / Dingbats / Emoticons / Miscellaneous Symbols / Enclosed Alphanumerics (+ supplement) / Miscellaneous Symbols and Pictographs / Transport and Map Symbols (`uucode_config.zig` IsSymbolComponent ~125).
   - `src/renderer/cell.zig` `constraintWidth` ~250: 1 cell, or 2 if next is space/empty (**only consulted for symbols**).
   - `src/font/Glyph.zig` `constrainInner` ~216: `.fit` scales about bbox **center**, then **clamp** (`.none` align — not vertical centering). `.none` size leaves scale=1 and bearings untouched.
   - packed budget_w = 1 cell; roomy = 2 **only if isSymbol** (constraintWidth). Non-symbols: both use wcwidth, same paint, same overflow.
   - After constrain, CoreText `x += (cell_width - face_width)/2` (`coretext.zig` ~371). Cell is `round(face_width)` integer pixels. Bounding rects, not image bounds.
   - `measure-ghostty` re-queries `+show-face` each run. JetBrains Mono loads from Ghostty’s embedded TTF (not AppKit; `CTFontCreateWithName` would silently become Helvetica).
5. Draw at `raster_pt * s` with origin placed so scaled ink lands at the constrained `(x,y)`.
6. Overflow / ink-width measured **after** paint on the constraint width (1 packed, 2 roomy). Density is **optical coverage** in the occupancy rect (wcwidth cells): `(255-min(R,G,B))/255`, guide-red zeroed. Ghostty sprite shades are alpha8 (`Shade` light=0x40 / medium=0x80 / dark=0xC0) so ░▒▓ are ~0.25/0.50/0.75, not binary-full.

`measure-ghostty` always finishes with `measure-sprites` then `recount-density` (unless `--no-sprites`).

## Outputs

- `.rasters/4x/packed/HH/HHHH.png`
- `.rasters/4x/roomy/HH/HHHH.png`
- `bmp-metrics-ghostty.tsv` — identity columns + per-context `scale`, `class`, `density`, `overflow_cells`, `ink_width_cells`, raster path. `cells` remains `wcwidth`.

## Script

`measure-ghostty` — reads existing TSV `ghostty` column; does not re-run `+show-face` unless a face is empty.

## Later

- **Vertical origin after shrink:** we scale from the baseline (same as Latin), so circled/symbol glyphs sit low in the cell. Next pass: scale about the ink-bbox center (or cell vertical mid) so packed `①` matches Ghostty’s vertical placement. Don’t clip overflow while doing that.
- Nerd `adjust-icon-height` if packed still disagrees with vim. Sprites stay tagged, not faked.
