---
slug: verification-provenance
type: form
depends:
  - identities-over-locations
  - gate-profile-divergence
  - cousin-store-lineage
---

# A dangling citation is a truth-status defect, and support needs re-checking

*Verification machinery is usually built as link-checking, which frames a broken reference as a housekeeping error; the stronger framing — already in force in this estate — is that a claim whose support cannot be located is a claim whose truth-status is defective, and that support decays after the check that established it.*

## The claim

**(A) Reference integrity is an epistemic property.** ASF's format rule states it plainly for cited experiments: *"An empirical claim citing an experiment with no matching recorded run is a truth-status defect."* Read generally: when a record's support is addressed but not resolvable — a premise slug that no longer says what the dependent assumed, an experiment with no recorded run, a quoted span that no longer locates in its primary — the correct report is about the *claim*, not about the link. This changes what a checker is for, and therefore where its failures go.

**(B) Verification is an event, and "verified" is derived.** The durable form is an append-only record per check — what was checked, against what criterion, by whom, when, with what outcome — kept per record rather than in a global log. A record's overall verification state is then a *function* of the latest event per criterion, not a hand-set field that forgets who said what. Mutable status fields lose exactly the history that would let a later reader judge whether the check still applies.

**(C) Support decays; a claim-time check is not a standing one.** Checking at the moment of writing accumulates silent lies afterwards: the cited record gets edited, the primary moves, the experiment is re-run with different parameters. So a corpus needs *standing re-verification* of support — cheaper than a deep audit and catching the class a rare deep audit is too infrequent to catch. The estate's live specimen of the failure is the comproprium provenance break, where 106 of 109 quoted spans stopped locating after one ordinary directory move and the corpus's own checker was disabled by the same move ([[provenance-rot-specimen]]).

**(D) Whether a check *gates* is the consuming deployment's call.** The same machinery can warn in one deployment and block in another, and the choice tracks the consumer's stakes and reversibility rather than any property of the field being checked. The estate supplies an unusually clean instance: two bibliography stores of one documented lineage, running the same layout and the same tooling, sit roughly ten-fold apart in verification coverage — and the one with the higher coverage is the one whose lint is run as the anonymization gate before journal submission, where a name that should have been removed cannot be un-submitted. (The gate is procedural — the tool blocks nothing mechanically in either deployment; what differs is one schema under two deployments with a ten-fold difference in how much the machinery is exercised.) Full counts and what they do and do not establish: [[gate-profile-divergence]].

**(E) A second axis, independent of (D).** Stakes decide whether failure *blocks*. What decides whether the system may act *without a human* is the grade of the evidence — machine-checkable anchors versus judgment. A high-stakes deployment can still auto-accept an identifier-grade check; a low-stakes one can still park a judgment call. Multi-agent agreement and generated formal expressions are judgment-grade unless tied to something mechanically checkable.

## Strength & grounds

**Heuristic**, with first-hand reading of two live rules. (A)'s sentence was read in `~/src/arch/asf/FORMAT.md` on 2026-08-05. Its companion design lives in a different file — `~/src/arch/asf/01-aat-core/OUTLINE-accepted.md`, an accepted-violations store whose rows are *"keyed by the (segment, depends-on) slug pair, so they survive OUTLINE row moves"* and which reports stale rows rather than accumulating dead exceptions. (B) and (C) are relata's shipped shape (`verifications/` as a sibling tree with per-key event directories, plus a non-destructive standing re-check), examined the same day; the transfer of that shape to claim corpora is proposed, not demonstrated. (D) rests on [[gate-profile-divergence]], whose counts are a survey's rather than this segment's; the gate-versus-warn reading is the estate steward's own, recorded 2026-07-23. Single estate, shared authorship throughout — what would raise it is an outside corpus reporting that framing dangling support as a truth-status defect changed what its authors did, rather than only what its linter printed.

## Working Notes

- The unbuilt piece with the clearest shape: standing re-verification that a cited record still asserts what its dependents assumed. Content collision catches the case where someone states the contradiction; nothing catches the case where the premise quietly narrowed.
- (E) has almost no expression in the estate — enforcement is discussed as gate-vs-warn and hardly ever as may-the-system-act-alone. Worth watching when promotion tooling is designed.
- Unintegrated influx behind this segment (do not cite as warrant): `plan/INFLUX/udon-analysis/doc-store-report-s12-2-outline-segments.md`, `plan/INFLUX/synthesis/relata-methods-for-verisectorium-2026-08-05.md` §§2.11/3.8, and `plan/INFLUX/synthesis/live-state-field-reports-2026-08-05.md` for the (D) counts. Live authorities: the ASF format document, the relata tree, and the two refs stores themselves.
