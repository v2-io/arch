# Verisectorium plan — canonical outline

*2026-08-05 (Joseph + Claude). This is the outline of **the very last ad-hoc verisectorium**: we use the pattern here for its plain utility in launching the project — deliberately NOT as an exemplary or reference instance. The segments directory is named [`last-adhoc-src/`](last-adhoc-src/) on purpose. When the principled instantiation machinery (ch. 11) exists, this instance is a natural candidate for migration through exactly the process ch. 4's migration gap describes. This instance also deliberately trials some tradeoff variations (no type prefixes on slugs; expected-vs-actual type columns; state first) — it is gathering data.*

*Local conventions live in [`ONTOLOGY.un`](ONTOLOGY.un) — column meanings, slug/assembly rules, gap semantics, expected segment shape. Short form: filename = slug in `last-adhoc-src/`, ordering lives here, cross-references use [[slug]], `state` is an independent flag starting `proposed` (first iteration of the row on the outline — not definitive-and-awaiting-draft), and a `--GAP--` in the Tag column means we don't know how many segments that region will eventually be — the Summary names the main topic or thing to establish.*

*Verisectorium: the recognition that one pattern keeps establishing itself between Joseph and agents because it is a **solution to 100% agentic turnover in highly complex, living creation projects** — and that its principles stand no matter the substrate. Gathered material awaiting processing: [`INFLUX/00-INDEX.md`](INFLUX/00-INDEX.md).*

---

## Part I — Basics

### Chapter 1 — Atom: segments, records, & their inner structure

| State    | Expected<br>Type | Type | Tag                             | Summary                                                                                                                                                                 |
| -------- | ---------------- | ---- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| proposed | def              |      | [[atom]]                        | An atom is a typed record with stable slug identity, a present-truth body, and declared companions                                                                      |
| proposed | form             |      | [[slug-identity]]               | Slug = identity is the pattern's one proven move-proof invariant; identity never carries order or path                                                                  |
| drafted  | emp              | norm | [[identities-over-locations]]   | References carry identities (slug / slug+section+span), never paths, positions, or presentation numbers (specimen: [[provenance-rot-specimen]])                         |
| drafted | form | form | [[atom-as-cluster]] | An atom is a cluster — body, working notes, events, companions — on different clocks; layout and visibility are separate questions, and events are forced by two independent constraints |
| drafted | form | form | [[write-semantics-declaration]] | Replace-vs-append is a per-type declaration, not a corpus-wide law; role (record mapping) and regime (write rules) are two axes that must stay apart |
| proposed | form             |      | [[atom-grain-parallelism]]      | Atom grain is chosen partly for parallelism: fact-grain atoms with their own confidence bands enable delegation and piecewise truth-propagation                         |
|          |                  |      | --GAP--                         | Inner-section schema: cadence vocabularies; section-vs-frontmatter epistemics (the vivarium / udon-needs principled disagreement); machine-readable qualifier placement |
|          |                  |      | --GAP--                         | Restatements, families, aliases, and same-claim-different-expression links (rename survival; sibling claims linked, never silently merged)                              |

### Chapter 2 — Order: high-level structure, outlines, & projections

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| proposed | form | | [[outline-as-organizing-principle]] | The outline is the organizing principle and front-door, not an index — it carries importance, flow, and the glue |
| proposed | form | | [[outline-as-cognition]] | Reordering and gap-declaring in the outline is critical thinking about the bigger picture — cognition agents cannot perform over concatenated segments |
| proposed | form | | [[multiple-views]] | Multiple views per substrate, at different rigor levels for different needs, sharing one atom set |
| drafted | form | form | [[view-edge-metadata]] | View-local metadata is an attribute of the membership edge; authored views are canonical about their own fields, generated ones about nothing (specimen: [[stage-denorm-zero-drift]]) |
| proposed | form | | [[dependency-order-tension]] | When a view's order must validate against atom dependencies, and what an accepted violation is (relation-keyed exception stores) |
| drafted | form | form | [[appendix-placement]] | Appendices sit at the bottom of the outline even when they are DAG dependencies — reader's path and verifier's path run opposite directions |
| proposed | norm | | [[absence-as-structure]] | Missing structure is marked visibly (gap rows, residue stores, censuses) without predicting inventory — absence handling is what actually rots estate-wide |
| | | | --GAP-- | Hotness / importance ranking feeding the outline so study-order keeps outline flow (the view half lives here; methodology is [[hotness-methodology]], ch. 14) |
| | | | --GAP-- | The estate layer: cross-instance references, many verisectoria per program, relocation without rot, concept coherence across members |

### Chapter 3 — Process I: steady-state, tracking, & sidecars

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| proposed | form | | [[working-notes-sidecar]] | Working Notes is the ubiquitous per-atom sidecar — forward residue, not history-dump — present in nearly every instance under some name |
| proposed | norm | | [[working-note-lifecycle]] | What earns a note, how notes retire, and why retirement backlog is where steady-state is won or lost |
| drafted | form | form | [[state-flags-not-gates]] | Independent resettable state flags (explored / drafted / checked…), not a one-way promotion ladder (specimen: [[ladder-never-fired]]) |
| proposed | form | | [[tracking-altitudes]] | Tracking layers by altitude: navigator → items → cross-cutting proposals; histories and archaeologies kept separate |
| proposed | form | | [[sidecar-conventions]] | BASENAME / organic-expansion conventions: one callable name, many manifestations over time, sidecars as their own basenames |
| drafted | form | form | [[decision-records]] | Decisions are their own artifact kind: confidence separate from content, load path named, revisit-when / expires-on carried, coupled hypothesis explicit where one exists |
| drafted | form | form | [[warrant-over-authority]] | Warrant is structurally primary over authority — agents dogmatize whatever the schema makes first-class, so the schema must privilege the truth-serving column |
| drafted | form | form | [[asked-and-answered]] | Open flags need asked-and-answered states so they stop repeat-billing every fresh agent |
| proposed | form | | [[history-layer]] | History is a governed layer: changelog vs frozen archaeology, retired-name discipline, record-grain history verbs agents don't reach for unaided |
| | | | --GAP-- | Terminology / lexicon / notation substores: per-term records, append-only decision events, generated views, bounded-context sharing across the estate — critical components, immature even in asf |

### Chapter 4 — Process II: flux, integration, & the real gates

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| drafted | form | form | [[influx-queues]] | "You erred" and "I am unsure" are different speech acts; typed residue must be stored, not reported (specimen: [[queue-typing-specimen]]) |
| proposed | form | | [[integration-metabolism]] | Archive-everything-then-adjudicate-only-improvements: the cycle that ends rediscovery chaos and orders discoveries automatically |
| proposed | form | | [[observable-crossings]] | Layer crossings should be recorded events, so integration backlog is a countable queue rather than a vibe |
| drafted | form | form | [[layer-speeds]] | Fast strata feed slow canon; rate failure and sensitivity failure are separate, early promotion is priced not forbidden, and separation can be bought on a cadence |
| proposed | norm | | [[gates-need-destinations]] | A gate exists only where there is a destination — an assurance something qualifies to be moved or treated differently; everything else is state |
| | | | --GAP-- | Whole-outline migration: moving an outline's worth of segments to a different verisectorium — the real gates plus "this snapshot is frozen history, superseded at …"; three partial estate answers not yet put side by side |

### Chapter 5 — Onboarding: orientation, coordination, & role activation

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| proposed | form | | [[orientation-scaffold]] | Orientation as a general scaffold with slot-in points for instance-specific SOPs |
| drafted | form | form | [[orientation-gate]] | Where stakes warrant, free-read of the outline + a cheap check before write access — pass dies with the context (specimen: [[outline-skipping-failure]]) |
| drafted | norm | norm | [[priming-discipline]] | Auditor-safe and verdict-bearing surfaces are declared and read in that order; attention is spendable and the spending is irreversible |
| proposed | form | | [[role-activation]] | Natural role activation: what makes an instance work must ride its front-door composition, not accumulate accidentally across files and memories |
| proposed | form | | [[work-coordination]] | Division of work over the atom grain: delegation, peer-voice briefs, harvester briefs, handoff routing (write-safety is ch. 13) |
| | | | --GAP-- | Onboarding for specific recurring roles (de-novo auditor, harvester, integrator, reviewer) with their differing read and priming needs |

### Chapter 6 — Releasing: tagging, versioning, & lifecycle

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| proposed | form | | [[lifecycle-events]] | Tagging, versioning, checkpointing; freeze-and-supersede as a first-class lifecycle event |
| proposed | form | | [[living-after-publication]] | Publication is a projection with gates at flux points, not an exit — submitted papers stay living through revisions and backports |
| proposed | form | | [[pipeline-seams]] | Publishing pipelines (phanero stages) meet the verisectorium at seams/fluxes; what crosses each seam is an adjudicated package |
| | | | --GAP-- | Ad-hoc views on demand; snapshot semantics; what a released artifact promises about its source segments |

---

## Part II — Deep dives

### Chapter 7 — Underlying principles

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| drafted | form | form | [[turnover-solution]] | The pattern is a solution to 100% agentic turnover in complex living projects — the core claim, stated precisely |
| drafted | form | form | [[session-cycle-ergonomics]] | Agents get concrete start/work/finish cycles without the completion ethos infecting the never-finishable whole |
| drafted | form | form | [[collision-staleness-detection]] | Present-truth statements can collide; append-only history cannot — collision is the staleness detector |
| drafted | disc | disc | [[first-principles-grounding]] | Honest AAT/TST grounding: identification conditions and transfer obligations at stated tiers — never sloganized compression (the deleted-P*-layer cautionary record) |
| drafted | form | form | [[substrate-independence]] | The principles stand regardless of implementation technology (markdown, udon, …) |
| | | | --GAP-- | Why it works even applied ad-hoc and unevenly — the latent-need account; which properties carry most of the effect |

### Chapter 8 — Truth & judgment

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| proposed | survey | | [[strength-ladders]] | At least three incompatible strength ladders live in the wild; none is a superset — the adjudication this project owes |
| proposed | form | | [[epistemic-axes]] | Strength × support-kind × convergence (independent failure modes) vs the single status scalar |
| proposed | form | | [[confidence-calibration]] | Confidence bands and calibration language — the VERA lineage (PROPRIUM qualified truths, ennaos architecture research, vox-vera verbal probabilities, comproprium precepts) |
| drafted | form | form | [[tribunal-record]] | Tribunal processes: typed voices with independent failure modes; the durable deliberation record, not the verdict, is the product |
| drafted | form | form | [[discussion-probes]] | Ungrounded-but-plausible interpretive claims are worse than gaps — the paragraph-level probe: derives / hypothesis+falsifier / post-hoc nothing |
| drafted | form | form | [[evidence-ledgers]] | One log-odds sum, priors held apart, absence ≠ refutation, correlated signals counted once, gate constants at one site with a defended chain |
| drafted | form | form | [[verification-provenance]] | A dangling citation is a truth-status defect; "verified" is derived from events; gating is the deployment's call and evidence-grade is a second axis (specimen: [[gate-profile-divergence]]) |
| proposed | form | | [[temporal-truth]] | Temporal truth factors: revisit criteria, expiry, re-grading records that predate their governance, check-resets on edit |
| | | | --GAP-- | Bayesian propagation across the dependency DAG (how a changed premise re-prices its dependents); special epistemic states |

### Chapter 9 — Pedagogy & causality

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| proposed | form | | [[experiential-reading]] | Predict → read → record the delta → wander → predict next: deep comprehension, causal-gap detection agents otherwise cannot do, wondering-notes as harvestable pedagogy |
| proposed | form | | [[dag-vs-exposition]] | The dependency-DAG vs exposition-order tension as a pedagogical question, and what agents' silent self-repair hides from them |
| proposed | form | | [[plain-language-briefs]] | Feynman-criterion briefs: a thoughtful non-specialist can re-derive the qualitative claim from the analog alone |
| proposed | form | | [[pedagogy-layering]] | Respectful pedagogy: mental model first, precision second; the analog must be isomorphic, not merely evocative |
| | | | --GAP-- | Bridge/gloss segment kinds; misconception censuses harvested from wondering-notes; pedagogy's seam with onboarding (ch. 5) |

### Chapter 10 — Complex pipelines

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| | | | --GAP-- | Parking lot for the unknown complexities ahead: multi-instance flows, adjudicated review-response packages and their AAT backports, harness integration, … little notes land here until they earn structure |

---

## Part III — Meta & implementation concerns

### Chapter 11 — Instantiation: the generator

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| proposed | form | | [[generator]] | Answer key questions → everything needed is built → point agents at it and let loose: the central deliverable |
| proposed | norm | | [[derivation-not-templates]] | Every generated choice traces to a principle (ch. 7) — customization is derivation, not template-picking |
| proposed | form | | [[instance-profiles]] | Profiles over the Part I concerns: atom vocabulary, view set, state-flag set, flux membranes, orientation scaffold, release wiring |
| | | | --GAP-- | The question-set itself; migration of existing ad-hoc instances (including this one) through the generator |

### Chapter 12 — Publishing & rendering mechanics

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| proposed | form | | [[derived-vs-authored]] | Derived artifacts are declared derived by construction (clobber guards, naming conventions, banners) — never hand-editable-looking |
| proposed | form | | [[multi-renderer-constraints]] | One source feeding several renderers imposes notation constraints on the substrate (the single-`$` class of rules) |
| proposed | form | | [[semantic-indexing]] | Search/discovery indexing wants declared boundaries: chunking, identity, and context that travels with a fragment |
| | | | --GAP-- | Low-level lint/format mechanics (fmt-md, lint-md, render-equality gates); storage layouts; build-implementation surveys |

### Chapter 13 — Concurrency mechanics

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| drafted | form | form | [[write-safety]] | One record per placement + atomic replace; no concurrent RMW of multi-record files without a membrane (specimen: [[lost-update-hazard]]) |
| drafted | form | form | [[partition-isolation]] | Isolation comes from layout, not locks; the git-conflict reassurance is structurally false for multi-record files, so record mapping must be declared |
| | | | --GAP-- | Same-key contention as surfaced judgment; single-writer membranes; what the git layer does and does not catch |

### Chapter 14 — Metrology

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| proposed | form | | [[corpus-instruments]] | Instruments that watch the corpus: segment-stats, health checks, lint censuses — with the taught-blindness hazard (instruments report clean over what they were told to ignore) |
| proposed | form | | [[hotness-methodology]] | Segment hotness / importance ranking methodology (consumed by ch. 2 views and ch. 5 orientation) |
| proposed | form | | [[observation-stores]] | Record process features wide before weights are known; the model is a consumer (feeds ch. 8 calibration) |
| | | | --GAP-- | What a standard health report contains; metrology for views and for process (time-in-state, crossing rates) |

### Chapter 15 — Steward interface

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| proposed | form | | [[steward-surfaces]] | What the steward actually reads: big-picture views over largely unread segments — by design, not neglect |
| proposed | form | | [[authority-routing]] | Decision/escalation queues and delegated grants: authority routes without becoming a bottleneck |
| | | | --GAP-- | Where steward attention is genuinely load-bearing vs delegable; council patterns; the steward's own tracking surfaces |

---

## *Appendices* Specimens & surveys

*Raw supporting material, per [[appendix-placement]]: cited by the claims that stand on it, placed here so the chapters stay readable. Each appendix row is expected to be referenced by at least one chapter-level `emp`/`form` claim; rows below not yet referenced are awaiting their claim rewrite (grok pass in flight).*

| State | Expected<br>Type | Type | Tag | Summary |
|---|---|---|---|---|
| drafted | obs | obs | [[provenance-rot-specimen]] | Full account: comproprium reorg broke 106/109 path-anchored spans, 18/18 slug refs survived; the checker was disabled by the move it existed to catch (grounds [[identities-over-locations]]) |
| drafted | obs | obs | [[ladder-never-fired]] | Full count: vivarium 115/115 draft; ASF mid-ladder but 0 format-clean/candidate (grounds [[state-flags-not-gates]]) |
| drafted | obs | obs | [[type-vocabulary-locality]] | The shape is portable; the type vocabulary is not, and should not be |
| drafted | survey | survey | [[view-genres]] | The view family observed live: pedagogical outlines, build manifests, narrative views, views over not-yet-existing segments, nested outlines |
| drafted | obs | obs | [[outline-skipping-failure]] | Full account: vivarium gate + asf audit/orientation SOPs treat outline-skip as known failure (grounds [[orientation-gate]]) |
| drafted | obs | obs | [[lost-update-hazard]] | Full account: concurrent RMW of multi-record files drops records without a git merge conflict (grounds [[write-safety]]; specimen vivarium DECISIONS log) |
| drafted | obs | obs | [[coupling-confounding]] | Co-change measurement is confounded by the layout it would justify; only directional asymmetries carry causal weight |
| drafted | survey | survey | [[tribunal-strand-survey]] | Four distinct designs called "the epistemic tribunal," separately citable — they share a name and a motive, not a design |
| drafted | survey | survey | [[rationale-capture-survey]] | Design-rationale prior art (IBIS/SEURAT/ADR) with verification registers preserved, and the capture-problem corpse (grounds [[decision-records]], [[tribunal-record]]) |
| drafted | obs | obs | [[authority-flag-specimen]] | Full account: an agent's eager provenance check aimed at the wrong column, re-raised by several agents, on a record predating its governance (grounds [[warrant-over-authority]], [[asked-and-answered]]) |
| drafted | obs | obs | [[stage-denorm-zero-drift]] | ASF's checked-but-untrusted duplicated `stage`: 0 mismatches over 168 compared rows (grounds [[view-edge-metadata]]) |
| drafted | obs | obs | [[gate-profile-divergence]] | One store design, two enforcement regimes: 13/188 vs 70/97 verification coverage on identical machinery (grounds [[verification-provenance]]) |

---

## Open ordering & seam questions

- **Ch. 2 ↔ ch. 6/12 seam**: ch. 2 owns which content in what order with what view-local meaning; ch. 6/12 own turning a chosen view into artifacts. Boundary object: the neurips manifest (row selection = ch. 2; `\appendix` injection = ch. 12).
- **Ch. 15 placement**: some steward material (queues, adjudication points) may migrate into ch. 3 / ch. 4 as segments get drafted.
- **Cross-cutting topics** (concurrency, instrumentation, decision records) deliberately appear in multiple chapters as distinct segments cross-referenced by slug — that is the pattern working, not duplication; watch for genuine forks.
- Whether this outline stays an organizing principle or proves principled enough to be more.
