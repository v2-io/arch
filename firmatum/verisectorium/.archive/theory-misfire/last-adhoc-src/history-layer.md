---
slug: history-layer
type: form
depends:
  - collision-staleness-detection
  - integration-metabolism
  - terminology-store-anatomy
---

# History is a governed layer, not the place things go when deleted

*A corpus that replaces rather than accumulates is buying its readability with a history layer, and that layer runs under laws of its own — because the mechanism that keeps present-truth surfaces honest cannot operate on it at all.*

## The claim

Replacement is what keeps a corpus's difficulty proportional to its content instead of its past ([[integration-metabolism]]). It works only because there is somewhere for the past to go. That somewhere is not a leftover; it is a designed layer, and it needs its own governance for a specific reason: **an append-only record cannot collide.** Two present-truth claims about one subject can contradict, which is what makes staleness detectable ([[collision-staleness-detection]]); two history entries never can, because a later entry supersedes without making an earlier one false *as history*. So the history layer gets no self-correction for free, and every property that a claim corpus enforces structurally has to be arranged deliberately here.

Four things follow.

**(A) The layer stratifies, and the strata have different laws.**

- *Forward-going narrative* — appended, written in current vocabulary, describing what changed and why. It is the working half and the one everything else points at.
- *Frozen archaeology* — everything before some discipline changed. Nothing is appended to it and nothing in it is corrected. It is a different kind of object from the changelog, not an older part of the same one.
- *Record-grain events* — per-record, addressable, append-only: who set this, when, on what grounds. The rung most corpora do not have.

**(B) Freezing needs a reading rule, not just a date.** A frozen file is unusable if the reader has to know which vocabulary regime it was written in — and if the file is honest about being frozen, that regime is by construction *not* the current one. So the freeze carries an instruction. The estate's worked form is a banner at the head of the archaeology naming the changed vocabulary, giving the old-to-new mapping in a table, and stating flatly that everything below uses the old terms. One instance carries a class-renumbering table on both its frozen and its live history file, plus the general rule for a framework rename: in the frozen trees, read the old name as the new one.

That rule has an exception which is easy to miss and structurally necessary: **documents whose subject is the retired name keep it literal.** A name-transition decision record is *about* the old name; silently rewriting it destroys the record. The general form: a mechanical vocabulary sweep must exempt the documents that adjudicated the vocabulary, and it will not know to unless the exemption is declared.

**(C) The record-grain rung is the missing one, and its absence is invisible because the general instruments answer a different question.** The natural reflex is that version control already carries history. It does, indexed the way it serves: `git blame` answers about *lines in files*. The question a claim corpus actually asks is record-indexed — *who set this status, when, knowing what?* — and no amount of blame reconstructs it once a file has been renamed, reordered, or swept. Two further facts compound it: agents essentially never reach for version control's mining verbs unaided, and this corpus's own recency ordering is poisoned by rename sweeps, so the cheap proxy is unavailable exactly where it would be used.

The estate has the rung built in one place and not generalized: a terminology store where every naming decision is an append-only event file under the term's own slug — 160 events across 149 term directories, each naming the decider, the action, the timestamp and the reason, addressable by term ([[terminology-store-anatomy]]). Its README states the argument plainly: mutable status fields lose the history, and lose it *silently*, when overwritten. The contrast sits in the same repository: a 1,417-line changelog carrying much of the same class of information greppable but not addressable — you can find it if you already suspect it exists, which is the failure mode of every unindexed history.

**(D) A number stated in a durable surface is a historical fact wearing present truth's face.** Counts drift the moment the corpus is touched, so a surface that states its own size is asserting something that will be false shortly and will not announce it. The practiced repair is to make the number a pointer: give the command that computes it, and mark any quoted figure as a dated snapshot for orientation only. The same discipline applies wherever a measurement is load-bearing — carry the date and the method, and let a re-run supersede by collision rather than leaving the stale figure to be trusted.

**Two smaller disciplines belong here because nothing else claims them.**

A refuted rationale is *deleted*, not archived in place. **relata** (`~/src/arch/firmatum/relata/README.md` §"Storage form") removed a files-versus-sqlite trade study that had become entangled with an unsound framing, and replaced it with an explicit anti-reconstruction instruction — *"do not inherit a prior conclusion or reconstruct the argument from memory; weigh it fresh when it is next worked"* — plus a neutral statement of current state, so the deletion does not leave a vacuum that invites reconstruction. Worth holding against itself: that deletion **did not propagate** to two older stores in the same lineage, which still carry the trade study verbatim as a table. A deletion in one store of a copied lineage is not a deletion in the lineage.

And **records that predate their own governance need an honest re-grading path** — classified at the grade the current process *would have* assigned, *marked as retroactive*, never silently upgraded. The occasion is Joseph's, on a 2026-07-12 council entry tagged `:by us`: *"probably council before we had that as an option, more or less."* The claim attached to it — that no verified decision-record state machine in the surveyed prior art covers this lifecycle case — is a specific, checkable gap claim that has not been confirmed and should not be repeated as though it had.

## Strength & grounds

**A formulation assembled from one corpus's practice plus two mechanisms observed elsewhere in the estate; the stratification is a design argument, the record-grain claim is the best-evidenced part.**

First-hand this pass (2026-08-06, live trees): the forward/frozen split with its reading-rule banners and stated freeze date; the 160-event / 149-directory terminology store and its stated rationale; the relative sizes that make the history layer larger than every working tracker combined ([[tracking-layer-census]]). The vocabulary-sweep exemption for name-decision records is stated in the corpus's own orientation file and is quoted, not inferred.

Inherited at one remove and marked as such: the deleted-rationale instance and the counts-are-pointers norm are read from relata's live README by an earlier pass of this project and re-stated here **without my own re-reading of that file** — so the paths and quotations carry that pass's register, not mine. The re-grading obligation is Joseph's, recorded in a 2026-07-29 design dialog; until this segment it was named in a Working Note and **asserted by no segment**, which is what it is owed. It is asserted here as an *obligation the pattern should carry*, not as a practice anyone has been observed running — no instance in this estate re-grades retroactively, and the accompanying prior-art gap claim is unconfirmed.

What is genuinely unestablished: that stratifying into three earns its cost in a small corpus, and that the record-grain rung generalizes past naming decisions. The one working instance is on a store whose records are short, whose decisions are discrete, and whose events are trivially collision-free by filename — which is exactly the easy case. Whether the same shape survives on decisions whose grain is diffuse is the open question, and [[observable-crossings]] proposes the transfer without demonstrating it.

## Working Notes

- The measurement that would settle (C): count how often a record-grain history question is actually asked. If the answer is rarely, the missing rung is a theoretical gap rather than a live cost, and the changelog is adequate.
- **Owed and not carried here** (TODO R27's other half, mark withdrawn on verification): the *spatial* leg of the same argument — that for space, native iteration by grep beat the corpora's built indexes, because an editing agent invalidates its own index by construction. The temporal/spatial contrast is the entry's actual thesis and only its temporal half is above. Its named specimen is relata's per-record decision history, not the terminology store used here; both illustrate addressability, and substituting one for the other was the defect.
- Not carried, deliberately: what earns a changelog entry versus a working note. The exclusion boundary is stated from the other side in [[working-notes-sidecar]] (narration belongs here, not there); the positive rule for this layer is unwritten anywhere in the estate that I found.
- Housekeeping fact worth not re-discovering: at least one load-bearing document exists in two byte-identical homes in different repositories — a live collision surface the moment either is edited, and the kind of thing only a hand-written note catches.
- Adjacent: [[decision-records]] (a decision's own record is a history object with prospective fields), [[observable-crossings]] (a crossing event is record-grain history for a disposition rather than a claim), [[absence-as-structure]] (a graveyard index — *do not use this, use that* — is named absence in the history layer).
