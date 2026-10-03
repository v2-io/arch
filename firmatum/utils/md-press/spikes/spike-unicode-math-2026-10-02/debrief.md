# Debrief for Joseph: the deterministic math converter

**The converter.** A deterministic converter does md-press's math job better than llama3.2 on the text md-press sends it today. The Python reference is `py/umath.py`. A delegated pure-Rust port in `rs/umath/` matches it with 0 differences over 4.6M inputs, and runs the 1.29M estate prose sites in 11 s on one thread.

- *Held out, on set B:* 84.7% of pieces exactly or equivalently right, against 45.3% for llama as md-press writes it after its gates.
- *Error per write:* 3.9% of the pieces it changes are wrong or over-converted, against llama's 4.9%. It changes 279 pieces where llama changes 122.
- *Muse Glimmer 30B* sits between them, on set A: 62.9% right, 2.9% of its writes wrong.
- *On whole prose lines,* the unit md-press would actually feed it, the frozen v4 scores 86.0% right and 3.5% wrong or over-converted, on 200 lines labeled after it was frozen. The recommended v5 adds two malformed-input guards and gives byte-identical output on every gold item.

It needs no model, it's a pure function of the text, and it is idempotent (iterated to a fixed point). KaTeX and MathJax both render all 4,837 distinct spans it emits over the estate and the gold sets. The port's fuzz found two adversarial inputs that made it emit undefined commands (`Ξ*^T` → `\Xi^{\astT}`) and one that crashed it (𝟊). v6 fixes both and adds a known-commands gate. So `--check` stops depending on ollama.

**Recommendation.** Replace the model on the sites md-press already triggers on. Don't widen the scope yet. On untriggered sites roughly one in five of its changes is wrong (D: 20.0% of 150).

## What decides the residue

I expected the hard boundary to be language understanding. It isn't, in the sense that the cases it gets wrong would also defeat a better line-level model. The two labelers per item settled them by reading things outside the line:

- **Labels glyph-identical to variables.** asf's canon writes the regime names W₀/W₁ as Unicode prose (W₁: 139 raw, 1 inside math) and keeps `$W_2$` for Wasserstein. causal-language's `H_D` hypothesis IDs appear 215 times in its markdown and LaTeX (H_D3 alone 135) and never inside math. `E³` and `X⁸` are grades with footnote markers.
- **Genre.** `t=5` in session listings, units like `kg/m³`, run metadata.
- **Mention versus use.** Glyph tables, a sentence *about* `𝒜`, a transcript line saying `n_past` lacks LaTeX.

These are conventions, so they want declaring, not inferring. I tested inference with house-usage statistics. Mined from asf's format-gated canon (`0[1-4]-*-core/src/`), they recover exactly {W₀, W₁}. Mined over the whole estate, the same statistic also flags `β_0` and `ϵ`, which are lazy unconverted math rather than labels: the estate is itself the population md-press is treating. In projects that never write LaTeX (causal-language), there is no evidence at all. So the labels question is yours: per-project declared lists, canon-mined data, or neither.

Three policy calls the labelers split on, which I left conservative:
- bare ASCII variables with no Unicode signal (`Here U is…`; the converter wants evidence first);
- statistics like `N=40`, `p<.05` (converted only when a relation ties a letter to a value);
- ranges inside relations (`ε ≈ 0.1-0.4`, emitted as `0.1\text{–}0.4`).

## What your suggestions bought

- **The "effectively-equivalent" class.** On the first dev run, 47 pieces scored "wrong". Many were spellings any reader would accept: `$\alpha/\beta$` vs `$\alpha$/$\beta$`, `\sqrt h`, `S_{id}` vs `S_{\text{id}}`. The scorer now compares visual atoms (glyph, script position, font) and separates *degraded* (left partly as written, safe) from *wrong* (prose italicized, structure changed). Every table also carries exact-only counts, so the class's effect stays visible.
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

- **Unwrap joins frontmatter.** YAML frontmatter that follows an HTML comment is joined like prose, so YAML keys end up inside the preceding key's `#` comment. It passes the render check. Two labelers found it independently, and I reproduced it on today's build (`firmatum/verisectorium/.../tools-are-observation-infrastructure.md`).
- **The splitter cuts expressions.** `split_at_prose_separators` cuts real expressions apart (`κ×A/tempo`, `0.02 ≤ |CUBE| ≤ 0.10`, `1/√α`). For this converter the measured cost is small: on D, whole lines score 86.0% against 84.5% fed piecewise through md-press's own splitters.
- **Wrong conversions pass today's gates.** On B, four of llama's wrong conversions pass all of them, including `operands_survive`: `ρ/R` → `\rho/\rho_R`, `T` → `\mathcal{T}`, a swallowed `?`, and a raw combining hat left in math.
- **The gates refuse good output.** Run over an earlier version of the converter's output on A+B, they refuse 77 correct conversions out of 540 while passing half its errors. They were tuned to LLM failure modes.

## On the brief

The brief worked: consent, the role split and the quoted disciplines all landed. One mechanical lesson cost me. I launched eight labelers without giving each its own scratch directory; two collided in the shared session scratchpad, and one saw the other's batch. I caught it in their reports, measured the contamination, and used per-agent directories from then on. The spike template might carry that line.
