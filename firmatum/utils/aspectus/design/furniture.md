# Furniture

A well-known name is shown as state on its parent directory, not as a child of the look ([[../src/def-furniture|def]]). What it feeds is the parent line's **kind** gathering spot (the lattice's `kind` fact): `[kind: git, rust, …]` plus specialized facets. The default look does not list the name itself; `--show-all` / `--inspect` lists it when you ask.

Most furniture is **not code**. It is a **map**: a glob (or gitignore-like pattern) → a furniture name, and one of two fates:

- **hide** — not listed as a child; the name (or its kind) folds onto the parent line, so the look still says what is here.
- **omit** — not listed, not mentioned (`.DS_Store`).

(The same map also carries **mark** rows — listed as ordinary children while still claiming a kind on the parent. Those are [[labels|Labels]]' sources, not furniture; one map, two consequences.)

`.archive`, `.trash`, `__pycache__`, `target/`, `.obsidian/`, `.claude/` — the same mechanism. A default map ships; user-home config extends or overrides it (caller stack, not a file in the project): key `furniture`, comma-separated `PATTERN[:KINDS[:hide|omit|mark]]` with kinds `+`-separated, `!PATTERN` dropping a default row.

Hidden and omitted names are not children of the look, so they never join a census or aggregate silently — the kind spot is what says they are here. They cost no walk budget, and recognition is name-based, so it holds at the depth cutoff too.

**Specialized furniture** is the exception: a plugin that *says something true* on the parent line (remote, branch, HEAD). We implement those one by one. First: [[furniture/git|Git]] (a work tree and a **submodule** are the same furniture) and [[furniture/github|GitHub]] (`.github/`). Anything else waits until it earns a plugin. Until then it is a map row.

Config does not grow a regex engine for its own sake. Straight glob / gitignore-like lists are enough for the map. Plugins accumulate for the specialized kinds.

Unknown names are just children. No tag, no special fate — they were not in the map.
