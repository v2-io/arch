<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: MOVED/udon/v2/theory/FORMAT.md (ported from asf FORMAT; process corpus for UDON theory)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/theory/FORMAT.md
  Do not edit here expecting to update the live original.
-->

# FORMAT — segment conventions for the udon-theory corpus

How claims are written, labelled, promoted, and superseded here.

Ported from [`asf/FORMAT.md`](../../../arch/asf/FORMAT.md) and the two routing SOPs it depends on ([`audit.sop/routing.sop.md`](../../../arch/asf/doc/sop/audit.sop/routing.sop.md), [`spikes.sop.md`](../../../arch/asf/doc/sop/spikes.sop.md)), trimmed to what a foundation corpus needs. Where a rule reads differently from its source, the divergence is stated with its reason. **Nothing in this document has authority beyond being true.** A rule found false is corrected here, not obeyed.

---

## 0. Truth is the arbiter; everything else is a proxy

This governs every section below it.

The job is to get the claims right. Provenance, git history, a CHANGELOG, an OUTLINE row, a spike's own framing, a segment's own assertion, an audit finding, agent consensus, even the convergence of several independent agents — **all of these are proxies for truth, and every one of them drifts.** They are useful for *locating* a question and for *cheap-screening* it. None of them settles it. A question is settled by re-deriving far enough to stand on constitutive structure, forced identities, and elementary steps — not on what any artifact says.

Two recurring traps this names, both observed:

- **An index is a lagging artifact.** "The outline says X" is not evidence that X holds; it is at most evidence about a document that may be stale. Cite it as location, never as authority.
- **"Verified against \<artifact\>" is proxy wearing verification's clothes.** Marking a step verified because a document says so verifies the document. The tell: a verification whose object is *what file F asserts* rather than *the claim holds*.

Every gate below is proxy-discipline — machinery for not being fooled by the cheaper proxies. When a gate is in tension with re-derived truth, truth wins and the gate is what gets corrected.

### 0a. Honest incompleteness is a complete discharge

§0 without this drives a verification regress: every gate spawns another, nothing is released, and an honest *not yet* feels like failure. It is not. The gates exist to prevent false confidence, never to forbid honest incompleteness. Duty is discharged — fully — when, at the current level of understanding:

1. the artifact carries its **honest tier**, not an inflated one;
2. its **Working Notes state precisely what is unresolved and what would resolve it**; and
3. the open remainder is **released to the standing cycle** (a bare `|gap` in the outline, a spike, the open-questions ledger).

Then stop. Strengthen-before-soften means *attempt* the strong result; it does not mean an honest landing at "conditional, and here is exactly what is open" is a failure. That landing is the discipline succeeding.

The self-check for regressing rather than discharging: another gate is being launched not because the corpus would otherwise *lie*, but because an honest lower tier feels insufficient. It is sufficient.

### 0b. Absence claims carry their search

An absence claim — *nothing measures this*, *no prior art covers this*, *this has no owner* — is a universal negative, and a universal negative is warranted only by the search that produced it. State where the search ran. An unsearched absence is the highest-confidence, lowest-warrant claim shape available, and it destroys information: the narrow true statement ("nothing measures X *for this stack*, and here is the instrument that would") is more useful than the broad false one.

Same discipline for impossibility claims. A no-go names its strongest counter-instrument and reports what happened when that instrument was tried.

---

## 1. Why segments, and what the arrangement buys

Two mechanisms, both from the cross-corpus analysis at [`arch/notes/outline-segments-generalization-2026-07-23.md`](../../../arch/notes/outline-segments-generalization-2026-07-23.md) §7, held at that note's own tier (lived report plus specimens for the efficacy claim; near-structural at the core):

**Collision.** A segment forces a present-truth claim, and two present-truth claims about one thing can *collide*. An append-only history layer is collision-free by construction — every entry stays true as a historical fact, so a buried correction has nowhere to surface. Staleness detection is therefore a byproduct of demanding present truth rather than a separate audit step. This is the reason granularity is not a stylistic preference: **a file carrying three claims can be half-stale without colliding with anything.**

**Gap as discontinuity.** Collision finds what a claim contradicts; it cannot find what nothing contradicts. Missing structure needs the complementary instrument: a **visible break** in the outline between neighbors. A bare `|gap` (no key, no slug, no predicted fill) states only that what precedes and what follows do not link directly, and that what follows may lack foundation. It does **not** know how many segments belong in the hole — inventing a typed inventory of future claims is the retired possibility-outline failure mode. Visibility without prediction; an invisible hole is still the worse default.

---

## 2. File organization

- **Segment files** live in `src/` — one claim per file.
- **Filename = slug**: `src/{slug}.udon`. No numbering in filenames.
- **Canonical ordering** lives in an outline, not in filenames — the slug is the stable identity; reordering rows costs nothing, renaming a slug costs everything downstream of it. Provisional canon: [`UDON-THEORY-canon.outline.udon`](UDON-THEORY-canon.outline.udon) (Part I seeded 2026-07-31). The older [`OUTLINE-possibilities.outline.udon`](OUTLINE-possibilities.outline.udon) is possibility texture only — not an ordering to build against (keyed gaps and mass provisional slots were a known failure mode).
- **Cross-references** use `#slug` — everywhere, always.

**Segment-set principle.** Every file in `src/` is a segment and conforms to the cadence in §4. This holds for drafts and for segments orphaned from the outline. Stages describe progress *within* these conventions, not exemption from them. Working material — notes, scratch, spikes — does not belong in `src/`.

**Notation.** Segments and the outline are UDON (`.udon`). This document is markdown because it is prose about conventions rather than part of the claim structure; that split is a convention chosen to have a standard and is revisitable. Segment prose is **block content**, not quoted strings, so the multi-line carve-out (`CARVEOUTS` **ML**) does not apply to it.

---

## 3. Frontmatter

A segment's head is its attribute section:

```udon
|segment[der-row-hedge-sorts] :type derived :status conditional :stage draft
  :depends [def-corpus-population def-attribute-edge]
  :from theory/to-integrate/primary/type-algebra.md§2
  :from spec/MODEL§3
```

### `type` — what kind of claim

ASF's set, adopted whole so a segment landing in ASF needs no translation: `postulate` · `definition` · `scope` · `formulation` · `derived` · `result` · `corollary` · `hypothesis` · `normative` · `empirical` · `observation` · `discussion` · `measurement` · `derivation` · `worked-example` · `detail` · `sketch` · `aside`. Kinds this corpus never uses are simply unused; the list is not trimmed, because a trimmed vocabulary drifts from the parent and the drift is worse than the unused rows.

The words carry deliberate restraint: **`postulate`** not *axiom*, **`result`** not *theorem*, **`derivation`** not *proof* — this corpus deploys existing mathematics and claims no foundational originality. External theorems keep their original names.

### `status` — epistemic strength

`axiomatic` · `exact` · `robust-qualitative` · `heuristic` · `conditional` · `empirical` · `discussion-grade` · `sketch`.

*Solid*, *confident*, *plausible*, *verified* are not strength words. **`verified` in particular is a stage, not a strength** — the two were once one column elsewhere, where `verified` meant both *checked against the code* and *feels solid*, with nothing distinguishing them.

`exact` is claimed defeasibly: valid under stated assumptions, subject to a found error. **A result is not down-tiered for being new** — that pays for the humility twice.

**A status is assigned by whoever read the source at the primary.** A status assigned from a summary is a proxy (§0) and is marked as one until someone opens the source. Label confidence uncorrelated with read-state is the characteristic failure of a fast first pass.

### `stage` — how far it has been checked

`gap` · `draft` · `deps-verified` · `claims-verified` · `format-clean` · `candidate`.

Orthogonal to `status`. **`status: exact, stage: draft` is coherent and common** — the argument is exact and nobody has audited it. That is the whole reason for two fields.

**Stage is a present-tense work-remaining marker, not a gate and not a trophy.**  
It answers *what work is still expected here now*. New information, better organization, and strengthening attempts all reset it; under a gating reading those resets look like regression, which pressures agents to defend rungs instead of improving segments. A corpus sitting at `draft` after a good reorganization is honest, not behind. **A ladder that only promotes accumulates falsehood.**

`gap` is the initial state: an outline row exists, no file does.

### `depends` — prerequisite slugs

The slugs this claim directly uses. The *kind* of each dependency is derivable from the referenced segment's own `type`, so typed edges are unnecessary. Outline order and `depends` are independent and auditable against each other — a segment ordered before something it depends on is a finding.

The graph runs one way. The corpus is **additive**: new claims arrive as new files; reorganization moves outline rows.

### `from` — where the material already is

Section-precise pointers to the material a segment would be written from. This field is what lets a claim be *integrated* rather than *re-derived*, and it is the field that carries the corpus's own integration debt: a row whose `from` is unread is a row nobody has warrant to label.

---

## 4. Document cadence

1. attribute section (§3)
2. `|title` — the human-readable form of the slug
3. `|summary` — one sentence, no more
4. `|formal-expression` — the claim
5. `|epistemic-status` — what is derived versus assumed, what is load-bearing, the ceiling
6. `|discussion` — interpretation and connections; brief
7. `|working-notes` — *optional*

**Cadence exemption.** `intro-*` segments and pure connective `disc-*` segments are exempt from `|formal-expression` and `|epistemic-status`. They do expository or framing work rather than making one formal claim, so the claim mould is the wrong shape. **They carry no independent warrant** — status `discussion-grade` means exactly that, not "uncertain but sort of a claim." Anything load-bearing (a mechanism, a norm, a hypothesis, a transmitted result) is a formulation / normative / hypothesis / derived / observation segment with a status that can be wrong. Using `discussion-grade` as a soft dump for undefended claims is a register failure. An intro may *point at* where weight lives; it may not *be* the weight.

### Epistemic Status is a section, not a caveat

Scope conditions, limits, and honest strength belong **inside** the segment that makes the claim, in a named section — not in a note beside a table, not in a parenthetical, not in another document. **Where the qualifier is adjacent to the claim rather than part of it, the claim travels and the qualifier does not.**

State the **maximum attainable status** and *the evidence-action that would raise it*: "Max attainable: empirical. Currently hypothesis because no experiment has run; the experiment is X." Naming the action turns the field into a strengthen-before-soften work generator. A ceiling that is inherently empirical will never become `exact`, and effort spent proving it is wasted; a sketch whose ceiling is `exact` should not be left at sketch.

**The ceiling is a first-class field, meant to be authoritative in the canonical outline.** Provisional exercise began with Part I of [`UDON-THEORY-canon.outline.udon`](UDON-THEORY-canon.outline.udon) (2026-07-31). This is a divergence from ASF and vivarium, where max-attainable is a prose convention inside Epistemic Status only, and the reason is that a ceiling behaves differently from a status: it follows from the **kind** of claim rather than from work done, so it can be assigned before a source is opened, where a status cannot. A segment carries it as metadata and in its prose; the outline carries it as `:max`, and a disagreement between the two is a finding. Two conventions travel with it:

- **A transmitted claim's ceiling is its source's own tier, never higher** (§8). A claim resting on a `discussion-grade` segment has a `discussion-grade` ceiling however well it is argued here.
- **A normative slot's ceiling is `decided`.** A decision is not truth-apt, so it has no strength rung (§9); recording that as the ceiling is more informative than leaving the field blank, because it says plainly that no amount of work turns this row into a result.

A ceiling beside an absent status is a **route**: it says what could be established at a slot, and that nobody has established it yet. Where a part's ceilings are uniformly `discussion-grade`, that part is bounded and should stop being pushed; where they are `exact` and no derivation exists, the theorem-level work is available and unwalked.

### Working Notes

Forward-work space, not canon. Exactly three things earn a note:

- **Forward pointers** — open follow-on, gating work, unresolved questions.
- **Regression guards** — a disconfirmed prediction, or a form corrected away from *on purpose*, recorded so a later reader who finds the corrected truth messier than the original is told not to restore it. Corrected truth is usually messier than what it replaced.
- **Dead-end warnings** — an approach found not to work. Preserve the dead end; it is what stops the next agent walking it.

What does not belong: **vanity-changelog** — past-work narration ("previously carried X", "the audit recommended a soften"). That is the history layer's job, and the urge is strongest exactly when the fix was a *deletion*, so there is no artifact to point at. Also: **unneeded spike references**, which pin the spike in place and trip the archivability test in §6.

### Segment voice, not diff voice

`|formal-expression`, `|epistemic-status` and `|discussion` present the current state. Not "landed 2026-07-29", not "the prior version treated X as…", not "promoted from spike Y". A segment is read by agents with no context for the chronicle of changes; diff voice forces them to reconstruct the prior state in order to parse the present one, and dates the segment.

**No first person, and no authorial presence.** A segment states what holds, or states honestly that something is one reading among several — *"one reading is"*, *"this is a formulation choice; the alternatives are"*. Where a claim's support is one agent's judgment, that is a `status` value (`discussion-grade`, `heuristic`) and an `|epistemic-status` sentence, not a byline. Where a phenomenological report genuinely carries signal, it is attributed to its reporter in the segment that records it as an observation — the reflector is disclosed, which is what makes it evidence rather than ornament.

---

## 5. Epistemic triage

Three questions when writing or reviewing a segment. They determine its honest `type` and `status`, and its ceiling.

1. **What prior objects make this claim well-typed?** What must exist for the claim to be statable at all? That answer is `depends`.
2. **What competing formulation would also fit those priors?** If none, it may be `derived` or `result`. If several fit and this is the most useful, it is a `formulation`. Most claims have alternatives; being honest about that is the point of the question.
3. **What observation would falsify this in practice?** A concrete falsifier makes it `empirical` or `hypothesis`. If nothing could distinguish it from an alternative and it is not a definition, something is wrong.

| If… | the segment is probably… |
|---|---|
| only one form fits the priors | derived or result |
| several fit; this is the cleanest | formulation |
| depends on the world, not the formalism | empirical or hypothesis |
| no falsifier and not a definition | under-specified — revisit |

**Explanatory prose faces the same triage.** An explanation is either *derived* (cite the chain), a *labelled hypothesis with a falsifier*, or it does not land. A plausible-sounding explanation not grounded in the material is worse than a gap: it reads as knowledge and blocks the search a visible gap invites.

**A convention honestly stated is a complete answer.** "Chosen among what seem to be essentially equal options in order to have a standard; revisit at will" is legitimate reasoning and needs no argument manufactured for it. What is illegitimate is dressing that choice as a derivation. Every claim is derived, evidenced, or decided; a decision stated plainly in its own register costs nothing, and a decision wearing a derivation's clothes costs the reader's trust in the derivations.

---

## 6. Promotion, spikes, and integration

Promotion runs in topological order over `depends`: leaves first, then their dependents. A segment should not reach `claims-verified` while a dependency sits at `draft` — a derivation whose premises are unchecked cannot be verified.

A segment can move **down** the ladder at any time: a dependency changed, an error surfaced, the scope shifted. Downgrade to `draft`, not to an intermediate rung, because the issue may cascade.

### Claims live in segments, not in spikes

Material derived in a spike lands in a segment or in a new appendix segment. Spikes record the *attempt*, the *failed branches*, the *reasoning trail*, and pointers to where the result now lives. A claim that stays only in a spike cannot be cross-referenced, does not appear in the outline, and is invisible to the corpus.

**A reference is not integration.** The decisive test for *integrated*: the load-bearing content appears in a segment, **verified first-hand** — not an outline label, not a Working-Notes pointer, not an agent's summary.

**Three-part completion criterion.** A spike is not integrated until all three hold; partial satisfaction is the looks-done-but-isn't trap.

1. **Content present in a segment**, verified first-hand, and provenance-clean — the post-correction truth, not a regressed restoration.
2. **Nothing *needs* to reference the spike.** No segment or outline row sends a reader to the spike to understand a claim. A breadcrumb in the history layer is fine; the test is *need*, not *mention*. After archiving, grep for the spike slug across `src/` and the outline and **reduce, not repoint**: every hit goes to the live canonical home, and any genuine open content the pointer was hiding surfaces into the segment's own Working Notes.
3. **The navigators are reconciled** at commit time, not deferred. A row still saying "partially landed in #X" when #X is superseded is a lie at the navigator level and is corrected with the same urgency as one in a segment.

### The four completion-states of a spike

Launch a spike broadly: push the thinking as far as it goes, until the claim yields or until it is clear *with specificity* why it cannot. Not "try the obvious fix" — "find the truth here, whatever it is."

- **(A) Strengthened to the claim.** A path is found that makes the asserted epistemology true.
- **(B) Strengthened past the claim.** The spike exceeds both the claim and its own initial skepticism — a uniqueness result where a bound was asked.
- **(C) A no-go.** A result that falsifies the claim and, in falsifying it, exposes the domain more clearly. See below.
- **(D) "Strengthen failed."** Failure reported without a no-go and without exhaustive effort. This is not a result; it is an alarm. Re-spike, find out why the structure did not yield, escalate.

Peer confidence that a strengthening *will* succeed has proven as unreliable as pessimism that it will fail — a cluster adjudication elsewhere in this program predicted a strengthening was "standard textbook, should not fail" and the hard spike returned a no-go. Relaying optimism about what a spike will *result in*, beyond higher truth, is unlikely to help and might prematurely induce delegator-pleasing behavior in some logogenic agent substrates; a brief that says what is wanted and leaves the outcome genuinely open costs nothing by comparison. A disconfirmed prediction is worth recording as a regression guard, since otherwise the next attempt is launched on the strength of the same optimism.

### The no-go protocol

A no-go collapses **two** things, and missing the second is the error this exists to prevent: the claim is now false, *and* the suggested downgrade is also false — it presumed a weaker-but-true residue, and the no-go says there is none in that form. **There is no downgrade option.**

**The invariant, absolute: a segment must never lie about its own status, even transiently.** The history layer carries the change and the reasoning. Segments carry present truth with correct bounds at all times.

So: mark the segment immediately if the correct new form is not obvious — before figuring out next steps takes any time at all — with the falsified status, a link to the spike, and a visible marker. In the segment, not in a tracker. A reader arriving mid-repair must see that it is known-broken.

**A falsified claim does not fall alone.** Every dependent must be found and either cascade-marked or re-derived. An unmarked dependent of a falsified claim is the same lie, one hop away, and the closure is *verified done* rather than left implicit.

### Integration is replacement

A refuted claim is **deleted**, not kept-softened-with-a-pointer. The epistemic label tracks current truth-status, not novelty or provenance. History — *this previously carried X*, *this is not a weakening* — lives only in the history layer. The urge to write reassurance into a segment body is itself the tell that the refuted form has not been deleted.

**Push each death toward a no-go where possible.** A refutation worked into "no X can Y, because Z" stops being history and becomes present-truth content, eligible for a segment on its own strength — and it is often more informative about the problem than the claim it killed. A retired-paths appendix holds what has not yet made that transition; a row leaves the appendix when it has.

---

## 7. Cross-references — every file here, not only segments

**References carry no path.** A path is a location and it rots; a slug is an identity and it does not. Elsewhere in this program, archiving one file dangled three pointers across two repositories in a single afternoon.

**A load-bearing citation carries its read-status.** Material actually read is marked (*read YYYY-MM-DD*) apart from material named-not-read. A source list that cannot tell the two apart has failed here before: a proposed term carve was retracted when its spike's sources turned out to be named rather than read.

The enforcement is structural rather than clerical, and it lives in the segment layer. A segment cites densely because its claims are transmitted or derived from named material; a source that was not read cannot be cited at the precision a claim needs, so the omission surfaces on the page and is checkable by anyone holding the source. **This is why the outline carries no ledger of who read what** — a read-state table records a fact about an author rather than about the claims, it is stale the moment anyone else reads anything, and it defends against a failure the absence of `status` in the outline already prevents. Where unread material breaks the link between two neighbors, that is a bare `|gap` between them (discontinuity only — no id, no predicted fill).

**A load-bearing measured number carries its era.** A measurement is a property of the artifact and version that produced it; crossing a version boundary requires a re-run or an era-scoped quote. Numbers pinned to the current parser carry that framing explicitly (ratified **S2**), because a descriptive pin read as language behavior is the reproduced mistake this corpus exists downstream of.

### 7.1 Local

| form | target |
|---|---|
| `#slug` | a segment in this corpus (`src/slug.udon`) |
| `[#slug](src/slug.udon)` | the same, linkable, from the outline |
| `DECISIONS[<slug>]` | an entry in a decision log, cited by its **exact** slug |

**Cite a decision by its exact slug.** A prefix is a dangling reference, not a short form, and an ellipsis resolves to nothing. Slugs here run long; write the whole one.

### 7.2 Cross-member — the program identity scheme

One scheme for all Archema members. Member namespace plus local identity; do not invent a second dialect that disagrees with it.

| form | target |
|---|---|
| `#asf/<volume>/<slug>` | an ASF claim segment. Volumes: `aat` · `tst` · `llm` · `eli` — bare, per ledger row **X1**, on the grounds that the bare token is more meaningful and memorable and the four are distinct enough to be safe |
| `#asf/term/<slug>` | an ASF terminology entry |
| `#vivarium/<slug>` | a vivarium claim segment |
| `#logos/<paper>` · `#logos/common/<slug>` | the philosophy portfolio |

Canonical example: `#asf/aat/der-directed-separation`.

*Known divergence, recorded rather than silently resolved:* `vivarium/FORMAT.md` §5.2 states the numbered form (`1-aat`, aligned with the `asf/0N-*-core/` directories) and adds "do not invent a second wikilink dialect that disagrees with this". Ledger **X1** and this corpus use the bare form. Two members therefore disagree, which is the situation that rule exists to prevent, and reconciling it is a program-level call rather than something either corpus settles locally.

**Prose form: always the hash form, with a space before `#`** so Obsidian treats it as a link start — `( #asf/aat/def-chronica)`, never `(#asf/…)`. An optional bracket alias maps 1:1 to the same identity; never a third spelling, and never a path-based link.

**`depends` lists this corpus's slugs only** — files that must exist under `src/`. Foreign prerequisites are not `depends` rows, because promotion assumes local files; cite them as `#asf/…` in Formal Expression or Epistemic Status instead. If a claim is unstatable without a foreign segment, say so in Epistemic Status and list the foreign identities there.

**Verify a slug exists before citing it.** A dead cross-reference is worse than none. **Forward references to unwritten segments in this corpus are expected** — they are dependency markers, not broken links.

**Never back-tick a slug when a reference is meant.** It renders poorly, is invisible to tag statistics and search, is not clickable, and cannot jump. Code-ticks are for literal code and literal syntax examples.

### 7.3 Do not restate a definition that lives elsewhere

Link to it and gloss it in one line — enough to recognize the term and gauge relevance, never enough to drift from it.

**An inlined definition is not a copy; it is a fork.** Until terminology tooling is unified across the program there is nothing holding the two in step, and they will diverge. Not hypothetical: elsewhere in this program a document restated a dictionary by hand and thereby used a name the dictionary had retired four days earlier, invented two channels that do not exist, and dropped two that do.

---

## 8. Transmitted claims

Most theoretic material here is **transmitted** from ASF rather than derived locally. A transmitted claim is defeasible on two independent grounds: the source could be wrong, *or* the carriage could be infidelious.

- Cite by slug, which carries the source's own status by reference. **Never upgrade a source's tier.** More of TST is `discussion-grade` than is `conditional`, and exactly one segment is `exact` — so "the theory grounds this" is usually carrying a discussion-grade hypothesis, which is a real position but not the one the phrase implies.
- Where a reader could mistake inheritance for local derivation, say so: *"exact at source, under its premises; the risk here is carriage."*
- Give the plain-word gloss regardless. The corpus stands alone.
- **A known change in a referenced source is a re-verify trigger.** While this is a separate corpus there is no live `depends` edge into ASF, so staleness is honest about the source, not only about local prose.
- **The transfer obligation.** Exporting a theory result requires naming which of the source domain's epistemic properties the target domain shares, which it approximates, and which it lacks, and accepting the corresponding strengthening or weakening. Documents hold three of six, lack two, and hold one weakly — so claims routing through the test channel, git-causal-discovery, or coherence-coupling cross that gap, and the transfer assumption is the interesting part rather than a formality.

---

## 9. The register discipline, stated once

Four registers, and prose is written so a reader can tell which without apparatus:

- **Derived** — follows from stated premises; the reasoning is on the page.
- **Evidenced** — observation, measurement, or testimony supports it, cited where it stands.
- **Decided** — someone chose. Legitimate; never dressed as a derivation.
- **Proposed** — an idea, generated here, closing nothing.

Two labels earn separation where they **route to different repairs**. That test is more portable than any particular axis set, and it is how three sibling corpora found the axes they needed — it caught status-and-stage collapsed into one column, support-kind embedded as rungs on a strength ladder, and a genre-code axis carrying no distinct work. Before adding a field, name the two repairs.

### No absolutes

*The most important* · *the deepest* · *the one thing that matters* · *the real finding* — **an absolute is a claim with no predicate.** Nothing could convict it, so it cannot be marked true at any tier. It gets written at the moment of surprise, which means it reports the author's state as a property of the world, and a future reader inherits one author's surprise-ordering as a ranking with no way to check it.

**The fix is precision, not silence.** *"Load-bearing for the algebra, the resolver, and the migration machinery"* is checkable; *"the load-bearing claim of the whole corpus"* is not. Where the felt sense is itself worth keeping, mark it as one and name its cause — *"on first encounter this read as the deepest thing here, because it unified the store and the seam, which had been held separately."* That says where a mind found load, why, and that it is a report rather than a measurement.

The same applies to impossibility claims and to absences (§0b): each needs the counter-instrument named and the search stated, or it is an absolute wearing a derivation's clothes.

### Framing glosses must be isomorphic

Framing prose may lead with a simpler mental model before the precise structure — that is pedagogy, not dumbing down. The constraint: the model must be isomorphic where it touches, so a reader predicting consequences from the gloss is not misled. An overclaiming scaffold is worse than none.

### Math and the linter

Math is LaTeX in every file — `$…$` inline, `$$…$$` on their own lines — never bare Unicode, never backtick-wrapped Unicode. Both renderers have to work, and GitHub's is stricter:

- no space just inside `$`: `$x^2$`, not `$ x^2 $`;
- `\lt` / `\gt`, never raw `<` or `>` — GitHub reads them as HTML and corrupts everything after;
- `\ast`, never a bare `*` inside `$…$` — the emphasis parser runs first;
- `\lvert…\rvert` / `\lVert…\rVert` for matched delimiters, `\vert` / `\Vert` for single bars; a raw `|` is ambiguous and breaks inside table cells;
- braces for multi-character sub/superscripts: `$x_{t+1}$`;
- `\begin{aligned}` inside `$$…$$`, never `\begin{align}`;
- **no `#slug` inside math** — not in `$…$`, not inside `\text{}` or `\underbrace{}_{}`. LaTeX reads `#` as a macro parameter, and a slug is a prose cross-reference rather than a mathematical object. Lift it into the prose.

**`fmt-md` is the mechanical check for the markdown here** (this file; the older per-repo linters are deprecated). It canonicalizes prose — unwrapping manually-wrapped paragraphs to one line each, including inside list items, blockquotes and footnotes — and leaves tables, code, frontmatter, math, wikilinks, HTML and meaning-bearing breaks alone. It edits named files in place; `--check` is a dry run, and `fmt-md - < FILE | diff FILE -` previews the edits. A render-equality gate runs before any write: it re-parses its own output and, if the rendered document would change at all, leaves the file byte-identical and reports instead. `--help` is the live source of truth.

A `.fmt-mdignore` (gitignore syntax, at or above a file) marks material as not-for-formatting and is honored even when the file is named explicitly — which matters for verbatim material: transcripts, provenanced copies, frozen archaeology. Reformatting those is render-equivalent and still wrong, and no automatic check can tell, because nothing about the rendered document changed.

**Do not run `fmt-md --allow-udon` (or `-` stdin) on a `.udon` file, and don't rely on `--force` for safety here.** As of `fmt-md` commit `caa502b` (installed in this environment 2026-07-30) the tool skips `.udon` files by default with an explicit message naming the reason (its line breaks carry meaning fmt-md has no model of); `--force` deliberately does **not** override that skip, only `--allow-udon` does — and stdin mode (`fmt-md - < f.udon`) is not covered by the skip and remains a live hole. The damage under `--allow-udon`, confirmed by direct test 2026-07-30, is not the attribute-joining described in the original 2026-07-29 assessment (`~/src/arch/utils/fmt-md/UDON-ASSESSMENT-2026-07-29.md`) — that write-up's own mechanism has since changed. The current mechanism is **hard-break marker injection**: attribute and other short lines get a trailing `"  "` appended (Markdown's hard-line-break syntax), including inside a `!:label:` verbatim body, while prose paragraphs are still joined. This still corrupts the document's reconstructed text value even though the render-equality gate stays silent — the injected markers preserve CommonMark's rendered output, which is exactly why the gate cannot catch it: "each text line's terminator is part of its text" (CORE §7.2; the text law, MODEL §6), and fmt-md's soundness argument is scoped to markdown's semantics, not UDON's.

**There is otherwise no canonicalizer and no conformance checker for `.udon`** — the parser in `../../core/` is 0.8-lineage and non-conformant to the ruled model, so it cannot serve as one either. Reading the ruled text is currently the only instrument, and a minimal utility suite is expected within a few sessions. Until then this section is the mitigation, which is a thinner defense than a tool and is stated as such.

**Look at the file anyway.** Format-clean is not renders-well, and a mechanical pass that skips code spans lets backtick-wrapped Unicode pass while still rendering badly.

---

## 10. Where the material is

| Source | Holds | Constraint on use |
|---|---|---|
| [`../current-0.9.1-spec/`](../current-0.9.1-spec/) | ruled law; the only recognition oracle | semi-frozen and spec-only (ledger **C8**); read whole before building on any of it — windowing a primary manufactures a confident reconstruction of the parts skipped |
| [`../spikes/`](../spikes/) | territory seeds, measured tables, the type algebra, the failure archaeology, the dialogue letters | the primaries. A synthesis is an input, not an authority; where a primary exists, cite it |
| [`../udon-needs/`](../udon-needs/) | the demand evidence | its chapters are **pre-segmentation by its own record** — the split is "coming, deliberately not yet", chapter-equals-file is "transitional", and its methods chapter is flagged overloaded. Take content, not shape |
| [`../DECISIONS.md`](../DECISIONS.md) · [`../OPEN.md`](../OPEN.md) | present truth · live questions | cited, never amended from here |
| `~/src/arch/asf/` | the theory, at its own tiers | §8 |
| `~/src/arch/asf/empirica/` | the experiment registry | an empirical claim citing an experiment with no matching recorded run is a **truth-status defect** |

**Agreement inside this estate is coherence, not corroboration.** Several of these corpora share one author, so when two agree that is one mind being consistent with itself — evidence about design intent, which is real and load-bearing, and not independent support. Say "these were built for each other", not "these converge".

**The steward's in-session brainstorms are pre-validation.** They live in the discussion registers with per-item assessment, are cited as discussion-thoughts, and are never marks or rulings. Fiat, when exercised, is marked as fiat expressly.

**Past decisions carry no normative standing, however accurately reported.** The 0.9-era spellings were forward-looking guesses aimed at exactly this work; correction authority belongs to the demand and theory side. A crisp measurement of an incumbent lends its crispness to the incumbent frame — that is the channel by which the reflex re-enters even under explicit counter-instruction, which is why the register marks apply to design reasoning and not only to claims.

---

## 11. Ambiguity

Ambiguity is information. A claim whose placement is unclear becomes an `explore-` segment or a bare `|gap` between the neighbors it fails to join; the arrangement absorbs that additively. Two segments that contradict is the collision mechanism working: report the disagreement factually — what each says, with cites — rather than picking a winner, because which resolution applies is a reasoning call that verdicts have gotten wrong.

This is a corpus under construction, and the outline is a proposal about organization rather than a finding about it. Moving rows costs nothing.

---

## 12. Open questions about these conventions

*Open, not deferred. None is settled by an agent session alone.*

1. **Which label axes does a derivation corpus need?** ASF's `type`/`status`/ `stage` travel to 02-TST untranslated, which is the argument for them. The demand corpus's support-kind axis is calibrated for surveyed evidence — design, observational, testimonial, measured — where the distinction that has done work here is *derived-here* versus *transmitted* versus *spec-grounded*. Adopting the first set wholesale would fail §9's own test. The replacement has not been designed, and the two locks (failure-mode independence; carriage-versus-source defeasibility) may be portable where the kinds are not.

2. **Does this corpus need kinds ASF does not?** ASF's set is a theory's. A foundation corpus states a good deal that is neither claim nor definition — a *territory consequence*, a *spelling election*, an *extraction target for the spec*. Part IX's rows are currently `normative`, which may be the wrong word for "a derivation that reached a spelling." Vivarium logged the same suspicion for specifications and declined to pick a word quickly. Coining is allowed; the constraint is that a coinage must not sit confusably beside ASF's vocabulary, and a paper drawing on both must read coherently.

3. **Does the promotion ladder survive a corpus whose claims are mostly transmitted?** ASF's gates ride on a partial order of proof steps. Here a large fraction of claims are inherited, where the meaningful gate is *carriage verified at the primary* rather than *derivation traced*. Whether that is a new gate, or is simply what `claims-verified` means for transmitted material, is undecided — and this is the part most likely to need rework and least worth defending.

4. **Where does this corpus end?** It touches the demand evidence at typing, addressing and schema, and no boundary is drawn. A claim that belongs in both places is currently a duplication risk with nothing detecting it.

5. **Should the outline itself be a segment set?** Its `|arc` blocks are prose nobody labels, on the argument that ordering narrative makes no claim about the world. If that argument is wrong, the arcs are unlabelled assertions in the one file de-novo auditors read.
