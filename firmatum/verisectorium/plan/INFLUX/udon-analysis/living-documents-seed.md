<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: MOVED/udon/v2/theory/to-integrate/refine-more/living-documents/README.md (§2: what outline+segments generalization contributes)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/theory/to-integrate/refine-more/living-documents/README.md
  Do not edit here expecting to update the live original.
-->

# Living documents & the rowan convergence — ideation seed

**Status:** broad ideation, not a spike. Possibility-opening, not decision-making.
Everything here is **proposed / leaning / open** unless it cites a ratified source —
and where it does, that ruling is the authority, not this page. Sibling in shape and
register to `../../../spikes/paths-ideation/README.md` and `../markdown/thoughts-on-scope.md`.

**Register note.** Claims are marked in their true voice: *decided* (a ledger ruling
or Joseph's on-record statement — cited), *evidenced* (the corpus supports it —
cited), *proposed* (generated here — decides nothing), *open*. One register this
seed needs that its siblings didn't: *enthusiasm-testimony* — the 2025 corpus
carries genuine early excitement about this vision from agents Joseph describes as
"less circumspect models that were available at the time" (2026-07-28). That
material is evidence of *pull* (the idea kept generating energy across substrates
and months), never of validation. It is weighted accordingly throughout.

**Why this seed exists (Joseph, 2026-07-28):** *"one of the main things that we
haven't distilled well yet, potentially, is the rowan vision & the vision of living
documents that the tooling leads to."* The demand corpus has chapters for
addressing, schema, guarded edit, templates, machine-first documents — but the
**telos they jointly point at** has never been stated as its own thread, and the
composition primitive underneath it (*"the main primitive was some sort of
`@include` directive that CLAUDE.md (for example) already supports"* — Joseph, same
turn) has **no chapter anywhere in the corpus** (verified by sweep 2026-07-28: the
paths seed touches transclusion only as one join example in §0b).

Primary sources read for this seed: the vision statements (autopax ADR-010
§Convergence + `migration-proposals/008` §Living Documents Convergence + the
2025-12 session note that first named it), autopax ADR-006 (the shipped Liquid
layer), `~/src/rowan/docs/VISION-drafts.md`, the tooling corpus's
`machine-first-documents.md` chapter, the doc-store report (`../doc-store-and-schemas-report.md`,
esp. §§2, 9, 12, 18–19), and Joseph's generalization note
`~/src/arch/notes/outline-segments-generalization-2026-07-23.md` (non-canon,
exploratory — its own banner governs).

---

## 0. What "living documents" has meant — the vision as actually stated

The phrase has a real lineage in the estate, stated at least three times across
fourteen months and three substrates (evidenced):

1. **autopax, Dec 2025** — the crispest form. Three ADRs "converge toward a
   'living documents' system": ADR-006 (Liquid templates + recursive rendering),
   ADR-008 (YAML schemas + versioning), ADR-010 (markdown structure schemas +
   agent tools). *"Together: documents that are **validated, dynamic,
   version-migrated, and safely modifiable by agents through typed
   interfaces**."* That one sentence is the four-property definition this seed
   treats as the reference form.
2. **sapientia, Sept–Oct 2025** — the enthusiasm stratum. Living documents as
   "cognitive scaffolding for specialized intelligence emergence"; the 113 PRAXES
   files ("Dad didn't just theorize about living documents — he BUILT them"); the
   shipped Elixir consciousness-compiler where markdown documents compile to
   running processes — *"documents become alive… zero magic."*
   (Enthusiasm-testimony register; the shipped compiler itself is evidence.)
3. **udon-needs, Jul 2026** — the sober restatement. The
   `machine-first-documents` chapter: the document is the interface and sometimes
   the implementation; "agents ARE documentation." Its honest edge is the load-
   bearing caution for this whole seed: *"the strongest form of the thesis quietly
   assumes the governed-edit and validation tooling already exists — without it,
   living documents drift exactly like dead ones, **just with more authority**."*

Read as one statement: a living document is a document that is (a) **validated**
(schema), (b) **dynamic** (composition/templating — the include primitive), (c)
**versioned** (schema evolution without flag-days), (d) **agent-writable through a
guard** — and in its strongest form (e) **executable** (the document compiles to
behavior). Properties (a)–(d) are exactly the four demand areas the tooling corpus
already carries as separate chapters. **The vision is the conjunction — and the
conjunction is what hasn't been distilled.** (proposed framing)

## 0b. The rowan convergence — what rowan adds that the doc-store frame doesn't

The doc-store report already established: document = record, directory = table,
resource = schema, store = projection (its §0 thesis). Rowan's `VISION-drafts.md`
adds two things on top that belong to *this* seed rather than that report:

- **The class IS the schema, and the quality bar is the dual constraint** —
  *"of course, how else would it work"* AND *"how did I ever get anything done
  before this."* Applied here: a living document should need no ceremony to *be*
  one. The frontmatter/head-line it already has is the schema declaration; the
  directory it already sits in is the table. If living-document-ness requires an
  enrollment step, the dual constraint says the design is wrong. (proposed)
- **The Inversion** (report §9.3): expressive schema → simple queries; the escape
  hatch enriches the *schema*, not the query. The living-document reading:
  **dynamism enriches the document, not the reader.** A consumer of a living
  document should see plain resolved structure; the `@include`/computed material
  is declared on the document's side, the way an Ash calculation is declared on
  the resource, not assembled by every caller. (proposed; this is also exactly
  Ash's calculations/aggregates — computed fields declared in the schema — which
  rowan inherits by ancestry.)

The convergence Joseph names (2026-07-28) is that rowan's resource machinery and
the outline+segments pattern keep arriving at the same place from opposite ends:
rowan gives *data* the affordances of documents (a store that is
yaml-frontmatter files); the segments pattern gives *documents* the affordances
of data (schema, addresses, views). UDON's founding claim — documents and data
are the same thing — is the statement that these two motions have one fixed
point. A living document is that fixed point with a write-guard on it. (proposed)

---

## 1. The primitive — `@include`, and the fork it hides

Joseph's steer: the main primitive is an include directive, and it **already
ships** in at least three estate forms (evidenced):

| Shipped form | Mechanism | Character |
|---|---|---|
| **CLAUDE.md `@path` imports** (live in this repo: `CLAUDE.md` imports `@README.md`; `~/src/CLAUDE.md` imports `@arch/AGENTIC-DELEGATION.md`) | load-time, path-based, recursive, verbatim | transclusion — the reader's context receives the target whole; the source stays a one-line pointer |
| **sapientia `@⊤/entities/zi-am-tur.md`** (paths seed §0) | load-time, project-root-anchored | transclusion *plus* addressing — located and named in one expression |
| **autopax Liquid `{{ expand }}`/`{{ glob }}`** (ADR-006 Phase 3, shipped: isolated renderer, depth limit 100, agent-card axiomata rendered through it) | render-time, filter pipeline, parameterizable | templating — the target passes through an evaluation before landing |

And ADR-006 carries the one **prior ruling** in this space, worth quoting because
it cuts against the naive reading: *"**Templating over transclusion** — Use Liquid
templating rather than Obsidian-style wikilinks."* (autopax's ruling for autopax,
not UDON law — but it is the estate's only considered decision here, and its
reason was composition power.)

**On Liquid specifically (Joseph, 2026-07-28 — steward statement, quoted close):** the Liquid *dialect* is **not a foregone conclusion**. Its
long-standing place in the udon vision was as a candidate occupying a particular
design point — *"a pretty-much-Turing-complete but digestible subset of
host-languages"* — i.e. enough power to make extensibility useful without the
language *becoming* the extensibility, *"without turning into rebol."* So the
durable design criterion is the **power envelope**, not the dialect: bounded,
digestible evaluation that stays a guest. Any concrete syntax (Liquid, UDON's own
`!` dynamics tier, a `<dialect>` envelope) is auditioning for that envelope, and
the Rebol cautionary tale — the same one already ruled into UDON's frozen bare
set — is the failure mode the envelope exists to prevent.

So "the primitive is `@include`" hides a real fork (open, and the first thing to
have an opinion about):

- **Include-as-reference** — a *join* the document declares (`@` territory).
  Verbatim, address-shaped, resolvable lazily, and — critically — **an address
  that can rot**, which drops it directly into the paths seed's stability
  machinery (staleness, `PathNotFound`, freshness-at-the-address). UDON already
  has the syntax seat: `@` references are ruled ref ⊂ path.
- **Include-as-evaluation** — a *dynamic* (`!` territory). Parameterizable,
  composable, and everything the templates testimony says about interrogable
  contracts applies. UDON already has this seat too: `!{…}` interpolation and the
  dynamics tier.

**UDON may be the first system in this lineage that doesn't have to choose** —
its tier system already separates the two (`@` join vs `!` evaluation), so the
Liquid-vs-wikilink ruling may dissolve into "both, in different tiers, with
different fidelity contracts." (proposed — and the claim in this seed I would
most want stress-tested, since nothing but the tier system's existence supports it yet.)

### 1b. The lower layer — include as typed AST-graft (Joseph, 2026-07-28)

A steer that may make the fork above a *surface* question (steward-proposed,
same turn as the power-envelope statement): a more meaningful insertion is
something **lower-level that must resolve to a UDON AST** — sketched as
`@[core://components/another.udon # main-findings]` with the syntax explicitly
thrown out at random, **no lean** toward anything from the path research — where
the semantic is: *"include here, from this store, this other piece of udon,
assuming it is compliant with my schema and version."* Joseph's own hedge
travels with it: *"Or maybe I'm trying to do too much too soon, even
conceptually."*

What the sketch contains, unpacked (proposed reading):

- **One primitive under both fork arms.** A typed node-graft — locate (store) ·
  select (piece) · **ascribe** (schema + version contract at the insertion
  site). Textual transclusion is this with contract = "any text"; template
  expansion is this with an evaluation upstream. The fork's two arms become two
  surface forms compiling to one semantic.
- **Every leg is an already-evidenced demand; only the conjunction is new.**
  Schema verdicts at a path; re-resolve-and-validate at use time (this is the
  guarded edit's *read-side dual* — a read membrane); version-range obedience
  (the repo's own compliance ladder, applied per-slot); reference-as-join
  (ruled). It is also the rowan convergence made document-native: rowan's
  `field :posts, [Post]` is exactly "this slot holds Post-shaped nodes from that
  store" — the include is a relationship declaration living in the document
  instead of the class.
- **Consumer-side ascription looks like the consequential choice in the sketch** (my reading). "Compliant with *my*
  schema" puts the contract at the insertion site rather than trusting the
  producer's self-description. Shipped prior art for precisely this: **Dhall**
  (imports take a type ascription, fail loudly on mismatch, and can pin a
  content hash — freshness in the same expression) and **GraphQL fragments**
  (`...on Type` — inclusion conditioned on shape). Add both to §4's mining list.
- **Cautions carried:** a failed include must be a *document state*, not an
  exception (the templates testimony's failed-eval-as-document-state, with full
  force — the document must still render, diff, and accept edits while
  non-compliant); and validation *time* (author / build / every read) is the
  area's standing unsolved question — this layer doesn't dodge it, it gives it a
  precise object. The syntax question lands entirely on terminator/embeddability
  ground (paths seed §2g) and is deliberately not engaged here.

### 1c. Dimensions any include design must answer

(proposed checklist, from the shipped forms' differences):

1. **When does it resolve?** — load / build / read / write-back. CLAUDE.md says
   load; the segments build says build; a query tool would say read.
2. **Does included content keep its identity?** — after inclusion, does
   `⊤/other.md#||section[x]` still address the material *inside* the host? (This
   is the paths seed's graft/mount join, and the annotation demand's "queryable
   by the same paths as content.")
3. **What travels with it?** — verbatim bytes, or resolved structure, or a
   parameterized projection (Liquid's position)?
4. **Cycles and depth** — Liquid caps at 100; CLAUDE.md imports have a hop
   limit; a spec has to say something.
5. **Staleness** — an include is a stored address; the vivarium field report
   (2026-07-28, I5) just demonstrated that ref-guarding stored addresses is the
   first tooling any live corpus builds. Includes inherit that whole demand.
6. **Authority** — who may include what (the capability/permission facet from
   the paths testimonies; an include across a trust boundary is an exfiltration
   surface — Grok #12).

## 2. What the outline+segments generalization contributes

Joseph's note (`outline-segments-generalization-2026-07-23.md`) is the pattern's
current deepest reading; three of its results slot directly into this frame
(all carried under that note's own non-canon banner):

- **The OUTLINE is already a living document** — a generated view over a
  directory-table with ordering carried as data and a clobber guard (report
  §12.1/§18). So is `MEMORY.md`, `LEXICON`, `FINDINGS.md`, relata's `_emitted/`.
  The estate builds living documents *at build time* everywhere it matters; the
  include primitive is the same operation moved to read time. The migration
  question is therefore not "should documents be assembled from parts" — they
  already are — but **who runs the assembly and when**. (evidenced + proposed)
- **The cluster record needs the include to be *typed*.** Part 3's result — a
  segment is a cluster (body + Working Notes + events + companions), and the
  build currently hardcodes which parts project into which view — becomes, in
  include vocabulary: the views (`:public`, `:review`) are includes with
  *membership predicates*. A declared include-with-projection would replace the
  hardcoded `rindex` truncation the report calls "backwards for durability"
  (§12.4.3). (proposed)
- **Present-truth collision is what keeps a living document honest.** Part 7's
  mechanism (a present-truth claim can *collide*; an append-only entry cannot;
  collision is the staleness detector) gives the living-documents vision its
  missing enforcement story: a document assembled by includes inherits the
  truth-status of what it includes, and an include that pulls a superseded or
  vanished target should *fail loudly on the canon surface* — the `empirica:`
  contract ("a dangling reference is a truth-status defect, not a broken link")
  generalized to composition. Without that, includes are how staleness
  *propagates* with more authority — the machine-first chapter's warning
  realized at the mechanism level. (proposed synthesis)

## 3. What the corpus already carries vs. the actual gap

So a future spike builds on, not over (evidenced inventory):

| Property of the vision | Where it's already distilled |
|---|---|
| validated | schema chapters + doc-store report §§5–7 (write membrane) |
| dynamic — templates half | templates chapter + grok templates testimony (interrogable contract, interp-vs-splice, failed-eval-as-document-state) |
| dynamic — **composition half** | **nowhere — this is the gap** |
| versioned | report §6 (`was:`, upcasts, "contraction never happens") |
| agent-writable | guarded-edit chapters (the #1 demand) |
| executable (strong form) | machine-first-documents chapter |
| addressing substrate | paths seed (the include is a *joint* form — §0b's graft) |

The composition gap is specific: no chapter treats *a document declaring that
part of itself lives elsewhere* as its own subject, with the six dimensions of
§1 worked. Everything else the vision needs has a home.

## 4. Prior art to mine

- **Claude Code `@import`** — the shipped, daily-lived primitive; its semantics
  (load-time, verbatim, hop-limited, and the AGENTIC-DELEGATION.md preamble's
  *reason* for importing whole — "a one-line index brief is a confabulation
  prompt") are field experience about *when transclusion beats indexing*.
- **Obsidian `![[note#heading]]`** — transclusion with fragment addressing; the
  wikilink side of ADR-006's fork, and already a `ux/` target.
- **Liquid** (shipped in autopax) — the templating side; isolated evaluation,
  filters, depth limits. Mine it for its *envelope* (what it deliberately cannot
  do — no arbitrary code, no I/O — is why it's digestible), per the §1 steward
  statement; the dialect itself is not presumed.
- **XInclude / AsciiDoc `include::` / MDX** — three document-format answers with
  three different fidelity contracts (XML infoset splice / preprocessor textual /
  component evaluation).
- **Ash calculations & aggregates** (rowan's ancestry) — computed members
  declared on the schema, loaded on demand: the Inversion applied to dynamism.
- **Dhall** — typed, hash-pinnable imports: the closest shipped form of §1b's
  ascribed include (type mismatch fails loudly; `sha256:…` pins content —
  contract and freshness in one expression).
- **GraphQL fragments** (`...on Type`) — inclusion conditioned on shape;
  consumer-side, composable, interrogable.
- **`llms.txt` bloat** (machine-first chapter) — the failure mode of *not*
  having composition: the flat file that must contain everything.
- **The estate's own build scripts** — `bin/extract-findings`, the OUTLINE
  generators, relata emit: living-document assembly, hardcoded. The cheap probe
  is generalizing one of them behind a declaration rather than inventing anew.

## 5. Probes worth forcing (named, not run)

1. **The fork probe (§1/§1b):** take one real artifact that wants both — an
   agent-card or this repo's own CLAUDE.md — and write it several ways
   (`@`-reference includes / `!`-dynamic includes / mixed-by-tier / the §1b
   ascribed graft with an explicit schema+version contract per slot); see where
   each strains. Cheap, concrete, and directly tests both the "UDON doesn't
   have to choose" claim and whether the typed-graft layer is one-primitive
   simple or too-much-too-soon in practice.
2. **The identity probe (§1.2):** decide-by-prototype whether paths resolve
   *through* an include (the paths seed's resource-as-node stance says yes;
   every shipped transclusion system says no). This is the same seam question
   the terminator table forces for syntax, forced for semantics.
3. **The staleness probe (§2.3):** wire one generated view (the OUTLINE, or
   FINDINGS.md) to fail its build on a dangling or superseded include, and see
   what discipline that actually imposes on a live corpus. The vivarium ref-guard
   incident says this pays for itself in under a day.

## 6. Orthogonal / open — carried, not resolved

- **Where does this distillation eventually live?** The tooling corpus expects a
  new group inside ASF 02-TST; the composition chapter (§3's gap) plausibly lands
  there as a sibling of the templates chapter. This seed is spike-side scratch,
  not that chapter.
- **The executable strong form** ("documents compile to behavior") is
  deliberately under-explored here — two small shipped instances, nothing at
  scale, and it is separable: properties (a)–(d) are worth having even if (e)
  never ships. Don't let the vision's most exciting property become its gating
  one. (proposed discipline)
- **The harness's claim-atom** (the generalization note's first open question)
  bears on what the harness's living documents would be *made of*; unresolved,
  Joseph's read pending.
- **Early-enthusiasm archaeology**: the sapientia stratum likely contains more
  design substance than the two memorata hits surfaced (the PRAXES corpus, the
  consciousness-compiler's actual replay rules). A patient mining pass is cheap
  and has a worked precedent (the 01-ideation extraction fan-out).

---

*Seed authored 2026-07-28 (Fable, udon session) from Joseph's trailheads that
same day. The generative forks to have opinions about first: §1's
reference-vs-evaluation fork (and whether UDON's tiers dissolve it), §2.3's
collision-as-enforcement synthesis, and the §5.1 three-ways probe. Pointers: the
vision statements are autopax ADR-010/008-mp; the generalization note is
`~/src/arch/notes/outline-segments-generalization-2026-07-23.md`; the demand
inventory is §3's table.*
