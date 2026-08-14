# git — finish note

*Landed. Source: `src/git.rs`. Tests: `tests/git_furniture.rs` (skips silently if no `git` on PATH).*

`.git` is not a child; the parent line carries `[git: remote<host/path> br<branch> @shorthead dirty<N>]`. No network anywhere. A submodule gitlink (`.git` file → `gitdir:`) and a linked work tree (`commondir`) read the same way — same furniture, tests cover both. Facts:

- **remote** — local config only, `origin` preferred, URL trimmed to `host/path` (scheme, `git@`, `.git` stripped). No remote → not shown.
- **branch** — from `HEAD`; detached prints `detached`.
- **short HEAD** — resolved from loose refs then `packed-refs` (gitdir, then commondir); unborn branch → no `@`. File parsing, no subprocess.
- **dirty** — the one `git` subprocess (`status --porcelain --untracked-files=normal`), count of dirty paths; clean prints nothing (quiet law). If git is unavailable or errors, the fact is not claimed — never guessed.

`(private)` is NOT here — that is the Private-remote row.

Open question from `design/furniture/git.md` (gitignore/gitattributes: map rows or plugin) — resolved as **map rows** feeding kind `git`; the plugin fires only on `.git` presence.
