"""Does the on-demand summarizer's input include earlier thinking, with and without custom instructions?

Takes the first two messages of a finished poc.py run (user turn + assistant turn with a thinking
block) and compacts them four ways: {default prompt, neutral instructions} x {thinking kept,
thinking stripped}. The compaction iteration reports its own input_tokens, so the drop when the
thinking block is removed is the thinking the summarizer was given. The neutral instruction asks
for nothing about reasoning, to avoid the reasoning_extraction safeguard.

Usage: ANTHROPIC_API_KEY=... python3 thinking_input.py runs/03-poc runs/04-thinking-input
"""
import json, sys
from api import Run
# Same values as poc.py (importing poc would run it).
MODEL = "claude-opus-5-5"
BETAS = ["compact-2026-09-04", "thinking-binding-controls-2026-08-01"]
SYSTEM = "You help plan a small software project. Keep visible answers short."
THINKING = {"type": "adaptive", "display": "summarized",
            "block_binding": {"prefix_mismatch_behavior": "error"}}

src, out = sys.argv[1], sys.argv[2]
turn1 = json.load(open(f"{src}/01-turn1.request.json"))["body"]["messages"]
reply = json.load(open(f"{src}/01-turn1.response.json"))["data"]["content"]
with_thinking = [*turn1, {"role": "assistant", "content": reply}]
without = [*turn1, {"role": "assistant", "content": [b for b in reply if b["type"] not in ("thinking", "redacted_thinking")]}]
NEUTRAL = "Summarize the conversation above for whoever continues it. Do not call tools. Respond with the summary only."

run = Run(out)
rows = []
for arm, cfg in (("default", {"type": "summarize"}), ("neutral", {"type": "summarize", "instructions": NEUTRAL})):
    for label, msgs in (("thinking", with_thinking), ("stripped", without)):
        status, d = run.call(f"{arm}-{label}", {"model": MODEL, "max_tokens": 4096, "system": SYSTEM,
                                               "thinking": THINKING, "messages": msgs, "compaction": cfg}, BETAS)
        it = next((i for i in d.get("usage", {}).get("iterations", []) if i["type"] == "compaction"), {})
        rows.append((arm, label, status, d.get("stop_reason"), it.get("input_tokens"), it.get("output_tokens")))
        summary = next((b.get("content") for b in d.get("content", []) if b["type"] == "compaction"), None)
        print(f"--- {arm}/{label}: HTTP {status} stop={d.get('stop_reason')} in={it.get('input_tokens')} out={it.get('output_tokens')}")
        print(summary or json.dumps(d.get("stop_details") or d.get("error")))
print()
for r in rows: print("%-8s %-9s %s %-10s in=%s out=%s" % r)
