# Dive: the asf lineage — segment kinds in the wild (2026-08-10)

*One of four parallel dives (this one: `~/src/arch/asf/` + the refs/relata lineage). All specimens verified at live sources this date; no influx copies quoted. Reader: the coord + Joseph synthesizing the four dives into taxonomy segments — so each specimen carries path, verbatim structure, the register vocabulary it actually uses (or "none"), and maintainer. Misfits against the epistemic map's carve (truth-apt / choices / directives / references / decisions) are collected in §M at the end; they are the yield.*

---

## 1. The claim-segment corpus (asf `src/`, four volumes)

**Kind as the corpus names it:** *segment* (typed claim file). Maintainer: Joseph + agents, gated by `FORMAT.md` promotion machinery.

**Frontmatter specimen** (`~/src/arch/asf/01-aat-core/src/def-mismatch-signal.md`):

```yaml
---
slug: def-mismatch-signal
type: definition
status: axiomatic
depends: [form-agent-model, def-observation-function, def-action-transition]
stage: deps-verified
---
```

Section cadence (FORMAT.md §Document Cadence): Title → one-sentence summary → `## Formal Expression` (with equation-level tags) → `## Epistemic Status` → `## Discussion` → `## Findings` *(optional)* → `## Working Notes` *(optional)*.

**Three orthogonal-by-design vocabularies on one file:**

- `type` (WHAT KIND, 19 values, each a terminology entry): postulate, definition, scope, formulation, derived, result, corollary, hypothesis, normative, empirical, observation, discussion, measurement, proposed-schema, derivation, worked-example, detail, sketch, aside. Note the deliberate humility policy stated in FORMAT.md: "`postulate` (not `axiom`), `result` (not `theorem`), `derivation` (not `proof`)" — the type vocabulary itself encodes a claim-posture stance toward the external world.
- `status` (EPISTEMIC STRENGTH, 8 values): axiomatic, exact, robust-qualitative, heuristic, conditional, empirical, discussion-grade, sketch. Explicit prohibition: "Do NOT use 'Solid,' 'Confident,' or 'Plausible' as tier labels."
- `stage` (PROCESS, 8 values, gate-advanced): missing, old, exploratory, draft, deps-verified, claims-verified, format-clean, candidate. Gates: dependency audit → content review → mechanical review → notes disposition. **Load-bearing caveat in FORMAT.md itself:** stage lint runs "as warnings only, never gate failures: the stage layer is known to go stale quickly … and is currently ignored in practice" (Joseph 2026-07-14, promotion methodology under reconsideration). A process ladder that its own spec declares decaying is evidence about process-state axes generally.

**Filename-prefix census** (`ls src | sed 's/-.*//' | sort | uniq -c`): vol 1 (171 files): deriv 32, disc 25, der 24, def 21, result 11, form 11, impl 9, scope 8, hyp 7, example 4, obs 3, detail 2, plus singletons (strategy, sketch, schema, post, norm, emp, img, internal, old, and unprefixed intro files like `causal-access-intro.md`). Vol 2 (02-tst): **43 of 71 files are `old-*`** (see §M5). Vol 3: scope 6, impl 4, disc 4, result 2, obs 2, hyp 2, der 2, def 1. Vol 4: def 7, der 5, scope 4, obs 2, hyp 2, norm 1, deriv 1. Prefix vocabulary is larger than the type table (impl-, img-, internal-, strategy- have no type row) and both `der-` and `deriv-` are live (derived-claim vs backing-derivation).

**Sub-file epistemic grain — equation-level tags** (FORMAT.md): `*[Definition (slug)]*`, `*[Derived (slug, from …)]*`, `*[Derived (Conditional on …)]*`, `*[Hypothesis]*`, `*[Empirical Claim]*`, `*[Formulation]*`, `*[Assumption]*`, `*[Postulate (slug)]*`. Register exists at *equation* grain, not just file grain (§M13).

**Findings sub-schema** (segment section → generated root `FINDINGS.md` via `bin/extract-findings`): per-Finding fields Brief / Impact / **Novelty Claim** (posture words: synthesis / differentiation / novelty / transfer / recognition — "do not force into a closed-set label") / Related Work (each line: citation + published-year + **found-date** + relationship) / **Search Log** (dated entries with a *status* and "why this depth was right at this point" — a freshness/effort register for prior-art search, §M8).

**Epistemic Triage + Max attainable** (FORMAT.md §Epistemic Triage): three diagnostic questions; **Max attainable status is prose convention, not a field** — "note it in the segment's Epistemic Status paragraph: *'Max attainable: [status]. Currently [status] because [reason].'*" Compare the template's Max-as-first-class-field: asf is the ancestor with the weaker mechanization. Also the **three rings** (inevitability core / canonical formulations / empirical-heuristic) — explicitly "not part of segment frontmatter — it's an analytical stance the reviewer takes" (§M7).

**Cadence exemption:** chapter-intro and most `disc-*` segments are exempted from Formal Expression / Epistemic Status "until dedicated norms for those file kinds are worked out" (Joseph 2026-07-14) — an in-canon admission that the claim-segment mold is wrong for expository kinds, with the exemption itself recorded so free-form ≠ drift.

## 2. OUTLINE machinery

`~/src/arch/asf/OUTLINE.md` (root, assembly index over four volume OUTLINEs) + per-component `NN-*/OUTLINE.md`. Component-outline row shape (`01-aat-core/OUTLINE.md`):

```
| §   | Type       | N | Tag                              | Claim                      | Stage           |
| I   | Definition |   | [#def-agent-environment](src/…)  | Agent-environment boundary | deps-verified   |
```

Registers used: Type + Stage (projections of frontmatter; `bin/lint-outline` checks consistency, warnings only). Structural heads are `## *Part*` / `### *Chapter*` with italic role-markers; embeds via `![[INTRODUCTION]]`; figures carry `{#fig-… caption="…"}` attributes. **Marginalia register:** live HTML comments in the outline body — `<!-- remove -- …two sentences, the first being an incomplete sentence… -->`, `<!-- salvageable -- … -->` — inline editorial judgments with their own micro-vocabulary (remove / salvageable), maintained ad hoc (§M17). Known live staleness class: OUTLINE `missing` rows for segments that exist at draft (documented incident in PRACTICA §item 4, 2026-06-30 — "the gap was navigator staleness, not absent content").

## 3. The terminology store (directory-as-table, decision-events)

`~/src/arch/asf/terminology/` — 176 entries, 149 decision dirs. Maintainer: agents propose, Joseph decides; `LEXICON.md` generated by `bin/term render`; permissive extractor / strict linter split ("render show[s] the current state of the world even when entries are mid-edit").

**Entry frontmatter specimen** (`entries/aporia.md`):

```yaml
slug: aporia
schema_version: 1
term: aporia
name: Aporia (Ἀπορία) (productive perplexity)
brief: Productive perplexity — the third phase of the adaptive cycle.
layer: framing-vocabulary
status: canon
tags: [cycle_phases, greek_vocabulary]
source_type: external
primary_source: Greek philosophical vocabulary (Plato; classical aporetic dialogue)
first_asf_mention: 01-aat-core/src/def-mismatch-signal.md
see_also: [prolepsis, aisthesis, epistrophe, praxis]
aliases: ["ἀπορία"]
do_not_confuse: []
```

Vocab in the wild: `status:` canon ×170, weak ×6 (only two rungs live, of vivarium's fuller working/draft/canon/weak/deprecated/superseded ladder the terminology-survey documents). `layer:` framing-vocabulary ×133, prose-symbol ×43 — an axis (what linguistic stratum the term occupies) the epistemic map doesn't carry. `source_type` + `primary_source` + `first_asf_mention` = provenance triple. `do_not_confuse` = a negative-reference field.

**Decision-event specimen** (`decisions/interiority-default/20260510T195841Z-joseph-canonicalize.md`):

```yaml
slug: interiority-default
action: canonicalize
decider: joseph
outcome: committed
timestamp: 20260510T195841Z
---
C1 clean canonicalize batch, naming-rename-plan.md
```

Action census across all 160 events: canonicalize 137, weak 6, add-cite 5, rename 4, **nuance-flag** 4, update-gloss 2, add-alias 2 — all decider=joseph. Design rationale stated in README: "Mutable status fields lose that history (and lose it silently when overwritten). Append-only files preserve it." Note the store runs BOTH: a mutable `status:` cell (projection) and the append-only ledger (events) — the evidence-ledger pattern the map's ④ describes, shipped here before relata.

## 4. SOPs as a segment population (`doc/sop/`)

10 files, `*.sop.md`. Maintainer: Joseph-ratified, agent-drafted. **Register vocabulary: no frontmatter at all.** Status rides in prose callouts — `sop-creation.sop.md` opens:

> `> [!note]` **Status:** authoritative (the convention itself; the home is young and still filling in). **Owns:** what an SOP is here… **See also:** …

So the sop store's implicit schema is Status / **Owns** (scope-of-authority declaration) / See-also — an authority register carried in a blockquote, not YAML. `agents.sop.md` doubles as the CLAUDE.md/AGENTS.md/GEMINI.md symlink target (one orientation file, three runtime names). The defining cut, verbatim (sop-creation.sop.md): "**if it fires a reflex, it is disposition (auto-loaded); if you follow it step by step, it is procedure (an SOP)**" — plus "defer-don't-fork: each rule is stated *once*, in its home; everywhere else *references* it." Disposition-vs-procedure is a segment-kind axis in its own right (§M14). Some SOPs carry explicit authority grades in the index itself: "authoritative (authored 2026-06-02)" vs "*(planned stub; authoritative content still at `msc/…`)*" — i.e., the directives register (proposed/ratified/ruled) exists here informally as prose.

## 5. Spikes (`spikes/`, 64 files + `.integrated/` + `.archived/` + `.routing-trail/`)

**Kind:** *spike* — a dated attempt-record. Specimen (`spike-adaptive-tempo-redundancy-penalty-2026-07-15.md`): no YAML; bold-field header block instead — `**Date:** … **Commissioned by:** Joseph (directly), following the 2026-07-14 adjudication… **Task:** …` then "**Verdict up front: DERIVED — with a correction that replaces the asserted claim.**" Verdict vocabulary observed: DERIVED, no-go, landed / partially-landed; disposition vocabulary from ROUTING.md and the sop: integrated / archived / routed / orphaned. Date-suffixed filenames (identity = slug **+ date**, unlike segments). Sub-populations: `PROPOSED.md` / `PROPOSED-ADVANCED.md` / `PROPOSED-MISC.md` (queued spike ideas), `INDEX.md`, plan files (`*-integration-plan.md` — "This document is a plan, not a derivation"), simulation scripts + `.png` outputs living beside prose. `ROUTING.md` self-describes as "*cycle state*, not governing content" — an explicit third category beside truth-apt and directive (§M3).

## 6. Empirica (`empirica/`, founding entry 2026-07-16)

**Kind pair:** *MANIFEST* (claims × tier × consumers, bidirectional with citing segments) + *RUNS* (run ledger: date, parameters, seed, output record). Specimen `track-b-nonlinear/MANIFEST.md`: per-piece table `| Piece | Claims (one line) | Tier | Consuming segments |`, tiers like "empirical (validation of derived results)". RUNS.md carries an **honest-gaps register**: "environments never captured…; the two May runs' write-ups do not state seeds — check script before rerun"; and a lifecycle status "**Vivarium-rerun status: planned** … on rerun, RUNS entries gain vivium references and this python corpus becomes provenance." Cited from canon as `empirica:<slug>[@run-date]`; "an empirical claim citing an experiment with no matching recorded run is a truth-status defect" (FORMAT.md). This is a whole segment family (registered experiment) with its own contract, absent from the map's carve (§M2).

## 7. Root-file population (asf root)

Attested at `~/src/arch/asf/`: `PRACTICA.md` (navigator; "top levels of the strategy DAG"; 🌟/⭐ priority glyphs; declares itself "**auditor-safe**" while marking its link-targets "priming-heavy" — see §M10), `TODO.md`, `PROPOSALS.md` (banded portfolio: Ready now / Soon / Later / Wait / Retired-superseded, per-proposal **Value** −10..+10 and **Independence** high/medium/low = parallelizability, provenance sigils G-BP/O-BP/C-BP/SP), `FINDINGS.md` (generated, "do not hand-edit", self-declares "a sampling, not yet exhaustive"), `JOSEPH-TODO.md` (decision valve → one-sitting briefs), `CHANGELOG.md`/`LOG.md` (history split at 2026-04-24), `NOTATION.md`, `LEXICON.md` (generated), `README.md` vs **`README-auditor.md`** (priming-stripped twin), `HISTORICAL-CONTEXT.md`, `CURRENT-VOL1.md`(+pdf, current-build snapshot), five `*-TODO.md` topic valves (FORMAT-, TERMINOLOGY-, BIBLIOGRAPHY-, INTEGRATION-CLEANUP-, TODO-big-picture), `TST-IDEAS.md`, `audits/` (per-audit findings + FINAL files keyed by 6-digit auditor ids, e.g. `audit-384279-FINAL-2026-05-27.md`, plus `ADJUDICATION-WORKING-218564/`), `_obs/` (drained-file archaeology, e.g. `NEXT-UP-drained-2026-07-15.md`), `msc/` (plans, working dirs), `releases/`, `mono/`, `ref/`.

## 8. The refs → relata lineage

**neurips/refs** (`~/src/neurips/refs/`): `entries/<bibkey>.yml` (pure YAML, no md body — fields title/year/journal/key/type/authors…), `verifications/<key>/<ts>-<actor>-<verdict>.md` (specimen verdict observed: `claim-supported`), `pdfs/`, `deny-list.yml` — an **enforcement register** ("DOIs that must not be cited… author surnames flagging potential self-citation… A finding here is not always a hard block… The lint tool surfaces; the human confirms") — the forbidden-rung analog of ISO term-status (§M12).

**relata** (code `~/src/arch/firmatum/relata/` → wait, live code is `~/src/relata/`-adjacent per lineage; the tree I verified: code at `~/src/arch/firmatum/relata/` was NOT found — the brief's path resolves, but the data tree is `~/.local/share/relata/` and that is what I verified): dirs entries / verifications / calibrations / ingest / quarantine / pending-machinery. Entry specimen (`entries/abbas-2010-role.yml`) adds provenance blocks like `undermind_imports:` (report, rank, relevance_score, imported-date, cite_key_hint). Verification-event specimen (`verifications/russell-1912-problems/20260509T195027Z-claude-verifier-doi-resolves.md`):

```yaml
key: russell-1912-problems
criterion: doi-resolves
verifier: claude-verifier
outcome: n/a
timestamp: 20260509T195027Z
```

— note `outcome: n/a`: **criterion-not-applicable as a first-class verification outcome** (§M11). CLI concepts (from `relata --help`, live): *designator* resolution "with a stated confidence"; *the ladder* — "commands act automatically when the evidence is identifier-grade; otherwise they present a choice… nothing ever waits invisibly (see: pending, decide)"; *membrane*; "every decision improves calibration" (a `calibrations/` dir exists — decisions feeding back into confidence, an axis nothing else in the estate has). `quarantine/` (specimen: `2026-07-13-false-pdf-registrations`) = a disgrace/recall state for entries, distinct from deletion.

---

## M. Misfits and axes the epistemic map's carve doesn't hold

1. **Spikes are a fifth-plus kind**: dated attempt-records with verdict-up-front, commissioned-by provenance, and a *disposition* lifecycle (routed → integrated/archived) — not truth-apt canon (they may contain refuted claims kept verbatim as history), not decisions, not references. Their identity includes a date; segments' identity forbids one. The map has no home for "immutable attempt whose truth has been extracted elsewhere."
2. **Registered experiments (empirica MANIFEST + RUNS)**: empirical-provenance objects with a bidirectional consumer contract and an explicit *record-gap register* ("honest gaps marked"). Neither reference nor claim; the claim lives in canon, the *backing* lives here, and staleness of the link is defined as a truth-status defect.
3. **Cycle-state trackers** (`spikes/ROUTING.md`: "*cycle state*, not governing content"; plan files: "a plan, not a derivation"; `audits/ADJUDICATION-WORKING-*/`): a self-aware third register — process-state documents that explicitly disclaim both truth-aptness and authority.
4. **Dual ladders on one atom, with documented decay asymmetry**: `status` (epistemic) vs `stage` (process) — and the spec itself records that stage "goes stale quickly … currently ignored in practice," lint demoted to warnings. Evidence that process-state axes need reset-on-edit/decay semantics (the map's process-state axis says this; asf is the confirming corpse).
5. **Era/dialect as a register**: 43 `old-*` files in 02-tst-core carry a header block (Origin / Relevance) and pre-AAT vocabulary deliberately not retro-translated ("historical files … preserve the terminology of their era" — FORMAT.md). Content whose *language* is frozen at a provenance era, riding inside a live corpus at `stage: old`.
6. **Max-attainable as prose vs field**: asf keeps the ceiling in the Epistemic Status paragraph by convention ("Max attainable: [status]. Currently [status] because [reason]"); the template made it a field. The lineage direction is worth recording in the taxonomy: ceiling started as voice-discipline, is becoming schema.
7. **Deliberately unrecorded classification**: the three rings are load-bearing for review strategy yet "not part of segment frontmatter — an analytical stance the reviewer takes." The taxonomy should have a place for axes that are *real but intentionally not persisted* (to avoid false authority of a written cell).
8. **Search-freshness register**: the Findings Search Log — dated entries each carrying a status and a *sufficiency judgment* ("why this depth was right at this point"). Neither evidence-strength nor process-stage: it's a decaying-diligence record, kin to the map's `current` freshness qualifier but per-search-act and append-only.
9. **Type ≠ status ≠ stage, and type carries posture policy**: the 19-type vocabulary encodes external claim-posture (postulate-not-axiom). The map's Axis 0 groups by broad kind; asf shows kind itself has a *humility/provenance* sub-axis (whose math is this?).
10. **Priming/audit-safety as a document axis**: README vs README-auditor; PRACTICA "auditor-safe" vs its "priming-heavy" link targets; audit SOP routing de-novo readers away from README. A per-document *contamination classification* — orthogonal to truth, authority, and process — that the map lacks entirely and that de-novo verification methodology depends on.
11. **`outcome: n/a` in relata verifications**: criterion-not-applicable as a recorded event outcome. Verification ledgers need at least {pass, fail, n/a}; a two-valued projection loses the "this rung doesn't apply to this atom" case (same shape as Max: not-applicable ≠ not-yet).
12. **Forbidden/deny as enforcement register**: refs `deny-list.yml` (must-not-cite DOIs, self-citation surnames, proper nouns) — with the surface-don't-block stance stated in-file. Parallels the ISO/TBX `forbidden` term-status the terminology-survey flagged; now attested twice in the estate.
13. **Sub-file epistemic grain**: equation-level tags give register at statement grain inside a segment. The map treats the segment as the atom; asf's practice says epistemic state is *multi-grain* (file status + per-equation tag + per-Finding novelty posture can disagree legitimately, e.g. a `conditional` segment containing `*[Derived]*` equations).
14. **Disposition vs procedure** (sop-creation.sop.md's reflex/steps cut) — a directive-kind sub-axis: two directive families with different *loading semantics* (always-present vs on-demand). The map's authority register grades how ruled a directive is, not which loading layer it must live in; asf treats the layer as the load-bearing distinction.
15. **Both projection AND ledger on one store**: terminology runs mutable `status:` cells alongside append-only decision events with an `action` vocabulary (canonicalize/weak/rename/nuance-flag/add-cite/add-alias/update-gloss) and a separate `outcome:` field (committed). The map's "④ ≈ ③ candidate unification: authority cell = projection of latest ledger event" is thus already shipped in asf terminology, not only relata — with the wrinkle that some actions (nuance-flag, add-cite) *don't* change the projected status: the ledger's event vocabulary is richer than any status ladder it projects to.
16. **Choice-portfolio axes**: PROPOSALS' banded readiness (Ready/Soon/Later/Wait/Retired) + Value estimate + Independence (parallelizability). The choice register in the map is one rung (decided); the live choice store grades *undecided* choices along three axes the map doesn't name — readiness, value, and conflict-surface.
17. **Marginalia**: inline HTML-comment editorial judgments in OUTLINE bodies (`remove --`, `salvageable --`) — a live annotation register with no schema, no maintainer discipline, and real content (some encode Joseph's aesthetic verdicts found nowhere else).

## Incidental finds / brief feedback

- **Brief path correction**: the brief pointed at `~/src/arch/firmatum/relata/` — the relata *code* tree exists there per the deployments catalog, but I verified the live data at `~/.local/share/relata/` and the CLI via `relata --help`; the firmatum copy's `IMPORT-ASF-TODO.md` suggests asf's own bibliography is *not yet* wired into relata (`BIBLIOGRAPHY-TODO.md` still at asf root) — a live corpus-gap if the taxonomy assumes the fourth-generation store covers asf.
- **`rg -h` hazard for other divers**: `rg -h "pattern" files...` prints ripgrep's help (`-h` = help, not no-filename); use `-I`/`-N`. Cost me one flooded call; noting so the synthesis doesn't inherit a bad tally from another dive.
- **Prefix/type drift**: vol-1 filename prefixes include impl-, img-, internal-, strategy- with no rows in FORMAT's type table, and both `der-`/`deriv-` conventions live. Harmless today, but a taxonomy keyed on slug prefixes will misclassify ~15 files; key on frontmatter `type` instead.
- The **deployments.md dangling "Notes copy" column** is confirmed dangling for the asf rows too; the live-source paths in its left columns remain accurate for everything I touched.
