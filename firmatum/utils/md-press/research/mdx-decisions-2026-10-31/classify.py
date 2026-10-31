"""Where md-press's decisions on MDX documents fall, in limen's MDX reading (FEEDBACK-2026-10-31 §3).

Inputs, beside this file:
  mdx-files.txt        the 1,144 *.md under ~/src whose structure limen's MX-1 changes
                       (paths relative to ~/src; found by hashing `limen-structure dump`
                       before and after MX-1 over every *.md there)
  md-press-report.txt  `md-press --no-log --no-math` over them, md-press 0.3.0, report only

For each break md-press would change (a join, or a keep it would mark), the two rows it
sits between are looked up in `limen-structure dump` (limen at 9a85e6e or later): inside an
md.code / md.code_indented block, a one-row md.html block (a component-tag row), or else.

  python3 -I classify.py [LIMEN_STRUCTURE_BINARY]
"""
import json, os, re, subprocess, sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
BIN = sys.argv[1] if len(sys.argv) > 1 else os.path.expanduser('~/src/limen/structure/target/release/limen-structure')
SRC = os.path.expanduser('~/src')

head = re.compile(r'^(.*): would change')
dec = re.compile(r'^\s+\[\w+\] L(\d+)(?:-(\d+))?\s+(join|keep)')
files, cur = {}, None
for line in open(os.path.join(HERE, 'md-press-report.txt'), encoding='utf-8', errors='replace'):
    m = head.match(line)
    if m:
        cur = m.group(1)
        files[cur] = []
        continue
    m = dec.match(line)
    if m and cur:
        a, kind = int(m.group(1)), m.group(3)
        b = int(m.group(2) or (a + 1))
        if kind == 'keep':
            files[cur].append(('keep', a, a + 1))
        else:
            files[cur].extend(('join', k, k + 1) for k in range(a, b))

tot, per_file = Counter(), Counter()
for f, ds in files.items():
    st = json.loads(subprocess.run([BIN, 'dump', os.path.join(SRC, f)], capture_output=True, text=True).stdout)
    code, tag = set(), set()
    for blk in st['blocks']:
        r0, r1 = blk['rows']
        if blk['kind'] in ('md.code', 'md.code_indented'):
            code.update(range(r0, r1 + 1))
        elif blk['kind'] == 'md.html' and r0 == r1:
            tag.add(r0)
    cats = set()
    for kind, l1, l2 in ds:  # 1-based lines
        rows = {l1 - 1, l2 - 1}
        cat = 'code' if rows & code else 'tag' if rows & tag else 'elsewhere'
        tot[(kind, cat)] += 1
        cats.add(cat)
    for c in cats:
        per_file[c] += 1

print('files md-press would change:', len(files))
for k in sorted(tot):
    print(f'{k[0]:5} {k[1]:9} {tot[k]:6}')
print('files touching each:', dict(per_file))
