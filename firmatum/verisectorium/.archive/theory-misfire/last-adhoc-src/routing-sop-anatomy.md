---
slug: routing-sop-anatomy
type: survey
depends: []
---

# The ASF routing SOPs, anatomized

*The estate's most-exercised process law — how audit findings and research spikes are dispositioned — laid out with its vocabularies, its scar record, and its two documents' relationship, as the material behind [[strengthen-before-routing]], [[adjudicator-not-confirmer]], [[honest-incompleteness-discharge]] and [[rule-grounds-and-posture]].*

## The corpus

Live at `~/src/arch/asf/doc/sop/`, read whole on 2026-08-06. Two documents carry the disposition discipline:

| File | Lines | Role | Recorded refinements |
|---|---|---|---|
| `audit.sop/routing.sop.md` | 407 | the shared core: what happens to a finding | **5** |
| `spikes.sop.md` | 355 | the spike-specific delta; defers everything shared | **10** |

Their relationship is the first thing worth recording, because it is a design decision stated with its reason: spike-routing and audit-routing are treated as **the same problem with two corpora**, and the spike document explicitly refuses to duplicate the shared core — it would *"fork a hard-won protocol so two copies could drift — the worse failure, because the no-go protocol is the part that must never drift."* Refinements discovered while exercising the spike corpus are written **into the audit document**, not into the spike one; two of the spike document's ten refinements say so explicitly and name it as the meta-stance working. That is a live instance of one corpus re-truthifying another's shared law without forking it, and the estate has no other example of it.

Three neighbouring SOPs complete the set and were read alongside: `audit.sop/de-novo.sop.md` (702 lines — the audit *walk*, whose priming discipline is already carried by [[priming-discipline]]), `multi-agent.sop.md` (74 lines — orchestration shapes), `git-hygiene.sop.md` (56 lines — commit granularity and the pre-spike seam).

## The vocabularies

Four separate closed vocabularies do the load-bearing work. Their sizes are the survey's point: this is what a disposition language looks like after roughly a year of exercise.

**Rule classification (4 tags, two axes).** Every rule in the routing document is tagged with its *groundedness* — why it exists — and the *posture* owed to it. `current ops` (provisional; revisit freely with a heads-up), `convention SOP` (coordination; arbitrary content, non-arbitrary consistency), `evolved ops` (hard-won; obey-first-then-ask, with the scar linked), `authoritative SOP` (mission-grounded; obey-first, question with front-line evidence). Tag frequencies in the routing document as of the read: authoritative 11, evolved 10, current ops 8, convention 4.

**Spike completion states (4).** *(A)* strengthened to the claim; *(B)* strengthened **past** the claim; *(C)* a no-go — a theorem that falsifies the claim and exposes the domain; *(D)* "strengthen failed" without a no-go and without exhaustive effort. The ordering is recorded as an observed frequency prior, explicitly marked as such and marked as awaiting data. (D) is declared *not a result* — "never a quiet landing; it is an alarm."

**Ghost-forms (4).** How a superseded prior claim is mentioned after a no-go lands: *(A)* its own no-go theorem in an appendix (most often); *(B)* a short no-go proof in the most relevant section; *(C)* disappearance to a changelog line (very rarely, and only when no one would attempt the approach again); *(D)* something else — "we cannot enumerate what we have not seen. If a ghost does not fit A–C, that is information; describe it and ask."

**Per-finding dispositions (13 named outcomes).** `resolved` · `resolved-by-strengthening` · `resolved-by-strengthening-then-no-go` · `correctly-rejected` · `architectural` · `subsumed-by-later-work` · `duplicate` · the four soft bands (`soft-polish` / `sentiment` / `considered-declined` / `research-seed`) · `process/instruction-feedback` · `actionable-open`. The spike corpus adds its own six-state set (`integrated-filed`, `integrated-misfiled`, `orphaned`, `correctly-superseded`, `archived`, `live-or-open`) and then, at Refinement 4, splits the last of those into five sub-dispositions because a single label was silently routing five different situations to the same place.

## The scar record

Both documents append dated **Refinements**, each stating what went wrong, what changed, and what transfers. Fifteen across the two. They are not a changelog: each is written as a lesson with a named body-signal, and several are explicitly *pre-emptive* — articulated from foresight before the failure bit, which the document notes is the cheapest kind to inherit.

Four worth having in front of you, because they are the mechanism-level evidence the claims stand on:

- **routing Refinement 1** — a fidelity review of a freshly-written correction, solicited *because* a fresh correction to an over-rotation is itself the highest-risk over-rotation candidate, found a genuine loosening in it. *"check the new filter externally before building on it — the conviction that the correction is clean is the same conviction the doc says fails."*
- **spikes Refinement 6** — the sharpest regression instrument turned out to be the *exclusion* pickaxe: `git log -S'<refuted form>'` returning **empty** proves the wrong form never entered canon, which is a stronger clean signal than "added then correctly deleted" and is immune to the rename sweeps that poison recency.
- **spikes Refinement 7** — the archive move *felt* like completion because a durable artifact shipped; it is the start. Navigator reconciliation was elevated into the completion criterion after a navigator entry was found asserting a spike "partially landed in `#X`" when `#X` was by then `status: false`.
- **spikes Refinement 10** — a catalog drifted for four months because the completion criterion named three navigators and not the fourth. Joseph's correction ran twice: first add the teeth, then, the same day, *remove* them — a binding-completeness registry would reimport exactly the friction the spike convention exists to forbid. The stated tell: *"when 'keep it honest' silently becomes 'keep it exhaustive.'"*

## The two calibrations recorded against the process itself

Both are unusual enough to record verbatim in effect, because they are the corpus disciplining its own confidence rather than its content:

1. **Peer-agent optimism is as unreliable as pessimism.** A cluster adjudication confidently predicted a strengthening "is standard textbook, should not fail"; the hard spike disconfirmed it into a no-go. The prediction is recorded *as disconfirmed*, in both the spike and the segment's notes, so it is not re-attempted on the strength of the optimism. The rule drawn: *run the hard spike; do not relay the optimism.*
2. **The convenience label is unreliable in both directions.** In a two-spike diagnostic pilot the index label was wrong in **both** cases and in **opposite** directions — understated for one, accurate for the other only by accidentally encoding an external block. The first-hand read was therefore budgeted as mandatory per slice rather than as a spot-check.

## Method & scope

All five SOP files read whole from the live tree on 2026-08-06; line counts, refinement counts and tag frequencies computed with `wc -l` and `grep -c` on the same read. Quotations are verbatim from those files. Enum contents are transcribed from the documents' own enumerations, not reconstructed.

The scope limit that matters: this is a survey of **what the documents say**, not of what the cycles did. Whether the disposition enum is used as written, whether the strengthen-first reflex fires at the claimed rate, and whether the frequency orderings recorded as priors match the record are all unmeasured here — and the documents themselves flag several of their orderings as current-ops-awaiting-data. Treat every frequency claim above as the corpus's own report of its practice.

## Working Notes

- The cheapest real measurement available: count dispositions actually recorded in `audits/STATUS.md` and the `pending-findings-*` ledgers by enum value. That converts the entire "the enum is exercised" assumption into a distribution, and would show which of the 13 states are dead letters.
- Not carried here and genuinely instance-specific: the LaTeX/lint mechanics, the directory-prefix six-digit ID convention, and the `AUDIT-WORKING-*` gold standing gate's contents (its *shape* is used by [[gates-need-destinations]]).
