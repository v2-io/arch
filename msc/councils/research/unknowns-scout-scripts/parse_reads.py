import glob,re,collections,sys
C=collections.Counter(); per={}
ACTS=("rederive","read-check","run","instantiate","read","assert","decline")
for p in glob.glob(sys.argv[1]+"/*.txt"):
    s=p.split("/")[-1][:-4]; L=open(p).read().split("\n")
    rows=[]
    for ln in L:
        m=re.match(r'^(✓~|✓|↻|⚠|\?) (\S+) (\S+)',ln)
        if not m: continue
        g,act,out=m.groups()
        if act not in ACTS: 
            C[("nonCheck",g,act)]+=1; continue
        stale=re.search(r'stale: ([^·]*)',ln)
        st=stale.group(1).strip() if stale else ""
        indep = " as integrator " not in ln
        rows.append((g,act,out,st,indep,ln))
        C[(g,out,"indep" if indep else "self", "own" if "own" in st else ("upstream" if "upstream" in st else st or "-"))]+=1
    per[s]=rows
for k,v in sorted(C.items(),key=lambda x:-x[1]): print(v,k)
print("segments with any check row:",sum(1 for r in per.values() if r))
print("segments with a holding (✓) check:",sum(1 for r in per.values() if any(x[0]=='✓' for x in r)), [s for s,r in per.items() if any(x[0] in('✓','✓~') for x in r)])
