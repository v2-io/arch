"""House-usage lexicon (corpus statistics): which Unicode math tokens does the
estate deliberately keep as prose, in files that otherwise write LaTeX?

For every file with >= 5 inline $..$ spans, count each Unicode token both as
raw prose (outside spans) and as a whole span (its LaTeX reversed to the same
token). A token written raw >= MIN_RAW times and inside math < MAX_SHARE of the
time is a house label (asf's W₀/W₁/W₂ regimes are the motivating case).
"""
import json, re, sys, collections, random
sys.path.insert(0, 'py')
import umath, reverse

TOKEN = re.compile(r'(?<![\wͰ-Ͽ])((?:[A-Za-z]|[Ͱ-Ͽ])(?:[₀-ₜ⁰-ⁿ²³¹ᵢ-ᵪⱼ]+|_[A-Za-z0-9]{1,3})?\'?)(?![\wͰ-Ͽ])')
MIN_RAW, MAX_SHARE = 8, 0.10
CANON = sys.argv[1] if len(sys.argv) > 1 else r'.'   # e.g. '/arch/asf/0[1-4]-[a-z]+-core/src/'


def build():
    by = collections.defaultdict(list)
    for l in open('data/bulk/sites.jsonl'):
        r = json.loads(l)
        if not r['display'] and re.search(CANON, r['file']):
            by[r['file']].append(r['body'])
    raw = collections.Counter(); math = collections.Counter(); files_raw = collections.defaultdict(set)
    for f, lines in by.items():
        nspans = sum(len([1 for a, b, k in umath.protected_ranges(t) if k == 'math']) for t in lines)
        if nspans < 5:
            continue
        for t in lines:
            prot = umath.protected_ranges(t)
            if not prot:
                masked = t
            else:
                mk = list(t)
                for a, b, k in prot:
                    for i in range(a, b):
                        mk[i] = ' '
                masked = ''.join(mk)
            for m in TOKEN.finditer(masked):
                tok = m.group(1)
                if any(ord(c) > 127 for c in tok):    # only Unicode-bearing tokens
                    raw[tok] += 1; files_raw[tok].add(f)
            for a, b, k in prot:
                if k != 'math':
                    continue
                inner = t[a + 1:b - 1]
                for variant in (0, 1):
                    rv = reverse.Rev(random.Random(variant)); rv.uni_digits = bool(variant); rv.tight = True
                    try:
                        u = rv.span(inner)
                    except Exception:
                        continue
                    if TOKEN.fullmatch(u):
                        math[u] += 1
    labels = {}
    for tok, n in raw.items():
        share = math[tok] / (math[tok] + n)
        if n >= MIN_RAW and share < MAX_SHARE and len(files_raw[tok]) >= 2:
            labels[tok] = {'raw': n, 'math': math[tok], 'files': len(files_raw[tok])}
    return labels


if __name__ == '__main__':
    lab = build()
    json.dump(lab, open('data/house-labels.json', 'w'), ensure_ascii=False, indent=1)
    for k, v in sorted(lab.items(), key=lambda x: -x[1]['raw']):
        print(k, v)
