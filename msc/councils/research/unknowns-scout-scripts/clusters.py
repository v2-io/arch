import re,json,collections,os
import os; root=os.environ.get("ASF_ROOT", os.path.expanduser("~/src/arch/asf"))
chap={}
for v in ("01-aat-core","02-tst-core","03-llm-core","04-eli-core"):
    cur=None
    for ln in open(f"{root}/{v}/OUTLINE.md"):
        m=re.match(r'^(#{2,3}) (.*)',ln)
        if m: cur=v[:2]+" "+re.sub(r'\*','',m.group(2)).strip()[:60]; continue
        if ln.startswith("## Source Material"): cur=None
        if cur and ln.lstrip().startswith("|"):
            cells=ln.split("|")
            for s in re.findall(r"#([a-z0-9][a-z0-9-]+)\]",ln):
                chap.setdefault("segment:"+s,cur)
R=json.load(open("segrows.json"))
G=collections.defaultdict(list)
for r in R: G[chap.get(r["seg"],r["vol"]+" (no outline row)")].append(r)
print(f"{'cluster':62} n  unch fnd q  lap item | indChk indFound hold")
out=[]
for c,rs in G.items():
    f=sum(r["open"].get("finding",0) for r in rs); q=sum(r["open"].get("question",0) for r in rs); l=sum(r["open"].get("lapsed-check",0) for r in rs); it=sum(r["open"].get("item",0) for r in rs)
    ic=sum(r["indep"] for r in rs); ifd=sum(r["indfound"] for r in rs); h=sum(r["hold"] for r in rs); un=sum(r["same"] for r in rs)
    out.append((c,len(rs),un,f,q,l,it,ic,ifd,h))
for o in sorted(out): print(f"{o[0][:62]:62} {o[1]:2} {o[2]:3} {o[3]:3} {o[4]:2} {o[5]:3} {o[6]:4} | {o[7]:4} {o[8]:4} {o[9]:3}")
json.dump({r["seg"]:chap.get(r["seg"]) for r in R},open("chap.json","w"))
