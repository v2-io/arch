# Walk bound

Walking a huge tree stops after N names looked-at and says so. This is not the line budget: `--lines` is how much we *print*; this is how many names we will even *stat*.

- An aggregate cut short by the bound is marked `≥` — a bounded census, count, or [[mass|mass]] never prints as exact, and never enters the [[cache|cache]] as exact.
- A directory the bound never entered is treated like a depth cutoff: its line says what is known, honestly partial.
- The bound rides the caller stack like `lines` and `depth`.

**Flag spelling open.** The first snapshot used `--visit N` (default 400) — a spelling Joseph read cold and did not understand ("I don't know what --visit … means", [[../ORIGIN-DISCUSSION.md|origin]]). Pick a name that teaches itself (`--max-stat`? `--walk N`?) when this lands; do not inherit `--visit` by default.
