# parent-state-not-children — finish note

*Landed. Tests: `furniture_is_absorb`, `unknown_hidden_remains_child`, `walk_does_not_open_git_or_target`.*

Names the table marks “do not list” do not appear as children. A short label is printed on the directory line instead (`[git, rust, build]`). Hidden names the table does not know (`.orient`, `.mystery`) are listed.

The label string `build` is what the table stamps on `target/`, `node_modules/`, `__pycache__/`, `.build/`, `.ruby-lsp/`. The founding notes said `Cargo.toml` + `target/` → `rust`; the table stamps `build` and `rust` separately. That split is unsettleed.
