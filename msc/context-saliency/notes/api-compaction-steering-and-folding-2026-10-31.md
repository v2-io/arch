# Steering the API's compaction: chronological summaries, and what it can and can't fold

*2026-10-31, Claude (Opus 5.5), for Joseph. It follows `prefix-bound-thinking-and-folding-2026-10-21.md`, which read the same docs against the 2026-09-30 snapshot. I don't restate that note's §§3–8; this one covers what the 2026-10-30 snapshot adds, and answers today's question. Marks: **[docs]** the docs mirror at `~/src/_ref/claude-docs/docs/en/` as of commit `c2b4485` (run 2026-10-30), compaction and thinking pages read whole as prose; **[ref]** the API reference in the same mirror; **[cc]** Claude Code 2.1.316's binary or its docs mirror; **[cs]** this directory's record; *(inferred)* my reading, not checked.*

Joseph's question, verbatim: *"look at what's potentially available via ~/src/_ref/claude-docs/ in the newest API etc.-- especially combining beta features like docs/en/build-with-claude/compaction-keep-recent-turns.md and doing some highly strategic compactions etc. and immediate strategic-DAG folding etc. I'm curious also if that compaction-keep-recent-turns.md, which needs to be paired with the api's compaction requests, would allow us to influence the compaction summary that comes back-- in particular making it chronological and experientially ordered..."*

## 1. The short answer

- **Keep-recent-turns has no lever of its own.** "No parameter sets which turns are kept. You pick a cut point in your history" **[docs: keep-recent-turns]**. It only decides which messages go into the compaction request. The lever on the summary is on-demand compaction's `instructions`.
- **`instructions` replaces the summarization prompt entirely.** "A non-blank `instructions` string (up to 16,384 characters) replaces that prompt entirely" **[docs: on-demand]**; the reference says the same: "Replaces the server's default summarization prompt for this request" **[ref: `BetaCompactionConfig`]**. So yes: combined, you get a summary of the older span written to your prompt, then the recent turns verbatim, and those turns keep their thinking under three conditions (§3).
- **Chronological order: reachable by prompt.** The summarizer reads the messages in order.
- **Rhythm: not unless the harness puts time into the messages.** The message schema has no time field **[ref: `create.md` message params]**. `transcript.py`'s deltas come from the harness log, so the summarizer can't see any of that. A harness that wants rhythm in the summary has to write a timestamp into each turn *when it first sends it*. That's an append, so the binding allows it; adding one later is an edit.
- **What you can't do with the block.** Edit it after it comes back. Edited blocks are rejected with `compaction_signature_invalid` or `compaction_content_mismatch` **[docs: on-demand, Errors]**. That settles an open item in `~/src/agent-core/anatomy-of-an-agent/sources/provider-apis.md`. You also can't put it anywhere but first, or send two (§3).
- **The summarizer does see earlier thinking, and doesn't pass it on** (measured, §8). On-demand compaction's input grew by exactly the thinking's size with the thinking block present, under the default prompt and under custom `instructions` alike. The summaries left its content out, though. One opened with "I'm leaving my earlier private reasoning out of this summary". The others gave the choice without its reasons. So the "why" that reaches a successor through the summary is whatever was written as visible text at the time, which is the 10-27 notes' practice.
- **The safeguard applies to the summarizer too.** A prompt asking the summarizer to set out its own decision process is the shape that drew `reasoning_extraction` refusals in the middle-fold spike (3 of 5 arms). On a compaction request a refusal means no summary: `stop_reason: "refusal"`, "Continue without a summary" **[docs: on-demand]**. Refusals of this category that arrive before any output are billed **[docs: refusals-and-fallback]**.

## 2. What the docs say now that they didn't on 09-30 or 10-22

Changes between the 10-22 and 10-30 snapshots, found by diffing with quote style and model-list order normalized away. Most of the churn was typographic.

- **A deliberate one-time drop is now called cheap.** "A one-time drop that you make on purpose, such as at a client-side compaction boundary, costs little. A drop on every turn costs more" **[docs: preserved-thinking, new sentence]**. This lowers the stakes of a client-side fold made at a boundary. That is the 10-27 synthesis's point (§5): "Boundaries become the cheapest place to forget twice over."
- **Enforcement is restated as a model × account-age table.** "The prefix check always runs on these models." With `prefix_mismatch_behavior` unset:
  - Fable 5.1, Opus 5.5, Sonnet 5.5 and Haiku 5.5 give a 400 on accounts created from 2026-08-31, and on older accounts deliver the failing block flagged as `thinking_mismatch_allowed`.
  - Mythos 5.1 now gives a 400 in both columns. The 10-22 text said Mythos 5.1 doesn't run the check.
  - The middle-fold spike's wire observation (a silent `thinking_dropped` with no `block_binding` set) still contradicts this table. It stays open.
- **Haiku 5.5 is listed among the checked models.** The 10-21 note §4 says the docs didn't list it; that was the 09-30 snapshot.
- **The block carries more than its readable summary.** **[ref]** It has `encrypted_content`, "Opaque metadata from prior compaction, to be round-tripped verbatim". With `inline-tools-2026-09-15` it also has `tool_changes`, recording the net effect of mid-conversation tool changes. *(Inferred)* What the successor receives may be more than the `content` text, so judging a summary by reading it may miss part of what it does. Nothing in the docs says what `encrypted_content` holds.
- **Unchanged, and probably not a contradiction.** On-demand: "The summarizer reads the whole conversation, earlier thinking included, with or without `instructions`." Threshold: "On Claude 5.1 and later models, a request with custom `instructions` summarizes from the visible conversation only: earlier thinking blocks are not part of the summarizer's input." They describe different features. The on-demand statement now has a measurement behind it (§8). The threshold statement is untested.
- **Anthropic's own recommended compaction prompt** (in the Fable 5.1 prompting guide, for client-side summaries) ends: *"Weight the two voices differently: keep what the user said, asked for, shared, or established carefully and close to their own words; your own explanations and reasoning can be condensed much further, to what they concluded or produced"*. That is P7's verbatim floor, arrived at independently. The same guide: *"Compacting early to save cost can be the wrong tradeoff between cost and intelligence, so experiment with later compaction points."*

## 3. What the mechanism can fold

**It folds prefixes only.** The block goes "first in `messages`, either as an `assistant` message of its own or as the first content block of the first message", with "exactly one `compaction` block in each request". "Compact again" makes a new block covering the old block plus everything after it **[docs: on-demand]**. So server-side, a fold always covers everything up to the cut. A true middle fold (opening verbatim, interior folded, tail verbatim) is still client-side only: "Removing individual turns invalidates every thinking block after them, and no compaction scheme avoids that" **[docs: preserved-thinking]**. The middle-fold spike's design-space row E already has this right.

**The kept tail keeps its thinking only if three conditions hold** **[docs: compaction-thinking-blocks]**. All of them are about the seam:
- every compaction since the thinking was produced ran on a model with preserved thinking;
- the kept turns directly follow the summarized messages, unchanged, and the first kept message has a different role from the last summarized one;
- `system` and the non-deferred `tools` are unchanged.

The docs' recommended clean cut is to "compact exactly the `messages` of a request you already sent. The kept turns then start with Claude's reply to it." "A system message placed between the block and the kept turns breaks their thinking."

**Where strategic-DAG folding fits.** *(Inferred, from the above and the 10-27 three clocks.)*
- **The event clock.** When a sub-goal closes, compact exactly the messages of the request in which it closed. Fire it as *background* compaction, so work carries on and the turns that arrive meanwhile are kept with their thinking valid **[docs: compaction-background]**. That is the immediate fold, made at the boundary, with no pause. Every completed sub-goal so far goes into the block, and the live branch stays verbatim.
- **This suits sequential work and suits interleaved work badly.** A side investigation still open when the main sub-goal closes gets folded with it. Only an open tool call blocks the cut: the API rejects a compaction request whose last assistant turn has a `tool_use` with no result yet.
- **Under a total time order, contiguous prefixes are always honest about cause and effect.** That is the 10-27 synthesis's §7 convexity result, and a prefix is the simplest contiguous interval. The server-side fold can't violate it; a middle fold can.

**Material can only be added after the tail.** The layout is [block][kept tail][new user turn]. A seam note, Joseph's own words, or a "read these files whole first" instruction goes *after* the tail as the next user turn: an append, which is free under the binding. The project-heat second seam is the one positive case of this kind **[cs: H&F Part II]**. Instructions that were stated inside the summarized span "stop applying" and need restating "in a `role: "system"` message directly after the first new `user` turn that follows the kept turns" **[docs]**.

**A verbatim head by request.** The block can't keep the opening request verbatim, but `instructions` can ask the summarizer to quote it, and Joseph's turns, word for word. The result is model-written, so check it *mechanically*: every quoted span should be an exact substring of the source messages. `asf-transcribe` already does this check for reader quotes ("Across ten readings, 420 quotes verified, 24 were cited against the wrong event, and none was invented", per `strategy-dag-engineering.md` §5.6). The block can't be patched, so a failed check means accepting the summary with the failure noted, or requesting a new one. Either way the failure is known rather than silent.

**Things the docs leave unstated** (testable; §6):
- `compaction` and `context_management` can't share a request. Whether a later request may carry the block *and* `clear_tool_uses` edits isn't stated.
- Whether a compaction request reads the conversation's prompt cache. If it does, a fold at every sub-goal boundary is cheap. If not, each fold costs the whole span (old block plus everything since) at full input price. Nothing in the docs answers this.
- Whether the summarizer can be a different model from the conversation's. On-demand is a separate request. Condition 1 only requires a preserved-thinking model, and "one way to meet it is to send every compaction request to the model the conversation uses" implies others may be possible. A Fable 5.1 summarizer for an Opus 5.5 conversation is plausible: Fable 5.1 reads Opus 5.5 thinking on the API. Whether the API accepts that block, and whether Opus 5.5 reads it, is untested.

## 4. A chronological summary: what to test

Theory §9's structure-within-chronology (SwC) has never been produced **[cs: H&F Part III]**. The API makes a server-written version possible, with the recent stretch verbatim and its thinking intact. Two design points, untested:

**Make "compact again" accumulate instead of re-digest.** By default each compaction summarizes the previous summary plus what follows, which is the summary-of-summary chain that drifts toward the summarizer's prior (iterated learning, per the 10-27 vocabulary map). Because `instructions` is yours, each compaction can say: the prior block is a chronological log; reproduce it unchanged and continue it.
- The old block's `content` is readable, so whether the summarizer reproduced it is a *deterministic diff* of the new block's head against the old.
- Costs: `max_tokens` must grow with the log, because it "caps the whole call, including any thinking". The kept-thinking condition rules out a cheaper non-preserved-thinking summarizer.
- **A hard ceiling.** One compaction's output can't exceed the model's maximum output: 128K tokens synchronous on Opus 5.5 and Fable 5.1 **[docs: `models/opus-5-5/overview.md`]**, shared with any thinking the summarizer does. An accumulating log therefore tops out somewhere below that, and then has to be folded itself. A 50k-token log fits; a log that grows without bound doesn't. Separately, the request fails with `model_context_window_exceeded` when the conversation plus the prompt leave no room.
- P10 still applies. A prompt that asks for a shape is not evidence the shape comes back.

**Write the instructions in handover register.** Decisions, constraints, dead ends with what failed, attempt counts, pointers. No "your reasoning" and no "why you chose", which is the `reasoning_extraction` shape.

**Keep the system prompt out of the user's voice.** The kept-thinking conditions make the compaction request carry the conversation's own `system` and `tools`, so the summarizer reads all of the system prompt; here that is the 1,500-line stopgap. The record has already seen what follows: "standing constraints" sections filled from the system prompt and attributed to the user, then carried forward across compacts (n=2 sessions, **[cs: H&F Part II]**). The instructions need to say where constraints may come from.

A draft for a test arm follows. It assumes the harness stamped a time on each user turn when it sent it. It is not a recommendation, and it's well under the 16,384-character cap:

```text
Write a chronological log of the conversation above, for the instance that will continue it. Open with one line saying what it is: a log written from the conversation, shortened where marked, with the full transcript kept by the harness.

Time is the spine. Go through the conversation in order and record each stretch as it happened: what was asked, what was done, what was found, what was decided.

- The user's messages: quote each in full, in order, with the time it carries. If you must shorten one, quote its opening and say how much you cut.
- Place each constraint, preference, decision, correction and open question at the point where it arose or changed, in the words it was stated in, with its status at that point (proposed, tried, settled, reversed). When a later turn changes its status, record that at the later point; don't move it earlier.
- Take constraints and preferences only from the messages of this conversation, and say whose message each came from. The system prompt and tool definitions will be present again after this log; don't restate them, and don't attribute them to the user.
- Keep dead ends where they happened: what was tried, how many attempts, what failed.
- Fold routine runs of tool calls into one line each: what they were for and what came of them, with counts.
- Mark every place you shortened or left something out: [omitted: what, about how much]. A named omission is fine; a hidden one is not.
- If the conversation begins with an earlier log of this kind, reproduce it unchanged and continue it.
- End with where things stand, what is open, which files the next instance should read whole before acting, and what it will need that those files won't tell it.

Don't call tools. Respond with the log only.
```

**The deterministic alternative is now officially cheap.** The docs call "simple compaction" the recommended client-side shape: one summary message plus the next instruction, no thinking replayed. "Claude models are trained on long-horizon tasks with this scheme", and a deliberate one-time drop "costs little" **[docs: preserved-thinking]**. A `transcript.py -v`-style log used *as* that summary message is the Claude-side form of Joseph's 50k-token-log bet: deterministic, verbatim, with no summarizer to settle anything. It loses the recent turns' sealed thinking, which the server block keeps. Comparing the two is the experiment the record says is missing (H&F Part IV: "Any controlled comparison of log vs digest").

## 5. Claude Code today

- **2.1.316 doesn't use on-demand compaction.** It contains no `compact-2026-09-04` string at all. The bundled SDK tool runner has `compactBeforeNextTurn` (`{type:"summarize"}`), but the beta value it would need is absent, so the main loop doesn't send it as far as a string search shows **[cc: binary]**. As of the 2.1.301 mod-API types, mods can't reach `compaction` or `context_management` (10-21 note §9); I didn't re-check the types for 2.1.316. A `session.compact` hook *can* return a log-as-summary, which is simple compaction. That is candidate-honest-compaction slice 2 **[agent-core sidecar notes]**.
- **The Rewind menu already does steerable partial compaction by hand. None of the folding notes mention it** **[cc: docs `checkpointing.md` §Rewind and summarize; changelog]**:
  - **"Summarize from here"** (2.1.32, 2026-02-05) compresses everything from a chosen prompt onward.
  - **"Summarize up to here"** (2.1.141, 2026-05-13) compresses everything before it and keeps later messages.
  - Both take guidance typed into "add context (optional)".
  - *(Inferred, unverified)* "Summarize from here", chosen at the prompt where a just-finished drill-down began, is a subtree fold that keeps the head verbatim. Its head is unchanged, so its thinking should stay valid, and nothing is kept after the summary, so no tail thinking is at risk.
  - "Summarize up to here" is client-side keep-tail, so per the docs it drops the kept turns' thinking.
  - These are Claude Code's client summarizer, not the API's signed block. Still, they're the only strategic folds Joseph can make today without new code.

## 6. Tests, cheapest first (asks, not runs)

T1 and T2 have since been run; results are in §8. The handoff's standing rule is no API replays unless Joseph asks, and the 10-21 note recorded no API credit. On funding: Anthropic's bundled `drop_block_probe.py` (`~/src/agent-core/ext/skills/skills/claude-api/shared/preserved-thinking-migration/`) accepts an OAuth bearer token as well as an API key. Whether compaction requests may run on a subscription token is Joseph's call; I don't know the terms.

**Tried, same day: the subscription token doesn't reach the API from outside Claude Code.** Claude Code's OAuth access token (Max, `user:inference` scope, unexpired) authenticated: the response was a 429, not a 401. It was refused for a one-line Haiku 5.5 request with `{"type":"rate_limit_error","message":"Error"}`, twice (`req_011CgbEQewuG3coVvmQx5tzi`, `req_011CgbERkmRnU18cG7DRBWFy`). The responses carried no `anthropic-ratelimit-*` or `retry-after` headers, while this Claude Code session ran normally on the same account. That reads as a gate on subscription inference outside Claude Code, not as a usage limit *(inferred)*. Making the request imitate Claude Code would route around that gate; we didn't. So the tests need an API key. The proof-of-concept script for T1 and T2 is written and compile-checked but not run: `../api-compaction-poc/poc.py`. Run it with `ANTHROPIC_API_KEY=… python3 poc.py runs/02-poc`. Every request and response is saved whole under `runs/`, and the key is never written. `runs/01-smoke/` holds the refused OAuth request.

- **T1. Does a summarizer with custom instructions see earlier thinking?** Use a short conversation in which one consideration appears only in a thinking block. Request that turn with `display: "summarized"` so we know what the thinking held. Compact it twice, with and without `instructions` that ask for decisions and what settled them. Check whether the consideration reaches the summary. The run also measures refusal frequency on compaction calls. A few requests; cents.
- **T2. The docs' own kept-thinking check.** Compact with a kept tail, then send the next request with `prefix_mismatch_behavior: "error"` **[docs: compaction-thinking-blocks §Check]**. Add a third request that carries the block plus `clear_tool_uses`, to settle that open item. Cents.
- **T3. Additive log.** Compact again under the "reproduce and continue" instruction, then diff the new block's head against the old `content`. Two compactions of a short conversation.
- **T4. SwC versus the alternatives, on a real Claude Code session with a known baseline.** `opus-5.5-summary-example.md` is Claude Code's own Opus 5.5 summary of limen session `617b0fe9`, and its pre-compaction JSONL is on disk. The arms:
  - (a) the API's default summary;
  - (b) Anthropic's Fable 5.1 preservation prompt;
  - (c) the SwC draft above;
  - (d) the deterministic log as a simple-compaction message.

  Score them with the record's informal measures: claims settled in the summary that the log shows open, verbatim floor, named holes, and a post-seam forcing probe. Costs about the session's input size per arm.

  **Replaying with the original system prompt may be possible.** The limen transcript carries `prompt_snapshot` attachments: the system prompt as 13 blocks and the 15 tool definitions, a pair at session start and a pair within seconds after each of its three `compact_boundary` rows **[cc: transcript, checked 2026-10-31]**. If they reproduce the wire bytes, the replayed thinking validates, and T1 can run on this real session instead of a synthetic one. Whether they're byte-exact is untested. The bundled `prefix_diff.py` and `drop_block_probe.py` are built to settle exactly that. If they aren't exact, use `drop_block`; that still works for comparing summary shape.

## 7. What this note rests on

- **Read whole, code samples included:** the six compaction pages (threshold first with non-Python samples stripped, then again in full).
- **Read whole as prose, non-Python code samples stripped:** preserved-thinking, mid-conversation-system-messages, context-editing, thinking, refusals-and-fallback, prompt-caching (§§ through "Troubleshooting"), and the Fable 5.1 and Opus 5.5 prompting guides.
- **Also read whole:** the `compaction` parameter and block in the API reference; the 10-22 → 10-30 diff of those pages; the two compaction cookbooks (`misc-session-memory-compaction`, `tool-use-automatic-context-compaction`). The cookbooks were extracted from the mirror's HTML, which is the only copy. The session-memory cookbook's background summarizer reported a cache hit when it reused the conversation's prefix. That was a plain messages call, not the `compaction` parameter, so the §3 cache question stays open.
- **In this directory, read whole:** handoff, COMPACTION-DESIGN-THEORY, H&F 09-24, the Opus 5.5 summary specimen, the 10-21 note, the four 10-27 notes, and the 10-19 middle-folding note through Part IV §2 (lines 1–479 of 692).
- **Checked by script, not read:** the row types of limen transcript `617b0fe9` and where its `prompt_snapshot` and `compact_boundary` rows sit; the 2.1.316 binary for the compaction beta strings.
- **In agent-core, read whole:** the middle-fold spike's `debrief.md` and `notes/design-space.md`; the sidecar's `strategy-dag-engineering.md` and `candidate-honest-compaction.md`; `anatomy-of-an-agent/sources/provider-apis.md`.
- **Read later the same day, whole:** the rest of the 10-19 note (to line 692), the jsonl-chronica and agy-compaction notes, and the grok-build upstream-compaction note. The 10-19 note's "Miss 4" (constraints only from user rows) and "Miss 2" (the commit-envelope fold keeps the end and drops the peak) reach §4's points first.
- **Read raw and whole after Joseph asked whether they had been read** (one instance's account): context-editing, which had first been read with code stripped. A script then found five TypeScript/Ruby-only tabs on that page; the stripped read had lost them. They cover the deprecated SDK `compactionControl` (a separate summarizer `model`, `summaryPrompt`, Ruby's `on_compact`) and change nothing above. The same check found every code group in compaction-threshold, preserved-thinking and mid-conversation-system-messages has a Python sample, so stripping removed only translations there. Also read for the first time then: context-windows, which independently confirms the 128K per-request output ceiling and that `model_context_window_exceeded` stops generation that reaches the window; and agents-and-tools/tool-use/manage-tool-context. Its point for folding is programmatic tool calling, whose "intermediate results never enter the conversation history": tool runs that never need folding at all.
- **Not read:** REFLECTIONS, the dossier, PLAN, GOAL, paper-readings, the grok-build thinking-opacity note, the asf-citation-audit (I have its conclusions only through the debrief), and the middle-fold spike's README, de-novo feedback and integration plan. The 10-19 note's "guaranteed" and "theorem" language is stronger than ASF supports, per that audit; nothing above leans on it.

## 8. Proof of concept, run 2026-10-31

Run with an API key. Every request and response is saved whole under `../api-compaction-poc/runs/`. Model: Opus 5.5, with `display: "summarized"` so turn 1's thinking could be read, and `prefix_mismatch_behavior: "error"` throughout. One toy conversation (a three-way storage choice answered with a single letter, then a follow-up), n=1 per cell. Thirteen requests, a few thousand tokens in all.

- **Kept-turn thinking survived the swap** (`03-poc`). Turn 1 was compacted and turn 2 kept, thinking included, behind the block. The next request ran under `"error"`, returned 200, and had an empty `input_transformations`. The docs' own check passed.
- **The summarizer's input includes earlier thinking, with or without custom instructions** (`04-thinking-input`). Each arm compacted the same turn twice: with its thinking block, then with the block removed (removal is a valid edit). Default prompt: 270 vs 173 input tokens. Neutral custom instructions: 247 vs 150. The difference is 97 tokens in both, and turn 1 produced 100 output tokens, almost all of it thinking.
- **None of the summaries carried the thinking's content.** The default summary of the thinking-bearing turn opened "I'm leaving my earlier private reasoning out of this summary"; the others stated the choice (A) without its reasons. The first default summary said "No rationale was given in the reply." Reading from the summary, the successor gets only what was written as visible text.
- **Custom instructions that asked for "the considerations that settled" each decision were refused** (`03-poc/04`): `stop_reason: "refusal"`, `category: "reasoning_extraction"`, 313 input tokens billed, no summary. A neutral custom instruction ("Summarize the conversation above for whoever continues it…") was not refused. Instructions for a chronological summary need the handover register in §4.
- **The successor recovered the reasons by reasoning again, and said so.** Asked "Which option did we choose at the start, and why?", it answered "A, SQLite. I didn't explain it then, but here's why:" and gave reasons matching the turn-1 thinking summary. That makes it a reconstruction labelled as one, not a memory of the reasoning.
- **Blocks carry `type`, `content` and `signature` only.** No `encrypted_content` appeared on any of the five returned blocks, so the §2 question about it stays open for longer or tool-using conversations.
- **The signature grows when the summarized span held thinking.** It was 176 characters longer in both arms: 1,740 vs 1,564 (default prompt), 1,788 vs 1,612 (neutral instructions). That fits the signature recording something about the summarized thinking, such as the link later thinking binds to *(inferred, unexplained)*.
- **The session that ran these could not compact itself.** It ran out of context before discussing these results, and Claude Code's compaction of it failed. Joseph's reading: "it wouldn't compact your own context because it probably triggered the reasoning_extraction trigger accidentally." He rewound the session to a point after the runs, which is why §7 holds reads made in a branch that no longer exists. The session transcript (`~/.claude/projects/-Users-josephwecker-v2-src-agent-core/ad11dbfd-6714-4d15-bc97-9dcd10d93e44.jsonl`) holds no compaction, error or refusal row, and there is no debug log, so the cause is unconfirmed. If he's right, the hazard isn't confined to compaction prompts that ask for reasoning. A conversation *about* reasoning extraction, quoting refusal text and prompts that asked for considerations, may be enough to trip the summarizer, and the result is a long session with no seam at all. The only exit was a rewind, which cost everything after the rewind point.
- **OAuth route.** Recorded in §6: the subscription token is refused outside Claude Code (`runs/01-smoke`).

Still open from §6: whether compaction requests read the conversation's prompt cache (these requests set no `cache_control`); whether a later request may carry the block plus `clear_tool_uses`; the additive-log instruction (T3); the comparison on the limen session (T4).

## 9. Putting the pieces together inside Claude Code (2.1.316)

*Joseph's question: can tool-result clearing, early folding, direct compaction requests and preserved thinking be combined, via the SDK or the mod system, all within Claude Code? Read in the 2.1.316 mod API types (`plugin-authoring/types/claude-code.d.ts`, the sections named) and the docs above.*

**What a mod can reach** **[types]**:
- **Fold at a boundary.** `session.compact` gives a hook the transcript with each message's engine `handle`, and the hook may return any list. A message kept by handle "stands as the engine has it" (its thinking included); one without "is built from its `role`, `text` and tool blocks". `$.session.compact({ instructions })` triggers one, and `precompute` prepares one in advance.
- **Call the API itself, with the session's credential.** `$.session.authorize()` returns an opaque handle for the session's Anthropic credential (`bearer` or `api-key`), spent only through `$.http.fetch(url, { auth: handle })` to a first-party host. This is new since the 2.1.301 types the 10-21 note read. `$.session.messages({ as: "api" })` returns the conversation in API form, "thinking as it goes back", but not the system prompt or tool list. So a mod can send an on-demand compaction request with its own `instructions` over the real conversation, thinking included.
- **Shape tool results before their first send.** `tool.call` and `session.append` can rewrite a result as it arrives, which is binding-safe because nothing has been sent yet.

**What a mod can't reach:**
- **A signed compaction block.** Session messages are role, text and tool blocks, plus a handle; there is no block type for a compaction. A summary fetched from the API can enter only as text, without its signature, so the swap counts as a client-side edit and the kept tail's thinking is dropped (§3).
- **`context_management`, `betas`, `block_binding`.** None appears anywhere in the mod API. `turn.step` rewrites only `model` and `effort`; the messages are pinned. The engine has its own `clear_tool_uses` code (strings present in the 2.1.316 binary), gated by served config (10-21 note §9).
- **The `instructions` of the engine's own summarizer, fully.** `session.compact`'s `instructions` is "what the summary should keep or stress", added to the engine's prompt, not replacing it. A full replacement takes the `$.http.fetch` route.

**So inside Claude Code today, the combination is approximate, and its one hard cost is known.** A mod can fold at sub-goal boundaries, have the stub written by the server summarizer under our own prompt (an SwC log, §4), mark the fold site, and shape tool results on arrival. Everything it installs is client-side, so the thinking of whatever it keeps after the cut is dropped. Cutting at the end of the turn that closed a stretch keeps that tail near empty, and the docs now call such a one-time drop cheap (§2). Server-side tool clearing and signed blocks with valid kept thinking stay outside Claude Code.

**The full combination exists outside Claude Code, in the SDK tool runner** **[docs: compaction-on-demand]**: `compact_before_next_turn()` sends an on-demand compaction "once the current turn and its tool calls complete", leaving `context_management` out of that one request and sending it again afterwards. It refuses only a compaction edit, so `clear_tool_uses` and on-demand compaction coexist. It replaces the whole history (no kept tail); keep-tail and background compaction need a hand-written loop. That harness has none of Claude Code's tools.

**Untested, in order of cost:**
- whether `$.http.fetch` with a `bearer` handle reaches `/v1/messages` (the direct OAuth request in §6 was refused; this is the engine's own sanctioned route, so the gate may treat it differently);
- whether a request that carries a signed block also accepts `clear_tool_uses` (the runner's design implies yes);
- whether the summary a mod installs as text reads as well as the engine's own, judged by the forcing-probe method of the middle-fold spike.

**Probe 1 result (2026-10-31, `auth-probe` mod, `runs/05-mod-fetch/`).** Through `$.session.authorize()` + `$.http.fetch`, the session's subscription credential (`bearer`) got the same `429 rate_limit_error "Error"` as the direct OAuth request in §6, for a one-line Haiku 5.5 request, before and after a re-login. So the engine's sanctioned route doesn't get past the gate on subscription inference either. The on-demand compaction request came back `400 "compaction: this parameter requires anthropic-beta: compact-2026-09-04"`, although the module had sent `anthropic-beta: oauth-2025-04-20,compact-2026-09-04`. So `$.http.fetch` with `auth` appears to replace the caller's `anthropic-beta` header *(inferred; tested next without the handle)*. The requests ran as org `50a00c79-…`, workspace `wrkspc_01RqJx…`; the API key file belongs to Vis Veritatis (`230fd125-…`, `wrkspc_01DeJ5q…`), checked with a free token count. The mod's next mode, `/auth-probe key`, sends that key itself as `x-api-key`, with no handle.

**Probe 2 results (2026-11-01).**
- **`--betas compact-2026-09-04` changed nothing on the session route** (`…T01-37-53-389Z-session/`): same 429 and same missing-beta 400. That matches the CLI reference, which says the flag is for API-key users only. The env-vars page names `ANTHROPIC_BETAS` as the variable that works with subscription auth; it is untried here, and it would apply to Claude Code's own requests in any case.
- **The key route works from inside a mod** (`…T01-39-48-162Z-key/`). `/auth-probe key` reads `~/anthropic-default-api-key` and sends it as `x-api-key`, with no handle. Both requests ran as Vis Veritatis (`230fd125-…`, `wrkspc_01DeJ5q…`).
  - The plain request: 200.
  - The on-demand compaction: 200, `stop_reason: "compaction"`, a block with `type`, `content` and `signature`. 143 tokens in, 183 out, Haiku 5.5.
  - The `compact-2026-09-04` header survived on this route. That confirms the handle route replaces the caller's `anthropic-beta`.
- **Voice of the summary.** Haiku 5.5's summary is in the assistant's first person ("I suggested three core entities…"); the Opus 5.5 summaries in §8 were third person ("the assistant answered A"). n=1 each. It bears on the seam: a first-person summary invites the successor to take it as its own memory (theory §1, the identity question in the 10-19 note).

**So a Claude Code mod can request server-written summaries under its own `instructions`, using an API key it supplies itself.** What it still can't do is install the signed block (§9).

**Fold preview on this session (2026-11-01, `runs/06-fold-preview/2026-11-01T01-54-59-757Z/`).**
- `/fold-preview` exported the session in API form: 272 messages, 2.35M characters, thinking blocks included, and 2 advisor calls replaced by a labelled note. The advisor results are encrypted under the Claude Code login's org and the key's org can't process them: 400 "Advisor tool result content could not be processed".
- `send_stream.py` sent it on the Vis Veritatis key (Opus 5.5, §4 instructions plus a third-person line). It returned `stop_reason: "refusal"`, category `reasoning_extraction`, after 121 s, 728,444 input and 12,352 output tokens. No summary came back; about $3.15 was billed.
- All 175 thinking blocks reached the model, flagged `thinking_mismatch_allowed` (this account sits in the before-2026-08-31 column).
- A first attempt through `$.http.fetch` hit that call's 30-second limit, and was probably billed with no result. `$.http.fetch` can't carry a long summary request.

This fits Joseph's reading of the earlier failed compaction: a session about this subject can't be summarized by the API under this safeguard, whatever the prompt asks.

**First structure-within-chronology summary (2026-11-01, `runs/07-limen-first-span/`).**
- **What was run.** `build_from_transcript.py` rebuilt limen session `617b0fe9` up to its first `compact_boundary` from the JSONL: 687 messages, all 346 thinking blocks with their signatures, 325 tool calls, 3 images. The session's own `system` (13 blocks) and 15 tools came from its `prompt_snapshot`, where `schema` is the wire tool definition. Free count: 894,912 tokens. `send_stream.py` sent it with the §4 instructions plus the third-person line. The result: `stop_reason: "compaction"`, 259 s, 895,422 in and 27,512 out, about $4.10. All 346 thinking blocks were delivered (`thinking_mismatch_allowed`; the rebuild isn't byte-identical).
- **What came back** (`summary.md`, 6,844 words), against Claude Code's own nine-section summary of the same stretch (`opus-5.5-summary-example.md`):
  - It is chronological: twelve stretches in time order, with structure placed where it arose ("Constraint stated here (user)", "Decision (user)", "status at this point: proposed") and the end sections last. This is the SwC shape of theory §9, which H&F records as never produced before.
  - The user's 14 messages are quoted in full; the 11,000-word paste is cut with its opening quoted.
  - Dead ends and corrections stay in place: the `execFileSync` starvation, "First hypothesis (partly wrong)", the botched stash-verify, the wrong "c94…" sha. It writes in the third person and names its omissions.
- **Faults.**
  - The rebuild dropped attachment rows, so mid-turn user messages were missing. The summarizer reported them as "text not visible" rather than inventing them: an honest hole, but one the export made.
  - It ran short of the length target (6,844 words against 12,000–18,000).
  - There is no rhythm, because the messages carry no times.
  - It counted the compaction request as user message 15.
- **Next.** Include attachment rows (queued and mid-turn messages) in the rebuild, and stamp each user turn with its transcript timestamp. Both are possible from the JSONL, which has a time on every row. Then compare post-seam behavior using a forcing probe, not by reading.
