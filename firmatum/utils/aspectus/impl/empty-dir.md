# Empty directory — finish note (2026-10-03)

Landed per [[../design/empty-dir|design]]. A directory whose readdir yielded no names says `[empty]` on its own line.

- **`src/n_level.rs`:** `Node.empty`, set in `gather_dir` from the raw `enumerate` result (`entries.is_empty() && !iter_err`) *before* `gather_dir_inner` applies the furniture map, ignore rules, or show-all. The depth-cutoff path runs through the same function, so an empty dir at the cutoff is marked too. That is where the old silence lived, because an empty census rendered nothing. Ignored dirs (`stat_only`), denied, cycle, and other-fs nodes never reach `gather_dir`, so they cannot claim it.
- **`src/ready.rs`:** `look_marks` appends `[empty]` last among the marks (fact `empty-dir`, office `mark`). The root facts line uses the same near-right, so an empty root prints `[empty]` above its path with no special case.
- **`src/json.rs`:** `"empty": true`, present-when-true like `denied`/`cycle`. This is additive within schema 1, and `truncated` is untouched.
- **`src/facts.rs`:** the `empty-dir` row went from `⇥` to `✓`, formats `[empty]*`. `aspectus config` shows it.
- **Help:** the "never lies by omission" paragraph names `[empty]`.
- **Tests:** `tests/empty_dir.rs` (6), covering expanded, cutoff, root, symlink-to-empty, the not-empty cases (ignored dir, hide-only, ignored-files-only, omit-only), JSON, and help. No golden moved. Suite 320 green.

**Dogfood** (release build vs installed v0.1.17, stamp line dropped). `~/src/aat-refactored --lines 300 --depth 4` changed one line, `audits/ … [empty]`, which is the inbox specimen. `~/src/arch` changed 5 (`INGEST/`, `.orient/`, `.integrated/`, an empty paper dir), and `~/src/arch/vivarium --depth 3` changed 3 (`doc/{design,plan,theory}/`). `~/src/arch/asf`, `~/src --depth 1`, and this crate: byte-identical.

**Calls made**

- The mark is last in the marks order. It can co-occur with almost nothing: `[walk bound]` needs entries, and `[denied]` and `[unreadable: io]` exclude it by construction.
- Width `7` in the inventory (the word's cells). The marks column is not a reserved column today. Near-right is still one ordered list painted after the far-right cells (grid-cleanup step 1's note).

**Seen while landing, not acted on**

- **Omit-only dirs stay bare** (design Open), e.g. `ds-only/` holding only `.DS_Store`.
- Vivarium's three empty doc dirs carry an *age* (`◉◎`) although git cannot track an empty dir. The age column's git-known gate (`n.heat.is_some() || n.git_ts.is_some()`) seems to admit dirs through a rolled-up value. I haven't traced it; worth a look when the heat/age column is next touched.
