# Balanced summarization

`--lines N` is a hard line budget for the whole look, including the header. `0` means no line limit. Built-in default is 80 (config `lines`).

Siblings **share** remaining lines. The first child does not take the rest. Every child gets its base line; extra lines round-robin, one at a time, only to children whose **capacity** (lines the subtree could use fully rendered) is not yet met — a share a child cannot spend flows on to siblings that can, until the budget or the tree is exhausted. This keeps both laws at once: fairness (a big directory absorbs extra only after every sibling has all it can use) and no parked lines (2026-08-14 audit finding 9: an 80-line asf look spent 62 while `01-aat-core/` sat at bare census; now 80/80). A share of exactly **2 is a dead value** for a dir with 2+ children — its recursion folds to a census spending 1 — so the useful step for such a dir is 1 → 3, and the allocator never parks a line on 2. Allocation is a pure function of tree + budget: two looks at an unchanged tree agree.

If a directory only has budget for **its own line**, leftover is a [[dir-census|dir census]] on that line (`autocolors/  [2: 1 .md, 1 dir]`). Do not spend a second line on `└── [+2: …]` — that is the same information.

`[+N: …]` is only for leftover **siblings** at a level where some siblings *were* listed.

`--explain-budget` writes the shares-and-why to stderr, including `unspent N (tree exhausted)` when the budget exceeds what the gathered tree can render — the honest name for lines nobody could use (deeper lines would need a deeper `--depth`, a different knob).
