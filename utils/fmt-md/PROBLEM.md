# fmt-md — a markdown formatter worthy of the internal corpus

*Planning stage, opened 2026-07-22. Nothing here is settled except the problem. Working name `fmt-md` per the utils naming rule (plain names; say what it does) — rename freely if the shape changes.*

## The commission (Joseph, 2026-07-22, paraphrased close to verbatim)

A markdown formatter usable across a wide array of internal projects, to our asf-discovered standards. The impetus: fix up `~/src/udon/v2/udon-needs/02-tooling-needs/src/addressing-is-the-long-pole.md` and everything else in that directory, where the existing asf tool won't be particularly effective. His framing of the effort bar: *"research and plan this out very carefully and thoughtfully first"* — a real package eventually (crate/gem/whatever the design warrants) with real fixtures, e.g. *"meaningful random markdown with random newlines inserted into prose getting properly removed where appropriate."* And explicitly: **if the best-researched answer is an existing linter plus the config that actually makes it work, that is a fully acceptable outcome** — the mandate is to look at the problem deeply and find root causes / root information needed before being biased by any solution space.

## The three hard sub-problems, in his ranking of difficulty

1. **Removing manual word-wrap newlines** — "the one that everyone thinks will be somewhat trivial but isn't." The ad-hoc asf linter's heuristics get ~60% of the way, most of the time. One online tool touted as doing it right **broke all tables on everything he tried it on** (identity of that tool not yet known — worth recovering if possible, as a cautionary fixture source).
2. **Standardized blank lines** — should be easier; most existing linters get this one right.
3. **Inline `$` / display `$$` math linting** — "tractable but only if real effort is put into it." The conventions are *discovered*, not stylistic: they are the empirical intersection of what renders correctly for humans and agents on GitHub (MathJax, strict), in Obsidian (MathJax variant, different quirks), and when pulled into LaTeX itself.

## Ground truth to read (primaries, not summaries)

- **The standard:** `~/src/archema-io/asf/doc/sop/format.sop.md` — especially §Line Wrapping (the *positive* spec: no fixed-column wraps, but one-logical-chunk-per-line is *allowed and diff-friendly* — so "unwrap everything to one paragraph line" is NOT the spec; the enemy is column-width wrapping specifically), §Math Formatting → Compatibility Notes (the discovered cross-renderer rules: `\vert` not `|`, `\lt`/`\gt`, `\ast` in inline math, brace-removal before `_` to dodge GFM emphasis, `_`→`-` inside `\text{}`, `$$` on own lines with blank lines around, `\begin{aligned}` not `\begin{align}`, no `#slug` inside math), and §Cross-References (space before `#slug` for Obsidian).
- **The incumbent:** `~/src/archema-io/asf/bin/lint-md` (~950 lines, Python, line-based regex + a per-line `find_math_spans`). Its hard-wrap join rule is literally "line ends in `\w` and next line starts with `[a-zA-Z]`" — that single fact explains most of the missing 40%: no list-item awareness beyond the letter test, no sentence-boundary judgment, per-line math spans (breaks on multi-line inline constructs), no table/blockquote nuance beyond skips.
- **The immediate patient corpus:** `~/src/udon/v2/udon-needs/02-tooling-needs/src/*.md` (29 files) plus `../reports/`, `OUTLINE.md`, `RESIDUALS.md` etc. in the parent. Features the asf corpus mostly lacks: YAML frontmatter **with inline `#` comments**, `[[wikilink|label]]` forms, hard-wrapped prose *inside numbered/bulleted list items* and blockquotes, bold-lead paragraph conventions. Little math. The asf segment corpus (`~/src/archema-io/asf/*/src/*.md`) is the math-heavy complement. Both must survive round-trips unharmed. **Corpus caveat (Joseph, mid-planning):** the asf tree in its *current* state is mostly already fixed — linter pass plus a lot of manual intervention afterward — so it serves as a clean/idempotence corpus, not a disease corpus. To actually experience the problem you need long examples with lots of parentheticals and complex prose that still carry the wraps. Two sources: (a) **asf git history** — because fixes were linter+manual, history holds labeled before/after pairs (wrapped original → human-approved final), a ground-truth scoring set for the join policy; (b) unfixed trees elsewhere under `~/src/` (and `_obs/` archaeology).
- **vivarium** (Joseph, mid-planning): also a patient. Newer files are asf-like from the start, but there's a lot — e.g. `vivarium/.archive/` (confirmed: ~1300 wrapped-length lines in top-level `.archive` files, plus `core/` subtree) — with **dense citations and parentheticals** that defy the existing linter. Citation parentheticals are a distinct stressor: `(Author 2024, pp. 3–5; cf. …)` wraps mid-parenthetical and is full of interior periods/commas/capitals that fool sentence-boundary heuristics.
- **Possibly relevant:** `~/src/archema-io/asf/.obsidian/plugins/obsidian-linter/` (installed; `data.json` holds its live config) — one existing tool already trusted enough to sit in the vault.

## The unifying reframe (Joseph, mid-planning 2026-07-22)

What we are attempting to fix universally is, specifically, *default forms of markdown produced by current frontier logogenic agents*. Frontier-model agents currently do NOT wrap markdown in chat/text responses, but almost always pre-wrap when writing files. Consequences:

- **Testable/generatable at will:** spin up a fresh instance (Sonnet etc.), have it write a long complex document to a file with no prior exposure to properly-formatted house-style md — that's a disease sample; the model's own chat-form output of the same content is the unwrapped ground truth.
- **Narrower target distribution than "any wrapped markdown":** agent-default wrapping has fittable signatures (per-model fill column, word-boundary breaks, characteristic continuation indents). Design the policy for this distribution, not for every historical human wrapping habit.
- **The math disease form is Unicode-by-default** (same message): agents use `$` notation when exposed to it but otherwise default to Unicode math (bare `η`, `Σ_t`, arrows) in files. So the math layer isn't only quirk-repair inside existing `$…$` — the on-distribution input includes Unicode/ASCII math in prose needing promotion to `$LaTeX$` (the incumbent's bare-greek / bare-ascii-math checks are its hardest and least auto-fixed — expression-boundary detection is the crux, cf. lint-md's deliberately conservative `fix_bare_unicode_greek`).
- **Fixture generation belongs in the package:** model defaults drift by release, so a regeneration harness (prompt battery × model roster) keeps the fixture corpus tracking the real noise distribution.

## Root-cause reading of why this is hard (session hypothesis — test it)

Unwrapping is hard because it is **not a lexical problem**. "Is this newline a manual wrap or a semantic break?" needs (a) a real block-structure parse (CommonMark-correct: lists, lazy continuation, tables, blockquotes, code, HTML blocks, frontmatter as a foreign region, math as a foreign region GFM parsers don't even know about), and (b) a *policy* over prose lines that the CommonMark spec is silent on, because to CommonMark a soft line break is already semantically nothing — which is exactly why generic formatters either normalize everything (destroying deliberate clause-per-line) or nothing. Tables break in naive tools because unwrap-then-reflow treats `|` lines as prose. Math breaks in *every* mainstream markdown toolchain because `$` is not CommonMark; a formatter that parses emphasis/escapes inside `$…$` will mangle it (the same class of bug as GitHub's emphasis-across-spans behavior, from the other side). So the load-bearing requirement for any candidate — existing or built — is: **CST-faithful parse with math and frontmatter as opaque first-class regions, and prose-line policy as an explicit, configurable judgment layer** (possibly conservative: only join when confident, flag when not).

## Success criteria (draft — sharpen during planning)

- Idempotent (`fmt(fmt(x)) == fmt(x)`), byte-stable on already-clean files.
- Never alters rendered semantics: tables, lists, code, links, wikilinks, frontmatter, footnotes, HTML — property-testable via render-tree comparison before/after.
- Unwraps column-width wraps including inside list items/blockquotes; leaves deliberate logical-chunk lines when policy says so (this policy needs a crisp statement — currently it does not have one anywhere).
- Applies the discovered math rules, or at minimum never damages math and flags what it can't fix.
- Round-trips the full asf + udon-needs corpora with zero render diffs.
- Usable standalone across projects (one binary/script, minimal config).
- **Extensible rule surface** (Joseph, mid-planning 2026-07-22): expandable to enforce link formatting and similar per-project conventions — rules as a first-class, growable layer rather than hardcoded checks.
- **Ecosystem affinity** (same message): *"If it's something that can fit in with the udon ecosystem and parsers being developed in rust, even better."* This rhymes with the standing program trend (asf agents.sop: durable shared content-parsing tools trending Rust on udon-core at `~/src/archema-io/common/utl/`). A leaning, not a foreclosure — the landscape research still gets to say "existing tool X + config wins."

## State

- [x] Problem statement (this file)
- [ ] Landscape research: existing formatters actually tested against corpus samples (mdformat, prettier `proseWrap`, dprint, pandoc round-trip, obsidian-linter, comrak/markdown-rs-based tools, anything else found)
- [ ] Requirements synthesis: the full de facto standard extracted from format.sop.md + lint-md + corpus reality, stated as a checkable spec
- [ ] Design decision: adopt / adapt / build, and in what language
- [ ] Then: fixtures, implementation, and the udon-needs cleanup itself
