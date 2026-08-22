# Grid cleanup — ready-facts, cells, rows

*Row opened by Joseph, 2026-08-15, on finding the `aspectus config` fact inventory "very wrong": its `place` words are inverted against what renders, and it is missing places the designs already specify. Before any density work ([[vertical-info|Vertical info]]) the renderer needs an actual model — his framing, verbatim:*

> *what I need us to do is to figure out the design and make sure we're going from  [raw-facts] -> [ready-facts] -> [cells: <glyphs,tree-lines,<name|vsummary>+decorators,near-right\*,far-right\*>] -> [rows (not just one implicit)]    Or something-- but we need to keep everything in the name or vertical-summary + decorators "column" in their column, and we need a way to get horizontal summaries and sizes and has and kind blocks cleaned up and in their columns. And we need to get them to fit while also having far-right aligned columns. etc.*

## Why the grammar is this careful — the native-agentic format (Joseph, 2026-08-22)

Human tools got alignment + color + icons; machine tools got JSON. The reader in between — one who wants **alignment without color, a few meaningful glyphs without tofu, units on every number** so the glance needs no parser and no arithmetic — never got a tool built for them. Joseph's framing, verbatim: *"our secret sauce in some ways is that humans have been developing utilities with a kind of default design instinct: humans? alignment and color/ansi and nerdfont glyphs etc. computers? structured output (e.g., json w/ no color). Almost all missing the sweetspot where positioning and delicate and parsimonious use of unicode (but not enough to throw off your internal alignment mechanisms) and positioning without color as the 'native agentic' — essentially kind of what you do constantly whenever you massage your shell output into a python statement giving it headers and reporting the various pieces in a format that makes it 'glanceable' without colors and without running a parser in your head."* The self-observation is the proof: agents rebuild this format by hand (`column -t`, a heading line) at token cost in the inner loop, because nobody shipped it.

Two facts about the reader that this rests on, both his, both measured before this tool existed: **agents have spatial awareness** — alignment is load-bearing for them, not a human courtesy (one of his earliest LLM experiments; it shaped udon, this tool, and `~/src/_gems/rubocop-tablecop/`; demonstrated on the coordinating instance 2026-08-22 with two renderings of one table — the ragged one had to be *parsed*, the aligned one was *read*); and the inner loop compounds — *"one of those subtle things that will make a huge difference over time."* That is why the significant digits line up in the count cell. [[lattice-2|Lattice 2]] is the fact table this grammar serves.

## What is wrong today (verified 2026-08-15 against `src/columns.rs`)

A node renders as one implicit `Row { tree-prefix, name+deco, cells[], rest }`, painted as: **name-column → far-right cells left-anchored at the name tab-stop → then `rest`** (every near-right part joined by two spaces, ragged). So:

- The "far-right" facts (`lines`, `heat · age`, …) sit *immediately right of the name*, not right-aligned to any edge; the "near-right" material (censuses, mass, `[git: …]`, `[has: …]`, marks) trails *after* them, out at the far right — the inventory's `place` column says the opposite of what the eye sees.
- `rest` is one string in **code order** (`rest_of`): title → kind word → globify count → census → mass lines → `[ignored×N]` → facets → `[has: …]` → `⊘` → `[denied]` → `[cycle]` → `[other fs]` → `[unreadable: io]` → `[walk bound]`. No stable column per part; the eye cannot scan any of them vertically.
- Three of `design/shorthand.md`'s six placement classes (far-left, near-left, glyph-block) have no tenants; the offices that are not line-placements (census bucket, deep-agg, weight) have no inventory row at all; `readme-title` is filed near-right in code, decoration in the shorthand table.
- `--lines` counts physical rows; nothing can wrap; the symlink decoration is name-fused, so a long target shoves every cell (Joseph's inbox specimen).

## The pipeline (proposed — Joseph's stages, my fill; ratify per stage)

```
raw-facts  ──▶  ready-facts  ──▶  cells  ──▶  rows  ──▶  paint
(Node)          (form+place+     (the row     (1..n per   (widths, stops,
                 office+marks,    grammar,     node; sub-  right edges,
                 quiet applied)   fixed order) rows own    color)
                                               no cells)
```

1. **raw-facts** — what walk/obtains/quiet already put on `Node`. Unchanged.
2. **ready-facts** — each fact rendered to `{text, place, office, unit/glyph, marks}` by *its own* formatter, quiet already applied, per node, no neighbor knowledge. The table below is the contract; `facts.rs` becomes this table (and `aspectus config` prints it truthfully, `place` *and* `office`).
3. **cells** — the row grammar, fixed order, left to right:
   `glyphs` (far-left glyph-block, fixed width, empty until it has tenants) · `tree-lines` · **`name-col`** = name **or** vertical-summary (a census standing where a name would) + decorators · **`near-right*`** = one sub-column per part-kind at a stable stop across the look (order proposed: title · kind-word · globify-count · census · marks · facets · has) · **`far-right*`** = numeric fact columns, right-aligned to a common right edge.
4. **rows** — a node emits a primary row plus zero or more **owned sub-rows** (indented under name-col, no far-right cells) for material that spills; **`--lines` counts logical lines** (a node with sub-rows costs one) — Joseph's inbox decision.
5. **paint** — computes stops and the right edge from the look (never the terminal), pads reserved sub-columns, trims trailing space. Determinism law unchanged.

## The ready-facts table (draft 2026-08-15 — carved from `columns.rs` / `n_level.rs`; **proposed** columns are mine)

Legend — **office**: `line` (a fact of this node's own line) · `census` (a bucket in a dir/leaf census) · `deep-agg` (aggregate below the line, [[mass|Mass]]) · `weight` (allocator only, no glyph) · `mark` (a fact about the look, not the inode). **place today** = where the current paint actually puts it. **place (design)** = shorthand/lattice's word. **proposed** = where this row proposes it lives after cleanup (⇒ *sub-row* = spills to an owned sub-row when it doesn't fit).

| #   | fact                                          | source              | ready form(s) today                                                                              | office                            | place today                | place (design)                                                  | proposed                                                                                           | default / quiet                            | marks                                |
| --- | --------------------------------------------- | ------------------- | ------------------------------------------------------------------------------------------------ | --------------------------------- | -------------------------- | --------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | ------------------------------------------ | ------------------------------------ |
| 1   | name                                          | readdir             | `name`, dirs `name/`; globified `out-[001-047].bak`                                              | line                              | name-col                   | decoration                                                      | **name-col**                                                                                       | on                                         | —                                    |
| 2   | vertical-summary (leaf census)                | allocator           | `[+ md×5 · txt×2]`, `[+ dir×2 ≈59f · md×27]`, `[+ ≥ …]`, `[+ single.name]`                       | census                            | own row, name-col position | (none — lattice `leaf-census` office)                           | **name-col** (a summary standing where a name stands)                                              | on, not quietable                          | `+`, `≥`                             |
| 3   | dir census                                    | depth/budget cutoff | `[dir×3 ≈120f · md×31]`, `[furniture/ ≈2f · md×2]`, `[thing.dat]`, `· ignored×N` inside, `[≥ …]` | census (about the line's subtree) | rest, ~4th part            | near-right (lattice `dir-census`)                               | **near-right: census column**; long ⇒ sub-row                                                      | on, not quietable                          | `≈` `≥`                              |
| 4   | mass lines                                    | deep walk           | `≈61k lines`, `~5.0M lines`, `≥434k lines`                                                       | deep-agg                          | rest, after census         | near-right (mass.md); **Joseph 2026-08-14: the `lines` column** | **far-right `lines`** (dirs), marks kept                                                           | on                                         | `≈` `~` `≥`                          |
| 5   | mass files                                    | deep walk           | `≈338f` inside the census dir bucket; `agents ≈1f` inside `has:`                                 | deep-agg                          | inside census / inside has | (census)                                                        | stays inside census / has                                                                          | on                                         | `≈` `≥`                              |
| 6   | line-count (file)                             | read                | `816`                                                                                            | line                              | far-right, left-anchored   | far-right                                                       | **far-right `lines`** (right-aligned)                                                              | on                                         | absent past read budget              |
| 7   | heat · age                                    | git log + mtime     | `0.44 · 8.7h ago`, `0.44`, ` · 8.7h ago` (git-known, unscored)                                   | line                              | far-right cluster          | far-right                                                       | far-right, two sub-columns; **dangling `·` to decide**                                             | on in-repo                                 | absent outside git                   |
| 8   | mtime (quiet/on)                              | stat                | `2.2h ago` / iso-8601 / epoch                                                                    | line                              | far-right                  | far-right                                                       | far-right `mtime`                                                                                  | quiet (redundancy-guarded vs cluster)      | —                                    |
| 9   | size                                          | stat                | `1.2M`, `20M`, bytes                                                                             | line                              | far-right                  | far-right                                                       | far-right `size`                                                                                   | quiet (outlier)                            | —                                    |
| 10  | permissions                                   | stat                | `700`, `4755`                                                                                    | line                              | far-right                  | far-right                                                       | far-right `perms` — or **glyph-block** later                                                       | quiet (odd for level; special bits always) | —                                    |
| 11  | owner                                         | stat                | `root`, `user:group`, ids                                                                        | line                              | far-right                  | far-right                                                       | far-right `owner`                                                                                  | quiet                                      | —                                    |
| 12  | initial-sha / latest-sha                      | git log             | `3a705fe`, `H~4`, full                                                                           | line                              | far-right                  | far-right                                                       | far-right (compose)                                                                                | off                                        | absent past log window               |
| 13  | git facet                                     | `.git` plugin       | `[git: remote<host/path> br<main> @sha dirty<N>]`, `detached`                                    | line (identity claim)             | rest, near end             | near-right                                                      | **near-right: facet column** ⇒ sub-row (this is the widest part)                                   | on                                         | —                                    |
| 14  | github facet                                  | `.github` plugin    | `[github: N workflows]`                                                                          | line                              | rest                       | near-right                                                      | facet column                                                                                       | on                                         | —                                    |
| 15  | has block                                     | furniture map       | `[has: agents ≈1f, archive ≥1186f, git, …]` (kind + hidden mass)                                 | line (contents claim)             | rest, last                 | near-right                                                      | **near-right: has column** ⇒ sub-row; **kind-glyph-block** candidate later                         | on                                         | `≈` `≥` on hidden masses; `≈0f` open |
| 16  | kind word                                     | quiet               | `binary` / `text`                                                                                | line                              | rest, 2nd                  | near-right (lattice INFO)                                       | near-right, small marks column                                                                     | quiet (differs from level plurality)       | —                                    |
| 17  | globify count                                 | globify             | `(44 files)` / `(24 dirs)`                                                                       | line (about a collapsed listee)   | rest, 3rd                  | (unplaced)                                                      | **decoration** — it completes the collapsed name                                                   | on                                         | —                                    |
| 18  | README title                                  | peek                | `"Rowan"` (quoted, 60-cap)                                                                       | line                              | rest, 1st                  | decoration (shorthand) / near-right (impl)                      | **decide**: decoration inflates the name column; near-right keeps it — impl chose near-right       | off                                        | —                                    |
| 19  | symlink target                                | lstat/readlink      | ` -> target`, ` -> target [broken]`                                                              | line                              | fused to name (deco)       | decoration                                                      | decoration **⇒ first to spill to a sub-row** (Joseph's specimen)                                   | on                                         | `[broken]`                           |
| 20  | gitignored                                    | ignore stack        | `⊘` (+ dim on TTY); `[ignored×N]` at expanded level; `ignored×N` inside census                   | mark (per entry) / census         | rest / rest / census       | near-right                                                      | marks column (glyph) / census                                                                      | on                                         | —                                    |
| 21  | denied                                        | walk                | `[denied]`, `[unreadable: io]`                                                                   | mark                              | rest                       | near-right                                                      | marks column                                                                                       | on, not quietable                          | ⇒ `≥` on parents                     |
| 22  | walk bound                                    | walk                | `[walk bound]` on cut dir; header too                                                            | mark                              | rest                       | near-right                                                      | marks column                                                                                       | on                                         | ⇒ exact counts kept                  |
| 23  | cycle / other fs                              | walk                | `[cycle]`, `[other fs]`                                                                          | mark                              | rest                       | near-right                                                      | marks column                                                                                       | on                                         | —                                    |
| 24  | important                                     | config set          | (no glyph)                                                                                       | weight                            | —                          | (weight)                                                        | — (row exists in the inventory as *weight*, no place)                                              | on                                         | —                                    |
| 25  | prior-name / created / git-letter / linkcount | unbuilt             | `was …`, iso, letter, n                                                                          | line                              | —                          | near-right / far-right                                          | as lattice; unbuilt                                                                                | —                                          | —                                    |
| 26  | headings                                      | look                | `lines  heat ·       age`                                                                        | look                              | header row 4               | (columns.md)                                                    | far-right heading row over the right-aligned columns; near-right sub-columns may earn headings too | on when any column speaks                  | —                                    |
| 27  | root facts line                               | root                | root's cells + `rest` pre-joined                                                                 | look                              | header row 2               | overview-invariants                                             | same grammar as a node row (cells + near-right), so it aligns with the tree                        | on when any                                | —                                    |
| 28  | config drift                                  | config stack        | (unbuilt) — `depth = 3 (user-home)`, `--lines 200`                                               | look                              | —                          | overview-invariants §Config drift                               | header row after the stamp                                                                         | on                                         | —                                    |

**Not ready-facts but rows the renderer emits:** the stamp line, the root path line, the feedback footer (outside `--lines`), `--explain-budget` (stderr).

## The ready-facts table, normalized (2026-08-15 — decided forms applied; ⏳ = waits on the subgroup-subject form)

Name-column tenants are mutually exclusive: **Name(+decorators) | Glob-Template | Leaf-Census** — one thing standing in the name position. Counts everywhere are the 12-cell count cell (`g ␠ m T · NNN . f s u`); marks `≈ ≥ ~` sit in the cell's `m` slot, never glued to words. Under a heading the cell's `g`/`u` blank but keep width.

| # | fact | office | place (normalized) | form (normalized) | status |
|---|---|---|---|---|---|
| 1 | name (+ decorators `/`, `-> target`, `[broken]`) | line | **name-col** | `name`, `name/`, `name -> target` | decided |
| 1b | glob-template | line (one listee for many) | **name-col** | `output-[001-047].bak` + a count cell `● 44.  𝓃` (was `(44 files)`) — where the cell sits (decoration vs first near-right) ⏳ | decided form; place ⏳ |
| 2 | leaf census (vertical-summary) | census | **name-col** (a summary standing where a name stands) | `+` then subject cells: `+ □ 2. 𝓃  ● ≈ 59. 𝓃  ⏳md 27` (was `[+ dir×2 ≈59f · md×27]`) | ⏳ subgroup form; `+` kept |
| 3 | dir census | census | near-right: **census column** ⇒ sub-row when long | subject cells: `□ 3. 𝓃  ● ≈ 120. 𝓃  ⏳md 31  ⏳ignored 3` (was `[dir×3 ≈120f · md×31]`); one-name forms (`[thing.dat]`, `[furniture/ …]`) keep the name | ⏳ subgroup form; delimiter (`⟨ ⟩` candidate) ⏳ |
| 4 | mass lines | deep-agg | **far-right `lines`** (heading names subject+unit ⇒ `g`,`u` blank) | `≈ 61.2K` / `~ 5.0M` / `≥ 434.0K` (was `≈61k lines` in the tail) | decided (Joseph's inbox) |
| 5 | mass files | deep-agg | inside census (`● ≈ 120. 𝓃`) / inside has (⏳) | count cell | decided form; has-form ⏳ |
| 6 | line-count (file) | line | far-right `lines` | `816.` (bare cell under heading; `1·099.` exact below 10K) | decided |
| 7 | heat · age | line | far-right, two sub-columns | `0.44 · 8.7h ago`; unscored-git line: **dangling `·` still to decide** | open (glyph) |
| 8 | mtime | line | far-right | `2.2h ago` / iso / epoch | unchanged |
| 9 | size | line | far-right `bytes` | count cell, unit `B`: `190.0KB`, `9·021. B` (was `human_size`) | decided |
| 10 | permissions | line | far-right (glyph-block later) | `700`, `4755` | unchanged |
| 11 | owner | line | far-right | `root`, `user:group` | unchanged |
| 12 | initial/latest sha | line | far-right | `3a705fe` / `H~4` / full | unchanged |
| 13 | git facet | line | near-right facet column ⇒ sub-row | `[git: remote<…> br<main> @sha ⏳dirty 4]` — the dirty count becomes a subgroup cell | ⏳ (git facet to be discussed) |
| 14 | github facet | line | facet column | `[github: ⏳workflows 3]` | ⏳ |
| 15 | has block | line | near-right has column ⇒ sub-row | `[has: agents, ⏳archive ● ≥ 1·186. 𝓃, git]` — hidden mass as a subject cell | ⏳ (kind-glyph-block later) |
| 16 | kind word | line | near-right marks column | `binary` / `text` | unchanged |
| 17 | *(merged into 1b)* | | | | |
| 18 | README title | line | **decide**: decoration vs near-right | `"Rowan"` | open |
| 19 | symlink target | line | decoration ⇒ **first to spill to a sub-row** | ` -> target`, ` -> target [broken]` | decided (inbox) |
| 20 | git status | line | **glyph-block, one cell** | `⊘ M A ⁇ R U D` (`C T` available), blank when clean, worktree wins; **the separate `⊘` marks-column glyph retires** | decided |
| 20b | ignored remainder | census / line | census cell / marks | `⏳ignored 3` as a subject cell (was `[ignored×3]`, `ignored×N`) | ⏳ |
| 21 | denied | mark | near-right marks column | `[denied]`, `[unreadable: io]` — the word beats any glyph (both witnesses) | decided (word) |
| 22 | walk bound | mark | marks column | `[walk bound]` | unchanged |
| 23 | cycle / other fs | mark | marks column | `[cycle]`, `[other fs]` | unchanged |
| 24 | important | weight | — (inventory row shows office `weight`, no place) | — | decided |
| 25 | prior-name / created / linkcount | line | as lattice; unbuilt | — | unbuilt |
| 26 | headings | look | far-right heading row; near-right sub-columns may earn headings | `lines  bytes  heat · age` — a heading names subject and/or unit and blanks the cells' `g`/`u` | decided |
| 27 | root facts line | look | header row 2, same grammar as a node row | cells + near-right, aligned with the tree | decided shape |
| 28 | config drift | look | header row after the stamp | `depth = 3 (user-home)  --lines 200` (unbuilt) | decided (inbox), unbuilt |
| — | *empty directory* | mark | marks column | **open**: `[empty]` / a shape — not `∅` (kept apart from `⊘`) | open |
| — | *tokens* | line / deep-agg | far-right `tokens` (later) | count cell, unit `𝓉`, `~` by nature | deferred |

## The count cell — Subject × Unit (decided with Joseph, 2026-08-15; only the nailed-down parts)

Every number in the look is one point in **Subject × Unit**. Today five grammars say "N of something" (`md×31`, `dir×3 ≈120f`, `≈61k lines`, `(44 files)`, `dirty<4>`, `901G`) because the axes were never named. From here, one grammar:

```
Subject   = files · dirs · files+dirs · (a subgroup — suffix, kind, pattern, git state: NOT YET DESIGNED)
Unit      = count/ordinality (𝓃) · bytes (B) · lines (𝓁) · tokens (𝓉)
```

A count renders in a **fixed 12-cell field**, so alignment is free and every count in the look reads alike:

```
col  1   2   3   4   5   6-8   9    10   11   12
     g   ␠   m   T   ·   NNN   .    f    s    u

g  subject glyph: ● files · □ dirs · ▣ files+dirs · blank when a column heading names the subject
␠  always a space
m  qualifier mark: ≈ (exact, grouped for the eye) · ≥ (floor) · ~ (estimated) · blank (exact)
T  thousands digit, or blank
·  U+00B7 middot as thousands separator when T is present, else blank
NNN hundreds..ones, right-aligned, blank-padded (spaces, not dots — a token-reader sees dot-fill as content)
.  always present — the structural anchor every row shares
f  one fraction digit when the value is scaled, else blank
s  scale: K M G T P, or blank
u  unit: 𝓃 = count · B = bytes · 𝓁 = lines · 𝓉 = tokens (decided 2026-08-15 — see the unit-letter note below; the unit slot always speaks: blank-means-count was dropped so a cell under a heading never carries its meaning by absence alone. `#` (cardinality) was the alternative — correct, but heavy; `‖·‖` is a norm, the wrong math; `|·|` is cardinality but bar-family)
```

Laws:

- **Scale at ≥ 10,000, not ≥ 1,000** — below ten thousand the value is exact to the unit (`1·099` lines, `2·428 B`); above it, three significant digits (`12.3K`, `999.9M`). Bytes scale by 1024, counts by 1000 (as `human_size` / `group_lines` do today).
- **Under a heading** that names subject and/or unit, `g` and/or `u` go blank — the field keeps its width, so `3.3PB` under a `bytes` heading sits exactly where it would beside a glyph.
- The **`·` filler as dotted leaders** is *not* the default (agents read fill as content); it may become `format.count = dotted` for human TTY callers — an overlay, same law as color.
- **`≡` for lines is retired** from the subject slot (its former home) — lines is a unit and lives at col 12. **Unit letters decided: `𝓁` (U+1D4C1) lines, `𝓉` (U+1D4C9) tokens** — mathematical-script *l* and *t*: the reader's own letter, costumed just enough not to look like `1` or a scale letter. Path to the decision, kept because it is a cross-substrate read: `≡`, `λ`/`τ` (Joseph), `ℓ`/`τ` (coordinator's lean — BMP, safe fonts) were floated; Grok read `ℓ` as *litre* ("kilolamberts" surfaced in its thinking) and `𝓁` as *lines* in a beat — *"the costume is doing the work of not looking like 1"* — and Joseph's terminal renders the script pair cleanest. Cost carried knowingly: both are outside the BMP (4-byte UTF-8, ~2 tokens each), the vocabulary's first non-BMP glyphs — the ones to watch in a fallback-font terminal; width-1 verified on Joseph's terminal, agents don't render.
- **Tokens** is the agent's unit ("what would this cost me to read") — model-dependent, therefore an estimate (`~`) by nature; deferred: likely a bytes-per-token prior per kind first, later a cached tokenizer and per-substrate history. Rides the read budget and cache like lines.
- Specimens (spaces shown as `␠` only where the eye needs them):

```
●   1·099.  𝓃       files, exact count
□  ≥9·021.  B       dirs, at least 9,021 bytes
▣  ≈   14.3GB       files+dirs, ≈14.3 GiB
●  ≈   61.2K𝓁       files, ≈61,200 lines
●  ~    2.4M𝓉       files, ~2.4M tokens (estimated)
        3.3PB       under a heading naming the subject (𝓃/B/𝓁/𝓉 still shows unless the heading names the unit too)
```

**Not decided here:** how a *subgroup* subject (suffix bucket `md`, kind `archive`, a glob pattern, `dirty`) is written before its cell — the census/facet forms wait on that. Consequences already implied: `(44 files)`, `dirty<4>`, `≈127f`, `md×31` all migrate into this grammar once the subgroup form is chosen.

## Meaningful agent glyph packs (2026-08-15, from Joseph's question "what do nerdfont glyphs look like to you?")

The glyph-block and the marks column want a vocabulary. The first thing to decide is *what kind of glyph* — and the reader settles it. Nerd Font glyphs (Private Use Area, U+E000–F8FF and the supplementary PUA) are, to a token-reading agent, **opaque**: a codepoint with no picture and no semantics — some tables are known (`\ue0a0` Powerline branch, devicons ~U+E7xx, Font Awesome ~U+F0xx) but nothing can be *read* from one without a legend. Three consequences, first-person testimony from the coordinating agent:

- **Cost.** Each is a 3-byte UTF-8 sequence that tokenizes as byte fragments — roughly 2–3 tokens per glyph against one for `·` or `≈` — and carries nothing readable for the spend.
- **Fragility.** Without the font it is tofu (▯); in a pipe, a pasted context, JSON, or a plain terminal it is noise. It is a *font-dependent rendering* — the class the eyes doc assigns to color: a redundant overlay for a human on a TTY, never the sole carrier.
- **The glyphs already in the look work for the opposite reason.** `⊘ ≈ ≥ ~ · ×` are BMP characters whose meaning the agent already holds — `⊘` reads as "excluded" cold, `≥` as a floor. They work *because the glyph is the token* (the SIGNA bet: the glyphs ARE the tokens, [[phenom-format|phenom-format]]).

So the law for this tool, agent-first: **a glyph pack is BMP Unicode with intrinsic meaning** — chosen so an agent guesses most of it right cold and learns the rest once from help (the law channel), stable thereafter (a learned glyph is an interface — [[shorthand|Shorthand]]). Nerd Font icons are at most an **opt-in human TTY overlay** (an `--icons`-style knob, off by default, never in the no-color look, never in JSON) — the `eza --icons` assumption inverted.

**Second witness, cross-architecture (Grok, 2026-08-15) — embedded whole; independent priors, so its agreement is signal:**

![[glyphs.md]]

What it adds beyond the testimony above, folded into the law: (a) a **confusables constraint** — pairs it will mix in one column (`∅ ⊘ ø ○ 0`; `≈ ~ ∼ ≃ ≅`; `× x ✕ ✖`; the four dashes; the four bars) — a pack never seats confusables in the same column, and `≈ ~ ≥` are the only members of their families that may coexist; (b) **density bars** (`░▒▓█`, `▁▃▅█`) read as magnitude cold — a candidate for mass/heat grade that beats stars, SIGNA's own mechanism; (c) the shipped `≈`-vs-`~` split is *learnable in one line but not self-explaining* — a finding for the mass-mark distinction, not only for glyphs; (d) `[denied]` as a word beats any glyph either witness would invent (its `⛔` suggestion is emoji-class, which its own emoji paragraph rules out of a column — noted, not adopted); (e) its "small usable set" is the **starting candidate pool** for every pack below. Still wanted before anything ossifies: a third-substrate cold read (a fresh Claude/Sonnet and a small local model) on the same candidate packs.

**Packs, not a pile.** A pack is a small closed set for one fact-family, one glyph per value, position-in-block = fact, glyph = value (permissions-block style). Candidate packs to draft (vocabulary Joseph ratifies before it ossifies; each needs a pipe-safety and tokenizer check — one BMP codepoint, no combining marks, no emoji-presentation variation selectors, width-1 in monospace):

| pack | values | candidate family (BMP, semantic) |
|---|---|---|
| **honesty marks** | exact-grouped / estimated / floor / excluded / denied / cut | `≈ ~ ≥ ⊘` (shipped) + one each for denied and walk-bound (today words: `[denied]`, `[walk bound]`) |
| **aliveness grade** | heat quantized to ~4 grades | graded stars or filled shapes: `✭ ✫ ★` / `○ ◔ ◑ ●` (SIGNA's density idea applied to magnitude, not time) |
| **git letter** | modified / added / untracked / renamed / conflict | letters are already the universal glyph (`M A ? R U`) — keep letters; a glyph pack would lose the reader's prior |
| **kind class** | dir / file / link / mount / special | `/` and `->` already carry dir/link as decoration; a class glyph is likely redundant — draft only if the has-block wants a compact form |
| **has-kinds** (kind-glyph-block) | git / rust / python / ruby / js / agents / build / archive / vault … | this is where a pack tempts most and is weakest: no BMP glyph *means* "rust"; two-letter tags (`rs py rb js ag bd ar ob`) at fixed positions may beat glyphs — decide by reading, not aesthetics |
| **movement (last-look)** | appeared / changed / gone | `+ Δ −` or `↑ ↻ ✕` — SIGNA-adjacent; that row's |

**Decided packs (Joseph, 2026-08-15):**

- **Unit/subject glyphs** — see §The count cell above (`● □ ▣` subjects; `𝓃 B 𝓁 𝓉` units; `≈ ≥ ~` marks). Consequence for the aliveness pack: `●` is taken, so a heat/aliveness grade cannot use the circle fill-family — density bars (`░▒▓█` / `▁▃▅█`) are the candidate; `○ ◔ ◑ ●` is tucked away for later (file types or sizes, Joseph).

- **git status — one cell.** Values: `⊘` gitignored · `M` modified · `A` added · `⁇` untracked · `R` renamed · `U` unmerged · `D` deleted (`C` copied / `T` typechange kept available so the pack is porcelain-complete). **Blank when clean** (quiet law: the letter's presence is the message). When index and worktree differ, the worktree state shows — dirty is the glance question. One cell, not the porcelain `XY` pair: a block has ~8–10 cells in all and the second would be blank most of the time. `⁇` (U+2047) over `?`: it is porcelain's own doubled `??` in one cell — louder for humans, and (Grok, same day) *"it is the ? I already have, just louder, and it stops me from stealing U for untracked."* `⊘` stays the ignored glyph — Grok reads it as clearly "ignore" where `∅` reads as "empty" — and it moves *into* this slot, so gitignored is marked once (retire the separate marks-column `⊘`; JSON `gitignored: true` unchanged). Human/terminal legibility is a criterion here alongside agent cold-reading — Joseph is the sensor for it. `∅` stays free (a future empty-dir mark must not use it either way, to keep the pair apart).

**Tests a pack must pass** before it ships: (1) an unprimed agent, shown the block cold, guesses ≥ half the values right; (2) the block survives `| cat`, JSON has fields not glyphs, and the no-color look is the primary; (3) tokenizer cost measured — a block should cost about as many tokens as it has glyphs; (4) taught in `--help` in one table, unchanged after.

## Decisions this row needs (Joseph)

- The **place** for each row above marked **decide** (7 dangling `·`, 18 title, 19 target-as-spill) and confirmation of the proposed near-right sub-column order (title · kind-word · globify · census · marks · facets · has) — or a different order.
- **Which near-right parts spill to sub-rows and when** — always (facets/has always below), or only past a look width — and the width knob (caller-stack key, never TTY).
- Whether near-right sub-columns are **reserved-when-present-in-look** (padded, like far-right today) or claim width only on rows that have them.
- The **far-right block ceiling** (four columns at `~/src` depth 2, six at `/tmp` — how many before quiet facts spill or drop).
- Names: is *grid* the row's word (the fact×place grid), and do `place`/`office` become the inventory's two columns?

## Foundations

[[../../../principles/src/form-agentic-eyes.md|form-agentic-eyes]] (channels: horizontal position, aligned edge; concern 9 units/labels) · [[shorthand|Shorthand]] (placement classes; per-kind edge offset) · [[columns|Columns]] (selection/format/alignment law) · [[aspect-lattice|Aspect lattice]] (offices) · [[vertical-info|Vertical info]] (the acceptance gauntlet this row unblocks).

## Not in this row

New facts. Glyph vocabulary for the glyph-block (shorthand's open). SIGNA. Per-kind edge offsets (noted seam). The density gauntlet's *acceptance* is [[vertical-info|Vertical info]]'s; this row is the model that makes it possible.
