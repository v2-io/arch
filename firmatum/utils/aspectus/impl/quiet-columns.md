# Quiet columns — finish note (2026-08-14, Wave D, cold baseline)

The cold law implemented as designed (`src/quiet.rs`, ~250-line core):
decision only — it annotates `Node.q` / `Node.kind_word` on the
**pre-budget** tree (level statistics over the full statted set, so
`--lines` cannot flicker quiet), and columns.rs renders. JSON carries the
underlying facts (mode/uid/gid/mtime/size) regardless — quiet is a
text-rendering law, not a data cut.

New substrate: every stat now also keeps `mode`, `uid`, `gid` on the node
(free — the stat had already been paid for). `columns.size/mtime` moved
from bool to tri-state (`columns::State`); `columns.permissions`,
`columns.owner`, `columns.filekind` are new quiet-default facts
(inventory rows flipped to built).

Calls made (all defaults awaiting ratification-on-contact):

- **Dial spelling:** `quiet.sensitivity` (default 1.0) + per-fact
  `quiet.sensitivity.size` / `.mtime` (env `ASPECTUS_QUIET_SENSITIVITY*`).
  mtime's window divides by the dial (higher = harder to surprise).
- **Constants:** size floor 256 KiB; recency window 1 day; cohort
  minimum 3; log-deviation threshold = the dial itself.
- **Majority = plurality with a unique max** (ties → no norm →
  convention leg decides alone). A single file is its own majority, so
  a lone `600` never cries.
- **Bin-like level:** files `755` count as conventional when >half the
  level's files carry any exec bit.
- **Special bits** (`setuid`/`setgid`/`sticky`) always speak, rendered
  4-digit octal (`4755`). macOS file *flags* (`uchg` etc.) are **not
  obtained** — absent, never faked; a later obtain can join the same law.
- **Owner rendering:** `/etc/passwd`-`/etc/group` name lookup (covers
  root everywhere; Directory-Services-only users fall back to the numeric
  id — honest, deterministic per machine). `format.owner = name|id`.
  Group leg compares **egid only** — the caller's supplementary group
  list is a recorded gap.
- **Kind word** (`binary` / `text`) lands near-right, before censuses.
  `Unknown` kinds are absent from the statistics and never speak (the
  sniff is a read; quiet spends no reads).
- **Caller identity** via direct `geteuid()`/`getegid()` FFI — no crate
  added; the tool stays zero-dependency.

Tests: `tests/quiet.rs` (12, the design's cold-wave table; subfeature 5
root-owned is covered at the decision layer in `src/quiet.rs` unit tests
— a real root-owned fixture needs privilege; subfeature 11's fixed fake
clock proved unnecessary — pinning fixture mtimes old keeps the window
stable without a wall-clock knob, which would itself have been a
determinism-law surface). Pre-existing fixtures that create fresh files
gained `backdate()` — their files now honestly surprise on recency, which
those tests were not about.

**Lived-use observation for the design's own Open:** on an active tree
(this repo, mid-wave) nearly every line speaks its mtime, right next to
heat's `· 0m ago` — the redundancy the design already suspects
(mtime-quiet vs the recency sort + heat cluster). Kept per spec; the
warm baseline is its real payoff. Not in this landing: warm baseline
(gated on Last look's store), `quiet_reasons` in JSON (leaning no).
