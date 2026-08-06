---
slug: post-living-collection
form: postulate
type-expected: postulate
status: axiomatic
max: axiomatic
state: [drafted]
depends: []
---

# Postulate: The Living Collection

The governed object is a living collection: open-ended, never "done," improved across unbounded sessions; deliverables are emissions from it, not its terminus.

## Formal Expression

*[Postulate (living-collection)]*

A verisectorium governs a collection $C$ whose revision horizon is unbounded: there is no state of $C$ designated as final, and no event after which improvement of $C$ ceases to be expected. Everything that leaves $C$ in finished form — a monograph build, a submitted paper, a rendered lexicon, a report — is an **emission**: a projection of $C$'s state at a moment, produced at a seam, whose completion terminates the emission and not the collection.

Two consequences are taken as part of the postulate's content rather than derived from it, because they are restatements of what "living" means:

1. **Completion semantics attach to sessions and emissions, never to $C$.** "Done" is well-defined for a work cycle and for an emission; applied to $C$ it is a category error.
2. **A gate is meaningful only where a genuine destination lies beyond it.** Emissions have destinations (a venue, a release point, a differently-governed tier), so their seams may gate. $C$ itself has no destination, so machinery that models progress through $C$ as approach-to-completion misdescribes the object it governs.

## Epistemic Status

Postulate — near-definitional of the theory's scope, and deliberately so: a collection that genuinely ships and stops (a one-off deliverable with no revision tail) sits outside verisectorium scope, and this postulate is the boundary marker. It is not vacuous, because the classification of real objects is falsifiable-in-practice: the estate's submitted papers were plausibly classified as shipping objects and turned out to be living (reviews, revisions, and backports continue modifying their segment corpora after submission — the phanero pipeline exists because the emission seam kept being crossed in both directions). The honest reading of that evidence is that living collections are more common than they appear, not that everything is one.

## Discussion

**Why the postulate leads the theory.** Most of the machinery this theory inherits — promotion ladders, completion gates, "candidate" states, done-ness framings — was designed, explicitly or by training prior, for objects that ship. The estate's field data shows what happens when that machinery governs a living collection instead: the promotion ladder had never fired anywhere (115/115 segments at first rung in the most actively worked corpus; the terminal gate never once exercised estate-wide). Under a shipping model that is failure; under this postulate it is the practice quietly refusing a model that misdescribes its object. The reinterpretation of that data is the single largest consequence of getting the postulate right, and it is why [[form-state-flags-not-gates]] follows from here rather than standing alone.

**Emissions are real and their seams may be strict.** The postulate does not soften edges. An emission can be gated as hard as its destination demands — venue submission, anonymization lint, release freezes. What the postulate forbids is importing the emission's completion semantics back into $C$: the paper being submitted does not make its source segments "done," and their continued revision is not scope creep but the normal case ( [[form-efflux-seams]]).

**Relation to session ergonomics.** Sessions have honest start/work/finish cycles; the collection does not. Holding both truths at once, without letting the session's completion drive leak into the collection's self-description, is the design problem [[claim-session-cycle-fit]] hypothesizes the atomized structure solves.

## Working Notes

- Frontmatter schema here is provisional pending the epistemology decision (see OUTLINE working notes). `form:` is the identity-stable kind per [[form-slug-form-kinds]]; `type-expected:` may be dropped once form/trajectory separation is fully carried by other fields.
- Open: whether "unbounded revision horizon" wants a formal statement in AAT vocabulary (the collection as an environment component with $P(\text{change}) \gt \varepsilon$ forever — TST's [[scope-evolving-software]] shape). Cheap to add if a drafted [[claim-comprehension-economics]] wants the bridge.
- Boundary case to treat when [[def-verisectorium]] is exercised: archives. A frozen archive is not living, but a *governed* archive (append-only, with named-absence discipline) is a degenerate living collection whose only permitted improvement is annotation. Whether that is in scope is undecided.
