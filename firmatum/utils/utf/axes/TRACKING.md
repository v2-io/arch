# axes track — present-tense notes

Parallel to parent Ghostty paint work. Do not overwrite `bmp-metrics*.tsv` / `.rasters/` / `corpus/` / `preview.html`.

Join: `hex` (`U+2460`) and `cp`. Packed raster `../.rasters/4x/packed/HH/HHHH.png`. Unicode 14.0.0.

## Landed

- `classify-unicode` — 37 positional-decimal Nd 0–9 sets auto-detected; 63 systems after overlays; 8 gradient families. Hebrew/Greek/Armenian/Georgian are `ciphered-cultural` with `ucd_numeric=no` (UCD does not mark those letters numeric). Suzhou/Hangzhou is `positional-decimal-not-Nd`.
- `measure-coverage` — 113 faces. Enclosed Alphanumerics (160 assigned): Ghostty mix is Arial Unicode MS 109 / Monaspace Xenon 28 / STIX Two Math 23. **Full cmap: JuliaMono, STIX Two Math** (LastResort ignored). Arial Unicode MS 86.9%. Apple Symbols: 0 of this block. Noto Sans Symbols / Symbola / DejaVu **not installed**. JuliaMono is Ghostty-embedded but not selected by `+show-face` for this block.
- `measure-tokens` — o200k_base ≈ o200k_harmony: 5190/61922 single-token, rest UTF-8 byte fallback. Circled numbers mostly *not* single-token (μ 2.54, 5/79 single). Braille: 1/256 single-token (the blank), llama fragments 247/256. Shade ░▒▓ and █ *are* single-token in o200k. Claude Xenova (unofficial) roundtrip-fails 2967 cps including ①, Ⅰ, fullwidth digits. Grok tokenizer unpublished; o200k is proxy.
- `fonts.toml` — extras + inherit from ghostty TSV. Add a face, re-run coverage.

- `analyze-png` — 57151 packed rasters from the **other track** (`.rasters/4x/packed/`, not re-painted). 16×16 occupancy + visual k-means k=64. Pixel-only cluster 22 holds all 256 braille plus ~1.4k other sparse marks (rep U+274C ✕) — theory is what separates them.
- `enrich-shape` — joins that grid to unicode-axes + `bmp-metrics-ghostty.tsv` `packed_density`. Braille 2×4 readout **256/256** matches the code-point bits (Ghostty sprites are the Unicode pattern). Optical density is **not** popcount/8 (μ measured 0.04 vs designed 0.5) — dots don’t fill the cell. Block-fill / shade / quadrants MAE 0.015–0.03 against designed fractions. Hybrid cluster uses designed fill, name-shape tags, theory family, bbox — not pixels alone.
- llama3.2:3b survey: every glyph including ASCII `1` reported UNREADABLE (prompt offered that template). Not a vision measurement; see `data/token-survey.md`.
- `axes/index.html` from `render-axes`.

## Open

- Private-dot faces (`.SF Malayalam`, `.New York`, …) log CoreText substitution to Times; coverage numbers for those names are not trustworthy.
- Noto Color Emoji TTF exists in Ghostty res but CoreText URL load returned missing.
- Re-run `survey-tokens` without the UNREADABLE template; try qwen3:4b / gpt-oss / lumin.
- Parent `render-corpus` could later join `unicode-axes.tsv` / `shape-axes.tsv` on `hex`. Not touching that script while the other agent owns it.

## Enclosed Alphanumerics mix (why)

Ghostty `+show-face` walks the system cascade, not “best coverage of the block”:

- STIX Two Math for ①–⑬ and ⓵–⓾ (double-circled) — math font sitting earlier for those cps
- Monaspace Xenon (user-installed) for ⑭–⑨, parenthesized ⑴–⑼, ⓪, negative circled 11–20, ⓿
- Arial Unicode MS (2002 pan-Unicode dump, Bitstream Cyberbit lineage) for the rest (parenthesized 10–20, dotted ⒈–⒛, circled/parenthesized latin)

Unifying the block without installing anything new: **force JuliaMono** (already in `ghostty/src/font/res/JuliaMono-Regular.ttf`, 160/160) or **STIX Two Math** (already used for 23). Installing Noto Sans Symbols is still a good *designed* alternative; not on disk yet.
