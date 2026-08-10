---
slug: form-steward-valve
form: formulation
type-expected: formulation
status: discussion-grade
max: decided
state: [drafted]
depends: [claim-decision-surfacing]
---

# Formulation: The Steward Valve

The valve is the durable queue of genuinely-reserved steward calls — an index *of briefs*, never of pointers — fed by a forcing function coupled to something agents already do, pruned on resolution, with a re-open path; and its admission test is decision-type, not artifact-shape.

## Formal Expression

*[Formulation (valve)]*

1. **Admission: the four criteria, as a filter.** An item enters the valve when its resolution requires a decision the steward reserved: **irreversible** · **authoring-voice** · **cross-project blast-radius** · **provenance-check** ("did this actually come from the steward?"). Everything else defaults-and-proceeds. The criteria filter false escalations by forcing the WHY-IT'S-YOURS field to name one.[^census] The axis is *decision-type, never artifact-shape* — a humble file can carry a reserved call and a grand directory none.[^spikes-6]

2. **Each entry is one line plus a link to its brief** — and the brief is the thing the steward acts from. The eight-field schema: DECISION (one sentence, stated as a choice) · WHY IT'S YOURS (one criterion, named) · CONTEXT, RECONSTRUCTED (3–5 sentences assuming zero scrollback — the load-bearing field; *if it cannot be written in five sentences, that is itself signal: the decision is not ripe or needs splitting*) · OPTIONS with one-line consequences · RECOMMENDATION + CONFIDENCE (mandatory) · HONEST UNCERTAINTY · REVERSIBILITY + BLAST-RADIUS · POINTERS (last, supplementary, never the substance).[^census]
3. **Intake is a forcing function, not a convention.** The valve's one recorded predecessor failed *at the intake seam only* — built well, it leaked within four weeks because intake depended on an agent remembering a mirror step; the items that reached it were processed correctly.[^census] So intake couples to cycle-close (no cycle commits with an unbriefed fork it produced) and to a mechanical check (grep the trackers for reserved-markers lacking valve entries — make the leak a loud signal instead of silent staleness).
4. **Batched gates get one standing brief with a default recommendation** — a decide-with-steward session over N accumulated items is a *ratify/redirect* of a pre-assembled recommendation, never a from-scratch deliberation over the pile.[^census]
5. **The re-open path.** A ratified decision reopened by later evidence re-enters the valve with a brief stating *what changed since he last decided*.[^census]
6. **Prune on resolution.** Resolved items leave the valve (their record lands in the decision layer); the valve is a live queue, and a stale entry is a label lying about status.

*[Anti-patterns, named from the record]*

- **Pointer-regression**: a brief degrading back into "see §SP-30" reintroduces the disease — the predecessor was pointer-only *by design* (an anti-fork choice) and that design decision is what made it unable to be the thing the steward acts from. *A stale mirror is worse than absent, because it looks authoritative.*[^census]
- **The wall**: a rich, correct working doc is *"unreadable as a decision surface"* — distillation into the brief is the work, not a lossy courtesy.[^census]
- **The missing recommendation**: escalation without a recommendation does half the work and hands back the deliberation-space the agent was positioned to narrow.

## Epistemic Status

Formulation — a design, chosen and defended; max attainable `decided`, current status `discussion-grade`: the design is synthesized from a verified failure anatomy plus existence proofs, but *this assembled form has never run*. What is well-evidenced: the failure modes it answers (the 0-of-6 intake leak, mechanically verified; the pointer-only insufficiency, steward-attested; the label-drift between valve and home trackers) and the capability (the estate's ad-hoc decision packages already exhibit the brief at full quality — the standard slot and the routing are what is missing). What is honestly untested: the forcing function (the census's own caveat — the mechanical check might flag defaults-with-flags as escalations, and its false-positive rate is unknown), and whether cycle-close coupling survives contact with real cycle pressure. Single-estate lineage throughout. Evidence-action: run it — this corpus's own parked decisions are the natural pilot batch.

## Discussion

**Why the valve is an organ-part and not a tracker.** Under [[claim-decision-surfacing]]'s reduction, the valve is the living collection's interface to its slowest, most reserved clock — the steward's judgment — and its design goal is stated in the source as lived relief: a decision arriving as *"here is the fork, here is the context reconstructed assuming you have zero scrollback, here is what I would do and why, and here is what I could not verify"* — actionable in minutes without reloading a session.[^census] Every property above serves that: briefs because pointers make him do the assembly; forcing functions because memory-dependent intake measurably dries up; default recommendations on batches because 22 from-scratch deliberations never get scheduled and one ratification does.

**The reservation discipline and the surfacing discipline are different muscles.** The estate's strongest-running related practice was *reservation* — "reserved for Joseph" carried across cycles with precise reopen notes inside a portfolio doc — while nothing *lifted* reservations out of the 115KB doc into anything he would see.[^census] The valve is precisely that missing lift layer; it does not replace home-tracker reservation, it mirrors it as briefs, and the mechanical check is what keeps mirror and home from drifting (the predecessor's third failure).

**Composition with this corpus.** PRACTICA's waiting-joseph states and parked-decisions block are the proto-valve here; [[form-pending-surface]] is the general queryable-unfinished-work surface of which the valve is the reserved-decisions specialization; and the norm's report-don't-reassure clause governs the brief's register — the recommendation is owed *because* it is the agent's honest read, not despite deference.

## References

[^census]: `~/src/arch/asf/msc/meta-process-review-2026-07-07/07-decision-routing-and-joseph-blockers-findings.md` (read whole 2026-08-09): the valve predecessor's design and verified failure anatomy (§2), the brief schema and design goal (§6a), intake forcing functions (§6c), batched gates and the re-open path (§6d), anti-patterns (§6e), the honest-uncertainty caveat on the mechanical check (§6f), and the reservation-vs-surfacing observation (§3, MP-07.6).
[^spikes-6]: `~/src/arch/asf/doc/sop/spikes.sop.md` §6 (pilot 023198, ratified 2026-05-17): "route to the Joseph batch anything whose resolution requires a decision Joseph reserved, file or dir" — the decision-type-not-artifact-shape criterion.

## Working Notes

- Frontmatter schema provisional pending the epistemology decision.
- Open (design): whether the brief is a file per decision (`decisions/D-<n>-<slug>.md`-shaped) or an entry in a decisions ledger with the valve as a view over it — the latter composes better with [[form-decision-records]]' schema family and [[claim-outline-as-view]]; undecided until form-decision-records drafts.
- Open: the four criteria are the estate's; the kit should let a deployment declare its own reserved-decision criteria (a solo-steward instance and a council-governed one differ here) — one axis of [[form-enforcement-profile]].
- Pilot noted in Epistemic Status: convert this corpus's parked decisions (wikilink form; claim- prefix revisit; instrumenta collision; the norm-family rendering review) into brief-form entries as the first live run.
