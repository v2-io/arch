import json,sys,difflib,collections,statistics
SP=sys.argv[1]
def load(w): return {r['id']:r for r in map(json.loads,open(f"{SP}/judged-{w}.jsonl"))}
L,M=load('llama'),load('muse')
for name,D in [('llama3.2:3b',L),('muse-glimmer-30B',M)]:
    c=collections.Counter(r['outcome'] for r in D.values()); why=collections.Counter(r['reason'] for r in D.values() if r['outcome']=='refused')
    s=[r['secs'] for r in D.values()]
    print(f"{name:18} converted {c['converted']:3}  unchanged {c['unchanged']:3}  refused {c['refused']:3}   median {statistics.median(s):.2f}s  total {sum(s)/60:.1f} min")
    for k,v in why.most_common(): print(f"{'':22}refused: {v:3} {k}")
both=collections.Counter((L[i]['outcome'],M[i]['outcome']) for i in L)
print("\n(llama, muse) outcome pairs:",dict(both))
same=sum(1 for i in L if L[i]['outcome']=='converted' and M[i]['outcome']=='converted' and L[i]['result']==M[i]['result'])
print("both converted identically:",same)
def edits(x,y):
    s=difflib.SequenceMatcher(None,x,y,autojunk=False)
    return " | ".join(f"«{x[p:q]}»→«{y[r:t]}»" for o,p,q,r,t in s.get_opcodes() if o!='equal')
mode=sys.argv[2] if len(sys.argv)>2 else 'diff'
for i in sorted(L):
    l,m=L[i],M[i]
    if mode=='diff' and l['result']==m['result']: continue
    print(f"\n#{i} [{l['outcome']}/{m['outcome']}] {l['text'][:150]}")
    if l['outcome']=='converted': print("   L:",edits(l['text'],l['result'])[:260])
    elif l['outcome']=='refused': print("   L refused:",l['reason'],"|",l['proposal'][:160])
    if m['outcome']=='converted': print("   M:",edits(m['text'],m['result'])[:260])
    elif m['outcome']=='refused': print("   M refused:",m['reason'],"|",m['proposal'][:160])
