# Furniture / Git

A git work tree does not list `.git` as a child. The parent line can say what git *is here*: local remote, branch, short HEAD, porcelain. No network in the default look. `(private)` / public is its own row, elevated to sit beside this plugin ([[../private-remote|Private remote]]) — the known-path is `gh`, and how a `gh` call squares with no-network is that row's design question to work through early.

A **submodule** is the same furniture. Nested `.git` (dir or gitlink) is hidden the same way, its facts on the parent; the parent line of that child directory can carry the same facts. It is not a second kind and not a child you open to discover it is a repo.

`--show-all` / `--inspect git` lists `.git` when you ask.

Related, specialized: [[github|GitHub]] for `.github/` (workflows, not objects). Gitignore / gitattributes names can be map rows, or hang on this plugin — undecided, not a third kind.
