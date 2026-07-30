# fmt-md status

*Updated 2026-07-29 (the `.udon` guard); body otherwise as of 2026-07-22, end of founding session.*

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

Math rule registry (lint-md's R22–R31 ports incl. the three glued-command / display-promotion rules the scorecard motivates), Unicode-math promotion (flag-first), blank-line normalization, config discovery + frozen-region exclusions (R10 — required before any repo-wide run), fixture-regeneration harness (R35), vivarium's verbatim-quoting files as fixtures with expected outputs. **Nothing has been run against any live tree**, per the standing ruling in PLAN.md; adoption happens when the tool earns it and Joseph points it somewhere.
