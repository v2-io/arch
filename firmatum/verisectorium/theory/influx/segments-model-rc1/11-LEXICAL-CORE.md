# The lexical core — identity, edges, and projections under the references lexicon

*Register: **proposed**. Adopted 2026-08-30 at the steward's direction after a whole read of the udon references def store (`firmatum/udon/v2/references/def/` — seven term-groups, every one `proposed`). This file states what that lexicon **supplies**, **corrects**, and **derives** for RC1; it deliberately does **not** restate any definition — per that store's own law, an inlined definition is a fork rather than a copy, so entries are cited and glossed in one line and the primaries win on every conflict. Citation convention used here: term-groups by entry name (`def-binding`), individual terms in that store's own `@{term}` notation. The cross-member reference form for the def store is an open naming question, routed to 90.*

## Why this lexicon, and what its agreement is worth

The two corpora are not strangers meeting. The steward's account of the lineage (2026-08-30), which is why this adoption is principled rather than opportunistic:

> "udon references (address theory as we fondly speak of it…) were primarily needed to specify the udon-store (don), which was *really*… all about allowing document parts to be a graph essentially, with event-sourcing like temporal dimension for full auditability etc. — all of which was for building tools to be able to automate the atom-processing for ASF and others, verisectorium now in particular."

So the def store and RC1 are one family attacking one problem from two altitudes: RC1 asks what a record's truth-state consists in; the references work asks how document parts are named, reached, and versioned so that state can be stored and audited at all. **Under the coherence default their agreement is worth ~zero as corroboration** — shared estate, shared author-family, shared problem, and RC1's coord read the def entries' *style* while writing 02. What adoption actually buys is three things, none of them mutual confirmation:

1. **A more carefully adjudicated vocabulary** for objects RC1 names loosely (identity, edges, staleness) — adjudicated over months, with `:avoid` lists, invariants, and named open questions.
2. **Some external formal warrant, which RC1 has almost none of.** The def store's resolution machinery is verified against the scope-graphs literature at the primaries (van Antwerpen 2018; Rouvoet 2020; Zwaan 2023/2024), with its inheritance conditions stated. Vocabulary imported from there arrives with grounding RC1 cannot manufacture internally.
3. **Divergence prevention.** Two estate corpora were describing the same objects in two dialects; one of them is the more careful.

## Four adoptions

### 1. Identity is a maintained binding, not an invariant

RC1 says *"identity is the slug; nothing mutable participates in it"* and treats that as a property the model may assume. `def-binding` is sharper and, adopted here, truer: a slug reaching a record is a `@{binding}` — a deliberate association held by a `@{maintainer}` — and *"the uniqueness of names is a **norm** a maintainer may police (cheapest at mint time)… instead of an invariant."* Its two failure modes are first-class and must surface: `@{dangle}` (the binding no longer reaches a record) and `@{collide}` (one maintainer holding two mints of one name), under the store's standing discipline that *"collision is never defined away and must be accounted for. Real systems have collisions and are judged by how they surface them."*

**What changes.** Identity stability stops being an assumption and becomes *upkeep with a named owner and two detectable failures* — which is a demand for instruments, not a weakening. A deployment must say who maintains slug bindings and what surfaces dangles and collisions; asf's `lint-outline` is already a partial dangle-detector, and nothing estate-side checks collide. Renaming also lands correctly: `def-location`'s *"more than one = aliases, all equally real"* is the alias-survival law with a definition under it.

### 2. Projections are generators — and there is no untimed evaluation

The deepest adoption, and it *derives* a law RC1 currently asserts. A projection — what turns a trail into a status — is not a reference standing for something; it is a `@{generator}` in `def-generator`'s sense: deferred recorded material that produces material rather than standing for it, and *"to `@{evaluate}` a generator is to perform that production — always from an origin, as of a moment."* Its invariant: *"There is no untimed evaluating, for the same reason there is no untimed resolving"* — the sibling law being `def-resolution`'s *"an 'untimed' answer is either a fiction or an unstated claim that the world has held still."*

**What changes.** A status word cached in frontmatter is an **evaluation carrying its moment**, never a standing fact. RC1's central prohibition sharpens accordingly: the label-lie is not merely "a cell no events support" — it is *an untimed answer*, an evaluation presented as current whose moment has passed. And **certificate decay stops being a separate mechanism**: a certificate that cannot decay is simply an evaluation pretending to be timeless. One law, two consequences, instead of two asserted rules.

### 3. Edges are references, and they reach by two different mechanisms

06 types the edge vocabulary but never says what an edge *is*. Under the lexicon an edge declaration is a `@{reference}`: recorded material standing in for `@{referents}`, written once and used many times, valid to write before its referents exist, resolved from an `@{origin}` as of a moment. Three consequences RC1 lacked:

- **Intended cardinality is declared, and gives misses their meaning.** `def-reference` puts `@{intended-cardinality}` on the reference at writing, *not* on whatever later processes it — so zero against {1,1} means *the writer's model of the world is currently wrong*; three against {1,1} means *the reference narrowed insufficiently*; zero against {0,N} is *simply the current answer*. RC1 today cannot distinguish an edge whose emptiness is a defect from one whose emptiness is a fact.
- **A broken edge is a `@{dangle}`** — a maintenance failure with an owner — rather than an unclassified "broken link."
- **Designator vs description** (`def-match`): an edge that reaches by consulting a binding (`supports: [slug]`) is a `@{designator}`; one that reaches by running a test at use — a glob, a query, "every record citing X" — is a `@{description}`, connecting by `@{match}`. Their failures differ in kind: *"a binding fails through its maintenance, a match through the world's motion."* A match *"fails through the world's motion: the same criteria, run later, quietly pick out something else — detectable only if the reference carries something to check against."* RC1 tacitly assumed every edge was declared; estate practice runs both, and the match-shaped ones need the check-against that only an explicit declaration can supply.

### 4. A record is a referent; manifestation is not identity

RC1 repeats that physical realization is a deployment matter but supplies no vocabulary for the boundary. The lexicon supplies it exactly: a record is a `@{referent}` whose `@{name}` is bound in a `@{scope}`; whether it manifests as a file, a directory, or an element inside a document is manifestation, not identity — and `def-location` lists *"a keyed element in a document"* among its `@{location}` examples, i.e. an inner scope that keeps its own bindings.

**What changes.** Two things RC1 was hand-waving become tractable. **Grain** (Q7; the coord's named nag) is inner referents: per-clause state attaches to keyed inner elements holding their own bindings in the record's scope, so clause-grain `supports:` is a reference into an inner `@{location}` rather than a new mechanism. And **layout migration** — one record per section growing into one record per file growing into a directory — is `NORMS.md`'s organically-expanding BASENAME ladder, which is precisely manifestation walking while the binding holds. The store-scope form of the same law.

## The four closure shapes, derived rather than asserted

03 lists reify / view-ify / cluster / exempt-visibly and claims they close over composites. Under the lexicon they stop being a list and become the four available answers to *how does this material stand in the naming and reaching machinery* — which is what an exhaustiveness claim needs before it can be tested:

| Shape | In lexicon terms | Why it is the honest move |
|---|---|---|
| **reify** | `@{mint}` a `@{binding}` for material that had no `@{name}` | unnamed material cannot be referenced, resolved, cited, or adjudicated; minting the name makes it all four at once |
| **view-ify** | declare it a `@{generator}`, not a `@{referent}` | it produces material from referents at use — hence "may reference and order, may not introduce," and hence "on disagreement, I lose": produced material never outranks its inputs |
| **cluster** | one `@{referent}`, one name, typed inner parts (inner `@{location}`s) | different write-semantics per part without splitting identity; `def-reference`'s own `|term-group` form is the live exemplar |
| **exempt visibly** | place it outside the `population` a `@{match}` tests against | archives must not answer live queries; the only legitimate exemption is one the matching machinery can see |

This is this file's own synthesis and the most exposed thing in it. Its value is that the exhaustiveness claim is now *falsifiable in a specific way*: a fifth legitimate shape would be material that neither gets a name, nor produces, nor sits inside a named thing, nor is excluded from matching — and if such material exists, the four-shape closure fails at that seam.

## Open, jointly

- **`@{moment}`** is load-bearing and undefined in *both* corpora — `def-resolution`'s working notes flag it (*"the as-of index every resolution carries"*), and every RC1 projection is as-of. One missing term, two demanding consumers; the shared demand is evidence it is real. Not solved here.
- **Are trail events themselves referents** with their own bindings (addressable, citable, `depends:`-able), or parts of a record's cluster? The event-sourcing lineage the steward named suggests the former; nothing forces it yet.
- **Accessibility** — `def-resolution` names it as the deliberately-open third component of resolution policy (*may this asker, via this route, reach this thing*). RC1's audience-safety and priming bounds look like the same object seen from the epistemic side; unadjudicated.
- **Citation form** for the def store from RC1 and from theory segments — an instance of the parked wikilink-form call, now with a second consumer.

## Working Notes

- Provenance: all seven def entries read whole 2026-08-30 by this session's agent; the lineage quotation is the steward's, same day, lightly trimmed. Every adoption above is *this file's* mapping of that lexicon onto RC1 — the def entries make no claim about verisectorium, and none of them was written with RC1 in view.
- Register honesty: the def store is `proposed` throughout, exactly as RC1 is; adopting its vocabulary transfers no tier. Where its own warrant is external and verified at primaries (the resolution machinery), that grounding travels with the terms used — and no further.
- Consequential edits made in the same pass, per integration-is-replacement rather than kept-with-a-pointer: 02's RECORD identity invariant and PROJECTION object; 06's new *What an edge is* section. Each is logged in its file.
- Not done here, deliberately: restating any definition (forbidden); re-cutting 03's dimension table against the lexicon (no demand yet); touching the founding questions (Q7's grain answer above wants the steward's eye before it edits 10).
- The steward's standing observation, recorded because it outranks this file: the def entries are the estate's best current exemplar of a mature atom, and the kit's exemplar-seed slot ( [[form-exemplar-seeds]]) probably wants them rather than anything RC1 would compose.
