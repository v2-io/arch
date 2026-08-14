# Furniture

Well-known guts are not children. The default look does not list them. `--show-all` / `--inspect` lists them when you ask.

Most furniture is **not code**. It is a **map**: a glob (or gitignore-like pattern) → a furniture name, and whether that name is absorbed (folded onto the parent, not listed) or omitted (not listed, not mentioned). `.archive`, `.trash`, `__pycache__`, `.DS_Store`, `target/`, `.obsidian/`, `.claude/` — the same mechanism. A default map ships; user-home config extends or overrides it (caller stack, not a file in the project).

**Specialized furniture** is the exception: a plugin that *says something true* on the parent line (remote, branch, HEAD). We implement those one by one. First: [[furniture/git|Git]] (a work tree and a **submodule** are the same furniture) and [[furniture/github|GitHub]] (`.github/`). Anything else waits until it earns a plugin. Until then it is a map row.

Config does not grow a regex engine for its own sake. Straight glob / gitignore-like lists are enough for the map. Plugins accumulate for the specialized kinds.

Unknown names are just children. No tag, no special fate — they were not in the map.
