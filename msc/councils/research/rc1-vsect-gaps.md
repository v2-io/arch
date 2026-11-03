# What RC1 holds that vsect does not yet

*2026-11-03. Claude (Opus 5.5, councils-46), for Joseph's coming look at ASF and vsect together. Written after reading the RC1 segments model whole (`firmatum/verisectorium/theory/influx/segments-model-rc1/`, all 13 files including the addendum and specimens), the Adjudication and Actors chapters of the verisectorium OUTLINE, `securing-contract-2026-08-14.md`, and the RC1 working praxes.*

*On vsect, I read only its README, `vsect help model`, and the four maturation-plan files in `firmatum/utils/vsect/`. I have not read vsect's design in `~/src/aat-v3/msc/vsect/`. "Not in vsect" below therefore means not in what I read. The vsect-1 reader, who did read the design, corrected two of my earlier claims; the corrections are marked.*

## RC1 in one sentence

*A record's kind is individuated by how it fails and what repairs it; it is adjudicated on its kind's epistemic dimensions; its state is the trail of securing acts, with every status a computation over that trail, never a hand-set word.*

vsect's first build already implements the spine of that sentence:
- trails;
- standing computed, never typed;
- checks bound to versions, which lapse when what they read changes;
- refutation only on evidence;
- questions that name who may close them;
- the `decline` act, which is RC1's remedy for under-recording.

## Not yet in vsect, or only planned

| RC1 | vsect, as far as I read |
|---|---|
| **Kinds by failure-and-repair**, with the founding questions Q0–Q8 (`10-FOUNDING-QUESTIONS.md`) | one segment kind and a catch-all `item`. Maturation-plan phase 3 runs the founding questions |
| **Epistemic dimensions as a vector**: Evidence, Backing, Fidelity, Efficacy, Comprehension, Conviction, Freshness, Disposition. No dimension's value substitutes for another's, and a ladder of unrolled permutations is the tell that two were fused (`03-DIMENSIONS.md`) | one checks ledger, with no dimension structure |
| **Decision dimensions**: `decision/grounding`, `consultation`, `accountability` (proposed → supported → ratified → ruled), `falsifiers`. Guard: *ratified* requires grounding explicit and consultation announced (`04-CYCLE.md`) | `decide` is designed (`vsect.md` §3.9, per the vsect-1 reader) but not built |
| **The commitment law**: a check corroborates only if its criterion was committed before contact with the outcome | **correction:** vsect's review items already carry `committed: before\|after` (README; the vsect-1 reader). My earlier "not recorded" was wrong. What remains to see is whether projections use it |
| **The coherence default**: agreement adds weight only where descent is shown cut, using `seeded-by` edges, which are transitive | phase 2 compares agent keys, which is lineage only. The plan's notes §1 already separate "another mind" from "an unprimed mind" |
| **Typed edges**: supports; rebuts versus undercuts (a hard cascade versus soft re-pricing); seeded-by; narrates; restates; supersedes, typed as revised, invalidated or alternate (`06-EDGES.md`) | `depends:` only. Phase 4 adds `narrates` |
| **Declared projections**: strongest-line, every-line-sustains, shown-independent-agreement (`02-RECORD-OBJECT-MODEL.md`) | raw evidence "until the store declares how" |
| **Eras**: measured results expire visibly when their era bumps | absent |
| **EPISODE** (`95-ADDENDUM`): a securing act that spans several records, such as a review, a cascade or a council session. Per-record trails become projections over episodes | **correction:** this is open question Q17 in vsect's design (the vsect-1 reader), not an omission |
| **The rule language** (`09-LANGUAGE.md`): declare a dimension, a kind or an edge kind; a rule says *when*, *allow*, *enabling* | not started |

## Where this meets councils

1. **A council moves a decision record's dimensions.**
   - Every seat is asked, so `decision/consultation` reaches feedback-collected.
   - The first round is sealed and the warrant is recorded, so `decision/grounding` is recorded before the call. RC1 calls grounds written afterwards "masquerading as derivation".
   - What would reopen the decision is `decision/falsifiers`.
   - An agent-led close lands at *supported*. *Ratified* and *ruled* stay with Joseph.
   - RC1's Q0 standing gate is the charter's grant.
2. **Delivery-sealing is the commitment law, in the form a council can use.** Each member commits a first position before seeing anyone else's.
3. **Hollow unanimity is the coherence default applied to votes.**
4. **A council session is the clearest case of an EPISODE**, so councils that write vsect events will force vsect's Q17.
5. **Each transition edge is a queue drained by a decision-maker** (Joseph, 2026-11-03; `../councils-system-model.md`). That gives RC1's "movers" their concrete form.

## For the ASF and vsect session

The two RC1 pieces that councils need most:
- the decision dimensions, with their guards;
- the use of `committed: before|after` in what a projection may count as corroboration.

The one that most changes vsect's shape is the dimension vector, since it replaces the single checks ledger. The EPISODE question is already open in the design, and councils are its strongest specimen.
