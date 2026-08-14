# readme-title — finish note

*Wave E, 2026-08-14. Peek in `n_level.rs::readme_title`; set-borrow via `important::Set::first_match`; rendering in `columns.rs::rest_of`; field in `json.rs`. 9 tests (`tests/readme_title.rs`).*

## What shipped

- **Shipped default OFF with the key working** (`readme-title = on|off`, env `ASPECTUS_README_TITLE`) — the design's ON-vs-QUIET Open stays Joseph's; he ratifies the default by flipping one value.
- Source file: the **important-files set**, borrowed via one method defined on `Set` (`first_match`) so the two rows cannot drift; config-list order picks among patterns, names within a pattern tie in sorted order.
- Title: first ATX heading (`#`–`######` + space, closing hashes stripped) within a **4 KB head-peek** (`TITLE_PEEK`), else first non-empty line; light emphasis strip (`*`/`_`/`` ` `` surrounds); truncation at 60 chars + `…` (`TITLE_CAP`, provisional).
- Truthful or silent: binary (NUL in the window), unreadable, empty, or past-the-window titles lend nothing — absence, never an error or placeholder. **Redundancy guard**: title equal to the folder name after case/punct-normalization prints nothing (`md-press/` + `# MD Press` is silent).
- Works on expanded *and* depth-cutoff dirs (the peek rides the same enumerate). Zero cost when off (no reads attempted).
- **Rendering: near-right, quoted, first in the rest-spot** — the design's stated office is INFO-hanging-on-the-name, which is this codebase's near-right class. The columns wave's `facts.rs` had provisionally filed the title under *decoration*; decoration inflates the name column, which would push every titled row past the tab-stop cap and shred the fact columns — near-right keeps the columns whole. Divergence-from-the-seed noted here for ratification; `facts.rs` row updated to near-right.

## Open still open

- Root-line title (the design's third Open): not shown — the reader is already inside; one constant when wanted.
- Quoting/dimming spelling and the 60-char cap — ratify.
- No frontmatter `title:` keys (Not-in-this-row holds).
