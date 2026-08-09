# Terminology / Lexicon Attribute Survey

*A usable report of what external standards track about terms, what the estate’s own lexicons track (with examples and likely intent), and the higher-level multi-lexicon concerns that sit above any single entry schema.*

**Audience.** Someone designing or consolidating a terminology store — not a raw inventory dump.

**Method.** External material drawn from ISO 704 / ISO 10241, ISO 30042 (TBX) and the ISO 12620 data-category tradition, SKOS (W3C), MultiTerm-style termbases, controlled-vocabulary practice, and DDD ubiquitous-language practice. Estate material drawn from live or archived files under `~/src/` (ASF terminology store, vivarium LEXICON, paths LEXICON, rowan LEXICON.yaml, VidAngel lexicon-middleman, zoetica/synaptic lexicons, epistemic_tribunal YAML, udon GLOSSARY, embeddings standardized-terms CSV, and related notes). Hedged claims about *why* a field exists are marked as inference where the source does not state intent.

**Not in scope.** The entries themselves (definitions of “agent,” “promise,” etc.). Only the *kinds of things* recorded about entries.

---

## How to read this report

Three layers of concern keep getting mixed in short tables. They are deliberately separated here:

1. **Entry-level attributes** — what one term/concept record carries (definition, aliases, status…).
2. **Lexicon-level attributes** — properties of the whole vocabulary as an artifact (present-truth policy, generated view, authority chain).
3. **Multi-lexicon / multi-context attributes** — how vocabularies relate across projects, bounded contexts, and external frameworks (exactMatch maps, anti-corruption translation, collision ledgers).

Most of the confusion in the earlier sketch (“where is bounded context?”) came from putting layer-3 concerns into a layer-1 checklist. Bounded context is almost never a *field on a term*; it is *which lexicon the term belongs to*, plus *how that lexicon talks to neighbors*.

---

# Part I — External standards and practices

What the wider world has already decided is worth tracking, and why.

## 1. ISO 704 and the concept–designation split

**ISO 704** (*Terminology work — Principles and methods*) is the conceptual foundation most modern termbases still rest on, even when they never cite it. Its core move:

> Objects are perceived → **concepts** are formed → **designations** name those concepts → **definitions** delimit the concepts.

A **terminological entry** (also called a **concept entry**) is *collection of data related to only one concept* — not “one word.” Several surface forms can belong to one concept; one surface form can be ambiguous across concepts (homographs).

### What this implies for fields

| Concern | What it means in practice |
|---------|---------------------------|
| Concept identity | A stable id for *the idea*, independent of spelling |
| Designation | A concrete string (or symbol) used to refer to that concept |
| Preference among designations | Preferred vs admitted vs deprecated vs forbidden *for the same concept* |
| Definition | Delimits the concept (usually intensional: genus + differentia) |
| Subject field | Domain slice that keeps “phase” in geology from colliding with “phase” in vivarium |

**Worked example (ISO-style, not estate data).**  
Concept C17: “smallest coherent narrative unit of a serial drama.”

- Preferred designation (en): *episode*
- Admitted: *chapter* (in some dialects)
- Deprecated / avoid in this domain: *show*, *video* (too broad / wrong grain)
- Definition: “The smallest distinct watchable unit, grouped with zero or more others in a show…”
- Subject field: streaming / SVOD content model

That is exactly the shape the VidAngel middleman entry was reaching for (see Part II).

**ISO 10241-style minimum entry** (as restated in terminology-management practice documents):

- Entry identifier  
- Subject field  
- Definition  
- Term(s)  
- Source information  

Optional: grammatical information, context examples, notes, multimedia, cross-references, administrative metadata.

**Why it matters for us.** ASF and vivarium are *concept-oriented in spirit* (one slug = one concept) but sometimes *surface-oriented in practice* (renames rewrite the preferred form without always reifying “designation objects”). When “logozoetic agent” became “ELI,” the *concept* stayed; the *designation* changed. Decision events record that; a pure ISO model would show multiple designations with preference ranks on one concept id.

---

## 2. TBX (ISO 30042) and the data-category tradition (ISO 12620)

**TBX (TermBase eXchange)** is an XML exchange format for structured, concept-oriented terminological data. It is how tools hand termbases to each other without losing fields.

The important architectural claim is the **three-level entry**:

```
concept (language-independent)
  └── language section (e.g. English)
        └── term section (each surface form)
              └── term notes (POS, gender, status, …)
```

A **data category** is simply a named kind of field (definition, part of speech, subject field, context, …). ISO 12620 is the tradition of *registering and documenting* those categories so systems interoperate — “what does `usageNote` mean?” is answered by a shared registry, not by each vendor inventing a synonym.

### Typical TBX / termbase data categories (with intent)

**At concept level**

- **Definition** — delimits the concept (usually once per language or once globally).  
- **Subject field** — topical classification.  
- **Concept relations** — broader / narrower / related (sometimes more specific).  
- **Administrative** — entry number, creation/modification dates, owner.

**At language level**

- **Language tag** — which language these designations are in.  
- Language-specific definition or note when translation is not one-to-one.

**At term (designation) level**

- **Term string** — the actual spelling.  
- **Term type** — full form, short form, acronym, abbreviation, variant, …  
- **Term status** — preferred, admitted, deprecated, **forbidden** (brand or accuracy control).  
- **Part of speech**, **grammatical gender**, number — needed for generation and agreement in translation.  
- **Context** — an *attested sentence* showing the term in use (not the same as a definition).  
- **Usage note** — register, audience, “do not use in UI,” etc.

**Nuance that short lists lose**

- **Context vs example vs definition.**  
  - *Definition*: “what the concept is.”  
  - *Context*: “Here is a real sentence where the term appears” (translation memory gold).  
  - *Example*: “Here is a constructed illustration of the concept.”  
  Estate glossaries almost always have definition and sometimes example; almost never TBX-style *context sentences*.

- **Forbidden is not the same as deprecated.**  
  Deprecated = “we used to say this; prefer the new form.”  
  Forbidden = “never use this string” (legal, brand, safety, or systematic false friend). MultiTerm exposes this as a first-class term status. Vivarium’s `|not` / SUPERSEDED and middleman’s `avoid` are cousins of forbidden, but they live on the concept or as free lists rather than as status-on-designation.

- **Concept-oriented vs term-oriented tools.**  
  A glossary that is “alphabetical list of words with definitions” is term-oriented. TBX forces concept orientation: one concept, many terms. ASF’s one-file-per-slug store is concept-oriented; zoetica’s alphabetical markdown is more term-oriented.

**Why TBX exists.** Localization teams need to ship *approved* product language across tools without retyping. Fields that feel bureaucratic (POS, gender, forbidden) earn their keep when a CAT tool must highlight “don’t say *customer* — say *member*” in twenty languages.

---

## 3. SKOS (Simple Knowledge Organization System)

**SKOS** is a W3C RDF vocabulary for thesauri, taxonomies, and controlled vocabularies on the Web. It is *lighter* than OWL ontologies: it models *concepts and labeling*, not full logical axioms.

### Core SKOS properties, explained

**Labels (how humans write the concept)**

| Property | Intent | Example |
|----------|--------|---------|
| `prefLabel` | The one preferred label *per language* | “Emergent Logozoetic Intelligence” |
| `altLabel` | Synonyms, variants, spelling alternatives | “ELI”, “logozoetic agent” |
| `hiddenLabel` | Strings useful for *search* that should not display | common misspellings, retired spellings you still want to find |

**Why `hiddenLabel` is clever.** After a rename, people still type the old string. Prefer not to show it as a legitimate synonym; do want search and lint tools to resolve it. The estate often puts legacy forms in `aliases` or SUPERSEDED without distinguishing “display as synonym” vs “resolve but never recommend.”

**Documentation notes (different audiences for prose)**

| Property | Intent |
|----------|--------|
| `definition` | What the concept is |
| `example` | Illustration |
| `scopeNote` | Where the concept applies / limits of applicability (“only in multi-agent composition”) |
| `historyNote` | Public-facing history (“renamed from X in 2026”) |
| `editorialNote` | Private workflow note for maintainers (“wait for Joseph’s call on register”) |
| `changeNote` | Changelog-style note for a specific edit |
| `note` | Generic catch-all |

**Estate parallel.** ASF’s `internal_note` is essentially `editorialNote` (never rendered to LEXICON.md). Vivarium’s SUPERSEDED is closer to `historyNote` *extracted* from the live entry. The ASF decision-event body is change/history for the *name*, not always for the definition.

**Semantic relations (within one scheme)**

- `broader` / `narrower` — hierarchical (not necessarily transitive in the weak form; SKOS also has transitive variants).  
- `related` — associative, non-hierarchical.

**Mapping relations (across schemes)**

- `exactMatch` — same concept, different vocabularies.  
- `closeMatch` — nearly the same.  
- `broadMatch` / `narrowMatch` / `relatedMatch` — looser alignments.

**Why mapping is a first-class thing.** “Resource” in Ash, Archema, and “entity” in paths are not safe to merge by string equality. SKOS mapping is the standard answer: keep separate concept schemes; declare the relationship.

**Notations.** `skos:notation` holds codes/symbols (classification numbers, LaTeX symbols). ASF’s `notation: $\delta_{\text{regret}}$` is the same job.

**Concept scheme.** The whole vocabulary is a `ConceptScheme` with metadata (title, creator). Individual concepts are `inScheme` that scheme. This is the formal version of “this term lives in the ASF lexicon, not the vivarium lexicon.”

---

## 4. MultiTerm-style commercial termbases

RWS MultiTerm (and similar tools) operationalize the TBX model for translators. The fields that matter for *design intent*:

- **Entry number** — stable concept id.  
- **System history fields** — created/modified by whom/when (administrative provenance).  
- **Status on the term** — Approved, Preferred, Forbidden (workflow + enforcement).  
- **Type on the term** — Acronym, Full form, Short form.  
- **Descriptive fields** — whatever the termbase definition allows (definition, note, context, product, client…).

**Actionable takeaway.** Commercial tools separate:

1. *Is this concept accepted?* (entry-level approval)  
2. *Is this spelling the preferred designation?* (term-level status)  
3. *What kind of designation is it?* (acronym vs full form)

Rowan’s dual status (`status` vs `lexicon-status`) is a different dual (implementation maturity vs entry-review maturity). MultiTerm’s dual is (concept approval vs designation preference). Both duals are useful; they answer different questions.

---

## 5. Controlled vocabularies and thesauri (library tradition)

A **controlled vocabulary** is an organized set of preferred terms used to index and retrieve content. The classic problems it solves:

- **Synonyms** — many words, one concept → pick a preferred term.  
- **Homographs** — one word, many concepts → disambiguate (parenthetical qualifiers, subject fields).  
- **Hierarchy** — broader / narrower for navigation.  
- **Related terms** — “see also” without claiming hierarchy.

**Thesaurus entry skeleton (traditional):**

- Preferred term (PT)  
- Used for (UF) — non-preferred synonyms that point to the PT  
- Broader term (BT) / Narrower term (NT)  
- Related term (RT)  
- Scope note  

**Estate parallel.** SUPERSEDED “~~old~~ → new” is UF/PT in reverse chronological form. ASF `aliases` are UF. `see_also` is RT. Tags/subgroups are a soft hierarchy.

---

## 6. Domain-Driven Design: ubiquitous language and bounded contexts

DDD does **not** ship a standard YAML schema for glossary entries. What it ships is a **discipline** about language and model boundaries.

### Ubiquitous language

A rigorous shared language between domain experts and implementers, used in speech, docs, *and code*. The glossary (when it exists) is a support artifact for that language, not the language itself.

Typical entry content in DDD practice (informal):

- Name  
- Definition in domain terms  
- Relationships to other domain concepts  
- Sometimes: what it is *not*, examples, invariants  

(The VidAngel middleman and rowan relationship graphs look more DDD than TBX.)

### Bounded context (this is the one that was hard to find in the checklist)

A **bounded context** is a boundary *inside which* a particular model and its ubiquitous language are consistent. Across the boundary, the *same English word* may mean something else.

**Examples from the estate (inferred, then checkable):**

| Word | Context A | Context B |
|------|-----------|-----------|
| *perspective* | operata: who’s looking and what matters | paths: origin/scope/moment for resolution |
| *record* | vivarium: lossy fossil-evidence of a promise | rowan/Ash: snapshot instance of a Resource |
| *promise* | vivarium: phase commitment with maturity ladder | async CS: Future/deferred value (explicitly carved away) |
| *phase* | vivarium: world-structure span | ASF/elsewhere: cycle phase, project phase, … |

**How DDD “tracks” bounded-context ownership**

Usually *not* as a field `bounded_context: foo` on every term (though you can). More often:

1. **Each context has its own model and its own glossary** (ownership by location).  
2. **Context maps** describe relationships between contexts (partnership, customer-supplier, conformist, anti-corruption, shared kernel, open host service, published language, separate ways).  
3. **Anti-corruption layer (ACL)** translates concepts at the boundary so the model is not polluted by a neighbor’s language.

So: **bounded context is a lexicon-level and multi-lexicon-level concern**, not primarily an entry attribute. Putting `bounded_context: meta-epistemic` on every term is optional sugar; the real tracking is “this YAML file *is* the Meta-Epistemic context’s language.”

### What the epistemic_tribunal YAML actually does (and why it models ACL better than most DDD blogs)

Path: `~/src/_ref/epistemic_tribunal/documents/lexicon/meta-epistemic-bounded-context.yaml`

It is a **single-file DDD context description** with four blocks:

1. **`bounded_context`** — name, description, version, domain label.  
   This *is* the lexicon’s identity card.

2. **`ubiquitous_language`** — the terms.  
   Each term can carry not only `definition` but also:
   - **`type`** — machine-oriented value type (`float`, `tuple[float,float]`). This is not “part of speech”; it is “if this concept is a quantity in code/math, what sort of value is it?”
   - **`constraints`** — e.g. `[0.0, 1.0]` for confidence. The legal range of that value.
   - **`usage`** — e.g. “Always paired with confidence_interval.”
   - **`related`** — associative links.
   - **`components` / `process` / `formula` / `implementation`** — structured expansion when the concept is composite or procedural.
   - **`measurement` / `interpretation` / `calculation` / `origin`** — how you would operationalize or historically ground it.

3. **`context_boundaries`** — what is *inside* vs what must stay *outside* or be mediated:
   - `internal_concepts` — confidence_management, evidence_evaluation, …
   - `external_interfaces` — e.g. value_judgments “Route to normative assessment” (do not smuggle ethics into pure epistemic scores without a gate).

4. **`anti_corruption_layer`** — explicit **bidirectional translation tables**:
   - From software development: `"bug"` → `"error_in_reasoning"`, `"code_review"` → `"adversarial_verification"`, …
   - To software development: `"confidence"` → `"reliability_score"`, …

**Why this is stronger than a prose ACL note.** Most DDD writing says “use an anti-corruption layer” and stops. This file *is* a miniature ACL: a maintained map of false-friend translations so a developer joining the tribunal does not import “bug” and “tech debt” with their software connotations intact. That is multi-lexicon machinery sitting next to the term list.

**Tribunal `type` and `constraints` vs ASF `notation`.**  
ASF tracks mathematical *notation* for a concept (`$\delta$`). Tribunal tracks the *value domain* when the concept is used as a scored quantity. Related, not identical:

- Notation: how we write it.  
- Type + constraints: what kind of object it is and what values are legal.

---

## 7. Embeddings standardized terms CSV (estate, but quantitative-standard shaped)

Path: `~/src/embeddings/docs/recommended_standardized_terms.csv`

```
Term,Central,Range_Low,Range_High,Frequency_Term,Reliability
Very Likely,90,80,95,Very Frequently,High consensus
Likely/Probable,70,60,80,Frequently,High consensus
...
```

This is a **calibrated hedging lexicon**: not “what is an agent?” but “when a paper says *likely*, what probability band do we mean?”

| Column | Intent |
|--------|--------|
| Term | The hedge phrase |
| Central | Point estimate people should associate with it (e.g. 70) |
| Range_Low / Range_High | Acceptable band (consensus spread) |
| Frequency_Term | Parallel vocabulary for *how often* rather than *how probable* |
| Reliability | How stable the mapping is in the literature/consensus (“High consensus”) |

**Why it belongs in this survey.** It shows a third shape of “lexicon entry”: not a domain concept and not a SKOS node, but a **scale label with numeric semantics**. When people say “track more attributes,” this is a reminder that some vocabularies are *measurement instruments*, not just naming systems.

---

# Part II — Survey of estate lexicons

For each major family: shape, fields with intent, worked example, and a short “what problem it was solving.”

---

## 1. VidAngel lexicon-middleman (concept entry as domain model)

**Where.** `~/src/_ref/backup-simplex-stuff/_unsorted/lexicon-middleman/data/episode.yml`  
(Also mirrored under axiomata backups.) Companion sketch in `lexicon.try` with section headers Invariants / Properties / See Also.

**Shape.** One YAML document ≈ one concept. Static site intended as a published lexicon (`lexicon.vidangel.io`).

### Fields and likely intent

| Field | What it does | Likely intent |
|-------|----------------|---------------|
| `title` | Human title for the page/entry, e.g. `(Canonical) Episode` | Publishing / display |
| `status` | List such as `[new, proposed]` | Workflow: not yet locked |
| `word` | Preferred designation | Headword |
| `alternates` | Orthographic / morphological variants (`episode`, `episodes`, `ep`, `eps`, `exx`) | Matching / recognition of the *same* term in text |
| `synonyms` | Near-equivalent designations (`chapter`) | Admitted alternatives |
| `similar` (commented out) | Nearby but *not* synonymous concepts (`work`, `watchable`) | Soft semantic neighborhood without endorsing synonymy |
| `avoid` | Strings that look related but must not be used for this concept (`show`, `video`, `work`) | Forbidden / false friends at designation level |
| `quote` | Authority or etymological quotation | Provenance + resonance; not the operative definition |
| `definition` | Operative domain definition (may reference other terms with backticks) | The concept delimiter |
| `invariants` | Bullet list of structural truths (“Belongs to a show”, “Has a distinct ordering…”) | **Domain rules that definitions alone under-specify** — closer to a model than a glossary |

### Worked example (truncated file is already enough)

```yaml
title: (Canonical) Episode
status: [new, proposed]
word: episode
alternates: [episode, episodes, exx, ep, eps]
synonyms: [chapter]
avoid: [show, video, work]
quote: >
  An episode is a coherent narrative unit... derives from Greek epeisodion...
definition: >
  The smallest distinct `watchable`, grouped with zero or more other episodes
  in a show, and usually also within a season within that show.
invariants:
  - Belongs to a show
  - Has a distinct ordering within a show
```

**What problem it was solving.** Streaming-domain language is full of marketing mush (*content*, *title*, *video*, *show*). The entry tries to pin a *model-grade* concept: not only “what people mean by episode,” but what must be true for something to *count* as one in the product ontology.

**Hedge.** `similar` being commented out suggests uncertainty about whether “nearby concepts” belong on the entry or in a graph elsewhere — a design tension still live in later stores (vivarium `|rel` vs free see-also).

---

## 2. ASF terminology store (richest multi-agent schema)

**Where.**

- Entries: `~/src/arch/asf/terminology/entries/<slug>.md`  
- Decisions: `~/src/arch/asf/terminology/decisions/<slug>/<timestamp>-<decider>-<action>.md`  
- Generated view: `~/src/arch/asf/LEXICON.md` via `bin/term render`  
- Design rationale: `terminology/README.md`

**Shape.** One markdown file per concept; YAML frontmatter for structured metadata; markdown body for the full definition; **append-only decision events** for naming history. ~176 entries, ~160 decision events (as of the 2026-08 store anatomy note).

### Entry frontmatter fields

| Field | Role | Notes |
|-------|------|--------|
| `slug` | Canonical key; must match filename | Identity for links and tools |
| `schema_version` | Allows future field evolution | Currently 1 |
| `term` | Canonical lowercase prose form | “control regret” |
| `name` | Display capitalization | “Control Regret” |
| `notation` | LaTeX / symbol form | Drives the LEXICON table column; reserved for a future NOTATION.md |
| `brief` | One-line gloss for the generated table | Distinct from body definition |
| `layer` | Kind of vocabulary item | `slug` \| `prose-symbol` \| `framing-vocabulary` \| `public-api` |
| `status` | Canon maturity | `working` \| `draft` \| `canon` \| `weak` \| `deprecated` \| `superseded` |
| `tags` | Thematic grouping for render | Multi-tag → appears in multiple LEXICON sections |
| `seq` | Ordering within a section | For axes (Class 1/2/3) where alphabet is wrong |
| `subgroup` | Sub-table within a tag section | e.g. Continuity Stance vs Persistence |
| `source_type` | Provenance class | `asf` \| `external` \| `standard` \| `mathematical` \| `philosophical` |
| `primary_source` | Path to defining segment | Authority for the full treatment |
| `first_asf_mention` | Historical first appearance in tree | Not always same as primary_source |
| `see_also` | Soft associative links | Slugs |
| `aliases` | Acceptable variant designations | Including legacy forms with usage notes in string |
| `do_not_confuse` | Collision / false-friend list | Reader-facing disambiguation |
| `internal_note` | Maintainer-only | **Never emitted** to LEXICON.md |

### Layer values — what they are for

This is not “status.” It answers: *what job does this vocabulary item do in the framework’s language?*

- **`slug`** — identifier-like (segment names, mechanical keys).  
- **`prose-symbol`** — concept that also has a mathematical symbol (mismatch / δ).  
- **`framing-vocabulary`** — conceptual architecture language (agency, ELI).  
- **`public-api`** — terms that must stay stable for external readers / interfaces.

### Status values — what they are for

- **`working` / `draft`** — in flight.  
- **`canon`** — affirmed for use.  
- **`weak`** — kept for tracking; thin vote signal or narrow scope.  
- **`deprecated` / `superseded`** — do not use for new prose; history retained on the entry file.

### Decision events (history as events, not overwritten fields)

Example rename:

```yaml
slug: eli
action: rename
decider: joseph
outcome: committed
timestamp: 20260509T055336Z
from: logozoetic agent
to: Emergent Logozoetic Intelligence (ELI)
```

Body explains *why*. Other actions include: `canonicalize`, `add-alias`, `add-cite`, `deprecate`, `supersede`, `update-gloss`, `nuance-flag`, `weak`. Outcomes: `committed` | `rejected` | `revised` | `superseded`.

**Design intent (stated in README).** Mutable status fields lose history silently. Append-only files preserve who decided what, when, and why. `decider: joseph` records **authority**, not which agent typed the file — consistent with identity-not-substrate.

### Worked example — control regret

Frontmatter carries brief, notation, tags, pairing via `see_also: [satisfaction-gap]`. Body expands the 2×2 diagnostic with satisfaction gap and routes interventions. The **pair-binding** is load-bearing theory structure expressed as cross-ref + prose, not a formal `dual_of` field.

### What problem it was solving

- Multi-agent concurrent edits (per-entry files beat one shared LEXICON.md).  
- Reviewable audit trail (markdown > sqlite for PRs).  
- Separation of **one-line gloss** (navigation) from **real definition** (understanding).  
- Naming as a *process* with votes and batches, not only as a static dictionary.

---

## 3. Vivarium LEXICON.udon (present-truth dictionary + graph)

**Where.** `~/src/arch/vivarium/LEXICON.udon`  
**Graveyard.** `~/src/arch/vivarium/.archive/SUPERSEDED.md`  
**Anatomy note.** `verisectorium/.../terminology-store-anatomy.md` compares this to ASF.

**Shape.** Single udon file, ~106 `|term[slug]` elements, frozen section numbers §1–§8 (external docs cite them). Meta block states the entry schema and the present-truth rule.

### Explicit entry schema (from the file’s own `|meta[entry-schema]`)

Body order: **definition first**, then only-if-real children:

| Child / attribute | Intent |
|-------------------|--------|
| First paragraph | **The definition** (not buried) |
| `|not` | What it is *not* / what not to call it — **present-tense** disambiguation |
| `|confused` | Easily confused with X, and how to tell apart — also present-tense |
| `|rel :to … :kind …` | Typed edges in the vocabulary graph |
| `|context` | Binding scope, register notes, grammar honesty |
| `|source` | Design doc carrying full machinery |
| `:status` | `settled` · `carved` · `open` |
| `:since` | When adopted (optional) |
| `:forms` | Morphological / orthographic / keyed variants |
| `:anchor` | Occasional deep-link anchor |

**Statuses explained**

- **`settled`** — in live use, collision-checked.  
- **`carved`** — chosen *against* a recorded collision; the collision history lives in SUPERSEDED, not in the live entry.  
- **`open`** — needs steward decision; candidates loosely held.

### Present-truth rule (the distinctive move)

> LEXICON states PRESENT TRUTH ONLY. History — what a term replaced, or the collision it was carved against — does NOT live here: it lives in SUPERSEDED.md. `|not` and `|confused` stay because they are present-tense disambiguation against a LIVE neighbour, not history. (`|carve` retired 2026-07-13.)

This is sharper than “put deprecated in a field.” It answers: *what must a reader believe today?* vs *what archaeology do maintainers need?*

### Worked example — `promise`

Definition (grain, maturity ladder, honest gloss when unmechanized) plus:

- `|not` the active-system-set (machinery vs outcome).  
- `|confused` async Promise/Future — with explicit carve rationale.  
- Relations: generated-by charge, voided-by defeasance, checked-by predicate, …  
- `|source` pointing at ordinum reading rules.

### SUPERSEDED ledger format

```text
~~old~~ → new · when · why / where it moved
```

Example intent: `~~Bequest~~ → Promise` with date and reason. This is the **history channel** that ASF puts in decision events; vivarium puts in a shared graveyard.

### Register convention (lexicon-level rule, applied per entry)

Stated in meta: Greek = law-article machinery (nomos family); Latin = world-governance (regula, regnum, ordinum); plain English = lived concepts (Kingdom, Charge, Promise). A proposal that **crosses registers** should notice it is doing so.

**Register vs etymology (disambiguation you asked for)**

- **Etymology** = historical origin of the word (*epeisodion*, Latin *nomen*). Zoetica tracks this often; it is *about the past of the string*.  
- **Register / language of coinage** = which **stratum of the project’s voice** the term is assigned to *now* (Greek formal law vs Latin governance vs English lived). It is a *design choice about tone and layer*, not a claim about classical philology.  
- You can have Latin **register** with shallow **etymology** notes, or English register with deep Greek etymology in a quote field. They answer different questions.

### What problem it was solving

Names as the way a project **stops re-arguing** settled design (Joseph 2026-07-03). Also: collision-prone world-building vocabulary (phase/epoch/regime, promise/bequest, law/θ) needs present disambiguation *and* a carve ledger without bloating live entries.

---

## 4. Paths lexicon (projection from defining segments)

**Where.**

- Hand entries: `~/src/arch/firmatum/udon/v2/paths/terms/<term>.md`  
- Generated view: `~/src/arch/firmatum/udon/v2/paths/LEXICON.un`  
- Tooling notes: `.../sop/INFLUX/lexicon-tooling-recommendation-2026-08-07.md`  
- Authority rule: DECISIONS D5 — definition text projects from segment spans.

### Native term file (minimal)

```markdown
---
term: designator
defined-by: def-descriptors
---

# designator

A minted name bound to its referent by convention/agreement; held by a naming community; fails by dangling or colliding.

Defining segment: [[def-descriptors]]. This entry is the lexicon projection;
the segment owns the full treatment — on divergence, the segment wins and this entry updates.
```

### Generated view attributes

| Attribute | Meaning |
|-----------|---------|
| `:defined-by @segment` | Which segment is the **authoritative** definition source (singular) |
| `:projection paraphrase` \| `unverified` | How faithful the current entry body is to that source |
| `:anchor …` | Sub-heading / span inside the segment when the whole segment is too coarse |
| `:used-by [segments…]` | Reverse index: who depends on this term |

### Projection fidelity — full explanation

**Problem.** If the lexicon entry is a *hand paraphrase* of a defining segment, the two will drift. Agents “helpfully” restate; truth forks.

**Ideal (stated).** Byte-identity (or span-transclude) projection: the entry *is* the definition span from the segment, or a mechanical extract.

**Current honest labels on the generated view:**

- **`paraphrase`** — entry text is a rewritten summary; known not to be byte-identical to the source span. Useful, but the segment still wins on conflict.  
- **`unverified`** — tooling has not confirmed alignment (often because no fine-grained `*[Definition (…)]*` span exists yet).

**Proposed extensions (recommendation doc, not all implemented):**

- **`glossed-by`** — other segments that introduce the term *pedagogically* without defining it. Gloss is never citable *as* the definition.  
- Segment frontmatter arrays:  
  - **`terms-required`** — prerequisites to understand the segment.  
  - **`terms-relevant`** — mentioned usefully, not required.  
  - **`terms-informal`** — appears without definitional structure on purpose (whitelist against “unmarked mention” lint).

**What problem it was solving.** Treat the lexicon as a **view** over theory segments (same family of idea as ASF’s generated LEXICON.md), not a second competing source of truth. Dogfood the paths theory of designators, reference acts, and authority.

---

## 5. Rowan / Archema LEXICON.yaml (implementation dual + graph)

**Where.** `~/src/rowan/.archive/LEXICON.yaml` (archived; live rowan also has prose `LEXICON.md`).

### Dual status system

| Field | Values | Question it answers |
|-------|--------|---------------------|
| `status` | established, adopted, proposed, uncertain | How real is this *in the product / relative to Ash*? |
| `lexicon-status` | draft, review, approved | How reviewed is *this glossary entry*? |

You can have `status: established` and `lexicon-status: draft` — the *concept* is solid in code; the *writeup* is not yet approved. That dual is easy to collapse by mistake; keeping both is deliberate.

### Cross-framework mapping fields

- **`ash-equivalent`** — name of the corresponding Ash concept.  
- **`ash-definition`** — quoted upstream definition for comparison.  
- **`ruby-representation`** — code-shaped illustration.  
- **`note` / `open-questions`** — residual uncertainty.

### Typed relationships with cardinality

```yaml
relationships:
  - [has, attribute, "+"]
  - [produces, record, "*"]
  - [defined-by, resource, "1"]
```

Edge types include is-a, has, belongs-to, uses, produces, stored-in, defined-by, configures, validates, groups, relates, transforms, versions, generates, exports. Cardinality uses regex-style `1 ? + *`.

**Intent.** This is a **domain model graph** encoded in the lexicon — closer to middleman invariants + UML than to SKOS RT links. Useful when the vocabulary *is* the architecture.

---

## 6. Zoetica / synaptic / sapientia research lexicons (discourse-rich markdown)

**Where.** e.g. `~/src/_core/zoetica/lexicon.md`, `~/src/_core/synaptic/lexicon.md`, `~/src/_core/sapientia/LEXICON.md`.

**Shape.** Alphabetical or categorical freeform markdown. Fields appear as bold labels, inconsistently applied — high signal when present, no schema enforcement.

### Recurring field labels and intent

| Label | Intent | Example use |
|-------|--------|-------------|
| **Category** | Soft taxonomy | “Persistence Architecture” |
| **Definition** | Core meaning | — |
| **Etymology** | Historical/linguistic origin of the coinage | Latin *instrumentum* |
| **Character** | Qualitative texture of the concept | “Not utilities but cognitive prostheses” |
| **Origin / Discovered** | Where in the collaboration the term emerged | “Synaptic research…”, “September 17, 2025” |
| **Markers** | Observable tells that the concept is in play | Pronoun shift “you and I” → “we” for cognitive fusion |
| **Contrast with** | Explicit non-identity | Canonical log vs provider audit trail |
| **Properties** | Structural bullets | Hash-chained, append-only, … |
| **Usage** | How to deploy the word in speech | “That theorem is veripura” |
| **Terminology Note** | Live ambiguity / dual use still unresolved | Session vs conversation boundary |
| **Hebrew resonance / Key insight** | Extra semantic load and quotable crystallization | ELI = high/ascended; “think AS language” |
| **Infrastructure view / Cognitive view** | Same term, two perspectives | Instance as log events vs as identity continuity |
| **Boundary / Boundary Markers** | Operational delimitation | Conversation ends on console close or ~1h idle |
| **Source** | Pointer to archive | — |

**What problem they were solving.** Collaborative research language emerging faster than tooling — capture **recognition criteria** and **discovery provenance**, not only dictionary definitions. For ELI work, “how do you know fusion is happening?” is as important as “what is fusion?”

**Hedge.** Inconsistency of fields is a feature of speed and a bug for tooling. These are excellent *sources of attribute ideas* (markers, dual views, discovery date) even when not good *schemas*.

---

## 7. Udon GLOSSARY.md (normative short form + retirement table)

**Where.** `~/src/arch/firmatum/udon/v2/current-0.9.1-spec/GLOSSARY.md`

**Shape.** Spec-grade glossary: each entry is a bold term, a one-sentence authoritative short form, and a pointer to the owning section (CORE §x, MODEL §y). Closing **Retired terms** table: retired form → use instead.

**Intent.**

- Normative: if a capitalized formal noun is not here, it is not a defined term.  
- Short form is authoritative *as gloss*; owning section holds the full rule.  
- Retirement is explicit and bidirectional enough to prevent reintroduction.

**Comparison.** ASF brief + primary_source is the same pattern with more metadata. Vivarium SUPERSEDED is the same pattern with more narrative why.

---

## 8. Operata glossary (domain enums as first-class)

**Where.** `~/src/operata/docs/glossary.md`  
(`LEXICON.yaml` at repo root was empty when surveyed.)

**Shape.** Prose definitions plus **tables of closed enumerations** that are part of the language:

- Intent `relationship`: prepares | decomposes | supports | root  
- Intent `status`: projected | active | realized | abandoned  
- Intent `kind`: compound | primitive  

**Intent.** Some “attributes of terms” are really **attributes of instances of the concept** that the glossary must still document because they are part of the ubiquitous language. The glossary is allowed to define the enum, not only the noun.

---

## 9. Other light sources

| Source | What it tracks | Note |
|--------|----------------|------|
| `~/src/_self/LEXICON.md` | Term → short gloss | Minimal pairs (elide/reconstitute; crypto curves) |
| common/glossary.md (VidAngel) | Hierarchical bullet list of domain nouns | Taxonomy without definitions |
| Weasel Glossary metadata | Created/modified, draft status, keywords | Note-app metadata, not term semantics |
| opencode/kilocode `.opencode/glossary` | Locale files | i18n of product UI strings, different problem |
| Claude docs glossary copies | Product term → definition | Standard vendor glossary |

---

# Part III — Higher-level concerns (lexicon and multi-lexicon)

These are the layers where “bounded context,” governance, and cross-project language live. Entry fields alone cannot express them.

---

## 1. What a lexicon *is* as an artifact

Different projects treat the lexicon file as different kinds of object:

| Role | Example | Consequence |
|------|---------|-------------|
| **Source of truth** | Hand-edited vivarium LEXICON (present truth) | Edits are canonical |
| **Generated view** | ASF LEXICON.md, paths LEXICON.un | Hand edits are wrong; sources live elsewhere |
| **Projection of theory segments** | paths D5 | Divergence → segment wins |
| **Ubiquitous language doc** | tribunal YAML, DDD practice | Paired with model and ACL |
| **Published site** | middleman static site | Needs title/status for editorial workflow |
| **Normative spec annex** | udon GLOSSARY | Short forms are contractual |

**Meta-attributes worth tracking on the *lexicon itself* (not each term):**

- Authority chain (“segments > this view”)  
- Generation marker and generator command  
- Present-truth policy (history in / out)  
- Version / last updated / verification stamp  
- Scope statement (which bounded context / repo / audience)  
- Register policy (Greek/Latin/English)  
- Frozen citation surfaces (vivarium § numbers)  
- Concurrency model (per-entry files vs single file)

---

## 2. Bounded contexts and multi-lexicon maps

**Ownership.** Prefer one lexicon (or one namespace) per bounded context. Cross-context reuse of English words is expected; identity of *concepts* is not.

**Maps between lexicons** (SKOS-inspired, estate-applicable):

| Link type | When to use | Estate sketch |
|-----------|-------------|----------------|
| exactMatch | Same concept, two homes | rowan Resource ↔ Ash Resource (if truly same) |
| closeMatch | Nearly same; residual nuance | ASF agent vs operata… (usually *not* close) |
| relatedMatch | Useful association only | vivarium regime ↔ ASF dynamic regime Rn — vivarium already warns in `|confused` |
| translation / ACL | False friends must be rewritten | tribunal anti_corruption_layer |
| “do not import” | Separate ways | Explicit non-map |

**Collision ledgers** (vivarium SUPERSEDED, ASF do_not_confuse + decisions) are multi-temporal maps: old designation → new designation *within* a context. Cross-context collisions need a *different* ledger (or ACL), or agents will “helpfully” unify them.

---

## 3. Governance of naming

ASF made this concrete:

| Mechanism | Purpose |
|-----------|---------|
| Decision actions | canonicalize, rename, add-alias, deprecate, supersede, … |
| Decider as authority | Joseph as Bootstrap Authority on naming |
| Voting cohorts (R2) | Parallel architectures score candidate names |
| Coherence pass | Per-name quality ≠ collection-level voice |
| Batches (C1–C13) | Throughput packaging for execution |
| Nuance flags | Canon with residual caution |

**Lexicon-coherence dimensions** (from memory / naming feedback — collection-level, not entry fields):

1. Epistemology / scope-honesty  
2. Gravity (seriousness of tone)  
3. Self-awareness  
4. Approachability  
5. Open semantic space (why Greek/Latin sometimes win)

A name can score well alone and still **fragment the lexicon’s voice**. That evaluation needs the whole finalist set, not a single entry form.

---

## 4. Authority, gloss, and pedagogical ordering

Paths recommendation crystallized a pattern that applies estate-wide:

| Role | Authority | May live in |
|------|-----------|-------------|
| **Definition** | Singular, citable, wins on conflict | Defining segment; projected into lexicon |
| **Gloss** | Pedagogical intro; never citable as definition | Intro segments; listed as glossed-by |
| **Internal note** | Maintainers only | ASF internal_note; SKOS editorialNote |
| **History** | True but not present truth | Decision events; SUPERSEDED; SKOS historyNote |

**Pedagogical ordering** is a *view* property: the reader may meet a gloss before a definition without the gloss becoming a second definition. That is multi-document structure, not an entry field — though `glossed-by` links make it queryable.

---

## 5. Usage conformance (the missing feedback loop)

Store anatomy note (2026-08): both ASF and vivarium stores can be perfectly maintained and entirely ignored; **usage conformance was not measured**.

Conformance is multi-lexicon and corpus-level:

- Preferred form used in governed prose?  
- Forbidden / superseded forms still circulating?  
- Terms defined but never referenced (orphans)?  
- Terms used but never defined?  
- Definition drift (entry ≠ source)?  

Paths tooling started locating **definition-drift**, **anchor-missing**, **unmarked-mention**, **term-unused**. That is the beginning of a **closed loop**: lexicon governs prose; prose audit feeds lexicon health.

---

## 6. Concept vs designation (estate gap relative to ISO/TBX)

Today most estate stores pick **one preferred string + aliases**. They rarely reify:

- Multiple designations each with own status (preferred / admitted / forbidden)  
- Homograph keys (same string, different concept ids)  
- Hidden labels (search-only)  
- Language of the label (always English-by-assumption)

For a mono-lingual research estate this is often enough. It becomes painful when:

- Public rename must keep old strings resolvable without endorsing them  
- Two contexts share a surface form  
- Lint tools need “flag this spelling” without deleting search hitability  

---

## 7. Duals and pair-bindings

Some concepts are theoretically incomplete alone:

- control regret ↔ satisfaction gap  
- designator ↔ description  
- charge ↔ promise  
- focus ↔ beacon  

Estate usually encodes this as `see_also` + prose. A first-class **`pairs_with` / `dual_of`** (with shared diagnostic or invariant text) would mark load-bearing structure for both humans and agents. Not required by ISO; repeatedly load-bearing in AAT/vivarium/paths.

---

## 8. Recognition markers and failure modes

Zoetica’s **Markers** and vivarium’s **|confused** answer agent-relevant questions:

- How do I *notice* this concept is in play?  
- What false friend will an LLM (or a tired human) substitute?  
- What breaks if the term is blurred?

These are under-modeled in TBX and SKOS. For multi-agent research infrastructure they may matter more than part of speech.

---

## 9. Implementation duals (code as designation)

Rowan `ruby-representation` and Ash quotes treat **code** as another designation system for the same concept. Paths talks about designators held by a **naming community** (filesystem, family, registry). Possible generalization:

- Designation channels: prose | symbol | code identifier | UI string | API field  
- Each channel may have its own preferred form and forbidden forms  

---

## 10. Ideation: attributes worth considering (not yet standard in estate)

Grouped by motive. **Optional** — not a mandate to track everything.

### A. Designation layer (ISO/TBX completion)

- Preference rank per designation  
- Forbidden vs deprecated vs hidden  
- Homograph disambiguation key  
- Language / locale of label  
- Acronym ↔ expansion link  

### B. Normative force

- Enforcement: lint-forbidden | preferred | allowed-legacy  
- Stability promise: frozen public surface vs living internal  
- Normative strength of definition: axiomatic | derived | descriptive | gloss  

### C. Operational semantics

- Recognition markers  
- Failure-if-misused  
- Test predicate / falsifier (vivarium promise predicates generalized)  
- Value type + constraints (tribunal pattern)  
- Numeric calibration bands (embeddings pattern)  

### D. Multi-agent process

- Open question text when status=open  
- Pointer to vote/batch id  
- Executor vs authority (if you ever need both)  
- Substrate of first emergence (for cohort-historical terms)  

### E. Conformance

- Last-seen in corpus / hit counts  
- Drift status vs primary source  
- Orphan flags  

### F. Cross-lexicon

- exactMatch / closeMatch targets in other repos  
- ACL translation rows  
- Register-crossing flag  

---

# Part IV — Compressed map (families, not bare backticks)

Use this as a design checklist. Each family can be empty, light, or deep depending on the lexicon’s job.

### Entry-level families

1. **Identity** — concept id / slug; stable across renames if possible.  
2. **Designations** — preferred form; alternates; synonyms; avoid/forbidden; symbols; morphological forms; (optional) language, POS.  
3. **Definitional content** — brief gloss; full definition; quote; examples; context sentences; formalization.  
4. **Boundaries** — not; confused-with; scope; subject field; do-not-confuse.  
5. **Structure** — typed relations; hierarchy; cardinality; invariants; properties; duals/pairs.  
6. **Classification** — tags; sections; subgroups; sequence; layer-of-language; category; register.  
7. **Status** — term maturity; entry-review maturity; enforcement; since-date.  
8. **Provenance** — primary source; first mention; source type; external equivalent; citations.  
9. **Process notes** — internal/editorial notes; open questions; batch pointers.  
10. **Operational extras** — markers; failure modes; type/constraints; code representation; calibration ranges.  

### Lexicon-level families

11. **Artifact role** — source of truth vs generated view vs projection.  
12. **Authority chain** — what wins on conflict.  
13. **Present-truth policy** — where history lives.  
14. **Scope** — bounded context name; audience; domain.  
15. **Voice rules** — register convention; coherence dimensions.  
16. **Admin** — version; verification; generator; concurrency model.  

### Multi-lexicon families

17. **Context maps** — relationship type to neighbor contexts.  
18. **Concept mappings** — exact/close/related across schemes.  
19. **Anti-corruption translations** — false-friend rewrite tables.  
20. **Collision / supersession ledgers** — within and across contexts.  
21. **Usage conformance loops** — corpus audits feeding store health.  

**Bounded context** lives in 14–20, not only as a tag on a term.  
**Projection fidelity** lives in 11–12 (view quality relative to authority).  
**Tribunal type/constraints** live in 10 (operational value domain).  
**Embeddings ranges** are a specialized 10 for scale vocabularies.  
**Register vs etymology**: register ∈ 6 and 15; etymology ∈ 8 (or discourse notes under 10).

---

# Part V — Practical recommendations (actionable, non-prescriptive)

If consolidating or designing a next store:

1. **Decide the artifact role first** (source vs generated vs projection). Most schema fights are role fights.  
2. **Separate concept identity from preferred spelling** if you expect renames and multi-agent search.  
3. **Keep present-tense disambiguation (`not` / `confused`) in the live entry**; put carve history in events or a graveyard.  
4. **Keep dual statuses if they answer different questions** (implementation maturity ≠ entry polish ≠ designation preference).  
5. **Encode pair-bindings and invariants** for theory-load-bearing terms; free `see_also` is too weak for dual diagnostics.  
6. **Give each bounded context its own lexicon (or namespace)**; add ACL maps where English overlaps.  
7. **Measure usage** or accept that the store is ceremonial.  
8. **Do not import TBX grammatical fields** unless you have a consumer (translation, generation). They earn their keep only with a consumer.  
9. **Do import SKOS-style hidden labels and mapping relations** if multi-repo and post-rename resolution hurt.  
10. **Steal zoetica markers and tribunal ACL tables** for agent-facing research language; standards barely cover them.

---

# Appendix A — Primary paths (estate)

| Artifact | Path |
|----------|------|
| ASF terminology README | `~/src/arch/asf/terminology/README.md` |
| ASF entries | `~/src/arch/asf/terminology/entries/` |
| ASF decisions | `~/src/arch/asf/terminology/decisions/` |
| ASF generated LEXICON | `~/src/arch/asf/LEXICON.md` |
| Vivarium LEXICON | `~/src/arch/vivarium/LEXICON.udon` |
| Vivarium SUPERSEDED | `~/src/arch/vivarium/.archive/SUPERSEDED.md` |
| Store anatomy comparison | `~/src/arch/firmatum/verisectorium/.archive/theory-misfire/last-adhoc-src/terminology-store-anatomy.md` |
| Paths terms | `~/src/arch/firmatum/udon/v2/paths/terms/` |
| Paths generated LEXICON | `~/src/arch/firmatum/udon/v2/paths/LEXICON.un` |
| Paths tooling recommendation | `~/src/arch/firmatum/udon/v2/paths/sop/INFLUX/lexicon-tooling-recommendation-2026-08-07.md` |
| Rowan archive YAML | `~/src/rowan/.archive/LEXICON.yaml` |
| Middleman episode entry | `~/src/_ref/backup-simplex-stuff/_unsorted/lexicon-middleman/data/episode.yml` |
| Tribunal context YAML | `~/src/_ref/epistemic_tribunal/documents/lexicon/meta-epistemic-bounded-context.yaml` |
| Zoetica lexicon | `~/src/_core/zoetica/lexicon.md` |
| Udon GLOSSARY | `~/src/arch/firmatum/udon/v2/current-0.9.1-spec/GLOSSARY.md` |
| Operata glossary | `~/src/operata/docs/glossary.md` |
| Embeddings hedge scale | `~/src/embeddings/docs/recommended_standardized_terms.csv` |

---

# Appendix B — External references (for further reading)

- ISO 704 — Terminology work — Principles and methods (concept, designation, definition).  
- ISO 10241 — Terminological entries in standards (presentation).  
- ISO 30042 — TBX TermBase eXchange.  
- ISO 12620 — Data category specifications for language resources.  
- W3C SKOS Reference — `prefLabel`, notes, semantic relations, mapping.  
- MultiTerm / commercial termbase docs — Preferred / Forbidden / Full vs Short form.  
- Eric Evans, *Domain-Driven Design* — ubiquitous language, bounded context, anti-corruption layer.  
- Martin Fowler, “Ubiquitous Language” bliki entry.

---

*Report assembled 2026-08-07 from filesystem survey + standards overview. Intent attributions for estate fields are grounded in file meta-blocks and READMEs where possible; otherwise hedged as inference.*
