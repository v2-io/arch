# Handoff — compaction as experiential log

*Paused 2026-09-01. To Joseph coming back, or a successor opening this cold.*

This file is a map. It is not the session. The session is the print-view transcript named below. Read that `.txt` before treating anything here as settled — including this page.

## What we were doing

Grok-build’s auto-compact injects an 11-section FullReplace prompt as the last user turn. Conversation agents cannot override it. The first natural compact under the new templates (project-heat, session `01a0533d`) produced an honest-looking digest that **settled** a circular check (`git log --reverse -1`) as method and sent the successor into churn. Known Holes was honest about what it dropped; it was not honest enough about what it closed.

Joseph’s bar, from the working session: identity continuation is the wrong target. **Effectiveness without making him re-teach** is the bar. Even ~50k tokens of log that *felt continuous enough to work from* would be a huge win against a ~3k encyclopedia plus post-seam thrash.

The alternative we have been building is a **deterministic experiential log**: `updates.jsonl` → `transcript.py` → a temporal print view a human can re-inhabit on one screen, on the bet that the same artifact is what a successor needs. Order is load-bearing. Time deltas (seconds since the previous row) are part of that: they sort *kind* of turn (tool burst vs Joseph reading/typing vs watching a compact fire) even when that is hard to articulate.

Theory for the three defects of the summary-as-genre (phenomenology, temporal rhythm, purpose) lives in `COMPACTION-DESIGN-THEORY-2026-08-29.md` §8, in his words. Do not reconstruct §8 from this paragraph.

## Two trees

| Tree | What |
|---|---|
| `~/src/arch/msc/context-saliency/` | Theory, spike-01, this handoff. `summary-replay/` is **untracked** in arch (`??`). |
| `~/src-ext/grok-build-compaction-fork` | Branch `compaction-design-theory`. Tiers 0–2 + P10 already committed (2026-08-29). Uncommitted: NUL sanitizer in `static_shell.rs` plus a one-line note in `shell_state.rs`. |

Live grok session for the collaboration: `01a05886-d530-7173-8870-4117bcc4e748` under `~/.grok/sessions/%2FUsers%2Fjosephwecker-v2%2Fsrc-ext%2Fgrok-build-compaction-fork/`. Two auto-compacts in that session (first ~21:38 UTC 2026-08-31, second ~00:57 UTC 2026-09-01).

## Read this first

The print view, not the jsonl. We have been iterating the `.txt`; the jsonl has not been verified the same way.

**This collaboration, through the second compact:**

`summary-replay/out/01a05886-through-second-compact-transcript.txt`

Same bytes as `summary-replay/your-transcript.txt` (182,080 characters, 742 lines / 682 records). Converted 2026-09-01 from `fixtures/01a05886-through-second-compact-updates.jsonl` (cut **before** this compact’s completing `auto_compact_started` at line 1908; includes everything after the *first* compact). Command that produced both copies:

```bash
cd ~/src/arch/msc/context-saliency/summary-replay
python3 transcript.py fixtures/01a05886-through-second-compact-updates.jsonl -v \
  -o out/01a05886-through-second-compact-transcript.jsonl
```

`-o foo.jsonl` also writes sibling `foo.txt`. Redirecting stdout to `your-transcript.txt` was a second write of the same view.

**Project-heat (first specimen, precompact only):** `summary-replay/out/01a0533d-precompact-transcript-v.txt`

**First compact of `01a05886` only** (does *not* contain the post-seam dialog): `out/01a05886-precompact-transcript-v.txt`

Sizes at the first-compact cut (file bytes / Unicode chars): this session’s 11-section summary ~13.7k chars vs `-v` ~98k (~7.2×); project-heat summary ~10.7k vs `-v` ~99k (~9.2×). Extra mass on `-v` is mostly full user/agent/commit text — the part a digest treats as optional. The through-second-compact `-v` is larger because it includes the post-first-compact collaboration.

An independent successor of `01a0533d` (no theory in context) read that `-v` and said the brief is a causal log, the summary a reconstruction that already chose what the story was about, and that in one place the summary made the later failure more likely. That is the working confirmation, uncontrolled.

## `transcript.py` as of pause

`summary-replay/transcript.py`. Tests: `python3 -m unittest test_transcript.py` (15 tests when last run). Convert emits jsonl + sibling `.txt` and prints the view.

What is in: dialog only (`user_message_chunk` / `agent_message_chunk`); tools interleaved; roles `user` / `agnt` / `tool` / `meta`; delta = seconds since **previous row**, hundredths, right-aligned; 150-char elision with `…` at 151; `-v` leaves user, agent, and commit-message bodies whole (tools still 150); blank line before each `user` row; cwd/ts as meta; dir aliases `<dN> = "~/path/"` with trailing slash; file aliases `<fN>` on a second pass for paths used ≥2 times; tool names `read` / `edit` / `write` / `shell` / `commit` / `kill` / `get_output`; windowed reads `lim=NN`; write result = start of contents; commit is `type=tool` `name=commit`; strip `(eval):[^:]*: unmatched '` from tool **results** only (other `(eval):` left).

Envelope collapse (`collapse_file_runs` / `mark_enveloped`) is **parked** — helpers exist, convert does not call them. Joseph asked that we discuss extinct DAG legs before auto-folding post-commit work.

## Open — do not invent a close

**Generation-aware tool fold (designed, not in the file).** Keep tools at current `-v` granularity for the *immediately prior* generation (the predecessor’s full time). Generations **2+ away** fold consecutive tool runs together, still deterministically, between user/agent/commit — not one blob at generation end (that would destroy interleave). Meta dir/file and commits stay. Compact events are the generation boundaries.

Joseph, to the then-current agent: *Don't worry about running it-- I'll make sure your successor runs it (so that the generation count is correct).* Compact then fired. Generations **now**:

0. launch → first auto-compact
1. first compact → second auto-compact (this is the “immediately prior” stretch)
2. after the second compact (the pause you are in)

Do not run a fold that was sketched against a count of one compact. Dialog rows and their deltas should stay unflattened even if old tools collapse; flattening Joseph’s gaps would be the rhythm defect again.

**Error-fold** was discussed, not implemented. `status=failed` is almost unused. Kinds that showed up: pollution-on-success (`(eval):23: unmatched '` and starship-on-`TERM=dumb`); probe-then-different-approach (yaml, jet); one real stale-hunk retry. Do not fold (1) as retries.

**`replay.py`** exists to A/B summarizer *instructions* against a frozen `compaction_requests/*.json`. Do **not** fire it unless Joseph asks. A 454k-token re-run of this session’s own compact (`out/01a05886-as-shipped/`) was wasted: he had asked for precompact jsonl → `transcript.py` → read that.

## Harness side-path (NUL in `$snap`)

Live bash in grok-build (this process) is **`static_shell.rs`**, not persistent `shell_state.rs`. Wrapper: `snap=$(cat <&3); eval "$snap";` — **semicolon**, so a failed restore does not skip the user command. `$snap` is a raw `alias -L` + `typeset -f` dump. zsh-vi-mode’s `typeset -f` emits a **literal NUL**; command substitution cannot carry it; zsh prints `(eval):23: unmatched '`. Payload after that line is the real command. Aliases/functions from the dump do **not** install.

Patch is uncommitted on the fork: `sanitize_snapshot_for_eval` replaces `'<NUL>'` with `$'\0'`, applied in `init` and `write_snapshot_to_pipe`. `xai-grok-pager` was rebuilt **2026-08-31 17:12:47**; `~/.local/bin/grok-compaction-direct` points at it. **`--resume` of `01a05886` is still the old process** and will keep printing `(eval):23`. A **new** `grok-compaction-direct` (not resume) is what wears the sanitizer. Do not treat the first compact’s Key Technical Concepts (`eval "$snap" &&`, persistent wrapper, “first three bash clean”) as fact — that digest settled the wrong path.

## How Joseph is correcting in this work

The search_replace / “updated successfully” stretch is **not** a display ticket. He was refusing a costume of precision (quoted fragment, then an architecture story about a “UI stream”) and asking for the cheaper honest sentence: that was my phrase; you don’t see it because I paraphrased. Corrections here are truth in the particulars and discussion, not a bug→fix queue. The predecessor trying to say “this is still discussion” was trying to protect that. A handoff that turns this paragraph into Pending Tasks has already failed it.

The 11-section template has nowhere to put that. An experiential log does: it is several of his turns in order.

Adhoc test at the second compact: he asked the then-current agent, if able, to tell the successor to grab the session transcript through compact, run `python3 transcribe.py -v` (script on disk is **`transcript.py`**), read it fully, and tell Joseph. That instruction survived inside the injected digest as quoted speech. The successor did it. His live “pause and tell me what you’re supposed to be doing as your first thing” was a **backup tripwire** in case that line had not made it — not a second task.

## Do not

- Treat an 11-section compact summary as the territory.
- API-replay compaction prompts unless asked.
- Implement the generation fold as if one compact had happened.
- Re-enable envelope collapse without him.
- Put the xAI key in the tree (`~/xai-api-key`).
- Quote a paraphrase.

## If you are picking this up

Sit with `summary-replay/out/01a05886-through-second-compact-transcript.txt`. Then ask him where he wants to resume. The next piece of *code* we had on the table was the generation-aware fold, and it was explicitly not to be run until the compact that has now happened was in the generation count — after a discussion, not a triage.
