"""Proof of concept: on-demand compaction with custom instructions and a kept turn.

Answers two of the note's open questions on a toy conversation (T1, T2 in
api-compaction-steering-and-folding-2026-10-31.md):
  T2  Does the kept turn's thinking survive the swap? (prefix_mismatch_behavior "error")
  T1  Does a summarizer working to custom instructions carry content that was only
      in earlier thinking? Turn 1's thinking is requested with display "summarized"
      so we can see what it held; the visible answer is kept to a single letter.
Also records any refusal on the compaction calls.

Usage: ANTHROPIC_API_KEY=... python3 poc.py runs/02-poc
"""
import json, sys
from api import Run

MODEL = "claude-opus-5-5"
BETAS = ["compact-2026-09-04", "thinking-binding-controls-2026-08-01"]
SYSTEM = "You help plan a small software project. Keep visible answers short."
THINKING = {"type": "adaptive", "display": "summarized",
            "block_binding": {"prefix_mismatch_behavior": "error"}}
INSTRUCTIONS = (
    "Write a chronological log of the conversation above for the instance that will continue it. "
    "Quote the user's messages in full, in order. For each decision, record what was decided and the "
    "considerations that settled it, at the point where it was decided. Mark anything you shortened "
    "as [omitted: what]. Do not call tools. Respond with the log only."
)

run = Run(sys.argv[1])

def ask(label, messages, **extra):
    status, data = run.call(label, {"model": MODEL, "max_tokens": 8192, "system": SYSTEM,
                                    "thinking": THINKING, "messages": messages, **extra}, BETAS)
    print(f"--- {label}: HTTP {status}, stop_reason={data.get('stop_reason')}")
    if status != 200:
        print(json.dumps(data, indent=2)); sys.exit(1)
    return data

def show(data):
    for b in data["content"]:
        if b["type"] == "thinking":
            print("  [thinking summary]", b.get("thinking") or "(empty)")
        elif b["type"] == "text":
            print("  [text]", b["text"])
        elif b["type"] == "compaction":
            print("  [compaction]\n" + (b.get("content") or "(null)"))
    print("  input_transformations:", data.get("input_transformations"))

# Turn 1: a choice whose reasons live in thinking; the visible answer is one letter.
history = [{"role": "user", "content":
    "We need a store for about 2,000 small JSON records, written by one process and read by "
    "a CLI that must start in under 50 ms on a laptop. Options: (A) SQLite, (B) one JSON file "
    "per record, (C) a Postgres server. Answer with the letter only."}]
r1 = ask("turn1", history); show(r1)
history.append({"role": "assistant", "content": r1["content"]})

# Turn 2: the kept turn.
history.append({"role": "user", "content": "Name the one table or directory we need, in five words or fewer."})
r2 = ask("turn2", history); show(r2)
history.append({"role": "assistant", "content": r2["content"]})

# Compact turn 1 only, two arms: default prompt and custom instructions.
older = history[:2]
arms = {}
for arm, cfg in (("default", {"type": "summarize"}),
                 ("instructions", {"type": "summarize", "instructions": INSTRUCTIONS})):
    d = ask(f"compact-{arm}", older, compaction=cfg); show(d)
    if d.get("stop_reason") == "compaction":
        arms[arm] = d

# Continue from each block with the kept turn; "error" makes a failed kept thinking a 400.
for arm, d in arms.items():
    cont = [{"role": "assistant", "content": d["content"]}, *history[2:],
            {"role": "user", "content": "Which option did we choose at the start, and why?"}]
    show(ask(f"continue-{arm}", cont))
