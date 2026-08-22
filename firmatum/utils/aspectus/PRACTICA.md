# PRACTICA

Live efforts only. Pipeline: [`ASPECTUS.outline.md`](ASPECTUS.outline.md) Part I. Do not grow a second DAG here.

Start at Part I. Follow Foundation for a why. Seeds: [`IMPLEMENTATION-NOTES.md`](IMPLEMENTATION-NOTES.md), [`FEATURE-PIPELINE.md`](FEATURE-PIPELINE.md). `design/` / `impl/` only when there is something to put in them.

## Session 2026-08-22 — resumed; footer → stderr (v0.1.9); multi-path built (v0.1.10); lattice-2 + filetype + wanderings

Later the same day: Joseph opened the design-first refactor of the facet/furniture/kind/mass/census cluster — [`design/lattice-2.md`](design/lattice-2.md) (his; seven fields, `stat` column ✓/↬/⇥, new positions `name-suffix`/`supplement`, specials rows, the `[project]` FileType sketch = the **cultivated header** idea: one synthesized designation per place from graded evidence, the claims cluster refounded). Two corrections landed with provenance: "determinism law" was agent-authored (terminal width *is* caller state — `form-agentic-eyes` concern 1, `columns.md`); "no retina" withdrawn (agents have spatial awareness — his early experiment; tablecop/udon/aspectus lineage). Thesis paragraph (native-agentic format) now heads `grid-cleanup.md`. New: [`design/filetype.md`](design/filetype.md) (detection ladder with suffix as tie-breaker; major/minor taxonomy; consumers; config) and [`design/wanderings-2026-08-22.md`](design/wanderings-2026-08-22.md) (udon/v2 principles applied; cultivated-header seeds; LuaJIT/WASM plugin leanings; the one-afternoon width×glyph experiment). **Next (Joseph's order, after his break):** the cultivated header brainstorm; subgroup-subject form; then shipped `defaults.toml` with `[layout]` lists (row to add); `--glob` later.

v0.1.9: feedback footer on stderr for every output, JSON `feedback` field dropped (stdout is data; the quiet norm's teaching carve). Inbox (2026-08-15, two entries): (1) Fable instance — `--lines 300 --depth 3 ~/src` cut mid-tree with **exit 1** and a silent tail → **not a bug**: the harness's ~30 KB output cap (42 KB look; cap lands at line 212, `umi/` at 230), exit 0 on the exact command; hazard: a harness cut is silent to the agent — proposal held: header states the look's own size (`300 lines · 42 KB`), or a `--caller` byte/line budget; wants a heat threshold filter (→ Focus/weight or a `--min-heat`, undesigned); (2) Joseph — multiple positional paths `aspectus … asf/{01-aat-core,…}` → routed to [`design/focus.md`](design/focus.md) §Multiple paths (small design: common root, depth from each selected path, siblings fold to remainders). Grid-cleanup discussion continues where PRACTICA's previous section left it.

## Session 2026-08-15 — orientation, inbox routed, usability pass; grid-cleanup design begun · paused Sat evening

**Where we stopped (resume here):** [`design/grid-cleanup.md`](design/grid-cleanup.md) is the live design — pipeline stages, both ready-facts tables (as-shipped, normalized), the count cell (Subject × Unit, 12-cell field, `● □ ▣` / `≈ ≥ ~` / `𝓃 B 𝓁 𝓉` — decided), git-status glyph slot (decided), glyph-pack law + Grok's `glyphs.md` embedded. **Next on the table, in order Joseph set:** the subgroup-subject form (suffix buckets, kinds, patterns, `dirty` — blocks census/has/facet forms and the census delimiter question `⟨ ⟩`); placement of mass lines and size (not decided — do not infer from the tables); `[denied]` and the marks column; empty-dir mark; glob-count place; README-title place; the git facet internals; then rows/sub-rows + far-right ceiling. Also open from the same thread: `aspectus config` should print the effective furniture map (defaults + overrides, by source), and whether the map ships as a real defaults file (natural home for a `--caller` map). Only after those: rows/impl notes, then delegate coding (opus/sonnet/grok) under supervision.


Coordinator oriented whole (outline, origin, all design/impl/def, audits, eyes doc; suite 232 green on 0.1.8+0b76d27). Inbox's three steward asks routed verbatim ([`design/vertical-info.md`](design/vertical-info.md) §Steward asks; [`design/overview-invariants.md`](design/overview-invariants.md) §Config drift). Usability/aesthetics pass recorded: [`audit/usability-aesthetics-2026-08-15.md`](audit/usability-aesthetics-2026-08-15.md). **Nothing implemented yet by decision** — the previous session's undiscussed spike (`2c7ded7`) was reset away and is reflog-only. Next: agree with Joseph what the vertical-info wave contains (mass→lines column, owned sub-lines w/ logical count, far-right ceiling, config-drift header) and which small fixes ride along (empty-dir mark, dangling `·`, `≈0f`, numeric-suffix junk, root age, help sections); put it on the pipeline; then delegate coding (opus/sonnet/grok) under supervision.

## Day closed — 2026-08-14, v0.1.8

The whole-tool day ended complete-as-specced minus the deliberately-open rows. Working: help/config/look floor, walk-bound+denied, censuses (reworked grammar), balanced allocator w/ redistribution, furniture+labels+git/github facets (`has:`), columns+sort (recency default, headings), mass+linecount, links (follow+recurse), heat+shas, quiet cold-baseline, important-files, JSON, ignored bodies (dim+⊘), globify, readme-title (off, key live), feedback footer → [`inbox.md`](inbox.md). Two de-novo audits + three hallway/witness threads all dispatched; closing audit verified all prior findings fixed. Perf: `~/src` ~1.5s, warm arch ~0.35s.

**Not built (honestly open):** cache+last-look (store designed, identity open), focus, vertical-info (the `~/src` density gauntlet — next), presets (cold look), phenom/SIGNA rendering, udon, private-remote (`gh` tension — Joseph), kind-query, prior-name, cross-level spend order. **Ratification queue:** in the designs' Opens + lattice (all marked; none blocking). **Standing ritual:** check inbox.md each session; reinstall on every consolidation; bump patch per consolidation.

## The build-day record (2026-08-14, kept for provenance)

Joseph's target: the tool complete as specced, today. Waves landed and committed: **A** — labels + furniture map + git/github facets (`[kind: …]`, `[git: remote br @sha dirty]`, noise-free first screens); **B** — columns + sort (caller-stack selection, placement enum, computed-tab-stop alignment per steward decision, **recency default sort** with key-within-weights survival — the call-it-again affordance is live), two-line header (stamp, then root). **C in flight**: mass (deep-agg) + linecount + links-follow-and-recurse + heat obtain (git-heat's model) + the reworked census rendering (no total, `kind×N`, dirs-separate-with-mass — provisional, globify/mass may intercede). **D-design in flight**: important-files / focus / last-look / quiet cold-baseline.

Master design landed: [`principles/src/form-agentic-eyes.md`](../../principles/src/form-agentic-eyes.md) — channels ↔ affordances ↔ concerns ↔ options; its own Part II chapter. New seeds: shorthand (placement classes + glyph-block), heat (git-heat + orient-rank priors).

**Ratification queue for Joseph** (nothing blocking): compose-surface spelling (config keys; `--sort KEY` + `-` reversal); census glyphs; `--walk` spelling; readme-title ON-vs-QUIET; linecount uncached-cost; gitignore global-excludes; partial-look exit code (pre-JSON, sized as its own row with cross-level spend order); lattice cells generally (first reasoned pass, `supported`).

Remaining after C/D: json, globify, gitignore-bodies, readme-title, shas, cache+last-look store, private-remote (`gh` tension — Joseph), udon (post-IR), kind-query.

New this pass (2026-08-14): [`design/mass.md`](design/mass.md) carries the short-term aim (calibrate how much the reader has *not* seen); [`design/cache.md`](design/cache.md) scopes purity and holds the open freshness question; [`design/quiet-columns.md`](design/quiet-columns.md) seeds caller-personalized surprisal (after-image); `--caller` and the channel-tuning law are now suite atoms (`principles/src/form-caller-key`, `norm-caller-tunes-the-channel`). Two-level is re-typed as a `milestone`.

The first-snapshot walk/absorb/allocator is **not** in the product. The binary is help, version, config, and the n-level look (default depth 2) with budget + censuses.

## Standing

- An `impl/` finish note is fine. Do not draft a `design/` file to unlock a checkbox.
- `future` stays on the outline.
