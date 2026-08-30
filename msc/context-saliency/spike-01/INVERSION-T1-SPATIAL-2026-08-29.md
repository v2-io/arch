# T1-spatial inversion — constructed anti-correlation, not a silent null

*Status: characterization of a gate result, not a replacement of it. Empirical numbers remain those in `GATE-VERDICT-2026-08-29.md` (ρ = −0.242 raw, −0.301 detrended; bottom-quintile flip-rate 1.000). This note is what that no-go **means**, and why it must not be filed as “prefill hidden-diff carries nothing.”*

## What the gate actually showed

On Plan-NIAH **as currently generated**, T1-spatial (`g_prefill`, ingestion-time hidden-diff rolled to segments) **does not rank causal spans**. Spearman against T3 occlusion-flip is negative; the *lowest* spatial quintile is the one that flips when occluded. That stands. It is a no-go for using this signal as a causality ranker **on this task family**. Span-ranking work proceeds on T3. That sentence does not get softened.

What does *not* follow — and must not be written as if it did — is “the signal is noise / dead / unused.” The failure is **signed and stable**: wrong direction, stronger after a linear position detrend, bottom quintile at 1.000. Noise does not pick a side and then dig in.

## The construction that produces the sign

Plan-NIAH’s 2×2 was built so that **causality lives in the terminal CODE line** and **interiors are screened**. That was not an aesthetic choice. v1 put the working arithmetic in the interior; occluding the terminal then failed to break the task; “terminal is the sole carrier” was false until we moved every recompute-sufficient token onto `CODE-k: NNN` (FINDINGS method lesson 3). The construction that makes T3 labels decidable is:

| role | informational job | **surface**, as generated |
|---|---|---|
| terminal `CODE-k: NNN` | sole causal carrier of the code (by construction) | terse, formulaic, repeated tag+integer |
| interior body | screened narrative; T3 should be null | colorful, varied, unique sentences from a distractor pool |
| count line | screened except in delayed_reuse | mid-form (“You count exactly N furns…”) |
| header | structurally load-bearing, informationally thin | short, repeated frame (“Room k is the {color} room.”) |

Prefill hidden-diff is the **ingestion-time sibling of EpiKV’s generated-token change**: `‖h_l(pos) − h_l(pos−1)‖` while *reading*. The quantity it is built to fire on is **representational change along the prompt**, which on text is close to **novelty / surprisal / “this token is not a copy of the last pattern.”** Formulaic `CODE-k: 172` after three similar CODE lines is *low* change. A new tapestry sentence is *high* change.

So on this generator, **the thing T1-spatial is likely to measure (ingestion-novelty) and the thing T3 labels (future behavioral necessity) are anti-correlated by design.** We required that anti-correlation in order to have a clean 2×2. We then asked whether a novelty-shaped instrument would rank the 2×2’s causal cells. An inversion is the result that construction predicts if the instrument is working *as a novelty meter*. It is not the result the calibration-gate *question* asked (“does it rank causality?”), so the gate still fails — but the sign is then evidence about the instrument, not a hole where the instrument should have been.

Two sentences that must stay distinct:

1. **Gate (causal ranking, this task):** T1-spatial does not track T3. Do not use it as a salience-for-retention score on Plan-NIAH-narrative.
2. **Characterization (what the signed failure is):** the inversion is the expected coupling of a novelty-shaped reader-signal to a generator that put causality in the *least novel* lines on purpose.

Filing only (1) buries (2). Filing only (2) as “so the gate doesn’t count” is a soften. Both are present-tense.

## What would distinguish the two readings

Equalize **surface novelty across roles**, leave the causal 2×2 intact:

- **`surface=formulaic`** (now in `tasks.py`): interiors become `AUX-k-i: 0` / `COUNT-k: N` / `ROOM-k: color` — same roles, same line count, same “code only on the CODE line,” but the screened body is no longer the colorful side of a novelty contrast.
- (Not yet built.) Narrative-embedded codes: causality in varied prose, formulaic distractors — the opposite polarity.

Predictions, pre-registered:

- If inversion **tracks** the novelty contrast (ρ rises toward 0 or flips positive under formulaic interiors), T1-spatial is characterized as an ingestion-novelty signal, usable as such, and the original no-go is “wrong target, live instrument.”
- If inversion **survives** equalization, the no-go deepens to anti-causal on matched surfaces — stranger, more publishable, and not explained by our construction.

Until that cell is scored, the inversion is a **structured open** attached to a **closed gate on this generator**, not a retraction.

## Discriminator cell, same evening (formulaic interiors, n small)

`surface=formulaic` in `tasks.py`; 7B traces `out/formulaic_chrono_{variant}_{7,11}.json`; T3 `out/occlusion_formulaic.json`. Same Spearman recipe as the gate (and the same linear detrend).

| cell | inferred rows | paired n | raw ρ | detrended ρ | top20% / bottom20% flip |
|---|---|---|---|---|---|
| narrative v4 **chrono only** (matched placement) | 4 | 20 | −0.244 | −0.347 | 0.250 / 1.000 |
| **formulaic** (seeds 7, 11; seed 11 not base-correct) | 2 | 10 | −0.577 | −0.640 | 0.000 / 1.000 |

The inversion **did not go away** when interiors were equalized to terse `AUX-k-i: 0` / `COUNT-k: N` tags. At this n it deepened. That is **not** yet a proof of anti-causal structure (one base-correct seed, n=10). It **is** enough to say the cheap rescue — “it was only the colorful-vs-CODE novelty contrast” — did not show up on the first cell.

So the construction note above still holds as the *reason the original gate was a stacked test*. The formulaic cell says: even after taking that stacking down a notch, the signed failure is still there. Next discriminator (more seeds, or narrative-embedded codes) is still open. The gate on Plan-NIAH-narrative is still closed.

## Adjacent notes (same mint, not the same claim)

- **T1-temporal 0/6 predicted sign is 6/6 the opposite sign** (near-boundary Δ depressed in every pair). n=6 is one coin-flip from noise; it claims nothing yet. Pre-register for the next mint: “boundary depression replicates.” Strengthen-before-soften on a null: check whether the null is an inverted regularity before writing “does not fire.”
- **Reversed placement is a capability finding**, not only a confound control. Reversed `delayed_reuse` on seeds 7 and 13 emitted codes in *transcript* order, not requested room-number order. Placement variation changed task difficulty, not just position statistics; those cells are not difficulty-matched to chrono. (Walk-task copy now says “room-number order, regardless of transcript order”; that is a generator fix going forward, not a rewrite of the v4 mint.)
