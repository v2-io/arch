---
slug: priming-discipline
type: norm
depends: []
---

# Auditor-safe and priming-heavy surfaces are declared, and read in that order

*A corpus that wants independent judgment about itself has to keep the material carrying its own verdicts separate from the material that merely orients — and has to say which is which, because by the time a reader notices the difference the reading has already happened.*

## The norm

**Declare two classes of surface.** *Orientation* material — structure, vocabulary, notation, format rules, the outline — may be read first by anyone, including a reader whose job is to judge the corpus. *Verdict-bearing* material — curated findings and rankings, positioning documents, the reasoning trail of spikes and prior audits, live trackers of known issues, changelogs and per-file history — is read **after** a first-hand reading of the thing being judged, never before. The test for which class a file is in: *does reading this tell me what someone else already concluded about the material I am about to judge?*

**Order applies per item, not just per session.** The same discipline runs at segment grain: read the record, form and record a reading, and only then open the spike that produced it, its git history, the prior audit that touched it, or the external work it cites. A reader who wants the spike *before* the record has surfaced something worth noting — the record may not be standing on its own.

**Why order rather than exclusion.** Nothing here is forbidden reading; the material is fair game in a later pass, where it does its actual job — telling you whether a concern you formed independently is already known. What cannot be recovered is the independent formation. Attention is spendable and the spending is irreversible: you cannot un-read the verdict, and an informed impression is not a first impression no matter how sincerely it is offered.

**A corpus that wants this must build the surface.** The discipline is unusable without an audit-safe entry point, so producing one is part of adopting the norm rather than a nicety — ASF ships a generated `README-auditor.md` beside its public README precisely because the public one carries findings and known-issues sections.

**Two consequences worth stating so they are not rediscovered as problems.** First, the value scales with the *number* of independent readings: the point of withholding prior verdicts is decorrelated error across readers, so anything that leaks between them — a shared brief, an orchestrator's framing, a well-known house opinion about which part is weak — quietly re-correlates them and costs more than it looks like it costs. Second, this design deliberately manufactures false positives: a reader kept away from the known-issues list will raise things the project already knows, and that mortality rate is the mechanism working, not carelessness. Saying so up front is part of the norm, because a reader watching its own candidates die has no other way to tell the two apart and will otherwise get quieter.

Decided for this corpus and recommended for the pattern generally. Not truth-apt; overturn expressly.

## Provenance and standing

The discipline is ASF's, in daily use: the live source is `~/src/arch/asf/doc/sop/audit.sop/de-novo.sop.md` (read first-hand 2026-08-05), which carries the auditor-safe README split, the priming blacklist by category, and the per-segment source ordering as a named section. The norm above is that SOP's discipline lifted out of its corpus and stated for the pattern. The two consequences in the last paragraph are not in the SOP: they are one reader's annotations on it, written 2026-07-30 at hypothesis rung by an annotator who had not run the protocol, and they are stated here because they are cheap to act on and self-labelling, not because they have been tested. The decorrelation claim in particular is checkable against an audit corpus — inter-reader finding overlap should be lower for independent passes than for passes that read prior audits first — and has not been checked.

## Working Notes

- The unaddressed flank in the source, worth carrying: this whole discipline guards against priming *by others* and says nothing about a reader's own accumulating model becoming the contaminant over a long walk. A cheap mitigation exists (predict a record's content before consulting your own running notes, so the self-priming is measured rather than assumed away) and has not been tried.
- Role-specific onboarding — what a de-novo auditor needs that a harvester or integrator does not — is the ch. 5 gap row; this norm is only the surface-declaration half.
- The annotated, genericized copy of the SOP — the source of the two hypothesis-rung consequences — is live at `~/src/arch/firmatum/udon/v2/theory/to-integrate/refine-more/de-novo-audit.md` (verified byte-identical to the intake copy `plan/INFLUX/udon-analysis/de-novo-audit-generic.md`, which is not a warrant and archives). Note a byte-identical twin tree exists at `~/src/MOVED/udon/` whose git history is behind as of 2026-08-05; the firmatum path is the live one.
- The related and distinct question of *what may serve as warrant* (an index, a status label, agreement among agents) is not covered here; it is closer to [[warrant-over-authority]].
