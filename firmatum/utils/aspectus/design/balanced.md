# Balanced summarization

`--lines N` is a hard line budget for the whole look, including the header. `0` means no line limit. Built-in default is 80 (config `lines`).

Siblings **share** remaining lines. The first child does not take the rest. Extra lines go to directories.

If a directory only has budget for **its own line**, leftover is a [[dir-census|dir census]] on that line (`autocolors/  [2: 1 .md, 1 dir]`). Do not spend a second line on `└── [+2: …]` — that is the same information.

`[+N: …]` is only for leftover **siblings** at a level where some siblings *were* listed.

`--explain-budget` writes the shares-and-why to stderr.
