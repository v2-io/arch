import json,re
T={json.loads(l)['gid']:json.loads(l)['text'] for l in open('tasks/batch-0.jsonl')}
for l in open('pass1/batch-0.jsonl'):
    d=json.loads(l); g=d['gold']
    outside=re.sub(r'\$[^$]+\$','',g)
    left=re.findall(r'[Ͱ-Ͽ₀-ₜ⁰-ⁿ∀-⋿²³‖′]|\w_\w',outside)
    if g!=T[d['gid']]: print(d['gid'],d['conf'],'|',g[:500])
    if left: print('   LEFT OUTSIDE:',d['gid'],left)
