# Debrief for Joseph: the deterministic math converter

**The converter.** On the text md-press sends to llama3.2 today, a deterministic converter does far more of the job, at about the same error rate per edit. The Python reference is `py/umath.py`. A delegated pure-Rust port in `rs/umath/` matches it with 0 differences over 4.6M inputs, and runs the 1.29M estate prose sites in 11 s on one thread.

**Held-out D, 200 whole prose lines** (the unit md-press would feed it). D is the only clean held-out set: the converter was frozen before any D label existed.
- *Converter v4:* 84.5% exactly or equivalently right; 5.3% of the lines it changes are wrong or over-converted.
- *llama3.2, run as md-press runs it* (its own pieces and gates): 37.5% right; 3.8% of the lines it changes are wrong.
- *The trade:* the converter changes 187 lines to llama's 80. That is about 94 more lines right than llama, and about three times as many wrong edits in absolute terms (10 to 3 per 200 lines).
- *How solid the rate is:* the counts are small enough that the per-edit rates aren't really distinguishable. The scoring choices move the converter's figures by a few points (82–86% right, 3.5–7.5% wrong+over; the README brackets this).
- *Muse Glimmer 30B* was only run on set A, which I later used for development: 60.4% right, 3.6% per write.

My first version of this debrief said the converter was more precise per write than llama (3.9% vs 4.9%). That was a cross-set comparison from B. The independent verifier ran llama on the held-out D lines and the rates came out level. I reproduced their run.

It needs no model, it's a pure function of the text, and it is idempotent (iterated to a fixed point). KaTeX and MathJax render all 4,820 distinct spans the current version emits over the estate and the gold sets. The verifier also found a bug class my scorer couldn't see: a word hyphen before an operator name became a minus (`$n$-dim` → `$n - \dim$`), at 25 estate sites. It is fixed in v7, and the scorer now catches it. The port's fuzz earlier found an input that crashed it (𝟊) and inputs that made it emit undefined commands (`Ξ*^T` → `\Xi^{\astT}`); v6 fixed those. So `--check` stops depending on ollama.

**Recommendation.** Replace the model on the sites md-press already triggers on. Don't widen the scope yet. On untriggered sites roughly one in five of its changes is wrong (D: 20.0% of 150), and the sites with Unicode script characters (`W₂`, `E³`, `m²`) are the worst of them (31%).

## What decides the residue

I expected the hard boundary to be language understanding. On D's untriggered sites it splits about evenly, between knowledge outside the line and judgment the line itself supports.

**Outside the line (about 16 of 30)** — the labelers read the canon, the paper or the rest of the table:
- **Labels glyph-identical to variables.** asf's canon writes the regime names W₀/W₁ as Unicode prose (W₁: 139 raw, 1 inside math) and keeps `$W_2$` for Wasserstein. causal-language's `H_D` hypothesis IDs appear 215 times in its markdown and LaTeX (H_D3 alone 135) and never inside math.
- **Footnote markers.** `E³` and `X⁸` are grades with footnote markers.

**In the line (about 11 of 30)** — the line carries the evidence:
- **Log fields.** `t=45` beside a timestamp.
- **Units.** A unit after a noun.
- **Mention versus use.** A sentence saying `n_past` lacks LaTeX; a quoted code comment.

The first kind are conventions, so they want declaring, not inferring. I tested inference with house-usage statistics. Mined from asf's format-gated canon (`0[1-4]-*-core/src/`), they recover exactly {W₀, W₁}. Mined over the whole estate, the same statistic also flags `β_0` and `ϵ`, which are lazy unconverted math rather than labels: the estate is itself the population md-press is treating. In projects that never write LaTeX (causal-language), there is no evidence at all. So the labels question is yours: per-project declared lists, canon-mined data, or neither.

The second kind is converter work: rules for log fields and units, or a model prompted for exactly the converter's leftovers (not yet tried).

Three policy calls the labelers split on, which I left conservative:
- bare ASCII variables with no Unicode signal (`Here U is…`; the converter wants evidence first);
- statistics like `N=40`, `p<.05` (converted only when a relation ties a letter to a value);
- ranges inside relations (`ε ≈ 0.1-0.4`, emitted as `0.1\text{–}0.4`).

## What your suggestions bought

- **The "effectively-equivalent" class.** On the first dev run, 47 pieces scored "wrong". Many were spellings any reader would accept: `$\alpha/\beta$` vs `$\alpha$/$\beta$`, `\sqrt h`, `S_{id}` vs `S_{\text{id}}`. The scorer now compares visual atoms (glyph, script position, font), and it separates *degraded* (left partly as written) from *wrong* (prose italicized, structure changed). The class had one real blind spot, which the verifier found: a word hyphen and a minus were the same atom. That is fixed, and every table also carries exact-only counts and stricter readings.
- **The corpus as data.**
  - *Reversal.* 50,348 estate lines already carry `$…$`. Reversing them to Unicode surfaced real gaps.
  - *Agents' own dialect.* Having Sonnet, Opus and Haiku rewrite 600 of them in the dialect they'd naturally use found gaps my own reverse map never produced: `∑_i`, `∇_θ`, `η^(k)*`, `Ṙ_min,i`, `δ_critical,k²`.
  - *Validity.* One of those produced `\eta^{(k)}^\ast`, which doesn't render. Validating every emitted span in KaTeX and MathJax then found 32 invalid spans of several classes (double scripts, `\sqrt` with no argument, a transcript swallowed as a bra-ket), and the converter now carries a structural TeX check and abstains rather than write a span that won't render.
- **Symbols seen in `$…$` as evidence (your `i'` example).** A bare letter, or a primed letter like `x'`, becomes math only on evidence: an operator ties it to math, or the same letter is an operand in this text's existing `$…$` or in spans the converter found. So `i'` converts beside `$i' = i + 1$` and stays prose in `rock 'n' roll`. On the dev sets (A+B, 540 items) it adds 10 correct items and 1 new wrong one; switching it off gives 477 exact-or-equivalent against 487 with it. The same evidence across a whole file is the obvious next step; md-press has the file.
- **Math-free text.** Over 2.47M sites from 13 external doc repos it changes 72 sites. Of the 15 distinct regions, 4 are wrong or doubtful: `τ²-bench`, `c_src`, `n_compactions`, and a sentence mentioning ∞.
  - *Bugs it caught on the way:* currency `$`, inline `<script>` JS, `200k` as a product.
  - *Property check.* A test over 920k real sites (shape, verbatim, idempotence, validity) caught 829 non-idempotent outputs in v3, where a partial first pass gets merged by a second. v4 iterates to a fixed point; real text now shows 0 violations.

## Things that didn't work

- **A random forest for bare letters.** 49k letters labeled by whether the estate's LaTeX authors wrapped them, grouped CV AUC 0.98. It barely transfers to agent-written Unicode. At p≥0.9 it gains 2 items on B and costs 1 on A; at p≥0.7 it gains 6 on B and adds 11 wrong across A+B.
- **The model as fallback.** Using llama or Muse only where the converter leaves a piece alone adds no correct conversions on A or B and adds one or two wrong ones.
- **My own `x_t+1` → `x_{t+1}` heuristic.** It produced `U_{M+U}_o` on the first held-out set, and it's gone. It came from my idea of how agents write, not from data, which is the trap the brief named.

## Incidental findings in md-press

- **Unwrap joined frontmatter** that follows an HTML comment, putting YAML keys inside the preceding key's `#` comment, and it passed the render check. Two labelers found it and I reproduced it. The coordinator has since fixed it in the working tree, with a regression test.
- **The gates let a deleted single `*` through.** The verifier found llama deleting italics markers on D. That is fixed in the working tree too.
- **The splitter cuts expressions.** `split_at_prose_separators` cuts real expressions apart (`κ×A/tempo`, `0.02 ≤ |CUBE| ≤ 0.10`, `1/√α`). For this converter the measured cost is small: on D (scorer v1), whole lines score 86.0% against 84.5% fed piecewise through md-press's own splitters.
- **Wrong conversions pass today's gates.** On B, four of llama's wrong conversions pass all of them, including `operands_survive`: `ρ/R` → `\rho/\rho_R`, `T` → `\mathcal{T}`, a swallowed `?`, and a raw combining hat left in math.
- **The gates refuse mostly good output, but not only good output.** On the converter's 7,826 changed triggered estate sites they refuse 22%. In a 30-sample of those refusals the verifier found about 25 good conversions and about 5 real errors. The plan now says to replace the gates deliberately, not drop them.

## On the brief

The brief worked: consent, the role split and the quoted disciplines all landed. Two lessons, one mine and one the verifier's:
- **Per-agent scratch directories.** I launched eight labelers without giving each its own scratch directory. Two collided in the shared session scratchpad, and one saw the other's batch.
- **Freeze before labeling.** Sets B and C were labeled while I was still developing the converter, and the labelers' reports, which named items, reached me before the freeze. I had decided not to act on them. But at least one rule (file names like `f_0080.xhtml`) arrived by two routes, my own drill and a report naming the same file, and it moved 11 of C's untriggered items. Only D was done in the right order: freeze, then launch the labelers, then read their reports.

The spike template might carry both.
