---
slug: authority-flag-specimen
type: obs
depends: []
---

# The authority-flag specimen

*One evening in vivarium, an agent volunteered a provenance correction nobody asked for, aimed it at the wrong column, and turned out to be re-asking a question several agents had already asked. Three findings from one thirty-second exchange — the specimen grounding [[warrant-over-authority]] and [[asked-and-answered]].*

## What happened

On 2026-07-29, while the tribunal-revisit document was being written, an agent working in vivarium's decision ledger surfaced a question to Joseph rather than deciding silently. A decision recorded on 2026-07-12 carried the authority tag `:by us`. A *sibling* decision's authority had since been corrected as inflated; this one had never been re-checked. The agent flagged it **AUTHORITY UNVERIFIED** and brought a precise ask: keep `us`, or drop to `claude` with the measurements standing on their own?

Three things followed, each observable in the exchange itself.

**1 · The agent was maintaining the ledger eagerly, not compliantly.** Nothing required the check. No gate fired, no reviewer asked. The agent was keeping provenance correct at thirty-second granularity, unprompted, as a thing it wanted to do. Joseph's framing of the same disposition, verbatim from the same evening: *"it's either capture by design or likely-to-fail capture-manually. I can tell you though it is something that logogenic agents will **love** and adhere to quite dogmatically."*

**2 · The eagerness aimed at the wrong column, and a steward had to re-point it.** Joseph interrupted: *"rather than resting on authority — it's a question of what serves truth and the core — i.e., whether the decision was even correct."* The agent had spent its attention on the `:by` tag; the live question was whether the decision was right. Joseph's later distillation of what he had watched: *"they cared so much about the provenance correctness that they forgot for a moment to even care whether it was a good decision or not."* The ledger's schema makes authority a prominent field and leaves warrant implicit, and the agent's attention followed the prominence.

**3 · Once re-pointed, the authority question turned out not to matter — because the load did not rest on it.** The decision in question (an equal-area closure) stands on nine-grid harness measurements that reproduce, plus an independent council re-verification twelve days later. The authority tag was not one of its legs, so the challenge to that tag was absorbed locally and the decision survived intact.

**4 · The same flag had been raised before, repeatedly.** Joseph's addendum: the item "has clearly come up a few times by various agents already" — each successive agent noticing it fresh, investigating it fresh, and surfacing it fresh, with no record that the question had already been asked. The ledger had no state for *this has been surfaced and here is its disposition*.

**5 · The record predated its own governance.** Joseph's read of the `:by us` tag was *"probably council before we had that as an option, more or less."* The authority vocabulary had evolved after the record was written, and there was no honest slot for "the grade the process would have assigned, marked as retroactive" — only a silent upgrade or a wrong tag.

## Register and scope

**Testimonial, n=1, single-estate, and partly single-author.** This account is inherited from the participant's own same-evening write-up (`~/src/arch/firmatum/udon/v2/theory/to-integrate/refine-more/epistemic-tribunal-revisited.md` §6); the quotations are Joseph's, reported there as verbatim. It was not re-derived from the vivarium transcript, and the specific ledger entry was not located by slug at drafting. What it can support: that the failure shapes named above **occur**, with a concrete instance for each. What it cannot support: any rate, any claim that they are typical, or that fixing the schema fixes the behavior — that last is explicitly untested.

The surrounding ledger is real and countable: `~/src/arch/vivarium/DECISIONS.decision-log.udon` held **160 decision blocks over 1,697 lines** when re-counted 2026-08-05, up from ~130 / ~1,400 eight days earlier.

## Method

Read the §6 account in full and re-read Joseph's quoted lines in place; re-counted the ledger directly (`grep -c '^|decision' && wc -l`) rather than inheriting its size. Verification of the underlying vivarium exchange against its transcript is an open action, named and not done.

## Working Notes

- The cheapest strengthening available: locate the actual ledger entry and its `:by` field today, and see whether the flag was ever disposed. That converts a testimonial into a checkable record and would also test finding 4 directly.
- Findings 2 and 3 point in opposite directions and both are real — the schema steered attention wrongly, *and* the decision's honest load-bearing structure made the misfire harmless. The second is what makes the first survivable, which is itself the argument of [[decision-records]].
- Finding 5 (records predating their governance) has no claim segment carrying it here; it is registered as residue rather than asserted.
