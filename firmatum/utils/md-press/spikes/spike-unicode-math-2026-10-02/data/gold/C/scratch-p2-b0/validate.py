#!/usr/bin/env python3
"""Independent check of pass2/batch-0.jsonl: shape + house lints."""
import json, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
SPIKE = os.path.abspath(os.path.join(HERE, '..'))
items = [json.loads(l) for l in open(os.path.join(SPIKE, 'tasks', 'batch-0.jsonl'))]
gold = [json.loads(l) for l in open(os.path.join(SPIKE, 'pass2', 'batch-0.jsonl'))]
assert [d['gid'] for d in items] == [g['gid'] for g in gold]

def spans(s):
    # split into outside / $...$ pieces ($$ display treated as a span too)
    return re.split(r'(\$\$.*?\$\$|\$[^$]+\$)', s)

bad = 0; changed = 0
for d, g in zip(items, gold):
    t, s = d['text'], g['gold']
    assert g['conf'] in ('high', 'medium', 'low')
    if s == t:
        continue
    changed += 1
    parts = spans(s)
    # shape: outside pieces verbatim, in order; each span matches something non-empty
    pat = ''.join(re.escape(p) if i % 2 == 0 else '(.+?)' for i, p in enumerate(parts))
    if not re.fullmatch(pat, t, re.S):
        print('SHAPE FAIL', g['gid']); bad += 1; continue
    old_spans = set(re.findall(r'\$[^$]+\$', t))
    for i, p in enumerate(parts):
        if i % 2 == 0 or p in old_spans:
            continue
        body = p[1:-1]
        probs = []
        if body != body.strip(): probs.append('padding')
        if re.search(r'[<>|*]', body): probs.append('raw <>|*')
        if re.search(r'[^\x00-\x7f]', body.replace('–', '')): probs.append('non-ascii')
        if re.search(r'_[A-Za-z0-9]{2,}', body): probs.append('unbraced multi-char sub')
        if re.search(r'\\text\{[^}]*_', body): probs.append('_ in text')
        if probs:
            print('LINT', g['gid'], probs, p); bad += 1
print(f'{len(gold)} items, {changed} changed, {len(gold)-changed} unchanged, {bad} problems')
from collections import Counter
print(Counter(g['conf'] for g in gold))
