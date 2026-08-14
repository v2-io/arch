# Sort

Children have a defined order. Every level of every look, text and JSON alike — [[two-level|Two-level]] uses the default; it does not own it. The order is a pure function of the tree and the caller stack, never of terminal, locale, or readdir order (determinism claim, outline working notes: same tree + same caller state ⇒ byte-identical look, so aspecta stay diffable).

## The shape: groupings, then a key, then the name

An order is:

1. **Groupings** — coarse partitions applied in sequence. Default: **dirs-first**. `--dotfiles-first` adds a second grouping (dot-names before the rest) *and implies dirs-first* (origin, verbatim ask) — so grouping order is dirs-first, then dotfiles-first within each partition.
2. **Key** — a lattice fact with `sort = Y` (name, size, mtime, created, line-count, filetype, filekind, child-count, initial/latest-sha, heat, last-look). **Default key: recency, newest first** (steward, 2026-08-14: *"a default sort of git-recency … would be … immediately obvious to an agent who simply called it again, even with a small `--lines`"* — the git-heat affordance imported: re-calling the look shows what moved, at the top, with no cache needed). Recency = mtime by default (already statted, universal, works outside git); since the [[heat|Heat]] wave landed, config `recency-source = git` (spelling provisional) uses git last-touch where the log pass covered a path, mtime elsewhere. `sort = name` restores alphabetical. A fact is sortable only once it is implemented; asking for an unbuilt key is a refusal that names the class, not a silent name-sort.
3. **Tiebreak** — always name, so the order is total and two runs cannot disagree.

Name comparison is bytewise/codepoint (what ships today in `src/n_level.rs`), not locale collation — locale would break byte-identity across machines. This is why `--dotfiles-first` under the default key is *almost* a no-op (`.` sorts before letters already): it earns its keep as a grouping that survives the other keys.

Sort is **per-level**: children order within their parent. It never moves a node across parents, and the leftover-siblings census `[+N: …]` is always the last line of its level.

## What sort does not decide: survival — but survival follows the key within weights

Under a tight budget the allocator also decides *which* children survive to be listed. That choice is **weight** (its own lattice office), not sort — but the keep-choice ranks **key-within-weights** (shipped with the recency default, 2026-08-14): dir-weight first, then the sort key, then name. This is what makes the recency affordance real — the recently-changed *survive* a small `--lines`, not merely sort high among survivors. When importance/Focus weights land they slot in ahead of the key ([[balanced|Balanced summarization]]).

## Foundations (clauses)

| Clause | Where |
|---|---|
| Deterministic ordering in machine mode | [[../../../principles/influx/cli-conventions/ai-agent-considerations\|Agent]] |
| Order resolves from the caller stack | [[config\|Config]] |
| Sortable facts are lattice cells, not inventions | [[aspect-lattice\|Aspect lattice]] |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Default | Dirs first, then recency (newest first), then name tiebreak, every level. | Fixture with set mtimes; stable across runs and in a pipe. |
| 2 | Dotfiles-first | Dot-names group before the rest; dirs-first still holds. | `.b/ a/ .x f` orders `.b/ a/ .x f`. |
| 3 | Key from lattice | Order by an implemented `sort=Y` fact; descending available for magnitudes. | Sort by mtime: newest per the chosen direction; tiebreak by name on equal mtimes. |
| 4 | Unbuilt key refused | A `sort=Y` fact not yet implemented → exit 2, class named, menu of available keys. | Ask for heat before Heat lands. |
| 5 | Total order | Two runs, same tree: byte-identical order regardless of readdir order. | Recreate fixture in shuffled creation order; diff. |
| 6 | Config | Default order rides the caller stack like `depth`/`lines`; flag wins. | User-home sets mtime; flag returns name. |
| 7 | JSON | Same order as text — one look. | When `--format json` exists: array order matches text lines. |
| 8 | Census placement | `[+N: …]` last at its level; a dir-census stays on its own line. | Budgeted fixture snapshot. |

## Open

- ~~**Flag spelling.**~~ **Shipped provisionally (2026-08-14, Joseph ratifies):** `--sort KEY` + config `sort` on one stack, flag wins; each key has a natural direction (name A→Z, recency newest-first, size largest-first) and a leading `-` reverses it. `--dotfiles-first` is its own flag (verbatim ask) and survives whatever the compose spelling becomes. Columns stayed config-only — if a shared compose grammar ever lands, `--sort`'s spelling should be re-examined with it.
- ~~**Sorting by a hidden fact.**~~ **Shipped as the leaning, scoped by the mtime-quiet steer:** an *explicitly asked* key implies its column `on` (the order is a claim; evidence on the line) unless the caller set it `off`; the recency *default* implies nothing — position carries the signal.
- ~~**Survival under budget.**~~ **Decided: key-within-weights** (see §Survival above).
- **Grouping toggles.** Is dirs-first itself defeatable (`--no-dirs-first`), or is it law? Origin implies it is the norm; nothing recorded says it is immovable. Not decided.

## Not in this row

Which children survive a budget (weight — [[balanced|Balanced summarization]], Focus). Important-files pinning (weight, not order — the pipeline row says so). What facts exist ([[aspect-lattice|Aspect lattice]]). Globify series ordering (numeric runs are that row's story).
