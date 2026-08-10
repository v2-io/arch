# Sweep: knowledge-organization & rationale-capture design systems

Domain: established academic/professional ontologies for organizing arguments, decisions, rationale, provenance, and bibliographic/epistemic grain — as sources for verisectorium's atom-kind and epistemic-state vocabulary, rather than as things to adopt wholesale. Per-system entries carry: structure (verbatim-or-near from primary source where I could reach it), what it's trying to be true about, its uncertainty/gap handling, the structural feature our atom-kinds would care about, and provenance + my confidence that I verified vs. recalled.

---

## 1. IBIS (Issue-Based Information System)

**Structure.** Three node types connected by typed links, forming a graph rather than a ladder: **Issue** (a question posed) → **Position** (a candidate answer to an issue) → **Argument** (supports or objects to a position). gIBIS/Compendium add typed relations (supports / objects-to / responds-to / generalizes / specializes / questions) and extra node types (Reference, Note, Decision).

**What it's trying to be true about.** The *deliberation structure* of a wicked, contested problem — not a settled body of knowledge. Rittel & Kunz built it explicitly for design/policy problems where the participants themselves disagree about the problem framing, not just the answer.

**Uncertainty/gap handling.** Structurally weak by our standards: there is no native node for "how confident are we," no terminal/resolved state in the base notation, and no place to record "we don't know" as distinct from "no one has raised this yet." Resolution of an issue is a social fact external to the model, not a property the model can hold. This is a real gap worth naming, not softening — IBIS models *contestation*, not *epistemic status*.

**Feature our atom-kinds would care about.** The clean separation of "question" (Issue) from "candidate answer" (Position) from "reason for/against" (Argument) as three distinct node kinds rather than folding all three into one "claim + metadata" object. If our Questions family and Assertions family are graph-adjacent the way IBIS's are, that's a validated shape, not a de novo choice.

**Provenance.** Rittel & Kunz, UC Berkeley, early 1970s; gIBIS (Conklin & Begeman, 1988); Compendium (Open University UK, ongoing). Long-lived, widely cited, but I did not reach a canonical W3C/ACM-grade primary spec — what's available is secondary synthesis (eight2late.com) plus the Compendium project's own docs. **Confidence: recalled from search-pass sources, not independently re-verified against a primary spec** — treat the node-type list as reliable (it's consistent across every secondary source) but the finer-grained link-typing as secondary-sourced.

- https://eight2late.com/2009/07/08/the-what-and-whence-of-issue-based-information-systems/
- https://eight2late.com/2014/11/24/from-information-to-knowledge-the-what-and-whence-of-issue-based-information-systems/
- https://www.argumentree.com/what-is/dialogue-mapping/

---

## 2. QOC (Questions, Options, Criteria)

**Structure.** **Question** (a design issue) → **Option** (a candidate answer) → **Criterion** (a desideratum used to judge options), with explicit **assessment relations** between each Option and each Criterion (support vs. deny/undermine — drawn as solid vs. dashed lines in the original notation).

**What it's trying to be true about.** Two things at once, and this is the structurally interesting part: (1) the space of *design possibilities that were considered* (Options), and (2) the *evaluative standards being applied* (Criteria) — kept as two independent axes rather than one axis. MacLean et al. built QOC partly as a **retrospective reconstruction tool** — explaining a design already made — as much as a prospective planning tool. That's a different epistemic stance from "decide, then record" (cf. ADRs below): QOC explicitly accommodates authoring *after* the fact, by someone other than the decider, from evidence of the artifact.

**Uncertainty/gap handling.** No explicit confidence scalar; uncertainty is implicit in how many Criteria an Option fails and how contested the assessment relations are. No terminal state — the "Design Space Analysis" is a snapshot, understood to be revisable as new Options or Criteria surface.

**Feature our atom-kinds would care about.** The **Criteria as an independent axis from Options** is the reusable idea: it separates "what are we choosing among" from "what are we choosing *by*" as two different kinds of atom, rather than collapsing the evaluative standard into the pro/con argument text the way IBIS does. If our Decisions family ever wants to track "the standard we're judging against" as a first-class, reusable, possibly long-lived atom (distinct from any one decision that invokes it), QOC is precedent.

**Provenance.** MacLean, Young, Bellotti, Moran; *Human-Computer Interaction* 6(3), 1991 — a genuine peer-reviewed journal article, not a blog restatement. **Confidence: recalled from secondary summaries (ACAWiki, ResearchGate figure caption, NAVER Labs project page) — I did not get behind the ACM paywall to the primary text.** The three-part structure and the solid/dashed assessment relation are consistent enough across independent secondary sources that I trust them; nuances of the paper's own caveats I have not seen firsthand.

- https://acawiki.org/Questions,_Options,_and_Criteria:_Elements_of_design_space_analysis
- https://dl.acm.org/doi/10.1207/s15327051hci0603%25264_2
- https://europe.naverlabs.com/history/past-research/design-space-analysis/

---

## 3. SKOS (Simple Knowledge Organization System) — VERIFIED against primary

**Structure, verbatim from the W3C Reference (fetched and confirmed directly).**

Property hierarchy (all `owl:ObjectProperty`):

```
skos:mappingRelation  (sub-property of skos:semanticRelation)
├── skos:closeMatch   (sub-property of skos:mappingRelation)
│   └── skos:exactMatch  (sub-property of skos:closeMatch)
├── skos:broadMatch   (sub-property of skos:mappingRelation AND skos:broader)
├── skos:narrowMatch  (sub-property of skos:mappingRelation AND skos:narrower)
└── skos:relatedMatch (sub-property of skos:mappingRelation AND skos:related)
```

Within a single scheme, the base semantic relations are `skos:broader` / `skos:narrower` (hierarchical) and `skos:related` (associative, non-hierarchical) — the mapping relations above are the **cross-scheme** counterparts of these three plus an added strength gradation.

**Transitivity — this is the load-bearing, precisely-stated engineering decision:**
- `skos:exactMatch` IS declared `owl:TransitiveProperty` (spec section S45).
- `skos:closeMatch` is explicitly declared **NOT** transitive, and the spec states *why*: "to avoid the possibility of 'compound errors' when combining mappings across more than two concept schemes." In other words: two `closeMatch` links chained (A closeMatch B, B closeMatch C) do **not** license inferring A closeMatch C — each hop degrades confidence and chaining silently compounds that degradation into a false certainty if treated as transitive.
- `broadMatch` / `narrowMatch` / `relatedMatch` inherit non-transitivity from their non-transitive parents (`broader`/`narrower`/`related`).

**Symmetry:** `relatedMatch`, `closeMatch`, `exactMatch` are all `owl:SymmetricProperty`. `narrowMatch` is `owl:inverseOf` `broadMatch` (not symmetric with itself — it's the *directional* pair).

**Integrity constraint (S46):** `exactMatch` is declared disjoint from `broadMatch` and `relatedMatch` (and by symmetry/inverse, from `narrowMatch`) — a concept pair cannot simultaneously be asserted exact-equivalent and broader/narrower/related. This is an explicit consistency check the standard itself enforces, not just documentation.

**Documentation/note-typing properties (Section 7, all `owl:AnnotationProperty`, six as sub-properties of the base `skos:note`):**

| Property | Purpose |
|---|---|
| `skos:note` | general documentation, base property |
| `skos:scopeNote` | intended usage scope / applicability |
| `skos:definition` | formal meaning of the concept |
| `skos:editorialNote` | info for editors/administrators only (not end-users) |
| `skos:changeNote` | record of modifications / version history |
| `skos:historyNote` | historical context / provenance of the concept |
| `skos:example` | illustrative usage instances |

**What it's trying to be true about.** Concordance and correspondence between independently-maintained controlled vocabularies (thesauri, classification schemes, subject headings) — SKOS is explicitly *not* trying to be a formal ontology-with-inference-guarantees (it deliberately stays below OWL-DL rigor); it's trying to be true about "how librarians/curators actually relate concepts across schemes in practice," including relations softer than logical equivalence.

**Uncertainty/gap handling.** The genuinely novel idea for us: **uncertainty is handled by *relation type*, not by a confidence scalar.** Instead of "concept A relates to concept B at confidence 0.8," SKOS asks "which of five named relations, each with different composability guarantees, actually holds" — and bakes the compounding-error risk directly into which relations are allowed to chain. That's a structurally different (and arguably more honest) way to represent "how sure are we this correspondence holds" than a numeric confidence value: the relation-kind itself IS the confidence statement, and it's non-transitive by design where chaining would be misleading.

**Feature our atom-kinds would care about.** If verisectorium atoms ever need "this atom in scheme/collection X corresponds to that atom in collection Y," SKOS's mapping-relation family — and specifically the transitive/non-transitive split with its stated compound-error rationale — is the single most directly transferable, most rigorously justified idea in this whole sweep. The note-typing table is also directly relevant to our Exposition family: it distinguishes "what this atom means" (`definition`) from "how it may be used" (`scopeNote`) from "why it changed" (`changeNote`/`historyNote`) from "note for maintainers, not readers" (`editorialNote`) as four genuinely different kinds of annotation, not one generic "notes" field.

**Provenance & confidence.** W3C Recommendation, SKOS Reference (2009, current). **Verified directly against the primary source this session** — the transitivity/symmetry/disjointness claims above are fetched from `skos-reference` itself, not recalled.

- https://www.w3.org/TR/skos-reference/ (primary, fetched and quoted above)
- https://www.w3.org/TR/skos-primer/
- https://arxiv.org/pdf/1302.1224 ("Key Choices in the Design of SKOS" — the design-rationale paper; not re-fetched this session, recalled from search pass)

---

## 4. PROV-O (W3C Provenance Ontology) — VERIFIED against primary; not in original assignment, added because load-bearing

**Structure, from the primary spec (fetched and confirmed).**

Three core classes:
- **`prov:Entity`** — "a physical, digital, conceptual, or other kind of thing with some fixed aspects."
- **`prov:Activity`** — "something that occurs over a period of time and acts upon or with entities"; carries temporal bounds (`startedAtTime`/`endedAtTime`).
- **`prov:Agent`** — "something that bears some form of responsibility for an activity taking place, for the existence of an entity, or for another agent's activity." Subclasses: `Person`, `Organization`, `SoftwareAgent`.

Core relations (domain → range, stated meaning):

| Relation | Domain → Range | Meaning |
|---|---|---|
| `wasGeneratedBy` | Entity → Activity | completion of production of a new entity by an activity |
| `used` | Activity → Entity | beginning of an activity's utilization of an entity |
| `wasDerivedFrom` | Entity → Entity | transformation of one entity into another, or an update producing a new entity |
| `wasAssociatedWith` | Activity → Agent | assignment of responsibility to an agent for an activity |
| `wasAttributedTo` | Entity → Agent | ascribing an entity to an agent |
| `wasInformedBy` | Activity → Activity | exchange where one activity uses an entity generated by another |
| `actedOnBehalfOf` | Agent → Agent | assignment of authority/responsibility from one agent to another for a specific activity |

**Revision/invalidation vocabulary — directly relevant to "how does the model handle a claim being superseded":**
- `wasInvalidatedBy` / `invalidatedAtTime` — records when an entity ceases to be valid (a specific, dateable event, not just a status flag).
- `wasRevisionOf` — links an entity to an earlier entity it substantially updates (this is PROV's "supersedes" relation, entity-to-entity rather than a document-status field the way ADRs do it).
- `alternateOf` — links entities that present different aspects of "the same thing" without one superseding the other (distinct from revision).
- **Qualification pattern**: PROV lets you attach extra metadata to a *specific instance* of a relation (e.g., this particular `wasGeneratedBy` edge, not the Entity or Activity in general) via an intermediate "Influence" object, without needing to retract or restate the base assertion. This is PROV's answer to "how do you add confidence/detail to one specific claim-edge without touching the rest of the graph."

**What it's trying to be true about.** *Where something came from and who is responsible for it* — origin and custody chains for data/claims/artifacts, independent of the content's truth-value. PROV explicitly does not attempt to say whether an entity's content is *correct* — only how it came to exist and who stands behind it.

**Uncertainty/gap handling.** PROV's philosophy is additive, not corrective: corrections happen via **new provenance assertions** (`wasRevisionOf`, `wasInvalidatedBy`) layered on top of the graph, never by deleting or mutating a prior assertion. This is structurally close to the ADR "supersede, don't rewrite" discipline (below) but generalized to arbitrary entities/activities/agents rather than just decision documents.

**Feature our atom-kinds would care about.** This may be the single most directly on-target system for Joseph's framing "what they need to be true about, and how do they explain their gaps" — PROV is a standardized, widely-adopted (W3C REC since 2013, used across scientific-data-provenance, Wikidata, many linked-data projects) answer to exactly "attach honest who/when/derived-from-what metadata to a claim without conflating that metadata with the claim's content," which is close to verisectorium's core design problem. The Entity/Activity/Agent split (a *thing*, vs. the *process* that produced/changed it, vs. the *responsible party*) is a three-way distinction our epistemic-state vocabulary may currently be flattening.

**Provenance & confidence.** W3C Recommendation, 2013, current. **Verified directly against the primary spec this session** (table above is fetched, not recalled) — though I did not fetch the full PROV-DM / PROV-CONSTRAINTS companion documents, only PROV-O itself, so subtler constraint rules (e.g., exact cardinality restrictions) are not independently confirmed.

- https://www.w3.org/TR/prov-o/ (primary, fetched and quoted above)
- https://www.w3.org/TR/prov-dm/ (the underlying data model — not fetched this session)

---

## 5. RDF statement-level metadata: reification / named graphs / singleton properties / RDF-star

**Structure.** Not one system but four competing engineering answers to one problem — attaching metadata to a single *statement* rather than to an entity:
- **Standard RDF reification** (`rdf:Statement` with `rdf:subject`/ `rdf:predicate`/`rdf:object`): describes a triple without asserting it — a well-known point of confusion, since a reified statement about "S P O" does not itself claim S-P-O is true.
- **Named graphs**: group whole sets of triples under one shared provenance context (a graph-level, not statement-level, grain).
- **Singleton properties**: mint a unique predicate-instance per statement so metadata attaches directly, no wrapper node.
- **RDF-star**: triples-as-terms, `<<s p o>> meta:confidence 0.8`; the currently-advancing W3C-track consolidation of this whole problem space.

**What it's trying to be true about.** Confidence, source, and timestamp for one specific claim, as distinct from claims about the same subject in general.

**Uncertainty/gap handling.** The unresolved-by-design nature of this cluster is itself the finding: the RDF community has tried at least four structurally different mechanisms over ~25 years and none has fully won — each trades grain (per-statement vs. per-batch) against performance and syntactic weight. Worth reporting as "the field has repeatedly hit this wall and not settled it," not as a solved precedent to copy.

**Feature our atom-kinds would care about.** Direct kin to verisectorium's "the atom carries its own epistemic state without conflating state with claim" problem — the same tension (is metadata a property of the claim-node, a property of a batch/context, or expressed by promoting the claim itself to a first-class subject) recurs, and the standard reification confusion ("describing ≠ asserting") is a sharp cautionary example of a design that looks right and quietly breaks the thing it's meant to protect.

**Provenance & confidence.** Not independently re-fetched this session — this entry is **carried over from the search pass, recalled not verified**. The RDF-star spec is genuinely in-progress (W3C RDF-star Working Group, started 2021) so its exact final shape should be treated as moving ground, not settled fact, if this gets cited downstream.

- https://arxiv.org/pdf/1509.04513
- https://arxiv.org/pdf/2211.16195
- https://www.researchgate.net/publication/351297611_Benchmarking_RDF_Metadata_Representations_Reification_Singleton_Property_and_RDF

---

## 6. ADRs (Architecture Decision Records) — VERIFIED against primary (Nygard's original post)

**Structure, verbatim from Nygard's 2011 post (fetched and confirmed).**

Four sections:
1. **Title** — short noun phrases, e.g. "ADR 1: Deployment on Ruby on Rails 3.0.10."
2. **Context** — "describes the forces at play, including technological, political, social, and project local" (deliberately value-neutral language, per Nygard — stating the forces, not yet the resolution).
3. **Decision** — "our response to these forces. It is stated in full sentences, with active voice." ("We will…")
4. **Consequences** — "describes the resulting context, after applying the decision" — including tradeoffs, not just benefits.

**Status field values, exactly as Nygard states them:**
- **"proposed"** — "if the project stakeholders haven't agreed with it yet."
- **"accepted"** — "once it is agreed."
- **"deprecated"** or **"superseded"** — "if a later ADR changes or reverses a decision."

**Immutability rationale, Nygard's own words:** "It's still relevant to know that it *was* the decision, but is *no longer* the decision." Nygard's stated reasoning is explicitly historical/anti-repetition: keeping the record prevents a team from unknowingly re-litigating and re-reversing a decision when circumstances shift back.

**What it's trying to be true about.** What a team actually decided, when, and why, framed as an accretive log rather than a living/mutable "current-state" document — the decision-as-of-the-time is the unit of record, not "our current best understanding."

**Uncertainty/gap handling.** Minimal and coarse — a 3-4-value status field, no confidence scalar, no explicit "gap" state (an unaddressed forcing factor would just be absent from Context, not flagged). The honesty mechanism is structural rather than expressive: don't edit history, supersede it with a link. `superseded`-by-itself (as a bare word) is under-specified without the forward link to the replacing ADR — practice (not the original post) added that convention; MADR (the actively maintained community template, adr.github.io/madr) formalizes it further and is the living current version of this idea, though I did not re-verify MADR's exact current field list against its repo this session (recalled from search pass, not re-fetched).

**Feature our atom-kinds would care about.** The cleanest, most battle-tested (Nygard 2011 → now near-universal in software engineering) instance of an **append-only, supersession-not-mutation lifecycle** for Decisions — directly relevant if our Decisions family wants a minimal, proven status vocabulary rather than a larger invented one. Notably it is industry-blog-provenance, not standards-body — genuinely load-bearing by adoption, not by institutional authority, which is itself worth flagging as a different *kind* of "battle-tested" than SKOS/PROV-O/LRM.

**Provenance & confidence.** Nygard, Cognitect blog, Nov 2011. **Verified directly this session** — the status values and immutability quote above are fetched from the primary post, not recalled.

- https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions.html (primary, fetched and quoted above)
- https://adr.github.io/madr/ (current community-canonical template — not re-fetched this session)

---

## 7. FRBR / IFLA LRM (Work / Expression / Manifestation / Item — WEMI)

**Structure — partially verified via secondary academic source (code4lib article quoting the LRM spec), primary PDF itself was inaccessible (blocked/unparseable both times attempted).**

Four entities (Group 1 in FRBR terms, "WEMI"), as quoted by a secondary source citing the LRM text directly:
- **Work**: "the intellectual or artistic content of a distinct creation."
- **Expression**: "a distinct combination of signs conveying intellectual or artistic content."
- **Manifestation**: "a set of all carriers assumed to share the same characteristics as to intellectual or artistic content and aspects of physical form."
- **Item**: "an object or objects carrying signs intended to convey intellectual or artistic content."

**Boundary rules (from the same secondary source's worked examples, not the primary spec's own boundary-rule prose, which I could not retrieve):**
- Expression vs. Manifestation: different *carriers/formats* with identical intellectual content (e.g., the same dataset as CSV vs. Excel) are different Manifestations of the *same* Expression — form/language changes make a new Expression, carrier/format changes make a new Manifestation of the same Expression.
- Manifestation vs. Item: exact bitstream/physical copies are different Items of the *same* Manifestation; the Manifestation is the "edition," the Item is "this copy."
- Work vs. Expression: content differences that are still recognizably the "same basic data" stay the same Work; a derived dataset with materially different informational content constitutes a new Work.

**Group 2 / Group 3 (from the same secondary source, not independently confirmed against the primary spec):**
- **Group 2** ("responsible entities"): persons and corporate bodies with agency over the resource — creators, publishers.
- **Group 3**: subject/topical entities.
- The secondary source explicitly notes it does **not** detail the exact relation types linking Groups 2/3 to WEMI — this is a real gap in my verification, not a settled fact I'm omitting for space.

**What it's trying to be true about.** *What thing is even being talked about* when people casually say "the same book" — separating "the idea," "a particular textual/linguistic realization of the idea," "a particular published/physical packaging," and "this one copy" as four different levels of the same referent, because library cataloging needs to know which level a given metadata field (translator? ISBN? shelf mark?) actually belongs to.

**Uncertainty/gap handling.** WEMI itself carries no native confidence/epistemic-status vocabulary — it's a pure grain/identity ontology, orthogonal to truth-value or confidence. Ambiguity in practice (is this really a "new Expression" or just a minor variant?) is handled by cataloger judgment against RDA rules, not by the model itself flagging uncertainty.

**Feature our atom-kinds would care about.** The clean four-level grain separation — abstract content / realized form / packaged product / concrete instance — is a genuinely different axis from anything in our current taxonomy's epistemic ladders, and if our Definitions/Assertions families currently conflate "the same claim restated differently" (Expression-level) with "a literally different claim" (Work-level) with "this specific file/commit carrying it" (Item-level), WEMI is 25+ years of library-science precedent for keeping those apart.

**Provenance & confidence — important caveat.** IFLA LRM (2017, consolidating FRBR/FRAD/FRSAD), genuinely canonical and standardized (underlies RDA, used by most of the world's library catalogs). **I was unable to independently verify the primary PDF this session** — the IFLA LRM PDF returned HTTP 403 on first attempt and was an unparseable/corrupted binary stream on the second (via rdatoolkit.org). What's reported above is one step removed: a secondary academic article (code4lib, peer-reviewed practitioner venue) that itself quotes/paraphrases the LRM spec, fetched and confirmed this session, but **not the LRM spec directly**. Flagging this explicitly per the steward's standard — this is the one system in the sweep where I have not closed the loop to the actual primary text despite two attempts, and the boundary-rule details above are the weakest-sourced claims in this document as a result.

- https://www.ifla.org/files/assets/cataloguing/frbr-lrm/ifla-lrm-august-2017_rev201712.pdf (primary — attempted, HTTP 403)
- https://www.rdatoolkit.org/sites/default/files/rsc/IFLA%20LRM%20what%20and%20why.pdf (attempted — PDF unparseable by the fetch tool)
- https://journal.code4lib.org/articles/16491 (secondary, fetched and quoted above — this is the actual source of the definitions given)

---

## 8. Toulmin Model of Argument

**Structure — could not reach primary text; both attempted primary/scholarly PDFs (Hitchcock's McMaster paper, direct 1958 text) returned unparseable binary streams; the account below is from general knowledge plus a search-pass synthesis, explicitly recalled not verified.**

Core triad: **Claim** (the conclusion) ← **Data/Grounds** (the evidence) ←  
**Warrant** (the licensing inference-rule connecting grounds to claim, often left implicit). Secondary triad, present when the argument is contested:  
**Backing** (justification for the warrant itself, when it's challenged),  
**Qualifier** (hedges the claim's strength — "presumably," "necessarily," "in most cases"), **Rebuttal** (explicit conditions under which the claim does NOT hold).

**What it's trying to be true about.** The anatomy of a single natural- language argument (originally: legal/practical reasoning, not formal logic) — Toulmin's express point in 1958 was that formal-logical validity is the wrong lens for real arguments, which instead succeed or fail by field-dependent warrants.

**Uncertainty/gap handling — this is the reason the system is in the sweep at all, and I want to flag clearly that I have NOT verified it against primary text this session.** As generally reported: **Qualifier** is a named, first-class slot for confidence-language attached to the claim ("necessarily" vs. "presumably" vs. "in most cases" are different Qualifier values, not one confidence scalar), and **Rebuttal** is a separate named slot for known conditions under which the claim fails — i.e., "how sure am I" and "when does this break" are kept as two different structural elements rather than one hedge. The Warrant/Backing pair is reportedly a second-order version of the same move: the *inference rule itself* can be doubted, and Backing is the recursive justification of that doubt — a plausible structural candidate for "uncertainty about uncertainty," but **I could not confirm this session that Toulmin's own text frames it this way rather than this being a later pedagogical gloss** (both PDF fetches for this system failed to yield readable primary text; what's above should be treated as plausible-and-widely-repeated, not confirmed).

**Feature our atom-kinds would care about.** If the Qualifier/Rebuttal split holds up under actual verification, it's the clearest classical precedent for separating "confidence in the claim" from "known exception conditions" as two different fields — which speaks directly to Joseph's "uncertainty about uncertainty" framing. **This is flagged here as the single most important item in the whole sweep still needing primary verification before being relied upon** — everything else in this file has at least one successful primary or credible-secondary fetch behind it; this one does not.

**Provenance & confidence.** Stephen Toulmin, *The Uses of Argument*, 1958. Foundational to nearly all downstream computational-argumentation work (including AIF, below). **Confidence: NOT verified this session — both attempted fetches (Hitchcock/McMaster scholarly PDF, Wikipedia) failed (binary-unparseable and 404 respectively). Everything in this entry is recalled from training/search-pass synthesis and should be spot-checked against a real copy of Toulmin or a clean secondary source before being treated as settled.**

- https://www.humanities.mcmaster.ca/~hitchckd/Toulminswarrants.pdf (attempted, unparseable)
- https://en.wikipedia.org/wiki/Toulmin_%28argumentation%29 (attempted, 404 — likely a slightly wrong slug; a working Wikipedia article on the Toulmin model almost certainly exists and should be retried, e.g. "Toulmin model of argumentation" or "Stephen Toulmin")
- https://www.sjsu.edu/writingcenter/docs/handouts/Toulmin%20Model%20of%20Argumentative%20Writing.pdf (not attempted this session)

---

## 9. Argument Interchange Format (AIF)

**Structure (recalled from search pass, not re-verified this session).** A typed bipartite graph: **I-nodes** (Information — the actual claim content) and **S-nodes** (Scheme applications), the latter subdivided into **RA-nodes** (rule-of-inference application), **CA-nodes** (conflict application — i.e., attack/rebuttal), and **PA-nodes** (preference application — i.e., "argument X outweighs argument Y"). A **Forms Ontology** layer types S-node instances against named, catalogued argumentation schemes (e.g., "this RA-node instantiates Argument from Expert Opinion").

**What it's trying to be true about.** An interchange format for computational argumentation *tools* to exchange argument-graph data with each other — an engineering/interoperability goal more than a knowledge-representation-for-its-own-sake goal.

**Uncertainty/gap handling.** Handled via the CA-node (conflict) and PA-node (preference) types — disagreement and relative-strength-of-competing- arguments are first-class edge-types, not scalar weights on I-nodes. This is structurally similar to SKOS's "the relation-kind IS the confidence statement" move, applied to argumentation instead of concept-mapping.

**Feature our atom-kinds would care about.** The three-way split of S-node into inference / conflict / preference is a clean generalization beyond IBIS's single generic "Argument" node — worth knowing about if our atom relations ever need to distinguish "this atom supports that one via inference" from "this atom conflicts with that one" from "this atom is preferred over that one when both hold," as three distinct relation-kinds.

**Provenance & confidence.** Multi-institution effort, Chesnevar et al. 2006, backed by University of Dundee Centre for Argument Technology (arg-tech.org).  
**Not re-verified this session** — carried over from the search pass, and I did not attempt to re-fetch the primary AIF spec PDF or the Wikipedia orientation page this round. Treat as recalled, medium-confidence.

- http://www.arg-tech.org/wp-content/uploads/2011/09/aif-spec.pdf (not re-fetched this session)
- https://en.wikipedia.org/wiki/Argument_Interchange_Format (not re-fetched this session)

---

## 10. Zettelkasten note-typing (Luhmann / Ahrens)

**Structure (recalled from search pass, not re-verified this session).**  
Three note types by function, not by status: **Fleeting notes** (transient capture, meant to be processed within days or discarded — explicitly disposable), **Literature notes** (brief own-words summaries of a source, always carrying a bibliographic pointer at creation time), **Permanent notes** (the only durable unit — self-contained, written as if for a stranger, deliberately decontextualized from the moment of capture so it survives being re-read years later out of its original context).

**What it's trying to be true about.** Not a claim's truth-status at all — a note's *survivability relative to the author's own future forgetting*. This is a genuinely different axis than confidence, agreement, or review status: it's asking "will this still make sense and matter to me (or a stranger) later," not "is this correct" or "has this been reviewed."

**Uncertainty/gap handling.** None in the epistemic sense — Zettelkasten is about *durability of form*, not confidence in *content*. Worth flagging that this is the one system in the sweep that answers a genuinely different question than the others, which is itself informative: it shows that "kind" and "epistemic status" and "durability" can be three orthogonal axes rather than points on one ladder, and this system exists almost entirely on the third axis.

**Feature our atom-kinds would care about.** The Fleeting/Literature/ Permanent distinction is a plausible model for our Exposition & Pedagogy family: distinguishing scratch material (discardable, not meant to survive) from properly-sourced-but-still-someone-else's-words material (Literature, requires provenance pointer) from genuinely-integrated, self-contained, durable material (Permanent) — organized by *what the atom is for*, not by how sure anyone is about it.

**Provenance & confidence.** Weakest-provenance entry in this set — Luhmann's own practice was never formally published as a spec; Ahrens's 2017 book is a popular/pedagogical restatement, not a standards document, and the practitioner-forum discussions cited are community consensus, not authority.  
**Not re-verified this session**; recalled from search pass. Reliable enough as "widely and consistently described this way across independent practitioner sources" but should not be cited with the confidence of SKOS or PROV-O.

- https://zettelkasten.de/posts/concepts-sohnke-ahrens-explained/
- https://forum.zettelkasten.de/discussion/2879/literature-notes-vs-permanent-notes

---

## What surprised me / what we did not ask about

**The pattern across the strongest systems: uncertainty is handled by *relation-kind*, not by a confidence scalar.** SKOS (exactMatch vs. closeMatch, transitive vs. not), PROV-O (wasDerivedFrom vs. wasRevisionOf vs. alternateOf), AIF (inference vs. conflict vs. preference edges), and arguably Toulmin (Qualifier vs. Rebuttal as separate slots) all converge on the same move: instead of one number expressing "how sure," they use several *named relation types* with different formal properties (transitive or not, symmetric or not, chainable or not), and the choice of which relation type applies **is** the honesty statement. This is a genuinely different design philosophy than a single confidence field with a value, and it recurred often enough across unrelated fields (library science, provenance tracking, computational argumentation) that I'd flag it as the strongest single takeaway of this sweep — worth the coordinator's direct attention as a candidate shape for verisectorium's own epistemic-state vocabulary, rather than a numeric-confidence-plus-notes design.

**The SKOS "compound error" rationale is a genuinely reusable engineering argument, not just a convenient fact.** The explicit reasoning — closeMatch isn't transitive *because* chaining approximate correspondences compounds error silently — is exactly the kind of "why," not just "what," that Joseph asked for. It's rare to find a standards body stating the failure mode it's defending against this plainly.

**What I did not get to, and should be flagged as a real gap rather than silently dropped:** (1) Toulmin's actual primary text — both attempts failed and everything in that entry is unverified recall; this is the weakest section of the document by a wide margin given how directly its Qualifier/Rebuttal split speaks to "uncertainty about uncertainty." (2) The IFLA LRM primary spec — verified only one hop removed, via a secondary article's quotations. (3) I did not search adjacent fields that plausibly have load-bearing material and were never in scope for this pass: **legal citation/precedent systems** (how case law handles "overruled but not yet formally superseded," "distinguished," "questioned but not overruled" — these are exactly the fine-grained supersession states our Decisions/ Assertions families might want and I suspect common law has centuries of this); **scientific replication/retraction vocabularies** (COPE's retraction guidelines, "expression of concern" as a distinct state from "retracted"); **epistemic-logic/probability calculi** (Bayesian confirmation, Dempster- Shafer belief functions) for the actual mathematics of "uncertainty about uncertainty" that Toulmin only gestures at qualitatively; and **version- control/git's own model** of supersession (a much more mechanically battle-tested append-only-history system than ADRs, running at vastly larger scale). None of these were assigned to this pass and I did not chase them, but they seem like plausible next-domain candidates given what actually turned out to be load-bearing here.

**One structural tension worth surfacing rather than resolving:** IBIS/QOC model *contested, unsettled* deliberation with no terminal state, while ADRs/PROV-O model *settled* decisions/facts with an explicit supersession-not-mutation discipline. Our taxonomy may need both postures available on different atoms simultaneously — some atoms are permanently "in deliberation" (more IBIS-shaped) and some are "decided, until superseded" (more ADR-shaped) — rather than one universal ladder covering both. Whether that's already implicit in the Assertions vs. Decisions family split, or a genuine gap, is a question for the coordinator and Joseph, not something I resolved here.
