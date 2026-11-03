# In-meeting acts with immediate effect, and a quick council walked end to end

*Revision 2, after `de-novo-feedback-1.md` §§2–4, 7–8. Joseph's two examples as event streams through the model, with the generic route the book provides and the charter-granted route; then a three-seat quick council taking the real vsect specimen to a recorded recommendation under one floor act per round and the blank-filling device. Every action named is in `../../model/actions.json`, `registry.json`, or the charter's `add_motion`s; the admitted-by column was re-derived against the expressions after the repairs — by reading, not evaluation (no engine exists).*

## 1. A research subcommittee while the question stays pending

**What it is not.** Subsidiary `commit` moves the pending question into the subcommittee's custody and suspends the council's series [13:1]. Joseph wants the question to stay pending while an inquiry runs beside it and its answer arrives as information.

**What the book has, exactly.** Breakout groups report "after it has been reconvened and the matter under consideration is again pending. These reports are in the nature of debate" [52:28] — and the groups "often … meet during a recess or adjournment". So the book's shape is *recess, inquire, resume, report-as-debate*. Running the inquiry **while debate continues** has no precedent in the book; the charters declare it as a departure. A referral may instruct the committee "to employ an expert consultant" [13:8(d)]; that, not 50:15's narrow non-member exception (which is for committees appointed to *act*), is the warrant for staffing an inquiry with expert forks.

**Without a charter grant** the council cannot commission one. Its lawful moves are to ask the parent for instructions [13:22, 52:15] — a motion to request, adopted by majority, whose effect is a message to the parent — or to recess until the parent answers. Revision 1's "Route A via question of privilege, staffed by a fork" was wrong on both counts (feedback §8): 19:7's "organization or existence" fits adding a member, not creating an inquiry, and a fork is a non-member the council had no warrant to seat.

**With the grant** (`motion/commission-inquiry`, a charter-added incidental reached through `action/move`): one floor act; no second [49:21(2)]; debatable as to the inquiry's desirability and details [13:7(5) by analogy]; majority. Because it goes through `move`, it is refused when the council is inquorate, when a debate-closing order precludes it, when a seat holds the floor, or when it is dilatory. On adoption: effect `spawn-subcommittee` with a derived special charter (parent = this council; referred = the brief; roll = the forks named). The pending stack is untouched. [derived from the charter overlay]

**The return.** `subcommittee-reported` is an input event; the report attaches to the pending question as an advisory input and appears in the next round's delta. No stack change. A recommendation in it reaches the floor only by a seat's motion [51:11]. The inquiry is a special committee and dissolves on its report [50:10]; if the council rises first, the inquiry dies with it unless the council's report asks the parent to keep it [50:30].

**Waiting instead of continuing.** `postpone-to-certain-time` to the event (03 §4), `recess` [20], or `fix-time-to-which-to-adjourn` to the event [22, 9:17]; each restores position exactly [8:7, 14:13–17].

## 2. Adding a member before the council continues

**Authority.** The appointing power adds members and fills vacancies [13:23, 50:11]; a committee may not except as "authorized in … instructions" [50:26], and a referral may authorize the committee "even to add to its own membership" [13:8(d)]. An **absent** seat is not a vacancy, and vacancy-filling is a separate, undelegated power in both charters (`may-fill-vacancies: false`).

**Generic route.** `raise-question-of-privilege` — the organization of the assembly is a privilege of the assembly [19:7]; a mechanical chair submits admissibility (`privilege-admissible` has no default, F8), costing a ruling round; if admitted, the motion is a main motion in a pushed context [19:10]; without a grant its only lawful content is a *request to the parent*.

**Charter route** (`motion/co-opt-member`, through `move`): one floor act, majority, out of order at the declared maximum (`ctx.config.may_add_members_max_total`). Effect `seat-member`: credential, `member-seated`, `membership_total` incremented, quorum recomputed [40:5], fresh counters; present once it first submits; its first delta is the whole record; it may vote on the pending question (the book's default) and the record shows when it joined. Removal mirrors the adoption threshold [50:14]. The `every-seat-heard` instruction means the final vote waits until the new seat has sat through a closed debate round.

## 3. A quick council, end to end

Specimen: keep or remove vsect's computed `closed:` / `owner:` lines. Charter: `charters/quick-council.json` revision 2 — three seats (tool-design, cold-reader, git-workflow), mechanical chair, quorum 2 of 3, the referral a resolution with a blank. Conventions, declared so the count is honest: one floor act per round (I1); every window-opening transition is followed by a round; a motion is stated the round after it is revealed; votes and consents are their own rounds; chair duties are derived events at round boundaries (shown as ′ rows, which are not rounds).

| r | state | actor | act | admitted by (as compiled, with the charter's patches) |
|---|---|---|---|---|
| 0′ | charter loaded; pin recorded | server | `charter-loaded`, `seat-credentialed` ×3 | — |
| 1 | call to order | all | roll round: three `present` | Sample Rules A §4; 40:11 |
| 1′ | 3 present ≥ quorum 2 | server | `quorum-ok`; announce *the* special order [41:57]; place the referred resolution-with-blank before the council — a **declared seating rule** (the book's nearest text: the secretary furnishes the referred paper [50:19]; the committee drafts [51:4]); `stated`; Q pushed as a main frame | charter `agenda.DERIVED`; patched `state-question` admits `moved` |
| 2 | Q pending with a blank; debatable | all asked; tool-design wins the floor (earliest claim, then rotation) | `claim-floor` + `speak-in-debate`: the trail is the truth; a typed line is a second copy | `floor-open`→`recognize`; `debatable(top)`; `stated` occurred |
| 3 | | cold-reader wins (has not spoken; 42:9(2) as a filter on second speeches, so tool-design cannot claim again while cold-reader claims) | speaks: without the line a grep reader sees nothing | as above |
| 4 | | git-workflow | speaks: the drift came from the commit seam, not the line | as above |
| 5 | blank open; suggestions are not floor acts [12:96: no recognition needed] | all (batched, commuting) | `fill-blank-proposal` ×2: "keep and sync" (tool-design, the parent's first), "remove; `vsect read` shows closure" (cold-reader); git-workflow passes | registry `fill-blank-proposal`: one proposal per member [12:97]; no second |
| 6 | proposals stated | all; nobody claims the floor | pass round | — |
| 6′ | a closed round with zero debate claims after the last speech | server | `put-question` on the proposals, in the order 12:107–111 give (names/places in proposal order): **vote round on "keep and sync"** | duty-sequencing rule; `put-question` third disjunct |
| 7 | vote in progress | all | signed ballot: 1–2 rejected (reasons are pointers to rounds 2–4) | `vote`: `voting-in-progress`; chair never votes (`chair_participates: false`) |
| 7′ | | server | announce; next proposal: vote round on "remove" | `announce-result`; 12:94 |
| 8 | | all | 2–1 adopted; the blank is filled; **the vote that fills a blank does not decide the question** [12:94]; the completed resolution is pending | |
| 9 | completed resolution pending | git-workflow wins the floor | `move amend`: insert "after migrating the 189 drifted files" — germane (default true) | `move`: rank-admissible; `applicable` (amendable target); no order precluding; quorum; floor; not dilatory |
| 9′ | revealed | server | states the amendment (the one-round window of 4:19 having passed) | patched `state-question` |
| 10 | amendment pending | all; cold-reader | `move commission-inquiry`: a fork of the vsect expert to count files opened without vsect in a week — Q and the amendment stay pending | charter `add_motion` through `move`: `applies_to` (t = null); `< max_concurrent`; quorum; floor |
| 11 | | all | consent round: no objection → adopted by unanimous consent [4:58–59]; effect `spawn-subcommittee` | consent round; `object-to-unanimous-consent` available to every seat |
| 12 | amendment pending; inquiry running | all; tool-design | `move recess` until `event/subcommittee-reported` (03 §4) | privileged recess [20:5]; majority |
| 13 | | all | vote round: 3–0 → `recessed` | |
| — | `subcommittee-reported`; the report attaches to Q | server | `resumed` at the exact point [8:7]; the delta carries the report | 20:10 |
| 14 | amendment pending | git-workflow | speaks to the amendment citing the report ("in the nature of debate", 52:28) | `speak-in-debate` |
| 15 | | all; nobody claims | pass round | |
| 15′ | | server | put the amendment: vote round | duty sequencing |
| 16 | | all | 3–0 adopted | |
| 17 | Q as amended pending | all; cold-reader | `move reconsider` on the "remove" proposal (round 8): **as compiled**, cold-reader voted yes on "remove" — in `prevailing_members`? `prevailing_members` is recorded for roll-call, and the charter classifies the signed ballot as roll-call, so yes: the first disjunct holds and the motion is in order; the committee rule's two-thirds applies unless every prevailing-side voter is present — all present → majority [37:35(3), computed from presence]. (Had cold-reader voted no, the as-compiled guard would send it to `prevailing-side-claim`, default null → a ruling round; under the F12 guard it would be refused on the record.) | `move`; `applicable` row for reconsider: `within-reconsider-window` (committee: always); `mover-was-prevailing-side` |
| 18 | | all | vote round on reconsider: 1–2 rejected | |
| 19 | Q as amended pending; `every-seat-heard` satisfied (every seat present through closed debate rounds 2–4, 14) | all; nobody claims | pass round | the instruction's conjunct (presence history, declared) |
| 19′ | | server | put the **recommendation** (the committee's vote is on what to recommend, 51:37; `effect.kind: recommendation`, F6) | duty sequencing |
| 20 | | all | signed ballot: 2–1 "recommend adoption as amended"; cold-reader's dissent is a minority view of right (`report-form`) with a pointer to round 3 | |
| 21 | disposed | git-workflow | `move rise-and-report` | charter `add_motion` through `move`; out of order while any seat claims the floor — none does |
| 22 | | all | consent round: adopted → effect: the report (charge, facts incl. the inquiry's, findings, recommendation as a resolution, per-seat votes with pointers, minority view) → `report-adopted`; the council dissolves [50:10] | 50:23 |
| parent | | Joseph | the record lands `deciders: council (3 seats) under grant; decided-by: delegated`; his act: support / ratify / rule / decline | udon ADR; 10:54 |

**Count, honestly:** twenty-two submission rounds (revision 1 said thirteen and was counting rows). Of them, six are votes or consents, four are pass rounds that exist only to satisfy "silence after being asked", three are the inquiry and recess. A council of three deciding one amended recommendation with a live inquiry costs on the order of twenty rounds of three sealed submissions. That is the price of I1 — one floor act per round — and the contention spike's standing policies ("pass unless X") are the mitigation: a seat whose policy already covers the state is not re-asked.

## 4. What the walkthrough exposed

- **The blank is the right referral shape for a choice.** Neither alternative is the drafted text; the two votes in rounds 7–8 decide the blank without deciding the question [12:94], and the germaneness fight of revision 1 never arises.
- **Rounds 6, 15 and 19 are the duty-sequencing rule made visible.** Without it a mechanical chair would have put each question the instant it was stated (feedback §3.2).
- **Round 17 shows the guard both ways.** As compiled, the prevailing-side rule is enforceable only because the charter classifies the signed ballot as roll-call; the committee branch and the exclusive recorded case (F12) are what would make it hold on every recorded vote without that classification.
- **Rounds 10–13 are the two in-meeting acts in their book shape**: commission, recess, resume where interrupted; continuing to deliberate instead of recessing is the declared departure, equally available.
- **Round 20 is the report's content**, and `report-form` is what makes the dissent part of it rather than a favor granted by leave — with its reasons at round 3, where they were answerable.
