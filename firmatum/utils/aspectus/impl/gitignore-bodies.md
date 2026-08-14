# gitignore-bodies — finish note

*Wave E, 2026-08-14. `src/ignore.rs` (new); walk integration in `n_level.rs`; rendering in `columns.rs`; fields in `json.rs`. 11 tests (`tests/gitignore_bodies.rs`), real binary, real repos, one cross-checked against `git check-ignore` itself.*

## What shipped

- **In-process git-semantics matcher** (`ignore::File` / `Stack`): nested `.gitignore` chain (deepest wins; last match within a file wins), `!` negation, dir-only (`/`-suffix), anchoring, `**`, `[...]` classes, escapes; `$GIT_DIR/info/exclude`; **global `core.excludesFile` honored** (repo config → `~/.gitconfig` → XDG git config → XDG `git/ignore` default) — the design's Open, implemented as the recorded leaning (look agrees with the user's git). No subprocess anywhere on the walk path.
- **Tracked beats ignored** via a hand parser for `.git/index` v2–v4 (v4 prefix-compression included). A dir with tracked content beneath is never treated as wholly ignored. On an unparseable index the patterns still apply but the tracked override has no evidence — recorded limitation; git never writes such an index in practice.
- **Walk shape:** ignore state enters per-dir on the raw names (before furniture filtering, so `.git`/`.gitignore` are seen even though the map hides them); furniture fates apply first and their rendering wins — the ignore rules only ever see the map's leftovers, so no double marks by construction. A nested repo (real `.git` or gitlink) replaces the outer repo's rules, as git does.
- **Ignored dirs**: one `stat_only` line — presence, no readdir, no census, no mass. Their innards cost **zero** walk budget (subfeature 9's test: 200 files inside an ignored dir under `--walk 50` trip nothing). **Ignored files**: never statted at all; counted into the typed remainder (`[ignored×3]` on an expanded level, `· ignored×3` inside a cutoff census — one spelling, provisional).
- **Mass** excludes ignored bodies in the deep phase too (each deep worker seeds its own `Stack::for_path` from the repo above its cutoff dir). The exclusion is *declared*, not a floor — an ignored dir does not `≥`-bound its parent's aggregates.
- **Rendering** (steward asks folded in): ignored entries carry the glyph `⊘` (near-right, [denied]'s office; spelling provisional for ratification) and the name renders **dimmed on a TTY** — glyph is the carrier, dim the redundant overlay, per form-agentic-eyes' color-never-sole-carrier law. Dim replaces the dir bold-blue on those lines (bold fights faint).
- **`--show-all`** restores ignored contents, marks kept — and restored bodies **join mass** (implemented leaning on the design's Open; show-all means show all; flagged there).
- **JSON**: `gitignored: true` per node, `ignored_files: N` per parent, `ignored: N` in census objects. Ignored cuts are declared shape, not `truncated`.
- **has-vocabulary three-way split** (steward ask): already structurally present and preserved — `git` = verified repo machinery (facet), `gitignore` = has-an-ignore-file (furniture word), and the new per-entry status is `gitignored` (JSON field name) — three claims, three carriers, none blurred.

## Judgment calls (flagged in design Open where open)

- An explicitly named locus is looked at even if itself ignored — the ask wins; only children are checked.
- The cheap-census-of-an-ignored-dir idea (design Open): **not** taken — nothing is read inside; the repo disclaimed it.
- Glyph `⊘` (U+2298), remainder spelling `ignored×N`: one constant each, Joseph ratifies.

## Performance

`~/src` 1.3–1.5s (floor ≤2s), warm `~/src/arch` 0.32–0.34s (floor ≤~0.4s). Per-repo cost is file reads only (index + ignore files); skipping ignored dirs *reduces* walk work on venv/log-heavy trees.
