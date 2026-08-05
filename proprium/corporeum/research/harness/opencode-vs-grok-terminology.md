# OpenCode vs Grok-build: how each harness abstracts the coding-agent problem

> **Provisional first pass.** Written to get the ball rolling (2026-08-05). Almost certainly needs independent verification, sharper edges, and more flesh — treat as working substrate, not settled law. Pair with the equally provisional [`grok-compaction/01-grok-build-terminology.md`](grok-compaction/01-grok-build-terminology.md).

*Audience: Joseph and later agents who already know the grok-compaction notes. This is a compare/contrast of **mental models** — what each system treats as the load-bearing objects, seams, and continuations — not a feature checklist or a turn-vocabulary drill.*

**Grok pole (already grounded; not re-derived here):**

- [`grok-compaction/01-grok-build-terminology.md`](grok-compaction/01-grok-build-terminology.md)
- [`grok-compaction/02-pre-and-post-compaction.md`](grok-compaction/02-pre-and-post-compaction.md)
- [`grok-compaction/03-post-compact-assembly.md`](grok-compaction/03-post-compact-assembly.md)

**OpenCode pole (source of truth for this note):**

- `/Users/josephwecker-v2/src-ext/opencode/` — especially `CONTEXT.md`, `specs/v2/session.md`, `packages/core/src/session/*`, `packages/opencode/src/session/*`, `packages/schema/src/session-message.ts`, `packages/llm/DESIGN.md`

**Epistemic markers:**

- **Verified** = read in OpenCode source/specs or already established in the grok notes.
- **Inference** = structural reading across files without a live OpenCode session artifact parallel to the grok compact experiments.
- **Product-live vs design-target:** OpenCode is mid V1→V2 migration. The **live interactive path** still centers on `packages/opencode` (`SessionPrompt.loop`, V1 messages/parts). The **domain language and destination model** live in `CONTEXT.md` + `packages/core` (`SessionRunner`, Context Epoch, event-projected history). Both are real; conflating them would mis-state either product or architecture.

---

## 0. One-sentence contrast

| | |
|--|--|
| **Grok-build** | A **conversation-state machine** for one frontier agent: retain an LLM-facing `chat_history`, append a raw event tape, and when the window fills **replace** the retained history with a thin seed (last real query + summary + chrome) so the **same turn loop can continue**. |
| **OpenCode** | A **session runtime product** for multi-provider coding agents: admit durable user inputs, project chronological Session History, assemble privileged System Context as a separate algebra, run explicit **provider turns** under process-local drains, and when the window fills **cut active model history at a compaction checkpoint** while the full transcript remains durable. |

They both solve “long multi-tool coding sessions under finite context.” They cut the problem at different joints.

---

## 1. Product stance: what kind of machine is this?

### Grok-build (from grounded notes)

- Internal/shell harness around xAI’s sampling stack (`xai-grok-shell` + `xai-chat-state` + sampling types).
- Abstraction gravity sits on **conversation items**, **real vs synthetic user**, **agent-loop iterations**, and **compaction epochs**.
- Autonomy surface that the notes emphasize: mid-turn auto-compact, `/goal` continuations, wake synthetics (task/subagent/scheduler), interjections.
- Multi-provider generality is not the center of the abstraction; the center is **faithful agent continuation under Grok-shaped messages (reasoning, tools, encrypted thinking)**.

### OpenCode (verified in README + package layout + CONTEXT)

- Public open-source **AI coding agent** with CLI/TUI, desktop, server, console, plugins, MCP, multi-provider catalog.
- Abstraction gravity sits on **Session as durable aggregate**, **Location placement**, **System Context**, **Session History projection**, **permissioned tools**, and **client/server contracts** (HttpApi → SDK).
- Effect-native core; SQLite event/projection storage; explicit separation of portable LLM protocol (`@opencode-ai/llm`) from session orchestration.
- Multi-agent is product-visible (`build` / `plan` primary; `general` / `explore` subagents; dedicated `compaction` agent).

**Implication:** Grok’s model optimizes for *one coherent agent mind continuing through compaction*. OpenCode’s model optimizes for *a durable, multi-surface session product that can host many agents, providers, and UIs without lying about history*.

---

## 2. Hierarchy of work units

### Grok-build hierarchy (from 01)

```
Session
  └─ Real user turn(s)          ← prompt_index; synthetic_reason == None
        └─ Agent-loop iteration(s)   ← build_request → sample → optional tools
              └─ parallel tool_calls in one sample
        └─ may cross Compaction epoch(s)   ← chat_history replaced; turn continues
```

Load-bearing distinctions:

1. **Real user turn** ≠ model sample ≠ compact epoch  
2. Synthetic user items are not “the user speaking”  
3. Auto-compact can fire **inside** the loop (pre-sampling / preflight) and **resume the same turn**

### OpenCode hierarchy (verified V2 language)

```
Session  (durable ID; location; agent/model selection; parent_id for subagents)
  └─ Admitted Prompt (inbox; not yet model-visible)
        └─ Prompt Promotion → user message in Session History
  └─ Session Drain (process-local only; no durable identity)
        └─ Provider Turn(s)   ← one llm.stream(request) each
              └─ local tool settlement(s) → maybe another Provider Turn
        └─ may compact before a Provider Turn (or on overflow recovery)
  └─ Context Epoch (immutable Baseline System Context + snapshot)
```

V1 live loop (`SessionPrompt.run` / `while (true)` in `packages/opencode/src/session/prompt.ts`) is closer to a classical agent loop:

- busy status → load compacted-filtered messages → detect tool unfinished / overflow → compaction task or sample → tools → continue  
- step counter + optional `agent.steps` max  
- last **user** message anchors agent/model for the next assistant message  

**Rhymes:** both have “user work unit → many model samples with tools → optional compact mid-work.”  
**Cut differently:**

| Concern | Grok | OpenCode |
|---------|------|----------|
| “Is execution a durable thing?” | Implicit in turn pipeline / goal state | **Explicitly no** for drains; durable objects are prompts, events, projected messages |
| “What is one model call?” | Model sample / loop iteration | **Provider Turn** (named, and LLM package forbids embedding tool loops inside it) |
| “What continues after tools?” | Same agent loop iteration chain | Reload projected history → **new** Provider Turn (V2); V1 continues outer `while` |
| “What is mid-run user input?” | Interjection synthetic / same turn | **Steer** (promote at next safe boundary) vs **queue** (promote when drain would idle) |

The OpenCode V2 docs are almost polemical about not inventing a durable “run” entity: recovery reasons from prompts, projected history, provider attempts, and tool state — not from an enclosing execution id. Grok’s notes don’t frame that refusal; the turn + goal tracker *act* as continuation state without OpenCode’s vocabulary of admission/promotion.

---

## 3. Conversation ontology

### Grok conversation items

Internal types (approx.): `system`, `user` (real **or** synthetic via `SyntheticReason`), `assistant` (+ `tool_calls`), `tool_result`, `reasoning` (often encrypted + summary), optional `backend_tool_call`.

**Critical move:** almost everything that is not human can still be a **user-role** item tagged with `synthetic_reason` (`compaction_meta`, `system_reminder`, `project_instructions`, `auto_continue`, `goal_*`, wakes, …). Turn boundaries and compact re-anchor consult that tag, not the wire role.

### OpenCode V2 message types

From `packages/schema/src/session-message.ts` (verified):

| Type | Role in the abstraction |
|------|-------------------------|
| `user` | Promoted human prompt (text, files, agent mentions) |
| `assistant` | Model turn: text / reasoning / **tools as content parts with state machine** (pending→running→completed/error) |
| `system` | **Mid-conversation** chronological instruction (effective context update), *not* the baseline system blob |
| `synthetic` | Model-visible non-human user-shaped context (lowers to user role) |
| `shell` | Shell interaction folded into history |
| `compaction` | Checkpoint: structured `summary` + token-bounded `recent` string |
| `agent-switched` / `model-switched` | Durable selection events; **not** lowered to the model |

V1 uses role-based messages (`user`/`assistant`) with **parts** (text, tool, compaction, agent, …) — same product idea, different carrier.

**Rhymes:** both refuse “flat chat roles are enough.” Both have reasoning, tools, and non-human instructional chrome.  
**Cut differently:**

| | Grok | OpenCode |
|--|------|----------|
| Non-human steering | Mostly **synthetic user** + reasons | Typed message kinds: `system` / `synthetic` / `compaction` / switch events |
| Tool results | Separate `tool_result` items | Nested under assistant **tool content parts** with lifecycle status |
| Compaction | Not a first-class history type in the seed; **rebuild** seed | First-class `compaction` message (or V1 compaction part on a user message) that **cuts** projection |
| Baseline instructions | System message item (stable across compact) | **Baseline System Context** stored on Context Epoch; separate from Session History |

OpenCode’s refusal of the phrase “system prompt” in `CONTEXT.md` is not cosmetic: they split **immutable cache baseline**, **chronological mid-conversation system messages**, and **per-turn agent/system assembly**. Grok keeps a simpler picture: one system item + synthetic users for the rest.

---

## 4. Context assembly: what does the model “see”?

### Grok

| Layer | Abstraction |
|-------|-------------|
| `chat_history.jsonl` / ChatState | **Retained LLM conversation SOT** for resume and `build_request` |
| `updates.jsonl` | Append-only **event tape** (UI/raw I/O); not the LLM shape |
| Per-sample `build_request` | Clone ± prune of ChatState; not generally snapshotted |

Pre-compact understanding ≈ chat_history. Compact **replaces** that SOT with a ~6-item seed. Full pre-detail survives in compaction_requests, updates, optional segments — **outside** the live retained conversation.

### OpenCode

| Layer | Abstraction |
|-------|-------------|
| Durable Session events (`session.next.*`) | Append-only aggregate tape (V2) |
| Projected Session messages | Read model for UI and runner |
| **Session History** | Projection **after** compaction + Context Epoch cutoffs — *this* is what goes to the model as chronological conversation |
| **Baseline System Context** | Exact joined privileged text for the active Context Epoch (cache prefix); **not** part of Session History |
| Context Snapshot | Model-hidden JSON of last-admitted Context Source values |
| `sessions.context(...)` | Message-only; docs warn it is **not** the full provider request |

V2 runner assembly (verified in `runner/llm.ts` + `history.ts` + `to-llm-message.ts`):

1. Initialize/prepare Context Epoch (baseline + optional mid-convo system update)  
2. Load history entries **≥ latest compaction seq**, excluding baseline-covered system messages  
3. Lower messages to `@opencode-ai/llm` Message list  
4. Attach agent system + baseline as `SystemPart`s, tools, generation controls  
5. Optionally compact **before** streaming if estimate exceeds window − buffer  

**Rhymes:** dual-layer durability (raw/full vs model-facing); care about cache-stable prefixes; project instructions / env facts as ambient context.  
**Cut differently:**

| | Grok | OpenCode |
|--|------|----------|
| Model SOT after compact | **Thin rebuilt seed** | **Still the same projected store**, filtered by compaction boundary |
| Full pre-compact chat | Lives in request artifacts / updates | **Stays in Session message store**; only active projection shrinks |
| Ambient env / AGENTS | Injected as synthetic users (and system) | **Context Sources** with load/compare/render; changes become mid-convo `system` messages at safe boundaries |
| When context can change | Mostly around turns / compact | Explicit **Safe Provider-Turn Boundary** — never async-pushed mid-sample |

OpenCode’s Context Epoch is the clearest concept Grok does not name: *the span during which one rendered system baseline remains the immutable provider-cache prefix*, ending on completed compaction, session move, or incompatible context transition. Grok *behaves* as if system + project_instructions are cache-stable chrome, but the notes model continuity primarily as **chat_history epochs**, not as a first-class system-context generation.

---

## 5. Compaction philosophies (the sharpest divergence)

### Grok: replace retained conversation

**Input path:** verbatim pre-chat (+ same tools) + summarizer **user** prompt; **same system** as the agent (no summarizer persona swap).

**Output path:** `build_compacted_history` → ~6 messages:

1. system  
2. user_info (`compaction_meta`)  
3. project instructions  
4. **last real user query** (re-anchor)  
5. summary carrier (“session continued…”)  
6. system_reminder  

**Deliberately absent today:** assistant / tool_result / reasoning tail (`for_compaction()` clears `recent_messages`). Mid-turn tool chains survive only inside prose summary + updates tape.

**Auto mid-loop:** pre-sampling and preflight gates; after replace, **same turn loop continues**.

**Recovery modes:** `summary` | `transcript` (pointer to updates) | `segments`.

### OpenCode: checkpoint cut + rolling summary (history preserved)

**V2** (`packages/core/src/session/compaction.ts`, `specs/v2/session.md`):

- Before provider turn: estimate full request vs `context − max(output, buffer)`.  
- Select older serialized conversation as **head**, keep token-bounded **recent** string.  
- Summarizer call: **tools = []**, messages = user(summaryPrompt) only — *not* the live agent system/tools.  
- Rolling: previous compaction summary can be updated.  
- Publish `Compaction.Ended` with `summary` + `recent` → projects a `compaction` message.  
- Full transcript remains durable; **active model history** starts at that checkpoint.  
- Completed compaction forces a **new Context Epoch** (fresh baseline) on next provider attempt.  
- Overflow recovery: one post-reject compact even if estimate missed; no recovery loops after durable assistant output.

**V1 live** (`packages/opencode/src/session/compaction.ts`):

- Compaction is a **task** in the prompt loop (like subtask).  
- Uses a dedicated **compaction agent** prompt (`agent/prompt/compaction.txt`).  
- Selects **head** to summarize vs **tail** of recent **turns** (default `tail_turns`, token budget for preserved recent).  
- Optional **tool-output prune** (erase old tool bodies past protect threshold) as a *separate* pressure valve.  
- Plugin hook `experimental.session.compacting` can inject context or replace prompt.  
- Overflow path can **replay** a prior real user message after compact.

**Lowering** (V2 `to-llm-message.ts`): compaction becomes a **user**-role checkpoint:

```text
<conversation-checkpoint>
  <summary>…</summary>
  <recent-context>…</recent-context>
</conversation-checkpoint>
```

### Side-by-side

| Axis | Grok | OpenCode |
|------|------|----------|
| What compact *is* | **Replace** LLM SOT | **Cut** active projection; durable log keeps everything |
| Structured recent tools after compact | **No** (today) | V1: **yes** (tail turns); V2: recent is **serialized text**, not live tool pairs |
| Re-anchor last user query | **Explicit** always | Implicit via remaining history / overflow replay; not the same 6-item formula |
| Summarizer identity | Same agent system + tools | Separate minimal call (V2) or compaction agent (V1) |
| Post-check on summary quality | Empty / &lt;500 chars → retry | Fail closed on empty/error (V2 returns false; attempt leaves prior boundary active) |
| Multi-compact | Multiple epochs under same real turn | Rolling summary updates + new epoch baseline each completed compact |
| Pointer to raw tape | First-class transcript/segments modes | Managed tool-output files + full message store; no grok-style “read updates.jsonl” mode in the same way |

**Inference (honest):** Grok’s seed is optimized for *continuation identity* (“you are still doing **this** user query; here is a summary”). OpenCode’s checkpoint is optimized for *honest history with a model-visible fold* (“earlier conversation was folded here; recent string + new baseline; transcript still exists for clients”). That matches OpenCode’s broader product need (UI replay, multi-client, share) more than Grok’s shell-centric dual file layout.

---

## 6. Tools: local actuation model

### Shared problem

Frontier coding agents need parallel tool calls, large outputs, permission, and coherent tool_use/tool_result pairing across retries and interrupts.

### Grok (from notes)

- Tools hang off assistant messages; results are `tool_result` items.  
- Parallel batches per sample; sequential batches across loop iterations.  
- Compact prepare: truncate dangling incomplete tool_use before summarizer.  
- Post-seed: no tool pairs to keep coherent.  
- Oversized bash → `terminal/*.log` recovery surface.  
- Synthetic wakes: `task_completed`, `subagent_completed`, …

### OpenCode (verified)

- **Canonical tool type** (`Tool.make`): description, input/output codecs, execute, optional `toModelOutput`.  
- Registration scopes: process **ApplicationTools** + **Location-scoped** ToolRegistry overlay.  
- Settlement owns generic output bounding; complete oversized text → **Managed Tool Output File** under shared tool-output dir; model sees bounded preview + path.  
- Permissions: rulesets (`allow` / `ask` / `deny`) on action×resource; session-scoped pending asks; agent-specific merges (`build` open, `plan` edit-deny, etc.).  
- Provider-executed tools kept distinct (metadata must round-trip; not generic-bounded the same way).  
- V2: record tool call **before** side effects; eager parallel local execution; await all before next provider turn; interrupted running tools become durable failures.  
- **Task tool** = subagent session (`parent_id`, depth limit, optional background job + notification).  

**Rhymes:** parallel tools, truncate/bound model-visible output, subagents as nested work.  
**Cut differently:**

| | Grok | OpenCode |
|--|------|----------|
| Tool result storage | Conversation items | Content parts with explicit lifecycle + optional managed files |
| Authorization | Less central in the grounded notes | First-class Permission service + agent rulesets |
| Tool registry | Session/shell integrated | Location authority + application overlay + plugins/MCP (V2 still catching up) |
| Subagents | Wake synthetics into parent | Child **Session** graph + task tool protocol |

OpenCode treats **permission and filesystem Location** as part of the tool problem. Grok’s grounded abstraction treats **conversation coherence through compact** as the hard part of tools.

---

## 7. Identity of “the agent”

### Grok

- One primary agent mind per session is the default story in the notes.  
- Steering modes (`/goal`, plan mode reset after compact, reminders) decorate that mind.  
- Synthetic reasons encode *why* a non-human message exists without changing “who” the agent is.

### OpenCode

- **Agents are named products**: `build`, `plan`, `general`, `explore`, `compaction`, plus user-defined.  
- Each agent: mode (`primary` | `subagent` | `all`), permission ruleset, optional model/variant/prompt/steps.  
- Session carries selected agent; switches are durable (`agent-switched`) and apply at next provider turn.  
- Skill guidance is agent-filtered Context Source.  
- Plan↔build is a permissioned mode switch with dedicated tools (`plan_enter` / `plan_exit`), not only a prompt.

**Implication:** OpenCode’s unit of *policy* is agent×permission×location. Grok’s unit of *continuity* is real-user-query×summary×side-state. Both need both policy and continuity; they lead with opposite ends.

---

## 8. Autonomy and interruption

### Grok patterns (notes)

- Mid-turn auto-compact without new human message  
- `/goal` synthetic continuations + token budget side-state  
- Interjection (Ctrl+Enter) without cancelling turn  
- Wake synthetics that **can** start prompt turns (`starts_prompt_turn`)  
- Multi-epoch overnight autonomy under one or few real user turns  

### OpenCode patterns (verified)

- **Steer vs queue** delivery on durable inbox  
- Promoting any new user input **resets** provider-turn / step allowance  
- `agent.steps` max → final step forces no tools + max-steps prompt  
- `sessions.interrupt` — process-local, preserves inbox  
- Background subagents (experimental flag) notify rather than poll  
- Explicit **no** post-crash automatic provider retry yet (V2 deferred)  
- Question tool / permission ask can halt the loop (decline ≠ tool error)  

**Rhymes:** both support long multi-step work and mid-flight human steering.  
**Cut differently:** Grok leans on **synthetic conversation items + goal tracker**; OpenCode leans on **durable inbox delivery modes + process coordinator + agent step budgets**. OpenCode is more explicit that drains die with the process; Grok’s session files (`chat_history` + updates) are the resume story the notes emphasize.

---

## 9. Persistence and multi-surface

### Grok session on disk (notes)

```
~/.grok/sessions/<id>/
  chat_history.jsonl   ← LLM SOT
  updates.jsonl        ← event tape
  compaction_requests/
  compaction_checkpoints/
  compaction/segments? 
  terminal/*.log
```

Resume = rehydrate ChatState from chat_history (not replay entire updates).

### OpenCode

- SQLite (Drizzle) for sessions, messages/parts (V1), V2 event aggregates + projections, context epochs, todos, permissions, workspaces…  
- Server HttpApi + generated clients (Promise/Effect) + embedded in-process host  
- Durable Session event stream (`sessions.events`) vs live-only instance stream  
- Share, desktop, TUI, web console — multi-client by construction  
- Location / workspace placement (future remote); move session clears Context Epoch  

**Rhymes:** durable session identity; separate rich tape vs model view.  
**Cut differently:** Grok is **file-sovereign session directory** with two primary logs; OpenCode is **database-backed session aggregate** with API projection and multi-client contracts. That product choice drives OpenCode’s event/projection discipline and Grok’s simpler replace-the-jsonl compact.

---

## 10. LLM package boundary

OpenCode’s `packages/llm/DESIGN.md` makes a clean cut Grok’s notes don’t need to state because sampling is in-house:

| LLM package owns | Session runtime owns |
|------------------|----------------------|
| Provider Turn = one request/response | Multi-turn agent loop / drain |
| Portable Request / Message / Tool def | Durable history, permissions, tools execute |
| Stream events | Projection, settlement, continuation |

V2 insists: **one explicit `llm.stream(request)` per provider turn**; do not hide an in-memory tool loop inside the LLM client. Grok’s agent loop owns sampling end-to-end in the shell; the “provider turn” concept is present as a sample but not elevated as a package boundary.

---

## 11. What each model makes easy / hard

### Grok’s abstraction makes easy

1. Reasoning about **real human turns** vs chrome  
2. **Mid-turn** compact and continue without inventing a new user message  
3. Exact pre-compact fidelity experiments (chat_history ≡ compact request)  
4. Compact as a **thin mental restart** with last query re-anchor  
5. Goal/overnight autonomy as “same turn, many epochs”  

### Grok’s abstraction makes hard (or currently weak)

1. Multi-client honest UI over full history after compact (history was replaced)  
2. Structured tool-chain resume after compact (recent_messages dropped)  
3. Independently evolving ambient context without synthetic-user proliferation  
4. Multi-agent policy product surface  

### OpenCode’s abstraction makes easy

1. **Full durable transcript** + multi-client replay while model sees a fold  
2. **System context** as typed, refreshable sources with cache-safe baselines  
3. **Permissioned multi-agent** product (plan vs build, subagent sessions)  
4. Clean **LLM vs session** package boundary; multi-provider catalog  
5. Explicit input admission (steer/queue) without overloading “user message”  

### OpenCode’s abstraction makes hard (or currently partial)

1. V1/V2 dual reality — live loop and domain language diverge mid-migration  
2. Post-crash continuation deliberately unfinished  
3. Compaction recent context as **serialized text** (V2) is weaker structured continuity than “keep last N tool turns” (V1 tail) or than a true tool-pair rehydrate  
4. More moving parts (epochs, snapshots, drains, locations) before a single shell session “just runs”  

---

## 12. Crosswalk of near-synonyms (use carefully)

These rhyme but are **not** equal:

| If you think… | Grok-ish | OpenCode-ish | Don’t assume |
|---------------|----------|--------------|--------------|
| Durable chat | Session + `chat_history` | Session + projected messages / events | Same replace-vs-filter semantics |
| One model call | Model sample / loop iteration | Provider Turn | Same tool-loop embedding |
| Work unit from human | Real user turn | Admitted→promoted prompt (plus V1 user message) | Compact ends the unit (false in both) |
| Context reset | Compaction epoch | Context Epoch **and/or** history cut at compaction message | Epoch means only one thing |
| Non-human instruction | Synthetic user + reason | `system` / `synthetic` / reminders / agent prompt | Same wire role |
| Keep going after tools | Agent-loop continue | Next Provider Turn after settlement | Drain identity is durable (OpenCode: no) |
| Keep going after compact | Loop continues on new seed | ContinueAfterCompaction rebuild; V1 compaction task then loop | Seed shape is comparable |
| Raw full record | `updates.jsonl` | Event aggregate + unpruned messages | Same schema or recovery UX |
| Ambient project law | `project_instructions` synthetic | AGENTS Context Source (+ V1 Instruction service) | Same update admission timing |

---

## 13. Implications for later work (observational, not a plan)

1. **If you care about personhood / audit honesty after fold:** OpenCode’s “never delete the transcript; cut the projection” is closer to an inviolable-recording instinct than Grok’s replace-chat_history. Grok compensates with updates + compaction_requests — powerful, but the *agent’s* SOT is the thin seed.

2. **If you care about mid-compact agent continuation quality:** Grok’s explicit **last real query re-anchor + loop continues** is a strong, simple story. OpenCode relies on checkpoint text + remaining tail + new baseline; V1 tail turns help; V2 recent string is coarser.

3. **If you care about ambient world model (date, AGENTS, skills) without cache thrash:** OpenCode’s Context Epoch / Context Source algebra is the more developed theory. Grok achieves partial stability with system + project_instructions synthetics and reminders.

4. **If you care about product multi-agent + permissions:** OpenCode is organized around it; Grok notes organize around conversation authenticity tags.

5. **If you care about porting ideas:** steal **joints**, not ontologies. High-signal joints that appear in both and in workshop notes elsewhere:
   - dual durability (tape vs model view)  
   - safe boundary for admitting context changes  
   - compact as epoch, not as “turn ended”  
   - subagent as nested session or wake, not fake user  
   - tool output bounding separate from transcript honesty  

6. **OpenCode V2 is the fair comparison target for architecture; V1 is the fair comparison for lived coding-agent behavior today.** Mixing them silently will produce false rhymes.

---

## 14. Source map (OpenCode)

| Topic | Path |
|-------|------|
| Domain language | `/Users/josephwecker-v2/src-ext/opencode/CONTEXT.md` |
| Session V2 design | `…/specs/v2/session.md` |
| Provider turn runner | `…/packages/core/src/session/runner/llm.ts` |
| History projection | `…/packages/core/src/session/history.ts` |
| Compaction (V2) | `…/packages/core/src/session/compaction.ts` |
| Context Epoch | `…/packages/core/src/session/context-epoch.ts` |
| Input admit/promote | `…/packages/core/src/session/input.ts` |
| Message types | `…/packages/schema/src/session-message.ts` |
| Lowering to LLM | `…/packages/core/src/session/runner/to-llm-message.ts` |
| System Context algebra | `…/packages/core/src/system-context/index.ts` |
| Live V1 loop | `…/packages/opencode/src/session/prompt.ts` |
| Live V1 compaction | `…/packages/opencode/src/session/compaction.ts` |
| Agents | `…/packages/opencode/src/agent/agent.ts` |
| Task/subagent | `…/packages/opencode/src/tool/task.ts` |
| Tool architecture | `…/packages/core/src/tool/AGENTS.md` |
| LLM package intent | `…/packages/llm/DESIGN.md` |
| AGENTS V2 rules | `…/AGENTS.md` (Session V2 section) |

Grok source map remains in the three compaction notes under `grok-compaction/`.

---

## 15. Caveats

- No parallel OpenCode live compact session was artifact-diffed the way grok part-1/part-2 were; OpenCode compact claims are **source/spec verified**, not empirically token-counted here.  
- OpenCode moves quickly; V2 event schemas are marked experimental in specs. Re-check `CONTEXT.md` and `specs/v2/session.md` before treating a detail as frozen.  
- Grok side is intentionally **not** re-walked against `~/src-ext/grok-build` in this pass; if those notes drift from code, this comparison inherits that drift.  
- This file’s name says “terminology” for continuity with the ask; the useful frame is **abstraction model**, not a glossary only.

---

*Written 2026-08-05 for the harness workshop. Extend when either side’s compact/session algebra changes underfoot.*
