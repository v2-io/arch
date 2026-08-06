Repro (dprint markdown 0.19, textWrap never): content of a ```text fence with deep leading whitespace (ASCII art) loses a column of indentation. Source: udon-needs reports/addressing-exploration.md, 2026-07-22 run. Verbatim fence content must be byte-opaque (R2).

```text
                    ┌─ box art
 assembly product ──┼─ leading spaces are content
```
