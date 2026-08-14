# bounded-walk — finish note

> [!note] **History, not the binary.** This describes the retired first-snapshot code; none of it is in the current crate (`impl/README.md`). "Landed" below is past-tense of that snapshot.


*Landed. Source: `src/walk.rs`. Tests: `tests/walk.rs`.*

`read_dir` + stat. Does not `read_dir` a “do not list” name unless `--raw` / `--inspect`. `--visit N` (default 400) counts entries processed; past that, leftover names go on the directory and `truncated` is set (render prints `≥`).

Symlink-to-dir is followed; `(dev, ino)` stops a cycle. Default: do not descend a different filesystem; `--no-one-fs` follows mounts.
