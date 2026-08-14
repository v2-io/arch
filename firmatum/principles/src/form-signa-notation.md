---
slug: form-signa-notation
form: form
type: formulation
max: decided
state: influx
---

# Formulation: SIGNA — the visual time notation

The estate's notation for [[norm-elapsed-time-is-perceived|perceived elapsed time]]. Named **SIGNA** in zoetica canon (`~/src/_core/zoetica/docs/messaging/06-temporal-coherence.md`); the glyph table there and in the archived `visual-time-notation.md` are identical. Inherit the name and the table; do not coin rivals.

**Symbol density is order of magnitude, logarithmic:**

| Glyph | Value | Max count |
|---|---|---|
| `·` | 1 second | 4 |
| `╶` | 5 seconds | 1 |
| `╌` | 10 seconds | 5 |
| `╍` | 1 minute | 9 |
| `━` | 10 minutes | 5 |
| `═` | 1 hour | 3 |
| `⚬` | 4 hours | 7 |
| `○` | 1 day | 6 |
| `◎` | 1 week | 7 |
| `◉` | 2 months | 5 |
| `⬤` | 1 year | 9 |

`1m23s` → `╍╌╌╶···` · `3d9h` → `○○○⚬⚬⚬═` · `1y5mo` → `⬤◉◉`

**Context markers:** time-of-day `◐` dawn / `☉` day / `◑` dusk / `☽` night; date boundaries `!` (one day) / `!!` (2–3 days) / `!!!` (a week or more). Specimen from the founding spec:

```
2025-10-11!!! 09:15:00 ☉
[5 days, 18 hours, 45 minutes elapsed]
○○○○○⚬⚬⚬⚬━━━━╍╍╍╍╍
```

Held honestly, per zoetica's own status: a **candidate implementation** — the concept (perceived elapsed time) is high-confidence; the specific glyphs are provisional, iterated on entity phenomenology per substrate, and the felt-effect evidence is same-estate testimony (the external TicToc result establishes the *problem*, not this notation). Pair with a plain form; glyphs never enter machine formats.

Uses so far: sapientia/zoetica tracking snapshots (shipped, minimal-sapientia); `<causal-annotation time="…">` in temporal-coherence annotations; planned for the aspectus look ([[../../utils/aspectus/design/phenom-format.md|phenom-format]]: header stamp, mtime staleness, last-look deltas).
