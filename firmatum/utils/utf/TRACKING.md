# utf raster tracking

Queue of live notes so they don’t vanish under a long pass. Status is present-tense. Ghostty clone `~/src-ext/ghostty` @ `600a86dc`.

## Open

### Match Ghostty placement (centering / rounding / integer cells)

Joseph, 2026-08-24: PNGs look more left-aligned than Ghostty. Later: *“We definitely want to fix to match ghostty as far as the centering/rounding/positioning etc. Bottom line, yes to anything that aligns us to ghostty rendering code.”*

What Ghostty actually does (`src/font/face/coretext.zig` ~340–380, `Metrics.zig` `calc`):

- `face_width` = max visible-ASCII advance (not a special space-only path; for this VictorMono they coincide).
- `cell_width` = `round(face_width)` (integer pixels). Same for height.
- Constrain against **face** span (`face_width` for 1 cell; `face_width + (n-1)*cell_width` for n).
- Then, unless `.stretch`: `x += (cell_width - face_width) / 2` — *“so that they aren't weirdly off to the left.”*
- `cell_baseline` centers the face in the rounded cell; constrain y is `rect.origin.y + cell_baseline`.
- Glyph size for constrain is `CTFontGetBoundingRectsForGlyphs`, not `CTLineGetImageBounds` (AA fringe is not part of the constraint box).

At 4× VictorMono NFM SemiBold the dx is only ≈ −0.05px, so the visible “too left” is mostly: (1) `.fit` clamp to `start_x=0` when fitted width fills the budget — Ghostty does this too; (2) fractional cell edges on our canvas; (3) Helvetica standing in for embedded JetBrains (Control Pictures). Still port the pipeline; don’t invent a second centering rule.

### Overflow: packed 0.0098 vs roomy 0; r-overflow looks broken

Joseph on U+224A `≈` (s=1, visually in-cell): packed overflow `0.0098`, roomy `0.0000`. On U+070B: packed `0.4681` (real), roomy `0` even though packed/roomy *paint* is the same.

Two bugs, not one:

1. **Wrong ruler for roomy on non-symbols.** Ghostty `constraintWidth` returns grid width immediately when `!isSymbol`. We still measured roomy overflow against **2 cells**, so any 1-cell spill became `r overflow 0` and showed as a diff. U+224A and U+070B are not `isSymbol`.
2. **Fractional cell edge.** `origin_x + cell_w` was 55.635; `xs.max()+1 = 56` → `(56-55.635)/37.09 = 0.0098`. One AA pixel straddling a non-integer edge. Ghostty’s cell is an integer pixel run `[origin, origin+cell_w)`.

### Dynamic Ghostty face stack

Joseph: too many typefaces locally; may tighten Ghostty config and rework the table. *“Are we pulling the ghostty stack dynamically? if not, can we?”*

**Until this pass: no.** `+show-face` ran in `measure-glyphs` into `bmp-metrics.tsv`; `measure-ghostty` reused that column. A config change did nothing until a full glyphs rebuild.

**Wanted:** `measure-ghostty` re-queries `+show-face` on each run (still using `bmp-metrics.tsv` for Unicode identity / wcwidth / block). After a font-stack edit, one rerun is enough.

### Embedded JetBrains Mono → Helvetica

Control Pictures (U+2400–U+2426): `+show-face` = `JetBrains Mono`, `used_face` = Helvetica. Ghostty’s fallback is **embedded** (`src/font/embedded.zig` variable / `res/JetBrainsMono*.ttf`), not in AppKit. `NSFont.fontWithName("JetBrains Mono")` is nil; `CTFontCreateWithName` silently substitutes Helvetica. Same hole as U+218A `↊`.

Load the embedded TTF by URL when the name is JetBrains (or when CoreText substitutes Helvetica/Times).

### Nerd `fit_cover1` / `center1`

Still not mimicked. PUA nerd icons may place/scale differently from `.fit`. Later.

## Landed (this directory, recent)

- `isSymbol` gate: Number Forms etc. get `.none` (no shrink). Enclosed Alphanumerics still `.fit`.
- Sprites auto-appended at end of `measure-ghostty`; optical coverage density (░▒▓ ≈ 0.25/0.50/0.75).
- HTML: unlabeled = packed; `r *` only when different; `used face` only when ≠ ghostty.
- Integer cell (`round(face_width)`), bounding-rect constrain, CoreText `dx = (cell−face)/2`, `constraintWidth` for roomy (non-symbols never get 2). Phantom `0.0098` gone; U+070B packed/roomy overflow now match (`0.4595`).
- Live `+show-face` on each `measure-ghostty` run. JetBrains Mono from Ghostty embedded TTF (Control Pictures `used=JetBrainsMono NF Regular`).

## After Joseph changes Ghostty fonts

Rerun `./measure-ghostty` (live `+show-face` + rasters + sprites + density) then `./render-corpus`.
