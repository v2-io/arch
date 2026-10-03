"""llama3.2:3b on held-out D lines, the way md-press would run it: md-press's own
piece splitter (rs/probe pieces_of), the exact request of py/propose_llama.py,
md-press's real gates (rs/probe judge), pieces re-joined. Reproduces the
verifier's run (de-novo-feedback-1 F3). Writes data/llama-D-lines.jsonl."""
import json, subprocess, sys, time, urllib.request
sys.path.insert(0, 'py')
import evaluate_c as E
E.SET = 'D'
items, P = E.gold()
inst = open('/Users/josephwecker-v2/src/arch/firmatum/utils/md-press/prompts/unicode-math.txt').read()
sel = [g for g in sorted(items) if items[g]['kind'] == 'line' and g in P[1] and g in P[2]]
inp = ''.join(json.dumps({'gid': g, 'text': items[g]['text']}, ensure_ascii=False) + '\n' for g in sel)
parts = {json.loads(l)['gid']: json.loads(l)['parts'] for l in
         subprocess.run(['rs/probe/target/release/pieces_of'], input=inp, capture_output=True, text=True).stdout.splitlines()}
reqs = []
for g in sel:
    for k, (p, sep) in enumerate(parts[g]):
        if sep:
            continue
        core = p.strip(' \t')
        body = {"model": "llama3.2:3b", "prompt": inst + "Input: " + core + "\nOutput:", "stream": False,
                "options": {"temperature": 0, "stop": ["\n"], "num_predict": 64 + len(core) // 2}}
        v = json.load(urllib.request.urlopen(urllib.request.Request('http://localhost:11434/api/generate',
                      json.dumps(body).encode(), {'Content-Type': 'application/json'}), timeout=600))
        first = next((l.strip() for l in v['response'].split('\n') if l.strip()), '')
        reqs.append({'gid': f'{g}#{k}', 'text': p, 'proposal': first})
jin = ''.join(json.dumps(r, ensure_ascii=False) + '\n' for r in reqs)
judged = {json.loads(l)['gid']: json.loads(l) for l in
          subprocess.run(['rs/probe/target/release/judge'], input=jin, capture_output=True, text=True).stdout.splitlines()}
with open('data/llama-D-lines.jsonl', 'w') as f:
    for g in sel:
        out = ''.join(p if sep else judged[f'{g}#{k}']['result'] for k, (p, sep) in enumerate(parts[g]))
        f.write(json.dumps({'gid': g, 'text': items[g]['text'], 'out': out}, ensure_ascii=False) + '\n')
print(len(sel), 'lines,', len(reqs), 'model calls')
