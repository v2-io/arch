import json, re, sys
sys.path.insert(0, '.')
from labels import LABELS
BASE='/Users/josephwecker-v2/src/arch/firmatum/utils/md-press/spikes/spike-unicode-math-2026-10-02/data/gold/C'
items=[json.loads(l) for l in open(BASE+'/tasks/batch-1.jsonl')]
assert set(LABELS)==set(d['gid'] for d in items), (set(LABELS)^set(d['gid'] for d in items))
SPAN=re.compile(r'\$[^$]+\$')
def check_edit(old,new):
    parts=[]; pos=0; spans=[]
    for m in SPAN.finditer(new):
        parts.append(re.escape(new[pos:m.start()])); parts.append('(.+?)'); spans.append(m.group()); pos=m.end()
    parts.append(re.escape(new[pos:]))
    assert '$' not in SPAN.sub('',new), ('stray $',new)
    assert re.fullmatch(''.join(parts), old, re.S), ('shape',old,new)
    for s in spans:
        c=s[1:-1]
        assert c==c.strip(), ('padding',s)
        assert not re.search(r'[<>*|]', c), ('raw char',s)
        assert not re.search(r'[^\x00-\x7f]', c.replace('–','')), ('non-ascii in span',s)
out=[]; bad=0
for d in items:
    edits,conf,note=LABELS[d['gid']]
    t=d['text']; res=[]; pos=0
    for old,new in edits:
        i=t.find(old,pos)
        if i<0: print('NOTFOUND',d['gid'],repr(old)); bad+=1; continue
        check_edit(old,new)
        res.append(t[pos:i]); res.append(new); pos=i+len(old)
    res.append(t[pos:]); gold=''.join(res)
    out.append({'gid':d['gid'],'gold':gold,'conf':conf,'note':note})
if bad: sys.exit(1)
with open(BASE+'/pass2/batch-1.jsonl','w') as f:
    for o in out: f.write(json.dumps(o,ensure_ascii=False)+'\n')
print('wrote',len(out), {c:sum(o['conf']==c for o in out) for c in ('high','medium','low')}, 'changed', sum(o['gold']!=d['text'] for o,d in zip(out,items)))
