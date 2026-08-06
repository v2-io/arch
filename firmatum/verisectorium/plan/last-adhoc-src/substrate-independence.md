---
slug: substrate-independence
type: form
---

# Substrate independence

*The verisectorium principles stand regardless of the technology that implements them; what varies by substrate is cost, not validity.*

The pattern's invariants do not name any file format. Stable identity (a slug the corpus addresses records by), outline-as-view (order and membership held apart from identity), present-truth bodies (statements that can collide with their successors), per-type write semantics, and sidecars on their own clocks — each is a claim about *structure and process*, and each is already expressed, live, across substrates that share no syntax:

- **Markdown + YAML frontmatter** — asf's four components, vivarium/core, udon-needs (whose frontmatter grew three epistemic axes and an embedded event log without leaving markdown).
- **UDON** — comproprium's precept/practice/exemplum corpus, the udon-theory process segments, vivarium's DECISIONS and LEXICON ledgers.
- **YAML entries + event directories** — the terminology and refs stores, whose atoms are records with no prose body at all.
- **An external data tree** — relata, where the store is not even inside the code repository, and the invariants (identity resolution, append-only events, generated views) hold most strictly of any instance.

A directory move this same week made the point empirically from the negative side: every slug-shaped reference in the moved corpus survived; every path-shaped provenance pointer broke (see [[provenance-rot-specimen]]). The invariant that held was the principle; the thing that rotted was a substrate detail standing in for one.

**The limit: independence is of principles, not of costs.** Substrates price the same principle very differently. UDON's sigils put record-structure in the token stream, so "find every status attribute" is a plain grep; markdown buys the equivalent only with convention plus a linter, and multi-record markdown files cannot express record boundaries the way multiple top-level UDON elements natively do. Frontmatter carries machine-readable per-atom schema cheaply; a pure-prose substrate pushes the same information into section conventions that drift. So substrate choice is a real engineering decision about enforcement cost, tooling reach, and failure modes — it is just not a decision the principles wait on. A deployment can adopt the pattern in whatever it writes today and migrate substrates later without the corpus losing its identity, because identity was never stored in the substrate.

This cuts both ways and should keep cutting both ways: the principle licenses neither "wait for udon" (the pattern already works in plain markdown, at higher policing cost) nor "markdown is enough" (several of the pattern's chronic pains — silent multi-record collisions, unqueryable containment, convention-only structure — are exactly the costs a structure-bearing substrate lowers).

## Strength & grounds

Held at **heuristic** strength. Grounds: the cross-substrate expression above is observed in this estate at gather time (2026-08-05 inventory and live-state reports, `plan/INFLUX/`), which is real breadth of *pressure* (a derivation corpus, a spec corpus, a bibliography, a precept store) but single-estate and shared-authorship — coherence, not independent corroboration. The negative specimen ([[provenance-rot-specimen]]) is measured. What would raise it: an outside-authorship corpus adopting the principles on a different substrate and reporting the same invariants holding; or a substrate migration of one of our own instances completing with identity and history demonstrably intact — the strongest available in-estate test.

## Working Notes

- The cost asymmetry deserves its own treatment once ch. 12 (publishing/rendering mechanics) and the ch. 1 inner-section-schema gap develop — this segment deliberately states only that costs differ, not a ranking.
- Watch for the first genuine counterexample: a principle that turns out to *require* a substrate affordance (candidate: per-part write rules on multi-record files may be inexpressible without declared record boundaries — if so, that principle is substrate-conditional and this segment narrows).
- Migration-with-identity-intact is both this segment's best test and ch. 4's whole-outline-migration gap; one exercise could serve both.
