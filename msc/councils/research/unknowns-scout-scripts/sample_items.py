# Seeded random sample of open items for the residue estimate (report §5). Prints each item's head.
# Open = not closed by `close` or by a `recheck` whose outcome is not `holds`.
import glob,yaml,random,os
root=os.environ.get("ASF_ROOT", os.path.expanduser("~/src/arch/asf")); os.chdir(root)
closed=set()
for p in glob.glob(".vsect/events/item/*/*.yaml"):
    e=yaml.safe_load(open(p))
    if e["act"]=="close" or (e["act"]=="recheck" and e.get("outcome")!="holds"): closed.add(e["about"])
op=[p for p in glob.glob("items/*.md")+glob.glob("items/imported/*.md") if "item:"+os.path.basename(p)[:-3] not in closed]
print(len(op),"open")
random.seed(20261104)
for p in random.sample(sorted(op),30): print("=====",p); print(open(p).read()[:900])
