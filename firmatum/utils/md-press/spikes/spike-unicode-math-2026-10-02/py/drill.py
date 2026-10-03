"""Math-free drill: run the converter over a sites file; print every change grouped."""
import json, sys, collections
sys.path.insert(0, 'py'); import umath
path = sys.argv[1]; show = int(sys.argv[2]) if len(sys.argv) > 2 else 60
n = ch = 0; c = collections.Counter(); ex = {}
for l in open(path):
    r = json.loads(l)
    if r['display']: continue
    n += 1
    o, info = umath.convert(r['body'])
    if o != r['body']:
        ch += 1
        for a, b, lat, conf in info:
            if lat:
                k = r['body'][a:b][:40]; c[k] += 1
                ex.setdefault(k, (r['file'], r['body'][max(0, a - 80):b + 60], lat))
print('sites', n, 'changed', ch, 'distinct regions', len(c))
for k, v in c.most_common(show):
    print(v, repr(k), '->', ex[k][2][:50], '|', ex[k][0].split('/src/')[1][:45], '|', ex[k][1][:130])
