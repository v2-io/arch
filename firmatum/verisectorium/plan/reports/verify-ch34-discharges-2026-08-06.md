# Verify: six ch.3/ch.4 DISCHARGED marks, 2026-08-06

Read against the delete-test (`plan/ONTOLOGY.un` §integrated-is-disposable): would the TODO row's *entry text*, if deleted right now, still be fully recoverable from the segment body it points to? Not "is the theme present" — is every clause landed at the strength the entry states it.

## Verdict summary

| Entry | Verdict |
|---|---|
| A6 → tracking-altitudes | **Sound.** Full landing. |
| R27 → history-layer | **Not sound — partial, and part of what landed is a substituted specimen.** |
| R35 → gates-need-destinations | **Mostly sound — core claim solid, cross-reference and one nuance missing.** |
| R-T1 → history-layer | **Partial.** Core rule lands; the evidentiary grounding (quote, date-tag, ADR-gap novelty) does not. |
| N12 → sidecar-conventions | **Partial — attribution stripped.** Claim lands; the two named specimens (comproprium, vivarium/ETHICS.md/LEXICON) and the ch.11 forward-link are gone. |
| N13 → sidecar-conventions | **Mostly sound — same attribution loss as N12, smaller.** |

Two of six (R27, N12) should not have been marked discharged as written. R35, R-T1, N13 are landings I'd call defensible but not clean — worth a decision on whether "the claim is stated" is the bar or "everything the entry says" is, since the entries were written with specific citations as part of the claim, not decoration.

## A6 → [[tracking-altitudes]] — SOUND

Entry: notes store as unowned decision queue, ASF's 2026-07-07 review finding (possibly-vacuous leakage bound, hard-ceiling schema convention) parked in per-segment Working Notes, no queue semantics/owner, quote "load-bearing theory-correctness items are parked where nothing watches them."

Segment (¶4, "Each altitude needs a consumer…") carries the quote verbatim, both named items ("a possibly-vacuous bound, a schema convention"), and the no-queue-semantics/no-owner framing. Nothing in the entry is missing at the source. Good discharge.

## R27 → [[history-layer]] — NOT SOUND

Entry has three components: (1) the spatial half — "For space, native iteration (grep) is natural and beat the built indexes"; (2) the temporal half — git-blame-is-serving-indexed-not-record-indexed, agents don't reach for git's mining verbs; (3) the specific contrast — **relata's** per-record decision history (addressable) vs a 1,405-line changelog (greppable, not addressable), with a commit-subject convention as the carving force.

Segment §(C) lands (2) cleanly — the git-blame framing and "agents essentially never reach for version control's mining verbs unaided" are both there, close to verbatim.

But: `grep -n "relata\|commit-subject\|spatial\|native iteration" plan/last-adhoc-src/history-layer.md` returns nothing. None of (1), the relata attribution, or the commit-subject-convention clause exist in the segment. What the segment substitutes instead is a *different* comparison — the asf terminology store (160 events / 149 directories) vs the same 1,405-line changelog. I checked `terminology-store-anatomy.md`: that's `asf/terminology/`, a genuinely different object from relata (confirmed against R26's own text two rows up, which names relata's verb topology separately from the terminology stores). So this isn't relata's history re-described in different words — it's a different specimen filling the same rhetorical slot, and the entry's actual named source is uncited anywhere in the segment.

The "split is diagnostic" framing — the entry's actual thesis, that temporal and spatial questions want different instruments — is entirely absent; only the temporal half survived. I would not consider this row honestly discharged: recommend reverting the mark, or adding a paragraph carrying the spatial contrast and the relata/commit-subject specifics (possibly citing relata directly, since `terminology-store-anatomy.md` already exists as its own segment and isn't a stand-in for relata).

## R35 → [[gates-need-destinations]] — mostly sound

Segment §(E) is close to verbatim on the core mechanism (independent vs correlated channels, "a schema constraint, a linter rule, and a CI job that all read the same declaration… buy approximately one channel"). Missing: the entry's explicit "with saturation under shared persistent bias" nuance (not fatal — arguably implied by "correlated"), and the cross-reference to `[[evidence-ledgers]]` as the theory-side twin — `grep -n "evidence-ledgers"` on the segment returns nothing. The entry frames this connection as part of the claim ("This is the theory-side twin of…"), not incidental; a reader who deletes the TODO row loses that pointer. Recommend adding the cross-reference to §(E) or its Working Notes; the core substance doesn't need rework.

## R-T1 → [[history-layer]] — partial

Segment (¶ "Two smaller disciplines…", second sentence) states the rule correctly: "records that predate their own governance need an honest re-grading path — classified at the grade the current process would have assigned, marked as retroactive, never silently upgraded." That's the entry's operative content and it's there.

Missing: the specific grounding the entry cites as part of the claim — Joseph's 2026-07-12 `:by us`-tagged quote ("probably council before we had that as an option, more or less"), and "the source notes no verified ADR state machine covers this lifecycle case, which if it holds is a second small novelty beside prospective falsification." The segment's own Strength & Grounds section is candid about this gap — it says the re-grading obligation is "carried above as an obligation named, not as a practice observed" and attributes it to "a design dialog," not the specific quote. That's honest about the segment's own limits, but it means the entry's evidentiary payload (the quote, the date, the ADR-gap novelty claim) is not recoverable if the TODO row disappears. Whether that's an acceptable strength-labeled abstraction or an actual gap is a judgment call; I'd lean toward flagging it rather than passing it silently, since "asserted by no segment anywhere, including this one" is a strong admission sitting right next to a DISCHARGED mark.

## N12 → [[sidecar-conventions]] — partial, attribution stripped

Segment's "half that governs non-atom files" section states both rules the entry claims (inherit-by-reference; the vivarium ETHICS.md failure specimen) close to verbatim in substance — the quotes ("ported by reference, not restated…", "used a name the dictionary had retired four days earlier, invented two access channels…") match.

But `grep -n "comproprium\|vivarium\|ETHICS.md\|LEXICON\|ch. 11" plan/last-adhoc-src/sidecar-conventions.md` returns nothing. The segment anonymizes both named sources to "one instance" / "its sibling," and drops the entry's closing clause "This is directly the shape ch. 11's generator would emit" — no ch.11 pointer anywhere. If the value of this entry included *which* corpora exhibit the pattern (useful for anyone wanting to go re-verify or extend it) and the forward-link to the ch.11 generator work, that's gone under the delete-test. This is the same shape of loss as R27 — real claim content survives, but the citations that let a reader independently verify or follow up do not.

## N13 → [[sidecar-conventions]] — mostly sound, same smaller loss

Segment states the rule and the quote ("'It is only a working document' is not an exemption") accurately. Missing: the "vivarium" attribution and the specific section numbers (§§1–4 govern segments, §§5–6 govern everything) — the segment paraphrases as "the early sections… the later ones" without naming which corpus or which sections. Smaller loss than N12 because the entry's core claim doesn't depend on the section numbers the way N12's depends on knowing where to look next (ch.11).

## The multi-row-entry question you flagged

I don't think "discharged once the content is stated somewhere citable, other rows cite it" is unreasonable as a policy, but two of your six specimens (R27, N12) show the actual risk of that policy: when the *landing* segment loses the entry's specific attributions in the name of "stating the claim," and other rows are told to cite it, they inherit the same loss — a second row citing sidecar-conventions for the N12 content will not be able to name comproprium or vivarium either, because the information isn't there to cite. That's not an argument against your policy in general — it's an argument that "content lands" has to mean the whole entry, citations included, not just the extractable thesis. I'd resolve this by fixing R27 and N12 rather than changing the policy.

## Parallel-batch check (A5, N15, N16, V10)

No collision. tracking-altitudes.md independently carries content matching both A6 and A5's "worked layered-navigator" specimen (six auditor-hidden / three auditor-safe) and also, incidentally, N15's steward-checklist-verbatim specimen (¶ "A last, smaller move…", matches N15's `processing-flow.md` description near-verbatim) — but A5 and N15 were marked discharged to `steward-surfaces`/`tracking-altitudes`+`steward-surfaces` by the other batch, not double-marked by you, so this is fine as executed. Worth a note to the other batch, not to you: N15 is stated in **both** tracking-altitudes and steward-surfaces now; not a defect, but worth knowing before someone assumes tracking-altitudes doesn't cover it.

One more thing outside your six, surfaced incidentally: `gates-need-destinations.md` §(B) already states V10's entire thesis and its six-checks specimen near-verbatim (act-vs-disposition, "attaches to all of them equally, which is to say none," the six-checks-returned-clean story) — but V10 was marked discharged to `role-activation` instead. Not a conflict, just means the same content now exists in two segments under two framings; might be worth a cross-reference from one to the other so a reader of either finds the twin.

## On your eight declined entries (R22, N14, R21, R17, R46, R31, R-T2, R-T3)

Spot-checked N14 against history-layer.md since it's the nearest miss: part (b) of N14 (deleting a refuted rationale, the anti-reconstruction instruction, the non-propagation to older stores) is **already stated in history-layer.md** ¶"Two smaller disciplines…", first sentence — close to verbatim, including the "did not propagate to two older stores" detail. But it's anonymized ("one live instance") rather than attributed to relata, and part (a) of N14 (the `[private-repo assumption]` greppable marker) is not in the segment at all. So declining to mark N14 discharged was the right call — it's genuinely partial, same failure shape as N12/R27 (attribution stripped, and here also missing half the entry) — but worth knowing the *other* half is already sitting there unattributed, so whoever finishes N14 should attribute-and-complete rather than re-derive from scratch. I did not check the other seven declined entries in similar depth; flagging that as unfinished on my end rather than reporting negative findings I don't have.
