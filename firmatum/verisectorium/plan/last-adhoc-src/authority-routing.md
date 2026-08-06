---
slug: authority-routing
type: form
depends:
  - warrant-over-authority
---

# Authority routes by admission test and degrades in named tiers

*A steward is a scarce, non-parallelizable resource; the corpus keeps him from becoming a bottleneck by defaulting-and-proceeding on everything that fails an explicit admission test, and by never letting the system resolve a genuine question silently when it cannot decide alone.*

## The claim

Every living corpus accumulates questions it cannot answer from inside itself. Two failure modes bracket the problem. Route too much to the steward and the queue becomes the critical path — work stalls behind a person who is not reading fast enough to clear it, and agents learn to ask rather than judge. Route too little and decisions get made in his name that were never his, which is the more expensive failure because it is silent: the next session inherits a fiction as its floor.

The pattern's answer has three parts.

**An admission test, stated positively, with default-and-proceed as the standing rule.** A decision *point* is not by itself the steward's; most have a sensible default that the lead takes. What earns admission is a property of the decision, not its difficulty: **irreversibility**, **authorial or publication voice** (things said in his name), **blast radius beyond the instance**, and **provenance doubt** — the "did this actually come from him?" case. Everything else defaults, and the close calls are recorded as defaulted rather than silently taken. Written the other way round — "escalate when unsure" — the queue fills with items whose real content is the agent's discomfort.

**Queue as pointers, not as a second copy of the work.** The steward's queue holds decision-*points*; the work items stay in their home trackers. A queue that duplicates context becomes a third tracker to keep in step, and the duplicate rots first. The unit of the queue is one item, decidable from its own entry — context, options, the lead's recommendation, and a pointer — so a batch of them can be answered in one sitting without the steward re-entering each subsystem.

**A degradation ladder, never silent resolution.** When the system reaches a choice it cannot make, it must degrade through *named* tiers rather than guessing: resolve automatically where the evidence clears the bar; ask interactively when a person is present; present a batch of choices when many are pending; emit a runnable report naming each waiting item, its reason, and the exact command that decides it; and only at the bottom leave the item in a spool. The bottom tier is the fallback, not the interface. Two rules make the ladder work: a wait state is asked *at the moment the context that makes it cheap to answer is still present* — a parked item is a question the system is asking, and its cost rises with every hour it waits — and the acts at a wait state stay distinguishable, since *re-present the stored decision*, *recompute the upstream stages*, and *decide* are routinely conflated and have different consequences.

**Delegated authority needs its own tier, and that tier must not launder.** Between "proposed" and "the steward decided" sits the real case: a recommendation adjudicated by a delegated agent under an explicit grant. Naming it records that the call was not made blindly in a corner while deliberately withholding the weight of *he instituted this* — so that if the truth later says otherwise, doing otherwise costs nothing. The load-bearing constraint is that such a tier **never upgrades the who-decided field**. Authority tags answer *who decided*; they are not a strength scale, and a measurement can stand while the verdict drawn from it remains unratified ([[warrant-over-authority]] is the schema-level statement of the same asymmetry).

**Where this bites hardest is at the end of a context.** Authority inflation is most tempting when the window is nearly full and tying off loose ends feels like diligence. A clean *undecided* left for the next mind costs less than a false *decided*, every time.

## Strength & grounds

**Heuristic, from three live sources read first-hand 2026-08-06.** The admission test, the pointers-only rule, the one-sitting brief format, and an explicit *lead will default-and-proceed* bucket are running in `~/src/arch/asf/JOSEPH-TODO.md` (with its brief set at `msc/decision-briefs-2026-07-15.md`). The delegated-grant tier and the who-decided/how-well-supported separation are law in `~/src/arch/vivarium/core/src/norm-decision-authority.md` (`status: exact` as process law), which records its own founding incident — a verdict marked closed in an onboarding file without the steward's adjudication — and states the end-of-context risk directly. The degradation ladder and the three distinguished acts are shipped in `~/src/arch/firmatum/relata` (README §§1–2), whose framing *a parked document is a question the system is asking* is the sharpest statement of the timing rule.

Three deployments, one estate, one steward: convergence of practice under a shared author, not independent corroboration. No instance has measured what routing volume actually costs, so the bottleneck argument is mechanism, not measurement.

## Working Notes

- Discharges TODO entry N19 (the ladder, and the three acts).
- **R42 is not discharged here, and a same-day mark claiming it was has been retracted.** R42 is about routing a *finding* through an integration report — a five-tag disposition vocabulary, a per-finding shape, an anchor a reader can resolve in seconds. That is a different mechanism from this segment's, and the two share only the word "routing." The confusion is worth recording rather than merely corrected: a slug's name colliding with an entry's vocabulary is exactly how an entry gets marked landed by a segment that does not carry it, and the queue's own admission rule ([[integration-metabolism]]) is the only thing standing in the way.
- Not carried: R47's `pending`-as-a-query claim — that wait states should be *computed* from event stores and gaps rather than maintained as a list. It is a genuine strengthening of the fourth tier above and belongs with the ch. 14 instruments material as much as here; left open deliberately rather than asserted, since the estate's own queues are all hand-maintained lists.
- Open, and the more interesting half: nothing here says which decisions are *genuinely* the steward's rather than merely conventionally reserved — the ch. 15 gap row names that question. The four admission criteria are one estate's answer, arrived at by accretion after specific failures; they have never been checked against a decision log to see how many entries they would have admitted.
- The grant itself is unexamined: what makes a delegation legible enough that an agent can tell whether a given call is inside it. Every source above assumes the grant is known; none records its form.
