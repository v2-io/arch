# Relata methods that would strengthen verisectorium

*Comparative report, 2026-08-05. Written after full read of the verisectorium notes gather and a thorough survey of live relata (README, TODO-ingest §7/§11/§16 spine, BETTER-DATA-AND-CALIBRATION-TODO, `docs/sys/*`, core lib modules, calibration scripts, fixtures). Copies under `verisectorium/notes/` are collation, not live authority; where something conflicts, originals win.*

**Readers:** Joseph and follow-on agents synthesizing the verisectorium program — people who already know outline+segments / directory-as-table, and want **relata-grounded improvements**, not a rehash of the gather index.

**Register:** Claims about what relata *does* are verified against code/docs at survey time. Claims about what would transfer to verisectorium are **proposed transfers** — design pressure, not ratified law. Where relata is weaker or orthogonal, that is named.

---

## 0. One-sentence frame

Verisectorium’s best current account of itself is **present-truth atoms + exposition views + optional enforcement profiles**. Relata’s best account of itself is **epistemic state as the primary object, with every automatic or human decision leaving an evidence vector and a calibration trail**. The productive comparison is not “cousin store features” but: **what disciplines would make a claim corpus as honest about *how sure and why* as relata is about bibliographic identity**.

The verisectorium notes already touch relata as lineage (cousin store, fourth generation of refs/terminology). They mostly capture **layout and CLI surface**. They under-capture the **decision epistemology** — which is exactly the canary that motivated this commission, and which is also the load-bearing half of the project’s own North Star (§7.8: the bibliography is the safe sandbox; the discipline is the deliverable).

---

## 1. What the notes corpus already holds (so transfers land on real gaps)

### 1.1 Load-bearing verisectorium commitments

From synthesis + primary + claim landings (not re-argued here):

| Commitment | Where it lives in the notes |
|---|---|
| Slug = identity; outline = order/view | `form-identity-ordering-split`, PROPOSALS §H, FORMAT |
| Present-truth surfaces enable collision; history alone does not | `form-present-truth-collision`, generalization Part 7 |
| Gaps mark discontinuity; they do not invent inventory | `norm-gap-as-discontinuity` |
| Proxies locate; they do not settle | `norm-proxy-discipline` |
| Shape portable; type vocab and gate-vs-warn are deployment | doc-store §12.2, deployments spectrum |
| Segment is often a *cluster* (body / WN / events / companions) | generalization Part 3 |
| Multi-timescale strata; mixing is mechanism; C1/C2 from AAT | generalization Part 5, tst-grounding, underlying-logical-model §3 |
| Authored view vs generated view; edge attrs vs record attrs | underlying-logical-model §5(d), PROPOSALS §H |
| Gate 4 dispositions exist as a three-valued act but are not recorded as events | generalization Part 6 |
| Promotion ladder largely aspirational; WN as overloaded queue | theory-content-lifecycle-findings |
| Tribunal: decision ≠ confidence; load-bearing paths; revisit/expires | tribunal-revisited |
| VERA strands want qualified truth + calibration language | vera synthesis; vox-vera; PROPRIUM |
| Cousin stores prove spine without pedagogy | instances synthesis |

### 1.2 Where the notes are thin on relata

- Cousin-store framing: external data tree, write-membrane, verification events, emit-as-view, lineage.
- **Missing or shallow:** Good-style weight-of-evidence ledger; prior/likelihood split; soft refutation; independence rule; seed-prior defended chains; calibration events + fit-as-proposal; observation-vs-model separation; designator ladder tiers; identifier-grade vs judgment-grade auto-tier; standing re-audit of *evidence* not just schema; regrowth gate / aliases as identity discipline; coverage honesty; cost-ordered early-cancel; corpus self-labeling as free training data.

The rest of this report fills that gap and maps transfers.

---

## 2. Relata’s real innovations (methods, not features)

These are ordered by transfer power into claim corpora, not by how impressive they look in a README.

### 2.1 Epistemic state is primary; the formatted record is a view

Relata refuses the market’s inversion (Zotero/Paperpile: formatted record primary). The first-class object is **graded, provenanced, defeasible belief** about identity and support, plus relations. `emit` produces `.bib`; `markdown/<sha>/` is a derived cache. Both are projections.

**Verisectorium parallel already half-there:** outline-as-view / PDF-as-rendering / LEXICON-as-generated. **Gap:** for *claims*, the “primary object” still often collapses to the segment prose + a status enum. Status is treated as truth-strength vocabulary, but it is rarely backed by a portable **evidence vector**. Relata shows what “epistemic state primary” looks like when it is structural rather than aspirational.

**Transfer:** Treat a segment’s present-truth body as the collision surface (already right), and treat its *epistemic support* the way relata treats identification: a ledger + events, not only a `status:` token. The status enum can remain the *projection* (coarse tier for readers and filters) while the ledger is the substrate.

### 2.2 One decision rule: log-odds + Σ WoE; no points-tangle

```
logodds(E) = log prior-odds(E) + Σ_i woe_i
accept iff posterior ≥ τ_high
```

Implemented once in `EvidenceLedger`; `Decider` is the single “candidate-source → verdict” definition. No per-factor thresholds, no parallel scoring systems. Geometric mean is explicitly the *degenerate case* of equal weights + flat prior — built once, not bolted on.

**Why it matters beyond PDFs:** the estate keeps inventing informal multi-signal judgments (audit findings, promotion readiness, “enough evidence to raise status,” tribunal confidence). Informal multi-signal judgment is exactly the points-tangle §7 exists to abolish.

**Transfer:** any multi-signal gate in a verisectorium (Gate 1–4, lint composite, “claims-verified”) should either (a) be a pure structural check, or (b) be a single ledger with named factors and defended weights — never a grab-bag of heuristics that each *feel* like a threshold.

### 2.3 Priors and likelihoods held strictly separate

Priors = population facts before content (base rates, fs-context, no-PDF tilt). Likelihoods = observed content (identifiers, title, fingerprint). Folding priors into the evidence sum is how base rates silently manufacture acceptance (the aczel/Measures lesson made structural).

**Transfer to claim corpora:**

| Prior-like | Likelihood-like |
|---|---|
| “Most segments at draft are incomplete” | “Gate 2 found Discussion claim ungrounded” |
| “This OUTLINE row is early in pedagogy” | “`depends:` slugs all exact” |
| “Authoring agent just landed this file” | “Formal Expression re-derives from listed premises” |
| Corpus-wide base rates of status | Primary-source verification of a cited theorem |

Collapsing these is how `stage` gets misread as epistemic strength (FORMAT already warns; lifecycle findings show the warning is not enough). Relata’s separation is the *mechanism* that makes the warning enforceable.

### 2.4 Soft refutation; absence is never refutation

A conflicting strong identifier is a large **finite** negative WoE (misextraction rate *b* > 0), not a hard veto. It (a) drags posterior down, (b) **suppresses short-tracking** (keep gathering), and does **not** special-case the arithmetic. **Absence ≠ conflict.**

**Transfer:**

- Missing `depends:` target, unread primary, or empty empirica run is **absence** (gap / named discontinuity / truth-status defect per contract) — not a silent “refuted.”
- An actual contradiction (two present-truth statements; DOI points at different work; Discussion claim contradicts Formal Expression) is **conflict** — soft in the sense that abundant independent corroboration can still force re-adjudication, hard in the sense that it must surface and block early “we’re done.”
- This is the operational twin of `form-present-truth-collision` + `norm-gap-as-discontinuity`: collision is conflict; bare gap is absence. Relata shows how to encode both in one sum without lying about either.

### 2.5 The independence rule (correlated signals are one channel)

Measured live: two “signals” derived from the same string (filename family, title family, or position+region of a DOI) must emit **at most one** factor. Double-counting cleared τ_high on a renamed file alone.

`BETTER-DATA-AND-CALIBRATION-TODO` generalizes: venue / publisher / pdf-source-domain are often one observation wearing three hats — record wide, model carefully; under naive Bayes collapse to one categorical factor.

**Transfer:** verisectorium audits and multi-agent reviews chronically multi-count correlated agreement (same author, same disposition, same session’s three agents, outline row + frontmatter stage + WN “feels solid”). The independence rule is portable methodology: **before combining signals, ask whether they share a generative source.** Convergent-lock language already exists in the estate (`epistemology-SYNTHESIS` in the generalization note); relata is the *implemented* form of that discipline on numerical factors.

### 2.6 Seed priors carry defended chains; fit is proposal, never auto-apply

§7.10: every seed WoE states P(obs|IS), P(obs|NOT), residual uncertainty, and what evidence would revise it. `CalibrationFit` produces a proposal with counts and warnings; adoption is a deliberate edit to the single calibration site (`EvidenceLedger::Defaults`). Strengthen-before-soften preserved at the code boundary.

**Honest gap in relata itself (do not romanticize):** as of 2026-07-13 notes, calibrations store was nearly empty; some later factors were hand-typed from corpus measurements with counts only in prose comments; fit *artifact* (numbers ⇢ data ⇢ date ⇢ counts, versioned in git) was missing. The **discipline** is the innovation; the production loop is still catching up. That honesty is itself transferable: do not claim a calibration system exists because the class names exist.

**Transfer:** any constant that gates a claim corpus (τ for auto-promote, stage vocabulary, “overall verified = these three criteria”) should have:

1. a single site,
2. a defended chain or a fit artifact,
3. human review before changing operational behavior.

Bare magic numbers in linters are points-tangle.

### 2.7 Every decision writes labeled calibration data (vector, not scalar)

`decide` (attach / create / reject / skip) writes one `CalibrationEvent` per (source, candidate) with **full ledger** + human/corpus verdict. Scalar confidence alone is explicitly insufficient for refit.

**Transfer to Gate 4 / promotion / tribunal:**

Generalization Part 6 already noticed: Gate 4 dispositions (resolved / deferred / promoted) exist as acts but leave no event — note and transport disappear together. Relata’s pattern is the missing piece:

```text
disposition event = {
  subject: segment-slug | WN-item-id,
  act: resolved | deferred | promoted | rejected,
  ledger or reasons: [...],   # not only the verdict
  actor, timestamp,
  revisit-when? / expires-on?  # tribunal columns if decision-grade
}
```

This also unifies with tribunal-revisited: **decision ≠ confidence**; load-bearing paths; revisit criteria. Relata’s ledger is the micro form; the tribunal record is the macro form; Gate 4 is the unbuilt middle.

### 2.8 Write-membrane with two distinct non-happy outcomes

Nothing external writes `entries/` directly. Drop → validate → promote. Outcomes:

- `.rejected` — submitter erred (schema, collision, deny-list)
- `.needs-review` — system honestly uncertain (candidates + ledgers in sidecar)
- `.skipped-nonbib` — wrong kind of drop

Conflating the middle with rejection mislabels system-uncertainty as user-error — called out as a small dishonesty the North Star forbids.

**Transfer:** agent-authored segments, OUTLINE edits, and terminology/relata-style sibling stores need the same speech-act split. Today, “lint failed” often collapses “you wrote invalid frontmatter” with “the system cannot decide whether this depends-edge is genuine.” Those need different repairs (fix your file vs adjudicate a hard call). Underlying-logical-model §4 already names this as batch residue with typed outcomes; relata is the shipped membrane.

### 2.9 Designator resolution ladder — never silent degradation

Five tiers: automatic → blocking interactive (JSON+exit for non-TTY) → batch choices → `pending` report → spool queue. Three acts never conflated: **rerun** (re-present stored memo) ≠ **`--retry`** (recompute pipeline) ≠ **decide**.

**Transfer:**

- Segment addressing (`#slug`, OUTLINE row, fuzzy title of a claim, agent paraphrase) is a designator problem. Silent “best guess” attach of a cross-ref is the bibliographic false-accept in theory clothes.
- Orphans, renamed slugs, and `OUTLINE-accepted` exceptions want the same ladder: auto when unique and high-confidence; otherwise visible pending with exact next command — not buried in a 17-item lint noise floor (lifecycle findings on lint-outline).

### 2.10 Identifier-grade vs judgment-grade automatic tier

What may auto-write canonical is drawn at **evidence quality**, not at “human vs machine”:

- Identifier-grade + consistency check → auto create/attach
- Judgment-grade (title/author/filename) → park until calibration earns open

τ encodes a verbal criterion first: unique-id **or** ≥2 corroborating non-id signals; lone weak signal escalates.

**Transfer:** auto-promotion of segment `stage`/`status`, auto-absorb of WN into body, or auto-merge of “duplicate” claims should use the same cut. LLM-generated formal expressions and multi-agent agreement are judgment-grade unless tied to identifier-grade anchors (machine-checkable depends, re-derivable math, empirica run ids). Gate 2’s “plausible Discussion is worse than a gap” is the same cost asymmetry: false foundation compounds; escalation is cheap.

### 2.11 Append-only events; latest wins operationally; trail never deleted

`verifications/<key>/<ts>-<verifier>-<criterion>.md`, `calibrations/`, `pdf-attempts/`. Mutable status fields lose history. Concurrent verifiers don’t collide.

**Transfer:** terminology already has `decisions/<slug>/`; claim segments mostly dump history into WN or global CHANGELOG. Relata shows the middle grain: **per-atom event directory**, criterion vocabulary, outcome enum, free-form note. “Overall verified” is a **derived** function of latest events per criterion — not a hand-set field that forgets who said what.

This is also the answer to “validate only at claim time accumulates silent lies” (`relata audit`): standing recheck of *evidence* (blob still a PDF, masthead still matches, preprint not silently standing for published). Verisectorium analog: recheck that cited `#slug` still asserts what dependents assumed; that `empirica:` still has a matching run; that WN “resolved” items did not reappear as body falsehoods.

### 2.12 Identity resolution: never mint; aliases; siblings link never merge

Resolve-then-enrich. Silent mint-on-collision created 29 duplicate clusters (measured). Merges record absorbed keys as `aliases:`; `emit` writes under the **cited** name. Citation-distinct versions use `same_work_as:` — link, never silent merge.

**Transfer:**

- Segment renames / slug migrations: alias table so `#old-slug` and dependents don’t rot (today: manual, error-prone).
- “Same claim, two expressions” (preprint-level Discussion vs exact Formal Expression; paper claim vs framework claim): link as siblings with explicit relation, don’t merge into one atom that corrupts both contexts.
- Regrowth gate after merge: every key-minting path checks aliases (relata relearned this when a re-drain recreated twins within hours).

### 2.13 Coverage honesty

A registered blob may be sample / chapter-1 / preprint. `coverage:` is load-bearing; silent `full` when sample cues fire is forbidden.

**Transfer:** partial segment bodies, bridge segments, “excerpt recapitulation of external theorem,” and paper-section lite files that stand in for full arguments need an honest coverage claim. Without it, agents verify against the wrong object with high confidence — the bibliographic failure mode in claim space.

### 2.14 Cost-ordered pipeline + short-track / early-cancel

Stages ordered by cost; short-track only when posterior ≥ τ **and** no standing refutation **and** robust margin against pending producers’ max yield. Parallelism changes *when* evidence arrives, never *how* the decision is defined (order-invariant sum).

**Transfer:** de-novo audit and multi-agent review are expensive. A principled early-stop (“this segment’s Formal Expression already fails Gate 2 probe 1; do not spend 40k tokens on prose polish”) is the short-track rule. A standing contradiction should *expand* work, not shrink it — same as refutation suppressing short-track.

### 2.15 Corpus verbs over serving verbs

Underlying-logical-model already notes: agents pick up `relata <verb>` intuitively; layout is opaque by construction. Resolve / show / verify / ingest / emit / pending / decide / audit.

**Transfer:** verisectorium tooling still speaks files and lines. The long-horizon move is corpus verbs: `segment show`, `outline via`, `status ledger`, `promote --dry-run`, `pending dispositions`, with the serving (paths, YAML, markdown) underneath. Relata is the estate’s best adoption proof that **bowl verbs beat better file tools**.

### 2.16 Observation layer vs model layer (standing design, partially unbuilt)

Joseph’s thesis in BETTER-DATA: record every deterministically-knowable feature append-only; the model (naive Bayes today, logistic later) is a **consumer**. Do not decline to record because you cannot yet weigh.

**Transfer:** process signals in claim work (page counts of a source PDF, which audit found which class of defect, co-change coupling, Gate outcomes, time-in-stage) are thrown away constantly. Record wide in an observation store; only later decide how they enter status or promotion. This is how multi-timescale strata become measurable instead of metaphorical.

---

## 3. Transfer map — concrete improvements to the verisectorium synthesis

Grouped by what they would strengthen, correct, or extend.

### 3.1 Strengthen: present-truth collision needs an *evidence* twin

**Notes already say:** present-truth makes staleness detectable via collision; history is collision-free.

**Relata adds:** detection is not only collision of *content*; it is also collision of *support*. Two segments can agree in prose while their verification events disagree; an entry can claim a PDF while the blob is an HTML login wall (lived failure → `audit`).

**Improvement:** synthesize a dual mechanism:

1. **Content collision** — Formal Expression / claim body (existing).
2. **Support collision** — evidence events / ledger factors / empirica contracts (relata-shaped).

Fable’s caveat (collision cannot find what nothing claims) stays; support collision cannot find what was never checked — that is absence, needing gaps and census.

### 3.2 Strengthen: status/stage vocabulary → projections of ledgers and events

**Notes already say:** type ⊥ status ⊥ stage; stage denorms and rots; warnings-only is a deployment choice.

**Relata adds:** “overall verified” is derived from latest events per criterion; schema validation ≠ evidence validation; deep validate rechecks bytes and claims.

**Improvement:**

| Projection | Substrate |
|---|---|
| `status:` (epistemic tier) | ledger over support factors + human adjudicated calibration |
| `stage:` (process) | event trail of gates/dispositions; OUTLINE column is denorm of that trail, not a second authority |
| “ready to cite / ready to promote” | τ-gated function of ledger, like auto-attach |

This would correct the lifecycle finding that the promotion ladder is aspirational: either make stage a real function of events, or stop projecting a ladder that never fires (integration-is-replacement on the process model itself).

### 3.3 Strengthen: Gate 4 and tribunal revisit as one family of pre-registered expectations

**Notes already say:** Gate 4 dispositions; tribunal revisit/expires; O15-style expectation bounds.

**Relata adds:** labeled decision store; rerun vs retry vs decide; pending as first-class UX; short-track margin.

**Improvement:** treat Gate 4 disposition events and tribunal adjudication records as the **same schema family** at different altitudes:

- micro: WN item disposition
- meso: segment promotion decision
- macro: council/governance decision

Shared columns: subject, verdict, confidence/ledger, load-bearing reasons, revisit-when, expires-on, actor, timestamp. Relata proves the micro/meso grain can be append-only files multi-agent-safe; tribunal-revisited sketches the macro.

### 3.4 Correct: “cousin store” undersells the epistemic deliverable

**Notes currently:** relata ≈ directory-as-table + membrane + events + emit.

**Correction:** the *product* is a **calibrated belief-update discipline** rehearsed on a safe domain (§7.8). Layout is how it survives multi-agent concurrent writes. If the synthesis treats relata only as “refs gen-4,” it will keep missing the transfer that matters for claim corpora and for VERA.

### 3.5 Correct: enforcement profile is not only gate-vs-warn on lint

**Notes already:** stakes × reversibility choose gate-vs-warn (logos anonymization vs ASF stage).

**Relata adds a second axis:** automatic vs park, drawn on **evidence grade**, independent of deployment stakes. Critical deployment can still auto-accept identifier-grade evidence; careful deployment can still park judgment-grade even when stakes are low.

**Improvement:** two-dimensional enforcement:

1. **Stakes × reversibility** → does failure block?
2. **Evidence grade** → may the system act without a human?

ASF today mostly has (1) for lint and almost no (2) for promotion.

### 3.6 Extend: aliases and same-work links into claim identity

**Notes have:** slug stability, no numbers in filenames, cross-refs by slug.

**Missing:** first-class rename survival and citation-distinct sibling claims.

**Extend:** `aliases:` / `same_claim_as:` (or typed `same_work_as`-analog) on segments; build and lint resolve through aliases; paper keys that cite old slugs keep working the way `emit` keeps cited bibkeys working.

### 3.7 Extend: pending as the visible queue for everything unfinished

Relata’s `pending` makes wait-states operable. Verisectorium’s unfinished work scatters across OUTLINE gaps, WN deluges, TODO.md, gold-lift parks, lint noise, and agent memory.

**Extend:** a deployment-level `pending` surface: parked promotions, needs-review membrane items, bare gaps, dispositions without events, empirica truth-status defects, unverified load-bearing cites. Not one more markdown list — a **query over event stores and outline gaps**, with “exact next command” the way relata does.

### 3.8 Extend: standing audit of support (anti silent-lie)

`relata audit` is non-destructive recheck. Verisectorium has de-novo audit (deep, episodic) and lint (shallow, continuous), but little **standing re-verification of support assumptions**.

**Extend:** a continuous or periodic support audit:

- cited `#slug` still has the formal content dependents used
- `empirica:` still has matching runs
- verification events still consistent with registered artifacts
- preprint/coverage mismatches for external deps via relata

This is cheaper than de-novo and catches the class of failure de-novo is too rare to catch.

### 3.9 Extend: observation store for process multi-timescale work

Notes circle strata clocks and C1/C2 without instrumentation. Relata’s BETTER-DATA thesis is the instrumentation program: record features even without weights.

**Extend:** define a minimal observation schema for claim work (segment, timestamp, event-kind, features{}, optional label later). Feed it Gate outcomes, disposition acts, co-edit stats, audit defect classes. Multi-timescale theory becomes falsifiable against a trail.

### 3.10 Orthogonal / do not force-fit

| Relata strength | Why not central to verisectorium core |
|---|---|
| Online registry ladder (Crossref/DataCite/arXiv) | Domain-specific to published works |
| PDF/OCR conversion pipeline | Presentation substrate, not claim epistemology |
| FRBR work/expression/item cataloging | Useful metaphor for claim versions; not a full FRBR program for theory |
| Undermind CSV reflist ingest | Paper-survey workflow, not outline+segments |
| Naive Bayes as *the* long-term model | Even relata is planning past it; transfer the observation discipline, not the model forever |

| Verisectorium strength | Where relata is weaker |
|---|---|
| Authored pedagogical outline with edge glosses | Relata has generated views, not multi-outline teaching paths |
| Present-truth claim bodies as collision surfaces | Entries are bibliographic identity, not theory content |
| Type vocabulary for claim kinds (postulate/result/…) | Bib types are publication forms, thinner epistemically |
| Explicit multi-layer segment cadence (FE / Discussion / WN) | Relata’s “layers” are more event-store than prose cadence |
| De-novo audit as cognitive experiment | Relata has no analog of slow ordered segment reflection |

Honesty: **relata is not a better verisectorium.** It is a better **epistemic membrane and calibration laboratory** that the verisectorium synthesis should import as method, while keeping outline+segments for what it is best at (present-truth exposition atoms).

---

## 4. Canary answers (background only — not the whole report)

### 4.1 Bayesian / calibration updates as bib-entries are adjudicated

**What exists (verified):**

1. **Online decision rule** — Good WoE sum, τ_high, soft refutation, prior/likelihood split (`EvidenceLedger`, TODO-ingest §7).
2. **Seed calibration** — defended chains in §7.10; single site in code; regression tests pin aczel-class failures.
3. **Labeled trail design** — `CalibrationEvent` per (source, candidate) with full ledger; `CalibrationFit` Beta-smoothed empirical LLR as **proposal only**.
4. **Confirm loop wiring** — `decide` writes events; corpus self-label (DOI+PDF) recognized as high-volume label source in BETTER-DATA.
5. **Independence discipline** and **observation-vs-model** separation as the next maturation step.

**What is not yet “a finished calibration system”:**

- Fit artifacts versioned in git with counts were still a named gap (2026-07-13).
- Confirm-loop volume was thin relative to corpus self-labels.
- Some measured factors were promoted into constants via prose comments rather than full fit pipeline.
- LLM adjudication rungs (Stages E/F) designed, not built — and doctrine says they adjudicate *over* evidence, never invent citation facts.

**For verisectorium:** import the **shape** (vector not scalar; seeds defended; fit proposes; independence; auto only on identifier-grade) even while relata finishes paying its own residual.

### 4.2 Multi-speed document process layers

Relata does not solve Kelvin–Helmholtz / AAT multi-timescale math. It **implements membranes and clocks**:

- fast: extractors, search, spool drops
- gated: decide / human confirm
- slow: seed refit, audit recheck, corpus-wide dedup
- impermeable by default: external writers → membrane

That is the operational form of underlying-logical-model’s “write membrane permeability as C2 coefficient.” Verisectorium theory has the theorem side; relata has a working membrane. The synthesis should join them: **C2 control = membrane + event-visible crossings**, not only slower humans.

---

## 5. Suggested synthesis deltas (actionable, prioritized)

If the next verisectorium synthesis cycle wants concrete work rather than more maps:

1. **Doctrine paragraph (cheap, high leverage):** “Epistemic state primary” for claim atoms — status is a projection of ledgers/events; outline remains exposition view; history remains append-only and non-colliding.
2. **Gate-4 event schema (medium):** append-only dispositions with reasons; wire `pending`; stop relying on WN emptiness as the only drain signal.
3. **Support-audit verb (medium):** standing non-destructive recheck of depends / empirica / external cites (call out to relata for bib side).
4. **Alias table for slugs (medium):** rename survival without breaking the estate.
5. **Two-axis enforcement profile (cheap conceptually):** stakes×reversibility × evidence-grade.
6. **Observation store spike (larger):** record process features without forcing early weights; feeds multi-timescale and calibration later.
7. **Do not:** replace OUTLINE+segments with relata layout; force naive Bayes onto Discussion prose; claim calibration is “done” because classes exist.

---

## 6. Feedback on this commission and adjacent observations

### 6.1 Report substance

The highest-value finding is not a feature list. It is that **verisectorium already has the right collision surface for content, and relata has the right membrane and ledger for support** — and the notes corpus had joined them only at the cousin-store / directory-as-table layer. Joining them at the **epistemic** layer is the real synthesis move.

### 6.2 Adjacent observations

- **Terminology + relata + claim segments** are three deployments of one family with different atoms (term / work-expression / claim). The missing shared module is not “another FORMAT” but **events + membrane + designator ladder + derived projections**.
- **Tribunal strand C** and **relata §7** are closer to each other than either is to “outline table format.” A future synthesis file might sit beside this one: `tribunal-and-calibration-as-one-spine`.
- **Lifecycle findings** (promotion ladder dead, WN queue) are exactly what you expect when dispositions and promotions leave no event store and no pending surface — relata’s UX is the counterfactual.
- **vox-vera / verbal probabilities** are a *presentation* calibration for confidence language; relata is a *decision* calibration for evidence combination. Both belong under VERA, at different layers; the notes gather lists both but does not yet stack them.
- Relata’s own BETTER-DATA doc is a model of **honest residual accounting** the verisectorium synthesis should copy when it claims methods.

### 6.3 On the brief

- **Strengths:** refused to hand a map; required full notes read before comparative pass; allowed reframing; canaries as background not checklist; deliverable location suggested not mandated.
- **Under-specified (mild):** “improve on the stuff already coming together” could mean doctrine, tooling, or claim landings — this report prioritized doctrine + transferable mechanisms over code plans.
- **Not extrusion:** peer framing held; no prescribed reading order beyond the explicit full-coverage constraint.
- **Missing context that would have helped slightly:** whether Joseph’s urgency was primarily for an upcoming VERA/tribunal write, for udon process claims, or for ASF promotion tooling. The report stays general enough for all three; a follow-up can narrow.

### 6.4 Coverage honesty for this survey

**Verisectorium notes:** indexes, all synthesis files, primary generalization, all udon-theory claims, core udon-analysis (logical model, living-docs seed head, tst-grounding head+conclusions, de-novo head, §12.2 extract), methodology, instances spectrum + missed synthesis + relata cousin copy, tribunal map + revisited + Gate 2, vera map + vox-vera heads + comproprium inventory. Large reference copies (full FORMAT, format.sop, PROPRIUM v2, ennaos 1257-line spec, tribunal TECHNICAL_ANALYSIS, full de-novo body) were read in substantial part or by structural sampling after inventory — load-bearing claims for *this* comparison were checked; line-by-line recitation of every precept segment and every ennaos subsection was not required for the transfer thesis and was not pretended.

**Relata:** README, CLAUDE, BETTER-DATA, TODO-ingest North Star + §7 + decision log pointers, sys docs for ledger/calibration/decider/verification/spool, evidence_ledger source head (single calibration site), architecture from code tree. Full 4800-line TODO-ingest session log and every test were not re-read end-to-end; open-item status is as of docs dated through mid-July 2026 and should be re-checked with `relata` live commands before operational claims.

---

## 7. Bottom line

Import from relata into the verisectorium program:

1. **Evidence vectors and event stores** under status/stage projections  
2. **Write-membrane speech acts** (rejected ≠ needs-review)  
3. **Soft refutation + absence discipline** aligned with collision + bare gaps  
4. **Independence rule** for multi-signal judgment  
5. **Defended seeds / fit-as-proposal** for any gate constants  
6. **Designator ladder** (never silent resolution)  
7. **Identifier-grade vs judgment-grade** auto-action boundary  
8. **Pending as operable queue**  
9. **Standing support audit** against silent lies  
10. **Observation-wide / model-careful** instrumentation for multi-timescale process  

Keep from verisectorium what relata does not replace: **present-truth claim atoms, authored exposition views, type vocabularies, and pedagogical multi-outline space.**

The synthesis is not “make claims like bibliography entries.” It is: **make belief about claims as honest, revisitable, and calibratable as relata makes belief about works — on top of the collision surface segments already provide.**
