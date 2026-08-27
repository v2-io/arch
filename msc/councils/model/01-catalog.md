# Component 1+2 — Motion Catalog & Precedence

*Stage-3 model per `INTERFACE.md`. Owns: the canonical motion registry, the Standard-Descriptive-Characteristics (SDC) attribute table, the precedence partial order with its guards, and the ballot-counting decision table. Data format: fenced JSON blocks in this file. All normative claims cite RONR 12th ed. §:¶. Rules are expressed in the model's own vocabulary, not transcribed.*

## 1. Registry schema

Each motion record:

```json
{
  "id": "kebab-case id (authoritative registry — INTERFACE.md §2)",
  "name": "RONR display name",
  "class": "M | S | P | I | B | M/B  (Table II key)",
  "rank": "1–13 for the ranked chain (1 = main, lowest), null for unranked",
  "interrupt": "false | true | {\"when\": \"condition\", \"cite\": \"…\"}",
  "second": "true | false | conditional object",
  "debatable": "no | yes | yes-restricted | yes-opens-question | if-target-debatable",
  "amendable": "no | yes | limited object",
  "vote": ["threshold or list of sufficient alternatives (INTERFACE.md §6)"],
  "reconsider": "yes | no | neg-only | aff-only | conditional object",
  "cites": ["SDC block and key paragraphs"]
}
```

`debatable` values: `yes-restricted` = debate confined to the motion's own merits, not the underlying question `[12:7(5), 13:7(5), 14:4(5)]`; `yes-opens-question` = debate may go fully into the merits of the question acted on `[11:2(5), 35:2(5), 36:4(5), 37:9(5)]`; `if-target-debatable` = inherits debatability from the motion it is applied to `[12:7(5), 37:9(5)]`.

## 2. Registry — ranked chain (component 2 backbone)

The thirteen ranked motions form a total order: each takes precedence over all lower ranks and yields to all higher `[5:11]`. Listed ascending.

```json
[
 {"id":"main-motion","name":"Main motion","class":"M","rank":1,
  "interrupt":false,"second":true,"debatable":"yes","amendable":"yes",
  "vote":["majority"],"vote_exceptions":"bylaw/special-rule thresholds; suspends-a-rule → two-thirds; changes-something-adopted → rescind thresholds [10:8(7)]",
  "reconsider":"yes","cites":["10:8"]},

 {"id":"postpone-indefinitely","name":"Postpone Indefinitely","class":"S","rank":2,
  "interrupt":false,"second":true,"debatable":"yes-opens-question","amendable":"no",
  "vote":["majority"],"reconsider":"aff-only",
  "notes":"kills the main question for the session on adoption [11:3]; dropped if main question is committed [11:4, 13:19]",
  "cites":["11:2"]},

 {"id":"amend","name":"Amend","class":"S","rank":3,
  "interrupt":false,"second":true,"debatable":"if-target-debatable",
  "amendable":{"primary":"yes","secondary":"no [12:7(6)]"},
  "vote":["majority"],"vote_notes":"majority regardless of target's own threshold [12:7(7)]",
  "reconsider":"yes",
  "notes":"must be germane [12:6, 12:16–21]; rank rises when applied to a variable in a higher-ranked motion (see §5.2)",
  "cites":["12:7"]},

 {"id":"commit","name":"Commit / Refer","class":"S","rank":4,
  "interrupt":false,"second":true,"debatable":"yes-restricted","amendable":"yes",
  "vote":["majority"],
  "reconsider":{"aff":"only until committee takes up the question [13:7(8)]","neg":"until progress makes it a new question, then renewable [13:7(8), 38:7(1)]"},
  "notes":"variants: committee-of-the-whole, quasi-committee, informal consideration [13:2]; blocked while an order closing debate at a set time is in force [13:7(2), 15:11]; not applicable to a reconsider alone, so out of order while a reconsider is pending in-series [13:7(2); Chart I]",
  "cites":["13:7"]},

 {"id":"postpone-to-certain-time","name":"Postpone to a Certain Time","class":"S","rank":5,
  "interrupt":false,"second":true,"debatable":"yes-restricted","amendable":"yes",
  "vote":["majority"],"vote_exceptions":"two-thirds if it makes the question a special order; an amendment adding that provision still needs only majority [14:4(7)]",
  "reconsider":{"aff":"yes","neg":"until progress [14:4(8)]"},
  "notes":"limit: not beyond next regular session within a quarterly interval; never such that it kills the question (that reading converts to postpone-indefinitely) [14:6, 14:9]; blocked under a debate-closing order [14:4(2)]; out of order while any undebatable question is immediately pending except division-of-question or consideration-by-paragraph [14:4(1)–(2); Chart I] (same condition, with the same two exceptions, applies to commit — plus commit's reconsider bar)",
  "cites":["14:4"]},

 {"id":"limit-extend-debate","name":"Limit or Extend Limits of Debate","class":"S","rank":6,
  "interrupt":false,"second":true,"debatable":"no","amendable":"yes (amendment also undebatable)",
  "vote":["two-thirds"],
  "reconsider":{"aff":"unexecuted part only, any time before exhaustion [15:5(8)]","neg":"until progress"},
  "notes":"superseding conflicting orders allowed while pending questions remain [15:17]; exhaustion at 15:18",
  "cites":["15:5"]},

 {"id":"previous-question","name":"Previous Question","class":"S","rank":7,
  "interrupt":false,"second":true,"debatable":"no","amendable":"no",
  "vote":["two-thirds"],
  "reconsider":{"aff":"only before any vote taken under the order [16:5(8)]","neg":"until progress"},
  "notes":"qualified form covers a consecutive series from the immediately pending question [16:5(2)]; competing wider/narrower PQ motions voted widest-first (quasi-filling-blanks) [16:5(6)]; nonstandard wordings ('call the question') are this motion [16:6]",
  "cites":["16:5"]},

 {"id":"lay-on-table","name":"Lay on the Table","class":"S","rank":8,
  "interrupt":false,"second":true,"debatable":"no","amendable":"no",
  "vote":["majority"],
  "reconsider":"neg-only (aff: use take-from-table instead) [17:3(8)]",
  "notes":"cannot be qualified [17:21]; carries the whole adhering bundle [17:5]; misuse-to-kill is out of order [17:2, 17:13–16]",
  "cites":["17:3"]},

 {"id":"call-for-orders-of-the-day","name":"Call for the Orders of the Day","class":"P","rank":9,
  "interrupt":true,"second":false,"debatable":"no","amendable":"no",
  "vote":["single-member-demand"],"vote_notes":"orders enforced on one member's demand; set aside only by two-thirds (as neg vote on proceeding, or aff vote to suspend/extend) [18:4(7), 18:8]",
  "reconsider":"no",
  "notes":"interrupts a pending question only when a special order is being neglected [18:4(1), 18:5]; except when a special order must be taken up, also yields to a reconsider being made or called up [18:4(1)]; not in committee of the whole [18:6]",
  "cites":["18:4"]},

 {"id":"raise-question-of-privilege","name":"Raise a Question of Privilege","class":"P","rank":10,
  "interrupt":{"when":"urgency justifies; not during voting/verification; not a speaker already speaking unless purpose would otherwise be defeated","cite":"19:6(3)"},
  "second":false,"second_notes":"the motion raised, if any, needs a second [19:6(4)]",
  "debatable":"no","amendable":"no",
  "vote":["chair-rules"],"vote_notes":"chair rules on admission; the admitted question is then an ordinary main motion [19:3, 19:6(7)]",
  "reconsider":"no","cites":["19:6"]},

 {"id":"recess","name":"Recess (privileged: moved while business pending)","class":"P","rank":11,
  "interrupt":false,"second":true,"debatable":"no","amendable":"yes (duration; undebatable)",
  "vote":["majority"],"reconsider":"no",
  "variant":"recess-main when no question pending or for a future time [20:3] — class M, debatable, amendable",
  "cites":["20:5"]},

 {"id":"adjourn","name":"Adjourn (privileged: unqualified, another meeting provided for, no adjournment hour set)","class":"P","rank":12,
  "interrupt":false,"second":true,"debatable":"no","amendable":"no",
  "vote":["majority"],"reconsider":"no (renewable after progress [21:6(8), 38:7(4)])",
  "not_in_order":"during voting or vote verification, before result announced (ballot-collection exception) [21:6(1)]",
  "variant":"adjourn-main when qualified, an adjournment hour already set, or adoption would dissolve the assembly [21:3] — class M, debatable, amendable, not privileged; retains privileged rules when unqualified even with no question pending [21:2]",
  "cites":["21:6"]},

 {"id":"fix-time-to-which-to-adjourn","name":"Fix the Time to Which to Adjourn (privileged: moved while question pending)","class":"P","rank":13,
  "interrupt":false,"second":true,"debatable":"no","amendable":"yes (date/hour/place; undebatable)",
  "vote":["majority"],"reconsider":"yes",
  "not_in_order":"if a later meeting of the same session is already scheduled [22:2]; movable even after vote to adjourn, before declaration [22:6(1)]",
  "variant":"fix-time-main when no question pending [22:4] — class M, debatable",
  "cites":["22:6"]}
]
```

## 3. Registry — incidental motions (unranked; admission by guard, §5.3)

```json
[
 {"id":"point-of-order","name":"Point of Order","class":"I","rank":null,
  "interrupt":true,"second":false,
  "debatable":"no (if chair submits to assembly: debatability follows appeal rules [23:2(5), 23:19])",
  "amendable":"no","vote":["chair-rules"],
  "reconsider":"no (assembly's vote on a submitted point: yes [23:2(8)])",
  "timeliness":"at the time of the breach [23:5]; continuing-breach exceptions [23:6]",
  "cites":["23:2"]},

 {"id":"appeal","name":"Appeal","class":"I","rank":null,
  "interrupt":{"when":"at the time of the ruling appealed","cite":"24:3(3), 24:8"},
  "second":true,
  "debatable":"yes, except: indecorum/rules-of-speaking, priority of business, or undebatable question immediately pending/involved → no [24:3(5)]; one speech per member, chair may speak twice [24:3(5)]",
  "amendable":"no",
  "vote":["tie-or-majority-sustains-chair"],"vote_notes":"majority in the negative required to reverse [24:3(7)]",
  "reconsider":"yes","cites":["24:3"]},

 {"id":"suspend-rules","name":"Suspend the Rules","class":"I","rank":null,
  "interrupt":false,"second":true,"debatable":"no","amendable":"no",
  "vote":["two-thirds"],
  "vote_exceptions":"ordinary standing rules → majority [25:15]; never against a negative vote as large as the minority the rule protects [25:2(7)]",
  "reconsider":"no","renewal":"not same purpose same meeting except unanimous consent; renewable after adjournment [25:6]",
  "unsuspendable":"bylaws (except rules-of-order-in-nature or self-suspending), law, fundamental principles, absentee protections, basic individual rights, rules applying outside the session [25:7–13]",
  "cites":["25:2"]},

 {"id":"objection-to-consideration","name":"Objection to the Consideration of a Question","class":"I","rank":null,
  "interrupt":{"when":"until debate begins or any subsidiary motion except lay-on-table is stated","cite":"26:2(1)"},
  "second":false,"debatable":"no","amendable":"no",
  "vote":["two-thirds-neg"],"vote_notes":"two thirds voting against consideration sustains; question put as 'shall the question be considered' [26:7–8]",
  "reconsider":"neg-only (the vote sustaining the objection) [26:2(8)]",
  "applies_to":"original main motions only [26:2(2), 10:6]","cites":["26:2"]},

 {"id":"division-of-question","name":"Division of a Question","class":"I","rank":null,
  "interrupt":false,"second":true,"debatable":"no","amendable":"yes",
  "vote":["majority"],"reconsider":"no",
  "notes":"parts must each stand alone and jointly equal the whole [27:5–6]; independent-subject series and non-conforming amendment series split on single-member demand without this motion [27:10–11]",
  "cites":["27:3"]},

 {"id":"consideration-by-paragraph","name":"Consideration by Paragraph / Seriatim","class":"I","rank":null,
  "interrupt":false,"second":true,"debatable":"no","amendable":"yes",
  "vote":["majority"],"reconsider":"no",
  "notes":"counterpart consider-as-a-whole has identical characteristics [28:5]",
  "cites":["28:2"]},

 {"id":"division-of-assembly","name":"Division of the Assembly","class":"I","rank":null,
  "interrupt":true,"second":false,"debatable":"no","amendable":"no",
  "vote":["single-member-demand"],"reconsider":"no",
  "window":"from casting of negative votes until immediately after result announced [29:4(1), 45:9]",
  "notes":"compels an uncounted rising retake only; counting needs chair's order or majority [29:5, 4:53]",
  "cites":["29:4"]},

 {"id":"motions-relating-to-voting","name":"Motions Relating to Methods of Voting and the Polls","class":"I","rank":null,
  "interrupt":false,"interrupt_notes":"preference in recognition to move re a vote just taken [30:3(3)]",
  "second":true,"debatable":"no","amendable":"yes",
  "vote":["majority"],"vote_exceptions":"close-polls → two-thirds [30:3(7)]",
  "reconsider":{"close-polls":"no","reopen-polls":"neg-only","timed close/reopen orders":"until executed","others":"yes [30:3(8)]"},
  "notes":"incidental only while the question is pending or vote just announced; else incidental main [30:2]; never retake by the same method [30:6]",
  "cites":["30:3"]},

 {"id":"motions-relating-to-nominations","name":"Motions Relating to Nominations","class":"I","rank":null,
  "interrupt":false,"second":true,"debatable":"no","amendable":"yes",
  "vote":["majority"],"vote_exceptions":"close-nominations → two-thirds [31:2(7)]",
  "reconsider":{"close-nominations":"no","reopen-nominations aff":"no","others":"yes [31:2(8)]"},
  "cites":["31:2"]},

 {"id":"request-to-be-excused-from-duty","name":"Request to Be Excused from a Duty","class":"I","rank":null,
  "interrupt":{"when":"requires immediate attention","cite":"32:2(3)"},
  "second":{"rule":"only when moved formally by the requester [32:2(4)]"},
  "debatable":"yes","amendable":"yes","vote":["majority"],
  "reconsider":"neg-only once the person has learned of the action [32:2(8)]",
  "notes":"resignations are this request [32:5]","cites":["32:2"]},

 {"id":"requests-and-inquiries","name":"Requests and Inquiries (family)","class":"I","rank":null,
  "members":{
    "parliamentary-inquiry":{"second":false,"vote":"not-voted (chair responds)","reconsider":"no [33:2, 33:3–5]"},
    "request-for-information":{"second":false,"vote":"not-voted","reconsider":"no [33:6–10]"},
    "withdraw-motion":{"vote":"majority","reconsider":"neg-only [33:2(8), 33:11–18]"},
    "modify-motion":{"vote":"majority (via chair/assembly once stated)","reconsider":"yes [33:19]"},
    "read-papers":{"vote":"majority","reconsider":"yes, until reading concluded [33:20]"},
    "any-other-privilege":{"vote":"majority","reconsider":"yes [33:22]"}},
  "shared":"interrupt when immediate response needed; not debatable; not amendable; grant-motions by another member need no second [33:2]",
  "cites":["33:2"]},

 {"id":"create-blank","name":"Create a Blank (by striking out)","class":"I","rank":null,
  "interrupt":false,"second":true,"debatable":"no","amendable":"no",
  "vote":["majority"],"reconsider":"no",
  "notes":"movable while a related amendment is pending; its adoption disposes of that amendment and pools the specifications as proposals [12:95]",
  "cites":["12:95"]},

 {"id":"fill-blank-proposal","name":"Proposal for Filling a Blank","class":null,"rank":null,
  "interrupt":"called out when chair asks","second":false,
  "debatable":"if-target-debatable","amendable":"no",
  "vote":["majority"],"vote_notes":"proposals voted seriatim until one attains majority; ordering rules: money-to-spend high→low [12:109], money-to-accept low→high [12:110], names in proposal order [12:107], places/dates/numbers per 12:111",
  "reconsider":"yes","limit":"one proposal per member per open slot [12:97]",
  "cites":["12:92–113"]}
]
```

## 4. Registry — bring-back class and incidental-main variants

```json
[
 {"id":"take-from-table","name":"Take from the Table","class":"B","rank":null,
  "interrupt":false,"interrupt_notes":"preference in recognition over a new main motion, incl. one made but not yet stated [34:2(3), 34:4]",
  "second":true,"debatable":"no","amendable":"no",
  "vote":["majority"],"reconsider":"no (renewable after any business transacted [34:2(8), 38:7(2)])",
  "window":"same session, or through next regular session if within quarterly interval [34:3]; cannot interrupt a connected series [34:5]",
  "precondition":"not in order until some business or interrupting matter has been dealt with since the tabling (renewal after rejection likewise waits for further business) [34:2(2)]",
  "cites":["34:2"]},

 {"id":"rescind-amend-adopted","name":"Rescind / Amend Something Previously Adopted","class":"M/B","rank":null,
  "interrupt":false,"second":true,"debatable":"yes-opens-question","amendable":"yes",
  "vote":["notice+majority","two-thirds","majority-entire-membership"],
  "vote_notes":"alternatives — any one suffices [35:2(7)]; the notice alternative requires notice stating the complete substance of the change, given at the previous meeting within a quarterly interval or in the call [35:2(7)]; in a committee: two-thirds, dropping to majority only when every member who voted for the target motion is present or received reasonable notice [35:2(7)]; bylaws/special rules follow their own amendment requirements",
  "reconsider":"neg-only",
  "not_applicable":"when reachable via a pending reconsider; executed acts impossible to undo; accepted resignations / elections-expulsions once notified [35:6]",
  "scope_rule":"amendments beyond the scope of given notice destroy the notice's effect (or are out of order where notice is required) [35:2(6), 35:4]",
  "subvariant":{"id":"rescind-and-expunge","vote":["majority-entire-membership"],"cite":"35:13"},
  "cites":["35:2"]},

 {"id":"discharge-committee","name":"Discharge a Committee","class":"B|M/B","rank":null,
  "class_rule":"B when the matter was referred while pending (subsidiary commit); M/B when referred by main motion [36:10–11]",
  "interrupt":false,"second":true,"debatable":"yes-opens-question","amendable":"yes",
  "vote":["notice+majority","two-thirds","majority-entire-membership"],
  "vote_exceptions":"majority alone when committee missed a prescribed reporting time, or while a partial report is being considered [36:4(7)]",
  "reconsider":"neg-only","cites":["36:4"]},

 {"id":"reconsider","name":"Reconsider","class":"B","rank":null,
  "making_vs_consideration":"MAKING takes precedence over everything and yields to nothing [37:9(1a)]; CONSIDERATION has only the rank of the motion to be reconsidered [37:9(1b)] — see §5.4",
  "interrupt":{"when":"making: after floor assigned, before the speaker begins; calling up: no","cite":"37:9(3)"},
  "second":true,"second_notes":"any member may second; call-up needs none [37:9(4)]",
  "mover":"prevailing side only (assembly); committee: anyone not on the losing side [37:10(a), 37:35]",
  "window":"same day, or next business day within the session [37:10(b)]; committees: no limit [37:35]",
  "debatable":"if-target-debatable; opens the target's merits [37:9(5)]",
  "amendable":"no",
  "vote":["majority"],"vote_notes":"majority regardless of the target's threshold [37:9(7)]; in a standing/special committee: two-thirds, dropping to majority only when every member who voted with the prevailing side is present or received reasonable notice [37:35(3)]",
  "reconsider":"no",
  "not_applicable":"renewable motions; certain negative votes now conflicting; partly executed affirmatives; notified contracts; undoable acts; votes on reconsider itself; finalized elections; results reachable by another majority-without-notice motion [37:9(2)]",
  "cites":["37:9"]},

 {"id":"reconsider-enter-minutes","name":"Reconsider and Enter on the Minutes","class":"B","rank":null,
  "delta_from_reconsider":"made same day as the vote only; outranks plain reconsider even mid-announcement; not where a day's delay defeats the object; not at last business meeting before a >quarterly gap; not callable-up same day (last-day-of-session exception) [37:47]",
  "applies_to":"exactly: (a) aff or neg vote on a main motion; (b) aff vote on postpone-indefinitely; (c) neg vote on objection-to-consideration when the session extends beyond the day [37:47(3)] — narrower than 'votes finally disposing of a main motion' (e.g., a sustained objection's aff vote is excluded)",
  "cites":["37:46–52"]},

 {"id":"renewal","name":"(rule cluster) Renewal of motions","class":null,
  "notes":"not a motion; the same-session non-renewal principle and its exceptions [38:1–9] live as guards: renewable-after-progress (commit, postpone, limit-extend, previous-question, lay-on-table [38:7(1)]), after-business (take-from-table [38:7(2)], orders-call [38:7(3)]), after-progress (adjourn, recess [38:7(4)]), nominations/polls [38:7(5)]; later-session renewal blocked while question remains within assembly's control [38:8]",
  "cites":["38:1–9"]}
]
```

**Incidental-main correspondences** (same name, moved when nothing is pending; class M, debatable and amendable per main-motion rules unless noted): adopt/accept/agree-to `[10:52]`; ratify `[10:54–57]`; recess-main, adjourn-main, fix-time-main `[20:3, 21:3, 22:4]`; limit-extend-for-meeting (M but still two-thirds) `[15:4]`; suspend-standing-rule-for-session (majority) `[25:15]`; commit-a-non-pending-subject `[13:6]`; postpone-a-non-pending-event (= rescind-amend-adopted thresholds) `[14:3]`; make-special-order-not-pending (two-thirds) `[41:44]`; voting/nomination method motions when no election pending `[30:2, 31:1n14]`. `NOTE(judgment):` these are modeled as *contextual variants* selected by the guard `no-question-pending`, not as separate registry rows, except where thresholds differ (noted above).

## 5. Component 2 — precedence

### 5.1 Semantics

`takes-precedence-over(B, A)` ⇔ B may be moved and stated while A is pending, whereupon B becomes the immediately pending question `[5:8]`. Pending motions are voted in reverse order of stating (LIFO), stopping if any of the halting motions named in `[10:34]` carries: recess, lay-on-table, postpone-to-certain-time, commit, or postpone-indefinitely (adjourn, when adopted, likewise ends consideration, but via session/meeting end `[21:7]` rather than the 10:34 list — see `03-engine` §3.3, which is authoritative for the unwind table). The ranked chain (§2, `rank` 1–13) is a total order `[5:11]`; incidental motions have no rank among themselves `[5:12, 6:19]`.

### 5.2 Rank modifications within the chain

- `amend` applied to any amendable motion takes precedence over *its target*, even a target above rank 3 (e.g., amending a pending postpone) `[6:7]`.
- `limit-extend-debate` applies to any *immediately pending debatable* motion or a consecutive series from it `[6:8, 15:5(2)]`; `previous-question` likewise to any immediately pending debatable-or-amendable motion or series `[16:5(2)]`. Both thereby out-rank their targets in the moment of application.
- Subsidiary motions cannot be applied to one another except per each SDC-2 (e.g., nothing tables an amendment alone — the main question goes with it `[12:7(2)]`).

### 5.3 Incidental admission rule

An incidental motion is in order only while `legitimately-incidental` to a pending motion or to business at hand; it then takes precedence over everything pending `[6:18]`. **With the exception of division-of-assembly — which yields to nothing in this rule** `[6:19]` — an incidental motion yields to privileged motions and generally to `lay-on-table`, **except** it does not yield to any motion ranking below the motion out of which it arose `[6:19, 6:21]`. Every motion yields to motions legitimately incidental to itself `[6:19]`. Per-motion yield exceptions override this general rule — see §5.6 for the appeal / submitted-point-of-order matrix.

### 5.6 Appeal / submitted point-of-order yield matrix (`catalog:table/appeal-yield-matrix`)

The general incidental rule (§5.3) is wrong for appeals unless refined by this four-case table `[24:3(1)–(2)]`, mirrored for points of order the chair submits to the assembly `[23:2(1)–(2)]`:

```json
[
 {"case":"debatable, non-adhering","yields_to":["privileged (when in order by rank)","limit-extend-debate","previous-question","commit","postpone-to-certain-time","lay-on-table"],"cite":"24:3(1)–(2)"},
 {"case":"debatable, adhering","yields_to":["privileged (when in order by rank)","limit-extend-debate","previous-question"],"note":"commit/postpone/table reach it only applied to the main question, carrying the appeal along","cite":"24:3(1)–(2), 13:7(2), 14:4(2), 17:3(2)"},
 {"case":"undebatable, adhering","yields_to":["privileged (when in order by rank)","lay-on-table (via the main question)"],"note":"yields to NO other subsidiary — even one out-ranking the motion it arose from","cite":"24:3(1)"},
 {"case":"undebatable, non-adhering","yields_to":["privileged (when in order by rank)"],"note":"yields to no subsidiary at all","cite":"24:3(1)"}
]
```

### 5.4 Reconsider's split rank

Making `reconsider` outranks everything, including rank 13 and a pending vote-to-adjourn before declaration `[37:9(1a), 21:10]`; it is then recorded and held. Its *consideration* carries only the target's rank: taken up immediately if the target would then be in order, else queued into the pending series at the target's rank position `[37:25–27]`. `reconsider-enter-minutes` outranks plain `reconsider` `[37:47(2)]`.

### 5.5 Admission guards (chart-condition set)

Named guards consumed by SDC-1 conditions; `03-engine` composes them into push-legality. Signature per INTERFACE.md §5.

```json
[
 {"guard":"no-question-pending","desc":"no motion currently pending before the assembly","cite":"4:4, 5:4"},
 {"guard":"another-has-floor","desc":"floor currently assigned to a member (see protocol:event vocabulary)","cite":"3:30–32"},
 {"guard":"immediately-pending-undebatable","desc":"the immediately pending question is not debatable","cite":"t4 chart; 15:5(2)"},
 {"guard":"immediately-pending-unamendable-and-undebatable","desc":"blocks previous-question","cite":"t4 chart; 16:5(2)"},
 {"guard":"pq-order-unexhausted","desc":"an order for the previous question covers motions not yet voted; blocks postpone-indefinitely/amend/commit/postpone/limit-extend","cite":"16:2, t4 chart"},
 {"guard":"debate-closing-order-active","desc":"an order closing debate on the main question at a set time/duration is in force; blocks commit and postpone (not a mere per-speech limit)","cite":"15:11–13, 13:7(2), 14:4(2)"},
 {"guard":"nonadhering-incident-pending","desc":"a point of order, undebatable appeal, or request not adhering to the main question is pending; blocks lay-on-table","cite":"t4 chart; 23:2(1), 24:3(1)"},
 {"guard":"suspend-rules-priority-pending","desc":"a suspend-the-rules touching priority of business is pending; blocks call-for-orders-of-the-day","cite":"18:4(1), t4 chart"},
 {"guard":"legitimately-incidental","desc":"per-motion applicability test for incidental admission (each I-motion's SDC-2)","cite":"6:18"},
 {"guard":"consideration-not-begun","desc":"no debate yet and no subsidiary motion except lay-on-table stated; window for objection-to-consideration","cite":"26:2(1)"},
 {"guard":"voting-in-progress","desc":"from first vote cast until all have presumably voted; blocks interruption incl. adjourn","cite":"45:6, 21:6(1)"},
 {"guard":"result-window-open","desc":"from result announcement until debate/business intervenes; window for division, retake motions, vote changes, points re the vote","cite":"45:9"},
 {"guard":"within-reconsider-window","desc":"same day or next business day of the session","cite":"37:10(b)"},
 {"guard":"mover-was-prevailing-side","desc":"eligibility to move reconsider","cite":"37:10(a)"},
 {"guard":"question-within-assembly-control","desc":"temporarily but not finally disposed of; blocks conflicting/duplicate main motions and later-session renewal","cite":"9:11, 10:26(5), 38:8"},
 {"guard":"same-question-decided-this-session","desc":"substantially same question finally disposed of this session; blocks renewal except via bring-back motions","cite":"10:26(3), 38:3(1)"},
 {"guard":"renewable-after-progress","desc":"progress in business/debate has made the motion a new question","cite":"38:7"},
 {"guard":"quorum-present","desc":"required for validity of all substantive action; limited motion set allowed absent it","cite":"40:6–7"},
 {"guard":"dilatory","desc":"chair's duty-veto over form-abuse; absolute filter above all admission rules","cite":"39:1–4"}
]
```

## 6. Ballot-counting decision table (45:31–36; t52)

Columns: `credited` (to a candidate/choice) and `counted` (toward votes cast for majority computation).

```json
[
 {"case":"clear meaning, eligible choice","credited":true,"counted":true,"cite":"45:32"},
 {"case":"ineligible or unidentifiable candidate","credited":false,"counted":true,"note":"illegal vote","cite":"45:32"},
 {"case":"unclear meaning, cannot affect result","credited":false,"counted":true,"cite":"45:33"},
 {"case":"unclear meaning, may affect result","credited":"assembly-decides","counted":true,"cite":"45:33"},
 {"case":"overvote (too many candidates for the office)","credited":false,"counted":true,"note":"one illegal vote","cite":"45:32"},
 {"case":"two+ filled ballots folded together","credited":false,"counted":"as one vote","cite":"45:34"},
 {"case":"blank folded with one filled ballot","credited":"filled ballot normally","counted":"blank ignored","cite":"45:34"},
 {"case":"blank / no-preference ballot","credited":false,"counted":false,"note":"abstention","cite":"45:31"},
 {"case":"identified nonmember ballot","credited":false,"counted":false,"cite":"45:35"},
 {"case":"unidentifiable nonmember ballots possibly affecting result","effect":"entire vote null and void; retake","cite":"45:35"},
 {"case":"multi-section ballot","rule":"each section tallied as an independent ballot; folding creates illegal votes only per duplicated section","cite":"45:36"}
]
```

Election-specific: no for/against boxes in elections `[45:25]`; write-ins always legal for eligible persons `[45:18, 46:2]`; multi-seat: every ballot voting in the section = one vote cast, seats fill by majority then vote-total order, ties and shortfalls go to repeated balloting with no forced dropouts `[46:32–33]`.

## 7. Interface exports

Other components may rely on: `catalog:motion/<id>` (registry rows above), `catalog:guard/<id>` (§5.5), `catalog:relation/takes-precedence-over` (§5.1–5.4 semantics), `catalog:table/appeal-yield-matrix` (§5.6), `catalog:table/ballot-counting` (§6). Threshold vocabulary per INTERFACE.md §6 plus the additions used here: `tie-or-majority-sustains-chair`, `two-thirds-neg`, `single-member-demand`, `chair-rules`, `not-voted`.

## 8. Test vectors bearing on this component

- `[10:34]` — the eight-motion pending series: full ranked-chain admission and reverse-order unwind.
- `[6:7]` — amend applied to a pending postpone (rank modification, §5.2).
- `[16:26–27]` — competing previous-question scopes, widest-voted-first (§2 previous-question notes).
- `[12:83–89]` — substitute sequence: amend-within-amend admission and germaneness.
- `[12:109–110]` — filling a blank with amounts: proposal ordering (§3 fill-blank-proposal).
- `[13:26]` — completing an incomplete commit via filling blanks.
- `[14:21]` — amending postpone into a special order: majority amendment flipping the postpone to two-thirds (§2 postpone notes).
- `[37:27]` — reconsider of a commit made while table/amend/main pending: split-rank queueing (§5.4).
- `[26:8]` — objection-to-consideration put/announce forms (two-thirds-neg threshold).
- `[45:37–38]` — tellers' reports as worked examples of §6 arithmetic.

`NOTE(judgment):` Registry granularity — Table II's ~98 rows collapse here to ~30 rule-bearing records plus contextual variants; rows that are pure cross-references (substitute → amend; refer → commit) are treated as aliases, and incidental-main duplicates as guard-selected variants (§4). Stage 4 may re-expand to Table-II row granularity mechanically from these records.
