---
slug: form-state-flags-not-gates
form: formulation
type-expected: formulation
status: discussion-grade
max: decided
state: [drafted]
depends: [post-living-collection]
---

# Formulation: State Flags, Not Gates

Process state is carried by independent, resettable check-flags per atom; promotion ladders are replaced, and gates survive only where a genuine destination lies beyond them.

## Formal Expression

*[Formulation (state-flags-not-gates)]*

Each atom carries a set of **state flags**, each an independent record that a named kind of work has been done — illustratively: `explored`, `drafted`, `checked`, `citations-check`, `prose-check`, `deps-check`. Three commitments define the formulation:

1. **Independence.** Flags are not rungs. No flag presupposes another; an atom can be `citations-check` clean and never prose-checked; there is no composite "level" an atom is at.
2. **Reset on edit.** Every flag names its reset condition, and content edits reset the flags whose verification the edit invalidates (an edit to the body resets `citations-check` and `prose-check`; a dependency's change resets `deps-check`). Resetting is bookkeeping, not demotion: on edit of a well-verified atom, reset the appropriate flags and/or launch a separate agent to re-verify — the atom's *strength* (its status, Organ III's separate axis) is untouched by the reset.
3. **Gates only at destinations.** A blocking check is legitimate exactly where something passes *through* to a differently-governed place — an emission seam ( [[form-efflux-seams]]: venue submission, release, anonymization), or promotion into a tier that is genuinely treated differently. Internal to the living collection there are no destinations ( [[post-living-collection]]), hence no internal gates — only flags, each re-runnable, none ratcheting.

The design criterion, stated as the formulation's test: **updating a well-verified atom must feel like advancement, not regression.** Any process vocabulary under which improving an atom reads as losing ground fails the formulation.

## Epistemic Status

Formulation — a design choice among alternatives (promotion ladders; no process-tracking at all; hybrid ladder-plus-flags), chosen and defended, not derived. Max attainable: `decided`. The *motivating evidence* is however strong and worth separating from the choice: (a) the estate-wide field measurement that the inherited promotion ladder never operated — 115/115 segments at first rung in the most actively worked corpus, the terminal gate never fired anywhere — which under [[post-living-collection]] reads as a category mismatch rather than a discipline failure; (b) the independent arrival of the same diagnosis in the udon-theory corpus, whose FORMAT states "stage is a present-tense work-remaining marker, not a gate and not a trophy… a ladder that only promotes accumulates falsehood." Per the estate's own coherence rule, (b) is one mind being consistent with itself across corpora — design-intent evidence, not independent corroboration; the formulation's external validation is genuinely open.

## Discussion

**Why ladders fail on living collections.** A ladder models approach to a terminus. On a shipping object that is the right model; on a living collection it produces three observed pathologies. *It never fires:* with no destination pulling atoms upward, promotion is always deferrable and always deferred — the estate's uniform first-rung data. *It punishes honesty:* under a ladder, editing a promoted atom demotes it, so the vocabulary itself pressures agents to defend rungs instead of improving atoms, and a corpus honestly reset after a good reorganization reads as having regressed. *It accumulates falsehood:* a promoted state that survives edits it should not survive is a standing lie about what has been checked — the ratchet converts staleness into false assurance.

**What the flags preserve from the gate era.** The *checks* were never the problem. The content of the inherited gates — dependency audits, derivation tracing, label accuracy, mechanical lint, working-note disposition — caught real defects and survives intact as flag definitions; what is dropped is only the ladder ordering and the ratchet. A flag is a gate's check made re-runnable and de-ranked.

**Composition with the other axes.** Flags are Organ III's *process-state* axis and interact with, but never substitute for, the strength axis: `checked` says someone did the work at a point in time; `status` says how strongly the claim is held; the evidence ledger ( [[form-evidence-ledger]]) says why. A reset flag with an unchanged status is the normal post-edit condition — work owed, confidence intact pending re-check.

**Tooling shape implied.** Reset-on-edit wants mechanical support (an edit hook or standing sweep that clears invalidated flags and enqueues re-verification — feeding [[form-pending-surface]]), because manual reset relies on exactly the diligence the ratchet-era data shows is not reliable. Until tooling exists the convention is the mitigation, stated as such.

## Working Notes

- Frontmatter schema provisional pending the epistemology decision; this segment's own `state: [drafted]` is the formulation dogfooding itself.
- Open (design): the flag vocabulary above is illustrative, not fixed — the kit ( [[form-instantiation-kit]]) should let deployments declare their flag set, with a small common core so cross-instance tooling has something to hold.
- Open (reconciliation): asf's live stage/gate machinery is the largest deployed contrary practice. Reconciliation is a real design act — mapping stage values onto flags loses the ordering on purpose, and the promotion-terminus question asf's own meta-process review flagged (steward note, 2026-07-14: the gating methodology under reconsideration) is the natural adoption seam. Proposed to asf, not adopted there.
- Open (empirical): the "advancement, not regression" criterion is testable — agent behavior around editing verified atoms under flag vocabulary vs ladder vocabulary. No measurement exists.
- Regression guard: do not reintroduce a composite per-atom "level" for dashboard convenience (a max or sum over flags) — that silently rebuilds the ladder; dashboards should show the flag vector.
