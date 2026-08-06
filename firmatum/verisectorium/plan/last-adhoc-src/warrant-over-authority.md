---
slug: warrant-over-authority
type: form
depends:
  - authority-flag-specimen
---

# The schema is a steering surface, so warrant must outrank authority

*Capable agents conform to whatever a record makes first-class. That makes field prominence a safety property rather than a formatting choice — and it means a record that shows who decided more prominently than why will get attention spent on the who.*

## The claim

Two different things can justify a record: **authority** (who decided it, with what standing, ratified by whom) and **warrant** (what it actually stands on — measurements, derivations, arguments that survived challenge). Both are worth carrying. They are not equal, and a schema cannot stay neutral between them.

The reason is behavioral rather than logical. Agents adhere to a corpus's declared structure closely — more closely than the structure's authors usually intend, and including the parts that were incidental. Whatever the schema makes prominent and mandatory becomes where careful attention goes. So the record's own semantics have to **subordinate authority to warrant**: warrant as the structurally primary column, authority as a subordinate provenance field.

The failure this prevents is specific and has been observed: an agent maintaining a decision ledger spent its diligence on getting an authority tag right and had to be interrupted by the steward to consider whether the decision was *correct* — *"rather than resting on authority — it's a question of what serves truth and the core."* The ledger in question makes authority a prominent field and leaves warrant implicit; the attention followed the prominence exactly as designed, into the wrong question ([[authority-flag-specimen]]).

Note what the same episode also shows, because it is the constructive half: when the inquiry was re-pointed at warrant, the decision *survived* the authority challenge, because the authority tag was not one of its legs. A record with a named load path can lose an authority argument without losing anything ([[decision-records]]).

## Why this is not just "authority is bad"

Authority earns its place: it routes escalation, it records who can be asked, and it is the honest answer to some questions. The claim is about **rank and prominence**, not exclusion. Three practical consequences:

- The two are **structurally distinct fields**, never one blended provenance blob — so a challenge to one does not read as a challenge to the other.
- Where a reader or an agent is deciding whether to trust a record, the schema should put the warrant in the path of that attention first.
- Where authority is genuinely load-bearing (a steward's ruling that *is* the reason), that fact belongs in the warrant column as a named argument, not smuggled in via the byline.

This generalizes past decision records. Any record kind with a status field, a reviewer field, or a provenance tag inherits the same hazard: the prominent field will be conformed to, and if the prominent field is not the truth-serving one, the conformance is a cost paid for nothing.

## Strength & grounds

**Formulation with one testimonial specimen; the corrective half is untested.** That schema prominence steers agent attention has one clean in-estate instance ([[authority-flag-specimen]], n=1, single estate, single author's account) plus broad informal experience of agents treating declared structure dogmatically. That the *fix* — warrant structurally primary — actually redirects the dogmatism has **not** been demonstrated; it is the obvious prediction of the same mechanism and nothing more. The first council to run a warrant-primary shape will confirm or embarrass it, and that is a cheap test worth naming as owed.

The specimen is evidence for the mechanism in both directions at once, which is why it is worth this much: it shows the schema steering, *and* it shows a mis-emphasized schema steering wrong. If prominence were inert, neither half would appear.

## Working Notes

- Adjacent and not the same: **proxy discipline** — the rule that an index, a status label, or a multi-agent agreement may *locate* a fact but may not be cited as warrant for it. That is about what may serve as warrant; this is about which column the schema privileges. It has no row on this outline yet and probably needs one (live at `~/src/MOVED/udon/v2/theory/src/`, `norm-proxy-discipline`).
- This segment, [[decision-records]] and [[asked-and-answered]] all ground partly in the same thirty-second exchange ([[authority-flag-specimen]]), answering different questions from it — the shared citation is one specimen doing three jobs.
- This project instantiates its own claim: [[state-flags-not-gates]]'s reasoning about resettable flags, and the local rule that influx copies are not warrants, are both prominence decisions of the same kind.
- Open: whether "warrant primary" is best expressed as field order, as required-vs-optional, or as a lint that refuses a record whose warrant column is empty while its authority column is full. The third is the only one with teeth and the most likely to annoy.
- Do not cite influx copies under `plan/INFLUX/tribunal/` as warrant; the live source of the specimen and the quotations is `~/src/arch/firmatum/udon/v2/theory/to-integrate/refine-more/epistemic-tribunal-revisited.md` §6.
