import json,difflib,re,collections
R=json.load(open("checkrows.json"))
def strip_cr(t):
    # remove "## Claim Record" sections up to next "## " heading
    return re.sub(r'(?ms)^## Claim Record.*?(?=^## |\Z)','',t)
res={}
for a,v in {(r["about"],r["tv"]) for r in R if r["status"]=="own"}:
    s=a.split(":")[1]
    old=open(f"vers/{s}@{v}.txt").read(); new=open(f"vers/{s}@CUR.txt").read()
    ow=old.split(); nw=new.split()
    sm=difflib.SequenceMatcher(None,ow,nw,autojunk=False)
    add=rem=0
    for op,i1,i2,j1,j2 in sm.get_opcodes():
        if op in("replace","delete"): rem+=i2-i1
        if op in("replace","insert"): add+=j2-j1
    crsame = strip_cr(old).split()==strip_cr(new).split()
    res[(a,v)]=dict(old=len(ow),new=len(nw),add=add,rem=rem,cr_only=crsame,frac=(add+rem)/max(1,len(ow)))
def cls(d):
    if d["cr_only"]: return "A claim-record-only"
    if d["frac"]<0.02: return "B <2% words changed"
    if d["frac"]<0.10: return "C 2-10%"
    if d["frac"]<0.50: return "D 10-50%"
    return "E >=50%"
P=collections.Counter(cls(d) for d in res.values()); print("pairs",sorted(P.items()))
for grp,flt in [("confirmed own-lapsed",lambda r:r["outcome"]=="confirmed"),("confirmed indep own-lapsed",lambda r:r["outcome"]=="confirmed" and r["indep"]),("broken/narrowed own-lapsed",lambda r:r["outcome"]!="confirmed")]:
    rs=[r for r in R if r["status"]=="own" and flt(r)]
    print(grp,len(rs),sorted(collections.Counter(cls(res[(r["about"],r["tv"])]) for r in rs).items()))
json.dump({f"{a}@{v}":d for (a,v),d in res.items()},open("diffs.json","w"),indent=0)
tot_old=sum(d["old"] for d in res.values())
print("median frac",sorted(d["frac"] for d in res.values())[len(res)//2])
