---
slug: build-forced-commitments
type: survey
depends: []
---

# What assembly forced the format to decide

*A comparative look at four live build pipelines over outline+segment corpora, and which model commitments each one forced its corpus to make. The evidence base for the claims about views, projection, derived artifacts, and renderer constraints.*

## Why the build layer is where this gets settled

A verisectorium can hold a great many opinions about views, evergreen segments, and identity without ever testing them, because nothing forces a decision. Assembly forces decisions: something has to say which records appear, in what order, with what number on them, and what happens to the parts of a record that this audience should not see. Each pipeline below made those calls under real pressure — a page budget, a submission deadline, a second render target — and the calls are legible in the tooling rather than in the doctrine.

Four pipelines, surveyed 2026-08-05 against their live trees.

## The four

### ASF monograph — `bin/build-monograph` (three stages, shipping)

Ingest → assemble → typeset. `OUTLINE.md` + `src/<slug>.md` go in; per-segment normalized **chunks** plus an `index.md` assembly manifest come out; the manifest is stitched into one assembled markdown; the PDF is typeset from that. The architectural commitments are stated in the design record (`~/src/arch/asf/msc/markdown-first-pipeline.md`, all three stages landed 2026-05-12), and four of them are load-bearing here:

- **The assembled markdown is the artifact; the PDF is one rendering of it.** Further renderers (HTML, EPUB, JSON-for-LLM) are expected to attach at the assembled-markdown layer without touching the earlier stages.
- **The index is the assembly manifest** — an ordered list of chunks with slugs, kinds, labels, and source hashes.
- **Cross-references are resolved at assembly time, not in chunks**, so a chunk survives reordering and renumbering unchanged.
- **The rendered number is assigned by the assembler**, from outline order, into the chunk's metadata block (`**Label**: 1.1`) — it is nowhere in the segment source.

Header level is the one place a segment's chunk depends on its position: a segment's authored H1 lands at H4 in main matter and H3 in an appendix, so moving a segment between the two regenerates its chunk.

### ASF digests — `bin/build-markdown` (shipping, one recipe)

A second, deliberately separate pipeline over the same segments. A digest is one file, `doc/digests/<name>.md.liquid`, whose YAML frontmatter *is* the build spec (`sources`, `filter`, `output`) and whose body is a Liquid template. The filter selects segments by frontmatter field or by section presence (`has_sections: [formal_expression]`); the template then chooses which *parts* of each selected segment to emit, from a parse that exposes `frontmatter`, `title`, `brief`, and each H2 section by normalized key. Design record: `~/src/arch/asf/msc/build-markdown-design.md`; first live recipe `doc/digests/math-core.md.liquid`.

It was built separate from the monograph pipeline on the judgment that the two shapes diverge sharply after the shared outline walk, and that ~20 lines of duplicated parsing is cheaper than a shared interface fitting neither.

### neurips — `bin/build` over `OUT.<stem>.md` manifests (most build-evolved)

Segments in `<paper>/src/`; a manifest per intended artifact. Paper 01 ships two live manifests (`OUT.confident-agent-neurips-2026.md`, `OUT.review.md`) over one `src/`. The manifest is a markdown table whose columns are `§ | Type | Slug | Title | Stage`; top-to-bottom is assembly order. Verified in `~/src/neurips/AUTHORING.md`:

- **Trimming is manifest selection, not segment editing.** *"trimming is which segments land in this manifest, not editing segments to fit a smaller form"*; *"Page count is observable, not actionable on segments."* The elaborated form, under *Reuse over re-edit*: *"When trimming, prefer to write a new manifest that selects a subset of existing segments rather than edit segments to fit a smaller form"* — with the reason that a tightened proof then propagates to every manifest for free, which matters most while the math is in flux. If a segment does not fit a manifest, **omit it**; do not fork-and-edit.
- **The `Type` column drives structure injection.** The build injects `\appendix` before the first `Appendix`-typed row and `\newpage` before a `Checklist` row; a `Bibliography` row places the references. The column is not descriptive metadata — it is a rendering directive attached to the membership.
- **No manual numbering anywhere.** Section, equation, and theorem numbers are produced by the renderer; the prior workspace's manual numbering *"broke under reorganization."* Appendix headings are authored un-numbered (`## Setup`) and render as A.1, A.2.
- **Single-`$` math is a hard requirement**, because the same segment text feeds both the LaTeX build and OpenReview's MathTeX abstract renderer: *"Authoring discipline that breaks abstract submission breaks the workflow."*
- **Preamble ownership is split by role.** Segment authors may not inject preamble; a missing package or environment goes to the build-pipeline owner, for cross-paper consistency.
- **Derived artifacts are named so their derivedness shows.** The 2026-05-06 refactor moved build output to per-manifest `.build/<stem>/` dirs and replaced the hand-editable `<paper>/refs.bib` with `<paper>/<stem>.extracted.bib`, whose *"naming is explicit-on-purpose so it's clear by construction that it's a build artifact"* (`BUILD-CHANGE.md`). The old file was left inert rather than silently kept working, precisely so nobody would hand-edit a file the build no longer read.

### Paper projects — pandoc concat (behavioral-floor, causal-language, logos)

Order lives in filenames (`src/NN-*.md`); the build is a concatenation with no manifest layer, and therefore no multi-view affordance. Nobody there has needed one. logos carries venue-specific outputs (Synthese LaTeX/Word, Inquiry T&F) that explicitly did not transfer when the shared refs machinery did.

## What the comparison shows

| Question assembly forces | ASF monograph | ASF digests | neurips | paper projects |
|---|---|---|---|---|
| Which records appear? | outline rows | recipe `filter` | manifest rows | every file in `src/` |
| Which parts of a record? | hardcoded (`--public` strips Working Notes) | Liquid template over parsed sections | whole segment | whole file |
| Where do rendering directives live? | in the build script | in the recipe | in the manifest `Type` column | nowhere |
| Where does the number come from? | assembler, into the index | not assigned | renderer (LaTeX) | renderer |
| How is derived-ness declared? | `.build/` gitignored | recipe `output:` field | naming convention + `.build/` | — |
| How many views over one substrate? | one per component (plus digests) | one per recipe | 2 live on paper 01 | one |

Two readings are worth separating. **The convergences**: three independent pipelines put ordering in a view file rather than in the records, and none of them lets a segment carry its own rendered number. **The divergences**: the same question — where do view-local rendering directives live? — has three different answers in one estate, which is the fact a standardized kit would have to absorb rather than legislate away.

The manifest layer is what buys multiple views, and the paper projects are the honest control: no manifest, one view, and no felt need for a second. That is evidence the layer earns its cost only when views actually multiply, not that it is universally correct.

## Method & scope

Read first-hand 2026-08-05 against live trees: `~/src/neurips/AUTHORING.md`, `~/src/neurips/BUILD-CHANGE.md`, the four live `OUT.*.md` manifests under `~/src/neurips/*/`; `~/src/arch/asf/msc/markdown-first-pipeline.md`, `~/src/arch/asf/msc/build-markdown-design.md`, `~/src/arch/asf/PROPOSALS.md` §H, `~/src/arch/asf/FORMAT-TODO.md`, `~/src/arch/asf/bin/` listing and `bin/lib/ingest.rb` (`--public` strip confirmed at the ingest stage). Quotations were located character-for-character in those files on that date.

Scope limits worth stating. This is four pipelines in one estate under substantially shared authorship — convergence between them is weak evidence of independent discovery and should be read as *the same practitioners reaching the same shape under different pressures*, not as multi-author corroboration. The neurips venue pressure (a hard page budget, an external submission system) is the closest thing here to an outside constraint, and it is the source of the sharpest formulations. Nothing here was measured over time; these are design commitments as of one day, some of them shipping for months, some (the proposed within-segment filters) still proposals.

## Working Notes

- The two ASF FORMAT copies gathered into influx (`format-segment-conventions.md`, `format.sop.md`) are the same live file — `asf/FORMAT.md` is a symlink to `asf/doc/sop/format.sop.md`. Worth remembering before anyone counts them as two sources.
- Not surveyed: vivarium's and comproprium's build/render paths, and udon's. A fifth and sixth column would sharpen the divergence row considerably.
- The proposed within-segment filters (`include-headers` / `exclude-headers` / `status-min`) live at `~/src/arch/asf/PROPOSALS.md`, §H item 5 and remain unimplemented as of this survey; the only shipping within-segment projection is the digest template and the hardcoded `--public` strip.
