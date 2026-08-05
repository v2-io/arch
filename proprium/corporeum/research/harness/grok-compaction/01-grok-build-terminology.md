# Grok-build terminology

*Working glossary for how grok-build abstracts modern frontier agent models: multi-step tool use, reasoning blocks, synthetic “user” injections, compaction, and autonomous goal runs. Grounded in `xai-grok-sampling-types` / chat-state / shell, and the Aug 2026 compaction research in this folder.*

Related: [`02-pre-and-post-compaction.md`](02-pre-and-post-compaction.md), [`03-post-compact-assembly.md`](03-post-compact-assembly.md).

---

## 1. Why this vocabulary exists

On the **wire**, many APIs look like a flat chat:

- roles: system / user / assistant / tool  
- tool results often sit in a user-adjacent or `tool` role  
- reasoning may be separate parts or encrypted blobs  
- one “conversation” mixes human text, tool I/O, and harness chrome  

Grok-build’s **internal model** separates:

1. **What the human (or harness) intended as a unit of work** — a *real user turn*  
2. **What the model does inside that unit** — *agent-loop iterations* (sample → tools → …)  
3. **What is injected to steer without being “the user speaking”** — *synthetic user* items  
4. **What is durable event tape vs retained LLM context** — *updates* vs *chat_history*  
5. **What survives a context reset** — *compaction epoch* / seed  

Without those distinctions, “turn,” “session,” and “user message” all collide.

---

## 2. Core hierarchy (big → small)

```
Session (on-disk id under ~/.grok/sessions/…)
  └─ Real user turn(s)          ← prompt_index; human (or wake) work unit
        └─ Agent-loop iteration(s)   ← one build_request → model sample → optional tools
              └─ (optional) parallel tool_calls in one sample
        └─ may cross Compaction epoch(s)   ← chat_history replaced; turn may continue
```

| Term | Meaning |
|------|---------|
| **Session** | Durable identity + files (`chat_history.jsonl`, `updates.jsonl`, checkpoints, …). Resume reloads session. |
| **Real user turn** | One unit of work keyed by a **real** user message (`synthetic_reason == None`, typically `<user_query>`). Owns a `prompt_index` slot when it starts a turn. |
| **Agent-loop iteration** | One cycle: prepare API payload → sample model → if tools, run them → maybe loop again. Logged as `loop_index` in the turn pipeline. |
| **Model sample** | One LLM invocation (one `ConversationRequest` / wire call). |
| **Compaction epoch** | Stretch of work under one generation of retained `chat_history`, between compact replaces. |
| **Completed turn** | Turn pipeline finished for that real prompt (`end_turn` / turn outcome). Not the same as “compact happened.” |

**Important:** One **real user turn** can span **many** loop iterations and **several** compaction epochs (especially under `/goal`). Between compacts you can have long agent activity with **zero additional full human turns**.

---

## 3. Conversation items (internal types)

Approximate mapping of `ConversationItem` (names as used in chat_history / requests):

| Type | Role in the abstraction |
|------|-------------------------|
| **system** | Identity + standing instructions (same system often kept through compact). |
| **user** | Either a **real** human/wake turn **or** a **synthetic** injection (see §4). |
| **assistant** | Model text + optional **tool_calls** (parallel batch in one sample). |
| **tool_result** | Tool output bound by `tool_call_id`. **Not** a user turn. |
| **reasoning** | Thinking block: often **encrypted_content** + optional **summary** text. Not user-visible turn structure. |
| **backend_tool_call** | Provider-hosted tool traces (e.g. web_search) when present. |

On the wire these may collapse into fewer roles; internally they stay distinct so compaction, prompt counting, and replay stay coherent.

---

## 4. Kinds of “user” messages (`SyntheticReason`)

Serialized on user items as `synthetic_reason` (snake_case). **`None`** = real user content (or treated as such for turn boundaries).

### Mid-turn / non-turn-starting (do **not** consume a new prompt-turn slot via `starts_prompt_turn`)

| Reason | Purpose |
|--------|---------|
| **`compaction_meta`** | Compaction pipeline chrome: user_info re-prefix, summary carrier (“session continued…”), related meta. |
| **`system_reminder`** | Runtime `<system-reminder>` (skills, todos, MCP, …). |
| **`project_instructions`** | AGENTS.md / CLAUDE.md-style project block; stable for KV-cache prefix. |
| **`auto_continue`** | Post-compact “keep working” injection (when used). |
| **`auto_recovery`** | Retry guidance after transient tool failure. |
| **`interjection`** | Human Ctrl+Enter mid-turn steering without cancelling the turn. |
| **`goal_summary`** | Goal orchestrator progress text (also used in ways that may or may not be a full turn — see note below). |
| **`stop_hook_feedback`** | Stop hook blocked exit; stay in same turn. |
| **`working_directory_switch`** | Session relocation context. |
| **`unknown`** | Forward-compat catch-all. |

### Wake / can start a prompt turn (`starts_prompt_turn == true`)

| Reason | Purpose |
|--------|---------|
| **`task_completed`** | Background bash finished → wake agent. |
| **`subagent_completed`** | Subagent finished → wake. |
| **`notification_drain`** | Idle drain of monitors / completions → wake. |
| **`goal_classifier_nudge`** | Goal not achieved yet; keep working. |
| **`scheduler_fired`** | `/loop` or scheduled task fire. |

**Note on `goal_summary`:** tagged `starts_prompt_turn = false` deliberately because the same reason is used for both legacy goal-continuation *turns* and in-turn directives; counting it as a turn start would break truncation/replay for the common in-turn case.

### Real user

- **`synthetic_reason` absent / null**  
- Typically body wrapped in `<user_query>…</user_query>`  
- Drives **`last_user_query`**, compact re-anchor, and (when starting a turn) **`prompt_index`**

**Compact re-anchor:** post-compact seed re-injects the last **real** user query (skips synthetics), then the summary carrier (`compaction_meta`).

---

## 5. Agent loop (inside one real turn)

```
Real user message
  └─ loop {
        pre-sampling auto-compact?     // estimate ≥ threshold
        build_request → model sample   // one LLM call
           · reasoning (encrypted / summary)
           · assistant text (may show in UI; does not end turn by itself)
           · tool_calls[] (0..N, often parallel)
        if tool_calls:
           run tools → tool_result items
           preflight compact?          // estimate > full window
           continue loop
        else:
           end turn (await next real user / wake)
     }
```

| Pattern | Meaning |
|---------|---------|
| **Parallel tools** | Multiple `tool_calls` on one assistant message → one batch. |
| **Sequential tools** | Later loop iterations after seeing prior `tool_result`s. |
| **Both** | Normal: parallel batch, then another sample, then another batch. |
| **Mid-turn text** | Assistant prose while tools still pending or between batches — not a new user turn. |

---

## 6. Compaction-related terms

| Term | Meaning |
|------|---------|
| **Pre-compaction chat_history** | Retained conversation immediately before replace (= agent context SOT). |
| **Compaction request** | Snapshot sent to summarizer: (verbatim) pre-chat + same tools + summarizer user prompt (+ result summary after). |
| **Summarizer prompt** | Final **user** message on the compact API call only (“produce a faithful summary…”). Not a real user turn of the session. |
| **Post-compact seed** | Rebuilt ~6-item history: system, user_info, project, last real query, summary carrier, system_reminder. No tool/assistant tail today. |
| **Transcript mode** | After compact, carrier points at `updates.jsonl` for recovery. |
| **Segments mode** | Carrier points at `compaction/segment_*.md`. |
| **Pre-sampling compact** | Auto-compact at start of loop iteration before next sample. |
| **Preflight overflow compact** | Auto-compact after tools if estimate exceeds full context window. |
| **Compaction epoch** | Work under one chat_history generation between replaces. |

Compact **ends an epoch**, not necessarily a **real turn**. Auto path often **continues the same turn’s loop** on the new seed.

---

## 7. Durability layers

| Store | Abstraction |
|-------|-------------|
| **`chat_history.jsonl`** | Retained **LLM-facing** conversation (resume + `build_request` substrate). |
| **`updates.jsonl`** | Append-only **event tape** (UI stream, tool raw I/O, compact markers). Multi-epoch. |
| **`build_request` / wire** | Per-sample view of chat_history (± prune); not generally snapshotted. |
| **Checkpoint / request artifacts** | Compact-time exports for recovery and offline analysis. |

---

## 8. Mapping from “frontier model” talk → grok-build

| Common language | Grok-build |
|-----------------|------------|
| Chat / thread | Session + retained `chat_history` |
| User message | Real user **or** synthetic user (must check `synthetic_reason`) |
| Assistant message | `assistant` (+ optional parallel tools) |
| Tool call / tool message | `tool_calls` on assistant + `tool_result` |
| Chain-of-thought / thinking | `reasoning` (encrypted + summary) |
| “The model called tools five times” | Five **samples** or five **batches** inside **one real turn** (count carefully) |
| Context full / summarize | Compaction (epoch boundary; turn may continue) |
| Autonomous agent overnight | Real turn(s) + many loops + many epochs + `/goal` synthetics |

---

## 9. Counting rules of thumb

1. **Don’t count synthetic user items as human turns** for “how many times the user spoke.”  
2. **Don’t count tool_results as user turns.**  
3. **One real turn ≠ one model call** — loop_index can be large.  
4. **One real turn ≠ one compact epoch** — compact can fire mid-turn.  
5. **Post-compact “user” messages** in the seed are mostly meta/project/summary; only the re-anchored query is “real” content-wise, and it may still be the *same* turn’s query.  
6. **`starts_prompt_turn`** answers “does this synthetic wake consume a prompt_index-style turn?” — not “is this human?”

---

## 10. Source anchors

| Concept | Rough location |
|---------|----------------|
| `SyntheticReason`, user constructors | `xai-grok-sampling-types` `conversation.rs` |
| `ConversationItem` | same |
| Agent loop / pre-sampling / preflight | `xai-grok-shell` `session/acp_session_impl/turn.rs` |
| Compact seed | `xai-chat-state` `compaction_utils::build_compacted_history` |
| Real-user boundary for compact | `extract_last_real_user_query`, `extract_messages_since_last_real_user` |
| Research notes | this directory |

---

*Living doc: extend as more harness seams (subagents, workflows, permissions) get named in research.*
