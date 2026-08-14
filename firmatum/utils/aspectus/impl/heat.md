# Heat — finish note (2026-08-14, Wave C)

git-heat's model reimplemented (~90-line core, `src/heat.rs`), constants
verbatim: commit-age decay `exp(-age/τ)`, `τ = half_life/ln 2`, normalized
to the ~2 ceiling, initial commit excluded, dir = max of non-noise leaves,
noise basenames `Cargo.toml` / `SOURCE_REV`. Half-life via config
`heat.half-life` (default 7).

Calls made:

- **Log cap 400 commits** (newest-first, reversed in code): at τ≈10 a
  touch 400 back contributes ~e⁻⁴⁰ — zero to double precision — so scores
  are unchanged; last-touch times past the cap are simply not claimed
  (git-recency falls back to mtime there). Keeps the obtain cheap on deep
  histories without a cache.
- **Default ON in-repo** (lattice): rendered as the `score · age`
  right-cluster (`1.01 · 13.6d ago`), age = mtime delta vs the look's own
  stamp (human-relative units m/h/d/w/mo/y). No heat → whole cluster
  silent → outside git the column vanishes (paint drops width-0 columns).
- **Repo root detection:** the `git` *kind* is not proof of a repo root (a
  lone .gitignore claims the kind); only a real `.git` entry roots a log
  pass — found and fixed in dogfooding, the crate-in-a-parent-repo case.
  The repo-root *line* carries the repo's max heat / newest touch.
- **Parallel obtains:** one `git log --name-only` per repo, but a program
  root full of submodules pays them in parallel threads (serial cost was
  ~1.2s over ~/src/arch; now ~0.1s wall). Warm ~/src/arch look ≈0.4s
  total; `columns.heat = off` returns it to ≈0.3s.
- **Recency source:** `recency-source = git` makes the default recency
  sort use git last-touch where the log covered a path, mtime elsewhere
  (design/sort.md's "config chooses" — key spelling awaits ratification).
- `--sort heat` is a built key.

Not in this landing: prior-name (same log pass, later registration),
signa density, heat color ramp, cache-by-HEAD. Tests: `tests/heat.rs`
(7, real git fixtures with spaced commit dates).

## Sha facts landed (Wave E, 2026-08-14)

The `H~N` / initial-sha / latest-sha row now rides the same log pass:
`--name-status` replaced `--name-only` (same cost; letters + shas were
already in the stream), and per-file facts grew `touch` (newest commit
touching the path: full sha + commits-behind-HEAD) and `intro` (oldest
A/C in the window, an R's new name, or the initial commit when the log
reached it — **never claimed past the cap**; a capped log simply omits
introduction, and untracked files claim nothing). Compose-only columns:
`columns.initial-sha` / `columns.latest-sha` = on, formats `short*` /
`h~n` (`H` at HEAD, `H~4` behind) / `full`; JSON always carries the full
shas plus `initial_behind` / `latest_behind`. Files only — a dir's
"introducing commit" is not a claim this row makes. Sha *sort* keys stay
honestly unbuilt (refused by name). Heading spellings (`initial-sha`,
`latest-sha`) provisional. Tests: `tests/shas.rs` (7). A rename counts
as introducing the *name* (R's new path) — prior-name's row owns the
deeper story when it lands.
