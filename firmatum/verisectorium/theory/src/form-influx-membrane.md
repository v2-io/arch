---
slug: form-influx-membrane
form: formulation
type-expected: formulation
status: discussion-grade
max: decided
state: [drafted]
depends: [def-atom, def-integration-replacement]
---

# Formulation: The Influx Membrane

Material enters a store only through a membrane — manifest plus payloads, adjudicated crossing, typed outcomes — and nothing writes the population directly.

## Formal Expression

*[Formulation (influx-membrane)]*

Every store that accepts external material carries an **influx**: a designated surface where incoming material waits, visible and unprocessed, until adjudication moves it across. Three commitments define the membrane:

1. **Nothing writes the population directly.** External material — gathers, spike results, audit findings, steward brainstorms, upgrade offers — lands on the influx surface first. The path is drop → adjudicate → land, and the adjudication is the membrane's whole function: it is where write semantics ( [[def-integration-replacement]]), provenance, and epistemic register get applied *before* the population changes.
2. **Manifest plus payloads.** The influx is legible at a glance: a manifest (the entry point any agent or person can find and append to) over payload items, with sidecar material as needed. An item and its payload are one unit — a payload with no manifest presence is invisible work; a manifest row with no payload is an unbacked claim.
3. **Typed outcomes — the speech-act split.** An adjudicated crossing resolves to one of at least three distinguishable outcomes, and conflating them is a small dishonesty: **rejected** (the submitter erred — malformed, colliding, out of scope; the repair is theirs), **needs-review** (the *system* is honestly uncertain — the repair is adjudication, and telling a submitter "you erred" here mislabels system-uncertainty as user-error), and **skipped** (wrong kind of drop; no fault, no landing). Landings themselves record what landed where. Membrane outcomes are append-only events regardless of the destination store's write semantics.

*[Formulation (designated-influxes)]*

A store may carry multiple influxes, and an instance's standard anatomy designates some: the **canon** store's primary influx is its *base material* (the pre-instance substrate it was founded from — gathers, prior notes), alongside ordinary flows (spike→canon, audit→canon); the **SOP** store's primary influx is the *meta-feedback channel* — the concrete realization of [[post-self-governance]]'s participant-feedback source/sink — with the *upgrade channel* (kit capabilities arriving for local adjudication) as a designated second.

## Epistemic Status

Formulation — a design choice among alternatives (direct writes with review-after; locked single-writer stores; no staging at all), chosen and defended; max attainable: `decided`. The evidence is one estate's, in two registers: a shipped, lived membrane (relata's drop → validate → promote pipeline with exactly this outcome split, multi-agent-safe in production) and a provisional convention draft (the estate NORMS document's INFLUX schema — manifest forms, payload lists, inbox/backlog/archive sidecars — explicitly pre-ratification). Same-author lineage throughout: this is coherence with design intent, not independent corroboration. The three-way outcome split is the formulation's most defensible part — it was learned in the shipped system against a live failure (uncertainty mislabeled as error) rather than designed a priori.

## Discussion

**Why a membrane and not review-after-write.** Under total turnover, the population is the one surface every future reader trusts; a direct write puts unadjudicated material *inside* the trust boundary, where it is indistinguishable from adjudicated truth until someone happens to check. The membrane keeps the trust boundary physical: everything inside the population has crossed; everything that hasn't crossed is visibly outside. This is also what makes multi-agent concurrency tractable at the entry point — writers contend on the influx surface, not on the population ( [[form-write-isolation]] governs the population side).

**The membrane is the timescale control.** Fast processes (gathers, extractors, agent output) accumulate on the influx surface at their own tempo; the crossing happens at adjudication tempo; the population changes at landing tempo. The membrane is thus the concrete mechanism by which [[form-timescale-strata]]'s separated clocks are enforced for written material — permeability is a governed property, not an emergent accident of who wrote fastest.

**Visibility is half the value.** An influx that is a designated surface with a manifest can be *queried*: what is waiting, how long, with what expected disposition — feeding [[form-pending-surface]]. The anti-pattern it replaces is ambient accumulation: material scattered into the population's margins, TODO lists, and chat history, where nothing can be counted and everything must be re-found. Today's founding corpus is the live example: steward brainstorms, a mined tooling gather, and cross-instance feedback all entered through one influx tree, and its index states at any moment what remains.

**What the membrane does not do.** It does not adjudicate — agents and stewards do, under Organ IV's disciplines; it does not guarantee quality — it guarantees *provenance and a crossing record*; and it does not replace the delete-test — leaving the membrane is governed by [[def-integration-replacement]], and the economics of leaving it *completely* are [[claim-dispatch-compounds]]'s subject.

## Working Notes

- Frontmatter schema provisional pending the epistemology decision.
- Open (design): the manifest's concrete schema — the NORMS draft sketches several payload-list forms and sidecar layouts (inbox/backlog/archive/trash) and is explicitly provisional; adoption or revision belongs to the instantiation kit ( [[form-instantiation-kit]]) with the config expressed per instance.
- Open: whether *efflux* seams ( [[form-efflux-seams]]) are the same formulation mirrored (an emission membrane with typed outcomes) or a genuinely different mechanism — the phanero pipeline's stage boundaries suggest mirror-with-destination-gates, but this is undrafted.
- Open: outcome vocabulary beyond the three-way split — relata adds quarantine and deny-list machinery; whether those are deployment extensions or core outcomes is unsettled.
- Evidence-action (per [[form-max-attainable]] discipline, though ceiling here is `decided`): the formulation's *defense* strengthens if a second, non-estate instance adopts membrane entry and reports the same uncertainty-vs-error split earning its keep.
