"""llama3.2:3b proposals for gold items (same request as md-press / the coordinator's propose.py)."""
import json, sys, time, urllib.request
inst = open('/Users/josephwecker-v2/src/arch/firmatum/utils/md-press/prompts/unicode-math.txt').read()
items = json.load(open('data/gold/all-items.json'))
sel = sys.argv[1]
out = open(f'data/llama-{sel}.jsonl', 'w')
for it in items:
    if it['gid'][0] not in sel:
        continue
    core = it['text'].strip(' \t')
    body = {"model": "llama3.2:3b", "prompt": inst + "Input: " + core + "\nOutput:", "stream": False,
            "options": {"temperature": 0, "stop": ["\n"], "num_predict": 64 + len(core) // 2}}
    t = time.time()
    v = json.load(urllib.request.urlopen(urllib.request.Request('http://localhost:11434/api/generate', json.dumps(body).encode(), {'Content-Type': 'application/json'}), timeout=600))
    first = next((l.strip() for l in v['response'].split('\n') if l.strip()), '')
    out.write(json.dumps({'gid': it['gid'], 'text': it['text'], 'proposal': first, 'secs': round(time.time() - t, 3)}, ensure_ascii=False) + '\n'); out.flush()
print('done')
