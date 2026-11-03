"""concilium core: pure functions over a charter and a canonical event log.

Nothing here reads a clock, a file, or arrival order. Everything that needs a
mind arrives as a member's intent inside a `round-closed` input event; the
store/server layer decides *when* a round closes (a wall-clock fact) and
records that as data (`closed_by`, `absent`, `gone`).

Canonical log = derived events (computed here) interleaved with input events
(`round-closed`, `effect-completed`). `replay()` recomputes every derived event
from the charter plus the input events and compares.

The council logic is deliberately crude (the contention-rounds and
charter-agenda spikes own it). It is a committee per the model's `committee`
configuration: no seconds [49:21(2)], no speech limits [49:21(3)], informal
discussion without a motion [49:21(4)], no closure motions [50:25] -- so
debate ends when a full round passes, an agentic divergence recorded here.
"""
import hashlib
import json
import copy

ENGINE_VERSION = "concilium-core/0"


def canon(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def sha(obj_or_str):
    s = obj_or_str if isinstance(obj_or_str, str) else canon(obj_or_str)
    return hashlib.sha256(s.encode()).hexdigest()


# ----------------------------------------------------------------- state ---

def initial_state(charter):
    seats = [s["id"] for s in charter["seats"]]
    return {
        "charter_sha": sha(charter),
        "seats": seats,
        "quorum": charter.get("quorum", len(seats) // 2 + 1),
        "agenda": [a["id"] for a in charter["agenda"]],
        "item_index": 0,
        "phase": "pre-convene",   # call-to-order | open | debate | vote | adjourned
        "pending": None,          # {"text", "makers"}
        "round": None,            # {"id", "kind", "item", "addressed"}
        "round_counter": 0,
        "open_rounds_this_item": 0,
        "debate_rounds_this_motion": 0,
        "inquorate_streak": 0,
        "last_recognized": {s: 0 for s in seats},
        "standing": {},           # seat -> {"item", "text", "since_round"}
        "present": [],
        "outcomes": [],           # per item
        "adjourned": False,
    }


def apply(state, ev):
    """Fold one canonical event into state (pure)."""
    st = copy.deepcopy(state)
    t, b = ev["type"], ev["body"]
    if t == "convened":
        st["phase"] = "call-to-order"
    elif t == "round-opened":
        st["round"] = {"id": b["round"], "kind": b["kind"], "item": b.get("item"),
                       "addressed": b["addressed"]}
        st["round_counter"] = b["n"]
        st["phase"] = b["kind"]
        if b["kind"] == "open":
            st["open_rounds_this_item"] += 1
        if b["kind"] == "debate":
            st["debate_rounds_this_motion"] += 1
    elif t == "round-closed":
        st["round"] = None
    elif t == "roll-taken":
        st["present"] = b["present"]
    elif t == "inquorate":
        st["inquorate_streak"] += 1
    elif t == "quorum-ok":
        st["inquorate_streak"] = 0
    elif t == "motion-pending":
        st["pending"] = {"text": b["text"], "makers": b["makers"]}
        st["debate_rounds_this_motion"] = 0
        for m in b["makers"]:
            st["last_recognized"][m] = b["round_n"]
            st["standing"].pop(m, None)
    elif t == "intent-standing":
        st["standing"][b["seat"]] = {"item": b["item"], "text": b["text"], "since_round": b["round_n"]}
    elif t == "makers-withdrew":
        st["pending"]["makers"] = b["makers"]
    elif t == "intent-lapsed":
        st["standing"].pop(b["seat"], None)
    elif t == "motion-withdrawn":
        st["pending"] = None
    elif t == "vote-result":
        st["outcomes"].append({"item": b["item"], "motion": b["text"], "result": b["result"],
                               "tally": b["tally"]})
        st["pending"] = None
    elif t == "item-closed":
        if b.get("disposition") not in ("voted", "done"):   # a vote-result already recorded the outcome
            st["outcomes"].append({"item": b["item"], "result": b["disposition"]})
        st["item_index"] += 1
        st["open_rounds_this_item"] = 0
        st["pending"] = None
        # standing intents are scoped to an item; they lapse with it (recorded)
        st["standing"] = {k: v for k, v in st["standing"].items() if v["item"] != b["item"]}
    elif t == "adjourned":
        st["adjourned"] = True
        st["phase"] = "adjourned"
        st["round"] = None
    return st


def fold(charter, events):
    st = initial_state(charter)
    for ev in events:
        st = apply(st, ev)
    return st


# ------------------------------------------------------ available actions ---

def available(state, charter, seat):
    """What `seat` may submit in the open round, each with its reason.

    The analogue of the RONR model's available_actions(member, role, state),
    narrowed to this toy committee. Reasons cite the model/RONR where the rule
    is RONR's and say `agentic` where it is this skeleton's divergence."""
    r = state["round"]
    if state["adjourned"] or r is None or seat not in r["addressed"]:
        return []
    k = r["kind"]
    acts = []
    if k == "call-to-order":
        acts.append({"action": "present", "reason": "answer the roll; quorum is counted from who answers [40:1]"})
        return acts
    pass_ = {"action": "pass", "reason": "agentic: passing is a real, recorded choice; a full round of passes ends debate (committee has no closure motion [50:25])"}
    if k == "open":
        stand = state["standing"].get(seat)
        acts.append({"action": "move", "fields": ["text"],
                     "reason": "no motion pending; in a committee a motion needs no second [49:21(2), 50:25]"
                               + (f"; you hold a standing intent from an earlier round (re-affirm by moving it, possibly revised): {stand['text']!r}" if stand else "")})
        acts.append({"action": "discuss", "fields": ["text"],
                     "reason": "informal discussion without a motion is allowed in small boards and committees [49:21(4)]"})
        acts.append(pass_)
    elif k == "debate":
        acts.append({"action": "speak", "fields": ["text"],
                     "reason": "a motion is pending and debatable; no speech limit in committee [49:21(3)]"})
        if seat in (state["pending"] or {}).get("makers", []):
            acts.append({"action": "withdraw",
                         "reason": "you are a maker of the pending motion; agentic simplification of withdraw-own-motion [33:11-19]"})
        acts.append(pass_)
    elif k == "vote":
        acts.append({"action": "vote", "fields": ["vote", "text?"],
                     "reason": "the question is put; vote yes, no or abstain (abstaining is recorded, not counted) [44:1-3]"})
    return acts


def validate(state, charter, seat, intent):
    """Return None if `intent` is admissible for `seat` now, else the reason."""
    if not isinstance(intent, dict) or "action" not in intent:
        return "intent must be an object with an 'action'"
    acts = {a["action"]: a for a in available(state, charter, seat)}
    if not acts:
        return "no open round is addressed to you"
    a = intent["action"]
    if a not in acts:
        return f"'{a}' is not available now; available: {sorted(acts)}"
    if a in ("move", "discuss", "speak") and not (intent.get("text") or "").strip():
        return f"'{a}' needs non-empty text"
    if a == "vote" and intent.get("vote") not in ("yes", "no", "abstain"):
        return "vote must be yes, no or abstain"
    return None


def normalize_intent(intent):
    out = {"action": intent["action"]}
    for k in ("text", "vote"):
        if intent.get(k) is not None:
            out[k] = intent[k] if k == "vote" else str(intent[k]).strip()
    return out


# -------------------------------------------------------------- resolve ---

def _open_round(state, kind, item=None):
    n = state["round_counter"] + 1
    return {"type": "round-opened",
            "body": {"round": f"r{n}", "n": n, "kind": kind, "item": item,
                     "addressed": list(state["seats"])}}


def _seeded_order(seed, seats):
    return sorted(seats, key=lambda s: sha(seed + "|" + s))


def _emit(state, evs, typ, body):
    ev = {"type": typ, "body": body}
    evs.append(ev)
    return apply(state, ev)


def convene(charter):
    st = initial_state(charter)
    evs = []
    st = _emit(st, evs, "convened", {"charter_sha": st["charter_sha"], "engine": ENGINE_VERSION,
                                     "seats": st["seats"], "quorum": st["quorum"]})
    o = _open_round(st, "call-to-order")
    st = _emit(st, evs, o["type"], o["body"])
    return evs


def _next_item_or_adjourn(st, evs, charter):
    if st["item_index"] >= len(st["agenda"]):
        st = _emit(st, evs, "adjourned", {"reason": "agenda complete"})
        return st
    o = _open_round(st, "open", st["agenda"][st["item_index"]])
    return _emit(st, evs, o["type"], o["body"])


def step(state, charter, inp):
    """Given the state just before input event `inp`, return the derived events
    that follow it. Pure; the only function that turns minds' intents into
    procedure."""
    evs = []
    st = apply(state, inp)
    if inp["type"] == "effect-completed":
        return evs  # v0: effects only annotate the record
    if inp["type"] != "round-closed":
        raise ValueError(f"not an input event: {inp['type']}")
    b = inp["body"]
    r = state["round"]
    assert r and r["id"] == b["round"], "close must match the open round"
    subs = b["submitted"]                       # seat -> normalized intent
    submitters = sorted(subs)
    pol = charter.get("debate_policy", {})
    max_open = pol.get("max_open_rounds", 3)
    max_debate = pol.get("max_debate_rounds", 3)
    max_inq = charter.get("round_policy", {}).get("max_inquorate_rounds", 2)
    item = r.get("item")

    # quorum: counted from submitters of this round (agentic: presence is
    # demonstrated by acting, per round, not by a standing roll)
    if len(submitters) < st["quorum"]:
        st = _emit(st, evs, "inquorate", {"round": r["id"], "submitted": len(submitters),
                                          "quorum": st["quorum"],
                                          "cite": "40:6-8 (without a quorum, no business but to adjourn or obtain one)"})
        if st["inquorate_streak"] >= max_inq:
            st = _emit(st, evs, "adjourned", {"reason": f"no quorum in {max_inq} consecutive rounds"})
            return evs
        o = _open_round(st, r["kind"], item)
        st = _emit(st, evs, o["type"], o["body"])
        return evs
    if st["inquorate_streak"]:
        st = _emit(st, evs, "quorum-ok", {"round": r["id"]})

    k = r["kind"]
    if k == "call-to-order":
        st = _emit(st, evs, "roll-taken", {"present": submitters,
                                           "absent": sorted(set(st["seats"]) - set(submitters))})
        return evs + _tail(st, charter, _next_item_or_adjourn)

    if k == "open":
        movers = {s: i for s, i in subs.items() if i["action"] == "move"}
        for s_ in sorted(st["standing"]):
            if st["standing"][s_]["item"] == item and s_ not in movers:
                st = _emit(st, evs, "intent-lapsed", {"seat": s_, "item": item,
                                                      "why": "not re-affirmed when the floor reopened" if s_ in subs else "holder absent when the floor reopened"})
        if movers:
            groups = {}
            for s, i in movers.items():
                groups.setdefault(i["text"], []).append(s)
            # ladder: standing-intent priority, then least recently recognized,
            # then a seeded order -- never arrival order
            seed = sha(r["id"] + "|" + canon(subs))
            order = _seeded_order(seed, st["seats"])

            def key(text):
                makers = sorted(groups[text])
                stand = any(st["standing"].get(m, {}).get("item") == item for m in makers)
                lr = min(st["last_recognized"][m] for m in makers)
                first = min(order.index(m) for m in makers)
                return (0 if stand else 1, lr, first)
            win = sorted(groups, key=key)[0]
            st = _emit(st, evs, "motion-pending", {
                "item": item, "text": win, "makers": sorted(groups[win]), "round_n": st["round_counter"],
                "ladder": "standing > least-recently-recognized > seeded(sha(round|submissions))",
                "co_made": len(groups[win]) > 1})
            for text, makers in sorted(groups.items()):
                if text == win:
                    continue
                for m in sorted(makers):
                    st = _emit(st, evs, "intent-standing", {"seat": m, "item": item, "text": text,
                                                            "round_n": st["round_counter"]})
            o = _open_round(st, "debate", item)
            st = _emit(st, evs, o["type"], o["body"])
            return evs
        any_discuss = any(i["action"] == "discuss" for i in subs.values())
        if any_discuss and st["open_rounds_this_item"] < max_open:
            o = _open_round(st, "open", item)
            st = _emit(st, evs, o["type"], o["body"])
            return evs
        voted = any(o["item"] == item for o in st["outcomes"])
        st = _emit(st, evs, "item-closed", {"item": item, "disposition": "done" if voted else "no-motion"})
        return evs + _tail(st, charter, _next_item_or_adjourn)

    if k == "debate":
        makers = st["pending"]["makers"]
        withdrawing = sorted(s for s, i in subs.items() if i["action"] == "withdraw")
        if withdrawing:
            remaining = [m for m in makers if m not in withdrawing]
            if not remaining:
                st = _emit(st, evs, "motion-withdrawn", {"item": item, "by": withdrawing})
                if st["open_rounds_this_item"] < max_open + 1:
                    o = _open_round(st, "open", item)
                    st = _emit(st, evs, o["type"], o["body"])
                    return evs
                st = _emit(st, evs, "item-closed", {"item": item, "disposition": "withdrawn"})
                return evs + _tail(st, charter, _next_item_or_adjourn)
            # verify-1 D1: re-emitting motion-pending here reset the debate-round
            # limit, so a procedural act silently extended charter policy. A
            # partial withdrawal only changes who the makers are.
            st = _emit(st, evs, "makers-withdrew", {"item": item, "by": withdrawing, "makers": remaining,
                                                    "note": "co-makers withdrew; the motion stands with the rest"})
        all_pass = all(i["action"] == "pass" for i in subs.values())
        if all_pass or st["debate_rounds_this_motion"] >= max_debate:
            st = _emit(st, evs, "question-put", {
                "item": item, "text": st["pending"]["text"],
                "why": "every member who acted passed" if all_pass else f"debate round limit {max_debate} reached (charter)"})
            o = _open_round(st, "vote", item)
            st = _emit(st, evs, o["type"], o["body"])
            return evs
        o = _open_round(st, "debate", item)
        st = _emit(st, evs, o["type"], o["body"])
        return evs

    if k == "vote":
        tally = {"yes": 0, "no": 0, "abstain": 0}
        for s in submitters:
            tally[subs[s]["vote"]] += 1
        result = "adopted" if tally["yes"] > tally["no"] else "rejected"
        st = _emit(st, evs, "vote-result", {"item": item, "text": st["pending"]["text"], "tally": tally,
                                            "by_seat": {s: subs[s]["vote"] for s in submitters},
                                            "result": result,
                                            "threshold": "majority of votes cast [44:1]"})
        if any(v["item"] == item for v in st["standing"].values()):
            # another member's motion on this item is still standing: reopen the
            # floor on the same item; holders re-affirm (priority) or it lapses
            o = _open_round(st, "open", item)
            st = _emit(st, evs, o["type"], o["body"])
            return evs
        st = _emit(st, evs, "item-closed", {"item": item, "disposition": "voted"})
        return evs + _tail(st, charter, _next_item_or_adjourn)

    raise ValueError(f"unknown round kind {k}")


def _tail(st, charter, fn):
    evs = []
    fn(st, evs, charter)
    return evs


# --------------------------------------------------------------- replay ---

INPUT_TYPES = ("round-closed", "effect-completed")


def replay(charter, events):
    """Recompute every derived event from charter + input events; return a
    list of mismatches (empty = the log replays to itself)."""
    bad = []
    expected = convene(charter)
    i = 0
    st = initial_state(charter)

    def strip(e):
        return {"type": e["type"], "body": e["body"]}

    pos = 0
    queue = list(expected)
    while pos < len(events):
        ev = events[pos]
        if queue:
            exp = queue.pop(0)
            if strip(ev) != exp:
                bad.append({"seq": ev.get("seq"), "expected": exp, "got": strip(ev)})
                return bad
            st = apply(st, ev)
            pos += 1
            continue
        if ev["type"] not in INPUT_TYPES:
            bad.append({"seq": ev.get("seq"), "error": "derived event with nothing to derive it", "got": strip(ev)})
            return bad
        queue = step(st, charter, strip(ev))
        st = apply(st, ev)
        pos += 1
    if queue:
        bad.append({"error": "log ends before derived events it implies", "missing": queue})
    return bad
