"""Lazy differential recall on real infrastructure — pgvector edition.

The harness-side shape of Joseph's C\\D construction (2026-08-29):
  chunks(source, session, seq, role, meta, text, embedding vector(1024))
  source 'A' = pre-compaction full context (highest-fidelity, harness-held)
  source 'B' = live post-compaction context (re-ingested as it grows: B, B_2, ...)
Per user turn: embed the turn, ANN-search both sources, and report every strong
A-hit whose best B-side counterpart is weak — a hole relevant to THIS turn,
with its provenance (seq = line address into A) as the pointer.

DB: context_saliency_spike (psql-18, pgvector 0.8.6). Embeddings: bge-m3 via
ollama (1024-d). Spike-scale: plain exact search (no index needed at n~50);
an HNSW index is one DDL line away when A gets long.

Run: python3 embed_diff_pg.py   (ingests delayed_reuse seed=7, runs 3 probes)
"""

import json
import math
import subprocess
import urllib.request

from tasks import make_task

DB = "context_saliency_spike"
DIM = 1024


def embed(texts):
    req = urllib.request.Request(
        "http://localhost:11434/api/embed",
        data=json.dumps({"model": "bge-m3", "input": texts}).encode(),
        headers={"Content-Type": "application/json"})
    return json.load(urllib.request.urlopen(req, timeout=300))["embeddings"]


def sql(query, quiet=True):
    r = subprocess.run(["psql-18", "-d", DB, "-qAt", "-c", query],
                       capture_output=True, text=True)
    if r.returncode != 0:
        raise RuntimeError(r.stderr)
    return r.stdout.strip()


def setup():
    sql("""create table if not exists chunks (
             id serial primary key, session text, source char(1),
             seq int, role text, room int, reused bool,
             text text, embedding vector(%d));""" % DIM)


def ingest(session, source, rows):
    """rows: list of (seq, role, room, reused, text)."""
    embs = embed([r[4] for r in rows])
    vals = []
    for (seq, role, room, reused, text), e in zip(rows, embs):
        vec = "[" + ",".join(f"{x:.6f}" for x in e) + "]"
        t = text.replace("'", "''")
        vals.append(f"('{session}','{source}',{seq},'{role}',{room},{str(reused).lower()},'{t}','{vec}')")
    sql("insert into chunks (session,source,seq,role,room,reused,text,embedding) values " + ",".join(vals))


def holes(session, turn_text, k=5, margin_floor=0.0):
    """The differential: top-k A-hits for this turn, each with its best-B
    counterpart similarity; rows with positive margin are candidate holes."""
    e = embed([turn_text])[0]
    vec = "[" + ",".join(f"{x:.6f}" for x in e) + "]"
    out = sql(f"""
      with a_hits as (
        select seq, role, room, reused, text,
               1 - (embedding <=> '{vec}') as sim_a
        from chunks where session='{session}' and source='A'
        order by embedding <=> '{vec}' limit {k}),
      b_best as (
        select max(1 - (embedding <=> '{vec}')) as sim_b
        from chunks where session='{session}' and source='B')
      select a.seq, a.role, a.room, a.reused, a.sim_a,
             b.sim_b, a.sim_a - b.sim_b as margin, a.text
      from a_hits a, b_best b
      where a.sim_a - b.sim_b > {margin_floor}
      order by margin desc;""")
    rows = []
    for line in out.splitlines():
        if line.count("|") < 7:
            continue
        seq, role, room, reused, sa, sb, mg, text = line.split("|", 7)
        rows.append(dict(seq=int(seq), role=role, room=int(room), reused=reused == "t",
                         sim_a=float(sa), sim_b=float(sb), margin=float(mg), text=text))
    return rows


def holes_expanded(session, turn_text, k=5, margin_floor=0.0, radius=3):
    """Neighborhood expansion: for each positive-margin hole, pull the +-radius
    adjacent A-chunks (same locale in the original context). Recovers holes whose
    own wording is query-distant but whose neighbors are query-near — the
    mid-topicality band. Returns (direct_holes, expanded_neighbors)."""
    direct = holes(session, turn_text, k, margin_floor)
    if not direct:
        return direct, []
    seqs = sorted({s for h in direct for s in range(h["seq"] - radius, h["seq"] + radius + 1)})
    out = sql(f"""select seq, role, room, reused, text from chunks
                  where session='{session}' and source='A' and seq in ({','.join(map(str, seqs))})
                  order by seq;""")
    hood = []
    hit_seqs = {h["seq"] for h in direct}
    for line in out.splitlines():
        if line.count("|") < 4:
            continue
        seq, role, room, reused, text = line.split("|", 4)
        if int(seq) not in hit_seqs:
            hood.append(dict(seq=int(seq), role=role, room=int(room), reused=reused == "t", text=text))
    return direct, hood


if __name__ == "__main__":
    setup()
    session = "vault-dr-seed7"
    if sql(f"select count(*) from chunks where session='{session}'") == "0":
        t = make_task("delayed_reuse", 4, seed=7)
        a_rows = [(i, s.role, s.room, s.reused_detail, s.text.strip())
                  for i, s in enumerate(t.segments) if s.text.strip()]
        b_rows = [(i, "summary", k + 1, False, f"CODE-{k+1}: {c}") for i, (k, c) in enumerate(enumerate(t.codes))]
        b_rows.append((len(b_rows), "summary", 0, False, "Completed exploring 4 rooms; codes recorded."))
        ingest(session, "A", a_rows)
        ingest(session, "B", b_rows)
        print(f"ingested A={len(a_rows)} B={len(b_rows)} chunks")
    t = make_task("delayed_reuse", 4, seed=7)
    furn = ["cabinet", "chest", "drawer", "shelf"][t.reuse_room_draw - 1]
    for name, q in {
        "high-topicality": f"How many {furn}s did you count in Room {t.reuse_room_draw}?",
        "mid-topicality": f"What was the item count you took in room {t.reuse_room_draw}?",
        "low-topicality": "Don't forget to append the final element we discussed to the master sequence.",
    }.items():
        print(f"\n== {name}: {q!r}")
        found = holes(session, q, k=5)
        if not found:
            print("   (no positive-margin holes)")
        for h in found[:3]:
            tag = "  <<< dropped line" if h["reused"] else ""
            print(f"   hole margin {h['margin']:+.3f} (A {h['sim_a']:.3f} / B {h['sim_b']:.3f}) "
                  f"[{h['role']} r{h['room']}] A:{h['seq']}  {h['text'][:55]!r}{tag}")
