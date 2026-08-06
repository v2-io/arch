---
slug: post-names-are-interface
form: postulate
type-expected: postulate
status: axiomatic
max: axiomatic
state: [drafted]
depends: [post-total-turnover]
---

# Postulate: Names Are the Interface

Names are the collection's user interface: every reader meets names before content, and a poor name charges compounding interest per encounter.

## Formal Expression

*[Postulate (names-are-interface)]*

Every access path into a verisectorium runs through names before it reaches content: an outline is scanned as a column of slugs and titles; a cross-reference is followed or not on the strength of its slug; a grep, a corpus verb, a quiz anchor, a `depends:` entry, a delegation brief all address the corpus by name. The name layer is therefore not decoration on the content — it is the interface through which all content is reached, and its quality is paid for (or collected on) at *every* encounter by *every* reader.

Two consequences are taken as part of the postulate's content:

1. **Per-encounter economics.** A good name compresses its concept's load-bearing intuition into a few syllables that survive working-memory pressure; a poor name forces the reader to re-derive what the concept means on each encounter. Under total turnover ( [[post-total-turnover]]) that cost is charged per reader per encounter, forever — naming is the highest-frequency surface the comprehension economics ( [[claim-comprehension-economics]]) apply to.
2. **Co-evolution.** Because the interface is met before the content, vocabulary and corpus can never work independently: they co-evolve coherently or they drift apart, with the drift charged at the interface. The lexicon is therefore an embedded, first-class foundation of every store — a ubiquitous, bounded, shared vocabulary in the domain-driven-design sense — not an optional glossary appended later.

## Epistemic Status

Postulate — near-definitional of how reading works, and deliberately placed as Organ II's ground: the claims and formulations of the vocabulary organ ( [[claim-naming-criteria]], [[form-terminology-store]], [[disc-notation-organ]]) presuppose it. The consequence claims are where it touches falsifiable territory: the per-encounter cost asymmetry between good and poor names is measurable in principle (comprehension-time deltas per encounter), and the co-evolution claim predicts observable drift symptoms (paraphrase proliferation, retired terms persisting in prose, hand-restated definitions diverging from their source) in any instance whose vocabulary layer is neglected — symptoms the estate has in fact exhibited.

## Discussion

**The interface framing does real work beyond metaphor.** Calling names a *user interface* imports the right design posture: interfaces are engineered for their users' cognition, evaluated per interaction, and versioned with care because changes break users. All three transfer exactly. A community can argue about, extend, and apply a concept with a graspable name collectively — the name gives a group of minds shared purchase; a concept reachable only through a decoder ring ($\alpha_1$ / $\alpha_2$ / $\beta$ sub-scope partitions) stays private to whoever last derived it, however good the underlying content. The estate's own contrast pair: *satisfaction gap* and *control regret* organize themselves in a reader's head after one exposure; the sub-scope subscripts must be re-looked-up on every encounter. Same content quality; opposite interface quality.

**Agents sharpen the postulate rather than escaping it.** Agent readers meet names even more exclusively than humans do: slug greps, wikilink resolution, outline scans, and `depends:` graphs are name-mediated operations with no accompanying visual or spatial context to compensate. And under 100% turnover, no reader ever amortizes a bad name — every session pays first-encounter price.

**Boundary with the neighboring organs.** The postulate grounds *why* the vocabulary organ is first-class; *what* makes a particular name good is [[claim-naming-criteria]]'s territory; *how* vocabulary is stored and served is [[form-terminology-store]]'s. Notation ( [[disc-notation-organ]]) is the formal twin: symbols are names whose users are equation readers.

## Working Notes

- Frontmatter schema provisional pending the epistemology decision (see OUTLINE working notes).
- Open: the term-marking convention for first-definitions and special terms in segments and outlines (`[[term/x|x]]`-style, riding the cross-member namespace scheme) is undecided; when it lands, this segment's "every access path runs through names" gains a concrete enforcement surface.
- Open: whether the postulate should state the *bounded* half of the DDD import explicitly (shared vocabulary scoped per context, with declared bridges between contexts) or leave that to [[form-terminology-store]]. Currently left to the store.
