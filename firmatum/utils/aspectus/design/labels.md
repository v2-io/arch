# Labels

A label on a line is a **claim**, not a guess dressed as one. An empty set prints nothing — no `[kind: ]`, no placeholder.

The shipped label is the **kind** fact of the [[aspect-lattice|lattice]]: the parent line's gathering spot, `[kind: git, rust, …]`, sorted and deduplicated. It is fed by names the map knows:

- **hide** rows ([[furniture|Furniture]]): the name is not a child; its kinds are the state that says it is here.
- **mark** rows: the name stays an ordinary child *and* its kinds are claimed on the parent — `Cargo.toml` → `rust`, `pyproject.toml` → `python`, `CLAUDE.md` → `agents`. Mark rows are label sources, not furniture: they occupy a child slot because seeing them is worth a line.

A rule may claim several kinds when the name honestly supports them all (`__pycache__/` → `build`, `python`). It claims none it cannot: `target/` claims only `build` — maven uses `target/` too, so `rust` from that name alone would be a guess; the `rust` claim arrives with `Cargo.toml`.

Unknown names claim nothing and stay children. Files do not carry kind labels on their own lines — kind is a place fact (the lattice's `kind` row), and the suffix already rides in the name.

`--kind` filtering ([[kind-query|Kind query]]) waits on nothing further: the claims now exist.

Config: the same `furniture` key that extends the map extends the labels (`PATTERN[:KINDS[:hide|omit|mark]]`, kinds `+`-separated) — one map, two consequences.
