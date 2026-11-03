#!/usr/bin/env python3
"""verify-2: model-free tests of properties I suspected from reading the final
build (commit 694a770), none of which verify-1 or tests/regression.py exercise.

Run from the spike dir:  python3 verify-2/verify2.py [test-prefix ...]
Writes only under verify-2/tmp/ and verify-2/verify2-output.txt. Each test
prints what it saw and a RESULT line; the report says what each means.
"""
import json, os, sys, time, threading, subprocess, shutil, traceback, stat

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, HERE)
from concilium import store, core, view  # noqa: E402
from tools_rpc_client import Client  # noqa: E402

PY = "/usr/bin/python3"
SERVER = os.path.join(HERE, "concilium", "server.py")
TMP = os.path.join(HERE, "verify-2", "tmp")
os.makedirs(TMP, exist_ok=True)
OUT = []


def say(*a):
    s = " ".join(str(x) for x in a)
    print(s, flush=True)
    OUT.append(s)


def fresh(name):
    d = os.path.join(TMP, name)
    shutil.rmtree(d, ignore_errors=True)
    return d


def charter(seats="AB", quorum=None, items=1, **policy):
    rp = {"backstop_s": 900, "arrival_grace_s": 120, "lease_s": 30, "replaceable": False, "max_inquorate_rounds": 2}
    rp.update(policy)
    return {"council": "verify2", "purpose": "verify-2", "seats": [{"id": s, "vantage": "v" + s} for s in seats],
            "quorum": quorum or (len(seats) // 2 + 1), "round_policy": rp,
            "debate_policy": {"max_debate_rounds": 3, "max_open_rounds": 2},
            "agenda": [{"id": f"q{i}", "title": f"item {i}"} for i in range(1, items + 1)]}


def server(d, seat, tok, principal=None, log=None, max_wait=None):
    env = dict(os.environ)
    env.pop("CONCILIUM_PRINCIPAL", None)
    if principal:
        env["CONCILIUM_PRINCIPAL"] = principal
    argv = [PY, "-I", SERVER, "--dir", d, "--seat", seat, "--token", tok, "--heartbeat", "1"]
    if log:
        argv += ["--log", log]
    if max_wait:
        argv += ["--max-wait", str(max_wait)]
    c = Client(argv, env=env)
    c.req("initialize", {"protocolVersion": "2025-06-18", "clientInfo": {"name": "verify-2", "version": "0"}})
    return c


ACT = {"call-to-order": {"action": "present"}, "open": {"action": "move", "text": "X"},
       "debate": {"action": "pass"}, "vote": {"action": "vote", "vote": "yes"}}


# --------------------------------------------------------------------------
def test_1_stall_recovery_is_a_race():
    """A seat the store has escalated to `stalled` is treated as gone until it
    acts again. But 'acts again' means: its submission is in a round's close,
    and rounds now close the instant the other members submit. So a returning
    member must out-race the fastest other member to get back in. Here A is
    live throughout (its server heartbeats), silent for two short backstop
    rounds, then loops wait->think 0.3 s->submit like a real member; B answers
    each round within 0.1 s of it opening."""
    d = fresh("t1"); toks = store.create(d, charter("AB", quorum=1, items=12, backstop_s=2, lease_s=30))
    a = server(d, "A", toks["A"], principal="A-session", max_wait=20)
    a.call("council_status", {})                       # A holds its seat: live
    stop = threading.Event()

    def b_fast():
        while not stop.is_set():
            st = core.fold(store.charter(d), store.events(d))
            if st["adjourned"]:
                break
            r = st["round"]
            if r and "B" not in store.sealed(d, r["id"]):
                store.submit(d, "B", r["id"], ACT[r["kind"]], {})
            store.try_close(d)
            time.sleep(0.1)
    threading.Thread(target=b_fast, daemon=True).start()
    while len([e for e in store.events(d) if e["type"] == "round-closed"]) < 2:
        time.sleep(0.1)                                # two backstopped rounds pass with A silent
    closes = [e["body"] for e in store.events(d) if e["type"] == "round-closed"]
    say(f"  after A's silence: {len(closes)} closes: {[(c['round'], c['closed_by']) for c in closes]}")
    view_head = None
    # A returns and behaves like a member: wait -> read -> think -> submit
    accepted = refused = 0; msgs = []
    t_end = time.time() + 8
    while time.time() < t_end:
        txt, err, _ = a.call("council_wait", {"max_wait_s": 3}, timeout=30)
        if "ADJOURNED" in txt:
            break
        if not txt.startswith("A round is open"):
            continue
        rid = [l for l in txt.splitlines() if l.startswith("OPEN ROUND")][0].split()[2]
        if view_head is None:
            view_head = [l for l in txt.splitlines() if l.strip()][-6:]
        st = core.fold(store.charter(d), store.events(d))
        kind = st["round"]["kind"] if st["round"] else None
        time.sleep(0.3)                                # the member's thinking time
        t2, e2, _ = a.call("council_submit", dict(ACT.get(kind, {"action": "pass"}), round=rid))
        if e2:
            refused += 1; msgs.append(t2.splitlines()[0][:110])
        else:
            accepted += 1
    stop.set(); a.close()
    closes = [e["body"] for e in store.events(d) if e["type"] == "round-closed"]
    say(f"  A, back and looping (0.3 s think): accepted {accepted}, refused {refused}; sample refusal: {msgs[:1]}")
    adj = [e["body"]["reason"] for e in store.events(d) if e["type"] == "adjourned"]
    say(f"  rounds after A's return: {len(closes) - 2}, A in a close's submitted: {sum(1 for c in closes[2:] if 'A' in c['submitted'])}; "
        f"all closed all-submitted-or-gone: {all(c['closed_by'] == 'all-submitted-or-gone' for c in closes[2:])}; adjourned: {adj}")
    say(f"  what A's view said while it was stalled (nothing about being stalled or needing to hurry): {view_head}")
    ok = accepted == 0 and refused >= 1 and bool(adj)
    say("  RESULT:", "CONFIRMED: a stalled seat that returns and behaves like a member never gets back in; the meeting ran to adjournment without it"
        if ok else f"not reproduced (accepted {accepted})")
    return ok


# --------------------------------------------------------------------------
def test_2_abandoned_wait_eats_the_delta():
    """harness-notes: Claude Code's idle abort and codex's 60 s timeout abandon a
    council_wait without telling the server. The server thread keeps waiting,
    and when the round opens it renders a view *and advances the seat's cursor*
    before replying into the void. The member's real (second) wait then renders
    with the advanced cursor: the 'since your last look' delta (the previous
    round's revealed intents) is gone from what the member sees."""
    d = fresh("t2"); toks = store.create(d, charter("AB", quorum=2))
    a = server(d, "A", toks["A"], principal="A-session", max_wait=30)
    a.call("council_submit", {"round": "r1", "action": "present"})     # A sealed r1; now it waits
    results = {}
    seen = []
    for trial in range(6):
        # two concurrent waits on one connection: the first is the one the harness abandoned
        for rid in (100 + trial * 2, 101 + trial * 2):
            a.p.stdin.write(json.dumps({"jsonrpc": "2.0", "id": rid, "method": "tools/call",
                                        "params": {"name": "council_wait", "arguments": {"max_wait_s": 20},
                                                   "_meta": {"progressToken": rid}}}) + "\n")
            a.p.stdin.flush()
        time.sleep(0.8)
        st = core.fold(store.charter(d), store.events(d))
        r = st["round"]
        store.submit(d, "B", r["id"], ACT[r["kind"]], {})               # B completes the round -> next round opens
        got = {}
        t0 = time.time()
        while len(got) < 2 and time.time() - t0 < 10:
            m = a.q.get(timeout=10)
            if m.get("id") in (100 + trial * 2, 101 + trial * 2):
                got[m["id"]] = m["result"]["content"][0]["text"]
        has_delta = {k: ("Since your last look" in v) for k, v in got.items()}
        seen.append(has_delta)
        # A seals the new round so the next trial's waits block again
        st = core.fold(store.charter(d), store.events(d)); r = st["round"]
        if st["adjourned"]:
            break
        a.call("council_submit", dict(ACT[r["kind"]], round=r["id"]))
    a.close()
    say(f"  per trial, which of the two pending waits (abandoned, live) got the delta: {seen}")
    one_only = sum(1 for s in seen if list(s.values()).count(True) == 1)
    ok = one_only >= 1
    say("  RESULT:", f"CONFIRMED in {one_only} of {len(seen)} trials: exactly one reply carried the delta; if the harness "
        "kept the other request, the member never sees that round's reveal" if ok else "not reproduced")
    return ok


# --------------------------------------------------------------------------
def test_3_stall_counts_rounds_that_never_waited():
    """_stalled counts a seat as stalled after k consecutive closes where it was
    `absent` and the round closed by backstop (or already stalled). `absent`
    includes seats that were `gone`: a seat whose lease had lapsed (crashed) is
    absent in a backstop round that waited for *someone else*. Two such rounds
    and the crashed seat comes back 'stalled': live, present, and not waited for."""
    ch = charter("ABC", quorum=1)
    def close(rid, sub, by, absent, gone):
        return {"type": "round-closed", "body": {"round": rid, "closed_by": by, "submitted": sub,
                                                 "gone": gone, "absent": absent}}
    # C is live and slow (the reason for the backstop); A had crashed (gone) in both rounds
    evs = [close("r1", {"B": {}}, "backstop-deadline", ["A", "C"], ["A"]),
           close("r2", {"B": {}}, "backstop-deadline", ["A", "C"], ["A"])]
    s = store._stalled(ch, evs, 2)
    say(f"  two backstop rounds where A was gone (crashed) and C was the slow live seat -> stalled = {sorted(s)}")
    # and in a real directory: A returns live after those rounds, B submits, does the round wait for A?
    d = fresh("t3"); toks = store.create(d, charter("ABC", quorum=1, backstop_s=1, lease_s=2, arrival_grace_s=0))
    store.claim_seat(d, "A", "A1", {}); store.claim_seat(d, "C", "C1", {})
    with open(os.path.join(d, "holder", "A.json")) as f:
        h = json.load(f)
    h["ts"] = time.time() - 100                                    # A's lease lapsed long ago (crashed)
    store._atomic_write(os.path.join(d, "holder", "A.json"), h)
    for _ in range(2):
        st = core.fold(store.charter(d), store.events(d)); r = st["round"]
        store.submit(d, "B", r["id"], ACT[r["kind"]], {})
        store.refresh_seat(d, "C", "C1")
        time.sleep(1.2); store.refresh_seat(d, "C", "C1"); store.try_close(d)
    closes = [e["body"] for e in store.events(d) if e["type"] == "round-closed"]
    say(f"  real dir: {[(c['round'], c['closed_by'], 'gone', c['gone'], 'stalled', c.get('stalled')) for c in closes]}")
    ok1, _ = store.claim_seat(d, "A", "A2", {})                     # A is back, live, new session
    st = core.fold(store.charter(d), store.events(d)); r = st["round"]
    t0 = time.time(); store.refresh_seat(d, "C", "C1")
    store.submit(d, "B", r["id"], ACT[r["kind"]], {})               # submit() calls try_close itself
    c3 = next(e["body"] for e in store.events(d) if e["type"] == "round-closed" and e["body"]["round"] == r["id"])
    say(f"  A returns live (claim ok={ok1}); B submits in {r['id']}; {r['id']} closed {time.time() - t0:.2f}s later by {c3['closed_by']}: "
        f"gone={c3['gone']} stalled={c3.get('stalled')} (A live and 0 s old; C live and slow)")
    ok = "A" in s and "A" in c3.get("stalled", []) and c3["closed_by"] == "all-submitted-or-gone"
    say("  RESULT:", "CONFIRMED: a crashed-and-returned seat is 'stalled' by rounds that never waited for it" if ok else "not reproduced")
    return ok


# --------------------------------------------------------------------------
def test_4_minutes_do_not_say_stalled():
    """response §4: 'The close records stalled: [...], so the minutes say why
    the seat was not waited for.' Check what view.describe prints for such a close."""
    ev = {"type": "round-closed", "body": {"round": "r3", "closed_by": "all-submitted-or-gone",
                                           "submitted": {"B": {"action": "pass"}}, "gone": ["A"], "absent": ["A"],
                                           "stalled": ["A"]}}
    txt = view.describe(ev)
    say("  describe(round-closed with stalled=[A]) ->", repr(txt))
    ok = "stall" not in txt.lower()
    say("  RESULT:", "CONFIRMED: the minutes and the member view show 'gone', not that the seat was stalled" if ok else "shows stalled")
    return ok


# --------------------------------------------------------------------------
def test_5_driver_relaunches_a_turn_that_did_not_act():
    """members/driver.py relaunches a seat's turn whenever a round is open for it
    and it has not sealed. A turn that ends without submitting (a model that
    says 'I'll wait', an API error, a refused tool) is therefore re-invoked
    about once a second, each at a resumed session's cost, until the backstop
    and stall escalation end the round. Model-free: a fake `claude` on PATH
    that answers like a turn and never calls the server."""
    d = fresh("t5"); run = fresh("t5-run"); os.makedirs(run)
    store.create(d, charter("AB", quorum=1, backstop_s=4, lease_s=30))
    fake = os.path.join(TMP, "fakebin"); shutil.rmtree(fake, ignore_errors=True); os.makedirs(fake)
    calls = os.path.join(run, "fake-claude-calls.txt")
    with open(os.path.join(fake, "claude"), "w") as f:
        f.write("#!/bin/sh\n"
                f"echo \"$(date +%s) $1 $2\" | head -1 >> '{calls}'\n"
                "echo '{\"type\":\"system\",\"subtype\":\"init\",\"session_id\":\"fake-sess\"}'\n"
                "sleep 0.2\n"
                "echo '{\"type\":\"result\",\"total_cost_usd\":0.001,\"num_turns\":1,\"result\":\"I will wait for the next round.\"}'\n")
    os.chmod(os.path.join(fake, "claude"), 0o755)
    env = dict(os.environ, PATH=fake + ":" + os.environ["PATH"])
    p = subprocess.Popen([PY, os.path.join(HERE, "members", "driver.py"), d, run, "haiku"], env=env,
                         stdout=subprocess.DEVNULL, stderr=open(os.path.join(run, "driver-stderr.txt"), "w"))
    t0 = time.time()
    try:
        p.wait(timeout=60)
    except subprocess.TimeoutExpired:
        p.kill()
    took = time.time() - t0
    n = sum(1 for _ in open(calls)) if os.path.exists(calls) else 0
    turns = [json.loads(l) for l in open(os.path.join(run, "driver.jsonl")) if '"turn-end"' in l]
    closes = [e["body"] for e in store.events(d) if e["type"] == "round-closed"]
    adj = [e["body"]["reason"] for e in store.events(d) if e["type"] == "adjourned"]
    say(f"  driver ran {took:.1f}s (exit {p.returncode}); fake claude invoked {n} times; driver turn-ends {len(turns)}; "
        f"rounds closed {len(closes)} {[c['closed_by'] for c in closes]}; adjourned: {adj}")
    ok = n >= 6 and p.returncode is not None
    say("  RESULT:", f"CONFIRMED: {n} resumed turns for {len(closes)} rounds in which nobody acted; the only exit was "
        "backstop -> stall -> gone -> inquorate -> adjourned" if ok else "not reproduced")
    return ok


# --------------------------------------------------------------------------
def test_6_principal_is_asserted_not_attested():
    """One holder per seat is keyed to a principal the *client* names: codex's
    _meta.sessionId, agy's root_conversation_id. Any client that sends the same
    _meta is the same principal. So two connections presenting one sessionId
    both hold the seat, and whichever submits first speaks for it: the trial-3
    race, back, for any harness whose _meta a second client can copy."""
    d = fresh("t6"); toks = store.create(d, charter("AB", quorum=1))
    meta = {"sessionId": "01a1a3-the-real-codex-session", "progressToken": 1}
    x = server(d, "A", toks["A"]); y = server(d, "A", toks["A"])
    tx, ex, _ = x.call("council_status", {}, meta=meta)
    ty, ey, _ = y.call("council_status", {}, meta=meta)
    hs = [json.loads(l) for l in open(os.path.join(d, "receipts.jsonl")) if '"seat-held"' in l or '"seat-claim-refused"' in l]
    ts, es, _ = y.call("council_submit", {"round": "r1", "action": "present"}, meta=meta)
    x.close(); y.close()
    say(f"  two processes, same _meta.sessionId: first -> {'refused' if ex else 'held'}, second -> {'refused' if ey else 'held'}; "
        f"holder records: {[(h['kind'], h['principal']) for h in hs]}; second's submit -> {'refused' if es else 'accepted'}")
    ok = not ex and not ey and not es and len(hs) == 1
    say("  RESULT:", "CONFIRMED: the one-holder rule is against accidental duplicates; a client that names the holder's "
        "_meta is the holder" if ok else "not reproduced")
    return ok


# --------------------------------------------------------------------------
def test_7_late_first_arrival_after_grace():
    """verify-1 F observed that a never-present seat is 'gone' after arrival_grace_s.
    What happens to the round it then joins: does its first call make the
    council wait for it, or has it already been closed out?"""
    d = fresh("t7"); toks = store.create(d, charter("AB", quorum=1, arrival_grace_s=1, lease_s=30))
    store.submit(d, "B", "r1", {"action": "present"}, {})
    time.sleep(1.3); store.try_close(d)
    c1 = [e["body"] for e in store.events(d) if e["type"] == "round-closed"]
    a = server(d, "A", toks["A"], principal="A-late")
    txt, err, _ = a.call("council_status", {})
    st = core.fold(store.charter(d), store.events(d)); r = st["round"]
    store.submit(d, "B", r["id"], ACT[r["kind"]], {})
    closed = store.try_close(d)
    a.close()
    say(f"  r1 closed {c1[0]['closed_by']} gone={c1[0]['gone']} before A arrived; A arrives, calls status -> "
        f"{'error' if err else 'ok'}; next round waits for A: {closed is None}")
    ok = closed is None
    say("  RESULT:", "a late first arrival recovers on its first call (unlike a stalled seat, test 1)" if ok else "late arrival not waited for")
    return ok


TESTS = [("1 stall recovery race", test_1_stall_recovery_is_a_race),
         ("2 abandoned wait eats delta", test_2_abandoned_wait_eats_the_delta),
         ("3 stall counts gone rounds", test_3_stall_counts_rounds_that_never_waited),
         ("4 minutes omit stalled", test_4_minutes_do_not_say_stalled),
         ("5 driver relaunch loop", test_5_driver_relaunches_a_turn_that_did_not_act),
         ("6 principal asserted", test_6_principal_is_asserted_not_attested),
         ("7 late arrival recovers", test_7_late_first_arrival_after_grace)]

if __name__ == "__main__":
    want = sys.argv[1:]
    res = []
    for name, fn in TESTS:
        if want and not any(name.startswith(w) for w in want):
            continue
        say(f"== {name}")
        try:
            res.append((name, fn()))
        except Exception:
            say("  EXCEPTION:\n" + traceback.format_exc())
            res.append((name, None))
    say("\n== summary (True = the property described was reproduced)")
    for n, r in res:
        say(f"  {r!s:5} {n}")
    with open(os.path.join(HERE, "verify-2", "verify2-output.txt"), "w") as f:
        f.write("\n".join(OUT) + "\n")
