# Token / glyph comprehension survey

Two layers that are easy to conflate:

1. **Quantitative** (`tokenization.tsv`) — how many BPE pieces, does round-trip equal the character.
2. **Qualitative** (this file) — when the character is *in the prompt*, does a mind see the intended number / density / letter, or a mark it has to look up.

Grok tokenizer is unpublished. o200k_base is a modern-BPE proxy, not Grok. Claude numbers below use the unofficial Xenova tokenizer (counts only). Local ollama answers, when present, are from `survey-tokens`.

## What I (this Grok 4.6 instance) actually see

Honest, not performed. I am looking at the characters as they appear in this document.

| hex | glyph | I see | As a number / density? | Notes |
|---|---|---|---|---|
| U+0031 | 1 | digit one | yes, immediately | baseline |
| U+0661 | ١ | arabic-indic one | yes, as 1 | I read it as the digit, not as a letter |
| U+09E7 | ১ | bengali one | yes, with a beat | I know this set; I would not confuse it with latin |
| U+2160 | Ⅰ | roman numeral one | yes | looks like a serifed I; I use context to not read it as Latin I |
| U+2460 | ① | circled 1 | **yes** | this is a number-presentation I actually use as 1 |
| U+24FF | ⓿ | black circled 0 | yes, 0 | heavier than ①; still a number |
| U+2776 | ❶ | dingbat inverse 1 | yes, 1 | same channel as ① |
| U+3021 | 〡 | vertical stroke | **not immediately 1** | I can recall “Suzhou/Hangzhou 1”; the glyph itself looks like a box-drawing / tally mark |
| U+4E00 | 一 | CJK “one” | yes | I read this as the word/number 一, not as a dash, in CJK context |
| U+96F6 | 零 | CJK “zero” | as the word 零 | not a digit; the number-system role is real but lexical |
| U+05D0 | א | alef | **not as 1** | I see a letter. Gematria=1 is knowledge I can apply, not perception |
| U+0391 | Α | Greek alpha | **not as 1** | letter. Milesian 1 is overlay knowledge |
| U+2153 | ⅓ | one third | yes | fraction glyph, I read the value |
| U+00BD | ½ | one half | yes | |
| U+2074 | ⁴ | superscript 4 | yes, as 4 in exponent position | |
| U+2588 | █ | full cell | density 1 | I use this as “full block” constantly |
| U+2591 | ░ | light shade | ~quarter | matches Ghostty sprite 0.25; I perceive “light fill” |
| U+2581 | ▁ | thin floor | low fill | I see a baseline bar, not “1/8” until I think |
| U+2800 | ⠀ | empty braille | density 0 | easy |
| U+28FF | ⣿ | full 8-dot | density 1 | I see a dense 2×4 grid, **not** a letter |
| U+2813 | ⠓ | 2×4 dots | a braille cell | I do *not* fluently decode this as Latin “h”; I would count dots |
| U+2605 | ★ | black star | fill=black | not a number |
| U+2606 | ☆ | white star | fill=white | pair with ★ is a 2-step fill series |
| U+2B50 | ⭐ | emoji-ish star | similar to ★ | presentation differs by face |
| U+25A0 | ■ | black square | fill | geometric, not a number |
| U+2500 | ─ | light horizontal | box-drawing | join system, not a gradient |
| U+FF11 | １ | fullwidth 1 | yes, 1 | I read it as 1; wider |
| U+218A | ↊ | turned 2 | **not 10** | I see a weird digit; duodecimal-ten is knowledge |
| U+1372 | ፲ | ethiopic ten | **not 10 until I recall** | I would describe “a Ge’ez number-form” |
| U+0BF0 | ௰ | tamil ten | **not 10 until I recall** | same |

**Pattern I trust:** presentation of *Western decimal* (① ❶ １ ⁴ ½) I actually read as numbers. Shade/block I read as density. Braille I read as a **grid**, not as language, and not as a smooth density slider unless I count. Ciphered alphabets (א Α) I read as letters. Suzhou / Ethiopic / Tamil units I can name if asked and would otherwise treat as “a mark.”

That matches the quantitative split: ascii/arabic-indic/hebrew-*letters* are often single-token in o200k (the letters are common); circled/parenthesized/suzhou/ethiopic/braille-nonzero are byte-fallback. Single-token is **not** the same as “read as a number” (alef is single-token and I do not read it as 1). Byte-fallback is **not** the same as “unreadable” (① is often 2 tokens and I still see a circled one).

## ollama llama3.2:3b (10 glyphs, 2026-08-24)

The prompt asked the model not to guess from the code point if the glyph was unreadable, and offered `UNREADABLE / REPLACEMENT` as a first-line option. **Every one of the ten answers used that option**, including ASCII `1` (U+0031). That is not usable as “llama cannot see Unicode.” It is usable as: a 3B local model, given an easy out, did not describe the mark.

Where it leaked knowledge, it was from the code point I had supplied, not from the glyph:

- U+05D0: named “Hebrew letter Aleph” while claiming unreadability
- U+3021: “Japanese kanji character for one” / value 1 / confidence 0 — the Hangzhou numeral *is* a one-stroke, so the value is right for the wrong script
- U+2588 FULL BLOCK → “Right-to-left mark”; U+2591 LIGHT SHADE → “Left-to-right Mark” — those are Unicode *names of other characters*, i.e. confusion, not vision

Full transcripts: `token-survey-ollama.md`. Worth re-running with a prompt that does **not** offer UNREADABLE as a template, and/or with qwen3:4b / gpt-oss / lumin.
