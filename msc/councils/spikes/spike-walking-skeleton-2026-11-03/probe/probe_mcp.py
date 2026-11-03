#!/usr/bin/env python3
"""Harness probe: a stdio MCP server that logs every JSON-RPC message both ways.

Tools:
  probe_echo {x}                       returns its args and the request _meta
  probe_sleep {seconds, progress_every} blocks, optionally sending progress
                                       notifications; logs any cancellation

Usage: probe_mcp.py --log PATH [--tag TAG]
Stdlib only. Requests are handled on threads so a cancellation can arrive
while a sleep is in progress.
"""
import json, os, sys, threading, time, argparse

ap = argparse.ArgumentParser()
ap.add_argument("--log", required=True)
ap.add_argument("--tag", default="")
args = ap.parse_args()

LOG = open(args.log, "a", buffering=1)
OUT_LOCK = threading.Lock()
LOG_LOCK = threading.Lock()
CANCELLED = set()
SESSION = f"{os.getpid()}-{int(time.time()*1000)}"


def log(kind, **kw):
    rec = {"ts": time.time(), "tag": args.tag, "session": SESSION, "kind": kind, **kw}
    with LOG_LOCK:
        LOG.write(json.dumps(rec) + "\n")


def send(msg):
    with OUT_LOCK:
        sys.stdout.write(json.dumps(msg) + "\n")
        sys.stdout.flush()
    log("out", msg=msg)


def parent_cmd():
    try:
        import subprocess
        return subprocess.run(["ps", "-o", "command=", "-p", str(os.getppid())],
                              capture_output=True, text=True).stdout.strip()[:400]
    except Exception as e:
        return repr(e)


log("start", pid=os.getpid(), ppid=os.getppid(), parent=parent_cmd(), cwd=os.getcwd(),
    env={k: v for k, v in os.environ.items()
         if k.startswith(("CLAUDE", "CC_", "MCP", "CODEX", "GROK", "GEMINI", "AGY"))})

TOOLS = [
    {"name": "probe_echo", "description": "Echo arguments and request metadata.",
     "inputSchema": {"type": "object", "properties": {"x": {"type": "string"}}}},
    {"name": "probe_sleep",
     "description": "Block for `seconds` seconds, then return. If progress_every>0, "
                    "send an MCP progress notification that often (when the client gave a progressToken).",
     "inputSchema": {"type": "object", "properties": {
         "seconds": {"type": "number"}, "progress_every": {"type": "number"}},
         "required": ["seconds"]}},
]


def result_text(rid, text, is_error=False):
    send({"jsonrpc": "2.0", "id": rid,
          "result": {"content": [{"type": "text", "text": text}], "isError": is_error}})


def handle_call(rid, params):
    name = params.get("name")
    a = params.get("arguments") or {}
    meta = params.get("_meta") or {}
    t0 = time.time()
    if name == "probe_echo":
        result_text(rid, json.dumps({"args": a, "meta": meta, "server_pid": os.getpid()}))
        return
    if name == "probe_sleep":
        secs = float(a.get("seconds", 1))
        every = float(a.get("progress_every", 0) or 0)
        tok = meta.get("progressToken")
        n = 0
        next_p = t0 + every if every > 0 else None
        while time.time() - t0 < secs:
            if rid in CANCELLED:
                log("sleep_cancelled", id=rid, after=time.time() - t0)
                return  # per spec: do not respond to a cancelled request
            if next_p and time.time() >= next_p and tok is not None:
                n += 1
                send({"jsonrpc": "2.0", "method": "notifications/progress",
                      "params": {"progressToken": tok, "progress": n,
                                 "message": f"still waiting ({int(time.time()-t0)}s)"}})
                next_p += every
            time.sleep(0.2)
        result_text(rid, f"slept {secs}s; progress notifications sent: {n}; token={tok!r}; "
                         f"server_pid={os.getpid()}")
        log("sleep_done", id=rid, secs=secs, progress_sent=n)
        return
    result_text(rid, f"unknown tool {name}", True)


def main():
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            msg = json.loads(line)
        except Exception:
            log("bad_json", line=line[:500])
            continue
        log("in", msg=msg)
        method = msg.get("method")
        rid = msg.get("id")
        if method == "initialize":
            pv = (msg.get("params") or {}).get("protocolVersion", "2025-06-18")
            send({"jsonrpc": "2.0", "id": rid, "result": {
                "protocolVersion": pv,
                "capabilities": {"tools": {}},
                "serverInfo": {"name": "probe", "version": "0.1"}}})
        elif method == "tools/list":
            send({"jsonrpc": "2.0", "id": rid, "result": {"tools": TOOLS}})
        elif method == "tools/call":
            threading.Thread(target=handle_call, args=(rid, msg.get("params") or {}), daemon=True).start()
        elif method == "notifications/cancelled":
            CANCELLED.add((msg.get("params") or {}).get("requestId"))
        elif method == "ping":
            send({"jsonrpc": "2.0", "id": rid, "result": {}})
        elif rid is not None:
            send({"jsonrpc": "2.0", "id": rid, "error": {"code": -32601, "message": f"method not found: {method}"}})
    log("stdin_closed")


try:
    main()
finally:
    log("exit")
