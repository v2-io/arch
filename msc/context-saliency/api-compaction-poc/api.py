"""Minimal Messages API client for the compaction proof of concept.

Auth: Claude Code's OAuth access token, read from the macOS keychain at call time.
The token is never printed, logged, or written to disk. Every request body and
response body is written whole to the run directory (headers other than
anthropic-beta are not recorded).
"""
import json, os, subprocess, sys, time, urllib.request, urllib.error

API = "https://api.anthropic.com/v1/messages"
OAUTH_BETA = "oauth-2025-04-20"


def _token():
    raw = subprocess.run(
        ["security", "find-generic-password", "-s", "Claude Code-credentials", "-w"],
        capture_output=True, text=True, check=True,
    ).stdout
    o = json.loads(raw)["claudeAiOauth"]
    if o["expiresAt"] / 1000 < time.time() + 60:
        sys.exit("OAuth token expires within a minute; refresh it by using Claude Code, then rerun.")
    return o["accessToken"]


class Run:
    def __init__(self, outdir):
        self.outdir = outdir
        os.makedirs(outdir, exist_ok=True)
        self.n = 0

    def call(self, label, body, betas=()):
        self.n += 1
        betas = [OAUTH_BETA, *betas]
        stem = os.path.join(self.outdir, f"{self.n:02d}-{label}")
        with open(stem + ".request.json", "w") as f:
            json.dump({"anthropic-beta": betas, "body": body}, f, indent=2)
        req = urllib.request.Request(API, data=json.dumps(body).encode(), method="POST")
        key = os.environ.get("ANTHROPIC_API_KEY")
        if key:
            betas = [b for b in betas if b != OAUTH_BETA]
            req.add_header("x-api-key", key)
        else:
            req.add_header("authorization", "Bearer " + _token())
        req.add_header("anthropic-version", "2023-06-01")
        req.add_header("anthropic-beta", ",".join(betas))
        req.add_header("content-type", "application/json")
        t0 = time.time()
        try:
            with urllib.request.urlopen(req, timeout=600) as r:
                status, data = r.status, json.loads(r.read())
        except urllib.error.HTTPError as e:
            status, txt = e.code, e.read().decode("utf-8", "replace")
            try:
                data = json.loads(txt)
            except ValueError:
                data = {"raw": txt}
        dt = time.time() - t0
        with open(stem + ".response.json", "w") as f:
            json.dump({"status": status, "seconds": round(dt, 2), "data": data}, f, indent=2)
        return status, data
