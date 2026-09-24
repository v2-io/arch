# grok-build upstream: what changed in compaction, 2026-08-15 → 2026-09-23

*An Opus 5.5 instance's reading of the upstream diff, done while rebasing the compaction fork. The range is `5163763e` (the fork's sync base, 08-15) to `upstream/main` `f0e3be11` (09-23). It spans 18 "Synced from monorepo" commits that carry no messages, so the diff is the only primary. The whole range touches 3,522 files; about 90 of them are compaction-related.*

**Read:** the non-comment diff lines of:
- `session/compaction.rs`
- `helpers/session_compact.rs`
- `helpers/compaction_context.rs`
- `helpers/full_replace_compaction.rs`
- `xai-chat-state/compaction_mode.rs`
- `xai-compaction-transcript/lib.rs`
- `compaction_config.rs`
- `compaction_segments.rs`

Also read:
- `compaction_utils.rs`, at the level of symbols plus the full text of `format_transcript_location` and `AUTO_CONTINUE_PROMPT`;
- the goal-pin functions in `acp_session_impl/goal.rs`;
- the doc headers of the new `memory/capture_transcript.rs` and `compaction_image_context.rs`.

**Not read:**
- `reminder.rs`, `code_compaction/failure.rs`, `sample.rs`, `intra_compaction/*`;
- recap / `summary_write.rs`, telemetry events;
- the goal-mode template bodies (line counts only);
- the tests.

## What did not change

- **The summarization instructions.** All five compaction templates are byte-identical:
  - `full_replace_summary_prompt.txt`
  - `compaction_developer_prompt.txt`
  - `compaction_user_prompt.txt`
  - `intra_compaction_system.txt`
  - `intra_compaction_user.txt`
- **The shell's own inline 9-section prompt**, in `build_compaction_prompt`. It still reads "6. All User Messages: List ALL messages from the user…".
- **The segment-store pointer text** shown to a successor: "Full verbatim rollouts of previous segments are available at {loc}/segment_*.md … if this summary is insufficient."
- **`AUTO_CONTINUE_PROMPT`**, which is unchanged. It is only referenced in order to *filter* legacy items, and no current code pushes it.
- **The static-shell capture's `\x01` markers.** This is the bug behind `(eval):23: unmatched '` (below).

None of the compaction-design-theory fixes appear upstream: the honesty banner, Known Holes, verbatim floors for the user's words, pinning of user-stated constraints, the non-completeness retitle, or addresses into the transcript.

## What changed

1. **Default compaction mode: `Summary` → `Segments`.** Every compaction now persists the verbatim pre-compaction rollout as `compaction/segment_NNN.md` plus an `INDEX.md`, and the per-segment cap rises from 512 KB to 5 MB. The successor gets the pointer above by default.
2. **Two-pass loses its own prompt.** The five-section two-pass instruction is deleted; pass 1 and pass 2 now call the same `build_compaction_prompt` as single-pass.
3. **Structured live state is re-injected around the summary.** The post-compaction system reminder, `CompactionStateContext`, gains:
   - pending scheduled loops (interval, next fire, prompt);
   - running workflow runs (objective, current phase, agents used against budget, elapsed), plus the list of available workflows;
   - a goal section (objective, status, tokens, elapsed, next step from the plan file);
   - v2 memory manifests ("MEMORY_COMPACTION_RECOVERY");
   - the MCP hint, pushed again;
   - image state: the last user turn's image parts, and up to 32 earlier image paths, verified to exist in the session's `assets/` directory.
4. **Goal pin across the seam.** Goal mode folds this into the compaction's user context:

   > "Objective: {objective}\nDo not restart this goal or revive a prior unrelated task. Continue from the current next step."

   It then re-seeds a goal-continuation message after the compaction.
5. **Input robustness.**
   - The verbatim → verbatim-fitted → lossy fallback ladder is lifted into named stages, and a compaction starts at "fitted" when the estimate is over budget.
   - Image budgeting and request-size pruning (`max_request_bytes`) apply to compaction input.
   - A new `Overflow` failure class separates context-length and payload errors from deterministic ones.
6. **Failure UX and telemetry.** Errors carry structured `{kind, message}` data, users see error messages with the internal prefix stripped and retry guidance attached, and there are timing and outcome events for compaction.
7. **A switch of model family triggers a compaction**, with its own banner.
8. **Memory extraction (new `capture_transcript.rs`).** When the extractor's input is too large it degrades in stages: "reasoning and images, then tool bodies, then the middle of the tool loop, then the user's own words." The user's words go last.
9. **Goal-mode templates cut sharply:** the planner −166 lines, the verifier −135, and the rules and strategist also trimmed. Not read.

## How it sits against the theory [mine; interpretation, not established]

Upstream answers post-seam disorientation with **harness-held live state**: goals, workflows, loops, memory, and images, pinned around the summary. The goal pin's wording ("do not … revive a prior unrelated task") names the same failure the project-heat successor showed.

On the other side of the theory's split, upstream does two things:
- It retains the verbatim record by default (Segments), which is the theory's "preservation is A + index".
- It leaves the summary's epistemics untouched. The template still asserts completeness in places, and the pointer to the record is framed as a fallback ("if this summary is insufficient").

Converging with the theory: retaining the record, and pinning state the harness can see structurally. Not addressed: false completeness, settling, keeping the user's words verbatim, and the relational objective.

## Rebase notes

`compaction-design-theory` is rebased onto `f0e3be11`, with the fork's six commits on top:
- the test import;
- tiers 0–1;
- tier 2;
- P10;
- no-silent-middle-cuts;
- the static-shell marker fix.

**Conflicts.** One was a doc-comment collision in the test import. The rest fall into three kinds:
- **Duplicate parameters.** Both sides added a parameter to `build_compaction_chat_history`. The merge keeps both, in the order `max_request_bytes, transcript_available`.
- **The deleted two-pass prompt.** Upstream deleted the two-pass prompt that the fork had extended with Known Holes. Two-pass now inherits the fork's template through `build_compaction_prompt`, which delegates to the crate. The two two-pass call sites pass `self.transcript_hint().is_some()`.
- **The inline prompt.** Upstream kept the shell's inline copy. The fork's delegation to `build_summary_prompt_with_transcript` replaces it.

**Survived as auto-merged:** the fork's P10 truncated-summary notice and the transcript-addressing wiring in `compaction.rs`. The pre-rebase branch is kept as `compaction-design-theory-pre-rebase-2026-09-24`.

**Tests on the rebased tree.**
- `xai-grok-compaction` 140/140.
- `xai-chat-state` 391/391.
- `xai-grok-tools` static-shell 6/6, plus the live rc test (ignored by default).
- Shell `compact` 217/217 with a 32 MB test stack, identical to upstream plus the one-line test import.

Upstream's own test issues:
- Its lib tests don't compile here without `use base64::Engine`, which is what the fork's first commit adds.
- Under the default test stack, `auth_retry_budget_tests::parked_turn_past_compact_threshold_does_not_auto_compact` overflows and aborts the run.
- `compaction_paths_note_survives_second_compaction_and_filters_missing_files` flakes in parallel runs (1 of 2) and passes alone. Its own cleanup finds the file already deleted.

## The `(eval):23: unmatched '` root cause

The static-shell capture wraps `alias -L` plus `typeset -f` in `\x01` markers and keeps `split('\x01')[1]`. zsh-vi-mode's `zvm_switch_keyword` compares `$keys` to a literal Ctrl-A, so the snapshot is cut at that byte, mid-quote. Every replay then fails to parse, and no aliases or functions install. With the real rc the replay ends with 2 aliases and 0 functions.

A NUL byte in the same dump is harmless: zsh carries NUL through `$(cat)` and `eval`. The fix is fork commit "static-shell: capture between text markers, first to last". It prints the file's previously unused `INIT_MARKER` and slices between the first and last occurrence. With the real rc the replay installs 230 aliases and 1674 functions. The earlier NUL-sanitizer patch rested on the wrong diagnosis and sits in the fork's stash as `stash@{0}`.

Upstream still has the `\x01` split: `static_shell.rs`, where it is the bug, and `terminal.rs`'s login-env capture, where a raw SOH in `$PATH` or env values is implausible.
