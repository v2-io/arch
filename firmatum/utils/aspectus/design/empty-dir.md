# Empty directory

A directory that holds nothing says so: `[empty]`, in the marks, on its own line. Before this row an empty directory rendered bare, so it looked the same as a directory whose contents were folded away or never obtained. Every non-empty directory carries *something* (children, a census, a has-spot, an ignored remainder), so bare already *meant* empty, but only to a reader who knew the census law. A cold reader sees "no facts obtained." Grok (2026-08-22) put it this way: *"silence is the one confession that doesn't confess."*

**Decided** (Joseph, 2026-10-03, [[../audit/inbox-2026-10-03|verbatim]]): a word, `[empty]`, following the precedent that `[denied]` beat any glyph for both cold readers ([[grid-cleanup|grid-cleanup]] §glyph packs (d)). Not `∅`, which is kept apart from the ignored glyph `⊘` so the confusable pair never shares a look.

**Demand** (four arrivals, all unprimed except the first): the usability pass, 2026-08-15 item 4 (`~/src/glimmer/`, `arch/INGEST/`); Grok's cold read, 2026-08-22 ([[../audit/hallway-2026-08-22|hallway]] ◆◆5); two agents in `~/src/aat-refactored` on 2026-09-29 (recorded in that repo's `spikes/fresh-view-2026-09-29/history-and-sourcing.md`, never filed); and the inbox entry of 2026-09-30 (`audits/`, `ls -la` shows only `.` and `..`). One counterpoint is worth keeping: on 2026-08-14 Grok read an empty *root* as fine (*"Empty directories print the path and nothing pretending to be children"*). The root's header names the place, so its silence costs less than a child's. The mark goes on the root too, for one law instead of two.

## The law

- **Truth condition: the directory's readdir finished and yielded zero names.** That is the whole test, before any furniture, ignore, or show-all filtering. It is a fact about the inode as this look read it, the same register as `[denied]`.
- **Spelling and place:** `[empty]`, near-right with the look's other marks (`[denied]`, `[walk bound]`, `[cycle]`, `[other fs]`). Lattice-2 `empty-dir` row.
- **Display:** always, not quietable. Like a census at a cutoff, it is existence information ([[summarization|Summarization]]'s law), not a surprise.
- **Every line a directory stands on:** an expanded level, a depth cutoff (where an empty census printed nothing before this row), a symlinked directory whose target is empty (the target is what readdir read; the `-> target` decoration says how it got there), and the root, whose facts line carries the mark above the path.
- **JSON:** `"empty": true` on the node, following the `denied` / `walk_bound` / `cycle` booleans. An empty directory is a complete answer, so it does not set `truncated`.

## What is *not* `[empty]` (each already speaks, or is not known)

| Directory | What its line says instead | Why not `[empty]` |
|---|---|---|
| gitignored (`⊘`) | `⊘` in the far-left cell | Never read (the repo disclaimed it). Emptiness is unknown, and claiming it would be a guess. |
| unreadable | `[denied]` / `[unreadable: io]` | Never fully read. A readdir that failed partway is not an empty one. |
| cycle / other filesystem | `[cycle]` / `[other fs]` | Not read here. |
| holds only furniture the map **hides** (`.git/`, `.archive/`) | `[has: git]`, `[has: archive 1f]` | It holds things. The has-spot is the presence claim. |
| holds only gitignored files | `[ignored×N]` | It holds things. The typed remainder says so. |
| holds only names the map **omits** (`.DS_Store`) | `[dot-only]` (since 2026-10-03) | It holds things. Joseph's rule: any dir whose names are all `.`-prefixed gets its own status — [[dot-only\|Dot-only directory]]. |

## Interplay

- **Censuses are unchanged.** An empty directory folded into its parent's census is still a `dir` with zero files (the `0f` noise stays suppressed, per [[dir-census|Dir census]]). The mark is a fact of the directory's *own* line. A census does not grow an "empty" bucket.
- **Mass is unchanged:** zero files, zero lines, so the `lines` cell stays blank.
- `--show-all` does not change anything here, because emptiness is decided before any filtering.

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Expanded level | An empty child dir's line ends `[empty]`. | Fixture with `empty/` beside a file. |
| 2 | Depth cutoff | An empty dir at the cutoff says `[empty]`, never a bare line or an empty census. | `--depth 1` over `a/b/` where `b/` is empty. |
| 3 | Root | `aspectus EMPTYDIR` shows `[empty]` on the facts line above the path. | Empty-dir root. |
| 4 | Not claimed | Ignored dirs, hide-only dirs, ignored-file-only dirs, and omit-only dirs carry no `[empty]`. | Fixture of each. |
| 5 | Symlink | A link to an empty dir: `link -> target  [empty]`. | Link fixture. |
| 6 | JSON | `"empty": true` on the node; `truncated` stays false. | Parse fixture output. |
| 7 | Help | The honesty paragraph names `[empty]`. | Help scrape. |

## Open

- ~~**Omit-only directories render bare.**~~ **Resolved 2026-10-03** (Joseph): *"any directory with *only* '.'-prefixed children … should get some other kind of status"* — broader than the omit case, by name not by fate. [[dot-only|Dot-only directory]] (`[dot-only]`).

## Foundations

[[denied|Denied]] (the word-mark precedent, a fact the look must not render as plain) · [[summarization|Summarization]] (never a silent shape) · [[dir-census|Dir census]] (empty directories print no census; this row gives them their own line fact instead) · [[grid-cleanup|Grid cleanup]] (marks column; the `[denied]` word precedent; `∅` kept apart from `⊘`) · [[lattice-2|Lattice 2]] (`empty-dir` row).
