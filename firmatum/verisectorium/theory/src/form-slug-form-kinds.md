---
slug: form-slug-form-kinds
form: formulation
type-expected: formulation
status: discussion-grade
max: decided
state: [drafted]
depends: [def-atom, claim-identity-ordering-split]
---

# Formulation: Slug Prefixes Carry Form-Kind Only

Nothing mutable participates in identity: slug prefixes carry the atom's form-kind, never its trajectory position — and same-noun paired forms are one concept's anatomy, not collisions.

## Formal Expression

*[Formulation (slug-form-kinds)]*

1. **The partition.** An atom's kind-vocabulary splits into **form-kinds** — what shape of speech-act the record is, stable for the record's life — and **trajectory-kinds** — where a claim currently sits on its evidential path, expected to move as evidence arrives. Movement along the trajectory is the system working ( [[form-state-flags-not-gates]] is the same recognition on the process axis).

2. **The identity rule.** Slug prefixes draw only from the form-kind vocabulary; trajectory position lives in mutable metadata (expected type, status, support-kind) and never in the name. The working form-kind vocabulary:

| Prefix | Speech-act |
|---|---|
| `def-` | coins — introduces a quantity, object, or term |
| `post-` | grounds — foundational, accepted rather than derived |
| `scope-` | bounds — restricts or broadens the domain |
| `form-` | decides — a representational or design choice among live alternatives |
| `norm-` | prescribes — grounded guidance with preconditions |
| `claim-` | asserts — a truth-apt statement, at whatever evidential position |
| `obs-` | reports — records what was observed, as distinct from generalizing over it |
| `meas-` | operationalizes — how a quantity is measured |
| `disc-` | discusses — expository or framing work carrying no independent warrant |

3. **`claim-` is the trajectory-neutral prefix.** `hypothesis`, `derived`, `empirical`, `result` are positions on *one* form's evidential path — a claim is a claim whether currently hypothesis-grade or proven, so the strengthening path `hypothesis → observation-backed → derived` is metadata movement under a constant name.

4. **Paired forms are encouraged.** Two records sharing a subject-noun under different form prefixes are one concept's anatomy, not a collision: `def-X` coins what `claim-X` asserts something about; `form-X` decides what `obs-X` reports on. Precedent: asf's `result-X` + `deriv-X` pairs (statement and backing derivation), unnamed as a pattern until now.

## Epistemic Status

Formulation; max attainable `decided`. It is a chosen trade — form-legibility in a flat population *and* identity purity — among genuinely live alternatives, each tried in this estate: bare subject-noun slugs (steward-tried; "quickly becomes very confusing in a flat folder even with the outline to guide"), full type-derived prefixes (asf's live discipline, mechanically enforced by its alignment tooling), and alias machinery ( [[form-alias-survival]] — which this formulation reduces the need for rather than replaces). The motivating specimen is same-session and sharp: six trajectory-prefixed slugs in this theory's own outline were renamed within hours of coining because their expected types were honesty-downgraded — an epistemic update forcing identity churn, the exact rot [[claim-identity-move-proofness]] measures, created by the naming convention itself. Single-estate evidence throughout; the partition's cleanliness beyond this vocabulary is untested.

## Discussion

**The failure mechanism, precisely.** asf's `type:` vocabulary serves double duty: it names form-kinds (`definition`, `formulation`, `scope`) *and* trajectory-kinds (`hypothesis`, `derived`, `empirical`, `result`), and its slug discipline derives prefixes mechanically from `type:`. The consequence is structural, not accidental: whenever evidence moves a claim along its path, the type changes, the prefix re-derives, the slug changes — and every reference to the old slug dangles *because the corpus learned something*. A naming convention under which honest updating breaks identity punishes exactly the behavior the system exists to produce. The independently-arrived udon-needs epistemology cut (support-kind × strength × register as axes separate from form) is the same partition made in metadata; this formulation extends it to names.

**Why not bare slugs.** Identity purity alone argues for no prefixes at all. The lived counter-evidence is navigational: in a flat population directory, form prefixes are the only at-a-glance structure `ls` gives, and the steward's bare-slug experiment found the confusion cost real even with the outline to guide. The trade accepts a *small* stable vocabulary in the name to keep the large mutable vocabulary out of it. Corpus verbs ( [[form-corpus-verbs]]) may eventually dissolve the navigational argument — a `ls-segments` that surfaces kind from frontmatter makes prefixes redundant to tooling — at which point this formulation should be revisited rather than defended.

**Relation to asf (proposed-back, not adopted).** This refines asf's role-prefix discipline at its one defect while keeping everything it got right: subject-noun-first naming, mechanical alignability, prefix-vocabulary compactness. Adoption there would mean partitioning its type vocabulary and migrating trajectory-prefixed slugs through its alias-less estate — a real cost its stewards weigh, not a correction this corpus can make for it.

## Working Notes

- Frontmatter schema provisional pending the epistemology decision.
- **Open revisit, on the steward's explicit reservation (2026-08-06): `claim-` worries Joseph a little** — fine for now, revisit when intuition is better. Recorded concerns to weigh then: "claim" is semantically broad enough to blur against the outline's own Claim column and the loose sense in which every segment claims something; candidate alternatives should be gathered rather than presumed (assert-, thesis-, prop-, or a coined term through the Organ II naming cycle).
- Open: the norm/claim boundary is exercised judgment, not yet a rule — a record that prescribes-by-asserting ("strengthen-first beats soften-first, therefore do it") was routed `claim-` on the truth-apt test, but the boundary deserves treatment when [[claim-strengthen-first]] drafts; if a crisp test emerges, it lands here.
- Open: the form-kind vocabulary is working, not closed — candidates not yet needed (`example-`, `detail-`, `sketch-`) should be admitted against the speech-act test (does it name a distinct act, or a trajectory position in disguise?) rather than imported wholesale from asf's list.
- Open: interaction with the cross-member namespace scheme (`#asf/aat/…`) — foreign identities keep their home convention; this formulation governs only locally-minted slugs.
- Regression guard: do not reintroduce trajectory prefixes for grep-convenience ("find all hypotheses") — that is a query over metadata, owed to tooling and views, not to names.
