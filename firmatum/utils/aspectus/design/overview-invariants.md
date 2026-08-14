# Overview invariants

Every look states the facts that locate it, on the header, before the children.

1. **Perspective root** — the absolute path of the locus, not `./`, not only the basename. `aspectus` and `aspectus PATH` both print the same kind of root: `std::path::absolute` of the path that was looked at (logical absolute, not necessarily `realpath`).
2. **Time of the look** — the system datetime when this aspecta was made. Default format is ISO-8601 / RFC-3339 UTC (`YYYY-MM-DDTHH:MM:SSZ`). Other formats wait on the lattice `format` office.

The header is **two lines** — root, then stamp (decided by Joseph, 2026-08-14: *"root and datetime snapshot should/could be their own lines. From machine perspective and human perspective, those newlines only make things more comprehensible"* — superseding the earlier one-line form):

**Order: stamp first, then root** — so the root line sits directly above its children and reads as connected to them (Joseph's vote, same day):

```
2026-08-14T06:12:03Z
/Users/joseph/src/arch/firmatum/utils/aspectus/
├── design/
└── src/
```

And the root path line stays **simple** (Joseph, same day: "We're going to need a simple header line :-)"): the path and nothing else. The root's own facts — heat·age, `[git: …]`, `[has: …]` — move to their own header line(s) between stamp and path or after the path (renderer's judgment on which reads connected to the tree), never crowding the path itself:

```
2026-08-14T20:33:52Z
0.35 · 2.2w ago  [git: remote<github.com/v2-io/agentic-systems> br<main> @3a705fe]  [has: agents, build, git, …]
/Users/josephwecker-v2/src/arch/asf/
├── audits/  …
```

No `./` child. The stamp is when *this* print ran, not mtime of the directory.

## Foundations

| Clause | Where |
|---|---|
| Stdout is the picture | [[../../../principles/src/norm-stdout-is-data\|norm-stdout-is-data]] |
| Success is quiet | [[../../../principles/src/norm-success-is-quiet\|norm-success-is-quiet]] |
| Paths named on the command line are CWD-relative; the *printed* root is absolute | [[../../../principles/src/norm-paths-relative-to-cwd\|norm-paths-relative-to-cwd]] (invoke) |

## Not in this row

Canonicalize / realpath. Local timezone offset. Extra header fields (uid, hostname) until someone names them.
