import json,re
for i,l in enumerate(open('tasks/batch-1.jsonl')):
    d=json.loads(l); t=d['text']
    outside=re.sub(r'\$[^$]*\$','',t)
    na=sorted(set(c for c in outside if ord(c)>127 and c not in '—–“”’‘…'))
    flag = ' \\( ' if '\\(' in t else ''
    if na or flag: print(i,d['gid'],' '.join(na),flag)
