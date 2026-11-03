#!/usr/bin/env python3
"""check-charter.py — resolve every reference a charter makes against the RONR-12 model.

Usage:  python3 tools/check-charter.py charters/quick-council.json [more charters...]

What it checks (load-time closure only — it evaluates nothing, so a clean exit means the
charter is WELL-FORMED against the model, not that any state is admissible):
  1. configuration id exists in council.json
  2. every restrict_actions / disable_* / excluded id resolves: motion/<id> to registry.json
     (ranked chain, incidentals, bring-back, aliases), action/<id> to actions.json or to an
     add_action in the configuration chain or in this charter
  3. every param id resolves to council.json parameters
  4. every add_action expr: operator closure (INTERFACE.md), guard/adjudged/timer/event ids resolve,
     quantifier variables written as ["var", ...], ctx paths rooted in schema.json context keys
     (unknown roots are reported as SCHEMA EXTENSIONS, not errors, when the action declares them
     in requires_schema)
  5. electronic_meeting_rules covers the 9:36 list
  6. logical_time maps every temporal anchor in vocabulary.json plus now/hour/minutes
  7. roll: unique seats, each with perspective and agent.identity in the vsect set; chair valid
  8. quorum basis in the council.json enum; value <= roll size
  9. kind-specific: special has >= 1 referred question and a final-report dissolution;
     standing has an order_of_business
Then it prints the member-initiated action space that results (allowed motions, allowed actions,
what the configuration chain already disabled, what this charter excluded), so the narrowing is
visible and verifiable by hand against the whitelist.
"""
import json, sys, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
MODEL = os.path.normpath(os.path.join(HERE, "..", "..", "..", "model"))

def load(name):
    with open(os.path.join(MODEL, name)) as f:
        return json.load(f)

council = load("council.json"); registry = load("registry.json"); actions = load("actions.json")
guards = load("guards.json"); adjud = load("adjudication.json"); lifecycle = load("lifecycle.json")
vocab = load("vocabulary.json"); schema = load("schema.json")

OPERATORS = {"and","or","not","if","=","!=","<","<=",">",">=","in","list","+","*","exists","forall","count",
             "param","var","ctx","attr","reg","frames","top","bottom","motion","class","erank","applied-to",
             "adheres-to","question","state","debatable","amendable","guard","adjudged","timer-open",
             "event-occurred","same-day","before","within-quarterly","anchor","now","rank-admissible","applicable"}
QUANT = {"exists","forall","count"}

GUARDS = {g["id"] for g in guards["guards"]}
PREDS = {p["id"] for p in adjud["predicates"]}
TIMERS = {t["id"] for t in lifecycle["timers"]["list"]}
EVENTS = {e["id"] for grp in vocab["events"].values() if isinstance(grp, list) for e in grp}
ANCHORS = set(vocab["temporal_anchors"]["values"])
PARAMS = {p["id"] for p in council["parameters"]["list"]}
CONFIGS = {c["id"]: c for c in council["configurations"]}
CTX_ROOTS = set(schema["context"].keys())
QUORUM_BASES = {"bylaws-specified","enrolled-majority","attendees","registered-delegates-majority","present"}
IDENTITIES = {"harness-agent","harness-session","declared"}
NINE36 = {"presence_and_quorum","quorum_doubt","recognition","motions_in_writing","voting","malfunction"}

MOTIONS = set()
for grp in ("ranked_chain","incidental","bring_back"):
    MOTIONS |= {m["id"] for m in registry[grp]["motions"]}
ALIASES = set(registry["aliases"]["entries"].keys())
BASE_ACTIONS = {a["id"] for a in actions["actions"]}

def config_chain(cid):
    chain = []
    while cid:
        c = CONFIGS[cid]; chain.append(c); cid = c.get("extends")
    return list(reversed(chain))

def chain_effects(chain):
    added, disabled_actions, disabled_motions = set(), set(), set()
    for c in chain:
        for op in c.get("overlay", []):
            if op["op"] == "add_action": added.add(op["action"]["id"])
            elif op["op"] == "disable_action": disabled_actions.add(op["action"])
            elif op["op"] == "disable_motion": disabled_motions.add(op["motion"])
    return added, disabled_actions, disabled_motions

problems, notes = [], []
def P(msg): problems.append(msg)
def N(msg): notes.append(msg)

def walk_expr(x, path, scope, in_quant, requires):
    if not isinstance(x, list) or not x: return
    head = x[0]
    if not isinstance(head, str) or head not in OPERATORS:
        P(f"{path}: unknown operator {head!r}"); return
    if head == "guard":
        gid = x[1] if len(x) > 1 else None
        if gid not in GUARDS: P(f"{path}: unresolved guard {gid}")
        else:
            g = next(g for g in guards["guards"] if g["id"] == gid)
            need = [b.split()[0] for b in g.get("bindings", [])]
            args = x[2:]
            if args and len(args) != len(need): P(f"{path}: guard {gid} expects {need}, got {len(args)} args")
            if not args:
                for b in need:
                    if b not in scope: P(f"{path}: guard {gid} binding {b} not in scope {sorted(scope)}")
    elif head == "adjudged":
        if x[1] not in PREDS: P(f"{path}: unresolved predicate {x[1]}")
    elif head == "timer-open":
        if x[1] not in TIMERS: P(f"{path}: unresolved timer {x[1]}")
    elif head == "event-occurred":
        if x[1] not in EVENTS: P(f"{path}: unresolved event {x[1]}")
    elif head == "ctx":
        root = x[1] if len(x) > 1 else None
        if root not in CTX_ROOTS:
            ok = any(r.startswith(f"ctx.{root}") for r in requires)
            (N if ok else P)(f"{path}: ctx root {root!r} is not in schema.json context" + (" (declared schema extension)" if ok else ""))
        return
    elif head == "param":
        if x[1] not in scope: P(f"{path}: param {x[1]} not declared in scope {sorted(scope)}")
        return
    elif head == "anchor":
        if x[1] not in ANCHORS: P(f"{path}: unknown anchor {x[1]}")
    if head in QUANT:
        v = x[1]; walk_expr(x[2], path+f".{head}L", scope, in_quant, requires)
        walk_expr(x[3], path+f".{head}P", scope | {v}, in_quant | {v}, requires); return
    for i, sub in enumerate(x[1:], 1):
        if isinstance(sub, str) and sub in in_quant and head not in ("var",):
            P(f"{path}[{i}]: bare binder {sub!r} inside quantifier — write [\"var\", ...]")
        walk_expr(sub, path+f"[{i}]", scope, in_quant, requires)

def check(path):
    global problems, notes
    problems, notes = [], []
    ch = json.load(open(path))
    cid = ch.get("configuration")
    if cid not in CONFIGS: P(f"configuration {cid!r} not in council.json"); return report(path, ch, None)
    chain = config_chain(cid)
    chain_added, chain_dis_actions, chain_dis_motions = chain_effects(chain)
    ops = ch.get("overlay", {}).get("ops", [])
    my_added = {op["action"]["id"]: op["action"] for op in ops if op["op"] == "add_action"}
    my_dis_actions = {op["action"] for op in ops if op["op"] == "disable_action"}
    my_dis_motions = {op["motion"] for op in ops if op["op"] == "disable_motion"}
    all_actions = BASE_ACTIONS | chain_added | set(my_added)

    def resolve(ref, where):
        kind, _, ident = ref.partition("/")
        if kind == "motion":
            if ident not in MOTIONS | ALIASES: P(f"{where}: unknown motion {ref}")
        elif kind == "action":
            if ident not in all_actions: P(f"{where}: unknown action {ref}")
        else: P(f"{where}: reference kind {kind!r} not motion/action")

    # params
    for op in ops:
        if op["op"] == "param" and op["param"] not in PARAMS: P(f"overlay: param {op['param']} not in council.json parameters")
        if op["op"] == "disable_action" and op["action"] not in all_actions: P(f"overlay: disable_action {op['action']} unknown")
        if op["op"] == "disable_motion" and op["motion"] not in MOTIONS: P(f"overlay: disable_motion {op['motion']} unknown")
    # restrict_actions
    ra = [op for op in ops if op["op"] == "restrict_actions"]
    allowed_m, allowed_a = set(), set()
    if ra:
        ra = ra[0]
        if ra.get("scope") != "member-initiated": N("restrict_actions has no explicit scope; the model's op is ambiguous about chair duties (model-defects.md F1)")
        for ref in ra["allowed"]:
            resolve(ref, "restrict_actions.allowed"); allowed_m.add(ref.split("/",1)[1])
        for ref in ra.get("plus", []):
            resolve(ref, "restrict_actions.plus"); allowed_a.add(ref.split("/",1)[1])
        for ref in ra.get("excluded_with_reason", {}):
            resolve(ref, "restrict_actions.excluded_with_reason")
            k, i = ref.split("/",1)
            if (k == "motion" and i in allowed_m) or (k == "action" and i in allowed_a): P(f"{ref} both allowed and excluded")
        # every registry motion is either allowed, excluded-with-reason, or chain-disabled
        for m in sorted(MOTIONS):
            if m not in allowed_m and f"motion/{m}" not in ra.get("excluded_with_reason", {}) and m not in chain_dis_motions | my_dis_motions:
                N(f"motion/{m} silently excluded (not allowed, no reason given, not disabled by the configuration chain)")
        for a in sorted(BASE_ACTIONS):
            roles = next(x for x in actions["actions"] if x["id"] == a)["roles"]
            if roles == ["member"] or roles == ["maker"]:
                if a not in allowed_a and f"action/{a}" not in ra.get("excluded_with_reason", {}) and a not in chain_dis_actions | my_dis_actions:
                    N(f"action/{a} (member-initiated) silently excluded")
    # add_action exprs
    for aid, a in my_added.items():
        scope = {"member"} | {p.split()[0].split("(")[0] for p in a.get("params", [])}
        walk_expr(a["expr"], f"add_action.{aid}.expr", scope, set(), a.get("requires_schema", []))
        for key in ("vote","second","debatable","amendable"):
            if key not in a: N(f"add_action.{aid}: no {key} declared (the model's add_action schema has no SDC fields — model-defects.md F3)")
    # 9:36
    emr = ch.get("electronic_meeting_rules", {})
    missing = NINE36 - set(emr.keys())
    if missing: P(f"electronic_meeting_rules missing 9:36 items: {sorted(missing)}")
    # logical time
    lt = ch.get("logical_time", {})
    for a in sorted(ANCHORS | {"now","hour","minutes"}):
        if a not in lt: P(f"logical_time does not map anchor {a!r}")
    # roll
    seats = ch.get("roll", {}).get("seats", [])
    ids = [s["seat"] for s in seats]
    if len(ids) != len(set(ids)): P("duplicate seat ids")
    for s in seats:
        if not s.get("perspective"): P(f"{s['seat']}: no perspective")
        if s.get("agent", {}).get("identity") not in IDENTITIES: P(f"{s['seat']}: agent.identity not in {sorted(IDENTITIES)}")
    chair = ch.get("officers", {}).get("chair")
    if chair not in ("mechanical","elected") and chair not in ids: P(f"chair {chair!r} is neither mechanical, elected, nor a seat")
    # quorum
    q = ch.get("quorum", {})
    if q.get("basis") not in QUORUM_BASES: P(f"quorum basis {q.get('basis')!r} not in enum")
    if q.get("value") is not None and q["value"] > len(ids): P("quorum exceeds roll size")
    # kind
    kind = ch.get("kind")
    if kind == "special":
        if not ch.get("referred"): P("special council with nothing referred")
        if not any("final report" in d.get("event","") for d in ch.get("dissolution",{}).get("on",[])): P("special council lacks final-report dissolution [50:10]")
        if "motion/main-motion" in allowed_m: N("special council allows main-motion: new business in a special committee needs a reason")
    elif kind == "standing":
        if not ch.get("agenda", {}).get("order_of_business"): P("standing council lacks an order_of_business")
    else: P(f"kind {kind!r} not special|standing")
    return report(path, ch, (chain, chain_dis_motions | my_dis_motions, chain_dis_actions | my_dis_actions, allowed_m, allowed_a, my_added))

def report(path, ch, space):
    print(f"\n### {os.path.relpath(path)}  ({ch.get('id')})")
    for p in problems: print("  PROBLEM:", p)
    for n in notes: print("  note:", n)
    if space:
        chain, dis_m, dis_a, am, aa, added = space
        print(f"  configuration chain: {' -> '.join(c['id'] for c in chain)}")
        print(f"  member-initiated motions allowed ({len(am)}): {', '.join(sorted(am))}")
        print(f"  member-initiated actions allowed ({len(aa)}): {', '.join(sorted(aa))}")
        print(f"  added by this charter ({len(added)}): {', '.join(sorted(added))}")
        print(f"  disabled by configuration chain or charter: motions {sorted(dis_m)}; actions {sorted(dis_a)}")
        chair_acts = [a['id'] for a in actions['actions'] if 'chair' in a['roles'] and a['id'] not in dis_a]
        sec_acts = [a['id'] for a in actions['actions'] if 'secretary' in a['roles'] or 'reporting-member' in a['roles']]
        print(f"  chair duties (never whitelisted; performed by {ch.get('officers',{}).get('chair')}): {', '.join(chair_acts)}")
        print(f"  secretary / reporting-member acts: {', '.join(sec_acts)}")
        print(f"  registry motions excluded: {len(MOTIONS - am)} of {len(MOTIONS)}")
    print("  RESULT:", "well-formed" if not problems else f"{len(problems)} problem(s)")
    return not problems

if __name__ == "__main__":
    ok = all([check(p) for p in sys.argv[1:]])
    sys.exit(0 if ok else 1)
