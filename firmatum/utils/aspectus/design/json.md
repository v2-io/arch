# JSON

*Shipped 2026-08-14 (Wave D) — [[../impl/json.md|finish note]] carries the schema calls (v1 field names, `has`/censuses/mass objects, present-when-true marks).*

`--format json` is the **same look as text** — same walk, same budget, same censuses, same marks — serialized instead of drawn. Text stays default. This is transport-guaranteed, not picture-true (Part II: a JSON Aspecta is verifiable-as-transport, not true-as-picture; do not advertise constrained decoding we don't do).

"Same look" is the binding claim: a machine caller and a human caller looking at the same tree with the same caller state see the same facts, the same cuts, the same honesty. JSON never gets a deeper walk, a bigger budget, or extra facts as a side effect of being machine-shaped.

## The honesty marks become data

Everything the text look confesses with glyphs must exist as fields, not as strings the caller regexes out of names:

- **Censuses** are objects: total plus typed buckets (`{"census": {"total": 9, "buckets": [{"kind": ".md", "n": 3}, {"kind": "dir", "n": 2}, …]}}`), dir census and leaf census distinguishable (the text look's `[N: …]` vs `[+N: …]`).
- **`denied`**, **walk-bound cut**, and **cycle/mount marks** are booleans/enums on the node (`"denied": true`, `"walk_bound": true`), never only rendered text.
- **`≥` (genuinely-unknown aggregate)** is a flag on the aggregate, per [[walk-bound|Walk bound]]'s law: exact counts stay exact; unknown says unknown.
- **A look-level `truncated`** (or equivalent) summarizes whether any cut/denial occurred, so a caller can branch without walking the tree. Its relationship to the **exit code of a partial look** is deliberately open in [[walk-bound|Walk bound]] — this row depends on that decision and does not make it; the outline's sequencing note (2026-08-14) already says it should be decided before this row lands.

## Formats collapse to canonical

Text formats are for eyes; JSON refuses them (lattice `unique` cells already say so):

- **Sizes always bytes** (integers). No `human`, no `log`.
- **Times in one canonical form** (leaning iso-8601 UTC strings, matching the overview invariant; epoch is the alternative — one constant, see Open).
- **No SIGNA glyphs ever** ([[phenom-format|phenom-format]]'s own rule: always paired with a plain form, never in JSON).
- **No color, no alignment padding.** Deterministic key order and node order (Sort's order), so two aspecta diff as data — the determinism claim from the outline working notes applies byte-for-byte here too.
- **Overview invariants** (absolute perspective root, UTC time of the look) are top-level fields, not a synthetic first node.

## Errors

In machine mode a failure carries a structured error on the machine channel (Part II headless contract): the refusal object (class, observed, next actions) as JSON, non-zero exit. Success emits the aspecta on stdout and nothing else.

## Foundations (clauses)

| Clause | Where |
|---|---|
| Serialization, not constrained decoding; do not advertise the wrong guarantee | [[../../../principles/influx/structured-output-two-mechanisms\|Structured output]] |
| Format flag family; stdout carries data; structured errors in machine mode | [[../../../principles/influx/headless-io-contract\|Headless]] |
| Truncation is a verdict, as data (`truncated: true`), never a tree that looks complete | [[../../../principles/influx/streaming-and-partial-documents\|Partials]] |
| The marks this row serializes | [[summarization\|Summarization]] · [[denied\|Denied]] · [[walk-bound\|Walk bound]] |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Same look | For a fixture tree, the node set, expansion choices, and census numbers in JSON equal the text look's under identical settings. | Parse both; compare. |
| 2 | Valid transport | Output is a single valid JSON document on stdout; stderr empty on success; exit 0. | `aspectus --format json \| jq .` |
| 3 | Bytes and canonical times | Sizes are integers (bytes); times one canonical form regardless of `format.*` config. | Config `format.size = human`; JSON still bytes. |
| 4 | Marks as data | Denied, walk-bound, censuses, `≥` appear as fields; no glyph-only fact exists. | Fixtures for each mark; assert fields. |
| 5 | Look-level truncated | Any cut or denial anywhere sets the top-level flag; a complete look clears it. | Bounded vs unbounded run on the same fixture. |
| 6 | Determinism | Two runs, same tree and caller state: byte-identical JSON, TTY or pipe. | Diff runs under different `COLUMNS`/pty. |
| 7 | Structured refusal | Bad usage with `--format json`: refusal object on the machine channel, exit 2. | `aspectus --format json --no-such`. |
| 8 | Help | `--format` documented with its values; examples gain a json line. | Help snapshot. |

## Open

- **Schema spelling** — shipped v1 as the leaning (children arrays, facts on the node, `"schema":1` from birth); field *names* stay ratifiable while consumers are zero.
- ~~**Canonical time form**~~ **Shipped: iso-8601 UTC string** (the overview invariant's form); epoch was the alternative — Joseph ratifies.
- ~~**Where the refusal object goes**~~ **Shipped provisionally: stderr** (stdout stays data-or-nothing); one constant to move if the call goes the other way. Config-file-set `format = json` still gets a text refusal on parse errors (parse precedes config resolution — recorded limitation).
- **Partial-look exit code** — owned by [[walk-bound|Walk bound]]'s open question; decide there first.
- `--format` spelling family already flagged open in the outline.

## Not in this row

NDJSON streaming (later, if we stream nodes). UDON (its own row, after the IR is stable; JSON is not gated on it). csv/yaml/tsv (refused, Part II). Any fact's semantics — this row serializes what the look already says.
