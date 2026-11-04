import json,collections,glob,yaml,re,os,subprocess
import os; root=os.environ.get("ASF_ROOT", os.path.expanduser("~/src/arch/asf"))
d=json.load(open("open.json"))["items"]
kinds=collections.Counter(x["kind"] for x in d); print("open rows by kind",kinds, "rows",len(d), "distinct",len({x['item'] for x in d}))
per=collections.defaultdict(collections.Counter)
for x in d:
    t=x.get("target","")
    if t.startswith("segment:"): per[t][x["kind"]]+=1
# segment meta
segs={}
for p in glob.glob(root+"/0*/src/*.md"):
    if "/old-" in p: continue
    s="segment:"+os.path.basename(p)[:-3]; fm=yaml.safe_load(open(p).read().split("---")[1]) or {}
    segs[s]=dict(vol=p[len(root)+1:len(root)+3],type=fm.get("type"),status=fm.get("status"),stage=fm.get("stage"),dep=fm.get("depends") or [],
        same=subprocess.run(["git","-C",root,"diff","--quiet","pre-integration-2026-10-30","HEAD","--",p[len(root)+1:]]).returncode==0)
Y=json.load(open("yrows.json"))
ind=collections.defaultdict(lambda:[0,0]); slf=collections.Counter()
for r in Y:
    if r["indep"]: ind[r["seg"]][0]+=1; ind[r["seg"]][1]+=r["found"]
    else: slf[r["seg"]]+=1
# holding checks from reads
hold=collections.Counter(); 
for p in glob.glob("reads/*.txt"):
    s="segment:"+os.path.basename(p)[:-4]
    for ln in open(p):
        if re.match(r'^✓',ln): hold[s]+=1
rows=[]
for s,m in segs.items():
    o=per.get(s,collections.Counter())
    rows.append(dict(seg=s,**{k:v for k,v in m.items() if k!="dep"},ndep=len(m["dep"]),open=dict(o),openN=sum(o.values()),indep=ind[s][0],indfound=ind[s][1],self=slf[s],hold=hold[s]))
json.dump(rows,open("segrows.json","w"))
print("open row kinds on segments:",sum((collections.Counter(r["open"]) for r in rows),collections.Counter()))
C=collections.Counter()
for r in rows:
    clean = r["openN"]==0
    C[(r["vol"], "unchanged-since-tag" if r["same"] else "changed", "no-open-rows" if clean else "open-rows")]+=1
for k,v in sorted(C.items()): print(v,k)
print("segments with no open rows:",sum(r["openN"]==0 for r in rows),"of",len(rows))
print("no open rows & unchanged since tag:",sum(r["openN"]==0 and r["same"] for r in rows))
print("open rows distribution:",sorted(collections.Counter(min(r['openN'],10) for r in rows).items()))
