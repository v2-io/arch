# Color ramps — provisional / exploratory

*Opened 2026-08-23 at Joseph's ask ("drop those notes into a design file… give it a row in the outline (provisional / exploratory)"). Nothing here is decided except the law it sits under: color is a **redundant overlay for a human on a TTY, never the sole carrier** ([[../../../principles/src/form-agentic-eyes|form-agentic-eyes]] channels table; [[color|Color]]). Both 2026-08-22 cold readers ran redirected and saw no color at all — every ramp must duplicate something the glyphs and positions already say. Joseph: "I have no intrinsic motivation to stay zero-dependency — it's just incidental so far."*

## What exists

`src/color.rs`: two hand-rolled escapes — bold-blue directories (`ESC[01;34m`) and dim (`ESC[2m`) — under `--color=auto|always|never` (auto = stdout is a TTY). git-heat's viridis ramp lives in its HTML viewer (CSS), not in terminal codes; nothing to reuse there.

## The terminal side is small

- **256-color:** `ESC[38;5;Nm` (foreground) / `ESC[48;5;Nm`; the 6×6×6 cube is `16 + 36r + 6g + b` for r,g,b ∈ 0..5, plus a 24-step gray ramp at 232–255.
- **Truecolor:** `ESC[38;2;r;g;bm`.
- **Detection:** honor `NO_COLOR` (the convention — any value disables); `COLORTERM=truecolor|24bit` ⇒ truecolor; `TERM` ending `-256color` ⇒ 256; else 16; and always the existing stdout-TTY gate. `CLICOLOR_FORCE` is the other common override.
- **Palettes are tables:** a viridis/magma/turbo ramp is ~16 RGB stops with linear interpolation; quantizing to the cube for the 256 fallback is a few lines.

## If we take a crate

| crate | what it buys | cost |
|---|---|---|
| `owo-colors` | 256 + truecolor API, `no_std`, itself zero-dep; no detection | tiny |
| `anstyle` + `anstream` | clap's family — capability detection and **automatic stripping on non-TTY**, which is exactly the `--color=auto` law | small; well maintained |
| `termcolor` | cross-platform incl. Windows console | small |
| `supports-color` | detection only (`NO_COLOR`, `COLORTERM`, CI) | tiny |
| `colorgrad` | named scientific palettes (viridis, inferno, turbo, …) and gradients | medium; saves real work only if several ramps are wanted |
| `ratatui`'s color types | — | drags a TUI crate; refused on principle |

Leaning: hand-rolled truecolor with a 256 fallback + `NO_COLOR` covers every human terminal in use here at ~150 lines; `anstream` is the one crate worth taking if detection/stripping ever grows past one gate.

## Where color would pay (each duplicating a glyph/position that already carries it)

- **Heat** — the density block (`░▒▓█`, far-left, decided 2026-08-23) tinted by the same grade; git-heat's proven affordance at zero glyph cost.
- **Age** — a second ramp or the same one on the age cell.
- **Kind tint** on the name (dirs already blue; `doc`/`image`/`exe` as tints).
- **Dim** on gitignored (shipped) and on the headings line (shipped).

## Shape when designed

A `[color]` section in `defaults.toml`: per-fact ramp name or off, `scheme = …` for named schemes, `NO_COLOR` honored above all. Colorschemes are Joseph's ("I'll worry about some colorschemes later"). Keep it off the critical path until the glyph questions settle — a ramp tempts the eye to stop reading the number, and the agent reader never sees it.
