#!/usr/bin/env python3
"""Calibrate the break classifier and set honest probability bands.

Method: isotonic regression on grouped out-of-fold probabilities (the
standard RF fix — RF probs compress toward the middle), exported as a
piecewise-linear table into model.json (portable: two float arrays).
Validation: the calibrated bands are then checked against the ORGANIC
hand-labeled breaks (AUDIT-2026-07-22.tsv, regenerated positions via the
same seed) — because calibrating on synthetic training data calibrates to
the wrong distribution, and the deployment bands must answer to organic
reality. Wilson intervals report the small-sample honesty.
"""

import json
import random
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from features import blocks_of, break_features, file_stats

import numpy as np
from sklearn.ensemble import RandomForestClassifier
from sklearn.isotonic import IsotonicRegression
from sklearn.model_selection import GroupKFold, cross_val_predict

HERE = Path(__file__).resolve().parent
TRAIN = HERE / "training"


def wilson(k, n, z=1.96):
    if n == 0:
        return (0.0, 1.0)
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * ((p * (1 - p) / n + z * z / (4 * n * n)) ** 0.5) / d
    return (max(0, c - h), min(1, c + h))


def audit_sample_breaks():
    """Reproduce the seeded 2026-07-22 hand-audit sample exactly and join
    verdicts by position. Returns [(features, verdict)] for in-scope rows."""
    random.seed(20260722)
    STRUCT = ('#', '|', '>', '-', '*', '`', '$', '+', '=', '!', '[')

    def breaks(path):
        lines = path.read_text(errors="replace").split("\n")
        out = []
        in_f = False
        for i in range(len(lines) - 1):
            a, b = lines[i], lines[i + 1]
            if a.strip().startswith("```"):
                in_f = not in_f
            if in_f or not a.strip() or not b.strip():
                continue
            sa, sb = a.strip(), b.strip()
            if sa.startswith(STRUCT) or sb.startswith(STRUCT):
                continue
            if a.endswith("  ") or a.endswith("\\"):
                continue
            out.append((i, a, b))
        return out

    def sample(pop_glob, n):
        files = sorted(TRAIN.glob(pop_glob))
        pool = []
        for f in files:
            for (ln, a, b) in breaks(f):
                pool.append((f, ln))
        return random.sample(pool, min(n, len(pool)))

    picks = sample("udon@cc389f9/**/*.md", 30) + sample("vivarium-archive/**/*.md", 18)
    verdicts = []
    for r in (HERE / "AUDIT-2026-07-22.tsv").read_text().rstrip().split("\n")[1:]:
        parts = r.split("\t")
        verdicts.append(parts[1])
    assert len(verdicts) == len(picks), (len(verdicts), len(picks))

    rows = []
    for (f, ln), v in zip(picks, verdicts):
        if v == "sampler-miss":
            continue
        lines = f.read_text(errors="replace").split("\n")
        fs = file_stats(lines)
        if not fs:
            continue
        # locate the break inside a paragraph block
        for blk in blocks_of(lines):
            for k in range(len(blk) - 1):
                if blk[k] == ln:
                    rows.append((break_features(lines, blk, k, fs), v))
    return rows


def main():
    model = json.loads((HERE / "model.json").read_text())
    keep = model["features"]

    data = [r.split("\t") for r in open(HERE / "DATASET.tsv").read().rstrip().split("\n")[1:]]
    hdr = open(HERE / "DATASET.tsv").readline().rstrip().split("\t")
    fn = hdr[4:]
    idx = [fn.index(k) for k in keep]
    tr = [r for r in data if r[0] != "fable-gold"]
    X = np.array([[float(x) for x in r[4:]] for r in tr])[:, idx]
    y = np.array([1 if r[3] == "phb" else 0 for r in tr])
    files = np.array([r[1] for r in tr])

    rf = RandomForestClassifier(n_estimators=150, max_depth=14, class_weight="balanced",
                                n_jobs=-1, random_state=20260722)
    oof = cross_val_predict(rf, X, y, cv=GroupKFold(5), groups=files,
                            method="predict_proba", n_jobs=-1)[:, 1]
    iso = IsotonicRegression(out_of_bounds="clip").fit(oof, y)

    # export the isotonic map as breakpoints (piecewise-linear, Rust-trivial)
    bx = iso.X_thresholds_.tolist()
    by = iso.y_thresholds_.tolist()
    rf.fit(X, y)

    # organic validation
    rows = audit_sample_breaks()
    Xo = np.array([[f[i] for i in idx] for f, _ in rows])
    yo = np.array([1 if v == "phb" else 0 for _, v in rows])
    po = iso.predict(rf.predict_proba(Xo)[:, 1])

    print(f"organic hand-audited in-scope breaks recovered: {len(rows)} "
          f"({int(yo.sum())} phb / {int((1-yo).sum())} wrap)")
    print("\ncalibrated-probability bands on ORGANIC audited breaks:")
    bands = [(0.0, 0.15), (0.15, 0.5), (0.5, 0.85), (0.85, 1.01)]
    for lo, hi in bands:
        m = (po >= lo) & (po < hi)
        n = int(m.sum())
        k = int(yo[m].sum())
        w = wilson(k, n)
        print(f"  [{lo:.2f},{hi:.2f}): n={n:3d}  phb={k}  "
              f"organic P(phb)={k/max(n,1):.2f}  Wilson95=({w[0]:.2f},{w[1]:.2f})")

    # calibrated training-CV band purity for comparison
    oof_c = iso.predict(oof)
    print("\nsame bands on calibrated training OOF (synthetic-heavy):")
    for lo, hi in bands:
        m = (oof_c >= lo) & (oof_c < hi)
        n = int(m.sum())
        k = int(y[m].sum())
        print(f"  [{lo:.2f},{hi:.2f}): n={n:5d}  P(phb)={k/max(n,1):.3f}")

    model["calibration"] = {
        "method": "isotonic on grouped OOF probabilities",
        "breakpoints_x": [round(v, 5) for v in bx],
        "breakpoints_y": [round(v, 5) for v in by],
        "organic_validation": {
            "n": len(rows), "phb": int(yo.sum()),
            "note": "bands validated on AUDIT-2026-07-22 hand labels; small n — Wilson intervals in calibrate.py output",
        },
    }
    (HERE / "model.json").write_text(json.dumps(model, separators=(",", ":")))
    print(f"\nmodel.json updated with isotonic map ({len(bx)} breakpoints)")


if __name__ == "__main__":
    main()
