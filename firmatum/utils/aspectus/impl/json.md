# JSON — finish note (2026-08-14, Wave D)

`--format json` (also config `format`, env `ASPECTUS_FORMAT`; flag wins)
serializes the **same look**: the emitter (`src/json.rs`, hand-rolled —
zero dependencies kept) receives the tree *after* the allocator, sort,
and annotations, so node set, expansion, censuses, and cuts equal the
text look's under identical settings (subfeature 1 asserts it). Text
stays default. `yaml`/`csv`/`tsv` are refused by the `--format` parse
with the reason; `udon` named as later.

Schema (v1, `"schema":1` from birth; field names provisional — design
Open "schema spelling" stays open at the naming level):

- Top level: `aspectus` (version), `schema`, `time` (the look's UTC
  stamp), `root` (absolute), `truncated`, `tree`.
- Node: `name`, `dir`, then facts when present — `lines`, `size`
  (always integer bytes), `mode` (octal string, 4 digits when special
  bits), `uid`/`gid` (numbers), `mtime` (iso-8601 UTC regardless of
  `format.*` config — **canonical time form chosen: iso string**, the
  overview invariant's; epoch was the alternative, Joseph ratifies),
  `heat` (number), `has`/`facets` (arrays — `has` follows the same-day
  `kind:`→`has:` rename), `link` + `broken`.
- Marks as data: `denied`, `walk_bound`, `cycle`, `other_fs`,
  `iter_err` — **present-when-true**; an absent mark is the honest
  false. `≥` is `"bounded":true` on census/mass objects.
- Censuses as objects: dir census `"census"` vs leaf census `"omitted"`
  (the `[N:…]`/`[+N:…]` distinction as field names), each
  `{total, dirs?, dir_name?, dir_files?, buckets:[{kind,n}], single?,
  bounded?}`. `mass` `{files, lines, estimated?, bounded?}` rides where
  the text look renders it (nodes with a dir census).
- Facts ride regardless of quiet — machine formats get facts, not
  affordances (quiet subfeature 12 asserts it).

Calls made:

- **`truncated`** = any denial, walk-bound cut, mid-iteration error,
  mount stop, or `≥`-floored aggregate anywhere. Depth cutoffs and
  line-budget folds are the look's honest *shape* (confessed by
  censuses), not truncation. **Exit stays 0 on a partial look** — the
  exit-code question is walk-bound's recorded open and was not decided
  here.
- **Refusal channel: stderr**, `{"error":{class, observed, next:[…]}}`,
  exit 2; stdout stays data-or-nothing. The design left the channel
  open — this is the provisional call, one constant to move.
  Machine-mode detection for refusals pre-scans argv + `ASPECTUS_FORMAT`
  (a config-*file* `format = json` still gets the text refusal — parse
  failures precede config resolution; recorded limitation).
- Key order fixed in the emitter, node order Sort's, floats via Rust's
  shortest-roundtrip `{}` — byte-identical across runs modulo `time`.
- One line, one document, trailing newline. No color, ever.

Tests: `tests/json_format.rs` (8, with a self-contained structural JSON
validator — no test dependencies either). Not in this landing: NDJSON
streaming, UDON, `git_ts`/last-touch as a JSON field (not in the text
look; revisit if an agent consumer asks).
