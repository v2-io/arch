# Detecting which Claude Code agent is running a command

*Written 2026-11-02 against Claude Code 2.1.317 on Joseph's machine. It is for whichever agent needs per-agent identity for ASF work. Every claim below is marked as **verified** (I ran it and checked the output on disk) or **from docs / not verified**. Behavior can change between Claude Code releases, so re-run the test at the end before you rely on this.*

## The problem

You might want a Bash command to know which agent issued it: the main session, a subagent, a nested subagent, or a Workflow worker. From inside a Bash call it can't tell.

- **Verified:** subagents and nested subagents see the **parent session's** `CLAUDE_CODE_SESSION_ID` in Bash. The rest of what Bash sees is identical across depths 0, 1 and 2. That covers `CLAUDE_PID`, `CLAUDE_CODE_BRIDGE_SESSION_ID`, `AI_AGENT`, the scratchpad directory, the messaging socket, and the environment block in the system prompt. Subagents run inside the same `claude` process, so even the parent PID matches.
- `CLAUDE_CODE_CHILD_SESSION=1` is set in the main session's Bash too. It only means "a subprocess Claude Code started" (docs: `env-vars.md`). It says nothing about subagents.
- No environment variable carries a per-agent id. The scratchpad path doesn't either.

The per-agent id does exist. Hook input carries `agent_id`. Transcripts and their `.meta.json` files carry it too, and so do OTEL spans and API request headers (`x-claude-code-agent-id` / `x-claude-code-parent-agent-id`, per the changelog). None of these is visible to a Bash command while it runs.

## What works: a PreToolUse hook that exports the identity

Hooks receive `agent_id` and `agent_type` on stdin when they fire inside a subagent. A PreToolUse hook can also replace a tool's input through `updatedInput`. The hook below prepends `export …;` statements to every Bash command, so the command can read its own identity from the environment.

**Verified end to end** in throwaway `claude -p` sessions, one with a depth-1 subagent and one with a depth-2 chain. Actual output, with the same lines confirmed in each agent's transcript on disk:

```text
L0 id=main parent= depth=0 wf= key=bb0e2798-587f-4708-ab4c-f99c8a831ba5
L1 id=a12437ce94ed59f1c parent=main depth=1 wf= key=bb0e2798-587f-4708-ab4c-f99c8a831ba5/a12437ce94ed59f1c
L2 id=ace6603228cb61702 parent=a12437ce94ed59f1c depth=2 wf= key=bb0e2798-587f-4708-ab4c-f99c8a831ba5/ace6603228cb61702
```

Details the test settled:

- **Permissions:** `updatedInput` takes effect with no `permissionDecision`, so the hook doesn't change permission behavior. The docs leave this unstated.
- **Id format:** the hook receives the bare id (`a12437ce94ed59f1c`), the same form transcripts use. The docs' examples show `agent-abc123`.
- **Meta file timing:** the agent's `.meta.json` already exists when the subagent makes its first Bash call. That lets the hook read the parent id and depth from it, since hook input has no parent field.

### Variables it exports

| Variable | Value |
|---|---|
| `CLAUDE_AGENT_ID` | subagent id, or `main` |
| `CLAUDE_AGENT_TYPE` | subagent type (e.g. `general-purpose`, `Explore`, or the `--agent` name); empty for a plain main session |
| `CLAUDE_AGENT_KEY` | `<session_id>` for main, `<session_id>/<agent_id>` otherwise. Unique per agent. |
| `CLAUDE_PARENT_AGENT_ID` | parent subagent id; `main` if the main session spawned it; empty for main itself, or if the meta file wasn't found |
| `CLAUDE_AGENT_DEPTH` | `0` for main, otherwise `spawnDepth` from the meta file |
| `CLAUDE_WORKFLOW_RUN_ID` | `wf_…` for Workflow workers; otherwise empty. **Not verified**, see below. |
| `CLAUDE_TOOL_USE_ID` | id of this Bash call |

### The hook script

Save it somewhere durable. `~/.claude/hooks/agent-identity-hook.py` is one reasonable place; where it goes is your call.

```python
#!/usr/bin/env python3 -I
"""PreToolUse hook (matcher: Bash): prefix each Bash command with exports that
identify which Claude Code agent issued it.

Why: inside subagents, Bash sees only the *parent* session's CLAUDE_CODE_SESSION_ID.
Hooks, however, receive agent_id / agent_type on stdin when firing inside a
subagent, and may rewrite the tool input via updatedInput.

Exports:
  CLAUDE_AGENT_ID         subagent id, or "main" for the top-level session
  CLAUDE_AGENT_TYPE       subagent type (or --agent name); empty if none
  CLAUDE_AGENT_KEY        "<session_id>" or "<session_id>/<agent_id>" (unique per agent)
  CLAUDE_PARENT_AGENT_ID  parent subagent id; "main" if spawned by the main session;
                          empty for the main session or if the meta file isn't found
  CLAUDE_AGENT_DEPTH      0 for main; spawnDepth from the agent's meta file otherwise
  CLAUDE_WORKFLOW_RUN_ID  wf_... if the agent is a Workflow worker; else empty
  CLAUDE_TOOL_USE_ID      id of this tool call

Returns only updatedInput (no permissionDecision), so permission behavior is
unchanged. Any failure falls back to exiting 0 with no output (command runs as-is).
"""
import glob, json, os, shlex, sys

try:
    d = json.load(sys.stdin)
    if d.get("tool_name") != "Bash":
        sys.exit(0)
    inp = dict(d.get("tool_input") or {})
    cmd = inp.get("command")
    if not isinstance(cmd, str):
        sys.exit(0)

    sid = d.get("session_id", "")
    aid = d.get("agent_id") or ""
    parent, depth, wf = "", "0", ""
    if aid:
        depth = ""
        # transcript_path is the main session's <project>/<session>.jsonl;
        # subagent meta files live under <project>/<session>/subagents/.
        tp = d.get("transcript_path") or ""
        sess_dir = tp[:-len(".jsonl")] if tp.endswith(".jsonl") else ""
        metas = []
        if sess_dir:
            metas = glob.glob(os.path.join(sess_dir, "subagents", f"agent-{aid}.meta.json")) + \
                    glob.glob(os.path.join(sess_dir, "subagents", "workflows", "*", f"agent-{aid}.meta.json"))
        if metas:
            m = json.load(open(metas[0]))
            parent = m.get("parentAgentId") or "main"
            depth = str(m.get("spawnDepth", ""))
            wfdir = os.path.basename(os.path.dirname(metas[0]))
            wf = wfdir if wfdir.startswith("wf_") else ""

    env = {
        "CLAUDE_AGENT_ID": aid or "main",
        "CLAUDE_AGENT_TYPE": d.get("agent_type") or "",
        "CLAUDE_AGENT_KEY": f"{sid}/{aid}" if aid else sid,
        "CLAUDE_PARENT_AGENT_ID": parent,
        "CLAUDE_AGENT_DEPTH": depth,
        "CLAUDE_WORKFLOW_RUN_ID": wf,
        "CLAUDE_TOOL_USE_ID": d.get("tool_use_id") or "",
    }
    prefix = "".join(f"export {k}={shlex.quote(v)}; " for k, v in env.items())
    inp["command"] = prefix + cmd
    print(json.dumps({"hookSpecificOutput": {"hookEventName": "PreToolUse", "updatedInput": inp}}))
except SystemExit:
    raise
except Exception:
    sys.exit(0)
```

### Registering it

Add this to a settings file's `hooks` key, merging it with any hooks already there. Use `~/.claude/settings.json` for every project, or a repo's `.claude/settings.json` for that repo only. When I checked, `~/.claude/settings*.json` had no hooks configured. Changing global settings is Joseph's decision.

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          { "type": "command", "command": "python3 -I ~/.claude/hooks/agent-identity-hook.py" }
        ]
      }
    ]
  }
}
```

To try it without touching any settings file, pass the JSON to a single run with `claude -p --settings <file>` (that's how I tested it).

### Re-testing it

```sh
claude -p --model haiku --settings <your-hook-settings.json> --allowedTools "Bash" "Agent" \
  'Run in Bash: echo "L0 id=$CLAUDE_AGENT_ID parent=$CLAUDE_PARENT_AGENT_ID depth=$CLAUDE_AGENT_DEPTH key=$CLAUDE_AGENT_KEY". Then launch one general-purpose subagent in the FOREGROUND to run the same command with L1 instead of L0 and return its output verbatim. Print both lines.'
```

Then confirm the output against the transcripts, not just the model's relay: `grep -rho 'L[01] id=[^"\\]*' ~/.claude/projects/<that-project-dir>/`.

### Not verified, or worth knowing

- **Workflow workers: not verified.** The docs say settings hooks fire inside all subagents, and the script's `wf_` lookup matches the on-disk layout (next section). I haven't run a Workflow with the hook installed.
- **Other hooks:** if another PreToolUse hook also returns `updatedInput` for Bash, I don't know how Claude Code combines them.
- **What the transcript records:** the transcript I checked records the command as Claude wrote it, without the prefix. I didn't check whether it also stores the rewritten version anywhere.
- **Cost:** one `python3` start per Bash call, not measured.
- **Main-session id:** after `/clear`, the main session's `session_id`, and so the key, changes. Docs: `env-vars.md` on `CLAUDE_CODE_SESSION_ID`.

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

This needs two tool calls and a marker that is truly unique. The hook is the better option wherever you can install one.

## Other channels, from the docs, not used here

- **OTEL:** `claude_code.tool` and `claude_code.llm_request` spans carry `agent_id`, `parent_agent_id`, and `workflow.run_id` (`monitoring-usage.md`).
- **Agent SDK:** messages carry `parent_tool_use_id` and `parent_agent_id` (`agent-sdk/{python,typescript}.md`).
- **SubagentStart / SubagentStop hooks:** they receive `agent_id`, and `SubagentStop` also gets `agent_transcript_path`. Neither fires for the main session.
