# Logical time: what the model's clock words mean when the members are agents

*Every temporal construct in the model, inventoried mechanically (`grep` over `../../model/*.json`, 2026-11-03), classified, and mapped. The mapping lives in each charter's `logical_time`; this file is why those mappings are what they are. Labels: **[derived]**, **[declared]**, **[guess]**.*

## 1. Inventory

| Construct | Where (count) | Class |
|---|---|---|
| `["now"]` | actions 2, adjudication 0 (revision 2 counted the string, not the operator — feedback-2 §10; `schema.json` has a `clock.now` field, not the operator); the lifecycle/precedence/scheduler counts are unverified against the operator form | the clock itself |
| `["before", a, b]` | actions 1, guards 4, lifecycle 1, precedence 4, scheduler 6 | comparison of clock values — each use compares `now` or an anchor against a scheduled `hour` or a session/day anchor (one, `reconsider-make-window`, compares two anchors) |
| `["same-day", a, b]` | actions 1 (call-up of reconsider-enter-minutes) | calendar day |
| `["within-quarterly", s1, s2]` | guards 3 (carryover-eligible, postpone-horizon, schedulable-to), lifecycle 2 (reconsider-callup-window, table-expiry) | calendar months |
| `["anchor", name, session?]` | guards 4, lifecycle 9 | named anchors: `same-day`, `next-business-day`, `end-of-session`, `next-regular-session`, `quarterly-interval` |
| `clock.current_day` | actions 5 (speech-counter keys), council 2, schema 1 | calendar day as a counter key |
| `clock.next_regular_session` | guards 5, lifecycle 4, schema 1 | a date |
| `clock.current_session` | guards 5, lifecycle 4, schema 1 | a session id, compared by `within-quarterly` and `anchor` |
| `clock.last_business_day_of_session` | actions 1, lifecycle 1 | calendar flag (enter-minutes rules) |
| `clock.adjourned_meeting_set` | lifecycle, precedence, scheduler | a session-continuation flag |
| `["event-occurred", ev, scope]` scope tokens `just-now`, `now`, `since-ruling`, `open-now`, `pending-consent` | actions, precedence, adjudication | windows the model leaves informal (`gaps.json event-scope-sugar`) — the carriers of "promptly" and "at the time" |
| `ScheduledItem.hour`, `Order.kind: close-debate-at-hour` | precedence 8, scheduler 10, actions 1, schema 1 | clock hour of day |
| `speech_limit.max_minutes`, `Member.speech_counters.minutes_used`, `Order.kind: limit-total-time` | council 2, protocol 1, schema 1 | duration |
| timers with `expires_on_event` only (vote-challenge-window, renewal-unblock, order-exhaustion) | lifecycle | **event-anchored; no clock at all** |
| the adjudication timeliness default (`breach-just-occurred`, `result-window-open`, `continuing-breach`) | adjudication | event-anchored |

**Finding.** [derived; corrected per feedback §9 — revision 1 counted six timers and its self-check passed the miscount] Of the model's seven timers, three are purely event-anchored and need nothing. The other four (`reconsider-make-window`, `reconsider-callup-window`, `reconsider-enter-minutes-callup`, `table-expiry`) and the three guards (`carryover-eligible`, `postpone-horizon`, `schedulable-to`) depend on exactly two calendar notions — the **day** and the **quarterly interval** — plus the **session**. Everything else is either a comparison against a scheduled hour (which can be any monotone value) or a duration. So the logical-time mapping is small: define *day*, *session*, *corridor*, *hour*, and *duration*, plus the three session flags and the event-window tokens, and the model runs — with every redefinition recorded as an interface change (§6).

## 2. The mapping

The hierarchy, smallest first: **tick** (one applied event) < **round** (one opportunity for every present seat to act on a state; 01 §3) < **sitting** (from call to order to adjournment or recess-end: the book's *meeting*, 8:2(1)) < **session** (8:2(2)) < **term** (the roll's life).

| Model word | Agentic meaning | Why | Status |
|---|---|---|---|
| `now` | `(sitting, round)` in lexicographic order — comparable across sittings | the clock's job is to order events and compare against scheduled hours; a per-sitting counter cannot serve `postpone-horizon` or a postponement into a later sitting (feedback §9) | [declared] |
| `current_day` (counter key) | the sitting id | the day's role is to renew debate rights [43:12] after a break; a sitting is the break | [declared] |
| `same-day` / `next-business-day` | same sitting / next sitting | as above; used by reconsider's make-window and enter-minutes rules | [declared] |
| `end-of-session` | special council: adoption of `move-rise-and-report`, discharge, or parent expiry — *all meetings of a special committee are one session* [50:23]; standing council: adjournment of a sitting that set no adjourned sitting [8:4, 8:9] | the book's own rule for each kind | [derived] |
| `next-regular-session` | special: `null`; standing: the next sitting under the charter's calling rule [50:21] | special committees have no regular sessions | [derived] |
| `within-quarterly(s1, s2)` | special: **false**; standing: `not clock.term_boundary_before_next_session` | see §3 | [declared] |
| `hour` on a scheduled item | a round number **or an event anchor** (`event/subcommittee-reported`, `event/parent-instruction`) | see §4 | [declared; interface extension] |
| `max_minutes` / `minutes_used` | tokens per speech / tokens used | the scarce resource is context, not wall-clock (coordinator's `initial-thoughts.md`); the book's distinction between *deadline* and *budget* orders [15:16] survives exactly as rounds-vs-tokens: a close-debate-at-round order eats rounds spent on interruptions; a token budget does not | [declared] |
| "a reasonable time" to wait for a quorum [40:11]; "a reasonable number of days" of notice [9:4, 9:13] | a declared number of rounds / the call of the sitting | the book says each body sets these itself [9:4] | [declared] |
| the round's **backstop** | the one wall-clock value in the system; recorded on `round-closed` as a fact | 01 §3 f; it **decides presence**, which feeds quorum and the threshold drops of 35:2(7)/37:35(3); latency invariance holds inside it (contention spike F4) | [declared] |

## 3. The quarterly interval, and why agents have no calendar decay

The book's own statement of purpose is 8:11–12: "the freedom of each new session … one session cannot place a question beyond the reach of a majority at a later session." Beyond the interval "business can go over … only by means of referral to a committee" [9:8], and a term boundary has the same effect [9:8, 21:7(c), 49:22]. So the principle is that a temporary disposition must not bind a later sitting whose *grounds* may have moved — revision 1 read it as continuity of membership, which is only the term-boundary half (feedback §9). [derived from 8:11–12, 9:7–10]

For agents, what moves between sittings is less memory than the world: a seat re-forked from the same frozen source carries the same reading, but main may have moved, the specimen's facts may have changed, the papers the question rests on may have lapsed. Revision 2 put the elapsed-world test inside the corridor predicate; feedback-2 §7 showed that `postpone-horizon` and `schedulable-to` evaluate it when a postponement is *moved*, about a future that is unknown, and that in an active refactor it would void carryover at nearly every adjournment. So the corridor is now a property of the schedule and the roll only — `within-quarterly(s1, s2) ≡ no seat crossed a term boundary between s1 and s2` — and the elapsed-world test is the standing charter's later-session rule `lapsed-grounds`: a carried-over question whose papers lapsed comes up with the lapse shown, open to renewal after progress [38:7], reconsideration and rescission, decided by the majority that can see what moved. That is 8:12's principle, the later majority's freedom. For a special council (one session, 50:23) the corridor never opens: at the end of its single session everything temporarily disposed falls to ground except what a subcommittee holds [21:7(c)], which is just dissolution. [declared; `06-adversarial-cases.md` §4 declares the three identities a forked seat splits]

Consequences the verifier can check against the guards: with `within-quarterly ≡ false`, `guard/postpone-horizon` and `guard/schedulable-to` reduce to "not beyond the end of the present session" — any round in the council's life; `guard/carryover-eligible` is false; `timer/table-expiry` and `timer/reconsider-callup-window` close at end-of-session. All consistent with a special committee's one-session life. [derived]

## 4. Event anchors as scheduled times (an interface extension)

Joseph's "research subcommittee that returns" and the coordinator's "postpone until the report arrives" both need a scheduled time that is an *event*, not a number. The lifecycle already has `expires_on_event` for timers; the scheduler does not have it for items. Two ways to supply it: [derived]

- **Overload `before`**: let `["before", ["now"], h]` with `h` an event reference evaluate to `["not", ["event-occurred", h, "session"]]`. No new operator; one evaluator rule; `ScheduledItem.hour` becomes `?round | ?event-ref`. This is an INTERFACE change (the operator set is closed) and must be recorded in `CHANGELOG.md` when integrated.
- **Or** express every event-anchored postponement as `postpone-to-certain-time` with `time = ["anchor", "event/<id>"]` and extend `anchor` to resolve event refs. Same semantics; the first is simpler.

Either way the book's own structure holds: a postponed question with an event-time is a general order whose "hour arrives" when the event fires [14:13–17], and the chair (mechanical) announces it without a motion [14:17]. The scheduler's dispatch steps 2–3 (`not before(now, hour)`) then work unchanged. The same device makes `fix-time-to-which-to-adjourn` adjourn "until subcommittee S reports" — an adjourned sitting [9:17] whose call is an event.

## 5. Interface changes this mapping makes

The operator set is closed (INTERFACE), so every per-council redefinition is an interface change to record in `CHANGELOG.md` when integrated — not only `before(now, event-ref)` (§4) but `within-quarterly` (fixed as month arithmetic in INTERFACE), `same-day`, `anchor`, `now` as a pair, and the event-scope tokens as definitions. Revision 1 flagged one; feedback §9 is right that all belong in one entry (model-defects F4, widened).

## 6. What does not translate, and why that is fine

- **Nothing in the model needs wall-clock duration except the backstop.** Every "promptly", "immediately", "at the time" is a window between events; 01 §3 shows each becomes exact under rounds.
- **The day-boundary erasure** [43:12 — debate rights renew per day] becomes a sitting-boundary erasure. In a special council that never recesses this never fires; the parent's token budget per question bounds debate instead [13:8(d)].
- **`reconsider-enter-minutes`** is built on the day boundary (not callable the day made, 37:47) to protect absentees. With every seat present every round of a sitting, and previous notice across sittings for standing councils, it has nothing left to protect; both charters exclude it with that reason. [derived; the honest remainder: a standing council whose sittings are far apart might want it back — it costs nothing to re-enable, which is the point of the whitelist]
