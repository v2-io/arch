#!/usr/bin/env python3
"""check-charter.py — check a charter against the RONR-12 model (revision 2).

Usage:  python3 tools/check-charter.py charters/quick-council.json [more charters...]
Exit 0 iff every charter is WELL-FORMED AGAINST THE MODEL'S NAMESPACE AND TYPES. It evaluates
nothing: a clean exit never means any state is admissible, and it never means a prose rule is
right. (de-novo-feedback-1 §5 showed revision 1 passed a charter with seven deliberate breakages;
tools/mutant.py reproduces that mutant and this revision refuses it.)

Checks:
  1. configuration id exists in council.json
  2. every motion/action reference resolves (registry incl. aliases, actions.json, the configuration
     chain's add_actions, this charter's add_motion/add_action); nothing is both allowed and
     disabled/excluded; an object-to-X action is not allowed while X is disabled
  3. params: id in council.json (or declared here by declare_param); VALUE matches the declared type
     (enum members, boolean, number, the ?{...} record shape with the model's own keys)
  4. patches: the surface resolves to an existing path in a model file; a patch to an `expr` field
     must be an expression, not prose
  5. every expression (add_motion applicability rows, add_action exprs, patches): operator closure,
     guard/adjudged/timer/event ids resolve, guard bindings resolvable, quantifier vars as ["var"],
     ctx roots in schema.json (or declared in requires_schema), ctx.config params declared
  6. add_motion records carry the registry's SDC fields and an applicability row
  7. electronic_meeting_rules: covers 9:36's list plus informed_acts and duty_sequencing; every
     value a non-empty string (or non-empty object)
  8. logical_time: maps every vocabulary anchor, now/hour/minutes, and the calendar clock fields
     of schema.json (current_session, last_business_day_of_session, adjourned_meeting_set); values
     non-empty (null allowed only for next-regular-session)
  9. roll, chair, quorum (basis in enum; value <= roll size; a numeric value only with basis
     bylaws-specified), kind-specific shape (special: referred non-empty; standing: order_of_business)
Then it prints the resulting member-initiated action space.
"""
import json, sys, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
MODEL = os.path.normpath(os.path.join(HERE, "..", "..", "..", "model"))
def load(name):
    with open(os.path.join(MODEL, name)) as f: return json.load(f)
FILES = {n: load(n + ".json") for n in ("council","registry","actions","guards","adjudication","lifecycle","vocabulary","schema","protocol","precedence")}
council, registry, actions, guards, adjud, lifecycle, vocab, schema = (FILES[k] for k in ("council","registry","actions","guards","adjudication","lifecycle","vocabulary","schema"))

OPERATORS = {"and","or","not","if","=","!=","<","<=",">",">=","in","list","+","*","exists","forall","count",
             "param","var","ctx","attr","reg","frames","top","bottom","motion","class","erank","applied-to",
             "adheres-to","question","state","debatable","amendable","guard","adjudged","timer-open",
             "event-occurred","same-day","before","within-quarterly","anchor","now","rank-admissible","applicable"}
QUANT = {"exists","forall","count"}
GUARDS = {g["id"]: g for g in guards["guards"]}
PREDS = {p["id"] for p in adjud["predicates"]}
TIMERS = {t["id"] for t in lifecycle["timers"]["list"]}
EVENTS = {e["id"] for grp in vocab["events"].values() if isinstance(grp, list) for e in grp}
ANCHORS = set(vocab["temporal_anchors"]["values"])
PARAM_TYPES = {p["id"]: p["type"] for p in council["parameters"]["list"]}
CONFIGS = {c["id"]: c for c in council["configurations"]}
CTX_ROOTS = set(schema["context"].keys())
CLOCK_FIELDS = {"current_session","last_business_day_of_session","adjourned_meeting_set"}
QUORUM_BASES = {"bylaws-specified","enrolled-majority","attendees","registered-delegates-majority","present"}
IDENTITIES = {"harness-agent","harness-session","declared"}
NINE36 = {"equipment","presence_and_quorum","quorum_doubt","recognition","motions_in_writing","voting","malfunction","informed_acts","duty_sequencing"}
SDC = {"class","interrupt","second","debatable","amendable","vote","reconsider"}
MOTIONS = {m["id"] for grp in ("ranked_chain","incidental","bring_back") for m in registry[grp]["motions"]}
ALIASES = set(registry["aliases"]["entries"].keys())
BASE_ACTIONS = {a["id"]: a for a in actions["actions"]}

problems, notes = [], []
P = lambda m: problems.append(m)
N = lambda m: notes.append(m)

# ---------------------------------------------------------------- helpers ---
def config_chain(cid):
    chain = []
    while cid: c = CONFIGS[cid]; chain.append(c); cid = c.get("extends")
    return list(reversed(chain))

def chain_effects(chain):
    added, dis_a, dis_m = set(), set(), set()
    for c in chain:
        for op in c.get("overlay", []):
            if op["op"] == "add_action": added.add(op["action"]["id"])
            elif op["op"] == "disable_action": dis_a.add(op["action"])
            elif op["op"] == "disable_motion": dis_m.add(op["motion"])
    return added, dis_a, dis_m

def check_param_value(pid, val, where):
    t = PARAM_TYPES.get(pid)
    if t is None: return
    if t.startswith("enum "):
        members = [m.strip() for m in t[5:].split("|")]
        if val not in members: P(f"{where}: param {pid} value {val!r} not in enum {members}")
    elif t.startswith("boolean"):
        if not isinstance(val, bool): P(f"{where}: param {pid} must be boolean, got {val!r}")
    elif t.startswith("?{"):
        keys = {k.strip() for k in t[2:t.index("}")].split(",")}
        if val is not None and (not isinstance(val, dict) or set(val) - keys):
            P(f"{where}: param {pid} must be null or a record with keys ⊆ {sorted(keys)}, got {val!r}")
    elif t.startswith("?body reference"):
        if val is not None and not isinstance(val, str): P(f"{where}: param {pid} must be a reference string or null")
    elif "body_kind enum" in t:
        if val not in {"assembly","committee","committee-of-the-whole","quasi-committee","convention","mass-meeting"}: P(f"{where}: bad body_kind {val!r}")

def resolve_surface(surface):
    """'file.id.field' or 'lifecycle.transitions.from->to' → (exists, is_expr_field)."""
    parts = surface.split(".")
    f = parts[0]
    if f == "lifecycle" and len(parts) == 3 and parts[1] == "transitions" and "->" in parts[2]:
        fr, to = parts[2].split("->")
        ok = any(t["from"] == fr and to in t["to"] for t in lifecycle["transitions"]["list"])
        return ok, False
    if f == "actions" and len(parts) == 3:
        a = BASE_ACTIONS.get(parts[1]); return (a is not None and parts[2] in a), parts[2] == "expr"
    if f == "guards" and len(parts) == 3:
        g = GUARDS.get(parts[1]); return (g is not None and parts[2] in g), parts[2] == "expr"
    return False, False

def walk_expr(x, path, scope, in_quant, requires, declared_params):
    if not isinstance(x, list) or not x: return
    head = x[0]
    if not isinstance(head, str) or head not in OPERATORS:
        P(f"{path}: unknown operator {head!r}"); return
    if head == "guard":
        gid = x[1] if len(x) > 1 else None
        if gid not in GUARDS: P(f"{path}: unresolved guard {gid}")
        else:
            need = [b.split()[0] for b in GUARDS[gid].get("bindings", [])]; args = x[2:]
            if args and len(args) != len(need): P(f"{path}: guard {gid} expects {need}, got {len(args)} args")
            if not args:
                for b in need:
                    if b not in scope: P(f"{path}: guard {gid} binding {b} not in scope {sorted(scope)}")
    elif head == "adjudged" and x[1] not in PREDS: P(f"{path}: unresolved predicate {x[1]}")
    elif head == "timer-open" and x[1] not in TIMERS: P(f"{path}: unresolved timer {x[1]}")
    elif head == "event-occurred" and x[1] not in EVENTS: P(f"{path}: unresolved event {x[1]}")
    elif head == "anchor" and x[1] not in ANCHORS: P(f"{path}: unknown anchor {x[1]}")
    elif head == "ctx":
        root = x[1] if len(x) > 1 else None
        if root == "config":
            pid = x[2] if len(x) > 2 else None
            if pid not in PARAM_TYPES and pid not in declared_params: P(f"{path}: ctx.config.{pid} is neither a council.json param nor declared by declare_param")
        elif root not in CTX_ROOTS:
            ok = any(r.startswith(f"ctx.{root}") for r in requires)
            (N if ok else P)(f"{path}: ctx root {root!r} not in schema.json context" + (" (declared schema extension)" if ok else ""))
        return
    elif head == "param":
        if x[1] not in scope: P(f"{path}: param {x[1]} not declared in scope {sorted(scope)}")
        return
    if head in QUANT:
        v = x[1]; walk_expr(x[2], path+f".{head}L", scope, in_quant, requires, declared_params)
        walk_expr(x[3], path+f".{head}P", scope | {v}, in_quant | {v}, requires, declared_params); return
    for i, sub in enumerate(x[1:], 1):
        if isinstance(sub, str) and sub in in_quant and head != "var": P(f"{path}[{i}]: bare binder {sub!r} inside quantifier — write [\"var\", ...]")
        walk_expr(sub, path+f"[{i}]", scope, in_quant, requires, declared_params)

# ------------------------------------------------------------------ check ---
def check(path):
    global problems, notes
    problems, notes = [], []
    ch = json.load(open(path))
    cid = ch.get("configuration")
    if cid not in CONFIGS: P(f"configuration {cid!r} not in council.json"); return report(path, ch, None)
    chain = config_chain(cid)
    chain_added, chain_dis_a, chain_dis_m = chain_effects(chain)
    ops = ch.get("overlay", {}).get("ops", [])
    declared = {op["param"] for op in ops if op["op"] == "declare_param"}
    my_motions = {op["motion"]["id"]: op["motion"] for op in ops if op["op"] == "add_motion"}
    my_actions = {op["action"]["id"]: op["action"] for op in ops if op["op"] == "add_action"}
    my_dis_a = {op["action"] for op in ops if op["op"] == "disable_action"}
    my_dis_m = {op["motion"] for op in ops if op["op"] == "disable_motion"}
    all_actions = set(BASE_ACTIONS) | chain_added | set(my_actions)
    all_motions = MOTIONS | ALIASES | set(my_motions)

    def resolve(ref, where):
        kind, _, ident = ref.partition("/")
        if kind == "motion" and ident not in all_motions: P(f"{where}: unknown motion {ref}")
        elif kind == "action" and ident not in all_actions: P(f"{where}: unknown action {ref}")
        elif kind not in ("motion","action"): P(f"{where}: reference kind {kind!r} not motion/action")

    # ops
    for i, op in enumerate(ops):
        w = f"overlay[{i}] {op['op']}"
        if op["op"] == "declare_param":
            if op["param"] in PARAM_TYPES: P(f"{w}: {op['param']} shadows a council.json param")
            if op.get("type") == "number" and not isinstance(op.get("value"), (int, float)): P(f"{w}: {op['param']} declared number, value {op.get('value')!r}")
        elif op["op"] == "param":
            if op["param"] not in PARAM_TYPES and op["param"] not in declared: P(f"{w}: param {op['param']} unknown")
            else: check_param_value(op["param"], op.get("value"), w)
        elif op["op"] == "disable_action":
            if op["action"] not in all_actions: P(f"{w}: {op['action']} unknown")
        elif op["op"] == "disable_motion":
            if op["motion"] not in all_motions: P(f"{w}: {op['motion']} unknown")
        elif op["op"] == "patch" and "add" in op:
            if op.get("surface") != "lifecycle.transitions": P(f"{w}: 'add' is only defined for surface lifecycle.transitions")
            else:
                t = op["add"]; states = {st["id"] for st in lifecycle["states"]}
                if t.get("from") not in states or t.get("to") not in states: P(f"{w}: added transition names unknown state(s) {t}")
                if t.get("on") not in EVENTS: P(f"{w}: added transition event {t.get('on')!r} not in vocabulary")
        elif op["op"] == "patch":
            ok, is_expr = resolve_surface(op.get("surface",""))
            if not ok: P(f"{w}: surface {op.get('surface')!r} does not resolve to a model path")
            rep = op.get("replace")
            if is_expr:
                if not isinstance(rep, list): P(f"{w}: patch to an expr field must be an expression, got {type(rep).__name__} (prose?)")
                else: walk_expr(rep, f"{w}.replace", {"member","q","m","t","claimant","item","kind","breach","f","time","series"}, set(), [], declared)
            elif isinstance(rep, str) and len(rep.split()) > 6: N(f"{w}: prose replacement on a non-expr surface — annotation-grade, not compiled")
        elif op["op"] == "add_action":
            a = op["action"]; scope = {"member"} | {p.split()[0].split("(")[0] for p in a.get("params", [])}
            walk_expr(a["expr"], f"{w}.{a['id']}.expr", scope, set(), a.get("requires_schema", []), declared)
            N(f"{w}: add_action {a['id']} bypasses action/move's conjuncts (quorum, floor, rank, dilatory, governing rules) — prefer add_motion (feedback §4)")
        elif op["op"] == "add_motion":
            m = op["motion"]; missing = SDC - set(m)
            if missing: P(f"{w}: add_motion {m['id']} lacks SDC fields {sorted(missing)}")
            if m["id"] in MOTIONS: P(f"{w}: add_motion {m['id']} collides with a registry motion")
            app = m.get("applicability")
            if not app or "applies_to" not in app: P(f"{w}: add_motion {m['id']} has no applicability row (move cannot reach it)")
            else:
                scope = {"member","m","t"} | {p.split()[0].split("(")[0] for p in m.get("params", [])}
                walk_expr(app["applies_to"].get("expr"), f"{w}.{m['id']}.applies_to", scope, set(), m.get("requires_schema", []), declared)
                for j, row in enumerate(app.get("out_of_order_when", [])):
                    walk_expr(row.get("expr"), f"{w}.{m['id']}.ooo[{j}]", scope, set(), m.get("requires_schema", []), declared)
        elif op["op"] == "restrict_actions": pass
        else: P(f"{w}: unknown overlay op")

    # restrict_actions
    ra = [op for op in ops if op["op"] == "restrict_actions"]
    allowed_m, allowed_a = set(), set()
    if ra:
        ra = ra[0]
        if ra.get("scope") != "member-initiated": P("restrict_actions must declare scope: member-initiated (model-defects F1)")
        for ref in ra["allowed"]: resolve(ref, "restrict_actions.allowed"); allowed_m.add(ref.split("/",1)[1])
        for ref in ra.get("plus", []): resolve(ref, "restrict_actions.plus"); allowed_a.add(ref.split("/",1)[1])
        for ref in ra.get("excluded_with_reason", {}):
            resolve(ref, "restrict_actions.excluded_with_reason"); k, i = ref.split("/",1)
            if (k == "motion" and i in allowed_m) or (k == "action" and i in allowed_a): P(f"{ref} both allowed and excluded")
        for m in allowed_m & (chain_dis_m | my_dis_m): P(f"motion/{m} both allowed and disabled")
        for a in allowed_a & (chain_dis_a | my_dis_a): P(f"action/{a} both allowed and disabled")
        if "object-to-standing-at-ease" in allowed_a and "stand-at-ease" in chain_dis_a | my_dis_a: P("action/object-to-standing-at-ease allowed while stand-at-ease is disabled")
        if "object-to-unanimous-consent" not in allowed_a: P("rights-critical action/object-to-unanimous-consent not whitelisted (model-defects F2)")
        if "move" not in allowed_a: P("action/move not whitelisted: no motion can be made")
        for m in sorted(MOTIONS):
            if m not in allowed_m and f"motion/{m}" not in ra.get("excluded_with_reason", {}) and m not in chain_dis_m | my_dis_m:
                N(f"motion/{m} silently excluded")
        for a, rec in sorted(BASE_ACTIONS.items()):
            if rec["roles"] in (["member"], ["maker"]) and a not in allowed_a and f"action/{a}" not in ra.get("excluded_with_reason", {}) and a not in chain_dis_a | my_dis_a:
                N(f"action/{a} (member-initiated) silently excluded")
        for m in my_motions:
            if m not in allowed_m: N(f"add_motion {m} is not in the whitelist")
    # 9:36
    emr = ch.get("electronic_meeting_rules", {})
    for k in sorted(NINE36 - set(emr)): P(f"electronic_meeting_rules missing {k}")
    for k, v in emr.items():
        if k == "cite": continue
        if (isinstance(v, str) and not v.strip()) or (isinstance(v, dict) and not v): P(f"electronic_meeting_rules.{k} is empty")
    # logical time
    lt = ch.get("logical_time", {})
    for a in sorted(ANCHORS | {"now","hour","minutes"} | CLOCK_FIELDS):
        if a not in lt: P(f"logical_time does not map {a!r}")
        elif lt[a] is None and a != "next-regular-session": P(f"logical_time.{a} is null")
        elif isinstance(lt[a], str) and not lt[a].strip(): P(f"logical_time.{a} is empty")
    # roll / chair / quorum
    seats = ch.get("roll", {}).get("seats", []); ids = [s["seat"] for s in seats]
    if len(ids) != len(set(ids)): P("duplicate seat ids")
    for s in seats:
        if not s.get("perspective"): P(f"{s['seat']}: no perspective")
        if s.get("agent", {}).get("identity") not in IDENTITIES: P(f"{s['seat']}: agent.identity not in {sorted(IDENTITIES)}")
    chair = ch.get("officers", {}).get("chair")
    if chair not in ("mechanical","elected") and chair not in ids: P(f"chair {chair!r} is neither mechanical, elected, nor a seat")
    if chair == "mechanical":
        cp = [op for op in ops if op["op"] == "param" and op["param"] == "chair_participates"]
        if not cp or cp[0]["value"] is not False: P("mechanical chair but chair_participates not set false (small-board inherits true)")
    q = ch.get("quorum", {})
    if q.get("basis") not in QUORUM_BASES: P(f"quorum basis {q.get('basis')!r} not in enum")
    if q.get("value") is not None:
        if q["basis"] != "bylaws-specified": P("a numeric quorum value requires basis bylaws-specified")
        if q["value"] > len(ids): P("quorum exceeds roll size")
        if q["value"] == len(ids) and len(ids) > 1: N("quorum equals the whole roll: every seat can block by silence under the backstop (feedback §6)")
    kind = ch.get("kind")
    if kind == "special":
        if not ch.get("referred"): P("special council with nothing referred")
        if "DERIVED_from_kind" not in ch.get("dissolution", {}) and not any("final report" in d.get("event","") for d in ch.get("dissolution",{}).get("on",[])): P("special council lacks final-report dissolution [50:10]")
    elif kind == "standing":
        if not ch.get("agenda", {}).get("order_of_business"): P("standing council lacks an order_of_business")
    else: P(f"kind {kind!r} not special|standing")
    return report(path, ch, (chain, chain_dis_m | my_dis_m, chain_dis_a | my_dis_a, allowed_m, allowed_a, set(my_motions) | set(my_actions)))

def report(path, ch, space):
    print(f"\n### {os.path.relpath(path)}  ({ch.get('id')})")
    for p in problems: print("  PROBLEM:", p)
    for n in notes: print("  note:", n)
    if space:
        chain, dis_m, dis_a, am, aa, added = space
        print(f"  configuration chain (as declared by the charter): {' -> '.join(c['id'] for c in chain)}")
        print(f"  member-initiated motions allowed ({len(am)}): {', '.join(sorted(am))}")
        print(f"  member-initiated actions allowed ({len(aa)}): {', '.join(sorted(aa))}")
        print(f"  added by this charter ({len(added)}): {', '.join(sorted(added))}")
        print(f"  disabled by chain or charter: motions {sorted(dis_m)}; actions {sorted(dis_a)}")
        chair_acts = [a for a, r in BASE_ACTIONS.items() if 'chair' in r['roles'] and a not in dis_a]
        print(f"  chair duties (never whitelisted; performed by {ch.get('officers',{}).get('chair')}): {', '.join(chair_acts)}")
        print(f"  registry motions excluded: {len(MOTIONS - am)} of {len(MOTIONS)}")
    print("  RESULT:", "well-formed against the model's namespace and types" if not problems else f"{len(problems)} problem(s)")
    return not problems

if __name__ == "__main__":
    ok = all([check(p) for p in sys.argv[1:]])
    sys.exit(0 if ok else 1)
