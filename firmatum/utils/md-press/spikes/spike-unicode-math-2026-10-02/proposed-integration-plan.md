# Proposed integration plan: a deterministic math converter in md-press

*Suggestions from the spiker, each with the evidence behind it. This is not a work order. The integrator will see the md-press code and the current state of the tree better than I could from inside the spike, and should override any of this where the evidence there points elsewhere. The measured results are in `README.md` §Results. The section at the end lists framings I suspect are mine rather than the evidence's.*

## What would be integrated

The pure-Rust port in `rs/umath/` (see `rs/umath/PORT.md`, written by the porting agent): 0 differences from the Python reference for v3 through v7 over gold, estate, math-free and fuzz inputs; v7 is its default. v7 is the converter I'd integrate. Only v3/v4 have clean held-out numbers (set D); v5–v7 are robustness and bug fixes made after D was seen. The deltas since v3 are small and listed in `notes/LOG.md` §19, §24, §26 and §27:
- the public `convert` iterates to a fixed point, and leaves the text as written if no fixed point is reached;
- a final self-check requires that the output is the input with only some regions replaced;
- hazards are checked before any rewrite;
- the `\(…\)` glyph set is exactly md-press's;
- `$$` inside running text and an invalid existing span both count as hazards;
- (v5) a site whose code and `$` delimiters interleave is left alone, and no span may contain the brackets of `[text](dest)` link text;
- (v6) fixes for the port's bug list: a 𝟊 crash, a superscript merge that produced undefined commands (plus a `commands_known` gate), the quoted-mention rule at end-of-text, and ⋃/⋂ as large operators;
- (v7) a word hyphen before an operator name is not a minus (`$n$-dim` had become `$n - \dim$` at 25 estate sites), and a site with a `$` inside inline code is left alone (md-press's `edit_pairs` doesn't mask code).
  - One known rough edge in v7: `log-det/λ` becomes `log-$\det/\lambda$`. The compound is split at the math boundary instead of kept whole. That beats v6's `$\log - \det/\lambda$`, but it isn't what the rule intended. The port pins it as frozen behavior.

The interface is `convert(text) -> (text', spans)`, applied to one prose site with its markdown structure already split off. That is exactly what `split_structure` produces in `src/math.rs`.

## Suggested shape

1. **Replace the model call, not the site machinery.** md-press's parse-sited design is right and the converter assumes it: whole paragraph/heading lines and table cells, with `split_structure` removing and reattaching the prefix verbatim. Keep `promote_site` and the cell handling. Inside it, call the converter on the whole site body instead of `promote_text` per piece.

2. **Feed whole sites; the piece splitting is optional for this converter.** `split_at_prose_separators` and `split_sentences` existed for the LLM's cost and safety, and they do cut real expressions in half. Independent labelers found about 9 of 135 pieces per batch cut mid-expression: `κ×A/tempo`, `0.02 ≤ |CUBE| ≤ 0.10`, `1/√α`, `R_test = Σ(w_i × coverage_i) / Σ(w_i)`. The converter makes the weak-glyph role decision itself, so it doesn't need the pre-split. The measured effect, though, is small. On held-out D lines (n=200), whole-line conversion gets 86.0% exact-or-equivalent and 3.5% wrong+over; the same lines through md-press's splitter, piece by piece, get 84.5% and 3.0%. On C lines: 84.0% / 3.0% vs 83.0% / 3.0%. Whole lines win 4 items and lose 2 on D (`notes/LOG.md` §23; scored with scorer v1). I'd still feed whole sites, because it is simpler and keeps expressions intact, but that's a mild preference, not a measured necessity.

3. **Gates: the LLM gates as they stand over-refuse for this converter, but they also catch some real errors, so replace them deliberately rather than drop them.**
   - **They over-refuse.** The verifier ran v6's output for all 7,826 changed triggered estate sites through md-press's real gates: 22% refused. A random 30 of the refused were about 25 good conversions (`±ρ`, `π*`, `λ ≈ 0.02/year`) and about 5 real catches (flattened PDF subscripts, inconsistently fragmented APA statistics, the hyphen-operator class v7 now fixes, `pilot-A · κ×A`).
   - **My earlier A+B figure undercounted what they catch.** I had reported that the gates refused 77 correct of 540 and passed 2 of 4 wrong outputs. The pre-feedback scorer could not see some of the errors they catch.
   - **Why they misfire on this converter.** They were built for LLM failure modes, and they don't know `₀`→`0`, `\text{word}`, `\succeq` and so on.
   
   The converter's own checks are structural:
   - the alignment self-check (the output is the input with regions replaced; the same idea as `edit_pairs`);
   - `tex_ok` on every span (balanced braces, required arguments present, no double scripts, no `&`/`#`/`%`), and from v6 `commands_known`. Over the 4,820 distinct spans v7 emits on the estate and gold, KaTeX (strict) and MathJax find 0 invalid;
   - fixed-point idempotence;
   - md-press's `protected_ranges` masking.
   
   - If defense-in-depth gates are wanted, the alignment check itself (`edit_pairs`: the output is the input with regions replaced) is the part that applies unchanged. `edits_confined` is more than that: it calls `operands_survive` and `region_reads_as_math` on every region and checks for prose-role weak glyphs and `**`/backticks, so keeping it keeps those.
   - What I'd do: run `edit_pairs` plus the emphasis and code-delimiter checks (including the new single-`*` check the coordinator added) as hard gates. Make `operands_survive`, `math_content_consistent` and `preserves_prose` Unicode-aware before trusting their refusals.
   - Before any of that, read a sample of what each gate refuses on the integrated tree. The verifier's 30-sample shows the refusals are mixed, not uniformly wrong.

4. **Scope: start with the sites today's trigger selects; expand only on evidence.** On whole triggered lines the converter does far more of the job than the model, at a similar error rate per edit (README §Results). On untriggered sites wrong+over is much higher: 20% on D, 26% against the worse labeler. Estate-wide, v7 changes 7,813 triggered and 8,290 untriggered sites, so untriggered sites are about half of what it would change. They are also where the label-vs-variable and log-field cases live (`t=5`, `H_D3`, `W₁`, `E¹`). Suggested sequence:
   - (a) replace the model on the triggered sites;
   - (b) build the label/convention mechanism (item 6) and the in-line genre rules (next section: log fields, units, footnote markers), and score them on a *fresh* labeled set;
   - (c) only then widen the trigger. Unicode script characters (`W₁`, `R²`, `10⁻⁶`) are the tempting first widening, but on D they were the *worst* untriggered subpopulation: 31% wrong+over, against 17% for the rest, because labels and units concentrate there. Scripts on Greek bases (`α₁`, `ρ²`) are the safest first slice to measure;
   - (d) run fully untriggered ASCII math (`n_past`, `O(n)`, `N=40`) only behind a flag.

5. **`--check` becomes reproducible again.** The converter is a pure function of the text and a compiled-in data table, with no ollama and no model version, so the proposal cache and its documentation can go. Speed is no longer a constraint: the Rust port converts all 1.29M estate prose sites in 11.2 s on one thread and 1.4 s on 12 (`rs/umath/PORT.md`). So whether to keep the trigger is purely a scope and precision decision (item 4).

5b. **From the porting agent, unmeasured but concrete** (`rs/umath/PORT.md` §How it should plug in):
   - **Let the converter own `\(…\)` normalization.** From v4 on, it checks `$` hazards on the raw text before normalizing; if md-press's `normalize_paren_math` runs first, the converter sees different text than was measured.
   - **Treat an `Err` as "leave the text and flag it".** From v6, only adversarial nesting returns one.
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

On D, about half of v4's untriggered errors need knowledge outside the line (labels), and about half are decidable from the line itself (genre and mention); see README §Where the hard boundary is. The ones a converter rule could address:
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
  - pseudo-formulas with word operands (`failed_systems(t) = {…}`, `impact(t) = Σ(…)`), which labelers typeset with `\text{}`. Whether md-press should is a style question, and `_` inside `\text{}` collides with the house `_`→`-` rule.
- **The emphasis collision in `deterministic-π* scope*`** (DL190): the `*` closes an italic in the source. Converting `π*` → `$\pi^\ast$` would repair the emphasis (both labelers did), but the converter's emphasis pairing currently reads that `*` as emphasis.

## Incidental md-press findings (independent of this converter)

- **Fixed in the working tree, not yet committed:** YAML frontmatter preceded by an HTML comment used to be joined by the unwrap stage, putting YAML keys inside comments. It passed the render check. Two C labelers found it and I reproduced it (`notes/LOG.md` §25). The coordinator has fixed it with a regression test; all 30 affected files now keep their frontmatter byte-identical.
- **Fixed in the working tree, not yet committed:** md-press's `edits_confined` checked only `**` and backticks, so a deleted single `*` emphasis marker passed. The verifier found llama doing this on D (DL160, DL172). The coordinator's fix: every `*` in a replaced region must survive as `*` or `\ast`.
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

- **The equivalence ladder** (`py/atoms.py`) is my construction, and it already hid one real bug class: until scorer v2, a word hyphen made into a minus scored as "equivalent". "Equivalent" still folds italic vs upright letters (`S_{id}` vs `S_{\text{id}}`), `E` vs `𝔼`, `\Sigma` vs `\sum`, prose text vs upright operator names, and merged slash lists (`$\alpha/\beta$` vs `$\alpha$/$\beta$`, which in math reads as division). These are judgment calls. `results.md` shows the worse-labeler and strict-slash readings next to the default.
- **"Letters need evidence."** The converter promotes a bare Latin letter only when the text gives evidence: an operator, application, or a symbol seen in this text's math. That costs recall, and labelers did promote bare variables. I chose precision; the LLM-era design made the same choice. Whether that's right for md-press is Joseph's call, not mine.
- **"The LLM adds nothing as a fallback"** was measured only with today's prompt and gates, on sets A and B, with llama3.2 and Muse. A model prompted for exactly the converter's residue wasn't tried. Since about half that residue is in-line genre and mention judgment, it's worth trying.
- **The weak-glyph operand rule** (an arrow or `·` is math only beside math) is inherited from md-press's current design. I strengthened it but never questioned it from scratch.
- **The learned bare-letter forest was measured and left out** (`notes/LOG.md` §13). It's a negative result on *this* training source (authors' wrapping choices in LaTeX-written lines). It is not evidence that learning can't help.
- **The house-label lexicon thresholds** (≥8 raw uses, <10% in math, ≥2 files, canon only) were set by me after looking at what they flag.
