"""Evaluate a converter on the agent-written Unicode dialect set (data/synth-agent)."""
import json, glob, sys, collections
sys.path.insert(0, 'py'); import umath, atoms
MODEL = {'0': 'sonnet', '1': 'sonnet', '2': 'opus', '3': 'haiku'}
def aligned(uni, gold):
    """Is every prose segment of the gold present, in order, in the rewrite?"""
    prose, cur, inm, esc = [], '', False, False
    for ch in gold:
        if ch == '$' and not esc:
            prose.append(cur); cur = ''; inm = not inm; continue
        esc = ch == '\\' and not esc
        if not inm:
            cur += ch
    prose.append(cur)
    pos = 0
    for seg in prose:
        i = uni.find(seg, pos)
        if i < 0:
            return False
        pos = i + len(seg)
    return True


def load():
    rows = []
    for f in sorted(glob.glob('data/synth-agent/out/batch-*.jsonl')):
        k = f[-7]
        T = {json.loads(l)['id']: json.loads(l) for l in open(f'data/synth-agent/tasks/batch-{k}.jsonl')}
        for l in open(f):
            if l.strip():
                r = json.loads(l); rows.append({'id': r['id'], 'model': MODEL[k], 'uni': r['unicode'], 'gold': T[r['id']]['latex']})
    return rows
if __name__ == '__main__':
    conv = umath.convert
    rows = load()
    bad = [r for r in rows if not aligned(r['uni'], r['gold'])]
    print('unaligned (dropped):', collections.Counter(r['model'] for r in bad))
    rows = [r for r in rows if aligned(r['uni'], r['gold'])]
    for r in rows:
        r['out'] = conv(r['uni'])[0]
        r['v'], _ = atoms.compare(r['out'], r['gold'], lenient=True)
        r['v_strict'], _ = atoms.compare(r['out'], r['gold'])
    json.dump(rows, open('data/synth-agent/eval-det.json', 'w'), ensure_ascii=False)
    for m in ('sonnet', 'opus', 'haiku', 'all'):
        R = [r for r in rows if m == 'all' or r['model'] == m]
        c = collections.Counter(r['v'] for r in R)
        print(f"{m:7} n={len(R):3} " + ' '.join(f"{k} {c[k]/len(R):.0%}" for k in ['exact', 'equivalent', 'degraded', 'over', 'wrong']),
              f"| ok {(c['exact'] + c['equivalent'])/len(R):.1%}")
