#!/usr/bin/env python3
"""Driver-mode members: the council invokes each member once per round it is
addressed in, by resuming that member's own Claude Code session for one turn.
No member ever blocks in council_wait; nobody has to stay awake.

  driver.py COUNCIL_DIR RUN_DIR [MODEL]

- The first turn of each seat starts a session (brief + current view); later
  turns are `claude -p --resume <that session>` with the delta since its last
  turn. (A member that is a fork of a prepared expert would differ only in its
  first turn: `--resume EXPERT --fork-session`.)
- All addressed seats are invoked concurrently, so no seat is asked first.
- The driver holds presence (lease heartbeats) for the seats it drives, since
  each turn's MCP server process lives only as long as the turn.
- Member tools in this mode: council_submit, council_status, council_record.
"""
import json, os, subprocess, sys, threading, time

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, HERE)
from concilium import store, view, core  # noqa: E402

C = os.path.abspath(sys.argv[1]); RUN = os.path.abspath(sys.argv[2]); MODEL = sys.argv[3] if len(sys.argv) > 3 else "haiku"
os.makedirs(RUN, exist_ok=True)
LOG = open(os.path.join(RUN, "driver.jsonl"), "a", buffering=1)
ch = store.charter(C); toks = store.tokens(C)
SEATS = [s["id"] for s in ch["seats"]]
SID = {}; CURSOR = {s: -1 for s in SEATS}; BUSY = set(); TURNS = {s: 0 for s in SEATS}
PROC = f"driver-{os.getpid()}"
L = threading.Lock()

BRIEF_TAIL = ("\n\nHow you take part in this mode: you are not waiting in a loop. Each time a round is open for you, "
              "you will be given a turn with the view below. In that turn, read it, submit your one sealed intent with "
              "council_submit (an action from the list the view gives you), and then end your turn. You will get "
              "another turn whenever the next round is open for you. council_status and council_record are there if "
              "you want to look around first.")


def note(kind, **kw):
    LOG.write(json.dumps({"ts": time.time(), "kind": kind, **kw}) + "\n")


LOST = set()
KEYS = {s: __import__("secrets").token_hex(16) for s in SEATS}   # holder keys; never logged
LAUNCHED = {}   # seat -> round id of its last turn (de-novo-feedback-2 §2.4: one turn per seat per round)


def principal(seat):
    # the same string server.py derives from CONCILIUM_PRINCIPAL for this seat's turns
    return f"launcher:driver-{os.getpid()}-{seat}"


def hold_seats():
    """verify-1 test A: the driver, not each per-turn server process, holds the
    seats it drives -- claimed here, refreshed every heartbeat, released when
    the driver ends. A seat found held by another principal is lost: the driver
    stops driving it and says so, rather than relaunching into refusals."""
    for s in SEATS:
        if s in LOST:
            continue
        ok, h = store.claim_seat(C, s, principal(s), {"proc": PROC, "client": "driver"}, key=KEYS[s])
        if not ok:
            LOST.add(s)
            note("seat-lost", seat=s, holder=h["principal"], epoch=h["epoch"])


def heartbeat():
    while True:
        for s in SEATS:
            if s not in LOST:
                store.refresh_seat(C, s, principal(s), KEYS[s])   # keeps the lease; not the seat acting
                h = store.holder(C, s)
                if not h or not store._same_holder(h, principal(s), KEYS[s]):
                    LOST.add(s)
                    note("seat-lost", seat=s, holder=(h or {}).get("principal"), epoch=(h or {}).get("epoch"))
        for s in SEATS:
            if s not in LOST:
                store.heartbeat(C, s, PROC, {"pid": os.getpid(), "client": "driver", "busy": s in BUSY})
        store.try_close(C)
        time.sleep(5)


def turn(seat):
    d = os.path.join(RUN, seat); os.makedirs(os.path.join(d, "cwd"), exist_ok=True)
    txt, last = view.render(C, seat, CURSOR[seat])
    vant = next(s["vantage"] for s in ch["seats"] if s["id"] == seat)
    if seat not in SID:
        prompt = (open(os.path.join(HERE, "members", "brief.md")).read()
                  .replace("{SEAT}", seat).replace("{VANTAGE}", vant)
                  .split("Please keep going until")[0] + BRIEF_TAIL + "\n\n" + txt)
        extra = []
    else:
        prompt = "A round is open for you. Here is what changed and what you may do:\n\n" + txt
        extra = ["--resume", SID[seat]]
    cfg = {"mcpServers": {"council": {"type": "stdio", "command": "/usr/bin/python3",
           "args": ["-I", os.path.join(HERE, "concilium", "server.py"), "--dir", C, "--seat", seat,
                    "--log", os.path.join(d, "server.jsonl"), "--tag", f"{seat}-t{TURNS[seat]}"],
           "env": {"CONCILIUM_TOKEN": toks[seat], "CONCILIUM_PRINCIPAL": f"driver-{os.getpid()}-{seat}",
                   "CONCILIUM_HOLDER_KEY": KEYS[seat]}}}}
    cmd = ["claude", "-p", prompt, "--model", MODEL, "--mcp-config", json.dumps(cfg), "--strict-mcp-config",
           "--tools", "", "--allowedTools", "mcp__council__council_submit,mcp__council__council_status,mcp__council__council_record",
           "--disallowedTools", "mcp__council__council_wait",
           "--setting-sources", "", "--output-format", "stream-json", "--verbose", *extra]
    t0 = time.time()
    note("turn-start", seat=seat, turn=TURNS[seat], resume=SID.get(seat), cursor=CURSOR[seat])
    out = subprocess.run(cmd, cwd=os.path.join(d, "cwd"), env=dict(os.environ, CLAUDE_CODE_DISABLE_AUTO_MEMORY="1"),
                         stdin=subprocess.DEVNULL, capture_output=True, text=True)
    with open(os.path.join(d, "stream.jsonl"), "a") as f:
        f.write(out.stdout)
    res = {}
    for l in out.stdout.splitlines():
        try:
            m = json.loads(l)
        except Exception:
            continue
        if m.get("type") == "system" and m.get("subtype") == "init":
            res["session"] = m["session_id"]
        if m.get("type") == "result":
            res.update(cost=m.get("total_cost_usd"), turns=m.get("num_turns"), usage=m.get("usage"),
                       is_error=m.get("is_error"), text=(m.get("result") or "")[:300])
    with L:
        if seat not in SID and res.get("session"):
            SID[seat] = res["session"]
        CURSOR[seat] = last
        TURNS[seat] += 1
        BUSY.discard(seat)
    r_now = store.state(C)[1]["round"]
    acted = bool(r_now is None or seat in store.sealed(C, LAUNCHED.get(seat, "")) or r_now["id"] != LAUNCHED.get(seat))
    if not acted:
        note("turn-no-act", seat=seat, round=LAUNCHED.get(seat),
             why="the turn ended without a submission; no second turn for this round")
    note("turn-end", seat=seat, secs=round(time.time() - t0, 1), rc=out.returncode, **res,
         stderr=out.stderr[-300:])


def main():
    hold_seats()
    threading.Thread(target=heartbeat, daemon=True).start()
    while True:
        store.try_close(C)
        cht, st = store.state(C)
        if st["adjourned"]:
            note("adjourned")
            break
        r = st["round"]
        if r:
            subs = store.sealed(C, r["id"])
            for s in r["addressed"]:
                with L:
                    go = s not in subs and s not in BUSY and s not in LOST and LAUNCHED.get(s) != r["id"]
                    if go:
                        BUSY.add(s)
                        LAUNCHED[s] = r["id"]
                if go:
                    threading.Thread(target=turn, args=(s,), daemon=True).start()
        time.sleep(1)
    while BUSY:
        time.sleep(1)
    for s in SEATS:
        if s not in LOST:
            store.release_seat(C, s, principal(s), "driver finished", KEYS[s])
    json.dump({"sessions": SID, "turns": TURNS}, open(os.path.join(RUN, "driver-summary.json"), "w"), indent=1)


if __name__ == "__main__":
    main()
