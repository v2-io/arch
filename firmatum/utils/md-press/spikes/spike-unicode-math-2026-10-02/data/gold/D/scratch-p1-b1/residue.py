import json,re
for l in open('pass1/batch-1.jsonl'):
    d=json.loads(l); g=d['gold']
    s=re.sub(r'\$[^$]+\$','§',g); s=re.sub(r'`[^`]*`','¤',s)
    hits=re.findall(r'.{0,12}(?:[Ͱ-Ͽᴀ-ᶿ⁰-₟∀-⋿̂̄√‖≤≥≈∝]|\b[A-Za-z]_[A-Za-z0-9{]|\b[A-Za-z]\^[\w{(]).{0,12}',s)
    if hits: print(d['gid'],d['conf'],hits)
