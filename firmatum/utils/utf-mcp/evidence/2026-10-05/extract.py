#!/usr/bin/env python3
"""Provenance for this directory: how its files were cut from the 2026-10-05
session scratchpad (a temporary directory, not kept). Re-running it needs that
scratchpad, so it is kept as a record of what was taken and what was left out.

Kept verbatim: utf-mcp call logs (they hold the exact arguments and results;
parent_command holds only test prompts) and agy's JSON results.
Reduced: CLI output streams and captured model requests, to the tool lists
offered, the tool calls made and their results (truncated), and final text.
Left out: full system prompts and request bodies, which carry machine-specific
context (e.g. a user's skills list) and are not needed to check any claim.
"""
import json, pathlib, shutil, sys

SRC = pathlib.Path(sys.argv[1])
OUT = pathlib.Path(__file__).resolve().parent

def jl(p):
    for line in open(p, encoding="utf-8"):
        line = line.strip()
        if line:
            try:
                yield json.loads(line)
            except Exception:
                pass

def write(name, obj):
    (OUT / name).write_text(json.dumps(obj, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")

def cut(s, n=400):
    s = s if isinstance(s, str) else json.dumps(s, ensure_ascii=False)
    return s if len(s) <= n else s[:n] + "…"

# --- utf-mcp call logs, verbatim
logs = OUT / "call-logs"; logs.mkdir(exist_ok=True)
for src, dst in [
    ("claude-test.jsonl", "claude-live-1.jsonl"),
    ("it-claude/utf-mcp-calls.jsonl", "claude-live-2-script.jsonl"),
    ("codex-test.jsonl", "codex-live-quota-exhausted.jsonl"),
    ("codex-fake2.jsonl", "codex-fake-gpt55-toolsearch.jsonl"),
    ("codex-fake3.jsonl", "codex-fake-codemode.jsonl"),
    ("it-codex/utf-mcp-calls.jsonl", "codex-fake-script-gpt55.jsonl"),
    ("it-codex2/utf-mcp-calls.jsonl", "codex-fake-script-codemode.jsonl"),
    ("it-grok/utf-mcp-calls.jsonl", "grok-fake-script.jsonl"),
    ("agy-test.jsonl", "agy-live-3-hook-overrides.jsonl"),
    ("hooktest.jsonl", "agy-hook-offline.jsonl"),
]:
    p = SRC / src
    if p.exists():
        shutil.copy(p, logs / dst)

# --- claude: init event (tools, MCP status, memory path) + tool calls
def claude_stream(p):
    out = {"init": None, "steps": []}
    for m in jl(p):
        t = m.get("type")
        if t == "system" and m.get("subtype") == "init":
            out["init"] = {k: m.get(k) for k in ("tools", "mcp_servers", "model", "memory_paths", "claude_code_version")}
        elif t == "assistant":
            for c in m["message"]["content"]:
                if c["type"] == "tool_use":
                    out["steps"].append({"tool_use": c["name"], "id": c["id"], "input": c["input"]})
                elif c["type"] == "text":
                    out["steps"].append({"text": c["text"]})
        elif t == "user":
            for c in m["message"]["content"]:
                if isinstance(c, dict) and c.get("type") == "tool_result":
                    out["steps"].append({"tool_result_for": c.get("tool_use_id"), "content": cut(c.get("content"))})
        elif t == "result":
            out["result"] = {k: m.get(k) for k in ("subtype", "num_turns", "is_error")}
    return out

for src, dst in [("claude-test.stream.jsonl", "claude-live-1.summary.json"), ("it-claude.out", "claude-live-2-script.summary.json"),
                 ("memprobe-base.out", "claude-memprobe-default.init.json"), ("memprobe-env.out", "claude-memprobe-disable-auto-memory.init.json")]:
    if (SRC / src).exists():
        write(dst, claude_stream(SRC / src))

# --- grok streams: offered tools, tool calls, results, final text
def grok_stream(p):
    out = {"tools_offered": [], "calls": [], "updates": [], "text": "", "end": None}
    for m in jl(p):
        t = m.get("type")
        if t == "available_commands" and m["tools"] not in out["tools_offered"]:
            out["tools_offered"].append(m["tools"])
        elif t == "tool_call":
            out["calls"].append({"id": m.get("toolCallId"), "tool": m.get("toolName"), "input": m.get("rawInput")})
        elif t == "tool_call_update" and m.get("status") in ("completed", "failed"):
            out["updates"].append({"id": m.get("toolCallId"), "status": m.get("status"), "content": cut(m.get("content") or m.get("rawOutput"))})
        elif t == "text":
            out["text"] += m["data"]
        elif t == "end":
            out["end"] = {k: m.get(k) for k in ("stopReason", "num_turns", "modelUsage")}
    return out

for src, dst in [("grok-test.stream.jsonl", "grok-live-1-tools-flag-ignored.summary.json"),
                 ("grok-live2.jsonl", "grok-live-2.summary.json"),
                 ("it-grok.out", "grok-fake-script.summary.json"),
                 ("tif-base.out", "grok-fake-tool-input-file-baseline.summary.json"),
                 ("tif-d2.out", "grok-fake-tool-input-file-deny-Read.summary.json"),
                 ("tif-sb-strict.out", "grok-fake-tool-input-file-sandbox-strict.summary.json"),
                 ("tif-new.out", "grok-fake-tool-input-file-after-repairs.summary.json")]:
    if (SRC / src).exists():
        write(dst, grok_stream(SRC / src))

# --- requests captured by the fake endpoint: tool names only
def tool_names(body):
    tools = list(body.get("tools") or [])
    for it in body.get("input") or []:
        if isinstance(it, dict) and it.get("type") == "additional_tools":
            tools += it.get("tools", [])
    def names(ts):
        return [{t["name"]: names(t.get("tools", []))} if t.get("type") == "namespace" else (t.get("name") or t.get("type")) for t in ts]
    return names(tools)

for src, dst in [("it-codex-req.jsonl", "codex-fake-script-gpt55.requests.json"), ("it-codex-req2.jsonl", "codex-fake-script-codemode.requests.json"),
                 ("it-grok-req.jsonl", "grok-fake-script.requests.json")]:
    if (SRC / src).exists():
        write(dst, [{"model": r["body"].get("model"), "tools": tool_names(r["body"]),
                     "items": [i.get("type") for i in r["body"].get("input", []) if isinstance(i, dict)]} for r in jl(SRC / src)])

# --- agy print-mode results
for n in (1, 2, 3):
    p = SRC / f"agy{n}.json"
    if p.exists():
        write(f"agy-live-{n}.result.json", {"result": json.loads(p.read_text().strip().splitlines()[-1]),
                                            "stderr": (SRC / f"agy{n}.err").read_text() if (SRC / f"agy{n}.err").exists() else None})
print("written to", OUT)
