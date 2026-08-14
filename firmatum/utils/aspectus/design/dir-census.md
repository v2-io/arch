# Dir census

An unexpanded directory still says what it held.

```
a/  [2: 1 dir, 1 .txt]
```

Total first, then buckets by suffix (files) or `dir` (subdirectories), most numerous first. No suffix → `other`. Empty directories print no census.

Used at a depth cutoff today. Later also when a line budget refuses to expand a directory.

**Open — the single-entry census.** `env/  [1: 1 other]` conceals exactly one name at zero savings (2026-08-14 audit finding 9, second half). Showing the entry instead would need either the name kept through gather at the cutoff, or a compact chain form (`env/thing.dat`) — both change how a dir census reads at a glance. Flagged, not decided; Joseph's call.
