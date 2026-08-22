# Focus — finish note, positional arity slice (2026-08-22)

What the binary does now, for the one subfeature that landed: **several
positional paths are one look**. `--focus`, stdin paths, `--rg` / `--glob`,
and the match mark are *not* built; the rest of design/focus.md is still
design.

```
aspectus --lines 200 --depth 4 ~/src/arch/asf/{01-aat-core,02-tst-core,03-llm-core,04-eli-core}
```

is the ask (Joseph, inbox 2026-08-15) and it now works. What comes out:
the header's root is `…/asf/`, the four volumes are the picture at four
generations *each*, and `asf/`'s twelve other children arrive as one line —
`[+ dir×12 ≈3205f · md×24 · other×2 · …]`. 0.1s on the real tree.

## The shape

- **Arity is the whole rule** (`main.rs`): none = here, one = the locus
  (and the only place a bare word is still classified as a possible
  mistyped verb), several = a focus set over their common ancestor. The
  old `only one PATH` refusal is gone.
- **`src/focus.rs`** (new, small) owns the set: common ancestor,
  nested-selection collapse, the post-walk fold, and the unlisted-match
  check. `Focus::is_selected` / `is_connective` are path predicates, so
  nothing depends on names.
- **Depth counts from each selected path.** `LookCtx.focus` changes the
  walk *only* on connective levels (the chain from the ancestor down);
  inside a selected subtree the ordinary depth law runs untouched. On a
  connective level each child is assigned a role: selected (restart the
  count at `--depth`), connective (descend for free), aside (walked one
  level for its census, then folded).
- **The fold runs after the deep phase** (`focus::fold_asides`), so a
  folded sibling reaches the remainder carrying its real mass —
  `dir×12 ≈3205f`, not a bare name count. That ordering *is* the
  difference between compressing the context and cutting it.
- **Survival tier**: `n_level::weight` gained the top arm —
  `focus (8) > dirs (4) > important (2) > plain (1)`, key-within-tier as
  ever. Under a budget too tight for the whole set, the selected are what
  survives.
- **JSON**: `root` is the ancestor; `matched: true` on the selected nodes;
  the folded siblings ride in the existing `omitted` census. No schema
  shape change (`SCHEMA` still 1).
- **Confessions on stderr, look still serviceable, exit 0**: a path that
  does not exist (named, counted); a path already inside another named one
  (dropped — depth counts from the outer, so the inner ask was served);
  and selected paths the budget could not give a line (see below). Only
  when *every* named path is missing does it refuse (exit 2) — there is
  then no place to look at all. `--explain-budget` gains a `focus:` line.

## Calls made (Joseph's to correct on contact)

- **No text mark on a matched line.** The design's mark spelling is Open
  and explicitly Joseph's to ratify, so nothing was invented: in this
  slice position already carries it (everything unselected at a connective
  level is folded, so the listed children *are* the selection). JSON gets
  `matched` because the design names that field. When the mark lands, the
  multi-path look should wear it too.
- **Leftover matches are typed on stderr, not in the census.** Subfeature 6
  says a silently-cut match is the worst lie this row has available. The
  census struct has no matched channel and its spelling is the same open
  interface question, so `--lines 5` on four volumes now prints
  `could not give every focus path a line (3): …` and names them. That is
  a placeholder for a census channel, not an argument against one.
- **Nested selections collapse to the outer one** rather than erroring or
  double-counting. Confessed by name.
- **Repeated identical paths dedupe**, so `aspectus X X` is `aspectus X`.
- One survivor after dropping (bad/nested) means the look is *exactly*
  today's single-path look at that path — no ancestor, no fold.

## One adjacent fix, made on the way

`apply_budget` did not reserve a line for a remainder census the node was
*already* carrying, so a level that arrived with an `omitted` (a walk-bound
cut before; a focus fold now) overshot `--lines` by one line per such
level. It now reserves it, and when it must drop children it merges them
into the census it already reserved instead of spending a second line.
`--lines N` comes out at exactly N on the real asf ask for N = 20, 40, 60,
200. No existing test moved.

## Tests

`tests/focus.rs` (12, real binary, isolated XDG) plus 5 unit tests in
`src/focus.rs`. Suite: 249 green. They cover the ask itself
(ancestor-as-root, depth-from-each), the connective chain spending no
depth, the typed one-line remainder with every unselected sibling in it,
the survival tier under squeeze, the budget promise, missing / nested /
all-missing paths, one-path-is-unchanged, determinism, JSON, and the help
page (which teaches the arity rule and carries the brace-expansion
example).

## Not in this row

`--focus PATH`, stdin paths, `--rg` / `--glob`, the match mark, match
counts, the `-q` that would drop the connective remainders entirely
(Joseph's leaning if `-q` ever lands, inbox 2026-08-22), and the
weight-office algebra when heat / importance / focus compose.
