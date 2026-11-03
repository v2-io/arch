"""
rounds.py — a small executable model of the round mechanism for agentic councils.

Revision 2 (after de-novo-feedback-1.md). This is NOT the RONR-12 model's engine
(stage 5, pending). It is a toy with enough parliamentary structure to exercise
the contention layer: a pending stack with ranks, a floor assigned once per
round, seconds, debate with maker/alternation preference, breach-specific points
of order, objection to consideration, sealed votes.

Properties the code is built to have (checked in test_rounds.py):

  F1  the record of a round is a function of the SET of (member, policy)
      responses received before the round closed, never of their order;
  F3  the record is equivariant under relabeling members, except through two
      declared devices: least-recently-given-the-floor, then a recorded lottery
      keyed by roster slot (the "numbered microphones" of 42:16);
  F4  wall-clock enters only through the presence deadline; a vote round closes
      at its deadline and a late ballot is never counted and never seen;
  I1  a floor act fires only if the state it was formed against is still the
      state in the respects it depends on (snapshot_key), so no act runs blind;
  P   a point of order names the transition it challenges, is timely only in
      the first round after it, and cannot be re-raised against the same
      transition — so halting the floor has a bounded cost (1.2 in the feedback).

What the code deliberately no longer has: persistent claim age (it was 42:16's
line, not 42:6, and it rewarded always-claiming), early close (a second clock
door), and the identity "interrupt class = fragile set" (fragility is one
sufficient reason for admission ahead of the floor; urgency, judgment-routed in
the model, is the other).
"""
from __future__ import annotations

import copy
import hashlib
import json
from dataclasses import dataclass, field, asdict
from typing import Optional

RANK = {"main": 0, "amend": 2, "commit": 3, "postpone": 4, "previous-question": 6,
        "lay-on-table": 7, "recess": 10, "adjourn": 11}
DEBATABLE = {"main": True, "amend": True, "commit": True, "postpone": True,
             "previous-question": False, "lay-on-table": False, "recess": False, "adjourn": False}


@dataclass(frozen=True)
class Act:
    kind: str          # move | second | speak | point-of-order | object-consideration | vote | pass
    member: str
    args: tuple = ()

    def key(self):
        return (self.kind, self.member, self.args)


FLOOR_KINDS = {"move", "speak"}
# canonical order among non-floor acts admitted in one round: by kind only (never by name);
# within a kind the acts are set-valued (points, seconds, objections) so order cannot matter
KIND_ORDER = {"point-of-order": 0, "object-consideration": 1, "second": 2}


@dataclass
class Policy:
    entries: list                 # of (guard | None, Act); guard: callable(state) -> bool
    standing: bool = False        # declared; the toy always asks everyone (see contention.md F2)


@dataclass
class Frame:
    fid: int
    kind: str
    maker: str
    text: str
    seconded_by: list = field(default_factory=list)
    stated: bool = False
    debated: bool = False
    made_round: int = 0
    stated_round: int = -1


@dataclass
class State:
    members: list                                   # roster order = slot order (declared device)
    needs_second: bool = True
    seed: str = "council-seed"                      # the recorded lottery seed (declared device)
    stack: list = field(default_factory=list)
    next_fid: int = 1
    phase: str = "open"                             # open | vote | ruling | objection-vote
    vote: Optional[dict] = None
    points: list = field(default_factory=list)      # pending: [{"by","text","target"}], sorted by target
    ruled_targets: list = field(default_factory=list)   # transitions already challenged (re-raise barred)
    speeches: dict = field(default_factory=dict)    # (member, fid) -> count
    last_side: dict = field(default_factory=dict)   # question fid -> side of the last speech on it
    last_floor: dict = field(default_factory=dict)  # member -> floor-grant index when last given the floor
    floor_grants: int = 0                           # 42:6's reset unit: the number of floor grants so far
    round_no: int = 0
    last_announced: Optional[dict] = None
    business_since_announce: bool = True
    record: list = field(default_factory=list)
    claim_log: list = field(default_factory=list)
    present: set = field(default_factory=set)
    claims_this_round: set = field(default_factory=set)   # members with a live floor claim in the open round

    def top(self) -> Optional[Frame]:
        return self.stack[-1] if self.stack else None

    def snapshot_key(self):
        """The implicit guard of an unguarded floor act: what it must find unchanged."""
        t = self.top()
        return (t.fid if t else None, t.stated if t else None, self.phase, len(self.points))

    def transitions_in_round(self, rnd: int):
        """Record indices of the challengeable transitions made in round rnd: floor acts, the chair's
        stating and putting, and the announcement of a result [45:9]."""
        return [i for i, e in enumerate(self.record)
                if e.get("round") == rnd and (e.get("act") in FLOOR_KINDS or e.get("chair") in ("stated", "put", "announced"))]

    def fingerprint(self) -> str:
        d = asdict(self)
        d["present"] = sorted(self.present)
        d["claims_this_round"] = sorted(self.claims_this_round)
        d["speeches"] = sorted((f"{k[0]}|{k[1]}", v) for k, v in self.speeches.items())
        d["last_floor"] = sorted(self.last_floor.items())
        d["last_side"] = sorted(self.last_side.items())
        return hashlib.sha256(json.dumps(d, sort_keys=True, default=str).encode()).hexdigest()


# ----------------------------------------------------------------------------
# Admissibility (the toy's available_actions)
# ----------------------------------------------------------------------------
def inadmissible_reason(a: Act, s: State) -> Optional[str]:
    t = s.top()
    if a.member not in s.present:
        return "not present"
    if s.phase in ("vote", "ruling", "objection-vote"):
        if a.kind in ("vote", "pass"):
            return None
        if a.kind == "point-of-order" and s.phase == "vote":
            text, target = a.args
            if target not in s.transitions_in_round(s.round_no - 1):
                return "the challenged transition is not one of the previous round's [23:5]"
            if (target, a.member) in s.ruled_targets or any(p["target"] == target and p["by"] == a.member for p in s.points):
                return "this member has already challenged that transition"
            return None   # a point against the put or the stating, before any member has voted [4:34, 45:6]
        return "voting in progress; no interruption [45:6]"
    if a.kind == "vote":
        return "no vote in progress"
    if a.kind in ("pass", "chair-proceed"):
        return None
    if a.kind == "point-of-order":
        text, target = a.args
        if target not in s.transitions_in_round(s.round_no - 1):
            return "the challenged transition is not one of the previous round's [23:5]"
        if (target, a.member) in s.ruled_targets or any(p["target"] == target and p["by"] == a.member for p in s.points):
            return "this member has already challenged that transition"
        return None
    if a.kind == "second":
        (fid,) = a.args
        fr = next((f for f in s.stack if f.fid == fid), None)
        if fr is None:
            return "no such motion"
        if not s.needs_second:
            return "seconds not required in this configuration [49:21(2)]"
        if fr.debated:
            return "debate begun; second immaterial [4:13]"
        if fr.maker == a.member or a.member in fr.seconded_by:
            return "maker cannot second / already seconded by this member"
        return None
    if a.kind == "object-consideration":
        if t is None or t.kind != "main" or t.debated or not t.stated:
            return "no stated, undebated original main motion pending [26:2(1)]"
        return None
    if a.kind == "speak":
        if t is None:
            return "nothing pending"
        if not DEBATABLE[t.kind]:
            return "undebatable"
        if not t.stated:
            return "not yet stated"
        n = s.speeches.get((a.member, t.fid), 0)
        if n >= 2:
            return "two speeches exhausted [43:12]"
        if n == 1 and any(s.speeches.get((m, t.fid), 0) == 0 for m in s.claims_this_round if m != a.member):
            return "second speech barred while another seeking the floor has not spoken [43:13]"
        return None
    if a.kind == "move":
        kind = a.args[0]
        if kind not in RANK:
            return "unknown motion"
        if t is None:
            return None if kind == "main" else "secondary motion with nothing pending"
        if not t.stated:
            return "pending motion not yet stated"
        if kind == "main":
            return "main motion while a question is pending [5:4]"
        if RANK[kind] <= RANK[t.kind]:
            return f"{kind} does not take precedence over pending {t.kind} [5:8-9]"
        if kind == "amend" and any(f.kind == "amend" for f in s.stack):
            return "a primary amendment is already pending [12:13]"
        if any(f.kind == "previous-question" for f in s.stack) and kind not in ("lay-on-table", "recess", "adjourn"):
            return "previous question pending precludes [16:2]"
        return None
    return f"unknown act {a.kind}"


def admissible(a: Act, s: State) -> bool:
    return inadmissible_reason(a, s) is None


# ----------------------------------------------------------------------------
# Applying acts, and the chair's mechanical duties
# ----------------------------------------------------------------------------
def apply(a: Act, s: State, note: str = "") -> State:
    s = copy.deepcopy(s)
    ev = {"round": s.round_no, "member": a.member, "act": a.kind, "args": list(a.args)}
    if note:
        ev["note"] = note
    if a.kind == "pass":
        return s
    if a.kind == "chair-proceed":
        t = s.top()
        if t is not None and s.needs_second and not t.seconded_by:
            s.stack.pop()
            s.record.append({"round": s.round_no, "chair": "no second; motion dies [4:10]", "on": t.fid})
        elif t is not None and t.stated and not DEBATABLE[t.kind] and s.phase == "open":
            s.phase = "vote"
            s.vote = {"on": t.fid, "ballots": {}, "threshold": "majority"}
            s.record.append({"round": s.round_no, "chair": "put", "on": t.fid})
        elif s.last_announced is not None and not s.business_since_announce:
            s.business_since_announce = True
        return s
    s.business_since_announce = True
    if a.kind == "move":
        kind, text = a.args[0], a.args[1]
        fr = Frame(s.next_fid, kind, a.member, text, made_round=s.round_no)
        s.next_fid += 1
        s.stack.append(fr)
        if not s.needs_second:
            fr.stated, fr.stated_round = True, s.round_no
            ev["chair"] = "stated"
        s.record.append(ev)
        s.floor_grants += 1
        s.last_floor[a.member] = s.floor_grants
        return s
    if a.kind == "second":
        (fid,) = a.args
        fr = next(f for f in s.stack if f.fid == fid)
        fr.seconded_by = sorted(set(fr.seconded_by) | {a.member})
        s.record.append(ev)
        if not fr.stated:
            fr.stated, fr.stated_round = True, s.round_no
            s.record.append({"round": s.round_no, "chair": "stated", "on": fid})   # the chair's act, its own entry
        return s
    if a.kind == "speak":
        side = a.args[0]
        t = s.top()
        t.debated = True
        s.speeches[(a.member, t.fid)] = s.speeches.get((a.member, t.fid), 0) + 1
        s.last_side[t.fid] = side
        s.record.append(ev)
        s.floor_grants += 1
        s.last_floor[a.member] = s.floor_grants
        return s
    if a.kind == "point-of-order":
        text, target = a.args
        s.points = sorted(s.points + [{"by": a.member, "text": text, "target": target}],
                          key=lambda p: (p["target"], p["text"]))   # ordered by content, never by slot
        s.record.append(ev)
        return s
    if a.kind == "object-consideration":
        s.phase = "objection-vote"
        s.vote = {"on": "objection-to-consideration", "ballots": {}, "threshold": "two-thirds-neg"}
        s.record.append(ev)
        return s
    if a.kind == "vote":
        (choice,) = a.args
        s.vote["ballots"][a.member] = choice     # sealed: never in the claim log, announced in-round
        return s
    raise ValueError(a.kind)


def chair_duties(s: State) -> State:
    s = copy.deepcopy(s)
    if s.phase == "open" and s.points:
        p = s.points[0]
        s.phase = "ruling"
        s.vote = {"on": f"point-of-order:{p['text']}", "ballots": {}, "threshold": "majority", "target": p["target"], "by": p["by"]}
        s.record.append({"round": s.round_no, "chair": "point submitted to the assembly [23:18-21]"})
    return s


def announce(s: State) -> State:
    """A vote round closes when the round closes: every present member has voted or is recorded as
    not voting. Thresholds count votes cast."""
    s = copy.deepcopy(s)
    v = s.vote
    not_voting = sorted(m for m in s.present if m not in v["ballots"])
    if not v["ballots"]:
        s.record.append({"round": s.round_no, "chair": "poll void: no ballots cast; the question is put again", "on": v["on"]})
        return s                                  # phase and vote unchanged: the poll repeats next round
    yes = sum(1 for c in v["ballots"].values() if c == "aye")
    no = sum(1 for c in v["ballots"].values() if c == "no")
    if v["threshold"] == "two-thirds-neg":
        adopted = no >= 2 * yes and no > 0
    else:
        adopted = yes > no
    s.record.append({"round": s.round_no, "chair": "announced", "on": v["on"],
                     "tally": dict(sorted(v["ballots"].items())), "not_voting": not_voting,
                     "result": "adopted" if adopted else "rejected"})
    s.last_announced = {"on": v["on"], "result": adopted}
    s.business_since_announce = False
    if s.phase == "ruling":
        s.ruled_targets = sorted(set(map(tuple, s.ruled_targets)) | {(v["target"], v["by"])})
        s.points = [p for p in s.points if not (p["target"] == v["target"] and p["by"] == v["by"])]
        s.phase = "open"
        s.vote = None
        if s.points:
            return chair_duties(s)             # the next pending point is submitted at once
    elif s.phase == "objection-vote":
        if adopted:
            s.stack.pop()
        s.phase = "open"
    elif s.phase == "vote":
        t = s.stack.pop()
        if adopted and t.kind == "previous-question":
            s.record.append({"round": s.round_no, "chair": "previous question ordered; question recurs without debate"})
        s.phase = "open"
    s.vote = None
    return s


def put_question_if_exhausted(s: State, asked_for_debate: bool, silent: set = frozenset()) -> State:
    """Exhaustion = every present member was asked and none claimed debate. A member that did not
    answer was not asked in the relevant sense: silence extends the question, it never closes it."""
    s = copy.deepcopy(s)
    t = s.top()
    if t is None or s.phase != "open" or s.points or not t.stated:
        return s
    if s.round_no <= t.stated_round or silent:
        return s
    if not DEBATABLE[t.kind] or not asked_for_debate:
        s.phase = "vote"
        s.vote = {"on": t.fid, "ballots": {}, "threshold": "majority"}
        s.record.append({"round": s.round_no, "chair": "put", "on": t.fid})
    return s


# ----------------------------------------------------------------------------
# Derived classifications — a diff tool, not an oracle (see contention.md §5.1)
# ----------------------------------------------------------------------------
def ordinary_course(s: State, exclude: str) -> list:
    """Probes for fragility: another member (one who can) speaking when the pending question is
    debatable; the chair proceeding to the next duty that no claimant blocks. A declared choice:
    the chair does not put a debatable question while anyone seeks the floor [42:22, 4:34], so the
    put is not part of the ordinary course FOR a member who is claiming."""
    t = s.top()
    outs = [Act("chair-proceed", exclude, ())]
    if t is not None and DEBATABLE[t.kind] and t.stated and (not s.needs_second or t.seconded_by):
        for m in s.members:
            if m != exclude and m in s.present and s.speeches.get((m, t.fid), 0) < 2:
                outs.append(Act("speak", m, ("for", "...")))
                break
    return outs


def is_fragile(a: Act, s: State) -> bool:
    """Fragile: admissible now, inadmissible after the ordinary course proceeds without it —
    another member's speech, the chair's next duty, or simply the round advancing with
    nothing done (a one-round window closes that way)."""
    if not admissible(a, s):
        return False
    for b in ordinary_course(s, a.member):
        if admissible(b, s) and not admissible(a, apply(b, s)):
            return True
    s_adv = copy.deepcopy(s)
    s_adv.round_no += 1
    return not admissible(a, s_adv)


def commute(a: Act, b: Act, s: State) -> bool:
    if a.kind in FLOOR_KINDS or b.kind in FLOOR_KINDS:
        return False
    if not (admissible(a, s) and admissible(b, s)):
        return False
    sa, sb = apply(a, s), apply(b, s)
    if not (admissible(b, sa) and admissible(a, sb)):
        return False
    n0 = len(s.record)
    def norm(x):
        y = copy.deepcopy(x)
        y.record = y.record[:n0] + sorted(y.record[n0:], key=lambda e: json.dumps(e, sort_keys=True))
        return y.fingerprint()
    return norm(apply(b, sa)) == norm(apply(a, sb))


# ----------------------------------------------------------------------------
# Preference in recognition
# ----------------------------------------------------------------------------
def lottery(s: State, member: str) -> str:
    """The recorded lottery: keyed by the committed seed, the floor-grant count (42:6's reset unit —
    a point of order does not yield the floor [42:20], so it does not redraw), and the roster slot."""
    slot = s.members.index(member)
    return hashlib.sha256(f"{s.seed}|{s.floor_grants}|{slot}".encode()).hexdigest()


def claim_key(a: Act, s: State):
    """ONE key per claim, depending on the claim and the state only, never on the other claims
    (independence of losing claims). Rungs: the maker's first speech [42:9(1)]; least recently given
    the floor; the lottery. Alternation [42:9(3)] is NOT a rung: a declared side is cheap talk, so the
    default records the side and leaves alternation to a charter that routes side to a judgment."""
    t = s.top()
    maker_pref = 0 if (a.kind == "speak" and t is not None and t.maker == a.member
                       and s.speeches.get((a.member, t.fid), 0) == 0) else 1
    return (maker_pref, s.last_floor.get(a.member, -1), lottery(s, a.member))


def choose_floor(cands: list, s: State) -> Optional[Act]:
    return min(cands, key=lambda a: claim_key(a, s)) if cands else None


# ----------------------------------------------------------------------------
# The round
# ----------------------------------------------------------------------------
def resolve(s: State, profile: dict, silent: set = frozenset()) -> State:
    """Pure in (s, profile, silent). Every policy is evaluated against the round-open state S0, which
    guards see WITHOUT this round's claims (so no guard can read another member's sealed response)."""
    s = copy.deepcopy(s)
    s.round_no += 1
    rnd = s.round_no
    for m in sorted(silent, key=s.members.index):
        s.claim_log.append({"round": rnd, "member": m, "intent": "no response by deadline (silent this round)"})

    # 0. who is seeking the floor: members with an ADMISSIBLE floor entry, judged before the 43:13 bar
    view = copy.deepcopy(s)
    view.claims_this_round = set()
    seeking = set()
    for m, pol in profile.items():
        for guard, act in pol.entries:
            if act.kind == "speak" and (guard(view) if guard else True) and admissible(act, view):
                seeking.add(m)              # 43:13 is about members who desire to SPEAK; a mover's claim is not one
                break
    s0 = copy.deepcopy(s)
    s0.claims_this_round = seeking
    s.claims_this_round = seeking

    # 1. live acts, all against S0; guards against the claim-free view
    nonfloor, floor = [], []
    asked_for_debate = False
    for m in profile:
        pol = profile[m]
        took_floor = took_ballot = False
        for guard, act in pol.entries:
            g_ok = guard(view) if guard is not None else True
            reason = inadmissible_reason(act, s0) if g_ok else None
            adm = (reason is None) if g_ok else None
            entry = {"round": rnd, "member": m, "intent": act.key() if act.kind != "vote" else ("vote", m, "(sealed)"),
                     "guard_held": g_ok, "admissible": adm}
            s.claim_log.append(entry)
            if g_ok and act.kind == "speak" and reason is not None and reason.startswith("second speech barred"):
                asked_for_debate = True        # 43:13 defers the speech; it does not close debate
            if not g_ok or not adm:
                continue
            if act.kind == "vote":
                if took_ballot:
                    continue                   # one ballot per member: the first (highest-priority) entry
                took_ballot = True
                nonfloor.append(act)
            elif act.kind in FLOOR_KINDS:
                if took_floor:
                    continue
                took_floor = True
                asked_for_debate |= act.kind == "speak"
                floor.append(act)
            else:
                nonfloor.append(act)
    s.claim_log.sort(key=lambda e: (e["round"], s.members.index(e["member"]) if e["member"] in s.members else -1, str(e["intent"])))

    # 2. a vote round is the poll — unless a point against the put/stating lands, which voids the poll
    if s.phase in ("vote", "ruling", "objection-vote"):
        points = [a for a in nonfloor if a.kind == "point-of-order"]
        if points:
            for a in sorted(points, key=lambda a: (a.args, s.members.index(a.member))):
                s = apply(a, s, note="point against the put/stating; the poll is void until ruled [4:34]")
            s.record.append({"round": rnd, "chair": "poll void: ballots discarded pending the point", "on": s.vote["on"]})
            s.vote = None
            s.phase = "open"
            return chair_duties(s)
        for a in nonfloor:
            if a.kind == "vote":
                s = apply(a, s)
        return announce(s)

    # 3. non-floor acts admissible at S0. Points first; if any point landed, every other non-floor act
    #    is deferred to the round after the ruling (the point must be disposed of before anything else
    #    [23:2(1)]); their windows survive because a ruling round opens no debate and no vote on them.
    snap0 = s0.snapshot_key()
    points = sorted((a for a in nonfloor if a.kind == "point-of-order"), key=lambda a: (a.args, s.members.index(a.member)))
    others = sorted((a for a in nonfloor if a.kind not in ("point-of-order", "vote")),
                    key=lambda a: (KIND_ORDER.get(a.kind, 9), a.args, s.members.index(a.member)))
    for a in points:
        s = apply(a, s, note="admitted ahead of the floor" + (" (fragile)" if is_fragile(a, s0) else ""))
    if points:
        for a in others:
            s.claim_log.append({"round": rnd, "member": a.member, "intent": a.key(), "deferred": "a point of order is pending"})
        return chair_duties(s)
    for a in others:
        s = apply(a, s, note="admitted ahead of the floor" + (" (fragile)" if is_fragile(a, s0) else ""))

    # 4. one floor act, only if its implicit guard still holds (I1)
    cands = [a for a in floor if admissible(a, s)]
    if s.snapshot_key() != snap0:
        cands = []
        s.record.append({"round": rnd, "chair": "floor claims deferred: state changed by acts admitted this round"})
    w = choose_floor(cands, s)
    if w is not None:
        s = apply(w, s, note="floor: " + ("debater" if w.kind == "speak" else "mover") +
                  f" lastfloor={s.last_floor.get(w.member, -1)}")

    # 5. the chair's duties — never on the strength of silence
    t = s.top()
    if t is not None and s.phase == "open" and s.needs_second and not t.seconded_by and rnd > t.made_round and not silent:
        return apply(Act("chair-proceed", s.members[0], ()), s)
    return put_question_if_exhausted(s, asked_for_debate, silent)


def collect_and_resolve(s: State, arrivals: list, deadline_cut: Optional[int] = None) -> State:
    """arrivals: (member, Policy) in arrival order. Responses at index >= deadline_cut are late:
    logged as supplementary, never applied (a late ballot is not counted and not seen)."""
    on_time = arrivals if deadline_cut is None else arrivals[:deadline_cut]
    late = [] if deadline_cut is None else arrivals[deadline_cut:]
    profile, refused = {}, []
    for m, p in on_time:
        if m in profile:
            refused.append(m)                 # the first submission is final; a replacement is refused
        else:
            profile[m] = p
    silent = set(s.present) - set(profile)
    s2 = resolve(s, profile, silent)
    for m in sorted(refused, key=s.members.index):
        s2.claim_log.append({"round": s2.round_no, "member": m, "intent": "duplicate submission refused (first is final)"})
    for m, p in sorted(late, key=lambda mp: s.members.index(mp[0])):
        s2.claim_log.append({"round": s2.round_no, "member": m, "intent": "late (supplementary): " +
                             "; ".join(("vote (sealed)" if a.kind == "vote" else str(a.key())) for _, a in p.entries)})
    return s2
