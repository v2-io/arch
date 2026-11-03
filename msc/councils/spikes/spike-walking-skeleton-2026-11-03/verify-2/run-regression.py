#!/usr/bin/env python3
"""verify-2: run the spike's own tests/regression.py unchanged, but with its
scratch and its output redirected under verify-2/ (the brief for this pass is
to write nothing outside verify-2/ and the report). Run from the spike dir:

  python3 verify-2/run-regression.py        -> verify-2/regression-output.txt
"""
import importlib.util, os, sys

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, "tests"))
spec = importlib.util.spec_from_file_location("regression", os.path.join(HERE, "tests", "regression.py"))
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
r.TMP = os.path.join(HERE, "verify-2", "tmp", "regression")
os.makedirs(r.TMP, exist_ok=True)
r.part1()
r.part2()
n_ok = sum(1 for _, ok in r.RESULTS if ok)
r.say(f"\n== {n_ok} of {len(r.RESULTS)} checks pass")
for n, ok in r.RESULTS:
    if not ok:
        r.say("  FAILED:", n)
with open(os.path.join(HERE, "verify-2", "regression-output.txt"), "w") as f:
    f.write("\n".join(r.OUT) + "\n")
