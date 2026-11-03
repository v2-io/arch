#!/usr/bin/env python3
"""verify-1: model-free re-derivations against the committed concilium code.

Run from the spike dir:  python3 verify-1/verify.py [test ...]
Writes nothing outside verify-1/tmp/ (which is git-ignored). Each test prints
PASS/FAIL against the *claim it checks*, which may be a claim the spike makes
or a property I expected the design to have; the report says which.
"""
import json, os, sys, time, signal, subprocess, shutil, multiprocessing, random, hashlib, traceback

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, HERE)
from concilium import store, core, view  # noqa: E402
from tools_rpc_client import Client  # noqa: E402

PY = "/usr/bin/python3"
SERVER = os.path.join(HERE, "concilium", "server.py")
TMP = os.path.join(HERE, "verify-1", "tmp")
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


def charter(seats="AB", quorum=None, **policy):
    rp = {"backstop_s": 900, "arrival_grace_s": 120, "lease_s": 30, "replaceable": False, "max_inquorate_rounds": 2}
    rp.update(policy)
    return {"council": "verify", "purpose": "verify-1", "seats": [{"id": s, "vantage": "v" + s} for s in seats],
            "quorum": quorum or (len(seats) // 2 + 1), "round_policy": rp,
            "debate_policy": {"max_debate_rounds": 3, "max_open_rounds": 2},
            "agenda": [{"id": "q1", "title": "one"}]}


def server(d, seat, tok, principal=None, extra_env=None):
    env = dict(os.environ)
    env.pop("CONCILIUM_PRINCIPAL", None)
    if principal:
        env["CONCILIUM_PRINCIPAL"] = principal
    env.update(extra_env or {})
    c = Client([PY, "-I", SERVER, "--dir", d, "--seat", seat, "--token", tok, "--heartbeat", "1"], env=env)
    c.req("initialize", {"protocolVersion": "2025-06-18", "clientInfo": {"name": "verify-1", "version": "0"}})
    return c


# ----------------------------------------------------------------- tests ---

def test_a_driver_gap():
    """Claim under test (debrief §1, plan §2.2): 'a seat has one holder at a time ...
    enforced on every call'. Driver mode: every per-turn server process exits by
    signal/stdin-close and *releases* the holder, so between turns the seat is
    unheld. Expectation: a stranger with the token can take the seat between
    turns and the driver is then refused its own seat."""
    d = fresh("a"); toks = store.create(d, charter("AB"))
    c1 = server(d, "A", toks["A"], principal="driver-1")
    t, err, _ = c1.call("council_status", {})
    assert not err, t
    c1.close(); time.sleep(0.5)                     # turn ends: stdin closed -> release_seat (ts=0)
    h = store.holder(d, "A")
    say("  after driver turn ended: holder ts =", h["ts"], "(0 means released)")
    c2 = server(d, "A", toks["A"], principal="stranger")
    t2, err2, _ = c2.call("council_status", {})
    h2 = store.holder(d, "A")
    say("  stranger claims seat A between driver turns ->", "REFUSED" if err2 else f"HELD epoch {h2['epoch']} by {h2['principal']}")
    c3 = server(d, "A", toks["A"], principal="driver-1")
    t3, err3, _ = c3.call("council_status", {})
    say("  driver's next turn ->", "REFUSED: " + t3[:90] if err3 else "ok")
    c2.close(); c3.close()
    gap = (not err2) and err3
    say("  RESULT:", "CONFIRMED DEFECT — seat unguarded between driver turns" if gap else "no gap found")
    return gap


def test_b_sigkill_lapse():
    """Claim (debrief §1): 'A crashed holder's seat frees itself after a lease'.
    Model runs only exercised SIGTERM (clean release). Check SIGKILL."""
    d = fresh("b"); toks = store.create(d, charter("AB", lease_s=4))
    c1 = server(d, "A", toks["A"], principal="X")
    c1.call("council_status", {})
    os.kill(c1.p.pid, signal.SIGKILL); c1.p.wait()
    c2 = server(d, "A", toks["A"], principal="Y")
    t, err, _ = c2.call("council_status", {})
    say("  immediately after SIGKILL of holder X: Y ->", "refused (lease live)" if err else "HELD (unexpected)")
    time.sleep(5)
    t2, err2, _ = c2.call("council_status", {})
    h = store.holder(d, "A")
    say("  after lease_s: Y ->", "refused" if err2 else f"held epoch {h['epoch']}, previous {h.get('previous')}")
    c2.close()
    ok = err and not err2 and h["epoch"] == 2 and h["previous"]["released"] is False
    say("  RESULT:", "claim holds (SIGKILL -> lapse -> takeover, epoch 2, 'lapsed' not 'released')" if ok else "FAIL")
    return ok


def _hammer(args):
    d, seat, n, seed = args
    sys.path.insert(0, HERE)
    from concilium import store as st_, core as co_
    rng = random.Random(seed)
    acc = 0
    t_end = time.time() + 40
    while time.time() < t_end:
        ch, st = st_.state(d)
        if st["adjourned"]:
            break
        r = st["round"]
        if not r or seat not in r["addressed"]:
            time.sleep(0.01); continue
        k = r["kind"]
        intent = ({"call-to-order": {"action": "present"}, "open": {"action": "move", "text": "X"},
                   "debate": {"action": "pass"}, "vote": {"action": "vote", "vote": "yes"}})[k]
        ok, msg = st_.submit(d, seat, r["id"], intent, {"proc": n})
        if ok:
            acc += 1
        time.sleep(rng.random() * 0.02)
    return seat, n, acc


def _ticker(d):
    sys.path.insert(0, HERE)
    from concilium import store as st_
    t_end = time.time() + 40
    while time.time() < t_end:
        st_.try_close(d)
        ch, st = st_.state(d)
        if st["adjourned"]:
            break
        time.sleep(0.005)


def test_c_concurrency():
    """Debrief 'places I'd look first': the store's lock discipline under many
    concurrent writers. 5 seats x 4 processes each + 2 tickers, all racing on
    one council directory, 8 meetings. Checks: chain, replay, each round closed
    once, seq contiguous, every accepted submission's sha appears in its round's
    close, sealed/ == logged."""
    allok = True
    for i in range(8):
        d = fresh(f"c{i}")
        store.create(d, charter("ABCDE", quorum=3, lease_s=9999, arrival_grace_s=9999))
        jobs = [(d, s, n, i * 100 + n) for s in "ABCDE" for n in range(4)]
        with multiprocessing.Pool(len(jobs) + 2) as pool:
            tick = [pool.apply_async(_ticker, (d,)) for _ in range(2)]
            res = pool.map(_hammer, jobs)
            for t in tick:
                t.get()
        evs = store.events(d); ch = store.charter(d)
        chain = store.verify_chain(evs); rep = core.replay(ch, evs)
        seqs = [e["seq"] for e in evs]
        closes = [e for e in evs if e["type"] == "round-closed"]
        rids = [e["body"]["round"] for e in closes]
        st = core.fold(ch, evs)
        # every accepted submission appears in a close
        shas = {}
        for e in closes:
            for s, it in e["body"]["submitted"].items():
                shas[(e["body"]["round"], s)] = core.sha(it)
        recs = [json.loads(l) for l in open(os.path.join(d, "receipts.jsonl"))]
        acc = [r for r in recs if r["kind"] == "submitted"]
        lost = [r for r in acc if shas.get((r["round"], r["seat"])) != r["intent_sha"]]
        dup = sum(1 for r in acc if r.get("replaced"))
        sealed_ok = all(store.sealed(d, e["body"]["round"]) == e["body"]["submitted"] for e in closes)
        ok = (not chain and not rep and seqs == list(range(len(evs))) and len(rids) == len(set(rids))
              and st["adjourned"] and not lost and dup == 0 and sealed_ok and len(acc) == 5 * len(closes))
        say(f"  meeting {i}: events {len(evs)} rounds {len(closes)} accepted {len(acc)} lost {len(lost)} "
            f"replaced {dup} chain {'OK' if not chain else chain} replay {'OK' if not rep else 'BAD'} "
            f"adjourned {st['adjourned']} sealed==logged {sealed_ok} -> {'OK' if ok else 'FAIL'}")
        allok &= ok
    say("  RESULT:", "lock discipline held under 20 writers + 2 tickers x 8 meetings" if allok else "FAIL")
    return allok


def test_d1_partial_withdraw_resets_debate_limit():
    """Expected property (core.py step/debate): the charter's max_debate_rounds
    bounds debate on a motion. A co-maker's withdrawal re-emits motion-pending,
    and apply() resets debate_rounds_this_motion on every motion-pending."""
    ch = charter("ABC", quorum=2)
    evs = core.convene(ch)
    st = core.fold(ch, evs)

    def close(subs):
        nonlocal st, evs
        inp = {"type": "round-closed", "body": {"round": st["round"]["id"], "closed_by": "all-submitted",
                                                "submitted": {s: core.normalize_intent(i) for s, i in subs.items()},
                                                "gone": [], "absent": []}}
        der = core.step(st, ch, inp)
        evs += [inp] + der
        st = core.fold(ch, evs)
        return der

    close({s: {"action": "present"} for s in "ABC"})
    close({"A": {"action": "move", "text": "X"}, "B": {"action": "move", "text": "X"}, "C": {"action": "pass"}})
    n = 0
    put = None
    for i in range(12):
        if st["round"]["kind"] != "debate":
            break
        n += 1
        subs = {s: {"action": "speak", "text": f"d{n}"} for s in "ABC"}
        if n == 2:
            subs["A"] = {"action": "withdraw"}        # co-maker A withdraws; B remains
        der = close(subs)
        if any(e["type"] == "question-put" for e in der):
            put = n
            break
    say(f"  max_debate_rounds=3; one co-maker withdrew in debate round 2; question put after debate round {put}")
    ok = put == 3
    say("  RESULT:", "limit held" if ok else f"CONFIRMED DEFECT — partial withdrawal reset the debate-round limit (put at {put}, expected 3)")
    return ok


def test_d2_all_gone_adjourns_in_ticks():
    """max_inquorate_rounds is counted in *closes*, and a round with everyone
    gone closes on every tick: with no member present the meeting adjourns in
    two ticks, however fast they come."""
    d = fresh("d2"); store.create(d, charter("ABC", quorum=2, arrival_grace_s=0, lease_s=1))
    t0 = time.time(); closed = []
    for _ in range(5):
        closed.append(store.try_close(d))
    st = core.fold(store.charter(d), store.events(d))
    say(f"  ticks: {closed}; adjourned={st['adjourned']} after {time.time() - t0:.2f}s; reason: "
        f"{[e['body'].get('reason') for e in store.events(d) if e['type'] == 'adjourned']}")
    say("  RESULT: observation (design): inquorate rounds are ticks, not time; a convene with nobody arriving adjourns on the second tick after arrival_grace_s")
    return True


def test_d3_stuck_live_member():
    """A member whose server process heartbeats but whose model never submits
    holds every round for the full backstop; it is 'absent' each time, never
    'gone', and nothing escalates."""
    d = fresh("d3"); toks = store.create(d, charter("AB", quorum=1, backstop_s=3, lease_s=30))
    c = server(d, "A", toks["A"], principal="stuck")       # A is live (heartbeats) but never submits
    c.call("council_status", {})

    def n_closed():
        return sum(1 for e in store.events(d) if e["type"] == "round-closed")

    def wait_close(n_before, limit=10):
        t = time.time()
        while n_closed() == n_before and time.time() - t < limit:
            store.try_close(d); time.sleep(0.1)          # the server's own heartbeat thread also ticks
        return time.time() - t

    t0 = time.time()
    store.submit(d, "B", "r1", {"action": "present"}, {})
    t1 = wait_close(0)
    st = core.fold(store.charter(d), store.events(d))
    store.submit(d, "B", st["round"]["id"], {"action": "pass"}, {})
    t2 = wait_close(1)
    closes = [e["body"] for e in store.events(d) if e["type"] == "round-closed"]
    c.close()
    say(f"  r1 closed after {t1:.1f}s by {closes[0]['closed_by']} (absent {closes[0]['absent']}, gone {closes[0]['gone']}); "
        f"r2 closed after {t2:.1f}s by {closes[1]['closed_by']} (absent {closes[1]['absent']}, gone {closes[1]['gone']})")
    ok = all(c_["closed_by"] == "backstop-deadline" and c_["absent"] == ["A"] and c_["gone"] == [] for c_ in closes)
    say("  RESULT:", "confirmed: a live-but-silent seat costs the full backstop every round, with no escalation to 'gone'" if ok else "unexpected")
    return ok


def test_d4_late_submission_dropped():
    """contention.md F4: 'A late response is logged as supplementary, never
    silently dropped.' In the skeleton a submission arriving after the close is
    refused and its content is recorded nowhere (receipt carries no sha/text)."""
    d = fresh("d4"); store.create(d, charter("AB", quorum=1, backstop_s=1))
    store.submit(d, "B", "r1", {"action": "present"}, {})
    time.sleep(1.2); store.try_close(d, force_deadline=True)
    ok, msg = store.submit(d, "A", "r1", {"action": "present", "text": "LATE-BUT-REAL"}, {})
    recs = open(os.path.join(d, "receipts.jsonl")).read()
    say(f"  late submit -> ok={ok}: {msg}")
    say(f"  content of the late intent anywhere in receipts/events: {'LATE-BUT-REAL' in recs or 'LATE-BUT-REAL' in open(os.path.join(d, 'events.jsonl')).read()}; "
        f"refusal receipt carries intent_sha: {'intent_sha' in [l for l in recs.splitlines() if 'submit-refused' in l][-1]}")
    return not ok


def test_d5_replace_promise():
    """The brief and the charters say submissions are final; the tool description
    and the status view say the opposite."""
    d = fresh("d5"); toks = store.create(d, charter("AB"))
    c = server(d, "A", toks["A"])
    tl, _ = c.req("tools/list")
    desc = {t["name"]: t for t in tl["result"]["tools"]}
    sub = desc["council_submit"]
    say("  council_submit description says:", repr(sub["description"][:160]))
    say("  seat_token advertised on single-seat server:", "seat_token" in sub["inputSchema"]["properties"])
    t, err, _ = c.call("council_submit", {"round": "r1", "action": "present"})
    t2, err2, _ = c.call("council_status", {})
    line = [l for l in t2.splitlines() if "You have submitted" in l]
    say("  status view after submitting:", line[0][:140] if line else "(no line)")
    t3, err3, _ = c.call("council_submit", {"round": "r1", "action": "present", "text": "changed my mind"})
    say("  second submit ->", ("REFUSED: " if err3 else "OK: ") + t3.splitlines()[0][:120])
    c.close()
    contradiction = "replace" in sub["description"].lower() and err3
    say("  RESULT:", "CONFIRMED: members are told twice they may replace; the server refuses" if contradiction else "consistent")
    return contradiction


def _regenerate(ch, inputs):
    """Rebuild a full canonical log (with hashes) from charter + input events."""
    evs = list(core.convene(ch)); st = core.fold(ch, evs)
    for inp in inputs:
        der = core.step(st, ch, inp)
        evs += [inp] + der
        st = core.fold(ch, evs)
    out = []; prev = "0" * 64
    for i, e in enumerate(evs):
        body = {"seq": i, "prev": prev, "type": e["type"], "body": e["body"]}
        rec = dict(body, hash=core.sha(body)); out.append(rec); prev = rec["hash"]
    return out


def test_e_replay_is_not_tamper_evidence():
    """'Every meeting's log replays to itself and the hash chain verifies.' Both
    checks are internal to events.jsonl: a writer with directory access can
    change what a member said, regenerate, and both checks pass. Replay also
    does not consult sealed/."""
    src = os.path.join(HERE, "runs", "trial-4-mixed", "council")
    d = fresh("e"); shutil.copytree(src, d, ignore=shutil.ignore_patterns("lock", "tokens.json"))
    ch = store.charter(d); evs = store.events(d)
    inputs = [{"type": e["type"], "body": e["body"]} for e in evs if e["type"] in core.INPUT_TYPES]
    # tamper: flip every yes vote to no in r8
    for inp in inputs:
        if inp["body"]["round"] == "r8":
            for s, it in inp["body"]["submitted"].items():
                it["vote"] = "no"
    new = _regenerate(ch, inputs)
    with open(os.path.join(d, "events.jsonl"), "w") as f:
        f.write("".join(json.dumps(r, sort_keys=True, ensure_ascii=False) + "\n" for r in new))
    shutil.rmtree(os.path.join(d, "sealed"))
    out = subprocess.run([PY, "-I", os.path.join(HERE, "concilium", "steward.py"), "replay", d], capture_output=True, text=True).stdout.strip()
    res = [e for e in store.events(d) if e["type"] == "vote-result"][0]["body"]
    say(f"  tampered trial-4 (all r8 votes -> no, sealed/ deleted): steward replay says: {out}")
    say(f"  vote-result now: {res['result']} {res['tally']}")
    ok = "OK" in out and res["result"] == "rejected"
    say("  RESULT:", "CONFIRMED: replay+chain are derivation checks, not tamper evidence; sealed/ is not cross-checked" if ok else "unexpected")
    return ok


def test_f_gone_before_grace_when_never_present():
    """try_close treats a never-present seat as 'gone' only after arrival_grace_s;
    a seat that was present once and whose lease lapsed is 'gone' immediately.
    Check the asymmetry a late-arriving member faces: if it arrives after the
    grace it is 'gone' until its first call, and its first council_wait finds the
    round already closed without it."""
    d = fresh("f"); toks = store.create(d, charter("AB", quorum=1, arrival_grace_s=1, lease_s=30))
    store.submit(d, "B", "r1", {"action": "present"}, {})
    time.sleep(1.3)
    rid = store.try_close(d)
    b = [e["body"] for e in store.events(d) if e["type"] == "round-closed"][0]
    say(f"  B present, A never arrived, grace 1s: r1 closed={rid} by {b['closed_by']} gone={b['gone']}")
    return True


TESTS = [("A driver-mode seat gap", test_a_driver_gap), ("B SIGKILL lease lapse", test_b_sigkill_lapse),
         ("C concurrency stress", test_c_concurrency), ("D1 partial withdraw / debate limit", test_d1_partial_withdraw_resets_debate_limit),
         ("D2 all-gone adjourn in ticks", test_d2_all_gone_adjourns_in_ticks), ("D3 stuck live member", test_d3_stuck_live_member),
         ("D4 late submission dropped", test_d4_late_submission_dropped), ("D5 replace promise", test_d5_replace_promise),
         ("E replay is not tamper evidence", test_e_replay_is_not_tamper_evidence), ("F never-present vs grace", test_f_gone_before_grace_when_never_present)]

if __name__ == "__main__":
    want = sys.argv[1:]
    for name, fn in TESTS:
        if want and not any(name.startswith(w) for w in want):
            continue
        say(f"== {name}")
        try:
            fn()
        except Exception:
            say("  EXCEPTION:\n" + traceback.format_exc())
    with open(os.path.join(HERE, "verify-1", "verify-output.txt"), "w") as f:
        f.write("\n".join(OUT) + "\n")
