"""
test_rounds.py — checks for the round mechanism, revision 3 (run: PYTHONPATH=. python3 test_rounds.py).

Each test names the claim in ../contention.md it supports and, where one exists, the finding in
../de-novo-feedback-1.md or ../de-novo-feedback-2.md whose probe it encodes. These check the toy;
they do not verify the RONR-12 model.
"""
from __future__ import annotations

import copy
import itertools
import random

from rounds import (Act, Policy, State, admissible, apply, collect_and_resolve, is_fragile, commute,
                    FLOOR_KINDS, claim_key)

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


def floor_winner(s: State):
    fl = [e for e in s.record if e.get("round") == s.round_no and e.get("act") in FLOOR_KINDS]
    return (fl[0]["member"], fl[0]["act"]) if fl else None


GUARDS = {
    "none": None,
    "stack>1": lambda st: len(st.stack) > 1,
    "stack==1": lambda st: len(st.stack) == 1,
    "open": lambda st: st.phase == "open",
    "top-amend": lambda st: st.top() is not None and st.top().kind == "amend",
}


def random_policy(m: str, s: State, rng: random.Random, guarded=True) -> Policy:
    t = s.top()
    choices = [Act("pass", m, ())]
    if s.phase in ("vote", "ruling", "objection-vote"):
        choices = [Act("vote", m, (rng.choice(["aye", "no", "abstain"]),)), Act("pass", m, ())]
        if s.phase == "vote":
            for tr in s.transitions_in_round(s.round_no):
                choices.append(Act("point-of-order", m, ("point by " + m, tr)))
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
    acts = rng.sample(choices, min(k, len(choices)))
    gs = [GUARDS[rng.choice(list(GUARDS))] if guarded else None for _ in acts]
    return Policy(entries=list(zip(gs, acts)))


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
    print(f"F1  arrival-order invariance: {trials} random states x 18 permutations, guarded policies — identical")


# ----------------------------------------------------------------------------
def relabel_state(s: State, old: str, new: str) -> State:
    s2 = copy.deepcopy(s)
    r = lambda x: new if x == old else x
    s2.members = [r(m) for m in s2.members]
    s2.present = {r(m) for m in s2.present}
    s2.speeches = {(r(k[0]), k[1]): v for k, v in s2.speeches.items()}
    s2.last_floor = {r(k): v for k, v in s2.last_floor.items()}
    s2.claims_this_round = {r(m) for m in s2.claims_this_round}
    s2.ruled_targets = sorted((tg, r(m)) for tg, m in map(tuple, s2.ruled_targets))
    for f in s2.stack:
        f.maker = r(f.maker)
        f.seconded_by = sorted(r(m) for m in f.seconded_by)
    for p in s2.points:
        p["by"] = r(p["by"])
    if s2.vote:
        s2.vote["ballots"] = {r(k): v for k, v in s2.vote["ballots"].items()}
        if "by" in s2.vote:
            s2.vote["by"] = r(s2.vote["by"])
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


def test_f3_relabeling(trials=600, seed=11):
    rng = random.Random(seed)
    for trial in range(trials):
        s = walk(fresh(needs_second=rng.random() < 0.5), rng, rng.randint(0, 7))
        prof = [(m, random_policy(m, s, rng)) for m in MEMBERS]
        victim = rng.choice(MEMBERS)
        new = rng.choice(["aaa", "zed", "mmm"])
        out = collect_and_resolve(s, prof)
        out_r = collect_and_resolve(relabel_state(s, victim, new),
                                    [(new if m == victim else m, relabel_policy(p, victim, new)) for m, p in prof])
        assert out.fingerprint() == relabel_state(out_r, new, victim).fingerprint(), f"trial {trial}"
    print(f"F3  relabeling equivariance: {trials} random states, guarded policies, one member renamed — identical")


# ----------------------------------------------------------------------------
def test_independence_of_losing_claims(trials=800, seed=5):
    """Feedback 2 §1.2: adding a claim that is not selected must not change which claim is selected,
    except the 43:13 deferral in favor of the added unspoken debater. A single per-claim key gives
    this by construction; this test checks the whole resolver, not just the key."""
    rng = random.Random(seed)
    checked = 0
    for trial in range(trials):
        s = walk(fresh(), rng, rng.randint(1, 6))
        if s.phase != "open" or s.top() is None:
            continue
        claimants = rng.sample(MEMBERS, rng.randint(1, 3))
        base = []
        for m in MEMBERS:
            if m in claimants:
                base.append((m, pol(rng.choice([Act("speak", m, (rng.choice(["for", "against"]), m)),
                                                 Act("move", m, (rng.choice(["amend", "postpone", "previous-question", "main"]), m))]))))
            else:
                base.append((m, pol(Act("pass", m, ()))))
        w0 = floor_winner(collect_and_resolve(s, base))
        if w0 is None:
            continue
        newcomer = rng.choice([m for m in MEMBERS if m not in claimants] or MEMBERS)
        extra = rng.choice([Act("speak", newcomer, (rng.choice(["for", "against"]), "x")),
                            Act("move", newcomer, (rng.choice(["amend", "postpone", "previous-question", "main"]), "x"))])
        prof = [(m, pol(extra) if m == newcomer else p) for m, p in base]
        out = collect_and_resolve(s, prof)
        w1 = floor_winner(out)
        checked += 1
        if w1 == (newcomer, extra.kind):
            continue                                   # the added claim won: allowed
        if w1 == w0:
            continue                                   # unchanged: the property
        # the one licensed exception: the old winner's second speech deferred by an unspoken newcomer's speech
        old_m, old_k = w0
        t = s.top()
        if (old_k == "speak" and extra.kind == "speak" and admissible(extra, opened(s))
                and s.speeches.get((old_m, t.fid), 0) == 1 and s.speeches.get((newcomer, t.fid), 0) == 0):
            continue
        raise AssertionError(f"trial {trial}: adding a losing claim {extra.key()} changed the winner {w0} -> {w1}")
    print(f"IND independence of losing claims: {checked} random contests, a non-winning claim never changed the winner "
          f"(except 43:13's deferral)")


# ----------------------------------------------------------------------------
def test_no_cloture_by_out_of_order_claim():
    """Feedback 2 §1.1: an inadmissible floor claim must not bar a second speech and close debate."""
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    s = collect_and_resolve(s, [("ada", pol(Act("speak", "ada", ("for", "a"))))] + passes("ada"))
    s = collect_and_resolve(s, [("ben", pol(Act("speak", "ben", ("against", "b"))))] + passes("ben"))
    out = collect_and_resolve(s, [("ben", pol(Act("speak", "ben", ("against", "b2")))),
                                  ("cyd", pol(Act("move", "cyd", ("main", "out of order"))))] + passes("ben", "cyd"))
    assert floor_winner(out) == ("ben", "speak"), [e for e in out.record if e.get("round") == out.round_no]
    # and a speech deferred by 43:13 keeps the question open rather than putting it
    out2 = collect_and_resolve(s, [("ben", pol(Act("speak", "ben", ("against", "b2")))),
                                   ("cyd", pol(Act("speak", "cyd", ("for", "c"))))] + passes("ben", "cyd"))
    assert floor_winner(out2) == ("cyd", "speak") and out2.phase == "open"
    print("1.1 an out-of-order claim is invisible to the 43:13 bar; a deferred second speech does not close debate")


# ----------------------------------------------------------------------------
def test_point_does_not_redraw_lottery():
    """Feedback 2 §1.3: a rejected point of order between two contests must not change a lottery tie."""
    changed = 0
    for seed in range(40):
        def setup():
            s = fresh(); s.seed = f"seed-{seed}"
            s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
            return collect_and_resolve(s, [("ada", pol(Act("speak", "ada", ("for", "a"))))] + passes("ada"))
        tie = [("ben", pol(Act("speak", "ben", ("for", "b")))), ("cyd", pol(Act("speak", "cyd", ("for", "c"))))]
        w_direct = floor_winner(collect_and_resolve(setup(), tie + passes("ben", "cyd")))
        s = setup()
        s = collect_and_resolve(s, [("eli", pol(Act("point-of-order", "eli", ("frivolous", last_transition(s)))))] + passes("eli"))
        s = collect_and_resolve(s, [(m, pol(Act("vote", m, ("no",)))) for m in MEMBERS])
        w_after = floor_winner(collect_and_resolve(s, tie + passes("ben", "cyd")))
        changed += w_direct != w_after
    assert changed == 0, changed
    print("1.3 the lottery is keyed by floor-grant count: a point of order redrew 0 of 40 ties (was 20 of 40)")


# ----------------------------------------------------------------------------
def test_point_in_vote_round_voids_poll():
    """Feedback 2 §1.4: a point against the put, raised in the vote round, voids the poll."""
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    s = collect_and_resolve(s, passes())
    assert s.phase == "vote"
    tr = last_transition(s)
    out = collect_and_resolve(s, [("eli", pol(Act("point-of-order", "eli", ("the put misstates the question", tr))))] +
                                 [(m, pol(Act("vote", m, ("aye",)))) for m in MEMBERS if m != "eli"])
    assert out.phase == "ruling" and out.vote["on"].startswith("point-of-order"), (out.phase, out.vote)
    assert not any(e.get("chair") == "announced" for e in out.record)
    s2 = collect_and_resolve(out, [(m, pol(Act("vote", m, ("no",)))) for m in MEMBERS])   # point not sustained
    assert s2.phase == "open" and s2.top() is not None
    # the announcement is itself challengeable in the round after it
    s3 = collect_and_resolve(s2, passes())                                                  # put again
    s3 = collect_and_resolve(s3, [(m, pol(Act("vote", m, ("aye",)))) for m in MEMBERS])
    ann = [i for i, e in enumerate(s3.record) if e.get("chair") == "announced"][-1]
    assert admissible(Act("point-of-order", "ben", ("conduct of the vote", ann)), opened(s3))
    print("1.4 a point in the vote round voids the poll; the announcement is a challengeable transition")


# ----------------------------------------------------------------------------
def test_point_and_objection_ordering():
    """Feedback 2 §1.5: a point and an objection in one round — the point is ruled first, the objection
    deferred with its window intact; two members' points on one transition are both ruled."""
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    tr = last_transition(s)
    out = collect_and_resolve(s, [("ben", pol(Act("point-of-order", "ben", ("not in order", tr)))),
                                  ("cyd", pol(Act("object-consideration", "cyd", ())))] + passes("ben", "cyd"))
    assert out.phase == "ruling" and out.vote["by"] == "ben"
    assert any(e.get("deferred") for e in out.claim_log if e["member"] == "cyd")
    out = collect_and_resolve(out, [(m, pol(Act("vote", m, ("no",)))) for m in MEMBERS])
    assert out.phase == "open" and admissible(Act("object-consideration", "cyd", ()), opened(out)), "objection window intact"
    # two points, two members, one transition: both ruled
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    tr = last_transition(s)
    out = collect_and_resolve(s, [("cyd", pol(Act("point-of-order", "cyd", ("outside the objects", tr)))),
                                  ("eli", pol(Act("point-of-order", "eli", ("conflicts with a rule", tr))))] + passes("cyd", "eli"))
    ruled = []
    for _ in range(3):
        if out.phase == "ruling":
            ruled.append(out.vote["by"])
            out = collect_and_resolve(out, [(m, pol(Act("vote", m, ("no",)))) for m in MEMBERS])
    assert sorted(ruled) == ["cyd", "eli"], ruled
    print("1.5 point before objection, objection deferred not lost; two members' points on one transition both ruled")


# ----------------------------------------------------------------------------
def test_silent_is_not_pass():
    """Feedback 2 §1.6: a member past the deadline must not count as having declined to debate or second."""
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    prof = passes("ben") + [("ben", pol(Act("speak", "ben", ("against", "b"))))]
    out = collect_and_resolve(s, prof, deadline_cut=4)        # ben late
    assert out.phase == "open" and not any(e.get("chair") == "put" for e in out.record), "silence must not put the question"
    s2 = fresh(needs_second=True)
    s2 = collect_and_resolve(s2, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    out2 = collect_and_resolve(s2, passes("ben") + [("ben", pol(Act("second", "ben", (1,))))], deadline_cut=4)
    assert out2.top() is not None, "a late seconder must not kill the motion"
    # an empty poll is void, not a rejection
    s3 = fresh()
    s3 = collect_and_resolve(s3, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    s3 = collect_and_resolve(s3, passes())
    out3 = collect_and_resolve(s3, [], deadline_cut=0)
    assert out3.phase == "vote" and any("poll void" in str(e.get("chair")) for e in out3.record)
    print("1.6 silence extends: no put, no 'no second', no 0-0 rejection on a round nobody answered")


# ----------------------------------------------------------------------------
def test_f4_deadline_and_vote_leak():
    rng = random.Random(3)
    for trial in range(100):
        s = walk(fresh(), rng, rng.randint(0, 4))
        prof = [(m, random_policy(m, s, rng)) for m in MEMBERS]
        cut = rng.randint(1, len(prof))
        fps = {collect_and_resolve(s, list(p1) + prof[cut:], deadline_cut=cut).fingerprint()
               for p1 in itertools.islice(itertools.permutations(prof[:cut]), 0, 24, 3)}
        assert len(fps) == 1
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    s = collect_and_resolve(s, passes())
    ballots = {"ada": "aye", "ben": "aye", "cyd": "no", "dee": "no"}
    prof = [(m, pol(Act("vote", m, (ballots[m],)))) for m in ["ada", "ben", "cyd", "dee"]] + \
           [("eli", pol(Act("vote", "eli", ("aye",))))]
    out = collect_and_resolve(s, prof, deadline_cut=4)
    ann = [e for e in out.record if e.get("chair") == "announced"][-1]
    assert ann["not_voting"] == ["eli"] and "eli" not in ann["tally"]
    assert not any(isinstance(e["intent"], tuple) and e["intent"][0] == "vote" and e["intent"][2] != "(sealed)"
                   for e in out.claim_log)
    # duplicates refused; first ballot per policy
    out2 = collect_and_resolve(s, [("ben", pol(Act("vote", "ben", ("aye",)), Act("vote", "ben", ("no",))))] +
                                  [("ben", pol(Act("vote", "ben", ("no",))))] +
                                  [(m, pol(Act("vote", m, ("no",)))) for m in MEMBERS if m != "ben"])
    ann2 = [e for e in out2.record if e.get("chair") == "announced"][-1]
    assert ann2["tally"]["ben"] == "aye"
    assert any("duplicate" in str(e["intent"]) for e in out2.claim_log)
    print("F4  deadline: on-time set only; late ballot not counted, not seen; duplicate submission refused; "
          "first ballot per policy")


# ----------------------------------------------------------------------------
def test_i1_implicit_guard():
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    out = collect_and_resolve(s, [("ben", pol(Act("speak", "ben", ("for", "b")))),
                                  ("eli", pol(Act("object-consideration", "eli", ())))] + passes("ben", "eli"))
    assert not any(e.get("act") == "speak" for e in out.record)
    assert out.phase == "objection-vote"
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    prof = [("ben", pol(Act("move", "ben", ("amend", "insert 'tag'")))),
            ("cyd", gpol((GUARDS["stack==1"], Act("move", "cyd", ("postpone", "until later"))),
                         (GUARDS["top-amend"], Act("speak", "cyd", ("against", "wrong question")))))] + passes("ben", "cyd")
    s1 = collect_and_resolve(s, prof)
    w = floor_winner(s1)
    assert w is not None
    if w[0] == "ben":
        s2 = collect_and_resolve(s1, prof)            # the same STANDING policy, re-evaluated next round
        assert floor_winner(s2) == ("cyd", "speak")
    print("I1  floor act deferred when in-round acts changed its snapshot; a standing guarded policy re-routes next round")


# ----------------------------------------------------------------------------
def test_ladder():
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    order = []
    for _ in range(5):
        s = collect_and_resolve(s, [(m, pol(Act("speak", m, ("for", m)))) for m in MEMBERS])
        order.append(floor_winner(s)[0])
    assert order[0] == "ada" and sorted(order) == sorted(MEMBERS), order
    # a declared side no longer buys the floor: honest and dishonest declarers are ranked the same
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] + passes("ada"))
    s = collect_and_resolve(s, [("ada", pol(Act("speak", "ada", ("for", "a"))))] + passes("ada"))
    w_honest = floor_winner(collect_and_resolve(s, [("ben", pol(Act("speak", "ben", ("against", "b")))),
                                                    ("cyd", pol(Act("speak", "cyd", ("against", "c"))))] + passes("ben", "cyd")))
    w_lie = floor_winner(collect_and_resolve(s, [("ben", pol(Act("speak", "ben", ("against", "b")))),
                                                 ("cyd", pol(Act("speak", "cyd", ("for", "c"))))] + passes("ben", "cyd")))
    assert w_honest == w_lie
    print("LAD maker first [42:9(1)]; everyone once before anyone twice [43:13]; then least-recently-floored, "
          "then lottery; a declared side is recorded and buys nothing")


# ----------------------------------------------------------------------------
def test_d1_d2():
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
    assert {k: is_fragile(a, relabel_state(o, "ada", "zed")) for k, a in probes.items()} == derived
    assert commute(Act("second", "ben", (t.fid,)), Act("second", "cyd", (t.fid,)), opened(s))
    print("D1/D2 fragility classification (a diff tool) and commutation, as before")


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
                                P("ben", Act("speak", "ben", ("against", "second speech")))] + passes("cyd", "ben"))
    s = collect_and_resolve(s, [P("ben", Act("speak", "ben", ("against", "second speech"))),
                                P("dee", Act("move", "dee", ("amend", "insert 'after an independent re-derivation of the eight checks'")))] + passes("ben", "dee"))
    tr = last_transition(s)
    s = collect_and_resolve(s, [P("dee", Act("move", "dee", ("amend", "insert 'after an independent re-derivation of the eight checks'"))),
                                P("ben", Act("speak", "ben", ("against", "second speech"))),
                                P("eli", Act("point-of-order", "eli", ("the last act was not in order", tr)))] + passes("dee", "ben", "eli"))
    while s.phase != "open":
        s = collect_and_resolve(s, [(m, pol(Act("vote", m, ("no" if m != "eli" else "aye",)))) for m in MEMBERS])
    s = collect_and_resolve(s, passes())                                                   # no debate on the amendment -> put
    s = collect_and_resolve(s, [(m, pol(Act("vote", m, ("aye",)))) for m in MEMBERS])
    s = collect_and_resolve(s, passes())
    s = collect_and_resolve(s, [(m, pol(Act("vote", m, ("no" if m == "ada" else "aye",)))) for m in MEMBERS])
    return s


if __name__ == "__main__":
    test_f1_invariance()
    test_f3_relabeling()
    test_independence_of_losing_claims()
    test_no_cloture_by_out_of_order_claim()
    test_point_does_not_redraw_lottery()
    test_point_in_vote_round_voids_poll()
    test_point_and_objection_ordering()
    test_silent_is_not_pass()
    test_f4_deadline_and_vote_leak()
    test_i1_implicit_guard()
    test_ladder()
    test_d1_d2()
    s = specimen()
    print("\nSpecimen record (committee configuration, no seconds):")
    for e in s.record:
        print("  ", e)
    print("\nall checks passed")
