"""concilium stdio MCP server: one process per member harness session.

  python3 -I path/to/concilium/server.py --dir COUNCIL [--seat S --token T] [--log PATH]
          [--max-wait S] [--progress-every S] [--tag TAG]

The seat is fixed at launch by --seat/--token (or CONCILIUM_SEAT/CONCILIUM_TOKEN),
and every tool also takes an optional `seat_token`, which overrides it: that
is how several seats can share one connection (a Claude Code subagent that
references its parent's server by name shares the parent's process).

Every JSON-RPC message in and out is appended to --log with the process id,
so a run can be reconstructed without trusting any model's account of it.

Delivery, and the two rules that come from "a returned result is not a delivered
result" (harness-notes; de-novo-feedback-2 §2.3):
- the seat's "since your last look" cursor advances only when the seat's
  submission is accepted (its own act), never when the server sends a view, so
  every view a member could act on carries what it has not yet acted after;
- a newer council_wait on a seat supersedes an older one still pending in this
  process, which then returns without rendering (a harness may have abandoned it).
"""
import argparse
import json
import secrets
import os
import sys
import threading
import time
import traceback

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from concilium import store, core, view  # noqa: E402

ap = argparse.ArgumentParser()
ap.add_argument("--dir", required=True)
ap.add_argument("--seat", default=os.environ.get("CONCILIUM_SEAT"))
ap.add_argument("--token", default=os.environ.get("CONCILIUM_TOKEN"))
ap.add_argument("--log", default=None)
ap.add_argument("--tag", default="")
ap.add_argument("--max-wait", type=float, default=float(os.environ.get("CONCILIUM_MAX_WAIT", 600)))
ap.add_argument("--progress-every", type=float, default=10.0)
ap.add_argument("--heartbeat", type=float, default=5.0)
A = ap.parse_args()
D = os.path.abspath(A.dir)
PROC = f"{os.uname().nodename.split('.')[0]}-{os.getpid()}-{int(time.time())}"
OUT = threading.Lock()
LOGL = threading.Lock()
LOGF = open(A.log, "a", buffering=1) if A.log else None
CLIENT = {}
CANCELLED = set()
WAIT_GEN = {}         # seat -> generation of the newest council_wait in this process
# exclusivity is keyed to (principal, holder key): the principal is what the
# caller *asserts* (de-novo-feedback-2 §2.5); the key is a secret this
# connection holds (or its launcher, for a driver), never sent by the client
HOLDER_KEY = os.environ.get("CONCILIUM_HOLDER_KEY") or secrets.token_hex(16)


def log(kind, **kw):
    if not LOGF:
        return
    with LOGL:
        LOGF.write(json.dumps({"ts": time.time(), "proc": PROC, "tag": A.tag, "kind": kind, **kw}) + "\n")


def send(msg):
    with OUT:
        sys.stdout.write(json.dumps(msg) + "\n")
        sys.stdout.flush()
    log("out", msg=msg)


def reply(rid, text, err=False):
    send({"jsonrpc": "2.0", "id": rid, "result": {"content": [{"type": "text", "text": text}], "isError": err}})


def resolve_seat(args):
    tok = args.get("seat_token")
    toks = store.tokens(D)
    if tok:
        for s, t in toks.items():
            if t == tok:
                return s, "token-argument"
        if A.seat:
            # trial-1: a model filled the optional field with its seat name.
            # The field is no longer advertised to single-seat servers; if a
            # model still sends one, say exactly what happened.
            return None, (f"seat_token was given but matches no seat. This connection already holds seat "
                          f"{A.seat!r}: call again without seat_token.")
        return None, ("seat_token matches no seat. Use the exact token you were given "
                      "(not your seat name).")
    if A.seat:
        if A.token and toks.get(A.seat) != A.token:
            return None, "launch token does not match seat"
        return A.seat, "launch-config"
    return None, "no seat: launch with --seat/--token or pass seat_token"


def principal_for(meta):
    """Who the caller says is acting (trial-3: seats need one holder, and the
    holder is a session or a driver, not a process). These are recorded
    assertions, not attestations: codex's sessionId and agy's conversation id are
    strings the client sends, and any client can send them. Exclusivity therefore
    also requires the holder key (HOLDER_KEY), which the client never sees.
    Order: an explicit principal from the launcher (a driver passes its own id,
    so its per-turn server processes are one holder); a session id the harness
    puts in _meta (codex sessionId, agy root conversation); else the harness
    process that launched this server (our parent: one Claude Code / grok
    session process)."""
    if os.environ.get("CONCILIUM_PRINCIPAL"):
        return "launcher:" + os.environ["CONCILIUM_PRINCIPAL"]
    m = meta or {}
    if m.get("sessionId"):
        return "codex-session:" + m["sessionId"]
    if m.get("antigravity.google/root_conversation_id"):
        return "agy-conversation:" + m["antigravity.google/root_conversation_id"]
    return f"parent-process:{os.uname().nodename.split('.')[0]}:{os.getppid()}"


PRINCIPALS = {}   # seat -> principal this process acts as


def attest(meta, how):
    """What the caller asserted about itself, recorded as such (not verified)."""
    return {"proc": PROC, "client": CLIENT, "seat_from": how,
            "meta_keys": sorted((meta or {}).keys()),
            "tool_use_id": (meta or {}).get("claudecode/toolUseId") or (meta or {}).get("callId")}


def heartbeat_loop():
    while True:
        # presence is heartbeated only for seats this process actually holds: a
        # refused duplicate must not make a dead holder look alive
        for s in list(PRINCIPALS):
            if s in PRINCIPALS:
                try:
                    store.refresh_seat(D, s, PRINCIPALS[s], HOLDER_KEY)
                except Exception:
                    log("refresh-error", err=traceback.format_exc())
            try:
                store.heartbeat(D, s, PROC, {"pid": os.getpid(), "client": CLIENT.get("name")})  # diagnostics only
            except Exception:
                log("heartbeat-error", err=traceback.format_exc())
        try:
            store.try_close(D)     # any live process can close a round whose condition holds
        except Exception:
            log("tick-error", err=traceback.format_exc())
        time.sleep(A.heartbeat)


TOOLS_ALL = [
    {"name": "council_status",
     "description": "Where the council stands for your seat: the agenda item, any pending motion, what happened since you last looked, and — if a round is open for you — exactly what you may submit, with reasons. Never blocks.",
     "inputSchema": {"type": "object", "properties": {"seat_token": {"type": "string"},
                                                      "since": {"type": "integer", "description": "show events after this seq (default: since your last look)"}},
                     "additionalProperties": False}},
    {"name": "council_wait",
     "description": "Block until a round is open that you have not yet answered, or the meeting adjourns, then return the same view as council_status. Returns early (saying so) after max_wait_s; just call it again. This is how you wait your turn: nobody is penalized for answering slowly.",
     "inputSchema": {"type": "object", "properties": {"seat_token": {"type": "string"},
                                                      "max_wait_s": {"type": "number"}},
                     "additionalProperties": False}},
    {"name": "council_submit",
     "description": "Submit your sealed intent for the open round. Nobody sees it until the round closes, when everyone's are revealed together. {REPLACE} `action` must be one of those listed for you in the view.",
     "inputSchema": {"type": "object", "properties": {
         "round": {"type": "string"}, "action": {"type": "string"},
         "text": {"type": "string"}, "vote": {"type": "string", "enum": ["yes", "no", "abstain"]},
         "seat_token": {"type": "string"}},
         "required": ["round", "action"], "additionalProperties": False}},
    {"name": "council_record",
     "description": "The council's full record so far (minutes): every round, every revealed intent, every outcome. For catching up after arriving late or restarting.",
     "inputSchema": {"type": "object", "properties": {"seat_token": {"type": "string"}},
                     "additionalProperties": False}},
]


def tools_for_this_process():
    """Single-seat servers do not advertise seat_token at all (trial-1 finding:
    an optional identity field visible to a model gets filled with a guess).
    Multi-seat servers (launched without --seat, e.g. one connection shared by
    several subagents) require it."""
    out = []
    replaceable = store.charter(D).get("round_policy", {}).get("replaceable", False)
    for t in TOOLS_ALL:
        t = json.loads(json.dumps(t))
        # verify-1 D5: the description promised replacement while every charter forbade it
        t["description"] = t["description"].replace(
            "{REPLACE}", "You may replace it until the round closes; only your last counts." if replaceable
            else "Your submission is final for the round: think before you submit.")
        props = t["inputSchema"]["properties"]
        if A.seat:
            props.pop("seat_token", None)
        else:
            props["seat_token"]["description"] = "the secret token for your seat, exactly as given to you (not the seat name)"
            t["inputSchema"].setdefault("required", []).append("seat_token")
        out.append(t)
    return out


def witness(seat):
    """Record the chain head this process is about to show a member, in this
    process's own log (which lives with the member's run, not in the council
    directory). steward witness checks a council log against these: a rewrite of
    any event a member was shown contradicts a head it recorded (verify-1 test E)."""
    evs = store.events(D)
    if evs:
        log("head-observed", seat=seat, seq=evs[-1]["seq"], hash=evs[-1]["hash"])


def tool_call(rid, params):
    name = params.get("name")
    args = params.get("arguments") or {}
    meta = params.get("_meta") or {}
    seat, how = resolve_seat(args)
    if not seat:
        reply(rid, how, True)
        return
    at = attest(meta, how)
    principal = principal_for(meta)
    ok, h = store.claim_seat(D, seat, principal, at, key=HOLDER_KEY)
    if not ok:
        ago = int(time.time() - h["since"])
        reply(rid, (f"Seat {seat} is already held by another live connection ({h['principal']}, holder since {ago}s ago, "
                    f"epoch {h['epoch']}). One seat has one holder at a time: two would let whichever answers first "
                    f"speak for the seat. This connection was not given the seat. If the other connection is gone, its "
                    f"hold lapses on its own after the lease; otherwise stop here."), True)
        return
    PRINCIPALS[seat] = principal
    at["principal"] = principal
    at["epoch"] = h["epoch"]
    store.receipt(D, "call", seat=seat, tool=name, attest=at)
    if name == "council_status":
        witness(seat)
        txt, last = view.render(D, seat, store.cursor(D, seat, PRINCIPALS[seat]) if args.get("since") is None else args["since"])
        reply(rid, txt)
    elif name == "council_submit":
        intent = {k: args[k] for k in ("action", "text", "vote") if k in args}
        ok, msg = store.submit(D, seat, args.get("round", ""), intent, at)
        if ok:
            # the member acted on round R, so it held every event up to R's opening:
            # that, and only that, is acknowledged (the reveal of R comes later)
            store.set_cursor(D, seat, PRINCIPALS[seat], store.round_opened_seq(D, args["round"]))
        witness(seat)
        txt, last = view.render(D, seat, store.cursor(D, seat, PRINCIPALS[seat]))
        reply(rid, ("OK: " if ok else "REFUSED: ") + msg + "\n\n" + txt, not ok)
    elif name == "council_wait":
        maxw = float(args.get("max_wait_s") or A.max_wait)
        tok = meta.get("progressToken")
        t0 = time.time()
        nextp = t0 + A.progress_every
        WAIT_GEN[seat] = gen = WAIT_GEN.get(seat, 0) + 1
        n = 0
        why = "timeout"
        if True:
            while True:
                if WAIT_GEN.get(seat) != gen:
                    store.receipt(D, "wait-superseded", seat=seat, after=round(time.time() - t0, 2), proc=PROC)
                    reply(rid, "This council_wait was superseded by a newer council_wait on your seat; "
                               "the newer one carries the view. Nothing to do here.")
                    return
                if rid in CANCELLED:
                    log("wait-cancelled", id=rid, seat=seat, after=time.time() - t0)
                    store.receipt(D, "wait-cancelled", seat=seat, after=time.time() - t0)
                    return
                store.try_close(D)
                ch, st = store.state(D)
                r = st["round"]
                if st["adjourned"]:
                    why = "adjourned"
                    break
                if r and seat in r["addressed"] and seat not in store.sealed(D, r["id"]):
                    why = "your-turn"
                    break
                if time.time() - t0 >= maxw:
                    break
                if tok is not None and time.time() >= nextp:
                    n += 1
                    send({"jsonrpc": "2.0", "method": "notifications/progress",
                          "params": {"progressToken": tok, "progress": n,
                                     "message": f"waiting for the council ({int(time.time() - t0)}s)"}})
                    nextp += A.progress_every
                time.sleep(0.5)
        witness(seat)
        txt, last = view.render(D, seat, store.cursor(D, seat, PRINCIPALS[seat]))
        store.receipt(D, "wait-returned", seat=seat, why=why, waited_s=round(time.time() - t0, 2),
                      progress_sent=n, proc=PROC)
        head = {"your-turn": "A round is open for you.",
                "adjourned": "The meeting has adjourned.",
                "timeout": f"No round for you yet after {int(maxw)}s. Call council_wait again to keep waiting."}[why]
        reply(rid, head + "\n\n" + txt)
    elif name == "council_record":
        witness(seat)
        reply(rid, view.minutes(D))
    else:
        reply(rid, f"unknown tool {name}", True)


def handle(rid, params):
    try:
        tool_call(rid, params)
    except Exception:
        log("tool-error", err=traceback.format_exc())
        reply(rid, "internal error: " + traceback.format_exc()[-800:], True)


def _release_all(why):
    """Release held seats when this process ends -- unless the holder is a
    launcher-declared principal (a driver), which outlives each server process
    and holds the seat itself (verify-1 test A: releasing here left a driven
    seat unheld between turns)."""
    for s, pr in list(PRINCIPALS.items()):
        if pr.startswith("launcher:"):
            continue
        try:
            store.release_seat(D, s, pr, why, HOLDER_KEY)
        except Exception:
            pass


def _on_signal(signum, frame):
    log("signal", signum=signum)
    _release_all(f"signal {signum}")
    store.receipt(D, "server-exit", proc=PROC, seat=A.seat, why=f"signal {signum}")
    os._exit(0)


def main():
    import signal
    for sg in (signal.SIGTERM, signal.SIGINT, signal.SIGHUP):
        signal.signal(sg, _on_signal)
    # never log the token (verify-1 / redaction of runs/smoke: argv carried --token)
    argv = [("<redacted>" if i and sys.argv[i - 1] == "--token" else x.split("=")[0] + "=<redacted>" if x.startswith("--token=") else x)
            for i, x in enumerate(sys.argv)]
    log("start", pid=os.getpid(), ppid=os.getppid(), dir=D, seat=A.seat, argv=argv,
        env={k: v for k, v in os.environ.items() if k.startswith(("CLAUDE", "CC_", "CODEX", "GROK", "GEMINI", "CONCILIUM"))
             and "TOKEN" not in k and "KEY" not in k})
    store.receipt(D, "server-start", proc=PROC, seat=A.seat, ppid=os.getppid())
    threading.Thread(target=heartbeat_loop, daemon=True).start()
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            msg = json.loads(line)
        except Exception:
            log("bad-json", line=line[:300])
            continue
        log("in", msg=msg)
        m, rid = msg.get("method"), msg.get("id")
        if m == "initialize":
            p = msg.get("params") or {}
            CLIENT.update({"name": (p.get("clientInfo") or {}).get("name"),
                           "version": (p.get("clientInfo") or {}).get("version"),
                           "protocol": p.get("protocolVersion")})
            send({"jsonrpc": "2.0", "id": rid, "result": {
                "protocolVersion": p.get("protocolVersion", "2025-06-18"),
                "capabilities": {"tools": {}},
                "serverInfo": {"name": "concilium", "version": core.ENGINE_VERSION},
                "instructions": "You hold a seat on an agentic council. Use council_wait to wait for your turn, council_submit to act, council_status to look."}})
        elif m == "tools/list":
            send({"jsonrpc": "2.0", "id": rid, "result": {"tools": tools_for_this_process()}})
        elif m == "tools/call":
            threading.Thread(target=handle, args=(rid, msg.get("params") or {}), daemon=True).start()
        elif m == "notifications/cancelled":
            CANCELLED.add((msg.get("params") or {}).get("requestId"))
        elif m == "ping":
            send({"jsonrpc": "2.0", "id": rid, "result": {}})
        elif rid is not None:
            send({"jsonrpc": "2.0", "id": rid, "error": {"code": -32601, "message": f"method not found: {m}"}})
    log("stdin-closed")
    _release_all("stdin closed")
    store.receipt(D, "server-exit", proc=PROC, seat=A.seat, why="stdin closed")


if __name__ == "__main__":
    main()
