---
slug: rationale-capture-survey
type: survey
depends: []
---

# Design-rationale capture: a field with fifty years of corpses

*What the design-rationale literature already solved, what it never attempted, and why its most sophisticated system died while its crudest one spread — inherited at a stated remove, with the verification register of every claim preserved.*

## Provenance register — read this before citing anything below

This survey does **not** rest on primary reading done by this project. Its content is inherited from an in-estate document (`~/src/arch/firmatum/udon/v2/theory/to-integrate/refine-more/epistemic-tribunal-revisited.md`, 2026-07-29, §4) which reports an adversarially-verified literature sweep run the same day, at three votes per claim, and which marks each of its bibliography entries as either **[sweep-verified]** (URL/DOI delivered by the sweep) or **[training recall]** (model memory, believed correct, unverified).

That distinction is preserved here and must be preserved by anything citing this segment:

- **Sweep-verified** items may be cited as *reported findings of a verified sweep*. They have not been re-checked here.
- **Training-recall** items may **not** be cited as established. They are named because the sweep's own findings reference them; treating them as citations is the generated-citation failure the estate has already paid for.
- Re-verifying any of this against the primaries is an open action, not a completed one.

Marking a claim's register is cheap; laundering an inherited verification into a first-hand one is the defect this section exists to prevent.

## The lineage, as the sweep reports it

Typed deliberation nodes are not new. IBIS (Issues / Positions / Arguments), its hypertext realization gIBIS, and QOC (Questions / Options / Criteria) established the shape between 1970 and 1991 — *all three attributions are training-recall*. The field had been surveyed as a recognized discipline by 2000 (Regli et al., *A Survey of Design Rationale Systems*, Engineering with Computers 16:209–235, doi:10.1007/PL00013715 — sweep-verified). By 2016 the decision layer was mainstream architectural content (van Vliet & Tang, *Decision Making in Software Architecture*, JSS 2016, doi:10.1016/j.jss.2016.01.017 — sweep-verified), and ISO/IEC/IEEE 42010:2011 *requires* rationale in an architecture description (sweep-verified as to the requirement; the 2022 superseding edition is training-recall).

## Four inheritances the sweep identifies as takeable whole

**1 · SEURAT's Argument Ontology** (Burge & Brown, 2004–08; the free primary is the DCC'04 paper at `web.cs.wpi.edu/~dcb/Papers/DCC-paper-04.pdf` — sweep-verified). Its representation, RATSpeak, structures deliberation as *decision problems → alternatives → arguments*, each argument for or against an alternative and grounded in one of four things: a **requirement**, an **assumption**, a **claim** (appeal to a quality the system should have), or **another alternative**. The relations are typed — satisfies / violates / supports / denies / presupposes / opposes. Claims map into a hierarchy of software-quality argument types, each entry carrying a **default importance inheritable by any rationale citing it and overridable per claim**. That is a ready-made answer to "load-bearing by what degree": a weight is an override against a vocabulary entry's default, not a bare float.

**2 · SEURAT's semantic inference.** It re-evaluates affected decisions when an **assumption is disabled** or an importance changes, and runs syntactic checks over structure alone (decisions with no selected alternative; selected alternatives with no supporting argument; one-sided argument sets). Assumption-as-ground is the formal seat of a revisit condition: a revisit condition *is* a monitored assumption. Two details the sweep specifically excluded: the CLIPS-rules implementation claim (refuted, 0–3) and "unanswered questions" as a syntactic check (unverified).

**3 · The ADR lineage's lifecycle machinery** (MADR — Kopp, Armbruster & Zimmermann, ZEUS 2018, `ceur-ws.org/Vol-2072/paper9.pdf`; log4brains; structured-MADR; e-ADR — all sweep-verified). Status state machines with enumerated transitions; bidirectional supersession where **both records survive**; mutability keyed to state; decisions kept in a separate store from implementation progress. The Y-Statement template's slot structure is worth naming on its own: *in the context of … facing … we decided for … and neglected … to achieve … accepting …* — the **neglected alternatives** and the **accepted downside** are mandatory slots, which is the advocate and the red-team compressed into one sentence.

**4 · The capture problem, which is the adoption law.** The sweep verifies the phrase "the spectre haunting all design rationale efforts" (Buckingham Shum et al., 2006, `oro.open.ac.uk/3032/`). The corpse is the evidence: SEURAT — typed rationale *with* working inference — died, while prose ADRs spread. The transferable warning is sharp and unwelcome: **a schema that asks a lone author to annotate structure repeats SEURAT.** The only escape the source proposes is structural — a deliberation with typed roles *emits* the record as exhaust, so nobody annotates — and it labels that as a lean, not a result.

## Where the estate's own practice already sits

Verified first-hand at drafting (2026-08-05), so this paragraph does not inherit its register:

- **The autopax ADR system** (`~/src/MOVED/autopax/docs/ADR/README.md`, read directly) is the most complete instance found: a status machine with every legal transition enumerated (DRAFT ↔ EXPLORING → PROPOSED → {ACCEPTED, REJECTED}; ACCEPTED → {SUSPENDED, SUPERSEDED}), **flags orthogonal to status** (`+EXECUTED`, `+AMENDED` compose with any state, so `REJECTED+EXECUTED` can say "decided against and fully removed" — something a single enum cannot express), mutability keyed to state (decided records frozen except typo fixes), status *groups* (INITIAL / UNDECIDED / TABLED / DECIDED / DEPRECATED) layered over the machine, and decisions kept separate from progress. Caveat: this tree now sits under `MOVED/`; the path is live today but the tree's location is not stable.
- **vivarium's council ledger** (`~/src/arch/vivarium/DECISIONS.decision-log.udon`, counted directly): **160 `|decision` blocks over 1,697 lines** as of 2026-08-05, against ~130 entries / ~1,400 lines reported by the 2026-07-29 source — roughly 30 decisions in eight days, which is the growth pressure that raised its own read-cost question. Its header declares a deliberate design: top-level self-contained blocks so a new entry can be appended in one atomic write **without reading the file**, with `:supersedes` available but not required.

## The claimed gap, with its scope caveat intact

The source names two things it found nowhere in the verified record: **prospective falsification as structure** ("this decision expires when X" / "revisit on Y" carried as data — ADR supersession is retrospective, someone later notices), and **the neutral observer and risk analyzer as distinct voices** (attack-shaped and advocate-shaped material is everywhere; a bias-auditor over *both* teams and a failure-shape analyst orthogonal to the pro/con axis appear in none of it).

The novelty claim carries its own honest scope limit, stated in the source and repeated here because it is the part most likely to get dropped: **four neighborhoods went unwalked** — policy-as-code, LegalRuleML, deontic knowledge representation, and constraint provenance — and **LegalRuleML's temporal-validity model is named as the most likely prior art for expiry specifically**. The gap is "not found in what was walked," not "does not exist."

## Working Notes

- Highest-value next action if this thread pulls: read SEURAT's DCC'04 primary directly before any argument-kind vocabulary is invented here, and run the LegalRuleML check on prospective expiry. Both would convert inherited register into first-hand register.
- The e-ADR instance (the whole record as a typed Java annotation) is the one case reported where justification occupies a named, tool-readable field — relevant if a machine-readable justification slot is ever wanted.
- Unresolved tension worth keeping visible: inheritance 4 says structure-demanding schemas die, while inheritances 1–3 are all structure. The proposed reconciliation (the process pays the capture cost by existing) is unproven; [[decision-records]] carries it at that strength.
