# fmt-md landscape research — existing tools, actually tested

*2026-07-22. Empirical survey for the fmt-md planning effort. Companion to `unwrap-failure-modes.md` (web-sourced failure catalog) and the parallel requirements report. Everything below was run, not read-about; reproduction paths are given per finding. Test workspace: the session scratchpad (`corpus/` samples + `work/` per-tool runs); all runs on copies, never live corpora.*

## Headline results

1. **dprint's markdown plugin, configured `{ "textWrap": "never", "emphasisKind": "asterisks" }`, reproduced Joseph's own human-approved unwrap byte-for-byte on 2 of 3 git-history ground-truth files** (third differed only by one inserted blank line before a list — a change *toward* the format.sop blank-line standard). It is idempotent, preserved every wikilink, every frontmatter inline `#` comment, and — checked mechanically — **changed zero math spans across all 169 asf `01-aat-core/src` segments**. It is the strongest off-the-shelf candidate, with three sharp edges documented below (verbatim-`markdown` fences get recursively formatted; malformed table rows get cells silently *deleted*; table re-padding churn).
2. **The "broke all tables" mystery tool is almost certainly plain `mdformat` without its GFM plugin** (or a same-class tool): run bare, it flattens every table into a one-line pipe-soup paragraph *and* backslash-escapes all math (`$\hat P_\Sigma$` → `$\\hat P\_\\Sigma$`). It is touted online as a safe CommonMark formatter; the table destruction is exactly the reported symptom. Reproduced directly.
3. **The incumbent `lint-md`'s missing ~40% has a one-line explanation**: its join rule fires only when a line ends in `\w` — so any line ending in `.` `)` `—` `:` `,` is never joined. Sentence-per-line and citation-parenthetical wraps are precisely the lines that end in punctuation. This is sharper than PROBLEM.md's framing: the letter-test on the *next* line is fine; the `\w$`-test on the *current* line is what forfeits most real cases.
4. **No existing tool implements the actual spec.** format.sop's policy is "no fixed-column wraps, but one-logical-chunk-per-line is allowed." Every real formatter tested (prettier, dprint, mdformat, pandoc) normalizes *all* soft breaks away, destroying deliberate clause-per-line prose; every linter tested (markdownlint, rumdl) wraps long lines but never unwraps short ones. Only `mdslw` occupies the policy middle ground (sentence-boundary lines are its fixed point) — and it mangles display math. The policy layer PROBLEM.md hypothesized is genuinely absent from the ecosystem.

## Method

- **Corpora (copies in scratch):** 4 udon-needs files (frontmatter inline `#` comments, wikilinks, wrapped list items/blockquotes); 3 math-heavy asf segments + later all 169 `01-aat-core/src` files; 4 vivarium `.archive` files (genuinely dirty; verbatim-quoted markdown, dense parentheticals); a purpose-built `torture.md` (every stressor class in one file).
- **Ground truth:** asf commit `de1082d` ("Fix many linting bugs and rerun linter", 2026-03-13) provides labeled before/after pairs where the "after" is linter-output-plus-manual-approval. Candidates were run on the `before` and diffed against the approved `after`. (Per Joseph's mid-flight note: current asf files are already clean, so they were used for idempotence/churn/damage checks, not for unwrap scoring.)
- **Oracles:** `diff` vs approved-after; `fmt(fmt(x)) == fmt(x)` idempotence; pandoc `-f gfm+tex_math_dollars -t html` render comparison (weak but automatic); a Python check that the multiset of `$…$`/`$$…$$` spans is unchanged across a run.

## Verdict table

| Tool (version) | Unwraps? | Tables | `$` math | Wikilinks | FM `#` comments | Idempotent | Disqualifier / cost |
|---|---|---|---|---|---|---|---|
| **dprint markdown 0.19 (dprint 0.55)** | yes | kept, re-padded | untouched (169/169 files) | kept | kept verbatim | yes | formats inside ` ```markdown ` fences; **deletes** overflow table cells; joins deliberate clause-lines |
| **prettier 3.6.2 `--prose-wrap never`** | yes | kept, re-padded; overflow cells kept | untouched in all samples | kept | kept (spacing normalized `  #`→` #`) | yes | **forces `*em*`→`_em_`, unconfigurable** — destroys `*[Definition (slug)]*` eq-tags; joins clause-lines |
| **mdformat 1.0 + gfm/frontmatter/myst** | yes | kept (with gfm plugin) | kept (with myst) | **escaped: `\[[w|l]\]`** | kept | yes | wikilink escaping breaks Obsidian; renumbers `2.`→`1.`; plugin-conflict warnings |
| **mdformat bare (no plugins)** | yes | **destroyed → one-line pipe soup** | **backslash-mangled** | escaped | kept | — | the cautionary tale; do not allow near the corpus |
| **pandoc 3.10 gfm round-trip `--wrap=none`** | yes | kept, restyle | **rewritten to `` $`x`$ ``/```` ```math ````** | escaped | **frontmatter deleted** | — | renderer round-trip, not a formatter; disqualified |
| **comrak 0.54 CLI `--width 0`** | no (preserves softbreaks) | kept, restyle `\|---\|` | kept (`math-dollars`) | kept (extension) | kept (`--front-matter-delimiter`) | ~ (injects `<!-- end list -->`) | not an unwrapper; value is as a *library* |
| **mdslw 0.17** | joins, then re-breaks at sentence ends | untouched (good) | **mangles `$$\begin{aligned}` blocks; non-idempotent there** | kept | kept | no | only tool with a real line-break *policy*; math handling disqualifies as-is |
| **markdownlint-cli 0.45 `--fix`** | no | strips cell padding | untouched | kept | kept | — | linter: flags MD013, can't unwrap |
| **rumdl 0.2.40 `--fix` + `MD013.reflow`** | no (reflow only wraps over-long lines) | untouched | untouched | kept | kept | — | Rust/fast/markdownlint-compatible, but no unwrap mode |
| **Obsidian Linter (asf vault)** | — | — | — | — | — | — | **installed but dormant: `data.json` shows 0 of 65 rules enabled, lintOnSave off** — not actually a trusted incumbent; also Obsidian-runtime-bound (no CLI) |

## Evidence highlights (spot-checkable)

**dprint exact match to human judgment.** `agent-environment.md` before → dprint(`textWrap: never, emphasisKind: asterisks`) → `diff` vs the approved after: empty. `model-class-fitness.md`: empty. `recursive-update.md`: one added blank line between `where:` and its list (format.sop-compatible). Prettier produced the identical prose joins but every run differs from approved output by the `_emphasis_` rewrite — prettier has *no option* for emphasis marker, so eq-tags like `*[Definition (agent-environment)]*` become `_[Definition (agent-environment)]_` corpus-wide. That alone removes prettier unless a post-pass reverts markers.

**dprint sharp edge 1 — recursive fence formatting.** Minimal repro: a ` ```markdown ` fence containing `wrapped\nverbatim *content*` came out joined to one line; an identical ` ```text ` fence was untouched. Cause: dprint formats fenced blocks whose language tag matches an installed plugin. Real-world hit: `vivarium/.archive/memory-surfaced-2026-07-13.md` quotes seventeen memory files verbatim in ` ```markdown ` fences; dprint reformatted *inside* the quotes (674 render-diff lines). Mitigations: tag verbatim quotes `text`, or per-block `<!-- dprint-ignore -->`, or a pre-pass guard.

**dprint sharp edge 2 — silent cell deletion.** Input row `| a | b | extra-cell |` under a 2-column header: dprint emits `| a  | b  |` — **`extra-cell` is gone**. Prettier keeps it (extends the delimiter row). Malformed-but-rendering tables exist in wild corpora; any adoption needs a pre-flight table-shape check or render-tree comparison gate.

**dprint sharp edge 3 — churn.** On the already-clean 169-file asf corpus, `dprint check` flags 125 files (~2,800 diff-lines), dominated by table re-padding (the corpus uses compact `|---|` delimiters; dprint aligns all columns — visually fine, git-history churn real; no config to leave table layout alone) plus blank-line normalization (26 files are whitespace-only diffs). Adoption implies a one-time reformat commit.

**The linter class can't help with sub-problem 1.** markdownlint and rumdl both model line length as "too long" (MD013), never "wrongly short"; rumdl's `reflow` re-wraps over-long lines to the limit but leaves short wrapped lines alone even at `line-length=100000`. Blank-line normalization (sub-problem 2) they do fine — as does dprint.

**Math rules (sub-problem 3): nothing touches them, in the good sense and the bad.** No tested tool applies the discovered cross-renderer rules (`\vert`, `\lt`/`\gt`, `\ast`, brace-removal before `_`, `$$` blank-line discipline) — and the well-configured tools at least don't *damage* math. The incumbent `lint-md` remains the only implementation of these rules anywhere. Composition test: **dprint → `lint-md --fix` → dprint reaches a fixed point** on a fixture carrying spaced-`$`, raw angles, `\begin{align}`, and hard wraps — the two tools' jurisdictions (structure/whitespace vs. math-span content) don't overlap, so they compose cleanly.

## The Rust build-on landscape (if adopt/adapt isn't enough)

- **`dprint-plugin-markdown`** (Rust, MIT) — not just a tool but a *fork/extend base*: the closest existing code to the target behavior, already carrying the unwrap engine that matched human judgment. Missing pieces would be patches, not greenfield: emphasis-marker preservation exists (`emphasisKind`); table-layout preservation, fence-recursion opt-out, overflow-cell safety, `$`-math opacity, and the clause-per-line policy would be additions. Its parser is pulldown-cmark-based.
- **comrak** (Rust library) — the strongest *parser* substrate: AST with source positions and first-class extensions for exactly this corpus's foreign regions — `math-dollars`, `wikilinks-title-after-pipe`, `front-matter` — plus a CommonMark printer. A custom formatter over comrak's AST starts with math/wikilinks/frontmatter already opaque, which is the load-bearing requirement PROBLEM.md names. Caveats: its printer normalizes (`<!-- end list -->` injection, table restyle), so byte-minimal output means writing your own emitter over the AST + original source spans.
- **markdown-rs (`markdown` crate)** — CommonMark + MDX/GFM/math extensions, "every byte accounted for, with positional info" — the other viable substrate; no formatter included.
- **pulldown-cmark** — event stream, fastest, but no CST and its `pulldown-cmark-to-cmark` round-trip is lossy; fine for oracles, weak for formatting.
- **mdslw** — small (pulldown-cmark-based) and the ecosystem's only *sentence-boundary line policy* implementation (suppression lists for abbreviations etc.); worth mining for policy design even though its math handling fails here.
- **rumdl / mado** — Rust linter frameworks with fix infrastructure and markdownlint rule compatibility; a plausible home for the *extensible rule surface* requirement (rumdl already supports inline enable/disable comments, per-rule config, and a fmt mode), but the unwrap engine would still have to be contributed.
- Generic CST machinery: [cstree](https://crates.io/crates/cstree) if a fully lossless homogeneous tree is wanted; likely overkill given comrak/markdown-rs positions + source text.

Sources consulted: [markdown-rs](https://github.com/wooorm/markdown-rs) · [rumdl](https://github.com/rvben/rumdl) · [rumdl.dev](https://rumdl.dev/) · [cstree](https://crates.io/crates/cstree) · [lib.rs/markdown](https://lib.rs/crates/markdown) — plus the issue links already cataloged in `unwrap-failure-modes.md`.

## What this implies for the adopt/adapt/build decision

The honest "existing tool + config" answer exists and is stronger than expected, but it is a *pipeline*, not a tool, and it has a policy hole:

1. **Near-term serviceable:** `dprint fmt` (`textWrap: never`, `emphasisKind: asterisks`) for unwrap + blank lines + structure, **then** the existing `lint-md --fix` for math rules. Proven fixed-point-stable; matched human ground truth on real history. Requires: verbatim-markdown-fence guard, malformed-table pre-check (or a render-diff gate like the pandoc-html oracle used here), and acceptance of one-time table-padding churn. It *will* join deliberate clause-per-line prose — acceptable for dirty-corpus cleanup passes (the March ground truth shows Joseph approved exactly that), not acceptable as a standing always-on formatter under format.sop's stated policy.
2. **The build case therefore reduces to the policy layer** (+ the sharp-edge guards): the one thing nowhere in the ecosystem is "join column-wrap debris, preserve deliberate logical-chunk lines, flag when unsure." Note the policy itself is unstated: the ground-truth pairs show *sentence-per-line prose being approvingly joined* in March 2026, while format.sop §Line Wrapping licenses clause-per-line — the requirements report should force a crisp statement (a workable candidate: a line break is preserved iff it falls at a sentence/clause boundary *and* the block is majority-boundary-broken; otherwise joined — mdslw's fixed-point behavior is an existence proof that boundary-detection is tractable in Rust).
3. **If built:** dprint-plugin-markdown fork vs. comrak-based fresh implementation are the two serious shapes; both fit the udon/Rust leaning. The fixture set is already seeded: the torture file, the `de1082d` before/after pairs (more exist in that commit: 14 files), the vivarium fence-quoting file, and every bullet in `unwrap-failure-modes.md`.

## The agent-written reframe, tested against the corpora

Joseph's mid-planning reframe (PROBLEM.md §The unifying reframe) — that the target is *markdown as pre-wrapped by frontier agents writing files* — is directly measurable on the dirty corpora, all of which are agent-written. Prose-line length histograms:

- **udon-needs:** sharp unimodal peak at 70–74 chars (154 of 343 prose lines in that one bucket; 60% of all lines in 65–79) — a characteristic fill column.
- **vivarium archive:** same shape, peak at 75–79 (268 of 1262; ~44% in 70–84) — a slightly different fill column, consistent with a different model/era.
- **asf `de1082d` pre-fix pairs:** *flat* distribution — because that corpus's disease is **sentence-per-line**, not column wrap.

So the reframe holds, and it splits the disease into two classes with different, cheap detectors: **(a) column-wrap** — lines cluster just under an inferable per-block/per-file fill column, break at word boundaries mid-clause → join with high confidence (the fill-column mode is a statistical prior no tested tool uses, and it is the strongest single lever available to a built policy layer: infer the column, join only lines whose break is explained by it); **(b) sentence-per-line** — flat lengths, lines end at `.`/`;`/`:` — which is precisely the form format.sop *licenses*, so class (b) is a policy question (canonicalize vs. preserve), not a detection question. The incumbent's `\w$` rule accidentally treats class (b) as protected — the March ground truth shows Joseph nevertheless approved joining it in segment prose. The requirements report should rule on class (b) explicitly; class (a) is uncontroversial.

## Corrections and adjacent findings (brief feedback)

- **PROBLEM.md's "Possibly relevant: obsidian-linter … one existing tool already trusted enough to sit in the vault"** — misleading as stated: it is installed but has zero rules enabled and lint-on-save off. It is shelfware, not an incumbent, and being Obsidian-runtime-bound it can't serve the agent/CLI audiences anyway.
- **PROBLEM.md's root-cause hypothesis is confirmed with one refinement:** the failures observed were less "no real parse" (three tested tools parse correctly) than (a) *no policy layer* over soft breaks and (b) *math/wikilinks as unknown regions* in most parsers. comrak is the notable exception on (b) — the foreign-region problem is already solved at the parser level in the Rust ecosystem.
- **The incumbent's `\w$` join-condition** (line 475 of `bin/lint-md`, `re.search(r'\w$', rstripped)`) is the single highest-leverage known bug if any interim patching is wanted before fmt-md lands.
- **Ground-truth mining is cheap and underused:** commit `de1082d` alone yields 14 labeled pairs; other lint-then-manual commits likely exist. A fixture harvester over asf history would give the future test suite real human-approved targets rather than synthetic ones.

## Reproduction

All artifacts live in the session scratchpad: `corpus/` (torture.md, udon/, asf/, hist/ before-after pairs, viv/) and `work/` (per-tool output dirs, install logs, dprint.json configs). Tools installed this session: mdformat(+gfm,frontmatter,myst via uv), comrak 0.54 (cargo), dprint 0.55.2 (brew), mdslw 0.17.2 (cargo, from git — not on crates.io), rumdl 0.2.40 (brew). The dprint config that matched ground truth:

```json
{ "markdown": { "textWrap": "never", "emphasisKind": "asterisks" },
  "plugins": ["https://plugins.dprint.dev/markdown-0.19.0.wasm"] }
```
