# grok-build thinking/deliberation opacity — investigation findings, 2026-08-29

*A fork's de-novo investigation of Joseph's question: why grok-build's thinking seems opaque and confused compared to Claude Code — whether agents are instantiated silently, whether thinking blocks are truncated or fail to persist in events.jsonl, and what's actually going on. Findings tiered: **[logs]** = observed in Joseph's real session files (strongest), **[code]** = read in the source with paths verified, **[inferred]** = consistent interpretation, not proven per-incident.*

## The core answer: the full thinking never reaches your machine in readable form **[logs]**

In `~/.grok/sessions/.../chat_history.jsonl`, every `reasoning` item has exactly two parts:

1. a **summary** of a few sentences ("Good, I can see the landscape. Now I need to read..."), and
2. an **`encrypted_content` blob** — the real chain-of-thought, encrypted by the xAI API, undecryptable locally, returned only for the provider's own cross-turn reuse.

This is the Responses-API reasoning model: xAI streams *summaries* of thinking plus an opaque blob. Claude Code appears less opaque because the Anthropic API streams the **actual thinking text**; grok-build structurally cannot show what it is never sent.

**The harness itself is faithful** **[code]**: `show_thinking_blocks` defaults ON, and the sampler forwards both `ReasoningSummaryTextDelta` and `ReasoningTextDelta` to the same display channel — it displays and persists everything it receives. The truncation is upstream, at xAI. **No harness change can recover what the API encrypts.** (Not checked: whether xAI offers a raw-CoT API tier — worth one question to xAI if full thinking matters.)

## events.jsonl is telemetry, not transcript **[code, confirmed in logs]**

Its `Event` enum is `turn_started / phase_changed / tool_started / permission_* / turn_ended / interjected / TodoGate* / LazinessClassifier* / GoalClassifier*` — no content-bearing variants at all. Thinking doesn't "fail to persist" there; content was never that file's job. The conversation (with the summary+encrypted reasoning items) lives in `chat_history.jsonl`; verbose pre-compaction transcripts in `compaction/segment_*.md` when enabled.

## Silent agents: yes — an entire ecology **[code; presence confirmed in logs]**

1. **Goal mode spawns a subagent troupe**: `goal_planner` (fail-closed), plural **verifier "skeptics"**, a `goal_strategist` (fires after N failed verifications), `goal_classifier`, `goal_summarizer` — each a separate model run with its own prompt template (`session/templates/goal_*.md`). They surface only as brief entries in the pager's "subagent strip."
2. **Subagent sessions are deliberately hidden**: they persist as full sessions on disk, but `is_hidden()` excludes any `session_kind` starting with `"subagent"` from history listings — confirmed in real logs (`session_kind: subagent_fork`). Their deliberation exists but is invisible unless you dig into `~/.grok/sessions/` directly.
3. **LazinessDetector**: a classifier model watches idle turns and can inject system-reminder "nudges" (off by default, two-step opt-in — check config.toml if enabled).
4. **Silent memory injection**: `xai-grok-memory` retrieves and injects memory snippets into prompts (126 memtrace logs found in `~/.grok/memtrace/`).
5. **Background model calls with no conversation presence**: title generation, `image_describe`, and the two-pass compaction (pass 1 summarizes the oldest 95% → note; pass 2 rewrites note+tail → final) — two full hidden LLM runs per compaction.

**[inferred]**: the "communicating briefly with other agents" perception most likely maps to the goal-role troupe plus hidden subagent sessions — consistent with the code and logs, not proven per-incident.

## "Formulating responses then rejecting and redoing" **[code]**

Almost certainly the **doom-loop discard-and-retry**: the xAI *server* runs a `doom_loop_check` (e.g. trigger `tail_repetition:8@thinking`); the harness classifies `DoomLoopDetected` as retryable with immediate backoff and **resamples the entire response** (retry budget up to 14). You watch a response begin, vanish, and restart. 413-image-strip retries and mid-turn interjection merges (`xai-interjection-core`) produce similar visible re-formulation.

## Consequence carried into the compaction work

For grok-hosted reasoning models, the harness-held pre-compaction record "A" does **not** contain full thinking in cleartext — summaries only. The lazy-differential design's fidelity claim ("A holds more than the API re-delivers") holds fully for Anthropic-style APIs and only partially here; the design doc now carries this as a provider-conditional caveat (commit `18c2c39`).

## Coverage honesty (the fork's own)

Log observation + code reading settled the main question, so no web research was performed; the raw-CoT-tier question to xAI is the one named unexplored lead. Attribution of any *specific* observed incident to a specific mechanism above was not done — the mechanisms are established, the per-incident mapping is inference.
