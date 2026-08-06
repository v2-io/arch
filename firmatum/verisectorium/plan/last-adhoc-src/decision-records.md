---
slug: decision-records
type: form
depends:
  - rationale-capture-survey
  - authority-flag-specimen
---

# Decisions are their own kind of record

*A decision is not a claim, and storing it as one loses the three things that let a later finding be absorbed instead of resisted: what the decision stands on, how sure it is (separately from what it is), and what would make it worth reopening.*

## The claim

Corpora built around claim atoms tend to treat decisions as just another claim: a present-truth statement in a body, superseded when it changes. That works until new information arrives — and then the difference shows.

A decision needs four things a claim does not:

**Its confidence recorded separately from its content.** A decision and a confidence answer different questions and are repaired differently: a wrong decision is repaired by re-adjudicating it; a wrong confidence is repaired by calibration. Fused into one field, neither repair is available — a hedged decision reads as a weak decision, and a confident record of a bad call looks the same as a good one. The test that decides it: **two labels earn separation when they route to different repairs.** That test looks portable well beyond decisions, but no segment here states it yet, so treat it as a local argument rather than an established principle being applied.

**Its load path named.** Which arguments is this decision actually standing on, and by what degree? A decision that knows its legs can absorb a refutation **locally**: the leg is attacked, the leg either holds or is replaced, and the rest of the decision is untouched. A decision whose reasoning was blended into prose has no legs to attack, so it defends itself totally or collapses totally. That is the mechanism behind the payoff Joseph names as the design's actual target — that new findings stop arriving as threats to the current mental model:

> …it gives sound **"intentional vs incidental" *with* reasoning** that easily incorporates new understanding and new data, instead of those new findings somehow being a threat to the current mental model or set of implicit/latent decisions mixed altogether.

**Its revisit conditions, registered in advance.** *Expires-on* (a date) and *revisit-when* (a condition) are the prospective half. They are what makes integrating a new finding tractable at all: without them, absorbing a datum means re-evaluating everything, which is diffuse, unbounded, and therefore silently never happens. With them, integration becomes a **query followed by a bounded work set** — *which revisit conditions does this datum satisfy?* fires a computable subset of decisions and touches nothing else. Integration debt stops being a vibe and becomes a countable queue: fired / pending / overdue. Date triggers are trivially census-able; condition triggers are the richer and harder class.

**Its hypothesis, where it has one.** Decisions are often coupled to a prediction — *we are deciding this because we expect X to happen*. Where that is true, saying so is cheaper and more honest than authoring falsification criteria cold, because the stated hypothesis **generates** its revisit conditions. It also has a compounding effect: a corpus of hypothesis-coupled decisions is a corpus of scored predictions, which is what turns a deciding body into a *learning* one.

Two of these are ordinary inheritance and two are not. Typed argument grounds, importance weights, status machines, and bidirectional supersession are all solved in the design-rationale and ADR literatures and should be adopted rather than reinvented ([[rationale-capture-survey]]). **Prospective falsification appears nowhere in what was walked** — and that "walked" is doing real work: four neighborhoods went unexamined, and one of them (LegalRuleML's temporal-validity model) is the named likely prior art. Treat the novelty as unestablished; treat the need as demonstrated.

## The adoption constraint, which is the hard part

The same literature carries a corpse. SEURAT — typed rationale with working semantic inference, the most sophisticated system in the field — died, while prose ADRs spread everywhere. The capture problem ("the spectre haunting all design rationale efforts") is the reason, and it transfers without modification: **a schema that asks a lone author to annotate structure after the fact will repeat SEURAT.**

The proposed escape is structural rather than motivational: if the deciding *process* has typed voices, the typed record falls out as exhaust and nobody annotates anything. Two pieces of evidence sit behind that, both from this estate and both weak in the same way: vivarium's council already writes decision-record embryos in its ledger unprompted, and an agent there was observed maintaining provenance eagerly at thirty-second granularity ([[authority-flag-specimen]]). Same estate, same month, one class of agent. **The escape is a lean, not a finding** — and it has a cheap canary: run one adjudication per week in the fuller shape and watch whether the council finds it heavier than what it already does.

## Strength & grounds

**Formulation, argued from a lived design plus inherited literature; the novel column is unverified and the adoption escape is untested.** The four-column structure is Joseph's, from a design in use; the separation-of-repairs argument and the bounded-work-set argument are analytic and stand on their own. The literature inheritance is real but held at one remove and with its verification registers intact ([[rationale-capture-survey]] — sweep-verified entries are reported findings, training-recall entries are not citable as established). The absence-of-prior-art claim is scoped to what was walked. No independent-authorship corroboration exists for the mechanism working in practice.

## Working Notes

- What the estate already has, unexploited: an ADR status machine with orthogonal flags and state-keyed mutability (autopax), and a growing council ledger whose gap from the fuller shape is almost exactly the three adjudicator columns — load-bearing degree, revisit-when, expires-on.
- Open and unhomed: **records that predate their own governance need an honest re-grading path** — retroactive classification at the grade the process would have assigned, marked as retroactive. No verified ADR state machine covers it, and no segment here carries it.
- This segment, [[warrant-over-authority]] and [[asked-and-answered]] all ground partly in the same thirty-second exchange ([[authority-flag-specimen]]) and answer different questions from it: what a decision record needs, which column the schema should privilege, and how an open flag carries its disposition. The shared citation is one specimen doing three jobs, not three segments restating one.
- Related: [[warrant-over-authority]] (which column the schema must privilege); [[asked-and-answered]] (the open-flag half); [[collision-staleness-detection]] (a decision's revisit trigger is a staleness detector with a pre-registered condition); [[tribunal-record]] (the process that emits these records).
- Do not cite influx copies under `plan/INFLUX/tribunal/` as warrant for anything above; the live source is `~/src/arch/firmatum/udon/v2/theory/to-integrate/refine-more/epistemic-tribunal-revisited.md`, and this segment replaces those pointers.
