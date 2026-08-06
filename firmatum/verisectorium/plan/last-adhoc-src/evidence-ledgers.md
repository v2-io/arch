---
slug: evidence-ledgers
type: form
depends: []
---

# Weight of evidence: one sum, priors held apart, correlated signals counted once

*Multi-signal judgments — is this claim ready, is this dependency genuine, do these three reviews agree — are made constantly and almost always by feel; there is a shipped alternative that costs one arithmetic rule and four disciplines, and it makes the judgment auditable instead of merely confident.*

## The claim

**(A) One rule, no points-tangle.** A judgment combining several signals should be a single sum in log-odds — a starting prior plus one weight-of-evidence term per observation, with acceptance at one threshold — rather than a collection of per-factor thresholds and heuristics that each *feel* like a gate. The alternative is not a worse formula; it is an un-auditable one, because no one can say afterwards which signal carried the decision.

**(B) Priors and likelihoods stay strictly separate.** Population facts known before looking at the content are the prior; things observed *in* the content are likelihood terms. Folding a base rate into the evidence sum is how base rates silently manufacture acceptance. In claim work the two are easy to confuse: *"segments at this stage are usually incomplete"* and *"this segment's listed premises do not re-derive its formal expression"* feel like the same kind of consideration and are not. This separation is the mechanism that makes the standing warning against reading process stage as epistemic strength actually enforceable rather than merely written down.

**(C) Absence is never refutation, and refutation is soft.** A missing dependency target, an unread primary, a cited experiment with no recorded run — these are *absences*, and their honest handling is a named gap or a truth-status defect, not a silent negative. A genuine contradiction is different: it is a large but finite negative term that can in principle be outweighed by abundant independent support, and its distinctive effect is to **stop early-stopping** — a standing contradiction should expand the work, not conclude it.

**(D) Correlated signals are one channel.** Two observations derived from the same source contribute one term, not two. This is the discipline that multi-agent review most reliably violates: three agents from one session with one brief and one disposition, or an outline row plus a frontmatter field plus a reviewer's impression all tracing to the same original judgment, present as convergence and are one observation wearing three hats. The rule to run before combining anything: *do these signals share a generative source?* A convergence lock should be armed only by support of at least two kinds with genuinely independent failure modes; agreement within one kind raises confidence without arming it.

**(E) Constants need one site and a defended chain.** Any number that gates a corpus — a promotion threshold, "verified means these three criteria" — belongs at exactly one place, with a written justification of why it is that number and what would revise it, and a change to it should be a deliberate human act rather than an automatic fit.

## Strength & grounds

**Heuristic**, resting on one shipped implementation read first-hand rather than on a derivation. All five disciplines are implemented and documented in relata, verified 2026-08-05 in `~/src/arch/firmatum/relata/lib/relata/evidence_ledger.rb`, whose own header states the single-sum rule, the strict prior/likelihood separation, refutation as a finite negative that suppresses early-stopping while *"absence of a factor is never a refutation"*, and the independence rule with its worked case — signals parsed from the same filename are emitted as at most one factor, because counting them separately let a lone renamed file clear the acceptance threshold. That module is named in its own code as the single calibration site. The corpus it governs held 2,277 entries and 219 verification directories on that date.

**The transfer is a proposal, not a result.** Relata adjudicates bibliographic identity, where the observations are cheap and often machine-checkable; claim corpora adjudicate whether an argument holds, where they are neither. What plausibly transfers is the *shape* — vector not scalar, priors apart, absence ≠ conflict, independence before combination — and not the arithmetic. The honest caution the source supplies about itself is worth repeating: the discipline is the innovation, its production calibration loop was still catching up as of mid-2026, and one should not claim a calibration system exists because the class names exist.

## Working Notes

- (D) is the item this project can act on immediately and at no cost — the shared-authorship caveat already carried in several drafted segments is exactly this rule applied by hand; stating it once here makes it citable instead of re-derived.
- The support side of collision belongs near here: two records can agree in prose while their evidence disagrees, which is a different detector from content collision ([[collision-staleness-detection]]) with different blind spots.
- Unintegrated influx behind this segment (do not cite as warrant): `plan/INFLUX/synthesis/relata-methods-for-verisectorium-2026-08-05.md` §§2.2–2.6. The relata tree is the live authority; re-check with its own commands before operational claims.
