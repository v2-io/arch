# Break classification — explication-phase feature inventory

*2026-07-22. Candidate features for classifying a newline as `wrap` (mechanical column-wrap) vs `phb` (pseudo-hard-break: author intended a visible line ending). Explication-phase discipline per Joseph: enumerate maximally, include confounded/correlated/categorical features freely — a random forest infers jointness and ignores non-contributors, so selection happens *after* measurement, not before. Nothing here is normalized; types are mixed deliberately.*

*Every feature is computed **marker-blind**: trailing hard-break markers (`"  "`, `\`) are stripped before feature computation, because on corpus-positive examples the marker IS the label — see §Leakage guards.*

Notation: line **A** = the line before the break; **B** = the line after; the *break column* `a` = visual width of A (tabs expanded, wide chars counted); `b` = width of B's first whitespace-delimited token.

**Scope (ruled 2026-07-22).** The classification target is soft breaks inside **paragraph blocks only** — including paragraphs living inside list items, blockquotes, and footnote definitions (C1 carries the container context; wrapped list/quote continuations were the incumbent's biggest blind spot, so container paragraphs stay in-scope). Breaks in non-paragraph constructs (headings, tables, fences, HTML blocks) are out of scope by construction — they are line-anchored syntax, never wrap-vs-phb questions, and fmt-md's joiner never touches them anyway.

**Parse-derived, not hand-rolled.** Features that concern inline constructs are computed from the CommonMark parse, not regex/parity counters: comrak with `sourcepos` gives every inline node — `Code`, `Math`, links, wikilinks, emphasis — a start/end line+column span (confirmed directly; fmt-md's reverted verbatim-opacity rule read exactly those spans). Parser-derived spans handle escaping, nesting, and multi-line inlines that parity counting gets wrong.

## A. Break-local

| id | name | type | definition |
|---|---|---|---|
| A1 | `a_width` | num | visual width of A (marker-stripped) |
| A2 | `b_first_word_width` | num | width of B's first token |
| A3 | `rejoin_width` | num | `a + 1 + b` — the width the line would have needed to avoid this break; the classic fit-failure geometry |
| A4 | `a_last_char_class` | cat | {sentence-punct, colon, semicolon, comma, em-dash, close-paren, close-bracket, quote, word, digit, other} |
| A5 | `a_last_word_closed_class` | cat | A's final word ∈ closed-class lists: {preposition, conjunction, article, auxiliary, pronoun, none} — function words almost never end intended lines |
| A6 | `a_complete_sentence` | bool | A ends at a sentence boundary at paren-depth 0, abbreviation list applied (`cf.`, `e.g.`, `pp.`, initials, year-close) |
| A7 | `a_complete_clause` | bool | same but any clause punctuation |
| A8 | `paren_depth_at_break` | num | running `(`-depth from block start; >0 is near-conclusive wrap evidence (143 instances in vivarium). Parens are not markdown syntax, so this one stays text-computed |
| A9 | `break_inside_inline_span` | cat | from the parse (comrak sourcepos on inline nodes): break falls inside {none, code-span, math-span, link/wikilink-target, emphasis-run} — replaces hand-rolled parity counters |
| A10 | `quote_parity_open` | bool | inside an unclosed `"` at the break (prose quotes aren't parse nodes; text-computed) |
| A13 | `b_first_char_class` | cat | {lower, upper, digit, bold-marker, backtick, open-paren, open-bracket, quote, dash, other} |
| A14 | `b_starts_label` | bool | B matches `^\*\*[^*]+:\*\*` (the `**Label:**` template) |
| A15 | `b_first_word_closed_class` | cat | B's first word in closed-class lists (lowercase continuation signal) |
| A16 | `b_lazy_structural_lookalike` | bool | B's first token would parse as a list marker / blockquote at line start (`\d+[.)] `, `- `, `> `) — the R15e misparse zone |
| A17 | `cap_after_nonterminal` | bool | B starts uppercase while A did not end a sentence |
| A18 | `lower_after_terminal` | bool | B starts lowercase after A ends `.` (abbreviation suspicion) |
| A19 | `b_indent_delta` | num | B's leading-whitespace width minus the container's content column (0 = clean continuation; >0 = nested-construct suspicion) |
| A20 | `joined_sentence_len_z` | num | length of the sentence spanning the break after joining, z-scored against the file's sentence-length distribution (implausibly long ⇒ phb) |

## B. Local context (±1 line)

| id | name | type | definition |
|---|---|---|---|
| B1 | `a_width_minus_prev` | num | A's width minus the previous line's width (consistency of right edge) |
| B2 | `template_sim_prev` | num | line-start pattern similarity of A vs previous line (shared prefix class: label/list/plain) |
| B3 | `template_sim_next` | num | same, B vs its next line |
| B4 | `prev_break_class_same` | cat | naive per-break classification of the *previous* break in the block (chains: wraps come in runs) |

## C. Block-scope (the CommonMark block containing the break)

| id | name | type | definition |
|---|---|---|---|
| C1 | `block_type` | cat | {paragraph, list-item-para, blockquote-para, footnote-def} |
| C2 | `block_n_lines` | num | physical lines in block |
| C3 | `right_edge_var` | num | variance of line widths excluding the last line (tight edge = wrap signature) |
| C4 | `last_line_short_ratio` | num | last-line width / mean of the others (wrap signature: short last line) |
| C5 | `frac_sentence_end` | num | fraction of block's lines ending at sentence boundary |
| C6 | `all_lines_boundary` | bool | uniformity — every line ends at sentence/clause boundary (the asf chunk-per-line signature is uniform, not partial) |
| C7 | `template_agreement` | num | fraction of lines matching the block's modal line-start pattern (`**X:**` each / `- ` each / sentence each) |
| C8 | `n_interior_labels` | num | count of `**Label:**` markers that would sit interior to the joined line (today's defect detector; ≥2 fired on all five real cases) |
| C9 | `block_has_hard_break` | bool | block already uses explicit hard breaks somewhere (mixed convention inside one block is strong evidence) |
| C10 | `offset_from_heading` | num | block's first line minus nearest preceding heading line (metadata blocks live at ~+2) |
| C11 | `is_first_block_after_title` | bool | the metadata-position special case |
| C12 | `container_depth` | num | quote nesting × list nesting |
| C13 | `n_wikilinks`, `n_math_spans`, `n_code_spans` | num | foreign-region density in block |
| C14 | `block_width_range` | num | max − min line width |
| C15 | `block_type_above`, `block_type_below` | cat | type of the adjacent blocks: {heading, thematic-break, paragraph, list, quote, fence, table, none/eof} — metadata blocks characteristically sit heading-above / hr-or-paragraph-below |
| C16 | `adjacent_template_agreement` | num | C7 computed on the neighboring paragraph blocks — a phb block amid uniformly-wrapped neighbors (or vice versa) is informative both directions |

## D. File-scope

| id | name | type | definition |
|---|---|---|---|
| D1 | `p50, p75, p90, p95` | num | percentiles of nonzero prose line widths (p90 ≈ the wrap column when one exists; supplied raw, jointness with A3 learned not asserted) |
| D2 | `mode_width` | num | mode of the 5-char-binned width histogram |
| D3 | `frac_60_90` | num | fraction of prose lines with width ∈ [60, 90] |
| D4 | `width_var` | num | overall prose line-width variance (flat distribution = not a column-wrapped file; flips interpretation of every break) |
| D5 | `hard_break_count`, `hard_break_density` | num | file's explicit hard-break usage (convention awareness — only 57/391 udon files use any) |
| D6 | `file_kind` | cat | from basename patterns: {README, INDEX, CLAUDE/AGENTS, review-\*, spec, segment, transcript, notes, other} |
| D7 | `path_tokens` | cat | membership flags: `.archived`, `.reviews`, `src/`, `reports/`, dotfile. **⚠ Batch-effect risk (Joseph):** positives and negatives will cluster in different paths, so this (and D10) can teach the model *corpus identity* instead of break physics. Kept for explication, but earmarked as stratification/evaluation variables rather than presumptive model features — see Leakage guards §5 |
| D8 | `has_frontmatter` | bool | |
| D9 | `n_blocks_multiline_frac` | num | fraction of blocks that are multi-line (overall wrap burden) |
| D10 | `corpus_id` | cat | {udon, asf, vivarium, generated-\<model\>-\<date\>, …} |

## E. Provenance (git — cheap and possibly gold)

| id | name | type | definition |
|---|---|---|---|
| E1 | `line_commit_kind` | cat | from `git blame`: the commit that introduced line A — {bulk-import, lint/reflow (message-matched), authored, unknown} |
| E2 | `line_age_bucket` | cat | blame date bucket (authoring era ⇒ model-default era) |
| E3 | `neighbors_same_commit` | bool | A and B introduced by the same commit (pasted-together vs written-together) |
| E4 | `file_has_reflow_history` | bool | file appears in a known reflow commit (already-cleaned files carry different priors) |

## F. Deferred / expensive

| id | name | type | definition |
|---|---|---|---|
| F1 | `llm_judgment` | cat | a small local model's wrap/phb/unsure vote as one feature among many (never a gate); defer until the cheap features are measured |
| F2 | `pos_tags` | cat | real POS tagging of A-final / B-initial words if closed-class lists prove insufficient |

## Leakage guards

1. **Strip the label from the input.** Corpus positives are identified by trailing `"  "` / `\`; every feature is computed on the marker-stripped line. Any feature that can see the marker reconstructs the label. **And marker ≠ intent (2026-07-22, tst corpus):** of ~1,900 markers in `~/src/_core/tst/planning/analysis`, essentially all were accidental trailing whitespace — 569 on list items, 223 inside code fences, zero on label-lines. Marker labels are valid only after a context filter (paragraph-internal prose lines only), and even then are noisy positives; every population's marker count must be context-audited before use (the udon DECISIONS.md markers were hand-verified genuine; the raw 977 total is inflated by an unknown factor).
2. **Generated negatives carry injection metadata** (exact break positions, model, date) — that metadata is for evaluation slicing only, never a feature.
3. **Split by file** (better: by file *and* corpus) for any holdout — breaks within a file are strongly dependent.
4. **Positive-class covariate shift is structural:** corpus phb examples come from convention-aware authors; the deployment target is convention-unaware blocks. Keep the five known real defects (2026-07-22 sweep) as a tiny gold test set that the training data never touches.
5. **Batch effects vs break physics (D7/D10).** Because label classes cluster by corpus/path, run leave-one-corpus-out cross-validation as the honesty check: if accuracy collapses when a corpus is held out, or if D7/D10 rank high in feature importance, the model is learning dataset identity. The static-model mitigation is sampling design (Joseph's, forthcoming) + stratified balance within paths; an online-updating forest would relax this differently, but the offline case must survive leave-one-corpus-out first.

## Status

Inventory only — no extraction, no model. Joseph has sampling/training-design ideas to contribute before that step; the extraction harness should be built against this table's ids so features can be added/dropped without rework.
