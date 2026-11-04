import glob,yaml,json,collections
import os; root=os.environ.get("ASF_ROOT", os.path.expanduser("~/src/arch/asf"))
ev=[yaml.safe_load(open(p)) for p in glob.glob(root+"/.vsect/events/segment/*/*.yaml")]
ev.sort(key=lambda e:e["at"])
nxt=collections.defaultdict(dict)
for e in ev:
    if e["act"]=="revise": nxt[e["about"]][e["from-version"][:8]]=(e["own-version"][:8],e.get("change"))
conf={}
for e in ev:
    if e["act"]=="confirm-rewording": conf[(e["about"],e["from-version"][:8],e["to-version"][:8])]=e["outcome"]
R=json.load(open("checkrows.json")); D=json.load(open("diffs.json"))
C=collections.Counter(); C2=collections.Counter()
for r in R:
    if r["status"]!="own": continue
    v=r["tv"]; changes=[]; seen=set()
    while v in nxt[r["about"]] and v not in seen:
        seen.add(v); w,c=nxt[r["about"]][v]; changes.append(c); v=w
    kinds=set(changes)
    k="redefined-in-chain" if "redefined" in kinds else ("unclassified-in-chain" if "unclassified" in kinds else ("reworded-only" if kinds else "no-chain"))
    d=D[f"{r['about']}@{r['tv']}"]
    C[(r["outcome"]=="confirmed", r["indep"], k)]+=1
    if r["outcome"]=="confirmed": C2[(k, "small(<10% or CR)" if (d["cr_only"] or d["frac"]<0.10) else "large")]+=1
for k,v in sorted(C.items(),key=str): print(v,k)
print(); 
for k,v in sorted(C2.items(),key=str): print(v,k)
# also revise counts
print(collections.Counter(e.get("change") for e in ev if e["act"]=="revise"))
