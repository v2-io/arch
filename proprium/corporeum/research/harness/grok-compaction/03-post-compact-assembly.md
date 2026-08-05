# Post-compact chat assembly (auto vs manual, tools, `/goal`)

*Harness source: `~/src-ext/grok-build` (xai-grok-shell + xai-chat-state). Empirical: session `019fbe7c-…` part-1 auto checkpoint + part-2 manual post-chat. Complements [`02-pre-and-post-compaction.md`](02-pre-and-post-compaction.md); terms in [`01-grok-build-terminology.md`](01-grok-build-terminology.md).*

---

## 1. What “pre-sampling” means

Auto-compact does **not** only run between human turns. Inside the agentic turn loop (`turn.rs`):

```
loop {
  1. check_auto_compact_needed()     // estimated tokens ≥ threshold
     → run_compact_only(Auto) if needed
  2. build_request() → sample model
  3. if tool_calls: run tools
  4. check_preflight_overflow()      // estimated tokens > full context window
     → run_compact_only(Auto) if needed
     → continue loop
}
```

| Gate | When | Metric |
|------|------|--------|
| **Pre-sampling** | Start of each loop iteration, **before** `build_request` | `get_estimated_total_tokens()` vs `threshold_percent` (e.g. 90%) |
| **Preflight overflow** | **After** tool outputs, before next sample | estimated total **>** full context window |

Both call `run_compact_only` → `run_compact_inner(..., trigger=Auto)`. Then the **same loop continues** (no user message required): next sample uses the **new** chat history.

“Estimated” = exact tokens from last API response + byte-estimate of items added since (tool results). So tools alone can push past the gate mid-turn.

Manual `/compact` is outside this loop (user command); request `trigger: manual`. Same assembly function afterward.

---

## 2. Post-compact seed shape (what replaces `chat_history`)

Built by `build_compacted_history` (`compaction_utils.rs`), then sanitize/validate, then `replace_conversation_for_compaction`.

### Fixed order (grok-build path: `summary_before_recent = false`)

| # | Item | Source | `synthetic_reason` |
|---|------|--------|--------------------|
| 0 | System | Original system message | — |
| 1 | User info / layout prefix | `build_user_message_prefix()` | `compaction_meta` |
| 2 | Project instructions | AGENTS/CLAUDE.md reminder (if any) | `project_instructions` |
| 3 | **Last real user query** | `extract_last_real_user_query` → wrapped `<user_query>…` | **none** (counts as real) |
| 4 | **Summary carrier** | cleaned summary + “session continued…” + mode **transcript_hint** | `compaction_meta` |
| 5 | System reminder | skills / todos / MCP / running tasks, etc. | `system_reminder` |

### Empirical (both part-1 auto and part-2 manual)

**Exactly 6 messages**, same type pattern:

```
system | user(meta) | user(project) | user(real query) | user(summary+hint) | user(skills reminder)
```

| | Part-1 auto (checkpoint) | Part-2 manual (post chat) |
|--|--------------------------|---------------------------|
| n | 6 | 6 |
| types | system 1 + user 5 | same |
| assistant / tool_result / reasoning | **0** | **0** |
| last real query | heatmap cells request | “Excellent work. Thank you!” |
| summary + transcript pointer | yes (~6k carrier) | yes (~2.9k carrier) |

So **manual vs auto does not change the seed skeleton** on this codebase. Difference is **when** compact runs and what the summarizer saw as input—not the post shape.

### What is deliberately **not** in the seed

`CompactionStateContext::for_compaction()` sets **`recent_messages = []`** before assembly.

`recent_messages` would otherwise be: assistant + tool_result items since the last **real** user turn (tool bodies already replaced with `"Tool call omitted..."`). That machinery exists to:

- keep tool_use / tool_result pairing across synthetic user injections  
- re-cue mid-turn work after compact  

**But the live path passes `state_context.for_compaction()`**, which **drops** `recent_messages`. Comment text about “grok-build retains recent_messages” is **out of date vs the code**: the function clears them; both empirical seeds have **no** assistant/tool tail.

**Implication:** In-flight tool work after the last real user message survives compact **only inside the prose summary** (and `updates.jsonl`), not as structured chat items. Post-compact the model restarts from: system + project + **original user query** + **summary** + reminders.

---

## 3. Tool-use coherence (summarizer input vs post seed)

### On the way *into* the summarizer

| Mechanism | Role |
|-----------|------|
| **Verbatim prepare** | Keep tool I/O; optional strip reasoning |
| **`truncate_trailing_incomplete_tool_call`** | Pop trailing `Assistant` that still has `tool_calls` without matching `ToolResult` (provider rejects dangling tool_use) |
| **Lossy / fit** | Only if input overflows (not part-1/part-2 success path) |

Relevant when auto fires **pre-sampling** after a model step that emitted tool_calls but tools haven’t completed—or a partial assistant. After a normal tool-complete iteration, pairs are closed; truncate is a no-op.

### On the way *out* into the new history

| Mechanism | Role |
|-----------|------|
| **`sanitize_compacted_history`** | Strip `ToolResult`s with no preceding assistant tool_call id |
| **`validate_compacted_history`** | Same invariant; if still broken → **fallback seed with no recent_messages** (already none) |

With recent_messages always emptied, sanitize is mostly defensive.

**There is no re-attachment of the pre-compact tool chain to the new seed.** Coherence for the next sample is: valid thin history (no dangling tools) + summary narrative.

---

## 4. After auto-compact: how the agent keeps going

`run_compact_only` (auto path):

1. Emits `AutoCompactStarted` (even when percentage is from preflight overflow wording “Context window N% full”).
2. `run_compact_inner(trigger=Auto, auto_continue=None)` — **no** auto-continue prompt injected into the seed on this path.
3. Replaces conversation with the 6-item seed.
4. Returns to the **turn loop**, which **continues** → next `build_request` / sample.

So multi-step autonomy across auto-compact is: **same user turn still open**, last real query re-anchored, summary as memory, **loop resumes**. No separate “auto_continue” user message is required for that (checkpoint `auto_continue` is optional and not set by `run_compact_only`).

Manual compact: user usually types next; seed still re-anchors `last_user_query` (whatever the last real human message was).

---

## 5. `/goal` and multi-compact

Goal mode does **not** replace the compact assembler. It layers **synthetic** directives and turn-end continuations.

### Relevant pieces

| Piece | Behavior |
|-------|----------|
| **Goal continuation directives** | Injected as synthetic user items (e.g. goal-related reasons; not always `starts_prompt_turn`) |
| **`is_real_user_turn` / `last_user_query`** | Skips synthetic reasons (AutoContinue, SystemReminder, GoalSummary, …). Compact re-anchors the **last human (or real) query**, not the latest goal nudge |
| **`recent_messages` boundary** | Uses last **real** user turn so goal synthetics don’t split tool pairs—but those messages are still **dropped** by `for_compaction()` today |
| **`maybe_queue_goal_continuation`** | After turn success/failure; can re-inject goal “keep going” after compact once the turn ends or mid-run depending on orchestrator path |
| **Token budget** | Goal tracks tokens; can pause/enforce budget independently of compact threshold |
| **Plan mode** | `reset_after_compaction()` on plan_mode after compact |

### Multi-auto-compact under goal (logical sequence)

```
User (or goal start) real query
  → agent loop: tools → tokens climb
  → pre-sampling or preflight auto-compact
  → seed: system + … + same last_user_query + summary + reminders
  → loop continues (still one human turn)
  → more tools → compact again …
  → turn ends → goal continuation may inject synthetic directive
  → new turn / same loop path depending on product wiring
```

Goal does **not** change the 6-item seed formula. It **does** mean:

- Summaries must carry enough goal state (todos, plan path, next step)—reminders may re-list todos/MCP/tasks.
- Continuity across compacts is **summary + last_user_query + goal tracker side state**, not a growing tool transcript in chat_history.
- Synthetic goal messages must not be mistaken for `last_user_query` (they aren’t, by design).

---

## 6. Type-count bridge (pre → request → post)

Part-2 manual compact (from [`part2-pre-vs-request.md`](part2-pre-vs-request.md)):

| type | pre chat | request | post seed |
|------|---------:|--------:|----------:|
| system | 1 | 1 | **1** |
| user | 39 | 40 (+ summarizer) | **5** (meta, project, last query, summary, skills) |
| reasoning | 126 | 126 | **0** |
| assistant | 126 | 126 | **0** |
| tool_result | 200 | 200 | **0** |
| **total** | **492** | **493** | **6** |

Pre ≡ request body; post is a **new construction**, not a subset of pre.

---

## 7. Conclusions

1. **Post-compact chat is a rebuilt seed**, not a truncated pre-chat. Canonical builder: `build_compacted_history`.
2. **Same skeleton for auto and manual** (empirically 6 items; no assistant/tool tail).
3. **`for_compaction()` drops `recent_messages`** → mid-turn tool chains are **not** rehydrated as structured items; rely on summary (+ updates/transcript mode).
4. **Pre-sampling / preflight** = auto-compact **inside** the agent loop; after replace, **loop continues** without a new human prompt.
5. **Tool coherence for the summarizer** = truncate incomplete trailing tool_use on input; for the **successor** = thin valid history + summary, not preserved tool pairs.
6. **`/goal`** rides the same compact path; multi-compact autonomy is loop continuation + goal side-state + synthetic directives, not a different chat assembler.

---

## 8. Open / caveats

- Whether any code path still passes **full** `state_context` (with recent_messages) — current `run_compact_inner` does not.
- Auto-continue prompt string exists in tests/helpers; `run_compact_only` passes `auto_continue: None` into the inner assemble/checkpoint path.
- Goal + compact interaction is source-reasoned; no dedicated multi-compact goal session artifact analyzed here yet.
- Comment/code drift on “retains recent_messages” is a product/truth hazard for future readers.

---

## 9. Source map

| Topic | Location |
|-------|----------|
| Seed builder | `xai-chat-state` `compaction_utils::build_compacted_history` |
| `for_compaction` / recent_messages | same file, `CompactionStateContext` |
| Truncate dangling tools | `prepare_conversation_for_verbatim_summarization` / `truncate_trailing_incomplete_tool_call` |
| Apply path | `xai-grok-shell` `session/compaction.rs` `run_compact_inner` |
| Pre-sampling / preflight | `check_auto_compact_needed`, `check_preflight_overflow`; callers in `turn.rs` |
| Auto path | `run_compact_only` |
| Goal continuation | `acp_session_impl/goal.rs` `prepare_goal_continuation`, `maybe_queue_goal_continuation` |
