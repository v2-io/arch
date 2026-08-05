# Part-2: pre-compaction `chat_history` vs compaction-request

*Session `019fbe7c-4e62-7f12-8db2-c07abbc428d9`. Manual `/compact` 2026-08-05 ~01:18 local. Trigger on request: `manual`.*

## Inputs

| Artifact | Path (under [`.analysis/`](.analysis/)) | Size |
|----------|-----------------------------------------|-----:|
| Pre chat | `019fbe7c-part2-chat_history-pre-compact.jsonl` | 1 217 821 B (492 lines) |
| Request | `019fbe7c-part2-compaction-request-3bc6897e-e5c3-4974-8c15-261945753a6e.json` | 1 310 600 B |

## Headline

**The request’s `chat_history` is a byte-for-byte faithful copy of the pre-compaction chat (all 492 messages), plus one appended summarizer user message.**  
It is **not** thinner. The ~93 KB larger request **file** is almost entirely **tools + summary result + JSON envelope**, not stripped conversation.

## Size breakdown (why request is larger)

| Component | Bytes | Notes |
|-----------|------:|--------|
| Pre `chat_history.jsonl` | 1 217 821 | baseline |
| Request total | 1 310 600 | **+92 779** vs pre |
| `chat_history` JSON in request | 1 227 703 | 493 messages |
| of which `hist[:-1]` | ~1 223 585 | same 492 msgs; ~5.7 KB JSONL-vs-array formatting delta |
| of which last msg (summarizer prompt) | ~4 116 | `Your task is to produce a faithful…` |
| `tools` (25 tool schemas) | 34 827 | |
| `summary` field (result) | 2 477 | written after success |
| Remainder envelope | ~45 593 | schema_version, ids, model, trigger, attempt_details, JSON braces/commas |

**Content-char sum:** pre 558 187 → request chat 562 212 (**+4 025** = summarizer prompt body only).

Joseph’s hypothesis is correct: **high-fidelity chat + tool definitions + summarizer prompt** (+ small meta/summary).

## Fidelity (pre[i] vs request.chat_history[i] for i = 0..491)

| Check | Result |
|-------|--------|
| Type sequence | **identical** |
| Content text | **492/492 equal** |
| Full fingerprint (content + tool_calls + reasoning encrypted/summary/ids + synthetic_reason) | **492/492 exact** |
| Field-level diffs | **none** |
| tool_result by `tool_call_id` | **200/200 equal** |
| assistant content + tool_calls | **126/126** |
| reasoning (id + summary + encrypted_content) | **126/126** |
| user content (first 39) | **39/39** |
| system | **equal** |

### Type counts

| type | pre | request | Δ |
|------|----:|--------:|--:|
| system | 1 | 1 | 0 |
| user | 39 | 40 | **+1** (summarizer) |
| reasoning | 126 | 126 | 0 |
| assistant | 126 | 126 | 0 |
| tool_result | 200 | 200 | 0 |

### Request-only tail

`hist[492]`: user message, 4025 chars, detailed summarizer prompt (`prompt_variant: detailed`). No `user_context`.

### Request meta

- `trigger`: **manual**
- `model`: grok-4.5
- `attempts`: 1, success
- No lossy strip: tool_results fully present (verbatim path)

## Implications

1. For this manual compact, **pre-compaction chat_history ≈ request.chat_history[0:-1]** with **perfect fidelity**.
2. “Request larger than chat” is expected and healthy: **not** compression of history, **additive** tools + prompt + result fields.
3. Thinner-than-chat only expected under **lossy / fit** overflow ladder — not observed here.
4. Safe to treat the part-2 request as a ground-truth export of retained agent context at compact time (plus summarizer machinery).

## Related

- [`.analysis/`](.analysis/) — post-compact checkpoint, post chat, timestamped pre snapshot
- [`02-pre-and-post-compaction.md`](02-pre-and-post-compaction.md) — synthesis
- [`03-post-compact-assembly.md`](03-post-compact-assembly.md) — post seed / auto mid-loop
- [`aug-04-analysis.md`](aug-04-analysis.md) — part-1 / modes / updates background
