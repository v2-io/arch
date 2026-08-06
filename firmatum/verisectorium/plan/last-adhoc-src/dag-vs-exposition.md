---
slug: dag-vs-exposition
type: form
depends:
  - dependency-order-tension
  - experiential-reading
---

# A reader who repairs a gap silently cannot report it

*The divergence between logical order and reading order is checkable by machine only where dependencies are declared; everywhere else, the only instrument is a reader walking the exposition in its own order — and that instrument works only if the reader is forbidden to fill gaps from context.*

## The claim

[[dependency-order-tension]] establishes that reading order and dependency order legitimately diverge and that the divergence must be declared to stay checkable. That contract governs the part of the problem a linter can see: a record whose declared prerequisites appear later in the view. It leaves the pedagogically decisive part untouched, because **exposition can fail in ways no declaration captures** — a record that is formally well-founded and still unreadable in position, a step the prose does not carry, a term used before it means anything.

Those failures have exactly one instrument: **a reader walking the view in its own order and reporting where the walk broke.** And that instrument is destroyed by the reader's most reflexive competence.

**Silent self-repair is the failure mode.** A capable reader who meets an unsupported step does not stall. It reconstructs the missing piece from adjacent context, from prior knowledge, from what the corpus obviously means — and proceeds, correctly, having repaired a defect it did not notice repairing. From inside, nothing happened; the reading felt smooth. Every such repair is a defect the exposition keeps. This is why comprehension is not evidence of good exposition and why fluent readers are poor exposition testers *unless* the reading is arranged so that repairs become visible.

Three arrangements make them visible, and they are cheap:

- **Walk the declared order and do not re-order.** Jumping ahead to fetch a missing prerequisite fixes the reader's problem and erases the corpus's. Reading in position, incompletely, with the gap noted, is what produces the finding — the incomplete absorption *is* the data.
- **Treat inaccessibility in position as a finding rather than as a personal failure.** A record that does not crystallize where it sits is reporting one of three things: it leans on something it did not declare, its position is wrong, or it does not stand on its own. Which of the three is the reviewer's question; that it happened is the reader's contribution.
- **Notice the urge to reach for the reasoning trail.** Wanting the history, the working notes, or the source discussion *before* reading a record is itself a signal that the record is not carrying itself. The urge is diagnostic and is lost the moment it is satisfied ([[priming-discipline]] governs the ordering; here the point is that the urge is *data*, not just a temptation).

**The reading order is therefore two things at once: a discipline for the reader and a claim under test.** A view asserting that its order is walkable is a falsifiable claim about the corpus, and the walk is its test. That framing is what converts an ordinary read-through into a measurement — and it depends entirely on serial reading ([[experiential-reading]]), because a reader holding the whole corpus in view has already repaired every gap in advance and has nothing left to report.

**One standing exception, and it is pedagogical rather than structural.** Supporting material placed downstream for the reader's sake ([[appendix-placement]]) *should* be read at the point it is first invoked, not deferred to the end — verifying a proof while the result that needs it is still fresh is the whole reason the material exists. Deferring it wastes it. The exception is worth naming because it is the one case where following the view's order literally is the wrong move, and a reader who has been told "do not re-order" will otherwise defer it.

## Strength & grounds

**Heuristic, one live source read first-hand 2026-08-06.** All of the above is running as the reading discipline in `~/src/arch/asf/doc/sop/audit.sop/de-novo.sop.md` §§4.2–4.3: the view's row order is named as *"a load-bearing claim"* under test; an undeclared backward dependency is a critical finding; the reader is told to continue in position with the gap noted and explicitly *not* to back up, since *"silently jumping forward defeats the audit"*; a record that fails to crystallize in position is itself a finding with the three named diagnoses; the spoiler-seeking urge is named as *"often a finding waiting to happen"*; and the supporting-material exception is stated with its reason.

That source is written for auditing one corpus. The generalization — that this is the only available instrument for the class of exposition defect a declaration cannot capture — is this segment's, and it is an argument from mechanism, not a measurement. **No corpus here has recorded what a walk found**, so there is no evidence about yield: how many defects this surfaces, or whether the found ones matter. Single estate, single protocol.

## Working Notes

- The cheap experiment nobody has run: two readers walk the same view, one serial and one having read the whole thing first, and their gap reports are compared. The prediction is stark enough to be worth testing — the batch reader should report near zero.
- Silent self-repair predicts an asymmetry worth checking: the more capable the reader, the fewer exposition defects it finds, up to the point where it stops needing the exposition at all. If true, the corpus's best readers are its worst pedagogy instruments, and exposition quality would have to be tested by its weakest legitimate consumer.
- Not carried here and owed to the ch. 9 gap: what a corpus *does* with a gap report — whether the repair is prose in the record, a new bridging record, or a re-ordering. The three have different costs and nothing adjudicates.
