# Grok compaction artifacts — analysis of 2026-08-04 arch session

*Primary subject: auto-compact in session `019fbe7c-4e62-7f12-8db2-c07abbc428d9`
(`~/src/arch`), 2026-08-04 ~11:16 local / 17:16 UTC. Config and on-disk artifacts
inspected 2026-08-04–05. Source cross-check against `~/src-ext/grok-build` (branch
`arch-prep` at time of write; harness also run via `grok-dev` 0.2.117).*

*Purpose: ground what “transcript mode,” the compact request snapshot, and
`updates.jsonl` actually are — sizes, completeness, and how they relate — so later
design work on post-compact recovery / hooks / modes is not arguing from shape alone.*

---

## 1. Config in force

`~/.grok/config.toml` (mtime **2026-08-01 00:42**):

```toml
[session]
auto_compact_threshold_percent = 90

[features]
compaction_mode = "transcript"

[compaction.pruning]
enabled = true
keep_last_n_turns = 5
soft_trim_threshold = 2500
soft_trim_head = 1200
soft_trim_tail = 1200
hard_clear_age_turns = 6

[compaction.memory_flush]
enabled = true
soft_threshold_tokens = 6000
max_flush_write_chars = 16000
```

Related defaults (not necessarily set in this file):

| Knob | Resolution (source) | Default |
|------|---------------------|---------|
| `compaction_mode` | env `GROK_COMPACTION_MODE` > config `features.compaction_mode` > remote > default | `summary` |
| `verbatim_input` | env / config / remote | **`true`** |
| Mode values | `summary` \| `transcript` \| `segments` | — |

**Modes** (`xai-chat-state` `compaction_mode.rs`):

| Mode | After compact the model gets | On-disk extra |
|------|------------------------------|---------------|
| `summary` | Structured summary only | checkpoint only |
| `transcript` | Summary **+** pointer to session `updates.jsonl` | same |
| `segments` | Summary **+** pointer to `compaction/segment_*.md` + `INDEX.md` | segment store |

This compact’s continuation footer matched **transcript**:

> If you need specific details from before compaction … read the full transcript at:  
> `…/019fbe7c-…/updates.jsonl`

---

## 2. Inventory of sessions that compact-fired (this machine)

| When | Project | Session | Mode (from continuation text) | Artifacts |
|------|---------|---------|-------------------------------|-----------|
| **2026-08-04** | `~/src/arch` | `019fbe7c-…` | **transcript** | checkpoint + request; **no** `compaction/` |
| 2026-08-01 | vivarium | `019fb6de`, `019fbc03` | segments | multi-checkpoint + segments |
| 2026-07-29 | vivarium | `019fae5d` | segments | |
| 2026-07-24 | vivarium | `019f91fc` | segments | |
| 2026-07-21 | udon | `019f85e7` | segments | |
| 2026-07-20 | archema-io | `019f814a` | segments | |

Config mtime is Aug 1; the Aug 4 arch compact is the clean post-config **transcript** sample. Earlier long sessions used **segments** (verbose `segment_*.md` ~500KB each).

**memory_flush:** interval / `pre_compact_on_error` notes exist under udon / vivarium / archema-io through ~Jul 24. No memory project for `~/src/arch`; no flush note tied to the Aug 4 compact. Pruning is harder to prove from disk alone (mutates in-memory / `chat_history`, not `updates.jsonl`).

---

## 3. Paths for the Aug 4 arch compact

Session root (URL-encoded cwd):

```
~/.grok/sessions/%2FUsers%2Fjosephwecker-v2%2Fsrc%2Farch/019fbe7c-4e62-7f12-8db2-c07abbc428d9/
```

| Artifact | Path under session |
|----------|--------------------|
| Append-only event log | `updates.jsonl` |
| Compaction **request** snapshot | `compaction_requests/e9a1cfa0-e18d-4ce7-9c6e-4ed15eab5cc3.json` |
| Compaction **checkpoint** (result seed) | `compaction_checkpoints/a64be72b-7685-4d1c-aa8d-cc300d286296.json` |
| Working chat (grows after compact) | `chat_history.jsonl` |
| Oversized bash full text | `terminal/call-*.log` |

Request metadata: `trigger=auto`, `prompt_variant=detailed`, `model=grok-4.5`, `schema_version=2`, one successful attempt.

Compact event fields (from `updates.jsonl`):

| Field | Value |
|-------|------:|
| `tokens_used` / window | 452 049 / 500 000 |
| `percentage` | **90** (matches config threshold) |
| `tokens_before` → `tokens_after` | 452 049 → **17 396** (~3.9%) |
| `prompt_index_at_compaction` | 52 |
| `elapsed_ms` | ~33 061 |

Signals (session end): `compactionCount: 2`, context later ~48% of 500k after continued work.

---

## 4. What the compaction *message* looks like

Synthetic user item (`synthetic_reason: compaction_meta`) in checkpoint / post-compact history:

1. Opener: *“This session is being continued from a previous conversation that ran out of context…”*
2. Structured **Summary** with numbered sections 1–9 (Primary Request, Key Technical Concepts, Files…, Optional Next Step, …) — `prompt_variant=detailed`
3. Mode footer:
   - **transcript:** pointer to `updates.jsonl` (this run)
   - **segments:** pointer to `compaction/segment_*.md` + `INDEX.md`

`compacted_history` is short (~6 messages): system, user_info/meta, project instructions, last user query, summary carrier, skills reminder — not the full prior chat.

---

## 5. Size ladder (this session)

| Layer                                              |        Bytes | Notes                                            |
| -------------------------------------------------- | -----------: | ------------------------------------------------ |
| **`updates.jsonl` full** (end of observed session) | **20.57 MB** | Pre + post compact                               |
| Pre-compact slice (before `auto_compact_started`)  | **10.26 MB** | 1432 lines                                       |
| Post-compact appends                               | **10.31 MB** | Same file keeps growing                          |
| **`compaction_requests/….json`**                   |  **2.00 MB** | Summarizer input (+ result summary field)        |
| Request `chat_history` JSON                        |     ~1.92 MB | 646 messages                                     |
| Request `chat_history` content chars               |     ~1.32 MB |                                                  |
| Request tool_result text alone                     |     ~1.09 MB |                                                  |
| Request `summary` field                            |      ~5.8 KB |                                                  |
| **`compaction_checkpoints/….json`**                |    **68 KB** | Immediate result seed                            |
| Current `chat_history.jsonl` (much later)          |     ~1.17 MB | **Not** the compact result — post-compact growth |

### Ratios

| Comparison | Ratio |
|------------|------:|
| Request file / full end-state updates | **~0.10** (~1/10) |
| Request file / pre-compact updates | **~0.20** (~1/5) |
| Result checkpoint / request file | **~0.034** (~3.4%) |
| Result content / request hist content | **~0.051** (~5%) |
| Tokens after / before | **~0.039** |

**Correction to a natural guess:** the compact *result* is **not** “about half the request.” It is a thin summary-carrier scaffold (~68 KB), order-of-magnitude smaller than the request.

### Where pre-compact updates bytes go (approx.)

| Component | ~Size |
|-----------|------:|
| Tool `rawOutput` JSON (structured + byte arrays) | **~5.2–7.8 MB** (completed tools ~5.2 MB in one pass) |
| Tool UI `content` JSON | ~1.2–1.9 MB |
| Extracted model-facing text (user/asst/thought/tool text from events) | ~1.55 MB |
| Streaming overhead, in_progress updates, recaps, envelopes | remainder of ~10.3 MB |
| Images in *pre*-compact slice | **0** (large base64 images in this session were **post**-compact) |

---

## 6. `updates.jsonl` completeness

### What it is

Append-only **ACP session event stream** (`method: session/update` or `_x.ai/session/update`). Not a clean chat transcript. Continuous across compact boundaries.

### Completeness verdict (this session)

| Question | Answer |
|----------|--------|
| Full lossy summary only? | **No** — raw event stream |
| Survives compact + chat pruning? | **Yes** (by design; never rewritten) |
| Every user/assistant turn present? | **Yes** as chunks across promptIndex 0…80 (compact at 52) |
| Every tool body fully inline as model text? | **No** — see §7 |
| Easy for the model to mine? | **Mediocre** — 10–20 MB JSONL, chunked, grep-oriented |
| Grows after compact? | **Yes** — same path; agent grepping “its” transcript mid-session sees a live log including its own later tools |

### Shell truncation (live, not post-hoc)

Large bash outputs use the familiar banner **while live**:

```text
exit: N [truncated: showing first/last … of … - full output at: …/terminal/….log]
```

That shape is what the model sees (`output_for_prompt` / tool_result). In this pre-compact window: **3** completed bash tools had the banner; **139/142** bash tools with a log matched raw bytes to `terminal/*.log`. For the 3 truncated cases, even `rawOutput` was short of the sibling log file — **fullest text is `terminal/*.log`**.

Hundreds of `"truncated": false` fields in command metadata are **not** content loss.

### Other caveats

- Image payloads can be huge base64 lines in the log; model may also get `image_compressed` downscales.
- Thoughts appear as `agent_thought_chunk` in updates; in chat/request as structured `reasoning` (`summary` + `encrypted_content`).
- Subagent work may live under separate session trees.

---

## 7. Request snapshot vs pre-compact `updates` (same timeframe)

### Source: how the request is built

Not “serialize updates.” Built from **in-memory `ConversationItem` history** (`session/compaction.rs`):

1. `get_conversation()` (retained chat; may already reflect hard-clear pruning)
2. **Prepare for summarizer:**
   - **Verbatim** (default when `verbatim_input=true`): keep tool I/O; optionally strip reasoning for Messages API; drop dangling trailing tool_use
   - **Lossy** (overflow ladder): drop tool results; flatten tool_calls to `[Called tools: …]`; images → `[image]`; strip reasoning
   - Optional `fit_conversation_to_budget` if still too large  
     Ladder: `Verbatim` → `VerbatimFitted` → `Lossy`
3. Append summarizer user prompt (`build_compaction_chat_history`)
4. Persist to `compaction_requests/{uuid}.json` for offline prompt iteration

This run still has **250 tool_results** → stayed on **verbatim** path (not lossy strip-tools).

Pruning comment (`mutations.rs`): hard-clear mutates in-memory conversation and `chat_history.jsonl`; **`updates.jsonl` retains original data for replay**.

### Count compare (pre-compact)

| Kind | Request snapshot | Pre-compact updates (reconstructed) |
|------|-----------------:|------------------------------------:|
| system | 1 | 0 (not an ACP chat chunk) |
| user | 59 | 52 text chunks |
| assistant | 165 | 75 coalesced message streams |
| tool_result / completed tools | **250** | **250** |
| reasoning / thought | 169 items | 168 thought streams |
| tool_call starts | 2 `backend_tool_call` | **252** `tool_call` |
| Extra in updates only | — | 813 `tool_call_update` (312 `in_progress`), 51 `turn_completed`, 17 `session_recap`, plan/task |
| Extra in request only | summarizer prompt (~4k), `tools[]` (25 schemas), result `summary` | — |

**Assistant 165 vs 75:** request has **90 empty-content assistants** that only carry `tool_calls`; updates emits separate `tool_call` events. Non-empty assistant **text is identical: 143 506 chars both sides**.

**Thoughts:** request `reasoning.summary` text ≈ **30.5k** chars; updates thought chunks ≈ **31.2k**. Same thinking, different packaging.

**Users:** request includes injected rows (`project_instructions`, `system_reminder`, date-change) and the final **summarizer prompt**; not all of those are plain `user_message_chunk`s.

### Tool bodies: the important semantic gap

Same **250** completed tools by id (~248 shared). “Complete” differs by layer:

#### Per-tool average sizes (pre-compact)

| Tool type | n | Request render (model-facing) | `rawOutput` JSON | UI `content` |
|-----------|--:|--------------------------------:|-----------------:|-------------:|
| Bash | 104 | ~4.1k | ~24k | ~4.2k |
| ReadFile | 61 | ~7.4k | ~25k | ~8.9k |
| GrepSearch | 25 | **~6.6k** | ~38k | **~78** (“found N matches”) |
| ListDir | 8 | **~3.9k** (tree text) | ~4.1k | **null** |
| SearchReplace | 46 | ~100 | ~5.7k | ~5.4k |

#### Pattern

| Layer | Role |
|--------|------|
| **Request `tool_result.content`** | **Model-facing render** — full listing, full grep dump, bash `output_for_prompt` |
| **Updates UI `content`** | Often short status for structured tools (`found 28 matches`, or null for ListDir) |
| **Updates `rawOutput`** | Typed structured payload — Grep `stdout` bytes + `file_matches`; ListDir `Content` dict; Bash `output` + `output_for_prompt` + `output_file` |

Examples:

- **ListDir:** UI `content = null`; structure in `rawOutput.Content`; request has rendered tree text.
- **GrepSearch:** UI `found 28 matches`; request full `<workspace_result>…` (~13k); raw has full stdout + matches.
- **Bash:** request usually equals `output_for_prompt` / UI text; large cmds already truncated live with `terminal/*.log` pointer.

Id-level: many “mismatches” are **request full render vs short UI text** — data still in structured `rawOutput`, not absent from the log.

### Mental model

```
                    live session
                         │
         ┌───────────────┴───────────────┐
         ▼                               ▼
  ChatState conversation            updates.jsonl
  (ConversationItem[],              (ACP event stream,
   model-facing strings,             structured rawOutput,
   pruning may hard-clear            never pruned;
   old tool results in RAM)          multi-epoch append)
         │
         │  prepare_…_verbatim (default)
         │  + summarizer prompt
         ▼
  compaction_requests/<uuid>.json
  = snapshot of summarizer INPUT
    (≈ agent context shape, not the log)
         │
         │  summary model
         ▼
  compaction_checkpoints/<uuid>.json
  + replace live chat with summary carrier
  + mode pointer (updates.jsonl | segments/)
```

| Question | Answer for this compact |
|----------|-------------------------|
| Same timeframe? | Yes — request = conversation **at** compact; updates slice = events **until** `auto_compact_started` |
| Is request “updates ÷ 5”? | Roughly in **bytes**, not by deleting every 5th fact — different **representation** |
| Better for “what the agent saw”? | **Request** (rendered tools, assembled turns, system + injections + summarizer prompt) |
| Better for “what happened / full I/O”? | **`updates.jsonl`** (+ `terminal/*.log` for oversized bash) |
| Same tool count? | Yes, 250 completed |
| Same tool *text*? | Bash/read often yes; **grep/listdir often no** in UI fields — full text in request render and/or structured raw |

---

## 8. Compaction modes vs this evidence

| Config intent | Observed Aug 4 | Older long sessions |
|---------------|----------------|---------------------|
| `compaction_mode = "transcript"` | Footer points at `updates.jsonl`; no `compaction/` dir | — |
| (earlier / other mode) | — | `segments` with `INDEX.md` + verbose `segment_NNN.md` |
| `auto_compact_threshold_percent = 90` | Event: `percentage: 90`, `reason: Context window 90% full` | — |
| `verbatim_input` default true | Request kept full tool_results | — |
| `memory_flush` enabled | No arch memory project / no Aug 4 flush note | Flush notes through ~Jul 24 on other projects |
| `pruning` enabled | Not fully proven from disk on this sample; design: does not touch updates | — |

---

## 9. Practical guidance (for agents / design)

1. **Do not treat `updates.jsonl` as a tidy novel.** Prefer `grep` / targeted reads; expect 10–20+ MB on long sessions.
2. **For “what did I see at compact time?”** open the **request** snapshot, not updates alone.
3. **For “recover exact tool I/O”** use updates `rawOutput` and/or `terminal/*.log`; for grep/listdir do not trust short UI `content`.
4. **Post-compact greps of “my transcript”** include **live appends** after the compact markers (`auto_compact_started` / `compaction_checkpoint` / `auto_compact_completed`).
5. **Segments mode** trades raw fidelity for legible markdown segments; transcript mode trades legibility for a single growing log pointer.
6. Immediate **checkpoint** (~tens of KB) is the true compact *result*; later `chat_history.jsonl` size is dominated by post-compact work.

---

## 10. Open threads (for deeper drill)

- Reconstruct exact request message order from updates (synthetic user injection path).
- Whether hard-clear ever left empty tool_results in a request while updates still held raw (not dominant on this sample).
- `memory_flush` gating for workspaces without a memory project / first flush after mode change.
- Soft-trim: only on API clone vs retained conversation — when does request see soft-trimmed bodies?
- Second compact in same session (`compactionCount: 2`) — where is the other boundary?
- Segments vs transcript quality under identical long work (vivarium multi-segment sessions as contrast corpus).
- Feasibility of post-compact auto-continue (hooks PreCompact/PostCompact are observe-oriented in current research notes).

---

## 11. Provenance

| Item | Source |
|------|--------|
| Config | `~/.grok/config.toml` |
| Session artifacts | `~/.grok/sessions/…/019fbe7c-…/` |
| Mode / prepare / fit | `~/src-ext/grok-build/crates/codegen/xai-chat-state/src/compaction_mode.rs`, `compaction_utils.rs` |
| Compact orchestration / request persist | `…/xai-grok-shell/src/session/compaction.rs`, `helpers/session_compact.rs` |
| Pruning vs updates | `…/xai-chat-state/src/actor/mutations.rs` |
| Binaries | `grok-dev` → source `xai-grok-pager` 0.2.117; official `grok` 0.2.118 |

*Analysis session: Grok Build on `~/src/arch`, 2026-08-04–05. Filename: `aug-04-analysis.md`.*
