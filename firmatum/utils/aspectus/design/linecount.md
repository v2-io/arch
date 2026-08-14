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

- **Unknown suffix / no suffix:** count (optimistic, risks reading big binaries) vs omit (quiet, undercounts `LICENSE`, `Makefile`)? Leaning: the suffix-map ships with the well-known extensionless text names, and a cheap null-byte sniff on the first block decides the rest — but "not magic" was the lattice's word for *kind*, so whether sniffing is acceptable here is Joseph's call.
- **Cost bound:** the [[walk-bound|walk bound]] bounds stats, not reads. An uncached count of a huge look multiplies I/O. Does line-count need its own read-budget/`--max-file-size` style bound (Resources names the axis), or is ON-by-default only tenable once Cache lands? Honest options: ship ON and slow, ship QUIET-until-cache, or bound reads and mark unread files. Not decided here; this is the one place the lattice's ON may need Joseph to re-weigh.
- Whether a very large single file's count renders full (`61234`) or grouped (`61k`) in text — format nuance, one constant.

## Not in this row

The cache itself ([[cache|Cache]] — key named here, machinery there). Mass's aggregation ([[mass|Mass]]). SIGNA rendering ([[phenom-format|phenom-format]]). Word/char counts (nobody asked).
