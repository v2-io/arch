# Walk bound — finish note (2026-08-14)

Landed per [[../design/walk-bound.md|design]], restructured mid-landing after the coordinator's asf dogfood and the de-novo audit ([[../audit/de-novo-2026-08-14.md|findings 1–3]]) independently showed the first shape dropping level members silently in readdir order.

- **Flag `--walk N`** / config key `walk` / `ASPECTUS_WALK`, riding the caller stack like `lines` and `depth`. Default **10000**; `0` = no bound. `--visit` not inherited (design records why). Spelling awaits Joseph's re-ratification.
- **Mechanism** (`src/n_level.rs`): per directory, `enumerate` reads *all* names + readdir type-hints (free, never cut; `None` = denied), sorted dirs-first-then-name; `WalkBudget` then charges one per **stat+recurse**, spent in that sorted order. Exhaustion mid-level → the remainder becomes an **exact** census: dir census `[N: …]` on the dir's own line (root header included) when nothing was listed, leaf census `[+N: …]` after some were (merged with any later line-budget omission via `merge_census`). The cut dir says `[walk bound]`.
- `≥` (`Census.bounded`) is reserved for genuinely-unknown counts: a fold hiding a `[denied]` child, or a mid-readdir entry error (`[unreadable: io]`). A budget cut never prints `≥` — its counts are exact.
- Determinism: membership total, expansion first-N-in-sort-order → a bounded look is repeatable (tested by double-run comparison below the timestamped header).
- **≥-into-aggregates law** ([[../design/cache.md|cache]]): `bounded` lives on the `Census` value, so a future cache inherits floor-not-fact by construction.
- Help: option line, example, closing honesty paragraph, same seam. `--explain-budget` adds a `walk: bound N reached…` stderr line.
- Tests: `tests/walk_bound.rs` (9) — exactness unbounded, quiet default, exact membership under cut, cut-listing confession, sorted deterministic expansion (double-run), caller stack, explain, usage refusal, help teaches.

Known minor: the `[+N: …]` remainder line isn't counted by `apply_budget`'s line accounting (a cut node can render one line over its share). Left simple; revisit if it bites.

Open (recorded in design): cross-level spend order is still depth-first — audit finding 3 argues for breadth-ish spend; sized as its own row. Partial-look exit code also open (audit finding 4).
