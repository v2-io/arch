# Primary excerpt — the `--since` caveat as shipped, memorata3/search.py

Extracted with `git show`, not retyped. Repo `~/src/memorata`, commit `6c71570`
(2026-07-30); the sentence was deleted the same evening by `ee5fc5c`, which is
why it is staged here rather than cited in place.

```python
        "--since", metavar="YYYY-MM-DD",
        help="Drop results whose elected date is earlier than this. Applied "
             "to the retrieved pool (see --pool), not pushed into retrieval — "
             "a narrow window may therefore return fewer than -n even when "
             "more exist in the index; widen --pool if so.",
    )
```

The same two help strings as the tool actually rendered them — the string
values, concatenated by `ast` from the same blob rather than by hand:

**`--since`** — Drop results whose elected date is earlier than this. Applied to the retrieved pool (see --pool), not pushed into retrieval — a narrow window may therefore return fewer than -n even when more exist in the index; widen --pool if so.

**`--in`** — Restrict search to paths. A pattern WITHOUT glob characters is a path prefix matched inside the index — `--in ~/src/udon` means that file or anything beneath that directory, INCLUDING paths that no longer exist on disk (the index keeps ~15k gone paths as provenance). A pattern WITH globs (* ? [) is still expanded against the live filesystem, so it can only reach files that still exist and, per shell rules, skips dotfile dirs like ~/.claude/projects. Repeatable; combinable with --in-from; the union is the scope. Note the flag is greedy — put the query before it, or use a second flag, so the query isn't swallowed as another pattern.

