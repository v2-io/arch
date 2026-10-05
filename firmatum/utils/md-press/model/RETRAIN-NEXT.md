# Retraining the break classifier — where to pick it up

*2026-10-05, written at the hand-off for whoever resumes, Joseph included. Joseph that day: "We probably also have enough data to shore up the random forest a bit and get better at the joins." What follows is what I know, what I measured, and what I'd do, marked as each. Nothing here has been started; the committed `model.json` is still the 2026-07-22 forest.*

## What the classifier gets wrong today, and why

**The probabilities sit on plateaus because the calibration is a step function.** `calibrate.py` fits isotonic regression, and the table exported into `model.json` has 21 breakpoints, and above 0.1 its output settles on five plateau values (0.400, 0.474, 0.714, 0.904, 1.000), joined by short linear ramps:

| raw forest score | calibrated p | what md-press does |
|---|---|---|
| 0.133–0.245 | 0.082 | join, confident |
| 0.247–0.320 | 0.333–0.400 | join, unsure |
| 0.333–0.427 | 0.474 | join, unsure |
| 0.433–0.567 | **0.714** | **keep and mark, unsure** |
| 0.573–0.760 | **0.904** | **keep and mark, confident** |
| 0.766– | 1.000 | keep and mark, confident |

Three consequences, each observed:

- **Where the forest is genuinely undecided (raw ≈ 0.5), md-press keeps the break and writes a mark,** which changes rendering. Over asf HEAD (`--no-math`, 2026-10-05), 818 of the 1,744 uncertain breaks sat at 0.70–0.74 and 872 at 0.40–0.49; only 54 fell anywhere else.
- **"Confident" keeps start at a raw score of 0.573.** Three obviously wrapped short lines (`A wrapped / paragraph of / prose here.`) are kept and marked at p 0.904, silently, before any review. Real files rarely wrap that short, but it's a rendering change made with confidence the forest didn't have.
- **The same construct lands on different sides of a step.** `N. **Author (Year).** *Title* ⏎ *Significance:* …` is kept at 0.71 in two files of asf's `ref/prior-art-analysis/`, joined at 0.47 in another, and joined at 0.33 in a fourth. The file-level features differ between them (see the leak below), so near a step the same shape flips.

In an eyeballed sample of 25 from each plateau (my own reading, not labels), about half of the 0.71 keeps were sentence-per-line paragraphs, which the 2026-07-22 policy says should join. The 0.47 joins included real meant breaks: reference-then-significance pairs, `(iii) … ⏎ (iv) …` enumerations, and a bold heading-line followed by an italic note.

**The training data has the problems the Opus audit named, all still open** (`AUDIT-pipeline-opus.md`; STATUS lists them as fix-list items 2–5):

1. Every negative (wrap) is synthetic, injected by one generator at columns 68–85, so the learned "wrap" geometry is that generator's band. Organic wraps are ragged and sit at other columns.
2. File-level features alone reach grouped-CV AUC 0.893, because the negatives live only in synthetic files and most positives come from chat/udon files: the file aggregates encode which *population* a break came from. `d_frac6090` is in the shipped model.
3. `blk_n` alone gets AUC 0.84, a construction artifact: injected wraps make 8-line blocks, organic wraps 3–5.
4. Paragraph-final intended breaks are dropped in label joining (~18% of synthetic phb labels).
5. The gold set (12 `**Label:**→**Label:**` breaks) only certifies the label-stack detector, not the hard class: prose-internal intended breaks with no label.

The audit's "highest-value next measurement" was never run: the false-positive rate on organic wraps, scored over the already hand-audited organic breaks (`AUDIT-2026-07-22.tsv`: udon@cc389f9, vivarium-archive).

## Data that exists now and didn't in July

- **asf's wrap debt is a large organic-wrap population.** On 2026-10-05, 939 of asf's 2,788 tracked `.md` files would change. Their joinable breaks are agent-written, real-column wraps, mixed with the meant breaks the plateaus show. They're unlabeled but abundant, and they are exactly the deployment distribution.
- **The usage log** (`~/.local/share/md-press/usage.jsonl`, from 2026-10-05). Each record holds the unsure decisions with both context lines, the classifier's 14 feature values, the default, the outcome, and whether it was flipped. `toggles_given: true` marks a reviewed run. On those runs, a flip is a strong label; an unsure decision left at its default is a weak one, since the reviewer may not have looked. The labels come from whoever reviews: mostly agents, sometimes Joseph. Keep that provenance and weigh it. Confident joins are logged only as counts, so the log can't tell you how often confident decisions are wrong; an eval set has to.
- **Public hard-wrapped markdown** (`research/markdown-corpora-2026-10-03.md`): about 178 of the Rust RFCs' 649 files wrap at 72–80 columns (Apache-2.0). It's human-written, so a different distribution from agent wraps, but it gives organic negatives that can be committed.
- **The generation harnesses in `README.md`** still work for positives: chat-block intent breaks, and Sonnet/Opus file-writing for instinct wraps and label stacks.

## What I'd do, in order (my lean, not a plan of record)

1. **Measure before changing anything.** Build an organic eval set that is never trained on:
   - a stratified sample from asf's two plateaus, the reference-shape cases, the enumeration cases, and short-line wraps;
   - labeled blind by two independent agents (the spike's labeling design, with each labeler given its own scratch directory), with disagreements adjudicated or kept as "ambiguous".
   
   Then run the audit's organic false-positive measurement on it and on `AUDIT-2026-07-22.tsv`. That gives today's honest per-band precision, which nothing currently reports.
2. **Remove the population leak.** Drop or rebuild the `d_*` file features and the `blk_n` magnitude, so the forest can't learn which corpus a break came from. Add organic negatives: measured-wrapped asf files and the RFCs. Recover paragraph-final labels.
3. **Recalibrate on organic held-out data, and smoothly.** Isotonic on synthetic out-of-fold scores is what produced the five steps. Platt scaling, or isotonic with a minimum bin size fit on the organic set, would give graded probabilities. Report the curve with Wilson intervals, as `calibrate.py` already does for its 48-break check.
4. **Set the bands from measured organic precision, including the default for the undecided middle.** The two errors aren't symmetric. A wrong keep adds a visible line break. A wrong join changes nothing rendered: a bare newline already renders as a space. It only drops a source-level hint of intent, and the report lists it so it can be flipped. That asymmetry argues for joining where the forest is undecided, but it should be decided by the step-1 numbers, not by argument. It's Joseph's call, since it revisits the ratified 0.5 band.
5. **Acceptance, before shipping a new `model.json`:**
   - On asf the uncertain decisions spread out instead of sitting on two values, and the reference shape gets one answer within a directory.
   - Gold-1 doesn't regress. STATUS records 11/12 at the 0.5 band, the twelfth adjudicated genuinely ambiguous at 0.497; the audit's uncalibrated run got 12/12.
   - udon's reproducibility test still converges (STATUS: *Reproducibility acceptance test*).
   - `cargo test` passes, and the asf report-then-fix-then-report cycle stays idempotent with 0 gate refusals.

The Rust side needs no structural change: features are computed in `classify.rs` in parity with `features.py`, so a feature change must land in both, with parity re-verified. I haven't checked whether a parity test exists; if not, write one first. A new `model.json` changes the build identity, which already invalidates old codes.

## Open questions for Joseph

- The undecided-middle default (step 4).
- Whether public corpora (RFCs, Stack Exchange) may enter training, or only evaluation.
- Whether agent-reviewed flips from the usage log count as training labels, or only as an eval source.
