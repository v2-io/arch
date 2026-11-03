"""
test_rounds.py — checks for the round mechanism (run: python3 test_rounds.py).

Each test names the claim in ../contention.md it supports. These are the toy's
checks; they do not verify the RONR-12 model, and the real sweep over
actions.json is owed (see contention.md, "What this spike did not do").
"""
from __future__ import annotations

import itertools
import random
import sys

from rounds import (Act, Policy, State, Frame, RANK, DEBATABLE, admissible, apply, resolve,
                    collect_and_resolve, is_fragile, commute, inadmissible_reason)

MEMBERS = ["ada", "ben", "cyd", "dee", "eli"]


def fresh(needs_second=False) -> State:
    s = State(members=list(MEMBERS), needs_second=needs_second)
    s.present = set(MEMBERS)
    return s


def opened(s: State) -> State:
    """The state as a member sees it when the NEXT round opens (round counter advanced),
    which is what timeliness checks like 'breach just occurred' are relative to."""
    import copy
    s2 = copy.deepcopy(s)
    s2.round_no += 1
    return s2


def pol(*acts, standing=False) -> Policy:
    return Policy(entries=[(None, a) for a in acts], standing=standing)


def gpol(*pairs) -> Policy:
    return Policy(entries=list(pairs))


# ----------------------------------------------------------------------------
# F1: arrival-order invariance
# ----------------------------------------------------------------------------
def random_policy(m: str, s: State, rng: random.Random) -> Policy:
    t = s.top()
    choices = [Act("pass", m, ())]
    if s.phase == "vote" or s.phase in ("ruling", "objection-vote"):
        choices = [Act("vote", m, (rng.choice(["aye", "no", "abstain"]),))]
    elif s.phase == "consent":
        choices = [Act("pass", m, ()), Act("object-consent", m, ())]
    else:
        if t is None:
            choices.append(Act("move", m, ("main", f"main by {m}")))
        else:
            for k in ("amend", "commit", "postpone", "previous-question", "lay-on-table", "adjourn"):
                choices.append(Act("move", m, (k, f"{k} by {m}")))
            choices.append(Act("speak", m, (rng.choice(["for", "against"]), f"{m} speaks")))
            choices.append(Act("second", m, (t.fid,)))
            choices.append(Act("object-consideration", m, ()))
        choices.append(Act("point-of-order", m, (f"point by {m}",)))
        choices.append(Act("demand-division", m, ()))
    k = rng.randint(1, 3)
    return Policy(entries=[(None, a) for a in rng.sample(choices, min(k, len(choices)))])


def strip_late(s: State) -> str:
    """Fingerprint without the supplementary (late) claim-log entries, which are the only
    arrival-dependent thing the design permits (F4)."""
    import copy
    s2 = copy.deepcopy(s)
    s2.claim_log = [e for e in s2.claim_log if not str(e.get("intent", "")).startswith("late")]
    return s2.fingerprint()


def test_f1_invariance(trials=200, seed=7):
    rng = random.Random(seed)
    for trial in range(trials):
        s = fresh(needs_second=rng.random() < 0.5)
        # walk a few rounds to reach assorted states
        for _ in range(rng.randint(0, 6)):
            prof = [(m, random_policy(m, s, rng)) for m in MEMBERS]
            s = collect_and_resolve(s, prof)
        prof = [(m, random_policy(m, s, rng)) for m in MEMBERS]
        fps = set()
        for perm in itertools.islice(itertools.permutations(prof), 0, 120, 7):
            fps.add(collect_and_resolve(s, list(perm)).fingerprint())
        assert len(fps) == 1, f"trial {trial}: {len(fps)} distinct outcomes under permutation"
    print(f"F1  arrival-order invariance: {trials} random states x 18 permutations each — identical records")


def test_f4_deadline():
    """With a presence deadline, the record depends only on the SET of on-time responses;
    late responses are logged as supplementary and nothing else changes."""
    rng = random.Random(3)
    for trial in range(100):
        s = fresh()
        for _ in range(rng.randint(0, 4)):
            s = collect_and_resolve(s, [(m, random_policy(m, s, rng)) for m in MEMBERS])
        prof = [(m, random_policy(m, s, rng)) for m in MEMBERS]
        cut = rng.randint(1, len(prof))
        on_time = prof[:cut]
        late = prof[cut:]
        fps = set()
        for p1 in itertools.permutations(on_time):
            for p2 in itertools.permutations(late):
                fps.add(strip_late(collect_and_resolve(s, list(p1) + list(p2), deadline_cut=cut)))
                if len(fps) > 1:
                    break
        assert len(fps) == 1, f"trial {trial}: on-time set fixed, record varied"
        out = collect_and_resolve(s, prof, deadline_cut=cut)
        for m, _ in late:
            assert any(e["member"] == m and str(e["intent"]).startswith("late") for e in out.claim_log)
            assert any(e["member"] == m and "silent" in str(e["intent"]) for e in out.claim_log)
    print("F4  presence deadline: record depends only on the on-time set; late responses logged, not dropped")


# ----------------------------------------------------------------------------
# D1: the fragile set is derived and matches RONR's no-floor-needed list
# ----------------------------------------------------------------------------
def test_d1_fragile_set():
    s = fresh(needs_second=True)
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] +
                               [(m, pol(Act("pass", m, ()))) for m in MEMBERS[1:]])
    t = s.top()
    assert t is not None and not t.seconded_by
    probes = {
        "second":               Act("second", "ben", (t.fid,)),
        "point-of-order":       Act("point-of-order", "ben", ("the motion was not in order",)),
        "object-consideration": Act("object-consideration", "ben", ()),
        "move amend":           Act("move", "ben", ("amend", "...")),
        "move previous-question": Act("move", "ben", ("previous-question", "...")),
        "speak":                Act("speak", "ben", ("against", "...")),
    }
    # before seconding, only the second is admissible among floor-ish acts; it is fragile (chair proceeds -> dies)
    assert admissible(probes["second"], opened(s))
    assert is_fragile(probes["second"], opened(s)), "a second must be admitted ahead of the floor"
    # now a seconded, undebated main motion: the classic window
    s2 = collect_and_resolve(s, [("ben", pol(Act("second", "ben", (t.fid,))))] +
                                [(m, pol(Act("pass", m, ()))) for m in MEMBERS if m != "ben"])
    derived = {name: is_fragile(a, opened(s2)) for name, a in probes.items() if admissible(a, opened(s2))}
    expected = {"point-of-order": True, "object-consideration": True,
                "move amend": False, "move previous-question": False, "speak": False}
    for k, v in expected.items():
        assert k in derived and derived[k] == v, f"{k}: derived={derived.get(k)} expected={v}"
    # after a vote is announced: the division demand is fragile (business intervening closes it)
    s3 = s2
    for _ in range(6):
        s3 = collect_and_resolve(s3, [(m, pol(Act("vote", m, ("aye",)) if s3.phase == "vote" else Act("pass", m, ()))) for m in MEMBERS])
        if s3.last_announced is not None and not s3.business_since_announce:
            break
    assert s3.last_announced is not None and not s3.business_since_announce
    assert is_fragile(Act("demand-division", "ben", ()), opened(s3))
    print("D1  fragile set derived by simulation = {second, point-of-order, objection-to-consideration, division}; "
          "{move, speak} not fragile — matches RONR's no-floor-needed / interrupt list for these acts")


# ----------------------------------------------------------------------------
# D2: commutation decides batching
# ----------------------------------------------------------------------------
def test_d2_commutation():
    s = fresh(needs_second=True)
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] +
                               [(m, pol(Act("pass", m, ()))) for m in MEMBERS[1:]])
    fid = s.top().fid
    assert commute(Act("second", "ben", (fid,)), Act("second", "cyd", (fid,)), opened(s)), "two seconds commute"
    s2 = fresh()
    s2 = collect_and_resolve(s2, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] +
                                 [(m, pol(Act("pass", m, ()))) for m in MEMBERS[1:]])
    s2 = opened(s2)
    assert not commute(Act("move", "ben", ("amend", "a")), Act("move", "cyd", ("amend", "b")), s2), \
        "two primary amendments do not commute [12:13]"
    assert not commute(Act("move", "ben", ("amend", "a")), Act("move", "cyd", ("previous-question", "")), s2), \
        "amend and previous-question do not commute [16:2]"
    assert not commute(Act("speak", "ben", ("for", "x")), Act("speak", "cyd", ("for", "y")), s2), \
        "two speeches do not commute (the record is ordered)"
    print("D2  commutation by simulation: seconds commute; amendments, PQ-vs-amend, and speeches do not")


# ----------------------------------------------------------------------------
# I1: one exclusive act per round, and guards keep a member's act its own
# ----------------------------------------------------------------------------
def test_i1_informed_acts():
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] +
                               [(m, pol(Act("pass", m, ()))) for m in MEMBERS[1:]])
    main_fid = s.top().fid
    # Same round: ben wants to amend; cyd wants to postpone — but ONLY if nothing but the main is pending;
    # otherwise cyd would rather speak against whatever was just moved.
    only_main = lambda st: len(st.stack) == 1
    prof = [
        ("ben", pol(Act("move", "ben", ("amend", "strike 'main', insert 'tag'")))),
        ("cyd", gpol((only_main, Act("move", "cyd", ("postpone", "until the Fable assessment lands"))),
                     (lambda st: len(st.stack) > 1 and st.top().kind == "amend",
                      Act("speak", "cyd", ("against", "the amendment measures the wrong question"))))),
        ("ada", pol(Act("pass", "ada", ()))), ("dee", pol(Act("pass", "dee", ()))), ("eli", pol(Act("pass", "eli", ()))),
    ]
    s1 = collect_and_resolve(s, prof)
    # exactly one exclusive act was admitted
    excl = [e for e in s1.record if e.get("round") == s1.round_no and e.get("act") in ("move", "speak")]
    assert len(excl) == 1, excl
    winner = excl[0]["member"]
    # the loser kept a claim and is re-asked against the new state; its guarded policy now routes differently
    if winner == "ben":
        assert "cyd" in s1.claims, "cyd's claim age is kept for the next round"
        s2 = collect_and_resolve(s1, prof)   # same standing policies re-evaluated against the new state
        excl2 = [e for e in s2.record if e.get("round") == s2.round_no and e.get("act") in ("move", "speak")]
        assert excl2 and excl2[0]["member"] == "cyd" and excl2[0]["act"] == "speak", excl2
        print("I1  one exclusive act per round: ben's amendment admitted; cyd's guarded policy, re-evaluated "
              "against the new state, spoke against it instead of postponing — cyd was never bound to a blind act")
    else:
        assert "ben" in s1.claims
        print("I1  one exclusive act per round: cyd's postponement admitted; ben keeps a claim (rotation chose cyd)")


# ----------------------------------------------------------------------------
# F3: ties are broken by claim age then rotation, never by arrival
# ----------------------------------------------------------------------------
def test_f3_claim_age():
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] +
                               [(m, pol(Act("pass", m, ()))) for m in MEMBERS[1:]])
    # all five want the floor to speak, same side; maker (ada) has preference first [42:9(1)]
    want = lambda m: pol(Act("speak", m, ("for", f"{m} speaks")))
    order = []
    for r in range(5):
        s = collect_and_resolve(s, [(m, want(m)) for m in MEMBERS])
        order.append([e for e in s.record if e.get("round") == s.round_no and e.get("act") == "speak"][0]["member"])
    assert order[0] == "ada", order
    assert sorted(order) == sorted(MEMBERS), order   # everyone spoke once before anyone spoke twice [42:9(2)]
    # alternation: with sides declared, the floor alternates when preference-equal
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] +
                               [(m, pol(Act("pass", m, ()))) for m in MEMBERS[1:]])
    s = collect_and_resolve(s, [("ada", pol(Act("speak", "ada", ("for", "..."))))] +
                               [(m, pol(Act("pass", m, ()))) for m in MEMBERS[1:]])
    side = {"ben": "for", "cyd": "against", "dee": "for", "eli": "against"}
    s = collect_and_resolve(s, [("ada", pol(Act("pass", "ada", ())))] +
                               [(m, pol(Act("speak", m, (side[m], "...")))) for m in MEMBERS[1:]])
    last = [e for e in s.record if e.get("round") == s.round_no and e.get("act") == "speak"][0]
    assert last["args"][0] == "against", last
    print("F3  preference ladder then claim age then rotation: maker first, everyone once before anyone twice, "
          "alternation by declared side; arrival never consulted")


# ----------------------------------------------------------------------------
# Division is vacuous for agents
# ----------------------------------------------------------------------------
def test_division_vacuous():
    s = fresh()
    s = collect_and_resolve(s, [("ada", pol(Act("move", "ada", ("main", "Keep main"))))] +
                               [(m, pol(Act("pass", m, ()))) for m in MEMBERS[1:]])
    s = collect_and_resolve(s, [(m, pol(Act("pass", m, ()))) for m in MEMBERS])        # nobody debates -> put
    assert s.phase == "vote"
    ballots = {"ada": "aye", "ben": "aye", "cyd": "no", "dee": "aye", "eli": "no"}
    s = collect_and_resolve(s, [(m, pol(Act("vote", m, (ballots[m],)))) for m in MEMBERS])
    ann = [e for e in s.record if e.get("chair") == "announced"][-1]
    assert ann["tally"] == ballots
    s = collect_and_resolve(s, [("cyd", pol(Act("demand-division", "cyd", ())))] +
                               [(m, pol(Act("pass", m, ()))) for m in MEMBERS if m != "cyd"])
    ev = [e for e in s.record if e.get("act") == "demand-division"][-1]
    assert "vacuous" in ev["chair"]
    print("Div division of the assembly: every vote is already an exact, attributed, sealed count — a retake "
          "reproduces the same tally; the demand is vacuous (its purpose, not its form, is what dissolved)")


# ----------------------------------------------------------------------------
# A scripted specimen for the write-up
# ----------------------------------------------------------------------------
def specimen():
    """Specimen 1 from ../../specimens-2026-11-03.md as a council might have run it (committee configuration)."""
    s = fresh()
    P = lambda m, *acts: (m, pol(*acts))
    rounds = [
        [P("ada", Act("move", "ada", ("main", "Reset canon to the pre-sprint tag")))] +
        [P(m, Act("pass", m, ())) for m in ["ben", "cyd", "dee", "eli"]],
        # round: maker speaks first; ben and cyd also claim
        [P("ada", Act("speak", "ada", ("for", "findability: the eight checks added nothing"))),
         P("ben", Act("speak", "ben", ("against", "I re-derived the mathematics; the checks are real"))),
         P("cyd", Act("speak", "cyd", ("against", "re-integration cost of a reset is high"))),
         P("dee", Act("pass", "dee", ())), P("eli", Act("pass", "eli", ()))],
        [P("ben", Act("speak", "ben", ("against", "I re-derived the mathematics; the checks are real"))),
         P("cyd", Act("speak", "cyd", ("against", "re-integration cost of a reset is high"))),
         P("ada", Act("pass", "ada", ())), P("dee", Act("pass", "dee", ())), P("eli", Act("pass", "eli", ()))],
        [P("cyd", Act("speak", "cyd", ("against", "re-integration cost of a reset is high"))),
         P("dee", Act("move", "dee", ("amend", "insert 'after an independent re-derivation of the eight checks'"))),
         P("ada", Act("pass", "ada", ())), P("ben", Act("pass", "ben", ())), P("eli", Act("pass", "eli", ()))],
        [P("dee", Act("move", "dee", ("amend", "insert 'after an independent re-derivation of the eight checks'"))),
         P("eli", Act("point-of-order", "eli", ("cyd's second speech came while dee was still seeking the floor",))),
         P("ada", Act("pass", "ada", ())), P("ben", Act("pass", "ben", ())), P("cyd", Act("pass", "cyd", ()))],
    ]
    for prof in rounds:
        s = collect_and_resolve(s, prof)
    # the assembly rules on eli's point (toy: by vote), then dee's amendment is re-asked
    while s.phase in ("ruling", "vote", "objection-vote"):
        s = collect_and_resolve(s, [(m, pol(Act("vote", m, ("no" if m != "eli" else "aye",)))) for m in MEMBERS])
    s = collect_and_resolve(s, [P("dee", Act("move", "dee", ("amend", "insert 'after an independent re-derivation of the eight checks'")))] +
                               [P(m, Act("pass", m, ())) for m in MEMBERS if m != "dee"])
    s = collect_and_resolve(s, [P(m, Act("pass", m, ())) for m in MEMBERS])                     # no debate on the amendment -> put
    s = collect_and_resolve(s, [(m, pol(Act("vote", m, ("aye",)))) for m in MEMBERS])            # amendment adopted
    s = collect_and_resolve(s, [P(m, Act("pass", m, ())) for m in MEMBERS])                     # main as amended -> put
    s = collect_and_resolve(s, [(m, pol(Act("vote", m, ("no" if m in ("ada",) else "aye",)))) for m in MEMBERS])
    return s


if __name__ == "__main__":
    test_f1_invariance()
    test_f4_deadline()
    test_d1_fragile_set()
    test_d2_commutation()
    test_i1_informed_acts()
    test_f3_claim_age()
    test_division_vacuous()
    s = specimen()
    print("\nSpecimen record (committee configuration, no seconds):")
    for e in s.record:
        print("  ", e)
    print("\nall checks passed")
