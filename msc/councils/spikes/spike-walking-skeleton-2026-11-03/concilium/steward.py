"""concilium steward CLI (for Joseph / the convener, and for debugging).

  python3 -m concilium.steward new DIR CHARTER.json      create + convene; prints seat tokens
  python3 -m concilium.steward status DIR [SEAT]          the view (as SEAT, else the first seat)
  python3 -m concilium.steward minutes DIR                the record
  python3 -m concilium.steward tick DIR [--deadline]      try to close the open round (or force the backstop)
  python3 -m concilium.steward replay DIR                 chain + every derived event recomputed + sealed files == revealed (internal consistency, not tamper evidence)
  python3 -m concilium.steward witness DIR LOG...         check the log against chain heads members' servers recorded in their own logs
  python3 -m concilium.steward presence DIR               live leases per seat
  python3 -m concilium.steward selftest                   latency-invariance property test (shuffled arrivals)
"""
import json
import os
import random
import shutil
import sys
import tempfile

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from concilium import store, core, view  # noqa: E402


def cmd_new(d, charter_path):
    ch = json.load(open(charter_path))
    toks = store.create(d, ch)
    print(json.dumps(toks, indent=1))


def sealed_check(d, evs):
    """Every closed round's revealed intents against the per-seat sealed files."""
    missing, mismatched = [], []
    for e in evs:
        if e["type"] != "round-closed":
            continue
        b = e["body"]
        sealed = store.sealed(d, b["round"])
        if not sealed and b["submitted"]:
            missing.append(b["round"])
        elif sealed != b["submitted"]:
            mismatched.append(b["round"])
    return missing, mismatched


def cmd_replay(d):
    """What this checks (verify-1 test E): that the log is internally consistent
    -- the hash chain links, every derived event is what the core computes from
    the charter and the input events, and every closed round's revealed intents
    equal the seats' sealed files. It is NOT tamper evidence against anyone who
    can write the council directory: such a writer can regenerate all three.
    `steward witness` is the check against records kept outside it."""
    ch = store.charter(d)
    evs = store.events(d)
    c = store.verify_chain(evs)
    bad = core.replay(ch, evs)
    missing, mismatched = sealed_check(d, evs)
    sealed_txt = ("OK" if not (missing or mismatched) else
                  f"MISMATCH in {mismatched}" if mismatched else f"sealed files MISSING for {missing}")
    print(f"events: {len(evs)}; chain: {'OK' if not c else c}; "
          f"derivation: {'OK' if not bad else 'MISMATCH'}; sealed vs revealed: {sealed_txt}")
    if bad:
        print(json.dumps(bad, indent=1))
    return 0 if not (c or bad or missing or mismatched) else 1


def cmd_witness(d, logs):
    """Check the council log against the chain heads that members' server
    processes recorded in their own logs (`head-observed` records, written to
    each member's --log, outside the council directory). Rewriting any event at
    or before a head a member was shown changes that head's hash, so it fails
    here unless every witness log is rewritten too. Logs from servers older than
    the witness change carry no heads and witness nothing."""
    evs = store.events(d)
    by_seq = {e["seq"]: e["hash"] for e in evs}
    n = bad = 0
    for path in logs:
        for line in open(path):
            try:
                r = json.loads(line)
            except Exception:
                continue
            if r.get("kind") != "head-observed":
                continue
            n += 1
            if by_seq.get(r["seq"]) != r["hash"]:
                bad += 1
                print(f"CONTRADICTED: {path} saw seq {r['seq']} = {r['hash'][:12]}, log has "
                      f"{(by_seq.get(r['seq']) or 'nothing')[:12]}")
    top = max((json.loads(l)["seq"] for p in logs for l in open(p) if '"head-observed"' in l), default=-1)
    print(f"witnessed heads: {n} from {len(logs)} log(s); contradicted: {bad}; "
          f"events covered by a witness: 0..{top} of {len(evs) - 1}")
    return 1 if bad or not n else 0


SCRIPT_CHARTER = {
    "council": "selftest", "purpose": "property test",
    "seats": [{"id": s} for s in "ABCDE"], "quorum": 3,
    "round_policy": {"backstop_s": 9999, "arrival_grace_s": 9999, "lease_s": 9999},
    "debate_policy": {"max_debate_rounds": 3, "max_open_rounds": 2},
    "agenda": [{"id": "q1", "title": "one"}, {"id": "q2", "title": "two"}],
}

# each round: seat -> list of intents (submitted in order; the last counts)
SCRIPT = [
    {s: [{"action": "present"}] for s in "ABCDE"},
    {"A": [{"action": "move", "text": "X"}], "B": [{"action": "discuss", "text": "hm"}, {"action": "move", "text": "Y"}],
     "C": [{"action": "move", "text": "X"}], "D": [{"action": "pass"}], "E": [{"action": "move", "text": "Z"}]},
    {"A": [{"action": "speak", "text": "for"}], "B": [{"action": "pass"}], "C": [{"action": "pass"}],
     "D": [{"action": "speak", "text": "against"}], "E": [{"action": "pass"}]},
    {s: [{"action": "pass"}] for s in "ABCDE"},
    {"A": [{"action": "vote", "vote": "yes"}], "B": [{"action": "vote", "vote": "no"}],
     "C": [{"action": "vote", "vote": "yes"}], "D": [{"action": "vote", "vote": "no"}, {"action": "vote", "vote": "abstain"}],
     "E": [{"action": "vote", "vote": "no"}]},
    # q1's motion failed while E's "Z" stands: the floor reopens on q1, E re-affirms
    {"A": [{"action": "pass"}], "B": [{"action": "pass"}], "C": [{"action": "pass"}], "D": [{"action": "pass"}],
     "E": [{"action": "move", "text": "Z"}]},
    {s: [{"action": "pass"}] for s in "ABCDE"},
    {s: [{"action": "vote", "vote": "yes" if s in "DE" else "no"}] for s in "ABCDE"},
    # q2
    {"A": [{"action": "pass"}], "B": [{"action": "move", "text": "Y"}], "C": [{"action": "pass"}],
     "D": [{"action": "pass"}], "E": [{"action": "move", "text": "Z"}]},
    {s: [{"action": "pass"}] for s in "ABCDE"},
    {s: [{"action": "vote", "vote": "yes"}] for s in "ABCDE"},
]


def run_script(d, rng, charter):
    store.create(d, charter)
    for rnd in SCRIPT:
        st = core.fold(charter, store.events(d))
        rid = st["round"]["id"]
        # interleave: flatten (seat, intent) with per-seat order kept, seats shuffled
        queues = {s: list(v) for s, v in rnd.items()}
        while any(queues.values()):
            s = rng.choice([k for k, v in queues.items() if v])
            it = queues[s].pop(0)
            # a seat's last intent must be its final one: only close-triggering
            # is the all-submitted condition, which fires on the first full set,
            # so seats with replacements submit them before the last seat's first.
            ok, msg = store.submit(d, s, rid, it, {"selftest": True})
            if not ok and "not open" in msg:
                break
    return open(os.path.join(d, "events.jsonl"), "rb").read()


def cmd_selftest(n=50):
    """Two variants of the same scripted meeting, n shuffled arrival orders each.
    replaceable=false: a seat's first submission is final -> expect 1 distinct log.
    replaceable=true: a later replacement races the all-submitted close -> expect >1
    (this is the finding the variant exists to show, not a bug to hide)."""
    rc = 0
    for replaceable in (False, True):
        ch = json.loads(json.dumps(SCRIPT_CHARTER))
        ch["round_policy"]["replaceable"] = replaceable
        tmp = tempfile.mkdtemp(prefix="concilium-selftest-")
        outs = {}
        for i in range(n):
            d = os.path.join(tmp, f"m{i}")
            b = run_script(d, random.Random(i), ch)
            outs.setdefault(b, i)
            if core.replay(ch, store.events(d)):
                print("replay mismatch in", d)
                return 1
            if store.verify_chain(store.events(d)):
                print("chain broken in", d)
                return 1
        shutil.rmtree(tmp)
        expect = "1" if not replaceable else ">1"
        print(f"replaceable={replaceable}: {n} shuffled arrival orders -> {len(outs)} distinct canonical log(s) "
              f"(expected {expect}); every run replays and chains OK")
        if (not replaceable and len(outs) != 1) or (replaceable and len(outs) == 1):
            rc = 1
    return rc


def main(argv):
    if not argv:
        print(__doc__)
        return 2
    c = argv[0]
    if c == "new":
        cmd_new(argv[1], argv[2])
    elif c == "status":
        d = argv[1]
        seat = argv[2] if len(argv) > 2 else store.charter(d)["seats"][0]["id"]
        print(view.render(d, seat, -1)[0])
    elif c == "minutes":
        print(view.minutes(argv[1]))
    elif c == "tick":
        print(store.try_close(argv[1], force_deadline="--deadline" in argv))
    elif c == "replay":
        return cmd_replay(argv[1])
    elif c == "witness":
        return cmd_witness(argv[1], argv[2:])
    elif c == "presence":
        print(json.dumps(store.leases(argv[1]), indent=1))
    elif c == "selftest":
        return cmd_selftest()
    else:
        print(__doc__)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]) or 0)
