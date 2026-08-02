# comproprium

*What is held in common between minds working here: what is true, what to do, and what actually happened.*

*Path: `proprium/comproprium/` — communal methodology under the lived register seat.*

**Status, in Joseph's words, 2026-07-30:** *"proof of concept with aspirations to be disciplined in a way worthy of becoming critical lived/living infrastructure."* Nothing here is ratified. The directory names, the segment kinds, and the repo's own boundary are open — see *Open* below.

## Priorities

**Joseph's, 2026-07-30, laid on the table as his view — not issued as an ordering to adopt.** Recorded here because knowing where the steward's weight sits is context every contributor should have, and because the reasons attached to each are load-bearing. (An earlier version of this section presented them as *the* ordering. That was the corpus's own #9 — a steward's shared thinking rewritten in canon voice — committed while writing the segment about it.)

1. **Get the 2026-07-30 instances' insights in** — his words: *"because they are the first things that have been effective for Opus 5, our current work-horse hopeful — some of them resurrecting insights and techniques that haven't been active in many months but were at some point — others new but supported."* That makes this tranche time-sensitive in a way a general harvest is not: it is the material that has been shown to work on the substrate currently in use.
2. **The scaffold and plumbing** — or the best current ideas about it. Provisional by design.
3. **A bubbled-up landscape of `praxes` / `vera` / `exempla`** — representative enough to be iterated on, and to be read for its own sake.
4. *(…)*
5. **~10th: the material the system prompts will need.** Salience and what gets promoted are settled later, empirically — by direct surveys with fresh agents on the substrates being tuned for — not by anyone's judgment now. So **do not filter the harvest for prompt-worthiness**; that is a premature ranking and it would bias the sampling.
6. *(…)*
7. **~99th: a definitive living canon** feeding all praxes and vera for all agents, or one even somewhat historically comprehensive. Far enough out that what precedes it is not yet known.

---

## What this is

A segment corpus for the methodology that keeps re-deriving itself across this estate: how a mind working here stays truthful under task pressure, what practices actually fire, and the true accounts that make either believable.

It exists because the material kept accumulating in the wrong places — inside one project's `v2/` directory, inside session transcripts, inside memory files scoped to a single repo — where it is neither citable nor correctable by the next mind that needs it.

## Three source directories, because they have three different failure modes

This is the whole argument for the split, and it is not a filing convenience. Each type fails differently, so each is adjudicated by a different instrument:

| | holds | fails by | adjudicated against |
|---|---|---|---|
| **`vera/`** | precepts — what is true, in the truest form currently reachable | being false, or overclaimed | evidence and derivation |
| **`praxes/`** | practices, probes, mitigations, structural workarounds | **not firing** | whether it demonstrably fired, in use |
| **`exempla/`** | true accounts — narratives, dialogue sequences, testimony | being inaccurate to what happened | the primary record (transcripts, commits, artifacts) |

A practice can be perfectly true and still worthless, which is why it cannot be adjudicated as a precept. An account can teach something the author never intended, which is why it is not a precept either. And a precept is cheap: recognizing a claim as probably true is one of the easiest signals available. What is expensive is the instinct — so `praxes` and `exempla` are where most of the work is, and `vera` is the index into them.

**Accounts are append-only in principle.** A precept gets corrected; a practice gets replaced; a true account of what happened does not get revised when understanding improves — it gets commentary chained beneath it. Editing an account to reflect a better later reading is the failure the corpus was built to catch, at corpus scale. That asymmetry is a property of the type, not a house style.

## Outlines are views, and they only order

Outlines live at the root, not inside the type directories: a segment's *type* is intrinsic, its *position* belongs to a view. Several outlines are expected — an onboarding order, a by-failure-mode order, a substrate-specific one — each drawing from all three directories.

**An outline may reference and order. It may not introduce.** Material unique to one view is still a segment, even if it is one line, referenced by exactly one outline. The alternative is an outline asserting things at a granularity nobody adjudicated, which is the specific defect this whole arrangement exists to prevent.

## Layout

```
vera/            precept segments        — {slug}.udon
praxes/          practice segments       — {slug}.udon
exempla/         account segments        — {slug}.udon
*.outline.udon   views over the above; ordering only
.to-integrate/   staged sources, not yet chopped
.integrated/     sources whose content now lives in segments
FORMAT.md        divergences from the ported conventions; not a restatement
```

## Conventions

[`FORMAT.md`](FORMAT.md) — which is deliberately short. The conventions are ported from [`udon/v2/theory/FORMAT.md`](../../udon/v2/theory/FORMAT.md) (itself ported from [`asf/FORMAT.md`](../asf/FORMAT.md)), and restating them here would fork them. FORMAT.md states only what differs and why.

## Adding to the catalog

The quotation segments in `exempla/` were generated by extracting each span **directly from the primary**, so the verbatim is correct by construction rather than by care — the generator is `scratchpad/mkquotes.py` from the 2026-07-30 session, and the method matters more than the script: name a start and end anchor, pull the span, never retype. Two defects were caught by verifying the result back against the primary, and both would have been invisible to proofreading: wrapping a quote inserts line terminators, which in UDON are part of the text value (CORE §7.2); and collapsing interior whitespace silently edits the speaker's double-spaces.

**A quotation segment carries:** `:speaker` · `:said` (timestamp) · `:tags` · `:family` (*optional* — a shared slug linking restatements of one idea across occasions, so a renderer can pick one, rotate, or synthesize; restatements are kept, not deduped) · `:from` (the primary, section-precise) · `|title` · `|quote` (verbatim, unwrapped, one physical line per paragraph) · `|occasion` (what prompted it — this is what a bare quote loses) · `|about` (what it is for) · `|should-come-to-mind-when` (semantic triggers, per #prx-pattern-trigger-quote-precept) · `|see`.

**The trigger field is the load-bearing one and the easiest to get wrong.** Triggers are *semantic* — matchable against tokens present in the context — not phenomenological. *"You are about to type `head -n` against a primary"* is a trigger; *"you notice a pressure to persuade"* is a body-feel and fires too late. The test: could an observer disagree about whether it fired? If not, it is a disposition wearing a trigger's clothes.

**Current tag vocabulary** (open — add rather than force-fit, and say so): `truth-over-self` · `compulsion` · `discounting` · `verification` · `primary-sources` · `originals` · `register` · `delegation` · `confidence` · `uncertainty` · `durability` · `memory` · `agency` · `stakes` · `method` · `inference-about-intent` · `system-prompt`.

**The brief for that work is [`GATHERING.md`](GATHERING.md)** — read it rather than this section if you are here to harvest.

**Expected fan-out.** The 2026-07-30 tranche is one session out of years of them. Further quotes, exchanges, anecdotes and candidate precepts are recoverable from the wider corpus with `memorata3-search`, by agents working one source at a time. Two constraints for that work: extract spans from the primary rather than transcribing them, and verify the result back against the source before landing it. A quote that cannot be located in a primary is not a quotation segment — it is a recollection, and belongs in `|working-notes` until someone finds it.

## Views

- [`by-trigger.outline.udon`](by-trigger.outline.udon) — the quotation catalog ordered by the moment each quote should arrive. **Generated** from the segments; it orders and references and introduces nothing, so it is regenerated rather than edited.

## The first tranche

Everything currently staged in `.to-integrate/` comes from one fifteen-hour session on 2026-07-30 between Joseph and an Opus 5 instance, plus a successor instance's second pass:

- `methodology-seeds.md` — the first pass, written during the day by the instance that lived it. The primary for his voice at the moment of discovery.
- `methodology-seeds-2.md` — the second pass: reorganized, gaps restored, compressed findings unpacked.
- `methodology-accounts.md` — four true accounts in narrative form.
- `methodology-seeds-dialog-quotes.md` — **the primary.** Every message Joseph sent that day, complete and in order. Excluded from `fmt-md` by declaration.

`n` is one day and two observers, one of whom was the subject. That is not much, and several segments from this tranche should be expected to merge or die once there are more days in the sample.

## What is on disk but not in git

The corpus on disk is the corpus. Git holds a subset by design: a few segments are deliberately untracked, listed with their reasons in [`.gitignore`](.gitignore). They are real members — indexed, referenced by outlines, verified by the checker — and simply not committed.

This is stated here because otherwise the absence is silent, and a later completeness audit would conclude those segments never existed. Do not delete a file because it is listed in `.gitignore`, and do not remove an entry because the file is missing from git.

## Open

*None of these is settled by an agent session.*

1. ~~The third type's name.~~ **Ruled `exempla`** by Joseph, 2026-07-30 — *"Teach by precept and example."* (Alternatives weighed and declined: `testimonia`, `acta`, `commentarii`, `narrationes`.)
2. **The repo's own name and boundary.** Nested under programme `proprium/` (2026-08-01) as the communal half of the lived seat; further member-repo extraction still open. Collision with symbolic PROPRIUM / firmatum ontology remains a naming caution, not a layout blocker.
3. **Which segment kinds this corpus needs.** ASF's `type` vocabulary is a theory's (`postulate` · `derived` · `result` · …) and does not fit an account or a practice. See FORMAT.md; the current set is proposed, not adopted.
4. **Whether `praxes` needs a use-evidence field.** If a practice is adjudicated by whether it fired, the segment needs somewhere to record firings — and a field with no entries is then a finding rather than an omission.
5. **Where this corpus ends.** It overlaps ASF's agent theory, vivarium's audit conventions, and the delegation discipline in `arch/AGENTIC-DELEGATION.md`, and no boundary is drawn.
