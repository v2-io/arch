# fmt-md — requirements synthesis

*Requirements half of the fmt-md planning effort, 2026-07-22. Companion to the landscape report in this directory; the plan document cites both. Sources read whole: `PROBLEM.md`, `asf/doc/sop/format.sop.md`, `asf/bin/lint-md`, `asf/.obsidian/plugins/obsidian-linter/data.json`, plus direct corpus measurement of `~/src/udon/v2/udon-needs/` (359 md files) and `~/src/archema-io/asf/0*-*/src/` (286 md files), `~/src/archema-io/vivarium/.archive/` (added mid-task), asf git-history reflow pairs (§3a), and the coordinator's `research/unwrap-failure-modes.md` catalog (§3b).*

Provenance codes used per requirement: **SOP** (format.sop.md prose), **LINT** (lint-md's encoded judgment), **PROB** (PROBLEM.md / Joseph's commission), **CORPUS** (corpus-observed, unwritten), **GAP** (nowhere stated; needs a decision). Scope note per rule: **U** = universal, **P** = per-project-configurable — per Joseph's mid-planning extensibility signal.

## 0. The headline corpus finding: the two corpora are polar opposites

Measured, not inferred:

| Corpus | Nonblank lines | Median length | p90 | Lines >120 |
|---|---|---|---|---|
| asf `0*/src/*.md` | 14,658 | (avg **280**) | — | 7,693 |
| udon-needs `02-tooling-needs/src/*.md` | 2,440 | **69** | 75 | 29 |
| vivarium `.archive/**` (3rd patient, added mid-task) | 3,179 | **78** | 288 | — |

The vivarium `.archive` corpus (16 files incl. `core/`) is a *mixed* wrapped corpus in asf house style (bold-leads, wikilinks, `#slug` refs) — p50 78 wrapped, p90 288 already-long — and it carries the densest citation/parenthetical prose (see R15f).

Mid-effort context from Joseph (via the coordinating agent): the asf tree has *already been fixed* — linter plus substantial manual intervention — so it is evidence of the **target** state, not the disease; the genuinely-dirty corpora are udon-needs and long complex-parenthetical prose elsewhere under `~/src/`. That reading matches the measurements:

The asf segment corpus is already essentially one-logical-line (with a deliberate sentence-per-line minority — e.g. `04-eli-core/src/deriv-identity-sufficiency-rate-bound.md`, full sentences each on their own line). The udon-needs tooling corpus is uniformly column-wrapped at ~72 with a tight right margin (p50 69 / p90 75), including inside numbered list items (149 wrapped list continuations) and blockquotes (392 wrapped blockquote continuations across udon-needs). The udon reports directory is *mixed-state* (p50 41 but 227 lines >200) — both styles inside single files.

This reframes the wrap problem (see §3): the question is less "is this newline a wrap?" per line and more "what style is this block in?" per block — and separately, per project, "what style should it be in?"

**Distribution provenance (Joseph's unifying reframe, PROBLEM.md).** The universal target is *default markdown produced by current frontier agents when writing files* — models don't wrap in chat, almost always wrap in files. Two consequences for reading the numbers above honestly: (a) the measured signatures (udon p50 69 / p90 75; vivarium p50 78; the ~72-col fill) are **measurements of particular models' file-writing defaults at particular dates**, not timeless properties of "wrapped markdown" — they will drift by model release, so the spec hardcodes none of them: R15's statistics are computed per file/block at run time, and only the *shape* of the signature (tight clustering + word-boundary breaks) is assumed; (b) the wrap-detection policy may be *fit* to this narrower distribution — agent-default wrapping is more regular than historical human wrapping (consistent fill column, clean word-boundary breaks, characteristic continuation indents), which is what makes R15's statistical approach tractable at all.

## 1. Foundation requirements

**R1. CommonMark/GFM-correct block-structure parse, CST-faithful.** Lists (incl. lazy continuation), blockquotes, fenced code, tables, headings, HTML blocks, thematic breaks. Bytes outside regions the active rules touch are preserved exactly. *(PROB root-cause section; the incumbent's line-based approach is the named cause of its 40% miss. U)*

**R2. Opaque first-class foreign regions**, never parsed as markdown, never reflowed, never edited except by rules that explicitly own them: fenced code (any fence length/char, incl. `mermaid` fences — present in corpus), inline code spans, YAML frontmatter, `$…$` / `$$…$$` math, HTML blocks/spans (present in udon-needs), Obsidian comments if encountered. *(PROB; LINT skips code/yaml/display-math today. U)*

**R3. YAML frontmatter is byte-opaque, not parse-and-reserialize.** The udon corpus carries frontmatter with inline `#` comments (e.g. `addressing-is-the-long-pole.md` lines 10–15: trailing comments on mapping entries and sequence items), flow-style arrays (`evidence: [T1, T2, T3]`), and unquoted values containing `:` and `;` (`status: cross-tier-convergent (demand); all syntax questions deliberately open`). Any standard YAML load/dump cycle destroys the comments and may reorder/requote — so frontmatter rules must operate on raw lines or a comment-preserving CST (a `yaml-edit`-class approach), or not at all. *(CORPUS — this is the single sharpest "would break the patient" hazard after tables. U)*

**R4. Wikilinks are atomic tokens.** Forms observed: `[[slug]]`, `[[slug|label]]`, `[[DECISIONS.md|design ledger]]` (path targets), `[[../02-normative]]` (relative paths), `[[src/img/orient-cascade.pdf]]` (embed-style asset refs in asf OUTLINE files). Never introduce or remove a line break inside `[[…]]`; treat the interior as opaque. *(CORPUS. U)*

**R5. `#slug-name` prose cross-references are load-bearing text, not noise.** Both corpora use them ( #counter-register etc. in udon; pervasive in asf). Rule: preserve always; optionally enforce space-before-`#slug` (Obsidian tag recognition — auto-fixable, `(#foo` → `( #foo`). *(SOP §Cross-References; LINT obsidian-spacing. Preservation U; space rule P — udon may not want it.)*

**R6. Callouts (`> [!note]`, `> [!warning] Title`) parse as blockquotes with a special first line.** The type/title line is never joined into the body; body lines are blockquote prose subject to the wrap policy. Present in both corpora. *(CORPUS. U)*

**R7. Footnotes** (`[^anchor]` refs and definition blocks; 34 udon-needs files) survive round-trip; definition blocks are paragraph-bearing containers for the wrap policy like list items. *(CORPUS. U)*

**R8. Hard line breaks.** Zero trailing-two-space breaks found in udon tooling src, but if present anywhere they are semantic — a two-space (or backslash) break is never a "wrap" candidate. *(CommonMark semantics; CORPUS-absent but must-not-corrupt. U)*

**R9. Tables are structure-opaque for the wrap policy.** No line in a table row is ever a join candidate; cell content is never reflowed. (The unnamed online tool that "broke all tables" is the standing cautionary fixture — PROBLEM.md wants its identity recovered.) Optional cosmetic rule: column-padding normalization — corpus has both padded and compact separators, so this is P and default-off. *(PROB; CORPUS.)*

**R10. Exclusion configuration is required — some corpus regions are deliberately frozen.** udon-needs `01-ideation/02-provenanced/copies/` is verbatim provenanced source material; asf `src/old-*.md`, `_obs/`, and `spikes/.integrated/` are frozen archaeology that tooling explicitly skips today. A corpus-wide `fmt-md .` run without path/prefix exclusions would rewrite provenance records — a truth-status defect, not just a style mistake. Requirement: ignore-paths + ignore-prefix config, with the asf `old-*` rule and a provenanced-copies convention as shipped defaults for those projects. *(SOP §Segment-set principle for `old-*`; CORPUS for `copies/`. P mechanism, U that the mechanism exists.)*

## 2. Invariants (the acceptance bar)

**R11. Idempotent:** `fmt(fmt(x)) == fmt(x)`. **Byte-stable on clean files.** *(PROB success criteria. U)*

**R12. Render-semantics preservation, property-tested:** rendered tree (CommonMark AST or HTML) identical before/after, modulo softbreak→space in joined prose. Both full corpora must round-trip with zero render diffs. *(PROB. U)*

**R13. Never emit fixed-column wrapping.** The output side is settled even where the input side isn't: format.sop forbids imposing reading-width wraps. fmt-md removes column wraps; it never creates them. *(SOP §Line Wrapping. U)*

**R14. Check mode and fix mode**, distinct exit codes, per-issue rule ids, and machine-readable output (agents are a primary consumer of the diagnostics — they read the output to decide fixes the tool declined to make). *(LINT precedent; PROB "flag when not confident". U)*

## 3. The wrap/unwrap policy — the named gap, now with corpus evidence

format.sop §Line Wrapping in full: *"Do not hard-wrap lines just to artificially impose reading width. … One sentence or clause per line is fine for diff-friendliness … but do not insert line breaks at a fixed column width. Think logical chunks and not line-length."* So two legal source styles exist (paragraph-per-line, chunk-per-line) and one illegal one (column-wrap). The unstated policy is how a tool tells them apart and what it converts *to*.

**The starting hypothesis in the brief (lowercase-mid-clause ⇒ wrap; clause-boundary break ⇒ chunk) is directionally right but insufficient as a per-line test.** Measured over udon tooling src, classifying every prose continuation pair (line + next line, both prose, next not starting with a structural character):

| Case | Count |
|---|---|
| line ends in `\w`, next starts lowercase | 1,219 |
| line ends in punctuation (`,` `—` `"` `)`…), next starts lowercase | 201 |
| line ends in `\w`, next starts uppercase | 73 |
| line ends sentence-punct, next starts uppercase | 51 |

In this corpus **all four rows are wraps** (the files are uniformly column-wrapped). The incumbent's rule (`\w` + letter) catches row 1+3 ≈ 84% — the measured "~60%, most of the time" is this plus its container blind spots. The lowercase heuristic alone would still leave the 51 sentence-boundary/uppercase wraps (e.g. `"…It hasn't." / "Counter-register row 8 is…"`) — which are *individually indistinguishable* from deliberate sentence-per-line chunking. No per-line classifier can decide those.

**R15. The decision unit is the block (paragraph, list-item body, blockquote paragraph, footnote body), informed by file-level statistics — not the individual newline.** A block whose internal line lengths cluster under a consistent right margin (corpus signature: p50 ≈ 69, p90 ≈ 75, i.e. tight clustering in ~60–80 with mid-clause breaks present) is column-wrapped: join *all* its soft breaks, including the sentence-boundary ones. A block whose every line ends at a sentence/clause boundary (`.` `?` `!` `:` `;` and em-dash chunk ends) with heterogeneous lengths is chunk-per-line: keep every break. *(GAP, resolved here as a proposal; CORPUS evidence above. U mechanism.)*

**R16. Ambiguity is explicit, with configurable disposition.** Short blocks (2–3 lines) genuinely lack statistics; mixed-evidence blocks exist (the reports dir is full of them, and even the patient file has one already-joined 130-char line inside otherwise-72-col prose). When block-level evidence is below threshold, consult file-level evidence (a file that is 95% confidently-wrapped licenses joining its ambiguous short blocks); when *that* is ambiguous, do the configured fallback: `join | leave | flag` — proposed default `leave + flag`. *(PROB "possibly conservative: only join when confident, flag when not". U mechanism, P thresholds/default.)*

**R17. Join semantics are container-aware.** Joining strips the continuation's container prefix, not just leading whitespace: list continuations (3-space or marker-aligned indent — both in corpus), blockquote `> ` prefixes, nested combinations. Join with exactly one space; never join across a blank line, a container-type change, or into/out of an opaque region. Lines whose *first* character is structural (`-` `*` `#` `>` `|` `$` `` ` `` digits+`.`) are never continuations — but note the letter-test's converse hazard: a continuation may legitimately start with `[` (a link) or `(` or `*emphasis*`, which the incumbent's `[a-zA-Z]` test misses (part of the residual 16%). *(LINT gap analysis; CORPUS. U)*

**R18. Target style is a per-project setting, separate from wrap detection.** Detection says "this block is column-wrapped"; the target says what to produce: `one-line-per-paragraph` (what asf's corpus reality is) or `preserve-chunks` (join only detected wraps, keep deliberate chunk breaks). The spec permits both source styles, so the tool cannot hardcode one. Proposed defaults: asf → one-line-per-paragraph; udon-needs → decided by Joseph (Q2 below). *(SOP permits both; GAP on which to produce. P)*

### 3a. Ground truth from asf git history — the implicit human policy, recovered

Because the asf fixes were linter-then-manual, the history holds labeled before/after pairs (coordinator-verified: `9686985`, `3b99ad8` "Reflow hard-wrapped prose … via bin/lint-md --fix"; the early "Fully lint everything" era `300a66c`/`349e3a1` etc.). Mined findings:

1. **`9686985`'s own diff shows the incumbent's residuals live:** its `--fix` output is *still partially wrapped* — every line ending in `,` / `—` / `)` stayed split (join test requires `\w$`), and continuations starting with `(` stayed split (continuation test requires `[a-zA-Z]`). Comma-ended wraps dominate complex parenthetical prose, so this pair of conditions is plausibly the bulk of the missing 40% — consistent with the §3 measurement (252 punctuation-ended continuations in udon src that the incumbent cannot join).
2. **Subsequent human/agent passes joined those residuals.** History search for joins of comma-ended wrapped lines finds them repeatedly (`1df2fb9` "Reflow big picture tracker", `67064c0`, `9a51728`, `84897ad` — including joins across `$math$,` boundaries). So the implicit human policy, where cleanup was carried to completion, is: **within a paragraph, join everything to one logical line, regardless of line-final punctuation** — punctuation-ended breaks were never treated as chunks-to-keep in these passes.
3. **The tolerated-debt caveat:** the current `doc/sop/spikes.sop.md` still carries mixed paragraphs (line lengths 217/149/140/75/158 within one paragraph) and untouched ~72-col wrapped blockquotes — lint-clean because the incumbent's checker shares its fixer's blind spots. So "lint-clean asf docs" is *not* a reliable gold standard; the **segment corpus** (avg 280, post-manual) is the trustworthy target-state evidence, and the ragged `doc/` files are themselves good dirty fixtures.
4. **Deliberate chunking is real but rare and shaped:** the surviving chunk-per-line segments (e.g. `deriv-identity-sufficiency-rate-bound.md`) are *complete sentences per line, every line*, kept through cleanup passes. This sharpens R15's chunk signature: uniformity of sentence-boundary endings across the whole block, not the presence of some.

These pairs should be extracted into the fixture set as scored ground truth (R34): pre-reflow file → post-manual file, with the tool's output diffed against the human-approved final.

### 3b. Joins that create constructs (from `research/unwrap-failure-modes.md`)

The companion failure-mode catalog's subtle class — joins that *mint* structure — adds requirements a destroy-only analysis misses:

**R15a. Setext guard.** Never join a paragraph line when the *following* line is `---`/`===` (the join target would become a setext heading, or an orphaned `---` becomes a thematic break). Equivalently: join decisions must be made on the parsed block, and re-parse equality (R12) must be checked structurally, not assumed. *(unwrap-failure-modes §creates. U)*

**R15b. No partial joins that leave structural characters at a new line start.** Any join/leave sequence inside a block must never land `- `, `1. `, `>`, `#`, or `    ` (4-space indent) at the start of a resulting line where it wasn't one before. *(same. U)*

**R15c. Emphasis/escape re-pairing check.** Joining changes adjacency: `*`/`_` runs inert at a line boundary can pair across the join (same mechanism family as the discovered GFM emphasis-across-`$`-spans rule). The render-equality property test must cover inline emphasis, not just block structure. *(same. U)*

**R15d. Join spacing composes with house rules.** A `#slug` landing directly after `(` post-join violates R5's space-before-`#` rule; the joiner either inserts the space when that rule is active or flags. *(same; SOP §Cross-References. P follows R5.)*

**R15f. Sentence/clause-boundary detection must be parenthetical-aware and abbreviation-aware.** The vivarium `.archive` corpus (added mid-task; measured directly) wraps *inside* parentheticals constantly — 143 continuation breaks occur at open paren depth > 0, and continuations start with capitals, years, dates, or `(` itself (52 `(`-starting continuations): `…(built on \`spike/hydrology\` 2026-06-29, **merged to main` / `2026-07-03** — 147 commits…`; `…(the` / `principled replacement for the FBM prior)…`. Consequences, corpus-verified:
  - A line ending with **unbalanced open `(`** (or ending inside one, tracked across lines) is near-conclusive *wrap* evidence — deliberate chunk lines do not split mid-parenthetical. This is a strong per-line join signal that works even in short blocks where R15's statistics are thin.
  - The chunk signature "line ends at a sentence boundary" must evaluate boundaries **at paren depth zero only**, and must not count abbreviation periods (`cf.`, `e.g.`, `i.e.`, `pp.`, `vs.`, initials, `2019).`-style year-close forms) as sentence ends — citation parentheticals like `(Author 2024, pp. 3–5; cf. …)` contain interior periods, commas, semicolons, and capitals that would otherwise fake both chunk-ends and sentence-starts.
  - The continuation test must not exclude digit-starting lines wholesale: `2026-07-03…` is a legitimate continuation; only a true ordered-list marker (`\d+[.)]` + space, per CommonMark) is structural. (This tightens R17's structural-starter list, which as first drafted would have missed these.) *(CORPUS — vivarium `.archive`; coordinator relay of Joseph's recollection that these files defied the incumbent, which the measurement corroborates: `\w$`+letter fails on most of these shapes. U)*

**Which wrap signals are distribution-specific vs universal** (per the unifying reframe): the *universal* (markdown-semantics) signals are R15f's paren-depth tracking, abbreviation handling, the structural-starter grammar (R17), and everything in §3b — those hold for any wrapped source. The *distribution-fitted* signals are the tight right-margin clustering, word-boundary breaks, and consistent continuation indents of R15 — valid because agent-default wrapping is regular, and re-validated against the live distribution by the R35 harness rather than assumed. If the tool ever meets historical human wrapping (irregular columns, hyphenation), the fitted layer degrades to the R16 fallback (leave + flag), which is the correct failure mode.

**R15e. Current-parse ≠ author-intent caution.** A continuation line already beginning with `2020.` or `- ` is *already misparsed* as a list item today; preserving the render preserves the bug. Flag (never silently "fix") such lines. *(same. U)*

## 4. Blank-line and structural normalization

**R19. Display `$$` on own lines, blank line before and after** — check + auto-fix. *(SOP §Math Formatting; LINT display-math-blanks. U where math exists.)*

**R20. Blank-line normalization (the "easier" sub-problem):** collapse 3+ consecutive blanks to ≤2 (exact max unwritten — Q5), blank line after headings and around block-level structures (tables, fences, display math), single trailing newline at EOF. *(PROB sub-problem 2; GAP on exact numbers — no source states them; corpus is mostly single-blank-between-blocks. P for maxima, U for EOF-newline.)*

**R21. Indented (4-space) bare-equation detection** — flag, don't fix (it's a code block to CommonMark; intent is display math). *(LINT check_indented_bare_math. P — asf-shaped.)*

## 5. Math rules (the discovered cross-renderer standard)

All from SOP §Compatibility Notes with LINT implementing; these are *discovered* constraints (GitHub-MathJax ∩ Obsidian ∩ LaTeX), not style. Universal-when-math-is-present; projects without math (udon-needs has almost none) simply never trigger them.

**R22.** No space just inside `$…$` (auto-fix). **R23.** `\vert`/`\lvert`/`\rvert` not `|`; `\Vert`/`\lVert`/`\rVert` not `\|` (fix for `\|`; flag bare `|` — context-dependent). **R24.** `\lt`/`\gt` not raw `<`/`>` in math (auto-fix). **R25.** `\ast` not `*` in *inline* math (auto-fix; display exempt). **R26.** `\begin{aligned}` not `\begin{align}` (auto-fix). **R27.** `_` → `-` inside `\text{}` (auto-fix). **R28.** Emphasis-vulnerability brace removal: `\hat{P}_` → `\hat P_` for the brace-removable command set (auto-fix); flag residual multi-span `_` vulnerability. **R29.** No `#slug` anywhere inside math mode incl. `\text{}`/`\boxed{}`/`\tag{}` (flag; fix is editorial). **R30.** Bare Unicode Greek math variables outside `$` (flag; auto-fix only the isolated-prose-token subcase — lint-md's conservative logic is a good spec); bare ASCII math patterns (`M_t`) and LaTeX commands outside math (flag only). **R31.** Multi-line inline-math spans: the incumbent's per-line `find_math_spans` breaks on them (named cause of misses). The new tool must either resolve math spans across soft breaks *before* wrap-joining (correct order: math regions first, then join) or flag unbalanced `$` on a line as unprocessable. Also: a lone `$` (currency) must not open a phantom span — require the no-space-after-`$` opening convention as the span heuristic, as the ecosystem does. *(PROB sub-problem 3; LINT architecture critique. U-when-math.)*

## 6. Extensibility and configuration

**R32. Rules are a first-class registry**, each carrying: id, description, universal/per-project scope, capability (fix vs flag), and provenance/rationale pointer. Adding a rule (Joseph's named example: link-formatting conventions) must not require touching the parser or wrap engine. *(PROB extensibility criterion, Joseph mid-planning. U)*

**R33. Config discovery per project** (config file at project root; sane zero-config defaults = the universal rules only). Per-project payload: target wrap style (R18), ambiguity disposition (R16), exclusions (R10), optional rule enables (R5 spacing, R9 table padding, R21), blank-line maxima (R20). *(PROB "usable standalone across projects, minimal config". U)*

**R34. Fixture discipline:** golden before/after pairs from both corpora; property tests including Joseph's named one — take clean prose, insert random newlines, verify recovery — plus its dual: take chunk-per-line asf segments, verify zero joins. The mixed-state reports files and the patient file's own already-joined long line are natural hard fixtures. *(PROB commission. U)*

**R35. Fixture-regeneration harness ships with the package.** Because the target distribution is *current-model file-writing defaults* and those drift by release, the package includes a harness that regenerates disease samples at will: a prompt battery (long complex documents spanning the construct inventory of §1 — lists, blockquotes, citations/parentheticals, tables, math, frontmatter) × a model roster, exploiting the chat-vs-file asymmetry Joseph identified — the same model's chat-form output of the same content is the unwrapped ground truth, giving labeled pairs without human annotation. Regenerated fixtures are dated and model-tagged (extending the provenance discipline of §0); the static corpus-derived fixtures (R34, §3a git pairs) remain as regression anchors. *(PROB §The unifying reframe. U)*

## 7. Findings adjacent to the assigned task

**F1. The Obsidian Linter config is normatively empty.** Every one of its ~70 rules is `enabled: false`; `lintOnSave` is false. The only live judgment in `data.json` is `minimumNumberOfDollarSignsToBeAMathBlock: 2`. So "one existing tool already trusted enough to sit in the vault" overstates it — the vault *installed* it and then turned everything off, which reads as either an unfinished configuration pass or a deliberate "no auto-linting in Obsidian" stance (Q6). Either way the fourth source in the brief contributes ~nothing to the spec, and the requirements here rest on three sources plus corpus.

**F2. The incumbent's miss is architectural, not parametric.** The measured gap decomposes as: ~16% of continuation pairs fail the `\w`+letter shape (punctuation-ended wraps, `[`/`(`-started continuations); plus the container blind spots (list/blockquote prefixes — 149+392 instances in udon-needs alone); plus per-line math spans. No amount of regex tuning closes this; R1/R15/R17 do. This confirms PROBLEM.md's root-cause hypothesis with numbers.

**F3. Frozen-region exclusion (R10) may be the highest-stakes requirement nobody wrote down.** The formatter's first realistic failure mode isn't a broken table — it's an agent running it repo-wide and silently reformatting provenanced verbatim copies and archaeology.

**F4. Quoted verbatim material is a policy edge.** udon-needs blockquotes include Joseph's verbatim messages ("typos included" is an explicit convention elsewhere in the program). Whitespace-only rejoining doesn't change words, but "verbatim" is a stance; whether blockquotes get a preserve-exactly option is Q4.

## 8. Open policy questions — for Joseph, not a tool author

**Q1.** Ratify (or amend) the wrap policy now assembled from R15 + §3a — proposed crisp statement: *a block is either column-wrapped (tight right-margin clustering with mid-clause breaks — join ALL its soft breaks, punctuation-ended included, per the recovered human policy) or deliberately chunked (every line ends at a sentence/clause boundary — keep ALL its breaks); blocks matching neither signature at confidence get the configured fallback, default leave + flag; sentence/clause boundaries are evaluated at paren-depth zero with an abbreviation list, and an unbalanced open parenthetical at line end is conclusive wrap evidence (R15f).* The §3a evidence makes this much less open than it was at brief time; what remains is ratification and the fallback default.

**Q2.** Target style for udon-needs after unwrapping (R18): full one-line-per-paragraph like the asf target state, or preserve-chunks? The commission ("fix up … that directory") implies unwrap, but not which target.

**Q3.** Mixed-state files (udon reports): normalize whole-file to target style, or only fix confidently-wrapped blocks and tolerate residual inconsistency?

**Q4.** Do blockquotes carrying verbatim quotations get an exemption from rejoining (preserve-exactly), or is whitespace-normalization of quotes acceptable?

**Q5.** Blank-line maxima and heading-spacing exact numbers (R20) — nowhere written; pick once.

**Q6.** Is the all-disabled Obsidian Linter config deliberate (no auto-lint in the vault) — i.e., should fmt-md likewise never run on-save, only on-demand?

**Q7.** Sentence-per-line vs clause-per-line: format.sop says "sentence or clause"; should fmt-md ever *convert between* chunk granularities, or is chunk-preservation always verbatim? (Proposed: always verbatim — conversion is authorial.)

## 9. Feedback on the brief

The brief's four-source framing held up with one correction (F1: source four is empty). The starting hypothesis on wrap detection was the right *signal set* but the wrong *decision unit* — the corpus measurement in §3 is the fresh-eyes contribution the brief asked for. One thing the brief undersold: the frontmatter-comment hazard (R3) is not just "a feature the asf corpus lacks" — it structurally rules out every formatter that normalizes frontmatter via a standard YAML library, which should be a hard filter in the landscape evaluation happening in parallel.
