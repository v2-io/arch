# Why "just remove the manual line breaks" fails — a failure-mode catalog

*2026-07-22, from web evidence + structural analysis of the CommonMark spec. Purpose: (a) cautionary priors for the design, (b) a mining source for adversarial fixtures — nearly every entry below should become a test case. Companion to `landscape.md` (tool-by-tool empirical results, separate agent).*

## The two root causes

1. **To CommonMark, the newline you want to remove is already invisible.** A soft line break renders as a space, so the *rendered* document doesn't distinguish wrapped from unwrapped — which means render-preservation alone cannot license a join *or* forbid one; the wrap/keep decision is pure source-level policy no spec addresses. This is why generic formatters either normalize everything (prettier `proseWrap`) or nothing.
2. **But line starts and line ends are load-bearing everywhere else.** Almost every markdown construct is line-anchored. Joining two lines can silently *destroy* a construct, and (less obviously) can *create* one. The catalog:

## Joins that destroy meaning

- **Hard line breaks: trailing two spaces (invisible!) or trailing `\`.** A deliberate rendered break; a joiner that strips/ignores trailing whitespace silently deletes it. Reportedly a huge share of markdown line-break bug reports involve disappearing trailing spaces to begin with.
- **Table rows** — each row is one line; joining a wrapped-looking row into its neighbor destroys the table. This is the "broke all tables" class: tools that treat `|` lines as prose (several prettier `proseWrap: never` issues are exactly this — #11571, #11799, #17148: long rows, CJK width, code-in-cell all trigger de-formatting/mangling even in a real parser).
- **List structure under lazy continuation** — a continuation line may be unindented (lazy) or indented to the content column; a joiner must know the item's content column to know what's continuation prose vs. a nested construct. Naive joiners flatten nested lists or merge sibling items.
- **Line-anchored constructs that only exist at line start:** ATX headings, blockquote `>` markers, reference-link definitions `[label]: url`, footnote definitions, fence openers/closers, frontmatter delimiters, HTML blocks, `[^n]:`. Joining any of these into the previous line turns structure into prose.
- **Indented code blocks** — 4-space-indented lines are code; joining them into prose (or unwrapping prose *into* a context that changes its indent) crosses the code boundary.
- **Math regions** — `$$` on its own line is positional; and inline `$…$` spanning a wrap point means the join must happen *inside* a region the markdown parser doesn't even recognize (GFM parsers have no math node — pipes-in-math inside tables, emphasis-across-spans etc. live here).

## Joins that CREATE constructs (the subtle class)

- **Setext promotion:** if a paragraph line is followed by a line of `---` or `===` (e.g., a thematic break the author wrapped tight), text + underline becomes a *heading*. Conversely, joining a paragraph's last line *up* can leave a `---` orphaned, turning a setext heading into paragraph + hr.
- **Accidental list items / blockquotes:** unwrap policies that *re*-wrap (or that partially join) can land `- `, `1. `, `>` , or `#` at a new line start, minting structure from prose. (The mirror: a continuation line that *begins* with `2020.` or `- ` was already being parsed as a list item when the author meant prose — the file's current parse may not match author intent, so "preserve the render" preserves the bug.)
- **Emphasis/escape re-pairing:** joining changes adjacency; `*` `_` runs that were inert at line boundaries can pair up across the join (the same mechanism as asf's discovered GFM emphasis-across-`$`-spans rule).
- **`#` at former line start** inside a joined line becomes literal `#` — fine — but a `#slug` cross-ref that lands directly after `(` violates the Obsidian space-before-`#` rule; join spacing interacts with house rules.

## Verified in-house evidence (asf git history)

- asf history holds **labeled before/after reflow pairs** (confirmed, e.g. commits `9686985`, `3b99ad8`, and the early "Fully lint everything" era) — wrapped originals vs. linter+human-fixed finals: a ground-truth scoring set for join policy.
- `9686985` catches the incumbent's biggest gap live: its `--fix` output is *still partially wrapped* — every line ending in punctuation (`,`, `—`, `)`) stayed split, because `lint-md`'s join test requires the line to end in a word character (`\w$`). Comma-ended wraps are the most common shape in complex parenthetical prose ⇒ this one condition plausibly accounts for the bulk of the missing 40%.

## Ecosystem verdicts (to be confirmed empirically by landscape agent)

- **Pandoc `-f gfm -t gfm --wrap=none`** is the community's standard answer (CommonMark forum thread) — and even its author's endorsement carries the caveat that it introduces *incidental normalization* (bullet markers, link styles, header styles change), i.e., it is a renderer round-trip, not a formatter: comments/exact source forms are not preserved, math survives only via the right extension flags, and documented writer round-trip bugs exist (e.g., pandoc #11542). Fails our byte-stability and minimal-diff virtues even where it "works."
- **Prettier `proseWrap: "never"`** — real parser, but normalizes aggressively (whole-table realignment → git-history churn, #12074), has the table bugs above, and knows nothing of `$` math or wikilinks.
- **mdformat** — escapes characters defensively (would backslash-escape our prose), needs plugins for math (`mdformat-myst`; mkdocs plugin has "smart dollar" handling); plugin-dependent behavior is a config-fragility risk.
- **Online "unwrap" tools** — text-level, no parse at all; table destruction guaranteed. (Likely the family of Joseph's "broke all tables" experience.)

## Consequences for the design

1. Unwrapping must run on a **block-structure parse** (which lines belong to which container, at what content column), with **math, frontmatter, wikilinks as opaque first-class regions** — not on lines.
2. Even with a correct parse, the join decision inside a paragraph is **policy**, and the spec-silent zone (wrap-to-remove vs. deliberate-clause-line) needs an explicit, stated rule with a conservative "leave + flag" fallback when confidence is low.
3. **Idempotence and minimal-diff are first-class requirements**, ruling out renderer-round-trip approaches even where semantics survive.
4. Every bullet above → fixture. Plus property tests: (a) render-tree equality before/after (necessary, not sufficient), (b) `fmt∘fmt = fmt`, (c) random-wrap injection into known-clean corpus files must restore the original exactly.

Sources: [CommonMark forum: remove/reverse hard wrapping](https://talk.commonmark.org/t/remove-reverse-hard-wrapping-in-md-output/3107) · [prettier #11571](https://github.com/prettier/prettier/issues/11571) · [prettier #11799](https://github.com/prettier/prettier/issues/11799) · [prettier #17148](https://github.com/prettier/prettier/issues/17148) · [prettier #11798](https://github.com/prettier/prettier/issues/11798) · [prettier #12074](https://github.com/prettier/prettier/issues/12074) · [pandoc #11542](https://github.com/jgm/pandoc/issues/11542) · [mdformat docs](https://mdformat.readthedocs.io/) · [mdformat-mkdocs](https://github.com/KyleKing/mdformat-mkdocs) · [CommonMark spec: hard line breaks](https://spec.commonmark.org/)
