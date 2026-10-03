import json,urllib.request,time,sys
SP=sys.argv[1]; which=sys.argv[2]
inst=open('/Users/josephwecker-v2/src/arch/firmatum/utils/md-press/prompts/unicode-math.txt').read()
sample=json.load(open(f"{SP}/sample.json"))
def post(url,body):
    t=time.time(); r=json.load(urllib.request.urlopen(urllib.request.Request(url,json.dumps(body).encode(),{'Content-Type':'application/json'}),timeout=600)); return r,time.time()-t
out=open(f"{SP}/prop-{which}.jsonl","w")
for r in sample:
    # md-press sends the trimmed core of each piece
    core=r['text'].strip(' \t'); p=inst+"Input: "+core+"\nOutput:"; n=64+len(core)//2
    if which=='llama':
        v,dt=post('http://localhost:11434/api/generate',{"model":"llama3.2:3b","prompt":p,"stream":False,"options":{"temperature":0,"stop":["\n"],"num_predict":n}}); txt=v['response']
    else:
        v,dt=post('http://127.0.0.1:8080/completion',{"prompt":p,"temperature":0,"n_predict":n,"stop":["\n"]}); txt=v['content']
    first=next((l.strip() for l in txt.split('\n') if l.strip()),"")
    out.write(json.dumps({"id":r['id'],"file":r['file'],"text":r['text'],"proposal":first,"secs":round(dt,3)},ensure_ascii=False)+"\n"); out.flush()
print(which,"done")
