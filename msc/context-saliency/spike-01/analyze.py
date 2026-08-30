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
    for room in range(1, n_rooms_of(r) + 1):
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
    roles = ("header", "interior", "count", "terminal") if has_header_role(r) else ("interior", "terminal")
    for room in range(1, n_rooms_of(r) + 1):
        for role in roles:
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


def n_rooms_of(r):
    return len(r["codes"])


def has_header_role(r):
    return any(sr[0] == "header" for sr in r["seg_roles"])


def p1_roles_report(r):
    """v4: header / interior(body) / count / terminal per-token T2-allhead."""
    groups = seg_groups(r)
    mass = per_token_mass(r)
    n_steps = len(mass)
    n_rooms = n_rooms_of(r)
    roles = ("header", "interior", "count", "terminal")
    print(f"\n== P1-roles T2-allhead [{r['variant']} seed={r['seed']}]  "
          f"answer={r['answer']} got={r['got']}  steps={n_steps} ctx={r['n_ctx']}")
    hdr = " ".join(f"{role:>12}" for role in roles)
    print(f"{'rm':>3} {hdr} {'T/body':>8} {'H/body':>8}")
    tb, hb = [], []
    for room in range(1, n_rooms + 1):
        ptok = {}
        for role in roles:
            idxs = [(i, n) for (i, n, _) in groups.get((role, room), [])]
            ntok = sum(n for _, n in idxs) or 1
            m = (sum(mass[t][i] for t in range(n_steps) for i, _ in idxs) / n_steps) if idxs else 0.0
            ptok[role] = m / ntok
        t_over_b = ptok["terminal"] / ptok["interior"] if ptok["interior"] else float("nan")
        h_over_b = ptok["header"] / ptok["interior"] if ptok["interior"] else float("nan")
        tb.append(t_over_b)
        hb.append(h_over_b)
        vals = " ".join(f"{ptok[role]:12.6f}" for role in roles)
        print(f"{room:3d} {vals} {t_over_b:8.2f} {h_over_b:8.2f}")
    print(f"  mean T/body={sum(tb)/len(tb):.2f}  H/body={sum(hb)/len(hb):.2f}  (T2-allhead, pre-filter)")


def t1_temporal_pair(r_s, r_d):
    """Matched-pair event alignment: delayed minus screened at the same step index.

    Uses per-layer z-scored g (global window — traces are <64 steps so rolling-64
    cannot run). Reports mean Δ at phase-boundary-adjacent steps vs elsewhere,
    and in the final quarter (where the reuse element is emitted).
    Bands are NOT the EpiKV 7–13/18–25 inherit — mean over all layers after z.
    """
    def zg(r):
        gs = [s["g"] for s in r["steps"]]
        T, L = len(gs), len(gs[0])
        cols = list(zip(*gs))
        means = [sum(c) / T for c in cols]
        stds = [max((sum((x - m) ** 2 for x in c) / T) ** 0.5, 1e-8) for c, m in zip(cols, means)]
        return [[(gs[t][l] - means[l]) / stds[l] for l in range(L)] for t in range(T)]

    zs, zd = zg(r_s), zg(r_d)
    n = min(len(zs), len(zd))
    # drop embed (layer 0); mean over transformer layers
    dmean = [sum(zd[t][l] - zs[t][l] for l in range(1, len(zs[t]))) / (len(zs[t]) - 1)
             for t in range(n)]
    phases = phase_of_steps(r_s)
    bounds = [i for i in range(1, min(len(phases), n)) if phases[i] != phases[i - 1]]
    near, far = [], []
    for t, d in enumerate(dmean):
        (near if any(abs(t - b) <= 1 for b in bounds) else far).append(d)
    last_q = max(1, n // 4)
    q = dmean[-last_q:]
    def avg(x):
        return sum(x) / len(x) if x else float("nan")
    print(f"\n== T1-temporal matched-pair Δ(delayed−screened) seed={r_s['seed']} n={n}")
    print(f"  near-boundary={avg(near):+.4f} (n={len(near)})  elsewhere={avg(far):+.4f} (n={len(far)})")
    print(f"  final-quarter={avg(q):+.4f} (n={len(q)})  (reuse element lives here on the delayed side)")
    print(f"  whole-trace={avg(dmean):+.4f}")
    if "entropy" in r_s["steps"][0]:
        es = [s["entropy"] for s in r_s["steps"][:n]]
        ed = [s["entropy"] for s in r_d["steps"][:n]]
        de = [ed[t] - es[t] for t in range(n)]
        print(f"  entropy Δ whole={avg(de):+.4f}  final-quarter={avg(de[-last_q:]):+.4f}")
    return {"near": avg(near), "far": avg(far), "quarter": avg(q),
            "delta_nf": avg(near) - avg(far) if near and far else float("nan")}


def t1_spatial_roles(r):
    """T1-spatial per-role means (prefill hidden-diff and KV var), n_rooms rooms.

    Raw (not position-detrended) — the positional trend is expected; this is a
    well-formedness + ordering preview, not a calibration verdict.
    """
    sp = r.get("t1_spatial")
    if not sp:
        return
    groups = seg_groups(r)
    n_rooms = n_rooms_of(r)
    gmean = sp["g_prefill_seg_mean"]  # [L+1][n_seg]
    n_seg = len(gmean[0])
    # mean over transformer layers (skip embed 0)
    g_tok = [sum(gmean[l][j] for l in range(1, len(gmean))) / (len(gmean) - 1)
             for j in range(n_seg)]
    kmean = sp["k_var_seg_mean"]  # [L][n_seg]
    k_tok = [sum(kmean[l][j] for l in range(len(kmean))) / len(kmean) for j in range(n_seg)]
    print(f"\n== T1-spatial raw (not detrended) [{r['variant']} seed={r['seed']}]")
    print(f"{'rm':>3} {'g_header':>10} {'g_body':>10} {'g_count':>10} {'g_term':>10} "
          f"{'k_header':>10} {'k_body':>10} {'k_term':>10}")
    for room in range(1, n_rooms + 1):
        def role_mean(sig, role):
            idxs = [i for (i, n, _) in groups.get((role, room), [])]
            if not idxs:
                return float("nan")
            # per-token: the stored values are already per-token means inside the segment
            return sum(sig[i] for i in idxs) / len(idxs)
        print(f"{room:3d} {role_mean(g_tok,'header'):10.4f} {role_mean(g_tok,'interior'):10.4f} "
              f"{role_mean(g_tok,'count'):10.4f} {role_mean(g_tok,'terminal'):10.4f} "
              f"{role_mean(k_tok,'header'):10.6f} {role_mean(k_tok,'interior'):10.6f} "
              f"{role_mean(k_tok,'terminal'):10.6f}")


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
            room = r["reuse_room"]
            # v4: count is its own role. v3: 3rd interior line of the room (header, distractor, count).
            count_idxs = [i for i, (role, rm, *rest) in enumerate(r["seg_roles"])
                          if role == "count" and rm == room]
            if count_idxs:
                idxs = [(count_idxs[0], r["seg_roles"][count_idxs[0]][3])]
            else:
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


def _spearman(xs, ys):
    def ranks(a):
        n = len(a)
        order = sorted(range(n), key=lambda i: a[i])
        r = [0.0] * n
        i = 0
        while i < n:
            j = i
            while j + 1 < n and a[order[j + 1]] == a[order[i]]:
                j += 1
            avg = (i + j) / 2 + 1
            for k in range(i, j + 1):
                r[order[k]] = avg
            i = j + 1
        return r
    if len(xs) < 3:
        return float("nan")
    rx, ry = ranks(xs), ranks(ys)
    n = len(xs)
    mx, my = sum(rx) / n, sum(ry) / n
    num = sum((a - mx) * (b - my) for a, b in zip(rx, ry))
    den = (sum((a - mx) ** 2 for a in rx) * sum((b - my) ** 2 for b in ry)) ** 0.5
    return num / den if den else float("nan")


def _band(xs):
    xs = [x for x in xs if x == x]
    if not xs:
        return float("nan"), float("nan"), float("nan")
    xs = sorted(xs)
    return xs[0], xs[len(xs) // 2], xs[-1]


def _spatial_score(r, roles, room):
    sp = r.get("t1_spatial") or {}
    gmean = sp.get("g_prefill_seg_mean")
    if not gmean:
        return float("nan")
    n_seg = len(gmean[0])
    g_tok = [sum(gmean[l][j] for l in range(1, len(gmean))) / (len(gmean) - 1)
             for j in range(n_seg)]
    groups = seg_groups(r)
    idxs = []
    for role in roles:
        idxs.extend(i for (i, n, _) in groups.get((role, room), []))
    if not idxs:
        return float("nan")
    return sum(g_tok[i] for i in idxs) / len(idxs)


def gate_summary(runs, occ=None):
    """Calibration-gate numbers: worst-case bands, T2 labeled pre-filter."""
    print("\n======== CALIBRATION GATE ========")
    temporal = []
    t2_tb = []
    by_key = {}
    for r in runs.values():
        by_key.setdefault((r["seed"], r.get("placement", "chrono")), {})[r["variant"]] = r
        if has_header_role(r):
            groups = seg_groups(r)
            mass = per_token_mass(r)
            n_steps = len(mass)
            for room in range(1, n_rooms_of(r) + 1):
                def ptok(role):
                    idxs = [(i, n) for (i, n, _) in groups.get((role, room), [])]
                    ntok = sum(n for _, n in idxs) or 1
                    m = (sum(mass[t][i] for t in range(n_steps) for i, _ in idxs) / n_steps) if idxs else 0.0
                    return m / ntok
                b, t = ptok("interior"), ptok("terminal")
                if b:
                    t2_tb.append(t / b)
    for key, pair in by_key.items():
        if len(pair) == 2:
            rec = t1_temporal_pair(pair["screened"], pair["delayed_reuse"])
            rec["key"] = key
            temporal.append(rec)
    if temporal:
        nfs = [t["delta_nf"] for t in temporal]
        qs = [t["quarter"] for t in temporal]
        lo, mid, hi = _band(nfs)
        qlo, qmid, qhi = _band(qs)
        print(f"T1-temporal Δ(near-boundary − elsewhere) band min/median/max={lo:+.4f}/{mid:+.4f}/{hi:+.4f} n_pairs={len(nfs)}")
        print(f"T1-temporal final-quarter Δ(delayed−screened) band min/median/max={qlo:+.4f}/{qmid:+.4f}/{qhi:+.4f}")
        # beat-random: does the sign of Δ_nf agree across pairs more than 50%?
        pos = sum(1 for x in nfs if x > 0)
        print(f"T1-temporal sign(near>far)={pos}/{len(nfs)} (random≈50%)")
    if t2_tb:
        lo, mid, hi = _band(t2_tb)
        print(f"T2-allhead (pre-filter) terminal/body per-room band min/median/max={lo:.2f}/{mid:.2f}/{hi:.2f} n={len(t2_tb)}")
    if occ:
        xs, ys, names = [], [], []
        runs_by = {(r["seed"], r.get("placement", "chrono"), r["variant"]): r for r in runs.values()}
        cond_roles = {
            "interior_body": (["interior"], "ctrl"),
            "header_ctrl": (["header"], "ctrl"),
            "narrative_ctrl": (["header", "interior", "count"], "ctrl"),
            "terminal_ctrl": (["terminal"], "ctrl"),
            "reuse_line": (["count"], "reuse"),
            "count_matched": (["count"], "reuse"),
        }
        n_infer = 0
        for row in occ:
            if row.get("skipped"):
                continue
            r = runs_by.get((row["seed"], row.get("placement", "chrono"), row["variant"]))
            if not r:
                continue
            n_infer += 1
            ctrl = row.get("ctrl_room")
            reuse = row.get("reuse_room")
            for cond, (roles, which) in cond_roles.items():
                if cond not in row or row[cond] is None:
                    continue
                room = ctrl if which == "ctrl" else reuse
                if room is None:
                    continue
                flipped = 1.0 if row[cond] != row["answer"] else 0.0
                score = _spatial_score(r, roles, room)
                if score == score:
                    xs.append(score)
                    ys.append(flipped)
                    names.append(cond)
        print(f"T3 inferred on {n_infer} base-correct rows; paired scores n={len(xs)}")
        if xs:
            rho = _spearman(xs, ys)
            # top/bottom 20% masking-style: mean flip rate
            n = len(xs)
            order = sorted(range(n), key=lambda i: xs[i])
            k = max(1, n // 5)
            bot = [ys[i] for i in order[:k]]
            top = [ys[i] for i in order[-k:]]
            def avg(a):
                return sum(a) / len(a)
            print(f"T1-spatial vs T3 Spearman ρ={rho:.3f} n={n}")
            print(f"T1-spatial top20% flip-rate={avg(top):.3f}  bottom20% flip-rate={avg(bot):.3f}  (k={k})")
            print(f"  (T2-allhead is pre-filter; this Spearman is g_prefill vs occlusion flip)")
        else:
            print("T1-spatial vs T3: no paired scores (too few base-correct or missing ctrl_room)")
    else:
        print("T1-spatial vs T3: no occlusion file")
    print("======== END GATE ========")


if __name__ == "__main__":
    files = sorted(glob.glob("out/v4instr_*.json")) or sorted(glob.glob("out/instr_*.json"))
    files = [f for f in files if "occlusion" not in f and "smoke" not in f]
    runs = {f: load(f) for f in files if "occlusion" not in f}
    for f, r in runs.items():
        if has_header_role(r):
            p1_roles_report(r)
            t1_spatial_roles(r)
        else:
            p1_report(r)
        p1_phase_report(r)
        p2_report(r)
    by_key = {}
    for r in runs.values():
        key = (r["seed"], r.get("placement", "chrono"))
        by_key.setdefault(key, {})[r["variant"]] = r
    for key, pair in by_key.items():
        if len(pair) == 2:
            print(f"\n== matched-pair reuse contrast, seed={key[0]} placement={key[1]}")
            reuse_contrast(pair["screened"], pair["delayed_reuse"])
            if "g" in pair["screened"]["steps"][0]:
                t1_temporal_pair(pair["screened"], pair["delayed_reuse"])
    occ = None
    for occ_path in ("out/occlusion_v4.json", "out/occlusion.json"):
        try:
            occ = load(occ_path)
            print(f"\n== occlusion sweep ({occ_path}) n={len(occ)}")
            for row in occ:
                print(row)
            break
        except FileNotFoundError:
            pass
    gate_summary(runs, occ)
