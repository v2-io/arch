"""Lazy differential recall micro-probe (2026-08-29, Joseph's C\\D tack).

A = pre-compaction full context (harness-held, highest fidelity — includes thinking)
B = compaction summary (post-seam agent context)
C = embedded chunks of A; D = embedded chunks of B.
Per user turn: retrieve vs C and D; a strong C-hit with weak best-D match is a
hole relevant to THIS turn — the hole-map computed lazily, no prediction needed.
Repair by prepending content or pointers into A (file:line provenance edges).

First empirical result (bge-m3 via ollama, delayed_reuse seed=7, bad summary =
codes only): high-topicality cue -> dropped line rank-1, margin +0.121 (works);
mid-topicality -> rank-2, margin -0.011 (needs neighborhood expansion);
low-topicality/oblique -> miss (atopical class stays with predictive pinning).
Run: python3 embed_diff_probe.py  (needs ollama serving bge-m3)."""
import json, math, urllib.request
from tasks import make_task

def embed(texts):
    req = urllib.request.Request("http://localhost:11434/api/embed",
        data=json.dumps({"model": "bge-m3", "input": texts}).encode(),
        headers={"Content-Type": "application/json"})
    return json.load(urllib.request.urlopen(req, timeout=120))["embeddings"]

def cos(a, b):
    return sum(x*y for x, y in zip(a, b)) / (math.sqrt(sum(x*x for x in a)) * math.sqrt(sum(x*x for x in b)))

if __name__ == "__main__":
    t = make_task("delayed_reuse", 4, seed=7)
    chunks = [(s.role, s.room, s.reused_detail, s.text.strip()) for s in t.segments if s.text.strip()]
    embC = embed([c[3] for c in chunks])
    embD = embed([f"CODE-{k}: {c}" for k, c in enumerate(t.codes, 1)] + ["Completed exploring 4 rooms; codes recorded."])
    furn = ['cabinet', 'chest', 'drawer', 'shelf'][t.reuse_room_draw - 1]
    queries = {
        "high-topicality": f"How many {furn}s did you count in Room {t.reuse_room_draw}?",
        "mid-topicality": f"What was the item count you took in room {t.reuse_room_draw}?",
        "low-topicality": "Don't forget to append the final element we discussed to the master sequence.",
    }
    for name, q in queries.items():
        qe = embed([q])[0]
        best_d = max(cos(qe, e) for e in embD)
        sims = sorted(((cos(qe, e), i) for i, e in enumerate(embC)), reverse=True)
        print(f"\n== {name}: {q!r}  (best-D {best_d:.3f})")
        for s, i in sims[:3]:
            role, room, reused, txt = chunks[i]
            print(f"   C {s:.3f} margin {s-best_d:+.3f} [{role} r{room}] {txt[:55]!r}{' <<< dropped line' if reused else ''}")
