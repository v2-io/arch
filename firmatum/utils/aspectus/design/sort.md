# Sort

Children have a defined order. Every level of every look, text and JSON alike — [[two-level|Two-level]] uses the default; it does not own it. The order is a pure function of the tree and the caller stack, never of terminal, locale, or readdir order (determinism claim, outline working notes: same tree + same caller state ⇒ byte-identical look, so aspecta stay diffable).

## The shape: groupings, then a key, then the name

An order is:

1. **Groupings** — coarse partitions applied in sequence. Default: **dirs-first**. `--dotfiles-first` adds a second grouping (dot-names before the rest) *and implies dirs-first* (origin, verbatim ask) — so grouping order is dirs-first, then dotfiles-first within each partition.
2. **Key** — a lattice fact with `sort = Y` (name, size, mtime, created, line-count, filetype, filekind, child-count, initial/latest-sha, heat, last-look). **Default key: recency, newest first** (steward, 2026-08-14: *"a default sort of git-recency … would be … immediately obvious to an agent who simply called it again, even with a small `--lines`"* — the git-heat affordance imported: re-calling the look shows what moved, at the top, with no cache needed). Recency = mtime today (already statted, universal, works outside git); git last-touch may refine it inside repos once that obtain lands ([[heat|Heat]] wave) — config chooses. `sort = name` restores alphabetical. A fact is sortable only once it is implemented; asking for an unbuilt key is a refusal that names the class, not a silent name-sort.
3. **Tiebreak** — always name, so the order is total and two runs cannot disagree.

Name comparison is bytewise/codepoint (what ships today in `src/n_level.rs`), not locale collation — locale would break byte-identity across machines. This is why `--dotfiles-first` under the default key is *almost* a no-op (`.` sorts before letters already): it earns its keep as a grouping that survives the other keys.

Sort is **per-level**: children order within their parent. It never moves a node across parents, and the leftover-siblings census `[+N: …]` is always the last line of its level.

## What sort does not decide: survival

Under a tight budget the allocator also makes an ordering decision — *which* children survive to be listed (today: dir-weight, then name, coinciding with the display order). That choice is **weight** (its own lattice office), not sort. The coincidence is worth keeping while it is free, but this row claims only display order; if a caller sorts by mtime, whether the survivors are the newest or still the weighted set is the allocator's law ([[balanced|Balanced summarization]] + Focus/weight), stated there. See Open.

## Foundations (clauses)

| Clause | Where |
|---|---|
| Deterministic ordering in machine mode | [[../../../principles/influx/cli-conventions/ai-agent-considerations\|Agent]] |
| Order resolves from the caller stack | [[config\|Config]] |
| Sortable facts are lattice cells, not inventions | [[aspect-lattice\|Aspect lattice]] |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Default | Dirs first, then name (codepoint), every level. | Fixture with mixed case, dotfiles, dirs; stable across runs and in a pipe. |
| 2 | Dotfiles-first | Dot-names group before the rest; dirs-first still holds. | `.b/ a/ .x f` orders `.b/ a/ .x f`. |
| 3 | Key from lattice | Order by an implemented `sort=Y` fact; descending available for magnitudes. | Sort by mtime: newest per the chosen direction; tiebreak by name on equal mtimes. |
| 4 | Unbuilt key refused | A `sort=Y` fact not yet implemented → exit 2, class named, menu of available keys. | Ask for heat before Heat lands. |
| 5 | Total order | Two runs, same tree: byte-identical order regardless of readdir order. | Recreate fixture in shuffled creation order; diff. |
| 6 | Config | Default order rides the caller stack like `depth`/`lines`; flag wins. | User-home sets mtime; flag returns name. |
| 7 | JSON | Same order as text — one look. | When `--format json` exists: array order matches text lines. |
| 8 | Census placement | `[+N: …]` last at its level; a dir-census stays on its own line. | Budgeted fixture snapshot. |

## Open

- **Flag spelling / compose surface.** `--sort mtime`? Direction (`--sort -mtime` vs `:desc`)? Shared grammar with [[columns|Columns]]' compose ask — one decision for both (lattice's own open question: does `--sort` exist, or config-only?). `--dotfiles-first` as its own flag is Joseph's verbatim ask and should survive whatever the compose spelling becomes. Not decided.
- **Sorting by a hidden fact.** Order by mtime while mtime's column is `OFF`: the reader sees an order it cannot verify. Does asking for a sort key imply its column `ON` (leaning: yes — the order is a claim, and the evidence should be on the line), or stay independent? Not decided.
- **Survival under budget.** When a non-default key is asked, does the allocator's keep-choice follow the key, the weights, or key-within-weights? The recency default's whole affordance ("obvious even with a small `--lines`") implies survivors follow the key — the recently-changed must *survive*, not just sort high. Leaning key-within-weights (importance still protects README); steward-implied, confirm at landing.
- **Grouping toggles.** Is dirs-first itself defeatable (`--no-dirs-first`), or is it law? Origin implies it is the norm; nothing recorded says it is immovable. Not decided.

## Not in this row

Which children survive a budget (weight — [[balanced|Balanced summarization]], Focus). Important-files pinning (weight, not order — the pipeline row says so). What facts exist ([[aspect-lattice|Aspect lattice]]). Globify series ordering (numeric runs are that row's story).
