"""concilium steward CLI (for Joseph / the convener, and for debugging).

  python3 -m concilium.steward new DIR CHARTER.json      create + convene; prints seat tokens
  python3 -m concilium.steward status DIR [SEAT]          the view (as SEAT, else the first seat)
  python3 -m concilium.steward minutes DIR                the record
  python3 -m concilium.steward tick DIR [--deadline]      try to close the open round (or force the backstop)
  python3 -m concilium.steward replay DIR                 recompute every derived event from the inputs; verify the chain
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


def cmd_replay(d):
    ch = store.charter(d)
    evs = store.events(d)
    c = store.verify_chain(evs)
    bad = core.replay(ch, evs)
    print(f"events: {len(evs)}; chain: {'OK' if not c else c}; replay: {'OK (every derived event recomputed identically)' if not bad else 'MISMATCH'}")
    if bad:
        print(json.dumps(bad, indent=1))
        return 1
    return 0 if not c else 1


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
