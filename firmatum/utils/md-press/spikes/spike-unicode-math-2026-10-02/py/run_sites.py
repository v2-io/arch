"""Run the converter over every prose site in data/bulk/sites.jsonl.
Writes data/bulk/conv-<tag>.jsonl: only sites the converter changed or errored on."""
import json, sys, time, traceback
from multiprocessing import Pool
sys.path.insert(0, 'py')
import umath

def work(line):
    r = json.loads(line)
    if r['display']:
        return None
    try:
        out = umath.convert(r['body'])[0]
    except Exception as e:
        return {**r, 'error': repr(e)[:200]}
    if out != r['body']:
        return {**r, 'out': out}
    return None

if __name__ == '__main__':
    tag = sys.argv[1]
    t = time.time()
    lines = open('data/bulk/sites.jsonl').readlines()
    n = ch = er = 0
    with Pool(12) as p, open(f'data/bulk/conv-{tag}.jsonl', 'w') as f:
        for res in p.imap(work, lines, chunksize=500):
            n += 1
            if res is not None:
                if 'error' in res:
                    er += 1
                else:
                    ch += 1
                f.write(json.dumps(res, ensure_ascii=False) + '\n')
    print(f'{n} sites, {ch} changed, {er} errors, {time.time() - t:.0f}s')
