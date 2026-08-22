# Focus

`--focus PATH`, stdin paths (`rg -l | aspectus`), or `--rg PATTERN` / `--glob GLOB` **reweight** the picture. Matches stay in place; the surroundings are not thrown away (origin: *"some kind of easy `rg -lco … | this-tree-viewer` or built in equivalent that will show the tree with those things highlighted but in the context of their surroundings"* — and the same for file filtering: *"basic common cases of find … but displayed in the context of surroundings"*).

**Never a cut.** The lattice's `filter` office is marked dangerous for a glance for exactly this reason: a filtered tree lies about the place — the reader loses the calibration of what surrounds its matches, which is the tool's first duty. Focus is the `weight` office: matched paths and their ancestor chains get the budget; everything else compresses toward censuses and remainders but keeps its typed, honest presence. The prior art is orient-rank's mark-in-place philosophy (cited in [[heat|Heat]]): *"agents free-read by outline order, not rank order"* — matches are marked and fed where they stand, not extracted into a ranked list. `rg -l` already gives the extraction; what the pipe into aspectus buys is the *context put back*.

## Inputs (all produce the same match-set; they compose by union)

| Ask | Meaning |
|---|---|
| `--focus PATH` (repeatable) | This path (file or subtree) is what I care about. |
| stdin paths, when stdin is a pipe | One path per line, relative to CWD or absolute (the `rg -l` contract). |
| `--rg PATTERN` | Built-in content-match equivalent; matches are files whose content matches. |
| `--glob GLOB` | Built-in name-match equivalent. |

- Stdin is read as paths only when it is a pipe and a focus-consuming look is running — never blocking on a TTY (Part II law). Blank lines skipped.
- A stdin/focus path that does not exist, or lies outside the locus, is confessed on stderr (count and specimens) and otherwise ignored — never fabricated into the look, never a hard failure of the whole look (the rest of the ask is still serviceable; matches-found-zero is see below).
- `--rg`/`--glob` are conveniences over the same IR as stdin paths; whether `--rg` shells to `rg` or uses a regex crate is the implementer's call — the *result contract* (files-with-matches) is the law. They are not `find`-style filters.

## What the weight does

1. **Survival:** matched files, and every ancestor directory on the path to a match, take the top weight tier — above [[important-files|important files]], above plain dirs (the caller's explicit ask beats every standing default): `focus > dirs > important > plain`, key-within-tier as ever ([[sort|Sort]]).
2. **Allocation:** the allocator's shares ([[balanced|Balanced summarization]]) weight toward subtrees containing matches, so focused regions render deeper/fuller and unfocused siblings fall back to their censuses. Unfocused siblings never disappear: their lines-or-censuses are the locating context.
3. **Depth:** a match deeper than `--depth` still gets its line — the expansion chain to a match overrides the depth cutoff (a focus ask that silently depth-hid its matches would be worthless). The walk bound still binds (it is a cost law, not a display law); a match beyond the walk bound is confessed by the ordinary `≥`/`[walk bound]` machinery plus a stderr note that the focus was not fully served.
4. **Order:** untouched. Matches sort among siblings by the ordinary key. Position is not the match channel — the mark is.
5. **The mark:** a matched line carries a match mark so match is distinguishable from context (spelling/glyph open — near-right or decoration class per [[shorthand|Shorthand]]; a learned glyph is an interface, so Joseph ratifies before it ossifies). Ancestors-of-matches are context, not matches: no mark, or a distinct containment form (open with the spelling).
6. **Leftover matches are typed.** If even the focused set exceeds the budget, unlisted matches fall into a census that *says they were matches* (e.g. a matched-count in the remainder) — a silent cut of the thing asked about is the worst lie available to this row.

Zero matches is an honest look, not an error: the ordinary picture plus a stderr note (`focus matched nothing`). Exit stays 0 — the look succeeded; whether an agent needs to branch on match-count without parsing is the same open exit-vocabulary question walk-bound carries.

## Multiple paths (Joseph, inbox 2026-08-15; routed 2026-08-22)

> I need to be able to do this:
> `aspectus --lines 200 --depth 4 ~/src/arch/asf/{01-aat-core,02-tst-core,03-llm-core,04-eli-core}`

Several positional paths are a **focus set**, not several looks: the look is of their **common ancestor** (the perspective root — overview invariant), the selected paths take the focus tier, and everything between and beside them is connective context that folds to remainders — *"elide between the selected aspects"* (Joseph) — never dropped (this row's law). Almost no new design; three small decisions (Joseph, 2026-08-22: 1 **agreed**; 2 **unsure** — *"let's drop it if -q is added?"*, i.e. fold by default, and a `-q`/`--quiet` (suite flag, not yet built here) would drop the connective remainders entirely; 3 **agreed**):

- **Depth counts from each selected path**, not from the common root — `--depth 4` means four generations under `01-aat-core/`, etc.; the connective ancestors (here just `asf/`) render as the chain to the matches, spending no depth. (Counting from the root would make the ask unexpressible: the user named the places they want *deep*.)
- **Unselected siblings fold**: at each connective level, non-selected siblings collapse to one leaf-census remainder (`[+ dir×9 ≈1.4Kf · md×27]`) — present, typed, one line — rather than each getting a line; the selected ones are the picture.
- **No common root but `/`** (paths on different devices, or `~/a` with `~/b/c`): the common ancestor is still the root, rendered honestly; if the ancestor is `/` itself the look is legal but the header says so. Shell brace expansion hands us plain paths, so this is the positional-arity rule: one path = the locus; several = focus over their ancestor. JSON: `root` is the ancestor; `matched` on the selected nodes as this row already specifies.

Also noted from the same inbox day (a Fable instance): a **heat threshold** — *"only dirs with heat > 0.2"* — as a focus-shaped ask (weight by a fact's value, or a threshold that promotes to the focus tier). Undesigned; the weight-office algebra (lattice Open) is where it belongs.

## Foundations (clauses)

| Clause | Where |
|---|---|
| Weight office; filter refused for the glance | [[aspect-lattice\|Aspect lattice]] |
| Glance before focus; do not make the glance be the read | Part II, agent-utility-exploration |
| Survival tiers and key-within-weights | [[sort\|Sort]] · [[important-files\|Important files]] |
| Shares and capacity | [[balanced\|Balanced summarization]] |
| Mark-in-place prior art | [[heat\|Heat]] (orient-rank) |
| Never block on stdin at a TTY | Part II, core-design-philosophy |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Pipe of paths | `printf 'a/b.rs\nc/d.md' \| aspectus` marks both, expands their chains, keeps siblings as censuses. | Fixture; snapshot. |
| 2 | Context preserved | Every unfocused top-level sibling still appears (line or census); nothing silently vanishes vs the unfocused look. | Compare looks; assert same level membership. |
| 3 | Deep match | Match at depth 5 under default `--depth 2` gets a line; unfocused areas stay at depth 2. | Fixture. |
| 4 | Order untouched | Matches sort by recency among siblings; only marks and budget differ. | Set mtimes; diff order against unfocused look. |
| 5 | Bad paths | Nonexistent / outside-locus stdin lines: stderr confession, look unaffected, exit 0. | Fixture with junk lines. |
| 6 | Typed match leftover | 200 matches, `--lines 20`: remainder census names its matched count. | Snapshot. |
| 7 | Zero matches | `--glob '*.zig'` in a Rust tree: ordinary look, stderr note, exit 0. | Fixture. |
| 8 | Union | `--focus` + stdin together: union of match-sets. | Fixture. |
| 9 | Determinism | Same tree, same ask: byte-identical. | Diff runs. |
| 10 | JSON | `matched` is a field on matched nodes; the ask does not change the schema shape. | Parse fixture output ([[json\|JSON]]). |

## Open

- **Mark spelling** (glyph, placement class, and whether ancestors-of-matches carry a containment form). Joseph ratifies — interface vocabulary.
- **Match counts** (`rg -c`-grade: show *how many* hits per file, origin's `-co` hint). Wants `--rg` to count, or stdin to accept `path:count`. Leaning: later, compose-grade — presence first. Not decided.
- **`--focus DIR` semantics grain:** a focused directory weights its whole subtree — does it also force full expansion, or just weight? Leaning weight-only (the allocator will usually expand it anyway; forced full expansion re-invents `--depth 0` locally). Ratify.
- Flag spellings `--rg` / `--glob` vs one `--match` family. One decision, with the compose-grammar question it brushes.

## Not in this row

The weight-office algebra when heat/importance/focus compose (lattice Open). `--kind` filtering ([[kind-query|Kind query]] — that one *is* a filter, and owns its own justification). Highlight rendering beyond the mark ([[color|Color]] may later paint matches on a TTY). What `rg` flags an agent uses upstream.
