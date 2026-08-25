# utf raster tracking

Queue of live notes so they don’t vanish under a long pass. Ghostty clone `~/src-ext/ghostty` @ `600a86dc`.

## Open

### Nerd `fit_cover1` / `center1`

Still not mimicked. `getConstraint(cp)` in `nerd_font_attributes.zig` can `.cover` / `.stretch` / `.fit_cover1` with `center1`. We only do symbol `.fit` or `.none`. PUA nerd icons may place/scale differently. Later.

### After Joseph tightens the Ghostty font stack

Live `+show-face` is in `measure-ghostty` now. One rerun then `./render-corpus`. Not a code gap.

### Placement eyeball

Integer cell, bounding-rect constrain, `(cell_width − face_width)/2` dx, `cell_baseline` / `face_y` are ported. `.fit` flush-left when fitted width fills the budget is Ghostty, not a leftover bug. Worth a look against the terminal; do not invent extra centering.

## Landed

- `isSymbol` gate: Number Forms etc. `.none`. Enclosed Alphanumerics still `.fit`.
- Sprites auto-appended at end of `measure-ghostty`; optical coverage density (░▒▓ ≈ 0.25/0.50/0.75).
- HTML: unlabeled = packed; `r *` only when different; `used-face` only when ≠ ghostty. Room density label is `r-dens`. Face names (`used-face`, `ghostty`) are stacked full-width under their labels.
- Integer cell (`round(face_width)`), bounding-rect constrain, CoreText dx, `constraintWidth` for roomy (non-symbols never get 2). Phantom `0.0098` gone; U+070B packed/roomy overflow match (`0.4595`).
- Live `+show-face` each `measure-ghostty` run. JetBrains Mono from Ghostty embedded TTF (Control Pictures `used=JetBrainsMono NF Regular`).
- `.gitignore`: `.rasters/`, `corpus/`, `__pycache__/`, `*.pyc`.
