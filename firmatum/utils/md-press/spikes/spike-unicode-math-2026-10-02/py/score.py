"""Scoring: compare converter outputs with gold, at the span level.

A label/output is "the original with some regions replaced by $..$ spans".
We recover (region, latex) pairs by md-press's own alignment (edit_pairs
port), drop identity regions (an existing span left as it was), normalize
LaTeX spellings that render the same, and classify each disagreement.
"""
from __future__ import annotations

import json
import re
import sys
from collections import Counter


def edit_pairs(original: str, cand: str):
    """Port of md-press's edit_pairs: [((a, b), latex)] or None if cand is not
    'original with regions replaced by $-spans'."""
    prose, maths = [], []
    cur, mcur = [], []
    in_math = False
    esc = False
    for c in cand:
        if c == '$' and not esc:
            if not in_math:
                prose.append(''.join(cur)); cur = []
            else:
                maths.append(''.join(mcur)); mcur = []
            in_math = not in_math
            continue
        esc = c == '\\' and not esc
        (mcur if in_math else cur).append(c)
    if in_math:
        return None
    prose.append(''.join(cur))
    n = len(prose) - 1
    if n == 0:
        return [] if original == cand else None
    if not original.startswith(prose[0]):
        return None
    p = len(prose[0])
    out = []
    for k in range(1, n + 1):
        seg = prose[k]
        if p >= len(original):
            return None
        q = p + 1
        if k == n:
            s = len(original) - len(seg)
            if s < q or original[s:] != seg:
                return None
            start = s
        else:
            f = original.find(seg, q)
            if f < 0:
                return None
            start = f
        out.append(((p, start), maths[k - 1]))
        p = start + len(seg)
    return out


def new_spans(original, cand):
    pr = edit_pairs(original, cand)
    if pr is None:
        return None
    res = []
    for (a, b), lat in pr:
        if original[a:b] == '$' + lat + '$':
            continue  # pre-existing span untouched
        res.append((a, b, lat))
    return res


SYN = [
    (r'\\le(?![a-zA-Z])', r'\\leq'), (r'\\ge(?![a-zA-Z])', r'\\geq'), (r'\\ne(?![a-zA-Z])', r'\\neq'),
    (r'\\epsilon', r'\\varepsilon'), (r'\\varphi', r'\\phi'), (r'\\rightarrow', r'\\to'),
    (r'\\lvert|\\rvert|\\vert|\\mid|\\lvert', '|'), (r'\\lVert|\\rVert|\\Vert|\\\|', '‖'),
    (r'\\ast', '*'), (r'\\lt(?![a-zA-Z])', '<'), (r'\\gt(?![a-zA-Z])', '>'),
    (r'\\(?:mathrm|operatorname|textrm|textnormal|mathit|textit|text)\s*\{([^{}]*)\}', r'\\T{\1}'),
    (r'\\(?:min|max|sup|inf|log|exp|ln|lim|det|arg|Pr)(?![a-zA-Z])', lambda m: r'\T{' + m.group()[1:] + '}'),
    (r'\\left|\\right|\\big|\\Big|\\bigl|\\bigr|\\Bigl|\\Bigr', ''),
    (r'\\[,;:!> ]|\\quad|\\qquad', ''),
    (r'\\top', 'T'), (r'\\intercal', 'T'),
    (r'\\dots|\\ldots|\\cdots', '…'), (r'\.\.\.', '…'),
    (r'\\leftrightarrow', '↔'),
    (r'\\Rightarrow|\\implies|\\Longrightarrow', '⇒'),
    (r'\\Leftrightarrow|\\iff|\\Longleftrightarrow', '⇔'),
    (r'\\lnot', r'\\neg'),
    (r'\\varnothing', r'\\emptyset'),
    (r'\\dagger', '†'),
]


def norm(lat: str) -> str:
    s = lat
    for pat, rep in SYN:
        s = re.sub(pat, rep, s)
    # \mathcal M -> \mathcal{M}
    s = re.sub(r'\\(mathcal|mathbb|mathbf|mathfrak|mathsf|hat|bar|tilde|vec|dot)\s+([A-Za-z0-9])', r'\\\1{\2}', s)
    s = re.sub(r'\s+', '', s)
    # {\mathcal{M}} -> \mathcal{M}
    for _ in range(3):
        s = re.sub(r'\{(\\[A-Za-z]+\{[^{}]*\})\}', r'\1', s)
        # single-token braces after _ or ^
        s = re.sub(r'([_^])\{([A-Za-z0-9*\'′]|\\[A-Za-z]+|\\T\{[^{}]*\}|\(\w\))\}', r'\1\2', s)
    s = s.replace("^{\\prime}", "'").replace("^\\prime", "'").replace('′', "'")
    s = s.replace("^*", "*").replace("^{*}", "*")
    s = s.replace('\\T{-}', '-').replace('\\T{–}', '-').replace('–', '-').replace('−', '-')
    return s


def classify(orig, gold, sysout):
    """Per-piece comparison. Returns dict with counters and a verdict."""
    G = new_spans(orig, gold)
    S = new_spans(orig, sysout)
    r = Counter()
    if G is None:
        return {'verdict': 'gold-malformed', 'c': r}
    if S is None:
        return {'verdict': 'sys-malformed', 'c': r}
    gmap = {(a, b): norm(l) for a, b, l in G}
    smap = {(a, b): norm(l) for a, b, l in S}
    used = set()
    for (a, b), gl in gmap.items():
        if (a, b) in smap:
            used.add((a, b))
            if smap[(a, b)] == gl:
                r['exact'] += 1
            else:
                r['content'] += 1
        else:
            ov = [k for k in smap if k[0] < b and k[1] > a]
            if ov:
                r['boundary'] += 1
                used.update(ov)
            else:
                r['missed'] += 1
    for k in smap:
        if k not in used:
            r['spurious'] += 1
    r['gold_spans'] = len(gmap)
    r['sys_spans'] = len(smap)
    wrong = r['content'] + r['boundary'] + r['spurious']
    if not gmap and not smap:
        v = 'both-unchanged'
    elif wrong == 0 and r['missed'] == 0:
        v = 'correct'
    elif wrong == 0:
        v = 'partial-safe'      # some gold spans not produced, nothing wrong written
    else:
        v = 'wrong'             # at least one written span disagrees with gold
    return {'verdict': v, 'c': r}


def load_gold(base):
    import glob
    items = {i['gid']: i for i in json.load(open(f'{base}/data/gold/all-items.json'))}
    passes = {}
    for p in (1, 2):
        d = {}
        for f in sorted(glob.glob(f'{base}/data/gold/pass{p}/batch-*.jsonl')):
            for line in open(f):
                if line.strip():
                    r = json.loads(line)
                    d[r['gid']] = r
        passes[p] = d
    return items, passes


def agree(orig, g1, g2):
    c = classify(orig, g1, g2)
    return c['verdict'] in ('correct', 'both-unchanged')


if __name__ == '__main__':
    base = sys.argv[1] if len(sys.argv) > 1 else '.'
    items, passes = load_gold(base)
    both = [g for g in items if g in passes[1] and g in passes[2]]
    ag = sum(agree(items[g]['text'], passes[1][g]['gold'], passes[2][g]['gold']) for g in both)
    print(f'items with both passes: {len(both)}; labelers agree (normalized): {ag} ({ag/len(both):.1%})')
