# Balanced summarization

`--lines N` is a hard line budget for the whole look, including the header. `0` means no line limit. Built-in default is 80 (config `lines`).

Siblings **share** remaining lines. The first child does not take the rest. Extra lines go to directories. If not every sibling can have a line, leftover is a [[leaf-census|leaf census]] (`[+N: …]`), never a silent cut.

`--explain-budget` writes the shares-and-why to stderr.
