---
slug: form-decision-records
form: formulation
type-expected: formulation
status: proposed
max: decided
state: [pre-drafted]
depends: [def-atom-cluster]
---

# Formulation: Decision Records

Dispositions and adjudications leave append-only events with reasons, revisit-when, expires-on — one schema family from working-note item to governance decision.

## Working Notes

*(pre-draft: caution + source notes from the 2026-08-09 coord, whose primaries-read context would otherwise be lost; drafting still owed from live sources)*

- **Misfire-feedback caution (2026-08-06), carried from the outline:** events carry the named *criterion* that would make bending visible — a record of "decided X" without the criterion it was decided against cannot show a later reader whether the decision is being bent.
- Sources read whole this session that bear directly: de-novo SOP §7.6–7.9 (the per-finding five-core-elements shape *with the reason each earned its place* — "a finding without counterevidence search reads as a complaint"; the disposition vocabulary New/Known-unintegrated/Known-resolved/Tooling-gap/Scope-status-mismatch; integration-debt vs theory-gap as *diagnosis*, different remediation and urgency); 07-decision-routing findings (the eight-field brief schema; decided-is-not-terminal, so records need a re-open event whose body states *what changed since the decision*); spikes.sop §3–5 (five-state disposition + sub-disposition tagging after Refinement 4's overload finding — an enum that routes differently must not share one label).
- Also read: the template's DECISIONS.ud form (decided-by vocabulary: steward/ratified/council/supported/defacto/proposed/transition — the *authority* axis of an event) and relata's `verifications/<key>/<ts>-<verifier>-<criterion>.md` (the shipped per-(record, criterion) append-only realization; latest-wins-per-criterion as projection, trail never deleted).
- Candidate carve from those, held loosely: minimal event tuple = (subject, act, criterion, actor+authority, date, outcome, revisit-when?, supersedes/reopens?) — the same family serving working-note dispositions, spike routing, valve decisions, and verification events; realization per-store varies (files, frontmatter, directories).
- Unresolved and genuinely open: whether the brief ( [[form-steward-valve]]) is a decision-record *kind* or a view over one.
