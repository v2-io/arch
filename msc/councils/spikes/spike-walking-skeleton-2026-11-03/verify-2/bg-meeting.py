#!/usr/bin/env python3
"""verify-2: seat a `claude --bg` member on a real council through the real
server (which sends progress every 10 s), and make it wait past the harness's
2-minute auto-background point before its turn comes. The spike's only evidence
for the backgrounded wait (`probes/b1`) used the probe server with
progress_every=0, so this is the first run in the configuration the plan
proposes to build on.

  python3 verify-2/bg-meeting.py          (from the spike dir)

Seats: A = the --bg haiku member (members/brief.md, unchanged); B = this script.
B answers the roll only after A has, plus 150 s; from then on B answers every
round within a second. Everything is timestamped into
verify-2/runs/bg-wait/orchestrator.jsonl; the server log is A/server.jsonl;
the session's own transcript is copied (trimmed) to session-extract.jsonl.
The background session is stopped at the end (`claude stop`).
"""
import json, os, subprocess, sys, time, glob, shutil

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, HERE)
from concilium import store, core  # noqa: E402

RUN = os.path.join(HERE, "verify-2", "runs", os.environ.get("BG_RUN", "bg-wait"))
MODEL = os.environ.get("BG_MODEL", "haiku")
C = os.path.join(RUN, "council")
SCRATCH = os.environ.get("BG_CWD") or os.path.join(os.path.dirname(os.environ.get("TMPDIR", "/tmp")), "bgcwd2")
CWD = os.path.join(SCRATCH)                      # outside any git repo, so --bg makes no worktree
os.makedirs(CWD, exist_ok=True)
os.makedirs(os.path.join(RUN, "A"), exist_ok=True)
LOG = open(os.path.join(RUN, "orchestrator.jsonl"), "a", buffering=1)
T0 = time.time()


def note(kind, **kw):
    rec = {"ts": time.time(), "t": round(time.time() - T0, 1), "kind": kind, **kw}
    LOG.write(json.dumps(rec) + "\n")
    print(json.dumps(rec), flush=True)


CHARTER = {
    "council": "verify2-bg-wait",
    "purpose": "verify-2: a claude --bg member waits through the real server past the 120 s auto-background point; does the server's 'your turn' wake it?",
    "parent": "verify-2 (nothing decided here is acted on)",
    "seats": [{"id": "A", "vantage": "the member whose wait is being tested; any view you have on the question", "model": os.environ.get("BG_MODEL", "haiku"), "harness": "claude --bg"},
              {"id": "B", "vantage": "a scripted seat that answers the roll late on purpose", "model": "script", "harness": "verify-2/bg-meeting.py"}],
    "quorum": 2,
    "round_policy": {"backstop_s": 900, "arrival_grace_s": 300, "lease_s": 30, "replaceable": False, "max_inquorate_rounds": 2},
    "debate_policy": {"max_debate_rounds": 2, "max_open_rounds": 2},
    "agenda": [{"id": "q1", "title": "Should a council member that waits in the background be told, when it wakes, how long it was idle?",
                "brief": "A mechanics question for the record only. Move, discuss or pass as you see fit."}],
}

if os.path.exists(C):
    print("council exists; remove", RUN, "to re-run"); sys.exit(2)
toks = store.create(C, CHARTER)
note("created", council=C, cwd=CWD)

brief = (open(os.path.join(HERE, "members", "brief.md")).read()
         .replace("{SEAT}", "A").replace("{VANTAGE}", CHARTER["seats"][0]["vantage"]))
cfg = {"mcpServers": {"council": {"type": "stdio", "command": "/usr/bin/python3",
       "args": ["-I", os.path.join(HERE, "concilium", "server.py"), "--dir", C, "--seat", "A",
                "--log", os.path.join(RUN, "A", "server.jsonl"), "--tag", "A", "--max-wait", "600"],
       "env": {"CONCILIUM_TOKEN": toks["A"]}}}}
cmd = ["claude", "--bg", "-n", "concilium-verify2-" + os.path.basename(RUN), "--model", MODEL,
       "--mcp-config", json.dumps(cfg), "--strict-mcp-config", "--tools", "",
       "--allowedTools", "mcp__council__council_wait,mcp__council__council_submit,mcp__council__council_status,mcp__council__council_record",
       "--setting-sources", "", brief]
env = dict(os.environ, CLAUDE_CODE_DISABLE_AUTO_MEMORY="1")
out = subprocess.run(cmd, cwd=CWD, env=env, capture_output=True, text=True, stdin=subprocess.DEVNULL)
open(os.path.join(RUN, "bg-launch.txt"), "w").write(out.stdout + out.stderr)
bg_id = None
for tok in out.stdout.split():
    if len(tok) == 8 and all(c in "0123456789abcdef" for c in tok):
        bg_id = tok; break
note("bg-launched", rc=out.returncode, id=bg_id, stdout=out.stdout.strip()[:300], stderr=out.stderr.strip()[:300])

ACT = {"call-to-order": {"action": "present"}, "open": {"action": "pass"}, "debate": {"action": "pass"},
       "vote": {"action": "vote", "vote": "yes"}}


def st_():
    return core.fold(store.charter(C), store.events(C))


def wait_for(pred, limit, what):
    t = time.time()
    while time.time() - t < limit:
        if pred():
            note("saw", what=what, after=round(time.time() - t, 1)); return True
        time.sleep(0.5)
    note("timeout", what=what, after=limit); return False


def stop_bg():
    if bg_id:
        s = subprocess.run(["claude", "stop", bg_id], capture_output=True, text=True)
        note("bg-stopped", rc=s.returncode, out=(s.stdout + s.stderr).strip()[:200])


# 1. A answers the roll (its first council_wait returns at once: r1 is open)
if not wait_for(lambda: "A" in store.sealed(C, "r1"), 180, "A present in r1"):
    stop_bg(); sys.exit(1)
t_a1 = time.time()
# 2. B stays silent 150 s: A's next council_wait should cross the 120 s auto-background point
note("holding", seconds=150, why="A's wait must run past 120 s before its turn comes")
time.sleep(150)
heads = [json.loads(l) for l in open(os.path.join(RUN, "A", "server.jsonl")) if '"notifications/progress"' in l]
note("progress-sent-so-far", n=len(heads))
ok, msg = store.submit(C, "B", "r1", ACT["call-to-order"], {"orchestrator": True})
note("B-present", ok=ok, msg=msg[:80], since_A_present=round(time.time() - t_a1, 1))
# 3. does A act in r2 (woken by the server's answer to its backgrounded wait)?
r2_t0 = time.time()
woke = wait_for(lambda: "A" in store.sealed(C, "r2") or st_()["round"] is None or st_()["round"]["id"] != "r2", 240, "A acted in r2 (or r2 closed)")
note("A-in-r2", acted="A" in store.sealed(C, "r2"), after=round(time.time() - r2_t0, 1))
# 4. from here B answers promptly until adjournment
t_end = time.time() + 600
while time.time() < t_end:
    st = st_()
    if st["adjourned"]:
        note("adjourned"); break
    r = st["round"]
    if r and "B" not in store.sealed(C, r["id"]):
        store.submit(C, "B", r["id"], ACT[r["kind"]], {"orchestrator": True})
        note("B-submitted", round=r["id"], round_kind=r["kind"])   # attempt 2 crashed here on a kwarg collision (kind=); fixed after
    store.try_close(C)
    time.sleep(1)
time.sleep(30)                                   # let A write its closing note
# 5. stop the background session and copy its transcript (trimmed) beside the run
stop_bg()
sid = None
for l in open(os.path.join(RUN, "A", "server.jsonl")):
    r = json.loads(l)
    if r.get("kind") == "start":
        sid = r.get("env", {}).get("CLAUDE_CODE_SESSION_ID"); break
note("session-id", sid=sid)
if sid:
    for p in glob.glob(os.path.expanduser(f"~/.claude/projects/*/{sid}.jsonl")):
        with open(os.path.join(RUN, "session-extract.jsonl"), "w") as f:
            for l in open(p):
                try:
                    m = json.loads(l)
                except Exception:
                    continue
                keep = {"type": m.get("type"), "timestamp": m.get("timestamp")}
                msg = m.get("message") or {}
                c = msg.get("content")
                if isinstance(c, str):
                    keep["text"] = c[:400]
                elif isinstance(c, list):
                    keep["content"] = [{k: (v if k != "text" and k != "input" else (v[:600] if isinstance(v, str) else v))
                                        for k, v in b.items() if k in ("type", "name", "input", "text", "tool_use_id")}
                                       if isinstance(b, dict) else b for b in c]
                    for b in keep["content"]:
                        if isinstance(b, dict) and isinstance(b.get("content"), list):
                            b["content"] = [{"text": (x.get("text") or "")[:600]} for x in b["content"] if isinstance(x, dict)]
                if m.get("type") in ("user", "assistant", "system", "permission-mode", "task-notification") or "task" in str(m.get("type")):
                    f.write(json.dumps(keep) + "\n")
        note("transcript-copied", src=p)
note("done")
