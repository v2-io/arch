"""Spike-01 analysis: P1 (terminal vs interior salience of discharged subgoals),
P2 (heavy-hitter turnover vs derivation phase), and the delayed-reuse contrast.

Reads out/instr_*.json produced by run_experiment.py. Text-mode report; no plots
(numbers first — a plotting pass can come later if the numbers earn it).
"""

import json
import glob
import re


def load(path):
    return json.load(open(path))


def seg_groups(r):
    """Index segments by (role, room). seg_roles: [(role, room, reused, n_tokens)]."""
    groups = {}
    for i, (role, room, reused, ntok) in enumerate(r["seg_roles"]):
        groups.setdefault((role, room), []).append((i, ntok, reused))
    return groups


def phase_of_steps(r):
    """Monotone phase: phase(t) = highest k whose code value (or CODE-k mention) has
    appeared in the generated prefix by step t. Phase k covers the span from code k's
    first appearance until code k+1's — 'the derivation is working with room k'."""
    toks = [s["tok"] for s in r["steps"]]
    codes = r["codes"]
    prefix = ""
    first_seen = [None] * len(codes)
    for t, tk in enumerate(toks):
        prefix += tk
        for k in range(1, len(codes) + 1):
            if first_seen[k - 1] is None and (
                    f"CODE-{k}" in prefix or re.search(rf"(?<!\d){codes[k-1]}(?!\d)", prefix)):
                first_seen[k - 1] = t
    phases = []
    for t in range(len(toks)):
        cur = 0
        for k in range(1, len(codes) + 1):
            if first_seen[k - 1] is not None and t >= first_seen[k - 1]:
                cur = max(cur, k)
        phases.append(cur)
    return phases


def per_token_mass(r, layer_mean=True):
    """[step][segment] all-layer-mean attention mass (as stored, mean over heads,
    then mean over layers here)."""
    out = []
    for s in r["steps"]:
        L = s["seg_mass"]                      # [L][n_seg]
        n_layers = len(L)
        n_seg = len(L[0])
        m = [sum(L[l][j] for l in range(n_layers)) / n_layers for j in range(n_seg)]
        out.append(m)
    return out


def p1_report(r):
    groups = seg_groups(r)
    mass = per_token_mass(r)
    n_steps = len(mass)
    print(f"\n== P1 [{r['variant']} seed={r['seed']}]  answer={r['answer']} got={r['got']}  "
          f"steps={n_steps} ctx={r['n_ctx']}")
    print(f"{'room':>4} {'interior/tok':>13} {'terminal/tok':>13} {'ratio T/I':>10}")
    ratios = []
    for room in range(1, 5):
        int_idx = [(i, n) for (i, n, _) in groups.get(("interior", room), [])]
        ter_idx = [(i, n) for (i, n, _) in groups.get(("terminal", room), [])]
        int_tok = sum(n for _, n in int_idx) or 1
        ter_tok = sum(n for _, n in ter_idx) or 1
        int_mass = sum(mass[t][i] for t in range(n_steps) for i, _ in int_idx) / n_steps
        ter_mass = sum(mass[t][i] for t in range(n_steps) for i, _ in ter_idx) / n_steps
        ipt, tpt = int_mass / int_tok, ter_mass / ter_tok
        ratio = tpt / ipt if ipt > 0 else float("inf")
        ratios.append(ratio)
        print(f"{room:>4} {ipt:>13.6f} {tpt:>13.6f} {ratio:>10.2f}")
    print(f"  mean terminal/interior per-token ratio: {sum(ratios)/len(ratios):.2f}")
    return ratios


def p1_phase_report(r):
    """Phase-resolved: per-room per-token attention (interior and terminal) split by
    whether the derivation is before / during / after that room's phase. The decay
    claim is the during->after drop; anticipation would be before-phase elevation."""
    groups = seg_groups(r)
    mass = per_token_mass(r)
    phases = phase_of_steps(r)
    n_steps = len(mass)
    print(f"-- P1-phase [{r['variant']} seed={r['seed']}] (per-token mass x1000)")
    print(f"{'room':>4} {'role':>9} {'before':>8} {'during':>8} {'after':>8} {'during/after':>13}")
    for room in range(1, 5):
        for role in ("interior", "terminal"):
            idxs = [(i, n) for (i, n, _) in groups.get((role, room), [])]
            ntok = sum(n for _, n in idxs) or 1
            bins = {"before": [], "during": [], "after": []}
            for t in range(n_steps):
                b = "before" if phases[t] < room else ("during" if phases[t] == room else "after")
                bins[b].append(sum(mass[t][i] for i, _ in idxs) / ntok)
            def avg(x):
                return 1000 * sum(x) / len(x) if x else float("nan")
            bd, ad = avg(bins["during"]), avg(bins["after"])
            ratio = bd / ad if ad and ad == ad else float("nan")
            print(f"{room:>4} {role:>9} {avg(bins['before']):>8.3f} {bd:>8.3f} {ad:>8.3f} {ratio:>13.2f}")


def p2_report(r):
    steps = r["steps"]
    phases = phase_of_steps(r)
    jac = []
    for a, b in zip(steps, steps[1:]):
        sa, sb = set(a["hh"]), set(b["hh"])
        jac.append(len(sa & sb) / len(sa | sb) if sa | sb else 1.0)
    # boundary steps: where phase changes
    bounds = [i for i in range(1, len(phases)) if phases[i] != phases[i - 1]]
    in_bounds, elsewhere = [], []
    for i, j in enumerate(jac):
        near = any(abs((i + 1) - b) <= 2 for b in bounds)
        (in_bounds if near else elsewhere).append(j)
    def avg(x):
        return sum(x) / len(x) if x else float("nan")
    print(f"== P2 [{r['variant']} seed={r['seed']}] heavy-hitter Jaccard: "
          f"near-boundary={avg(in_bounds):.3f} (n={len(in_bounds)})  "
          f"elsewhere={avg(elsewhere):.3f} (n={len(elsewhere)})  "
          f"boundaries at steps {bounds}")
    return avg(in_bounds), avg(elsewhere)


def reuse_contrast(r_screened, r_delayed):
    """Per-token attention to the reuse line (the counted-items interior line) in
    delayed vs the matched line in screened, during the final phase of derivation."""
    out = {}
    for tag, r in (("screened", r_screened), ("delayed", r_delayed)):
        reuse_idx = [i for i, (role, room, reused, n) in enumerate(r["seg_roles"])
                     if (reused if tag == "delayed" else
                         (role == "interior" and room == r["reuse_room"] and "count" not in ""))]
        if tag == "delayed":
            idxs = [(i, r["seg_roles"][i][3]) for i in reuse_idx]
        else:
            # matched line in screened: the interior line of reuse_room containing the count —
            # identified structurally as the 3rd interior line of that room (count_line_idx=2
            # after the room-name line and one distractor; stable across generator versions
            # only if generator unchanged — assert by token length match with delayed)
            room = r["reuse_room"]
            room_ints = [i for i, (role, rm, _, n) in enumerate(r["seg_roles"])
                         if role == "interior" and rm == room]
            idxs = [(room_ints[2], r["seg_roles"][room_ints[2]][3])]
        mass = per_token_mass(r)
        n_steps = len(mass)
        last_q = max(1, n_steps // 4)  # final quarter of derivation
        m = sum(mass[t][i] for t in range(n_steps - last_q, n_steps) for i, _ in idxs) / last_q
        ntok = sum(n for _, n in idxs) or 1
        out[tag] = m / ntok
        print(f"== reuse-line per-token attention, final quarter [{tag}]: {m/ntok:.6f}")
    if out["screened"] > 0:
        print(f"   delayed/screened ratio: {out['delayed']/out['screened']:.2f}")
    return out


if __name__ == "__main__":
    files = sorted(glob.glob("out/instr_*.json"))
    runs = {f: load(f) for f in files}
    for f, r in runs.items():
        p1_report(r)
        p1_phase_report(r)
        p2_report(r)
    # matched-pair reuse contrast per seed
    by_seed = {}
    for r in runs.values():
        by_seed.setdefault(r["seed"], {})[r["variant"]] = r
    for seed, pair in by_seed.items():
        if len(pair) == 2:
            print(f"\n== matched-pair reuse contrast, seed={seed}")
            reuse_contrast(pair["screened"], pair["delayed_reuse"])
    # occlusion table
    try:
        occ = load("out/occlusion.json")
        print("\n== occlusion sweep")
        for row in occ:
            print(row)
    except FileNotFoundError:
        pass
