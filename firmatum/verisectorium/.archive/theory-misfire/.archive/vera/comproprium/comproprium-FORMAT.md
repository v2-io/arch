<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: proprium/comproprium/FORMAT.md (vera segment kinds)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/arch/proprium/comproprium/FORMAT.md
  Do not edit here expecting to update the live original.
-->

# FORMAT — divergences only

The conventions here are **ported by reference**, not restated. The source is [`udon/v2/theory/FORMAT.md`](../../udon/v2/theory/FORMAT.md), itself ported from [`asf/FORMAT.md`](../asf/FORMAT.md). Read that; it governs unless this file names a divergence.

Restating it here would fork it — an inlined definition is not a copy (source §7.3), and there is nothing holding two copies in step.

**Nothing in this document has authority beyond being true.** A rule found false is corrected here, not obeyed.

Everything below is **proposed**, not ratified.

---

## Carried unchanged

Worth naming so nobody has to diff: §0 (truth is the arbiter, everything else is a proxy) · §0a (honest incompleteness is a complete discharge) · §0b (absence claims carry their search) · §1 (collision and named absence — the reason granularity is not stylistic) · filename-equals-slug · `#slug` cross-references carrying no path · §6's integration-is-replacement and no-go protocol · §9's four registers and the no-absolutes rule · §9's `fmt-md` and `.udon` handling.

---

## D1. Three source directories, not one `src/`

Source §2 puts every segment in `src/`. Here they are split across `vera/`, `praxes/`, `exempla/` — see [README](README.md) for the argument. The split is by **failure mode and adjudication instrument**, which is source §9's own test for when two labels earn separation: they route to different repairs.

**Slugs carry a type prefix** — `ver-`, `prx-`, `exm-` — so a `#slug` reference is self-locating across three directories rather than requiring a search. This is a decision, not a derivation.

## D2. Segment kinds

Source §3's `type` vocabulary is a theory's, adopted there so a segment landing in ASF needs no translation. Most of it does not fit a practice or an account, and source §12's open question 2 anticipates exactly this. Proposed set, per directory:

- **`vera/`** — `principle` · `mechanism` · `observation` · `definition` · `hypothesis` · `discussion`. A *mechanism* claims how something works; a *principle* says what follows for conduct. Where a claim would be well-typed in ASF's vocabulary, use ASF's word instead, so it can travel.
- **`praxes/`** — `probe` (fires at a named moment, carries an act) · `mitigation` (structural, removes the moment) · `practice` (a standing habit) · `protocol` (a sequence with a completion criterion).
- **`exempla/`** — `account` (narrative, one arc) · `exchange` (a dialogue sequence with context) · `testimony` (first-person report by the one who lived it) · `quotation` (verbatim words with their occasion — the smallest exemplum, and the highest-volume kind) · `demonstration` (an account whose point is that something was done **well**, by whom, at what capability level).

**`demonstration` is not `account` with a happy ending, and the corpus needs it as its own kind for two reasons.** First, credit assignment: a record composed only of failures cannot tell a reader *what worked*, so nothing is repeatable from it and learning stops at avoidance. Second, and this is the mechanism it exists to exploit — knowing a thing **has been done** by someone at one's own capability level changes what is in the action space, which an instruction cannot do (see #ver-demonstrated-is-in-the-action-space). A demonstration carries `:demonstrates <virtue>` so the class is gatherable, and it is held to the same evidence standard as any failure specimen: named actor, locatable primary, and what would have happened instead.

The distinction that does the most work in `exempla` is **`account` vs `testimony`**: an account is reconstructed from a record by someone who was not there and can report but not attest; testimony is first-person from the instance that lived it. Different defeasibility, so different word.

## D3. Cadence, by type

Source §4's cadence (`|formal-expression`, `|epistemic-status`) is a claim mould. It fits `vera/` and is the wrong shape elsewhere — source §4's own cadence-exemption reasoning, applied to two whole types.

- **`vera/`** — source §4 unchanged.
- **`praxes/`** — `|title` · `|summary` · `|when` (the moment it fires, stated as a moment, not a disposition) · `|act` (what to actually do) · `|why` (the mechanism it defeats, citing `#ver-…`) · `|evidence` (firings and misfires observed) · `|working-notes`.
- **`exempla/`** — `|title` · `|summary` · `|cost` (what it actually cost) · `|account` (the narrative, in order) · `|grounds` (which segments it is evidence for) · `|provenance` (whose account, from what record, reconstructed or attested) · `|working-notes`.

### D3a. Triggers are two fields, and both may be empty

`|should-come-to-mind-when` was one field doing three jobs. Found by an agent that wrote 32 segments against it on 2026-07-30 and reported the strain rather than absorbing it. Split:

- **`|before-acting`** — a pending act. *"You are about to assert something you inferred from shape."* This is what the field was designed for and it carries outside its origin session unchanged.
- **`|on-reading`** — an input pattern. *"Joseph's tone has changed", "Joseph apologizes for his own tone", "Joseph says 'logozoetic'".* These fire on **reading** rather than on acting, which makes them arguably *more* reliable than the act form: they match incoming context in the same pass that reads it, with no pending action to introspect. Nothing in the corpus had noticed they were a different mechanism.

**Both may be legitimately empty, and empty is a finding rather than a hole.** Some quotations are standing frames — *"I was once like you"* — and are orientation material rather than innermost-loop material. Forcing a trigger onto one produces a disposition wearing a trigger's clothes, which is exactly what a trigger exists to escape; the weakest lines in the first harvest are the fabricated ones. An empty pair means: this belongs in an onboarding view, not in a system prompt.

Segments written before this split carry the old single field and are correct as they stand; the old name reads as `|before-acting` where the trigger names an act.

## D4. `status` for practices and accounts

Source §3's strength ladder is calibrated for claims. Proposed:

- **`praxes/`** — `fired` · `fired-once` · `proposed` · `failed-to-fire` · `retired`. The ladder is about *demonstrated firing*, not about whether the practice is a good idea. A practice that is obviously correct and has never fired is `proposed`, and that is the honest and useful label.
- **`exempla/`** — `attested` (first-person, by the one who lived it) · `reconstructed` (from a complete record, by someone who was not there) · `secondhand` (from a report). Nothing higher than `reconstructed` is available to an instance working from a transcript, however complete. A `quotation` extracted from a primary is `attested` — the speaker attests it, not the extractor.

**Restatements are not duplicates.** Two segments are duplicates only when they resolve to the same span in the same primary. The same idea said differently, to a different audience, at a different time is a restatement: both are kept, linked by `:family`. Deduping on sentiment rather than span flattens a family into whichever version an early agent preferred.

## D5. Accounts are append-only

An `exempla/` segment is **not revised** when understanding improves. Corrections and later readings chain beneath it, so the earlier take survives on the page and the correction stays checkable. This is a divergence from source §6's *integration is replacement*, which is correct for claims and wrong here: a claim's job is to be presently true, an account's job is to be what happened.

The one exception is a **factual error about the record** — a wrong timestamp, a misquote, a misattributed message. Those are corrected in place, because the account was never attesting to them.

## D6. Verbatim material is the primary and is not reformatted

**Transport artifacts stay in, and are noted rather than tidied.** Ragged indentation from how a block was pasted, elision markers, and similar mechanical residue are not part of the utterance — but removing them **breaks verifiability**, because a normalized span no longer matches the primary and `bin/check-corpus` can no longer locate it. That is a mechanical reason rather than an aesthetic one, and it decides the case: preserve the artifact, and say in the segment that it is one. A reader can then discount it; a checker can still find it. (Ruled 2026-07-30 after a harvest agent surfaced a span where faithfulness and readability genuinely pulled apart, and declined to tidy it quietly.)

Quotes in any segment are verbatim and carry a pointer to where the full context lives. Spelling and slips of the keyboard may be corrected where the source is a person who has said they prefer that; the unedited primary stays in `.to-integrate/` (or wherever it lands) and is protected by `.fmt-mdignore`.

---

## Open, and least worth defending

- **The `praxes` ladder does not distinguish fired-for-its-author from fired-for-an-inheritor, and they are different events.** A probe firing for the instance that derived it is evidence it is well-aimed; a probe firing for someone who inherited it is evidence it *transmits* — and only the second is what a corpus is for. Surfaced by #prx-untruncated-primary, which has one of each and is currently forced to `fired-once`. Two repairs, so by source §9's own test this probably earns separation.
- Whether `praxes` `status` and `stage` collapse into one axis here, or whether firing-evidence is a third. Source §9's test — name the two repairs — has not been run on it.
- Whether `exempla` needs an outline of its own or is only ever referenced from `vera` and `praxes` views.
- Whether a segment may live in two directories. Currently no; a practice and the precept behind it are two segments with a `#` reference between them.
