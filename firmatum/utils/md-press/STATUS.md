# md-press status

*Updated 2026-10-02 (math on by default; math-pass safety rework; quieter output; the two FEEDBACK files' defects). 2026-08-06: renamed from `fmt-md`, parser-sited math pass. `.udon` guard 2026-07-29. Founding-session body 2026-07-22.*

## Math on by default, and what that required (2026-10-02)

Joseph asked for the math pass on by default and the tool a little quieter. Turning it on surfaced that the pass was not safe to run unasked, so the default flip came with a rework; the order below is the order of the evidence.

**The pass had been silently corrupting markdown structure, and every gate passed it.** Run live over sampled estate lines, `llama3.2:3b` turned `- **Narrow Lindy Effect (median)**: … α = 1` into `-Narrow Lindy Effect (median): … $\alpha = 1$` (list marker unmade, bold gone), un-nested an indented list item and dropped its trailing hard break, and converted prose arrows to `$\to$`. The gates compared multi-letter words and only refused *added* punctuation, so lost whitespace, markers, and emphasis were invisible to them. Repairs:

- **The model sees only prose.** `split_structure` removes indentation, `>` and list markers, task boxes, heading `#`s, and definition labels before the model, and the trailing hard break after it; both are reattached verbatim.
- **Alignment gate** (`edit_regions` / `edits_confined`): a proposal is kept only if every character outside its new `$…$` spans is the original's own, in order, and the replaced regions contain no protected text, no prose-role glyph, and no `**` or backtick. The earlier word and punctuation gates stay as defense in depth.
- **Inline masking** (`protected_ranges`): code spans, existing `$…$` *and* `$$…$$` (the old single-`$` toggle exposed display-math interiors), link destinations, wikilinks, autolinks / HTML tags, and bare URLs are invisible to the detector, to `\(…\)` normalization, and to the gate. This fixes FEEDBACK-08-12 §1 (`\(1\)` in a link destination → `$1$`); separately, the short-identifier clause of `\(…\)` normalization now requires a letter, so `\(1\)` in prose stays a parenthesized number.
- **Interiors of multi-line `$$` blocks** are never sent to the model (they are paragraph text to the parser and LaTeX to the reader), and `$$` blank-line insertion uses the line's container prefix (`>` inside blockquotes — an empty line would end the quote) and only acts at parse-identified prose sites.
- **Weak glyphs are judged by their operands.** `→ · × ≈ ≤ ≥ ≠ ± ≡ √` are also house punctuation. One counts as an operator only with a math-looking operand on one side and math, a number, or nothing on the other; otherwise it is prose: it doesn't trigger the model, and the line is split at it so the model never sees it (one refused piece no longer blocks the rest of the line). Math-looking operand: a single letter (primed/starred), sub/superscript on a single-symbol base (`M_t`, `c_min`, not `if_else_directive`), an isolated non-unit Greek letter (`μs`, `kΩ` are units), a strong glyph, `f(x)`-style application, or an adjacent existing math span. This fixes FEEDBACK-08-12 §2 (`·` separators → `$\cdot$`).

**Measured effect on what calls the model** (md-press's own detector at its own parse sites, over the estate's 12,281 tracked `.md` files outside `.moved*`/`_older`/`_ref`/`ruby-community`, ignore-excluded files not counted; the probe was a scratch crate linking the library): **44,647 → 8,635 would-be model calls** (5,316 → 2,034 files). Before, 66% of calls were triggered *only* by `→` and/or `·`, and samples of those were nearly all prose. After, a uniform sample of 40 remaining firings was about 36 real math; the misses are minor (`1 m → 32 m` reads the unit `m` as a variable, `η2p = .653`, a lone `π`). Joseph raised training a second random forest to gate the model; on this evidence the operand rule gets the trigger's precision to roughly nine in ten without labels, and the residual risk sits in what the model proposes, which the alignment gate now covers. A forest remains the right tool if the operand rule's misses turn out to matter.

**Cost, and the cache.** The model re-emits everything it is shown, and unwrapped asf paragraphs run past 1,000 characters, so a call on a whole paragraph took seconds. Pieces are now also split at sentence ends (`split_sentences`: `. `/`? `/`! ` before a capital or opening mark, outside protected ranges) and only the sentences holding math are sent. Proposals are cached on disk (`$XDG_CACHE_HOME` or `~/.cache`, then `md-press/math/`), keyed by model name, prompt-file contents, and exact text (FNV-1a 128, stable across Rust versions). Only raw proposals are stored; every gate re-runs on every use, so a gate change takes effect at once. Measured on asf's math-heaviest spike (63 KB, 66 firing sites): 72 s cold, with ollama shared with another run; 0.03 s from cache, byte-identical output, idempotent. `MD_PRESS_CACHE=off` disables it, and deleting the directory is always safe. This also makes `--check` with math reproducible on any machine that has seen the text before.

**Reading real output, both directions.** I went through every accepted edit and every refusal on two math-dense files: asf's `spikes/spike-passivity-composition.md`, and `AISI-responses`' `asf-notes/31-asf-orientation-notes.md` as it was before its `--math` run.
- *Wrong edits that had passed:* a prose `+` wrapped as `$+$` ("PID + non-positive-real plant") while promoting the β in the same sentence, now refused because a replaced region must itself read as math (a trigger, LaTeX, an existing span, or math-looking operands, numbers, and operators with at least one operand or relation). And `ν_M >> ν_Σ >> ν_O` became `$\nu_M \gt \nu_\Sigma \gt \nu_O$`, weakening "much greater"; the count of `≫`/`≪` relations (`>>`, `≫`, `\gg`) must now be preserved, and postprocess writes `>>` as `\gg`.
- *Refusals that were the gate's fault:* about a third of n31's 26. The consistency table didn't accept `\epsilon` for `ε` (only `\varepsilon`), treated `\frac`, `\prime`, `\hat`/`\bar`, `\max`/`\min`/`\log` and the like as invented content, and the prose skeleton counted subscript words (`ρ_base`, `α_min`) as prose that went missing. Fixed: n31 is down to 19 refusals, and its accepted edits read correctly (`ε*` → `$\epsilon^\ast$`, `ε_max` → `$\epsilon_{\max}$`, `π_cont` → `$\pi_{\text{cont}}$`).
- *A pre-existing postprocess bug:* the glued-command fixer (`\Vertw` → `\Vert w`) split real longer commands, `\infty` → `\in fty` (likewise `\int`, `\top`, `\cdots`, `\leqslant`, `\gtrsim`). The gates had been catching the damage as "invented content", at the cost of refusals. It now splits only whole command names that aren't known commands.
- The model request now stops at the first newline and caps its length near the input's. Only the first line was ever used.

FEEDBACK-08-12 §4 is reproduced and fixed on that same n31 file from git: pressing the pre-math version twice now changes nothing, and `--check` after a write exits 0. The original mechanism was compound. The math pass stripped classifier-written `  ` markers, so kept breaks went soft again; the second run's classifier then saw promoted text (`$\rho$` for `ρ`) beside one of them and joined it, which is the classifier-input effect Joseph named. A surviving marker is a deterministic keep, so the classifier is never consulted again.

**End-to-end over all of asf** (a scratch copy of its 2,682 tracked `.md` files; the live tree was not touched). First, a cold run on the final build: 33 minutes, with ollama serialized. An earlier build's pass 2 then exposed two more math-stage bugs, both pre-existing but previously confined to `--math`. An all-whitespace table cell grew two spaces per run (its leading and trailing padding were the same bytes, added twice). And the display-math blank-line fixer rebuilt text with `lines()` + `join`, dropping one trailing blank line per run and flattening CRLF to LF. Both are fixed: the math stage now carries each line's own ending through, and empty cells are left alone. On the fixed build, from cache: pass 1 in 2 s; **pass 2 changed 0 files**; `--check` afterwards exits 0 and prints nothing.

A structural audit compared each file's `--no-math` output with the full output, so it saw only what the math stage did. 480 files were touched: 1,633 line edits and 874 blank lines beside `$$`. Every edit lies inside a new `$…$` span, with no change to list/quote prefixes, table `|` counts, or hard-break markers. The audit flagged 2 suspects; both, on inspection, were the `$$` blank-line rule. 1,267 passages in 359 files were left as written because no proposal passed the gates. That is a high refusal rate (roughly 4 refused passages for every 5 edited lines) and it is where the model's usefulness is now limited: each refusal, listed by `-v` with its proposal, is a candidate example for `prompts/unicode-math.txt`.

**Model comparison: llama3.2:3b vs Muse Glimmer 30B** (Joseph's request, 2026-10-02). The sample was 240 of the 8,027 distinct pieces md-press sends to a model estate-wide: 120 from asf, 120 from everywhere else, seeded at random and collected through the fake-model seam, so they are exactly what the pipeline sends. Both models got the identical few-shot prompt as a raw completion (temperature 0, stop at newline, the same length cap). Muse ran under llama.cpp b10358 with its DFlash drafter (`~/models/muse-glimmer/notes.md`). Every proposal went through md-press's real gates, and every accepted conversion was then read by hand.

| | converted | unchanged | refused | wrong but accepted | median s/call |
|---|---|---|---|---|---|
| llama3.2:3b | 88 | 61 | 91 | 5 | 0.58 |
| Muse Glimmer 30B | 138 | 64 | 38 | 1 (minor) | 3.34 |

- llama3.2's accepted errors: `ρ/R` → `\rho/\rho`, `𝒜` → `\mathcal{S}`, `||δ||` → `\delta` (norm dropped), and two full stops swallowed into a span. It also regurgitates its few-shot examples ("The gain η exceeds ρ under load.", "Here is the converted line:"), which the gates refuse.
- Muse's one slip is a misplaced brace (`\delta_{\mathrm{sat}^{(1)}}`). It got cases right that llama could not: `>>` → `\gg`, `η2p` read as partial eta-squared `\eta^2_p`, `X < n_future × Y`, a full sea-level equation. Where it declined, it mostly returned the line unchanged rather than mangling it.
- Chat mode with reasoning was also correct on a 3-line smoke test, but took 40–55 s per line (~2–3K characters of hidden reasoning), and this build ignored every per-request way to turn thinking off. Raw completion is the usable mode.
- Cost: about 5.8× llama per call. A cold estate press would be ~7 hours instead of ~1.5; the cache makes repeats free. It needs ~20 GB resident and, today, the T7 drive, and md-press would need a llama.cpp (or ollama MLX `muse-glimmer:30b-mlx`, not pulled) backend.

**A gate that came out of reading those errors** (`operands_survive`). Every letter-run, digit-run, Greek letter, script capital, norm or absolute-value bar, and trailing full stop or comma in the replaced text must reappear in the span that replaced it. The content-consistency gate only checked that a span invented nothing *anywhere in the line*, so swapping one variable for another present elsewhere passed. Re-judging the stored proposals: it refuses exactly llama's five wrong conversions and none of Muse's 138.

**The run had a panic waiting in it**: `protected_ranges` skipped an escaped character as two bytes, landing mid-character on `\×` (present in the estate). Found by the estate-wide probe, fixed before any write.

**Unavailable model is one state, not a flood.** `Model` carries a circuit breaker: the first server- or model-level failure (ollama not running, model not pulled, an error object instead of a completion) marks it down for the run, and the run continues unwrap-only with one end-of-run line naming the cause and the fix (`try \`ollama pull …\``). This was found live: Joseph keeps the large model files on an external drive (`T7 Archive`), symlinked from `~/.ollama/models/blobs`. With the drive unmounted the links dangle, `ollama list` still shows every model, and generation fails "not found". Every would-be line printed its own `model error`. On 2026-10-02 md-press's model, `llama3.2:3b` (~2 GB), was re-pulled, which replaced its dangling link with a real file on the internal drive, so md-press no longer depends on the drive being mounted. The drive's copy of that blob is now unreferenced.

**CLI.** Math is on unless `--no-math` or `MD_PRESS_MATH=off|0|no|false`; `MD_PRESS_MATH=MODEL` or `--math=MODEL` picks the model; flags beat the environment (caller-stack tuning — the tree never sets it). `--check` includes math, since a check that ignored it would pass files the next write then changes. `--no-math` is the deterministic check. `--check -` is now a real check: nothing on stdout, the render gate runs, and the exit code answers (FEEDBACK-08-12 §5).

**Quieter.** An asf-wide press (2,682 files, `--no-math`) wrote 2,801 stderr lines before and 2,121 after. Per file, only the classifier's uncertain breaks are listed (one short header plus one `joined`/`kept p=…` line each; the how-to-undo legend prints once per run; `-q` hides them). These stay on by default: FEEDBACK-08-12 credits them with surfacing real damage, and sampling the current asf set confirms they carry signal (the p≈0.40–0.47 "joined" plateau includes enumerations like `(i) **Model D:** … \n (ii) **Model S:** …` that look wrongly joined). Everything else is one end-of-run line: files skipped by `.md-pressignore` or as `.udon` (named individually when there are three or fewer), math passages left as written because a proposal failed a gate (not printed in `--check`, where it changes nothing), and a missing model. `-v` lists what the summaries count.

**Unwrap-stage fixes in the same pass.** The paragraph-final trim was Unicode-aware and deleted a U+00A0 line after a hard break (FEEDBACK-08-12 §3, bisected on the original `AISI-responses/aisi.md` from git: the "whitespace-only" line was `    \u{a0}`; trims are ASCII-only now). Display math keeps its own lines: a break before a `$$`-led line, after a whole-line `$$…$$`, and inside a multi-line `$$` block is preserved deterministically, so FEEDBACK-08-22 §1's p≈0.47 joins no longer happen. With the math pass, the `$$` blank-line rule then gives those blocks FORMAT's own-paragraph shape. Lines that look like table rows are preserved like definition-looking lines (asf's `FINDINGS.md` has a table whose header is glued after `**Related Work:**`, so CommonMark sees a paragraph; joining its rows rendered identically and buried the table).

**Frontmatter after a leading HTML comment** (found by the unicode-math spike, `spikes/spike-unicode-math-2026-10-02/`; reproduced and fixed the same day). CommonMark only recognizes byte-0 frontmatter. A provenance comment above it, as in verisectorium's 30 mining copies, made the YAML parse as prose. Joining then folded each key into the previous key's `# …` comment (`support-kind:` vanished into `register:`'s comment), and the render check could not see it. A `---` block opening with a `key:` line, preceded only by blank lines and HTML comments, is now opaque to joining and to math. All 30 files keep their frontmatter byte-identical.

**Single-`*` emphasis** (found by the spike's independent verification, `spikes/spike-unicode-math-2026-10-02/de-novo-feedback-1.md`): the alignment gate checked replaced regions for `**` only, so a proposal that deleted one `*` of an emphasis pair passed; two of llama3.2's accepted outputs on the spike's held-out set did exactly that. `operands_survive` now requires every `*` in a replaced region to survive as `*` or `\ast`. Re-judging the 240-piece comparison changes no earlier correct conversion.

Pinned by `tests/regressions.rs` (21 offline tests, including a fake-model seam replaying the observed bad proposals).

**Still open:** FEEDBACK-08-22 §2 — the GitHub render-compat checks `bin/lint-md` ran that md-press does not (raw `<`/`>`/`|`/`*` inside existing math, emphasis-vulnerable `_` across spans, LaTeX commands outside math, `#` in math; `$$` blank lines are now covered whenever the math pass runs, `--check` included). Joseph's ruling makes these md-press bugs; they want a check-vs-fix design (several are safe auto-fixes) rather than a bolt-on. Also open: in dense files a noticeable share of math passages have every proposal refused (14 of 66 sites in the spike above). Each is a ready-made example for `prompts/unicode-math.txt`; `-v` lists them.

## Renamed fmt-md → md-press (2026-08-06)

Joseph's call, same session as the math-pass rework: still descriptive, pronounceable, memorable ("press" carries both the ironing and printing-press senses). Compat kept in two places: `.fmt-mdignore` files are honored equally and indefinitely alongside `.md-pressignore` (pinned by test — an exclusion must never expire because the tool changed its name), and `~/.cargo/bin/fmt-md` is a shim script that prints a deprecation notice to stderr and execs `md-press "$@"`, so stale instructions in older docs/memories keep working while telling their reader to update. Env vars renamed `FMT_MD_*` → `MD_PRESS_*`.

## The `.udon` guard (2026-07-29)

`.udon` files are skipped by default, even when named explicitly, overridable with `--allow-udon`. This closes the exposure named in `UDON-ASSESSMENT-2026-07-29.md`, which had answered "extend fmt-md to accept `.udon`?" with a reasoned no-go but left the tool itself unchanged — it checked no extension at all, so `fmt-md $(find . -name '*.udon')` processed them exactly as the assessment documented, and udon's `.fmt-mdignore` excluded only three specific paths rather than `*.udon` as a class.

Implemented as an extension-keyed guard in `exclude.rs` (`FOREIGN_EXTENSIONS` / `foreign_language`) rather than an ignore-file line, so it travels with the file into trees that have no `.fmt-mdignore` and cannot be forgotten when a directory is added. Two design points worth keeping:

- **It sits in front of the render-equality gate, not behind it.** The gate compares CommonMark renders, and a corrupted UDON attribute line renders as unremarkable markdown text. The tool's central safety claim has no UDON referent to be true or false about, so no amount of gate-tightening could have covered this. Pinned by three tests in `tests/exclude.rs` that assert *both* halves — the damage happens, and the render check stays silent — in the same shape as the transcript test, so neither half can be "fixed" by accident.
- **`--force` deliberately does not override it.** `--force` means "I know about the verbatim exclusions"; folding `.udon` under it would mean a `--force` run aimed at transcripts also disables language protection, which is the accident shape in miniature. It needs its own opt-in.

Known hole, documented rather than papered over: the guard reads the filename, so `fmt-md - < f.udon` is unprotected. Stdin mode writes to stdout, so in-place corruption of a tree — the accident actually being defended against — is not reachable that way.

Still open from the assessment's recommendations: nothing in `udon-core` yet provides UDON-aware reflow, which remains the right home for the capability if it is ever wanted.

## Where it stands

Phases 0–2 of PLAN.md are substantively done: the crate exists, builds, and the **unwrap engine matches human ground truth everywhere the difference isn't (a) a rule scheduled for Phase 3 or (b) the historical "after" being itself defective.** `cargo test` is the proof surface:

- `invariants_hold_on_all_fixtures` — idempotence + render-equality (comrak HTML fingerprint, whitespace-collapsed outside `<pre>`) over the full fixture corpus (asf history pairs + udon-needs + vivarium samples).
- `random_wrap_recovery` — Joseph's named property: deterministic random-wrap injection into canonical files recovers the original exactly.
- `ground_truth_score_report` — **11/18 exact byte matches** against the human-approved reflow pairs from asf history. The 7 non-exact, each examined and attributed:

| Pair | Δlines | Attribution |
|---|---|---|
| pearl-causal-hierarchy | 1 | `\ltt` glued command *pre-existing in before*; human fixed by hand → Phase 3 rule (glued `\lt`/`\gt`/`\Vert` + letter) |
| sector-condition-stability | 1 | same (`\Vertw`) |
| persistence-condition | 1 | same (`\Vertw`) |
| sector-condition-derivation | 4 | the March pass *merged a `[^lure1957]` footnote definition into the previous one* (historical casualty; that footnote has rendered as plain text since). fmt-md preserves the definition → we are more correct than the "after" |
| composition-closure | 39 | human promoted standalone `$…$` lines to `$$` display blocks → Phase 3 promotion rule; fmt-md already preserves their lines |
| spike-routing | 325 | "after" is raw `lint-md --fix` output with its known punctuation-ended residuals; fmt-md joins them (deliberately better) |
| audit-routing-instructions | 376 | same |

## Engine shape (what's implemented)

Deterministic join-all per the ratified policy — no statistics, no flags in the join path. All decisions are grammatical/structural:

- comrak parse (tables, footnotes, math-dollars, wikilinks, frontmatter, tasklist, strikethrough); multi-line `Paragraph` nodes are the only join targets; everything else is emitted byte-identically.
- Container-aware continuation stripping (blockquote depth from AST ancestry; list/footnote indentation).
- Preserved breaks, each a crisp category: CommonMark hard breaks (trailing `  `/`\`); definition-looking continuations (`[^x]:`/`[x]:` — CommonMark parses them as lazy continuation but the author meant a definition, R15e); standalone eq-tags (`*[Definition (…)]*`); whole-line single-span math; equation-after-colon (line ending `:` announces the `$…`-led line that follows — the house pseudo-display idiom, recovered from the March pairs).
- Footnote-definition hoisting in comrak's AST handled (spans sorted; overlaps dropped toward not-joining).
- The binary refuses to write any file whose result would change the rendered document (built-in render-equality gate, exit 2 + report).

## Math pass rework — parser-sited (2026-08-06)

The math pass no longer scans raw output lines with its own fence toggle; it promotes only at **prose sites the unwrap parse identified** (`MathSite` per output line in `Classified`): whole paragraph/heading lines, or table cells individually (comrak sourcepos columns are byte offsets; verified by test). Code blocks, frontmatter, and HTML have no sites and are structurally out of reach — the ruling that motivated this (Joseph, 2026-08-06, on `asf-agent-scopes.md`): hand the model parser-delimited prose pieces, never markdown structure, and no second parse. Cells keep their padding (promotion runs on the trimmed interior) and a proposal may not change the line's `|` count.

Same session, three gate/coverage fixes, each pinned by a test in `tests/math.rs`:

- **Detector/consistency-map sync.** `⊃ ⊂ ⊆ ⊇ ∪ ∩ ∉ ∅ ¬` fired the detector but their LaTeX commands were missing from the consistency map, so a *correct* proposal could never pass. Map filled in; Greek table completed (κ ξ ζ χ ψ ι υ + capitals); script capitals (`𝒪` → `\mathcal{O}`) token-check via `SCRIPT_LETTERS`.
- **Deterministic `\(…\)` → `$…$` normalization** (`normalize_paren_math`), applied before the model when the interior indicates math (LaTeX command, math glyph/Greek, `_`/`^`, or a short space-free identifier — `\(see above\)` stays prose). On a flagged line the deterministic part survives (`MathOutcome::Flagged` carries it); stderr says so.
- **One-directional punctuation gate** in `preserves_prose` — the previously documented tolerance bit for real: the model turned `¬agency` into `$\lnot$-agency`, invisible to word-token comparison. A proposal may lose punctuation into a span but may not mint any outside one.

One prompt example pair added (set operators — the model had been choosing `\succ` for `⊃`).

## Math pass (model-assisted, landed 2026-07-22 evening)

`--math[=MODEL]` (default `llama3.2:3b` via local ollama API, temp 0): Unicode/bare-math promotion with the division of labor the probe suggested — the model only judges *expression boundaries*; everything else is deterministic.

- **Detector** (`math::needs_math_pass`): glyph/Greek scan outside code/math spans — most lines never touch the model.
- **Prompt** (`prompts/unicode-math.md`): few-shot Input/Output pairs — a long rule-list made the 3B model *worse* (it wrapped whole lines in math); examples fixed it. **Edge cases entrain by appending example pairs** to this file (no rebuild — disk copy overrides the compiled default).
- **Post-processing** (`math::postprocess`): house rules applied inside proposed spans — `\lt`/`\gt`, `\ast`, glued-command spacing (`\Vertw` → `\Vert w`), `$`-interior space trim. Plus deterministic `$$` blank-line normalization (R19, `fix_display_math_blanks`).
- **Three gates, reject-to-flag:** balanced `$`; prose-preservation (multi-letter word tokens outside spans unchanged); **math-content consistency** — every LaTeX command must map back to a glyph in the original and every span token must exist in the original. That last gate exists because the model *did* hallucinate (`\to M_{t+1}` copied from a few-shot example into a line that had neither) — caught, gated, and the offending example de-collided (its variables renamed away from corpus vocabulary).
- Failure mode is always "line flagged on stderr, byte-identical output" — and each flagged line is a candidate new example pair for the prompt file: the accumulation loop closes itself.
- Known tolerance: the prose gate compares word tokens, so pure punctuation drift (an added comma) could pass unseen; instructions tell the model not to, but a stricter punctuation-aware gate is future work.

Live verification: `FMT_MD_OLLAMA=1 cargo test live_model` (offline suite never touches the model; 9/9 tests green).

**The two stages carry different guarantees, and the docs must keep saying so** (Joseph caught this being blurred, 2026-07-22): unwrapping is gated on whole-file *render-equality*; the math pass **deliberately changes rendering** — that is its purpose — so that gate cannot and does not apply to it. Its substitute is the three per-line checks above. Any future stage needs its own explicit answer to "what, exactly, is it not allowed to change?" — a blanket claim would be false the moment a stage improves rendering on purpose.

## Install and docs

[`README.md`](README.md) carries build/usage/scope; `fmt-md --help` carries the same in short form. Install is plain `cargo install --path .` — no bespoke script. A symlink-based installer was written and then deleted the same day: `~/.cargo/bin` had simply been missing from `~/.zshrc` (which had also left earlier `cargo install`s — `comrak`, `mdslw` — unreachable); adding that one line beat maintaining a script (Joseph's call, 2026-07-22).

Dogfooding notes, both of which found real defects worth keeping in mind:

- Running the README's own instructions found `--help` unimplemented (args were treated as filenames). Fixed, plus unknown-flag rejection.
- Running fmt-md on its own docs found (a) that `prompts/unicode-math` is *data, not prose* — its per-line records would have been joined, correctly per CommonMark and disastrously per meaning, so it is now `.txt`; and (b) a live R15e misparse in `PLAN.md`, where a wrapped line beginning `+ dprint outputs` had been parsed as a nested list item since it was written. fmt-md preserved the existing (mis)parse rather than compounding it — the designed behavior — and the source was reworded by hand. Both are the argument for exclusions (R10) and against ever running this recursively without them.

## First live run — udon `v2/` (2026-07-22)

294 files reformatted across five commits (`.archived` 137, spec suite 16, v2 ledgers 3, udon-needs 141, one dotfile), 256 protected by exclusion, zero rendering differences under an independent pandoc oracle. What it taught:

**Exclusions (R10) landed, and they are not optional.** `.fmt-mdignore`, gitignore syntax, honoured from any ancestor directory *even for files named explicitly on the command line* — because the realistic accident is an agent running `fmt-md $(find . -name '*.md')`. `--force` overrides. This exists because the first `.archived` pass reformatted 20 raw AI session transcripts, joining pasted shell scripts into single lines. **Every render check passed, correctly**: the rendered document genuinely does not change. Verbatim material can only be protected by declaration — the damage is to the file's purpose, and purpose is not a rendering property. Pinned by `tests/exclude.rs`, whose last test asserts *both* halves (the joiner acts, the render check stays silent) so neither can be "fixed" by accident later.

**A tempting rule was implemented and reverted the same day:** making `$…$` and `` ` `` spans opaque to joining, so accidental math in transcripts would be safe. `random_wrap_recovery` failed immediately, which was correct — a column-wrapper splits `$\mathcal` / `M_{adm}$` all the time, and repairing exactly that is the point of the tool. Joining inside such spans is render-neutral (LaTeX ignores whitespace in math mode; CommonMark converts line endings in code spans to spaces). The reasoning is preserved as a comment in `lib.rs` where the rule would go, because the instinct to add it will recur.

**Two verification lessons.** An independent oracle is worth the cost — pandoc found what comrak-based self-checking structurally could not, even though the finding turned out to be over-strict (raw pandoc compares math content as literal text; no reader could see the difference). And a structural census is worth running on spec-like corpora, where `|` may be *language syntax* rather than a table — but write it carefully: the first attempt used `grep "^\|"`, in which `\|` is regex alternation, so it silently matched every line and "found" damage that did not exist.

**Surfaced, not silently fixed:** `current-0.9.1-spec/CORE.md`'s header block (title / `**Status:**` / `**Companions:**`) carried no hard breaks and was therefore already rendering as one run-on paragraph; it now reads that way in source. If those lines are meant to render separately they need trailing double-spaces, which fmt-md preserves once present.

## The break classifier (model/, 2026-07-22 evening)

A trained wrap-vs-phb classifier now lives at `model/model.json` (committed: 12 features, 150 depth-capped trees, 556 KB; training corpus and copies stay gitignored/regenerable via `model/extract` + `model/features.py`). Honest numbers: grouped-by-file CV AUC 0.9975 / P 0.960 / R 0.818 on the labeled set; hand-verified organic precision ≈60% at the 0.5 threshold rising sharply above ~0.75 (errors concentrate at low probability); 11/12 on the deployment-gold set with the twelfth hand-adjudicated as *genuinely ambiguous* (a margin-explained break in a `·`-joined label run — the model's 0.497 was the honest answer). Full provenance: `model/README.md`, the Opus pipeline audit at `model/AUDIT-pipeline-opus.md`, and the audit fix-list (organic negatives into training, per-population balance, paragraph-final label recovery, calibration) — items 2–5 still open.

**Ratified integration design (Joseph, 2026-07-22)** — the unwrap stage always extracts features and consults the model per break; behavior by probability band (bands are config, seeded 0.5/0.85, to be calibrated against organic precision):

- **p ≥ 0.85** — treat as phb: keep the break and *write real `"  \n"` hard-break markers*, silently. fmt-md becomes the one writer of the markers nobody remembers.
- **0.5 ≤ p < 0.85** — keep + mark as above, and note on stderr: *"Wasn't sure about the following, but kept the line break (now marked). Manually concatenate if that was wrong:"* + the lines.
- **p < 0.5** — join (the default action), and for the upper part of the band note on stderr: *"Wasn't sure about the following and joined the lines. Manually re-separate and append `  ` (two spaces) at the break to make the separation permanent:"* + the lines.

The philosophy: act reasonably on every break, never mint a triage pile — stderr is courtesy, not homework. **Per-stage guarantee (third entry in the standing rule):** marker insertion deliberately changes rendering (a bare newline becomes `<br>`), so this stage sits outside the unwrap render-equality gate, like `--math`; its guarantee is that it only ever *adds* the two-space marker at an existing break or joins at a soft break — never touches content bytes. Rust port of the 12-feature extractor + tree eval is the implementation step (pure arithmetic + one JSON load; no ML runtime needed).

## Reproducibility acceptance test (2026-07-22, ratified by demonstration)

A fresh fmt-md run over udon's pre-conversion tree (`cc389f9`, in a worktree, current `.fmt-mdignore` supplied) converges with the repo's HEAD **byte-for-byte except one file**: `CORE.md`'s header block, whose `"  "` hard-break markers were added by hand. That is the exact operation the classifier integration automates (a ≥0.85-band label stack), so the acceptance test for the Rust port is: **the same worktree experiment converges with HEAD, no hands, with exactly five improvements over HEAD** — verified 2026-07-22 by running the committed model over the pre-conversion files: the four `.reviews` header stacks band at p 0.91–1.00 (keep + mark; HEAD has them joined because the afternoon run predated the classifier), and `CORE.md`'s header likewise. Known deliberate non-target: `DEEPENING-CYCLES.md`'s wrapped metadata block — the model correctly joins its mid-value wraps but also joins the one label-boundary break inside the wrapped block at p 0.07 (the within-block mixed case the Opus audit flagged as thin in training; audit fix-list item 2). Small-file `file_stats` fallback is a port requirement (crashes otherwise — found in this test).

## Not yet (Phase 3+, per PLAN.md)

Math rule registry (lint-md's R22–R31 ports incl. the three glued-command / display-promotion rules the scorecard motivates — FEEDBACK-08-22 §2 inventories what asf misses), blank-line normalization beyond display math, a recursive mode, fixture-regeneration harness (R35), vivarium's verbatim-quoting files as fixtures with expected outputs. (Unicode-math promotion and frozen-region exclusions, once on this list, exist — see above. Live trees: udon `v2/` 2026-07-22, `AISI-responses` 2026-08-12 with `--math`, and asf's commit gate since 2026-08-22.)
