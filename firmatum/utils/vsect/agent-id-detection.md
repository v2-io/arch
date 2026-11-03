# Detecting which Claude Code agent is running a command

*Written 2026-11-02 against Claude Code 2.1.317 on Joseph's machine. It is for whichever agent needs per-agent identity for ASF work. Every claim below is marked as **verified** (I ran it and checked the output on disk) or **from docs / not verified**. Behavior can change between Claude Code releases, so re-run `~/src/agent-core/claude-code/agent-identity/test-agent-identity` before you rely on this.*

## The problem

You might want a Bash command to know which agent issued it: the main session, a subagent, a nested subagent, or a Workflow worker. From inside a Bash call it can't tell.

- **Verified:** subagents and nested subagents see the **parent session's** `CLAUDE_CODE_SESSION_ID` in Bash. The rest of what Bash sees is identical across depths 0, 1 and 2. That covers `CLAUDE_PID`, `CLAUDE_CODE_BRIDGE_SESSION_ID`, `AI_AGENT`, the scratchpad directory, the messaging socket, and the environment block in the system prompt. Subagents run inside the same `claude` process, so even the parent PID matches.
- `CLAUDE_CODE_CHILD_SESSION=1` is set in the main session's Bash too. It only means "a subprocess Claude Code started" (docs: `env-vars.md`). It says nothing about subagents.
- No environment variable carries a per-agent id. The scratchpad path doesn't either.

The per-agent id does exist. Hook input carries `agent_id`. Transcripts and their `.meta.json` files carry it too, and so do OTEL spans and API request headers (`x-claude-code-agent-id` / `x-claude-code-parent-agent-id`, per the changelog). None of these is visible to a Bash command while it runs.

## What works: a PreToolUse hook that exports the identity (installed estate-wide 2026-11-02)

Hooks receive `agent_id` and `agent_type` on stdin when they fire inside a subagent, and a PreToolUse hook can replace a tool's input through `updatedInput`. The installed hook prefixes every Bash command with a single export:

```text
export CC_AGENT='<session>/<agent-id> parent=<parent key> type=<agent type> depth=<n> src=agent-identity-hook/2'; <the command>
```

The key comes first: `<session_id>` for the main session, `<session_id>/<agent_id>` for a subagent. Parent and depth are read from the agent's `.meta.json`, which already exists when a subagent makes its first Bash call. The hook receives the bare id (`a12437ce94ed59f1c`), the form transcripts use, not the docs' `agent-abc123`.

The source, the installed settings, the variable's full contract and a re-test script live in `~/src/agent-core/claude-code/agent-identity/` (`README.md` there). That repo is private, on Joseph's machine. It also installs a global git hook that writes `Claude-Agent:` trailers into every commit an agent makes. vsect reads `CC_AGENT` (`vsect whoami`; README "Who acted is read from the harness").

**Verified end to end, against disk.** Depth 0, 1 and 2 in throwaway `claude -p` sessions. Each echoed key matched the agent's own transcript file name and each parent matched `parentAgentId` in the harness's meta file. A commit made by the depth-2 agent carried its key and its parent's.

**What the first design got wrong.** It said `updatedInput` leaves permission behavior unchanged. It does not: the permission check runs on the *rewritten* command. In default mode, each `export` was a separate operation needing approval, so an allowed `echo` was refused. In auto mode, the classifier denied ordinary read-only commands carrying the first version's nine `CLAUDE_*` exports (6 of 24, against 0 of 56 with no hook). A single packed variable measured 0 of 48. Installed, under a natural request, it measured 0 of 64 against a control's 0 of 63; when asked for commands "exactly as written", it measured 4 of 112 against 0 of 64. With the allow rule `Bash(export CC_AGENT=*)`, default mode behaves as before. The measurements are in that README.

**Not verified:** Workflow workers (the `wf=` lookup matches the on-disk layout below, but no Workflow has run with the hook), how two PreToolUse hooks that both rewrite a Bash command combine, and whether `--resume` or a compaction keeps the session id. `/clear` changes it (docs: `env-vars.md` on `CLAUDE_CODE_SESSION_ID`).

## Where identity lives on disk (verified; 786 meta files surveyed on this machine)

```text
~/.claude/projects/<project-slug>/
  <session_id>.jsonl                                  main transcript
  <session_id>/subagents/agent-<id>.jsonl             every subagent, any depth (flat)
  <session_id>/subagents/agent-<id>.meta.json         agentType, toolUseId, spawnDepth, parentAgentId (depth ≥2)
  <session_id>/subagents/workflows/wf_<run>/agent-<id>.{jsonl,meta.json}  Workflow workers
  <session_id>/subagents/workflows/wf_<run>/journal.jsonl
```

- Every transcript line carries `sessionId` (the root session) and `agentId`. Subagent lines also have `isSidechain: true`.
- Nested subagents are stored **flat** in the root session's `subagents/`, not in a folder under their parent. Lineage comes from `parentAgentId` in the meta file. That key is absent at depth 1, meaning the parent is the main session; all 209 agents at depth ≥2 had it.
- Workflow-worker meta files have `agentType: "workflow-subagent"` and `workflowPhase`, but no `parentAgentId` or `toolUseId`. The run id appears only in the directory name. All 154 surveyed were depth 1.
- `meta.json` gains `"finished": true` once an agent completes.
- The session's temp dir also has `tasks/<agentId>.output`, a symlink to that agent's transcript.

## Without hooks: finding your transcript in two calls (verified)

A Bash call's text is written to the transcript only **after** the call finishes, so grepping for it during the same call finds nothing. A later call can find it:

1. Call 1: run anything containing a unique marker, e.g. `: whoami-<uuid>`.
2. Call 2: search for that marker under `~/.claude/projects/<slug>/$CLAUDE_CODE_SESSION_ID.jsonl` and `…/$CLAUDE_CODE_SESSION_ID/subagents/` (use `find … -exec grep -l`, since zsh aborts on unmatched globs). The file that matches is yours, and its name gives your agent id.

This needs two tool calls and a marker that is truly unique. The hook is the better option wherever you can install one. On Joseph's machine it is installed, so this matters only for harnesses without it.

## Other channels, from the docs, not used here

- **OTEL:** `claude_code.tool` and `claude_code.llm_request` spans carry `agent_id`, `parent_agent_id`, and `workflow.run_id` (`monitoring-usage.md`).
- **Agent SDK:** messages carry `parent_tool_use_id` and `parent_agent_id` (`agent-sdk/{python,typescript}.md`).
- **SubagentStart / SubagentStop hooks:** they receive `agent_id`, and `SubagentStop` also gets `agent_transcript_path`. Neither fires for the main session.
