# Proposed integration plan: a deterministic math converter in md-press

*Suggestions from the spiker, each with the evidence behind it. This is not a work order. The integrator will see the md-press code and the current state of the tree better than I could from inside the spike, and should override any of this where the evidence there points elsewhere. The measured results are in `README.md` §Results. The section at the end lists framings I suspect are mine rather than the evidence's.*

## What would be integrated

The pure-Rust port in `rs/umath/` (see `rs/umath/PORT.md`, written by the porting agent): 0 differences from the Python reference for v3, v4 and v5. Mirroring v6 was in progress at the time of writing; check PORT.md for which version is primary. v6 is the measured converter plus robustness fixes. Its output equals v4's on 1,389 of the 1,390 gold items, with identical verdicts. The v3→v6 deltas are small and listed in `notes/LOG.md` §19, §24 and §26:
- the public `convert` iterates to a fixed point, and leaves the text as written if no fixed point is reached;
- a final self-check requires that the output is the input with only some regions replaced;
- hazards are checked before any rewrite;
- the `\(…\)` glyph set is exactly md-press's;
- `$$` inside running text and an invalid existing span both count as hazards;
- (v5) a site whose code and `$` delimiters interleave is left alone, and no span may contain the brackets of `[text](dest)` link text;
- (v6) fixes for the port's bug list: a 𝟊 crash, a superscript merge that produced undefined commands (plus a `commands_known` gate), the quoted-mention rule at end-of-text, and ⋃/⋂ as large operators.

The interface is `convert(text) -> (text', spans)`, applied to one prose site with its markdown structure already split off. That is exactly what `split_structure` produces in `src/math.rs`.

## Suggested shape

1. **Replace the model call, not the site machinery.** md-press's parse-sited design is right and the converter assumes it: whole paragraph/heading lines and table cells, with `split_structure` removing and reattaching the prefix verbatim. Keep `promote_site` and the cell handling. Inside it, call the converter on the whole site body instead of `promote_text` per piece.

2. **Feed whole sites; the piece splitting is optional for this converter.** `split_at_prose_separators` and `split_sentences` existed for the LLM's cost and safety, and they do cut real expressions in half. Independent labelers found about 9 of 135 pieces per batch cut mid-expression: `κ×A/tempo`, `0.02 ≤ |CUBE| ≤ 0.10`, `1/√α`, `R_test = Σ(w_i × coverage_i) / Σ(w_i)`. The converter makes the weak-glyph role decision itself, so it doesn't need the pre-split. The measured effect, though, is small. On held-out D lines (n=200), whole-line conversion gets 86.0% exact-or-equivalent and 3.5% wrong+over; the same lines through md-press's splitter, piece by piece, get 84.5% and 3.0%. On C lines: 84.0% / 3.0% vs 83.0% / 3.0%. Whole lines win 4 items and lose 2 on D (`notes/LOG.md` §23). I'd still feed whole sites, because it is simpler and keeps expressions intact, but that's a mild preference, not a measured necessity.

3. **Gates: give it its own invariants, and don't run it through the LLM gates as they stand.** Run through today's gates, 90 of 540 converter outputs on A+B were refused; 77 of those were exact or equivalent to gold. Meanwhile the gates passed 2 of the converter's 4 wrong outputs. Those gates were built for LLM failure modes; they don't know `₀`→`0`, `\text{word}`, `\succeq`, and so on. The converter's own checks are structural:
   - the alignment self-check (the output is the input with regions replaced; the same idea as `edit_pairs`);
   - `tex_ok` on every span (balanced braces, required arguments present, no double scripts, no `&`/`#`/`%`). Over 4,551 distinct emitted spans it agreed with KaTeX (strict) and MathJax: 0 invalid;
   - fixed-point idempotence;
   - md-press's `protected_ranges` masking.
   
   If defense-in-depth gates are wanted, `edits_confined` (the alignment gate) is the one that still makes sense. `operands_survive`, `math_content_consistent` and `preserves_prose` would need Unicode-aware updates before they could judge this converter's output fairly.

4. **Scope: start with the sites today's trigger selects; expand only on evidence.** On whole triggered lines the converter is the strongest measured option (README §Results). On untriggered sites, which it would also convert, wrong+over is higher. Those sites are most of what it changes estate-wide (16k sites vs 8.6k triggered), and that's where the label-vs-variable and log-field cases live (`t=5`, `H_D3`, `W₁`, `E¹`). Suggested sequence:
   - (a) replace the model on the triggered sites;
   - (b) add Unicode sub/superscript digits (`W₁`, `R²`, `10⁻⁶`) to the trigger, since they are real math the trigger misses;
   - (c) run fully untriggered ASCII math (`n_past`, `O(n)`, `N=40`) only behind a flag until a label/convention mechanism (item 6) exists.

5. **`--check` becomes reproducible again.** The converter is a pure function of the text and a compiled-in data table, with no ollama and no model version, so the proposal cache and its documentation can go. Speed is no longer a constraint: the Rust port converts all 1.29M estate prose sites in 11.2 s on one thread and 1.4 s on 12 (`rs/umath/PORT.md`). So whether to keep the trigger is purely a scope and precision decision (item 4).

5b. **From the porting agent, unmeasured but concrete** (`rs/umath/PORT.md` §How it should plug in):
   - **Let the converter own `\(…\)` normalization.** From v4 on, it checks `$` hazards on the raw text before normalizing; if md-press's `normalize_paren_math` runs first, the converter sees different text than was measured.
   - **Treat an `Err` as "leave the text and flag it".** Only adversarial nesting returns one in v6.
   - **Span offsets are chars, md-press indexes bytes.** The output text alone needs no conversion.
   - **Keep `tools/differential.sh` plus the fuzz as the regression gate** for any later converter version.

6. **Conventions and labels from canon, as data.** `py/lexicon.py` mines asf's format-gated canon (`0[1-4]-*-core/src/`) for Unicode tokens the house deliberately keeps as prose. It finds exactly W₀ and W₁ and nothing spurious. Over the whole estate the same statistic is polluted by unconverted math (it flags `β_0`, `ϵ`), so the canon restriction is load-bearing. The same mining approach could settle other conventions where labelers split and canon doesn't: `M_{\tau^+}` appears 390× and `M_\tau^+` 0×; `\phi` vs `\varphi`; `S_{\text{id}}`. I'd treat the mined table as build-time data, regenerated by a script, never hand-edited, possibly per project.

7. **Exclusions, not converter rules, for glyph-as-content files.** These are better declared in `.md-pressignore` than inferred:
   - glyph tables (`firmatum/utils/utf/bmp-tables.md`);
   - glyph-perception survey data (`asf/empirica/glyph-magnitude-perception/`);
   - vendored READMEs (`_self/refs/…llama.cpp`);
   - raw tool-output captures (`_self/tmp-asf-xref.md`);
   - session listings.
   
   Labelers in every round flagged these as files md-press arguably shouldn't touch. This is the same declared-not-inferred principle md-press already applies to transcripts.

## Next converter improvements (informed by D, so unmeasured; a fresh labeled set is needed to score them)

On D, v4's errors and misses are mostly not converter bugs. The ones a converter rule could address:
- **Untriggered precision** (DU, 20% wrong+over):
  - a declared or canon-mined label list per project (asf: W₀/W₁/W₂ regimes and `W₁ᶜ`-style variants; causal-language: `H_D1`, `H_D3`, …);
  - "single capital + superscript digit alone in a table cell" as a footnote marker (`E³`, `X⁸`);
  - SI unit expressions (`m/s`, `kg/m³`, `m²` with no adjacent math) left as written;
  - `key=N` fields in log-shaped lines (a timestamp or a code-span ID earlier on the line) left as written.
- **Recall on lines** (D's 21 degraded):
  - `~` as `\sim` between math operands (`a_t ~ π(·)`);
  - precomposed macron or dot letters before `/` or an operator (`ā/(1-β)`);
  - `dV/dt` with no Unicode signal;
  - ASCII single-letter variables (`O, A, h`, `x`, `k`, `E[size]`), which is the policy question in the do-not-inherit list;
  - pseudo-formulas with word operands (`failed_systems(t) = {…}`, `impact(t) = Σ(…)`), which labelers typeset with `	ext{}`. Whether md-press should is a style question, and `_` inside `	ext{}` collides with the house `_`→`-` rule.
- **The emphasis collision in `deterministic-π* scope*`** (DL190): the `*` closes an italic in the source. Converting `π*` → `$\pi^\ast$` would repair the emphasis (both labelers did), but the converter's emphasis pairing currently reads that `*` as emphasis.

## Incidental md-press findings (independent of this converter)

- **YAML frontmatter preceded by an HTML comment** is parsed as paragraph text and *joined* by the unwrap stage: `register: … # comment` swallows `support-kind: …`, so YAML keys disappear into comments, and `  ` markers are added to other keys. The render-equality check passes, so nothing stops it. Example: `firmatum/verisectorium/theory/influx/instrumenta/bridges/tools-are-observation-infrastructure.md`. Two C labelers reported it independently; I reproduced it with today's build (`notes/LOG.md` §25).
- **A stray code fence right after frontmatter** flips fence pairing for the whole file: about 108 of 961 files in `_core/tst/planning/analysis/`. ASCII-laid-out formulas then parse as prose and get unwrapped (B245, B252). A warning might be worth having.
- **Box-drawing tables without a fence** are unwrapped into one paragraph (A205). That's render-equal, and the on-disk layout is lost.
- **llama3.2 wrong-but-accepted on set B under today's gates, including `operands_survive`:**
  - `ρ/R` → `\rho/\rho_R`
  - `T` → `\mathcal{T}`
  - a swallowed `?`
  - a raw combining hat left inside math (`p̂_k`)
  
  The data is in `data/llama-B-judged.jsonl`.
- **The splitter cutting expressions** (item 2) is a hard limit for the pieces it cuts: no converter can recover a span across the cut. The measured net cost for this converter is about 1–1.5 points.
- **Currency `$` beside a trigger glyph** (`≤$1M`) is a pairing hazard for any converter, including today's model path. The converter refuses any site with a `$` followed by a digit.

## Do-not-inherit (framings that may be mine, not the evidence's)

- **The equivalence ladder** (`py/atoms.py`) is my construction. "Equivalent" folds italic vs upright letters (`S_{id}` vs `S_{\text{id}}`), `E` vs `𝔼`, `\Sigma` vs `\sum`, and math vs text font for punctuation. These are judgment calls; a stricter scorer would move items from "equivalent" to "wrong". Exact-only numbers are reported alongside so the effect can be seen.
- **"Letters need evidence."** The converter promotes a bare Latin letter only when the text gives evidence: an operator, application, or a symbol seen in this text's math. That costs recall, and labelers did promote bare variables. I chose precision; the LLM-era design made the same choice. Whether that's right for md-press is Joseph's call, not mine.
- **"The LLM adds nothing as a fallback"** was measured only with today's prompt and gates, on sets A and B, with llama3.2 and Muse. A model prompted for exactly the converter's residue wasn't tried.
- **The weak-glyph operand rule** (an arrow or `·` is math only beside math) is inherited from md-press's current design. I strengthened it but never questioned it from scratch.
- **The learned bare-letter forest was measured and left out** (`notes/LOG.md` §13). It's a negative result on *this* training source (authors' wrapping choices in LaTeX-written lines). It is not evidence that learning can't help.
- **The house-label lexicon thresholds** (≥8 raw uses, <10% in math, ≥2 files, canon only) were set by me after looking at what they flag.
