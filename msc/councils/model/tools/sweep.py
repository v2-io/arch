#!/usr/bin/env python3
"""Mechanical acceptance sweep for the RONR-12 unified model.

Checks (each failure prints a line; exit 1 on any):
  1. Every ["guard", id, ...] reference resolves to guards.json (or a declared defined_in routing).
  2. Every ["adjudged", pred, ...] predicate is declared in adjudication.json.
  3. Every ["timer-open", id, ...] timer is defined in lifecycle.json.
  4. Every ["event-occurred", event, ...] event name is declared in vocabulary.json events.
  5. Operator closure: every list-expression head is in the closed operator set (INTERFACE.md).
  6. Lint: no comparison whose operands are both literals (always-true/false expressions).
  7. Guard polarity: no guard id starting with "not-" that twins a declared positive guard.
     ("no-..." names are legitimate positives — no-question-pending — and are not checked.)
  8. Guard binding resolvability: a guard call with explicit args must supply exactly its
     declared bindings; a no-arg call's declared bindings must each match an identically-named
     in-scope binding (quantifier variables, or names introduced by an enclosing "params"/
     "bindings" declaration, plus the ambient decision-point names).
  9. Bare-binder lint: inside a quantifier body, a bare string equal to an in-scope binder
     must be written ["var", ...].

Known limits (deliberate): no type checking, no evaluation, no arity checking beyond guard
args; a clean exit means identifier/binding hygiene, not correctness.

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
QUANTIFIERS = {"exists", "forall", "count"}
# names any decision point supplies (available_actions evaluation context)
AMBIENT = {"member"}

guards = {g["id"]: g for g in DATA["guards.json"]["guards"]}
preds = {p["id"] for p in DATA["adjudication.json"]["predicates"]}
timers = {t["id"] for t in DATA["lifecycle.json"]["timers"]["list"]}
events = set()
for grp in DATA["vocabulary.json"]["events"].values():
    if isinstance(grp, list):
        events |= {e["id"] for e in grp}

problems = []

def declared_names(d):
    """Names a dict introduces for its subtree via params/bindings declarations."""
    out = set()
    for key in ("params", "bindings"):
        v = d.get(key)
        if isinstance(v, list):
            out |= {s.split()[0].split("(")[0] for s in v if isinstance(s, str)}
    return out

def walk(x, path, scope, quant):
    if isinstance(x, dict):
        ns = scope | declared_names(x)
        for k, v in x.items():
            walk(v, f"{path}.{k}", ns, quant)
        return
    if not isinstance(x, list) or not x:
        return
    head = x[0]
    if isinstance(head, str) and head in OPERATORS:
        if head == "guard":
            gid = x[1] if len(x) > 1 else None
            g = guards.get(gid)
            if g is None:
                problems.append(f"{path}: unresolved guard {gid}")
            else:
                need = [b.split()[0].split("(")[0] for b in g.get("bindings", [])]
                args = x[2:]
                if args and len(args) != len(need):
                    problems.append(f"{path}: guard/{gid} takes {len(need)} binding(s) {need}, called with {len(args)} arg(s)")
                if not args and "defined_in" not in g:
                    missing = [b for b in need if b not in scope]
                    if missing:
                        problems.append(f"{path}: guard/{gid} bindings {missing} unresolvable in scope {sorted(scope)}")
        if head == "adjudged" and (len(x) < 2 or x[1] not in preds):
            problems.append(f"{path}: unresolved adjudged predicate {x[1] if len(x)>1 else '?'}")
        if head == "timer-open" and (len(x) < 2 or x[1] not in timers):
            problems.append(f"{path}: unresolved timer {x[1] if len(x)>1 else '?'}")
        if head == "event-occurred" and (len(x) < 2 or x[1] not in events):
            problems.append(f"{path}: undeclared event {x[1] if len(x)>1 else '?'}")
        nscope, nquant = set(scope), set(quant)
        if head in QUANTIFIERS and len(x) >= 2 and isinstance(x[1], str):
            nscope.add(x[1]); nquant.add(x[1])
        if head in COMPARISONS and len(x) == 3:
            def lit(o):
                if isinstance(o, list):
                    return False
                if isinstance(o, str):
                    return o not in scope
                return True
            if lit(x[1]) and lit(x[2]):
                problems.append(f"{path}: literal-literal comparison {x}")
        for i, sub in enumerate(x[1:], 1):
            if head in QUANTIFIERS and i == 1:
                continue
            if isinstance(sub, str) and sub in nquant and head not in ("var", "param"):
                problems.append(f"{path}[{i}]: bare binder '{sub}' — write [\"var\", \"{sub}\"]")
            walk(sub, f"{path}[{i}]", nscope, nquant)
    else:
        for i, sub in enumerate(x):
            walk(sub, f"{path}[{i}]", scope, quant)

for f, d in DATA.items():
    walk(d, f, set(AMBIENT), set())

for g in guards:
    if g.startswith("not-") and g[len("not-"):] in guards:
        problems.append(f"guards.json: negated twin {g}")

for p in problems:
    print("FAIL", p)
print(f"sweep: {len(problems)} problem(s) across {len(FILES)} files")
sys.exit(1 if problems else 0)
