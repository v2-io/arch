# Dive: udon v2 trees + verisectorium-as-specimen — segment kinds in the wild

*Dive record (2026-08-10, parallel-dive agent, udon-neighborhood assignment). Specimen-grade: every entry names its live path; verbatim material quoted at origin. Everything read is live-tree (see §0 on the MOVED hazard). Locators + observed vocabulary — not adjudication.*

## 0. Tree-liveness verification (brief asked; the answer matters)

- **Live tree: `~/src/arch/firmatum/udon/v2/`** — recent git activity (compose-defs commit 2026-08-10), `paths/` and `for-joseph/` present.
- **`~/src/MOVED/udon/` is a stale near-duplicate, not a pointer.** `MOVED/udon/v2/` lacks `paths/`, `for-joseph/`, `spikes/` extras; `theory/to-integrate/` has *diverged* (live tree has `lexical-forms-discussion-2026-08.md`, `K9-DRAFT-2026-08-08.md`, `acid-for-corpora.md`; MOVED has an `acid-for-corpora/` dir instead). `udon-needs/02-tooling-needs/src/` diffs clean today, but the trees are drifting, not mirrored. **Hazard worth recording somewhere authoritative:** an agent landing in MOVED would read confidently and wrongly.
- **The brief's ".ud/.un dialects" is half right:** `find v2 -name "*.un"` (excluding .archive) returns **nothing**. Live dialects in this neighborhood are `.ud`, `.udon`, `.md`. If `.un` exists it is elsewhere or archived.

---

## 1. The paths deployment (`~/src/arch/firmatum/udon/v2/paths/`) — the freshest def-segment form

### 1a. `def/*.ud` — the `term-group` segment (6 files, third edition, reset 2026-08-08)

**Kind as the corpus names it:** `|term-group` (udon block), one per concept-cluster, with nested `|term[...]` one-liners for sub-terms. Exemplar skeleton, `def/def-binding.ud`:

```udon
|term-group[binding][name][mint][maintainer][dangle][collide]
  :status proposed

  A **BINDING** is the first of the two mechanisms by which a @{reference} ...

  |term[name      ] A repeatable token ...
  |term[binding   ] A deliberate, maintained association from a name to a referent.
    :synonyms [declaration] ; the scope-graphs literature's word
    |rels :binds   @{name}       {1,1}
          :to      @{referent}   {1,1}
          :kept-by @{maintainer} {1,1}
    |invariants
      - ...
  |discussion
  |examples          (a markdown-style table of Name | Maintainer rows)
  |working-notes
```

- **Register vocabulary actually used:** exactly one field, `:status proposed` — uniform across all six. Note the value is from the **decided-by family** (proposed), not the evidence ladder, even though the outline assigns these rows `Max: axiomatic`. A definition in this corpus behaves like a *choice awaiting ratification*, not a truth-apt claim — evidence for the epistemic map's ④-covers-③ hunch, and a small crack in the carve: **def- sits in "truth-apt" on the map, but the freshest def practice statuses it on the decided ladder.**
- **Structural apparatus with no .md equivalent:** `@{term}` = load-bearing-but-referenced term mark (drives generated links); bracket-args on the block header declare which terms the group coins; `|rels` carries typed cardinality-annotated relations *inside the definition*; `:synonyms` with a provenance comment. State rides **inline as udon attributes**, not a frontmatter block — the segment head *is* the attribute section.
- **Maintainer:** the paths instance (steward-reset third edition; ORIENT governs).
- **Generated views:** `LEXICON.md` (pending assembler) + `LEXICON-overview.md` via `bin/compose-defs` — the generated file self-marks with a `GENERATED VIEW` HTML comment. Interim carrier `defs.source.ud` is **bannered**: "applicable ONLY until its content is integrated into def/ … retired by delete-test." A segment kind with a *scheduled death* — no ladder in the map names that state (it is not `transition` exactly; it's deliberate-interim).

### 1b. `DECISIONS.ud` — the decision ledger in udon dialect

Exemplar (whole entry, verbatim):

```udon
|decision[decided-by-vocabulary]
  :holds The seven-value decided-by vocabulary above governs this ledger.
  :why The weight of a decision is who stood behind it and how.
  :decided-by steward
  :date <2026-08-08>
  :cite Second edition D1 (in .archive/second-theory-iteration-2026-08-08/DECISIONS.md), carried at reset.
```

Head-comment carries the seven-value vocabulary verbatim (steward / ratified / council / supported / defacto / proposed / transition) — identical wording to `verisectorium/template/DECISIONS.ud`, which additionally adds the cite-by-slugged-link rule ("identifiers that will be *read* carry their referent's name"). Append-or-expressly-overturn; lower supersedes earlier. Contrast **`v2/DECISIONS.md`** (root, markdown): same present-truth ledger contract, but entries are **table rows with bare ordinal IDs** (C0–C5, W0, R1…) — the exact anti-pattern the template's cite rule was minted against. The two editions coexist live; a taxonomy axis: *decision-identity = slug vs ordinal* is a real in-the-wild split.

### 1c. Root-file population of one deployment (dialect noted)

| File | Kind (self-named) | State carrier | Notes |
|---|---|---|---|
| `CLAUDE.md` | front door | none | layout table with **priority + modify-frequency columns** — a register (how often may this be edited) no map axis names |
| `ORIENT.md` → `sop/src/disc-orient.md` | discussion segment serving as root file | .md YAML: `slug/form/status/max/state/depends` (`status: discussion-grade`, `max: decided`, `state: [drafted]`) | **root-type file dissolved into the segment store** — ORIENT is a *projection* of a sop segment; the root-file class and segment class are converging |
| `PRACTICA.ud` | strategic navigator | `|practica` + `|area` blocks: `:priority primary/secondary/background`, `:state active/blocked-on-clawback/seeded`, `:opened <date>`, `:updated` | area-state vocabulary (`blocked-on-clawback`, `seeded`) is a **WIP-state axis** distinct from both process-state flags and evidence ladders |
| `ADDRESSING-THEORY.outline.md` | canon view | table columns `Max` / `State` per row | outline duplicates segment frontmatter → deliberate collision-detection surface ("a disagreement between the two is a finding") |
| `DECISIONS.ud`, `CHANGELOG.md` | ledger + history layer | see 1b | CHANGELOG self-describes: "archaeological src, not active work" |
| `INFLUX/`, `sop/INFLUX/` | membrane | per-record frontmatter, see 1d | |
| `ref/` | citable-primaries dir | symlinks to live sources; copies only when source archived/external | a **reference organ realized as symlink farm** — freshness by construction, no `current` field needed |

### 1d. The influx record — a frontmatter family the map has no column for

`paths/sop/INFLUX/lexicon-tooling-recommendation-2026-08-07.md`, verbatim frontmatter:

```yaml
kind: recommendation
for: [steward, verisectorium-theory]
from: lexicon-tooling agent (paths/ session, 2026-08-07)
status: proposed
concerns: [bin/refresh-lexicon, term-marking convention, gloss segments, DECISIONS D5]
```

This is **correspondence metadata** — addressee, sender, subject-index — not epistemic state. Membrane-crossing records are a segment kind in the wild whose primary axes are *routing* (for/from/concerns), with epistemics reduced to one `status: proposed`. The map's carve (truth-apt/choices/directives/references/decisions) has nowhere to put it; `form-influx-membrane` in the template sop presumably will, but the *specimen* shows the axis set it needs.

---

## 2. udon theory (`~/src/arch/firmatum/udon/v2/theory/`) — FORMAT.md and its `.udon` segments

### 2a. FORMAT.md — the most explicit two-field epistemics statement in the estate

Fields (§3): `type` (ASF's 18-kind set adopted whole, unused kinds deliberately kept: "a trimmed vocabulary drifts from the parent and the drift is worse than the unused rows") · `status` (8 rungs: `axiomatic · exact · robust-qualitative · heuristic · conditional · empirical · discussion-grade · sketch`) · `stage` (`gap · draft · deps-verified · claims-verified · format-clean · candidate`) · `depends` · `from` (section-precise integration pointers — **a field the map lacks**: "a row whose `from` is unread is a row nobody has warrant to label").

Load-bearing register rulings, verbatim:

- "**`verified` in particular is a stage, not a strength** — the two were once one column elsewhere, where `verified` meant both *checked against the code* and *feels solid*."
- "**`status: exact, stage: draft` is coherent and common** … That is the whole reason for two fields."
- "**Stage is a present-tense work-remaining marker, not a gate and not a trophy** … **A ladder that only promotes accumulates falsehood.**" (resets-on-reorganization stated explicitly — the map's process-state axis, argued from first principles)
- "**A status is assigned by whoever read the source at the primary.** A status assigned from a summary is a proxy (§0) and is marked as one." — an **assignment-warrant rule** attached to the ladder itself; no map axis carries *who may write a status value and on what warrant*, yet two corpora here have one (cf. §3's verified-events).
- §0b "**Absence claims carry their search**" — a warrant rule for a specific claim *shape*, orthogonal to strength.
- Max-attainable as first-class outline field (`:max`), with the two travel conventions (transmitted ceiling = source's tier; normative ceiling = `decided`) — already in the map, but note FORMAT names it a **deliberate divergence from ASF/vivarium** where max is prose-only convention.

### 2b. `.udon` theory segment specimen — `src/norm-proxy-discipline.udon`

```udon
|segment[norm-proxy-discipline] :type normative :status decided :stage draft
  :max-attainable decided
  :depends [form-present-truth-collision norm-gap-as-discontinuity]
  :from theory/FORMAT.md§0

  |title Proxies locate; they do not settle
  |summary ...
  |formal-expression ...
  |epistemic-status ...
  |discussion ...
  |working-notes
    Keep FORMAT §0 as the full prose home; this segment is the citable kernel.
    If FORMAT and this segment diverge, that is a collision — fix one.
```

Same cadence as verisectorium's .md segments, carried as `|`-sections instead of `##` headings; attributes inline on the block header. **Dialect-neutrality finding:** the attribute *set* (type/status/stage/max/depends/from + titled sections) is invariant across `.udon` and `.md`; what changes is (a) carrier syntax (inline attrs vs YAML block), (b) that udon lets state ride *per-block at any depth* (`:status` on a `|term-group`, `:state` on a `|area`, `:decided-by` on a `|decision`) where .md frontmatter is one-block-per-file. **The taxonomy can be dialect-neutral at the axis level but not at the granularity level: udon carries state at sub-file grain natively; markdown needs one-atom-per-file to match.** That is arguably *why* one-claim-per-file exists in the .md corpora at all.

Also live: `status: decided` **on the evidence-ladder field** of a normative segment — the choice register and evidence ladder share one `status:` slot in this dialect, disambiguated by `type`. The map draws them as separate registers keyed off form-kind; this specimen confirms the keying (register applied = f(type)) but shows the wild stores them in *one field*, so any migration/lint must consult type before validating status values.

### 2c. Canon outline head-comment as deletion record

`UDON-THEORY-canon.outline.udon` opens with a verbatim deletion rationale ("a sterilized paste of TST vocabulary … **Not demoted — deleted.**") — integration-is-replacement practiced *in an outline comment*, i.e. the history layer leaking (deliberately) into the view head. Worth noting as a specimen of where the "history lives only in history layers" rule bends: reset rationales ride at the top of the reset artifact.

---

## 3. udon-needs/02-tooling-needs — the trio-ratified three-axis system in situ (30 chapters)

### 3a. The frontmatter, two contrasting specimens verbatim

`src/method-evidence-tiers.md` (a **method** segment — the conventions chapter itself):

```yaml
slug: method-evidence-tiers
type: method
register: decided          # this chapter states the report's conventions; a method is chosen, not measured
support-kind: —            # a method segment carries no evidence of its own
strength: —                # decided chapters take no strength rung (see "How the axes cross")
convergent: —
stage: drafted
consumers: both
verified:
  - 2026-07-22 · content · pilot-A · three-axis + two-lock + event-log schema landed and self-consistent
sources: [...]
apparatus-note: >
  ... Those codes [T1–T5] are retired from this report's live vocabulary ...
```

`src/errors-that-teach.md` (a **principle** segment):

```yaml
type: principle
register: [derived, evidenced]   # design criteria derived from the C3 gate; convergence evidences them
support-kind: [theoretic, observational, testimonial, design]
strength: robust-qualitative     # the direction holds across every independent kind here; no single magnitude
convergent: [design, testimonial, theoretic]   # ... observational is a descent-echo (11/14 = one design copied), not a fourth
verified:
  - 2026-07-22 · content · pilot-A · convergence audited under the failure-mode key: 3 independent + observational descent-echo, not 4
```

**Findings against the epistemic map:**

1. **The map's "event layer … shipped only in relata so far" is false.** All 30 chapters here carry `verified:` as an **append-only event list in frontmatter** — `date · aspect · agent · note` — projecting nothing (the events *are* the record). This predates and parallels relata's design. Correct the map.
2. **`register:` takes lists** (`[derived, evidenced]`) — a claim can sit in two registers at once, with the comment explaining the split. The map's Axis 0 assumes one register per segment.
3. **`convergent:` is not a boolean** — it names *which* support-kinds converge, and its comments carry negative adjudications ("descent-echo, not a fourth") — the convergence lock does live per-segment, with its audit trail inline as YAML comments.
4. **YAML comments are load-bearing register carriers** throughout — every axis value here travels with its one-line warrant. No map axis names "warrant-adjacent-to-value," but this corpus treats it as mandatory practice.
5. **`consumers: both`** — an audience axis (harness-reader vs udon-reader), nowhere in the map.
6. **`—` (em-dash) is a live value** meaning *this axis does not apply to this kind* — kind-conditional axis applicability made explicit, exactly the map's "register fixes strength" mechanism, notated.

### 3b. Segment kinds attested in the OUTLINE's Type column

Method · Counterposition · Finding · Principle · (plus, per outline elsewhere: Demand, Design, Stress-test, Bridge). **`counterposition`** (`src/counter-register.md`) is the standout misfit: `strength: mixed` with the real strength rung carried **per-row in a table column** ("The rung leads each cell") — a segment that is itself a register, whose epistemic state is a *distribution*, not a value. Its stated function is capping: "a thesis can be no stronger than the counter-evidence standing against it is weak." The map has no home for capping-segments or per-row state.

### 3c. Root-file population here

`OUTLINE.md` (spine of an *anthology* — bridges in src/, deep reports in reports/: a two-tier corpus shape the deployments catalog doesn't name) · `NOTATION-KEY.md` (**apparatus registry**: T1–T5/C1–C16/S1–S12/SC#N index-code namespaces with the rule "codes appear only in metadata and source notes, never in body prose — if body prose leans on a code, that is a defect") · `RESIDUALS.md` (honest-coverage ledger: "what the spine deliberately did not absorb / known gaps / bolt-on points" — a **negative-space organ** no map axis or organ names) · `DEEPENING-CYCLES.md` (workflow) · `CHANGELOG.md` · `notes/` (channels, incl. the epistemology reasoning record and `TST-extension-memo.md`, the outward proposal).

### 3d. `v2/OPEN.md` — an open-question class vocabulary

Rows carry a **Class** column: `WAIT-DEMAND` · `STEWARD / fact` · `agent-suggested (open)` — i.e., *what kind of event would close this and who may close it*. Plus a steward-quoted holding instruction (jaw 2026-07-28) that rows posed against a stale snapshot must not be pressed as binary calls. This is a **question-state register** (the dual of decided-by: undecided-by) that no current map axis covers, and it pairs naturally with ④.

---

## 4. Verisectorium as its own specimen (this repo)

### 4a. theory/src frontmatter — the `form` / `type-expected` dual

`theory/src/claim-absence-vs-conflict.md`:

```yaml
slug: claim-absence-vs-conflict
form: claim
type-expected: derived
status: proposed
max: robust-qualitative
state: [pre-drafted]
depends: [claim-truth-over-proxy]
```

Two kind-fields coexist: **`form`** (stable speech-act, = slug prefix) and **`type-expected`** (trajectory position, explicitly *expected not settled*). This is `form-slug-form-kinds`' form-kind/trajectory-kind partition realized in metadata — the most principled answer in the estate to the "kind" ambiguity, and the axis the taxonomy segments should probably adopt as *two* axes rather than one Type column. (`template/sop/src/form-slug-form-kinds.md` carries the 9-prefix speech-act table: def/post/scope/form/norm/claim/obs/meas/disc.)

### 4b. `state:` is an overloaded flag-list — a live conflation

Observed `state` values across this repo: `[pre-drafted]`, `[drafted]` (process flags) **and** `[override]`, `template`, `proposed` (template-maintenance values, in `template/sop/src/dir-orient.md` frontmatter and the template SOP outline's State column). The map's own axes say maintenance and process-state are orthogonal — but the *artifacts* store both in one field/column. Either the map legislates the split into two fields, or documents why one slot multiplexes them (disambiguated by store?). The template SOP outline also shows compound max values (`ruled-current`, i.e. authority × freshness in one cell) — another one-slot multiplex.

### 4c. Template-specific observations

- `template/def/` and `template/src/` are **empty**; the whole seeded content is the sop store + ledgers + navigators. The template's real payload is *organs and registers*, not content — consistent with the theory's substrate-neutrality claim, and evidence that root-file population + sop store is the minimal deployable kernel.
- Template `DECISIONS.ud` ships a **deploy-time seed pattern**: a commented-out decision block with `[placeholders]` and "uncomment by deleting this comment line and outdenting" — a segment state best named *seed/uninstantiated*, distinct from gap and from pre-drafted.
- The template SOP outline column set — `State | Type (or) Expected | Designator | Description | Epistemic Status | Max` — vs the paths outline's `Expected Type | Tag | Claim | Max | State` vs udon-needs' `§ | Type | Tag | Claim | Stage`: three view schemas over near-identical underlying axes. View-column naming is *not* converged even where the axes are.

---

## 5. Misfit summary — what does not fit the carve (the taxonomy's work list)

1. **Definitions status on the decided ladder** (paths def/: `:status proposed` under `Max: axiomatic`) — def- straddles truth-apt and choices; coining is an act, the coined content is truth-apt. Possibly def- needs the same two-axis split as everything else rather than a single register assignment.
2. **Influx/membrane records** — routing axes (kind/for/from/concerns), correspondence not assertion. No carve column.
3. **Counterposition / register-segments** — per-row state, distribution-valued strength, capping function.
4. **Method segments** — self-referential conventions chapters; `register: decided` but not a decision-ledger entry; closest to choices but living in src/ as a chapter.
5. **Question-state** (OPEN.md classes: WAIT-DEMAND / STEWARD-fact / agent-suggested) — the undecided dual of ④.
6. **Interim-bannered carriers** (`defs.source.ud`) and **deploy-time seeds** (template DECISIONS placeholder) — scheduled-death and uninstantiated states, neither in process-state.
7. **Negative-space ledgers** (RESIDUALS.md) and **apparatus registries** (NOTATION-KEY.md, with the codes-never-in-prose rule) — root-type kinds attested once each, both verisectorium-important.
8. **Warrant/assignment rules riding the axes**: who may assign a status (FORMAT: whoever read the primary), absence-claims-carry-their-search, YAML-comment-per-value warrants, `verified:` events. The map has an event layer; it lacks the **assignment-warrant axis** these all instantiate.
9. **Audience axis** (`consumers:`), **modify-frequency register** (paths CLAUDE.md column), **`from:` integration-pointer field** — small but real, each carried by exactly one system.

## 6. Direct corrections to existing influx artifacts

- **epistemic-map.md**: "event layer … shipped only in relata so far" → also shipped in all 30 udon-needs chapters (`verified:` lists, 2026-07-22). Also: Axis 0 assumes scalar register; udon-needs uses register *lists*.
- **glimpse/brief**: no `.un` files exist in live udon v2 (excl. archives); MOVED/udon is stale-and-diverging, not merely mid-reorg.
- **deployments.md** "udon theory | `*.outline.udon` | process segments": now 6 substantive segments incl. normative/formulation kinds, plus the canon outline carries `:max` per row — richer than "process segments" suggests. And the paths deployment (its successor-in-practice) is absent from the catalog entirely — it postdates it; worth a row (atom: term-group + segment; rules: ORIENT + FORMAT-by-inheritance; ledger: DECISIONS.ud; scale: 6 def + sop store).

## 7. Adjacent finds

- **ORIENT-as-symlink-to-segment** (paths): the root front door is *itself* a sop segment — root-file class and segment class unifying. If the taxonomy treats "root-type files" as a separate population, this specimen says the separation may be temporary scaffolding: root files are views/projections of segments wherever the store is mature enough.
- **The gloss-vs-definition problem** (paths sop/INFLUX record §1) is a live, well-argued case that the taxonomy will need a `gloss`/pedagogical-introduction kind distinct from `def`, with an explicit anti-laundering rule ("letting them share a slot launders register upward"). The record is addressed to verisectorium-theory and appears not yet ingested here.
- **`for-joseph/` (udon v2 root)** — a whole steward-valve *directory* (QUEUE, PLAIN-DECISIONS, MORNING-ADJUDICATION, UNIF-PASS-QUESTIONS): the valve organ at population scale, not a single JOSEPH-TODO file.
- **PRACTICA area-states** (`blocked-on-clawback`, `seeded`) suggest the process-state axis wants a *blocked-on* relational value, not just progress flags.

*Willing to stay on the line for synthesis follow-ups.*
