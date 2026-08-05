# Pre- and post-compaction: grounded model

*Synthesis of the Aug 4–5 work on session `019fbe7c-4e62-7f12-8db2-c07abbc428d9` (`~/src/arch`), config, harness source (`~/src-ext/grok-build`), and the part-2 manual-compact experiment. Read with [`01-grok-build-terminology.md`](01-grok-build-terminology.md) and [`03-post-compact-assembly.md`](03-post-compact-assembly.md). Also: [`part2-pre-vs-request.md`](part2-pre-vs-request.md), [`aug-04-analysis.md`](aug-04-analysis.md); raw snapshots in [`.analysis/`](.analysis/).*

---

## 1. What “the agent’s context” is

| Layer | Role |
|--------|------|
| **`chat_history.jsonl`** | Durable **retained conversation** the agent works from. Resume loads this (if present). Right before compact, this **is** what the agent last “understood” as chat state. |
| **`updates.jsonl`** | Durable **event tape** (ACP stream). Append-only, multi-epoch, never rewritten by prune/compact. Richer raw I/O; not the LLM chat shape. |
| **`build_request` / API wire** | Built **each model sample** from in-memory ChatState (clone ± soft-prune / image budget). Not snapshotted per turn. |

**Resume** does not re-run 10 MB of updates into the API. It rehydrates ChatState from `chat_history.jsonl` (rebuild-from-updates only if that cache is empty).

**Caveats that don’t overturn the headline:** hard-clear may already have replaced *old tool bodies* with placeholders in chat_history; the next HTTP call may soft-trim a clone further without rewriting the file.

---

## 2. Size relationship (pre-compact tape vs chat)

On the first compact (auto, ~90% of 500k tokens):

| Artifact | Order of magnitude |
|----------|-------------------|
| Pre-compact `updates.jsonl` | **~10 MB** (events + structured `rawOutput` + progress) |
| Model-facing / chat-shaped content | **~1.3–2 MB** |
| Compaction-request file (part 1) | **~2.0 MB** |
| Immediate post-compact checkpoint | **~68 KB** |

**~10 MB updates ≠ ~10 MB of LLM context.** Chat / request history is the right analogue of what the model carried; the tape is mostly logistics and raw tool structure.

---

## 3. Modes and config (this machine)

```toml
# ~/.grok/config.toml (as of this work)
auto_compact_threshold_percent = 90
compaction_mode = "transcript"   # summary | transcript | segments
# pruning + memory_flush also enabled; verbatim_input defaults true
```

| Mode | After compact the model gets |
|------|------------------------------|
| `summary` | Structured summary only |
| `transcript` | Summary + pointer to **`updates.jsonl`** ← this session |
| `segments` | Summary + pointer to **`compaction/segment_*.md`** |

Part 1 footer (transcript) pointed at `updates.jsonl`. Older long sessions (vivarium/udon) used **segments**.

---

## 4. Pre-compaction: request vs chat_history

### Design (source)

At compact time the harness:

1. Reads live conversation from ChatState (same content mirrored in `chat_history.jsonl`).
2. **Prepares** it (default **verbatim**: keep tool I/O; optional strip reasoning for some backends; drop dangling incomplete tool_use). Lossy/fit only if input overflows.
3. Appends the **summarizer user prompt** (`build_compaction_chat_history`).
4. Attaches the **same tool definitions** as normal turns (KV-cache / prefix stability).
5. Persists that payload as `compaction_requests/<uuid>.json`, then samples.

**System prompt is not swapped** for a “summarizer persona.” Same system as the agent; the **last user message** is the summary job.

### Empirical proof (part 2 — manual `/compact`, 2026-08-05)

| | Pre `chat_history.jsonl` | Request `chat_history` |
|--|--------------------------|-------------------------|
| Messages | 492 | 493 (= 492 + summarizer prompt) |
| Fidelity `pre[i]` vs `hist[i]` | — | **492/492 exact** (content, tool_calls, reasoning, ids) |
| tool_results | 200 | 200 equal by id |
| File size | 1.22 MB | request file **1.31 MB** |

**Message-type counts** (load-bearing for comparing pre-chat → request → post-compact assembly):

| type | pre | request | Δ |
|------|----:|--------:|--:|
| system | 1 | 1 | 0 |
| user | 39 | 40 | **+1** (summarizer) |
| reasoning | 126 | 126 | 0 |
| assistant | 126 | 126 | 0 |
| tool_result | 200 | 200 | 0 |

Request larger **because it adds**, not because it thins:

- 25 tool schemas (~35 KB)
- summarizer prompt (~4 KB)
- result `summary` field + JSON envelope (~rest of +93 KB)

**Conclusion:** On verbatim path, **`compaction_request.chat_history[0:-1]` ≡ pre-compaction chat_history.** Best durable stand-in for “pre-compaction-chat-history” once live chat has been replaced. Segments (if on) are a parallel archive format, not a thinner chat.

**Manual vs auto:** request field `trigger` is authoritative (`auto` / `manual`). Manual usually lands on a clean turn boundary; auto can fire **pre-sampling** inside the agent loop (dangling tool_use / incomplete tool_results; `truncate_trailing_incomplete_tool_call` on prepare). Event stream may still say `auto_compact_completed` for both. See §11 (next work).

---

## 5. During the compact call

| Input | Same as live agent? |
|-------|---------------------|
| System message | **Yes** |
| Prior conversation items | **Yes** (verbatim), + final summary user turn |
| Tools | **Yes** (same effective set) |
| Job specification | **Only** via final user prompt (detailed sections 1–9) |

Steering for summary quality/format is almost entirely **prompt**. Changing system text *can* nudge behavior (system is still present) but the compact user prompt is the primary control surface.

---

## 6. Post-reception: what the harness checks

After the model returns a summary, the harness does **not** enforce section format, user-quote completeness, or max length.

| Check | Behavior |
|--------|----------|
| Empty response | Fail → **retry** (transient) |
| **Degenerate** | After cleaning, seed **&lt; 500 chars** → reject → **retry** |
| Cleaning | Strip/unwrap analysis/summary tags; neutralize echoed control tags; collapse blank lines |
| Packaging | Prepend “This session is being continued…” carrier |
| Transport | Idle timeout, wall-clock budget, context overflow (input ladder), HTTP class |
| Replacement history | Structural sanitize/validate (e.g. tool-call shape), not prose quality |

**Content discipline = prompt-only.** Harness only refuses empty/stub summaries.

---

## 7. Post-compaction state (assembly)

Live `chat_history.jsonl` is **replaced** by a **rebuilt seed**, not a truncated pre-chat. Detail: [`03-post-compact-assembly.md`](03-post-compact-assembly.md).

### Seed order (`build_compacted_history`, grok-build path)

| # | Content | Notes |
|---|---------|--------|
| 0 | System | Same system prompt |
| 1 | User info / layout | `compaction_meta` |
| 2 | Project instructions | if any (`project_instructions`) |
| 3 | **Last real user query** | re-wrapped `<user_query>`; not synthetic |
| 4 | **Summary carrier** | “session continued…” + cleaned summary + mode pointer |
| 5 | System reminder | skills / todos / MCP / tasks… |

Empirically **6 messages** for both part-1 **auto** and part-2 **manual** — **no** assistant / tool_result / reasoning in the seed.

| type | pre (part-2) | request | **post seed** |
|------|-------------:|--------:|--------------:|
| system | 1 | 1 | **1** |
| user | 39 | 40 | **5** |
| reasoning | 126 | 126 | **0** |
| assistant | 126 | 126 | **0** |
| tool_result | 200 | 200 | **0** |

`for_compaction()` **clears `recent_messages`**, so mid-turn tool chains are **not** rehydrated as structured items (only in summary + `updates.jsonl`). Comment in source about “retaining” recent_messages is **wrong relative to current code**.

### Auto mid-loop (“pre-sampling”)

```
loop { check_auto_compact → sample → tools → check_preflight_overflow → … }
```

After `run_compact_only`, the **same agent loop continues** (no new human turn). Seed re-anchors `last_user_query` + summary; next `build_request` uses the thin history.

Tool-use hygiene: **summarizer input** may `truncate_trailing_incomplete_tool_call`; **post seed** has no tool pairs to keep coherent—sanitize is defensive only.

### `/goal`

Does not change the assembler. Continuity across multi-auto-compact = summary + last real user query + goal tracker side-state + later synthetic goal directives (skipped for `last_user_query`). See `03-post-compact-assembly.md` §5.

### Recovery surfaces (unchanged)

| What remains for the past | |
|---------------------------|--|
| **`compaction_requests/…`** | Full (verbatim) pre-chat + prompt + tools + result summary |
| **`compaction_checkpoints/…`** | Seed snapshot (= post live history at replace) |
| **`updates.jsonl`** | Entire tape (keeps growing post-compact) |
| **`compaction/` segments** | Only if `segments` mode |
| **`terminal/*.log`** | Full bash for oversized commands |

---

## 8. End-to-end picture

```
Live turns
  → append updates.jsonl
  → append/replace chat_history.jsonl   ← agent context SOT for resume/API prep
  → each sample: build_request(ChatState) → wire (not stored)

Compact (auto @ threshold or manual /compact)
  → conversation ≈ chat_history
  → prepare (verbatim default)
  → + summarizer user prompt + same tools
  → persist compaction_request
  → sample summary
  → reject if empty / <500 cleaned chars (retry)
  → clean + wrap continuation carrier
  → replace chat_history with thin scaffold
  → checkpoint; mode pointer (updates or segments)
  → continue (updates keeps appending)
```

---

## 9. Conclusions (working claims)

1. **Pre-compact agent understanding** lives in **`chat_history.jsonl` / ChatState**, not in the 10 MB updates tape.
2. **Compaction-request** (verbatim) is a **high-fidelity export** of that chat + tools + summarizer prompt — empirically **exact** on part 2 for all retained messages.
3. Request is **larger** than chat when it adds tools/prompt/envelope, not because it summarizes early.
4. **Post-compact live chat** is thin; recovery of pre-detail is request (chat-shaped), updates/segments/terminal (rawer).
5. **System identity is unchanged** during compact; only the final user turn reassigns the task.
6. **Harness post-checks are minimal** (empty / too-short); almost all summary policy is prompt-side.
7. **`transcript` mode** is working on this session; config options for mode/threshold showed up in real artifacts.
8. **Post-compact seed** is a fixed ~6-item rebuild (not truncated pre-chat); auto and manual match; **no** structured tool tail (`recent_messages` dropped). See [`03-post-compact-assembly.md`](03-post-compact-assembly.md).
9. **Auto multi-step continuity** = turn loop continues after compact + last_user_query re-anchor + summary; `/goal` adds side-state/synthetics, not a different seed.

---

## 10. Artifact index

| Path | What |
|------|------|
| [`01-grok-build-terminology.md`](01-grok-build-terminology.md) | Turns, loops, synthetic user kinds, epochs |
| **`02-pre-and-post-compaction.md`** | This synthesis (sandwiched by 01 + 03) |
| [`03-post-compact-assembly.md`](03-post-compact-assembly.md) | Post seed, pre-sampling, tools, `/goal` |
| [`part2-pre-vs-request.md`](part2-pre-vs-request.md) | Part-2 fidelity report (pre-chat ≡ request) |
| [`aug-04-analysis.md`](aug-04-analysis.md) | Part-1 / modes / updates inventory (longer) |
| [`.analysis/`](.analysis/) | Raw snapshots (pre/post chat, request, checkpoint) |

Session id: `019fbe7c-4e62-7f12-8db2-c07abbc428d9` · cwd: `/Users/josephwecker-v2/src/arch`.
