"""Send a request.json exported by the auth-probe mod's /fold-preview, streaming, and save the result.

The key is read from ~/anthropic-default-api-key and never printed or written. Betas come from the
export's export-meta.json. Writes, beside request.json: response-meta.json (status, stop reason,
stop details, usage, input_transformations, request id, org), summary.md and block.json.

Usage: python3 send_stream.py runs/06-fold-preview/<timestamp>
"""
import json, os, sys, time, urllib.request, urllib.error

d = sys.argv[1]
body = open(os.path.join(d, "request.json"), "rb").read()
betas = json.load(open(os.path.join(d, "export-meta.json")))["betas"]
key = open(os.path.expanduser("~/anthropic-default-api-key")).read().strip()

req_body = json.loads(body)
req_body["stream"] = True
req = urllib.request.Request("https://api.anthropic.com/v1/messages", data=json.dumps(req_body).encode(), method="POST")
for k, v in {"x-api-key": key, "anthropic-version": "2023-06-01", "content-type": "application/json",
             "anthropic-beta": ",".join(betas)}.items():
    req.add_header(k, v)

meta, block, t0 = {"betas": betas}, None, time.time()
try:
    with urllib.request.urlopen(req, timeout=1800) as r:
        meta["status"] = r.status
        meta["request_id"] = r.headers.get("request-id")
        meta["org"] = r.headers.get("anthropic-organization-id")
        event = None
        for raw in r:
            line = raw.decode("utf-8", "replace").rstrip("\n")
            if line.startswith("event:"):
                event = line[6:].strip()
            elif line.startswith("data:"):
                data = json.loads(line[5:])
                if event == "message_start":
                    msg = data.get("message", {})
                    meta["input_transformations"] = msg.get("input_transformations")
                    meta["usage_start"] = msg.get("usage")
                elif event == "content_block_start" and data.get("content_block", {}).get("type") == "compaction":
                    block = data["content_block"]
                elif event == "content_block_delta" and data.get("delta", {}).get("type") == "compaction_delta" and block is not None:
                    block["content"] = (block.get("content") or "") + (data["delta"].get("content") or "")
                elif event == "message_delta":
                    meta["stop_reason"] = data.get("delta", {}).get("stop_reason")
                    meta["stop_details"] = data.get("delta", {}).get("stop_details")
                    meta["usage"] = data.get("usage")
                elif event == "error":
                    meta["stream_error"] = data
except urllib.error.HTTPError as e:
    meta["status"] = e.code
    meta["request_id"] = e.headers.get("request-id")
    try:
        meta["error"] = json.loads(e.read())
    except ValueError:
        meta["error"] = "unparsed"
meta["seconds"] = round(time.time() - t0, 1)
json.dump(meta, open(os.path.join(d, "response-meta.json"), "w"), indent=2)
if block is not None:
    json.dump(block, open(os.path.join(d, "block.json"), "w"))
    if block.get("content"):
        open(os.path.join(d, "summary.md"), "w").write(block["content"])
print(json.dumps({k: meta.get(k) for k in ("status", "stop_reason", "stop_details", "seconds", "request_id", "error")}, indent=2))
print("input_transformations:", len(meta.get("input_transformations") or []), "| summary chars:", len((block or {}).get("content") or ""))
