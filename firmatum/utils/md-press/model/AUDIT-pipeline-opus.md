# Pipeline audit — wrap-vs-phb classifier (Opus, 2026-07-22)

*Cold skeptical read of `model/features.py` + `model/extract` + the emitted
`DATASET.tsv`, against the spec in `research/break-features.md` and the
provenance in `model/README.md`. Every number below was reproduced or
computed by execution over the on-disk corpus; probe script and commands are
listed at the end. Unprimed — I did not see Joseph's suspicion list.*

## Bottom line

The reported numbers are **real and faithfully reported** — I reproduced
AUC 0.9975 / P 0.996 / R 0.801, LOCO 0.995 / 0.905 / 0.997, and 12/12 gold
exactly. There is **no label-contamination leak** in the classic sense: the
marker-stripping guard is genuinely implemented, and within a synth file both
classes share identical file-level features so the file fingerprint cannot
separate them there.

But the good numbers do **not** certify deployment performance, and the two
headline claims most likely to be over-read are the two the evaluation is
structurally unable to test:

1. **`precision 0.996` is precision against *synthetic* wraps.** 100% of the
   30,623 negatives are machine-injected by one generator at one column band.
   The organic-wrap false-positive rate — the number that decides whether the
   deployed linter flattens real text — is measured by **nothing in this run.**
2. **`12/12 gold` certifies the *easy* genre only.** The gold set is, by
   construction, twelve `**Label:**→**Label:**` stack breaks. It demonstrates
   the label-template detector fires; it does not touch the prose-internal /
   mid-sentence intended-break class the README itself (V-12) calls
   per-line-indistinguishable.

None of this is dishonesty in the pipeline — the code and README are candid
about the synthetic negatives and the PU framing. The gap is between what the
metrics *measure* and what a reader will *take them to mean*. Details and the
one measurement that would close the gap follow.

---

## Confirmed real / good

- **Numbers reproduce bit-for-bit.** `python3 features.py` → AUC 0.9975,
  P 0.996, R 0.801; LOCO synth-asf 0.9954 / synth-chat 0.9048 / synth-tst
  0.9969; gold 12/12. No misreporting.
- **Marker-stripping guard is real.** `break_features` computes A via
  `.rstrip()` (removes the `"  "` hard-break marker) before any feature; the
  udon path uses the trailing-space marker only to *select* the row, never as
  a feature. Guard §1 is honored in code. I looked for a feature that could
  see the marker and did not find one.
- **Within-synth-file discrimination is genuine.** Because the synth
  construction puts wrap *and* phb labels in the same file with the same
  file-level `d_*` values, grouped-by-file CV genuinely forces the model to
  use break-local features to separate the two classes inside synth-tst
  (110 phb / 6968 wrap). That part of the AUC is earned.
- **Class geometry is not a single-feature giveaway.** `a_width` alone gets
  only 0.62 AUC, `rejoin_w` 0.59. The classes are genuinely multi-feature —
  the forest is doing real joint work, not reading one leaked column.

## Confirmed limitations (the load-bearing findings)

### 1. The entire negative class is synthetic, at one injected column band — and that band *is* what the model learned as "wrap"

`cmd_synth` wraps every long prose line at `col = 68 + randrange(18)`
(68–85), backing up to the nearest space. The learned wrap geometry matches
the injector exactly:

```
a_width  wrap  p5/25/50/75/95 = 61 / 67 / 72 / 76 / 81
a_width  phb   p5/25/50/75/95 = 20 / 35 / 62 / 80 / 104
```

The wrap class is a tight 61–81 band — the injector's parameter, not a law of
organic wrapping. Real LLM wraps in this ecosystem land at ~58, ~72–85, ~75–78
(README's own measurements) and are *ragged*, not uniform. `blk_edge_var`
(right-edge tightness, a top-8 feature) is unrealistically clean on synth
because one algorithm wrapped every line; organic wraps have more variance.
So the features that separate best on synth are precisely the ones most likely
to regress on organic wraps at other columns. **AUC 0.9975 answers "synth-wrap
vs phb," which is easier and different from the deployment question,
"organic-wrap vs phb."**

### 2. Leave-one-corpus-out cannot fire on the risk it was built for

Spec guard §5 prescribes LOCO to catch dataset-identity learning. As
implemented, the three scored folds hold out `synth-asf`, `synth-chat`,
`synth-tst` — all three produced by the *same injector*. The other two
corpora (chat, udon-markers) are single-class phb and are skipped
(`AUC n/a`). There is **no organic-wrap corpus in the data to hold out**, so
LOCO measures topic transfer within synthetic negatives. The honesty check
looks satisfied but is structurally blind to the synth→organic gap — the one
batch effect that actually threatens deployment.

### 3. Corpus-identity leak via file-level features — quantified

File-level `d_*` features **alone** (p50, p90, var, frac6090, nlines), with
zero break-local information, reach **grouped-CV AUC 0.893**. Because the
negative class lives only in synth files while 89% of positives
(258/291) come from chat / udon / synth-tst files, the file aggregates encode
which *population* a break came from — `d_nlines` phb-mean 399 vs wrap-mean
217, `d_frac6090` ranks #6 in the full model. Grouping by file does not defend
against this (it isolates files, not populations). This is guard §5's D7/D10
worry realized in the D-features that survived into the model.

### 4. Block-size separation is construction-inflated

`blk_n` alone gets **0.84 AUC** (wrap blocks mean 8.1 lines, phb 3.8). The
injector explodes one clean long line into an 8+ line wrapped block, while
phbs sit in naturally short blocks (141 of 291 are ≤2-line blocks — pure
key/value pairs). The signal is real but its *magnitude* is a synthetic
artifact: an organic wrap of a normal paragraph makes a 3–5 line block, not 8.

### 5. The gold set certifies a narrower claim than "deployment defects"

`gold_rows()` emits a break only when **both** `lines[blk[k]]` and
`lines[blk[k+1]]` match `LABEL_RE`. The gold set is therefore twelve
`**Label:**→**Label:**` breaks (`blk_label_frac == 1.0` for all 12). And
68% of *training* positives (197/291) are label-template-associated. So the
top features (`b_first_cls`, `blk_label_frac`, `blk_int_labels`, `b_label`)
and the gold recall are all the same thing: the `**Label:**`-stack detector —
which the pre-existing C8 interior-label heuristic already did (README:
"≥2 fired on all five real cases"). Two consequences:

- The **hard class is absent from the eval**: a prose-internal, mid-sentence
  intended break with no label and no marker (the V-12 case the README flags
  as per-line-indistinguishable) appears nowhere in gold and thinly in
  training (~94 non-label positives, many of them short chat/udon breaks).
- Even on the easy genre, calibration is modest: the lowest gold prob is
  **0.52** — one specimen barely clears the 0.5 threshold.

### 6. Recall 0.801 is the honest soft spot

Precision-against-synthetic is the strong-looking number and the less
meaningful one. **Recall is the deployment-critical metric** — a missed phb is
an intended break the linter flattens, which is the actual defect — and at
0.5 threshold the model misses 20% of true phbs *even in the easy synthetic
setting*. That is the number I would trust least to hold up on organic text.

### 7. Minor: silent label-join loss

The synth sidecars carry 164 phb + 33,142 wrap labels; `DATASET.tsv` contains
135 synth phb + 30,623 wrap. ~18% of synth **phb** labels and ~7.6% of wrap
labels are dropped because `blocks_of()` (blank-line approximation) doesn't
place the labeled line as a non-terminal break in a prose block —
paragraph-*final* intended breaks are systematically lost. Small, and the
coming comrak port should reconcile it, but it biases the phb class toward
paragraph-interior breaks and is worth a reconciliation check at port time.

## Can't tell without X

**The deployment false-positive rate on organic wraps — the single number
that matters and that nothing here measures.** It needs organic wraps in the
eval. They exist: `AUDIT-2026-07-22.tsv` hand-classifies 48 organic breaks
(vivarium 18/18 wrap, udon ~16/17 wrap), and the underlying corpora
(`udon@cc389f9`, `vivarium-archive`) are on disk — but the pipeline reads udon
*only* for phb markers and never scores its ordinary paragraph breaks as
negatives. **Highest-value next measurement:** score the trained model over
paragraph-internal breaks in the hand-audited organic populations and report
the phb false-positive rate. That directly measures deployment precision and
would either validate the geometry-is-real hypothesis or expose the synth
gap — one run, read-only, against data already labeled.

Second: seed the gold set (and, ideally, training positives) with the
non-label prose-internal / mid-sentence class, so "gold recall" tests the hard
target rather than the template detector.

---

## How to reproduce

- Headline numbers: `python3 model/features.py` (regenerates `DATASET.tsv`,
  read-only over `training/`).
- Probes (single-feature AUCs, `a_width` bands, `d_*`-only and geometry-only
  grouped-CV AUCs, importances, phb composition): I ran a standalone script
  over the emitted `DATASET.tsv` — reproduced here in prose so it can be
  regenerated: single-feature `roc_auc_score(y, feature)`; `d_*`-only and
  `[a_width,rejoin_w]`-only models via `GroupKFold(5)` grouped by file;
  phb-composition counts by `b_label` / `blk_label_frac` / `a_sent_end` /
  `blk_n`. Key results inline above.
