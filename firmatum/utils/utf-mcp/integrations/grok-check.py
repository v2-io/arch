#!/usr/bin/env python3
"""Post-run check for integrations/grok.sh: did any use_tool call make grok read
a local file (tool_input_file / file) instead of passing arguments inline?

    integrations/grok-check.py WORKDIR [LOG] [TAG]

Reads WORKDIR/grok-stream.jsonl (written by grok.sh). LOG defaults to
WORKDIR/utf-mcp-calls.jsonl. Exit status:
  0  stream read; no file arguments
  3  at least one use_tool call named a file: a "harness_warning" record is
     appended to LOG and the calls are printed on stderr
  4  the stream could not be read as grok's streaming-json (missing, empty, or
     no events of a known type), so the check cannot vouch for the run. A
     format change in a later grok lands here rather than passing silently.

It parses every line as JSON and inspects each tool_call event whose toolName
is use_tool; rawInput is accepted as an object or as a JSON-encoded string.
"""
import datetime, json, os, sys

if len(sys.argv) < 2:
    sys.exit(__doc__)
work = sys.argv[1]
log = sys.argv[2] if len(sys.argv) > 2 else os.path.join(work, "utf-mcp-calls.jsonl")
tag = sys.argv[3] if len(sys.argv) > 3 else None
stream = os.path.join(work, "grok-stream.jsonl")
if tag is None and os.path.exists(log):
    for line in open(log, encoding="utf-8"):
        try:
            tag = json.loads(line).get("tag")
        except ValueError:
            continue
        if tag:
            break

known, use_tool, flagged = 0, 0, []
try:
    lines = open(stream, encoding="utf-8").read().splitlines()
except OSError as e:
    print(f"grok-check: cannot read {stream}: {e}", file=sys.stderr)
    sys.exit(4)
for line in lines:
    try:
        ev = json.loads(line)
    except ValueError:
        continue
    if not isinstance(ev, dict):
        continue
    if ev.get("type") in ("available_commands", "tool_call", "tool_call_update", "text", "thought", "usage", "end", "error"):
        known += 1
    if ev.get("type") == "tool_call" and ev.get("toolName") == "use_tool":
        use_tool += 1
        raw = ev.get("rawInput")
        if isinstance(raw, str):
            try:
                raw = json.loads(raw)
            except ValueError:
                raw = {"_unparsed": raw}
        if not isinstance(raw, dict) or any(k in raw for k in ("tool_input_file", "file", "_unparsed")):
            flagged.append(raw)
if known == 0:
    print(f"grok-check: {stream} has no grok events; cannot check this run", file=sys.stderr)
    sys.exit(4)
if flagged:
    for f in flagged:
        print(f"grok-check: use_tool named a file: {json.dumps(f, ensure_ascii=False)[:300]}", file=sys.stderr)
    rec = {"ts": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="milliseconds").replace("+00:00", "Z"),
           "event": "harness_warning", "harness": "grok", "warning": "use_tool named a local file (tool_input_file/file)",
           "calls": flagged, "stream": stream}
    if tag:
        rec["tag"] = tag
    with open(log, "a", encoding="utf-8") as fh:
        fh.write(json.dumps(rec, ensure_ascii=False) + "\n")
    sys.exit(3)
print(f"grok-check: ok ({use_tool} use_tool call(s), none named a file)", file=sys.stderr)
