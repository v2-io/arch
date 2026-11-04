import glob,yaml,re,collections,sys,json
import os; root=os.environ.get("ASF_ROOT", os.path.expanduser("~/src/arch/asf"))
cur={}
for p in glob.glob("reads/*.txt"):
    h=open(p).readline(); m=re.search(r'v:([0-9a-f]+)',h); cur["segment:"+p.split("/")[-1][:-4]]=m.group(1)
ev=[]
for p in glob.glob(root+"/.vsect/events/*/*/*.yaml"):
    d=yaml.safe_load(open(p)); ev.append(d)
ev.sort(key=lambda e:e["at"])
SEC=("rederive","read-check","run","instantiate","read","assert","decline")
chk=[e for e in ev if e["act"] in SEC]
print("checks",len(chk), collections.Counter(e["about"].split(":")[0] for e in chk))
def same(a,b): return a.startswith(b) or b.startswith(a)
C=collections.Counter(); rows=[]
for e in chk:
    about=e["about"]; b=e.get("basis") or {}
    tv=e.get("target-version") or b.get(about)
    if about not in cur: C["target gone"]+=1; continue
    own = not same(tv,cur[about])
    changed=[k for k,v in b.items() if k!=about and k in cur and not same(v,cur[k])]
    gone=[k for k in b if k not in cur and k!=about]
    read=set((e["actor"].get("read") or []))
    readchanged=[k for k in changed if k in read]
    role=e["actor"].get("role") or ""
    indep = not ("integrator" in role or "author" in role or "adopter" in role)
    status = "own" if own else ("upstream" if (changed or gone) else "binds")
    rs = "own" if own else ("upstream-read" if readchanged else ("upstream-unread-only" if changed or gone else "binds"))
    C[(status,)]+=1; C[("readset",rs)]+=1; C[("indep" if indep else "self",status)]+=1
    rows.append(dict(id=e["id"][:8],about=about,status=status,rs=rs,indep=indep,outcome=e["outcome"],line=e.get("line"),nchanged=len(changed),read=len(read),tv=tv[:8]))
for k,v in sorted(C.items(),key=lambda x:str(x[0])): print(v,k)
json.dump(rows,open("checkrows.json","w"))
# confirm-rewording effect: how many confirm events, outcomes
cr=[e for e in ev if e["act"]=="confirm-rewording"]; print("confirm-rewording",len(cr),collections.Counter(e["outcome"] for e in cr))
print("keys of a confirm event:", list(cr[0].keys()))
