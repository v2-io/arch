<!--
  Verisectorium theory INFLUX — instrumenta mining, verbatim copy.
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/udon-needs/02-tooling-needs/src/priorities-and-spike-agenda.md
  Copied: 2026-08-06 (mining agent, first outside reader of the theory corpus).
  This is a copy, not authority — do not edit here; segments draft from live sources.
  NOTE: the MOVED/ tree is mid-reorg; verify the source path before drafting from it.
-->

---
slug: priorities-and-spike-agenda
type: synthesis
register: decided
support-kind: —
strength: —   # a balance sheet for downstream weighing, not a verdict; ranks rather than evidences
convergent: —   # synthesis over the drafted chapters; carries no independent evidence of its own
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: both
depends: [schema-guarded-mutation, addressing-is-the-long-pole, streaming-and-partial-documents, persistence-is-imported, context-economy]
---

# The balance sheet, and what the design probes ahead should answer

**What this chapter is.** Not a conclusion — an inventory arranged for weighing. The stages after this report (capability prioritization, then design decisions, then the tool pipeline itself) need three things laid out with full information: the **demands** with their evidence strength and measured cost of absence; the **capability inventory** this report generated, each entry carrying its own grounding, tensions, and downsides; and the **questions** each design probe must answer. The weighing is theirs. Where this chapter ranks, it ranks *demand strength* — how many independent kinds of evidence ask, how broadly, at what measured cost — never build order.

## The demand ranking (strongest evidence first)

1. **Schema-guarded structural mutation** — every kind of evidence asks; measured cost of absence (recovery collapsing to one scenario in six without compensating infrastructure); a stated owner demand; a gap no shipping tool fills. It pulls paths, schema, spans, and round-trip at once — the organizing *customer* of the design agenda, not one feature among many.
2. **Stable addressing with loud, typed failure** — the long pole; nearly everything else consumes it.
3. **Partial-document honesty as a public surface** — verdicts, mid-stream state, prefixes that parse; the ecosystem pays a standing reassembly tax for formats without it.
4. **Reinjection-channel formats** — durable agent state, cold-start-reconstructible; the only cross-session persistence mechanism there is, with a lived wound as its cost-of-absence exhibit.
5. **Teaching refusals and description-as-teaching** as the pervasive error contract — cheap everywhere once adopted; the report's strongest cross-evidence lock.
6. **Context-economy compatibility** — compact, self-describing, addressable payloads that survive the spill/defer/prune machinery.
7. **The headless contract** for every CLI either consumer ships — a floor, not a differentiator.

## The capability inventory (the balanceable half)

The ranking above weighs *demands*. The report's other product is the **capability cards and ✦-marked proposals** scattered through every chapter — each an affordance that could answer some demand, each carrying (by the methods chapter's convention) its principles, its hypothesized impact in the theory's own quantities, what it stands in tension with, and its downsides. Harvest them by skimming for `[!capability]` and ✦ across the chapters; the tension links between them ("in tension with: byte-fidelity round-trip…") are the beginnings of the trade-off structure the prioritization stage exists to navigate. Nothing in the inventory is decided; several entries mutually exclude; that is what makes it a balance sheet rather than a plan. Notable clusters, as an entry map: guard-and-transaction capabilities (around the flagship demand); address-and-freshness capabilities (paths, hashes, leases, staleness scoping); context-economy capabilities (budgets, spills, focus questions); continuity capabilities (canonical hashing, witness records, good-enough linting); and human-surface capabilities (review-grade diffs, use-the-product verification, bidirectional steering).

## What each design probe inherits (questions, not answers)

**The paths probe** inherits the [[addressing-is-the-long-pole| addressing chapter]] whole, with [[addressing-exploration| the addressing exploration]] as its deep context. Decisive questions: the smallest in-document reference form that is a *true* subset of the full path language; where paths terminate in every value context (the forced embeddability table); relational-first versus tree-first as primary mental model (stress the one-day sample against prose-heavy and append-only documents); and the anchor-kind space (relative / absolute / home / document-root / project-root — the last already shipped once in this programme, sigil undecided). Standing decisions travel with it: cross-document addressing is in scope; the existing reference form does not grow incrementally. The failure vocabulary — not-found, not-unique, plural, stale — is part of the deliverable, not polish. And one question added by a cross-family reviewer: are paths strictly *necessary* for guarded mutation, or one targeting mechanism among several? A weaker-but-sooner guard (exact-match targeting plus post-edit conformance check) might exist; the probe should answer rather than assume.

**The dialects probe** — the least-fed area relative to its leverage, and this report's writing said so early. Entry points now on record: the in-vivo typed-value experiment (wire the existing temporal grammar in as a live sub-parser and let running code force the define/compile/declare/invoke/error questions — the preferred first move of every reviewer who considered it); the template product shape (compile → interrogate for requirements → build) with UDON-as-context pulling directives toward path expressions; the two-evaluation-sites resemblance (log the unification pressure, don't unify early); badly-typed values on an unbounded stream (what does a failed-dialect event look like to a consumer that cannot accumulate?); mid-stream reconfiguration; and whether bracketed and quoted captures are sugar for dialect-typed captures — the reframe that may dissolve the multi-line question entirely rather than answer it.

**The schema probe** runs *against* the [[schema-guarded-mutation| guarded-mutation chapter]]'s transaction shape: static versus composable or nested (the owner's own open question); what conformance means on a *partial* document (compose with the verdict channel); the guard's failure vocabulary as schema output; the evolution demand (declared renames, read-time translation, schema history — from the family that has lived without it); and enforcement profiles (casual / careful / critical) as schema *strictness*, distinct from schema *shape*. External anchor: schema- serialization mismatch as the dominant real-world tool-fault class.

**The value-typing probe** — the frozen-bare-set-plus-envelope boundary is already decided; what demand still has to shape: dispatch order for unlabelled typed values under multiple dialects; who routes nested typed values; and recognition-time versus resolution-time typing for stream consumers. Runs jointly with the dialects probe or immediately after.

## Sequencing dissents worth carrying (cross-family reviews, 2026-07-22)

Two product-strategy positions from reviewers outside this model family, carried visibly rather than adjudicated here: (a) **near-term value first** — teaching refusals, partial-document honesty, and a distinct staleness failure class are shippable *before* the paths-and-schema program lands; the demand ranking is a strength ordering, not a build order that parks all value behind the long pole. (b) **The human-verification surface is load-bearing, not garnish** — plausible wrongness is the failure class nothing mechanical catches, so the thinness of the human-side evidence is a risk to fund against, not a sign of low importance.

## The probe discipline (learned the hard way, twice)

Probes run against *scenarios from this report's chapters* — the lived situations, the affordance lists, the walkthroughs — never as free essay prompts; their outputs are demands and evidence for the product graph, never design pins (an earlier architecture effort in this project pinned its shape before the demand work and was archived for it; this report exists partly because of that lesson). And each probe should name in advance what would *falsify* its favorite shape — the counter-evidence discipline applied forward.
