#!/usr/bin/env python3
"""Inventory ASF Claude Code transcripts across ~/.claude and every ~/.claude.bak.* snapshot.

The ASF project dir was renamed twice (agentic-systems -> archema-io-asf -> arch-asf), so all
three lineages are matched. Copies of one session recur across snapshots; the largest copy
of each session id wins. Main sessions and subagent transcripts are counted separately.

Dialogue chars = text a human or model wrote (user text, assistant text); tool calls and tool
results are counted apart, since a dream input built from these would likely strip or compress them.

Before the 2026-03-16 rename the repo was `act`, so that lineage is included too.

Usage: inventory.py [--with-program-root] [--json OUT]
"""
import glob, json, os, re, sys
from collections import defaultdict

HOME = os.path.expanduser("~")
ROOTS = [f"{HOME}/.claude"] + sorted(glob.glob(f"{HOME}/.claude.bak*"))
LINEAGE = re.compile(r"-v2-src-(act|agentic-systems|archema-io-asf|arch-asf)(-|$)")
EXTRA = []
if "--with-program-root" in sys.argv:  # sessions started at ~/src/arch (or archema-io): mixed ASF + cross-member work
    EXTRA.append(r"-v2-src-(arch|archema-io)$")
if "--with-tft" in sys.argv:  # the pre-AAT TFT bridge work
    EXTRA.append(r"-v2-src-agentic-tft$")
if EXTRA:
    LINEAGE = re.compile("|".join([LINEAGE.pattern] + EXTRA))

def scan(path):
    s = dict(first=None, last=None, user_chars=0, asst_chars=0, tool_chars=0, user_turns=0, asst_turns=0)
    with open(path, errors="replace") as f:
        for line in f:
            try:
                r = json.loads(line)
            except ValueError:
                continue
            ts = r.get("timestamp")
            if ts:
                s["first"] = min(s["first"] or ts, ts); s["last"] = max(s["last"] or ts, ts)
            t = r.get("type"); msg = r.get("message") or {}
            if t not in ("user", "assistant"):
                continue
            c = msg.get("content")
            blocks = [{"type": "text", "text": c}] if isinstance(c, str) else (c or [])
            for b in blocks:
                if not isinstance(b, dict):
                    continue
                bt = b.get("type")
                if bt == "text":
                    n = len(b.get("text") or "")
                    if t == "user": s["user_chars"] += n; s["user_turns"] += 1
                    else: s["asst_chars"] += n; s["asst_turns"] += 1
                elif bt in ("tool_use", "tool_result"):
                    s["tool_chars"] += len(json.dumps(b.get("input") if bt == "tool_use" else b.get("content")))
    return s

best = {}  # (kind, session_id) -> (size, path)
for root in ROOTS:
    for d in glob.glob(f"{root}/projects/*"):
        if not LINEAGE.search(os.path.basename(d)):
            continue
        for p in glob.glob(f"{d}/*.jsonl"):
            key = ("main", os.path.basename(p)[:-6])
            sz = os.path.getsize(p)
            if sz > best.get(key, (-1,))[0]: best[key] = (sz, p)
        for p in glob.glob(f"{d}/*/subagents/*.jsonl"):
            key = ("sub", os.path.basename(p)[:-6])
            sz = os.path.getsize(p)
            if sz > best.get(key, (-1,))[0]: best[key] = (sz, p)

rows = []
for (kind, sid), (sz, p) in best.items():
    s = scan(p); s.update(kind=kind, session=sid, bytes=sz, path=p)
    rows.append(s)

agg = defaultdict(lambda: defaultdict(int))
for r in rows:
    a = agg[r["kind"]]
    a["n"] += 1
    for k in ("bytes", "user_chars", "asst_chars", "tool_chars"): a[k] += r[k]
    if r["user_turns"] + r["asst_turns"] == 0: a["empty"] += 1

dated = sorted(r["first"] for r in rows if r["kind"] == "main" and r["first"])
print(f"snapshot roots scanned: {len(ROOTS)}")
if dated: print(f"main-session span: {dated[0][:10]} .. {max(r['last'] for r in rows if r['last'])[:10]}")
for kind in ("main", "sub"):
    a = agg[kind]
    dial = a["user_chars"] + a["asst_chars"]
    print(f"{kind:4} sessions={a['n']:5} (empty {a['empty']})  raw={a['bytes']/2**20:8.1f} MB  "
          f"dialogue={dial/1e6:7.2f} Mchar (~{dial/4/1e6:5.2f} Mtok @4c/t)  tool={a['tool_chars']/1e6:7.2f} Mchar")

by_month = defaultdict(lambda: [0, 0])
for r in rows:
    if r["kind"] == "main" and r["first"]:
        m = r["first"][:7]; by_month[m][0] += 1; by_month[m][1] += r["user_chars"] + r["asst_chars"]
print("main sessions by month (count, dialogue Mchar):")
for m in sorted(by_month): print(f"  {m}  {by_month[m][0]:4}  {by_month[m][1]/1e6:6.2f}")

if "--by-project" in sys.argv:
    # A session counts once, under the project dir of the copy kept (the largest across snapshots).
    proj = lambda r: re.sub(r"^-Users-josephwecker-v2", "", (r["path"].split("/projects/")[1]).split("/")[0])
    tab = defaultdict(lambda: defaultdict(int))
    for r in rows:
        t = tab[proj(r)]; k = r["kind"]
        t[k + "_n"] += 1; t[k + "_bytes"] += r["bytes"]; t[k + "_dial"] += r["user_chars"] + r["asst_chars"]
        t["first"] = min(t["first"] or "9", r["first"] or "9") if t["first"] else (r["first"] or "")
        t["last"] = max(t["last"] or "", r["last"] or "")
    print(f"\n{'project dir':40} {'main':>5} {'MB':>7} {'dlg Mtok':>8} | {'sub':>5} {'MB':>7} {'dlg Mtok':>8} | span")
    tot = defaultdict(int)
    for p in sorted(tab, key=lambda p: tab[p]["first"] or ""):
        t = tab[p]
        for k in ("main_n", "main_bytes", "main_dial", "sub_n", "sub_bytes", "sub_dial"): tot[k] += t[k]
        print(f"{p:40} {t['main_n']:5} {t['main_bytes']/2**20:7.1f} {t['main_dial']/4e6:8.2f} | "
              f"{t['sub_n']:5} {t['sub_bytes']/2**20:7.1f} {t['sub_dial']/4e6:8.2f} | {(t['first'] or '')[:10]} .. {t['last'][:10]}")
    print(f"{'TOTAL':40} {tot['main_n']:5} {tot['main_bytes']/2**20:7.1f} {tot['main_dial']/4e6:8.2f} | "
          f"{tot['sub_n']:5} {tot['sub_bytes']/2**20:7.1f} {tot['sub_dial']/4e6:8.2f} |")

if "--json" in sys.argv:
    out = sys.argv[sys.argv.index("--json") + 1]
    json.dump(sorted(rows, key=lambda r: r["first"] or ""), open(out, "w"), indent=1)
    print(f"wrote {out}")
