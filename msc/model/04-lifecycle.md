# Component 4 — Question Lifecycle (timed statecharts with history)

*Stage 3 model per `INTERFACE.md`. Data format: prose spec + fenced JSON blocks in this file (no sibling data files). Scope: the lifecycle of a **question object** — chiefly a main motion and its adhering bundle — from introduction to final disposition and beyond (force-and-effect, reconsideration, renewal). The pending *stack* itself is `engine:` territory; the floor cycle that fires most events is `protocol:` territory; what may be scheduled when is `scheduler:` territory. This component owns the states a question can be in, the legal transitions between them, the clocks that expire them, and the history (resume-where-interrupted) semantics.*

## 1. Overview

Every question is an object instantiated when a member makes a motion `[4:4]` and destroyed only by falling out of all constitutive registers (e.g., withdrawn: treated as never made `[33:18]`). Its statechart has four regions running in parallel:

- **R1 Disposition** — the core region: introduction → pending → temporarily disposed ↔ pending → finally disposed.
- **R2 Reconsiderability** — a window opened by the disposition vote and closed by clock or by events that make reconsideration impossible `[37:9(2)]`.
- **R3 Effect** — for adopted questions: in force until rescinded/amended, or executed; suspended while a reconsideration is pending on it `[37:11]`, `[10:26(4)n3]`.
- **R4 Control** — a derived property, `within-control-of-assembly`, true exactly when the question is *temporarily but not finally disposed of* `[9:11]`, `[38:8]`; it gates conflicting/substantially-same main motions `[10:26(5)]` and renewal `[38:8]`.

## 2. States (R1 Disposition)

```json
{
  "component": "lifecycle",
  "kind": "states",
  "states": [
    {"id": "moved",                "phase": "introduction", "cite": "[4:4]",
     "note": "made but not yet seconded/stated; property of the maker [4:19, 33:12]"},
    {"id": "seconded",             "phase": "introduction", "cite": "[4:9-14]"},
    {"id": "pending",              "phase": "active",       "cite": "[4:3]",
     "note": "stated by chair; on the engine:stack. Sub-flag: immediately-pending vs stacked [5:7]"},
    {"id": "committed",            "phase": "temporary",    "cite": "[13:1]",
     "note": "in committee's hands; carries pending amendments only [10:8(2), 13:19]"},
    {"id": "postponed-definitely", "phase": "temporary",    "cite": "[14:1]",
     "note": "becomes scheduler:order/general or scheduler:order/special [14:13-14]"},
    {"id": "tabled",               "phase": "temporary",    "cite": "[17:5]",
     "note": "serialized bundle in secretary's care; restorable at will of majority [17:1]"},
    {"id": "unfinished-business",  "phase": "temporary",    "cite": "[21:7(b), 41:21-23]",
     "note": "carried over by adjournment ending session, next regular session within quarterly-interval"},
    {"id": "reconsider-pending",   "phase": "temporary",    "cite": "[37:11]",
     "note": "a made-but-undecided reconsider holds the underlying vote's effect suspended"},
    {"id": "adopted",              "phase": "final",        "cite": "[4:43]"},
    {"id": "rejected",             "phase": "final",        "cite": "[4:43]"},
    {"id": "suppressed-for-session", "phase": "final",      "cite": "[11:3, 26:5]",
     "note": "killed by postpone-indefinitely or sustained objection-to-consideration; final for the session only"},
    {"id": "withdrawn",            "phase": "final",        "cite": "[33:18]",
     "note": "as if never made; freely renewable [38:2]"},
    {"id": "died",                 "phase": "final",        "cite": "[17:8, 21:7(c)]",
     "note": "table expiry, or fell to ground at session end beyond quarterly-interval / term turnover; renewable as new"},
    {"id": "ruled-out-of-order",   "phase": "final",        "cite": "[4:16-17]",
     "note": "subject to appeal via catalog:motion/appeal [24]"},
    {"id": "rescinded",            "phase": "final",        "cite": "[35:1]",
     "note": "terminal for a previously adopted question's effect"}
  ]
}
```

`NOTE(judgment):` RONR does not name "suppressed-for-session" as one state; it treats indefinite postponement `[11:3]` and a sustained objection `[26:5]` separately but with identical lifecycle consequences (dead this session, renewable next `[38:3(2)]`, negative/affirmative-vote reconsideration asymmetries handled in R2). Merging them is a modeling choice; the originating event is retained on the object.

## 3. Transitions (R1)

Events are `protocol:` core-vocabulary events unless qualified. Guards reference `catalog:` predicates; the legality of *making* the disposing motion at all (precedence, applicability) is computed by `engine:` + `catalog:` and is **not** re-checked here — this region assumes a legally adopted disposing motion and records its lifecycle effect.

```json
{
  "component": "lifecycle",
  "kind": "transitions",
  "transitions": [
    {"from": "(start)", "on": "made",      "to": "moved",    "cite": "[4:4]"},
    {"from": "moved",   "on": "withdrawn", "to": "withdrawn","cite": "[4:19, 33:12]", "note": "maker's unilateral right before stating"},
    {"from": "moved",   "on": "lifecycle:event/no-second",   "to": "died", "cite": "[4:10]", "note": "freely renewable [38:2]"},
    {"from": "moved",   "on": "seconded",  "to": "seconded", "cite": "[4:9]"},
    {"from": "seconded","on": "stated",    "to": "pending",  "cite": "[4:15-16]", "note": "ownership passes from maker to assembly [4:19]"},
    {"from": "seconded","on": "ruled-out-of-order", "to": "ruled-out-of-order", "cite": "[4:16-17]"},

    {"from": "pending", "on": "put",       "to": "adopted | rejected", "cite": "[4:41-49]",
     "guard": "catalog:threshold satisfied per motion's SDC 7"},
    {"from": "pending", "on": "referred",  "to": "committed", "cite": "[13]",
     "note": "drops adhering postpone-indefinitely [11:4, 13:19]; pending reconsiders of adhering motions thereafter ignored [13:7(2), 37:34]; exhausts debate-limit and previous-question orders [13:21, 15:18, 16:11]"},
    {"from": "pending", "on": "postponed", "to": "postponed-definitely", "cite": "[14]",
     "guard": "lifecycle:guard/postpone-horizon", "note": "carries whole adhering bundle [14:18]"},
    {"from": "pending", "on": "tabled",    "to": "tabled",   "cite": "[17:5]",
     "note": "carries whole adhering bundle; a pending amendment-to-something-previously-adopted goes alone [17:5]"},
    {"from": "pending", "on": "lifecycle:event/postponed-indefinitely", "to": "suppressed-for-session", "cite": "[11:3]"},
    {"from": "pending", "on": "lifecycle:event/objection-sustained",    "to": "suppressed-for-session", "cite": "[26:5]",
     "note": "original main motions only, before consideration begun [26:2(1-2)]"},
    {"from": "pending", "on": "withdrawn", "to": "withdrawn", "cite": "[33:13-18]", "note": "requires leave of assembly; adhering motions evaporate with it [33:16]"},

    {"from": "pending", "on": "recessed | lifecycle:event/interrupted", "to": "pending", "cite": "[8:7, 18:7, 19:10, 41:53]",
     "note": "history: not a state change; see §5"},
    {"from": "pending", "on": "meeting-ended", "to": "pending", "cite": "[21:7(a)]",
     "guard": "session continues (adjourned meeting set)", "note": "resumes at exact point, after minutes [9:19]"},
    {"from": "pending", "on": "session-ended", "to": "unfinished-business", "cite": "[21:7(b)]",
     "guard": "lifecycle:guard/carryover-eligible"},
    {"from": "pending", "on": "session-ended", "to": "died", "cite": "[21:7(c)]",
     "guard": "NOT lifecycle:guard/carryover-eligible", "note": "falls to ground; introducible as new"},

    {"from": "committed", "on": "lifecycle:event/reported", "to": "pending", "cite": "[51:36-44]",
     "note": "restored with referred amendments first in order [51:42-43]; debate/PQ orders exhausted even same session [13:21, 16:12]; same-question amendment prohibitions lifted [13:20]"},
    {"from": "committed", "on": "lifecycle:event/discharged", "to": "pending | died", "cite": "[36:10-11]",
     "note": "pending automatically if referred while pending via subsidiary commit and no later time set; if task was assigned by main motion, a new main motion is needed or it dies [36:11]"},
    {"from": "committed", "on": "session-ended", "to": "committed", "cite": "[9:8, 21:7(c)]",
     "note": "referral survives session boundaries and term turnover"},

    {"from": "tabled", "on": "taken-from-table", "to": "pending", "cite": "[34:6]",
     "note": "restored in same condition so far as possible; an adhering postpone whose time has passed is ignored [34:6]"},
    {"from": "tabled", "on": "lifecycle:timer/table-expiry", "to": "died", "cite": "[17:8, 34:3]"},

    {"from": "postponed-definitely", "on": "lifecycle:event/time-arrived", "to": "pending", "cite": "[14:13-17, 41:19]",
     "note": "via scheduler; announced by chair, no motion needed [14:17]"},
    {"from": "postponed-definitely", "on": "session-ended", "to": "unfinished-business | died", "cite": "[21:7(b-c), 41:23(c)]",
     "guard": "lifecycle:guard/carryover-eligible"},
    {"from": "unfinished-business", "on": "lifecycle:event/reached-in-order", "to": "pending", "cite": "[41:23-24]"},

    {"from": "adopted | rejected", "on": "reconsidered", "to": "pending", "cite": "[37:7, 37:19]",
     "guard": "R2 window open", "note": "restored to exact position the moment before the original vote; original vote canceled [37:12, 37:19]"},
    {"from": "adopted", "on": "lifecycle:event/rescinded", "to": "rescinded", "cite": "[35:1]",
     "guard": "NOT lifecycle:guard/unrescindable"},
    {"from": "adopted", "on": "lifecycle:event/amended-as-adopted", "to": "adopted", "cite": "[35:1]",
     "note": "self-loop; text modified, force continues"}
  ]
}
```

### Guards declared here

- `lifecycle:guard/postpone-horizon` — target time ≤ end of next regular session when that session is within `quarterly-interval`; otherwise ≤ end of present session; and postponement must not be tantamount to killing the question `[14:6, 14:9]`.
- `lifecycle:guard/carryover-eligible` — next regular business session begins within `quarterly-interval` AND no specified portion of the body's membership ends its term before then `[9:8-9, 21:7(b-c)]`. (Referral to committee is the sole carryover channel when this fails `[9:8, 9:10]`.)
- `lifecycle:guard/unrescindable` — effect already fully executed or impossible to undo; contract with notified party; resignation acted on / election-or-expulsion result with person present or notified; or question reachable instead by a live reconsider `[35:6]`.

## 4. R2 — Reconsiderability window

Opened at the disposition vote; per-motion applicability (which votes are reconsiderable at all, affirmative-only, negative-only) is `catalog:` data (SDC 8, Table VII) — this region owns the **clocks and the hold**.

```json
{
  "component": "lifecycle",
  "kind": "timers",
  "timers": [
    {"id": "reconsider-make-window", "start": "disposition vote", "expires": "end of same-day, or next-business-day within the same session", "cite": "[37:8(b), 37:10(b)]",
     "note": "no time limit in standing/special committees [37:35(1)]"},
    {"id": "reconsider-callup-window", "start": "reconsider made but not taken up", "expires": "end-of-session; extended through next-regular-session (or an intervening special meeting called for it) when within quarterly-interval", "cite": "[37:15]"},
    {"id": "reconsider-enter-minutes-callup", "start": "motion made (same day as vote only)", "expires": "not callable same day (exception: last-day-but-not-last-meeting of a final session); otherwise as reconsider-callup-window", "cite": "[37:46-47]"},
    {"id": "table-expiry", "start": "tabled", "expires": "end of next-regular-session when within quarterly-interval, else end-of-session; term turnover shortens to end-of-session", "cite": "[17:8, 34:3, 34:3n1]"},
    {"id": "vote-challenge-window", "start": "result announced", "expires": "immediately after announcement, before debate or business intervenes (continuing-breach exceptions aside)", "cite": "[45:9, 23:5-6]"},
    {"id": "renewal-unblock", "start": "final disposition without adoption", "expires": "opens at start of any later session, unless the question went over as not finally disposed of", "cite": "[38:3(2), 38:8]"}
  ]
}
```

While a made reconsider is undisposed, the underlying question sits in `reconsider-pending`: all action under the vote is suspended `[37:11]`; termination of the hold (reconsider decided, withdrawn, expired, or fallen to ground) either cancels the original vote (adopted reconsider `[37:12]`) or retroactively revives it in full force `[37:11-12, 21:7(c)n6]`. The window closes early — regardless of clock — when the vote becomes unreachable: provisions partly executed, contract-with-notice, election final per `[46:46]`, or a same-result path available by majority without notice `[37:9(2)]`.

## 5. History semantics (the statechart's H\* mechanism)

Interruption never destroys position. On `resumed`, consideration continues **at the exact point where it was interrupted**, with the interrupted speaker re-assigned the floor where applicable: recess `[8:7, 20:10]`, orders of the day completed `[18:7, 18:11]`, question of privilege disposed of `[19:10, 19:17]`, special-order preemption `[41:53]`, adjourned meeting continuing a session `[9:19, 21:7(a)]`.

What history does **not** preserve across boundaries:

- **Day boundary** — per-member debate rights renew each day per question (`protocol:` counters reset) `[43:12, 14:19, 34:6]`.
- **Session boundary** — debate-limit and previous-question orders are exhausted `[15:18, 16:11]`; same-session "same question" amendment prohibitions lapse (amendments may be re-offered on reconsideration or take-from-table at a later session) `[34:6n2, 37:19n6]`; renewal unblocks `[38:3(2)]`.
- **Referral** — exhausts debate orders even within the session `[13:21, 16:12]` and lifts intra-session same-question amendment bars `[13:20]`.

`NOTE(judgment):` I model "restored in the same condition so far as possible" `[17:7, 34:6]` as: the engine restores the serialized frame verbatim, then the lifecycle applies the boundary-crossing erasures above as a post-restore normalization step. RONR states the erasures rule-by-rule; the two-phase reading is a modeling convenience.

## 6. R3/R4 — Effect and control

- Adopted main motions have continuing force until a termination written into them arrives or they are rescinded/amended `[10:26(4)n3]`; conflicting later motions are null and void unless adopted by the rescind-grade threshold `[23:6(b), 39:5]`.
- `within-control-of-assembly` ≡ R1 state ∈ {pending, committed, postponed-definitely, tabled, unfinished-business, reconsider-pending} `[9:11, 38:8]`. While true: no main motion may conflict with or substantially duplicate the question `[10:26(5)]`; the listed extraction routes (discharge, take-from-table, suspend-the-rules, reconsider call-up) are the only ways back in `[10:26(5)(a-d)]`.
- Actions null and void ab initio (conflict with bylaws/law, absentee-protection or basic-rights violations) never validly enter `adopted` at all; a continuing-breach point of order can declare this at any time while effect continues `[23:6, 23:9]`.

## 7. Test vectors (Form-and-Example traces exercising this component)

- `[11:6-8]` — postpone-indefinitely adopted and rejected paths into/around `suppressed-for-session`.
- `[14:21-22]` — postpone made a special order; `postponed-definitely` → scheduler → `pending` at 11 A.M.
- `[17:20-24]` + `[34:8-10]` — table, right-of-way on return, restore semantics.
- `[21:17-19]` — adjournment with business pending; `[21:7(a-c)]` branch table.
- `[36:12-15]` — discharge with and without a specified later time.
- `[37:36-45]` — reconsider made/called-up/adopted; restoration to pre-vote position.
- `[37:46-52]` — reconsider-and-enter-on-the-minutes clock behavior.
- `[51:37-44]` — committee reports back: automatic re-pending, referred-amendment ordering, indefinite-postponement recommendation sequencing.
- `[9:7]` worked dates — quarterly-interval arithmetic for `carryover-eligible`, `table-expiry`, `postpone-horizon`.
