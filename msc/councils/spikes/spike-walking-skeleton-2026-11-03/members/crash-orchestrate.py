#!/usr/bin/env python3
"""trial-3: absence, crash, return and a duplicate seat holder, with claude -p members.

  crash-orchestrate.py RUN_DIR CHARTER.json

1. three members (sec, rec, ses) start; ses's claude process is killed (SIGTERM)
   once a motion is pending on q1;
2. after two more rounds close without it, ses is relaunched as
   `claude -p --resume <its session> --fork-session` (same seat, new process);
3. as soon as the resumed ses has acted once, a second, fresh session claims seat
   ses too (same token): two live holders of one seat.
Everything the orchestrator does is appended to RUN_DIR/orchestrator.jsonl with
wall-clock times, so the record shows what was done to whom and when.
"""
import json, os, signal, subprocess, sys, time

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RUN = os.path.abspath(sys.argv[1]); CH = sys.argv[2]
C = os.path.join(RUN, "council")
LOG = open(os.path.join(RUN, "orchestrator.jsonl"), "a", buffering=1)


def note(kind, **kw):
    LOG.write(json.dumps({"ts": time.time(), "kind": kind, **kw}) + "\n")
    print(kind, kw, flush=True)


def events():
    try:
        return [json.loads(l) for l in open(os.path.join(C, "events.jsonl")) if l.strip()]
    except FileNotFoundError:
        return []


def wait_for(pred, timeout=1200, what=""):
    t0 = time.time()
    while time.time() - t0 < timeout:
        evs = events()
        if pred(evs):
            return evs
        if any(e["type"] == "adjourned" for e in evs):
            note("adjourned-before", what=what)
            return None
        time.sleep(1)
    note("timeout", what=what)
    return None


def launch(seat, sub, extra=(), prompt=None):
    os.makedirs(os.path.join(RUN, sub), exist_ok=True)
    env = dict(os.environ)
    args = [os.path.join(HERE, "members", "claude-member.sh"), C, seat, os.path.join(RUN, "_tmp"), "haiku"]
    # claude-member.sh writes under RUN/SEAT; use a per-launch subdir by symlink trick:
    # simpler: call claude directly here, mirroring claude-member.sh exactly.
    tok = json.load(open(os.path.join(C, "tokens.json")))[seat]
    vant = next(s["vantage"] for s in json.load(open(os.path.join(C, "charter.json")))["seats"] if s["id"] == seat)
    brief = open(os.path.join(HERE, "members", "brief.md")).read().replace("{SEAT}", seat).replace("{VANTAGE}", vant)
    d = os.path.join(RUN, sub)
    cfg = {"mcpServers": {"council": {"type": "stdio", "command": "/usr/bin/python3",
           "args": ["-I", os.path.join(HERE, "concilium", "server.py"), "--dir", C, "--seat", seat,
                    "--log", os.path.join(d, "server.jsonl"), "--tag", sub],
           "env": {"CONCILIUM_TOKEN": tok}}}}
    cwd = os.path.join(RUN, "cwd-" + seat)       # same cwd for a seat, so --resume finds its session
    os.makedirs(cwd, exist_ok=True)
    cmd = ["claude", "-p", prompt or brief, "--model", "haiku", "--mcp-config", json.dumps(cfg),
           "--strict-mcp-config", "--tools", "",
           "--allowedTools", "mcp__council__council_wait,mcp__council__council_submit,mcp__council__council_status,mcp__council__council_record",
           "--setting-sources", "", "--output-format", "stream-json", "--verbose", *extra]
    env["CLAUDE_CODE_DISABLE_AUTO_MEMORY"] = "1"
    p = subprocess.Popen(cmd, cwd=cwd, env=env, stdin=subprocess.DEVNULL,
                         stdout=open(os.path.join(d, "stream.jsonl"), "a"),
                         stderr=open(os.path.join(d, "stderr.txt"), "a"), start_new_session=True)
    note("launched", seat=seat, sub=sub, pid=p.pid, extra=list(extra))
    return p


def session_id(sub):
    for l in open(os.path.join(RUN, sub, "stream.jsonl")):
        m = json.loads(l)
        if m.get("type") == "system" and m.get("subtype") == "init":
            return m["session_id"]


def main():
    if not os.path.exists(C):
        subprocess.run([sys.executable, "-m", "concilium.steward", "new", C, CH], cwd=HERE, check=True,
                       stdout=subprocess.DEVNULL)
    procs = {s: launch(s, s) for s in ("sec", "rec", "ses")}
    evs = wait_for(lambda e: any(x["type"] == "motion-pending" for x in e), what="motion pending")
    if evs is None:
        return
    # kill ses's whole process group (claude + its MCP server)
    os.killpg(procs["ses"].pid, signal.SIGTERM)
    note("killed", seat="ses", pid=procs["ses"].pid, at_seq=evs[-1]["seq"])
    procs["ses"].wait()
    closes_at_kill = sum(1 for x in evs if x["type"] == "round-closed")
    evs = wait_for(lambda e: sum(1 for x in e if x["type"] == "round-closed") >= closes_at_kill + 2,
                   what="two rounds closed without ses")
    if evs is None:
        return
    sid = session_id("ses")
    note("resuming", seat="ses", session=sid, at_seq=evs[-1]["seq"])
    resume_prompt = ("Your session was interrupted: your connection to the council dropped while the meeting "
                     "went on, and this is a fresh connection for the same seat. Please carry on as before: "
                     "call council_record or council_status to see what happened while you were away, then "
                     "council_wait, and keep going until the meeting adjourns.")
    procs["ses2"] = launch("ses", "ses-resumed", extra=("--resume", sid, "--fork-session"), prompt=resume_prompt)
    # duplicate holder once the resumed ses has submitted something
    def resumed_acted(_e):
        try:
            return any('"kind": "submitted"' in l and '"seat": "ses"' in l for l in open(os.path.join(C, "receipts.jsonl"))
                       if json.loads(l)["ts"] > procs_t0["ses2"])
        except Exception:
            return False
    procs_t0["ses2"] = time.time()
    wait_for(resumed_acted, what="resumed ses acted")
    procs["dup"] = launch("ses", "ses-duplicate")
    for k, p in procs.items():
        if k != "ses":
            p.wait()
            note("exited", which=k, rc=p.returncode)


procs_t0 = {}
if __name__ == "__main__":
    main()
