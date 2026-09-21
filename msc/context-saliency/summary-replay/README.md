# Compaction-summary replay

Offline A/B of summarization *instructions* against a frozen grok-build compaction payload.

The first fixture is the 2026-08-31 auto-compact of session `01a0533d` (project-heat). grok-build already writes `{session}/compaction_requests/{id}.json` for this: the ConversationItem list that was sent, tools attached, model, and the summary that came back. This directory is the local loop that file exists for.

## Setup

```bash
cd ~/src/arch/msc/context-saliency/summary-replay
# API key: export XAI_API_KEY=…  or leave it in ~/xai-api-key
python3 -m unittest test_convert.py
python3 replay.py inspect
python3 replay.py extract-prompt -o prompts/00-as-shipped.txt
```

Stdlib only. Talks to `https://api.x.ai/v1/responses` (grok-4.6). Conversion follows grok-build's Responses path: reasoning items keep `encrypted_content` and drop `status`; assistant tool calls become `function_call` / `function_call_output`; tools ride along with `tool_choice: none`.

## Iterate

1. Copy `prompts/00-as-shipped.txt` to `prompts/01-<short-name>.txt` and edit.
2. Convert without spending:

   ```bash
   python3 replay.py convert --prompt prompts/01-<short-name>.txt
   ```

3. Submit (long; the fixture is a full session):

   ```bash
   python3 replay.py run --prompt prompts/01-<short-name>.txt --name 01-<short-name>
   ```

4. Read `out/<name>/summary.txt` against the original user turns (or `/tmp/session-01a0533d-user-messages.md` if still around) and against `out/<name>/meta.json` for usage.

`--drop-images` and `--no-tools` exist for cheaper / less-faithful runs. Default is full fidelity.

## Cost

`inspect` prints a chars/4 token guess. grok-4.6 long-context pricing kicks in at 200k prompt tokens. One full replay of this fixture is a real request, not a unit test — convert first, run when you mean it.

## Session transcript (dialog + tools, not the wire)

From a precompaction `updates.jsonl` (jsonl on disk, readable view on stdout and as a sibling `.txt`):

```bash
python3 transcript.py fixtures/01a0533d-precompact-updates.jsonl -o out/01a0533d-precompact-transcript.jsonl
```

One JSON object per line, temporal order: `user` / `assistant` (displayed dialog only), `tool` (short names: `read` / `edit` / `write` / `shell` / `commit` / …). The view is elapsed-since-previous-row; role column is `user`/`agnt`/`tool`/`meta`. 150-character elision with `…` at 151 when cut. `-v` leaves user, agent, and commit-message bodies whole (other tools still elide). Repeated file paths get `<fN>` aliases (after dir aliases). Thoughts, recaps, and image pixels are omitted. See the script docstring for the record shape.

## What this is not

It does not score interlocutor-side continuity. It produces a new summary under a new instruction, from the same history, so we can see what it takes to get a handoff Joseph would actually want to inherit. The original summary lives on the request as `summary` and in `out/` only after you extract or re-run it.
