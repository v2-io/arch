# Lab notebook — unicode-math spike (2026-10-02)

Chronological, unpolished. Claims here are working claims; the README/debrief carry the corrected ones.

## 0. Orientation (read whole): STATUS.md, README.md, src/math.rs, src/lib.rs, src/main.rs, model/README.md, FEEDBACK-08-12/08-22, asf format.sop.md §Math/§Notation, inputs/README.md, harness/*.

## 1. Literature scan (quick)

- **Unicode TN28 (Sargent, UnicodeMath v3.3, Jan 2025) §5 "Recognizing Mathematical Expressions"** is the direct prior art: a deterministic character-property state machine for math-zone autodetection in plain text (deployed in Word/RichEdit). Single ASCII letters other than a/A/I start a math zone; a/A/I only if followed by an operator/comma/period; Greek letters, math alphanumerics, U+2200–22FF etc. start one; an unrecognized letter span ends a zone unless preceded by a binary/relational op or a number, or followed by `^`; spaces end a zone except around operators; trailing space/,/./( excluded; paren and bra-ket counters. Its own caveat: "recognition heuristics are not infallible" — math zones are preferable. Saved text: scratchpad (pdftotext of the PDF).
- T.V. Raman's AsTeR thesis: token classification by TeX atom classes (Ord/Op/Bin/Rel/Open/Close/Punct) — same idea, the classifying vocabulary I'd use.
- A USPTO "formula detection engine" patent: seed from math Unicode blocks, expand.
- pylatexenc (unicode↔latex char tables), Obsidian "fix-math" plugin (heuristic delimiters).
- Nothing found on learned sequence labeling for *this* setting (Unicode math in markdown prose); formula detection work is overwhelmingly PDF/image. Roughly what I expected: the known art is deterministic seed-and-expand; no ceremony.

What's different here vs TN28: agent-written markdown dialect — `_word` subscripts (`κ_processing`, `n_future`), hyphen compounds (`κ-bridge`, `deterministic-π*`), house uses of `→ · ×` as punctuation, markdown emphasis inside/around math, escaped table pipes, existing `$…$` mixed in.

## 2. Gold labels (launched early, background)

Blind double-labeling of 540 real pieces: set A = the coordinator's 240 sample (gid A###, so all three converters compare on the same items), set B = 300 fresh random distinct pieces (seed 20261002; gid B###) = **held-out test, not to be inspected during development**. Shuffled, 4 batches × 135, two independent passes (8 Opus agents). Brief verbatim: data/gold/BRIEF.md. Labelers barred from the other pass and from the model proposals (anchoring).
Agents: p1b0 afb3c0bf21188e256 · p1b1 a88c09f0415f46462 · p1b2 a8d06b2c566bba8af · p1b3 a7f65b6c95d84db66 · p2b0 a6460f04ef4e252c3 · p2b1 a4385ef90fa5432fc · p2b2 a50ca53cb0fbe41b1 · p2b3 a778bf9b631175f18.
Contamination note: before building B I printed 70 seed-7 random pieces to get a feel; a few may be in B (to check: count overlap).

## 3. Joseph, mid-spike (verbatim)

> Because most of the corpus here is already transformed into latex, you have the expected output and you can easily just create unicode versions (or delegate sonnet and opus agents to do so) for the "source" data...
> That will give you significant data-- especially all of the markdown in asf/

Plan: (i) extract prose lines with `$…$` from asf (and elsewhere) through md-press's own parse; (ii) Unicode versions two ways — agent-written (Sonnet/Opus/Haiku asked to write the line as they naturally would without `$` math: this samples the disease dialect itself) and a deterministic reverse map (cheap, unlimited, but drawn from *my* idea of the distribution — the bias the brief warns about); (iii) real-piece blind gold stays the arbiter for any claim.

## 4. Labeler reports (p1b1, p1b2, p2b1, p2b2 done) — incidental findings, high value

- **md-press's piece splitting cuts live math** (independently found by 3 labelers): `split_at_prose_separators` splits at weak glyphs judged prose when a neighbour is a word: A122 source `κ×A/tempo/persistence` (cut at ×), A130 `0.02 ≤ |CUBE| ≤ 0.10` (CUBE judged prose), A224 `1/√2` cut to `1/`, B252 `Σ(users(s) × criticality(s))`, A221 `−0.332°·sin(8ψ)`, A056 `1/α vs 1/√α`, A203, A096, A179, B053, B158/B269, A199. ⇒ Any converter fed pieces is capped. **Reframe candidate:** a deterministic converter is cheap, so it can take the whole prose site (line/cell) and the weak-glyph role decision becomes part of span detection, the same problem rather than a pre-split.
- **Existing `$…$` adjacent to new math** (`$k$ ≈ 0.5`): merge into one span is the right answer and md-press's gate already allows merging (pre-existing content survives inside a wider span). Labelers split on whether it's producible.
- **Currency `$`** on the same site (B078 `≤$1M`): any new span pairs with it. Hazard class for any converter and for md-press's balance gate.
- **Stray code fence after frontmatter** flips fence pairing (B245, B252; ~108 of 961 files in `_core/tst/planning/analysis/`) → ASCII-laid-out formulas parsed as prose and unwrapped. Unwrap-stage issue, not math.
- **Out-of-scope material reaching the math pass**: glyph-perception survey data (asf/empirica/glyph-magnitude-perception), vendored llama.cpp README, transcripts, PDF-pasted papers (η2p = η²_p lost superscript). `.md-pressignore` candidates.
- **Conventions labelers settled from asf on disk**: `\mathbb{E}`, `\mid` for conditioning, φ→`\phi` (asf uses `\phi` 141× vs `\varphi` 4×), sub-scope labels as math (`metric-$\alpha_2$`), ranges with the dash outside math (`$\rho = 0.80$-$0.94$`).
- **Ill-defined classes** (labeler-named): slash lists `α/β` (separate spans vs one), relation with one-word operand (`period ∈ {…}`), ASCII variables beside Unicode math, Greek-as-label. Scoring should treat these as their own classes.

## 5. Gold landed — and one independence incident (my brief's fault)

All 8 batches done (1,080 labels). **Incident:** the session scratchpad is shared by every agent I launch (and by the coordinating session — it holds their judged/prop files too). p1b0 and p2b0 both wrote `scratchpad/gold0.py`; p2b0's review printout ran on p1b0's file, so p2b0 saw pass-1 batch-0 labels (it reports its decisions were already written and restored them verbatim). Evidence it may still matter: batch-0 byte-identical agreement 129/135 vs 118/112/122 in batches 1–3. ⇒ Inter-labeler agreement is reported on batches 1–3 only; batch 0 is still usable as gold (contamination makes the two passes more alike, which makes "match either labeler" scoring stricter, not looser). Lesson for the brief template: give each parallel agent its own scratch path.

Inter-labeler agreement (normalized, span-level exact): 504/540 = 93.3% all; batches 1–3: 373/405 = 92.1%. Disagreements on A are mostly spelling the normalizer doesn't fold (`\mathbb{E}` vs `E`, `{\sim}1`), restructurings (`\prod(p_j^{Pa_j})` vs `\prod_j p_j^{Pa_j}`), merge-with-existing-span vs not, and keep-vs-convert on glyph-as-label (`(B∞)`, `adaptive ⊃ agency`). B078 (`≤$1M`) is unrepresentable for both labelers (currency `$`).

## 6. First dev numbers (set A, 240) and Joseph's equivalence point

v0 det: correct 169 / partial-safe 24 / wrong 47 (llama 77/145/18, muse 130/94/16; "wrong" = written span disagrees with *both* labelers, span-normalized). Error classes in det's 47: slash pairs `α/β` (labelers: `$\alpha$/$\beta$`, asf canon 15:1), op-anchor bug (operands not absorbed: `p ≤ $0.005$`), tuples `(O, Σ)`, application with words `ρ(consistency, −IQR)`, `√` arg grouping, contractions (`I'd` → `$I'd$`!), Greek numerals `α΄`, box-drawing tables, splitter fragments (`t < τ}`), derivatives `d‖δ‖/dt`, `↓κ`, escaped star `ε\*(N)`, isotope `δ¹⁸O`, PDF-flattened `τ1`/`η2p`, word subscripts text-vs-italic (`S_id`, `d_FR`, `ρ_oc`), prose arrows between existing spans (`$h$ → $Q_\theta$`), number-label operands (`Class 1 → κ≈0`).

Joseph (verbatim, mid-spike): "There should probably be an "effectively-equivalent" class". Agreed — several "wrong" are style variants. Plan: a LaTeX→visual-atom renderer (glyph, script position, font class) — also the reverse map for synthesis — and a graded verdict: exact / equivalent (same atoms modulo spacing, braces, punctuation font) / degraded (system leaves in text font some atoms gold typesets; no meaning change) / wrong (any other atom difference: prose turned math-italic, structure or symbol changed).

## 7. Joseph, mid-spike (verbatim)

> A couple of minor thoughts:
> Whether or not a symbol is seen in a prior pass's $..$ spans might weigh it toward being a variable. " i' will" might not seem like math normally, but if within the same block there is a span that has $  i' = i + 1 ...$ or something it might indicate higher probability of being math.  The other thought-- be sure that there are plenty of tests that just throw lots of normal math-free text at it. The one thing you *shouldn't* need to worry about as per md-press's architecture, IIRC, is things within code fences...

Response: symbol table (bare letters seen as operands in this text's spans or existing `$…$`) already in; extending to decorated forms, and demoting primed Latin letters (`x'`) from unconditional anchor to evidence-needing. Math-free drill: every prose site in the estate that today's trigger does NOT select, through the converter; any change there is a candidate false positive. Code fences: confirmed out of reach (no MathSite); inline code is masked by protected_ranges, which I ported.

Dev A after fixes so far (tuned on A — no longer an unbiased estimate): det exact+equiv 218/240 (90.8%), wrong 2 (A112 glyph-survey soup; A211 integral limits), degraded 20 (ASCII variables the labelers promoted).

## 8. Precision drills

**Estate, all 1.33M prose sites** (probe `rs/probe` → `sites.jsonl`; 8,638 triggered by md-press's own detector): converter changes 19.3k sites, 12.8k of them untriggered. Untriggered changes are mostly real math the trigger misses (`W₁` Unicode subscripts aren't in the trigger at all; `n_past`, `M_t`, `O(n²)`, `R²`, `10⁻⁶`), plus real false positives found: numeric-only spans (`Elixir ≥ $1.6$`), `|---|` table rules as abs-value, transcript truncations (`import p...sor`), log fields `t=9`.

**External math-free markdown** (13 `_ref` repos: anthropic SDKs, codex, gemini-cli, ink, obsidian-help/linter, pachyderm, claude-docs…; 5,143 files, 2.47M sites): 471 changed. Three bug classes: (1) **currency `$`** — `$5 / MTok · Output pricing: $25` reads as an existing math span; (2) **inline `<script>` JS** in a docs page — `f=3*!!a,g=4,h=e+g+f`; (3) **number+unit** `>=200k` → `$200k$`.

## 9. HELD-OUT RESULT — v1 frozen (py/frozen/umath_v1.py, sha1 0a2afaf1…) run once on set B

Set B (300 fresh random pieces, never inspected during development; 4 of them were in my early 70-piece eyeball sample):
| system | exact | equivalent | degraded | over | wrong | exact+equiv |
|---|---|---|---|---|---|---|
| none (leave as is) | 17 | 22 | 261 | 0 | 0 | 39 (13.0%) |
| det v1 | 191 | 63 | 35 | 2 | 9 | 254 (84.7%) |
Same build on dev A (tuned on it): 220/240 (91.7%), 2 wrong. So ~7 points of the A number is fit to A.
Verdicts are the visual-atom ladder, best against either labeler. "degraded" = left some gold-typeset atoms as prose (safe under-conversion), "over" = typeset a Greek/symbol the labelers left, "wrong" = anything else (prose letters italicized, structure/symbol differs).
From here on B is *seen*: any later change is tuned on A+B and needs a fresh set for an honest number.

B v1 errors (9 wrong, 2 over), read in full:
- **3 of 9 wrong are one speculative heuristic of mine**: `x_t+1` → `x_{t+1}` (subscript absorbs a glued `+`/`-` and a short token). On B it produced `U_M/(U_{M+U}_o)` (B122, B133, B208) and `\Sigma_{t-as}` (B180, the hyphen variant). It came from *my idea* of agent Unicode, never from data — exactly the brief's warning. Removed: render literally.
- `kg/m³` → `kg/$m^3$` (unit with superscript); `0₃ₓ₃` (ₓ = ×); PDF-flattened `eθl′`, `D X ∪ S`; pseudo-formula `Σ(sprint=1 to t/2weeks)`; fragment `e^(-γ` (splitter cut); `[ε]` in a Unicode code chart.

## 10. Joseph's data idea, realized: estate LaTeX → Unicode round trip

50,348 distinct estate prose lines already carry inline `$…$` (31k in asf). `py/reverse.py` renders each span back to agent-style Unicode with seeded per-document dialect knobs (Unicode vs `_1`/`^2` digits, `<=` vs `≤`, tight vs spaced operators, `‖` vs `||`, braces vs parens on compound scripts). Gold = the original line. Lines whose gold still has Unicode math outside its spans are excluded (incomplete gold); reversals that would lose information (`\mathcal{AB}`, `\begin`, `\binom`) are skipped rather than silently degraded.

r1 (5,000 lines, full reversal): 63% exact+equiv, 7% wrong — but reading 30 wrongs showed most were *my reverse map's* artifacts (unbraced `_\text{obs}`, `\mathfrak` flattened to plain letters). Fixed (fraktur/bold → Unicode math alphanumerics, which carry the information). r2: 67.6% exact+equiv, 24.9% degraded, 3.2% over, 4.2% wrong; partial reversal (half the spans left as LaTeX, so symbol evidence exists) 81.3% / 3.1% wrong.
Degraded split: 933 of 1,198 are ASCII-only misses (bare `S`, `x`, `A → G` the authors typeset; nothing Unicode to see) — the synthetic set is harsher than real agent Unicode here, because LaTeX authors wrap every variable.
Real converter bugs it surfaced: **`ⁿ ᵀ ᵢ` are `isalpha()` and were lexed as foreign words** (so `ℝⁿ`, `Jᵀ` never parsed); spaces inside norms `‖ x - y‖`; `p‖q` inside brackets; `let a_t = …` hit my JS guard (too broad); words inside script parens `δ_(critical,k)`; set braces `{ξ_k}`; a term whose `_{…}` fails to parse leaving `t$_{macro_cost}` dangling (must abstain, never split a token).

## 11. Gates × det, and llama on B

**md-press's gates applied to det's outputs** (A+B, 540): converted 410, unchanged 40, refused 90. Of the 90 refused, 77 are exact/equivalent per gold, 11 degraded, 2 wrong; the gates pass 2 wrong + 1 over. ⇒ The gates, calibrated to LLM failure modes, refuse ~14% of det's *correct* conversions while catching half of its rare errors. Causes: content gate doesn't know `₀`→`0`, `\succeq`, `\text{word}`; prose skeleton counts `\text` words. Integration consequence: det needs its own constructive invariants, not today's gates (plan).

**llama3.2:3b on B** (same request as md-press; judged by md-press's real gates): ok 136/300 (45.3%), wrong-but-accepted 5 — new specimens that pass today's gates *including* `operands_survive`: `ρ/R` → `\rho/\rho_R` (B034), `T` → `\mathcal{T}` (B128), swallowed `?` (B133), raw combining hat left inside math `p̂_k` (B082). Worth the coordinator's attention independent of this spike.

Current det (tuned on A+B): B 261/300 (87.0%), wrong 2; A 221/240 (92.1%), wrong 2.

## 12. Launched (background)

- **Set C — final held-out** (`data/gold/C/`, brief `data/gold/C/BRIEF.md`): 450 items, blind to kind: 150 fresh triggered pieces (CP), 100 whole triggered *lines* (CL — tests feeding whole sites instead of md-press's pieces), 150 *untriggered* sites where det (as of now) proposes a change (CU — precision of scope expansion), 50 untriggered sites with `_ ^ = < >` where det proposes nothing (CN — missed ASCII math). 3 batches × 2 passes, Opus, per-agent scratch dirs. Agents: p1b0 aa0758510dfdc7366, p1b1 add99226d8ef712b5, p1b2 ae692c70b7ee677c0, p2b0 a463aadbdbaeabbab, p2b1 a2459822123f58187, p2b2 affaec165ea79a824. **I will not open C until the converter is frozen.** (CU/CN were selected using the current converter — fine for estimating that converter's precision on what it changes; the frozen version will differ slightly.)
- **Agent-written Unicode dialect** (`data/synth-agent/`, brief `BRIEF.md`): 600 estate LaTeX lines; each agent rewrites the math as it would naturally write it without LaTeX, prose byte-identical. Models: b0 Sonnet a55672dae40dedc7b, b1 Sonnet a23abcf642929cd51, b2 Opus ab9fe2b12d3910eb6, b3 Haiku a5eb54e9577cbaf0b (Haiku brief carries a suggested chunking, per the lower-order-model exception).
- (C labelers p1b0, p2b1 reported. Their final messages name specific C items; I'm deliberately *not acting* on anything C-specific until the converter is frozen and C is scored. Anything I change afterwards because of C gets logged as C-informed.)

## 13. Learned bare-letter component: measured, not recommended

`py/letters.py`: 49,317 standalone single Latin letters the det converter leaves alone, from 1,500 estate files' LaTeX lines after full reversal; label = the author had it inside `$…$` (9,213 positives). 20 features (letter identity/case, neighbours' kinds, preceding preposition/article/capitalized word, line and document symbol evidence, distance to nearest det span, …). RF 150×depth 12; grouped-by-file 5-fold CV AUC 0.984; at p≥0.9 precision 0.989 / recall 0.249; at 0.8, 0.969 / 0.477.
**Transfer to real agent-Unicode pieces (with document context from each item's source file):** B 261 → 263 ok at p≥0.9 (no new wrong); A 221 → 220 with one new wrong (A229, the RF promoted `t` in a splitter fragment det had deliberately abstained on — RF bypassed det's guards). At p≥0.7: +6 ok on B but +11 wrong across A+B (glyph surveys, udon `:x`, a quoted stimulus `X→Y→Z`, `$m$` as a unit, isotope `δ¹⁸O`). ⇒ The authors' wrapping choices on LaTeX-written lines don't teach the agent-Unicode decision well; the residual letters need context the forest doesn't see. Measured and left out of the frozen converter.

## 14. Agent-written Unicode dialect (Joseph's idea, natural form) — first read

Sonnet×2, Opus, Haiku (Haiku still running) rewrote 600 estate LaTeX lines as they'd naturally write them. det (pre-fix): Sonnet 300 lines 67% ok / 25% degraded / 7% wrong; Opus 150 64% / 26% / 6%; Haiku partial similar. Reading every wrong/over: a third are **lossy rewrites** (agent flattened `\mathcal{C}_t` to `C_t`, `\mathbf 0` to `0`, `\dot\delta` to `dδ/dt`, added an explicit `·` the gold leaves implicit) — not converter errors; the scorer needs a lenient-alphabet / implicit-product reading for this set. The rest are **real dialect gaps my own synthesis never produced**: `∑_i` / `∇_θ` (large operators taking scripts), `∑ ν·η` (large operator then a space), `η^(k)*` → double superscript `^{(k)}^\ast` (invalid TeX!), `𝟙_[m > 1]`, `r_{one-for-one}` rendered as minus signs, `Ṙ_min,i` (precomposed dotted letter; unbraced comma subscript), `δ_critical,k²`, a span starting right after a dangling `^`, and my new juxtaposition rule pulling the possessive `s` of `B6's` into `$s\alpha'$`. Exactly the brief's warning: the distribution I synthesize from is my own.

## 15. Validity as a truth source: KaTeX + MathJax over every emitted span

Installed `katex` and `mathjax-full` (scratchpad) and rendered every distinct span det emits over the estate (4,640 distinct, 27,352 occurrences): **32 invalid** (0.7%), by both renderers. Classes: double subscripts from my comma-subscript rule (`(M_t,G_t)` → `M_{t,G}_t`, `Q(s_t,a_t)`), chained snake subscripts (`σ_source_phrase`, `λ_ij_new`, `κ_W₁`), double superscripts (`η*^(k)`), `\sqrt` with no argument (radicand a parenthesized group, or a bare `√` mention), a transcript `⟨Read(...)|…⟩` absorbed whole as a bra-ket, a digit-led snake identifier. Also: KaTeX rejects `_\min` (MathJax accepts) — now emitted as `_{\min}`.
Consequence for design: a well-formedness check on every emitted span (no double scripts, required arguments present, braces balanced) is a cheap constructive invariant the converter can enforce itself in Rust, abstaining on failure.

## 16. FINAL HELD-OUT RESULT — set C, frozen v2 (py/frozen/umath_v2.py, sha1 19be3815…), first look

Labelers equivalent-or-better with each other: 416/450 (92.4%).
| kind | n | v2 ok (exact+equiv) | v2 wrong+over | v1 ok | none ok |
|---|---|---|---|---|---|
| triggered pieces (CP) | 150 | 82.7% | 5.3% (7 wrong, 1 over) | 82.7% | 14.0% |
| whole triggered lines (CL) | 100 | 83.0% | 3.0% | 81.0% | 16.0% |
| untriggered, det changed (CU) | 150 | 80.0% | 13.3% (19 wrong, 1 over) | 70.0% | 43.3% |
| untriggered, `_^=<>` but det unchanged (CN) | 50 | 100% | 0% | 100% | 100% |
"none" scores 6 wrong itself: golds that restructure flattened text (`η2p` → `\eta^2_p`) make "left as written" a structural mismatch, so ~1% of any system's wrong is that floor.
From here C is seen.

C wrong/over for v2 (35 items), read in full and sorted by cause:
- out of scope at file level (glyph tables `[ξ]` `[ℐ]`, YAML flattened by md-press's own unwrap — see below, verbatim tool output, garbled PDF CSV, scraped abstract): 6
- PDF-flattened scripts (`θl`, `eθl′`, `D X ∪ S`): 3
- project label conventions (asf writes the W₀/W₁/W₂ regime labels as Unicode prose hundreds of times and reserves `$W_2$` for Wasserstein): 4; footnote marker `E¹`: 1; product name `μTOSCA`: 1 (over)
- session-listing fields `t=5`: 4
- letter glued to an existing span the author deliberately kept upright (`H$_\kappa$`, `(P$^{\Diamond}$)`): 3; label after `Appendix` (`Appendix E`): 1
- `\(…\)` delimiters (md-press normalizes these *before* the model today, so this wouldn't arise in the pipeline): 2; markdown-escaped LaTeX export: 1
- converter errors proper: `$\hat\kappa$-is-a-$do(G)$` → `$a - do(G)$` (article in a hyphen chain), `Casella, G.` (an initial), `M_τ⁺` (sign superscript belongs to the subscript), `η²p` (APA partial eta squared): 4–5
**Incidental md-press bug (two C labelers, independently, verified by one with `md-press - | diff`):** a file whose YAML frontmatter follows an HTML comment gets the frontmatter parsed as a setext-heading paragraph and joined (CL058, `verisectorium/.../tools-are-observation-infrastructure.md`). Render-equal, so the gate can't see it.

## 17. v3 = post-C fixes (C-informed; no fresh held-out number exists for v3)
Fixing only the converter-addressable causes above, each as a general rule, never item-specific: letters glued to an existing span stay out of it; hyphen chains `is-a-` keep the article; initials (`Casella, G.`, `E. L.`); labels after label nouns (`Appendix E`, `Section B`, …) even when the letter is a known symbol; `\(…\)` normalized first, as md-press does; `⁺`/`⁻` right after a subscript belong to it.
- `⁺`/`⁻` after a subscript: the C labelers disagree *with themselves* across items (CP077 both wrote `M_{\tau^+}`, CP101 both wrote `M_\tau^+`). asf canon decides it: `M_{\tau^+}` 390×, `M_\tau^+` 0×. Kept the rule for letter indices; a digit index keeps the sign outside (`W₀⁺` → `W_0^+`). CP101 is therefore scored "wrong" against labelers who departed from canon — noted, not "fixed".

## 18. v3 frozen (py/frozen/umath_v3.py, sha1 eb5faf14…) + fresh set D

v3 = v2 + post-C fixes (§17) + canon-mined house labels. **House-label lexicon** (`py/lexicon.py`): count each Unicode token raw-in-prose vs inside `$…$` in files that write LaTeX. Over the whole estate it flags W₀ (good) but also β_0, β_exp, ϵ — lazy unconverted math, not labels: *the estate is itself the disease population, so its usage isn't a clean signal of intent.* Restricted to asf's format-gated canon (`0[1-4]-*-core/src/`) it yields exactly {W₁ (139 raw / 1 math), W₀ (24 / 0)} and nothing spurious; W₂ is correctly not flagged (Wasserstein `$W_2$` exists). So corpus statistics are a sound label detector only on curated canon. (C-informed: the labelers named the W₁ class.)
v3 on everything (all tuned or informed by A, B, C): A 221/240 (92.1%), B 264/300 (88.0%, 2 wrong), C 386/450 (85.8%, wrong+over 4.4%), agent dialect 70.6% ok / 4% wrong (lenient), math-free 72 sites changed of 2.47M, 0 invalid spans of 4,551.
**Set D** (fresh, selected with frozen v3, labels pending): 200 whole triggered lines (DL), 150 untriggered sites v3 changes (DU), 50 untriggered with `_ ^ = < >` or Unicode scripts that v3 leaves (DN). 6 Opus labelers; brief `data/gold/D/BRIEF.md` (now also bars `py/`, `rs/`). Agents: p1b0 a48ad131a1a3c102e, p1b1 a4f715f51aa8142a9, p1b2 a6268417dd22d85c5, p2b0 ae2468745bed5cfa5, p2b1 a9fab27f398057073, p2b2 aac74dc4c6c84fe21.

## 19. Properties (the brief's adversarial drill), and v4

`py/properties.py` checks, on every changed estate site, every non-ASCII-bearing math-free site, and 200,000 random adversarial strings (math glyphs × markdown constructs × traps like `café`, `p. 3`, `10x`, `H$_\kappa$`): **shape** (output = input with regions replaced by `$…$`; code spans masked), **verbatim** (code, links, URLs, wikilinks, HTML intact; existing math's content intact), **idempotence** (convert∘convert = convert), **validity** (`tex_ok` on every span), **no crash**.
v3 failed it: 829 idempotence violations on real text (a partial first pass `P$(failure) × $T` that a second pass merges — would make md-press's `--check` dirty right after a write), fuzz crashes, `\(\>85\%\)` normalized into an invalid span (my port of md-press's paren rule used a wider glyph set than md-press's), a two-char `<<` op eating into an HTML tag, `$$` inside running text re-pairing.
**v4** = v3 + public `convert` iterated to a fixed point (≤4 passes, else leave as written) + a final self-check (output must align as input-with-regions-replaced, else leave as written) + hazards checked before any rewrite + md-press's exact paren glyph set + inline `$$` and invalid existing spans as hazards.
v4 result: **real text (920,682 sites, 16,154 changed): 0 violations of any property, 0 crashes.** Fuzz: 10 of 200,000 (0.005%) remain, each with an unmatched backtick or a `$` inside a link destination — places where the checker's own masking and the converter's `protected_ranges` disagree; not resolved.
v4 scores: A+B 487/540 (90.2%); C 389/450 (86.4%, C-informed).
