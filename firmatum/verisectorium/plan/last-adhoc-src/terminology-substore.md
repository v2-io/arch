---
slug: terminology-substore
type: form
depends:
  - terminology-store-anatomy
  - name-collision-across-stores
---

# Vocabulary is a second store, not a section of the first

*A term is referenced by many claims, changes on a clock of its own, and re-prices everything that uses it when it changes — which is the same argument that made claims into atoms, applied one layer down.*

## The claim

A claim corpus runs on a vocabulary, and the vocabulary is not part of any one claim. The moment a term is used by several atoms, three properties appear that force it into a store of its own:

- **Its own write clock.** A definition is refined for reasons that have nothing to do with any claim standing on it.
- **Fan-out on change.** Renaming or narrowing a term re-prices every claim that used it. Whether that re-pricing is even *findable* depends on the term having an identity to search for.
- **A different truth-status.** A naming choice is not truth-apt. It is decided, and it can be decided badly and later differently without anything having been false. Storing decisions in a structure built for claims flattens exactly the thing that matters about them ([[decision-records]]).

So the terminology substore is not a convenience for readers. It is the same atomization move that produced the claim corpus, run on a second population — and the estate's evidence is that the machinery transfers nearly intact: slug identity, one record per file, a generated reader-facing view, per-record concurrency safety ([[cousin-store-lineage]] on how far that spine has copied).

**Two things the vocabulary store needs that the claim store does not.**

*A decision event log.* Because naming is decided rather than derived, the interesting history is *who decided this, when, and why*, and a mutable status field loses it silently at the moment of overwrite. The working instance makes each decision an append-only file under the term's own slug, with the decider, action and timestamp in the filename — so concurrent decisions on one term cannot collide, and the write-safety property falls out of the naming scheme rather than being enforced ([[terminology-store-anatomy]]).

*A graveyard that actively refuses.* A retired term is not merely absent; readers and agents keep reaching for it, because it is what they learned. Supersession recorded only on the surviving record does not reach them — the estate has an independent instance of exactly this, where frontmatter supersession failed to stop readers and a banner in the body was needed. The repair a second corpus runs is a named-absence index whose entries say *do not use this anymore, use that*, kept outside the live dictionary so the dictionary itself states present truth only ([[absence-as-structure]]).

**A distinction the present-truth rule would otherwise destroy, and which one instance gets right:** *what this is not* and *what this is easily confused with* look like history and are not. They are present-tense disambiguation against a **live** neighbour, and a mechanical "history goes elsewhere" sweep deletes them. The general form is worth holding beyond lexicons: a rule that partitions by *tense* has to be checked against material that is retrospective in form and present-tense in function.

**The estate's own worked failure is the sharpest evidence for making the reader-facing view derived.** One corpus keeps two vocabulary surfaces. The prose lexicon is *generated* from the per-term records and its header says so; its notation reference is *hand-maintained*, and the same corpus's process law declares it **a lagging index that the live theory drifts away from, never to be cited as authority** — with a recorded incident where an agent leaned on it as a verified pillar and the real argument turned out stronger without it. Same corpus, same class of surface, one generated and one not: the generated one is trusted, and the hand-maintained one had to be legislated around. The records already carry a notation field intended to drive the reference eventually; the generator has not been written. **A vocabulary view that is not derived does not stay wrong quietly — it becomes a proxy that has to be marked untrustworthy, and then everything downstream pays the marking.** ([[derived-vs-authored]] carries the general principle; this is its cost in one corpus, at the surface where the cost is most easily missed because a notation table looks static.)

**Bounded contexts are the unbuilt half.** Collision detection runs *inside* a store and is structurally blind between them ([[name-collision-across-stores]] — four live referents of one name, caught only by a hand-written table). A vocabulary substore is the instrument that would catch it, and only if its identity scheme is scoped across the estate rather than per-repository. One corpus has defined such a scheme — a member namespace plus local identity, with a dedicated term namespace — and nothing consumes it. Every store in the estate is currently a private context that believes it is the only one.

## Strength & grounds

**A formulation with two live instances read first-hand, and its most important claim untested.** The forcing argument (own clock, fan-out, non-truth-apt) is analytic and does not rest on the instances. The two instances are measured this pass ([[terminology-store-anatomy]]): a 176-entry directory store with 160 decision events, and a 106-term single-file dictionary with a 94-line graveyard. The derived-versus-hand-maintained contrast is first-hand within one corpus and is a natural experiment only in a weak sense — the two surfaces differ in more than derivedness (one is prose, one is symbols), so the mechanism is argued, not isolated.

The row this segment replaces called these components *critical and immature even in asf*, and the reading holds up. What is shipped is the storage and the events. What is not shipped anywhere: the cross-store namespace being consumed by any instrument, the notation generator, and — the one that would change how much any of this is worth — **any measurement of whether a governed vocabulary is actually used in the prose it governs.** Both stores could be immaculate and entirely ignored, and nothing currently in place would show it. Treat every claim here about the substore's value as resting on the fan-out argument alone until that measurement exists.

## Working Notes

- The owed measurement, cheap: grep each retired headword across the live tree it governs. Non-zero hits price the graveyard immediately; zero hits across a large corpus is the first real evidence that vocabulary governance takes.
- Named absence, from a 2025 in-estate instance nobody has compared against: a lexicon expressed as a bounded context in YAML, with per-term definition, type, constraints, usage and related-terms. It is a prior answer to this row and it sits unexamined (registered as R-T9b).
- Open: whether notation deserves a third store or is a projection of the term records. The corpus's own schema treats it as a field on the term, which answers the question by construction if the generator is ever written.
- Adjacent: [[decision-records]] (a naming decision is a decision record; the two schemas plausibly compose rather than duplicate), [[history-layer]] (the event log is record-grain history, and this store is its only working instance), [[type-vocabulary-locality]] (why the *type* vocabulary must stay local even as the term store's machinery is shared).
