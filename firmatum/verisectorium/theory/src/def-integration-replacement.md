---
slug: def-integration-replacement
form: definition
type-expected: definition
status: axiomatic
max: axiomatic
state: [drafted]
depends: [def-atom, post-living-collection]
---

# Definition: Write Semantics and the Delete-Test

Write semantics are a per-store declaration — replacement corpora versus append-only-account corpora — and the delete-test governs every claim that something has been integrated.

## Formal Expression

*[Definition (write-semantics-declaration)]*

Every store in a verisectorium declares its **write semantics** as part of its law, choosing per store (never per whim, never left implicit):

- **Integration-is-replacement.** The store carries present truth only. Integrating new material *replaces* the prior state: a refuted claim is deleted, not kept-softened-with-a-pointer; the epistemic label tracks current truth-status, not provenance; history lives in the history layer. Claim corpora are the canonical case.
- **Append-only accounts.** The store accumulates records that remain true *as records* — events, decisions, verifications, testimonies, precedent accounts. Nothing is rewritten; correction arrives as a later entry. Event trails, decision logs, and account corpora are the canonical case.

The declaration is intrinsic to the store's atom kind, not a style preference: a present-truth claim body *must* replace or it accumulates falsehood, and an account *must* append or it destroys the evidential trail. One instance routinely contains both kinds side by side under one meta-process.

*[Definition (delete-test)]*

An item is **integrated** — and may be moved to an `.integrated/` surface — only if it passes the **delete-test**: *assume the item disappears entirely; is every piece of its information either landed in the population or truly disposable?* Partial satisfaction fails the test. A register *about* the item's remainders (a TODO entry, a breadcrumb, a pointer) is not the remainders landed — the register locates information; it does not contain it.

Two distinct terminal surfaces follow, and they are truth-claims, not storage bins:

- **`.integrated/`** asserts: this item's load-bearing content is present in the population, verified first-hand.
- **`.archive/`** asserts: this item was consciously set down, not landed, with the reason recorded.

Collapsing the two is a label lying about status at the directory level.

*[Definition (graduation-completeness)] — the historical condition*

A store's *graduation* — emptying a mined-out source surface, retiring an influx tree — carries a condition the delete-test alone cannot check: a present-state delete-test verifies only that *currently-present* items are landed or disposable; it cannot detect information lost to an **earlier incomplete pass**. Graduation therefore requires the graduation record to attest completeness *at the time of each dispatch* — per-item verification events, or an honest bounded-guarantee statement scoping which residents were verified under which policy — not merely a clean present surface.

## Epistemic Status

Definitional, with the load-bearing content being the *laws the definitions carry* rather than claims about the world; max attainable: `axiomatic`. The delete-test is steward law (Joseph, 2026-08-05), articulated against a live failure and enforced by reversion — see Discussion. The per-store declaration resolves what the estate's instances exhibit as principled disagreement (replacement in the claim corpora; append-only accounts in the precept/practice corpora; both in relata) into a declared axis rather than drift: the variation traces to the atom's truth-conditions, which is the working discriminator for intrinsic-versus-drift variation. The graduation-completeness condition was surfaced as a gap by post-misfire feedback (2026-08-06) and has an independent estate specimen predating it.

## Discussion

**The delete-test's founding specimen.** The law was ratified in reversion: a batch dispatch declared source files integrated on the strength of TODO entries *describing* their remainders, the steward applied the delete-test, and the state was reverted — the `.integrated/` surface emptied back to the live queue. The failure mode has a name from the estate's proxy discipline: register-about-the-information mistaken for information-landed. The test's phrasing ("assume it disappears") is what makes it mechanical: it converts an integration claim from a judgment about effort into a judgment about *loss under deletion*, which any reader can check.

**Why breadcrumbing fails the test by construction.** A breadcrumb — the item left in place with a pointer to where it "landed" — keeps the item on the live surface, so every future reader still scans it, and its half-dispatched state reads as live work. The economic consequence is developed at [[claim-dispatch-compounds]]; the definitional point is simpler: *a reference is not integration*, and the decisive check is the load-bearing content present in the population, verified first-hand — not an index label, not a pointer, not an agent's summary.

**Reduce, not repoint.** When an item graduates, references to it across the population are *reduced* to the live canonical home (the atom where the content now lives, or the history layer for narrative), never repointed at the item's new archive location — repointing preserves the dependency on a retired artifact and trips every future archivability test.

**The graduation-completeness condition's estate specimen.** asf's spike corpus carries exactly this scar: a pre-policy bulk move filed sixty-four spikes to `.integrated/` without per-spike verification, and the corpus's law now states a *bounded guarantee* — forward and per-cycle, with the pre-policy residents named as un-discharged integration debt until verified or consciously set down. That is the honest form when history is already imperfect: scope the guarantee rather than let a clean surface imply a clean past.

**Relation to the membrane.** Write semantics govern what a landing *means*; the membrane ( [[form-influx-membrane]]) governs how material *reaches* a landing. The two compose: membrane outcomes are append-only events regardless of the destination store's semantics, which is how a replacement corpus still keeps a trail of what crossed into it.

## Working Notes

- Frontmatter schema provisional pending the epistemology decision (see OUTLINE working notes).
- Open: whether `.integrated/` retention is itself per-store law or universal. Current practice keeps the files (archive-preserves, search-space shrinks — [[claim-dispatch-compounds]]); a store could conceivably declare true deletion with history-layer-only retention, and nothing here forbids it, but no instance does.
- Open: the graduation record's *form* — per-item verification events versus a manifest with attestation versus the bounded-guarantee prose form. Candidate resolution rides [[form-decision-records]]' schema family.
- Forward pointer: the per-store declaration is one axis of [[form-enforcement-profile]]; the misfire-feedback caution applies — write-regime and role/mapping are two axes, resist collapsing them into one dial.
