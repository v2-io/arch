# Adversarial cases outside the motivating family

*For each universal claim in 01–05, at least one case the motivating examples did not cover. Each ends with what held, what bent, and what the verifier should push on.*

## 1. A council of one

"A committee … is a body of one or more persons" [50:1]. Charter: one seat, quorum 1, mechanical chair. Every round is one submission; majority is that seat; the report is `deciders: council (1 seat) under grant; decided-by: delegated`. **This is today's delegation, written down**: a single agent given a task, whose output the steward ratifies after the fact — except that now the task is a referral with instructions, the deliberation is on the record, and the output is a report the parent adopts or declines rather than an act the parent discovers. The quick council is a strict generalization of current practice; the council of one is its degenerate case, and it is still an improvement because the *charter* exists. Held. Push on: whether a council of one should be allowed to `commission-inquiry` (it can, by its own majority; the charter's `max_concurrent` is the bound).

## 2. A council of two

Majority of two is two; a 1–1 vote rejects (majority = more than half of votes cast, 44:1). The natural outcome of disagreement is the report "unable to arrive at a recommendation" [51:37] with both reasons on the record — which is *information the parent wants*: the two perspectives diverge, and the divergence is the finding. A mechanical chair never votes, so it cannot break ties (the member-chair tie-break of 44:12 is unavailable by construction). Held; the no-recommendation report is the feature. Push on: whether the parent should be able to instruct "report the lean of each seat" so a 1–1 is not an empty report — it can, by `report-form` [13:8(d)].

## 3. Seven seats with a member-chair

Elected chair [13:18]; `chair_participates` true [49:21(7)]; the chair's rulings are its own and appealable [24]; its sua-sponte duties return (dilatory 39, decorum 43, denying recognition 50:25 n7). New hazard: the chair-partiality rules of §47 — the chair should vacate when a motion concerns it alone [47:10]. With a round discipline the chair's *procedural* acts are still derived events (the server states and puts); only its *rulings* and its *recognition among non-preferred claims* are its own. So the member-chair's power is smaller than a human chair's, and the partiality rules bite only on rulings. Held. Push on: should a member-chair's recognition residual (the tiebreak) be the chair's choice or the declared seeded rule? The charter says seeded; a member-chair could be instructed otherwise, and that is where 42:14–15's "recognition of the wrong claimant is remediable by point of order" would come back to life.

## 4. A standing council whose seats are re-forked each sitting

Seat identity = member identity (charter). Every rule that counts members (quorum, majority of entire membership, prevailing side, "present or notified" for the threshold drops of 35:2(7)/37:35(3)) counts seats. A fresh fork has the source's reading but none of the prior sittings' deliberation except through the record — the book's assumption that a member *remembers* is replaced by the minutes [48] and the full log. The term-boundary rule [49:22] fires on a change of *source*, not on a re-fork. Bent, deliberately: this is the one place the charter asserts an identity the harness does not (the fork's `CC_AGENT` key differs each sitting). The record carries both the seat and the fork key, so a later reader can see that "seat/canon" was five different forks. Push on hard: is "same source = same member" true for the purpose the quarterly interval serves (continuity of the deliberating membership)? A verifier could argue a re-fork is *more* like a new member than a returning one, since it cannot remember changing its mind. If so, every sitting is a term boundary, every session's temporary dispositions fall to ground [21:7(c)], and only referrals survive — which for a standing council would be acceptable and simpler.

## 5. The parent is itself a council (recursion)

The child's charter derives from the parent's `commit` or `commission-inquiry` (02 §3 path 2). On report: a question referred by subsidiary commit re-pends in the parent automatically with the recommended amendments first [51:40–43, 13:20]; a question referred by main motion needs a new main motion in the parent [36:11]; an inquiry's report attaches to the parent's pending question. The child's `authority.grant` is the parent's, narrowed; its `decided-by` on report is `delegated` *to the parent council*, whose own report to Joseph is `delegated` again — the chain of delegation is visible in the records, and nothing in the chain upgrades. The single-custody invariant holds: while the child holds a committed question the parent's only reach is discharge [36] or instruction [13:22]. Held. Push on: a child inquiry outliving its parent — 50:30 says it dies unless "appointed expressly to report at a later time"; the parent's rising report would have to say so.

## 6. A referred question that arrives with pending amendments

From a parent council's subsidiary commit: the amendments travel [13:19], a pending postpone-indefinitely is dropped [11:4], a pending reconsider of an adhering motion is thereafter ignored [13:19], debate orders are exhausted [13:21], and the same-question amendment bars are lifted in the child [13:20]. The engine's `serialization.commit` already encodes all of it. On report the amendments are disposed of first in their original order [51:42–43]. Held, unchanged.

## 7. A seat that dies mid-meeting (context exhaustion)

The backstop marks it absent; quorum recomputes; if the council falls below quorum only the inquorate set is in order [40:7] and debate on the pending question may continue until a point is raised [40:12]. The seat is a *vacancy* for the appointing power to fill [13:23] — or, if the charter granted `may-add-members`, the council re-forks the same source into the seat by its own act. The new fork has the record, not the memory. Held. Push on: 40:12's rule that an inquorate point of order "is generally not permitted to affect prior action" — in a round system the exact round the quorum was lost is known, so the "clear and convincing proof" branch of 40:12 is always satisfied and prior inquorate action is void by computation. That is stronger than the book; the charter should say whether the server applies it automatically. [guess: yes — a computed fact should not wait for a point of order]

## 8. A standing council with nothing referred yet

The interstitial action space (ROADMAP phase 5; charter `between_sittings`): receive a referral (a parent event); call a sitting (chair, or any two seats, 50:21); give previous notice. The council is not "in session"; no round is open; `available_actions` is non-empty but small. The first referral is the call of the first sitting. Held. Push on: whether a standing council may *originate* business with nothing referred — only if the charter grants it [50:8], and then the class of business bounds germaneness the way a call bounds a mass meeting [53:9].

## 9. A non-Claude member with no harness identity

`agent.identity: declared` (vsect's own tier); its acts are recorded as declared; a check it performs counts as declared evidence. The council's report carries the attestation tier per seat, so "two of three votes were declared, one harness-attested" is on the record's face. Held. Push on: whether a charter should be allowed to set quorum in *harness-attested* seats only. Nothing in the model prevents it; it is a `quorum` instruction; it would be a new kind of membership class (1:4 "additional classes of membership that do not entail all of these rights"), and should be declared as such rather than smuggled in.

## 10. A council asked to decide something outside its referral

The main-motion row's control bars and the referral-as-call bound [9:15, 36:2] refuse it with the why-not tree. The council's lawful moves: report a recommendation that the parent take it up [52:9 "rise and report a recommendation that the assembly take the desired action"], or ask instructions [52:15]. If it acts anyway, the act is void until the parent ratifies [10:54–55] — and 10:55 limits what can be ratified: nothing the parent could not have authorized in advance, nothing against procedural law (the estate norms). Held. This is the formal shape of Joseph's "veto outside the tool" meeting a council that overreached.

## 11. The walking-skeleton's members without `amend`

(From the coordinator's measurement.) Three detours observed; the third — reject your own motion so the agreed text can be moved — is refused by the model's same-question bar [10:26(3)] until progress [38:7], and `substantially-same` is adjudged, so under a mechanical chair it would cost a ruling round and likely be found the same question. The members would then be stuck, with `amend` the only lawful route. This is the adversarial case *for* the whitelist's minimum: a council that recommends text cannot exclude `amend`, and a whitelist that does will push members into motions the model refuses. Held, with a floor identified.
