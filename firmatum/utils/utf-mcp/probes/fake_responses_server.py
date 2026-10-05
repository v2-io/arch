#!/usr/bin/env python3
"""A local stand-in for the OpenAI Responses API, for testing how `codex exec`
and `grok -p` wire an MCP server without spending model quota.

It records every request body to a JSONL file (so the exact `tools` array the
model would see can be inspected), then streams a scripted reply:

  request k:    the k-th call from --calls (a JSON list of objects
                {"name": ..., "arguments": {...}} or {"type": "custom", "name": ...,
                "input": "..."} or {"type": "tool_search", "arguments": {"query": ...}}),
                while any remain
  afterwards:   a short text reply quoting the start of the last tool output

    python3 tests/fake_responses_server.py --port 8765 --log /tmp/req.jsonl \
        --calls '[{"name":"tool_search","arguments":{"query":"unicode"}}]'

For grok: GROK_CLI_CHAT_PROXY_BASE_URL=http://127.0.0.1:8765 grok -p ...
For codex, run with
    -c model_provider=fake \
    -c model_providers.fake.name=fake \
    -c model_providers.fake.base_url=http://127.0.0.1:8765/v1 \
    -c model_providers.fake.wire_api=responses
"""
import argparse
import json
import sys
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

ap = argparse.ArgumentParser()
ap.add_argument("--port", type=int, default=8765)
ap.add_argument("--log", required=True)
ap.add_argument("--calls", default="[]")
opts = ap.parse_args()
state = {"n": 0}


def sse(handler, event, data):
    state["seq"] = state.get("seq", 0) + 1
    data = dict(data, sequence_number=state["seq"])
    handler.wfile.write(f"event: {event}\ndata: {json.dumps(data)}\n\n".encode())
    handler.wfile.flush()


class H(BaseHTTPRequestHandler):
    def log_message(self, *a):
        pass

    def do_GET(self):
        # model listing etc.
        body = json.dumps({"object": "list", "data": [], "models": []}).encode()
        self.send_response(200)
        self.send_header("content-type", "application/json")
        self.send_header("content-length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_POST(self):
        n = int(self.headers.get("content-length", 0))
        raw = self.rfile.read(n)
        try:
            body = json.loads(raw)
        except Exception:
            body = {"_unparsed": raw[:2000].decode("utf-8", "replace")}
        with open(opts.log, "a") as f:
            f.write(json.dumps({"path": self.path, "body": body}, ensure_ascii=False) + "\n")
        state["n"] += 1
        self.send_response(200)
        self.send_header("content-type", "text/event-stream")
        self.end_headers()
        rid = f"resp_fake_{state['n']}"
        model = body.get("model", "fake") if isinstance(body, dict) else "fake"
        base = {"id": rid, "object": "response", "created_at": 0, "model": model}
        sse(self, "response.created", {"type": "response.created", "response": dict(base, status="in_progress", output=[])})
        inputs = body.get("input", []) if isinstance(body, dict) else []
        last_out = [i for i in inputs if isinstance(i, dict) and i.get("type", "").endswith("_output")]
        calls = json.loads(opts.calls)
        k = state.get("next", 0)

        def offered(b):
            names = set()
            tools = list(b.get("tools") or [])
            for it in b.get("input") or []:
                if isinstance(it, dict) and it.get("type") in ("additional_tools", "tool_search_output"):
                    tools += it.get("tools", [])
            while tools:
                t = tools.pop()
                names.add(t.get("name") or t.get("type"))
                tools += t.get("tools", []) if t.get("type") == "namespace" else []
            return names

        # Emit the next scripted call only to a request that offers that tool
        # (side requests such as session titling get a plain text reply).
        want = None
        if k < len(calls):
            c = calls[k]
            want = c["name"] if c.get("type") != "tool_search" else "tool_search"
            if c.get("namespace"):
                want = c["namespace"]
        if want is not None and want in offered(body if isinstance(body, dict) else {}):
            state["next"] = k + 1
            if c.get("type") == "tool_search":
                item = {"type": "tool_search_call", "id": f"ts_{k}", "call_id": f"call_fake_{k}", "execution": "client",
                        "arguments": c.get("arguments", {}), "status": "completed"}
            elif c.get("type") == "custom":
                item = {"type": "custom_tool_call", "id": f"ctc_{k}", "call_id": f"call_fake_{k}", "name": c["name"], "input": c["input"], "status": "completed"}
            else:
                item = {"type": "function_call", "id": f"fc_{k}", "call_id": f"call_fake_{k}", "name": c["name"],
                        "arguments": json.dumps(c.get("arguments", {}), ensure_ascii=False), "status": "completed"}
                if c.get("namespace"):
                    item["namespace"] = c["namespace"]
        else:
            txt = "done"
            if last_out:
                out = last_out[-1].get("output")
                txt = "tool said: " + (out if isinstance(out, str) else json.dumps(out, ensure_ascii=False))[:300]
            item = {"type": "message", "id": "msg_1", "role": "assistant", "status": "completed",
                    "content": [{"type": "output_text", "text": txt, "annotations": []}]}
        sse(self, "response.output_item.added", {"type": "response.output_item.added", "output_index": 0, "item": item})
        sse(self, "response.output_item.done", {"type": "response.output_item.done", "output_index": 0, "item": item})
        sse(self, "response.completed", {"type": "response.completed", "response": dict(base, status="completed", **{
            "output": [item],
            "usage": {"input_tokens": 1, "input_tokens_details": {"cached_tokens": 0}, "output_tokens": 1,
                      "output_tokens_details": {"reasoning_tokens": 0}, "total_tokens": 2}})})


print(f"fake responses server on 127.0.0.1:{opts.port}", file=sys.stderr, flush=True)
ThreadingHTTPServer(("127.0.0.1", opts.port), H).serve_forever()
