"""Spike-01 experiment driver.

Runs, on matched-pair vault tasks:
  A. Instrumented runs (screened + delayed_reuse, same seed) -> per-step
     segment-attention matrices, heavy-hitter sets, hidden-diff signals.
  B. Occlusion sweep (T3 ground truth) over several seeds:
     screened:      occlude one discharged room's interior (expect: answer unchanged)
                    occlude that room's terminal CODE line (expect: answer breaks)
     delayed_reuse: occlude the reuse line (expect: breaks)
                    occlude a different room's interior (expect: unchanged)

Outputs JSON files in ./out/.
"""

import json
import os
import sys
import time
import urllib.request

from tasks import make_task, occlude
from rig import run_instrumented, run_plain, extract_master

os.makedirs("out", exist_ok=True)

MUSE_URL = "http://localhost:8837/v1/chat/completions"


def run_plain_muse(prompt_text, max_new=1800):
    """Behavioral-tier run via llama-server (Muse-Glimmer-30B Q4). Greedy.
    Muse is a thinking model: reasoning streams to `reasoning_content` and the
    final text to `content` — return both so MASTER extraction sees either."""
    req = urllib.request.Request(MUSE_URL, data=json.dumps({
        "messages": [{"role": "user", "content": prompt_text}],
        "temperature": 0, "max_tokens": max_new}).encode(),
        headers={"Content-Type": "application/json"})
    r = json.load(urllib.request.urlopen(req, timeout=900))
    msg = r["choices"][0]["message"]
    return (msg.get("reasoning_content") or "") + "\n" + (msg.get("content") or "")


def occlusion_sweep(seeds=(1, 2, 3, 4, 5, 6), n_rooms=4, runner=run_plain, tag="",
                    placement="chrono", require_base_correct=True):
    results = []
    skipped = 0
    for seed in seeds:
        for variant in ("screened", "delayed_reuse"):
            t = make_task(variant, n_rooms=n_rooms, seed=seed, placement=placement)
            base = extract_master(runner(t.prompt))
            row = {"seed": seed, "variant": variant, "placement": placement,
                   "answer": t.answer, "base": base, "reuse_room": t.reuse_room_draw}
            if require_base_correct and base != t.answer:
                row["skipped"] = "base_incorrect"
                print(f"seed={seed} {variant:13s} SKIP base={base} expect={t.answer}", flush=True)
                skipped += 1
                results.append(row)
                continue
            # choose a discharged room that is NOT the reuse room for the control occlusion
            ctrl_room = next(r for r in range(1, n_rooms + 1) if r != t.reuse_room_draw)
            # v4 roles: interior = body (header preserved); narrative = header+body+count
            conds = [("interior_body", ("interior", ctrl_room)),
                     ("header_ctrl", ("header", ctrl_room)),
                     ("narrative_ctrl", ("narrative", ctrl_room)),
                     ("terminal_ctrl", ("terminal", ctrl_room))]
            if variant == "delayed_reuse":
                conds.append(("reuse_line", ("reuse_line", t.reuse_room_draw)))
            else:
                conds.append(("count_matched", ("count", t.reuse_room_draw)))
            for name, target in conds:
                got = extract_master(runner(occlude(t, target)))
                row[name] = got
                print(f"seed={seed} {variant:13s} {name:18s} expect={t.answer} base={base} got={got}", flush=True)
            results.append(row)
    json.dump(results, open(f"out/occlusion{tag}.json", "w"), indent=1)
    print(f"occlusion wrote {len(results)} rows skipped_base_incorrect={skipped}", flush=True)
    return results


def instr_path(out_prefix, placement, variant, seed, n_rooms=4, model_tag=""):
    """v4 4-room 7B keeps `out/v4instr_{placement}_{variant}_{seed}.json`.
    Extra tags (qwen3, 16r, …) go in the stem so the 12-trace mint is not overwritten."""
    parts = [out_prefix]
    if model_tag:
        parts.append(model_tag)
    if n_rooms != 4:
        parts.append(f"{n_rooms}r")
    parts += [placement, variant, str(seed)]
    return "_".join(parts) + ".json"


def instrumented_pair(seed=7, n_rooms=4, out_prefix="out/instr", placement="chrono",
                      model_tag="", surface="narrative"):
    """Never overwrite out/instr_*.json (v3)."""
    paths = []
    for variant in ("screened", "delayed_reuse"):
        path = instr_path(out_prefix, placement, variant, seed, n_rooms, model_tag)
        t = make_task(variant, n_rooms=n_rooms, seed=seed, placement=placement,
                      surface=surface)
        print(f"instrumented {variant} seed={seed} placement={placement} "
              f"n_rooms={n_rooms} tag={model_tag or '-'} ...", flush=True)
        r = run_instrumented(t.prompt, t.segments)
        r["variant"] = variant
        r["seed"] = seed
        r["placement"] = placement
        r["n_rooms"] = n_rooms
        r["task"] = "walk"
        r["model"] = os.environ.get("SPIKE_MODEL", "")
        r["answer"] = t.answer
        r["got"] = extract_master(r["gen_text"])
        r["codes"] = t.codes
        r["reuse_room"] = t.reuse_room_draw
        r["room_order"] = t.room_order
        r["surface"] = surface
        json.dump(r, open(path, "w"))
        paths.append(path)
        print(f"  {r['n_steps']} steps in {r['seconds']}s; answer {r['answer']} got {r['got']} -> {path}", flush=True)
    return paths


if __name__ == "__main__":
    mode = sys.argv[1] if len(sys.argv) > 1 else "all"
    t0 = time.time()
    if mode == "smoke-spatial":
        paths = instrumented_pair(seed=7, out_prefix="out/v4instr", placement="chrono")
        r = json.load(open(paths[0]))
        sp = r["t1_spatial"]
        print("t1_spatial keys", list(sp.keys()), flush=True)
        print("g_prefill_seg_mean layers×segs",
              len(sp["g_prefill_seg_mean"]), "×",
              len(sp["g_prefill_seg_mean"][0]), flush=True)
        print("roles", sorted({sr[0] for sr in r["seg_roles"]}), flush=True)
        print("entropy first/last", r["steps"][0].get("entropy"),
              r["steps"][-1].get("entropy"), flush=True)
    elif mode == "mint":
        # 3 seeds × 2 placements × 2 variants = 12 traces. Walk-task, 4 rooms.
        # Skip pairs already on disk (7B smoke writes seed=7 chrono).
        import glob as _glob
        for seed in (7, 11, 13):
            for placement in ("chrono", "reversed"):
                planned = [f"out/v4instr_{placement}_{v}_{seed}.json"
                           for v in ("screened", "delayed_reuse")]
                if all(os.path.exists(p) for p in planned):
                    print(f"skip existing seed={seed} placement={placement}", flush=True)
                    continue
                instrumented_pair(seed=seed, out_prefix="out/v4instr", placement=placement)
        # T3 on base-correct instrumented instances only (got already recorded).
        occ_rows = []
        skipped = 0
        for path in sorted(_glob.glob("out/v4instr_*.json")):
            r = json.load(open(path))
            t = make_task(r["variant"], n_rooms=r.get("n_rooms", 4),
                          seed=r["seed"], placement=r.get("placement", "chrono"))
            row = {"seed": r["seed"], "variant": r["variant"],
                   "placement": r.get("placement", "chrono"),
                   "answer": r["answer"], "base": r["got"],
                   "reuse_room": r["reuse_room"], "from": path}
            if r["got"] != r["answer"]:
                row["skipped"] = "base_incorrect"
                skipped += 1
                occ_rows.append(row)
                print(f"occl SKIP {path} base={r['got']} expect={r['answer']}", flush=True)
                continue
            ctrl_room = next(x for x in range(1, t.n_rooms + 1) if x != t.reuse_room_draw)
            row["ctrl_room"] = ctrl_room
            conds = [("interior_body", ("interior", ctrl_room)),
                     ("header_ctrl", ("header", ctrl_room)),
                     ("narrative_ctrl", ("narrative", ctrl_room)),
                     ("terminal_ctrl", ("terminal", ctrl_room))]
            if r["variant"] == "delayed_reuse":
                conds.append(("reuse_line", ("reuse_line", t.reuse_room_draw)))
            else:
                conds.append(("count_matched", ("count", t.reuse_room_draw)))
            for name, target in conds:
                got = extract_master(run_plain(occlude(t, target)))
                row[name] = got
                print(f"occl {r['placement']} seed={r['seed']} {r['variant']:13s} {name:18s} "
                      f"base={r['got']} got={got}", flush=True)
            occ_rows.append(row)
        json.dump(occ_rows, open("out/occlusion_v4.json", "w"), indent=1)
        print(f"occlusion_v4 rows={len(occ_rows)} skipped_base_incorrect={skipped}", flush=True)
    elif mode == "formulaic":
        # inversion discriminator: same 2×2, formulaic interiors (Fable 2026-08-29)
        import glob as _glob
        for seed in (7, 11):
            instrumented_pair(seed=seed, out_prefix="out/formulaic", placement="chrono",
                              surface="formulaic")
        occ_rows, skipped = [], 0
        for path in sorted(_glob.glob("out/formulaic_*.json")):
            r = json.load(open(path))
            t = make_task(r["variant"], n_rooms=r.get("n_rooms", 4),
                          seed=r["seed"], placement=r.get("placement", "chrono"),
                          surface="formulaic")
            row = {"seed": r["seed"], "variant": r["variant"], "placement": r.get("placement", "chrono"),
                   "surface": "formulaic", "answer": r["answer"], "base": r["got"],
                   "reuse_room": r["reuse_room"], "from": path}
            if r["got"] != r["answer"]:
                row["skipped"] = "base_incorrect"
                skipped += 1
                occ_rows.append(row)
                print(f"occl SKIP {path} base={r['got']} expect={r['answer']}", flush=True)
                continue
            ctrl_room = next(x for x in range(1, t.n_rooms + 1) if x != t.reuse_room_draw)
            row["ctrl_room"] = ctrl_room
            conds = [("interior_body", ("interior", ctrl_room)),
                     ("header_ctrl", ("header", ctrl_room)),
                     ("narrative_ctrl", ("narrative", ctrl_room)),
                     ("terminal_ctrl", ("terminal", ctrl_room))]
            if r["variant"] == "delayed_reuse":
                conds.append(("reuse_line", ("reuse_line", t.reuse_room_draw)))
            else:
                conds.append(("count_matched", ("count", t.reuse_room_draw)))
            for name, target in conds:
                got = extract_master(run_plain(occlude(t, target)))
                row[name] = got
                print(f"occl formulaic seed={r['seed']} {r['variant']:13s} {name:18s} "
                      f"base={r['got']} got={got}", flush=True)
            occ_rows.append(row)
        json.dump(occ_rows, open("out/occlusion_formulaic.json", "w"), indent=1)
        print(f"occlusion_formulaic rows={len(occ_rows)} skipped={skipped}", flush=True)
    elif mode == "qwen3":
        for seed in (7, 11):
            instrumented_pair(seed=seed, out_prefix="out/qwen3instr", placement="chrono")
    elif mode == "len2k":
        n = int(sys.argv[2]) if len(sys.argv) > 2 else 16
        for seed in (7, 11):
            for placement in ("chrono", "reversed"):
                instrumented_pair(seed=seed, n_rooms=n, out_prefix="out/len2k",
                                  placement=placement)
    elif mode in ("all", "instr"):
        for seed in (7, 11):
            instrumented_pair(seed=seed, out_prefix="out/v4instr", placement="chrono")
    if mode in ("all", "occl"):
        occlusion_sweep()
    if mode in ("all", "occl-muse", "muse"):
        occlusion_sweep(runner=run_plain_muse, tag="_muse")
    print(f"total {time.time()-t0:.0f}s", flush=True)
