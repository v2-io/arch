<!--
  Verisectorium notes gather — extract, not full-file authority.
  Provenance: firmatum/udon/v2/theory/to-integrate/refine-more/doc-store-and-schemas-report.md §12.2
  (lines 1844–2067)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/arch/firmatum/udon/v2/theory/to-integrate/refine-more/doc-store-and-schemas-report.md
  Do not edit here expecting to update the live original.
-->

# Extract: Doc-store report §12.2 — OUTLINE + segments

*From the doc-store-and-schemas report. Directory-as-table applied to prose;
the estate instance the rest of that review stands on.*

---

### 12.2 OUTLINE + segments — the instance we are standing on

Pass 1 named this and did not open it. It is the estate's most complete answer to
"directory-as-table applied to prose," it has a written schema spec, and — because
`udon-needs` inherits it — it is the one instance whose design decisions we are
currently living inside.

**The shape.** A component (`01-aat-core/`) holds `src/` — 170 flat segment files — plus
an `OUTLINE.md`. The OUTLINE is literally a sequence of markdown **tables**, one per
chapter, whose columns are `§ | Type | N | Tag | Claim | Stage` and whose `Tag` cell is a
link into the record: `[#def-control-regret](src/def-control-regret.md)`[^asf-outline].
A segment is a record with YAML frontmatter and a prose body[^asf-segment]:

```yaml
---
slug: def-control-regret
type: definition
status: exact
depends:
  - def-value-object
  - def-satisfaction-gap
stage: draft
---
```

**The segment-set principle** is the invariant that makes it a table, and it is stated as
load-bearing for tooling[^format-segmentset]:

> **Every non-`old-*` file in a component's `src/` directory is a segment and conforms to
> the cadence below.** This holds even for drafts, missing-stage entries, or segments
> orphaned from `OUTLINE.md`. … The `old-*` filename prefix is the *only* mechanism for
> placing a file in `src/` that is exempt from FORMAT. … Tools that need the canonical
> segment set … treat `{component}/src/*.md` minus `old-*.md` as authoritative. Adding a
> non-conforming file to `src/` will silently break these tools, so don't.

Three things to take from that. Membership is decided by **directory plus filename
convention** — no manifest, no registry. The single exemption is encoded in the
*filename*, so it is visible in a directory listing and in every diff. And the cost is
stated honestly: the failure mode of violating it is *silent*.

The companion rule is that ordering is not identity[^format-fileorg]:

> **Filename = slug**: `src/{slug}.md`. No numbering in filenames. **Canonical ordering**
> lives in each component's `OUTLINE.md` … not in filenames. The ordering will change as
> the theory develops; **the slug is the stable identity**. **Cross-references** use
> `#slug-name` — everywhere, always.

That is the same conclusion relata reached from the opposite direction (§8.3): position
is volatile, name is stable, and every reference goes through the name.

**Two orthogonal status axes, and an explicit warning against conflating them.**
`type` is a 20-value enumeration of *what kind of claim* this is (postulate, definition,
scope, formulation, derived, result, corollary, hypothesis, normative, empirical,
observation, discussion, measurement, proposed-schema, derivation, worked-example,
detail, sketch, aside)[^format-type]. `status` is an 8-value enumeration of *epistemic
strength* (axiomatic, exact, robust-qualitative, heuristic, conditional, empirical,
discussion-grade, sketch)[^format-status]. `stage` is *"orthogonal to epistemic status.
Tracks where the segment is in our working process, not how strong the claim
is"*[^format-stage].

**Every value in both enumerations links into `terminology/entries/`.** The schema's own
vocabulary is stored as records in the *other* doc-store (§12.1), and `terminology`
carries tags named `segment_types`, `epistemic_vocabulary`, and `process_vocabulary` to
group them[^term-render-vocab]. That is a cross-store join between a schema and its
glossary, and it means renaming a `type` value is a record edit in one store that
propagates to the linter of another. The rationale for the vocabulary is itself
recorded — *"`postulate` (not `axiom`), `result` (not `theorem`), and `derivation` (not
`proof`) avoid the framing that AAT claims foundational mathematical originality where it
does not"*[^format-type] — and it names an exception with a reason: external theorems
keep their original names *"these are other authors' terms and renaming them would
obscure provenance."*

**The duplicated field, and the policy that saves it.** `stage` lives in *both* the
segment frontmatter and the OUTLINE row — a denormalization, and therefore a drift risk.
The spec's answer is the sentence §1.5 leans on[^format-stage-stale]:

> `bin/lint-outline` verifies consistency between the two (mismatches, missing/off-
> vocabulary values, `missing`-vs-file-exists) — **as warnings only, never gate
> failures**: the stage layer is known to go stale quickly under rearranging, pedagogical
> reorientation, and continued refinement, and is currently ignored in practice; **do not
> read low stage values as low epistemic strength** (that's `status`), and do not alarm
> over low acceptance counts.

A duplicated field is permitted, checked, and explicitly *not* trusted. That is a better
answer than either normalizing it away or pretending it is reliable — and `udon-needs`
demonstrates the drift live: its segments carry `stage: drafted (bridge form, 2026-07-22
— absorbed tables now live in the promoted report)`, prose in an enum slot[^udonneeds-seg].

**But warnings-only is a deployment choice, not a property of the field** — and an earlier
draft of this review got that wrong, generalizing one deployment into a rule about
metadata. Joseph (steward, 2026-07-23) states both halves: in `udon-needs`, `stage` is
better understood as a *current-polish-level* state field than as a gate; in other
deployments of the same system — `~/src/arch/logos/`, where papers are assembled and
prepared for **journal submission** — *"the gating action is critical."*

The estate carries the counter-case directly, in the same store family. `logos/refs/` is
a **fourth-generation descendant** of the `neurips/refs` line (§1.2), and its adaptation
note is explicit about what transferred[^logos-provenance]:

> **Adaptation provenance (2026-05-09).** Borrowed from `~/src/neurips/` (the NeurIPS
> 2026 umbrella). The data layout, atomicity contract, and CLI verbs transfer **verbatim**.
> **What does NOT transfer:** the NeurIPS `bin/build` LaTeX pipeline (output format here is
> venue-specific — Synthese is LaTeX/Word; Inquiry is Taylor & Francis — and not yet
> wired).

Same per-entry YAML, same `safe_write`, same append-only verification events, same lint.
And there the lint **gates**: *"`bin/refs lint` is **the anonymization gate before
submission** — it scans every entry and every cited key against the deny-list. Run it
before each Synthese / Inquiry submission."*[^logos-gate] A name that should have been
anonymized and was not cannot be un-submitted.

So the honest generalization is not about the field at all:

> **Whether a check gates is a function of the consuming deployment's stakes ×
> reversibility, not of the field or the schema.** A polish level is revisable at any
> time, so gating on it costs more than it protects and the value rots between reads —
> advisory. An anonymization state at submission is irreversible, so it gates. One schema,
> two enforcement regimes, chosen by the consumer rather than declared by the store.

That reframes what a reader may infer from "the system does not gate on this." It is
evidence about *this deployment's* stakes, not about the field's reliability — though the
two often coincide, and in ASF's case they do: the spec gives the *reason* as staleness
(*"known to go stale quickly under rearranging, pedagogical reorientation, and continued
refinement"*[^format-stage-stale]), which is a claim about the field, alongside the
implicit claim that nothing downstream is irreversible enough to warrant blocking.

This is also the shape of the **enforcement-profile** idea the tooling corpus already
carries — casual / careful / critical as a per-consumer setting rather than a per-schema
one. The doc-store instances in this review are a natural experiment in it: terminology's
render-never-blocks and ASF's warnings-never-fail sit at *careful*; relata's ingest
membrane (nothing enters canonical unvalidated, §7) and logos's pre-submission lint sit at
*critical*; and both pairs run on substantially the same machinery. **The profile belongs
to the deployment; the schema should be able to serve all three.**

**`depends:` is deliberately untyped**, and the reason is worth quoting because the
estate contains the opposite decision too[^format-depends]:

> List the slugs this claim directly depends on. **The type of each dependency
> (definition import vs logical antecedent vs scope assumption) is derivable from the
> referenced file's own `type` field — no typed edges needed.**

Edge type inferred from node type. OPERATA (§16) chose typed edges instead
(`contributes_to` with weights, `blocks`, `related`). Two defensible answers in one
estate: infer the edge from the target when the target's type determines it, name the
edge when the *same pair* can stand in several relations.

**The DAG is the workflow.** Promotion runs in topological order over `depends:`:
*"Promote leaves first, then their dependents. A segment should not reach
`claims-verified` while any of its dependencies is still at `draft` — you cannot verify a
derivation whose premises have not been checked."* Segments are grouped into batches by
DAG depth, parallel within a batch, and downgrade is explicit — *"a segment can be
downgraded (e.g., `candidate` → `draft`) when a dependency changes, an error is found, or
the claim's scope shifts"*[^format-promotion]. Four named gates advance the stage, and
each gate's completion criterion is written out (Gate 1 requires, among other things,
that the dependency be *genuine* — *"not merely 'related' or 'mentioned in
Discussion'"*)[^format-gate1].

**Accepted violations are records, keyed by relation.** `OUTLINE-accepted.md` is a lint
whitelist, and its design is unusually good[^outline-accepted]:

> each table row records one dependency-ordering violation in `OUTLINE.md` that is
> accepted *by design*, with its grounding. The tool still prints accepted violations
> (marked ✓ …) but exits green when only accepted violations remain; any ordering
> violation not listed here stays red. **Rows are keyed by the (segment, depends-on) slug
> pair, so they survive OUTLINE row moves**; if a slug is renamed or the violation is
> otherwise resolved, the row goes stale and the tool reports it as a warning — prune
> stale rows when you see that warning.
>
> To accept a new violation: add a row with the two slugs, the acceptance date, and **a
> reason grounded in a citable record (CHANGELOG entry, decision memo) — not
> convenience.**

Four properties any exception store should copy: keyed by the **relation**, not the
position, so it survives reordering; **staleness is detected and reported**, so dead
exceptions surface instead of accumulating; accepted violations are still **printed**, so
the exception stays visible rather than becoming invisible; and the justification must
cite a **durable record**, which makes "we were in a hurry" unwritable.

**Cross-store referential integrity with stated violation semantics.** Segments cite
experiments as `empirica:<experiment-slug>` optionally `@<run-date>`. The registry is
canon; the contract is that the experiment carries a `MANIFEST.md` *"kept bidirectional
with the citing segments"* and that the claim traces to a recorded run in its `RUNS.md`
with date, parameters, explicit seed, and output. And then the sharp part: *"**An
empirical claim citing an experiment with no matching recorded run is a truth-status
defect.**"*[^format-empirica] A dangling reference is not a broken link — it is a
statement about the claim's truth-status. That is the strongest formulation of
referential integrity found in this review.

**What udon-needs inherited and changed.** Around the same spine it adds the event and
open-item layers — `CHANGELOG.md`, `RESIDUALS.md`, `DEEPENING-CYCLES.md`, `notes/`,
`reports/`[^udonneeds-layout]. The `02-tooling-needs/` OUTLINE keeps the
table-per-part shape and the `§ | Type | Tag | Claim | Stage` columns, but fills `Type`
with a *domain-local* vocabulary — Finding, Principle, Demand, Method, Counterposition —
rather than ASF's theory types[^udonneeds-outline]. The shape is portable; the type
vocabulary is not, and should not be. Its segment frontmatter extends the schema
substantially[^udonneeds-seg]:

```yaml
slug: addressing-is-the-long-pole
type: demand
register: [evidenced, decided]
support-kind: [design, observational, testimonial]
strength: robust-qualitative
convergent: [design, observational, testimonial]
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim…
stage: drafted (bridge form, 2026-07-22 …)
consumers: both (udon-primary)
depends: [schema-guarded-mutation, freshness-and-atomicity]
opens: reports/addressing-exploration.md
handoff-routing: feeds the paths design probe (phase 3); …  # auditor apparatus
sources:
  - ../reports/addressing-exploration.md  # the body report this bridge opens
  …
```

Three additions worth naming. ASF's single `status:` becomes **three axes** — `register`
(how the claim was arrived at) × `support-kind` × `strength` — plus a `convergent:` list
naming *which independent legs agree*; that is §17's "record the vector, not the verdict"
applied to prose. The `verified:` field is an **event log embedded in the record**, where
terminology and relata put events in sibling directories — a third answer, cheaper to
write and harder to append concurrently. And `sources:` carries per-entry inline comments
explaining *why* each source is listed, which is provenance a bare list cannot hold.

