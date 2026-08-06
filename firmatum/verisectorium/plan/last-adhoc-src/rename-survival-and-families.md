---
slug: rename-survival-and-families
type: form
depends:
  - slug-identity
---

# When an identity must change, and when two names must not become one

*A stable identity is not a permanent one: names improve, records merge, claims split. Two rules keep those events from corrupting the corpus — the retired name goes on resolving forever, and two records that merely say the same thing differently are linked rather than merged.*

## The claim

[[slug-identity]] keeps identity out of the way of housekeeping. It says nothing about the events where an identity legitimately *must* change, and those events are ordinary: a name that turned out to describe the wrong thing, two records discovered to be one claim, one record found to be two.

**(A) Retirement, not deletion: the old name goes on resolving.** When an identity is retired — by rename or by merge — the corpus keeps it as an alias on the survivor rather than dropping it. Resolution falls back to the alias index, and anything the corpus *emits* is written under the name the consumer cited, not the name the corpus now prefers. The consequence is the point: a rename inside the corpus requires zero edits outside it. Uniqueness has to be enforced across identities *and* aliases together, or the alias index becomes a second source of collisions.

**(B) The counter-rule: siblings link, never merge.** Two records that express the same claim differently — for a different audience, at a different time, at a different strength — are not duplicates, and merging them destroys a real distinction while pretending to tidy. They are kept and linked. The distinction has to be drawn **mechanically**, not by similarity of sentiment: two records are the same record when a reader following a citation to either arrives at the same content at the same place. Similarity judgments flatten a family into whichever version an early reader happened to prefer, and the loss is invisible afterwards.

**(C) Merging is not idempotent unless the minting sites cooperate.** A corpus that merges duplicates and then keeps creating records will recreate them, because whatever produced the collision the first time is still running. So the alias index must be consulted at *every* site that mints an identity — not only by the dedup pass — or the twins regrow. This is the same shape as [[collision-staleness-detection]] seen from the write side: without a check at the moment of creation, the corpus's own intake silently undoes its curation.

**(D) Two strategies exist, and which one a deployment needs depends on where its consumers are.** The alternative to aliasing is *rewriting the world*: change the name and mechanically update every reference to it. That is a complete answer when the corpus and everything that cites it live in one tree — and it is what the estate's claim corpora actually do. It stops being an answer the moment a citation exists somewhere the rename cannot reach: another repository, a published artifact, a reader's notes, a frozen archive. Notably, the rewrite strategy usually *declares* such places — the excluded directories a rename tool refuses to touch — which means it also declares exactly where the old identity stops resolving. Those exclusions are correct (historical material should keep its era's names) and they are also, precisely, the alias problem, unaliased.

## Strength & grounds

**Heuristic, with one worked implementation and one independent restatement.**

(A), (B) and (C) are read first-hand from relata, the estate's most-evolved identity store (`~/src/arch/firmatum/relata/README.md` §4, read 2026-08-06). Its rules are stated there with their reasons: citation-distinct versions become distinct entries linked by `same_work_as:` because *"silent merge would corrupt citations, the one thing relata exists to protect"*; retired keys *"live forever as `aliases:`"*, `resolve` falls back to the alias index, and *"`emit` writes the entry under the cited name"*; `validate` enforces alias uniqueness corpus-wide; and every key-minting site checks the alias index — the README states the reason as a first-hand incident, *"a re-drain once recreated merged twins within hours."* (C) is therefore evidence-backed rather than merely argued, though the evidence is that store's own account of its history, which this pass did not independently reconstruct from its logs.

(B)'s mechanical-test half has a second, independent statement in a different domain: comproprium's format rules for a quotation corpus — *"Restatements are not duplicates. Two segments are duplicates only when they resolve to the same span in the same primary… Deduping on sentiment rather than span flattens a family into whichever version an early agent preferred"* (`~/src/arch/proprium/comproprium/FORMAT.md`, read 2026-08-06), with `:family` as the link. A bibliography store and a precept corpus arriving at the same two rules, in vocabulary neither borrowed from the other, is the strongest agreement available for anything in this chapter — and it is still one estate and one steward, so it is convergence of intent rather than corroboration by independent design.

(D) is an observation with a live instance on each side and no comparison between them: ASF renames by rewriting (`bin/rename-slug` moves the file, rewrites the `slug:` field, and replaces `#OLD` references, `depends:` entries and path references repo-wide, with a post-check for residuals and a fixed exclusion list of frozen directories — read 2026-08-06); no claim corpus in the estate carries an alias mechanism at all. Nobody has measured what the rewrite strategy costs, or how often a reference outside the rewritable tree has actually been broken by it — which is the obvious next measurement and would decide whether claim corpora need aliases or genuinely do not.

## Working Notes

- Registered in `plan/TODO.md` as N1, N2 and R48. R48 also carries a count of duplicate clusters from an earlier reading; the live store's own figures differ and are dated, and its README declines to state them as canon at all (*"counts are pointers, not numbers"* — registered separately as N16), so no census is stated here. If this segment ever needs one, it wants an appendix `obs` with a re-run.
- Unexamined and probably important: **splits**. Aliasing answers many-names-to-one-record; nothing in the estate answers one-name-to-two-records, where the retired identity has *two* successors and cannot resolve to either. The relata rules do not cover it and the claim corpora do not either.
- The estate's claim corpora also lack the *forward* half — nothing links two segments that state the same claim at different strengths for different audiences, which is exactly what [[multiple-views]]' "omit rather than fork" rule exists to prevent needing. If projection ([[selection-and-projection]]) matures, the need may not arise; if it does not, families will.
