# Aspect lattice

How a **fact** may enter the one look (`aspectus` / `aspectus PATH`). Not a verb table. Verbs stay `help`, `version`, implicit **show** (and later only a new *act*: `config` already, maybe `snapshot` / `invalidate-cache`).

**Status of this grid:** first *reasoned* pass, 2026-08-14 — reworked cell-by-cell by the coordinating agent as end-user under Joseph's explicit grant ("redo the lattice cells to be more the tool you and other agents will want and need"), from the original agent-guessed grid he had partially corrected. Decided-by: `supported` — Joseph corrects on contact; the §Reasoning section says why each departure was made so his corrections can target the reasoning, not just the cell. It remains **orthogonal to implementation priorities** and **not a replacement for a row's own design**: a cell is a claim that an office exists, not the story, fixtures, or law of that office.

[[config|Config]] is the first arbitrator of these cells (which facts are `ON` / `OFF` / `QUIET`, which format is starred). Caller stack, not a file in the project — so a given agent's look does not drift when the tree does.

## The governing principle (why the cells lean the way they do)

**Minimum surprise from the tool; maximum surprisal per glyph in the look.** Every default-ON cell must earn the tokens it spends for a cold agent reader; everything else waits for surprise (QUIET) or an ask (config/compose). Corollaries applied throughout:

- **Almost no own flags.** A glance tool with a flag museum fails Beauty. Facts are asked for through config and one shared compose surface ([[columns|Columns]]/[[sort|Sort]] grammar, spelling open); own flags exist only for the look's *shape* (`--depth`, `--lines`, `--walk`) and suite floor (`--color`, `--config`, `--caller`, `--format` later).
- **The reader's units are lines and kinds, not bytes.** For the text-dominated loci this tool serves, line-count is the honest mass unit; byte size earns its place only at magnitude outliers (quiet) and in deep aggregates.
- **Aliveness beats ownership.** Recency (mtime, heat, last-look) answers "where is the action" — the second question every cold agent asks after "what is this place." Identity facts (owner/group/perms) matter only as anomalies.
- **Aggregates are the mission.** The census family (dir census, leaf census, and [[mass|Mass]] = the deep aggregates) is what fixes "SURPRISE! this directory is far more than you thought" — so aggregate offices get filled generously, column offices stingily.

## Offices (columns of the grid)

| Office | In the look |
|---|---|
| **column** | Where the fact *sits* on the line. The binary here (`Y` = column, `INFO` = right of the name) is now the coarse form of a richer **placement vocabulary** — decoration / far-left / near-left / near-right / far-right / glyph-block — see [[shorthand|Shorthand]] (Joseph, 2026-08-14). Cells stay coarse until facts claim finer placements. |
| **default** | In the default look: `ON` · `OFF` · `QUIET` (in the look only when it surprises). |
| **quiet** | This fact *can* appear only when it surprises (per-fact law lives with [[quiet-columns|Quiet]]: cold baseline = place norms; warm = caller after-image). |
| **sort** | May be the order key ([[sort|Sort]]). |
| **flag** | `Y` = own flag. `COMPOSE` = via the shared ask only. `NO` = refused a flag (refusal names the config path). |
| **format** | Rendering options, `*` default. `pattern` = user strftime-like. `signa` = [[phenom-format|SIGNA]] glyphs (perceived magnitude). |
| **dir-census** | An unexpanded directory reports this about *itself* (shallow: its own children). |
| **deep-agg** | The recursive aggregate below a line ([[mass|Mass]]'s office — cache-backed, `≈`/`≥`-honest, furniture excluded). |
| **leaf-census** | Leftover *files* at a level report this (`[+47: 31 .bak]`). |
| **filter** | May *cut* the look. Dangerous for a glance; Focus is weight, not cut. |
| **weight** | May reweight the allocator (Focus, heat, importance). Not sort. |
| **unique** | What only this fact has — which quantity, absence, obtain cost, … |

Marks: `Y` in scope · `INFO` hangs on the name · `ON`/`OFF`/`QUIET` default · `COMPOSE` shared ask only · `NO` refused · `*` format default · `—` not this fact.

## Lattice

| Fact | column | default | quiet | sort | flag | format | dir-census | deep-agg | leaf-census | filter | weight | unique |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| name | Y | ON | — | Y | NO | basename* / relative / absolute | — | — | — | — | — | Always on. The unconditional tiebreak; the default *key* is recency (mtime) since 2026-08-14 ([[sort|Sort]]). Perspective root is an [[overview-invariants\|overview invariant]], not this cell. |
| child-count | (census) | ON | — | Y | NO | n* | Y | Y | Y | — | — | **Unified with the census** (the old open question, answered): child-count *is* the `N` of a census; an expanded dir shows its children, an unexpanded one shows `[N: …]`. No separate INFO. Deep form is mass's file-count. |
| line-count | Y | ON | Y | Y | COMPOSE | physical* / non-blank / signa | Y | Y | Y | NO | — | **The mass unit for text loci.** Non-binary only; binary omits, never `0`. Obtain is a read; cache by ino+mtime+size. Deep-agg (`≈61k lines`) is [[mass\|Mass]]'s headline number. |
| modified (mtime) | Y | QUIET | Y | Y | COMPOSE | iso-8601* / rfc-3339 / epoch / pattern / signa | **Y (newest-below)** | **Y (newest-below)** | — | NO | Y | **Aliveness — and the default sort key** (recency, newest first; steward 2026-08-14). Quiet = recent vs siblings/now (warm baseline: newer than the caller's last look). The dir-census/deep-agg form is *newest mtime beneath* — "where is the action" for an unexpanded subtree. `signa` renders the delta as perceived elapsed time. |
| size | Y | QUIET | Y | Y | COMPOSE | human* / bytes / log | Y | Y | Y | NO | Y | Quiet = magnitude outlier among siblings (the 400MB file in `src/`). **Which:** `st_size` ≠ allocated ≠ recursive du; deep-agg is the du-like walk, `≈` under cache/bounds; furniture/gitignored bodies never silently join it. JSON always bytes. |
| created (btime) | Y | OFF | Y | Y | COMPOSE | iso-8601* / epoch / pattern / signa | — | — | — | NO | — | Omit where the FS can't say (Linux w/o statx btime). Never fake as mtime. |
| filetype (suffix) | (census) | ON | — | Y | NO | suffix* / bare | Y | Y | Y | Y | — | Lives in the censuses (`170 .md`), not as a per-line column — the suffix is already in the name. Globify hangs here. Not MIME-from-magic. |
| filekind (text/image/binary/dir/link) | INFO | QUIET | Y | Y | NO | word* | — | Y | Y | — | — | **Demoted from ON-column**: the `/` on dirs and the suffix carry kind for free; the *word* appears only when kind surprises (a binary in a source dir; a huge image among .md). Kind from a config suffix-map, not magic. |
| kind (of the place) | INFO | ON | — | — | NO | facet* / first-line | — | — | — | Y | — | The parent line's gathering spot: `[kind: git, rust, …]` + specialized facets (`[git: remote (private) br stat]`). Furniture *feeds* this fact. Filter is `--kind`, not a cut of children. |
| git letter | Y | ON | Y | — | NO | letter* / porcelain | Y (dirty-count) | Y (dirty-count) | — | — | Y | Dirty only; clean prints nothing. Census form: `3 dirty` beneath an unexpanded subtree — churn visibility without expansion. Weight: dirty files are likely Focus targets. |
| heat | Y | **ON** (in-repo, once obtained) | Y | Y | COMPOSE | score* / signa | Y | Y | — | NO | Y | Commit-decay, visible set only. **Default ON inside a repo** (steward, 2026-08-14): with the recency default-sort, heat-as-column gives the git-heat affordance whole — position says what just moved, score says what *has been* hot; paired as the `score · age` right-cluster. Omit outside git; obtain is one log pass, cache-keyed by HEAD when Cache lands. `signa` density is the alternate rendering. |
| initial-sha / latest-sha | Y | OFF | — | Y | COMPOSE | short* / H~N / full | — | — | — | NO | — | One row now: both are Heat-family obtains, compose-only, omit outside git. |
| prior name | INFO | QUIET | Y | — | NO | was* | — | — | — | — | — | Only if recently renamed. |
| symlink target | INFO | ON | — | — | NO | target* / target+type | — | — | — | — | — | Plus broken. Followed-and-recursed per origin ([[links-and-fs\|Links]] row); cycle does not hang. |
| denied | INFO | ON | — | — | NO | denied* / denied+class | Y (`≥` effect) | Y (`≥` effect) | — | — | — | A fact about the look, not the inode. Not quietable — existence-honesty ([[denied\|Denied]], shipped). |
| walk/bound marks | INFO | ON | — | — | NO | `≥` / `[walk bound]` | Y | Y | Y | — | — | Also look-facts, shipped. Not quietable. |
| important (witness files) | (weight) | ON | — | — | NO | — | — | — | — | — | Y | README etc. (config-defined): an allocator **weight**, not a column — they survive tight budgets ([[important-files\|Important files]]). READMEs may also lend the dir a title ([[readme-title\|README title]]). |
| last-look delta | INFO | QUIET | Y | Y | COMPOSE | marker* / delta / signa | Y (changed-count) | Y (changed-count) | — | NO | Y | Delta of other facts vs the caller's after-image. Quiet by nature: unchanged prints nothing. `signa` for the elapsed-time half. Identity of a look still open ([[cache\|Cache]]). |
| permissions | Y | QUIET | Y | — | NO | octal* / symbolic / octal+flags | — | — | — | — | — | Quiet = not usual (`644`/`755`); flags = append-only / immutable / uchg when set. |
| owner / group | Y | QUIET | Y | — | NO | name* / id | — | — | — | — | — | One row now: quiet = not-you / not-usual. Never a default column. |
| cloud | INFO | QUIET | Y | — | NO | evicted* | — | — | — | — | — | Evicted/not-hydrated only; omit where the FS has no such bit. |
| linkcount | Y | OFF | Y | — | NO | n* | — | — | — | — | — | Quiet candidate = nlink>1, but OFF entirely until someone wants it. |
| filesystem | — | OFF | — | — | COMPOSE | — | — | — | — | NO | — | Already `--no-one-fs`; not a column. |
| mentions (inbound refs) | Y | OFF | Y | Y | COMPOSE | n* / recent | — | Y | — | NO | Y | **From orient-rank** (`hotness_mention`, generalized from slug-corpus to any tree): how often other files in the locus reference this name/link. Obtain is a grep-scale pass; cache-keyed. Quiet = referenced-much-more-than-siblings. |
| preeminence (dependents) | Y | OFF | Y | Y | COMPOSE | n* | — | Y | — | NO | Y | **From orient-rank**: transitive inbound-dependency count — what would notice if this changed. Needs a per-kind link/dep extractor (wikilinks, imports — config/plugin map, kin to furniture's). The comprehension-priority signal. |
| working-surface | INFO | QUIET | Y | Y | NO | mark* | Y (count) | Y (count) | — | NO | Y | **From orient-rank** (molten residue): file carries live working markers (Working Notes, TODO/OPEN sections — config-defined patterns) × recency. Quiet: marks only where present. Census form: `3 molten` beneath an unexpanded subtree. |

## Reasoning (the departures, so corrections can target the why)

1. **Flag museum dissolved.** Every per-fact `flag: Y` (size, mtime, created) became `COMPOSE`. An agent asks for facts through config (standing preference) or the compose surface (one grammar, shared with Sort); own flags are reserved for look-shape. If one fact ever proves flag-worthy by lived frequency, promote it *then* — demand-evidence, not anticipation.
2. **mtime ON→QUIET, but census-promoted.** A timestamp on every line is the classic `ls -l` habit and mostly noise; *recent* mtime is signal. Meanwhile "newest-below" on unexpanded dirs is new and answers where-is-the-action at zero expansion cost — the aliveness analog of mass.
3. **filekind demoted, filetype census-only.** Both were spending default glyphs on what the name already says. The kind *word* survives as surprise (binary-in-source is exactly worth a glyph).
4. **child-count unified with census** — the grid's own open question, answered the obvious way.
5. **deep-agg added as an office.** Mass was designed ([[mass|Mass]]) after this grid was drafted; it is not one fact but an *office* several facts have (line-count, size, file-count, newest-mtime, dirty-count, changed-count). Naming the office keeps mass from becoming a parallel universe.
6. **SIGNA joined the format vocabulary** (mtime/created/heat/last-look) per [[phenom-format|phenom-format]] — always paired with a plain form, never in JSON.
7. **git letter gains census forms** (`3 dirty` below) — churn visibility for unexpanded subtrees; the same office mass uses, applied to aliveness.
8. **important-files as pure weight** — it was a row wanting a column; it never was one (the pipeline row already said "Sort is Sort's; this row does not own order" — its office is allocator survival).
9. **Rows merged**: initial/latest-sha (one obtain family), owner/group (one law). Rows added: denied, walk marks (shipped look-facts belong in the grid), important, last-look delta (was `last-look`).
10. **orient-rank factors adopted as facts** (Joseph, 2026-08-14: "feel free to use vivarium's ideas — all tuned specifically for orientation = comprehension — as new columns/aspects"): mentions, preeminence, working-surface — generalized from the slug-corpus originals (`arch/vivarium/bin/orient-rank`) to any tree via config/plugin link-extractors. All OFF/compose + quiet-capable + weight-eligible: they are comprehension-priority signals, and together with heat/freshness/importance they are the factor set orient-rank already combines (rank-CDF + geometric mean — see [[heat|Heat]] for the composition prior art). None spends default glyphs until it earns them by lived use.

## Open (play)

- Compose grammar spelling (shared with [[columns|Columns]]/[[sort|Sort]]) — Joseph ratifies.
- Whether `weight` merges Focus/heat/importance into one declared office algebra or stays per-fact `unique` notes.
- `deep-agg` staleness marks: `≈` (cached, may drift) vs `≥` (bounded/denied) are different honesties — rendering distinction to nail in [[mass|Mass]]/[[cache|Cache]].
- Single-entry census display (`env/ [1: 1 other]` → show the name?) — open in [[dir-census|Dir census]], Joseph's call.
- Whether `quiet` thresholds are per-fact config keys or one sensitivity dial.
