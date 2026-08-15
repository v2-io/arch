#!/usr/bin/env python3
"""vsl -- a first executable cut of RC1's declarative language (09-LANGUAGE).

Register: spike / ground-validation. "vsl" is a working name. stdlib only.

What this implements, from the model:
  declarations : dimension / kind / record / rule / era
  acts         : event (append-only), era-bump, checkpoint (a read, not a write)
  projections  : per-dimension position, strongest-leg, floor, lock -- always
                 COMPUTED from the trail; there is no syntax to set a status.
  guards       : rules gate lifts (e.g. ratified requires grounds + consult);
                 falsifier invalidation reopens a decision with what-changed.
  decay        : era-keyed events expire visibly on era-bump; a fresh committed
                 event re-arms.
  lints        : fusion (a ladder whose rungs are cross-products of two declared
                 dimensions' vocabularies is rejected); allow-targets must be in
                 the target dimension's own ladder (substitution has no syntax);
                 `status:` anywhere is a parse error by design.

Order-invariance: per the model's carve, RECORDING is order-invariant -- all
projections are computed as maxima/minima over the event multiset, verified by
--shuffle -- while guard-checking at append time is legitimately order-
sensitive (economy is not a race condition; the ledger is).
"""
import sys, random

class Dimension:
    def __init__(s, name):
        s.name = name; s.rungs = []; s.channels = {}; s.refutes = set()
        s.family = None; s.ceiling = ''; s.strong_at = None
    def rung_index(s, rung): return s.rungs.index(rung) if rung in s.rungs else -1

class Kind:
    def __init__(s, name):
        s.name = name; s.speech_act = ''; s.write = ''; s.opens = {}  # foundation->dim

class Record:
    def __init__(s, name):
        s.name = name; s.kind = None; s.foundation = []
        s.assumptions = set(); s.falsifiers = set(); s.reopened = False

class Rule:
    def __init__(s, name):
        s.name = name; s.when = []   # list of (dim, min_rung)
        s.allow = None               # (dim, rung)

class Event:
    def __init__(s, eid, date, record, dim, channel, attrs):
        s.eid, s.date, s.record, s.dim, s.channel, s.attrs = eid, date, record, dim, channel, attrs

class VSLError(Exception): pass

class Model:
    def __init__(s):
        s.dims = {}; s.kinds = {}; s.records = {}; s.rules = []
        s.eras = {}; s.trail = []; s.log = []

    # ---------- lints ----------
    def lint_fusion(s, dim):
        """Reject a ladder whose rungs are concatenations of EXACT rungs from
        two different already-declared dimensions (the unrolled-permutation
        tell: `ruled-assumptions-explicit` etc.). Shared single tokens are fine;
        cross-products of independently-movable values are not."""
        owners = {}  # exact rung name -> set of dims that declare it
        for d in s.dims.values():
            for r in d.rungs:
                owners.setdefault(r, set()).add(d.name)
        for rung in dim.rungs:
            toks = rung.split('-')
            for cut in range(1, len(toks)):
                a, b = '-'.join(toks[:cut]), '-'.join(toks[cut:])
                for da in owners.get(a, ()):  # noqa
                    for db in owners.get(b, ()):  # noqa
                        if da != db:
                            raise VSLError(
                                f"FUSION: dimension '{dim.name}' rung '{rung}' is "
                                f"'{a}' ({da}) x '{b}' ({db}) unrolled longhand -- "
                                f"two independently-movable dimensions are sharing a "
                                f"field; split them.")

    def lint_rule(s, rule):
        dim, rung = rule.allow
        if dim not in s.dims:
            raise VSLError(f"rule '{rule.name}': unknown dimension '{dim}'")
        if s.dims[dim].rung_index(rung) < 0:
            raise VSLError(
                f"SUBSTITUTION-SHAPED: rule '{rule.name}' tries to allow "
                f"'{rung}' on {dim}, which is not in that dimension's own ladder -- "
                f"no rule may place another dimension's value here.")

    # ---------- projections (computed, order-invariant) ----------
    def expired(s, ev):
        era = ev.attrs.get('era')
        if not era: return False
        key, val = era.split('@')
        return s.eras.get(key) != val

    def position(s, rec, dim):
        """Highest rung reached via non-expired channel events; refutation notes
        and decline notes carried beside, never as the position."""
        d = s.dims[dim]; idx = 0; notes = []
        for ev in s.trail:
            if ev.record != rec.name or ev.dim != dim: continue
            if ev.channel in d.refutes:
                notes.append(f"{ev.channel}: {ev.attrs.get('note','(refutation)')}")
                continue
            if ev.attrs.get('outcome') == 'declined-lift':
                notes.append(f"declined-lift: {ev.attrs.get('reason','(reason owed)')}")
                continue
            tgt = d.channels.get(ev.channel)
            if tgt is None: continue
            if s.expired(ev):
                notes.append(f"expired@{ev.attrs['era']}: re-run pending"); continue
            idx = max(idx, d.rung_index(tgt))
        return idx, notes

    def base_family(s, rec, dim):
        fam = s.dims[dim].family or dim
        return fam.split(':')[-1]  # transmission:<src> -> look through the wrapper

    def project(s, rec, label):
        out = [f"-- checkpoint [{label}] record={rec.name} (all values computed; none stored) --"]
        dims = [d for f, d in s.kinds[rec.kind].opens.items() if f in rec.foundation]
        positions = {}
        for dim in dims:
            idx, notes = s.position(rec, dim)
            positions[dim] = idx
            d = s.dims[dim]
            rung = d.rungs[idx] if d.rungs else '?'
            note = ('  [' + '; '.join(notes) + ']') if notes else ''
            out.append(f"   {dim:28s} {rung}{note}")
        acted = {d: i for d, i in positions.items() if i > 0 or any(
            ev.record == rec.name and ev.dim == d for ev in s.trail)}
        if positions:
            strongest = max(positions, key=lambda d: positions[d])
            floor = min(acted, key=lambda d: acted[d]) if acted else strongest
            out.append(f"   strongest-line: {strongest} @ {s.dims[strongest].rungs[positions[strongest]]}")
            out.append(f"   floor:         {floor} @ {s.dims[floor].rungs[acted[floor]]}")
            strong = [d for d, i in positions.items()
                      if s.dims[d].strong_at is not None
                      and i >= s.dims[d].rung_index(s.dims[d].strong_at)]
            fams = {s.base_family(rec, d) for d in strong}
            lock = 'ARMED' if len(fams) >= 2 else 'not armed'
            out.append(f"   lock:          {lock}  (counting lines: {sorted(strong)}; base families: {sorted(fams)})")
        if rec.reopened:
            out.append(f"   STATUS: reopened -- {rec.reopen_note}")
        s.log.extend(out)

    # ---------- acts ----------
    def append_event(s, ev):
        d = s.dims.get(ev.dim)
        rec = s.records[ev.record]
        # falsifier invalidation (works on any dimension's arrival)
        inval = ev.attrs.get('invalidates')
        if inval:
            if inval in rec.assumptions and inval in rec.falsifiers:
                rec.reopened = True
                rec.reopen_note = (f"what-changed: {ev.attrs.get('note', ev.channel)} -- "
                                   f"assumption '{inval}' no longer holds; verify the model of WHY "
                                   f"it was decided, then decide together")
                s.log.append(f"   !! falsifier fired on {rec.name}: {rec.reopen_note}")
        if d is None:
            s.trail.append(ev); return
        # guard: does some rule govern lifting to this channel's target?
        tgt = d.channels.get(ev.channel)
        if tgt is not None:
            for rule in s.rules:
                if rule.allow == (ev.dim, tgt):
                    for (cdim, crung) in rule.when:
                        idx, _ = s.position(rec, cdim)
                        need = s.dims[cdim].rung_index(crung)
                        if idx < need:
                            s.log.append(
                                f"   !! GUARD [{rule.name}]: refused '{ev.channel}' on {ev.dim} "
                                f"-- requires {cdim} >= {crung} (currently "
                                f"{s.dims[cdim].rungs[idx]})")
                            return
        if ev.attrs.get('assumptions'):
            rec.assumptions |= set(ev.attrs['assumptions'].split(','))
        if ev.attrs.get('triggers'):
            rec.falsifiers |= set(ev.attrs['triggers'].split(','))
        s.trail.append(ev)

# ---------- parser ----------
def parse(path, model):
    cur = None
    for lineno, raw in enumerate(open(path), 1):
        line = raw.rstrip()
        if not line or line.lstrip().startswith('#'): continue
        if 'status:' in line.replace('era-bump', ''):
            raise VSLError(f"line {lineno}: 'status:' has no syntax here BY DESIGN -- "
                           "every status is a projection over the trail.")
        indented = line.startswith((' ', '\t')); parts = line.split()
        if not indented:
            kw = parts[0]
            if kw == 'dimension':
                cur = Dimension(parts[1])
            elif kw == 'kind':
                cur = Kind(parts[1]); model.kinds[cur.name] = cur
            elif kw == 'record':
                cur = Record(parts[1]); model.records[cur.name] = cur
            elif kw == 'rule':
                cur = Rule(parts[1]); model.rules.append(cur)
            elif kw == 'era':
                model.eras[parts[1]] = parts[2]; cur = None
            elif kw == 'era-bump':
                old = model.eras.get(parts[1]); model.eras[parts[1]] = parts[2]; cur = None
                model.log.append(f"   == era-bump {parts[1]}: {old} -> {parts[2]} "
                                 f"(era-keyed certificates now expire, visibly)")
            elif kw == 'end-dimension':
                model.lint_fusion(cur); model.dims[cur.name] = cur; cur = None
            elif kw == 'end-rule':
                model.lint_rule(cur); cur = None
            elif kw == 'event':
                eid, date, rec, dim, channel = parts[1:6]
                attrs = dict(p.split('=', 1) for p in parts[6:])
                model.append_event(Event(eid, date, rec, dim, channel, attrs))
            elif kw == 'checkpoint':
                model.project(model.records[parts[2]], parts[1])
            else:
                raise VSLError(f"line {lineno}: unknown form '{kw}'")
            continue
        key, _, val = line.strip().partition(': ')
        if isinstance(cur, Dimension):
            if key == 'rungs': cur.rungs = val.split()
            elif key == 'channels':
                cur.channels = dict(c.split('->') for c in val.split())
            elif key == 'refutes': cur.refutes = set(val.split())
            elif key == 'family': cur.family = val
            elif key == 'ceiling': cur.ceiling = val
            elif key == 'strong-at': cur.strong_at = val
        elif isinstance(cur, Kind):
            if key == 'speech-act': cur.speech_act = val
            elif key == 'write': cur.write = val
            elif key == 'opens':
                cur.opens = dict(c.split('->') for c in val.split())
        elif isinstance(cur, Record):
            if key == 'kind': cur.kind = val
            elif key == 'foundation': cur.foundation = val.split()
        elif isinstance(cur, Rule):
            if key == 'when':
                cur.when = [tuple(c.split('>=')) for c in val.split()]
            elif key == 'allow':
                cur.allow = tuple(val.split('->'))

def run(path, shuffle=False):
    model = Model()
    if shuffle:
        # replay: parse once collecting event lines, shuffle them within their
        # pre-checkpoint groups is complex; instead verify the projection fn is
        # order-invariant by shuffling the trail post-hoc and re-projecting.
        parse(path, model)
        snapshot = [l for l in model.log if l.startswith('   ') and '@' not in l[:6]]
        base = [l for l in model.log]
        random.seed(11); random.shuffle(model.trail)
        model.log = []
        for name, rec in model.records.items():
            model.project(rec, 'shuffled-final')
        return base, model.log
    parse(path, model)
    return model.log, None

if __name__ == '__main__':
    path = sys.argv[1]
    log, _ = run(path)
    print('\n'.join(log))
    # order-invariance check: shuffle the trail, recompute final projections
    m1, m2 = Model(), Model()
    parse(path, m1); parse(path, m2)
    random.seed(7); random.shuffle(m2.trail)
    for name in m1.records:
        m1.log = []; m2.log = []
        m1.project(m1.records[name], 'final'); m2.project(m2.records[name], 'final')
        same = m1.log == m2.log
        print(f"order-invariance [{name}]: {'HOLDS' if same else '*** VIOLATED ***'}")
