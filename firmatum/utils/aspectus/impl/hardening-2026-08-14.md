# hardening-2026-08-14 — finish note

*The pass over the hallway testimony (`audit/hallway-2026-08-14.md`, `audit/hallway-grok-2026-08-14.md`) plus the steward's same-day column-headings decision. Suite: 186 tests green (×3, with failures displayed — an earlier `grep FAILED` filter had briefly masked a test-compile error; caught and re-verified). Nothing committed — Joseph reviews, dogfoods on the testers' repro commands, commits, reinstalls.*

## The headline diagnosis: the "hang" was never `--explain-budget`

Reproduced, sampled (`sample` on the live process): all threads blocked in unbounded `git log --name-only` slurps and whole-file reads; **2.8 GB resident, 35 s** for default `~/src` — *with or without* the flag. Grok's differential ("same tree fine without the flag") compared different depths/builds; the flag is innocent. Four root causes, all fixed:

1. **Visible files were read whole regardless of size or budget** (`count_file_at(visible)` exemption) — a 2 GB `.dump` at `~/src` depth ≤3 was slurped for a line count. Now: visible files get a 256 KiB floor of budget-exemption, then degrade like the deep walk (count absent, mass `~`).
2. **Unknown-suffix sniff read the whole file first.** Now a 1 KiB bounded read judges before any slurp.
3. **Heat forked one unbounded `git log` per repo, 30+ concurrent.** Now: shared pool (`POOL_THREADS = 8`), output capped at 4 MiB/repo (largest measured local log ≈1.2 MB; a capped log drops only decay-zeroed tail).
4. **Deep mass and git porcelain were serial.** Both are now bounded-parallel post-passes (`n_level::deep_phase`, `git::annotate`), with `hidden_phase` alongside.

**Numbers** (warm): `~/src` default 35 s / 2.8 GB → **2.7 s / 216 MB**; `--depth 1 --lines 40 ~/src` 6.2 s → **1.1 s**; `--explain-budget ~/src` hang → **1.5 s**; `~/src/arch` 0.39 s (unregressed).

## Directory line-totals (grok's ≈56k/≈43k/≈25k/≈272k)

Three moves, recorded as a proposal in `design/mass.md` §Mass-mark distinction (glyphs await Joseph):

- **`~` vs `≈`**: `≈` now means *exact count, grouped for the eye*; `~` means *estimated past the read budget — this walk's number, not the directory's*. `≥` floor unchanged. File-counts are never estimated, so they keep `≥`/`≈` — and they are now stable across flag combos (asf `audits/` reads `≈1512f` under every flag tried).
- **Constant estimator** (32 B/line) replaced the look-observed ratio, which made a dir's estimate depend on what the walk had read *before* it.
- **Deterministic budget shares**: each cutoff subtree gets `remaining/n` of the read budget in the parallel phase, so `--inspect git` spending reads inside `.git` no longer starves a sibling's total. Line-estimates still shift a little when the *cutoff set* changes (more cutoffs → smaller shares) — but they now wear `~`. Tuning option left open: size-proportional shares.
- Side effect worth knowing: a symlink diamond spanning two cutoff subtrees counts in each (each dir's total describes that dir); parent aggregates still count once via `mass_dup`. `--show-all` totals still grow — that is membership, not instability (the look counts what it shows).

## Column headings + format consistency (steward, three arrivals)

Per `design/columns.md` §Column headings: a dimmed headings line (`lines   heat · age`, spellings provisional) right-aligned over the fact columns, directly above the first child; present only when a fact column renders; **charged to `--lines`** via a second allocation pass on a pre-budget clone (headed look = exactly `--lines`; a look that loses all columns when tightened comes in one under — never over). Riding with it: `format.mtime` gains `relative` (`2.2h ago`) as the text default — one time register with the heat cluster (`iso-8601`/`epoch` still available; JSON always iso-8601); a *quiet* mtime stays silent where the heat cluster already carries the age (an explicit `on` renders everywhere); a file-argument's header facts carry their column words (`20 lines`, `heat 0.01 · …`) since no headings line follows. Help teaches heat's two clocks ("0–~2 scale — not a size — counting in commits; age … wall-clock").

## Presence survives hiding (three testimonies)

Implemented the recorded leaning (`design/furniture.md`, status note added): hidden furniture dirs get a readdir-only deep file-count (no stats; 20k-name cap → `≥`), folded onto the kind word — `[has: archive ≈8f, …]` — summed per kind, JSON as additive `hidden` array. `git`/`github` excluded (their facets speak; `.git`'s object store is not working weight). The single-line-for-huge-masses idea and the agent `--caller` default map stay with Joseph, untouched.

## `has:` credibility (`.pytest_cache [kind: git]`)

`.gitignore`/`.gitmodules`/`.gitattributes` now claim their own words, never `git`; at a real repo (where `.git` claims `git`) the family words fold away as subsumed. So `.pytest_cache/` says `[has: gitignore, …]` — true and unmisleading — and repo roots stay `[has: git, …]`. Furniture map additionally learned `*.egg-info/`, `.pytest_cache/`, `.mypy_cache/`, `.ruff_cache/`, `.tox/` (build+python, hide).

## Refusals and identity

- `--inspect NO-SUCH-KIND` → exit 2, names the kind and the menu (`Map::known_kinds`).
- Gitlink `.git` under `--inspect git` shows `[gitdir: …]` — behavior made true where the promise couldn't be; help example says so.
- `--config MISSING` → exit 2 `--config file not found` (an explicitly named file deserves a refusal).
- `aspectus version` → `0.1.0+SHA[.dirty] (built …Z)`; build.rs watches the gitdir's HEAD/index so the stamp actually moves; dirty state marked. (The stale-install class of confusion now self-identifies.)
- JSON heat rounded to ≤4 decimals, trailing zeros trimmed (0.35220434162551345 → 0.3522). Judgment call, noted: 2 decimals is the eye's form; 4 keeps machine sort margins.
- JSON additions: `hidden` array (above). Schema stays 1 — born the same day, additive.
- `--lines 1/2`: structural 3-line floor on a non-empty root (stamp, facts/census, path) — overshoots rather than omits; now *stated* in the `--lines` help (the impl/overview-invariants.md recorded corner, resolved as a documented floor).

## Findings judged by-design or deferred (with reasons)

- **`--show-all` raising totals** — membership, by design; the census law says hidden names are not counted, and the has-spot (now with magnitude) says they're here.
- **`--sort size` not ranking dirs** — untouched; wants a decision (mass-lines? bytes?) that belongs with the lattice, not a hardening pass.
- **Root-files-vs-directories budget tension, first-contact sort preset, agent caller-default map** — routed to Joseph per the audit file; nothing here forecloses them.
- **Heading spellings, `~` glyph, `relative` mtime default** — implemented leanings, flagged for ratification in design (`columns.md`, `mass.md`, this note).

## Test changes

New `tests/hardening.rs` (8 tests, real binary). Budget arithmetic in `balanced`/`important`/`sort`/`json_format` bumped by the headings line's cost, each with a dated comment; `heat`/`quiet`/`overview`/`columns` scrapes repointed where they had used ISO-`Z` or `"ago"` as proxies now that mtime speaks relatively; `quiet::budget_independent` compares facts whitespace-normalized (alignment is look-relative by design; the 80 MB whale's count is now honestly absent past the budget).
