# Which checked versions equal the tag text? Compares each checked version's bare text (vers/)
# with the tag file's body minus frontmatter and Working Notes (tag/), as whitespace tokens.
# Validated: reproduces "equal" on all 75 segments byte-identical between the tag and 2133cf2e.
# Supersedes yields.py's `tagtext` field, which relied on `at-commit` (absent on most events).
import json,re,os,glob,yaml,collections
root=os.environ.get("ASF_ROOT", os.path.expanduser("~/src/arch/asf"))
def body(t):
    b=t.split("---",2)[2] if t.startswith("---") else t
    return re.split(r'(?m)^## Working Notes',b)[0].split()
SEC=("rederive","read-check","run","instantiate","read","assert","decline")
def indep(e):
    r=e["actor"].get("role") or ""; return not ("integrator" in r or "author" in r or "adopter" in r)
ev=[yaml.safe_load(open(p)) for p in glob.glob(root+"/.vsect/events/segment/*/*.yaml")]
C=collections.Counter(); cache={}
for e in [e for e in ev if e["act"] in SEC]:
    s=e["about"].split(":")[1]; v=(e.get("target-version") or e["basis"][e["about"]])[:8]
    if (s,v) not in cache:
        tp=f"tag/{s}.md"; vp=f"vers/{s}@{v}.txt"
        cache[(s,v)]=("new-since-tag" if not os.path.exists(tp) else "?" if not os.path.exists(vp) or os.path.getsize(vp)==0
                      else "tag-text" if body(open(tp).read())==open(vp).read().split() else "changed")
    C[(cache[(s,v)],"indep" if indep(e) else "self","found" if e["outcome"]!="confirmed" else "confirmed")]+=1
for k,n in sorted(C.items()): print(n,k)
print("distinct checked versions:",collections.Counter(cache.values()))
