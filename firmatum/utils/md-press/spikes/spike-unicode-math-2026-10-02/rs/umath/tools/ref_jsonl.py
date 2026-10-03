"""Reference side of the differential: the frozen Python converter with the
same JSONL contract as the Rust bin (`umath`).

  python3 rs/umath/tools/ref_jsonl.py py/frozen/umath_v4.py [--spans] [--changed-only] [--procs N] < in.jsonl
"""
import importlib.util
import json
import sys
sys.dont_write_bytecode = True  # never write __pycache__ under py/ (read-only for this crate)
from multiprocessing import Pool

ARGS = sys.argv[1:]
MOD = ARGS[0]
SPANS = '--spans' in ARGS
CHANGED = '--changed-only' in ARGS
PROCS = int(ARGS[ARGS.index('--procs') + 1]) if '--procs' in ARGS else 12

U = None


def init():
    global U
    spec = importlib.util.spec_from_file_location('umath_ref', MOD)
    U = importlib.util.module_from_spec(spec)
    sys.modules['umath_ref'] = U
    spec.loader.exec_module(U)


def work(line):
    if not line.strip():
        return None
    r = json.loads(line)
    try:
        out, info = U.convert(r['text'])
    except Exception as e:
        return json.dumps({'id': r.get('id'), 'error': type(e).__name__}, ensure_ascii=False)
    if CHANGED and out == r['text'] and not info:
        return None
    o = {'id': r.get('id'), 'out': out}
    if SPANS:
        o['spans'] = [[a, b, lat, conf] for a, b, lat, conf in info]
    return json.dumps(o, ensure_ascii=False)


if __name__ == '__main__':
    lines = sys.stdin.read().split('\n')
    if PROCS <= 1:
        init()
        res = map(work, lines)
    else:
        res = Pool(PROCS, initializer=init).imap(work, lines, chunksize=500)
    w = sys.stdout
    for x in res:
        if x is not None:
            w.write(x + '\n')
