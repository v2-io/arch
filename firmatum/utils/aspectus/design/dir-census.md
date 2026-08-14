# Dir census

An unexpanded directory still says what it held. *(Rendering reworked 2026-08-14 from Joseph's redundancy observation + naive first pass, improved per the eyes doc; shipping-provisional, Joseph ratifies glyphs — and per his same-day caution, **expect [[globify|Globify]] and [[mass|Mass]] to intercede on the exact forms**: the principles below are firmer than the specimens, and implementations should keep the rendering easy to move.)*

## The problems with the first form

`design/  [32: 31 .md, 1 dir]` spends glyphs three ways badly: the **total is computable** (and purely redundant when one bucket: `[2: 2 .md]` — the day's first finding); **a `dir` bucket is a category error** — a subdirectory among files is not a same-kind item, it is a *container of unknown mass*, and counting it as "1 dir" buries exactly the unknown-unknowns the tool exists to surface; and `N .md` reads ambiguously against names.

## The reworked form

```
├── impl/      [md×24]
├── tests/     [rs×15]
├── design/    [furniture/ ≈2f · md×31]
├── src/       [dir×3 ≈120f · rs×12 · md×3]
└── [+ md×5 · txt×2]
```

Principles, each earning its glyphs:

1. **No total.** The sum is computable; the buckets are the information.
2. **`kind×N` notation** — count binds to kind unambiguously; no dot-prefix collision with names; suffixless buckets keep their word (`other×4`, or the extensionless name when singular).
3. **Dirs leave the suffix census.** Subdirectories render first, as containers with their **mass prevalent**: `dir×3 ≈120f` (deep file-count when known — [[mass|Mass]]'s office; `≥` under bounds, absent uncached until Mass lands, never fake). A container's census-presence *is* its mass, not its count.
4. **Name beats count at n=1.** Exactly one dir → its name: `[furniture/ ≈2f · …]`. This also answers the single-entry-census open (`env/  [1: 1 other]` → `env/  [thing.dat]`): a census that would conceal exactly one cheap name shows the name. (Bounded: names longer than a constant fall back to the count form.)
5. **`+` unifies to one meaning: "unlisted."** The leaf census (leftover *siblings*) keeps its `+` prefix — `[+ md×5 · txt×2]` on its own closing line; a dir's own census needs no `+` because position (on the dir's line) already says "below this." One glyph, one law; position disambiguates beside-vs-below.
6. Buckets sort: dirs/containers first (mass-descending), then suffixes count-descending, `·` separators.

Empty directories print no census. *(Shipped 2026-08-14 with Wave C mass — one render function, `Census::render` in `src/n_level.rs`; `≥ ` inside the bracket marks bounded membership, empty-container `≈0f` suppressed, name-length cap 24. All cheap to move.)* Everything else (walk marks, `≥` on bounded counts, denied) composes unchanged.

## Foundations

[[../../../principles/src/form-agentic-eyes.md|form-agentic-eyes]] (aggregation channel; earned glyphs; absent-never-faked) · [[summarization|Summarization]] (the law this renders) · [[mass|Mass]] (the deep numbers) · [[leaf-census|Leaf census]] (the `+` sibling form).

## Open

- Glyph ratification: `×`, `·`, `≈`/`≥` placement, the `f` unit (`≈120f` vs `≈120 files` vs bare) — Joseph.
- Whether dir buckets show `≈lines` instead of/alongside `≈files` for text-dominated loci (the mass unit question, lattice line-count row).
- The n=1 name-length constant.
