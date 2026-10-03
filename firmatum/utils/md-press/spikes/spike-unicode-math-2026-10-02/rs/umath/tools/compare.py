"""Compare two JSONL result files (Rust vs Python) record by record.

  python3 rs/umath/tools/compare.py rust.jsonl ref.jsonl [inputs.jsonl]

Records are keyed by id; a record present on one side only is a difference
(with --changed-only both sides omit exactly the unchanged, span-free ones).
Errors compare by presence (the Rust side names the Python exception class).
"""
import json
import sys


def load(p):
    d = {}
    for line in open(p):
        if line.strip():
            r = json.loads(line)
            d[json.dumps(r['id'])] = r
    return d


A, B = load(sys.argv[1]), load(sys.argv[2])
texts = {}
if len(sys.argv) > 3:
    for line in open(sys.argv[3]):
        r = json.loads(line)
        texts[json.dumps(r['id'])] = r['text']
diffs = []
for k in sorted(set(A) | set(B)):
    a, b = A.get(k), B.get(k)
    if a is None or b is None:
        diffs.append((k, 'missing on ' + ('rust' if a is None else 'ref'), a, b))
        continue
    if ('error' in a) != ('error' in b):
        diffs.append((k, 'error mismatch', a, b))
        continue
    if 'error' in a:
        continue
    if a['out'] != b['out']:
        diffs.append((k, 'out', a, b))
        continue
    if 'spans' in a or 'spans' in b:
        sa = [(x[0], x[1], x[2], round(x[3], 12)) for x in a.get('spans', [])]
        sb = [(x[0], x[1], x[2], round(x[3], 12)) for x in b.get('spans', [])]
        if sa != sb:
            diffs.append((k, 'spans', a, b))
print(f'{len(A)} rust records, {len(B)} ref records, {len(diffs)} differences, '
      f'{sum("error" in r for r in B.values())} reference errors')
for k, why, a, b in diffs[:40]:
    print('---', k, why)
    if k in texts:
        print('  in  :', texts[k])
    print('  rust:', json.dumps(a, ensure_ascii=False)[:600])
    print('  ref :', json.dumps(b, ensure_ascii=False)[:600])
