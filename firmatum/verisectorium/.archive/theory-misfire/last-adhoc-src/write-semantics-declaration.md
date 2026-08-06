---
slug: write-semantics-declaration
type: form
depends:
  - atom
---

# Write semantics is declared per type, not legislated corpus-wide

*Whether a record is replaced by its successor or accumulates entries forever is a property of what kind of record it is — so it belongs in a declaration the tooling can read, not in a corpus-wide rule that some part of the corpus will always be violating.*

## The claim

**(A) The estate holds both semantics at once, on principle.** Claim segments follow integration-is-replacement: a superseded statement is deleted, not kept-and-softened, because present-truth bodies are what make collision detectable ([[collision-staleness-detection]]). Quotation and account records follow append-only: an exemplum that quoted someone in March is not falsified by a better exemplum in August, and rewriting it would destroy the record. Decision and verification events are append-only by construction. These are not one instance being sloppy; they are different write semantics correctly chosen for different kinds. A corpus-wide "integration is replacement" rule mislabels half of them, and a corpus-wide "append-only" rule mislabels the other half.

**(B) So the semantics rides the type.** The unit that can carry it is the type declaration a record already has. A reader — human or tool — that knows a record's type should be able to derive whether replacing its body is the correct edit, whether a second record on the same subject is a collision or a sibling, and whether deletion is a legitimate operation or a data loss.

**(C) Two axes that must stay apart: role and regime.** *Role* declares the record mapping of a file — one record per file, many sibling records in one file, an interior fragment meant to be included elsewhere, or a view over other records. *Regime* declares the write rules — append-only, single-writer, membrane-gated. They are separable, and collapsing them is the same mistake as collapsing epistemic strength with process stage: a multi-record file may be append-only or freely rewritten, and a single-record file may be either. Declaring both is what lets tooling know, for example, that the ordinary "concurrent edits show up as a git conflict" reassurance does not hold for a given file ([[partition-isolation]]).

**(D) Scope limit.** This says the declaration belongs on the type; it does not say what the type vocabulary should be — that is deployment-local and should stay so ([[type-vocabulary-locality]]) — nor does it specify a declaration syntax.

## Strength & grounds

**Heuristic.** The (A) contrast was observed live on 2026-08-05 across this estate's instances: the replacement discipline is written into the ASF and vivarium format rules and into this project's own law, while comproprium's format prescribes append-only accounts for its quotation records. The (C) split is a design position taken in the udon file-roles correspondence and restated here rather than independently arrived at. Single-estate and single-authorship: what this shows is that one corpus family needed both semantics simultaneously, not that every corpus does. The strongest available in-estate test is cheap — take one instance that currently has no declaration, write the type-to-semantics table it is implicitly running, and see whether any record turns out to be under the wrong one.

## Working Notes

- A live candidate for exactly that test: comproprium carries one record typed `pattern`, which is not in the four-value vocabulary its own format document declares for that directory (verified first-hand 2026-08-05 in `~/src/arch/proprium/comproprium/`). A record outside the declared type set is a record whose write semantics nothing states — the failure this claim predicts, sitting in the corpus already.
- Open: whether "collision" and "sibling" can be distinguished mechanically from the type declaration alone, or whether same-claim-different-expression needs its own explicit link (the ch. 1 gap row on restatements and families).
- Unintegrated influx behind this segment (do not cite as warrant): `plan/INFLUX/udon-analysis/underlying-logical-model.md` §5 (file roles) and `plan/INFLUX/synthesis/live-state-field-reports-2026-08-05.md`. Live originals: `~/src/arch/firmatum/udon/v2/theory/to-integrate/primary/underlying-logical-model.md`; the comproprium and vivarium format documents in their own trees.
