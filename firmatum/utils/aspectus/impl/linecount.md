# Line counts — finish note (2026-08-14, Wave C)

Landed as designed, uncached-first. `src/kind.rs` is the suffix-map
(shipped text/binary suffixes + extensionless text names; config key
`kinds = "SUFFIX:text|binary, !SUFFIX"` overrides, env `ASPECTUS_KINDS`);
counting is `physical*` / `non-blank` via `format.line-count`. Binary
omits, never 0; empty text is a real 0; unterminated final line counts.

Calls made (each flagged in the design's Open, now resolved here):

- **Unknown suffix:** the design's leaning implemented — well-known
  extensionless names ship in the map, everything else gets a null-byte
  sniff of the first 1KB. The sniff decides count-vs-omit only, never a
  rendered kind word, so "kind from a map, not magic" is intact.
- **Cost bound:** the lattice's ON survives contact — a **read budget**
  (config `reads`, bytes, default 64MB, 0 = unlimited) bounds file-content
  reads. *Visible* files are read even past the budget (the look's own
  lines stay exact); the deep mass walk is where the budget bites, and
  there the degraded form is estimation from `st_size` at the look's own
  observed bytes/line (marked `≈` — see impl/mass.md). Measured: warm
  ~/src/arch stays ~0.3s without heat, ~0.4s with.
- **Grouping:** per-file counts render full; grouping (`61k`) is the mass
  channel's only.

Not in this landing: btime rider (lattice OFF — no obtain built), signa
format, cache. Tests: `tests/linecount.rs` (9), unit tests in kind.rs.
