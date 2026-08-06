<!--
  Verisectorium notes gather — survey + extracts, not live authority.
  Sources read first-hand 2026-08-05: neurips/AUTHORING.md (build-relevant sections),
  neurips/BUILD-CHANGE.md (whole head), neurips/bin + asf/bin listings,
  neurips/01-tragedy-confident-agent tree. ASF build design docs were already
  copied under ../asf/ (markdown-first-pipeline-design, build-markdown-design,
  build-latex-plan-early); this file adds the neurips/paper-project half Joseph
  noted was missing, and names the cross-instance build questions.
-->

# Build issues across the instances — what assembly forces the format to decide

*The build layer is where the verisectorium model gets stress-tested: every
projection commitment (outline-as-view, evergreen segments, WN filtering,
identity vs presentation-number) was either discovered or hardened because
something had to assemble a PDF. ASF's half of this story is in
`../asf/` (see synthesis B); this note adds neurips and the paper projects.*

## 1. neurips — the most build-evolved instance

Pipeline: `bin/build` (umbrella-level, cwd-aware) assembles `<paper>/src/*.md`
segments per **assembly manifest** `OUT.<stem>.md`, through kramdown→LaTeX→
`lualatex`, with refs auto-emitted from the umbrella `refs/entries/*.yml`.

Build-forced model insights (AUTHORING.md, verified against the live tree):

- **Multiple manifests per paper = multiple outlines over one substrate.**
  `OUT.full-paper.md` and `OUT.neurips-2026-paper.md` (9-page budget) select
  different segment subsets. Stated as law: *"trimming is which segments land
  in this manifest, not editing segments to fit a smaller form"* and *"page
  count is observable, not actionable on segments."* This is PROPOSALS §H.4
  (outlines cheap / segments expensive) independently reached under venue
  pressure — the strongest cross-instance confirmation of outline-as-view.
- **The manifest's `Type` column drives structure injection** — `\appendix`
  before the first Appendix-typed row, `\newpage` before Checklist rows,
  `Bibliography` row placement. The view's row metadata (edge attributes)
  carries *rendering directives*, not just ordering — a concrete instance of
  the underlying-logical-model's view-with-edge-attributes.
- **No manual numbering anywhere** (sections, equations, theorems) — the prior
  workspace's manual numbering "broke under reorganization"; pipeline owns
  layout, authors own content. Same lesson as ASF's no-numbers-in-filenames,
  pushed down to intra-document structure.
- **One source, two renderers** forces authoring constraints: single-`$` math
  is a hard requirement because the same segment text feeds both the LaTeX
  build and OpenReview's MathTeX abstract renderer. Presentation-neutrality of
  the substrate is real only if the *notation* stays in the intersection of
  target renderers — a constraint class ASF's markdown-first commitments don't
  yet name.
- **Preamble ownership is split by role:** segment authors may not inject
  preamble; missing packages/environments go through the build owner for
  cross-paper consistency. A governance boundary (who may change the shared
  projection) distinct from the content boundary.
- **Derived-artifact hygiene is a lived problem:** the 2026-05-06 refactor
  (BUILD-CHANGE.md) exists largely to relocate/rename derived artifacts so
  their derivedness is visible by construction (`.build/` per-manifest dirs;
  `<stem>.extracted.bib` "naming is explicit-on-purpose"), and to kill an
  orphaned hand-editable file (`refs.bib`) that the build no longer read —
  the generated-vs-authored declaration problem, solved by naming convention.
- **Anonymization lint gates** (`bin/refs lint` against `refs/deny-list.yml`,
  scanning segments *and* bib) — the stakes × reversibility specimen, here in
  its original home.

## 2. Paper projects (behavioral-floor, causal-language, logos)

- behavioral-floor and causal-language: pandoc-concat over `src/NN-*.md` —
  order in filenames (violating identity/ordering split) because the venue
  skeleton is fixed and cheap beats principled at that scale. The build is a
  concat; no manifest layer, so no multi-view affordance — and notably nobody
  has needed one there yet. Instance evidence that the *manifest layer is what
  buys multi-view*, and is worth its cost only when views multiply.
- logos: venue-specific outputs (Synthese LaTeX/Word; Inquiry T&F) explicitly
  did NOT transfer with the refs-store machinery ("output format here is
  venue-specific and not yet wired") — the population/serving split holding
  while the projection layer lags.

## 3. ASF (pointer — copies under `../asf/`)

`bin/build-monograph` (OUTLINE+segments→chunks→assembled md→PDF),
`bin/build-markdown` (digest recipes as data; `--public` strips Working
Notes), `bin/lint-outline`, plus the design docs already gathered:
markdown-first commitments (index = assembly manifest; cross-refs at assembly;
PDF one rendering among several), build-latex early forks (WN strip,
slug→label, type→theorem env), and FORMAT's three formats
(sources / intermediates / finals).

## 4. Cross-instance build questions a standardized kit must answer

1. **Where do view-local rendering directives live?** neurips puts them in
   manifest row types; ASF hardcodes them in the build script; the
   living-documents seed proposes declared include-filters. Three answers, one
   question.
2. **Which parts of a segment does a view project?** (WN stripping today =
   hardcoded `rindex` truncation in ASF; §H.5 proposes header-name filters.)
3. **How is derived-vs-authored declared?** clobber guards (terminology),
   naming conventions (`.extracted.bib`, `.build/`), "Do not hand-edit"
   banners (LEXICON) — convention everywhere, declaration nowhere.
4. **What notation constraints does multi-renderer targeting impose on the
   substrate?** (single-`$` class of rules; fmt-md's render-equality gate is
   the same concern from the canonicalization side.)
5. **Who owns the shared projection?** (neurips' preamble-ownership split is
   the only instance with an explicit answer.)
