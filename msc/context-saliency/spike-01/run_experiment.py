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

from tasks import make_task, occlude
from rig import run_instrumented, run_plain, extract_master

os.makedirs("out", exist_ok=True)


def occlusion_sweep(seeds=(1, 2, 3, 4, 5, 6), n_rooms=4):
    results = []
    for seed in seeds:
        for variant in ("screened", "delayed_reuse"):
            t = make_task(variant, n_rooms=n_rooms, seed=seed)
            base = extract_master(run_plain(t.prompt))
            row = {"seed": seed, "variant": variant, "answer": t.answer, "base": base,
                   "reuse_room": t.reuse_room_draw}
            # choose a discharged room that is NOT the reuse room for the control occlusion
            ctrl_room = next(r for r in range(1, n_rooms + 1) if r != t.reuse_room_draw)
            conds = [("interior_ctrl", ("interior", ctrl_room)),
                     ("terminal_ctrl", ("terminal", ctrl_room))]
            if variant == "delayed_reuse":
                conds.append(("reuse_line", ("reuse_line", t.reuse_room_draw)))
            else:
                # matched line in screened (should be harmless there)
                conds.append(("reuse_line_matched", ("interior", t.reuse_room_draw)))
            for name, target in conds:
                got = extract_master(run_plain(occlude(t, target)))
                row[name] = got
                print(f"seed={seed} {variant:13s} {name:18s} expect={t.answer} base={base} got={got}", flush=True)
            results.append(row)
    json.dump(results, open("out/occlusion.json", "w"), indent=1)
    return results


def instrumented_pair(seed=7, n_rooms=4):
    for variant in ("screened", "delayed_reuse"):
        t = make_task(variant, n_rooms=n_rooms, seed=seed)
        print(f"instrumented {variant} seed={seed} ...", flush=True)
        r = run_instrumented(t.prompt, t.segments)
        r["variant"] = variant
        r["seed"] = seed
        r["answer"] = t.answer
        r["got"] = extract_master(r["gen_text"])
        r["codes"] = t.codes
        r["reuse_room"] = t.reuse_room_draw
        json.dump(r, open(f"out/instr_{variant}_{seed}.json", "w"))
        print(f"  {r['n_steps']} steps in {r['seconds']}s; answer {r['answer']} got {r['got']}", flush=True)


if __name__ == "__main__":
    mode = sys.argv[1] if len(sys.argv) > 1 else "all"
    t0 = time.time()
    if mode in ("all", "instr"):
        instrumented_pair(seed=7)
        instrumented_pair(seed=11)
    if mode in ("all", "occl"):
        occlusion_sweep()
    print(f"total {time.time()-t0:.0f}s", flush=True)
