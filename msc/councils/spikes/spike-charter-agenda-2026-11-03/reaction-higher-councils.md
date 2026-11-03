# Reaction to the higher-council principles

*By the spiker, 2026-11-03, after reading `~/src/arch/msc/high-councils/higher-council-principles.md` whole. The coordinator parked this spike's serial-floor findings on its arrival and said a reaction was optional. These are the places where that document and this spike meet, with what each changes in the other. My reading throughout; Joseph has not ruled on any of it.*

## 1. The unanimity tier is where the book's letter and the higher council agree — and that dissolves part of the 9:34 departure

01 §4 landed on two tiers: by 51:2's letter a written committee may report only what *every* member agreed to; majority decisions are claimed by principle as a declared departure from 9:34. The higher-council default is "decide only with unanimity, else hold" (§1.5; Joseph: "the default … is to table issues that are not yet unanimous").

Put together: **a council that decides only by unanimity is a valid committee by the book's letter, round discipline or no round discipline**, because its report contains exactly what every member agreed to [51:2]. The departure I spent 01 defending is needed only for *majority* decisions — which the higher-council shape does not take. So for councils run in that shape the departure is not a departure; the letter and the principle coincide, from two sources that never met. The charters' `report-label` instruction would read simply "a report of what every seat agreed to [51:2]" for a unanimity council, and the polling fallback becomes the main path rather than the fallback.

What the round discipline still buys a unanimity council is not validity but *evidence*: the sealed first counsel round is the record of whether the unanimity was earned (§3.4's "hollow unanimity"). That is the contention spike's property S doing a different job than the one it was built for.

## 2. The report-as-motion repair is the proposition-last shape

Feedback-2 §4 forced the report's text to be the content of the motion to rise, adopted at the end (51:2). The Twelve's procedure (§1.6) has the recommendation "remarkably summarize the feelings of the total Council," moved and voted last. These are the same shape. And the book's *committee* chapters already have it — a committee deliberates, then adopts a recommendation [51:36–44] — which is why 04 §6 found the committee machinery "survives best": RONR's committee is closer to a higher council than RONR's assembly is. The quick-council charter's subject referral (revision 3: no resolution drafted by the parent; the council drafts, 51:4) is the same move from the other side.

## 3. `discuss-informally` is the hinge, and my charters had dropped it

The coordinator's reading that 49:21(4) is RONR's nearest hinge is right, and reading it against my charters found a defect: the small-board chain *adds* `discuss-informally` (in order only when no question is pending), and both my whitelists silently excluded it, because the checker reported silent exclusions only among the model's base actions, not the chain's additions. Fixed in this commit: `action/discuss-informally` is whitelisted in both charters and the checker now reports chain-added member actions a whitelist drops.

With it, the higher-council item path fits the model without a new mechanism: a subject is referred (no motion pending) → informal discussion rounds in which every seat speaks → a recommendation moved last as a main motion → amendable → the sustaining round. The model's `unanimous-consent` threshold is already in the vocabulary. What the model lacks is the sustaining round's *vocabulary* — sustain / sustain with reservation / not settled / cannot sustain — which is a `Vote.tally` extension, and a *hold* disposition with a diagnosed deficit, which revision 3 already has the pieces for: `postpone-to-certain-time` to an event anchor ("until the two research assignments report"), `engage-consultant` for the research, and the standing council's `lapsed-grounds` rule for taking the item up again with what changed shown. "Each dissenting seat names what would change its view" is the event the postponement is anchored to.

## 4. `every-seat-asked` becomes structural

Feedback-2 §2.2's repair (the final vote waits until every seat has been *asked* in a closed debate round) is a rule I had to declare. In counsel rounds where every seat speaks, it is the round's own shape. That is a small sign the higher-council form is the better fit for what the rule was protecting.

## 5. The presider is a third chair mode, and it answers the 2–1 bloc cost

My charters have a mechanical chair (every contested ruling goes to a majority — feedback-2 §9's cost) or an elected member-chair. The higher council has a *presider* who frames the item, is revealed last in the first round, calls for the recommendation when convergence is sensed, and confirms — or holds, or escalates. That is a chair mode neither of mine is: it does not rule points of order by majority, it does not preside impartially, and it is accountable. §3.2's lean to seat the accountable presider inside the council, as an agent holding Joseph's grant with escalation to him, would make `officers.chair: presider/<seat>` the right charter value for these councils. Not implemented here — it is shape-dependent and parked with the rest — but it is where the chair question should land.

## 6. What the parked findings look like from here

Most of 01's ten rows are about contention for a floor: interruption, recognition, exhaustion, yield (b, c, g, i). In counsel rounds where every seat speaks there is no floor to contend for, and those rows dissolve rather than translate. What survives of 01 is the timeliness of a point of order (a), consent as a sustaining round (d), vote windows (e), presence (f), and the vacuities (j). I think the coordinator's reading is right that much of the floor churn came from replaying machinery for conflicting interests in a body that has none — with one reservation: the point-of-order timeliness and the sealing property are not adversarial machinery, and a unity-of-purpose council still needs them, for the same reason the Twelve "rarely interrupt" but do object when an objection "throws light."

## 7. The germaneness test moves up a level

Ballard's admission test — an item that cannot be linked to the council's purpose "does not belong on the agenda" — is the same predicate as F17's scope bar on main motions, applied to the agenda instead of the floor. The standing charter's `class_of_business` is that purpose; the quick charter's referred subject is it for one item. The predicate is the model's own `germane`, raised against the purpose rather than a pending motion.

## 8. Where I would push back, lightly

- **"Unanimity or hold" needs the hold to have an end condition**, or it is the veto by silence of feedback-2 §2.2 in another form. The document has the condition (§3.1 step 7: each dissenting seat names what would change its view; research assignments with a return date). The charter should require it: a hold without a named deficit and an anchor is not in order.
- **A unanimity council still needs the rights floor** — the lone objection, the point of order, the warning intent §3.4 proposes. Unity of purpose is a reason those will rarely fire, not a reason to remove them; the ROADMAP's rule that rarity never buries them applies with more force, not less.
- **The 51:2 coincidence is a fact about committees, not about assemblies.** A standing council that may *act* for the parent (`may_act_with_effect: true`, 50:4–5) steps outside what 51:2 covers, and the departure returns for it.
