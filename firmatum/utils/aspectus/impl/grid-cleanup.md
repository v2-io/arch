# Grid cleanup — implementation plan (2026-08-22, coordinator; code follows design/grid-cleanup.md + design/lattice-2.md, never leads)

Staged so the look never moves while the code does, and so each slice is one agent's job with one review seam.

1. **Prefactor, output-identical** — restructure `columns.rs` (and the render half of `n_level.rs`) into the pipeline the design names: ready-facts (per-node formatters → `{text, place, office, marks}`) → cells (a row grammar with named slots, in a fixed order) → rows (a node may own sub-rows; none yet) → paint (stops, right edge, color). **The rendered bytes do not change** in this slice; snapshot tests over fixture trees pin that (and `~/src/arch/asf` / this crate dogfooded by diff). `facts.rs` becomes the inventory lattice-2 describes (fact, derived-from, position, display, sort, width, formats) and stops lying about `place`. No new facts, no new glyphs.
2. **Shipped defaults** (design/defaults.md) — embedded `defaults.toml`; `[layout]` lists feed the cell order; maps as data; `config` prints the effective grid.
3. **Count cell** — far-right `lines`/`bytes` rendered in the decided 12-cell grammar (design/grid-cleanup.md §The count cell). Mass lines join the `lines` column (Joseph's inbox ask). The census's *internal* form is **not** changed here (subgroup-subject form undecided).
4. **Filetype** (design/filetype.md) — `kind.rs` → `filetype.rs`; consumers keep today's output until a census grain is asked for.
5. **git-status far-left glyph** — first far-left tenant via `[layout]`; `⊘` moves into it; the marks-column `⊘` retires.

Not in this plan (undecided): subgroup-subject form · census delimiter · mass/size placement beyond step 3 · `[denied]` glyph · empty-dir mark · the header designation · sub-row spill rules · `-q`.

## Step 1 landed — prefactor, output-identical (2026-08-22)

**What the binary does now.** Exactly what it did before, byte for byte. What changed is underneath: the renderer has the model design/grid-cleanup.md and design/lattice-2.md name.

- **`src/ready.rs` (new) — the ready-facts layer.** Each fact is rendered by its own small formatter into a `Ready { fact, text, position, office }`: quiet already applied, no tab-stop, no width, no neighbor. `Position` is lattice-2's vocabulary (`far-left · level-location · name-location · name-suffix · after-name · near-right · supplement · far-right`) plus two words lattice-2 uses in prose and the inventory needed to stay honest: `in-cell` (a mark inside another fact's cell — the `≈ ≥ ~` slot) and `—`/unplaced (filetype, important, furniture-fate render nothing of their own). `Office` is `line · census · mark · look · weight`. `columns()` names the far-right columns and their headings in one list, and cells + headings are both generated from it, so they cannot drift apart.
- **`src/columns.rs` — selection/format + the cell/row grammar.** `Slots` is the named-slot row: `far_left · level · name · name_suffix · after_name · near_right · supplement · far_right`, filled from the ready-facts in a fixed order. `rest_of`'s single code-order string is gone — near-right is now an ordered list of typed parts (title · kind-word · glob-count · dir-census · mass-lines · ignored-remainder · facets · has · look-marks) that paint happens to join with two spaces today. Paint is the only layer that knows about other rows (tab-stop, per-column right edges, the heat cluster's two sub-columns, color).
- **`src/facts.rs` — rewritten as lattice-2's inventory.** Rows now carry `stat · fact · derived-from · position · office · display · sort · width · ask · formats`. Two honesty rules it now lives under: `position` says where the fact paints **today** (the old `place` words were inverted), and `stat` (`✓ ↬ ⇥`) says how settled that is — so every row the design still means to move reads as `↬` instead of being silently misfiled. `sort` lists only keys `sort.rs` actually accepts. `aspectus config` prints the table plus a `derived-from` block; `tests/columns.rs::config_shows_fact_inventory` moved with it (slugs are the lattice's now: `size` ⇒ `bytes`, `child-count` is gone — it is the census's totals).
- **`tests/grid_snapshot.rs` + `tests/snapshots/` (new).** Five fixture looks — furniture, facets, dir census, leaf census, symlinks (one broken), quiet perms/size/mtime, a real git repo with an ignored remainder — pinned byte-for-byte. Heat is off in all of them on purpose (`score · age` is wall-clock, so a golden carrying it would rot; heat's paint stays pinned by `tests/heat.rs`). `ASPECTUS_SNAPSHOT_BLESS=1 cargo test --test grid_snapshot` refreshes them deliberately.

**Calls made** (all reversible, all inside the slice): near-right stays *one ordered list* rather than splitting into near-right + supplement (reasoning below); the `/` on a dir is its own ready-fact (`directory-glyph`, name-suffix) but is still painted inside the name's color run, because that is what the bytes are; `symlink-target` moved from "decoration fused to the name" to lattice-2's `after-name` slot, same bytes; the leaf-census remainder row is built through `ready::leaf_census` so the three name-location tenants (name / glob-template / leaf-census) are visibly mutually exclusive in the code.

**Verified.** 254 tests green (249 before + 5 snapshots). Old binary (built from `HEAD`) vs new, run back to back: byte-identical on `~/src/arch/asf` (depth 4), `~/src` (depth 1), `~/src/arch/vivarium`, `~/src/ops`, `~/src/memorata`, `/tmp`, this crate, a file argument, `--format json`, and under `columns.size/mtime/permissions/owner/initial-sha/latest-sha = on`, `columns.heat = off`, `--sort name`, `--show-all`, `readme-title = on`, a focus look, and `--lines 8`. The only diffs seen were the stamp second and the build sha in `--help`.

**Flagged for the design (not acted on).**

- **Supplement has no tenant, and the criterion for one is a decision that has not been made.** lattice-2 defines it as "additional info cell between near and far-right columns **or under columns on next line if it doesn't fit**" — that is spill semantics, so its tenancy is decided by *"which near-right parts spill to sub-rows and when"*, which design/grid-cleanup.md §Decisions still lists as open. Splitting near-right today would mean inventing that criterion in code. The slot is declared and empty; when the spill rule lands, the first tenants are the two parts the design already marks `⇒ sub-row` — the git facet and the `[has: …]` block, which are the widest parts and the ones that shove the far-right cluster.
- **`readme-title`'s place is still open** (after-name vs near-right); the implementation's choice is near-right and the inventory now says so plainly rather than filing it as decoration.
- **`git-status` and `gitignored` are one fact wearing two rows today.** The inventory shows `gitignored` painting `⊘` in the near-right marks and `git-status` marked `↬` at the same place; step 5 collapses them into the far-left cell. Nothing was changed here.
- **`files` / `dirs`** have no independent ready-fact — they exist only inside `Census::render`. Their inventory rows say `near-right (inside the census)`, which is true, but the count-cell slice will want them as real formatters; that is where `Census::render` gets carved up.

## Step 3 landed — count cell (2026-08-22)

**What the binary does now.** Far-right `lines` and `bytes` render as the 12-cell field (`g ␠ m T · NNN . f s u`). File line counts are `816.` / `1·099.`; an unexpanded dir's deep line total sits in the same column with its mark in the `m` slot (`≈  61.2K`, `~   5.0M`, `≥ 434.0K`); the trailing `≈61k lines` after the census is gone. The `bytes` heading replaced `size`; `human_size` is gone. JSON is unchanged. `Census::render` is untouched.

- **`src/count_cell.rs` (new).** One function, `count_cell(value, mark, subject, unit, show_unit) → 12 chars`. Scale at ≥ 10,000 (1000 for counts/lines, 1024 for bytes); below that, exact with `T·` only when ≥ 1,000; at and above, one fraction digit and `K M G T P`. `Exact` becomes `≈` when the formatter scales; `Floor` / `Estimated` keep their face. Subject glyphs `● □ ▣` and units `𝓃 B 𝓁 𝓉` are implemented; this slice blanks both under the `lines`/`bytes` headings (width kept). Unit tests cover 1·099, 9·021, 190.0K, 14.3G, 1.0M, 3.3P, 0, 912, plus heading-blanking and mark precedence.
- **`src/ready.rs`.** `lines_cell`: files from `n.lines` (binary omits, empty text is `0.`); dirs only when unexpanded (`leftover` + `mass.lines > 0`) — the mass tail *moved*, expanded dirs still speak through their children. `fmt_size`: `human` is the count cell; `format.size = bytes` stays the raw integer (a 12-cell exact form cannot hold ≥ 10,000 without scaling). `mass_lines` is gone from near-right. `far_right_header` shows units (the root facts line sits above the headings; a file root has no headings line).
- **`src/columns.rs`.** Heading `size` → `bytes`. Root facts trim the 12-cell pad (that line is not in the grid) and drop the `"N lines"` wrap.
- **Help.** The line-count/mass paragraph and the `≈ ~ ≥` paragraph say the new form.
- **Tests.** Snapshots re-blessed (`ASPECTUS_SNAPSHOT_BLESS=1`). Every changed scrape carries a `2026-08-22 count-cell slice` comment.

**Calls made**

- Unexpanded dirs only for the deep total (the leftover gate). lattice-2 can be read as *every* dir carrying Σ subtree; this slice moved the existing tail, it did not add a new number on expanded dirs.
- `format.size = bytes` remains a raw integer, not an unscaled count cell.
- File-root facts show `𝓁` in the unit slot rather than wrapping `"N lines"`.
- Scaled exact values wear `≈` (grouped for the eye), including file sizes — so a quiet whale is `≈  80.0M`, not `80M`.

**Awkward — the two places you named**

- **`.`-always.** Every file is `1.` `2.` `3.` `11.`. The dots align (that is the point) and they are loud on small numbers. The heading `lines` (5 chars) right-aligns to cell 12 of a field whose `.` is cell 9, so the word sits over the ones digit, the `.`, and the blank `f s u` — the tens digit of `11.` sticks out left of the heading.
- **Heading-blanking + `trim_end`.** When `lines` is the last column (no heat, outside git), paint eats the three trailing spaces of the 12-cell; the heading (which ends in `s`) is not eaten, so it overhangs the `.` by 3. When heat or another column follows, the 12-wide field is preserved and the gap after the `.` is the blanked `f s u` plus the next column's pad — visible in the kitchen snapshot as a wide hole between `11.` and the census when quiet `perms` also claims a column. The field keeps its width in the layout; the last column's visible bytes do not.

**Flagged for the design (not acted on)**

- Small mass lost its `≈`: `≈6 lines` is now `6.` (exact, ungrouped, mark blank). Joseph's inbox mock had `≈101` / `≈681` for ungrouped totals. The count-cell law says blank; the mock said ≈. Which?
- `3.3PB` under a heading that names only the subject still wears `≈` once scaled. The design specimen omitted the mark to show g-blanking.
- Dir byte totals (lattice-2 `dir: Σ descendants`) are still unbuilt. This slice only replaced `human_size` on file `st_size`.
- `files`/`dirs` as count cells still wait on the subgroup-subject form (`Census::render` unchanged).

## Step 5 landed — git-status far-left glyph (2026-08-22)

**What the binary does now.** Inside a live git work tree, every row of the look pays one cell to the left of the tree prefix: porcelain `M A ⁇ R U D C T` (`⁇` = U+2047 for `??`), `⊘` when the entry is gitignored, blank (a space) when clean. Outside any repo the block is absent — kitchen and the other non-git goldens did not move. The marks-column `⊘` is gone; TTY dim on the name stays; JSON `gitignored: true` is unchanged. `[layout] far-left` is what turns the cell on (`git-status` in the list paints; `mtime`/`bytes` in that list still have no compact form). `aspectus config` prints `(unbuilt: mtime, bytes)` on far-left, not `(unbuilt position)`. Help teaches the pack in one short table. `--sort git` is refused by name.

- **`src/git.rs`.** The one porcelain subprocess (`status --porcelain --untracked-files=normal`) now yields the dirty *count* (parent-line facet, as before) and a per-path XY map. Worktree letter wins when it is not space; otherwise the index letter. Unmerged pairs (`DD AU UD UA DU AA UU`, or either column `U`) render as `U`. Paths are repo-relative; rename/copy use the new name; quoted paths unquote. The enclosing work tree of the look is obtained even when `.git` sits above the asked path (a look at `src/` still gets letters). Nested repos use their own map.
- **`src/ready.rs`.** `far_left` / `far_left_blank` / `git_status_cell`. Directories are blank unless gitignored (`⊘`) or porcelain `?? dir/` (`⁇`). `look_marks` no longer emits `⊘`.
- **`src/columns.rs`.** `[layout] far-left` membership on `Cols`; paint fills the slot in list order for facts that have a formatter. Far-left is excluded from `name_width` (it is a look prefix, not the name column — including it would steal from STOP_CAP and shift everything right of the tree). Headings and remainder rows pay a blank cell; stamp, root-facts, and root-path do not (a copied path must not start with a space). No heading word over the block.
- **`src/n_level.rs`.** `in_git` / `git_letter` on `Node`.
- **`src/facts.rs`.** `git-status` paints `far-left` today (`{⊘ M A ⁇ R U D C T}*`); `gitignored` is unplaced (JSON field; glyph in git-status).
- **`src/config.rs`.** `UNBUILT_POSITIONS` is just `supplement`. Far-left names the entries it cannot paint.
- **`src/sort.rs`.** `git` is an unbuilt key.
- **Help.** Pack table; gitignored paragraph no longer claims a marks-column `⊘`.
- **Tests.** `tests/git_furniture.rs` letters / absent-outside-repo / layout-off; gitignore tests assert `⊘` is before the name; `--sort git` refused; config scrape for `(unbuilt: mtime, bytes)`. Git-repo golden re-blessed 2026-08-22 (dated comment in `tests/grid_snapshot.rs`); the other four goldens did not move. Suite green (288 tests).

**Calls made**

- **Directories:** porcelain `?? dir/` is `⁇` (design/grid-cleanup.md §Step 5 landed — a newly-added directory that looked clean would be a silent lie). Tracked-and-clean dirs stay blank; gitignored dirs keep `⊘`. Dirty submodules, typechange-to-dir: still blank (not asked).
- **`--untracked-files=normal` does not name files inside an untracked directory** (`?? newdir/` is the whole entry). The dir itself now shows `⁇`; children under it still look clean. Extending porcelain with `-uall` would be a second contract, not this slice.
- **"The look contains a repo" means git facts actually obtained** (facet or porcelain), not merely a `.git` name. Furniture decoys (empty `.git/` in `tests/two_level.rs`) do not widen the look. A live enclosing repo above the asked path does.
- **Look-wide presence.** Once any node is in a live work tree, every *tree* row and the headings line pay the cell — including non-repo siblings of a nested repo. Stamp, root-facts, and root-path do not. A look at a parent of repos (e.g. `~/src`) grows by one column for the tree.
- **No JSON `git_status` letter.** Design said JSON `gitignored: true` unchanged and was silent on a status field; not added.
- **`--sort git` stays unbuilt.** lattice-2's Initial table lists `sort git`; the Time/aliveness row lists `—`. The ask was to refuse by name; facts.rs `sort` stays `None` (only keys `sort.rs` accepts appear there).
- **`two_level::twice_same_tree` stamp-strip.** Pre-existing flake: strip required `"  "` on the stamp line, so two runs that straddled a second failed. The test now drops line 1. Not a render change.

**The two silences, decided 2026-08-22 (design/grid-cleanup.md §Step 5 landed) and now in the binary.** Untracked dirs show `⁇`; stamp/facts/path skip the cell. Remaining adjacent (not acted on): dirty-submodule / typechange-to-dir letters; `-uall` for children of an untracked dir; look-wide vs per-row for mixed trees (non-git siblings still pay the blank cell). Two `git-status` rows in lattice-2 (Initial: `sort git`; Time/aliveness: `sort —`) — one cell was painted; sort remains unbuilt.

## Step 6 landed — heat density (2026-08-23)

**What the binary does now.** Heat paints as a two-cell density block at far-left, first in the block, before git-status. Nine grades `  ` · ` ░` · `░░` · `░▒` · `▒▒` · `▒▓` · `▓▓` · `▓█` · `██`; blank outside git (block absent) and for score 0 / unscored (cells paid, blank). Any positive score is at least ` ░`; linear over [0, 2] into grades 1–8, capped at `██`. A directory's grade is its rolled-up heat (max of leaves, as today). The far-right cluster is `age` alone under an `age` heading (SIGNA by default). `format.heat = score` restores today's `score · age` cluster on the far-right (path kept). `layout.far-left-gap` (default 2, env `ASPECTUS_LAYOUT_FAR_LEFT_GAP`) is spaces between the block and the tree prefix, paid only when the block exists — headings and tree rows pay it; stamp / facts / path do not. JSON heat stays a number. `aspectus config` paints heat on far-left (no longer in `(unbuilt: …)`); mtime/bytes compact forms stay unbuilt. Help teaches the density pack next to git-status and SIGNA in one compact table.

**Score → grade mapping (first cut).** Grade `k` (1..=8) is the bin `((k−1)/4, k/4]` of the score — `ceil(score × 4)`, clamped to 1..=8. Equivalently bins of width 0.25 on [0, 2]: `(0, 0.25] → ░`, `(0.25, 0.50] → ░░`, …, `(1.75, 2] → ██`, `> 2 → ██`. Score 0 / `None` → blank `  `. Joseph corrects on contact.

**SIGNA (same hour, design/phenom-format.md §Decided).** `format.mtime = signa*` (relative / iso-8601 / epoch kept). Mixed-radix from the zoetica table; two most-significant unit ranks; seconds-grain (`· ╶ ╌`) omitted when a minute or larger is present; ≥10 years is `⬤` ×9 capped; right-aligned under `age`; JSON stays iso-8601. Month/year seconds match `rel_age` (2_629_746 / 31_556_952). 0 elapsed renders `·` (just now) so a speaking mtime that is "now" is not blank.

- **`src/heat.rs`.** `density_grade` / `density_cell` / the nine-grade table. Unit tests for the bins.
- **`src/signa.rs` (new).** The run. Unit tests against the table-matching primary examples, omit-seconds, two-unit bound, year cap.
- **`src/ready.rs`.** Far-left paints heat then git-status; far-right is `age` under density or the cluster under score.
- **`src/columns.rs`.** `HeatFmt`, `TimeFmt::Signa`, `far_left_gap`, gap in paint, cluster alignment only on score.
- **`src/config.rs`.** `FAR_LEFT_PAINTABLE` includes `heat`; `age` maps to `columns.heat` (one toggle); env keys; won unit `spaces` on the gap.
- **`src/facts.rs`.** Heat position `far-left`, formats `density* / score`; mtime formats `signa* / relative / iso-8601 / epoch`.
- **Help.** Density pack + SIGNA glyph table.
- **Tests.** Snapshots: git-repo re-blessed (gap; heat still off so density cells are not in the golden — ages would rot); kitchen / census / leaf / columns-on did not move.

**Calls made**

- **0 elapsed is `·`, not blank.** A speaking quiet-mtime of "now" would otherwise look silent.
- **Two-unit truncation is by unit rank, not glyph count.** 3h 15m is `═══━` (the primary's full-run example `═══━╍╍╍╍╍` is three ranks; the 5 minutes drop). 1y 5mo is `⬤◉◉` (4 of 5 months), matching the primary's two-unit example.
- **7 seconds is `╶··`, not seven dots.** The table's max-count on `·` is 4; mixed-radix is the law. The primary's `·······` is a full-run illustration that exceeds it.
- **`age` in `layout.far-right` shares the `columns.heat` toggle.** `columns.heat = off` removes density *and* the age column (today's "off removes the cluster"), which is why snapshots pinning heat off stay time-stable.
- **Quiet mtime uses `format.mtime` too** — recent files outside git now speak SIGNA in the mtime column. Same format, different place.

**Root facts / file-as-PATH (asked to say so).** Header lines do not pay the density block. With density the root's *heat* is not shown as a number (children's blocks carry aliveness). The facts line *does* still show unlabeled SIGNA age (`━━╍╍  [has: …]` in dogfood) — not heat; I didn't invent a label. A file-as-PATH under density shows `age <signa>` and no heat at all (no block on the header, no children). Flagged rather than invented.

**Flagged for the design (not acted on)**

- **Unlabeled SIGNA on the root facts line** is cryptic next to `[has:…]` (`━━╍╍` with no heading above it). Previously the cluster was somewhat self-labeling (`0.71 · 0m ago`).
- **File-as-PATH under density has no heat at all.** If that's wrong, the header would have to pay the block, against the copied-path rule, or the facts line would have to carry the two cells.
- **Two-unit coarsening at hour/minute** (9.6h → `⚬⚬═` = 9h). The dropped remainder is the spec; worth knowing it is visible in dogfood.
- **Primary examples vs column form** as above (7s, 3h15m).
- **mtime compact at far-left** still unbuilt (`(unbuilt: mtime, bytes)`).

Suite: 314 tests green (302 + 11 lib (4 density + 7 SIGNA) + 1 score-path integration). Not committed, not `cargo install`.
