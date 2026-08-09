---
slug: claim-outline-as-view
form: claim
type-expected: derived
status: heuristic
max: robust-qualitative
state: [drafted]
depends: [def-atom, claim-identity-ordering-split]
---

# Claim: Outlines Are Views

Outlines are cheap and atoms are expensive; the substrate is presentation-neutral, any number of views can coexist over shared atoms, and trimming or reshaping an exposition is a view operation, never an atom edit.

## Formal Expression

*[Claim (outline-as-view)]*

Let the corpus be the population of atoms ( [[def-atom]]) and an **outline** be an authored selection + ordering + framing over that population. Then:

1. **Cost asymmetry.** Creating or reorganizing an outline costs one file and its framing prose; editing atoms costs propagation through everything depending on them. The default for any exposition-shaped proposal is therefore outline-first: before scoping work as atom rewrites, ask whether it lands as a new view.
2. **Presentation-neutral substrate.** Atoms carry no presentation commitments — no positional numbering, no audience assumptions, no venue shape ( [[claim-identity-ordering-split]]). Everything presentational (ordering, numbering, layer filtering, rendering directives) is view property, resolved at assembly.
3. **Multiplicity.** Any number of views coexist over one population — pedagogical walk, page-budgeted venue subset, status-filtered promoted view, entry-path per audience, paper draft — each buildable, each coherent, none requiring atom edits. Fitting a smaller form means selecting fewer atoms into that view's manifest, never editing atoms to fit ("page count is observable, not actionable on segments").
4. **Canon among the many.** One view is the canon — the authoritative pedagogical ordering — and its primacy is declared rather than emergent ( [[form-canon-view]]). Multiplicity and canon-primacy are compatible commitments, not tensions.

## Epistemic Status

Heuristic, with `robust-qualitative` attainable. The qualitative claim (cost asymmetry and the resulting outline-first default) is supported by the estate's strongest available evidence pattern: the same law was articulated twice under *different real pressures* — in asf under audit-portfolio pressure ("outlines are cheap; segments are expensive… before scoping a proposal as a segment rewrite, ask: can this land as a new outline?") and in the neurips instance under venue page-budget pressure (multiple assembly manifests selecting different subsets of one substrate). Honesty about that evidence per the estate's own rule: both articulations arose in one steward's estate, so this is coherence — one program being consistent with itself — plus genuine *pressure-diversity*, which is more than design intent but less than independent replication. The quantitative form (how strongly the asymmetry holds as atom count and view count scale) is unmeasured. Raising to `robust-qualitative` requires either an external instance or a measured cost comparison on this estate; the evidence-action is the natural first observation-store query ( [[form-observation-store]]).

## Discussion

**Why the claim is load-bearing beyond economics.** The cost asymmetry is the visible half; the cognitive half is the deeper stake. Outline altitude is where reorganization and structural conceptualization happen at all for agent readers — pre-concatenated corpora flatten under whole-context attention, and the pondering that outlines afford is precisely what concatenated reading suppresses ( [[claim-outline-altitude-cognition]], hypothesis-grade). A system that made views expensive would not merely waste effort; it would price out the altitude at which its own structure gets re-thought.

**What rides on view rows.** View metadata does real work beyond ordering: type columns driving structure injection at assembly, importance stars driving orientation quizzes, framing prose carrying the pedagogical scaffold, filters selecting atom layers per audience ( [[form-view-filters]]). A view is an authored document with edge attributes, not a bare sequence — which is why views are *authored* (and steward-leveraged, [[norm-outline-first]]) rather than generated, and why generated views ( [[form-generated-views]]) are a separate, declared kind.

**Which columns are the view's own.** Nearly every column a view row displays — type, epistemic status, process state, ceiling — is a view-local *denormalized copy* of the atom's frontmatter, kept for scanability and owed a staleness check against its source; the designator is not even a copy but the atom's identity surfacing (implicit in the record's filename). The one genuinely view-native content column is the **description**: not a copy of the atom's summary but the view's own *perspective* on the atom — what this exposition needs the reader to take the atom as — which legitimately differs across coexisting views over one atom (steward articulation, 2026-08-09). This sharpens [[form-view-edge-metadata]]: denorm columns are cached atom facts riding the edge; the description is a true edge fact.

**Failure mode the claim predicts.** Where an instance lacks the view layer (order carried in filenames, one hard-coded assembly), reordering requires renames, multi-audience exposition requires duplication, and the paper-vs-corpus tension becomes real instead of dissolved. The estate's paper-lite instances exhibit exactly this: workable at small scale, with the manifest layer's absence visible as the inability to hold a second view. The claim's practical content for the kit: the view layer is what buys multi-view, and it is worth its cost as soon as views multiply — which for living collections is early.

## Working Notes

- Frontmatter schema provisional pending the epistemology decision.
- Open: the view-dialect question ( [[disc-view-dialects]]) — where view-local rendering directives live (manifest row types vs build hard-coding vs declared filters) is one question with three live answers; this claim is agnostic among them but the kit cannot be.
- Open: whether "trimming is view membership, never atom editing" needs a stated exception class — an emission whose destination *legally requires* content changes (anonymization) currently routes through build-time transforms rather than atom edits, which preserves the law but deserves explicit treatment at [[form-efflux-seams]].
- Evidence-action logged (per [[form-max-attainable]]): to raise status, measure reorganization cost with vs without a view layer on this estate's own instances, or document a genuinely external instance arriving at the same law.
