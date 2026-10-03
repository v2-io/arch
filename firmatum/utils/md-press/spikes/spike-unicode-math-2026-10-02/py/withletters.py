"""det + the bare-letter forest: promote residual single letters the forest
is confident about, using document context (letters the rest of the file
already uses as math)."""
import json, pickle, sys, collections, re
sys.path.insert(0, 'py')
import umath, letters

_model = None


def model():
    global _model
    if _model is None:
        _model = pickle.load(open('data/letters-rf.pkl', 'rb'))
    return _model


def doc_context(texts):
    c, n = letters.doc_symbols(texts)
    # existing $..$ spans count as evidence too
    for t in texts:
        for a, b, k in umath.protected_ranges(t):
            if k == 'math':
                n += 1
                for ch in set(umath.SINGLE_LETTER_IN_TEX.findall(t[a:b])):
                    c[ch] += 1
    return c, n


def convert(text, ctx=None, th=0.9):
    out, info = umath.convert(text)
    if out == text and not any(ch.isalpha() for ch in text):
        return out
    syms, nsp = ctx if ctx else doc_context([text])
    if CURRENCY_OR_HAZARD(text):
        return out
    feats, spans = letters.letter_features(text, syms, nsp)
    if not feats:
        return out
    m = model()
    P = m.predict_proba([f for _, f in feats])[:, 1]
    add = [st for (st, f), p in zip(feats, P) if p >= th]
    if not add:
        return out
    # rebuild: existing det spans + new single-letter spans
    edits = [(a, b, '$' + lat + '$') for a, b, lat, c in info if lat] + [(st, st + 1, '$' + text[st] + '$') for st in add]
    edits.sort()
    res, pos = [], 0
    for a, b, rep in edits:
        if a < pos:
            continue
        res.append(text[pos:a]); res.append(rep); pos = b
    res.append(text[pos:])
    return ''.join(res)


def CURRENCY_OR_HAZARD(t):
    return bool(umath.CURRENCY.search(t)) or umath.dollar_hazard(t) or any(0x2500 <= ord(c) <= 0x257F for c in t)
