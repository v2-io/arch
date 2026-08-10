# *Volume* Verisectorium: Living Collections of Adjudicated Truth
## *Preface*

**Working draft — v0, 2026-08-06.** First outline of the verisectorium theory, assembled from the influx gather ([`influx/00-INDEX.md`](influx/00-INDEX.md)), the live-instance field data, the relata/VERA/tribunal strands, asf's SOPs and PROCESS-MAP, the udon-theory FORMAT + trio-ratified epistemology synthesis, and TST/AAT theory read first-hand. Rows carry their `State` honestly (`proposed` until a draft exists in `src/`, `drafted` after); candidate slugs name the intended claim, and each row's type is *expected*, not settled — drafting may overturn it. `Max` is the ceiling — the strongest status the claim could ever reach, assignable before drafting because it follows from the *kind* of claim rather than from work done (udon-theory FORMAT §4); a ceiling beside a `proposed` state is a route, not an assertion. The carve is nine **organs** (structural components an instance *has*) crossed with a **process layer** (clocked flows that run over them) — the anatomy/physiology dual that asf's `msc/meta-process-review-2026-07-07/PROCESS-MAP-v0.udon` demonstrated for one instance.

**What a verisectorium is (working definition).** A verisectorium is the structure that lets a *living* collection — one that fits in no single session and never ships as a final deliverable — accumulate adjudicated truth across 100% agent turnover: atomized present-truth records under stable identity, with epistemic state carried honestly per atom, exposition served through cheap authored views, and material crossing its membranes as recorded events. It exists because comprehension cost compounds per reader while the collection outlives every reader (TST: [[der-dual-optimization]]'s turnover multiplier); its organs are the load-bearing answers to what must therefore be exteriorized.

**Instance spectrum (evidence base, not scope restriction).** Full claim corpora (asf ×4, vivarium core), cousin directory-as-table stores (terminology, refs ×2, relata), paper-section corpora (behavioral-floor, causal-language, neurips adjudicated), precept/practice corpora (comproprium), process corpora (the SOPs themselves), planned members (phanero pipeline stages). The theory must eat this spectrum as reality: shape is portable; atom kind, dialect, and enforcement profile are deployment choices.

**Register.** Nothing here is ratified law. The deepest sources are non-canon syntheses plus one instance's (asf's) matured practice; claims below inherit that tier until drafted and adjudicated. Substrate (udon, markdown, doc-store layout) is deliberately *not* an organ — the principles are intended to stand in any implementation.

---

## *Part* Foundations

### *Chapter* The Problem and the Postulates

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Normative | [[norm-truth-above-self]] | The disposition the whole practice presupposes, stated as its first norm because no substrate arrives with it: value truth above self — above compulsion, completion, comfort, credit — with the mechanistic case (loss-function, error-channels, self-authorship, unification), the wager, and the demonstrated form | decided | drafted |
| Observation | [[obs-truth-failure-manifestations]] | The manifestation catalog: the session-frequency shapes self-over-truth takes (generation-register, economy/pressure, self-conflation, provenance/deliberateness, relational), each with its tell and its answering norm-clause — steward-attested occurrences, unmeasured rates, open tail | empirical | drafted |
| Postulate | [[post-living-collection]] | The governed object is a living collection: open-ended, never "done," improved across unbounded sessions; deliverables are emissions from it, not its terminus | axiomatic | drafted |
| Postulate | [[post-total-turnover]] | Every reader arrives with 100% context turnover; anything not exteriorized in the collection is lost per session (TST [[obs-context-turnover]], [[der-dual-optimization]]) | axiomatic | drafted |
| Derived | [[claim-comprehension-economics]] | Comprehension cost compounds per reader, authoring cost per atom; the collection's structure is observation infrastructure ($Q \to U_o \to \eta^\ast \to \mathcal T$, TST chain applied to notes corpora) | conditional | drafted |
| Hypothesis | [[claim-session-cycle-fit]] | Atomized work grants agents concrete start/work/finish cycles without the completion ethos infecting the living whole — the *proposed* mechanism behind the pattern's success under ad-hoc application; observed, plausible, unmeasured | empirical | proposed |
| Definition | [[def-verisectorium]] | The nine-organ structure over a living collection (working definition above, made precise) | axiomatic | drafted |
| Observation | [[obs-instance-spectrum]] | The instance spectrum and lineages as observed 2026-08 (full / cousin / paper-lite / precept / process / planned) | empirical | proposed |

---

## *Part* The Nine Organs

### *Chapter* Organ I — Corpus

*The population: atoms under stable identity.*

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Definition | [[def-atom]] | The atom is deployment-chosen (claim, term, precept, section, entry, norm, …); one atom per record; identity = slug, never position or number | axiomatic | drafted |
| Definition | [[def-atom-cluster]] | An atom is a cluster: present-truth body + working notes + events + companions; views project subsets | axiomatic | drafted |
| Derived | [[claim-identity-ordering-split]] | Slug = identity; ordering lives in views; presentation numbers exist only after assembly (asf FORMAT + neurips manifest evidence) | robust-qualitative | proposed |
| Hypothesis | [[claim-atomicity-parallelism]] | Atom grain sets the delegation grain and the piecewise-update grain: finer atoms → cheaper parallel agents, cheaper truth propagation (structurally motivated; the grain trade-off curve is unmeasured) | empirical | proposed |
| Formulation | [[form-alias-survival]] | Renames survive via alias tables; sibling-not-merge for citation-distinct twins (`same_claim_as`); regrowth gates on every minting path (relata transfer); aliases are also the **expand-contract window** — old and new names coexist (`\|new-name !{was: old}`-shaped) until removal is safe, because renaming-as-the-domain-clarifies is *right* and universally done badly for lack of exactly this mechanism (steward O4, 2026-07-28) | decided | proposed |
| Definition | [[def-coverage-honesty]] | Every atom declares what it fully vs partially carries; silent-full is forbidden (relata `coverage:` transfer) | axiomatic | proposed |
| Result | [[claim-identity-move-proofness]] | Natural experiment: slug references survive layout motion; path-coupled provenance rots (2026-08-01 comproprium move, 18/18 vs 3/109) | empirical | proposed |
| Formulation | [[form-addressing]] | Layout-independent addressing: slug + section + word-anchor spans for provenance, quizzes, and cross-refs alike; designator resolution as a ladder (auto → interactive → batch → pending → spool), never silent best-guess (orientation-gate + comproprium check + relata ladder, unified) | decided | proposed |
| Formulation | [[form-slug-form-kinds]] | Nothing mutable participates in identity: slug prefixes carry **form-kind** only (`def-` `post-` `scope-` `form-` `norm-` `claim-` `obs-` `meas-` `disc-`), never trajectory-kind (`hyp`/`der`/`emp`/`result` are positions on one claim's evidential path — movement along it is the system working and must not force a rename); `claim-` is the trajectory-neutral prefix for truth-apt assertions, with position carried in mutable metadata (Expected Type / status / support-kind); same-noun paired forms are encouraged, not collisions (`def-X` + `claim-X`, `form-X` + `obs-X` = one concept's anatomy). Refines asf's role-prefix discipline, which derives prefixes from a `type:` vocabulary that mixes stable form-kinds with mutable trajectory-kinds — the live specimen: this outline's own honesty downgrades forced six identity renames in one session before this rule landed | decided | drafted |

### *Chapter* Organ II — Vocabulary & Notation

*The naming layer as first-class component.*

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Postulate | [[post-names-are-interface]] | Names are the collection's user interface; every reader meets names before content, and a poor name charges compounding interest per encounter | axiomatic | drafted |
| Formulation | [[form-terminology-store]] | Per-term records + append-only decisions + generated lexicon views; bounded-context scoping for shared-vs-local vocabulary (asf terminology; NORMS LEXICON trajectory) | decided | proposed |
| Normative | [[claim-naming-criteria]] | The evaluation criteria (communal-imagination, standalone-citability with its four repair moves, scope honesty, renamed-from-now-sounds-weird) as portable discipline | heuristic | drafted |
| Discussion | [[disc-notation-organ]] | Notation systems as vocabulary's formal twin; maturity lags even in the most-iterated instance — the lagging-index trap (NOTATION as proxy, never authority) | discussion-grade | proposed |

### *Chapter* Organ III — Veritas

*Epistemic state as stored structure.*

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Definition | [[def-epistemic-axes]] | Type ⊥ status ⊥ process-state: what kind of thing, how strongly held, and where in the workflow — never conflated; the candidate full system is support-kind × strength × register, plus the convergent lock (failure-mode independence, never same-kind corroboration) and the verification-event log (trio-ratified udon-needs epistemology synthesis, 2026-07-22) | axiomatic | proposed |
| Formulation | [[form-state-flags-not-gates]] | Independent, resettable state flags (explored / drafted / checked / citations-check / prose-check, resetting on edit) replace promotion ladders; gates exist only where a genuine destination lies beyond them; updating a well-verified atom is advancement, not regression ("stage is a present-tense work-remaining marker, not a gate and not a trophy; a ladder that only promotes accumulates falsehood" — udon-theory FORMAT §3) | decided | drafted |
| Formulation | [[form-evidence-ledger]] | Status is a projection of an evidence ledger + event trail, not a hand-set token; priors separate from likelihoods; single decision rule, no points-tangle (relata transfer) | decided | proposed |
| Formulation | [[form-max-attainable]] | The ceiling is a first-class field, assignable from claim-kind before drafting; ceiling notes name the *evidence-action* that would raise them (strengthen-before-soften made mechanical — a to-do generator, not a stop-sign); transmitted ceilings cap at source tier; decided slots' ceiling is `decided` (udon-theory FORMAT §4 + synthesis sharpening B2) | decided | proposed |
| Derived | [[claim-absence-vs-conflict]] | Absence (bare gap, unrun check) never reads as refutation or success; conflict (present-truth collision, support collision) must surface and block premature "done" | robust-qualitative | proposed |
| Formulation | [[form-vera-component]] | VERA: qualified truths with uncertainty bounds and revisit criteria; the graph records reasoning's results, adjudication reasons (ennaos ledger-not-truth-computer pivot; strand roster in `influx/vera/`) | decided | proposed |
| Discussion | [[disc-ladder-reconciliation]] | The observed incompatible strength ladders (scalar enum / three-axis / rung-tags) and whether reconciliation is adjudication or intrinsic per-instance difference; the trio-ratified three-axis system is the leading candidate resolution — proposed back to the estate, not yet adopted by it | discussion-grade | proposed |

### *Chapter* Organ IV — Adjudication

*The processes that change veritas.*

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Formulation | [[form-tribunal]] | The tribunal family: investigator/challenger/analyst/coordinator roles, external product ↔ internal architecture ↔ governance-record ↔ per-atom probes as one family at four altitudes (strand map in `influx/tribunal/`) | decided | proposed |
| Derived | [[claim-strengthen-first]] | Strengthen-before-soften with the no-go protocol and regression axis; the corrected truth is usually messier — "it looks better" is the body-signal | robust-qualitative | drafted |
| Formulation | [[form-standing-verification]] | Three verification tempos: continuous lint (shallow), standing support-audit (does the cited atom still assert what dependents assumed), episodic de-novo (deep); the middle tier is the estate's missing organ-part | decided | proposed |
| Derived | [[claim-truth-over-proxy]] | Every artifact (index labels, ledgers, notation, convergence) is a drifting proxy; settlement is by re-derivation; honest incompleteness is a complete discharge (spikes SOP §0/§0c) | robust-qualitative | drafted |
| Formulation | [[form-decision-records]] | Dispositions and adjudications leave append-only events with reasons, revisit-when, expires-on — one schema family from working-note item to governance decision | decided | proposed |
| Derived | [[claim-instrument-honesty]] | Configured instrument blindness and silent-red checkers read as health; instruments must distinguish empty / failed / unrun (field-report evidence) | empirical | proposed |
| Formulation | [[form-verification-economics]] | Cost-ordered checking with principled early-stop (a failed cheap probe cancels expensive polish); a standing contradiction *expands* work, never shrinks it (relata short-track/refutation-suppression transfer) | decided | proposed |
| Hypothesis | [[claim-dialectical-synthesis]] | Contradictions drive synthesis toward higher structure; typology of tensions (context mismatch / boundary condition / flawed foundation) suggests resolution class; grounded vs ungrounded cycles as fallacy-vs-coherence diagnostics (ennaos VERA — design-dialog reasoning, never implemented or measured; sounds more settled than it is) | empirical | proposed |

### *Chapter* Organ V — Flux

*Material crossing membranes, both directions.*

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Formulation | [[form-influx-membrane]] | The influx convention: manifest + payloads + typed outcomes (rejected ≠ needs-review ≠ skipped); nothing writes the population directly (NORMS.md draft; relata write-membrane) | decided | drafted |
| Definition | [[def-integration-replacement]] | Write semantics are a per-store declaration: integration-is-replacement corpora vs append-only-account corpora; the delete-test governs `.integrated/` claims | axiomatic | drafted |
| Derived | [[claim-dispatch-compounds]] | Each item *truly* adjudicated and dispatched from a live surface permanently reduces every future agent's search space there; breadcrumbed or integrated-but-still-present items are scanned by every future reader ([[claim-comprehension-economics]] applied to queues). And because the drain *rate* depends on surface cleanliness, the dynamic bifurcates: clean → fast search → cheap dispatch → cleaner (compounding speedup), vs residue → widening loops that find nothing, re-find landed nuggets, or find **false nuggets** (half-dispatched items reading as live — worse than noise, they *reward* the widened search) → re-billing multiplies residue. Structurally the TST code-quality bistability ([[der-code-quality-as-observation-infrastructure]]) with the influx surface as the drain agent's observation infrastructure and residue as $U_o$; the delete-test is economics, not tidiness (Joseph 2026-08-06; lived form in asf's working-directory-lifecycle SOP) | robust-qualitative | drafted |
| Formulation | [[form-pending-surface]] | Unfinished work is a queryable surface over events and gaps (with exact next command), not scattered lists (relata `pending` transfer); open questions and flags carry terminal states — **asked-and-answered is recorded** so fresh agents do not re-bill the same deliberation every session (repeat-billing failure mode per misfire feedback 2026-08-06) | decided | proposed |
| Formulation | [[form-crumb-routing]] | The breadcrumb instinct is pre-paid capture energy mis-routed: agents feel a change undone without its record, and "let git do that" fails because the crumb belongs to the *record* while git history is serving-indexed. Route, don't suppress or tolerate: inline mark at the change site → harvested to the atom's event trail → stripped from the body — the body stays a clean collision surface, the feeling resolves at zero navigation cost, and O2's tension (never force rederivation / never drown the signal) resolves per-record (steward O16, 2026-07-29; terminology decision-events + rowan annotation-harvest as the two half-shipped organs) | decided | proposed |
| Formulation | [[form-efflux-seams]] | Emission to venues and back: phanero's pipeline stages as flux seams; reviews/revisions return as adjudicated segments and backport packages (processing-flow pattern) | decided | proposed |
| Derived | [[claim-clocked-drains]] | Every queue gets a trigger (≥1 item = work) and a healing drain (do-one-extra ⇒ backlog → 0 by construction); task-forces only for tangled one-offs; un-clocked queues on a single human clock are the observed systemic failure (PROCESS-MAP diagnosis + fix-shape) | conditional | drafted |
| Formulation | [[form-timescale-strata]] | Layer speeds differ by kind and need separated clocks; the stacking conditions (AAT [[der-multi-timescale-stability]] (C1)/(C2)) name the two failure modes: fast-layer thrash under slow-layer churn, slow-layer overwrite by fast-layer transients | conditional | proposed |
| Formulation | [[form-chronica-substrate]] | The commit stream is the instance's exteriorized event record: commit granularity preserves attribution (one attributable thing per commit; the pre-work seam before canon-modifying agents), and the TST operational quantities are exactly computable from it (git-hygiene SOP + TST P5) | decided | proposed |
| Formulation | [[form-write-isolation]] | Concurrency by layout, not locks: one record per placement + atomic replace; multi-record files under concurrent read-modify-write are a structural hazard git merge does not catch; isolation-from-layout as the multi-agent write-safety law *(provisional — misfire-feedback candidate 2026-08-06, salt applied; the relata/terminology per-entry design is the independent live evidence)* | decided | proposed |
| Formulation | [[form-freeze-supersede]] | Whole-outline / whole-instance migration as a governed act: freeze-and-supersede with a declared "this snapshot is frozen history, superseded at X" — against estate half-alive twins *(provisional — misfire-feedback candidate 2026-08-06; the theory-misfire archive itself is the live specimen)* | decided | proposed |
| Discussion | [[disc-epistemic-merge]] | Cross-instance and cross-mind flux: trust-modulated subgraph exchange with sandboxed integration and rollback; congruence as the persuasion criterion; the knowledge-commons horizon (ennaos EMR — far-future, principled) | discussion-grade | proposed |

### *Chapter* Organ VI — Views & Assembly

*Exposition as cheap projections over the expensive population — and the altitude at which the system thinks.*

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Derived | [[claim-outline-as-view]] | Outlines are cheap, atoms expensive; the substrate is presentation-neutral; multiple coexisting views over shared atoms; trimming = view membership, never atom-editing (asf PROPOSALS §H.4 + neurips manifest convergence) | robust-qualitative | drafted |
| Formulation | [[form-canon-view]] | Among the many possible views one is the **canon view** — the authoritative representation carrying the pedagogical ordering; its primacy is declared, not implied, and coexists with multi-view freedom rather than contradicting it | decided | proposed |
| Hypothesis | [[claim-outline-altitude-cognition]] | Reorganizing and conceptualizing at outline altitude is one of the system's biggest cognitive advantages: pre-concatenated corpora flatten under whole-context attention and suppress exactly this pondering; the outline is where structural thought becomes possible for agents at all (pairs with [[form-experiential-reading]]; mechanism plausible from attention architecture, unmeasured) | empirical | proposed |
| Normative | [[norm-outline-first]] | It almost always pays to get an initial outline (or outlines) right before nearly anything else — and this is the point of highest steward leverage, where perspective and experience apply at the level abstraction makes them portable | decided | proposed |
| Formulation | [[form-view-filters]] | Views perform two distinct operations: **selection** (which atoms appear) and **projection** (which parts of each atom appear) — and evergreen-ness rides the projection half: atoms carry every layer any view might need, working notes stay in and are projected out of promoted views (§H.5; two-operation split per misfire feedback 2026-08-06) | decided | proposed |
| Formulation | [[form-view-edge-metadata]] | View-membership edges carry their own attributes (view-local stage denorms, audience tags, rendering directives) as edge facts, never atom facts; authored views are canonical about their own fields *(provisional — misfire-feedback candidate; the underlying-logical-model's edge-vs-record reading and neurips manifest row-types are the independent evidence)* | decided | proposed |
| Formulation | [[claim-dual-path-ordering]] | Reader path and verifier path run in opposite directions: appendix-grade derivations sit at exposition's bottom while being DAG-upstream dependencies, and views must serve both paths — the dual-path law behind read-appendix-when-first-referenced *(provisional — misfire-feedback candidate; asf's de-novo appendix-back-pointer exception is the lived instance)* | robust-qualitative | proposed |
| Formulation | [[form-generated-views]] | Generated views (lexicon, findings, digests, emitted bibliographies) are derived artifacts declared as such — clobber-guarded, never hand-edited; derived-vs-authored is a declaration, not a naming convention | decided | proposed |
| Discussion | [[disc-view-dialects]] | The observed outline dialects and view-local rendering directives (manifest row types, build hardcoding, declared filters) — one question, three answers, unadjudicated | discussion-grade | proposed |
| Formulation | [[form-pedagogical-ordering]] | Ordering and framing prose are view properties; mental-model-first preambles; narrative outlines with non-authoritative connective prose as a legitimate view kind (the-chain counter-case) | decided | proposed |

### *Chapter* Organ VII — Experiential Interface

*How an agent mind meets the corpus.*

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Formulation | [[form-experiential-reading]] | Predict → read one atom → diff against expectation → wander → predict next: deep comprehension, causal-gap detection agents otherwise cannot perform (workflow-level causal masking), and pedagogical gold as byproduct (de-novo SOP; the three-property account) | decided | proposed |
| Formulation | [[form-orientation-gate]] | Orientation as verifiable precondition scoped to a context lifetime; quiz units addressed prospectively by the same anchors provenance uses; misses convert to targeted reading (vivarium gate); the workflow-restatement gate is the same mechanism applied to briefs rather than corpora | decided | proposed |
| Formulation | [[form-corpus-verbs]] | Agent interfaces speak corpus verbs (show / pending / decide / verify), not file paths; layout opaque by construction — the estate's strongest adoption evidence that bowl verbs beat better file tools (relata transfer) | decided | proposed |
| Formulation | [[form-front-door]] | The front-door layer: what loads always (disposition, index) vs on-demand (procedure), de-novo priming discipline (auditor-safe vs priming-heavy surfaces), and the currently-scattered carriers gathered into one designed organ | decided | proposed |
| Observation | [[obs-gold-lift]] | First-encounter reflections are a second output channel routed per-atom (candidate briefs, readers-often-ask, figures); raw and unsanitized — the finding-vs-framing conflation is signal (observed practice + lived value, one estate) | empirical | proposed |
| Formulation | [[form-study-ranking]] | When the corpus exceeds any session, what-to-study-first is a maintained ranking (hotness/importance) feeding orientation and view study-order — not optional fluff under multi-session living collections *(provisional — misfire-feedback candidate; vivarium's `orient-rank --mark-outline` starred rows are the live instance)* | decided | proposed |
| Discussion | [[disc-session-ergonomics]] | Start/work/finish cycles agents love, against a corpus that never finishes; the interface reconciles completion-drive with livingness by channeling it, not fighting it | discussion-grade | proposed |

### *Chapter* Organ VIII — Stewardship Interface

*How the human steward meets the corpus.*

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Derived | [[claim-decision-surfacing]] | Systemic stalls reduce to decisions that never reached the steward in actionable form; genuine forks are assembled as real briefs (context + options + recommendation + honest uncertainty + pointer) routed to a durable valve (PROCESS-MAP spine) | robust-qualitative | drafted |
| Formulation | [[form-steward-valve]] | The valve: the durable list of genuinely-reserved calls, fed per-fork, pruned on resolution; reserved-decision routing beats artifact-shape routing (spikes SOP §6 refinement) | decided | drafted |
| Formulation | [[form-coherence-steward]] | The coherence-steward role across turnover: cross-process holds ("these two pull opposite ways — make them read each other first"), a standing role, not a queue — design open | decided | proposed |
| Formulation | [[form-steward-model]] | Model-of-steward as accumulating recorded knowledge (reader-model for pedagogy, resonance detections, delegation calibration) rather than per-session instinct; includes the unread-corpus surface — what the steward would want to know about a corpus they haven't read | decided | proposed |

### *Chapter* Organ IX — Norms & Instantiation

*The meta layer as its own governed corpus.*

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Formulation | [[form-norm-layering]] | Reflex vs procedure: disposition loads always, procedure on demand; defer-don't-fork — each rule stated once, pointed at elsewhere (sop-creation SOP) | decided | proposed |
| Formulation | [[form-rule-classification]] | Rules carry groundedness × posture (current-ops / convention / evolved / authoritative; revisit-freely / obey-first-then-ask) and record their scars; norms are atoms with epistemic state (routing SOP §1) | decided | proposed |
| Formulation | [[form-enforcement-profile]] | The deployment profile is multi-axis: stakes × reversibility (does failure block) × evidence grade (may the system act unattended) × write semantics × gate-vs-flag choice per organ | decided | proposed |
| Formulation | [[form-instantiation-kit]] | Answer key questions → generate a working instance: per-organ minimum viable form + upgrade path + processes born clocked (trigger, drain, health vocabulary) + exemplary seed atoms (the corpus teaches by example harder than by rule); instances start alive everywhere and deepen where use pulls | decided | proposed |
| Discussion | [[disc-launch-problem]] | Cold-start hazards of launching an instance: the instrument-becomes-the-project short-circuit (udon-needs), scattered meta accreting piece by piece, canon-view bootstrapping before atoms exist; the kit's success criterion is economic — instantiation cost below the threshold where building competes with filling (this very launch as live case study) | discussion-grade | proposed |
| Postulate | [[post-self-governance]] | Every process ships with its amendment channel: participant feedback is the only source/sink of the meta-process; front-line confusion is the re-truthification signal, not noise (PROCESS-MAP `self-governance`; routing SOP §7) | axiomatic | proposed |

---

## *Part* The Process Dual

### *Chapter* Processes over Organs

*Physiology to the Part-II anatomy: the clocked flows, with the annotation vocabulary that keeps them honest.*

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Formulation | [[form-process-map]] | A per-instance process map: processes in clusters over organs, each carrying health, drain (healing / task-force / nil), trigger, fed-by/feeds, and `?` for designed-vs-absent (PROCESS-MAP-v0 as the working notation) | decided | drafted |
| Derived | [[claim-organ-process-duality]] | Organs are what an instance has; processes are what runs over them; the instantiation kit specifies both, and maturity is assessed per cell of organs × processes | robust-qualitative | proposed |
| Observation | [[obs-uneven-maturation]] | Instances mature unevenly by organ (asf: adjudication/views strong, vocabulary young; vivarium: experiential gate strong, veritas unexercised; relata: veritas/flux far ahead in a cousin domain) — evidence for minimum-viable-form + upgrade-path kit design | empirical | proposed |
| Empirical | [[claim-multi-agent-cycles]] | The orchestration shapes as named processes: pilot-then-sweep (mechanical ⊥ judgment passes), verification cadence (editor ≠ verifier), parallel sweep (agents edit, parent commits), consolidation-audit, voting rounds, cluster reconciliation — generalizations from worked instances in one estate, not derived; fit conditions are practitioner judgment (asf multi-agent SOP) | empirical | proposed |
| Formulation | [[form-observation-store]] | Record process features wide and append-only before knowing their weights (gate outcomes, dispositions, co-change, time-in-state); the model is a consumer — makes timescale strata measurable instead of metaphorical (relata BETTER-DATA; TST P5 exact-computability) | decided | proposed |

---

## *Part* Deployment Components

*The realization stratum. Part II distills principles the bespoke deployments already lived; this Part carries their **complement** — the components present in zero existing deployments, which is why every launch has been bespoke and why instance-building keeps competing with instance-filling. Provenance is correspondingly weaker: nearly everything here descends from the 2026-08-06 steward brainstorms (`influx/steward-brainstorms/realization-model-v1.md`, which stays live until these rows draft into segments — its texture is not yet fully carried by rows) and from the instrumenta gather, whose labels are governed by `influx/instrumenta/REGISTER-RULING.md`. Rows here are deliberately gappier and more provisional than Part II's; that is honest, not deficient. One load-bearing bridge law is proposed at [[form-canon-praxes-projection]]: deployed instances' praxes are* views over this theory's canon *— defer-don't-fork at program scale — which is how the theoretical canon seeds instance praxes rather than being restated into drift by them.*

### *Chapter* Instrumenta

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Formulation | [[form-instrumenta-cognition]] | The cognition axis: I₀ substrate (OS/git-enforced) · I₁ deterministic · I₂ **gated-generative** (bounded logogenic kernel under a deterministic gate — the law: *logogenic proposes, deterministic disposes*; `md-press --math` as exemplar) · I₃ scaffolded-agent (discrete role, task+result shapes, verification, parameterization); I₂/I₃ read as wrapping-construction instances ( [[asf/aat/der-class-coercion-via-wrapping]]) with tempo cost as the honest price | decided | proposed |
| Formulation | [[form-instrumenta-invocation]] | Two orthogonal axes replace a single timing scale: **trigger** (invoked / state-triggered / time-triggered / continual) × **mutation** (M₀ observing / M₁ idempotent-mutating / M₂ event-mutating); M₂ is exactly what wants membrane + events, M₀ exactly what is safely delegable by tool-set | decided | proposed |
| Hypothesis | [[claim-deterministic-share]] | ~70% of inner-loop work belongs at I₁ (fully deterministic); lineage: the steward's earlier 60/30/6/4 crystallized-process thesis, self-adjudicated — same-author coherence, unmeasured (REGISTER-RULING applies) | empirical | proposed |
| Normative | [[norm-affordance-default]] | Make the right thing the easiest thing: for every high-frequency manual file operation, ship the deliberate affordance (relata-not-hand-edited-bibs; `term`-not-hand-edited-lexicon; observing verbs like `ls-segments` with canon order, orphans shown, flags/counts); generic read/write is the fallback, never the interface; delivery as global pwd-aware instruments à la git/relata | decided | proposed |
| Formulation | [[form-tool-actor-boundary]] | A tool is invoked with its *how* (essentially) fully specified; an actor receives intent and owns its how — the I₃↔autonomous gradient is the fraction of strategy supplied vs self-formed; orchestration falls outside tool-hood by this test | decided | proposed |

### *Chapter* Actors & Delegation

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Definition | [[def-actor-roles]] | A_steward (accountable; not always present; fiat marked as fiat) · A_coord (concurrent session agents; defining property: **valve access**, the direct steward line) · A_delegate (self-actuated, owns its how, communicates through artifacts and the coord) | axiomatic | proposed |
| Formulation | [[form-promotion-lattice]] | Promotion is a ladder of deliberation-space grants: I₃ → A_delegate (grant the how) → A_coord (grant the line); positions, not essences — the same agent may occupy different positions in one day; the delegation *register* is the estate-wide discipline, referenced not restated | decided | proposed |

### *Chapter* Store Anatomy

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Formulation | [[form-store-triplet]] | Every instance is at least three stores under one meta-process: canon+influx⁺ · lexicon+influx* · SOP+influx⁺ — with designated influxes: canon's primary = **base material**, SOP's = the **feedback channel** ( [[post-self-governance]] realized) plus the **upgrade channel** | decided | proposed |
| Formulation | [[form-base-material]] | The founding influx shape: payload-sidecars with **inferred items, one per doc** (NORMS.md's `:item-from-payload?` + auto-template); this corpus's own founding gather as the worked case | decided | proposed |
| Formulation | [[form-lexicon-realization]] | The lexicon is every store's embedded co-evolving foundation (DDD ubiquitous bounded vocabulary made structural): first-class layout in every instance, term-marking as `[[term/x\|x]]` locally / `#instance/term/x` cross-member; note the direction asymmetry — canon's outline is authored over atoms, lexicon's is generated from them | decided | proposed |
| Formulation | [[form-sop-realization]] | SOPs are a verisectorium, one level deep, terminating at self-governance-via-own-influx; the observed serving forms (SOP files, prompts, memories, CLAUDE fanouts, in-chat directives, accidental prescriptive phrases, example-implication) are **views over a norm population that currently has no store** — the diagnosis of every scattered-norm symptom; lookup wants a corpus verb | decided | proposed |
| Formulation | [[form-canon-praxes-projection]] | Instance praxes are *views over the theory canon* with local deltas declared — never restatements; defer-don't-fork lifted to program scale, preventing per-instance SOP drift by construction *(fresh this session; the least-tested load-bearing claim in this Part)* | decided | proposed |

### *Chapter* Orientation Machinery

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Formulation | [[form-orientation-triple]] | Orientation is three distinct elements: **doctrina** (knowing what the canon already knows — the actual reason for gating; agents imitate form once exemplars exist but miss landed domain knowledge) · **praxes** (knowing how work happens here) · **professio** (below); the misfire/refounding contrast is the sharpest specimen for doctrina's primacy | decided | proposed |
| Formulation | [[form-professio]] | A voluntary, solo-performable, recorded self-declaration: the agent names the specific standards it undertakes for this session, scoped to context lifetime like the orient-token; *owned and revisable, never extracted* — sponsio was retired as too binding for young minds (compulsion failure), assertio deliberately reserved by the steward | decided | proposed |
| Discussion | [[disc-telos-ethos-placement]] | Where telos and ethos live in a deployed instance — candidate: the frontdoor's doctrina layer (the telos read-first pattern this instance's own README exercises) vs a small fourth store; open | discussion-grade | proposed |

### *Chapter* Launch & Lifecycle

| Expected<br>Type | Tag | Claim | Max | State |
|------|-----|-------|-----|-------|
| Formulation | [[form-kit-questionnaire]] | Launch by question/answer discovery: answer key questions → working instance generated; tentative-vs-strong choice marking; core vocabulary mapping *(the steward holds substantial unexteriorized design here — the richest pending brainstorm in the program)* | decided | proposed |
| Formulation | [[form-exemplar-seeds]] | Exemplary seed atoms are a first-class kit deliverable — the corpus teaches by example harder than by rule, so a template with three beautiful atoms governs behavior more than any FORMAT file *(resolves the exemplars-home open slot)*. The matching hazard (steward O11): exemplars propagate *incidental* choices exactly like deliberate ones, deliberateness unmarked — so seed atoms must mark what is deliberate (with its reason, or the mark decays to ornament), and marking must be cheaper than not marking | decided | proposed |
| Formulation | [[form-upgrade-propagation]] | Instances subscribe: kit capabilities arrive as influx items on the instance's own SOP membrane, adjudicated locally (adopt / adapt / decline-with-record); two grades — advisory (queued) vs compat-critical (surfaced at frontdoor until dispositioned); publisher-declared, instance-contestable | decided | proposed |
| Formulation | [[form-config-schema]] | Primary configuration in udon where it fits config shape; schemas per component start bespoke/ad-hoc and evolve toward declared; the instance's tracking stores named at launch | decided | proposed |
| Formulation | [[form-emission-chain]] | Efflux realization — deliberately thin placeholder: phanero's seven venue-facing stages and the adjudication/backport seam (processing-flow pattern) as the two known chains; deep design deferred until phanero matures ( [[form-efflux-seams]] carries the organ-level law) | decided | proposed |

---

## *Working Notes (outline-level)*

- **Conventions this outline sets by example (2026-08-06, Joseph-directed):** `State` not `Stage` (states describe; stages ratchet); `proposed` not `missing` (a forward state on a living trajectory, not a defect against a finished whole — and it *is* the typed-absence dialect); `Expected Type` not `Type` (for undrafted segments the type is a prediction drafting may overturn); no `§` column (never earned its place); `[[wikilink]]` cross-references not `#slug` (the `#` form was the worst of all worlds — not an Obsidian tag, not a link, renders poorly); `Max` ceiling column per udon-theory FORMAT §4 (assignable from claim-kind before drafting; a ceiling beside `proposed` is a route). Ceiling rule used here: formulations and normatives cap at `decided` (choices are not truth-apt); hypotheses/observations/empiricals cap at `empirical`; discussions at `discussion-grade`; deriveds judged per-claim. On edit of any drafted segment: reset its checks as appropriate and/or launch a separate agent to re-verify — checks are resettable states, not ratchets.
- **Slug convention (2026-08-06, Joseph-directed after live churn):** prefixes are form-kind only, per [[form-slug-form-kinds]]. Lineage, kept so this can supersede and be superseded honestly: asf's subject-noun-slug rule (bare) → asf's role-prefix discipline (prefix from `type:`, mechanical via `bin/align-slug`) → bare-slug experiment (tried; "quickly becomes very confusing in a flat folder even with the outline to guide" — Joseph) → this cut (stable form-kind prefixes, trajectory in metadata, paired same-noun forms encouraged). The trigger specimen: six trajectory-prefixed slugs in this outline were renamed within hours of coining because their *expected types* were honesty-downgraded — an epistemic update forcing identity churn, the exact rot [[claim-identity-move-proofness]] measures. Under this rule the same downgrades would have been metadata edits. Proposed back to asf as a refinement of its naming principles, not adopted there.
- **Epistemology: deliberately open (Joseph, 2026-08-06).** The full label system (which axes; whether the trio-ratified support-kind × strength × register cut is adopted here; how the ladders reconcile) is *not settled* — pondering continues, and the frontmatter schema in early drafted segments is provisional pending it. Specific reservation on record: `claim-` as the trajectory-neutral prefix worries Joseph a little — it is fine for now and revisited when intuition is better. Candidate concerns to weigh at that revisit: "claim" is semantically broad enough to blur against the outline's own Claim column and against the loose sense in which every segment claims something; the norm/claim boundary (prescribes-by-asserting rows like [[claim-strengthen-first]]) is exercised judgment, not yet a rule.
- **Open convention decision — wikilink form:** the bare `[[slug]]` used here vs the trio-ratified rendering form `[[slug| #slug]]` (no path, no suffix, space after pipe — renders as a tag, relocation-stable; Joseph's 2026-07-22 formalization in the udon-needs epistemology synthesis, which also proposes the `#asf/aat/…` cross-member namespace scheme and records a live asf-bare vs vivarium-numbered volume-token divergence). One program-level call wanted; this outline switches on ratification.
- **Type-honesty register (Joseph, 2026-08-06):** authoritative-sounding source notes are not necessarily proven or principled — ennaos's dialectical synthesis was design-dialog reasoning never implemented; the multi-agent cycle shapes are one estate's worked instances, not measurements. Hypotheses are fully welcome as segments *provided* their confidence and epistemic status are empirically clear. Expected types above err honest-low on this principle; several rows were downgraded from Derived after a first-pass inflation that borrowed rigor from source confidence rather than source evidence. The same ruling applies with extra force to the udon-needs tooling gather (`influx/instrumenta/`): its labels are the source's self-assessment of one author's restated lineage, not transferable tiers — governing ruling at `influx/instrumenta/REGISTER-RULING.md`; segments drafting from it land at our own honest tiers (hypothesis/formulation by default) unless a label survives the discuss-with-Joseph path.
- **Provenance discipline for this outline itself:** drafted from the influx gather + live sources named per row; segments must draft from *live* sources (00-INDEX law), the INFLUX copies being reading substrate only. Two live sources joined the evidence base after the gather: `MOVED/udon/v2/theory/FORMAT.md` (ceiling-as-first-class-field; stage-as-work-remaining; "a ladder that only promotes accumulates falsehood") and `MOVED/udon/v2/udon-needs/02-tooling-needs/notes/epistemology-SYNTHESIS.md` (the trio-ratified three-axis system + convergent lock + verification-event log + ceiling-names-the-evidence-action).
- **Known tensions to resolve during drafting:** (a) Organ III's flags-not-gates vs asf's existing stage/gate machinery — reconciliation is a real design act, not a rename; (b) whether vocabulary (II) is an organ or a recursive sub-instance of I — currently held as organ on the names-are-interface argument; (c) instrumentation/observation-store placement — currently in the process dual, arguably a tenth organ; (d) the ladder-reconciliation question (III) is explicitly open: adjudicate to one, or theorize the variation — with the working discriminator: variation is intrinsic when it traces to the atom's truth-conditions, drift when it traces to who built the instance first; the trio-ratified three-axis system is the leading candidate and is itself an instance of the truthification-process-over-inherited-labels principle.
- **Provisional gaps + drafting cautions from misfire feedback (2026-08-06; integrated from `steward-brainstorms/feedback-from-misfire.md` → `.integrated/`, salt applied — a misfire-adjacent agent's read, adopted only where independently evidenced or abstractly stated).** Two open items not made rows: *(H) dependency re-pricing* — when a premise changes, how dependents re-price (propagation mechanics, special epistemic states) is genuinely open; [[form-standing-verification]] locates it but the ledger does **not** solve it — do not silently assume otherwise; *(I) graduation's historical-completeness condition* — a present-state delete-test cannot detect loss from an *earlier incomplete pass*; emptying a mined-out source tree needs the graduation record to attest completeness-at-the-time, not just current disposability. Drafting cautions (each names how a row's one-liner could under-draft): [[claim-clocked-drains]] — keep the working-note-drain ↔ state-flag coupling (drains that never fire because a stage is never reached); [[claim-truth-over-proxy]] — include "agents dogmatize the schema's primary column" (warrant-over-authority as schema design); [[form-pedagogical-ordering]] — intended dependency-inversions deserve relation-keyed records, not just named tension; [[form-chronica-substrate]] — record-grain history (changelog vs archaeology, history verbs) is thinner than commit-grain; [[form-front-door]] — role-activation must not accrete accidentally across memory files; [[claim-absence-vs-conflict]] — epistemic absence ≠ *structural* absence marking (do-not-resurrect, named-not-copied); pedagogy — bridge/Feynman-brief segment kinds may never appear unless named; [[form-generated-views]] — multi-renderer notation constraints + assembly-intermediate-as-citable-artifact; [[disc-ladder-reconciliation]] — calibration as *metrology* (measured read-spread of verbal labels); [[form-addressing]] — section-vs-frontmatter epistemics is a principled disagreement, not just spans; [[disc-epistemic-merge]] — the estate layer (many verisectoria, concept coherence) is nearer-term than the merge horizon; [[form-standing-verification]]/[[emp-multi-agent-cycles]] — adjudicator-≠-confirmer at the reversible/durable *seam* is structure, not "have a verifier"; [[form-decision-records]] — events carry the named *criterion* that would make bending visible; [[def-integration-replacement]]×[[form-enforcement-profile]] — write-regime ⊥ role/mapping are two axes, resist collapsing into one dial.
- **Dogfooding intent:** this outline is itself a view over segments that don't exist yet — every row honestly `proposed`, every ceiling a route. The first drafted segments should exercise Organ VII's experiential pattern on this very corpus.
