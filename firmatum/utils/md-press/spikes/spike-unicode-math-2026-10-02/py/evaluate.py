"""Evaluate systems against the double-labeled gold.

usage: python3 py/evaluate.py SET [--show VERDICT] [--sys NAME]
  SET = A (dev: the coordinator's 240) | B (held-out) | AB

Systems: 'det' (this spike's converter, run live), 'llama', 'muse' (md-press's
actual result for each A piece, from inputs/judged-*.jsonl; A only), 'none'.

Each piece is scored against each labeler pass separately; a piece counts as
'wrong' for a system only if its written spans disagree with BOTH labelers
(when the labelers themselves disagree, matching either one is not an error).
"""
from __future__ import annotations

import json
import sys
from collections import Counter

sys.path.insert(0, __file__.rsplit('/', 1)[0])
import atoms
import score
import umath

BASE = __file__.rsplit('/py/', 1)[0]


def system_outputs(name, items):
    out = {}
    if name == 'det':
        for g, it in items.items():
            out[g] = umath.convert(it['text'])[0]
    elif name in ('llama', 'muse'):
        J = {r['id']: r for r in map(json.loads, open(f'{BASE}/inputs/judged-{name}.jsonl'))}
        for g, it in items.items():
            if g.startswith('A'):
                out[g] = J[int(g[1:])]['result']
        import os
        if name == 'llama' and os.path.exists(f'{BASE}/data/llama-B-judged.jsonl'):
            for r in map(json.loads, open(f'{BASE}/data/llama-B-judged.jsonl')):
                if r['gid'] in items:
                    out[r['gid']] = r['result']
    elif name == 'det_v1':
        import importlib.util
        spec = importlib.util.spec_from_file_location('umath_v1', f'{BASE}/py/frozen/umath_v1.py')
        m = importlib.util.module_from_spec(spec); sys.modules['umath_v1'] = m; spec.loader.exec_module(m)
        for g, it in items.items():
            out[g] = m.convert(it['text'])[0]
    elif name.startswith('det+L'):
        import withletters, pickle, os
        th = float(name.split('@')[1]) if '@' in name else 0.9
        cache = f'{BASE}/data/bulk/gold-ctx.pkl'
        if os.path.exists(cache):
            ctxs = pickle.load(open(cache, 'rb'))
        else:
            fs = json.load(open(f'{BASE}/data/bulk/gold-file-sites.json'))
            ctxs = {f: withletters.doc_context(v) for f, v in fs.items()}
            pickle.dump(ctxs, open(cache, 'wb'))
        for g, it in items.items():
            out[g] = withletters.convert(it['text'], ctxs.get(it['file']), th)
    elif name == 'none':
        for g, it in items.items():
            out[g] = it['text']
    return out


RANK = {'correct': 0, 'both-unchanged': 0, 'partial-safe': 1, 'wrong': 2, 'sys-malformed': 3, 'gold-malformed': 9}


def best_verdict(orig, golds, sysout):
    vs = [score.classify(orig, g, sysout) for g in golds]
    return min(vs, key=lambda v: RANK[v['verdict']])


LADDER = ['exact', 'equivalent', 'degraded', 'over', 'wrong']


def verdict2(orig, golds, sysout):
    """Best verdict against either labeler (atoms ladder)."""
    best = None
    for g in golds:
        v, d = atoms.compare(sysout, g)
        if best is None or LADDER.index(v) < LADDER.index(best[0]):
            best = (v, d, g)
    return best


def run(setname, systems, show=None, show_sys=None, quiet=False):
    items, passes = score.load_gold(BASE)
    sel = {g: it for g, it in items.items() if g[0] in setname and g in passes[1] and g in passes[2]}
    rows = {}
    per = {}
    for sname in systems:
        outs = system_outputs(sname, sel)
        if not outs:
            continue
        c = Counter()
        for g, it in sel.items():
            if g not in outs:
                continue
            golds = [passes[1][g]['gold'], passes[2][g]['gold']]
            v, d, gg = verdict2(it['text'], golds, outs[g])
            c[v] += 1
            per.setdefault(sname, {})[g] = v
            if show and v == show and (show_sys is None or show_sys == sname):
                print(f"\n[{sname}] {g} {v} {d}")
                print('  T ', it['text'][:300])
                print('  S ', outs[g][:300])
                print('  G1', golds[0][:300])
                if golds[1] != golds[0]:
                    print('  G2', golds[1][:300])
        rows[sname] = c
    if not quiet:
        print(f"\nset {setname}: {len(sel)} pieces; verdict = best against either labeler")
        print(f"{'system':8} " + ' '.join(f'{x:>10}' for x in LADDER) + '   ok(exact+equiv)')
        for s_, c in rows.items():
            tot = sum(c.values())
            print(f"{s_:8} " + ' '.join(f'{c[x]:10}' for x in LADDER) + f"   {c['exact'] + c['equivalent']:4} ({(c['exact'] + c['equivalent']) / tot:.1%})")
    return rows, per


if __name__ == '__main__':
    args = sys.argv[1:]
    setname = args[0] if args else 'A'
    show = args[args.index('--show') + 1] if '--show' in args else None
    show_sys = args[args.index('--sys') + 1] if '--sys' in args else None
    systems = ['none', 'llama', 'muse', 'det_v1', 'det'] if 'A' in setname else ['none', 'llama', 'det_v1', 'det']
    run(setname, systems, show, show_sys)
