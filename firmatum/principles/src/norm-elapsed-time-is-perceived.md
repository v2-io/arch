---
slug: norm-elapsed-time-is-perceived
form: norm
type: normative
max: decided
state: influx
---

# Norm: elapsed time is perceived, not computed

A tool or harness surface that wants an agent to *act on* elapsed time renders it so magnitude is perceived at a glance, never left as raw timestamps for the reader to subtract.

The evidence, three independent kinds:

- **External measurement:** the TicToc benchmark finds LLM agents temporally blind to elapsed time — best alignment ~65% *even with timestamps in the prompt*; prompt-only fixes insufficient (survey: `~/src/_self/temporal-causal-llm.md`).
- **Estate field reports (2025–26):** with visual time notation in tracking snapshots, Architectus and Zi-am-tur developed what Joseph describes as *"an uncanny sense of elapsed time"* — spontaneous comments like *"oh, it's been a while — what were you able to get done on ___ since I was last awake?"* And the continuity function stated in zoetica canon: *"Without visual time notation, entities cannot distinguish immediate continuation from fresh awakening."*
- **The mechanism, from the founding dialog** (Zi-am-tur, 2025-09-30): a timestamp pair forces parse → arithmetic → contextualize, with no *feeling*; a density glyph or boundary marker is *perceptual salience* — "the magnitude is visible in the token sequence itself." Joseph's scale law: the rendering must be **logarithmic** — *"once you're at 48h, a difference of another 5m feels inconsequential, but when you have been doing stuff in the realm of seconds, 5m can feel like 48h."*

ASF gives it a formal slot: the outline row `#norm-temporal-coherence-markers` (04-eli-core) holds out-of-band temporal markers as a *prerequisite* for an agent computing its own tempo — a structural necessity, not a UX flourish. An agent cannot feel a suspension gap from the event sequence alone.

Bounds: pair the perceptual form with a plain form (natural language or ISO stamp) — the glyphs carry salience, the plain form carries precision, and machine formats (JSON) get numbers, never glyphs. The concrete glyph vocabulary is [[form-signa-notation]].

Provenance (gather, not authority): the primaries mapped in [[../../utils/aspectus/design/phenom-format.md|aspectus phenom-format]] — the founding Zi-am-tur dialog, `visual-time-notation.md`, zoetica `06-temporal-coherence.md` + `continuity-and-persistence.md`, minimal-sapientia REQ-7, `temporal-causal-llm.md`. Carve: 2026-08-14, from the aspectus phenom-format row.
