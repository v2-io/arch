"""concilium store: a council directory shared by any number of processes.

No daemon. Every member's MCP server process (one per harness session) and
the steward CLI open the same directory; mutual exclusion is an flock on
`<dir>/lock`. Layout:

  charter.json            fixed at creation; its sha is in the `convened` event
  tokens.json             seat -> secret token (mode 600; never in the log)
  events.jsonl            canonical log: {seq, prev, hash, type, body}; no wall-clock inside
  receipts.jsonl          non-canonical journal: wall-clock receipts of everything
                          that happened (submissions incl. replacements, deliveries,
                          closes, presence changes), with attestations
  sealed/<round>/<seat>.json   a seat's current sealed intent (replaceable until close)
  rounds/<round>.json     {opened_at} wall-clock, for the backstop deadline
  presence/<seat>/<proc>.json  lease heartbeats from each server process holding the seat

Why sealed intents live in per-seat files and not in the log: appending them
as they arrive would put arrival order into the canonical log. Only the close
(sorted by seat) enters the log, so two runs whose rounds received the same
intents produce byte-identical logs whatever the arrival order.
"""
import fcntl
import json
import os
import secrets
import time
from contextlib import contextmanager

from . import core


def _p(d, *a):
    return os.path.join(d, *a)


@contextmanager
def locked(d):
    fd = os.open(_p(d, "lock"), os.O_CREAT | os.O_RDWR, 0o644)
    try:
        fcntl.flock(fd, fcntl.LOCK_EX)
        yield
    finally:
        fcntl.flock(fd, fcntl.LOCK_UN)
        os.close(fd)


def _atomic_write(path, obj, mode=0o644):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    tmp = f"{path}.tmp.{os.getpid()}"
    with open(tmp, "w") as f:
        f.write(json.dumps(obj, indent=1, sort_keys=True))
    os.chmod(tmp, mode)
    os.replace(tmp, path)


def _read_json(path, default=None):
    try:
        with open(path) as f:
            return json.load(f)
    except FileNotFoundError:
        return default


def receipt(d, kind, **kw):
    rec = {"ts": time.time(), "kind": kind, "pid": os.getpid(), **kw}
    with open(_p(d, "receipts.jsonl"), "a") as f:
        f.write(json.dumps(rec, sort_keys=True) + "\n")


def charter(d):
    return _read_json(_p(d, "charter.json"))


def tokens(d):
    return _read_json(_p(d, "tokens.json"), {})


def events(d):
    out = []
    try:
        with open(_p(d, "events.jsonl")) as f:
            for line in f:
                if line.strip():
                    out.append(json.loads(line))
    except FileNotFoundError:
        pass
    return out


def verify_chain(evs):
    prev = "0" * 64
    for i, e in enumerate(evs):
        body = {"seq": e["seq"], "prev": e["prev"], "type": e["type"], "body": e["body"]}
        if e["seq"] != i or e["prev"] != prev or e["hash"] != core.sha(body):
            return f"chain broken at seq {i}"
        prev = e["hash"]
    return None


def _append(d, evs_new, existing):
    prev = existing[-1]["hash"] if existing else "0" * 64
    seq = len(existing)
    lines = []
    for e in evs_new:
        body = {"seq": seq, "prev": prev, "type": e["type"], "body": e["body"]}
        rec = dict(body, hash=core.sha(body))
        lines.append(json.dumps(rec, sort_keys=True, ensure_ascii=False))
        prev = rec["hash"]
        seq += 1
        if e["type"] == "round-opened":
            _atomic_write(_p(d, "rounds", e["body"]["round"] + ".json"), {"opened_at": time.time()})
            receipt(d, "round-opened", round=e["body"]["round"], kind_=e["body"]["kind"])
    with open(_p(d, "events.jsonl"), "a") as f:
        f.write("".join(l + "\n" for l in lines))
        f.flush()
        os.fsync(f.fileno())


def create(d, charter_obj):
    os.makedirs(d, exist_ok=False)
    _atomic_write(_p(d, "charter.json"), charter_obj)
    toks = {s["id"]: secrets.token_urlsafe(16) for s in charter_obj["seats"]}
    _atomic_write(_p(d, "tokens.json"), toks, mode=0o600)
    with locked(d):
        receipt(d, "created", charter_sha=core.sha(charter_obj))
        _append(d, core.convene(charter_obj), [])
    return toks


def state(d):
    ch = charter(d)
    return ch, core.fold(ch, events(d))


# ------------------------------------------------------------- presence ---

def heartbeat(d, seat, proc, info):
    _atomic_write(_p(d, "presence", seat, proc + ".json"), dict(info, ts=time.time()))


def leases(d, now=None):
    """seat -> list of live process leases (heartbeat within lease_s)."""
    now = now or time.time()
    ch = charter(d)
    lease_s = ch.get("round_policy", {}).get("lease_s", 30)
    out = {}
    for s in [x["id"] for x in ch["seats"]]:
        live = []
        pdir = _p(d, "presence", s)
        if os.path.isdir(pdir):
            for fn in sorted(os.listdir(pdir)):
                if fn.endswith(".json"):
                    rec = _read_json(_p(pdir, fn), {})
                    if rec and now - rec.get("ts", 0) <= lease_s:
                        live.append(dict(rec, proc=fn[:-5]))
        out[s] = live
    return out


def ever_present(d, seat):
    pdir = _p(d, "presence", seat)
    return os.path.isdir(pdir) and any(fn.endswith(".json") for fn in os.listdir(pdir))


# -------------------------------------------------------- seat holders ---
# trial-3 finding: two live processes holding one seat raced each round, and
# whichever answered first spoke for the seat (a latency race the rounds exist
# to remove). So a seat has exactly one holder at a time: a *principal* (a
# harness session, or a driver), not a process. A second principal is refused
# while the holder's lease is live; after it lapses the seat can be taken, and
# the epoch increments so the record shows the change of mind behind the seat.

def holder(d, seat):
    return _read_json(_p(d, "holder", seat + ".json"))


def claim_seat(d, seat, principal, attest, now=None):
    """Returns (ok, info). Called on every tool call; also refreshes the lease."""
    now = now or time.time()
    lease_s = charter(d).get("round_policy", {}).get("lease_s", 30)
    with locked(d):
        h = holder(d, seat)
        if h and h["principal"] != principal and now - h["ts"] <= lease_s:
            receipt(d, "seat-claim-refused", seat=seat, principal=principal, holder=h["principal"],
                    epoch=h["epoch"], attest=attest)
            return False, h
        if not h or h["principal"] != principal:
            epoch = (h["epoch"] + 1) if h else 1
            rec = {"principal": principal, "epoch": epoch, "since": now, "ts": now,
                   "previous": h and {"principal": h["principal"], "epoch": h["epoch"],
                                      "lapsed_for_s": None if h["ts"] == 0 else round(now - h["ts"], 1),
                                      "released": h["ts"] == 0}}
            _atomic_write(_p(d, "holder", seat + ".json"), rec)
            receipt(d, "seat-held", seat=seat, principal=principal, epoch=epoch,
                    previous=rec["previous"], attest=attest)
            return True, rec
        h["ts"] = now
        _atomic_write(_p(d, "holder", seat + ".json"), h)
        return True, h


def refresh_seat(d, seat, principal):
    with locked(d):
        h = holder(d, seat)
        if h and h["principal"] == principal:
            h["ts"] = time.time()
            _atomic_write(_p(d, "holder", seat + ".json"), h)


def release_seat(d, seat, principal, why):
    with locked(d):
        h = holder(d, seat)
        if h and h["principal"] == principal:
            h["ts"] = 0
            _atomic_write(_p(d, "holder", seat + ".json"), h)
            receipt(d, "seat-released", seat=seat, principal=principal, epoch=h["epoch"], why=why)


# ------------------------------------------------------------ cursors ---
# trial-5 finding: a delivery cursor kept in a server process's memory resets
# whenever the process does (every driver turn, every restart), and the member
# is then re-sent the whole log as its "delta". The cursor belongs to the seat's
# holder, persisted; a new holder starts from nothing and gets a full catch-up.

def cursor(d, seat, principal):
    rec = _read_json(_p(d, "cursors", seat + ".json"), {})
    return rec.get("seq", -1) if rec.get("principal") == principal else -1


def set_cursor(d, seat, principal, seq):
    _atomic_write(_p(d, "cursors", seat + ".json"), {"principal": principal, "seq": seq})


# ------------------------------------------------------------- rounds ---

def sealed(d, round_id):
    out = {}
    rdir = _p(d, "sealed", round_id)
    if os.path.isdir(rdir):
        for fn in sorted(os.listdir(rdir)):
            if fn.endswith(".json"):
                out[fn[:-5]] = _read_json(_p(rdir, fn))["intent"]
    return out


def submit(d, seat, round_id, intent, attest):
    """Seal `intent` for `seat` in `round_id`. Returns (ok, message)."""
    with locked(d):
        ch, st = state(d)
        r = st["round"]
        if r is None or r["id"] != round_id:
            cur = r["id"] if r else None
            receipt(d, "submit-refused", seat=seat, round=round_id, current=cur, attest=attest)
            return False, f"round {round_id} is not open (open round: {cur})"
        err = core.validate(st, ch, seat, intent)
        if err:
            receipt(d, "submit-refused", seat=seat, round=round_id, why=err, attest=attest)
            return False, err
        norm = core.normalize_intent(intent)
        path = _p(d, "sealed", round_id, seat + ".json")
        replaced = os.path.exists(path)
        if replaced and not ch.get("round_policy", {}).get("replaceable", False):
            receipt(d, "submit-refused", seat=seat, round=round_id, why="already sealed", attest=attest)
            return False, (f"you already submitted in {round_id}; submissions are final "
                           "(charter round_policy.replaceable=false)")
        _atomic_write(path, {"intent": norm})
        receipt(d, "submitted", seat=seat, round=round_id, intent_sha=core.sha(norm),
                replaced=replaced, attest=attest)
    try_close(d)
    return True, ("replaced your sealed intent" if replaced else "sealed") + f" for {round_id}"


def try_close(d, now=None, force_deadline=False):
    """Close the open round if its condition holds. Idempotent and safe to call
    from any process at any time; returns the round id closed, or None."""
    now = now or time.time()
    with locked(d):
        ch, st = state(d)
        r = st["round"]
        if r is None or st["adjourned"]:
            return None
        pol = ch.get("round_policy", {})
        subs = sealed(d, r["id"])
        addressed = r["addressed"]
        opened = _read_json(_p(d, "rounds", r["id"] + ".json"), {}).get("opened_at", now)
        convened = _first_ts(d)
        live = leases(d, now)
        grace = pol.get("arrival_grace_s", 120)
        gone = []
        for s in addressed:
            if s in subs or live.get(s):
                continue
            if ever_present(d, s) or now - convened > grace:
                gone.append(s)
        waiting = [s for s in addressed if s not in subs and s not in gone]
        backstop = pol.get("backstop_s", 900)
        if waiting and not (force_deadline or now - opened > backstop):
            return None
        closed_by = ("all-submitted" if not gone and not waiting else
                     "all-submitted-or-gone" if not waiting else "backstop-deadline")
        body = {"round": r["id"], "closed_by": closed_by,
                "submitted": {s: subs[s] for s in sorted(subs)},
                "gone": sorted(gone), "absent": sorted(set(addressed) - set(subs))}
        close_ev = {"type": "round-closed", "body": body}
        derived = core.step(st, ch, close_ev)
        _append(d, [close_ev] + derived, events(d))
        receipt(d, "round-closed", round=r["id"], closed_by=closed_by, waited_s=round(now - opened, 3),
                live={s: [l["proc"] for l in v] for s, v in live.items()})
        return r["id"]


def _first_ts(d):
    try:
        with open(_p(d, "receipts.jsonl")) as f:
            return json.loads(f.readline())["ts"]
    except Exception:
        return time.time()
