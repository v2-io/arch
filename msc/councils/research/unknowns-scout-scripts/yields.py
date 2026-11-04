import glob,yaml,json,collections,subprocess,re,os
import os; root=os.environ.get("ASF_ROOT", os.path.expanduser("~/src/arch/asf")); TAG="pre-integration-2026-10-30"
ev=[yaml.safe_load(open(p)) for p in glob.glob(root+"/.vsect/events/*/*/*.yaml")]
ev.sort(key=lambda e:e["at"])
SEC=("rederive","read-check","run","instantiate","read","assert","decline")
path={}
for p in glob.glob(root+"/0*/src/*.md"): path["segment:"+os.path.basename(p)[:-3]]=p[len(root)+1:]
fm={}
for s,p in path.items():
    t=open(root+"/"+p).read().split("---")
    try: fm[s]=yaml.safe_load(t[1]) or {}
    except Exception: fm[s]={}
cache={}
def tagtext(seg,commit):
    k=(seg,commit)
    if k in cache: return cache[k]
    p=path.get(seg)
    if not p: cache[k]=None; return None
    r=subprocess.run(["git","-C",root,"diff","--quiet",TAG,commit,"--",p]); cache[k]=(r.returncode==0)
    return cache[k]
def indep(e):
    r=e["actor"].get("role") or ""; return not ("integrator" in r or "author" in r or "adopter" in r)
rows=[]; seen_rev=collections.defaultdict(list)
for e in ev:
    if e["act"] not in SEC: continue
    seg=e["about"]; rev=(e.get("from") or "").split("#")[0]
    if indep(e) and rev not in seen_rev[seg]: seen_rev[seg].append(rev)
    rank=seen_rev[seg].index(rev)+1 if indep(e) and rev in seen_rev[seg] else 0
    f=fm.get(seg,{})
    rows.append(dict(seg=seg,rev=rev,indep=indep(e),rank=rank,found=e["outcome"]!="confirmed",outcome=e["outcome"],line=e.get("line"),
        tagtext=tagtext(seg,e.get("at-commit")) if e.get("at-commit") else None,type=f.get("type"),status=f.get("status"),stage=f.get("stage"),
        vol=path.get(seg,"?")[:2],act=e["act"],at=e["at"]))
json.dump(rows,open("yrows.json","w"),default=str)
def rate(rs): 
    n=len(rs); k=sum(r["found"] for r in rs); return f"{k}/{n}={k/n:.2f}" if n else "0/0"
I=[r for r in rows if r["indep"]]; S=[r for r in rows if not r["indep"]]
print("indep",rate(I),"self",rate(S), "self rederive", rate([r for r in S if r["act"]=="rederive"]))
print("indep by rank:",{k:rate([r for r in I if min(r['rank'],4)==k]) for k in (1,2,3,4)})
print("indep on tag text vs sprint text:", "tag",rate([r for r in I if r["tagtext"] is True]),"sprint",rate([r for r in I if r["tagtext"] is False]),"unknown",rate([r for r in I if r["tagtext"] is None]))
print("indep first-review on tag text:",rate([r for r in I if r["tagtext"] is True and r["rank"]==1]))
for key in ("line","type","status","vol"):
    g=collections.defaultdict(list)
    for r in I: g[r[key]].append(r)
    print("indep by",key,{k:rate(v) for k,v in sorted(g.items(),key=lambda x:-len(x[1]))})
# per review (indep): share of reviews that found anything
rv=collections.defaultdict(list)
for r in I: rv[r["rev"]].append(r)
print("indep reviews",len(rv),"with any found",sum(any(x["found"] for x in v) for v in rv.values()))
