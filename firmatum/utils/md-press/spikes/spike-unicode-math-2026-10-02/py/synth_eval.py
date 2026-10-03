"""Synthetic round-trip evaluation: estate LaTeX lines -> Unicode (reverse.py) -> converter -> compare."""
import json, random, sys, collections
sys.path.insert(0, 'py')
import umath, atoms, reverse
from multiprocessing import Pool

import re
INCOMPLETE = re.compile('[\u0370-\u03ff\u2080-\u209c\u2070-\u207f\u00b2\u00b3\u00b9\u2200-\u22ff\U0001d400-\U0001d7ff‖]')
P_REV = float(sys.argv[3]) if len(sys.argv) > 3 else 1.0


def one(args):
    k, rec = args
    rng = random.Random(1000 + k)
    # gold that still carries Unicode math outside its spans is incomplete
    masked = ''.join(' ' if any(a <= i < b for a, b, kk in umath.protected_ranges(rec['body'])) else ch
                     for i, ch in enumerate(rec['body']))
    if INCOMPLETE.search(masked):
        return {'k': k, 'skip': 'gold-incomplete'}
    try:
        uni = reverse.unicodify_line(rec['body'], rng, P_REV)
    except reverse.Unmappable as e:
        return {'k': k, 'skip': str(e)[:30]}
    except Exception as e:
        return {'k': k, 'skip': 'err:' + repr(e)[:40]}
    out = umath.convert(uni)[0]
    v, d = atoms.compare(out, rec['body'])
    return {'k': k, 'file': rec['file'], 'gold': rec['body'], 'uni': uni, 'out': out, 'v': v}

if __name__ == '__main__':
    N = int(sys.argv[1]); tag = sys.argv[2]
    L = json.load(open('data/bulk/latex-lines.json'))
    random.seed(7)
    S = random.sample(range(len(L)), N)
    with Pool(12) as p:
        res = p.map(one, [(k, L[k]) for k in S], chunksize=50)
    skip = collections.Counter(r['skip'] for r in res if 'skip' in r)
    R = [r for r in res if 'skip' not in r]
    c = collections.Counter(r['v'] for r in R)
    print('pairs', len(R), 'skipped', sum(skip.values()), skip.most_common(6))
    for v in atoms_ladder if False else ['exact', 'equivalent', 'degraded', 'over', 'wrong']:
        print(f'  {v:11} {c[v]:6} {c[v]/len(R):.1%}')
    json.dump(R, open(f'data/bulk/synth-{tag}.json', 'w'), ensure_ascii=False)
