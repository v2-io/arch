"""
test_rounds.py — checks for the round mechanism, revision 2 (run: PYTHONPATH=. python3 test_rounds.py).

Each test names the claim in ../contention.md it supports and, where it exists, the finding in
../de-novo-feedback-1.md whose probe it now encodes. These check the toy; they do not verify
the RONR-12 model.
"""
from __future__ import annotations

import copy
import itertools
import random

from rounds import (Act, Policy, State, admissible, apply, collect_and_resolve, is_fragile, commute,
                    FLOOR_KINDS)

MEMBERS = ["ada", "ben", "cyd", "dee", "eli"]


def fresh(needs_second=False, members=MEMBERS) -> State:
    s = State(members=list(members), needs_second=needs_second)
    s.present = set(members)
    return s


def opened(s: State) -> State:
    s2 = copy.deepcopy(s)
    s2.round_no += 1
    return s2


def pol(*acts) -> Policy:
    return Policy(entries=[(None, a) for a in acts])


def gpol(*pairs) -> Policy:
    return Policy(entries=list(pairs))


def passes(*exclude):
    return [(m, pol(Act("pass", m, ()))) for m in MEMBERS if m not in exclude]


def last_transition(s: State) -> int:
    return s.transitions_in_round(s.round_no)[-1]


def random_policy(m: str, s: State, rng: random.Random) -> Policy:
    t = s.top()
    choices = [Act("pass", m, ())]
    if s.phase in ("vote", "ruling", "objection-vote"):
        choices = [Act("vote", m, (rng.choice(["aye", "no", "abstain"]),)), Act("pass", m, ())]
    else:
        if t is None:
            choices.append(Act("move", m, ("main", "main by " + m)))
        else:
            for k in ("amend", "commit", "postpone", "previous-question", "lay-on-table", "adjourn"):
                choices.append(Act("move", m, (k, k + " by " + m)))
            choices.append(Act("speak", m, (rng.choice(["for", "against"]), m + " speaks")))
            choices.append(Act("second", m, (t.fid,)))
            choices.append(Act("object-consideration", m, ()))
        for tr in s.transitions_in_round(s.round_no):
            choices.append(Act("point-of-order", m, ("point by " + m, tr)))
    k = rng.randint(1, 3)
    return Policy(entries=[(None, a) for a in rng.sample(choices, min(k, len(choices)))])


def walk(s: State, rng: random.Random, n: int) -> State:
    for _ in range(n):
        s = collect_and_resolve(s, [(m, random_policy(m, s, rng)) for m in s.members])
    return s


# ----------------------------------------------------------------------------
def test_f1_invariance(trials=200, seed=7):
    rng = random.Random(seed)
    for trial in range(trials):
        s = walk(fresh(needs_second=rng.random() < 0.5), rng, rng.randint(0, 6))
        prof = [(m, random_policy(m, s, rng)) for m in MEMBERS]
        fps = {collect_and_resolve(s, list(p)).fingerprint()
               for p in itertools.islice(itertools.permutations(prof), 0, 120, 7)}
        assert len(fps) == 1, f"trial {trial}: {len(fps)} distinct outcomes under permutation"
    print(f"F1  arrival-order invariance: {trials} random states x 18 permutations — identical records")


# ----------------------------------------------------------------------------
def relabel_state(s: State, old: str, new: str) -> State:
    """Rename a member keeping its roster slot (the verifier's probe, feedback 1.3)."""
    s2 = copy.deepcopy(s)
    def r(x):
        return new if x == old else x
    s2.members = [r(m) for m in s2.members]
    s2.present = {r(m) for m in s2.present}
    s2.speeches = {(r(k[0]), k[1]): v for k, v in s2.speeches.items()}
    s2.last_floor = {r(k): v for k, v in s2.last_floor.items()}
    s2.claims_this_round = {r(m) for m in s2.claims_this_round}
    for f in s2.stack:
        f.maker = r(f.maker)
        f.seconded_by = sorted(r(m) for m in f.seconded_by)
    for p in s2.points:
        p["by"] = r(p["by"])
    if s2.vote:
        s2.vote["ballots"] = {r(k): v for k, v in s2.vote["ballots"].items()}
    for e in s2.record:
        if "member" in e:
            e["member"] = r(e["member"])
        if "tally" in e:
            e["tally"] = {r(k): v for k, v in e["tally"].items()}
        if "not_voting" in e:
            e["not_voting"] = sorted(r(m) for m in e["not_voting"])
    for e in s2.claim_log:
        e["member"] = r(e["member"])
        if isinstance(e["intent"], tuple):
            e["intent"] = (e["intent"][0], r(e["intent"][1]), e["intent"][2])
    return s2


def relabel_policy(p: Policy, old: str, new: str) -> Policy:
    return Policy(entries=[(g, Act(a.kind, new if a.member == old else a.member, a.args)) for g, a in p.entries])


def canon(s: State, old: str, new: str):
    """The record with `new` mapped back to `old`, for comparison."""
    s2 = relabel_state(s, new, old)
    return s2.fingerprint()


def test_f3_relabeling(trials=600, seed=11):
    """F3: resolve commutes with relabeling a member (slot kept). Feedback 1.3 measured 347/1500
    divergences in the first toy; this is the property test it asked for."""
    rng = random.Random(seed)
    for trial in range(trials):
        s = walk(fresh(needs_second=rng.random() < 0.5), rng, rng.randint(0, 7))
        prof = [(m, random_policy(m, s, rng)) for m in MEMBERS]
        victim = rng.choice(MEMBERS)
        new = rng.choice(["aaa", "zed", "mmm"])
        s_r = relabel_state(s, victim, new)
        prof_r = [(new if m == victim else m, relabel_policy(p, victim, new)) for m, p in prof]
        out = collect_and_resolve(s, prof)
        out_r = collect_and_resolve(s_r, prof_r)
        assert out.fingerprint() == canon(out_r, victim, new), f"trial {trial}: record moved with the member's name"
    print(f"F3  relabeling equivariance: {trials} random states, one member renamed to sort first/last — identical")


# ----------------------------------------------------------------------------
def test_f4_deadline_and_vote_leak():
    """F4: record depends only on the on-time set; a late ballot is never counted and never seen
    (feedback 1.5: the first toy let a late voter see a partial tally)."""
    rng = random.Random(3)
    for trial in range(100):
        s = walk(fresh(), rng, rng.randint(0, 4))
        prof = [(m, random_policy(m, s, rng)) for m in MEMBERS]
        cut = rng.randint(1, len(prof))
        fps = set()
        for p1 in itertools.islice(itertools.permutations(prof[:cut]), 0, 24, 3):
            fps.add(collect_and_resolve(s, list(p1) + prof[cut:], deadline_cut=cut).fingerprint())
        assert len(fps) == 1
    # the leak probe
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    s = collect_and_resolve(s, passes())                       # nobody debates -> put
    assert s.phase == "vote"
    ballots = {"ada": "aye", "ben": "aye", "cyd": "no", "dee": "no"}
    prof = [(m, pol(Act("vote", m, (ballots[m],)))) for m in ["ada", "ben", "cyd", "dee"]] + \
           [("eli", pol(Act("vote", "eli", ("aye",))))]
    out = collect_and_resolve(s, prof, deadline_cut=4)          # eli is late
    ann = [e for e in out.record if e.get("chair") == "announced"][-1]
    assert ann["not_voting"] == ["eli"] and "eli" not in ann["tally"], ann
    assert out.phase == "open" and out.vote is None, "the vote round closed at its deadline"
    assert not any(isinstance(e["intent"], tuple) and e["intent"][0] == "vote" and e["intent"][2] != "(sealed)"
                   for e in out.claim_log), "no ballot content in the claim log"
    assert any(str(e["intent"]).startswith("late") and "sealed" in str(e["intent"]) for e in out.claim_log)
    print("F4  deadline: record depends only on the on-time set; the vote round closed at its deadline, "
          "eli recorded as not voting, no ballot visible before announcement")


# ----------------------------------------------------------------------------
def test_point_of_order_bounded():
    """Feedback 1.2: one member could halt the floor every round. Now a point names the transition
    it challenges, is timely only in the next round, and cannot be re-raised against it."""
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    speeches, points = 0, 0
    for _ in range(20):
        if s.phase != "open":
            s = collect_and_resolve(s, [(m, pol(Act("vote", m, ("no",)))) for m in MEMBERS])
            continue
        trs = s.transitions_in_round(s.round_no)
        eli = [Act("point-of-order", "eli", ("obstruct", tr)) for tr in trs] or [Act("pass", "eli", ())]
        s = collect_and_resolve(s, [("ben", pol(Act("speak", "ben", ("for", "..."))))] +
                                   [("eli", pol(*eli))] + passes("ben", "eli"))
        speeches = sum(1 for e in s.record if e.get("act") == "speak")
        points = sum(1 for e in s.record if e.get("act") == "point-of-order")
    assert speeches >= 2, (speeches, points)
    # and a second point against an already-ruled transition is refused
    s2 = fresh()
    s2 = collect_and_resolve(s2, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    tr = last_transition(s2)
    s2 = collect_and_resolve(s2, [("eli", pol(Act("point-of-order", "eli", ("x", tr))))] + passes("eli"))
    s2 = collect_and_resolve(s2, [(m, pol(Act("vote", m, ("no",)))) for m in MEMBERS])
    assert not admissible(Act("point-of-order", "eli", ("x again", tr)), opened(s2))
    print(f"P   points of order are breach-specific and not re-raisable: an obstructor got {points} halts "
          f"in 20 rounds and ben still spoke {speeches} times")


# ----------------------------------------------------------------------------
def test_mover_claim_does_not_flip_debater():
    """Feedback 1.4: a third member's motion claim must not change which debater speaks."""
    def setup():
        s = fresh()
        s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
        s = collect_and_resolve(s, [("ada", pol(Act("speak", "ada", ("for", "..."))))] + passes("ada"))
        return s
    base = [("ben", pol(Act("speak", "ben", ("for", "b")))), ("cyd", pol(Act("speak", "cyd", ("against", "c"))))]
    s1 = collect_and_resolve(setup(), base + passes("ben", "cyd"))
    s2 = collect_and_resolve(setup(), base + [("dee", pol(Act("move", "dee", ("amend", "x"))))] + passes("ben", "cyd", "dee"))
    w1 = [e for e in s1.record if e.get("act") in FLOOR_KINDS][-1]
    w2 = [e for e in s2.record if e.get("act") in FLOOR_KINDS][-1]
    assert w1["member"] == "cyd" and w1["act"] == "speak", w1          # alternation [42:9(3)]
    assert w2["member"] in ("cyd", "dee"), w2                            # dee may win the race, never ben
    print("1.4 alternation is ranked among debaters first; a mover's claim cannot flip which debater speaks")


# ----------------------------------------------------------------------------
def test_i1_implicit_guard():
    """I1: a floor act fires only if the state it assumed is unchanged by this round's non-floor acts."""
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    tr = last_transition(s)
    # ben wants to speak; eli objects to consideration in the same round (a non-floor act that changes phase)
    out = collect_and_resolve(s, [("ben", pol(Act("speak", "ben", ("for", "b")))),
                                  ("eli", pol(Act("object-consideration", "eli", ())))] + passes("ben", "eli"))
    assert not any(e.get("act") == "speak" for e in out.record), "the speech must not fire blind"
    assert any("deferred" in str(e.get("chair", "")) for e in out.record) or out.phase == "objection-vote"
    # and the guarded-policy routing from the first spike still holds
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    only_main = lambda st: len(st.stack) == 1
    prof = [("ben", pol(Act("move", "ben", ("amend", "insert 'tag'")))),
            ("cyd", gpol((only_main, Act("move", "cyd", ("postpone", "until later"))),
                         (lambda st: len(st.stack) > 1 and st.top().kind == "amend",
                          Act("speak", "cyd", ("against", "wrong question")))))] + passes("ben", "cyd")
    s1 = collect_and_resolve(s, prof)
    fl = [e for e in s1.record if e.get("round") == s1.round_no and e.get("act") in FLOOR_KINDS]
    assert len(fl) == 1
    if fl[0]["member"] == "ben":
        s2 = collect_and_resolve(s1, prof)
        fl2 = [e for e in s2.record if e.get("round") == s2.round_no and e.get("act") in FLOOR_KINDS]
        assert fl2 and fl2[0]["member"] == "cyd" and fl2[0]["act"] == "speak", fl2
    print("I1  a floor act is deferred when in-round acts changed its snapshot; guarded routing re-decides next round")


# ----------------------------------------------------------------------------
def test_f3_ladder():
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    want = lambda m: pol(Act("speak", m, ("for", m)))
    order = []
    for _ in range(5):
        s = collect_and_resolve(s, [(m, want(m)) for m in MEMBERS])
        order.append([e for e in s.record if e.get("round") == s.round_no and e.get("act") == "speak"][0]["member"])
    assert order[0] == "ada", order
    assert sorted(order) == sorted(MEMBERS), order
    print("F3  ladder: maker first [42:9(1)]; everyone once before anyone twice [42:9(2) via 43:13]; "
          "then least-recently-floored, then a recorded lottery by slot")


# ----------------------------------------------------------------------------
def test_d1_classification_and_d2():
    s = fresh(needs_second=True)
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    t = s.top()
    assert is_fragile(Act("second", "ben", (t.fid,)), opened(s))
    s2 = collect_and_resolve(s, [("ben", pol(Act("second", "ben", (t.fid,))))] + passes("ben"))
    o = opened(s2)
    tr = last_transition(s2)
    probes = {"point-of-order": Act("point-of-order", "cyd", ("x", tr)),
              "object-consideration": Act("object-consideration", "cyd", ()),
              "move amend": Act("move", "cyd", ("amend", "...")),
              "move previous-question": Act("move", "cyd", ("previous-question", "...")),
              "speak": Act("speak", "cyd", ("against", "..."))}
    derived = {k: is_fragile(a, o) for k, a in probes.items()}
    assert derived == {"point-of-order": True, "object-consideration": True, "move amend": False,
                       "move previous-question": False, "speak": False}, derived
    # classification is stable under relabeling (feedback 1.3's third channel)
    o_r = relabel_state(o, "ada", "zed")
    derived_r = {k: is_fragile(a, o_r) for k, a in probes.items()}
    assert derived_r == derived
    assert commute(Act("second", "ben", (t.fid,)), Act("second", "cyd", (t.fid,)), opened(s))
    o2 = opened(collect_and_resolve(fresh(), [("ada", pol(Act("move", "ada", ("main", "m"))))] + passes("ada")))
    assert not commute(Act("move", "ben", ("amend", "a")), Act("move", "cyd", ("amend", "b")), o2)
    print("D1/D2 fragility classification (as a diff tool, not an oracle) and commutation: as before; "
          "classification now stable under relabeling")


# ----------------------------------------------------------------------------
def specimen():
    s = fresh()
    P = lambda m, *acts: (m, pol(*acts))
    s = collect_and_resolve(s, [P("ada", Act("move", "ada", ("main", "Reset canon to the pre-sprint tag")))] + passes("ada"))
    s = collect_and_resolve(s, [P("ada", Act("speak", "ada", ("for", "findability: the eight checks added nothing"))),
                                P("ben", Act("speak", "ben", ("against", "I re-derived the mathematics; the checks are real"))),
                                P("cyd", Act("speak", "cyd", ("against", "re-integration cost of a reset is high")))] + passes("ada", "ben", "cyd"))
    s = collect_and_resolve(s, [P("ben", Act("speak", "ben", ("against", "I re-derived the mathematics; the checks are real"))),
                                P("cyd", Act("speak", "cyd", ("against", "re-integration cost of a reset is high")))] + passes("ben", "cyd"))
    s = collect_and_resolve(s, [P("cyd", Act("speak", "cyd", ("against", "re-integration cost of a reset is high"))),
                                P("dee", Act("move", "dee", ("amend", "insert 'after an independent re-derivation of the eight checks'")))] + passes("cyd", "dee"))
    tr = last_transition(s)
    s = collect_and_resolve(s, [P("dee", Act("move", "dee", ("amend", "insert 'after an independent re-derivation of the eight checks'"))),
                                P("eli", Act("point-of-order", "eli", ("the last speech was not germane to the pending question", tr)))] + passes("dee", "eli"))
    while s.phase != "open":
        s = collect_and_resolve(s, [(m, pol(Act("vote", m, ("no" if m != "eli" else "aye",)))) for m in MEMBERS])
    s = collect_and_resolve(s, [P("dee", Act("move", "dee", ("amend", "insert 'after an independent re-derivation of the eight checks'")))] + passes("dee"))
    s = collect_and_resolve(s, passes())
    s = collect_and_resolve(s, [(m, pol(Act("vote", m, ("aye",)))) for m in MEMBERS])
    s = collect_and_resolve(s, passes())
    s = collect_and_resolve(s, [(m, pol(Act("vote", m, ("no" if m == "ada" else "aye",)))) for m in MEMBERS])
    return s


if __name__ == "__main__":
    test_f1_invariance()
    test_f3_relabeling()
    test_f4_deadline_and_vote_leak()
    test_point_of_order_bounded()
    test_mover_claim_does_not_flip_debater()
    test_i1_implicit_guard()
    test_f3_ladder()
    test_d1_classification_and_d2()
    s = specimen()
    print("\nSpecimen record (committee configuration, no seconds):")
    for e in s.record:
        print("  ", e)
    print("\nall checks passed")
