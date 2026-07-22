# fmt-md model workspace — wrap vs phb classifier

*Skeleton 2026-07-22. Feature inventory: [`../research/break-features.md`](../research/break-features.md). Sampling/training design: Joseph's, forthcoming — this directory only makes the corpus regenerable and measured.*

## Layout

- `training/<provenance>/…*.md` — extracted corpus, **gitignored**; regenerate with `./extract`.
- `MANIFEST.tsv` — committed; one row per extracted file: where it came from (repo/commit/session), and its **measured** disposition stats. Nothing about a file's class is assumed from location — see below.
- `extract` — the regeneration script.

## Provenance dirs and what each donates

| dir pattern | source | donates |
|---|---|---|
| `<repo>@<commit>/` | git version of a repo tree (e.g. `udon-v2@cc389f9/` pre-cleanup) | breaks labeled by measurement + marker |
| `<repo>@first/` | each file at its *first* commit (often still hand-wrapped) | old-era negatives — subject to the measurement rule |
| `claude-chat/<session>/<n>.md` | assistant **text blocks** from `~/.claude/projects/**/*.jsonl` | **phb-by-provenance**: chat UIs render bare newlines, so models emit intended breaks with no marker — 10,584 blocks surveyed, zero trailing-space hard breaks. This is the convention-unaware positive class the corpus lacks (leakage guard §4) |
| `claude-write/<session>/<n>.md` | `Write`-tool `.md` payloads from the same jsonl | **wrap-by-provenance**, *paired within-session with the chat blocks* — same model, same day; ~half of 666 surveyed payloads measurably wrapped |
| `vivarium-archive/` | live `~/src/archema-io/vivarium/.archive` | mixed hard cases (citation parentheticals) |
| `asf@HEAD/` (01-aat-core/src) | canonical segment corpus | the **no-op anchor**: phbs ≈ 0 (metadata lives in frontmatter; cadence headers absorb pseudo-headers — Joseph 2026-07-22), join_pct 0.001; any phb-flag inside it is a false positive |
| `tst-planning-analysis/` | `~/src/_core/tst/planning/analysis` (961 files, 2025-08 era agent output) | **re-assessed after context audit**: the 1,881 markers are ~all accidental trailing whitespace (569 on list items, 223 inside code fences, zero label-lines) — *not* phb gold. Actual value: the only clean, **math-dense**, asf-adjacent population at scale — domain-matched no-op corpus + math-rule fixtures; also a distinct 2025-08 model-era for E2. Standing lesson: **marker ≠ intent; context-filter every marker population** (see break-features guard §1). Rest of `~/src/_core/tst/` still a reservoir |
| `asf@de1082d^/` + `asf@de1082d/` | the reworked pair: pre-reflow vs linter+human-approved | human-adjudicated joins (negatives) + kept breaks (positives); note the coarse D-gate cannot tell these apart at file level (sentence-per-line lines are long) — the round-trip delta metric is the right instrument here |

## The two-population structure (Joseph, 2026-07-22 — the load-bearing framing)

*"LLM file output gives us perfect cases of 'needs unwrapping and has pseudo-hard-breaks'; chat markdown gives us perfect 'already unwrapped and has pseudo-hard-breaks.'"* Formally: chat text is a **pure positive sample** (within-paragraph breaks ≈ all intended; models never wrap in chat and never emit markers there — 10,584 blocks, zero trailing-space breaks), while file output is a **wrap-dominated mixture** (mostly mechanical wraps with the same author's intended breaks scattered through). Pure-positive + mixture is the **PU-learning** setting (positive-unlabeled; Elkan & Noto 2008): class-prior estimation and calibrated classification *without pure negatives* — which is exactly right, since instinct-wrapped text can never be certified wrap-only. The within-session pairing (chat and Write payloads from the same model, same day) minimizes the distribution shift that generic PU setups suffer.

**Why the ambiguity class exists at all (Joseph, 2026-07-22):** *"I've seen deliberate `  \n` so rarely that I had completely forgotten it was a thing."* CommonMark's official intended-break mechanism failed adoption so thoroughly that even the most convention-aware author in this ecosystem doesn't use it — intent is expressed by bare newlines universally, which is exactly what makes wrap-vs-phb undecidable from syntax. Corollaries: (a) corpus markers are an *agent-only dialect*, sparse even there (the verified-genuine udon ledger markers are Claude-written) — marker labels are auxiliary, chat-provenance is the primary positive class; (b) the lint deliverable closes the loop — when a phb block is detected, the fix is inserting the markers nobody remembers to write, translating bare-newline intent into the form renderers honor (the CORE.md repair, mechanized).

## The measurement rule (Joseph, 2026-07-22)

Provenance is *not* trusted for class labels: in asf even `msc/`/spike/audit files often got formatted eventually, so "old = wrapped, recent = clean" is unreliable. Instead the extractor measures every file version (width percentiles, variance, hard-break count — the D-features) and records the stats in MANIFEST.tsv. Training rows are drawn only from versions whose measured disposition is high-confidence; ambiguous versions are held out or used as hard evaluation cases. The stronger per-file boundary, when needed: detect the *reflow transition commit* by diff shape (deletions ≫ insertions with joined-line structure) rather than assuming first/HEAD dispositions.

## Generation (the controlled-label source)

Two harnesses, both feeding `training/generated-*/` through the same measurement rule:

- **Local ollama** (free, endless): topic × doc-type prompts via the API. Pilot finding (2026-07-22, llama3.2:3b): the **no-wrap instruction works reliably** (p90 ≈ 220 both trials) — a dependable on-demand source of convention-unaware *positives*; but **instinctive wrapping is inconsistent outside file-writing contexts** (one default doc wrapped at ~58, the other came out clean) — confirming Joseph's mechanism: manual wrapping is a *file-writing* instinct, not a chat one, and an API text completion is neither. So ollama defaults are usable only after measurement; don't assume the disease.
- **Sonnet fleet via Workflow** (`gen-md-corpus`): parallel writers each asked to Write a realistic doc (memo/tutorial/runbook/postmortem/ADR/…) to a file — a genuine `Write`-tool context, so the wrap instinct engages — half with the no-wrap instruction, half untouched. Topics/genres varied per Joseph: an endless source if you keep finding things for them to make.

Instructed-to-wrap would also be possible but is a *different distribution* than instinct-wrap; prefer instinct (file-writing contexts) for negatives and instruction only for positives.

**Measured instinct rates (2026-07-22, naturalistic task framing — markdown never mentioned):**

- **Opus wraps ≈ 2× Sonnet** on matched tasks (6/11 vs 3/12) and wraps genres Sonnet leaves clean (RFC, incident report, glossary) — Joseph's hunch confirmed. Both wrap ADR and research-summary reliably (~72–85 col): dependable disease elicitors across Claude tiers.
- Genre dominates framing: explicit "write a markdown document" vs task-only framing made no per-genre difference.
- Small local models (hermes3:3b especially) mostly emit *mixed* structure — usable only through the measurement gates.
- **Fable marker-dialect probe** (10 fresh instances, phb-*eliciting* genres — memo heads, sign-offs, contact blocks): **zero markers emitted**, disconfirming the recalled marker habit for uncontaminated instances. Intent split 5/5 between **bare newlines** (12 label-run sites that render run-on — fresh gold-defect specimens of the deployment-target class, on demand) and **restructuring** (blank-separated lines/lists/tables). 2/10 wrapped (~75–78). Context-salience variant (does discussing rendering flip the dialect on?) is an open, testable follow-up.
- **Two-phase pipeline** (context-aware topic-gen → blind generators → independent labelers, Joseph's isolation design; `LABELS-sonnet-2026-07-22.json`): labeler-vs-geometry agreement high where it matters (the one measured-wrapped doc is the one labeled wrapped, at the right column); labelers read the coarse "mixed" gate as clean — a gate-calibration datum, not a disagreement. **14 of 24 docs carry labeler-identified deliberate paragraph-internal breaks** — dominantly metadata key-value stacks (To/From/Date heads, postmortem field blocks): a naturally-occurring, high-frequency phb genre, with line-level third-source labels.

## Reservoirs not yet extracted

- **memorata3** (postgres, `psql-18 -d memorata3`): 67,133 md `path_records`, content-addressed (`content_id`, sha256, birthtime/mtime, jsonb metadata) — sampling breadth beyond the three touched corpora, and free dedup (content-addressing collapses provenanced copies).
- The R35 generation harness (prompt battery × model roster) — for models underrepresented in `~/.claude` history.

## Label-noise audit (2026-07-22, seeded random sample, hand-classified)

48 paragraph-internal breaks (seed 20260722; 30 udon@cc389f9, 18 vivarium-archive), each read in context — `AUDIT-2026-07-22.tsv`. Findings:

- **vivarium: 18/18 genuine wraps** — cleaner than feared.
- **udon: of 17 genuine paragraph breaks, 16 wrap + 1 phb (~6% contamination)** — and the one phb (an unmarked event-stream listing) sits in a raw session transcript, i.e. in material the live tree's `.fmt-mdignore` already excludes.
- **13/30 udon samples were sampler leakage, not label noise**: frontmatter, code in transcripts, list-item boundaries — all things the crude line-based audit sampler sees but a comrak-parse-based extractor excludes structurally. The noise Joseph worried about is real but *localized*: apply the parse-based break definition plus the live exclusion patterns (copies/, session-vault) to the git snapshot, and measured contamination drops to ≈6% (U) / ≈0% (V) — and the PU frame absorbs that rate rather than requiring zero.
- V-12 confirmed the per-line-indistinguishable class in the wild: a sentence-boundary break inside a uniformly-wrapped paragraph — only block context decides it.

Per Joseph's outlier method: these audited rates become the population priors, and files whose feature aggregates sit far from their population's audited profile get flagged for the same hand treatment rather than trusted.

## Standing rules

- The five real defects from the 2026-07-22 sweep are the gold test set; they never enter `training/`.
- Everything here is regenerable; if a source moves, fix `extract` and the manifest, don't hand-curate the corpus.
