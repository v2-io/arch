"""Held-out evaluation on set C or D (by item kind), frozen converters only.
usage: python3 py/evaluate_c.py [--set D] umath_v3 umath_v4 ..."""
import json, glob, sys, collections, importlib.util
sys.path.insert(0, 'py')
import atoms
LADDER = ['exact', 'equivalent', 'degraded', 'over', 'wrong']

def load_frozen(name):
    spec = importlib.util.spec_from_file_location(name, f'py/frozen/{name}.py')
    m = importlib.util.module_from_spec(spec); sys.modules[name] = m; spec.loader.exec_module(m)
    return m

SET = 'C'


def gold():
    items = {i['gid']: i for i in json.load(open(f'data/gold/{SET}-items.json'))}
    P = {}
    for p in (1, 2):
        P[p] = {}
        for f in glob.glob(f'data/gold/{SET}/pass{p}/batch-*.jsonl'):
            for l in open(f):
                if l.strip():
                    r = json.loads(l); P[p][r['gid']] = r
    return items, P

def best(out, golds):
    vs = [atoms.compare(out, g)[0] for g in golds]
    return min(vs, key=LADDER.index)

if __name__ == '__main__':
    args = sys.argv[1:]
    if '--set' in args:
        SET = args[args.index('--set') + 1]; del args[args.index('--set'):args.index('--set') + 2]
    items, P = gold()
    systems = {'none': lambda t: t}
    for nm in args or ['umath_v2']:
        m = load_frozen(nm); systems[nm] = (lambda m: lambda t: m.convert(t)[0])(m)
    both = [g for g in items if g in P[1] and g in P[2]]
    ag = sum(atoms.compare(P[1][g]['gold'], P[2][g]['gold'])[0] in ('exact', 'equivalent') for g in both)
    print(f'{SET} items with both labels: {len(both)}; labelers equivalent-or-better: {ag} ({ag/len(both):.1%})')
    res = {}
    for sn, fn in systems.items():
        by = collections.defaultdict(collections.Counter)
        for g in both:
            v = best(fn(items[g]['text']), [P[1][g]['gold'], P[2][g]['gold']])
            by[items[g]['kind']][v] += 1; by['ALL'][v] += 1
            res.setdefault(sn, {})[g] = v
        print(f'\n{sn}')
        for k in ['piece', 'line', 'untrig-changed', 'untrig-unchanged', 'ALL']:
            c = by[k]; n = sum(c.values())
            if not n:
                continue
            print(f"  {k:17} n={n:3} " + ' '.join(f'{x} {c[x]:3}' for x in LADDER) + f"   ok {(c['exact'] + c['equivalent'])/n:.1%}  wrong+over {(c['wrong'] + c['over'])/n:.1%}")
    json.dump(res, open(f'data/gold/{SET}/verdicts.json', 'w'), indent=0)
