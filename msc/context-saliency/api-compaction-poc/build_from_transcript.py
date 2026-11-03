"""Rebuild a Claude Code session, up to its first compaction, as an on-demand compaction request.

Reads the transcript JSONL up to the first `compact_boundary` row, joins Claude Code's per-block rows
back into whole messages (thinking blocks kept, with their signatures), and takes `system` and `tools`
from the session's `prompt_snapshot` attachment. Writes request.json and export-meta.json for
send_stream.py. Sends nothing.

Usage: python3 build_from_transcript.py <transcript.jsonl> <out_dir> <instructions.txt>
"""
import json, os, sys

src, out, instr_path = sys.argv[1], sys.argv[2], sys.argv[3]
rows = []
system = tools = None
for line in open(src):
    o = json.loads(line)
    if o.get("type") == "system" and o.get("subtype") == "compact_boundary":
        break
    a = o.get("attachment")
    if isinstance(a, dict) and a.get("type") == "prompt_snapshot" and a.get("tools"):
        system = system or a.get("systemPrompt")
        tools = tools or a.get("tools")
    if o.get("type") in ("user", "assistant") and isinstance(o.get("message"), dict):
        rows.append(o)

msgs = []
for o in rows:
    m = o["message"]
    content = m.get("content")
    blocks = [{"type": "text", "text": content}] if isinstance(content, str) else list(content or [])
    blocks = [b for b in blocks if not (b.get("type") == "text" and not b.get("text"))]
    if not blocks:
        continue
    if msgs and msgs[-1]["role"] == m["role"]:
        msgs[-1]["content"].extend(blocks)
    else:
        msgs.append({"role": m["role"], "content": blocks})

# The request must not end in a tool call with no result.
if msgs and msgs[-1]["role"] == "assistant" and any(b.get("type") == "tool_use" for b in msgs[-1]["content"]):
    msgs.pop()

def sys_blocks(s):
    if isinstance(s, str):
        return [{"type": "text", "text": s}]
    return [{"type": "text", "text": x} if isinstance(x, str) else {"type": "text", "text": x.get("text", "")} for x in s]

# The snapshot's `schema` is the tool definition as Claude Code sent it on the wire.
api_tools = [t["schema"] for t in (tools or [])]

body = {"model": "claude-opus-5-5", "max_tokens": 32000, "system": sys_blocks(system or ""),
        "tools": api_tools, "messages": msgs,
        "compaction": {"type": "summarize", "instructions": open(instr_path).read().strip()}}
os.makedirs(out, exist_ok=True)
text = json.dumps(body)
open(os.path.join(out, "request.json"), "w").write(text)
count = lambda t: sum(1 for m in msgs for b in m["content"] if b.get("type") == t)
meta = {"source": src, "rowsRead": len(rows), "messages": len(msgs), "requestChars": len(text),
        "systemBlocks": len(body["system"]), "tools": len(api_tools),
        "thinkingBlocks": count("thinking"), "toolUses": count("tool_use"), "images": count("image"),
        "betas": ["compact-2026-09-04", "thinking-binding-controls-2026-08-01"]}
json.dump(meta, open(os.path.join(out, "export-meta.json"), "w"), indent=2)
print(json.dumps(meta, indent=2))
