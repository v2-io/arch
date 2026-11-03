# In-meeting acts with immediate effect, and a quick council walked end to end

*Joseph's two examples — a research subcommittee that returns without interrupting, and adding a member before continuing — as event streams through the model, with the generic route the book already provides and the charter-granted route that makes each a one-round act. Then a three-seat quick council taking a real specimen from `../../specimens-2026-11-03.md` to a recorded recommendation. Every action named is in `../../model/actions.json`, `registry.json`, or the charter's `add_action`s; every cite read whole. Labels as before.*

## 1. A research subcommittee while the question stays pending

**What it is not.** Subsidiary `commit` moves the pending question into the subcommittee's custody; the council's series is suspended [13:1, engine `serialization.commit`], and the parent's only reach is discharge or instruction (the single-custody invariant). Joseph wants the opposite: the question stays pending, the inquiry runs beside it, and its answer arrives as information.

**The book's own shape for this.** Breakout groups: "the conclusions reached by the various breakout groups are conveyed … directly to the assembly after it has been reconvened and the matter under consideration is again pending. These reports are in the nature of debate" [52:28]. And the referral itself may instruct the committee "to employ an expert consultant" [13:8(d)], and subcommittees "are responsible to and report to the committee and not to the assembly" [50:15]. So the inquiry is a subcommittee of the council, staffed by non-members only if the charter authorizes [50:15], and its return is *debate*, not a recommendation that re-pends.

**Route A — generic model, no charter grant** (available to any council; costs an admissibility round). [derived]

| round | state | act | cite |
|---|---|---|---|
| k | Q pending (main frame); debate round | seat B submits `raise-question-of-privilege` with a motion: "that a subcommittee of one, a fork of expert E, determine F and report to this council; the council to continue consideration meanwhile." Interrupt-capable when urgency justifies [19:6(3)]; in a round nobody is mid-speech, so it is "after another has been assigned the floor and before he has begun to speak" | 19:1–3, 19:6 |
| k+1 | the chair must rule `privilege-admissible` (default `null` — a ruling is required); mechanical chair → `submitted` [23:18–21]; the admissibility question is undebatable [19:6(5)] | consent round, majority: "shall the question be admitted as one of privilege (organization of the assembly, 19:7)?" | 19:7, 19:9 |
| k+2 | admitted → engine `interrupt(context)`: a new context is pushed with the motion as a main question; it is debatable and amendable [19:3]; a consent round adopts it | adopted → effect `spawn-subcommittee` (adapter) with a derived special charter: parent = this council, referred = F, roll = the named fork | 19:10, engine `context_stack` |
| k+3 | context pops; "business is resumed at exactly the point at which it was interrupted" — Q is immediately pending again, the interrupted holder re-addressed | debate continues | 19:10 |
| later | `subcommittee-reported` (input event) | the report attaches to Q as an advisory input and appears in the next round's state delta; no stack change; any seat may cite it; a recommendation in it reaches the floor only by a seat's motion | 52:28, 51:11 |

**Route B — charter-granted act** (`commission-inquiry`, one round). The parent's instruction pre-admits the privilege question and sets bounds (max concurrent inquiries; non-member staffing). The act is in order while any question is pending, needs no second [49:21(2)], is debatable only as to the desirability and details of the inquiry (the commit form, 13:7(5)), majority. On adoption the same effect fires. The pending stack is untouched. [derived; `charters/quick-council.json` overlay]

**Waiting instead of continuing.** If the council would rather not deliberate without the answer, it does one of three book things: `postpone-to-certain-time` with an event time ("until S reports", 03 §4); `recess` [20]; or `fix-time-to-which-to-adjourn` to that event [22, 9:17]. Each restores position exactly [8:7, 14:13–17].

**Dissolution of the inquiry.** It is a special committee; its final report dissolves it [50:10]. If the council rises before the inquiry reports, the inquiry dies with its parent [50:30] unless the council's report asks the parent to keep it — "appointed expressly to report at a later time" [50:30].

## 2. Adding a member before the council continues

**Authority.** The appointing power fills vacancies and adds members [13:23, 50:11]; a committee may not — "except … as authorized in … instructions given to the committee" [50:26], and the referral may authorize the committee "even to add to its own membership" [13:8(d)]. So the act exists in the book exactly when the charter grants it; without the grant the council's only move is to *ask* [52:15 "rise … to request instructions"; for an ordinary committee, a message to the parent], and the parent's act seats the member [13:23]. [derived]

**Route A — generic**: `raise-question-of-privilege` — the organization of the assembly is a privilege of the assembly [19:7] — then as in §1: admissibility round, context push, main motion "that a seat for perspective P, held by agent A, be added", adopt, pop. Without the charter's grant the adopted motion is a *request to the parent* (effect: message), not a seating.

**Route B — charter-granted** (`co-opt-member`): one round; majority; bounded by `max_total` [13:8(d)]. Effect `seat-member`: credential issued, `member-seated` event, `assembly.membership_total` incremented, quorum recomputed if its basis is a majority [40:5], fresh counters. [derived]

**What the new seat can do.** It is present once it first submits; its first state delta is the whole record. The book lets any member present vote; the charter leaves that, and the record shows when the seat joined. Whether a seat that missed the debate should be asked to read before voting is a disposition question (coordinator's open item) — the mechanism to enforce it, if wanted, is a standing rule the charter authorizes. Removal mirrors the adoption threshold [50:14]. [derived; the "may vote at once" default is the book's, kept]

## 3. A quick council, end to end

Specimen: `specimens-2026-11-03.md` Reply 1 §7 / Reply 2 §6 — keep or remove vsect's computed `closed:` / `owner:` frontmatter lines. Charter: `charters/quick-council.json` (three seats: tool-design, cold-reader, git-workflow; mechanical chair; quorum 3). Round numbers are illustrative; each row names the actions that would appear in `available_actions` for the acting seat, with the conjunct that admits them. [derived from the model; not an engine run — the engine is stage 5]

| r | state | seat / server | act | admitted by |
|---|---|---|---|---|
| 0 | charter loaded; pin recorded | server | `charter-loaded`, `seat-credentialed` ×3 | — |
| 1 | call to order | all | roll round: each seat submits `present` | Sample Rules A §4; 40:11 |
| 1' | 3 present ≥ quorum 3 | server (chair duty) | `quorum-ok`; announce *the* special order [41:57]; **state Q** — no motion needed [51:40]; Q pushed as a main frame; `stated` | 47:7(2),(4) |
| 2 | Q pending, debatable | all | open round. tool-design: `claim-floor`+`speak-in-debate` ("state is computed, never typed; the lines are a second copy"); cold-reader: speaks ("grep readers lose `closed:` without it"); git-workflow: `pass` | `floor-open`, `debatable(top)`, `stated` occurred; counters < limit (no limit, 49:21(3)) |
| 3 | both spoke once | git-workflow | `move amend`: "strike 'keep … and sync' and insert 'remove the lines; vsect read shows closure marks'" (germane: default true unless challenged) | `move`: rank-admissible (amend over main), `applicable` (amendable target, germane), no order precluding, in good standing |
| 3' | amendment revealed (one-round window, 01 §3 h) | server | next round: **states the amendment**; pushed above Q | 4:15–16 |
| 4 | amendment immediately pending | cold-reader | `raise-point-of-order`: "the amendment is not germane — it reverses the question" | timely: the breach (stating) is visible this round, 23:5 |
| 4' | adjudication raised(`germane`, amendment) | server | mechanical chair submits [23:18–21]; **ruling round**, majority, debatable per appeal rules: 2 of 3 say germane (an amendment to reverse is germane — 12:16-21's own example class) | adjudication `submitted → final` |
| 5 | amendment pending; debate | all | debate round; tool-design proposes a secondary amendment ("…and the `read` output shows closure in the first line") | amend applied to amend; degrees cap not hit [12:11–13] |
| 6 | secondary pending | all | all `pass` → exhaustion | `put-question` third disjunct: no debate claims |
| 6' | | server | puts the secondary: sealed vote round → 3–0 adopted, reasons recorded | 4:34–49, 48:5(2c) |
| 7 | primary pending | all | all pass → put → 2–1 adopted (cold-reader no: "removes the one thing a cold reader had"; reason recorded) | majority [12:7(7)] |
| 8 | Q as amended (recommended) pending | git-workflow | `commission-inquiry`: "a fork of the vsect expert: how many files are opened without vsect in a week (shell history)?" — Q stays pending | charter add_action; `< 2 active` |
| 8' | | all | consent round: no grounded objection → adopted (unanimous consent, 4:58) → effect `spawn-subcommittee` | consent round |
| 9 | Q pending; inquiry running | all | `recess` until `event/subcommittee-reported` (event-anchored, 03 §4) | privileged recess [20:5], majority |
| — | `subcommittee-reported` fires; the report attaches to Q | server | `resumed` at the exact point [8:7]; the delta carries the report | 20:10 |
| 10 | Q pending | all | debate on the facts; cold-reader moves to reconsider the primary amendment — in order: committee rules, any member not on the losing side (it *was* on the losing side → **not** in order; the model refuses with the why-not: `mover-was-prevailing-side` false, computed from the roll call) | 37:35(2); 01 §6 |
| 10' | | tool-design | moves to reconsider instead (voted yes; not losing side); two-thirds unless all prevailing-side voters present — all present → majority [37:35(3)]; vote round: 1–2 rejected | computed threshold |
| 11 | Q as amended pending | all | all pass → put the **recommendation**: 2–1 "recommend adoption as amended", reasons recorded; cold-reader's dissent is a minority view of right (charter `report-form`) | 51:37, 51:64–71 |
| 12 | disposed | git-workflow | `move-rise-and-report` → consent round → adopted | 50:23 |
| 12' | | server | emits the report: charge, facts (incl. the inquiry's), findings, recommendation as a resolution, per-seat votes with reasons, minority view; `report-adopted`; the council **dissolves** [50:10]; the record persists | 51:7 |
| parent | | Joseph | the record lands `deciders: council (3 seats) under grant; decided-by: delegated`; his act: support / ratify / rule / decline | udon ADR; 10:54 |

Thirteen rounds of three sealed submissions each, plus six derived chair events, for a decision with an amendment, a germaneness ruling, a secondary amendment, a live inquiry, a reconsideration attempt refused on the record, and a dissent preserved. Nothing in the stream needed a rule the model lacks except the three charter-added acts, the event-anchored recess, and the `ctx.subcommittees` record.

## 4. What the walkthrough exposed

- **Round 3′ is the maker's protection.** Stating one round after revealing keeps 4:19 alive; a server that states instantly must route modification through consent [33:11–19]. Either is a charter line; the first is cheaper for agents, who rarely need to withdraw but often need to fix wording.
- **Round 10 is the gain.** A human council could not have refused the reconsider on the record; the book admits its prevailing-side rule is honor-system (`RONR-OBSERVATIONS.md` §3). Here the refusal carries its justification tree.
- **Rounds 8–9 are the two in-meeting acts in their natural order**: commission, then recess until the answer, then resume where interrupted. Continuing to deliberate instead of recessing is equally in order — the choice is the council's, by majority on the recess.
- **Round 11 is the report's content**, and the charter's `report-form` instruction is what makes the dissent part of it rather than a favor granted by leave.
