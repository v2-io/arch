# Fresh-reader verification: the seven tribunal segments (2026-08-05)

*Read cold, as a reader landing on each via wikilink with nothing else loaded, then cross-checked against `ONTOLOGY.un`'s split test and `OUTLINE.md`'s promised rows. Verdicts first, evidence below.*

## Verdicts at a glance

| File | Clears the Joseph bar? | Stands alone? | Split landed? | Verdict |
|---|---|---|---|---|
| `decision-records.md` | Yes, with one stumble | Yes | Yes (vs `rationale-capture-survey`) | Ship, fix one sentence |
| `warrant-over-authority.md` | Yes | Yes | Yes (vs `authority-flag-specimen`) | Ship as-is |
| `asked-and-answered.md` | Yes | Yes | Yes (vs `authority-flag-specimen`) | Ship as-is |
| `tribunal-record.md` | Mostly, one jargon stumble + scope question | Yes | Yes (vs both appendices) | Ship, consider a split |
| `tribunal-strand-survey.md` | N/A (appendix) | Yes | — | Ship, but **has no outline row** |
| `rationale-capture-survey.md` | N/A (appendix) | Yes | — | Ship, but **has no outline row** |
| `authority-flag-specimen.md` | N/A (appendix) | Yes | — | Ship, but **has no outline row** |

**The one finding I'd fix before anything else**: three of your seven files have no row anywhere on `OUTLINE.md` — not even a `proposed` one. `ONTOLOGY.un` treats the row as how a slug gets identity on this outline, and your four claim files `depends:` on exactly these three. That's not a readability problem, it's the pattern's own law not yet satisfied for its own material.

---

## The bookkeeping gap: no rows for three appendix files

I grepped `OUTLINE.md` for `tribunal-strand-survey`, `rationale-capture-survey`, and `authority-flag-specimen` — zero hits, in the Appendices table or anywhere else. The existing Appendices table lists seven specimens/surveys (`provenance-rot-specimen`, `ladder-never-fired`, `type-vocabulary-locality`, `view-genres`, `outline-skipping-failure`, `lost-update-hazard`, `coupling-confounding`); your three new appendix files aren't among them.

This isn't pedantry — it's the exact failure `ONTOLOGY.un` names: *"a dep on a slug with no row anywhere is an error."* `decision-records` and `tribunal-record` both `depends: [rationale-capture-survey]`; `tribunal-record` also depends on `tribunal-strand-survey`; `warrant-over-authority` and `asked-and-answered` both depend on `authority-flag-specimen`. Four drafted claims currently point at ungrounded identities by your own local law.

Also worth a pass: all four chapter-level rows (`decision-records`, `warrant-over-authority`, `asked-and-answered` in ch. 3; `tribunal-record` in ch. 8) still show `State = proposed` and an empty `Type` column, even though all four files are drafted with `type: form` in frontmatter. `ONTOLOGY.un` says Type is "filled at drafting time from the segment's own declaration" — that hasn't happened yet on the outline side.

Neither of these is a prose problem. Both are quick, mechanical fixes: add three appendix rows, flip four State values to `drafted`, copy `form` into four Type cells.

---

## Per-pair split verdict (the actual test from ONTOLOGY.un)

*"the claim is understandable without the appendix; the appendix carries the verification load without the claim."*

**`decision-records` / `rationale-capture-survey`** — clean split. The claim states its four requirements and argues them without needing a single fact from the survey; it cites the survey only for "this part is inherited, not invented" and names what the survey does and doesn't establish (`"the novel column is unverified"`). The survey stands completely alone as a literature account with its own verification register. This pair matches the target exemplar's shape well.

**`warrant-over-authority` / `authority-flag-specimen`** — clean split, and actually the strongest pair of the seven. The claim reproduces the two load-bearing quotes itself, so a reader never has to leave it to feel the argument land; the appendix gives the full five-finding narrative with dates, method, and its own honest "testimonial, n=1" scope note. Neither depends on the other for comprehension.

**`asked-and-answered` / `authority-flag-specimen`** — same specimen grounding two different claims (see over-production note below), but the split itself works: the claim states its own mechanism (repeat-billing under turnover) and only borrows one sentence of the specimen for texture.

**`tribunal-record` / `tribunal-strand-survey` + `rationale-capture-survey`** — split works, but it's carrying more weight than the others. `tribunal-record` explains the role-set, the record-as-product distinction, and the Gate-2 thin form entirely in its own words — a reader never needs the strand survey to follow the argument. Where it leans on `rationale-capture-survey` (the capture-problem escape), it states the borrowed claim's strength honestly rather than assuming it. So the split test passes. But see the scope note below — this segment may be doing three jobs.

---

## Where a cold reader actually stumbles

**`decision-records.md`**, this sentence:

> "This is the general separation test — *two labels earn separation where they route to different repairs* — applied at the governance layer."

This is the one place across all seven where a fresh reader hits a wall. It's phrased as *reminding* the reader of an already-established principle ("the general separation test") that this segment never introduces and that no `depends:` link supplies. Cold, it reads like a dropped reference to a segment that doesn't exist yet (maybe it's meant to route to a future ch. 1 or ch. 8 segment?). Either cut the "general…test" framing and just state the two-repairs point directly, or — if this really is meant to gesture at a not-yet-written principle — say so ("we'd guess this generalizes; no segment states it yet") rather than asserting it as settled vocabulary.

**`tribunal-record.md`**, this phrase:

> "The role-set is not arbitrary and not merely 'more perspectives.' It is the convergent-lock rule institutionalized as an org chart..."

Same shape of stumble: "the convergent-lock rule" is invoked as known vocabulary. It's explained one clause later ("agreement between two channels is worth something only if the channels can fail independently"), which mostly rescues it — a patient reader recovers within the same sentence — but the label itself is never grounded to anything, and I couldn't find a `[[convergent-lock]]`-shaped segment anywhere in `last-adhoc-src/` to route it to. If that segment doesn't exist yet, consider dropping the name and keeping only the explanatory clause, which already carries the whole point on its own.

Everything else I read cold landed without a fall-off point — including the denser passages (SEURAT, RATSpeak, the Y-Statement template in `rationale-capture-survey`; the four-strand table in `tribunal-strand-survey`) which are appendix-grade and correctly don't owe plain-language treatment the way the four claims do.

---

## Standalone check

All seven pass the "no other segment loaded" test in the sense that matters — none of them requires you to have *already read* its appendix or its claim to follow the argument being made in front of you. The two stumbles above are jargon-drops, not appendix-dependency; a reader falls off a phrase, not off the file's structure.

One soft note: `tribunal-record.md`'s closing "Working Notes" mentions an "internalized form... described in `[[tribunal-strand-survey]]` strand B" that it explicitly says is "not this project's material" — that's honest and fine, but it does mean a reader who only has `tribunal-record` loaded gets a dangling pointer to content the claim itself declines to use. Not a defect, just worth knowing it's there on purpose.

---

## Over-production: did one topic become too many segments?

Short answer: mostly no, with one real question.

`decision-records`, `warrant-over-authority`, and `asked-and-answered` share one grounding specimen (`authority-flag-specimen`) but answer three genuinely different questions — what a decision record needs structurally, which field a schema should make prominent, and how an open flag should carry disposition. I'd have expected to feel these blur into each other reading them back to back, and they didn't; each has a claim the other two don't make. That's the atom-grain-parallelism norm working, not padding. The one thing I'd tighten: none of the three *says*, in its own text or in the OUTLINE row, that all three come from the same thirty-second exchange — a reader who happens to read two of them back-to-back will notice the overlap in citations before they understand why. A one-line Working Notes addition on each ("this and [[sibling]] both ground in the same specimen, answering different questions") would pre-empt that.

The real question is `tribunal-record.md`. It currently does three jobs: (1) states the role-set/independent-failure-modes claim, (2) states the record-as-product/durability claim, (3) introduces the Gate-2 "thin enforceable form" as a scaled-down version of the same idea. The OUTLINE row for ch. 8 only promises the first two ("Tribunal processes: typed voices with independent failure modes; the durable deliberation record as the product"). The Gate-2 material is a third, self-contained claim (a paragraph-level reviewer discipline is structurally the same job as a council) that could stand as its own segment — it already has its own worked example (the passed-review Discussion claim) and its own strength assessment ("the strongest-supported part of this segment," by the file's own words). I'd ask whether that sentence is a tell: if the strongest-supported part of a segment is also its least-connected part, that's often a sign it wants to be its own atom. Not a strong recommendation either way — the file argues for keeping it together ("the pattern scales down") — but it's the one place I'd want your judgment over mine, since you know whether ch. 8 wants that granularity.

---

## Recapitulation check

None of the seven struck me as describing a design at length without ever asserting what's true. Each states an explicit claim early and returns to it; `tribunal-record` is the one that could be misread this way on a skim (it spends real space narrating the role-set), but it does commit to claims ("worth having," "worth keeping... because the deliberation is what a later finding needs to attack") rather than only describing. The Strength & Grounds sections across all four claim segments are doing real epistemic work, not decoration — they name specifically what's established vs. leaned-on vs. untested, which is exactly the thing that would be missing from a recapitulation.

---

## Match against the OUTLINE rows written before drafting

- `decision-records` — matches its row closely (confidence-vs-content, load path, revisit conditions all present). The row doesn't mention "hypothesis where it has one," which the draft adds as a fourth pillar — a genuine addition, not a drift, and probably worth folding one clause into the row summary.
- `warrant-over-authority` — matches its row exactly; the row's "agents dogmatize whatever the schema makes first-class" is close to verbatim in the draft's opening line.
- `asked-and-answered` — matches its row exactly.
- `tribunal-record` — matches two-thirds of its row (roles + durable record) but the row doesn't anticipate the Gate-2 thin-form section, which is now roughly a third of the file. See the over-production note above — this is the same finding from the row-fidelity angle.

---

## On this brief itself

Nothing wrong with it that I'd flag as a problem — it was unusually explicit about what "clear" would look like (the ONTOLOGY split test) which made this a checkable task rather than a vibe-check, and naming your own blind spot (having lived in the material all session) up front is exactly the condition that made a second reader useful here. One small thing: you asked me to check `plan/OUTLINE.md` promised rows for four slugs, and the investigation that turned out to matter most was actually the *absence* of rows for three others you didn't ask about — worth remembering that "check whether X matches" often surfaces "X doesn't exist" as the more important finding, and that's fine, not a brief defect.

I'm glad to stay on the line if anything above needs pressure-testing, or if you want a second pass once the OUTLINE rows are fixed.
