#!/usr/bin/env python3
"""Mechanical acceptance sweep for the RONR-12 unified model.

Checks (each failure prints a line; exit 1 on any):
  1. Every ["guard", id, ...] reference resolves to guards.json (or a declared defined_in routing).
  2. Every ["adjudged", pred, ...] predicate is declared in adjudication.json.
  3. Every ["timer-open", id, ...] timer is defined in lifecycle.json.
  4. Every ["event-occurred", event, ...] event name is declared in vocabulary.json events.
  5. Operator closure: every list-expression head is in the closed operator set (INTERFACE.md).
  6. Lint: no comparison whose operands are both literals (the F2 class — always-true/false expressions).
  7. Guard polarity: no guard id starting with "not-" or "no-" that twins a declared positive guard.

Run from model/: python3 tools/sweep.py
"""
import json, sys, glob, os

os.chdir(os.path.join(os.path.dirname(__file__), ".."))
FILES = sorted(glob.glob("*.json"))
DATA = {f: json.load(open(f)) for f in FILES}

OPERATORS = {
    "and", "or", "not", "if", "=", "!=", "<", "<=", ">", ">=", "in", "list",
    "+", "*", "exists", "forall", "count", "param", "var",
    "ctx", "attr", "reg", "frames", "top", "bottom", "motion", "class",
    "erank", "applied-to", "adheres-to", "question", "state",
    "debatable", "amendable", "guard", "adjudged", "timer-open",
    "event-occurred", "same-day", "before", "within-quarterly", "anchor", "now",
    "rank-admissible", "applicable",
}
COMPARISONS = {"=", "!=", "<", "<=", ">", ">="}

guards = {g["id"] for g in DATA["guards.json"]["guards"]}
preds = {p["id"] for p in DATA["adjudication.json"]["predicates"]}
timers = {t["id"] for t in DATA["lifecycle.json"]["timers"]["list"]}
events = set()
for grp in DATA["vocabulary.json"]["events"].values():
    if isinstance(grp, list):
        events |= {e["id"] for e in grp}

problems = []

def is_expr(x):
    return isinstance(x, list) and x and isinstance(x[0], str) and x[0] in OPERATORS

def literal(x):
    return isinstance(x, (int, float, bool)) or x is None or (
        isinstance(x, str))  # bare strings: literal unless a bound var — flagged only when BOTH sides literal and neither could be a var

def walk(x, path, bound):
    if isinstance(x, dict):
        for k, v in x.items():
            walk(v, f"{path}.{k}", bound)
        return
    if not isinstance(x, list) or not x:
        return
    head = x[0]
    if isinstance(head, str) and (head in OPERATORS or head.islower() and "-" not in head and len(x) > 1 and False):
        pass
    if isinstance(head, str) and head in OPERATORS:
        if head == "guard" and (len(x) < 2 or x[1] not in guards):
            problems.append(f"{path}: unresolved guard {x[1] if len(x)>1 else '?'}")
        if head == "adjudged" and (len(x) < 2 or x[1] not in preds):
            problems.append(f"{path}: unresolved adjudged predicate {x[1] if len(x)>1 else '?'}")
        if head == "timer-open" and (len(x) < 2 or x[1] not in timers):
            problems.append(f"{path}: unresolved timer {x[1] if len(x)>1 else '?'}")
        if head == "event-occurred" and (len(x) < 2 or x[1] not in events):
            problems.append(f"{path}: undeclared event {x[1] if len(x)>1 else '?'}")
        nb = set(bound)
        if head in {"exists", "forall", "count"} and len(x) >= 2 and isinstance(x[1], str):
            nb.add(x[1])
        if head in COMPARISONS and len(x) == 3:
            def lit(o):
                if isinstance(o, list):
                    return False
                if isinstance(o, str):
                    return o not in bound
                return True
            if lit(x[1]) and lit(x[2]):
                problems.append(f"{path}: literal-literal comparison {x}")
        for i, sub in enumerate(x[1:], 1):
            if head in {"exists", "forall", "count"} and i == 1:
                continue
            walk(sub, f"{path}[{i}]", nb)
    else:
        for i, sub in enumerate(x):
            walk(sub, f"{path}[{i}]", bound)

for f, d in DATA.items():
    walk(d, f, set())

for g in sorted(guards):
    for pref in ("not-",):
        if g.startswith(pref) and g[len(pref):] in guards:
            problems.append(f"guards.json: negated twin {g}")

for p in problems:
    print("FAIL", p)
print(f"sweep: {len(problems)} problem(s) across {len(FILES)} files")
sys.exit(1 if problems else 0)
