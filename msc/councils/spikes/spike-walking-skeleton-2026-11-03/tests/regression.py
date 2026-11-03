#!/usr/bin/env python3
"""Model-free regression tests for the skeleton, reproducible from the commit.

  python3 tests/regression.py            (from the spike directory)

Part 1 runs the verifier's own tests (verify-1/verify.py) unchanged, with their
scratch redirected to tests/tmp/, so their verdicts on the fixed code speak for
themselves. Part 2 is this author's tests: one per fix made in response to
de-novo-feedback-1, and the three smoke scenarios (formerly runs/smoke, which
had no script) re-done on the current code. Output: tests/regression-output.txt.
"""
import io, json, os, shutil, signal, subprocess, sys, time, contextlib, importlib.util

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, HERE)
from concilium import store, core  # noqa: E402
from tools_rpc_client import Client  # noqa: E402

TMP = os.path.join(HERE, "tests", "tmp")
os.makedirs(TMP, exist_ok=True)
PY = "/usr/bin/python3"
SERVER = os.path.join(HERE, "concilium", "server.py")
OUT = []
RESULTS = []


def say(*a):
    s = " ".join(str(x) for x in a)
    print(s, flush=True)
    OUT.append(s)


def verdict(name, ok, detail=""):
    RESULTS.append((name, ok))
    say(f"  => {'PASS' if ok else 'FAIL'}: {name}" + (f" — {detail}" if detail else ""))


def fresh(n):
    d = os.path.join(TMP, n)
    shutil.rmtree(d, ignore_errors=True)
    return d


def charter(seats="AB", quorum=None, **pol):
    rp = {"backstop_s": 900, "arrival_grace_s": 120, "lease_s": 30, "replaceable": False, "max_inquorate_rounds": 2}
    rp.update(pol)
    return {"council": "regression", "purpose": "regression", "seats": [{"id": s, "vantage": "v" + s} for s in seats],
            "quorum": quorum or (len(seats) // 2 + 1), "round_policy": rp,
            "debate_policy": {"max_debate_rounds": 3, "max_open_rounds": 2}, "agenda": [{"id": "q1", "title": "one"}]}


def server(d, seat, tok, principal=None, log=None, key=None):
    env = dict(os.environ)
    env.pop("CONCILIUM_PRINCIPAL", None)
    env.pop("CONCILIUM_HOLDER_KEY", None)
    if principal:
        env["CONCILIUM_PRINCIPAL"] = principal
    if key:
        env["CONCILIUM_HOLDER_KEY"] = key
    argv = [PY, "-I", SERVER, "--dir", d, "--seat", seat, "--token", tok, "--heartbeat", "1"]
    if log:
        argv += ["--log", log]
    c = Client(argv, env=env)
    c.req("initialize", {"protocolVersion": "2025-06-18", "clientInfo": {"name": "regression", "version": "0"}})
    return c


# ------------------------------------------------- part 1: verifier's tests ---

def part1():
    say("== PART 1: verify-1/verify.py tests, unchanged, against the current code")
    sys.path.insert(0, os.path.join(HERE, "verify-1"))
    import verify as v                     # importable by name, so the C test's worker processes can unpickle
    v.TMP = os.path.join(TMP, "verify1")
    os.makedirs(v.TMP, exist_ok=True)
    v.say = say
    # what each verify-1 test returning True meant in the report, and what a fixed build should return
    expect = {"A driver-mode seat gap": (False, "True = defect present"),
              "B SIGKILL lease lapse": (True, "True = claim holds"),
              "C concurrency stress": (True, "True = lock discipline held"),
              "D1 partial withdraw / debate limit": (True, "True = limit held"),
              "D3 stuck live member": (None, "True = every round waits the full backstop; see part 2 for the escalation, which needs >= 2 rounds"),
              "D4 late submission dropped": (None, "returns not-ok (refused) either way; look at the printed content line"),
              "D5 replace promise": (False, "True = contradiction present"),
              "E replay is not tamper evidence": (False, "True = steward replay says OK on the tampered copy")}
    for name, fn in v.TESTS:
        if name not in expect:
            continue
        say(f"-- verify-1 {name}")
        try:
            r = fn()
        except Exception as e:
            say("  EXCEPTION", repr(e))
            r = "exception"
        want, meaning = expect[name]
        if name.startswith("E "):
            rc = subprocess.run([PY, "-I", os.path.join(HERE, "concilium", "steward.py"), "replay",
                                 os.path.join(v.TMP, "e")], capture_output=True, text=True).returncode
            say(f"  (verify-1's E decides by finding 'OK' in the output, which the new replay line contains even when it fails; "
                f"steward replay's exit code on the tampered copy is {rc})")
            verdict("verify-1 E: replay no longer passes the copy with sealed/ deleted", rc == 1,
                    "a full regeneration that also rewrites sealed/ still passes replay; see the witness test")
            continue
        if want is None:
            say(f"  (returned {r}; {meaning})")
        else:
            verdict(f"verify-1 {name}", r == want, f"returned {r}, want {want} ({meaning})")


# ------------------------------------------------- part 2: author's tests ---

def t_driver_holds():
    say("-- driver holds its seats between turns (fix for verify-1 A)")
    d = fresh("driver"); toks = store.create(d, charter("AB"))
    pr = "launcher:driver-1"; key = "driver-1-secret"
    ok, h = store.claim_seat(d, "A", pr, {"test": 1}, key=key)        # what driver.py does at start
    c1 = server(d, "A", toks["A"], principal="driver-1", key=key)     # turn 1 (the driver passes its key)
    t, err, _ = c1.call("council_status", {})
    c1.close(); time.sleep(0.4)                                       # turn ends (stdin closed)
    h = store.holder(d, "A")
    c2 = server(d, "A", toks["A"], principal="stranger")
    t2, err2, _ = c2.call("council_status", {})
    store.refresh_seat(d, "A", pr, key)                               # driver heartbeat
    c3 = server(d, "A", toks["A"], principal="driver-1", key=key)     # turn 2
    t3, err3, _ = c3.call("council_status", {})
    c2.close(); c3.close()
    say(f"  after turn 1 ended: holder ts={'live' if h['ts'] else 0}; stranger -> {'refused' if err2 else 'HELD'}; driver turn 2 -> {'refused' if err3 else 'ok'}")
    verdict("driver seat is held between turns", bool(h["ts"]) and err2 and not err3 and not err)


def t_stall_escalation():
    say("-- a live-but-silent seat escalates to gone after 2 backstopped rounds (fix for verify-1 D3)")
    d = fresh("stall"); toks = store.create(d, charter("AB", quorum=1, backstop_s=2, lease_s=30))
    c = server(d, "A", toks["A"], principal="stuck")            # A holds its seat (live) and never submits
    c.call("council_status", {})
    acts = {"call-to-order": {"action": "present"}, "open": {"action": "move", "text": "X"},
            "debate": {"action": "pass"}, "vote": {"action": "vote", "vote": "yes"}}
    t_end = time.time() + 30
    while time.time() < t_end:                                    # B answers every round at once
        st = core.fold(store.charter(d), store.events(d))
        if st["adjourned"]:
            break
        r = st["round"]
        if r and "B" not in store.sealed(d, r["id"]):
            store.submit(d, "B", r["id"], acts[r["kind"]], {})
        store.try_close(d)
        time.sleep(0.05)
    c.close()
    opened = {json.loads(l)["round"]: json.loads(l)["ts"] for l in open(os.path.join(d, "receipts.jsonl")) if '"round-opened"' in l}
    closed = {json.loads(l)["round"]: json.loads(l)["ts"] for l in open(os.path.join(d, "receipts.jsonl")) if '"round-closed"' in l}
    closes = [e["body"] for e in store.events(d) if e["type"] == "round-closed"]
    for b in closes:
        say(f"  {b['round']}: open {closed[b['round']] - opened[b['round']]:.1f}s, {b['closed_by']}, absent {b['absent']}, "
            f"gone {b['gone']}, stalled {b.get('stalled', [])}")
    later = closes[2:]
    ok = (len(closes) >= 4 and [b["closed_by"] for b in closes[:2]] == ["backstop-deadline"] * 2
          and all(b.get("stalled") == ["A"] and b["closed_by"] == "all-submitted-or-gone"
                  and closed[b["round"]] - opened[b["round"]] < 1.0 for b in later))
    verdict("stall escalates after 2 backstops and stays escalated", ok, f"{len(later)} later rounds")


def t_stall_clears():
    say("-- a stalled seat that acts again is no longer stalled")
    ch = charter("AB", quorum=1)
    def close(rid, sub, by, absent, stalled=()):
        return {"type": "round-closed", "body": {"round": rid, "closed_by": by, "submitted": sub, "gone": list(stalled),
                                                 "absent": absent, **({"stalled": list(stalled)} if stalled else {})}}
    evs = [close("r1", {"B": {}}, "backstop-deadline", ["A"]), close("r2", {"B": {}}, "backstop-deadline", ["A"]),
           close("r3", {"B": {}}, "all-submitted-or-gone", ["A"], ["A"])]
    a = store._stalled(ch, evs, 2)
    evs.append(close("r4", {"A": {}, "B": {}}, "all-submitted", []))
    b = store._stalled(ch, evs, 2)
    say(f"  stalled after r3: {sorted(a)}; after A acts in r4: {sorted(b)}")
    verdict("stall clears when the seat acts", a == {"A"} and b == set())


def t_late_and_message():
    say("-- late intent recorded; own-completion refusal says what happened (verify-1 D4, 3.3)")
    d = fresh("late"); toks = store.create(d, charter("AB", quorum=1, backstop_s=1))
    ok1, _ = store.submit(d, "B", "r1", {"action": "present"}, {})
    time.sleep(1.1); store.try_close(d, force_deadline=True)
    ok2, m2 = store.submit(d, "A", "r1", {"action": "present", "text": "LATE-BUT-REAL"}, {})
    late_recorded = "LATE-BUT-REAL" in open(os.path.join(d, "receipts.jsonl")).read()
    ok3, m3 = store.submit(d, "B", "r1", {"action": "present", "text": "second try"}, {})
    say(f"  late A -> {m2}")
    say(f"  B again after its own close -> {m3[:120]}")
    verdict("late intent recorded in receipts", late_recorded)
    verdict("own-completion refusal names the cause", "already counted" in m3)


def t_witness():
    say("-- witness: members' recorded heads catch a regenerated log (answer to verify-1 E)")
    d = fresh("witness"); toks = store.create(d, charter("AB", quorum=2))
    logs = {s: os.path.join(TMP, f"witness-{s}.jsonl") for s in "AB"}
    for p in logs.values():
        if os.path.exists(p):
            os.remove(p)
    cs = {s: server(d, s, toks[s], principal="p" + s, log=logs[s]) for s in "AB"}
    plan = [("present", None), ("move", "X"), ("pass", None), ("vote", "yes")]
    for act, arg in plan:
        st = core.fold(store.charter(d), store.events(d))
        rid = st["round"]["id"]
        for s in "AB":
            a = {"round": rid, "action": act}
            if act == "move":
                a["text"] = arg if s == "A" else None
                if s == "B":
                    a = {"round": rid, "action": "pass"}
            if act == "vote":
                a["vote"] = "yes"
            cs[s].call("council_submit", {k: v for k, v in a.items() if v is not None})
    for c in cs.values():
        c.call("council_status", {})
        c.close()
    st = core.fold(store.charter(d), store.events(d))
    r_ok = subprocess.run([PY, "-I", os.path.join(HERE, "concilium", "steward.py"), "witness", d, *logs.values()],
                          capture_output=True, text=True)
    say("  untampered:", r_ok.stdout.strip())
    # tamper exactly as verify-1 test E: flip the vote, regenerate the whole log, rewrite sealed/ to match
    spec = importlib.util.spec_from_file_location("verify1", os.path.join(HERE, "verify-1", "verify.py"))
    v = importlib.util.module_from_spec(spec); spec.loader.exec_module(v)
    ch = store.charter(d); evs = store.events(d)
    inputs = [{"type": e["type"], "body": e["body"]} for e in evs if e["type"] in core.INPUT_TYPES]
    for inp in inputs:
        for s_, it in inp["body"]["submitted"].items():
            if it.get("vote") == "yes":
                it["vote"] = "no"
                with open(os.path.join(d, "sealed", inp["body"]["round"], s_ + ".json"), "w") as f:
                    json.dump({"intent": it}, f)
    new = v._regenerate(ch, inputs)
    with open(os.path.join(d, "events.jsonl"), "w") as f:
        f.write("".join(json.dumps(r, sort_keys=True, ensure_ascii=False) + "\n" for r in new))
    rep = subprocess.run([PY, "-I", os.path.join(HERE, "concilium", "steward.py"), "replay", d], capture_output=True, text=True)
    r_bad = subprocess.run([PY, "-I", os.path.join(HERE, "concilium", "steward.py"), "witness", d, *logs.values()],
                           capture_output=True, text=True)
    say("  tampered (votes flipped, log regenerated, sealed/ rewritten to match): replay says:", rep.stdout.strip())
    say("  tampered: witness says:", r_bad.stdout.strip().splitlines()[-1])
    verdict("witness passes the untampered log", r_ok.returncode == 0)
    verdict("replay still passes a fully regenerated log (by design: internal consistency only)", rep.returncode == 0)
    verdict("witness catches the regenerated log", r_bad.returncode == 1)


def t_smoke_protocol():
    say("-- smoke c1 reproduced: tokens, final submissions, shared-connection seat_token")
    d = fresh("smoke1"); toks = store.create(d, charter("AB", quorum=2))
    a = server(d, "A", toks["A"]); b = server(d, "B", toks["B"])
    tl, _ = a.req("tools/list")
    single = "seat_token" not in tl["result"]["tools"][0]["inputSchema"]["properties"]
    _, e1, _ = a.call("council_submit", {"round": "r1", "action": "present"})
    t2, e2, _ = a.call("council_submit", {"round": "r1", "action": "present"})
    t3, e3, _ = b.call("council_status", {"seat_token": "B"})          # a model's guess
    a.close(); b.close()
    m = Client([PY, "-I", SERVER, "--dir", d, "--heartbeat", "1"])  # multi-seat (no --seat)
    m.req("initialize", {"protocolVersion": "2025-06-18", "clientInfo": {"name": "regression", "version": "0"}})
    tl2, _ = m.req("tools/list")
    multi_req = "seat_token" in tl2["result"]["tools"][0]["inputSchema"].get("required", [])
    t4, e4, _ = m.call("council_status", {"seat_token": toks["B"]})
    t5, e5, _ = m.call("council_status", {})
    m.close()
    say(f"  single-seat hides seat_token: {single}; 2nd submit refused: {e2}; guessed token refused with guidance: {e3} ({t3[:70]}...)")
    say(f"  multi-seat requires seat_token: {multi_req}; real token -> {'ok' if not e4 else 'refused'}; no token -> {'refused' if e5 else 'ok'}")
    verdict("protocol smoke", single and not e1 and e2 and e3 and "already holds seat" in t3 and multi_req and not e4 and e5)


def t_smoke_holders():
    say("-- smoke c2 reproduced: one holder per seat; clean release; SIGKILL lapse; launcher principals outlive their process")
    d = fresh("smoke2"); toks = store.create(d, charter("AB", lease_s=4))

    def harness_server(seat):
        # a distinct parent process per server, so each gets its own parent-process principal
        # (the path Claude Code and grok members take), with the server as that parent's child
        argv = ["/bin/sh", "-c", f"{PY} -I {SERVER} --dir {d} --seat {seat} --token {toks[seat]} --heartbeat 1; :"]
        c = Client(argv)
        c.req("initialize", {"protocolVersion": "2025-06-18", "clientInfo": {"name": "regression", "version": "0"}})
        return c
    x = harness_server("A"); x.call("council_status", {})
    y = harness_server("A")
    _, ey1, _ = y.call("council_status", {})
    x.close(); time.sleep(0.5)                                  # clean exit: stdin closes, server releases
    _, ey2, _ = y.call("council_status", {})
    z = harness_server("A")
    _, ez1, _ = z.call("council_status", {})
    # crash: kill Y's server (the sh's child) with SIGKILL, no release
    ypid = int(subprocess.run(["pgrep", "-P", str(y.p.pid)], capture_output=True, text=True).stdout.split()[0])
    os.kill(ypid, signal.SIGKILL)
    t0 = time.time()
    while True:
        _, ez, _ = z.call("council_status", {})
        if not ez or time.time() - t0 > 10:
            break
        time.sleep(0.5)
    took = time.time() - t0
    z.close()
    # a launcher principal (a driver) is not released when one of its turns' servers exits
    l1 = server(d, "B", toks["B"], principal="drv"); l1.call("council_status", {}); l1.close(); time.sleep(0.5)
    hb = store.holder(d, "B")
    hist = [json.loads(l) for l in open(os.path.join(d, "receipts.jsonl")) if '"seat-held"' in l and '"seat": "A"' in l]
    say(f"  Y refused while X live: {ey1}; Y holds after X exits cleanly: {not ey2}; Z refused while Y live: {ez1}; "
        f"Z holds {took:.1f}s after Y's SIGKILL (lease 4s); epochs {[h['epoch'] for h in hist]}; "
        f"previous released flags: {[h['previous']['released'] for h in hist[1:]]}; launcher-held B after its server exited: "
        f"{'held' if hb['ts'] else 'released'}")
    ok = (ey1 and not ey2 and ez1 and not ez and 3 <= took <= 6 and [h["epoch"] for h in hist] == [1, 2, 3]
          and hist[1]["previous"]["released"] is True and hist[2]["previous"]["released"] is False and hb["ts"])
    verdict("holder exclusivity", ok)


def t_smoke_backstop():
    say("-- smoke c3 reproduced: live-but-silent seat; backstop closes the round, seat absent not gone")
    d = fresh("smoke3"); toks = store.create(d, charter("ABC", quorum=2, backstop_s=3, arrival_grace_s=1, lease_s=15))
    cs = {s: server(d, s, toks[s], principal="p" + s) for s in "ABC"}
    cs["C"].call("council_status", {})
    t0 = time.time()
    cs["A"].call("council_submit", {"round": "r1", "action": "present"})
    cs["B"].call("council_submit", {"round": "r1", "action": "present"})
    t, _, _ = cs["A"].call("council_wait", {"max_wait_s": 15}, timeout=30)
    took = time.time() - t0
    for c in cs.values():
        c.close()
    b = [e["body"] for e in store.events(d) if e["type"] == "round-closed"][0]
    say(f"  r1 closed after {took:.1f}s by {b['closed_by']}, absent {b['absent']}, gone {b['gone']}")
    verdict("backstop", b["closed_by"] == "backstop-deadline" and b["absent"] == ["C"] and b["gone"] == [] and 2.5 <= took <= 6)


def t_replace_text():
    say("-- what the member reads matches the charter (verify-1 D5)")
    for rep in (False, True):
        d = fresh(f"replace{rep}"); toks = store.create(d, charter("AB", replaceable=rep))
        c = server(d, "A", toks["A"])
        tl, _ = c.req("tools/list")
        desc = [t for t in tl["result"]["tools"] if t["name"] == "council_submit"][0]["description"]
        c.call("council_submit", {"round": "r1", "action": "present"})
        t, _, _ = c.call("council_status", {})
        line = [l for l in t.splitlines() if "You have submitted" in l][0]
        c.close()
        says_replace = "may replace" in desc and "may replace" in line
        says_final = "final" in desc and "final" in line
        say(f"  replaceable={rep}: description/view say {'replace' if says_replace else 'final' if says_final else 'MIXED'}")
        verdict(f"text matches replaceable={rep}", says_replace if rep else says_final)


def part3():
    say("== PART 3: verify-2/verify2.py tests, unchanged, against the current code")
    sys.path.insert(0, os.path.join(HERE, "verify-2"))
    import verify2 as v2
    v2.TMP = os.path.join(TMP, "verify2")
    os.makedirs(v2.TMP, exist_ok=True)
    v2.say = say
    # True from these tests meant "the defect was reproduced"; a fixed build should return False
    expect = {"1 stall recovery race": False, "3 stall counts gone rounds": False, "4 minutes omit stalled": False,
              "5 driver relaunch loop": False, "6 principal asserted": False, "7 late arrival recovers": True}
    for name, fn in v2.TESTS:
        say(f"-- verify-2 {name}")
        if name.startswith("2 "):
            say("  (not run as written: it passes when exactly one of two pending waits carries the delta, which "
                "supersession now makes true by design; t_abandoned_wait below tests the property instead)")
            continue
        try:
            r = fn()
        except StopIteration:
            if name.startswith("3 "):
                # its live part looks for r3's close right after the returned seat's first call; the fix is
                # that r3 now waits for that seat, so there is no close to find
                d3 = os.path.join(v2.TMP, "t3")
                st3 = core.fold(store.charter(d3), store.events(d3))
                waits = st3["round"] is not None and "A" not in store.sealed(d3, st3["round"]["id"])
                say(f"  (StopIteration: r3 did not close; it is open and waiting for the returned seat A: {waits})")
                r = False if waits else "unexpected"
            else:
                r = "exception"
        except Exception as e:
            say("  EXCEPTION", repr(e)); r = "exception"
        verdict(f"verify-2 {name}", r == expect[name], f"returned {r}, want {expect[name]}")


def t_abandoned_wait():
    say("-- an abandoned council_wait cannot eat the delta (fix for verify-2 §2.3)")
    d = fresh("abandoned"); toks = store.create(d, charter("AB", quorum=2))
    a = server(d, "A", toks["A"], principal="A-session")
    a.call("council_submit", {"round": "r1", "action": "present"})
    acts = {"call-to-order": {"action": "present"}, "open": {"action": "move", "text": "X"},
            "debate": {"action": "pass"}, "vote": {"action": "vote", "vote": "yes"}}
    live_ok = []; old_superseded = []
    for trial in range(4):
        ids = (200 + trial * 2, 201 + trial * 2)        # first = the one the harness abandoned, second = live
        for rid in ids:
            a.p.stdin.write(json.dumps({"jsonrpc": "2.0", "id": rid, "method": "tools/call",
                                        "params": {"name": "council_wait", "arguments": {"max_wait_s": 20},
                                                   "_meta": {"progressToken": rid}}}) + "\n")
            a.p.stdin.flush()
            time.sleep(0.3)
        st = core.fold(store.charter(d), store.events(d)); r = st["round"]
        store.submit(d, "B", r["id"], acts[r["kind"]], {})
        got = {}; t0 = time.time()
        while len(got) < 2 and time.time() - t0 < 10:
            m = a.q.get(timeout=10)
            if m.get("id") in ids:
                got[m["id"]] = m["result"]["content"][0]["text"]
        live_ok.append("Since your last look" in got.get(ids[1], ""))
        old_superseded.append("superseded" in got.get(ids[0], ""))
        st = core.fold(store.charter(d), store.events(d))
        if st["adjourned"]:
            break
        a.call("council_submit", dict(acts[st["round"]["kind"]], round=st["round"]["id"]))
    # and a status call between acts repeats the delta rather than consuming it
    st = core.fold(store.charter(d), store.events(d))
    t1, _, _ = a.call("council_status", {}); t2, _, _ = a.call("council_status", {})
    a.close()
    say(f"  live wait carried the delta: {live_ok}; abandoned wait superseded: {old_superseded}; "
        f"two status calls in a row both show the delta: {'Since your last look' in t1 and 'Since your last look' in t2}")
    verdict("the live wait always carries the reveal", all(live_ok) and all(old_superseded))
    verdict("a view is not consumed by sending it", "Since your last look" in t1 and "Since your last look" in t2)


def t_bad_key():
    say("-- a client that copies the holder's principal is still refused (fix for verify-2 §2.5)")
    d = fresh("key"); toks = store.create(d, charter("AB", quorum=1))
    meta = {"sessionId": "same-asserted-session", "progressToken": 1}
    x = server(d, "A", toks["A"]); y = server(d, "A", toks["A"])
    _, ex, _ = x.call("council_status", {}, meta=meta)
    _, ey, _ = y.call("council_status", {}, meta=meta)
    _, es, _ = y.call("council_submit", {"round": "r1", "action": "present"}, meta=meta)
    x.close(); y.close()
    say(f"  first -> {'refused' if ex else 'held'}; copy of its sessionId from another connection -> "
        f"{'refused' if ey else 'HELD'}; its submit -> {'refused' if es else 'ACCEPTED'}")
    verdict("asserted principal alone does not hold a seat", not ex and ey and es)


def part2():
    say("== PART 2: author's tests on the current code")
    for t in (t_driver_holds, t_stall_escalation, t_stall_clears, t_abandoned_wait, t_bad_key, t_late_and_message, t_witness,
              t_smoke_protocol, t_smoke_holders, t_smoke_backstop, t_replace_text):
        try:
            t()
        except Exception:
            import traceback
            say("  EXCEPTION\n" + traceback.format_exc())
            RESULTS.append((t.__name__, False))


if __name__ == "__main__":
    part1()
    part2()
    part3()
    n_ok = sum(1 for _, ok in RESULTS if ok)
    say(f"\n== {n_ok} of {len(RESULTS)} checks pass")
    for n, ok in RESULTS:
        if not ok:
            say("  FAILED:", n)
    with open(os.path.join(HERE, "tests", "regression-output.txt"), "w") as f:
        f.write("\n".join(OUT) + "\n")
