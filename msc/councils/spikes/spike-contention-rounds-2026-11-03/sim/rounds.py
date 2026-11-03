"""
rounds.py — a small executable model of the round mechanism for agentic councils.

This is NOT the RONR-12 model's engine (that is stage 5, pending). It is a toy
state with just enough parliamentary structure to exercise the contention layer:
a pending stack with ranks, a floor that is assigned once per round, seconds,
debate with maker/not-yet-spoken/alternation preference, points of order,
objection to consideration, unanimous consent, sealed votes, and a division
demand kept in deliberately to show it is vacuous.

What it demonstrates (see ../contention.md for the argument and test_rounds.py
for the checks):

  F1  the record of a round is a function of the SET of (member, policy)
      responses, never of their arrival order;
  F2  every present member is asked at every state;
  F3  ties among preference-equal claimants are broken by a neutral rule
      (claim age in rounds, then a rotation pointer), never by arrival;
  F4  wall-clock enters only through the presence deadline, and a late
      response is logged as supplementary, never silently dropped;
  D1  the class of acts that must be admitted ahead of the floor ("fragile":
      their admissibility is destroyed by an ordinary floor act) is DERIVED
      by simulation, not curated — and it coincides with RONR's own
      no-floor-needed list;
  D2  non-exclusive acts are batched in one round iff they commute, checked
      by simulation (apply in both orders, compare), not by a hand list;
  I1  at most one exclusive act is admitted per round, so that every act is
      made against a state that includes every previously admitted act
      (the "informed act" property RONR's serial floor gives for free).

Everything here is deterministic given the profile; there is no clock.
"""
from __future__ import annotations

import copy
import hashlib
import json
from dataclasses import dataclass, field, asdict
from typing import Callable, Optional

# ----------------------------------------------------------------------------
# Motion kinds: a small ranked chain (low -> high), per precedence.json rank_order
# ----------------------------------------------------------------------------
RANK = {
    "main": 0,
    "amend": 2,
    "commit": 3,
    "postpone": 4,
    "previous-question": 6,
    "lay-on-table": 7,
    "recess": 10,
    "adjourn": 11,
}
DEBATABLE = {"main": True, "amend": True, "commit": True, "postpone": True,
             "previous-question": False, "lay-on-table": False, "recess": False, "adjourn": False}
NEEDS_SECOND_DEFAULT = True  # ordinary-society; the committee configuration sets False


# ----------------------------------------------------------------------------
# Acts
# ----------------------------------------------------------------------------
@dataclass(frozen=True)
class Act:
    kind: str                    # move | second | speak | point-of-order | object-consideration |
                                 # object-consent | demand-division | vote | pass
    member: str
    args: tuple = ()             # kind-specific, hashable

    def key(self):
        return (self.kind, self.member, self.args)


EXCLUSIVE_KINDS = {"move", "speak"}          # need the floor: one per round
VOTE_KINDS = {"vote"}


@dataclass
class Policy:
    """A member's response for one round: a prioritized list of (guard, act).

    guard: callable(state) -> bool, or None meaning "the state I saw is still the
    state in the respects this act depends on" (implemented as: the top frame id
    and the phase are unchanged since the round opened).
    A policy may yield several non-exclusive acts and at most one exclusive act.
    """
    entries: list  # of (guard | None, Act)
    standing: bool = False   # if True, the server keeps consulting it in later rounds without re-asking


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
    members: list
    needs_second: bool = NEEDS_SECOND_DEFAULT
    stack: list = field(default_factory=list)          # of Frame, bottom first
    next_fid: int = 1
    phase: str = "open"                                 # open | vote | consent | ruling | objection-vote
    vote: Optional[dict] = None                         # {"on": fid or label, "ballots": {m: choice}, "threshold": "majority"}
    consent: Optional[dict] = None                      # {"text": ..., "objections": [...]}
    points: list = field(default_factory=list)          # pending points of order (texts)
    claims: dict = field(default_factory=dict)          # member -> round first claimed (continuous)
    speeches: dict = field(default_factory=dict)        # (member, fid) -> count
    last_side: Optional[str] = None                     # for alternation
    rotation: int = 0
    round_no: int = 0
    last_announced: Optional[dict] = None               # for the challenge window
    business_since_announce: bool = True
    record: list = field(default_factory=list)          # the minutes-grade events
    claim_log: list = field(default_factory=list)       # every intent submitted, admitted or not
    present: set = field(default_factory=set)

    def top(self) -> Optional[Frame]:
        return self.stack[-1] if self.stack else None

    def snapshot_key(self):
        """What an unguarded intent is implicitly guarded on."""
        t = self.top()
        return (t.fid if t else None, self.phase, len(self.points))

    def fingerprint(self) -> str:
        d = asdict(self)
        d["present"] = sorted(self.present)
        d["claims"] = sorted(self.claims.items())
        d["speeches"] = sorted((f"{k[0]}|{k[1]}", v) for k, v in self.speeches.items())
        return hashlib.sha256(json.dumps(d, sort_keys=True, default=str).encode()).hexdigest()


# ----------------------------------------------------------------------------
# Admissibility: the toy's available_actions. Returns a reason string or None.
# ----------------------------------------------------------------------------
def inadmissible_reason(a: Act, s: State) -> Optional[str]:
    t = s.top()
    if a.member not in s.present:
        return "not present"
    if s.phase in ("vote", "ruling", "objection-vote"):
        if a.kind == "vote":
            if a.member in s.vote["ballots"]:
                return "already voted"
            return None
        return "voting in progress; no interruption [45:6]"
    if a.kind == "vote":
        return "no vote in progress"
    if s.phase == "consent":
        if a.kind in ("object-consent", "pass", "chair-proceed"):
            return None
        return "consent pending; only objection or silence"
    # phase open
    if a.kind == "pass":
        return None
    if a.kind == "point-of-order":
        # timely only against the most recent transition (toy: the previous round's admitted act)
        if not s.record or s.record[-1].get("round") != s.round_no - 1:
            return "breach not just occurred [23:5]"
        return None
    if a.kind == "demand-division":
        if s.last_announced is None or s.business_since_announce:
            return "no vote just announced / business intervened [45:9]"
        return None
    if a.kind == "chair-proceed":
        return None   # the chair's next duty, used as a probe transition (see is_fragile)
    if a.kind == "second":
        (fid,) = a.args
        fr = next((f for f in s.stack if f.fid == fid), None)
        if fr is None:
            return "no such motion"
        if not s.needs_second:
            return "seconds not required in this configuration [49:21(2)]"
        if fr.debated:
            return "debate begun; second immaterial [4:13]"
        if fr.maker == a.member:
            return "maker cannot second"
        if a.member in fr.seconded_by:
            return "already seconded by this member"
        return None
    if a.kind == "object-consideration":
        if t is None or t.kind != "main" or t.debated:
            return "consideration already begun or no original main pending [26:2(1)]"
        if not t.stated:
            return "the question has not been stated yet"
        return None
    if a.kind == "speak":
        if t is None:
            return "nothing pending"
        if not DEBATABLE[t.kind]:
            return "undebatable"
        if s.needs_second and not t.seconded_by:
            return "not yet seconded/stated"
        if s.speeches.get((a.member, t.fid), 0) >= 2:
            return "two speeches exhausted [43:8]"
        if s.speeches.get((a.member, t.fid), 0) == 1 and any(
            s.speeches.get((m, t.fid), 0) == 0 and m in s.claims for m in s.present if m != a.member
        ):
            return "second speech barred while another seeking the floor has not spoken [43:13]"
        return None
    if a.kind == "move":
        kind, text = a.args[0], a.args[1]
        if kind not in RANK:
            return "unknown motion"
        if t is None:
            if kind != "main":
                return "secondary motion with nothing pending"
            return None
        if s.needs_second and not t.seconded_by:
            return "pending motion not yet seconded"
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
# Applying an act (pure: returns a new state), plus the chair's deterministic duties
# ----------------------------------------------------------------------------
def apply(a: Act, s: State, note: str = "") -> State:
    s = copy.deepcopy(s)
    ev = {"round": s.round_no, "member": a.member, "act": a.kind, "args": list(a.args)}
    if note:
        ev["note"] = note
    if a.kind == "pass":
        return s
    s.business_since_announce = True
    if a.kind == "move":
        kind, text = a.args[0], a.args[1]
        fr = Frame(s.next_fid, kind, a.member, text, made_round=s.round_no)
        s.next_fid += 1
        s.stack.append(fr)
        if not s.needs_second:
            fr.stated = True
            fr.stated_round = s.round_no
            ev["chair"] = "stated"
        s.record.append(ev)
        s.claims.pop(a.member, None)
        return s
    if a.kind == "chair-proceed":
        # the chair's next mechanical duty in the ordinary course, when no member is claiming anything:
        t = s.top()
        if t is not None and s.needs_second and not t.seconded_by:
            s.stack.pop()                       # "there is no second; the motion is not before the meeting" [4:10]
            s.record.append({"round": s.round_no, "chair": "no second; motion dies [4:10]", "on": t.fid})
        elif s.phase == "consent" and s.consent is not None and not s.consent["objections"]:
            s.record.append({"round": s.round_no, "chair": "no objection; adopted by unanimous consent [4:58]"})
            s.consent = None
            s.phase = "open"
        elif t is not None and t.stated and not DEBATABLE[t.kind] and s.phase == "open":
            s.phase = "vote"                    # an undebatable question is put as soon as it is stated
            s.vote = {"on": t.fid, "ballots": {}, "threshold": "majority"}
            s.record.append({"round": s.round_no, "chair": "put", "on": t.fid})
        elif s.last_announced is not None and not s.business_since_announce:
            s.business_since_announce = True    # the next item of business; the challenge window closes [45:9]
        return s
    if a.kind == "second":
        (fid,) = a.args
        fr = next(f for f in s.stack if f.fid == fid)
        fr.seconded_by = sorted(set(fr.seconded_by) | {a.member})
        if not fr.stated:
            fr.stated = True
            fr.stated_round = s.round_no
        ev["chair"] = "stated"
        s.record.append(ev)
        return s
    if a.kind == "speak":
        side, text = a.args[0], a.args[1]
        t = s.top()
        t.debated = True
        s.speeches[(a.member, t.fid)] = s.speeches.get((a.member, t.fid), 0) + 1
        s.last_side = side
        s.record.append(ev)
        s.claims.pop(a.member, None)
        return s
    if a.kind == "point-of-order":
        s.points.append({"by": a.member, "text": a.args[0]})
        s.record.append(ev)
        return s
    if a.kind == "object-consideration":
        s.phase = "objection-vote"
        s.vote = {"on": "objection-to-consideration", "ballots": {}, "threshold": "two-thirds-neg"}
        s.record.append(ev)
        return s
    if a.kind == "object-consent":
        s.consent["objections"] = sorted(set(s.consent["objections"]) | {a.member})
        s.record.append(ev)
        return s
    if a.kind == "demand-division":
        ev["chair"] = "division: retaken by the same exact ballots — identical result (vacuous for agents)"
        s.record.append(ev)
        return s
    if a.kind == "vote":
        (choice,) = a.args
        s.vote["ballots"][a.member] = choice
        return s  # votes are sealed; the record carries the tally at announcement
    raise ValueError(a.kind)


def chair_duties(s: State) -> State:
    """The chair's mechanical, duty-shaped acts, applied whenever exactly one is due."""
    s = copy.deepcopy(s)
    if s.phase == "open" and s.points:
        # a point of order must be disposed of before anything else [23:2(1)]
        s.phase = "ruling"
        s.vote = {"on": f"point-of-order:{s.points[0]['text']}", "ballots": {}, "threshold": "majority"}
        s.record.append({"round": s.round_no, "chair": "point submitted to the assembly [23:18-21]"})
    return s


def announce_if_complete(s: State) -> State:
    """Close a vote when every present member has voted (or is recorded silent)."""
    s = copy.deepcopy(s)
    v = s.vote
    if v is None:
        return s
    if any(m not in v["ballots"] for m in s.present):
        return s
    yes = sum(1 for c in v["ballots"].values() if c == "aye")
    no = sum(1 for c in v["ballots"].values() if c == "no")
    if v["threshold"] == "two-thirds-neg":
        adopted = no >= 2 * yes and no > 0   # objection sustained by two-thirds in the negative [26:7]
    else:
        adopted = yes > no
    s.record.append({"round": s.round_no, "chair": "announced", "on": v["on"],
                     "tally": dict(sorted(v["ballots"].items())), "result": "adopted" if adopted else "rejected"})
    s.last_announced = {"on": v["on"], "result": adopted}
    s.business_since_announce = False
    if s.phase == "ruling":
        s.points.pop(0)
        s.phase = "open"
    elif s.phase == "objection-vote":
        if adopted:
            s.stack.pop()  # main question dismissed for the session [26:5]
        s.phase = "open"
    elif s.phase == "vote":
        t = s.stack.pop()
        if adopted and t.kind == "previous-question":
            # vote immediately on the next frame; the toy puts it in the next round
            s.record.append({"round": s.round_no, "chair": "previous question ordered; question recurs without debate"})
        s.phase = "open"
    s.vote = None
    return s


def put_question_if_exhausted(s: State, asked_for_debate: set) -> State:
    """Debate exhaustion is derived: no timely claim for debate [put-question's third disjunct]."""
    s = copy.deepcopy(s)
    t = s.top()
    if t is None or s.phase != "open" or s.points:
        return s
    if s.needs_second and not t.seconded_by:
        return s
    if s.round_no <= t.stated_round:
        return s   # stated this round: nobody has yet been asked at a state where it was pending
    if not DEBATABLE[t.kind] or not asked_for_debate:
        s.phase = "vote"
        s.vote = {"on": t.fid, "ballots": {}, "threshold": "majority"}
        s.record.append({"round": s.round_no, "chair": "put", "on": t.fid})
    return s


# ----------------------------------------------------------------------------
# Derived classifications (D1, D2)
# ----------------------------------------------------------------------------
def ordinary_floor_acts(s: State) -> list:
    """The ordinary course of business at s, as probes for fragility: another member speaking
    in debate when the pending question is debatable, and otherwise the chair proceeding to the
    next duty (putting an undebatable question, declaring no second, closing a window).
    Deliberately NOT another member's competing motion: being precluded by a higher-ranked
    motion is contention, resolved by the ladder, not a timeliness window."""
    t = s.top()
    someone = sorted(s.present)[0]
    outs = [Act("chair-proceed", someone, ())]
    if t is not None and DEBATABLE[t.kind] and t.stated and (not s.needs_second or t.seconded_by):
        outs.append(Act("speak", someone, ("for", "...")))
    return outs


def is_fragile(a: Act, s: State) -> bool:
    """D1: an act is fragile at s if it is admissible now and some ordinary floor act
    (by another member) would make it inadmissible. Such acts must be admitted ahead
    of the floor or the floor race deprives their owner of a timely right."""
    if not admissible(a, s):
        return False
    for b in ordinary_floor_acts(s):
        if b.member == a.member:
            b = Act(b.kind, next(m for m in sorted(s.present) if m != a.member), b.args)
        if not admissible(b, s):
            continue
        s2 = apply(b, s)
        if not admissible(a, s2):
            return True
    return False


def commute(a: Act, b: Act, s: State) -> bool:
    """D2: two acts commute at s if both orders are admissible and reach the same state.
    Exclusive (floor) acts never commute: the floor is sequential by definition, and the
    order of speeches is part of the record even when the rest of the state agrees."""
    if a.kind in EXCLUSIVE_KINDS or b.kind in EXCLUSIVE_KINDS:
        return False
    if not (admissible(a, s) and admissible(b, s)):
        return False
    sa = apply(a, s)
    sb = apply(b, s)
    if not (admissible(b, sa) and admissible(a, sb)):
        return False
    sab = apply(b, sa)
    sba = apply(a, sb)
    # compare everything except the order of record entries from this pair
    def norm(x: State):
        y = copy.deepcopy(x)
        y.record = sorted(y.record[-2:], key=lambda e: json.dumps(e, sort_keys=True)) if len(y.record) >= 2 else y.record
        return y.fingerprint()
    return norm(sab) == norm(sba)


# ----------------------------------------------------------------------------
# Preference in recognition (protocol.json preference_in_recognition, toy subset)
# ----------------------------------------------------------------------------
def preference_tuple(a: Act, s: State, all_debate: bool = True):
    """Lower sorts first. Ladder:
      1. the maker's first speech on the immediately pending question beats everyone [42:9(1)];
      2. alternation between declared sides [42:9(3)] — only when every candidate is a debate
         claim, because the book ranks debaters among debaters and says nothing about a mover
         versus a debater (that is the race);
      3. earliest claim round (RONR's "rose first", in fair time);
      4. rotation pointer.
    42:9(2) (no second speech while an unspoken member seeks the floor) is an admissibility
    filter on the second speech itself [43:13], not a rung here, so it never ranks a debater
    above a mover."""
    t = s.top()
    maker_pref = 1
    alternation_pref = 0
    if a.kind == "speak" and t is not None:
        if t.maker == a.member and s.speeches.get((a.member, t.fid), 0) == 0:
            maker_pref = 0
        side = a.args[0]
        if all_debate and s.last_side is not None and side == s.last_side:
            alternation_pref = 1
    claim_age = s.claims.get(a.member, s.round_no)
    idx = s.members.index(a.member)
    rot = (idx - s.rotation) % len(s.members)
    return (maker_pref, alternation_pref, claim_age, rot)


# ----------------------------------------------------------------------------
# The round
# ----------------------------------------------------------------------------
def resolve(s: State, profile: dict, silent: set = frozenset()) -> State:
    """profile: {member: Policy}. silent: members whose response did not arrive by the
    presence deadline (the one place wall-clock enters). Pure in (s, profile, silent)."""
    s = copy.deepcopy(s)
    s.round_no += 1
    rnd = s.round_no
    for m in sorted(silent):
        s.claim_log.append({"round": rnd, "member": m, "intent": "no response by deadline (silent this round)"})

    # 1. each member's live acts: every live non-exclusive entry, the first live exclusive one
    live = []
    asked_for_debate = set()
    for m in sorted(profile):
        pol = profile[m]
        took_exclusive = False
        for guard, act in pol.entries:
            g_ok = guard(s) if guard is not None else True
            s.claim_log.append({"round": rnd, "member": m, "intent": act.key(), "guard_held": g_ok,
                                "admissible": admissible(act, s) if g_ok else None})
            if not g_ok or not admissible(act, s):
                continue
            if act.kind in EXCLUSIVE_KINDS:
                if took_exclusive:
                    continue
                took_exclusive = True
                if act.kind == "speak":
                    asked_for_debate.add(m)
                    s.claims.setdefault(m, rnd)
                else:
                    s.claims.setdefault(m, rnd)
            live.append(act)
    # members present but not claiming the floor this round drop their claim age
    for m in list(s.claims):
        if not any(a.member == m and a.kind in EXCLUSIVE_KINDS for a in live):
            s.claims.pop(m)

    # 2. votes (sealed, additive) — only in a vote phase; nothing else is admissible then
    votes = [a for a in live if a.kind in VOTE_KINDS]
    for a in sorted(votes, key=Act.key):
        s = apply(a, s)
    if votes or s.phase in ("vote", "ruling", "objection-vote"):
        s = announce_if_complete(s)
        return s

    # 3. fragile acts first (derived), each admitted in canonical order, re-checking admissibility
    fragile = sorted((a for a in live if a.kind not in EXCLUSIVE_KINDS and is_fragile(a, s)), key=Act.key)
    rest_nonexcl = [a for a in live if a.kind not in EXCLUSIVE_KINDS and a not in fragile]
    for a in fragile:
        if admissible(a, s):
            s = apply(a, s, note="fragile: admitted ahead of the floor")
    # 4. remaining non-exclusive acts: batch the pairwise-commuting ones; the rest compete for the slot
    batch, compete = [], []
    for a in rest_nonexcl:
        if not admissible(a, s):
            continue
        if all(commute(a, b, s) for b in rest_nonexcl if b is not a and admissible(b, s)):
            batch.append(a)
        else:
            compete.append(a)
    for a in sorted(batch, key=Act.key):
        if admissible(a, s):
            s = apply(a, s, note="non-exclusive, commuting")
    # 5. a point of order halts the floor for this round: it must be disposed of first
    if s.points:
        s = chair_duties(s)
        return s
    # 6. exactly one exclusive act, by the ladder; losers keep their claim age (I1, F3)
    excl = [a for a in live if a.kind in EXCLUSIVE_KINDS and admissible(a, s)] + [a for a in compete if admissible(a, s)]
    if excl:
        all_debate = all(a.kind == "speak" for a in excl)
        excl.sort(key=lambda a: (preference_tuple(a, s, all_debate), a.key()))
        winner = excl[0]
        s = apply(winner, s, note="floor: " + "/".join(str(x) for x in preference_tuple(winner, s, all_debate)))
        if winner.kind in EXCLUSIVE_KINDS:
            s.rotation = (s.members.index(winner.member) + 1) % len(s.members)
    # 7. the chair's derived duties
    if s.consent is not None and s.phase == "consent":
        if s.consent["objections"]:
            s.record.append({"round": rnd, "chair": "objection heard; regular cycle [4:59]"})
        else:
            s.record.append({"round": rnd, "chair": "no objection; adopted by unanimous consent [4:58]"})
        s.consent = None
        s.phase = "open"
        return s
    t = s.top()
    if t is not None and s.phase == "open" and s.needs_second and not t.seconded_by and rnd > t.made_round:
        s = apply(Act("chair-proceed", sorted(s.present)[0], ()), s)   # no second arrived in the round after making
        return s
    s = put_question_if_exhausted(s, asked_for_debate)
    return s


# ----------------------------------------------------------------------------
# Collection: where arrival order would enter, and doesn't
# ----------------------------------------------------------------------------
def collect_and_resolve(s: State, arrivals: list, deadline_cut: Optional[int] = None) -> State:
    """arrivals: a list of (member, Policy) in the order they arrived. deadline_cut: if set,
    responses at index >= deadline_cut arrived after the presence deadline; they are logged
    as supplementary and the member is silent for this round. The result depends on the SET
    of on-time responses only."""
    on_time = arrivals if deadline_cut is None else arrivals[:deadline_cut]
    late = [] if deadline_cut is None else arrivals[deadline_cut:]
    profile = {m: p for m, p in on_time}
    silent = set(s.present) - set(profile)
    s2 = resolve(s, profile, silent)
    for m, p in late:
        s2.claim_log.append({"round": s2.round_no, "member": m, "intent": "late (supplementary): " +
                             "; ".join(str(a.key()) for _, a in p.entries)})
    return s2
