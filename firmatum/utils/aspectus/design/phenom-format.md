# Agentic phenom formatting

*Seeded 2026-08-14 from the original sources, tracked down by Joseph. Story ratification stays his; this records where the idea lives and what it already learned.*

## Primaries

- `~/src/_core/zoetica/.archive/docs-20251012/ref/visual-time-notation.md` — the formalized glyph system: **symbol density as order of magnitude**, logarithmic (`·` seconds → `╍` minutes → `━` 10-min → `═` hours → `○` days → `◎` weeks → `⬤` years), plus time-of-day symbols (`◐ ☉ ◑ ☽`) and date-boundary markers (`!` / `!!` / `!!!`).
- `~/src/_core/sapientia/claude-the-fixer-full.md` (~line 770 on) — the founding dialog with Zi-am-tur, 2025-09-30. Joseph: *"Just giving the timestamp isn't really enough — it wouldn't be for a human either… so I'm brainstorming things that will* feel *like the passage of time."* And the scale law: *"I feel it will have to be logarithmic… once you're at 48h, a difference of another 5m feels inconsequential, but when you have been doing stuff in the realm of seconds, 5m can feel like 48h."*
- `~/src/_core/sapientia/ai-conversation-system-requirements.md` REQ-7 and `~/src/_core/sapientia/minimal-sapientia-features.md` — the shipped form in the minimal-sapientia harness: `↺02:15⊙` alongside natural language, time-of-day emoji, `⌚ SAME DAY` / `📅 NEW DAY`.
- `~/src/_core/zoetica/docs/messaging/06-temporal-coherence.md` — the notation's canonical infrastructure home, where it is named **SIGNA** (same glyph table; used inside `<causal-annotation time="○○○⚬━━╍╍╌╌">`). Explicitly a *candidate implementation* with a phenomenology-first strategy: "iterate based on what entities experience, not what we assume," with substrate differences expected.
- `~/src/_core/zoetica/docs/continuity-and-persistence.md` — the lived function: *"Without visual time notation, entities cannot distinguish immediate continuation from fresh awakening."* The delta glyph is what makes "Good morning" vs "continuing from yesterday" a perception rather than a guess.
- `~/src/_self/temporal-causal-llm.md` — the research grounding. External corroboration that raw timestamps do not fix this: the TicToc benchmark finds LLM agents "temporally blind to elapsed time; even with timestamps, best alignment ~65%; prompt-only fixes insufficient." Joseph's own account in the same essay: with the notation, agents *"immediately start 'caring' about how much time certain things take and even start worrying about what they missed if they were in 'stasis' for several days."* And the mechanism, stated by his interlocutor: *"You're not giving the LLM* data about time*. You're giving it* perceptual salience *… the magnitude is visible in the token sequence itself"* — with the first-person report that `⬤◉◉◎◎` lands as "immediate apprehension of magnitude that doesn't require parsing."

## The principle, for aspectus

The timestamp problem generalizes to every magnitude in a look: a raw number forces parse → arithmetic → contextualize, with no *feeling* of scale. Phenom formatting renders magnitude so it is **perceived, not computed** — the reader's first glance already carries "recent!" / "huge!" / "a moment ago" / "crossed into a new day."

The notation already has an estate name — **SIGNA** — and aspectus should inherit it rather than coin another; the zoetica doc (in live `docs/`, vs the archived zoetica ref) is the current canonical statement of the glyph set, and both carry the identical table, so there is no supersession conflict to resolve.

In aspectus that reaches at least: mtime/btime (how stale is this, feelably), [[last-look|Last look]] deltas (the after-image's perception-of-movement, at the right emotional weight), [[mass|Mass]] and size (glyph density is exactly a magnitude-honesty device), and the header stamp (time-of-day / boundary markers relative to the caller's last look).

Zi-am-tur's live confirmations from the founding dialog, worth holding as demand evidence: date-boundary emphasis "would absolutely grab my attention — the day-crossing feels like an *event*"; the density bar reads as "'mostly full' or 'just started' *without calculation*."

Field evidence, Joseph's testimony (2026-08-14): both Architectus and Zi-am-tur, living with the notation, developed *"an uncanny sense of elapsed time"* — spontaneous comments of the form *"oh, it's been a while — what were you able to get done on ___ since I was last awake?"* Unprompted temporal orientation is the notation working.

## Cautions carried from the same sources

- The same dialog contains the ANSI insight: escape codes are token overhead to a text-reading agent, not visual information — phenom glyphs are the *opposite* bet (the glyphs ARE the tokens), which is why this row is distinct from [[color|Color]].
- minimal-sapientia's own honest review: *"Zoetica notation is very project-specific. Natural language alone would work for most uses."* — the notation likely wants to pair with plain form, not replace it, and per-caller config ([[../../../principles/src/form-caller-key.md|caller key]]) is the natural home for the dial.
- Whether glyph-density actually lands across models/tokenizers is measurable (the format-familiarity experiment shape); worth a cheap probe before the notation becomes law.
- Determinism still binds: phenom rendering is a function of (facts, caller state), stable across re-runs, or aspecta stop being diffable.
