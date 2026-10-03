# Ignored-dir bytes — how big is what the look does not open

A body the look declines to open still says how big it is. That covers a gitignored directory (`⊘`) and a hidden furniture directory (`target/`, `.venv/`, `.claude/`, `.archive/`). "Why is this directory huge" is a natural question to bring to a look, and before this row the look pointed away from the answer.

**Demand** ([[../audit/inbox-2026-10-03|inbox, verbatim]]). An Opus 5.5 instance hunting disk for Joseph (2026-09-24) found the answer inside an unopened body twice in one session: `target/` at 110G of a 116G checkout, then `models/` at 36G of safetensors. Both were disclosed (`[ignored×1]`, `⊘`), so nothing was hidden by omission, but neither said its size. Its wish: *"a cheap byte mass on ignored/⊘ entries (even `≥` or approximate)."*

**Decided** (Joseph, 2026-10-03), in two steps:

1. The coordinator's read was "a byte total on the `⊘` line itself, walk-bounded and `≥` when cut, never counted in the parent's mass." Joseph: *"I agree with your principled instinct."*
2. Once in the code, the 110G `target/` turned out to be **hidden furniture**, not a `⊘` line. It renders as `[has: build ≥17643f]` on its parent, because furniture fates apply before ignore rules (design/gitignore-bodies.md §Order with furniture). The first shape would not have reached it. Joseph: *"Yes, please on making sure that the target/ somehow gets its information delivered as appropriate however you see fit, in addition to ignored stuff."*

## The law

- **One weighing, two kinds of body.** Each unopened body (a `⊘` dir, or a hidden furniture dir other than `.git`/`.github`, whose facets speak for them) gets one walk. That is readdir for the tree plus one `lstat` per non-directory entry for its size.
- **What "bytes" means:** Σ `st_size` (the apparent size, the same number the `bytes` fact is for a file) over every non-directory entry in the body. Hardlinked inodes count once per body (cargo's `deps/` hardlinks its binaries; counting them twice would double a `target/`). Symlinks weigh their own inode and are never followed. Directories add nothing. The walk stays on the body's filesystem unless `--no-one-fs`.
  - *Call:* apparent size rather than allocated blocks (`du`). The `bytes` column already means `st_size` for files, and one column must not mean two things. The two agree closely on build trees; sparse, compressed, or cloned files read larger here than in `du`.
- **Bounded, and never silently.** One body visits at most **20,000 names**. Past that, or on an unreadable entry or a mount, both figures are floors (`≥`).
  - The cap is per body. It is **not** the `--walk` budget, and ignored dirs still cost the walk nothing (design/gitignore-bodies.md subfeature 9 holds). It is not mass's name cap either.
  - 20,000 is the old hidden-count cap, so hidden file counts do not move.
  - *Why not higher* (measured 2026-10-03, warm, best of 3; impl note has the table): at 20k, `~/src/arch --depth 4` costs 0.41 → 0.51 s. At 100k it is 1.13 s, and at 250k it is 2.43 s. Single-threaded `lstat` runs about 5 µs per name on this machine, and vivarium's `target/` alone is 118k names.
  - *The price, knowingly paid:* big build trees read as floors well under the truth. vivarium `target/` reads `≥5.8GB` (`du` 32G), this crate's `≥1.1GB` (6.7G), grok-build's `≥3.9GB` (15G). The floor still answers "is this where the disk went", and it never claims less than the truth.
- **Never mass.** No body's bytes or files enter any aggregate: not the parent's mass, not its census, not a dir byte total when that row lands. Mass stays the project's own weight (design/mass.md). The body's figure is a fact of the body's own line or has-word.
- **Never opened.** Weighing reads names and sizes only. No content is read, nothing inside prints, and there is no census of the innards.

## Where it shows

**A `⊘` line: in the far-right `bytes` column**, as a count cell (`≈    2.9M`, `≥  892.0M`).

- It speaks whenever the column exists, which it does under the quiet default. It is existence information, like mass at a cutoff, not a surprise.
- `format.size = bytes` gives the raw integer (`3050000`, `≥…` when a floor).
- *Call:* `columns.size = off` silences it. The caller's explicit ask wins, as `lines` off silences mass. The `⊘` still says the dir is there.

**A hidden furniture dir: on its has-word, after the file count**, e.g. `[has: build ≥17643f ≥3.9GB, git, rust]`, or `agents ≥12150f ≈2.8GB`.

- *Call, form:* the has-block's internal form waits on the undecided subgroup-subject form ([[grid-cleanup|grid-cleanup]] §The count cell, "Not decided here"). Meanwhile bytes ride in the count cell squeezed for prose (`count_cell::compact`): the same mark, digits, scale, and unit, without the padding, and no `.` when no fraction follows. This moves with the has-block when that form lands.
- *Call, quiet threshold:* a has-word's bytes speak at **≥ 1 MiB, or whenever they are a floor**. Below that, a body answers no disk question (`agents ≈1f 6·010B`, `build ≈3f ≈69.1KB`) and only lengthens the widest near-right part, the first candidate to spill (grid-cleanup §Decisions). `⊘` lines always speak, because their bytes sit in an aligned cell that costs no inline width.
- Several hidden dirs claiming one kind on one line sum, as their file counts already did. Hardlinks are deduplicated within one body, not across sibling bodies.

**JSON:**

- `hidden[]` entries gain `bytes` and `bytes_bounded` (`bounded` stays the file count's floor, as before).
- A `⊘` node gains `ignored_body: {files, bytes, bounded?, bytes_bounded?}`.
- Any floor sets the look-level `truncated`, by the same rule hidden counts already followed.
- Additive within schema 1.

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | `⊘` bytes | An ignored dir's line carries its body's bytes in the `bytes` column; the column appears for it. | `tests/ignored_bytes.rs::ignored_dir_says_how_big` |
| 2 | Hardlinks once | A hardlinked file weighs once (2.9M, not 5.8M). | same; `body_tests::hardlinks_once_symlinks_unfollowed` |
| 3 | Raw format | `format.size = bytes` gives the integer. | `raw_bytes_format` |
| 4 | Off is off | `columns.size = off` silences it. | `size_off_is_off` |
| 5 | Off the walk budget | `--walk 3` over a 33-entry ignored body trips nothing. | `weighing_is_off_the_walk_budget` |
| 6 | JSON | `ignored_body`; `hidden[].bytes`. | `json_ignored_body`, `hidden_furniture_has_word_bytes` |
| 7 | Has-word | ≥ 1 MiB speaks; a tiny body keeps its count only. | `hidden_furniture_has_word_bytes` |
| 8 | Floors | The name cap floors both figures, never silently. | `body_tests::cap_floors_bytes_and_files` |
| 9 | Help | Help says ignored dirs say how big they are, and that has-words carry bytes from 1 MiB up. | `help_teaches_ignored_bytes` |

## Open

- **Floors on big build trees read 2–5× under the truth** (numbers above). Ways to raise the cap without the cost, none taken here:
  - (a) `getattrlistbulk` on macOS: names and sizes per directory read, not per name. Zero-dependency means hand-declared FFI.
  - (b) The [[cache|Cache]] row: a body's weight keyed and reused.
  - (c) Extrapolate past the cap from the visited names' mean, marked `~`. That is an estimate where today there is a floor, and a byte distribution as skewed as a `target/`'s (tiny fingerprints, GB `.rlib`s) makes it a poor one.
- **`.git` is not weighed** (its facet speaks; *"`.git`'s object store is not the repo's working weight"*, design/furniture.md). A multi-GB `.git` is a real answer to a disk question. Whether the git facet should carry its object store's size is Joseph's call.
- **"Very large hidden masses may earn a single line"** (design/furniture.md, unratified). A 32G `target/` now says its size on a has-word. Whether that size should promote it to its own `⊘`-like line is still Joseph's.
- **The 1 MiB has-word threshold** is a constant, not a quiet law. A statistical form (relative to the look's visible bytes) waits on dir byte totals.
- **Kindless hidden names are not weighed.** Weighing follows the has-word, so a hidden dir whose map row claims no kind has nowhere to say its size. The only shipped row like that is `.DS_Store = ":omit"` (*not listed, not mentioned*), which is a file. A user-config row that omits a *dir* with a kind would be weighed onto that kind's word. That is consistent with the has-block already naming the kind, but it is not what "not mentioned" promises.

## Foundations

[[gitignore-bodies|Ignored bodies]] (presence without innards; walk-cost law) · [[mass|Mass]] (mass excludes ignored bodies) · [[walk-bound|Walk bound]] (`≥` honesty) · [[furniture|Furniture]] (presence survives hiding; hidden counts) · [[grid-cleanup|Grid cleanup]] §The count cell (bytes form) · [[lattice-2|Lattice 2]] (`bytes` row).
