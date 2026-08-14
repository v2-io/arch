# Heat

*Seeded 2026-08-14 from Joseph's pointers to shipped, tuned prior art. Adopt, don't invent.*

## The model: adopt `git-heat`'s

`~/.local/bin/git-heat` (also `firmatum/utils/code/`) carries a worked commit-decay model, already tuned in use:

- `raw = Σ over non-initial touches of exp(-age/τ)`, **age in commits behind HEAD** (not wall-clock), `τ = half_life / ln 2`, normalized `heat = 2·(1−exp(−1/τ))·raw` so touched-every-commit converges to ~2 at any half-life. Half-life choices `21…1`, default **7**.
- **Dir heat = max of non-noise leaves beneath** (sum drowns in big dirs) — this is heat's `deep-agg` office answered.
- **Initial commit excluded**; **noise basenames** (`Cargo.toml`, `SOURCE_REV`, extensible) get heat 0 and don't join the rollup — kin to furniture's exclusion from mass; the noise map likely merges with the furniture/config map rather than living separately.
- The lattice's `H~N` / introducing-sha / last-touch-sha facts are the same obtain (one `git log --name-only` pass); one row's implementation feeds all three.

## The weight office: `orient-rank`'s lessons

`arch/vivarium/bin/orient-rank` is the estate's shipped orientation-importance ranker, and two of its decisions transfer to aspectus's **weight** office directly:

- **Heat is one factor, not the ranking.** It combines freshness / churn / mentions / preeminence / working-surface / pagerank via rank-CDF normalization + geometric mean (a weak axis pulls harder than arithmetic — deliberate), with a "tip fiat" pinning the very-recently-touched to the top. Aspectus's allocator weight should expect the same shape eventually: heat, Focus, importance composing — the lattice Open ("weight office algebra") now has prior art.
- **Mark in place; don't reorder.** `--mark-outline` stars hot segments *in outline order* because "agents free-read by outline order, not rank order" — the exact philosophy of aspectus Focus ("matches stay in place; surroundings are not thrown away") independently arrived at. Heat in the look marks hot lines where they stand; it does not resort the tree unless Sort is asked for heat.
- Its docstring's humility is worth inheriting verbatim: *"This is deliberately imperfect. Equal-weight CDF-normalized factors are good enough to move with confidence; rebalance when we have evidence."*

## Visual affordances (from git-heat's shipped look, Joseph's screenshots 2026-08-14)

What makes the git-heat view legible at a glance, transferable to the text look:

- **The `score · age` cluster**: heat and recency paired as one compact unit, right-aligned at a consistent column — two aliveness facts read as one glance-stop (`1.01 · 13.6d ago`). When aspectus shows heat, pairing it with the mtime delta in one cluster is the proven rendering.
- **Human-relative ages with auto-chosen units** (`13.6d ago`, `2.2w ago`, `3.5w ago`) — the *delta* register for recency, distinct from the ISO *timestamp* register; a plain-form sibling of SIGNA for the same fact, and the natural `format` option for mtime-as-aliveness.
- **Color as the magnitude channel** (viridis-like ramp per row): on a TTY, heat color is a whole extra channel costing zero glyphs — a natural later extension of [[color|Color]] (which currently only paints dirs). Machine path unaffected (color never survives a pipe under auto).
- **Hot-first within an expanded dir** in the git-heat view is its fixed sort; in aspectus that is just [[sort|Sort]] asked for key=heat — not a heat-row invention.
- **Order by recency, score by decay — deliberately not the same axis** (Joseph, 2026-08-14): git-heat *orders* rows by last edit while heat is the score/color, so "the hot stuff *tends* to the top, but you get stuff at the very top that was the very most recently changed — implicit 'what *has* been hot vs what is definitively most recently changed.'" Two aliveness registers, separable at a glance because one lives in position and the other in color/number. An anti-collapse instance: sustained-heat and latest-touch route to different reader questions (where has work been living vs what just happened) — a merge that "simplifies" them into one score loses the distinction. orient-rank's tip fiat is the same insight formalized (very-recent pins to top *regardless* of decayed score). For aspectus: sort-by-mtime with heat as a shown fact reproduces it; the lattice's separate mtime and heat rows are already right — do not fuse them into one "activity" fact.

Visible set only (never a reason to widen the walk); omit outside git; `signa`-density is the natural text format (lattice). Obtain is one log pass per repo — cache-keyed by HEAD, which makes it the second-cheapest cache client after mass. Whether aspectus *shells to* `git-heat`, imports its model, or reimplements the ~40-line core is the implementer's call — the model constants above are the law either way.

## Shipped (2026-08-14, Wave C)

Reimplemented core (`src/heat.rs`), constants verbatim; default ON in-repo as the `score · age` cluster (age = mtime delta, human-relative); dir max rollup; repo-root line carries the repo's max; noise silent; absent outside git. Log capped at 400 newest commits (contribution beyond is zero to double precision; last-touch past the cap not claimed — git-recency falls back to mtime). Per-repo log passes run in parallel. `--sort heat` built; `recency-source = git` (key spelling provisional) routes the default recency sort through git last-touch. Prior-name / signa / color ramp / cache: later rows. Details: impl/heat.md.

**Sha facts shipped (2026-08-14, Wave E):** the lattice's `H~N` / initial-sha / latest-sha row now rides the same log pass (`--name-status`): compose-only columns `columns.initial-sha` / `columns.latest-sha`, formats `short* / h~n / full`, files only, absent outside git or past the log window — never guessed. JSON carries full shas + behind-counts. Sha *sort* keys stay unbuilt/refused. Details: impl/heat.md §Sha facts.
