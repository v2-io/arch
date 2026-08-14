# Furniture

A well-known name is shown as state on its parent directory, not as a child of the look ([[../src/def-furniture|def]]). What it feeds is the parent line's **has** gathering spot (the lattice's `has` fact, renamed `kind:`→`has:` 2026-08-14 — the map detects contents, not identity): `[has: git, rust, …]` plus specialized facets. The default look does not list the name itself; `--show-all` / `--inspect` lists it when you ask.

Most furniture is **not code**. It is a **map**: a glob (or gitignore-like pattern) → a furniture name, and one of two fates:

- **hide** — not listed as a child; the name (or its kind) folds onto the parent line, so the look still says what is here.
- **omit** — not listed, not mentioned (`.DS_Store`).

(The same map also carries **mark** rows — listed as ordinary children while still claiming a kind on the parent. Those are [[labels|Labels]]' sources, not furniture; one map, two consequences.)

`.archive`, `.trash`, `__pycache__`, `target/`, `.obsidian/`, `.claude/` — the same mechanism. A default map ships; user-home config extends or overrides it (caller stack, not a file in the project): key `furniture`, comma-separated `PATTERN[:KINDS[:hide|omit|mark]]` with kinds `+`-separated, `!PATTERN` dropping a default row.

Hidden and omitted names are not children of the look, so they never join a census or aggregate silently — the has-spot is what says they are here. They cost no walk budget, and recognition is name-based, so it holds at the depth cutoff too.

**Presence must survive hiding (three independent testimonies, 2026-08-14** — `audit/hallway-2026-08-14.md`): full invisibility of a hidden dir re-hides a mass — *"there's a difference between 'don't go in' and 'doesn't exist'"* (the `.archive` avoidance advisory works *because* agents know it's there; hiding `.claude` from an agent caller is "a self-own"). Leaning for ratification: the hidden thing's magnitude folds into the has-facet — `[has: archive ≈127f, git, …]` — presence and weight without a child slot; very large hidden masses may earn a single line; and the agent-caller default map may hide less (`--caller` overlay territory). Joseph ratifies the shape. *Facet-mass implemented 2026-08-14 (hardening pass): hidden dirs get a readdir-only deep file-count (cap 20k names, `≥` floor; specialized kinds git/github excluded — their facets speak), rendered on the kind word and carried in JSON as `hidden`. The single-line-for-huge-masses idea and the agent caller map remain unimplemented, Joseph's.*

**Specialized furniture** is the exception: a plugin that *says something true* on the parent line (remote, branch, HEAD). We implement those one by one. First: [[furniture/git|Git]] (a work tree and a **submodule** are the same furniture) and [[furniture/github|GitHub]] (`.github/`). Anything else waits until it earns a plugin. Until then it is a map row.

Config does not grow a regex engine for its own sake. Straight glob / gitignore-like lists are enough for the map. Plugins accumulate for the specialized kinds.

Unknown names are just children. No tag, no special fate — they were not in the map.
