# sort — finish note

*Landed. Source: `src/sort.rs` (keys, comparator, per-level apply), wired in `main.rs::resolve_look` and — for survival — `n_level.rs::apply_budget`. Tests: `tests/sort.rs`.*

**Default: recency, newest first, dirs first** (steward, 2026-08-14 — the call-it-again affordance). Recency = mtime today; git last-touch may refine it in repos when Heat lands. `sort = name` / `--sort name` restores alphabetical.

**Shape**: groupings (dirs-first; `--dotfiles-first` adds dot-names-first within each partition and coexists with any key), then the key, then name (bytewise) as the unconditional tiebreak — the order is total, creation order invisible (tested by shuffled fixtures). A node with no value for the key sorts after those with one, in either direction.

**Spelling** (provisional, Joseph ratifies): `--sort KEY` flag and `sort` config key ride the same stack, flag wins (tested). Each key has a natural direction — name A→Z, recency newest-first, size largest-first — and a leading `-` reverses it (`--sort -recency` = oldest first). Built keys: `recency`/`mtime`, `name`, `size`. Any other lattice `sort=Y` fact refuses with exit 2, class named ("not built yet"), menu of built keys; a non-fact refuses as "not a sortable fact".

**Survival is key-within-weights** (the steward-implied leaning, implemented): `apply_budget`'s keep-choice ranks weight (dirs) first, then the sort key, then name — so under a tight `--lines` the recently-changed *survive*, not merely sort high. Display order is then the full comparator, applied per level after the allocator; the leftover census `[+N: …]` stays the last line of its level. Sort still never moves a node across parents.

**Sorting by a hidden fact** — implemented as the design's leaning, split by the mtime-stays-quiet steer: an *explicitly asked* key implies its column `on` (the order is a claim; evidence on the line) unless the caller set the column `off`; the recency *default* implies nothing — position carries the signal, the value stays quiet.

Env: `ASPECTUS_SORT`, `ASPECTUS_DOTFILES_FIRST`. Grouping toggles (`--no-dirs-first`) remain unbuilt and undecided, as designed.
