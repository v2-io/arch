# Important files — finish note (2026-08-14, Wave D)

Pure survival weight, as designed (`src/important.rs`, small): the
allocator's keep-choice gains the middle tier —
`dirs > important files > plain files`, key-within-tier, name tiebreak
(`n_level::weight`, one arm added). No column, no glyph, no order change:
with budget to spare the look is byte-identical to a no-importance build
(subfeature 2 asserts README sits dead last under recency when oldest).

Calls made:

- **Config key `important`** (env `ASPECTUS_IMPORTANT`), comma-separated
  basename globs, same grammar family as `furniture`: config entries go
  **in front** of the shipped defaults (order matters — readme-title will
  borrow this exact set and break ties by list order), `!PATTERN` drops a
  row. Spelling and extend-in-front semantics await ratification.
- **Default set:** `README*, AGENTS.md, CLAUDE.md` — the design's leaning,
  shipped for Joseph to correct on contact.
- Annotation is pre-budget, files only (dirs already outrank the tier).
- **No JSON field** — the design's leaning: importance is caller-config
  echo, not a fact of the tree; the JSON look's fields don't vary with
  one caller's weights.
- The lattice `important` row is a weight office with no line placement,
  so it is **not** in the `facts.rs` inventory table; `--important`
  refuses as an unknown option rather than with a config-path teaching
  line. Flag if that ever bites an agent.

No ancestor rescue (subfeature 4 fixture: a docs/ holding only a README
renders exactly as its census). Too-many-importants compete by the key;
the leftover falls into the typed census (subfeature 5).

Tests: `tests/important.rs` (6). Not in this row: early listing (lost to
Sort's law), highlighting glyph (leaning against, design Open), README
title, Focus composition.
