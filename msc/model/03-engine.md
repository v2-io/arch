# Component 3 — Pending-Question Stack Machine

*Stage-3 model per `INTERFACE.md`. Data format: fenced JSON blocks inline. External references qualified as `component:kind/id`. Internal names unqualified.*

## 3.0 Scope

This component owns the dynamics of the pending series: what may become pending while other questions are pending, in what order pending questions are decided, and how pending bundles are serialized out of (and restored into) the live stack. It consumes the motion registry and Standard Descriptive Characteristics from `catalog:*`, rule-state from `rules:*`, per-question object state from `lifecycle:*`, and emits/consumes the core event vocabulary via `protocol:*`. It does not model debate turn-taking (component 5), vote arithmetic (component 1), or when scheduled business preempts (component 7) — it exposes the operations those components invoke.

Foundational principle: only one question is considered at a time; once a motion is before the assembly it must be adopted, rejected, or otherwise disposed of before other business (with privileged exceptions) is introduced `[5:4]`. The pending series exists because secondary motions may be made and must be decided while a main motion remains pending `[5:5–7]`.

## 3.1 Core structures

### 3.1.1 Frame

A **frame** is one pending question: a reference to a `lifecycle:*` question object plus its stack role.

```json
{
  "frame": {
    "question_ref": "lifecycle:question/<uuid>",
    "motion_id": "catalog:motion/<id>",
    "class": "M | S | P | I | B | M/B",
    "applied_to": "<frame-id | null>",
    "adheres_to": "<frame-id | null>",
    "effective_rank": "<rank token — see 3.2.2>",
    "debate_state_ref": "protocol:counters/<id>",
    "orders_in_force": ["rules:order/<id>", "..."]
  }
}
```

- `applied_to`: the frame this motion operates on (e.g., an amendment's target) `[6:6(1)]`. Null for privileged motions, which relate to no pending question `[6:11]`.
- `adheres_to`: adherence per `[10:35]` — the motion must be decided before its target can be decided, and travels with the target when the target is serialized (3.4). `applied_to` and `adheres_to` usually coincide; they diverge for non-adhering incidentals (e.g., a point of order not adhering to the main question `[23:2(1)]`) and for privileged frames, which never adhere `[10:35]`.

### 3.1.2 The pending stack

An ordered sequence of frames; bottom is normally a main-class frame (`M`, `M/B`, or `B` behaving as main rank `[6:26]`); the top frame is the **immediately pending question** `[5:7]`. Push order equals the order motions were stated by the chair; a motion is pending only once stated `[4:3]`.

### 3.1.3 Business contexts (stack of stacks)

Certain events interrupt pending business without disposing of it: an order of the day taken up at its hour `[18:7]`, an admitted question of privilege `[19:10]`, a special order interrupting at its hour `[41:53]`. The interrupting matter is itself a main motion and may accumulate its own secondary motions, independent of the interrupted series (its disposal by commit/postpone/table "is independent of, and does not affect, any other matter that they may have interrupted" `[13:7(2)]`, `[14:4(2)]`, `[17:3(2)]`).

Model: a **context stack** of pending stacks. Interruption pushes a fresh context; completing the interrupting business pops it and consideration resumes at the exact point of interruption `[18:7]`, `[19:10]` — the resume point (who had the floor, etc.) is `lifecycle:*` history plus `protocol:*` state, not engine state. Recess and adjournment freeze the whole context stack rather than pushing a context `[8:7]`, `[21:7(a)]`.

`NOTE(judgment):` RONR does not use a nested-context vocabulary; it says interrupted business "is resumed where it left off." The context stack is the minimal structure that makes 13:7(2)/17:3(2) independence and 18:7/19:10 resumption both literal.

## 3.2 Push legality

### 3.2.1 The decision function

```
may-push(motion, target_frame, context) =
      class-admissible(motion, stack)            // 3.2.2 rank test, or incidental/reconsider special rule
  AND applicable(motion, target_frame)           // catalog SDC 2 [7:2]
  AND floor-admissible(motion, protocol-state)   // catalog SDC 3 via protocol:* [7:2]
  AND guard-no-order-precluding(motion, top-frames)   // 3.2.4
  AND rules-permit(motion, rules-state)          // rules:* (suspensions, special rules, bylaws conflicts [10:26])
  AND guard-not-dilatory(motion)                 // [39:1–4]
  AND guard-not-same-question(motion)            // [10:26(3–5)], [12:25], [38:3(1)]
```

Guards declared here:

- `no-question-pending`: the pending stack of the active context is empty — precondition for main-class motions `[4:4]`, `[10:8(1)]`.
- `no-order-precluding`: no unexhausted order for `catalog:motion/previous-question` or a debate-closing form of `catalog:motion/limit-extend-debate` blocks this motion — the previous question, once ordered, precludes all subsidiaries except `lay-on-table` `[16:2]`; an order closing debate at a set time precludes `commit` and `postpone-to-certain-time` `[15:11]`; a bare speech-length limitation precludes nothing `[15:10]`.
- `not-dilatory`: motion is not absurd, frivolous, or obstructive; chair-adjudicated `[39:1–4]`.
- `not-same-question`: motion does not present substantially the question already decided this session or held within the assembly's control `[10:26(3–5)]`, `[38:3]`; for amendments, the same-content-and-effect test `[12:25]`.

### 3.2.2 Ranks

Rank tokens for the thirteen ordered motions, low → high (`catalog:*` is authoritative for the list; the ordering relation is component 2's partial order) `[5:11]`, `[t3–t5]`:

`main < postpone-indefinitely < amend < commit < postpone-to-certain-time < limit-extend-debate < previous-question < lay-on-table < call-for-orders-of-the-day < raise-question-of-privilege < recess < adjourn < fix-time-to-which-to-adjourn`

Class-admissibility for ranked motions: pushable iff its effective rank exceeds the effective rank of every frame currently pending in the active context (equivalently, of the top frame) `[5:8–9]`, subject to per-motion conditions carried by `catalog:*` SDC 1 (e.g., privileged `adjourn`/`recess`/`fix-time` lose privileged status under the conditions of `[20:3]`, `[21:3]`, `[22:3]` and are then main-class).

**Rank modification — Amend:** an amendment applied to a ranked motion above `amend` (e.g., to `postpone-to-certain-time`, `recess`, `fix-time-to-which-to-adjourn`) takes precedence over its target even though its base rank is lower `[6:7]`, `[12:7(1b)]`. So `effective_rank` is positional: `rank(amend applied to F) = just-above(rank(F))`. Secondary amendment likewise sits just above the primary it amends; no third degree `[12:11–12]`.

**Degrees-of-amendment cap:** at most one primary and one secondary amendment pending per amended target at a time `[12:13]` (relaxed by the filling-blanks device, which is `catalog:*` data and outside the stack cap `[12:92–95]`).

### 3.2.3 Incidental motions

Incidentals have no rank among themselves `[5:12]`, `[6:19]`. Class-admissibility is the guard `legitimately-incidental`: the motion arises out of a pending question, a question just pending, or business at hand, per its own SDC conditions `[6:15–18]`. When legitimately incidental, it takes precedence over the question(s) it arises from and must be decided first `[6:18]`; it yields to motions of higher rank than the question it arose from, not to lower ones `[6:21]`, and generally yields to privileged motions and (if adhering) to `lay-on-table` `[6:19]`, `[23:2(1)]`.

### 3.2.4 Reconsider — split rank

`catalog:motion/reconsider` has two ranks `[6:26(5)]`, `[37:9]`:

- **Making**: outranks everything; may be made while any question is pending and even after a vote to adjourn before the chair declares adjournment `[37:9(1a)]`. If not then considerable, the engine records it against the target vote (a lifecycle effect — suspends execution of the reconsidered vote `[37:11]`) without pushing a frame.
- **Consideration**: takes only the rank of the motion to be reconsidered `[37:9(1b)]`. When called up (or reached in series order `[37:27]`), it is pushed at that effective rank. A reconsider pending within a series slots at its target's rank position and is taken up when all higher frames are disposed of `[37:27]`.

`reconsider-enter-on-minutes` outranks plain reconsider at making time and cannot be called up the day made (exceptions per `catalog:*`) `[37:46–47]`.

### 3.2.5 Objection to consideration — timing window

`objection-to-consideration` is pushable only against an original main motion before debate has begun and before any subsidiary except `lay-on-table` has been stated `[26:2(1)]`. Window state comes from `protocol:event/debate-opened` and stated-subsidiary history.

## 3.3 Disposal: the unwind rule

The top frame is decided first; frames are voted in reverse order of stating `[10:33]`. After the top frame is disposed of, the question recurs on the new top frame, which reopens to debate/amendment if debatable `[10:33]`.

**Series-halting adoptions** `[10:34]`: adoption of any of the following disposes of (or suspends) the rest of the series, which is not further voted on now:

| Adopted frame | Effect on remaining series |
|---|---|
| `postpone-indefinitely` | main question killed for the session; series ends `[11:3]` |
| `commit` | bundle → committee (3.4.3); series suspended `[13:1]` |
| `postpone-to-certain-time` | bundle → scheduler as order of the day (3.4.2) `[14:13]` |
| `lay-on-table` | bundle → table (3.4.1) `[17:5]` |
| `recess` / `adjourn` | context stack frozen; disposition of pending business per `[21:7]` via `lifecycle:*`/`scheduler:*` |
| `objection-to-consideration` sustained | main question dismissed for the session `[26:5]` |

Under an order for the previous question covering several frames, voting proceeds top-down without debate; adoption of a postpone/commit/postpone-indefinitely within the series still halts further voting `[16:9]`.

**Other pops:** rejection of the top frame simply recurs to the next frame. Withdrawal of a motion removes its frame; anything adhering to it ceases to be before the assembly with no further disposition `[33:16]`, `[33:18]`. Withdrawal of the main frame clears the series.

## 3.4 Serialization and restoration (bundles)

A **bundle** is a frame together with everything adhering to it `[10:35]`, serialized out of the live stack as a unit and restorable as a unit ("everything is in the same condition, so far as possible, as it was" `[17:7]`, `[34:6]`).

```json
{
  "bundle": {
    "kind": "tabled | postponed | committed | reconsider-held",
    "frames": ["<serialized frames, bottom-first, adherence links intact>"],
    "orders_in_force": ["rules:order/<id>"],
    "created": "protocol:event/<tabled|postponed|referred>",
    "expiry_ref": "lifecycle:timer/<id>"
  }
}
```

### 3.4.1 Lay on the table → take from the table

The entire adhering series goes to the table together; nothing adhering can be left behind, and non-adhering frames (e.g., a pending `recess`) do not go `[17:3(2)]`, `[17:5]`, `[10:35]`. Restoration by `take-from-table` reinstates the series with the same immediately pending question `[34:6]`. Expiry (question dies on the table) is a `lifecycle:*` timer against `end-of-session` / `next-regular-session` within `quarterly-interval` `[17:8]`, `[34:3]`. Debate-right renewal and order-exhaustion on restoration are `protocol:*`/`lifecycle:*` effects `[34:6–7]`.

### 3.4.2 Postpone to a certain time

Same bundle semantics: adhering subsidiaries (`postpone-indefinitely`, `amend`, `commit` pending below it), incidental `division-of-question`/`consideration-by-paragraph`, and adhering debatable appeals/points of order are postponed with the main question and restored in the same condition at the set time `[14:18]`. The bundle is handed to `scheduler:queue/general-orders` or `scheduler:queue/special-orders` per the adopted form `[14:13–14]`. A pending `postpone-to-certain-time` in a restored bundle whose set time has passed is ignored on restoration `[34:6]`.

### 3.4.3 Commit (referral) — lossy serialization

Referral serializes a *reduced* bundle `[10:8(2)]`, `[13:19]`:

- carried: main question, pending amendments, pending `division-of-question` / `consideration-by-paragraph`, adhering debatable appeals / submitted points of order;
- dropped: a pending `postpone-indefinitely` (ignored; adoption of commit implies the assembly declines it `[11:4]`);
- ignored thereafter: any reconsider of an adhering subsidiary/incidental made but not taken up `[13:19]`, `[37:34]`.

On report, the amendments that were pending are re-presented first, in their original order, before new amendments of the same degree `[13:20]`, `[51:36–43]`; referral exhausts debate-limit and previous-question orders even within the same session `[13:21]`, `[16:12]`.

### 3.4.4 Reconsider-held votes

The making of a reconsider (3.2.4) creates a `reconsider-held` record suspending action under the target vote until the reconsider is decided, withdrawn, or its call-up window lapses `[37:11]`. Not a stack frame until taken up.

## 3.5 Engine operations (external API)

| Operation | Caller | Effect |
|---|---|---|
| `try-push(motion, target)` | protocol (on `stated`) | 3.2 legality; push frame |
| `dispose(frame, outcome)` | protocol (on `adopted`/`rejected`/`withdrawn`/`ruled-out-of-order`) | 3.3 unwind |
| `serialize(kind)` | 3.3 series-halting adoptions | 3.4 bundle out |
| `restore(bundle)` | protocol (`taken-from-table`, `called-up`), scheduler (postponed time reached) | reinstate series |
| `interrupt(context)` / `resume()` | scheduler (special order), protocol (question of privilege, orders of the day) | 3.1.3 context push/pop |
| `record-reconsider(target-vote)` | protocol | 3.4.4 |

## 3.6 Test vectors (Form-and-Example traces)

- `[10:34]` — the eight-motion series (main; postpone-indefinitely; amend; commit; postpone; ballot-vote motion; lay-on-table; recess): push legality at each step, reverse-order unwind, halt-on-adoption rows of 3.3.
- `[6:7]` — amend applied to a pending `postpone-to-certain-time`: effective-rank modification while `postpone-indefinitely`/`amend`/`commit` (against the main question) are precluded.
- `[12:83–89]` — substitute walk: primary substitute + secondary amendments to each element; ordering of amendment opportunities.
- `[16:9]`, `[16:26–27]` — previous question ordered on parts/whole of a series; competing PQ forms voted largest-scope first; voting halt when a covered `commit`/`postpone` is adopted.
- `[17:5]` + `[34:6]` — table and restore of a resolution + amendment + commit bundle, same immediately pending question.
- `[13:19]`/`[11:4]` — referral drops postpone-indefinitely, carries amendments.
- `[14:18]` + `[14:21–22]` — postpone with adhering motions; special-order form handed to scheduler.
- `[37:27]` — reconsider of a commit vote made while main + amend + lay-on-table pending: recorded, then slotted at commit's rank after table motion fails.
- `[19:14–17]` — question of privilege (executive session) interrupting a pending resolution: context push, disposal, exact resumption.
