import json, re
T = {}
order = []
for l in open('tasks/batch-2.jsonl'):
    d = json.loads(l); T[d['gid']] = d['text']; order.append(d['gid'])
G = [json.loads(l) for l in open('pass1/batch-2.jsonl')]
assert [g['gid'] for g in G] == order
bad = 0
for g in G:
    src = T[g['gid']]
    parts = re.split(r'(\$\$[^$]+\$\$|\$[^$]+\$)', g['gold'])
    rx = ''.join(('(.+?)' if (i % 2 and p not in src) else re.escape(p)) for i, p in enumerate(parts))
    if not re.fullmatch(rx, src, re.S):
        bad += 1; print('SHAPE FAIL', g['gid'])
    for sp in re.findall(r'(?<!\$)\$([^$]+)\$(?!\$)', g['gold']):
        if sp != sp.strip(): print('PAD', g['gid'], repr(sp))
    assert g['conf'] in ('high', 'medium', 'low')
print('checked', len(G), 'bad', bad)
