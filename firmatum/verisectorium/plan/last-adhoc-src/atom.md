---
slug: atom
type: def
depends: []
---

# Atom: the unit a verisectorium is made of

*An atom is one topic, addressed by a stable identity, carrying a declared type and a body that states present truth, with its companions declared rather than implied. It is the unit that everything else in the pattern — views, dependencies, gates, delegation — operates over.*

## The definition

An **atom** is a record with four properties. Each is separable from the others, and an arrangement missing any one of them is doing something else.

**1. One topic.** An atom says one thing. A reader arriving at it should be able to state what it is about in a sentence, and a second claim that wants saying is a second atom. This is the property that lets everything downstream be additive: a view can include this claim without dragging in three others, a dependency can name it precisely, and two agents can work on neighbouring claims without meeting.

**2. A stable identity.** The atom has a name — a slug — that is the thing's identity and nothing else. It does not encode where the atom sits, when it was written, what order it is read in, or how strong it currently is. That identity is what every reference in the corpus points at ([[slug-identity]], [[identities-over-locations]]).

**3. A declared type, from a local vocabulary.** The atom says what kind of record it is. The type is what tells a reader whether it is definitional or claim-bearing, what evidence it owes, and — often overlooked — whether it is replaced by its successor or accumulates ([[write-semantics-declaration]]). The vocabulary of types is chosen per deployment and is not portable between them ([[type-vocabulary-locality]]); what is portable is that a record declares its kind.

**4. A present-truth body, with declared companions.** The body states what is true now, at honest strength, rather than narrating what changed — history lives in a history layer. Everything the atom stands on or carries alongside is declared rather than implied: the prerequisite atoms it depends on, the specimens grounding it, the working notes and events that ride with it ([[atom-as-cluster]]). Present truth is what makes a contradiction between two atoms visible as a collision rather than as an accumulation ([[collision-staleness-detection]]).

### What an atom is not

- **Not a file.** A file is a placement, and placement is a substrate decision — one atom per file is the common and safest arrangement ([[write-safety]]), but a deployment may hold several records in one file or spread one record's parts across several. What makes an atom is the four properties, not the storage.
- **Not a section of a document.** A section's identity is its position in the document; an atom's identity is its own.
- **Not a fixed grain.** How much a "topic" is — a whole result, one fact, a definition — is a design choice with consequences, not a natural boundary ([[atom-grain-parallelism]]).

### On the word

This project calls its own records **segments**, which is the estate's habitual term. Live deployments call theirs by their local kind: claim segments, nomoi, demands, entries, precepts, paper arguments. **Atom** is the pattern-level word, used when speaking of the unit as such rather than of any deployment's instance of it — and it is deliberately not the name of a file format.

## Strength & grounds

**Stipulative.** This segment fixes vocabulary; it makes no claim that atoms so defined are good, necessary, or better than alternatives. The claims are elsewhere and are separately defeasible: that identity must be stable and location-free ([[slug-identity]]), that the grain is chosen partly for parallel work ([[atom-grain-parallelism]]), that the parts run on different clocks ([[atom-as-cluster]]), that write semantics rides the type ([[write-semantics-declaration]]), that present truth is what makes staleness detectable ([[collision-staleness-detection]]).

The one thing worth defending about the definition itself is its *shape*: the four properties are stated as separable so that a deployment can be described honestly as having three of them. That is not decoration — the estate contains such cases, and calling them "not really instances" would lose the information ([[type-vocabulary-locality]], [[cousin-store-lineage]]). The definition earns its keep if it makes those partial instances describable; it fails if every instance turns out to be either all four or none.

## Working Notes

- The founding argument for one-topic-per-file in this estate is recorded in [[navigation-relocation-specimen]], which argues it on navigation grounds — and predicts an exposition layer would become unnecessary, which is where ch. 2 picks it up. Registered as A1 in `plan/TODO.md`.
- Deliberately unsettled here: whether "one topic" can be given any test sharper than a competent reader's judgment. Every live instance in the estate leaves it to judgment; the closest thing to a mechanical rule found so far is a *duplicate* test rather than a grain test (two records are duplicates when they resolve to the same span in the same primary — comproprium's rule, registered as N2).
- The sub-record identity tier is not covered here: ASF gives named atoms *within* a segment (equations, tables) their own kebab-case reference tags resolving independent of position. That tier is registered as A2 in `plan/TODO.md` and belongs to the ch. 1 inner-section gap.
