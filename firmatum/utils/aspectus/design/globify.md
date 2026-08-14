# Globify

A real sequence of names collapses to its pattern (origin, verbatim: *"file 'globification'-- e.g., 'some-directory/[NN]-output.bak[.NNN] (47 files)' some kind of 'there's a lot of these, and they follow a pattern, let's show the pattern."*):

```
├── output-[001-047].bak  (47 files)
```

One line now carries what 47 would have — the pattern *is* the information; the individual names were noise. This is listing compression, a third member of the honesty family: censuses say *how much* was not listed; globify says *exactly what* was listed, compactly.

**The sin is false-positive collapse.** Names that are not a series must never be fused into a fake one — `chapter-1.md` and `chapter-2.md` in a book draft are two documents an agent must see, not a `chapter-[1-2].md` blur. Every guard below leans toward *not* collapsing; a missed collapse costs lines, a wrong collapse costs truth.

## What is a real sequence

Candidate group: names identical except for **exactly one numeric run** (same prefix, same suffix, same position). Then the guards:

- **Minimum count:** fewer than N members do not collapse (default leaning 5; config key). Small groups are cheap to list and expensive to misread.
- **One varying field:** if more than one numeric run varies across the group, no collapse.
- **Width honesty:** zero-padding is part of the pattern — `[001-047]` only if all members are 3-wide; mixed widths don't collapse (they are probably not one series).
- **Same kind:** all files or all dirs, never mixed; symlinks never join.
- **Gaps are told:** the range shows actual min–max and the true count — `output-[001-047].bak (44 files)` legitimately means gaps exist; count ≠ span is the reader's gap signal, never papered over.
- **Important files never collapse** ([[important-files|Important files]]'s set is exempt), and a member individually matched by [[focus|Focus]] stays listed by name.

## Where it acts

At each level, after sort, before the line budget spends: a collapsed group becomes **one listee** costing one line, sorted by its pattern name. Members are *listed* (compactly), not leftover — they do not enter the leaf census, and a dir census of an unexpanded directory is unaffected (censuses count files, not listees). Facts on the collapsed line aggregate over members where meaningful: total size/lines as the deep-agg-style sum, mtime as newest (marks per the lattice's aggregate honesties). Collapsed *dirs* are not expanded (expanding one exemplar would be a guess dressed as a look).

`--show-all` (or a config off-switch, `globify = off`) restores the individual names.

## Foundations (clauses)

| Clause | Where |
|---|---|
| Low-ambiguity channel: the pattern + exact count, not prose | [[../../../principles/influx/tools-are-observation-infrastructure\|Observation]] |
| Leftover vs listed distinction; never a silent cut | [[summarization\|Summarization]] · [[leaf-census\|Leaf census]] |
| Deterministic membership and rendering | outline determinism claim (Working Notes) |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Collapse | 47 `output-NNN.bak` render as one line, pattern + `(47 files)`. | Fixture. |
| 2 | Below threshold | 4 members list individually. | Fixture at N−1. |
| 3 | Not a series | `chapter-1.md`/`chapter-2.md` (below threshold) and any mixed-width or two-field-varying group stay individual. | Fixtures for each guard. |
| 4 | Gaps | 44 files spanning 001–047: `[001-047] … (44 files)`. | Fixture with gaps. |
| 5 | Budget arithmetic | The collapsed group costs exactly one line; leaf census counts are unaffected by members. | `--lines` fixture; compare with `globify = off`. |
| 6 | Exemptions | An important file inside the numeric pattern stays listed by name; the rest still collapse if they clear the threshold without it. | Fixture with `README-001.md`-style trap. |
| 7 | Off switch | Config `globify = off` (and `--show-all`) restore names. | Same fixture. |
| 8 | Determinism | Same tree: identical grouping and rendering across runs. | Diff runs. |
| 9 | JSON | A collapsed group is a structured node: pattern, min, max, width, count, member kind — members recoverable in principle, not a lossy string ([[json\|JSON]]). | Parse fixture output. |

## Open

- **Threshold default** (5? 8?) and its config key spelling — Joseph ratifies.
- **Pattern spelling:** origin sketch is `[NN]`-style placeholders *and* the range form; the range (`[001-047]`) says strictly more. Leaning range-only; ratify. Also date-like runs (`2026-08-14`) — a numeric run that is really a date series collapses fine under the same rules, but the range render of dates may deserve its own look later; for now they are just digits.
- **Aggregate facts on the collapsed line** — which ones earn the width (count always; size/lines when those columns are on?). Lattice-office question as much as this row's.
- Whether hex runs (`deadbeef` cache names) count as numeric — leaning no (decimal digits only) until wanted; most such dirs are furniture anyway.

## Not in this row

Non-numeric pattern families (same-suffix grouping is the censuses' job). Expanding an exemplar of collapsed dirs. Furniture (a `target/` full of series should already be hidden before globify ever sees it).
