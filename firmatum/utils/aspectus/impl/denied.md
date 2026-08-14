# Denied — finish note (2026-08-14)

Landed per [[../design/denied.md|design]]. An unreadable dir or name says so; nothing renders as empty or plain.

- `gather_at`: `enumerate` (readdir) failure → the node itself is `[denied]` (root included — a denied root prints its header line with the mark, exit 0: the look succeeded at saying what it could not see). A child whose stat fails → denied child line (dir-ness from the `read_dir` file-type hint), and the look continues; one denied child no longer kills the whole look (previously `?` propagated).
- At the depth cutoff the same `enumerate` path applies: failure → `[denied]` on the dir line, never an empty census (the old code returned a silent zero-census).
- Mid-iteration entry errors (`ReadDir` yielding `Err`) mark the parent `[unreadable: io]` — the class-carrying render the design allows.
- A denied dir contributes no census; when children are folded into a dir census and one of them is denied, the census is marked `bounded` → `[≥N: …]` (the parent's aggregate is incomplete, same mark as [[../design/walk-bound.md|walk bound]]).
- INFO hangs on the name, default ON, no quieting flag added.
- Help: the closing honesty paragraph names `[denied]`, same commit.
- Tests: `tests/denied.rs` (4) — expanded denied dir, denied-at-cutoff not an empty census, denied root, help teaches. chmod-000 fixtures; skip with a stderr note when euid 0.

Not covered yet (waiting on their rows): denied's interaction with [[../design/mass.md|Mass]] (no mass contribution) and JSON (`denied: true` on the node).
