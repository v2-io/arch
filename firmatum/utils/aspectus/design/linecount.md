# Line counts

Non-binary files show a line count. This is the founding request that kept getting lost (origin: *"show number of lines (for non-binary filetypes)"*, and later, when Grok flattened it: *"I can't find my original feature request about giving line-counts for text/md/udon/etc. files … (and code source files)"*) — and the lattice has since promoted it to **the mass unit for text loci**: for the trees this tool serves, lines are the honest measure of how much there is to comprehend; bytes are the anomaly channel.

Lattice cells this row implements (line-count row): column `Y`, default **ON**, sortable, `COMPOSE` (no own flag), formats `physical* / non-blank / signa`.

- **Non-binary only; binary omits, never `0`.** A `0` on a `.png` is a lie about what kind of thing it is; absence is the honest mark. Empty text file is a real `0`.
- **What is non-binary:** the config suffix-map that already drives filekind (lattice: *"kind from a config suffix-map, not magic"*). A suffix the map calls text counts; a suffix it calls binary omits. Unknown suffixes: see Open.
- **What is a line:** `physical*` = newline-terminated lines as `wc -l` would say, plus a final unterminated line counts as one (the reader's units, not the byte's). `non-blank` excludes whitespace-only lines. `signa` waits on [[phenom-format|phenom-format]].
- **Obtain is a read.** This is the first fact whose cost is file *content*, not stat. Cache key is `ino+mtime+size` — but the [[cache|Cache]] row is not built yet, and this design works **uncached first**: correct, honest, merely slower. Nothing here may depend on a cache existing.
- **Feeds mass.** The deep aggregate (`≈61k lines`) is [[mass|Mass]]'s headline number; this row supplies the per-file fact and the per-file honesty (binary excluded from line totals there too).

## Birthtime rider

The pipeline row carries a small sibling fact: **btime** (created), per its lattice row — default OFF, obtainable where the filesystem can say (macOS; Linux with statx birthtime), **omitted elsewhere, never faked as mtime**. It rides this row only because both land as "new per-file facts through Columns"; it has no interaction with counting.

## Foundations (clauses)

| Clause | Where |
|---|---|
| Reads can explode; bounds exist; cache is keyed when it exists | [[../../../principles/influx/cli-conventions/performance-and-resources\|Resources]] |
| Selection/format/placement mechanics | [[columns\|Columns]] · [[aspect-lattice\|Aspect lattice]] (line-count, created rows) |
| A fact with nothing true to say prints nothing (binary, btime-less FS) | [[../../../principles/influx/quick-tooling-conventions\|Silence]] |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Counts on text | A `.md` with 12 lines shows `12`. | Fixture. |
| 2 | Binary omits | A `.png` shows no count — not `0`, no placeholder. | Fixture; assert absence. |
| 3 | Empty is zero | An empty `.txt` shows `0`. | Fixture. |
| 4 | Unterminated final line | `"a\nb"` counts 2. | Fixture. |
| 5 | Suffix-map driven | Config marking a suffix binary removes its count; text restores it. | Override in user-home config. |
| 6 | `non-blank` | `format.line-count = non-blank` excludes whitespace-only lines. | Fixture with blanks. |
| 7 | Compose only | No `--line-count` flag; asked for/refused through config and the compose surface; `OFF` via config removes the column. | Per [[columns\|Columns]] tests 2–3. |
| 8 | Unreadable file | A file that can't be read for counting gets no count and the [[denied\|Denied]]-family honesty, never a guess. | chmod-0 fixture. |
| 9 | Uncached correctness | With no cache present, counts are exact; two runs identical. | Determinism diff. |
| 10 | btime | Where the FS has it and it's ON via config, ISO form; where the FS can't say, absent. | macOS fixture; absence path may be a unit test on the fallback. |

## Open

- ~~**Unknown suffix / no suffix**~~ **Shipped as the leaning (2026-08-14):** the map carries the well-known extensionless text names; a null-byte sniff of the first 1KB decides the rest, count-vs-omit only (never a rendered kind word). Joseph ratifies or reverses.
- ~~**Cost bound**~~ **Shipped: a read budget** (config `reads`, bytes, default 64MB, 0 = unlimited). Visible files are read even past it — the look's own lines stay exact; the deep [[mass|Mass]] walk is where it bites, degrading to size-based estimation marked `≈`. The lattice's ON survives with the glance staying fast (~0.4s warm on ~/src/arch with heat, ~0.3s without).
- ~~Full vs grouped~~ — per-file counts render full; grouping is the mass channel's.

## Not in this row

The cache itself ([[cache|Cache]] — key named here, machinery there). Mass's aggregation ([[mass|Mass]]). SIGNA rendering ([[phenom-format|phenom-format]]). Word/char counts (nobody asked).
