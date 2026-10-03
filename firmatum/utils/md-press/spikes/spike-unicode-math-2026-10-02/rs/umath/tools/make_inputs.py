"""Build differential inputs {"id","text"} from the spike's data, into rs/umath/scratch/.

  estate.jsonl    data/bulk/sites.jsonl, display:false, id = 0-based line number
  mathfree.jsonl  data/bulk/mathfree-sites.jsonl, same
  gold.jsonl      data/gold/{all,C,D}-items.json, id = gid (text field)
"""
import json

S = 'rs/umath/scratch/'
for src, dst in (('data/bulk/sites.jsonl', 'estate.jsonl'), ('data/bulk/mathfree-sites.jsonl', 'mathfree.jsonl')):
    n = 0
    with open(src) as f, open(S + dst, 'w') as g:
        for i, line in enumerate(f):
            r = json.loads(line)
            if r.get('display'):
                continue
            g.write(json.dumps({'id': i, 'text': r['body']}, ensure_ascii=False) + '\n')
            n += 1
    print(dst, n)
n = 0
with open(S + 'gold.jsonl', 'w') as g:
    for src in ('data/gold/all-items.json', 'data/gold/C-items.json', 'data/gold/D-items.json'):
        for it in json.load(open(src)):
            g.write(json.dumps({'id': it['gid'], 'text': it['text']}, ensure_ascii=False) + '\n')
            n += 1
print('gold.jsonl', n)
