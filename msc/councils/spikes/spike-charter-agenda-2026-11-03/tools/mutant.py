#!/usr/bin/env python3
"""mutant.py — reproduce de-novo-feedback-1 §5's seven deliberate breakages of quick-council.json
and run the checker on the result. The checker must report every one; exit 0 iff it does.

Usage: python3 tools/mutant.py
"""
import json, os, subprocess, sys, tempfile, copy

HERE = os.path.dirname(os.path.abspath(__file__))
src = json.load(open(os.path.join(HERE, "..", "charters", "quick-council.json")))
m = copy.deepcopy(src)
ops = m["overlay"]["ops"]
def setp(pid, val):
    for op in ops:
        if op["op"] == "param" and op["param"] == pid: op["value"] = val; return
    ops.append({"op": "param", "param": pid, "value": val})
setp("session_scope", "no-such-scope")                       # 1 out-of-enum
setp("speech_limit", "banana")                               # 2 wrong type
for k in m["logical_time"]:                                  # 3 prose/empty mappings
    if k != "desc": m["logical_time"][k] = ""
ops.append({"op": "patch", "surface": "actions.no-such-action.expr", "replace": "do the thing"})  # 4 bad surface + prose expr
ops.append({"op": "disable_motion", "motion": "amend"})      # 5 allowed and disabled
for op in ops:                                               # 6 instruction bound drifts from the expression
    if op["op"] == "declare_param" and op["param"] == "may_add_members_max_total": op["value"] = "ninety-nine"
for k in m["electronic_meeting_rules"]:                      # 7 empty rules
    if k != "cite": m["electronic_meeting_rules"][k] = ""

with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False, dir=HERE) as f:
    json.dump(m, f); path = f.name
try:
    r = subprocess.run([sys.executable, os.path.join(HERE, "check-charter.py"), path], capture_output=True, text=True)
    out = r.stdout
    expect = ["session_scope value 'no-such-scope' not in enum", "speech_limit must be null or a record", "logical_time.now is empty",
              "does not resolve to a model path", "motion/amend both allowed and disabled", "declared number, value 'ninety-nine'",
              "electronic_meeting_rules.voting is empty"]
    missed = [e for e in expect if e not in out]
    print(out)
    print("mutant checker exit:", r.returncode, "| expected problems found:", len(expect) - len(missed), "of", len(expect))
    for e in missed: print("  MISSED:", e)
    sys.exit(0 if r.returncode == 1 and not missed else 1)
finally:
    os.unlink(path)
