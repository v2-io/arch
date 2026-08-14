# budget-not-cutoff — finish note

> [!note] **History, not the binary.** This describes the retired first-snapshot code; none of it is in the current crate (`impl/README.md`). "Landed" below is past-tense of that snapshot.


*Landed. Tests: `tests/budget.rs`.*

`--lines N` is a hard cap including the directory’s own line. Siblings share what is left.

`tree -L` and broot height-fill are the thing this is not: 8 directories and 20 lines, every sibling gets at least 1, none gets 18, the first child does not take the rest. If they cannot all have a line, leftover names are printed (not dropped).
