"""Bare single-letter variables: data, features, and a random forest.

Positives/negatives come from lines the estate already wrote in LaTeX: after
reversing every span to Unicode, a standalone single Latin letter that sat
inside a $..$ span is a variable (1); one the author left in prose is not (0).
Only letters the deterministic converter did NOT already include are used —
the forest's job is the residue.
"""
import json, random, re, sys, collections
sys.path.insert(0, 'py')
import umath, reverse

PREPS = {'of', 'for', 'with', 'to', 'in', 'on', 'at', 'by', 'from', 'over', 'where', 'and', 'or', 'than',
         'when', 'if', 'that', 'let', 'set', 'each', 'every', 'some', 'any', 'all', 'as', 'is', 'are', 'be'}
ARTS = {'the', 'a', 'an', 'this', 'that', 'its', 'our', 'their'}
COPULA = {'is', 'are', 'was', 'were', 'be', 'has', 'have', 'denotes', 'equals', 'grows', 'can', 'must', 'may',
          'will', 'would', 'should', 'does', 'satisfies', 'approaches', 'stays', 'remains', 'becomes'}
FEATS = ['upper', 'aAI', 'ord', 'in_line_syms', 'doc_freq', 'doc_spans', 'line_spans', 'dist_span',
         'prev_kind', 'next_kind', 'prev_cap_word', 'prev_prep', 'prev_art', 'next_copula', 'next_poss',
         'paren_alone', 'glued_punct_after', 'line_start', 'next_word_len', 'n_letters_line']
KIND = {'start': 0, 'end': 0, 'word': 1, 'Word': 2, 'num': 3, 'punct': 4, 'open': 5, 'close': 6, 'op': 7,
        'emph': 8, 'dash': 9, 'other': 10, 'term': 11, 'mspan': 12}


def kind_of(u):
    if u is None:
        return 'start'
    if u.kind == 'term':
        if u.term.mathness in ('word', 'label', 'ident'):
            return 'Word' if u.text[:1].isupper() else 'word'
        if u.term.base == 'mspan':
            return 'mspan'
        if u.term.mathness == 'num':
            return 'num'
        return 'term'
    if u.kind in KIND:
        return u.kind
    if u.kind in ('hyph',):
        return 'op'
    return 'other'


def letter_features(text, doc_syms, doc_spans):
    """[(char_start, feature_vector)] for each standalone single-letter token
    not already inside a det span."""
    out_text, info = umath.convert(text)
    spans = [(a, b) for a, b, lat, c in info if lat]
    toks = umath.mark_emphasis(umath.lex(text))
    us = umath.units_of(toks)
    line_syms = set()
    for a, b in spans:
        for ch in re.findall(r'(?<![A-Za-z])[A-Za-z](?![A-Za-z])', text[a:b]):
            line_syms.add(ch)
    res = []
    nletters = sum(1 for u in us if u.kind == 'term' and u.term.mathness == 'letter')
    for k, u in enumerate(us):
        if not (u.kind == 'term' and u.term.mathness == 'letter' and u.b - u.a == 1):
            continue
        t = toks[u.a]
        st = t.a
        if any(a <= st < b for a, b in spans):
            continue
        L = t.text

        def nb(d):
            j = k + d
            while 0 <= j < len(us) and us[j].kind == 'ws':
                j += d
            return us[j] if 0 <= j < len(us) else None
        p, nx = nb(-1), nb(+1)
        pw = p.text.lower() if p is not None and p.kind == 'term' else ''
        nw = nx.text.lower() if nx is not None and nx.kind == 'term' else ''
        dist = min([abs(st - a) for a, b in spans] + [abs(st - b) for a, b in spans] + [999])
        f = [int(L.isupper()), int(L in 'aAI'), ord(L.lower()) - 96, int(L in line_syms),
             doc_syms.get(L, 0) / max(1, doc_spans), doc_spans, len(spans), min(dist, 200),
             KIND.get(kind_of(p), 10), KIND.get(kind_of(nx), 10),
             int(p is not None and kind_of(p) == 'Word'), int(pw in PREPS), int(pw in ARTS),
             int(nw in COPULA), int(us[k + 1].kind == 'prime' if k + 1 < len(us) else 0),
             int(k > 0 and us[k - 1].kind == 'open' and k + 1 < len(us) and us[k + 1].kind == 'close'),
             int(k + 1 < len(us) and us[k + 1].kind == 'punct'), int(p is None), len(nw), nletters]
        res.append((st, f))
    return res, spans


def doc_symbols(lines):
    """Letter -> number of det spans (across these lines) that use it as an operand."""
    c = collections.Counter()
    n = 0
    for text in lines:
        _, info = umath.convert(text)
        for a, b, lat, conf in info:
            if lat:
                n += 1
                for ch in set(re.findall(r'(?<![A-Za-z])[A-Za-z](?![A-Za-z])', text[a:b])):
                    c[ch] += 1
    return c, n


def build(n_files, seed=3):
    L = json.load(open('data/bulk/latex-lines.json'))
    by = collections.defaultdict(list)
    for r in L:
        by[r['file']].append(r['body'])
    files = sorted(by)
    rng = random.Random(seed)
    rng.shuffle(files)
    rows = []
    for fi, f in enumerate(files[:n_files]):
        uni = []
        for body in by[f]:
            try:
                u = reverse.unicodify_line(body, rng)
                uni.append((u, list(reverse.RANGES)))
            except Exception:
                continue
        syms, nsp = doc_symbols([u for u, _ in uni])
        for u, ranges in uni:
            feats, _ = letter_features(u, syms, nsp)
            for st, fv in feats:
                y = int(any(a <= st < b for a, b in ranges))
                rows.append({'file': fi, 'y': y, 'x': fv, 'ctx': u[max(0, st - 40):st + 40]})
    return rows


if __name__ == '__main__':
    rows = build(int(sys.argv[1]))
    json.dump(rows, open('data/bulk/letters.json', 'w'), ensure_ascii=False)
    c = collections.Counter(r['y'] for r in rows)
    print(len(rows), c)
