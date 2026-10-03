#!/usr/bin/env python3
"""Build pass1/batch-2 gold from per-item replacement specs, and verify shape.

Each spec: gid -> (edits, conf, note). edits = list of (old, new) or (old, new, 'all').
Replacements apply only to text outside $...$ spans (existing or already inserted),
except when `old` itself contains '$' (deliberate absorption of an existing span).
"""
import json, re, sys, os

HERE = os.path.dirname(os.path.abspath(__file__))
TASKS = os.path.join(HERE, '..', 'tasks', 'batch-2.jsonl')
OUT = os.path.join(HERE, '..', 'pass1', 'batch-2.jsonl')

from specs import SPECS  # noqa


def math_intervals(s):
    out = []
    for m in re.finditer(r'\$\$.+?\$\$|\$[^$]+?\$', s):
        out.append((m.start(), m.end()))
    return out


def apply(text, old, new, mode):
    absorbing = '$' in old
    ivs = [] if absorbing else math_intervals(text)
    pat = old if mode == 're' else re.escape(old)
    hits = [(m.start(), m.end()) for m in re.finditer(pat, text)
            if all(not (a < m.end() and m.start() < b) for a, b in ivs)]
    if not hits:
        raise ValueError(f'no free occurrence of {old!r}')
    if mode != 'all' and len(hits) != 1:
        raise ValueError(f'{len(hits)} free occurrences of {old!r}; use all')
    for s, e in reversed(hits):
        text = text[:s] + new + text[e:]
    return text


def shape_ok(src, gold):
    # gold literals must reproduce src exactly; each $..$ in gold matches a non-empty src region
    parts = re.split(r'(\$[^$]+?\$)', gold)
    pat = ''
    for p in parts:
        if p.startswith('$') and p.endswith('$') and len(p) > 1:
            pat += '(.+?)'
        else:
            pat += re.escape(p)
    return re.fullmatch(pat, src, flags=re.S) is not None


def main():
    items = [json.loads(l) for l in open(TASKS)]
    out = []
    bad = 0
    for it in items:
        gid, text = it['gid'], it['text']
        if gid not in SPECS:
            print('MISSING', gid); bad += 1; continue
        edits, conf, note = SPECS[gid]
        g = text
        for e in edits:
            old, new = e[0], e[1]
            mode = e[2] if len(e) > 2 else 'one'
            assert new.startswith('$') and new.endswith('$') and not new.startswith('$ ') and not new.endswith(' $'), (gid, new)
            try:
                g = apply(g, old, new, mode)
            except ValueError as ex:
                print('ERR', gid, ex); bad += 1
        if g != text and '$main' not in text and not shape_ok(text, g):
            print('SHAPE?', gid); bad += 1
        out.append({'gid': gid, 'gold': g, 'conf': conf, 'note': note})
    extra = set(SPECS) - {i['gid'] for i in items}
    if extra:
        print('EXTRA specs', extra); bad += 1
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, 'w') as f:
        for o in out:
            f.write(json.dumps(o, ensure_ascii=False) + '\n')
    print('wrote', len(out), 'bad', bad)
    if '-v' in sys.argv:
        for it, o in zip(items, out):
            if o['gold'] != it['text']:
                print(f"--- {o['gid']} [{o['conf']}]\n  {o['gold']}")


if __name__ == '__main__':
    main()
