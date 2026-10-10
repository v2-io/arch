---
slug: norm-three-audiences
form: norm
type: normative
max: decided
state: influx
decided-by: steward
---

# Norm: output has three audiences, not two

A tool's output has three kinds of reader, and each has its own channels:

| Audience | Who reads | Channels it has | Channels it lacks |
|---|---|---|---|
| **human-with-terminal** | a person at a TTY | everything below, plus ANSI (color, emphasis, cursor movement) as an information and comprehension channel | — |
| **agentic** | a token-reading mind, through a pipe | alignment, tables, censuses, labels and units, and careful Unicode: the perceptual look of [[form-agentic-eyes]] | ANSI, and any glyph its tokenizer delivers as something else |
| **deterministic** | a program in a pipeline | fields: JSON, udon, or, for simple scripts, `\0`-delimited records | every affordance; it parses, it doesn't perceive |

The agentic reader is its own audience. It is neither a human without colour nor a parser.

Joseph, 2026-10-10: *"we really do need to start thinking in terms of 'human-with-terminal' (i.e., allows ansi affordances as an info/comprehension channel), 'agentic' (i.e., avoid ansi and certain glyphs, but careful unicode (as per arch/asf/empirica/glyph-sequence-perception/ findings) affordances available, as is (still) alignment etc.), vs 'deterministic' (i.e., json or udon etc. or in simpler scripts, \0 delimiters etc.-- meant for pipelines to other deterministic routines). Too much currently puts agentic too much into first or the second other ones."*

## Why

The two-way split (TTY or not) sends every agent down the machine path, because agents call tools through pipes. The agent then gets verbose JSON and has to reassemble its meaning itself. Joseph, the same day: *"all of the frontier agents I work with would be able to get a feel for the data in tables and meaningful aspectus-like … but it's definitely not verbose json that must be conceptually reassembled for semantic meaning"*.

The occasion, measured on 2026-10-10 in `ai-risk-model`: `bin/source-search` defaults to JSON whenever stdout isn't a terminal. Four source-expert agents judging its output each wrote a script to compress that JSON into a digest before they could reason about it, and put the cost at about 250 to 1,000 tokens a result. Memorata's agent default is JSON too. The gathered convention this suite descends from equates the two outright: its "agent mode" is non-TTY output, with "structured output preferred" and `--format=json` (`../influx/cli-conventions/ai-agent-considerations.md`).

## Careful Unicode

"Careful" means a glyph whose reading has been checked across model families, not assumed. The evidence is in `arch/asf/empirica/glyph-sequence-perception/`. Its MANIFEST holds no claims yet, so everything below is at observation tier:
- Many glyph runs are recognised as sequences by minds of several families (`BEST.md`: block-element fills, circled and parenthesized letters, super- and subscript digits, and more).
- **U+2581 `▁` isn't the bottom of the fill ramp for Gemini.** Both Gemini models leave it out of the ramp `▁▂▃▄▅▆▇█` (Gemini Flash in 81% of the triads holding it, Gemini Pro in 51%), while Claude and Grok order it in. The hypothesised mechanism is that SentencePiece delivers it as its word-boundary marker. This is a supported observation; the tokenizer itself hasn't been inspected (`OBSERVATIONS.md`, 2026-10-04). A ramp that has to read the same to every family should start at `▂`, or say what `▁` means.
- Families sometimes split on what comes next, mostly by keeping a glyph's form against its meaning (`OBSERVATIONS.md`, 2026-10-06; seven cases).

## Collisions

Two atoms carved before this one conflate the agentic and deterministic audiences:
- [[norm-machine-path-is-detected]]: "Non-TTY stdout … select[s] the machine path";
- [[form-agentic-eyes]], concern 6: "Machine formats get facts, not affordances … glyphs, alignment, SIGNA, and color exist only for the perceptual look".

By this directory's rule, the steward's words win and those atoms get rewritten. That rewrite isn't done in this carve. Each now carries a line pointing here.

## How a tool tells them apart (proposed, not decided)

Claude's proposal, 2026-10-10, for Joseph to accept, change or refuse:
- **A TTY means human-with-terminal.** [[norm-color-follows-tty]] already governs ANSI.
- **No TTY means agentic, by default.** A deterministic consumer gets fields only when it asks: `--format json`, `--format udon`, `-0`. The reason: a deterministic consumer is code that someone writes once, and can pass a flag once. An agent arrives cold on every call, and shouldn't need tribal knowledge to be given a readable look. That is [[norm-machine-path-is-detected]]'s own reason, and it applies to agents with more force than to scripts.
- **Harness signals can confirm, not gate.** Claude Code's shells carry `AI_AGENT` (seen as `claude-code_2-1-296_agent`) and `CLAUDECODE=1` (2026-10-10). A tool may use such a signal to confirm the agentic audience, but must never require it, per [[norm-machine-path-is-detected]].
- **The cost:** a script that parsed a tool's default output breaks when the default becomes agentic text. Scripts should ask for their format by name, and the agentic look should keep its shape between releases, since glyphs and placements are interfaces ([[form-agentic-eyes]], concern 5).
- **Open:** whether the agentic look is the human look without ANSI, or adds things a human wouldn't need (more labels, first-contact glosses), or drops things.

## Provenance

The norm and the three audiences are Joseph's words, quoted above, given in an `ai-risk-model` session on 2026-10-10 (Claude Opus 5.5, ai-risk-model-61) while discussing `source-search`'s output. The careful-Unicode section reports the glyph study's own tiers. The detection section is Claude's proposal and is not decided.
