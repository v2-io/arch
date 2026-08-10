# Record-kind taxonomy — first filled draft (coord, want-mode)

*A filled copy of [[TAXONOMY-DRAFT]] (2026-08-10): the coord's first pass at what we **want**, designed from the theory's first principles — not a summary of the survey, and deliberately free to differ from most or all surveyed practice. Every fill is `proposed`; the questions the filling surfaced are in Working Notes, which may be the more valuable half. The skeleton file stays pristine as the draft of record; content moves there as discussion settles it.*

*(Guiding Context is inherited from [[TAXONOMY-DRAFT]] unchanged and not repeated here.)*

## Taxonomy (provisional)

### Kind Families

#### Overview

*Design spine: a family is individuated by its failure-mode + adjudication-instrument pair (the generative-law candidate, here used as the design rule rather than asserted as law). Ordered by how much of the estate's truth rides on them.*

| Kind Family | Examples | Defining Concerns |
| ----------- | -------- | ----------------- |
| **Assertions** | claim, postulate, observation, derived/result, measurement | fails by being false or overclaimed; adjudicated by evidence + re-derivation → Evidence axis (+ Ceiling) |
| **Definitions** | def/term-group, term, lexicon entry, notation entry | dual failure: bad *coinage* (choice — Authority axis) and bad *fit-to-usage* (empirical — Evidence axis). Two axes on one kind, by design (resolves the def straddle) |
| **Decisions & Conventions** | decision entry, convention, ruling, scope call | fails by silent overturn or drift-from; adjudicated by authority events → Authority axis (event-projected, always) |
| **Norms & Directives** | normative, dir- directive, SOP, FORMAT/charter | fails by not binding or mis-guiding; adjudicated by authority + participant feedback → Authority + Posture + Freshness |
| **Practices** | praxis, workflow pattern, orchestration shape | fails by not firing when its moment comes; adjudicated by demonstrated firing → Efficacy axis |
| **Accounts** | exemplum (account/exchange/testimony/quotation/demonstration), spike record, experiment run, audit finding | fails by infidelity to what happened; adjudicated by witness position + fidelity-to-primary; append-only write semantics as a defining concern |
| **Prohibitions** | residue entry, forbidden term, deny-list entry, regression guard | fails by resurrection or over-blocking; adjudicated by refutation provenance → Usage-License axis + mandatory replace-with pointer |
| **Questions** | open question, demand, pending item, residual | fails by re-billing or silent loss; adjudicated by routing to a terminal state → Question-Lifecycle axis (the undecided dual of Authority) |
| **References** | ref entry, bib entry, citable primary pointer | fails by dangling or staleness; adjudicated by verification events + freshness → Currency + Conflict-Precedence |
| **Instruments** *(meta-records)* | manifest, navigator, ledger, process map, valve, adversarial review, standing brief | fails by lying about the status of what it tracks (the label-lie); adjudicated by re-derivation against the population (proxy discipline) → Charter + Derivation-Declaration |

*Deliberate exclusion: **Exposition** (discussion, aside, preamble, narrative connective) is designed OUT of the atom-kind families — framing prose is view property ( [[claim-outline-as-view]]), exposition weight is a projection concern, and a `disc-` segment is an Assertion at `discussion-grade` (declared non-load-bearing), not a separate kind. This is the want-mode answer to the exposition stress test; it must survive our own disc- segments (see WN-3).*

#### Universal (or nearly) Defining Concerns

- **Identity** — stable slug; only stable properties (kind) may ride in it; everything mutable stays out.
- **Kind declaration** — every record says what family it is, legibly to a fresh mind (the de-novo-auditor criterion applied at the record level).
- **Event trail** — who did what, when, against which criterion; every status cell is a projection of events, never a hand-set token (want-mode: this goes universal even where the survey shows it only four-generations-deep).
- **Process state** — a vector of resettable check-flags; explicitly not a ladder, never summed into a composite level.
- **Ownership/Maintenance** — who maintains the text (kit / other project / this instance), declared whenever the record rides in more than one place.
- **Freshness** — whether the record claims currency, and what resets that claim.
- **Coverage** — what the record fully vs partially carries; silent-full forbidden.

### Full Axis / Orthogonal Atom Concern Taxonomy

#### Overview

*Importance: `veritas-critical` (the type system fails without it) > `governing` (work routes wrongly without it) > `amplifying` (compounding convenience). Want-mode ruling: an axis earns `veritas-critical` only if the de-novo auditor needs it to know what to audit.*

| Axis/Concern | AKA | Description | Example Kinds | Example Fields | Importance |
| ------------ | --- | ----------- | ------------- | -------------- | ---------- |
| **Kind** | form, type (stable half) | which family + kind; the axis all others key off | all | `form:`, slug prefix | veritas-critical |
| **Evidence** | status, strength, support | how strongly held and by what kind of support; the trio cut (support-kind × strength) is the want | assertions, definitions (fit half) | `status:`, `support:`, `verified:` | veritas-critical |
| **Authority** | decided-by, ratification | who stands behind it, how firmly; projected from decision events | decisions, norms, definitions (coinage half), directives | `decided-by:`, authority cell | veritas-critical |
| **Ceiling** | max, max-attainable | per laddered axis: the highest rung this record could reach, from kind (+ ownership for authority); names the action that would raise it | any record on a laddered axis | `max:` | veritas-critical |
| **Process State** | stage (retired), checks, flags | resettable named-work-done flags; reset-on-edit | all | `state:`, check vector | veritas-critical |
| **Efficacy** | fired-status | demonstrated firing of a practice | practices | `:status fired` family | governing |
| **Witness Position** | attestation, fidelity | proximity to the primary event + verbatim-fidelity | accounts | `attested/reconstructed/secondhand`, `:date-confidence` | governing |
| **Usage License** | allowed-use, term-status | what use a record permits/forbids of its content | prohibitions, terms | "allowed as [HYPOTHESIS]", `forbidden` | governing |
| **Question Lifecycle** | open-state, disposition | progress of an undecided item toward a terminal state | questions, flux items | question-class, typed outcomes | governing |
| **Ownership** | maintenance, template-state | verisectorium- / other-project- / target-maintained | any templated record | `state: [template/override]` (today, conflated) | governing |
| **Freshness** | currency, staleness | decaying claim of currency; reset by underlying change | references, directives, instruments | `current`, verified-as-of dates | governing |
| **Grain** | per-clause state, scope-of-status | at what depth state attaches (record / section / clause / embedded term) + the projection rule upward | assertions, definitions | per-clause `:by`, FE-section maxes | governing |
| **Conflict Precedence** | arbiter-role | who wins on collision — arbiter / co-equal / defers | references, charters, instruments | "not the arbiter", "Appendix A wins" | governing |
| **Coverage** | carried-scope | full / partial-declared / extract | accounts, references, instruments | `coverage:` | governing |
| **Audience Safety** | priming, auditor-safe | which reader-roles a record is safe for (de-novo contamination) | instruments, front-door surfaces | AVOID lists, README-auditor split | amplifying |
| **Salience** | retrieval, hotness | when this should come to mind; study ranking | practices, norms, hazards | `should-come-to-mind-when`, ★ marks | amplifying |
| **Routing** | correspondence | whom a flux record is for/from/concerning | flux records, questions | `kind/for/from/concerns` | amplifying |

#### Principled Fields, Usage/SOPs, & Variations

##### Authority

###### Primary MECE States

Note that these are ordered by increasing weight of backing — same execute/revisit license from `supported` up; the rungs differ in the *intent* declared behind the record (steward re-cut, 2026-08-10).

| Rank | Tag/Key | State | Synonyms | Description | Importance |
| ---- | ------- | ----- | -------- | ----------- | ---------- |
| 1 | `proposed` | Proposed | draft, candidate | put forward by anyone; binds nothing | required rung |
| 2 | `supported` | Supported | trusted, provisional | steward/council trusts it as actionable-for-net-good without full review; easiest revisit | required rung |
| 3 | `ratified` | Ratified (steward/council) | approved, adopted | reviewed and stood behind after a real read | required rung |
| 4 | `ruled` | Ruled (steward/council) | fiat, decreed | an exercise of reserved authority; fiat marked as fiat | required rung |
| — | `defacto` | De-facto | unratified-in-use | operating as decided without ever being decided; recorded so the record exists | honesty state (off-ladder) |
| — | `transition` | Transition | being-fixed | rejected but still present somewhere; defacto-being-repaired | honesty state (off-ladder) |

###### Usage, Standard Operating Procedures, and Mix Variations

- Authority is **projected, never hand-set**: the cell is a denorm of the latest decision event (ledger primary; DECISIONS' decided-by vocabulary and this ladder are one vocabulary by design).
- A record's authority **ceiling derives from ownership**: target-maintained records can reach `ruled` locally; kit-maintained copies cap at `ratified` (you can ratify adopting what you don't own, not rule its text); an upstream change temporarily un-ratifies the copy.
- Composes with Evidence, never substitutes: on truth-apt content authority governs *adoption*, evidence governs *truth* — a `ruled` assertion is still `heuristic` if that's where its evidence sits.

##### Efficacy

###### Primary MECE States

Note that these are ordered by increasing demonstrated reliability-in-the-moment — adjudicated only by real firings, never by endorsement (a steward may *ratify* a practice's text; only events move this axis).

| Rank | Tag/Key | State | Synonyms | Description | Importance |
| ---- | ------- | ----- | -------- | ----------- | ---------- |
| 1 | `proposed` | Proposed | untried | designed but never observed to fire | required rung |
| 2 | `fired-once` | Fired once | validated-once | one recorded firing with its specimen | required rung |
| 3 | `fired` | Fired | proven, standing | fires reliably when its trigger occurs; specimens accrue | required rung |
| — | `failed-to-fire` | Failed to fire | missed | trigger occurred, practice did not fire — a finding, not a shame event | honesty state (off-ladder) |
| — | `retired` | Retired | superseded | deliberately stood down, reason recorded | terminal (off-ladder) |

###### Usage, Standard Operating Procedures, and Mix Variations

- Want-mode extension beyond the surveyed form: firings are **events with actors** — fired-for-author and fired-for-inheritor are different events (the surveyed FORMAT names this gap itself); the ladder rung is a projection over *whose* firings exist, and inheritor-firings are the ones that matter for a turnover corpus.
- `failed-to-fire` feeds the practice's revision the way a refutation feeds a claim — it is this axis's version of the strengthen-first trigger.

### Family, Kinds, Synonyms, and Concerns/Fields

| Family/Kind | Synonyms | Universal Axes | Defining Axes | Additional Axes |
| ----------- | -------- | -------------- | ------------- | --------------- |
| Assertions | claims, truth-apt segments | all universals | Evidence, Ceiling, Grain | Salience |
| Definitions | defs, terms, vocabulary | all universals | Authority (coinage) + Evidence (fit) | Usage License, Grain |
| Decisions & Conventions | rulings, ledger entries | all universals | Authority | Question Lifecycle (via re-open) |
| Norms & Directives | SOPs, dir-, charters | all universals | Authority, Freshness | Salience, Audience Safety, Conflict Precedence |
| Practices | praxes, patterns | all universals | Efficacy | Salience |
| Accounts | exempla, spikes, runs, findings | all universals | Witness Position, Coverage | Audience Safety |
| Prohibitions | residue, forbidden, deny-lists | all universals | Usage License | Evidence (of the refutation) |
| Questions | opens, demands, pendings | all universals | Question Lifecycle, Routing | Salience |
| References | refs, bib entries, pointers | all universals | Freshness, Conflict Precedence | Coverage |
| Instruments | manifests, navigators, ledgers, maps | all universals | Conflict Precedence, Freshness + charter (a concern, not an axis) | Audience Safety, Routing |

## Working Notes

*The questions this fill surfaced — likely the more valuable half of the document:*

- **WN-1 (Evidence axis deliberately not templated yet).** I filled Authority and Efficacy as the two exemplar axis-sections and *stopped* before Evidence — because its MECE-states table forces the biggest open call: one strength ladder with a named ordering ("cost to overturn"?) vs the trio's two-dimensional cut (support-kind × strength) where no total order exists. The skeleton's "ordered by increasing ((something))" is exactly what the estate's ladders never declared, and choosing the ordering *is* the epistemology decision PRACTICA holds as deliberately open. Wants its own discussion round, possibly the joint one.
- **WN-2 (the def straddle, want-mode answer).** Definitions carry two axes at once: Authority on the coinage, Evidence on fit-to-usage. Fresh defs are honestly `proposed`+unmeasured; a term can be `ratified` yet fit-poor (rename pressure), or fit-proven yet never ratified. Does this dual survive contact, or is it over-engineering for a kind most deployments status with one field?
- **WN-3 (exposition excluded — the riskiest call).** Designing exposition out of the families (view-property + discussion-grade assertions) contradicts asf's aside/sketch/detail types and our own `disc-` prefix. If a disc- segment is just an assertion at discussion-grade, the `disc-` prefix marks a *rung*, not a kind — which violates the stable-identity rule we ourselves wrote. Either the exclusion is wrong, or `disc-` is a legacy prefix the taxonomy should eventually retire. Flagged, unresolved.
- **WN-4 (Prohibitions: family or profile?).** A residue entry is arguably an Assertion (of refutedness) whose defining axis is Usage License. I kept the family because its failure mode (resurrection) and instrument (refutation provenance + replace-with) are genuinely distinct — but if the generative law is applied strictly, "distinct axis profile" and "distinct family" may be the same statement (E3 again, now with a concrete test case).
- **WN-5 (Ceiling generalized).** Want-mode ruling embedded above: every laddered axis takes a ceiling, and authority ceilings derive from ownership (the template's Max column already behaves this way). Means `Max` is not an evidence-only concept — the map's §5 needs updating if this holds.
- **WN-6 (event trail promoted to universal).** The survey shows events four-generations-deep but nowhere universal. Making "every status cell is a projection of events" universal is the single largest divergence from surveyed practice in this fill — it implies tooling debt everywhere. Is that the want, priced honestly?
- **WN-7 (charter as concern-not-axis).** Instruments' "charter sentence" (append-only / refresh-in-place / lagging-index) is a defining *concern* with no MECE ladder — first live specimen of the concern⊃axis distinction the skeleton's working note anticipated.
- **WN-8 (the missing write-semantics column).** Write semantics (replacement / append-only / generated) appears in family definitions but got no axis row — it's kind-intrinsic, not state. Confirm it belongs in the kind-declaration schema rather than the axis table, or give it a row.
- **WN-9 (auditor walk pending).** The de-novo-auditor criterion hasn't been exercised against this fill: the natural test is walking one corpus (this one, or paths) asking only "what should be audited?" from kind + carried state per this table. The stopping points become the next round's fills or refutations.
