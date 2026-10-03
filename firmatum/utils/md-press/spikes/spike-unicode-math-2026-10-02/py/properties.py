"""Property checks on the frozen converter over every changed estate site, the
math-free corpus, and random fuzz. Claims checked:
  shape      output = input with some regions replaced by $..$ (md-press edit_pairs)
  verbatim   every protected range of the input (code, links, URLs, wikilinks, HTML,
             existing math) appears unchanged in the output
  idempotent convert(convert(x)) == convert(x)
  valid      tex_ok on every emitted span
  no-crash   on fuzz
"""
import json, random, sys, collections, re
sys.path.insert(0, 'py/frozen'); sys.path.insert(0, 'py')
import umath_v6 as m
import score
from multiprocessing import Pool

def check(text):
    try:
        out, info = m.convert(text)
    except Exception as e:
        return ('crash', repr(e)[:120], text[:200])
    if out == text:
        return None
    # independent shape check: mask inline code (a `$` inside backticks is not a
    # delimiter) in both, then md-press's edit_pairs alignment
    def mask(t):
        return re.sub(r'`[^`]*`', lambda mm: '`' + 'c' * (len(mm.group()) - 2) + '`', t)
    if score.new_spans(mask(text), mask(out)) is None:
        return ('shape', '', text[:200])
    for a, b, k in m.protected_ranges(text):
        seg = text[a:b]
        if k == 'math':   # existing math may be merged into a wider span: its content survives
            d = 2 if seg.startswith('$$') else 1
            seg = seg[d:-d] if len(seg) >= 2 * d else seg
        if seg not in out:
            return ('verbatim', k, text[:200])
    out2 = m.convert(out)[0]
    if out2 != out:
        return ('idempotent', out2[:200], out[:200])
    for a, b, lat, c in info:
        if lat and not m.tex_ok(lat):
            return ('valid', lat, text[:200])
    return ('ok',)

GLYPHS = list('αβγδεηθκλμνπρστφχψωΓΔΘΛΣΦΨΩ‖|≤≥≠≈→←↔⇒∈∉⊂⊆∪∩∞∂∇∑∏√±×·−+-=<>/*^_{}()[]⟨⟩$`\\.,;:!?\'"’“”—–…²³¹⁰₀₁₂ᵢⁿᵀ𝒜𝒯ℝ𝔼ℓ%#&@ ') + ['  ', ' x ', ' the ', ' a ', ' I ', 'M_t', '**', '[[w]]', '](http://x.y/(1))', '`code`', '$x$', '$$', 'η*', 'x\'', 'café', 'ô', 'W₁', 'H$_\\kappa$', '\\(x\\)', '10x', 'e.g.', 'p. 3', 'O(n)', 'kg/m³', 'μs', '𝟊', '†', 'ᵀ', 'ⁱ', '*^T', '⋃', '"κ_t']

def fuzz(n, seed):
    rng = random.Random(seed)
    return [''.join(rng.choice(GLYPHS) for _ in range(rng.randint(1, 40))) for _ in range(n)]

if __name__ == '__main__':
    texts = []
    for f in ('data/bulk/conv-v6.jsonl',):
        for l in open(f):
            texts.append(json.loads(l)['body'])
    for l in open('data/bulk/mathfree-sites.jsonl'):
        r = json.loads(l)
        if not r['display'] and any(ord(c) > 127 or c in '=<>_^' for c in r['body']):
            texts.append(r['body'])
    nreal = len(texts)
    texts += fuzz(200000, 7)
    with Pool(12) as p:
        res = p.map(check, texts, chunksize=500)
    c = collections.Counter(r[0] if r else 'unchanged' for r in res[:nreal])
    cf = collections.Counter(r[0] if r else 'unchanged' for r in res[nreal:])
    print('real', nreal, dict(c)); print('fuzz', len(texts) - nreal, dict(cf))
    shown = collections.Counter()
    for r in res:
        if r and r[0] not in ('ok',) and shown[r[0]] < 6:
            shown[r[0]] += 1
            print(r)
