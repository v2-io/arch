---
slug: source-tree-graduation
type: form
depends:
  - integration-metabolism
  - absence-as-structure
---

# Emptying a source tree is a governed operation with a provenance step

*Moving a whole body of prose out of a live tree — after its claims have been landed as segments — needs written completion conditions, and the one that is routinely missing is that the delete-test verdict must itself be checked against the source's prior state rather than against how complete the result looks.*

## The claim

A corpus that grows by mining prose into claims eventually has to deal with the mined-out prose. Leaving it live is not neutral: it taxes every `ls` and every search, it is a decision point on every encounter, and — the real cost — it stays readable as law, so agents keep re-mining it and keep treating superseded design prose as current. So the source is emptied. That operation is [[integration-metabolism]]'s delete-test applied at tree scale, and at tree scale it needs conditions written down, because the person running it is not the person who mined it.

The estate has one instance with the conditions written, and they are worth stating as a set because each covers a different way the move goes wrong:

1. **Claim homes exist** for every load-bearing assertion the file carried — or the file is honestly claim-empty, pure process or history.
2. **Nothing live depends on the path** — no segment, no tooling, no citation reaches into it for current truth.
3. **A manifest entry** names the path, the date, and the superseding slugs (or records it as pure ice).
4. **The move preserves history and repoints citations** that still named the old path.
5. **Provenance is checked before the move:** diff against the last known-good pre-mining parent, or document the intermediate with a section-by-section checklist.

**The fifth is the one worth arguing for, because it is the only one that is not obvious and it was added after a failure.** Its own statement is the sharpest line in the discipline: ***"'Looks complete' and 'claim homes exist' are not provenance."*** Conditions 1–4 are all evaluated against the *current* state of two things — the source, and the corpus that absorbed it. None of them can see content that went missing at an intermediate step, because a partially-mined file and a fully-mined one look identical when you are checking whether the claims you can see have homes. Only a comparison against the source's earlier state can surface what stopped being visible.

That generalizes past trees. **A delete-test run on the present state of an artifact cannot detect loss that happened before the test ran** — and multi-pass integration, which is the normal case, is exactly where that loss occurs. The repair is cheap where version control is present and is essentially never done unprompted.

**Two further properties of the instance are load-bearing and easy to lose in the summary.**

*No path is excluded by design.* The graduation rules apply to any tree — archives, design documents, theory notes, plans, even reference material if a prose note is genuinely superseded. The instance records that an earlier version of its own rule *banned* a particular directory from graduating and names that as a mistake, corrected. The general shape: a completion rule scoped by *location* rather than by *condition* will exempt whatever it did not anticipate, and the exemption will read as principle.

*"Still live under X" means not-yet-mined, not excluded.* Which is the same failure read forward — a readiness state being mistaken for a permanent category.

**And the remainder needs somewhere honest to go that is neither canon nor ice.** Material peeled out that is genuinely novel but not ready for primacy has three declared homes — a working note on the related segment, an honestly-staged exploratory segment, or the session-trail store — with both failure directions named: do not leave it in design prose as though it were still unmined source, and do not over-promote it to top tier merely to clear a file. This is [[honest-incompleteness-discharge]]'s admission rule appearing independently, from the migration side.

## What this does not cover

The outline row that occasioned this segment asks a **larger** question: moving an outline's worth of segments into a *different* verisectorium, with the gates that requires and the frozen-snapshot-superseded-at record it leaves behind. This segment does not answer it. What is described above is one corpus emptying its own substrate into its own claim store — a within-instance operation. Cross-instance migration has at least three partial estate precedents (a theory corpus absorbed into another as a section and later restored to its own component; a component whose `src/` holds 72 files of which **43** are superseded-regime `old-*` carryovers beside only 29 live segments, with the carryovers exempted from the component's own linter by its format law; and a tree relocation that left two byte-identical corpora with divergent histories, where citing the stale twin as live is an active hazard). They have not been put side by side, and no instance has written completion conditions for the cross-instance case. That remains a gap, and it is a genuine one rather than an unwritten segment.

## Strength & grounds

**One shipped instance, read first-hand and whole; the generalization of its fifth rule is argued.**

Read on 2026-08-06 from the live tree at `~/src/arch/vivarium/.super-archive/`: the governing README and its manifest, which indexes eleven dated batches over roughly a week and names, per graduated path, the superseding slugs or an explicit ice verdict. The rules and both quoted phrases are verbatim from that README; the provenance rule carries its own occasion (a 2026-07-23 audit that found an intermediate-peel failure class) and the path-ban correction carries its own date and attribution to the steward.

What that establishes: the operation is real, is running at corpus scale, and has produced written conditions with at least two of them added by failure. What it does not establish: that the conditions are sufficient, or that they hold outside a corpus whose claim homes are unusually well-instrumented. It is one instance, in one estate, on one tree, and the manifest is its own report of its own compliance — nobody outside it has checked a graduated batch against rule 1.

The generalization in the middle of (5) — that a present-state delete-test cannot see multi-pass loss — is analytic and needs no instance. Its *frequency* is unmeasured, which matters, because a rule that guards a rare failure and costs a diff per file is a different proposition from one that guards a common one.

## Working Notes

- This project has the failure the fifth rule guards against, close at hand: its own 2026-08-05 dispatch moved partially-absorbed files under a locally-invented reading and was reverted by the steward's delete-test. That test was run on present state and caught it — so the instance is evidence that present-state testing catches *some* of the class, and says nothing about what it misses.
- Owed, cheap, and specific: pick one graduated batch and run rule 5 backwards — diff the graduated file against its pre-mining parent and check the delta against the named superseding slugs. One such check would convert this segment's central claim from argued to demonstrated, in either direction.
- Adjacent: [[integration-metabolism]] (the delete-test this refines — and the refinement is a strengthening that segment does not currently carry), [[absence-as-structure]] (a manifest of graduated paths is a named-absence store), [[observable-crossings]] (each manifest row is a crossing event written by hand, which is the shape that segment proposes to systematize).
