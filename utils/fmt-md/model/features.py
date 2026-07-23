#!/usr/bin/env python3
"""Build the per-break feature/label dataset and train the first RF.

First-measurement implementation of research/break-features.md: features are
text-computed with a blank-line block approximation (the durable extractor
will be comrak-parse-based in Rust; this pass exists to measure separability
and prune the feature set before committing that port — approximations are
documented inline and none touch the label pipeline, which is exact).

Label sources (per-break):
  synth-*/            wrap + phb, EXACT by construction (.labels.tsv sidecars)
  udon@cc389f9        phb: context-filtered genuine markers (marker-stripped)
  claude-chat/        phb: paragraph-internal breaks, pure by provenance
Gold eval (never trained on): generated-fable-phb label-runs (12 sites) —
the deployment-target class: intent, no marker.

Outputs: DATASET.tsv (one row per break), RF metrics on grouped CV +
leave-one-corpus-out + OOB, feature importances, gold-set recall.
"""

import math
import re
import statistics
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
TRAIN = HERE / "training"

STRUCT = ("#", "|", ">", "```", "---", "===")
LIST_RE = re.compile(r"^(\s*)([-*+]|\d+[.)])\s")
LABEL_RE = re.compile(r"^\s*\*\*[A-Z][^*\n]{0,50}?:?\*\*")
ABBREV = ("cf.", "e.g.", "i.e.", "pp.", "vs.", "etc.", "al.", "no.", "fig.")
FUNC_WORDS = set("the a an of to in on for and or but with as at by from is are was were be been "
                 "that which who whom this these those it its".split())


def is_structural(line):
    s = line.strip()
    if not s:
        return True
    if s.startswith(STRUCT) or LIST_RE.match(line):
        return True
    return False


def sent_end(s):
    s = s.rstrip()
    if not s or s[-1] not in ".?!:;":
        return False
    last_word = s.split()[-1].lower() if s.split() else ""
    return not any(last_word.endswith(a) for a in ABBREV)


def char_class(c):
    if c.isalpha():
        return 1 if c.islower() else 2
    if c.isdigit():
        return 3
    return {".": 4, "?": 4, "!": 4, ",": 5, ";": 6, ":": 7, "—": 8, "-": 8,
            ")": 9, "]": 9, '"': 10, "'": 10, "*": 11, "`": 12, "(": 13, "[": 13, "$": 14}.get(c, 0)


def blocks_of(lines):
    """Blank-line block segmentation, fence-aware. Yields (start, end) of
    prose blocks (all lines non-structural). Approximation of the comrak
    paragraph parse — lists/quotes are excluded rather than container-parsed,
    which under-covers container paragraphs; acceptable for measurement."""
    in_fence = False
    cur = []
    start = 0
    if lines and lines[0].strip() == "---":          # YAML frontmatter: foreign region
        for j in range(1, min(len(lines), 60)):
            if lines[j].strip() == "---":
                start = j + 1
                break
    for i, l in enumerate(lines):
        if i < start:
            continue
        s = l.strip()
        if s.startswith("```"):
            in_fence = not in_fence
            if cur:
                yield cur
                cur = []
            continue
        if in_fence or not s:
            if cur:
                yield cur
                cur = []
            continue
        if is_structural(l):
            if cur:
                yield cur
                cur = []
            continue
        cur.append(i)
    if cur:
        yield cur


def file_stats(lines):
    widths = [len(l.rstrip()) for l in lines if l.strip() and not is_structural(l)]
    if len(widths) < 3:
        return None
    ws = sorted(widths)
    n = len(ws)
    return {
        "d_p50": ws[n // 2],
        "d_p90": ws[int(n * 0.9)],
        "d_var": statistics.pvariance(widths) if n > 1 else 0,
        "d_frac6090": sum(1 for w in widths if 60 <= w <= 90) / n,
        "d_nlines": len(lines),
    }


def end2_class(s):
    """Multi-char closer classes: naked-label enders rank before generic."""
    s = s.rstrip()
    for pat, code in ((":**", 1), ("**:", 2), (".**", 3), ("**", 4)):
        if s.endswith(pat):
            return code
    if not s:
        return 0
    return {":": 5, ";": 6, ",": 7, ".": 8, "?": 8, "!": 8, ")": 9, "]": 9,
            '"': 10, "—": 11, "-": 11, "`": 12, "$": 13}.get(s[-1], 14 if s[-1].isalnum() else 0)


def start2_class(s):
    """Multi-char opener classes incl. sentence-case vs caps vs bare-cap."""
    s = s.lstrip()
    if s.startswith("**"):
        return 1
    if not s:
        return 0
    c0 = s[0]
    c1 = s[1] if len(s) > 1 else ""
    if c0.isupper():
        if c1.islower():
            return 2      # sentence-case word
        if c1.isupper():
            return 3      # ALL-CAPS word
        return 4          # bare capital (variable-ish)
    if c0.islower():
        return 5
    if c0.isdigit():
        return 6
    return {"`": 7, "*": 8, "[": 9, "(": 10, '"': 11, "$": 12, "_": 13}.get(c0, 14)


def style_features(a, b):
    """Styling self-containment (Joseph, 2026-07-22): a line that opens AND
    closes its own inline styling is likelier an independent unit; a style
    span crossing the break is mid-construct. No length constants."""
    def marks(s):
        return s.count("**"), s.count("`"), s.count("$")
    ab, at, ad = marks(a)
    bb, bt, bd = marks(b)
    a_balanced = int(ab % 2 == 0 and at % 2 == 0 and ad % 2 == 0)
    b_balanced = int(bb % 2 == 0 and bt % 2 == 0 and bd % 2 == 0)
    cross = int(ab % 2 == 1 or at % 2 == 1 or ad % 2 == 1)
    a_s = a.strip()
    b_s = b.strip()
    a_self = int(a_s.startswith(("**", "*", "`", "_")) and ab >= 2 and a_balanced)
    b_self = int(b_s.startswith(("**", "*", "`", "_")) and bb >= 2 and b_balanced)
    # bold coverage of A: fraction of chars inside ** spans
    cov = 0.0
    if ab >= 2 and a_s:
        inside = False
        n_in = 0
        i = 0
        chars = a_s
        while i < len(chars):
            if chars.startswith("**", i):
                inside = not inside
                i += 2
                continue
            if inside:
                n_in += 1
            i += 1
        cov = n_in / max(1, len(a_s))
    return [a_balanced, b_balanced, cross, a_self, b_self, cov]


FEATURE_NAMES = [
    "a_width", "b_first_w", "rejoin_w", "a_last_cls", "a_func_end",
    "a_sent_end", "a_clause_end", "paren_depth", "b_first_cls",
    "b_label", "b_lower_func", "cap_nonterm", "lower_term", "b_indent",
    "a_minus_prev", "a_end2", "b_start2",
    "a_marks_bal", "b_marks_bal", "span_crosses", "a_self_styled",
    "b_self_styled", "a_bold_cov",
    "blk_n", "blk_edge_var", "blk_frac_sent",
    "blk_all_bound", "blk_label_frac", "blk_int_labels", "blk_range",
    "d_p50", "d_p90", "d_var", "d_frac6090", "d_nlines",
]


def break_features(lines, blk, k, fstats):
    """Features for the break after lines[blk[k]] (marker-stripped)."""
    a = lines[blk[k]].rstrip()          # marker-blind: rstrip removes "  "
    b = lines[blk[k + 1]]
    bs = b.strip()
    a_words = a.split()
    b_first = bs.split()[0] if bs.split() else ""
    # paren depth from block start to end of a
    depth = 0
    for i in blk[: k + 1]:
        for c in lines[i]:
            if c == "(":
                depth += 1
            elif c == ")":
                depth = max(0, depth - 1)
    blk_lines = [lines[i].rstrip() for i in blk]
    widths = [len(x) for x in blk_lines]
    edge = widths[:-1] if len(widths) > 1 else widths
    sent_ends = [sent_end(x) for x in blk_lines]
    labels_n = sum(1 for x in blk_lines if LABEL_RE.match(x))
    prev_w = len(lines[blk[k - 1]].rstrip()) if k > 0 else len(a)
    f = [
        len(a),
        len(b_first),
        len(a) + 1 + len(b_first),
        char_class(a[-1]) if a else 0,
        int(bool(a_words) and a_words[-1].lower().strip('*_`"\'') in FUNC_WORDS),
        int(sent_end(a) and depth == 0),
        int(bool(a) and a[-1] in ".?!,;:" and depth == 0),
        depth,
        char_class(bs[0]) if bs else 0,
        int(bool(LABEL_RE.match(b))),
        int(b_first.lower() in FUNC_WORDS and b_first[:1].islower()),
        int(bool(bs) and bs[0].isupper() and not sent_end(a)),
        int(bool(bs) and bs[0].islower() and a.endswith(".")),
        len(b) - len(b.lstrip()),
        len(a) - prev_w,
        end2_class(a),
        start2_class(bs),
        *style_features(a, bs),
        len(blk),
        statistics.pvariance(edge) if len(edge) > 1 else 0.0,
        sum(sent_ends) / len(blk_lines),
        int(all(sent_ends)),
        labels_n / len(blk_lines),
        labels_n,
        max(widths) - min(widths),
        fstats["d_p50"], fstats["d_p90"], fstats["d_var"],
        fstats["d_frac6090"], fstats["d_nlines"],
    ]
    return f


def emit_rows():
    rows = []  # (corpus, file, line_of_A, label, features)

    # --- exact synth labels ---
    for lab_file in TRAIN.rglob("*.labels.tsv"):
        md = lab_file.with_suffix("").with_suffix(".md")
        if not md.exists():
            md = Path(str(lab_file)[: -len(".labels.tsv")] + ".md")
            if not md.exists():
                continue
        lines = md.read_text(errors="replace").split("\n")
        fstats = file_stats(lines)
        if not fstats:
            continue
        want = {}
        for r in lab_file.read_text().split("\n")[1:]:
            if "\t" in r:
                ln, cls = r.split("\t")
                want[int(ln) - 1] = cls  # 0-based index of line A
        corpus = md.relative_to(TRAIN).parts[0]
        for blk in blocks_of(lines):
            for k in range(len(blk) - 1):
                if blk[k] in want:
                    rows.append((corpus, str(md), blk[k],
                                 want[blk[k]], break_features(lines, blk, k, fstats)))

    # --- genuine markers (phb), context-filtered, marker-stripped ---
    for p in (TRAIN / "udon@cc389f9").rglob("*.md"):
        sp = str(p)
        if "copies/" in sp or "session-vault" in sp:
            continue
        lines = p.read_text(errors="replace").split("\n")
        fstats = file_stats(lines)
        if not fstats:
            continue
        for blk in blocks_of(lines):
            for k in range(len(blk) - 1):
                raw = lines[blk[k]]
                if raw.endswith("  ") and raw.strip():
                    rows.append(("udon-markers", sp, blk[k], "phb",
                                 break_features(lines, blk, k, fstats)))

    # --- chat paragraph-internal (phb by provenance) ---
    for p in (TRAIN / "claude-chat").rglob("*.md"):
        lines = p.read_text(errors="replace").split("\n")
        fstats = file_stats(lines)
        if not fstats:
            continue
        for blk in blocks_of(lines):
            for k in range(len(blk) - 1):
                rows.append(("chat", str(p), blk[k], "phb",
                             break_features(lines, blk, k, fstats)))
    return rows


def gold_rows():
    """Deployment-target eval: fable bare-newline label-run breaks."""
    rows = []
    for p in (TRAIN / "generated-fable-phb").rglob("*.md"):
        lines = p.read_text(errors="replace").split("\n")
        fstats = file_stats(lines)
        if not fstats:
            continue
        for blk in blocks_of(lines):
            for k in range(len(blk) - 1):
                if LABEL_RE.match(lines[blk[k]]) and LABEL_RE.match(lines[blk[k + 1]]):
                    rows.append(("fable-gold", str(p), blk[k], "phb",
                                 break_features(lines, blk, k, fstats)))
    return rows


def main():
    rows = emit_rows()
    gold = gold_rows()
    with open(HERE / "DATASET.tsv", "w") as f:
        f.write("corpus\tfile\tlineA\tlabel\t" + "\t".join(FEATURE_NAMES) + "\n")
        for c, fp, ln, lab, feats in rows + gold:
            f.write(f"{c}\t{fp}\t{ln}\t{lab}\t" + "\t".join(f"{x:.4g}" for x in feats) + "\n")

    from collections import Counter
    print("rows:", Counter((c, l) for c, _, _, l, _ in rows))
    print("gold:", len(gold))

    import numpy as np
    from sklearn.ensemble import RandomForestClassifier
    from sklearn.model_selection import GroupKFold, cross_val_predict
    from sklearn.metrics import roc_auc_score, precision_recall_fscore_support

    X = np.array([f for *_, f in rows])
    y = np.array([1 if l == "phb" else 0 for _, _, _, l, _ in rows])
    files = np.array([fp for _, fp, *_ in rows])
    corpora = np.array([c for c, *_ in rows])

    rf = RandomForestClassifier(n_estimators=400, class_weight="balanced",
                                oob_score=True, n_jobs=-1, random_state=20260722)

    # grouped CV by file
    proba = cross_val_predict(rf, X, y, cv=GroupKFold(n_splits=5),
                              groups=files, method="predict_proba", n_jobs=-1)[:, 1]
    auc = roc_auc_score(y, proba)
    pred = (proba >= 0.5).astype(int)
    pr, rc, f1, _ = precision_recall_fscore_support(y, pred, average="binary", zero_division=0)
    print(f"\nGrouped-by-file 5-fold CV:  AUC={auc:.4f}  P={pr:.3f} R={rc:.3f} F1={f1:.3f}")

    # leave-one-corpus-out (the batch-effect honesty check)
    print("\nLeave-one-corpus-out AUC:")
    for c in sorted(set(corpora)):
        tr, te = corpora != c, corpora == c
        if len(set(y[te])) < 2:
            print(f"  {c:22} (single-class holdout, AUC n/a; n={te.sum()})")
            continue
        rf.fit(X[tr], y[tr])
        print(f"  {c:22} AUC={roc_auc_score(y[te], rf.predict_proba(X[te])[:,1]):.4f}  n={te.sum()}")

    # full fit: OOB, importances, gold recall
    rf.fit(X, y)
    print(f"\nOOB score (row-level, optimistic): {rf.oob_score_:.4f}")
    imp = sorted(zip(FEATURE_NAMES, rf.feature_importances_), key=lambda t: -t[1])
    print("\nTop features:")
    for n, v in imp[:12]:
        print(f"  {n:16} {v:.3f}")
    if gold:
        Xg = np.array([f for *_, f in gold])
        pg = rf.predict_proba(Xg)[:, 1]
        print(f"\nGold (fable bare-newline phbs, never trained on): "
              f"{(pg >= 0.5).sum()}/{len(pg)} recalled as phb; "
              f"probs: {' '.join(f'{p:.2f}' for p in sorted(pg))}")


if __name__ == "__main__":
    main()
