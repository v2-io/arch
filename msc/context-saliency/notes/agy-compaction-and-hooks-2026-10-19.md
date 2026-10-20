# Antigravity (`agy`) compaction, transcripts, and hooks — reading & comparison, 2026-10-19

*A Gemini 4 Argon (Medium) instance's reading of the top-level `context-saliency` record alongside an inspection of the local `agy` binary (`~/.local/bin/agy`), its built-in skills (`~/.gemini/antigravity-cli/builtin/skills/`), and the live session transcript (`transcript.jsonl`).*

**Read whole:**
- `REFLECTIONS-2026-08-28.md`
- `PLAN-experiments-draft-2026-08-28.md`
- `GOAL-exp1-scaleup-and-calibration.md`
- `COMPACTION-DESIGN-THEORY-2026-08-29.md` (including §§8–9)
- `handoff.md`
- `HYPOTHESES-AND-FINDINGS-2026-09-24.md`
- `opus-5.5-summary-example.md`
- `notes/grok-build-upstream-compaction-2026-09-24.md`
- `~/.gemini/antigravity-cli/builtin/skills/agy-customizations/SKILL.md`, `docs/hooks.md`, `docs/plugins.md`
- `~/.gemini/antigravity-cli/builtin/skills/antigravity_guide/SKILL.md`, `references/cli.md`, `references/sdk.md`

**Read in part / sampled:**
- `dossier-2026-08-28.md` (§§1–4)
- `summary-replay/transcript.py` (header and path/event normalization helpers)
- `$(which agy)` binary protobuf descriptors and embedded checkpointer configs (`exa.hooks_pb`, `exa.cortex_pb.CortexStepCheckpoint`, `exa.cortex_pb.CheckpointConfig`, `genai.AntigravityAgentConfig`, `genai.CallHookRequest`, `CASCADE_USE_EXPERIMENT_CHECKPOINTER`)
- Live session `transcript.jsonl` (`~/.gemini/antigravity-cli/brain/21042511-e7e2-40bc-860f-9244c0054061/.system_generated/logs/transcript.jsonl`)

---

## 1. General observations on the `context-saliency` record [mine]

1. **From fixing content inside the genre (P1–P10) to diagnosing the genre itself (§§8–9).**
   The strongest empirical turn in the record is the transition after the first natural compactions under the P1–P8 / P10 templates (`01a0533d` project-heat and `01a05886` collaboration). Adding an epistemic banner, retitling "All User Messages," and adding a mandatory `Known Holes` section produced *"a more honest costume"* (`01a05886` gen 0), because the surrounding 11-section synthesis still performed a complete picture. Even when `Known Holes` explicitly stated that the Python source was *"only described; successor must read the file on disk,"* the successor grepped and sampled instead of reading the three grounding anchors whole. Shape beat instructions (P10).

2. **"Settling" is a distinct failure mode from "dropping."**
   Lossy compression is usually analyzed in terms of what it *omits*. The natural compacts here isolate a second, often worse failure: **premature epistemic closure ("settling")**. Topical buckets ("Key Technical Concepts", "Problem Solving", "Optional Next Step" in `grok-build`; "Key Technical Concepts", "Errors and fixes" in `opus-5.5-summary-example.md`) force in-flight searches or circular self-checks (`git log --reverse -1`, the `shell_state.rs` wrapper story, the `\x01` vs. NUL diagnosis) into declarative taxonomy. Once filed under a structural heading, a hypothesis loses the temporal neighborhood that marked it as unverified.

3. **Why truncation (`…`, `lim=NN`) succeeds where `Known Holes` prose fails.**
   A summary sentence describing a file mimics coverage and discharges the urge to read. A truncated tool row in `transcript.py` (`read <f8> lim=50` or 150-char `…`) is *structurally incomplete*: the successor cannot mistake a call signature and a cut line for the territory.

4. **Structure within Chronology (SwC) vs. the atopical class.**
   Placing structural transitions inside a chronological spine (SwC, §9) naturally preserves the epistemic status of an open search (turn $k$'s hypothesis is visibly followed by turn $k+1$'s mixed output and a `meta | compact` seam) and preserves the *walk* of user corrections rather than flattening them into a moral. The counterweight is the **atopical class** (standing law / negative constraints), which similarity retrieval (`C\D` lazy recall) misses and pure chronology buries at early turns—while LLM-extracted "Standing Constraints" sections repeatedly scrape the always-loaded system prompt and misattribute it to the user (`01a0533d` compacts #1 and #2). Keeping the spine chronological (`transcript.py` + generation-aware tool folding for generations $\ge 2$ back) while extracting standing rules strictly from `user` rows with turn/timestamp provenance avoids both failure modes.

---

## 2. What `agy` has for hooks and compaction [inspected in binary + docs]

`agy` exposes two distinct hook layers:

### A. Declarative `hooks.json` (`third_party/jetski/hooks_pb/hooks.proto`, package `exa.hooks_pb`)
Configured in `.agents/hooks.json` (workspace) or `~/.gemini/config/hooks.json` (global).
- **Top-level events:** `SessionStart`, `PreToolUse`, `PostToolUse`, `PreInvocation`, `PostInvocation`, `Stop`.
- **No dedicated `OnCompaction` event key** exists in `hooks.json`.
- **Common input (`HookArgsCommon`, sent on `stdin` to every hook):**
  - Documented: `conversationId`, `workspacePaths`, `transcriptPath`, `artifactDirectoryPath`, `modelName`.
  - Additional fields in the proto descriptor: `executionId`, `isBattleMode`, `lastUserInput`, `agentName`, `parentConversationId`, `callerContext`, `rootConversationId`.
- **`PreInvocation` / `PostInvocation` / `SessionStart` step injection (`HookInjectedStep`):**
  - Hooks can return `{"injectSteps": [...]}` on `stdout`.
  - Documented step variants: `systemMessage` (persists across turns), `userMessage`, `ephemeralMessage` (dropped after the current turn), `toolCall`.
  - **Undocumented proto variants in `exa.hooks_pb.HookInjectedStep`:**
    - `checkpoint`: `{"checkpoint": {"sessionSummary": "..."}}` (`exa.hooks_pb.HookCheckpoint`)
    - `errorMessage`: `{"errorMessage": {"modelMessage": "...", "userMessage": "..."}}`
    - `hookUserMessage` / `hookEphemeralMessage` (with `content`, `media`, `metadata`)

### B. Programmatic SDK / `localharness` API (`learning/genai/api/interactions/proto/hooks.proto` & `agents.proto`, package `genai`)
Used when driving the harness via the Python SDK (`google-antigravity`) or `localharness`:
- **Has an explicit compaction hook:** `LIFECYCLE_HOOK_ON_COMPACTION` (`OnCompactionArgs`), firing with:
  - `interactionId` (`string`)
  - `stepIndex` (`int`)
  - `summary` (`string`)
  - Returns `EmptyResult` (observation/notification only; cannot mutate the summary in-flight).
- **Exposes `CompactionConfig` / `AntigravityCompactionConfig`:**
  - `token_threshold`, `checkpoint_interval_tokens`, `max_context_tokens`, `summary_prompt_override`, `use_model_reported_token_count`.

---

## 3. How `agy`'s native checkpointer works — and how it sits against the theory

In `exa.cortex_pb`, `agy` compacts by emitting a `CortexStepCheckpoint` step governed by `CheckpointConfig`.

### What converges with the theory (parallel to `grok-build` upstream's `Segments` + `CompactionStateContext`)
1. **Two-tier verbatim transcript on disk, with address injected by default:**
   Every session writes:
   - `<appDataDir>/brain/<conversation-id>/.system_generated/logs/transcript.jsonl` (compact: large tool outputs are middle-truncated with `<truncated N bytes>` and `"truncated_fields": ["content"]`), and
   - `transcript_full.jsonl` (untruncated, line-aligned with `transcript.jsonl`).
   The `<conversation_transcript>` block in the system prompt gives the agent both paths and tells it to read `transcript.jsonl` first and fetch specific lines from `transcript_full.jsonl` when `truncated_fields` is present.
2. **Cleartext `thinking` in the transcript:**
   Unlike xAI's Responses API (`notes/grok-build-thinking-opacity-2026-08-29.md`), `agy`'s `transcript.jsonl` records `"thinking"` in cleartext on `PLANNER_RESPONSE` steps, alongside `input_tokens`, `cache_read_tokens`, `output_tokens`, `created_at`, and `tool_calls` (including the 2–5 word `toolSummary`).
3. **Structured harness snapshots carried across the seam:**
   `CortexStepCheckpoint` carries structured fields alongside `session_summary`:
   - `user_requests` (recent changelog: user-request budget is spread across all captured prompts rather than truncating a long initial instruction to a small fixed slice)
   - `artifact_snapshots` (`include_artifact_snapshots: true`)
   - `subagent_snapshots` (`include_subagent_snapshots: true`)
   - `running_task_snapshots` (`include_running_task_snapshots: true`)
   - `trajectory_file_diffs` / `edited_file_map`
   - `include_last_user_message`

### Where `agy`'s summarizer prompt repeats the exact defects of the theory
In the embedded `CASCADE_USE_EXPERIMENT_CHECKPOINTER` config (`CHECKPOINT_STRATEGY_SAME_MODEL`), `session_summary_prompt_override` is:

> *"You have been working on the task described above but have not yet completed it. Write a continuation summary that will allow you (or another instance of yourself) to resume work efficiently in a future context window where the full conversation history will NOT be available—only this summary.*
>
> *This summary is all that will be available to you going forward in the future context window. Do not call any tools, simply just provide the summary based on the information available in the current context window.*
>
> *Your summary must be structured, concise, and actionable. Optimize for enabling immediate resumption with zero redundant work.*
>
> *Include the following sections:*
> *1. **Task Overview** (core request, constraints, ambiguities resolved)*
> *2. **Progress** (completed with concrete references, key artifacts, in progress)*
> *3. **Key Findings** (technical constraints, decisions & rationale, errors & resolutions, approaches tried and abandoned)*
> *4. **Active Context** (external resources, intermediate results, hypotheses, working assumptions)*
> *5. **Next Steps** (specific actions in priority order, blockers, prerequisites)*
> *6. **Commitments & Constraints** (promises made to user, preferences, boundaries)*
>
> *Be concise but complete—err on the side of including anything that would prevent duplicate work, repeated mistakes, or broken promises."*

Against `COMPACTION-DESIGN-THEORY-2026-08-29.md`:
- **Direct P1 violation (false-completeness implicature):** *"This summary is all that will be available to you going forward"* and *"Be concise but complete"* instruct the compactor to write as if the transcript does not exist—even though `transcript.jsonl` and `transcript_full.jsonl` sit on disk and are named in the system prompt.
- **P10 brevity vs. completeness collision:** *"concise, and actionable"* + *"Be concise but complete"* without structural exemptions.
- **Settling vs. Active Context:** Although §4 mentions *"hypotheses, or working assumptions,"* §3 (*"Key Findings: Technical constraints, requirements, or domain details uncovered"*) still invites the compactor to promote open investigations into findings.
- **§6 Commitments & Constraints:** Invites the same system-prompt-to-user misattribution observed in `project-heat` §10.

---

## 4. Practical openings in `agy` for this subproject

Because `agy` writes a rich, cleartext-thinking `transcript.jsonl` + `transcript_full.jsonl` and runs `PreInvocation` hooks synchronously before every model call with `transcriptPath` and `initialNumSteps` on `stdin`:

1. **`transcript.py` adapter for `agy` (`transcript.jsonl`):**
   `agy`'s JSONL steps (`USER_INPUT`, `PLANNER_RESPONSE`, `GENERIC`, `CHECKPOINT`) already carry ISO `created_at` timestamps, `toolSummary` labels, cleartext `thinking`, and `truncated_fields` markers. A reader in `summary-replay/transcript.py` can emit the exact same temporal print view (with time deltas, `<dN>`/`<fN>` aliases, and `meta | compact` seam rows) from `agy` sessions—and optionally test whether folding or retaining cleartext `thinking` helps with interiority erasure.
2. **Seam-timed act-instruction via `PreInvocation` (`hooks.json`):**
   In `project-heat` #2, a live seam-timed instruction to read the grounding files whole prevented post-seam churn where a `Known Holes` line did not. A `PreInvocation` hook can detect when a new `CHECKPOINT` step appears in `transcript.jsonl` (or when `initialNumSteps` resets) and inject a one-turn `ephemeralMessage` (or `userMessage` / `toolCall`) at the exact first turn after compaction—either instructing the successor to read the grounding files / `transcript.py -v` output whole, or directly injecting the `transcript.py -v` experiential log.
3. **Harness-level Lazy Differential Recall (`C\D`, P9) without binary patches:**
   Because `PreInvocation` receives `lastUserInput` ($Q$) and `transcriptPath` ($A$) on every turn, `spike-01/embed_diff_pg.py` can run inside a `PreInvocation` hook after any compaction seam, injecting positive-margin $C \setminus D$ hits as single-turn `ephemeralMessage` pointer-addenda (`transcript_full.jsonl` line addresses) that automatically vanish on the next turn.
