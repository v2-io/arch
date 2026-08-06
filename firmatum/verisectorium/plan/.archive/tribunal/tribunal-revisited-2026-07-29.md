<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: firmatum/udon/v2/theory/to-integrate/refine-more/epistemic-tribunal-revisited.md (Fable 2026-07-29)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/arch/firmatum/udon/v2/theory/to-integrate/refine-more/epistemic-tribunal-revisited.md
  Do not edit here expecting to update the live original.
-->

# The epistemic tribunal, revisited

*2026-07-29, Fable — laid down the evening Joseph described it, while fresh. Register: §1 is Joseph's description, verbatim where quoted, steward-experiential; §2–§3 are my connections, proposed; §4 is verified external evidence (cites the same-day rationale sweep); §5 carries Joseph's core-schema suspicion at his own stated register — "I suspect" — pre-validation, deciding nothing. Every `udon` block ran through the current parser before landing here.*

---

## 1. The structure, as Joseph described it

A design he worked about a year ago — his gloss now: "almost dullfully obvious nowadays" (§4 disputes that), "an early attempt at something qualitatively different than 'grounding by citing the internet.'" The roles, near-verbatim:

> Basically there was the **advocate**, the **red-team**, a **neutral observer** who looked at potential motivation and systematic biases etc. that the two teams might not be aware of otherwise, sometimes a **risk-analysis specific analyzer** who could give inputs into which options etc. could fail in which ways (orthogonal to the pro/con teams), and a final **adjudication node** that would receive all three+ inputs before outlining **decisions, distinct from confidences**, and what new information and so forth would potentially make the decision **revisitable** etc. — basically analytically determining **which arguments were load-bearing and by what degree** and therefore building in its own **falsification / revisit / expires-on criteria**.

And the payoff he named, which is the design's actual target:

> …it gives sound **"intentional vs incidental" *with* reasoning** that easily incorporates new understanding and new data, instead of those new findings somehow being a threat to the current mental model or set of implicit/latent decisions mixed altogether.

Lived practice, not hypothesis (steward-experiential): recent vivarium decisions have been decided *by council* with steward support rather than ratified by Joseph directly — "and everyone has been the happier for it. I teach what true principles I can, and they govern themselves."

Two structural observations worth pinning to the description (mine, *proposed*):

- **The record is the product.** Multi-agent debate is commonplace now; what remains rare is treating the *deliberation itself* as the durable artifact. The adjudicator's output — decision distinct from confidence, load paths, revisit conditions — is governance-as-record, not grounding-as-process. A decision that knows which arguments it stands on can absorb a refutation *locally*; a latent decision blended into everything defends itself totally or collapses totally. That is the anti-calcification mechanism made architectural, and it is why new findings stop arriving as threats.
- **Revisit-criteria are O15 one altitude up.** Expectation bounds on operations and expires-on criteria on decisions are the same move — attach the falsifiable expectation to the act, before the outcome exists — at the edit layer and the governance layer respectively. One family: *pre-registered revisitability*.
- **Expiry triggers make information-integration measurable and tractable** (Joseph's sharpening, same conversation). Without pre-registered triggers, integrating a new finding means re-evaluating *everything* — diffuse, unbounded, so it silently doesn't happen (the latent-decision failure his description names). With them, integration becomes a **query followed by a bounded work set**: "which revisit conditions does this datum satisfy?" fires a computable subset of decisions for re-adjudication, and nothing else needs touching. Two trigger classes with different costs: date-based (`:expires-on` — trivially census-able; overdue-count is a number) and condition-based (`:revisit-when` — the richer class, and §4.1 shows SEURAT already mechanized its cheapest form: an assumption-ground being disabled *is* the trigger firing). The measurability is the O7 move applied to belief-maintenance: integration debt stops being a vibe and becomes a countable queue — fired / pending / overdue — which is what makes "the decisions know what they are" an operable property rather than an aspiration.
- **A decision is often coupled to a hypothesis, and when it is, the coupling should be explicit** (Joseph's refinement, same conversation). Triggers have two sides: **premises going in** (assumptions — if disabled, revisit; SEURAT mechanized exactly this side in 2004) and **predicted outcomes coming back** (empirical results that would trigger revisit). Falsification criteria is the broader category capturing both — but in practice, his observation, the hypothesis form is the easier handle: *"I'm making this decision because I hypothesize that X will happen"* — and the stated hypothesis then **generates** its falsification criteria rather than someone authoring them cold. Two consequences worth pinning: RATSpeak's four grounds (requirement / assumption / claim / alternative) contain no predicted-outcome ground — the hypothesis side appears to be *another* gap in the walked record, not just a refinement of ours; and explicit hypotheses are what make the adjudicator's `:confidence` **calibratable** — a corpus of hypothesis-coupled decisions is a corpus of scored predictions over time (relata's predicted-vs-actual loop, at the governance layer), which is what turns a council from a decision procedure into a *learning* one. The sketch's missing child, parse-checked:

```udon
|adjudication :by council :confidence 0.6
  :decision per-collection default with per-file override
  |hypothesis[h1].outcome
    :expect role declarations keep drift-within-table rare in practice
    :falsified-if two role-declared corpora exhibit f1 within a quarter
    :check-by <2026-10-15>
```

  — with `:revisit-when` then *derivable* from the hypotheses it stands on, rather than hand-written beside them. (O15 symmetry, third appearance: the expectation bound on an operation is an outcome-hypothesis at micro scale; the decision-hypothesis is the same object at governance scale.)

## 2. The connection to tiers of voice

UDON's founding pedagogy (the README's *Tiers of Voice*) is that a document holds multiple layers of expression — prose, comments, elements, inline structure, attributes, dynamics — **coexisting without collapsing into each other**. The tribunal is the same principle applied to deliberation: each role is a *voice with a distinct epistemic job and a distinct failure mode*, and the record's whole value depends on the voices staying typed rather than blending into a summary.

The mapping is tighter than analogy (*proposed*):

- **Register-collapse is the shared enemy.** A con blended into decision prose loses its provenance; a confidence stated in the decision's voice becomes the decision. The estate's whole hand-register discipline (status banners, pre-validation marks, "no lean") exists because blended voices rot — and the tribunal record is that discipline generated *structurally*, one element per voice.
- **The voices are chosen for independent failure modes** — advocate (motivated construction), red-team (motivated destruction), observer (the biases *both* share — the single-author caution as a node), risk analyzer (failure shapes orthogonal to the pro/con axis entirely). That is the convergent-lock rule (support-kinds with independent failure modes) *institutionalized as org-chart*: the adjudicator can weigh the inputs precisely because it knows their failure modes differ.
- **Decision distinct from confidence** is the two-labels-route-to-different-repairs test passing: a wrong decision is repaired by re-adjudication; a wrong confidence is repaired by calibration. Fusing them was the scalar-confidence mistake (record the vector, not the verdict) at the governance layer.
- **The O11 byproduct.** Whatever passed through the tribunal is *intentional by construction, with its reasoning attached*; what didn't is incidental by default. The deliberateness bits that O11 says training will never supply — the tribunal emits them as exhaust.

## 3. What it looks like with udon stylings

*Illustrative spelling only — no syntax is proposed here; the element vocabulary is the content.* This parses today (checked; the envelope date draws the specified no-dialects interim warning):

```udon
|deliberation[file-roles-declaration].council :status adjudicated :date 2026-07-29
  :question Are file roles declared per-file, or per-collection with override?
  |advocate :by agent-a
    |argument[a1].precedent :weight 0.8
      Designator conventions already carry mapping information de facto,
      and the O10 rungs make filename-declaration the governance on-ramp.
  |red-team :by agent-b
    |attack[r1] :targets a1 :kind silent-confound
      A designator can lie and nothing re-checks it; the probe found
      role-relevant drift invisible to any per-file declaration.
  |observer :by agent-c
    |bias[o1] :kind sampling
      Both teams argue only from this estate's corpora; external
      multi-tenant tables were never examined by either.
  |risk-analysis :by agent-d
    |failure-mode[f1] :option per-file :shape drift-within-table
      Files in one directory silently disagreeing about their own mapping.
    |failure-mode[f2] :option per-collection :shape stale-manifest
  |adjudication :by council :confidence 0.6
    :decision per-collection default with per-file override
    :load-bearing [a1 r1]
    :revisit-when a second estate adopts roles and f1 is observed live
    :expires-on <2026-10-15>
```

What the format contributes, rung by rung: the voices are **elements** (typed, addressable, extractable — `grep '|attack'` is a query); identity makes arguments **citable across records** (`:targets a1`; a future deliberation can `:supersedes` a specific argument, not a whole document); `:load-bearing [a1 r1]` makes the load path **data** — the thing that lets a new finding attack one leg; the confidence is an attribute *of the adjudication*, not of the decision; and `:revisit-when` / `:expires-on` are the prospective-falsification columns. Note also what the estate already has: vivarium's council decisions land in `DECISIONS.decision-log.udon` as `|decision[slug] :status … |reason |impact |ref` — measured this morning at 99%/91%/88% child-presence. **The council already writes tribunal-output embryos in UDON.** The delta between that de-facto schema and the sketch above is almost exactly the adjudicator's distinctive columns: load-bearing-degree, revisit-when, expires-on.

## 4. The literature connection — what it confirms, what it adds, what it lacks

Grounding: the same-day verified sweep, [`../../../udon-needs/01-ideation/02-provenanced/syntheses/external-rationale-structure-2026-07-29.md`](../../../udon-needs/01-ideation/02-provenanced/syntheses/external-rationale-structure-2026-07-29.md). Multi-agent-debate remarks below are training-knowledge, marked.

**What the literature confirms:** the tribunal's *node-typing* instinct is the IBIS/QOC lineage — IBIS's Issues/Positions/Arguments as typed deliberation nodes (Kunz & Rittel, 1970[^kunz-rittel]), gIBIS's hypertext realization (Conklin & Begeman, 1988[^gibis]), QOC's Questions/Options/Criteria (MacLean et al., 1991[^qoc]) — surveyed as a recognized field by 2000 (Regli et al.[^regli2000]). The decision layer as first-class architectural content was mainstream by 2016: architecture redefined around the set of design decisions (van Vliet & Tang[^vanvliet-tang], on the lineage of Jansen & Bosch 2005[^jansen-bosch] and Kruchten's decision ontology[^kruchten]), with ISO/IEC/IEEE 42010:2011 *requiring* rationale in architecture descriptions[^iso42010]. The field walked toward this for decades.

**What the literature adds — four inheritances worth taking whole:**

1. **SEURAT's Argument Ontology** (Burge & Brown, 2004–08[^burge-dcc04][^burge-seurat][^burge-jss08][^burge-diss]; verified 3-0×4): a *hierarchical vocabulary of typed argument kinds carrying default importance weights, inheritable by any rationale citing them, overridable per-claim*. This is exactly what "load-bearing by what degree" needs as vocabulary — sharper than free-form pro/con, with typed relations the `:targets`/`:kind` slots above should inherit rather than reinvent. Concrete semantics in §4.1.
2. **SEURAT's semantic inference**: re-evaluating decisions *when an assumption is disabled* — the revisit machinery half-built, mechanically. The tribunal's `:revisit-when` plus a census tool (O7) is that, grown up.
3. **The ADR lineage's lifecycle machinery** (verified 3-0×9[^madr-zeus][^log4brains][^structured-madr][^eadr]): status state machines with enumerated transitions, bidirectional supersession where both records survive, mutability keyed to state, decisions-vs-progress in separate stores. The tribunal record's *lifecycle* half is solved; adopt it. Concrete anatomy in §4.2.
4. **The capture problem as the adoption law** — "the spectre haunting all design rationale efforts" (Buckingham Shum et al., 2006, verified[^bshum2006]) — and its corpse: SEURAT's typed-rationale-with-inference *died* while prose ADRs spread. The warning transfers directly: **a schema that demands tribunal structure from a lone author repeats SEURAT.** The tribunal's answer is structural and (my *lean*) is the adoption breakthrough: the deliberation *generates* the typed record as a byproduct of deciding — the roles pay the capture cost by existing. Nobody annotates; the process exhausts structure.

### 4.1 SEURAT's semantics, concretely (verified at the free primaries; the JSS 2008 paper is paywalled[^burge-jss08])

RATSpeak — SEURAT's representation, extended from Lee's Decision Representation Language[^lee-drl] because DRL lacked explicit argument *direction* — structures deliberation as: **decision problems → alternatives → arguments**, where each argument is *for or against* an alternative and points at one of four grounds: a **requirement** (argues satisfaction/violation), an **assumption** (may become false — the revisit hook), a **claim** (an appeal to a quality the system should have), or **another alternative** (dependencies). The typed relation vocabulary between arguments and grounds: **satisfies / violates / supports / denies / presupposes / opposes**. Every claim then maps into the **Argument Ontology** — a hierarchy of common software-quality argument types (the "ilities"), where abstract levels subdivide into concrete ones, each entry carrying a **default importance that is inheritable by any rationale citing it and overridable per claim or argument** — the shared vocabulary that makes *semantic* inference possible at all.

The inference SEURAT actually runs, in two families (rationale stored in MySQL, SQL-assisted): **syntactic** — over structure alone: decisions with no selected alternative; selected alternatives with no supporting arguments; argument sets that are one-sided ("biased"); **semantic** — over content: when an *assumption is disabled* or an *importance/priority changes*, affected decisions are re-evaluated and violations surface (requirement violations; tradeoff violations — a declared tradeoff whose two sides aren't both argued). Two details did not survive verification and are excluded deliberately: the CLIPS-rules implementation claim (refuted 0-3) and "unanswered questions" as a syntactic check (unverified).

What the tribunal sketch should lift verbatim, my *lean*: the four ground-types (requirement/assumption/claim/alternative) as the `:kind` vocabulary for `|argument` and `|attack`; the six typed relations for `:targets` semantics; assumption-as-ground as the formal seat of `:revisit-when` (a revisit condition *is* a monitored assumption); and inheritable-default-importance as the mechanism behind load-bearing weights — so `:weight` is not a bare float but an override against a vocabulary entry's default.

### 4.2 ADR best practices, concretely (the practitioner consensus, verified where marked)

The **Y-Statement**[^ystatement] is the field's one-sentence argument template — slots, in order: *In the context of* ⟨use case/component⟩, *facing* ⟨concern⟩, *we decided for* ⟨option⟩ *and neglected* ⟨alternatives⟩, *to achieve* ⟨quality/desired consequence⟩, *accepting* ⟨downside/accepted tradeoff⟩, optionally *because* ⟨additional rationale⟩. Note what the slot structure forces that prose does not: the **neglected alternatives are mandatory** (no decision without a considered field), and the **accepted downside is mandatory** (no decision pretending to be free). Those two slots are the advocate and red-team voices compressed into a sentence.

**MADR**[^madr-zeus] (Markdown ADRs, the most-adopted structured format) adds per-option **pros/cons lists**, a **Decision Outcome** section that maps the Y-statement's "to achieve" clause into a mandatory *justification slot*, and a **Date/Status change table** — lifecycle as data. **adr-tools**[^adr-tools] writes **typed supersedes/amends links** into both records at supersession time. **log4brains**[^log4brains] enforces *immutability with status as the only mutable field*. **e-ADR**[^eadr] embeds the whole record as a typed Java annotation (`@MADR(title, contextAndProblem, alternatives, chosenAlternative, justification, relatedDecisions)`) — the one instance where justification occupies a named, tool-readable field, though its content is still free text within the slot.

And the estate's own autopax ADR system[^autopax-adr] is the most complete single instance found anywhere in the day's reading — practices worth naming as *the* best-practice set: a **status state machine with every legal transition enumerated** (DRAFT ↔ EXPLORING → PROPOSED → {ACCEPTED, REJECTED}; ACCEPTED → {SUSPENDED, SUPERSEDED}); **flags orthogonal to status** (`+EXECUTED`, `+AMENDED` compose with any state — `REJECTED+EXECUTED` means "decided against and fully removed," which a single enum could never say); **mutability keyed to state** (decided records frozen except typo fixes; substantive change requires the `+AMENDED` flag, authorized and logged); **bidirectional supersession with both records surviving**; **two edge kinds deliberately distinguished** (`blocked_by`/`needed_for` as decision-lifecycle dependencies vs `related` as conceptual); **decisions and implementation-progress in separate stores by explicit design**; and **estimation in session counts, not calendar time**. Every one of those maps onto a tribunal-record need; none needs inventing.

**What the tribunal has that the verified literature lacks — the two genuinely novel columns:**

- **Prospective falsification as structure.** Nothing verified carries "this decision expires when X" or "revisit on Y" as data. ADR supersession is retrospective — someone later notices. Pre-registered revisit conditions appear nowhere in the walked record. *(Scope honesty: four neighborhoods went unwalked — policy-as-code, LegalRuleML[^legalruleml], deontic KR, constraint provenance — and LegalRuleML's temporal-validity model is the named most-likely prior art for expiry specifically. Check before claiming novelty loudly.)*
- **The neutral observer and the risk analyzer as distinct voices.** The verified lineages have advocate-shaped and attack-shaped material; a *bias-auditor over both teams* and a *failure-shape analyst orthogonal to the pro/con axis* appear in none of it. These are also precisely the two voices the estate's methodology already runs by hand (the single-author caution; the counter-register).

**On "dully obvious nowadays"** *(training-knowledge, my push-back)*: what's obvious now is multi-agent debate[^debate] — and the debate/LLM-as-judge literature optimizes answer accuracy and *discards the deliberation*. Today's own deep-research harness is a degenerate tribunal (extractors, three refuters, vote-threshold adjudication — no observer, no risk node, no revisit criteria, deliberation gone when the report lands). The tribunal's distinctive content — the durable, load-path-bearing, self-expiring record — is not what got obvious. It's still the empty quadrant.

## 5. The core-schema suspicion (Joseph's, pre-validation)

His words: *"I suspect this may be one of the original 'udon core schemas' — a built-in schema that is specifically for anything deliberative where the choices made by agents accumulate and get magnified and need to know what they are."*

Carried at that register — a suspicion, deciding nothing — with three observations beside it (*proposed*):

- It would join the O12 family as a natural sibling: the epistemic-register schema marks a *claim's* standing; the tribunal schema records a *decision's* provenance. They compose — an adjudication emits claims that carry registers; the register schema's `decided` entries would cite tribunal records as their warrant. Together they are the decision layer (fiat strata) given its two artifact kinds: the standing and the act.
- The "choices accumulate and get magnified" clause is the demand condition, and it selects the right corpora: exactly where agent decisions compound across sessions — DECISIONS ledgers, spec rulings, schema adjudications, migration calls — which is where the estate already pays the O3 archaeology cost when reasons weren't captured.
- The O9/capture gate applies with full force, corpse in evidence: this becomes a core schema only if tribunal records are *cheaper to produce than prose deliberation* — which the process-generates-the-record property makes plausible and nothing yet demonstrates. The cheap first test: vivarium's council keeps deciding as it already does, and one adjudication per week lands in the sketch's shape; if the council finds the shape heavier than its current `|decision` embryo, that's the canary singing.
- **The first customer is now explicitly waiting** (same evening, vivarium): their DECISIONS ledger — ~130 entries / ~1,400 lines, partial decision-representation tooling already shipped (FORMAT citation form, `bin/check-decision-refs`) — surfaced its own growth question (full-history + generated projection vs status quo vs split), and Joseph's answer routed it here: *"status quo until it hurts. We're actually coming up with a more principled approach right now as we speak over in udon."* Their agent's named needs are this document's parts list: a **generated projection of currently-relevant decisions** (the canonical/derived split the estate already runs — LEXICON over terminology — applied to the ledger, which is also the read-cost answer to a 1,400-line session tax), **citation guidance** (the X1 form), and — from O16 — the **event-routing** that keeps the ledger's body present-truth while the change-marks land addressably. The deferral is a commitment signal with a deadline attached: "until it hurts" means the design owes them something before the hurt arrives.

## 6. Live specimen, same evening (testimonial; the exchange happened in vivarium as this document was being written)

Joseph's framing of the adoption fork, verbatim: *"it's either capture by design or likely-to-fail capture-manually. I can tell you though it is something that logogenic agents will **love** and adhere to quite dogmatically."* And the specimen, moments old: a vivarium agent surfaced a thirty-second authority question rather than deciding silently — a 2026-07-12 decision tagged `:by us` whose sibling's authority had been corrected as inflated, this one never re-checked, flagged **AUTHORITY UNVERIFIED** and brought to Joseph with the precise ask (keep `us`, or drop to `claude` with the measurements standing on their own?).

Three things the exchange teaches, each landing on an open edge above:

- **The demand side of capture-by-design is confirmed in the strongest available register: spontaneous practice.** The agent wasn't complying with a capture requirement; it was *eagerly maintaining* the provenance ledger, unprompted, at thirty-second granularity. The capture problem's corpse (§4) was a *manual-annotation* corpse; the live evidence says the record-keeping itself is something this class of agent reaches for. The residual adoption risk inverts accordingly — not "will they capture" but the failure the same exchange displays:
- **Dogmatism attaches to whatever the schema makes first-class — so the schema is a steering surface.** The agent adhered to the authority column until Joseph interrupted: *"rather than resting on authority — it's a question of what serves truth and the core — i.e., whether the decision was even correct."* The correction re-pointed the inquiry from the `:by` tag to the warrant — and the decision absorbed the authority question *locally*, exactly as §1 predicts, because its load didn't rest on that leg: the equal-area closure stands on nine-grid harness measurements that reproduce plus an independent council re-verification twelve days later. **Design consequence (proposed, and I'd rank it with the schema's top constraints): authority/provenance and warrant/load-bearing must be structurally distinct columns, with the record's own semantics subordinating the first to the second** — because agents will dogmatize the record's letter, so the letter must privilege the truth-serving column. This is O6 imitation operating at the governance layer: the schema's emphases will be conformed to, incidental ones included.
- **Records predating the governance mechanism need an honest re-grading path.** Joseph's read — *"probably council before we had that as an option, more or less"* — is a lifecycle case no ADR state machine in the verified literature covers: retroactive classification at the grade the process *would have* assigned, marked as retroactive. The authority vocabulary evolves; old records need something `was:`-shaped for their authority column, never silent upgrade.
- **An unadjudicated flag repeat-bills.** Joseph's addendum: this same questionable item "has clearly come up a few times by various agents already" — each successive agent re-noticing, re-investigating, and re-surfacing the same open authority question de novo. Eager adherence makes open flags a *recurring tax* rather than a one-time note: the discipline that surfaces defects also re-surfaces them until disposed. The record therefore needs asked-and-answered as a state — a flag that has been surfaced carries its prior surfacings and its disposition (answered / deferred-with-reason / awaiting-X), so the next agent inherits the adjudication trail instead of re-deriving the question. This is relata's rerun ≠ retry ≠ decide at the governance layer: re-encountering an open flag should jump to the pending decision with its evidence, never re-run the noticing. (FOR-JOSEPH's `:status open/answered` items are the estate's embryo of exactly this.)

## Bibliography (footnotes)

*Provenance discipline: entries marked **[sweep-verified]** carry the URLs/DOIs exactly as delivered by the adversarially-verified 2026-07-29 rationale sweep (3-vote per claim). Entries marked **[training recall]** are from model memory, believed correct, and MUST be verified against the primary before any external citation — the house has been burned by generated citations before.*

[^kunz-rittel]: Kunz, W. & Rittel, H.W.J., *"Issues as Elements of Information Systems,"* Working Paper No. 131, Institute of Urban and Regional Development, UC Berkeley, 1970. **[training recall]** — the IBIS origin paper; the Issues/Positions/Arguments node typing.

[^gibis]: Conklin, J. & Begeman, M.L., *"gIBIS: A Hypertext Tool for Exploratory Policy Discussion,"* ACM Transactions on Office Information Systems 6(4):303–331, 1988. **[training recall]** — IBIS realized as hypertext; the direct ancestor of Compendium.

[^qoc]: MacLean, A., Young, R.M., Bellotti, V.M.E. & Moran, T.P., *"Questions, Options, and Criteria: Elements of Design Space Analysis,"* Human-Computer Interaction 6(3–4):201–250, 1991. **[training recall]**.

[^regli2000]: Regli, W.C., Hu, X., Atwood, M. & Sun, W., *"A Survey of Design Rationale Systems: Approaches, Representation, Capture and Retrieval,"* Engineering with Computers 16:209–235 (2000), doi:10.1007/PL00013715. **[sweep-verified]** — https://www.researchgate.net/publication/220677804

[^bshum2006]: Buckingham Shum, S., Selvin, A., Sierhuis, M., Conklin, J., Haley, C. & Nuseibeh, B., *"Hypermedia Support for Argumentation-Based Rationale: 15 Years on from gIBIS and QOC,"* in *Rationale Management in Software Engineering*, Springer, 2006, pp. 111–132. **[sweep-verified]** — https://oro.open.ac.uk/3032/ — source of the Compendium architecture/limitations self-report and the capture-problem "spectre" passage.

[^lee-drl]: Lee, J., *"Extending the Potts and Bruns Model for Recording Design Rationale,"* ICSE 1991 (the Decision Representation Language). **[training recall]** — DRL is RATSpeak's named base per the sweep-verified SEURAT papers; this specific citation for DRL itself is from memory.

[^burge-dcc04]: Burge, J.E. & Brown, D.C., *"An Integrated Approach for Software Design Checking Using Design Rationale,"* Design Computing and Cognition '04. **[sweep-verified]** — https://web.cs.wpi.edu/~dcb/Papers/DCC-paper-04.pdf — the free primary for the Argument Ontology semantics (§4.1's quotes verified here).

[^burge-seurat]: Burge, J.E. & Brown, D.C., *"SEURAT: Integrated Rationale Management,"* (tool paper). **[sweep-verified as source; venue from training: ICSE 2008 demo track — verify]** — https://www.researchgate.net/publication/221556079

[^burge-jss08]: Burge, J.E. & Brown, D.C., *"Software Engineering Using RATionale,"* Journal of Systems and Software 81(3), 2008, doi:10.1016/j.jss.2007.05.004. **[sweep-verified; paywalled — claims verified via DCC'04 + dissertation]** — https://www.sciencedirect.com/science/article/abs/pii/S0164121207001203

[^burge-diss]: Burge, J.E., *Software Engineering Using design RATionale*, PhD dissertation, Worcester Polytechnic Institute, 2005. **[sweep-verified]** — digital.wpi.edu/downloads/6t053g09f

[^vanvliet-tang]: van Vliet, H. & Tang, A., *"Decision Making in Software Architecture,"* Journal of Systems and Software (2016), doi:10.1016/j.jss.2016.01.017. **[sweep-verified]** — https://www.researchgate.net/publication/291185587 — source of the "complementing or even replacing" redefinition claim.

[^jansen-bosch]: Jansen, A. & Bosch, J., *"Software Architecture as a Set of Architectural Design Decisions,"* WICSA 2005. **[training recall]** — named in the sweep-verified van Vliet & Tang lineage; exact citation from memory.

[^kruchten]: Kruchten, P., *"An Ontology of Architectural Design Decisions in Software-Intensive Systems,"* 2nd Groningen Workshop on Software Variability, 2004. **[training recall]** — same status as above.

[^iso42010]: ISO/IEC/IEEE 42010:2011, *Systems and software engineering — Architecture description*. **[sweep-verified as to the rationale requirement; edition note: superseded by 42010:2022 — training recall]**.

[^madr-zeus]: Kopp, O., Armbruster, A. & Zimmermann, O., *"Markdown Architectural Decision Records: Format and Tool Support,"* ZEUS Workshop 2018. **[sweep-verified]** — https://ceur-ws.org/Vol-2072/paper9.pdf

[^ystatement]: The Y-Statement template: Zimmermann, O. et al. — commonly cited to *"Sustainable Architectural Design Decisions"* (Zdun, Capilla, Tran, Zimmermann), IEEE Software 30(6), 2013, and Zimmermann's subsequent articulations. **[training recall for the attribution; the slot list itself is sweep-verified via the MADR/ZEUS paper's mapping]**.

[^adr-tools]: Pryce, N., *adr-tools* — https://github.com/npryce/adr-tools **[URL from training recall; the typed supersedes/amends-links behavior is sweep-verified]**.

[^log4brains]: Vaillant, T., *log4brains* — https://github.com/thomvaill/log4brains/ **[sweep-verified**, incl. source files AdrStatus.ts, AdrDto.ts, SupersedeAdrCommand.ts; live-project caveat: feature descriptions verified 2026-07-29, can drift**]**.

[^structured-madr]: *Structured MADR* — https://github.com/zircote/structured-madr (README + SPECIFICATION.md + schemas/). **[sweep-verified; small community project — evidence someone walked here, not field practice]**.

[^eadr]: *e-ADR* (embedded ADRs as Java annotations) — http://adr.github.io/e-adr/ (published to Maven Central). **[sweep-verified]**.

[^legalruleml]: OASIS LegalRuleML Core Specification v1.0 — https://docs.oasis-open.org/legalruleml/legalruleml-core-spec/v1.0/os/legalruleml-core-spec-v1.0-os.pdf; Athan et al., *"LegalRuleML: Design Principles and Foundations."* **[sweep-fetched as sources; produced no verified claims this pass — the unwalked-neighborhood caveat applies]**.

[^autopax-adr]: In-estate primary: `~/src/autopax/docs/ADR/README.md`, read whole and characterized in `doc-store-and-schemas-report.md` §11 (2026-07-23 pass); the practices quoted in §4.2 carry that report's path:line footnotes.

[^debate]: Multi-agent debate / LLM-as-judge: e.g. Irving, G., Christiano, P. & Amodei, D., *"AI Safety via Debate,"* arXiv:1805.00899 (2018); Du, Y. et al., *"Improving Factuality and Reasoning in Language Models through Multiagent Debate,"* arXiv:2305.14325 (2023). **[training recall, both — cited only for the contrast claim that this literature optimizes answer accuracy rather than persisting deliberation; verify before external use]**.

## Working Notes

*(X4 sidecar.)* The nearest next artifacts if this thread pulls: a close read of SEURAT's ontology before any vocabulary is invented (the sweep's routing note says the same); probe-8-style extraction over vivarium's council records specifically, asking what the de-facto tribunal schema already is; and the LegalRuleML check on prospective expiry. The §4 claim that process-generated capture escapes the capture problem now has two legs — the happy council, and §6's spontaneous-practice specimen — but both are this estate, this month. And the steering question ("does dogmatism follow the schema's emphasis even when the emphasis is wrong?") got its first instance *inside §6's own anecdote*, per Joseph's distillation of it: *"they cared so much about the provenance correctness that they forgot for a moment to even care whether it was a good decision or not."* The de-facto ledger makes authority prominent and warrant implicit; a capable agent's attention followed the prominence, and it took a steward interruption to re-point it at correctness. One anecdote, n=1, marked as such — but it is evidence for the mechanism in both directions at once: the schema steers (so the design consequence in §6 bullet 2 works), and a mis-emphasized schema steers *wrong* (so getting the emphasis right is not polish, it is the safety property). The remaining untested half is whether the fix — warrant structurally primary — actually redirects the dogmatism, which the council will demonstrate or embarrass the first week it runs the sketch's shape.
