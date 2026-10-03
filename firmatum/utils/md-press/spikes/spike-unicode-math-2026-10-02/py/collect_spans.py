"""Collect every $..$ span det emits over the estate sites + agent synth + gold sets, for validation."""
import json, sys
sys.path.insert(0, 'py'); import umath
from multiprocessing import Pool

def work(text):
    out, info = umath.convert(text)
    return [lat for a, b, lat, c in info if lat]

if __name__ == '__main__':
    texts = []
    for l in open('data/bulk/sites.jsonl'):
        r = json.loads(l)
        if not r['display']:
            texts.append(r['body'])
    for i in json.load(open('data/gold/all-items.json')) + json.load(open('data/gold/C-items.json')):
        texts.append(i['text'])
    with Pool(12) as p:
        res = p.map(work, texts, chunksize=1000)
    seen = {}
    for lats in res:
        for lat in lats:
            seen[lat] = seen.get(lat, 0) + 1
    with open('data/bulk/spans.jsonl', 'w') as f:
        for k, (lat, n) in enumerate(sorted(seen.items(), key=lambda x: -x[1])):
            f.write(json.dumps({'id': k, 'tex': lat, 'n': n}, ensure_ascii=False) + '\n')
    print(len(seen), 'distinct spans,', sum(seen.values()), 'occurrences')
