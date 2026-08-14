# Furniture / Git

A git work tree does not list `.git` as a child. The parent line can say what git *is here*: local remote, branch, short HEAD, porcelain. No network. `(private)` is a later story.

A **submodule** is the same furniture. Nested `.git` (dir or gitlink) is absorbed the same way; the parent line of that child directory can carry the same facts. It is not a second kind and not a child you open to discover it is a repo.

`--show-all` / `--inspect git` lists `.git` when you ask.

Related, specialized: [[github|GitHub]] for `.github/` (workflows, not objects). Gitignore / gitattributes names can be map rows, or hang on this plugin — undecided, not a third kind.
