---
slug: def-atom
form: definition
type-expected: definition
status: axiomatic
max: axiomatic
state: [drafted]
depends: [post-living-collection]
---

# Definition: The Atom

The atom is the deployment-chosen unit of the population — one atom per record, identity carried by slug and never by position or number.

## Formal Expression

*[Definition (atom)]*

An **atom** is the smallest independently-adjudicable record in a verisectorium's population. Three commitments define it:

1. **Kind is a deployment choice.** What the atom *is* — claim, term, precept, practice, section, bibliographic entry, norm, demand, nomos, … — is chosen per instance and declared, not fixed by the theory. Observed kinds across the instance spectrum ( [[obs-instance-spectrum]]): claims (asf, vivarium), terms (terminology stores), works-with-epistemic-state (relata), precepts/practices/exempla (comproprium, several kinds under one meta-process), demands and findings (udon-needs), paper sections (the paper-lite instances).
2. **One atom per record.** A record carries exactly one atom. A record carrying three claims can be one-third stale without colliding with anything — the collision surface exists only at the grain of the record, so the adjudication grain and the record grain must coincide.
3. **Identity is the slug.** The atom's identity is a stable name, never a position in an ordering, never a presentation number, never a filesystem location ( [[claim-identity-ordering-split]], [[form-slug-form-kinds]]). Everything else about the atom — its ordering in views, its epistemic state, its process flags — may change without its identity changing.

"Record" is deliberately substrate-neutral: today a record is usually a file in a flat directory, but the definition binds to whatever the deployment's storage resolves a slug to (relata's externalized data tree is the existing proof that the population can live behind an interface).

## Epistemic Status

Definitional. The truth-apt content adjacent to it lives elsewhere: the *strength* claims about what atomization buys — collision-detectability, delegation grain, piecewise truth propagation — are [[claim-comprehension-economics]], [[claim-atomicity-parallelism]], and the collision mechanism transmitted from the cross-corpus analysis (at that source's own tier: lived report plus specimens, near-structural at the core). Commitment 2's justification is the collision argument; the definition records the commitment, the claims carry the warrant.

## Discussion

**Why the kind is free but the grain is not.** The instance spectrum shows atom kinds varying wildly with the pattern intact — which is the evidence that kind is a deployment parameter. The grain commitment does not vary: every mature instance converged on one-unit-per-record, and the instances that violate it (multi-claim section files in the paper-lite corpora) are exactly the ones whose staleness is hardest to detect. Comproprium's contribution is that *multiple* kinds can share one population under one set of meta-processing rules — kinds coexist; grains do not blur.

**The segment-set principle travels.** From asf: every record in the population's directory *is* an atom and conforms to the deployment's cadence — drafts included, orphans included. Process states describe progress within the conventions, never exemption from them, and tooling is entitled to rely on this (a non-conforming file in the population silently breaks every instrument that walks it). Working material that is not an atom belongs outside the population.

**What the atom is not.** It is not the unit of *exposition* (that is the view's row — [[claim-outline-as-view]]), not the unit of *storage* (a record may be realized as a file, a directory, or an interface-resolved entry), and not a single text blob (its internal anatomy is [[def-atom-cluster]]).

## Working Notes

- Frontmatter schema provisional pending the epistemology decision (see OUTLINE working notes).
- Open: the grain judgment itself — "one claim" is a boundary call (asf allows corollaries to live with their parent when they reinforce its independence; anything independently referenceable gets its own record). The judgment is real and deployment-exercised; whether the theory can say more than "independently-adjudicable and independently-referenceable" is open.
- Open: vocabulary precision for record vs atom vs file — this draft uses *atom* (the unit), *record* (its storage realization), *file* (one common realization). If the lexicon store lands these terms, this segment should cite rather than re-define.
