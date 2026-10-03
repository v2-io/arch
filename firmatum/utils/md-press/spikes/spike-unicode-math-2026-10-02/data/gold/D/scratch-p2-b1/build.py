import json, re, os, sys
here = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, here)
from spec import S

base = os.path.dirname(here)
items = [json.loads(l) for l in open(os.path.join(base, 'tasks/batch-1.jsonl'))]
assert len(items) == len(S) == 133, (len(items), len(S))

def apply(text, reps):
    cur, out = 0, text
    for old, new in reps:
        j = out.find(old, cur)
        if j < 0:
            raise ValueError(f'not found after {cur}: {old!r}')
        out = out[:j] + new + out[j + len(old):]
        cur = j + len(new)
    return out

span_re = re.compile(r'\$[^$]+\$')

def check_shape(inp, gold):
    # gold = text segments interleaved with $..$ spans; text segments must appear in order in inp,
    # with arbitrary replaced regions between them.
    parts = span_re.split(gold)
    pat = '(.*?)'.join(re.escape(p) for p in parts)
    m = re.fullmatch(pat, inp, re.S)
    assert m, 'shape violation'
    for sp in span_re.findall(gold):
        body = sp[1:-1]
        assert body == body.strip(), f'padding in {sp}'
        for bad in '<>*|':
            # raw bars allowed only if already present in the input region (existing spans)
            if bad in body and sp not in inp:
                raise AssertionError(f'raw {bad} in new span {sp}')

outp = os.path.join(base, 'pass2/batch-1.jsonl')
os.makedirs(os.path.dirname(outp), exist_ok=True)
nchg = 0
with open(outp, 'w') as f:
    for d in items:
        reps, conf, note = S[d['gid']]
        gold = apply(d['text'], reps)
        check_shape(d['text'], gold)
        nchg += gold != d['text']
        f.write(json.dumps({'gid': d['gid'], 'gold': gold, 'conf': conf, 'note': note}, ensure_ascii=False) + '\n')
print('ok', len(items), 'changed', nchg)
