<!--
  Verisectorium notes gather — extract, not full-file authority.
  Provenance: arch/asf/doc/sop/agents.sop.md §§ Theory Structure + File Organization
  (lines ~66–86 and ~267–282; also respectful-pedagogy / Feynman OUTLINE-preamble
  material from Working Conventions ~249–251)
  Copied: 2026-08-05
  Full agents.sop is agent-onboarding for the whole ASF project.
  Do not edit here expecting to update the live original.
-->

# Extract: agents.sop — theory structure & file organization

*From `arch/asf/doc/sop/agents.sop.md`. The load-bearing verisectorium bits:
slug = identity, ordering in OUTLINE, component = OUTLINE + src/, outline
preambles as framing layer (respectful pedagogy).*

---

## Theory Structure

## Theory Structure

Claim segments live in `{component}/src/` directories. **Each file is like a high-level proof step** — one move per file. Given what came before, this one thing follows, or is defined, or restricts scope.

**File identity and ordering:**
- **Filename = slug**: `01-aat-core/src/{slug}.md` or `02-tst-core/src/{slug}.md`. No numbering in filenames.
- **Slug form is `{role-prefix}-{subject-noun}`** — the role prefix is derived mechanically from the segment's `type:` frontmatter; the subject-noun names what the segment actually defines. Run `bin/align-slug SLUG` to align a single segment; `bin/align-slug --all` to sweep the repo. No-op if already aligned.
- **Slug role-prefix mapping.** `bin/align-slug` derives the prefix from the segment's `type:` via its `TYPE_TO_PREFIX` table — the single source of truth (edit the constant + re-run `--all` to change the project-wide mapping). It also strips a redundant trailing `-{type}` from the subject-noun (`bias-bound-derivation` → `deriv-bias-bound`). (`bin/lint-outline` keeps a separate, more-aggressive prefix table for graphviz node labels — intentionally distinct.)
- **Ordering lives in OUTLINE.md files**, not in filenames. The slug is the stable identity; the linearization will change.
- YAML frontmatter: `slug`, `type`, `status`, `depends` (list of prerequisite slugs). See `FORMAT.md` for details.
- Cross-component dependencies use the same slug system — TST segments reference AAT slugs directly (e.g., `#post-temporal-optimality`).

**Cadence per file** (see `FORMAT.md` for full spec):
1. YAML frontmatter (slug, type, status, depends)
2. Title
3. One-sentence summary
4. Formal Expression (with equation-level tags)
5. Epistemic Status paragraph
6. Discussion (interpretation, connections — brief)
7. Working Notes (optional — active development questions, removed at `candidate` stage)


---

## Working Conventions (OUTLINE framing excerpts)

Each segment's `## Findings` Brief field aspires to the **Feynman criterion**: *if you can't explain it simply, you don't understand it yet.* The benchmark is whether a thoughtful non-specialist can re-derive the qualitative claim from the everyday analog the Brief reaches for, *without* seeing the symbols. Alan Walton's bathtub gloss of the persistence condition (water = belief-reality gap; faucet = rate of change in reality; drain = learning rate; bathtub size = how wrong we can be while still keeping up; overflow when faucet outpaces drain at full) is the canonical example — and notably, it came from a sympathetic outside reader working it out for himself on first encounter, which is the diagnostic to aim for. The same aspiration governs the README, OUTLINE preambles, and any pedagogical or casual-curious-reader-facing material; the Brief field is where the aspiration is institutionalized in the schema, but the principle is general. See `FORMAT.md` §Findings — Brief for the schema-level statement. The standard is genuinely high — most segments do not yet meet it, and reaching it for a given finding is non-trivial work that often produces the Brief *last*, after the formalism stabilizes enough that the load-bearing structure becomes legible to plain language.

**Respectful pedagogy — the active posture, and the ordering discipline.** Joseph named this direction (2026-05-14): the monograph prose is moving toward *respectful pedagogy* — "we have to build mental models that scaffold the math and segments to enable comprehension where the math is thick but the findings important." This raises the Feynman-criterion from a Findings-Brief-local aspiration to an active posture for *all* framing-level prose, and adds an **ordering discipline**: framing-level material (OUTLINE preambles, README, paper/chapter introductions) should lead with the mental model — a "preamble to the preamble" — *then* give the precise structure as a second layer. The scaffold comes first and stands alone; it is owed to the reader, not optional ornament ("respectful" is load-bearing in the name — cf. the Alan-Walton first-human-review thread where academic-register prose lost a sympathetic mathematician-practitioner's sustained attention). Worked example: the `01-aat-core/OUTLINE.md` "Reading AAT" preamble is two layers — *the mental model first* (the measuring-stick / stability-certificate scaffold) then *the precise structure* (certificate-and-facets with segment refs). The honesty constraint is sharp here because framing prose is auditor-visible and priming-heavy: the analog must be **isomorphic, not merely evocative** — a reader perturbing the analog must get predictions that hold against the formalism (measuring-stick = the metric; flat direction = rank-deficiency; can't re-graduate off it = Sylvester's law; survives coarsening but leaks = Schur complement plus a memory term). Scaffolding that overclaims is worse than none. Keep Layer 0 to one tight paragraph — "scaffold then precise," not "explain everything twice."

---

## File Organization

**Root (read `PRACTICA.md` first — the navigator, auditor-safe).** `CLAUDE.md` / `AGENTS.md` / `GEMINI.md` symlink to this file · `OUTLINE.md` top-level assembly · `README.md` (+ auditor variant `README-auditor.md`) public snapshot, *auto-generated* · `FINDINGS.md` novel-results catalog, *auto-generated* · `TODO.md` tactical items **(priming-heavy — auditor-hidden)** · `TERMINOLOGY-TODO.md` naming-cycle execution queue · `PROPOSALS.md` architectural-moves portfolio **(auditor-hidden)** · `CHANGELOG.md` cycle record (2026-04-24→) · `LOG.md` frozen pre-2026-04-24 archaeology · `FORMAT.md` → [`format.sop.md`](format.sop.md) · `NOTATION.md` symbol reference · `LEXICON.md` prose vocabulary, *auto-generated* · `JOSEPH-TODO.md` short queue of decisions genuinely reserved for Joseph — pointers only, work items stay in their home trackers **(tracker-class — auditor-hidden)** · `TODO-big-picture.md` forcing-bound-spine correction + census, parent navigator PRACTICA **(self-declared priming-heavy — auditor-hidden)** · `TST-IDEAS.md` candidate-material substrate from the 2026-05-21 TST corpus-mining cycle — a substrate, not a TODO list **(judged-distinctive content — auditor-hidden)** · `HISTORICAL-CONTEXT.md` long-form positioning & lineage document **(auditor-hidden per the audit AVOID list; the audit-safe distillation is README-auditor's *Position & Lineage*)** · `BIBLIOGRAPHY-TODO.md` citation-system discipline/tooling workstreams (parent navigator `FORMAT-TODO.md` Workstream A) · `CURRENT-VOL1.md` root copy of the assembled Volume 1 monograph (placed 2026-05-16 for discoverability; the segments are the source — treat as a build snapshot, not a place to edit theory). The layered doc model (PRACTICA → TODO → PROPOSALS; CHANGELOG/LOG = history; README = human-facing; this file = agent-onboarding) is spelled out in *Current Priority* above.

**Components** — each is an `OUTLINE.md` + a `src/` of slug-named segments: `01-aat-core/` (AAT) · `02-tst-core/` (TST) · `03-llm-core/` (logogenic) · `04-eli-core/` (logozoetic).

**Supporting:**
- `bin/` — build / lint / generation / slug tools. *Language, descriptively rather than as a rule:* the tree is a historical mix — the 2026-04-26 convention was Ruby for internal process scripts and Python for community-facing artifacts (sims, reproducibility code), which is why `lint-md` / `lint-outline` / `md2context` are Python and most of the rest Ruby. The current stance (2026-07-16): durable, shared, content-parsing tools are trending **Rust on udon-core** as they migrate to the Archema program level (`~/src/arch/utils/` — see the program `TODO.md`), which gets the udon parser without maintaining Ruby/Python bindings; migrated tools are fresh implementations by thoughtful agents (existing sources and git are reference, not a spec to replicate — bug-for-bug parity was explicitly declined as unprincipled); community-facing artifacts still tend toward Python for audience reasons; and one-cycle disposable scripts are whatever is fastest to write. Judgment over rule — pick for audience, durability, and toolchain fit. (Superseded `build` / `build-tex` in `_obs/`.) `bin/lint-outline` with no arguments lints **all four** component OUTLINEs with cross-component dependency resolution; deliberate ordering exceptions (e.g. the introduced-before-used meta-segment placements) live in per-component `OUTLINE-accepted.md` whitelist files (slug-pair-keyed, reason + record citation per row — still printed, marked accepted, exit stays green); frontmatter↔OUTLINE stage drift is reported warning-only (stage is process-state, not epistemic strength — see [`format.sop.md`](format.sop.md) §stage). `bin/lib/` + `mono/` + `bin/build-monograph` are the three-stage markdown-first monograph pipeline (ingest → assemble → typeset; `--target scrbook`|`kaobook`) — architecture + chunk-format contract in [`msc/markdown-first-pipeline.md`](../../msc/markdown-first-pipeline.md), state in [`FORMAT-TODO.md`](../../FORMAT-TODO.md).
- `terminology/` — **source-of-truth for prose vocabulary**: per-term entries + append-only decision events, `bin/term` CLI, `LEXICON.md` generated from it; multi-agent-safe by per-entry design. See [`terminology/README.md`](../../terminology/README.md).
- `doc/` — long-lived process docs, including **`doc/sop/`** (the SOPs — index at the top of this file) and `doc/DOMAINS.md` (domain-instantiations working draft, exploratory status — canon-cited from `01-aat-core/INTRODUCTION.md` as a breadth reference).
- `spikes/` — research spikes (reasoning trails); entry point [`spikes/INDEX.md`](../../spikes/INDEX.md). `spikes/PROPOSED.md` (+ `-ADVANCED`/`-MISC`) is an *optional* spike-idea repository, **not** a registry — freshness + mutual-link are the only disciplines, not completeness (see [`spikes.sop.md`](spikes.sop.md)).
- `audits/` — audit-cycle outputs; routing status [`audits/STATUS.md`](../../audits/STATUS.md), process in the [audit SOP](audit.sop.md). **Standing, non-optional gate: before any processing / `.integrated/` move / deletion of an `AUDIT-WORKING-*` dir (de-novo first-encounter "gold"), consult Joseph and decide _with him_** ([`audits/README.md`](../../audits/README.md)). `ADJUDICATION-WORKING-*` dirs carry no such gate.
- `msc/` — other working artifacts (session residue; `naming/`, `reflections/`). `_obs/` — superseded docs (archaeology).
- `ref/` — external prior-art PDFs ([`ref/INDEX.md`](../../ref/INDEX.md)) + internal scaffolds (incl. `agentic-tft/` pre-AAT bridge work). The scaffolds are working aids, **not** segment-citable sources — canon cites the *external* prior-art directly, self-contained per *Prior art integration* above. (G2/G3 reconciliation tracked in `INTEGRATION-CLEANUP-TODO.md`.)

**Sibling projects** (not in this repo, but relevant): `~/src/_core/tst/` prior TST corpus (most absorbed into `02-tst-core/`; the 965 vault analyses remain) · `~/src/shoshin/` PROPRIUM runtime prototype (Python skeleton; only code attempt at the architecture) · `~/src/firmatum/` PROPRIUM ontology + architecture source (defines what an ELI is) · `~/src/embeddings/` epistemic-hedging-geometry experiments ($\rho = 0.991$ vs psychometric data; supports the logogenic claim) · `~/src/arch/vivarium/` the authored-world calibration laboratory (supporting project as of 2026-07-04: a principled simulated planet where AAT quantities have ground truth by construction — what it offers immediately / short-term / future is in [`../vivarium.md`](../vivarium.md); its side of the bridge is `~/src/arch/vivarium/ETHICS.md` — the former `ASF.md` was dissolved 2026-07-11, moratorium there, the rest re-homing).
