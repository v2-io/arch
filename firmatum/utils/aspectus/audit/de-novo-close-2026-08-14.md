# De-novo close audit — 2026-08-14 (evening)

*Fresh outside read (Fable 5), binary `aspectus 0.1.7+6a6dece.dirty (built 2026-08-14T21:51:59Z)`, release profile, macOS. Method: I probed the binary and read the code and design surfaces **before** opening `audit/`; the prior de-novo and hallway reports were read afterward, and everything below that overlaps them was re-verified against the current binary myself. Grounds per finding: **verified** = I ran it or read the exact lines; **struck me** = end-user impression from my own use. One environmental note: a commit landed in the enclosing repo mid-audit (21:53:51Z), which explains a heat-value shift I chased between runs — not a defect.*

*A caveat on my own commands: two "exit=0" readings early in my session were the exit of `head`, not aspectus; both were re-run clean before anything below was written. Every exit code stated here is from an unpiped invocation.*

---

## 1. Heat is repo-relative, so multi-repo looks — and `--sort heat` — invert the aliveness answer

**Verified** (behavior + `src/heat.rs` model). The decay ages in *commits behind that repo's HEAD*, normalized so touched-every-commit → ~2 — per repo. So a repo dormant for months whose last few commits concentrated on the same files carries ~1+ forever, while a large, furiously active repo's max leaf can sit at 0.3.

Specimen: `aspectus --sort heat ~/src` ranks, as the estate's hottest places: `memorata/` 1.31 (2.1w dormant), `embeddings/` 1.19 (2.8mo), `behavioral-floor/` 1.16 (2.8mo), `ruby-community/` 1.08 (**7.1mo**) — while `arch/` (0.33), where today's work actually lives, doesn't make the top screen. The default recency sort saves the ~/src default look, and the paired age is the tell for a careful reader — but the number/color channel is the *magnitude* channel, and at any multi-repo root it answers "which repo's recent commits were concentrated" while reading as "where is work alive." As the intended agent end-user: at `~/src` I would have mis-weighted the estate on first contact if I sorted by the fact the tool itself calls aliveness.

Nothing in `design/heat.md`, help, or the lattice scopes the score as within-repo-comparable-only. Options are yours (cross-repo normalization by wall-clock; suppressing/flagging heat sort above repo boundaries; a gloss in help), but the *claim scope* seems to need writing wherever heat is defined. Note the killer-look (`--depth 1 ~/src`) is exactly the surface where this bites.

## 2. `Cargo.toml` is a heat-noise basename — which makes a freshly-touched Cargo.toml the *loudest* row in the look

**Verified** (`src/heat.rs` `NOISE_BASENAMES = ["Cargo.toml", "SOURCE_REV"]`, adopted verbatim from git-heat per `design/heat.md`; behavior on this crate's own root). Chain: noise ⇒ `heat: None` ⇒ the heat cluster renders empty ⇒ quiet's redundancy guard (`src/columns.rs:470`) doesn't fire ⇒ the raw mtime speaks in the quiet mtime column. Today's root look:

```
├── Cargo.lock       7          0.27 ·    6m ago
├── Cargo.toml      15  6m ago
```

Same commit, same mtime — one row in the heat register, its sibling alone in a different column. Three separable problems:

- **The silencing self-defeats**: the fact suppressed as noise re-emerges as the visually most anomalous cell in the level (and drags a `mtime` heading into existence for everyone).
- **The row is arguably inverted for this ecosystem**: `Cargo.lock` (mechanical churn) carries heat; `Cargo.toml` (dependency *intent*) is erased. git-heat's tuning context may have justified it; as world-law in a tree-glance it reads wrong — I edited-my-deps is exactly a where-is-work-alive fact.
- **Undocumented**: help describes heat with no mention that some basenames claim nothing; JSON silently omits `heat` for them (fact absent — consistent with the absent-never-faked law, but a reader can't distinguish "noise-listed" from "never committed").

`design/heat.md` says the noise map "likely merges with the furniture/config map rather than living separately" — that open is now load-bearing.

## 3. JSON `truncated` does not see the hidden-furniture `≥` floor — code contradicts its own impl note

**Verified** (`src/json.rs:285–292` vs `impl/json.md` §truncated). The rule as written: truncated = "any denial, walk-bound cut, mid-iteration error, mount stop, **or `≥`-floored aggregate anywhere**." On this crate at defaults, the root carries `hidden: [{kind: "build", files: 19063, bounded: true}]` — a walk-bounded `≥` floor — and top-level `truncated` is `false` (`aspectus --format json --depth 1 .`). `truncated()` checks `leftover.bounded` and `mass.bounded` but never `has_counts`. Either the code grows one clause or the impl note carves furniture floors out deliberately — as stands, a machine caller branching on `truncated` per the documented contract gets a false all-clear. (With a *visible* bounded cut it works: `--walk 60` ⇒ `truncated: true`, verified.)

## 4. An unexpanded directory's mass drifts ~50% with the look's depth; the smallest figure is the one marked exact

**Verified**, minimally isolated: `aspectus --depth {1,2,3} --lines 60 ~/src` gives `memorata/` (unexpanded in all three) `≈28k` / `~37k` / `~44k` lines. The register discipline itself is working — the ≈/~ distinction and the constant `EST_BYTES_PER_LINE` fix (`src/n_level.rs:306`) landed, and the drift is honestly confessed as estimate — so this closes most of grok's "totals unstable" finding. What remains: `EST_BYTES_PER_LINE = 32` overestimates prose-heavy trees by ~50–60% (markdown runs 60–80 bytes/line, more since md-press unwrapped paragraphs estate-wide), and *which* dirs degrade to `~` is a function of what the walk read first, so the same dir's number still moves flag-to-flag within its marked register. A per-kind bytes-per-line prior (or calibrating the constant against a few real trees) would shrink the drift cheaply; not prescribing, sizing.

## 5. Quiet's mtime leg is an absolute window, not a sibling norm — a fresh tree speaks mtime on every line

**Verified** (`src/quiet.rs:288–293`, deliberate per its comment "truthfully wall-clock-relative"; behavior on a fresh fixture: every file `0m ago`, all sibling-identical). Outside git (no heat cluster to absorb it) a recently-created tree gets a fully-populated mtime column — the one fact the level's norm can *never* silence is the one help frames under "Norms come from the full level, so --lines cannot flicker them." The code's rationale is defensible (recency is about now, not about siblings); the help/design framing overspells it. One sentence in help scoping mtime as window-based (and perhaps a level-uniformity damper — when every sibling speaks, nobody is surprising) would reconcile claim and behavior.

## 6. Day's prior findings — state at close (all re-verified on 0.1.7, not inherited)

- **Walk-bound level confession** (morning finding 1, was highest-severity): **fixed.** `--walk 60 .` — unstatted root siblings appear as `[+ dir×2 · md×6 · …]`, `[walk bound]` on the cut dirs and header, `--explain-budget` reports the bound. The tree no longer looks finished.
- **Bounded-walk determinism** (finding 2): **fixed** as far as observable — two bounded runs byte-identical (`diff` clean).
- **Allocator under-spend** (finding 9): **fixed** — the asf look now spends 80/80 (was 62/80); `--explain-budget` shows redistribution and honest `unspent … (tree exhausted)` only when true.
- **`aspectus -- --help` refusal class** (finding 11): **fixed** — now `not found`, exit 2.
- **grok's `--explain-budget ~/src` hang**: **fixed** — 1.3s. **Perf regression**: **fixed** — `~/src` depth-1 in 0.37s.
- **grok's `--inspect no-such-kind` silent exit 0**: **fixed** — named refusal listing the known kinds, exit 2.
- **`.pytest_cache` kind-git misfire**: not directly re-tested, but the `kind:`→`has:` rework landed and `has:` claims on my looks were accurate everywhere I checked (verified on ~6 roots).
- **Contract floor** re-probed on 0.1.7: refusals classed and menued (`not found` / `unknown option` / `not a sortable fact`), exit 2, JSON refusals as structured objects on stderr in machine mode (exit 2 — verified unpiped); `--lines 1/2` overshoot to the documented 3-line floor rather than omit; broken symlink says `[broken]`; cycle says `[cycle]`; fifo doesn't hang the count; denied renders `[denied]` exit 0; SIGPIPE clean. `cargo test --release`: ~250 tests, all pass.

## 7. Smaller findings

- **Verified:** `has:` entries can claim `≈0f` (`agents ≈0f`, e.g. `~/src/causal-language`, `~/src/memorata`) — a furniture presence whose census says zero files reads as a contradiction ("has agents, none of them"). Presumably an empty or gitignored `.claude/agents/`; either suppress the count at zero or let the mark say what zero means.
- **Verified:** suffix-bucket censuses produce junk kinds on man-page/numbered files: `~/src/_ref` census reads `… 1×1 · 2×1 · 3×1 · 4×1 · 5×1 · 6×1 …`. Cosmetic, but it spends glyphs teaching nothing; an `other` fold for single-file numeric suffixes may be worth a config default.
- **Verified (framing only):** help says `--depth N … (default 2)` as static law while the caller stack routinely overrides (my user-home config sets `depth = 3`, so every "default" look I ran all session was depth 3 — `aspectus config` disclosed it, which is the designed answer). An agent quoting help's "default 2" to another agent transmits a falsehood about the effective channel. Maybe nothing to do; noting because help-is-law is this project's own doctrine and the two law surfaces can disagree per-machine.
- **Struck me:** a symlinked dir escaping the locus is followed and expanded by design (origin-ratified, `design/links-and-fs.md`) — my fixture's `link-out -> /etc` printed /etc's guts inside the look, budget and all. The `->` target on one line is the only tell that a whole subtree isn't under the root. For agents-first use (looks get pasted into contexts, shared, reasoned over) an out-of-root subtree is a scope/privacy event the eye can miss at depth; whether it earns a mark like `[cycle]`/`[other fs]` do is a design question I'd at least want asked in links-and-fs's Open list.
- **Struck me:** the root line's age is the root inode's mtime, which for a dir means "last direct add/remove" — asf's root said `0.35 · 2.2w ago` directly above children at `44.6h`/`2.2d`. Honest fact, stale-feeling glance; the root is the one line where "newest beneath" might serve the living-system aim better than the inode fact.
- **Verified:** `PRACTICA.md` "Now" section still narrates Wave C/D in flight; Waves through E are landed and committed (outline + version ritual current). Referent-drift discipline says the live-effort surface should move at the seam; it's ~2h stale on a same-day cadence, so noting rather than pressing.

## 8. End-user testimony (the part you asked for as primary evidence)

I came to this cold, and the experience matched the hallway witnesses I only read afterward: the `~/src` depth-1 look is genuinely the killer artifact — 30 repos with branch/sha/dirty/mass/aliveness in one screen; `[has: …]` answered "what is this place" before any README; the censuses discharged the go-probe-it anxiety `tree` leaves. The refusals are the best I've seen in a CLI — every wrong move I made taught me the right one, with the class named and a next action. The help page's def-prose paid off in one read.

Two friction points from my own use, both already on the pipeline: (a) the ~/src look's long lines — census + mass + git + has on one line runs past 300 chars and my eye lost the fact columns repeatedly; the `vertical-info` row's diagnosis is exactly right and, for what it's worth, it's the row I'd want next as a user. (b) heat needed a trip to `design/heat.md` before I trusted what it was — the score is unglossed at first contact (third independent arrival on this; the two hallway testers said the same).

---

*Nothing fixed, nothing committed. Staying on the line for follow-ups.*
