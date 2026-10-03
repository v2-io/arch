import json,re
T={json.loads(l)['gid']:json.loads(l)['text'] for l in open('tasks/batch-2.jsonl')}
n=0
for l in open('pass2/batch-2.jsonl'):
    d=json.loads(l)
    if d['gold']!=T[d['gid']]:
        n+=1
        g=d['gold']
        old=set(re.findall(r'\$[^$]+\$',T[d['gid']]))
        for m in re.finditer(r'\$[^$]+\$',g):
            if m.group() in old: continue
            print(d['gid'], d['conf'][0], '…'+g[max(0,m.start()-22):m.end()+12]+'…')
print(n,'changed')
