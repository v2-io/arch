# Ignored-dir bytes — finish note (2026-10-03)

Landed per [[../design/ignored-bytes|design]], at the widened scope Joseph decided mid-slice: `⊘` lines **and** hidden furniture.

**What the binary does now**

- A gitignored dir's line carries its body's bytes in the far-right `bytes` column: `⊘ ├── .venv/ … ≥  892.0M`.
- A hidden furniture dir's has-word carries them after its file count, from 1 MiB up or when a floor: `[has: build ≥16764f ≥5.8GB, …]`, `[has: agents ≥12150f ≈2.8GB, …]`.
- One body is weighed with one readdir walk plus one `lstat` per non-dir. Hardlinks count once, symlinks are never followed, and the walk stays on one filesystem. The cap is 20,000 names per body, then `≥`.
- The weighing is never on the `--walk` budget, never in mass, and never opens the body.

**Code**

- **`src/n_level.rs`:**
  - `Body { files, bytes, files_bounded, bytes_bounded }`.
  - `weigh_body` / `weigh` replace `count_names`, so one walk yields both figures. File-count semantics are unchanged: symlinks and specials floor the count, same cap.
  - `hidden_phase(node, abs, one_fs)` is now the body phase. It collects hidden dirs *and* unread `⊘` nodes in one traversal order, runs them on the same bounded pool, and assigns `has_counts: Vec<(String, Body)>` and `Node.body`.
  - `Node.unread` marks `stat_only` nodes, the only ones weighed as `⊘` bodies. Under `--show-all` ignored dirs are expanded, so they are not "unread" and not weighed.
  - `BODY_NAME_CAP` replaces `HIDDEN_COUNT_CAP`, same value.
- **`src/ready.rs`:** `fmt_body_size` (count cell, or raw with `≥`) for unread dirs in the `bytes` column, ahead of the quiet rule. `has_block` appends `body_bytes_compact` when `bytes ≥ HAS_BYTES_SPEAK_AT` (1 MiB) or bounded.
- **`src/count_cell.rs`:** `compact()`, the count cell squeezed for prose (`≈15.0GB`, `2·428B`, `512B`), with a unit test.
- **`src/json.rs`:** `hidden[].bytes` / `bytes_bounded`; `ignored_body {files, bytes, …}` on `⊘` nodes. `truncated` counts byte floors too.
- **`src/main.rs`:** passes `one_fs`. Help says ignored dirs say how big they are; it previously said "unweighed", which is now false and has been replaced. The furniture paragraph names has-word bytes.
- **`src/facts.rs`:** `bytes` and `has` derived-from say where bytes now come from.
- **Tests:** `tests/ignored_bytes.rs` (7), `n_level::body_tests` (2: cap floors, hardlinks once / symlinks unfollowed), `count_cell` compact test. No golden moved, because the fixtures have no hidden body ≥ 1 MiB and no `⊘` dir. Suite 332 green.

**Cost** (warm, best of three, installed v0.1.19 → this build, seconds)

| Look | v0.1.19 | this build |
|---|---|---|
| `~/src/arch --lines 300 --depth 4` | 0.40 | 0.51 |
| `~/src --depth 1` | 0.43 | 0.46 |
| asf | 0.14 | 0.14 |
| this crate | 0.17 | 0.21 |
| grok-build | 0.25 | 0.28 |
| vivarium `--depth 3` | 0.12 | 0.15 |

Cap alternatives measured on arch: 100k → 1.13 s, 250k → 2.43 s, 1M → 2.56 s. At this crate, 250k → 1.28 s.

- One cold first run measured 1.25 s on arch. It was not reproducible warm, and the cold cost is not separately measured here.
- Eight threads vs one: on arch, wall 0.48 vs 0.60 s. `lstat`s scale poorly across threads on APFS: `sys` stays ≈ 0.9 s at every thread count.
- Arch's old ≈0.4 s floor (impl/gitignore-bodies.md) is now ≈0.5 s.

**Dogfood**

- `~/src --depth 1` changed 13 lines, all has-words gaining bytes. Among them is `limen/`'s `[has: agents ≥12150f ≈2.8GB, …]`: `du -sh ~/src/limen/.claude` = 2.8G, a real disk finding a look had never shown.
- arch: `⊘ .venv/ ≥ 892.0M` (`du` 1.1G), `⊘ results/ ≈ 49.6M`, `⊘ vendor/ ≈ 38.8M`; asf: `⊘ .build-scrbook/ ≈ 36.5M`. 18 lines changed.
- This crate changed one line, `[has: build ≥17942f ≥1.1GB, …]`; `du` 6.7G.
- grok-build: `[has: build ≥17643f ≥3.9GB, …]`; `du` 15G.
- vivarium: `build ≥16764f ≥5.8GB`; `du` 32G.
- aat-refactored: byte-identical.

**Calls made** (all in the design, with reasons):

- Apparent size, not blocks.
- A 20k cap per body.
- `columns.size = off` silences `⊘` bytes.
- The has-word form (squeezed count cell) and its 1 MiB threshold.
- Hardlink dedupe per body.
- `⊘` bytes always speak.

**Flagged** (design Open):

- The floors read 2–5× under `du` on big build trees.
- `.git` is still unweighed.
- The single-line-for-huge-masses idea.
- A kind-bearing omit row would be weighed.

## `.git` weighed (2026-10-03, same day; Joseph: "yes, IMO")

- **`src/furniture.rs`:** `speaks_for_itself` is now `github` only, so `.git` dirs join `hidden_dirs` and the body phase weighs them. Gitlink *files* (submodules, linked worktrees) are not dirs and are not weighed.
- **`src/ready.rs`:** the `git` has-word speaks bytes only (`git ≈654.2MB`), with the same 1 MiB / floor rule and no `≈Nf`.
- **Help + `facts.rs`:** say so.
- **Test:** `ignored_bytes::git_store_weighed_bytes_only` (a bare word under 1 MiB; `[has: git ≈3.0MB]` above; JSON `hidden[]` kind `git`). Suite 336 green. No golden moved: the git-repo fixture's store is under 1 MiB.
- **Dogfood:** `~/src --depth 1` changed 21 lines, all git words gaining bytes. Checked against `du`:

  | Store | Look says | `du` says | Note |
  |---|---|---|---|
  | arch | `≈654.2MB` | 664M | Includes `modules/` at 634M, the submodules' stores. |
  | grok-build | `≈55.4MB` | 56M | |
  | limen | `≈56.5MB` | 57M | |

  arch and grok changed 1 line each. aat, asf, vivarium, and this crate are byte-identical. asf and vivarium are submodules, so their word stays bare.
- **Cost** (warm, best of 3): arch 0.53 → 0.58 s; `~/src`, asf, this crate, grok, and vivarium unchanged to the hundredth.
- **Calls:** has-word over facet; bytes only. Reasons are in the design §`.git`. The huge-mass-own-line Open now carries Joseph's verbatim reason it stays open; nothing was built for it.
