---
slug: identity-regime-archaeology
type: obs
depends: []
---

# Three identity regimes, physically present in one corpus

*The full account of a corpus that migrated its records through three different notions of identity — positional numbers, then labels, then slugs — and kept every generation on disk, so the migration can be read directly rather than reconstructed. The specimen grounding [[slug-identity]].*

## What happened

ASF's TST component (`~/src/arch/asf/02-tst-core/`) is the estate's oldest continuously-worked claim corpus, and its `src/` directory holds both the live segments and the unconverted prior-generation files side by side. The prior-generation files are the record of two abandoned identity schemes.

**Regime 1 — positional numbers.** The earliest records were named by their position in a numbered sequence, and the numbers were the identity: a claim *was* `FP-002`, and other documents cited it that way. Fifteen of the surviving prior-generation files still carry that number in an `older-tag:` frontmatter field, in two series (`FP-…` for first principles, `DEF-…` for definitions).

**Regime 2 — labels.** The next generation replaced the sequence numbers with typed labels — `T-02` for theorems, `D-01` for definitions — carried in a `label:` frontmatter field, with the label repeated in the filename (`old-tst-t02-implementation-time-lower-bound.md`) and in the document's own `# T-02 Theorem …` heading. Twenty-five of the prior-generation files carry a `label:`. This regime is not positional in the same way — the label is stable under insertion in a way a pure sequence number is not — but it is still an ordinal in a global namespace, and the number appears in the filename, the frontmatter, the title, and every citation of it.

**Regime 3 — slugs.** The live segments are named by a subject-noun slug with a role prefix derived from the record's own type (`def-implementation-time.md`, `der-dual-optimization.md`, `emp-changeset-size-principle.md`). No number appears anywhere: not in the filename, not in the frontmatter, not in the heading. Ordering lives in `OUTLINE.md`, and `depends:` names prerequisite slugs.

The same claim can be followed across all three. What was `FP-002` became `T-02 (Implementation Time Lower Bound)` and is now `def-implementation-time` — and the *content* moved less than the identity did. The migration was one of naming, not of substance, which is what makes the cost visible: under regimes 1 and 2 the identity had to be rewritten wherever it was cited, because it was carried in filenames, headings, and cross-references at once.

The prior-generation files are exempt from the corpus's format rules by an explicit, declared mechanism rather than by neglect: `FORMAT.md` states that the `old-*` filename prefix *"is the only mechanism for placing a file in `src/` that is exempt from FORMAT,"* and the tools that need the canonical segment set (`bin/lint-outline`, `bin/align-slug`, `bin/build`) are written to skip that prefix. That is why the archaeology survives intact: the corpus decided to keep its predecessors rather than delete them, and gave the decision a mechanism.

## The counts

| Measure | Value | As of |
|---|---|---|
| Files in `02-tst-core/src/` | 72 | 2026-08-06, counted first-hand |
| Live slug-named segments (regime 3) | 29 | same run |
| Prior-generation `old-tst-*` files | 43 | same run |
| …carrying a `label:` (regime 2) | 25 | same run |
| …carrying an `older-tag:` number (regime 1) | 15 | same run |

The regime-1 and regime-2 counts overlap: a file may carry both a `label:` and the `older-tag:` it superseded, which is precisely the trace of the first migration being recorded rather than erased.

## Method & scope

Counted in `~/src/arch/asf/02-tst-core/src/` on 2026-08-06: total files; files not matching `old-tst-*`; and `grep` counts of the `label:` and `older-tag:` frontmatter keys across the `old-tst-*` set. The three-way correspondence for one claim was read directly from `old-tst-t02-implementation-time-lower-bound.md` (frontmatter `label: T-02`, `older-tag: FP-002`) and `def-implementation-time.md`. The exemption rule was read in `~/src/arch/asf/FORMAT.md` (§Segment files / cadence).

**Scope.** One corpus, one lineage, two migrations. It shows that this corpus tried positional and label identity and left both, and it shows what the leaving cost — a rewrite of every filename, heading, and citation. It does **not** show that slug identity is better in general, does not measure the migration's cost against any alternative, and cannot: nobody ran the counterfactual corpus. The direction of travel is a fact; the reason for it is testimony from the corpus's own format documents, not an experiment.

## Working Notes

- The same `old-*` exemption that preserves this archaeology is also a taught blindness in the corpus's instruments — a linter reporting zero orphans over a directory it has been told not to look at. That reading belongs to ch. 14 ([[corpus-instruments]]), registered as R3 in `plan/TODO.md`; this segment deliberately carries only the identity half.
- Counts are dated and cheap to re-run; a re-run supersedes them and the collision is the point ([[collision-staleness-detection]]).
- Unexamined here: whether the two migrations were executed by tooling or by hand, and how long each took. `bin/align-slug` and `bin/rename-slug` exist now, which suggests at least the second was tooled, but that was not checked.
