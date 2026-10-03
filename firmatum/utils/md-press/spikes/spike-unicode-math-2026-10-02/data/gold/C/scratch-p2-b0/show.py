import json, re
T = {json.loads(l)['gid']: json.loads(l)['text'] for l in open('tasks/batch-0.jsonl')}
for l in open('pass2/batch-0.jsonl'):
    g = json.loads(l)
    if g['gold'] != T[g['gid']]:
        old = set(re.findall(r'\$[^$]+\$', T[g['gid']]))
        new = [s for s in re.findall(r'\$[^$]+\$', g['gold']) if s not in old]
        print(g['gid'], g['conf'], '|', ' ; '.join(new))
