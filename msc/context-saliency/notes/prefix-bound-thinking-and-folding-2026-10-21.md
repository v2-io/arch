# Prefix-bound thinking, and what it does to folding

*2026-10-21, Claude (Opus 5.5), from a conversation with Joseph after the anatomy of an agent's context (`~/src/agent-core/anatomy-of-an-agent/`). It answers his question of what exactly must stay unchanged in a context on the newest Claude models, and what happens when it doesn't, and then follows it into what a middle fold in Claude Code can and can't do. Marks: **[docs]** the Anthropic docs snapshot of 2026-09-30 (`~/src/_ref/claude-docs/docs/en/`), read whole for the pages named; **[src]** Claude Code 2.1.307's extracted code, or the mod API types bundled with the plugin-authoring skill (build 2.1.301); **[wire]** Joseph's Claude Code transcripts; *(inferred)* my reading, not checked.*

Joseph's question, verbatim: *"In practice, it seems like the ability to do something like we have wanted to do in ../arch/msc/context-saliency/notes/aat-strategy-epistemics-and-middle-folding-2026-10-19.md seems rather seriously hobbled if the entire past context is essentially immutable (for, say, Opus 5.5)? I know Anthropic has always required the retention of encrypted thinking blocks adjacent to tool usage -- but it used to discard them on the wire or at the server after that tool result batch was in, and it always discarded thinking summary pieces as one would expect. The only question *used* to be how much of the cache you were willing to blow away (and incur the associated cost). Can you verify and clarify exactly what must be kept immutable and what happens when it is not etc.?"*

## 1. The short answer

Nothing in the past context is immutable. What changed is the price of editing it. On Fable 5.1, Opus 5.5 and Sonnet 5.5, a client-side edit anywhere before a thinking block invalidates that block and every later one. The text, the tool calls and the tool results stay the client's to change. What an edit costs is the sealed reasoning from the edit point onward, plus the cache, as before.

## 2. What changed, and when

- **Before Opus 4.5 (and Sonnet 4.6), models kept only the last turn's thinking.** "When you pass older thinking blocks back, the API strips them automatically" **[docs: `thinking.md`]**. Joseph's memory of the old behavior is right for these models.
- **From Opus 4.5 and Sonnet 4.6, all prior turns' thinking is kept by default,** and all Fable and Mythos models keep it too **[docs: `thinking.md`, `context-editing.md`]**. Through Opus 5 nothing bound it, so a middle edit cost only cache.
- **The summaries never reached the model.** "Summarization is processed by a different model from the one you target in your requests. The thinking model does not see the summarized output." The server rebuilds the thinking from the `signature` **[docs: `thinking.md`]**.
- **The binding is new with Fable 5.1, Opus 5.5 and Sonnet 5.5.** Mythos 5.1 and earlier models don't run the check **[docs: `preserved-thinking.md`]**. The census dates the model-binding announcement to 2026-09-01 (`cm:0fe37f81c0e42ef2`).

## 3. What exactly is checked

From `preserved-thinking.md` **[docs]**:

- **The prefix:** the top-level `system`, the `tools`, and every message before the block, compared as the client sent them. JSON formatting and key order don't matter; values do.
- **Not part of it:** `effort`, `max_tokens`, `output_config`, `tool_choice`, `metadata`, `thinking.display`, `cache_control` markers, and anything server-side clearing or compaction changes ("the check compares what you sent, not the server's edited copy").
- **The chain:** each thinking block records the one before it. Removing blocks from the start, from the end, or all of them is valid. Removing one from the middle invalidates every later block, and putting a removed block back invalidates the ones produced while it was gone.
- **Still valid:** appending; adding a `defer_loading` tool nothing has referenced yet; a cleared turn-scoped system message left in place.
- **Invalid:** editing, reordering or deleting any earlier message; re-rendering the first user message; clearing or shortening an earlier `tool_result`; adding or removing a reminder in an earlier turn; changing `system` or `tools`; a document URL whose bytes change.
- **Also bound:** the model that can read a block (Fable 5.1 reads Opus 5.5's, not the reverse), and for Sonnet 5.5 the account. Those drops are silent without the beta header.

## 4. What happens when it changes

- **`prefix_mismatch_behavior: "error"`:** a 400 naming the first failing block. This is the default for accounts created from 2026-08-31.
- **`"drop_block"`:** the request succeeds. The failing block and every later one are dropped and not billed, the cache restarts at the edit, and each drop is listed in `input_transformations` (`prefix_binding_mismatch`). "Claude can sometimes think more to re-create the dropped thinking."
- **Older accounts that set nothing:** the check runs but the blocks still reach the model, flagged as `thinking_mismatch_allowed` under the beta header `thinking-binding-controls-2026-08-01`.
- **A tampered signature** is a different failure: always a 400.
- **Observed, contrary to the above** **[wire]**: on Claude Code 2.1.307 today, requests that set no `block_binding` (but send the beta header) had stale thinking dropped and listed as `thinking_dropped`: after Claude Code's own `/compact` and after a mod's fold, on Sonnet 5.5, and also on Haiku 5.5, which the docs don't list among the checked models. By the docs those requests should have got a 400 or the block delivered and flagged. Either the snapshot is stale or enforcement has changed. Found by the middle-fold spike (§12); I confirmed the entries in its lab transcripts.

## 5. The tool-round warning, which looked like a contradiction

The docs warn: "Don't compact between an assistant turn's `tool_use` and the `tool_result` that answers it. Send that assistant turn back with its thinking intact so the model finishes the round with its reasoning." This isn't a second rule. The binding rule says which thinking survives an edit: thinking before the edit, and thinking produced after it, but not thinking that already existed after the edit point when it was made. The warning is about where to put the seam: an open round is where the reasoning matters most, and it can only keep its thinking if it sits wholly after an API-written seam. So a fold belongs just after a round closes.

## 6. How much thinking is at stake

From one Opus 5.5 Claude Code session in ai-risk-model (Oct 6–10, about 1,400 requests, 5 compactions) **[wire]**. Thinking isn't counted separately, so this is output tokens minus the visible text and tool input at about 3.6 characters per token, and rough:

- Sealed thinking was about **56% of everything the agent produced** (about 750k tokens of thinking against about 600k visible).
- The median request with thinking carried about 460 tokens of it; the 90th percentile about 1,400; the largest about 8,200.
- Each stretch between compactions retained about 30k–210k tokens of thinking, 11–22% of the window at its fullest.
- Individual blocks don't grow over a stretch (median signature about 1,500 characters, flat); the number of them does. Signature length tracks the thinking behind it (correlation 0.92 with the estimate, above a floor of about 820 characters), which fits the docs' "an encrypted copy of the full reasoning".

So after a mid-session fold the agent still sees what it did and said after the fold point, but not more than half of what it produced there, which is mostly the "why". The models before Opus 4.5 worked this way on every turn, so it's a degradation, not a breakage. The docs give no figure for the quality cost.

## 7. What Claude Code has already lost this way

Traced by a separate agent in `~/src/agent-core/anatomy-of-an-agent/sources/claude-code-thinking-drops.md` (structure only, from all of Joseph's transcripts, Claude Code's own `thinking_drop` and `compact_boundary` records, and the 2.1.307 code):

- **Each auto-compaction (2.1.285–2.1.296)** kept a short verbatim tail that starts with the last assistant turn, thinking included. That block was produced against the pre-compaction history, so it failed every later check. Exactly one block per compaction was lost, the reasoning behind the step in flight: 44–3,691 thinking tokens per compaction in the ai-risk-model session. Blocks produced after the compaction stayed valid. This is the docs' "keep-tail compaction" case.
- **Resumed subagents and forks (2.1.278–2.1.289)** were rebuilt from the transcript, and the rebuild differed from what had been sent. About 1,760 blocks were lost over 46 resumes, up to 312 at once: the whole earlier reasoning of an agent at the moment it was asked a follow-up. The 2.1.290 changelog fixes it.
- **The reporting is paced, not the dropping.** After a server-side change on 2026-10-06 (inside one running 2.1.289 process), a stale block was listed only on requests five minutes or more after the previous response; cache arithmetic shows it was out of the model's view on every request. The raw counts I first gave (about 5,000 records) overstate distinct blocks lost about 70 times. Counting listed responses counts idle gaps, not losses.
- **Now:** drop mode was a served setting. From 2.1.299 Claude Code sends no `block_binding` (the selector reads `CLAUDE_CODE_POLISHED_DEWDROP`, then served config, then a flag defaulting to none). The ~11,300 logged requests since showed no drops only because nothing was edited: on 2.1.307 a `/compact` still loses the kept tail's thinking, with no `block_binding` sent (§4). If the API rejects a block, 2.1.307 strips thinking from that block on and records it durably. Its compaction can strip the tail's thinking first, but that is off by default (`tengu_wise_comet`).

## 8. The server-side tools that don't invalidate thinking

As of the snapshot there are three context-management edits, and none replaces an arbitrary range or takes custom text **[docs: `context-editing.md`; API reference `api/beta.md`]**:

- **`clear_tool_uses_20250919`:** `trigger` (input tokens or tool uses), `keep` (recent tool uses, 0 or more), `clear_at_least`, `exclude_tools` (names never cleared), `clear_tool_inputs` (`true` or a list of tool names). Results are cleared oldest-first and replaced with fixed placeholder text. Responses report `applied_edits` with `cleared_input_tokens` and `cleared_tool_uses`. Paired with the memory tool, "Claude receives an automatic warning to preserve important information" near the threshold, which is the closest shipped analog to an agent writing its own fold stub.
- **`clear_thinking_20251015`:** `keep` is `"all"` or the last N assistant turns. Kept thinking keeps the cache; clearing restarts it at the clearing point. The docs say nothing about how much earlier thinking is worth keeping, beyond "No intelligence impact" from preserving it. *(Inferred:)* a sliding `keep: N` moves the clearing point every turn and probably restarts the cache each time, so clearing in occasional large steps would be cheaper.
- **`compact_20260112` (threshold compaction):** `trigger` (150k by default), `instructions`, `pause_after_compaction`. "Thinking blocks from before a `compaction` block aren't carried forward"; messages re-inserted after the block must have their thinking stripped or dropped.

And one separate request type:

- **On-demand compaction (`compact-2026-09-04`):** the client chooses when and where to cut (between a reply and the next user message). The API summarizes everything from the start, or from the previous block, up to the cut, under `instructions` of up to 16,384 characters, and returns a signed block that can't be edited. The kept tail keeps its thinking if it directly follows the summarized messages, unchanged, and every compaction since that thinking was produced ran on a preserved-thinking model. The opening request goes into the summary; it can't stay verbatim ahead of the block. **[docs: `compaction-on-demand.md`, `compaction-thinking-blocks.md`, `compaction-keep-recent-turns.md`]**

Because these settings sit outside `system`, `tools` and `messages`, they can change between requests without tripping the check. *(Inferred, untested:)* a harness could choose the clearing cut per request (set `keep` so the boundary lands where a finished sub-goal ends), and could use tool names as the unit of what is clearable. And if clearing is recomputed on every request (the docs neither say nor deny it; the client "maintains the full, unmodified conversation history"), sending one request without the edit would bring cleared content back with thinking still valid: a server-side unfold.

**A discrepancy in the docs:** `compaction-on-demand.md` says the summarizer "reads the whole conversation, earlier thinking included, with or without `instructions`"; `compaction-threshold.md` says that on 5.1 and later "a request with custom `instructions` summarizes from the visible conversation only". Untested. It matters for a summary steered to keep the "why".

## 9. What a Claude Code mod can do

From the mod API types (2.1.301) and the reference bundled with the plugin-authoring skill **[src]**:

- **`session.compact` replaces the whole window.** The hook receives the transcript being compacted (its own `instructions` and `messages` rewritable going down) and may return any list of messages. A message passed back with the engine's `handle` "stands as the engine has it", thinking included; one without "is built from its `role`, `text` and tool blocks", so it carries none. `$.session.compact({ instructions })` triggers one from a plugin (`trigger: 'plugin'`); a `precompute` trigger computes one ahead for the next compaction. It works per `agentId` for subagents and forks.
- **So an in-place middle fold is buildable today:** head by handle, a stub the mod writes, tail by handle. Any thinking in the kept tail is then stale: dropped, rejected and stripped by Claude Code's heal path, or read stale on an unenforced account, depending on the served mode and the account (§7). Or the mod rebuilds the tail without handles, which is the docs' "strip thinking from kept turns".
- **Shaping content before its first send is free under the binding.** `session.append` lets a mod rewrite any row as it's stored (tool results, reminders, attachments, hook context; thinking and `tool_use` blocks are put back whole). `tool.call` can rewrite a tool's result; `prompt.attachment` rewrites or drops reminders.
- **Per request, `turn.step` changes only `model` and `effort`;** the messages are fixed.
- **Not reachable from a mod:** `context_management`, `block_binding`, `thread`, `cache_control`, system-role notes (a plugin appends user-role meta rows or notices the model never reads), and turn-scoped messages.
- **Claude Code compacts on the client.** In 2.1.307 the only references to `compact-2026-09-04` are in its bundled API skill's docs. The engine has code for `clear_tool_uses`, gated by a served flag and not plugin-controllable.
- *(Inferred, untested:)* `$.http.fetch` with the session's own credential handle, and `$.model.complete`, would let a mod call on-demand compaction itself, but a `session.compact` result is built from role, text and tool blocks, so a signed compaction block has nowhere to go in the main conversation.

## 10. Message Threads (undocumented)

Claude Code sends the beta `message-threads-2026-08-12`; its code calls the feature "tether" **[src: 2.1.307]**. A request carries `thread: {type: "create"}` with the full history, or `thread: {type: "continue", previous_message_id}` with only the messages after that anchor, leaving out `tools` when the server's copy matches and trimming `system`. Threads expire after an idle lifetime the server sets (`tengu_jaunty_charm`). Server errors include `thread_not_found`, `thread_fingerprint_mismatch`, `thread_already_continued` (each anchor continues once, so a thread is a line, not a tree) and `thread_unsupported_request`. Claude Code falls back to a full request when the anchor was rewritten or the transcript drifted, when the request carries ephemeral messages, tool additions or removals, search results or server tool use, while tool results are being cleared, or after the server dropped thinking the thread produced. It is gated to first-party use (`CLAUDE_CODE_TETHER_LIVE` or `tengu_curious_tower`), and none of Joseph's logged requests carry a `thread` field **[wire]**. Not in the 2026-09-30 docs.

## 11. What this means for middle-folding

*(Inferred, for Joseph's judgment.)*

- **A fold's price is the thinking in its kept tail at fold time,** not the size of the fold (§7). Head thinking stays valid, the folded stretch's thinking goes by design, and thinking produced afterwards binds to the folded history. So a fold that reaches up to the present, made just after a stretch closes and before the next reply, keeps no tail thinking and costs nothing beyond the cache. A fold that leaves recent turns verbatim behind it costs their reasoning.
- **Untested:** every drop Claude Code saw removed a run at the *start* of the thinking sequence. A middle fold leaves a gap between head thinking and new thinking. Blocks produced after the fold should stay valid, since the server already saw the gap when it produced them, but that hasn't been observed. The claude-api skill bundled in Claude Code ships a `drop_block_probe` script that could check it once API credit is available.
- **The 2026-10-19 note's own design points the right way.** Its Invariant 2 (a first-person `<causal-annotation>` in the stub) puts the folded stretch's "why" into text, which the binding can't reach. The turns after the fold still lose their thinking, so only what the agent wrote there in visible text survives. That argues for having the agent annotate its recent turns too, before the fold.
- **A hybrid may avoid the cost entirely for tool-heavy stretches:** the agent appends its own peak/end stub as ordinary text (an append, so valid), then server-side clearing removes the stretch's old tool results (server-side, so valid). Claude Code mods can't set the clearing, so this needs either the API directly or a change in Claude Code.
- **What would settle the cost:** replay a transcript with and without a fold and compare the next decisions (the note's KL(π_full ‖ Q_folded)). Joseph has no API credit at the moment, so this waits.

The proof-of-concept mod and its trajectories are being spiked in `~/src/agent-core/spikes/middle-fold/`.

## 12. What the middle-fold spike found

From `~/src/agent-core/spikes/middle-fold/` (Fable 5.1, after an Opus 5.5 predecessor was stopped by the `reasoning_extraction` safeguard); its `debrief.md` has the detail and `de-novo-feedback-1.md` an independent check. In brief:

- **A mod fold works on the wire.** In one headless process a `session.compact` hook's messages stood with no model call (about 10 ms): 30,017 tokens became 4,379, head, stub and tail verbatim by handle, and the prompt cache was reused up to the cut.
- **The cost is where the cut sits.** The stub is written when the stretch closes (an append, free); the cut can wait for the next compaction (free) or come at the end of the turn that closed the stretch (cache, plus that turn's thinking, whose textual residue is the stub). The kept tail's thinking behind the cut was dropped, as §7 predicts. Whether head thinking survives a middle fold is still untested: no run had thinking before the cut.
- **Mechanics a real mod has to handle:** the hook's view has one row per content block, parallel tool calls split across rows, and unparsed tool input arriving as raw text; `$.session.compact` is refused from `prompt.submit` and in headless sessions (so a turn-end fold works only interactively, via `turn.complete`); and a hook-authored compaction did not survive `--resume` into a new process.
- **The fold site has to be marked.** With a thin unmarked stub, a model confidently denied having run a command that had been folded away; one sentence at the seam saying what was folded and how to recall it turned that into an honest "it was folded out of my context".
- **The safeguard is a design constraint.** Asking a model to report on its own decision process ("which approaches were tried, what settled it") was refused as `reasoning_extraction` in three of five runs; factual questions were never refused. A stub that records what settled a choice can look like that from outside, so the mod now asks for a handover note (decisions, constraints, dead ends) rather than "reasoning".
- **What a fold keeps is not yet measured.** Both fixtures left every fact on disk and the agent re-read it after the fold; the next fixture has to make the folded interior the only copy. "ASF-justified", per the spike's audit of the 2026-10-19 note, honestly means framed and measured with ASF (a fold compresses $M_t$ relative to $\mathcal{C}_t$, judged by retained sufficiency, evaluated with forcing probes), not guaranteed by it.

## Open

- Whether a middle fold's gap leaves thinking produced after it valid (§11).
- What server state paces the reporting of dropped blocks, contrary to the docs' "every response" (§7).
- Whether server-side clearing is recomputed on every request (§8).
- Which docs page is right about custom `instructions` and earlier thinking (§8).
- What Message Threads mean for folding: a continue from an anchor sends only newer messages, so a fold before the anchor would force a create *(inferred)*.
- Why stale thinking is dropped for requests that set no `block_binding`, and on Haiku 5.5 (§4).
