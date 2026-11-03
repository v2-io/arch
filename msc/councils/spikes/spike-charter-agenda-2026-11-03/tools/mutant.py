#!/usr/bin/env python3
"""mutant.py — run both verifiers' mutants of quick-council.json through the checker.

Set A (de-novo-feedback-1 §5): seven breakages the revision-1 checker passed.
Set B (de-novo-feedback-2 §8): "banana" in every logical-time and 9:36 value; nonsense SDCs on an
added motion; the model's own `note` op; a two-seat roll.

Prints, per breakage, whether the checker CAUGHT it, ACCEPTED it on purpose (note op; two-seat
roll is a note, not a problem), or CANNOT catch it because it is prose (the banana rule strings).
Exit 0 iff every catchable breakage is caught and every deliberate acceptance holds.
"""
import json, os, subprocess, sys, tempfile, copy

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = json.load(open(os.path.join(HERE, "..", "charters", "quick-council.json")))

def run(m):
    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False, dir=HERE) as f:
        json.dump(m, f); path = f.name
    try:
        r = subprocess.run([sys.executable, os.path.join(HERE, "check-charter.py"), path], capture_output=True, text=True)
        return r.returncode, r.stdout
    finally: os.unlink(path)

def setp(ops, pid, val):
    for op in ops:
        if op["op"] == "param" and op["param"] == pid: op["value"] = val; return
    ops.append({"op": "param", "param": pid, "value": val})

ok = True
def expect(label, out, needle, kind="CAUGHT"):
    global ok
    hit = needle in out
    if kind == "CAUGHT":
        print(f"  [{'caught' if hit else 'MISSED'}] {label}"); ok &= hit
    elif kind == "ACCEPTED":
        print(f"  [{'accepted' if not hit else 'WRONGLY REFUSED'}] {label}"); ok &= not hit
    else:
        print(f"  [prose — cannot be caught] {label}")

# ---- Set A
a = copy.deepcopy(SRC); ops = a["overlay"]["ops"]
setp(ops, "session_scope", "no-such-scope"); setp(ops, "speech_limit", "banana")
for k in a["logical_time"]:
    if k != "desc": a["logical_time"][k] = ""
ops.append({"op": "patch", "surface": "actions.no-such-action.expr", "replace": "do the thing"})
ops.append({"op": "disable_motion", "motion": "amend"})
for op in ops:
    if op["op"] == "declare_param" and op["param"] == "may_add_members_max_total": op["value"] = "ninety-nine"
for k in a["electronic_meeting_rules"]:
    if k != "cite": a["electronic_meeting_rules"][k] = ""
rc, out = run(a)
print("Set A (feedback-1 §5):")
expect("out-of-enum param", out, "session_scope value 'no-such-scope' not in enum")
expect("wrong-typed param", out, "speech_limit must be null or a record")
expect("empty logical-time mapping", out, "logical_time.now is empty")
expect("patch to a nonexistent surface", out, "does not resolve to a model path")
expect("prose where an expr is required", out, "must be an expression")
expect("amend allowed and disabled", out, "motion/amend both allowed and disabled")
expect("instruction bound not a number", out, "declared number, value 'ninety-nine'")
expect("empty 9:36 rule", out, "electronic_meeting_rules.voting is empty")

# ---- Set B
b = copy.deepcopy(SRC); ops = b["overlay"]["ops"]
for k in b["logical_time"]:
    if k not in ("desc", "next-regular-session"): b["logical_time"][k] = "banana"
for k in b["electronic_meeting_rules"]:
    if k != "cite": b["electronic_meeting_rules"][k] = "banana"
for op in ops:
    if op["op"] == "add_motion" and op["motion"]["id"] == "rise-and-report":
        op["motion"]["class"] = "Q"; op["motion"]["debatable"] = "banana"; op["motion"]["vote"] = ["banana"]
ops.append({"op": "note", "desc": "annotation-grade content the model's own op carries", "cite": "—"})
b["roll"]["seats"] = b["roll"]["seats"][:2]
rc, out = run(b)
print("Set B (feedback-2 §8):")
expect("class 'Q' on an added motion", out, "class 'Q' not in vocabulary motion_classes")
expect("debatable 'banana'", out, "debatable 'banana' not in")
expect("vote ['banana']", out, "vote ['banana'] must be a non-empty list of vocabulary thresholds")
expect("the model's own note op", out, "unknown overlay op", kind="ACCEPTED")
expect("two-seat roll under enrolled-majority warns", out, "enrolled-majority on a roll of 2 is the whole roll")
expect("'banana' as a logical-time mapping", out, "", kind="PROSE")
expect("'banana' as a 9:36 rule", out, "", kind="PROSE")
print("mutant: all catchable breakages caught and deliberate acceptances held" if ok else "mutant: FAILURES above")
sys.exit(0 if ok else 1)
