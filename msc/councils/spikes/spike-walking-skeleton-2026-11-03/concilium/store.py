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
  sealed/<round>/<seat>.json   a seat's sealed intent (final unless the charter says replaceable)
  rounds/<round>.json     {opened_at, closed_at} wall-clock: the backstop deadline, and stall clearing
  holder/<seat>.json      the seat's one holder: principal (asserted by the caller), hash of a
                          holder key (a secret the client never sees), epoch, lease ts, last_call
  cursors/<seat>.json     what the holder has acknowledged by acting (see server.py)
  presence/<seat>/<proc>.json  diagnostics only; liveness is the holder lease

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
    # prefixed: a bare token_urlsafe value starts with "-" about 1 time in 64, and
    # argparse then reads `--token -xyz` as a missing value (found by the
    # regression suite as an intermittent 60 s RPC timeout)
    toks = {s["id"]: "ct_" + secrets.token_urlsafe(16) for s in charter_obj["seats"]}
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


def _same_holder(h, principal, key):
    return h["principal"] == principal and h.get("key") == (core.sha(key) if key else None)


def claim_seat(d, seat, principal, attest, now=None, key=None):
    """Returns (ok, info). Called on every tool call; refreshes the lease and
    records `last_call` (a real call: the seat's own act, which clears a stall).
    A holder is (principal, key): the principal is what the caller asserts, the
    key a secret held by the connection or its launcher (de-novo-feedback-2
    §2.5: two clients presenting the same codex sessionId were one holder)."""
    now = now or time.time()
    lease_s = charter(d).get("round_policy", {}).get("lease_s", 30)
    with locked(d):
        h = holder(d, seat)
        if h and not _same_holder(h, principal, key) and now - h["ts"] <= lease_s:
            receipt(d, "seat-claim-refused", seat=seat, principal=principal, holder=h["principal"],
                    epoch=h["epoch"], attest=attest)
            return False, h
        if not h or not _same_holder(h, principal, key) or not h["ts"]:
            epoch = (h["epoch"] + 1) if h else 1
            rec = {"principal": principal, "key": core.sha(key) if key else None, "epoch": epoch,
                   "since": now, "ts": now, "last_call": now,
                   "previous": h and {"principal": h["principal"], "epoch": h["epoch"],
                                      "lapsed_for_s": None if h["ts"] == 0 else round(now - h["ts"], 1),
                                      "released": h["ts"] == 0}}
            _atomic_write(_p(d, "holder", seat + ".json"), rec)
            receipt(d, "seat-held", seat=seat, principal=principal, epoch=epoch,
                    previous=rec["previous"], attest=attest)
            return True, rec
        h["ts"] = now
        h["last_call"] = now
        _atomic_write(_p(d, "holder", seat + ".json"), h)
        return True, h


def refresh_seat(d, seat, principal, key=None):
    """Heartbeat: keeps the lease, does not count as the seat acting."""
    with locked(d):
        h = holder(d, seat)
        if h and h["ts"] and _same_holder(h, principal, key):
            h["ts"] = time.time()
            _atomic_write(_p(d, "holder", seat + ".json"), h)


def release_seat(d, seat, principal, why, key=None):
    with locked(d):
        h = holder(d, seat)
        if h and _same_holder(h, principal, key):
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


def round_opened_seq(d, round_id):
    return next((e["seq"] for e in events(d) if e["type"] == "round-opened" and e["body"]["round"] == round_id), -1)


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
            closed = next((e["body"] for e in events(d)
                           if e["type"] == "round-closed" and e["body"]["round"] == round_id), None)
            # verify-1 D4 / contention.md F4: a late intent is recorded (here, in the
            # non-canonical receipts), never silently dropped. The round is closed,
            # so recording its content reveals nothing that is still sealed.
            receipt(d, "submit-refused", seat=seat, round=round_id, current=cur, attest=attest,
                    why="round closed" if closed else "no such open round",
                    late_intent=intent, intent_sha=core.sha(intent))
            if closed and seat in closed["submitted"]:
                # verify-1 3.3: a member's own submission usually completes the round,
                # so its second try met a misleading "not open"; say what happened
                return False, (f"your submission for {round_id} was already counted, and {round_id} has closed "
                               f"(all seats had answered). Submissions are final. The open round is now {cur}: "
                               f"read the view, then submit there if you want to act.")
            return False, (f"round {round_id} has closed without your submission; it is recorded as late. "
                           f"The open round is {cur}.")
        err = core.validate(st, ch, seat, intent)
        if err:
            receipt(d, "submit-refused", seat=seat, round=round_id, why=err, attest=attest,
                    intent_sha=core.sha(intent))
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
        # verify-1 4.5 (and a regression test of the holder fix): presence files and
        # holders were two sources of truth for "is this seat here", and they
        # disagreed both ways (a fresh holder with no presence yet counted as gone;
        # a cleanly released seat counted as live for a lease). The seat's holder
        # lease is now the one source: live = held, not released, lease fresh.
        lease_s = pol.get("lease_s", 30)
        live = {}
        for s in addressed:
            h = holder(d, s)
            live[s] = [{"proc": h["principal"]}] if h and h["ts"] and now - h["ts"] <= lease_s else []
        grace = pol.get("arrival_grace_s", 120)
        gone = []
        evs_now = events(d)
        stalled = _stalled(ch, evs_now, pol.get("absent_rounds_to_gone", 2))
        if stalled:
            # de-novo-feedback-2 §2.1: "until it acts again" -- any call from the
            # seat's holder after the last close clears the stall, so this round
            # (and the next) waits for it. No bound on thinking time is added.
            last = [e["body"]["round"] for e in evs_now if e["type"] == "round-closed"][-1]
            closed_at = _read_json(_p(d, "rounds", last + ".json"), {}).get("closed_at", now)
            for s_ in list(stalled):
                h = holder(d, s_)
                if h and h.get("last_call", 0) > closed_at:
                    stalled.discard(s_)
        for s in addressed:
            if s in subs:
                continue
            if live.get(s) and s not in stalled:
                continue
            if s in stalled or holder(d, s) or now - convened > grace:
                gone.append(s)
        waiting = [s for s in addressed if s not in subs and s not in gone]
        backstop = pol.get("backstop_s", 900)
        if waiting and not (force_deadline or now - opened > backstop):
            return None
        closed_by = ("all-submitted" if not gone and not waiting else
                     "all-submitted-or-gone" if not waiting else "backstop-deadline")
        body = {"round": r["id"], "closed_by": closed_by,
                **({"stalled": sorted(stalled & set(gone))} if stalled & set(gone) else {}),
                "submitted": {s: subs[s] for s in sorted(subs)},
                "gone": sorted(gone), "absent": sorted(set(addressed) - set(subs))}
        close_ev = {"type": "round-closed", "body": body}
        derived = core.step(st, ch, close_ev)
        _append(d, [close_ev] + derived, events(d))
        rp = _p(d, "rounds", r["id"] + ".json")
        _atomic_write(rp, dict(_read_json(rp, {}), closed_at=time.time()))
        receipt(d, "round-closed", round=r["id"], closed_by=closed_by, waited_s=round(now - opened, 3),
                live={s: [l["proc"] for l in v] for s, v in live.items()})
        return r["id"]


def _stalled(ch, evs, k):
    """A seat is stalled after k consecutive closes that each *waited for it*:
    it was absent, not gone (de-novo-feedback-2 §2.2: a crashed seat used to be
    counted in rounds that waited for someone else), and the round ran to the
    backstop -- or the seat was already stalled in that close. Only rounds that
    already waited the full backstop start a streak, so slow thinking never counts.
    Pure over the log; `try_close` then removes any seat that has called since
    (its own act; §2.1). k=0 disables (a council that must never shrink, e.g.
    unanimity-or-hold, sets 0). Charter: round_policy.absent_rounds_to_gone."""
    if not k:
        return set()
    closes = [e["body"] for e in evs if e["type"] == "round-closed"]
    out = set()
    for seat in [x["id"] for x in ch["seats"]]:
        n = 0
        for c in reversed(closes):
            waited = (seat in c["absent"] and seat not in c["gone"] and c["closed_by"] == "backstop-deadline")
            if waited or seat in c.get("stalled", []):
                n += 1
                if n >= k:
                    out.add(seat)
                    break
            else:
                break
    return out


def _first_ts(d):
    try:
        with open(_p(d, "receipts.jsonl")) as f:
            return json.loads(f.readline())["ts"]
    except Exception:
        return time.time()
