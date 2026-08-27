# Components 6 + 7 — Rule Hierarchy (defeasibility) & Scheduler (orders of the day / agenda)

*Stage 3 of the RONR-12 model. Conventions per `INTERFACE.md`. Data format: fenced JSON blocks inline. Internal IDs are kebab-case; everything prefixed `rules:` or `scheduler:` is exported; other components' items are referenced qualified (`catalog:…`, `lifecycle:…`, `protocol:…`).*

---

# Component 6 — Rule hierarchy

The rule hierarchy answers one question the other components must ask constantly: **which rule governs right now, and can it be set aside?** It is a defeasible-priority model, not an automaton: rules belong to classes; classes are totally ordered by authority; within the order, specificity defeats generality; and a distinct *suspension* mechanism can temporarily disable some rules but not others.

## 6.1 Rule classes

```json
{
  "rules:classes": [
    {"id": "procedural-law", "desc": "procedural rules prescribed by federal/state/local law applicable to the body", "cite": ["1:5", "25:8"]},
    {"id": "charter", "desc": "corporate charter (certificate/articles of incorporation); also charters issued by a superior society to a subordinate unit", "cite": ["2:5-7", "2:7n4"]},
    {"id": "constitution", "desc": "constitution when a separate document from bylaws; otherwise merged into bylaws", "cite": ["2:10-12"]},
    {"id": "bylaws", "desc": "the combined basic instrument of the society", "cite": ["2:8-13", "56:1"]},
    {"id": "special-rules-of-order", "desc": "adopted rules of parliamentary procedure superseding the parliamentary authority", "cite": ["2:15-16", "2:20-22"]},
    {"id": "parliamentary-authority", "desc": "the adopted manual (RONR); binding where not inconsistent with anything above", "cite": ["2:18"]},
    {"id": "standing-rules", "desc": "administrative rules not about parliamentary procedure", "cite": ["2:23-24"]},
    {"id": "convention-standing-rules", "desc": "hybrid class adopted per convention session; may modify parliamentary-authority rules; expires at end of session", "cite": ["59:27", "59:34-37"]},
    {"id": "custom", "desc": "established practice treated as if a rule until it conflicts with any written rule and a point of order is raised, whereupon it falls", "cite": ["2:25"]},
    {"id": "general-parliamentary-law", "desc": "default substrate for a body with no adopted rules", "cite": ["1:5"]}
  ]
}
```

## 6.2 Authority ordering

Higher defeats lower on conflict. `NOTE(judgment):` convention-standing-rules are placed as shown because they supersede the parliamentary authority for the session `[59:27]` but may not conflict with the society's bylaws `[59:27]`.

```json
{
  "rules:authority-order": [
    "procedural-law", "charter", "constitution", "bylaws",
    "special-rules-of-order", "convention-standing-rules",
    "parliamentary-authority", "standing-rules", "custom",
    "general-parliamentary-law"
  ],
  "cite": ["2:2", "2:7", "2:12", "2:16", "2:18", "2:25", "59:27"]
}
```

Qualifications that a flat ordering cannot express (kept as override rules):

- **Specificity defeats generality within and across documents at the same level**: a particular statement governs over a general one to which it is an exception `[3:2, 56:68(3)]`.
- **Anti-delegation lock**: a special rule of order cannot supersede a parliamentary-authority rule that the authority says is alterable only in the bylaws `[2:16n5, 56:49]`.
- **Name does not control class**: a rule's adoption/amendment/suspension treatment follows its *content*, not what the society calls it `[2:24, 25:14]`.
- **Other manuals**: merely *persuasive* where the adopted authority is silent; never binding `[2:18-19]`.
- **Precedent** (chair rulings + appeal outcomes, with recorded reasons): persuasive only; weight grows with repetition and consistency; overridable by later ruling, appeal, or rule change `[23:10-11]`.

## 6.3 Interpretation principles (tie-breakers when a rule is ambiguous)

Exported as `rules:interpretation/*` for use by any component resolving a guard against written rules; all from `[56:68]`, ambiguity threshold and majority-decides from `[56:68(1)]`.

```json
{
  "rules:interpretation": [
    {"id": "society-decides", "desc": "the society itself resolves ambiguity, by majority; no interpretation may contradict a clear meaning", "cite": ["56:68(1)"]},
    {"id": "harmonize", "desc": "prefer the reading that does not conflict with or nullify another provision", "cite": ["56:68(2)"]},
    {"id": "specific-over-general", "desc": "specific statement defeats general", "cite": ["56:68(3)", "3:2"]},
    {"id": "expressio-unius", "desc": "authorizing some things of a class prohibits the unmentioned things of that class", "cite": ["56:68(4)"]},
    {"id": "lesser-included-privilege", "desc": "a granted privilege includes lesser privileges but prohibits greater ones", "cite": ["56:68(5)"]},
    {"id": "limitation-bounds", "desc": "a prohibition/limitation blocks everything beyond it, permits what is less and unmentioned same-class things not evidently improper", "cite": ["56:68(6)"]},
    {"id": "definite-penalty-fixed", "desc": "a definite penalty may be neither increased nor diminished", "cite": ["56:68(7)"]},
    {"id": "general-term-covers-specifics", "desc": "a rule using only the general term applies to all specific terms wholly included under it", "cite": ["56:68(8)"]}
  ]
}
```

## 6.4 Suspension model

Suspension = temporarily disabling a rule for a stated purpose within a meeting, via `catalog:motion/suspend-rules` (or its incidental-main equivalent `[10:26(1)n1, 25:15]`). The engine (component 3) asks this component two questions: *is this rule suspendible?* and *at what threshold?*

```json
{
  "rules:suspendibility": [
    {"class": "procedural-law", "suspendible": false, "unless": "the law provides for its own suspension", "cite": ["25:8"]},
    {"class": "charter", "suspendible": false, "unless": "charter or law so provides", "cite": ["2:7", "25:7"]},
    {"class": "bylaws", "suspendible": false, "unless": "provision provides for own suspension, OR provision is in the nature of a rule of order (then two-thirds)", "cite": ["2:8(4)", "2:21", "25:7", "62:12n5"]},
    {"class": "special-rules-of-order", "suspendible": true, "threshold": "two-thirds", "cite": ["2:21", "25:14"]},
    {"class": "parliamentary-authority", "suspendible": true, "threshold": "two-thirds", "cite": ["25:14"]},
    {"class": "standing-rules", "suspendible": true, "threshold": "majority", "scope-limit": "only rules applying within a meeting context; at most for the session", "cite": ["2:23", "25:15"]},
    {"class": "convention-standing-rules", "suspendible": true, "threshold": "majority", "note": "suspending BOTH the convention rule and the underlying general rule requires two-thirds; no suspension for remainder of session; single-application rules cannot be suspended (that would be rescission); the rule naming the parliamentary authority cannot be suspended as such", "cite": ["59:37", "59:47n7"]}
  ]
}
```

**Absolutely unsuspendable, regardless of vote** (`rules:unsuspendable/*`):

```json
{
  "rules:unsuspendable": [
    {"id": "fundamental-principles", "examples": ["one question at a time", "one person one vote", "vote limited to members present", "no cumulative voting absent bylaw"], "cite": ["25:9", "45:2", "45:56", "46:43"]},
    {"id": "absentee-protections", "examples": ["quorum requirement", "special-meeting scope limited to call", "previous-notice requirements"], "cite": ["25:10", "40:9"]},
    {"id": "basic-individual-rights", "examples": ["a particular member's right to attend, make motions, nominate, speak, give notice, vote — removable only by discipline"], "cite": ["25:11"]},
    {"id": "minority-protection-floor", "desc": "no rule protecting a minority of a given size may be suspended over the negative vote of that minority", "cite": ["25:2(7)"]},
    {"id": "ballot-secrecy", "desc": "a bylaw ballot requirement cannot be suspended so as to expose votes", "cite": ["25:7", "45:20"]},
    {"id": "outside-meeting-rules", "desc": "rules with application outside the current session cannot be suspended (rescind/amend instead)", "cite": ["25:13", "2:23"]},
    {"id": "order-of-business-wholesale", "desc": "the order of business cannot be dispensed with entirely in advance; only passed over item-by-item or by proceeding to a subject (two-thirds)", "cite": ["25:12"]}
  ]
}
```

Suspension mechanics the engine consumes: the motion names its *purpose*, never the rules `[25:4]`; nothing beyond that purpose is authorized `[25:4]`; renewal for the same purpose barred within the same meeting but allowed after adjournment `[25:6]`; a defeated suspension's purpose may be reached instead at the class thresholds for rescind/amend if it is really a standing rule, etc. (route via 6.5).

## 6.5 Adoption / amendment / rescission thresholds by class (decision table)

Thresholds use INTERFACE vocabulary; alternatives are disjunctive ("any one suffices").

```json
{
  "rules:change-thresholds": [
    {"class": "charter", "adopt": "per law", "amend": "per law + charter limits", "cite": ["2:6"]},
    {"class": "bylaws", "adopt": "majority (only at founding)", "amend": ["per its own amendment article", "else: notice+two-thirds", "else: majority-entire-membership"], "cite": ["54:20", "56:50", "57:1"]},
    {"class": "special-rules-of-order", "adopt": ["notice+two-thirds", "majority-entire-membership"], "amend": "same as adopt", "cite": ["2:22"]},
    {"class": "parliamentary-authority", "adopt": "in bylaws (preferred); else as a special rule of order", "cite": ["2:15"]},
    {"class": "standing-rules", "adopt": "majority (no notice)", "rescind-amend": ["notice+majority", "two-thirds", "majority-entire-membership"], "cite": ["2:23", "35:2(7)"]},
    {"class": "convention-standing-rules", "adopt": "two-thirds as a package if any rule individually needs two-thirds; parliamentary rules individually two-thirds, non-parliamentary individually majority", "amend-rescind": ["two-thirds", "majority of all registered voting members", "non-parliamentary: notice(prior day)+majority"], "cite": ["59:34-36"]},
    {"class": "agenda-program", "adopt": "majority (two-thirds if session already has an order of business it conflicts with or if creating special orders there)", "amend-after-adoption": ["two-thirds", "majority of all registered voting members", "unanimous-consent"], "cite": ["41:61", "41:63", "59:58-59"]},
    {"class": "custom", "fall": "automatic upon point of order citing conflict with any written rule", "cite": ["2:25"]}
  ]
}
```

## 6.6 Violation consequences — null-and-void hooks

The hierarchy defines which adopted acts are void and remain attackable without timeliness limits (consumed by `lifecycle` for the continuing-breach window and by `protocol` for point-of-order admissibility):

```json
{
  "rules:continuing-breach-grounds": [
    {"id": "conflicts-bylaws", "cite": ["23:6(a)"], "exception": "session-scoped incidental main motion conflicting with a suspendible rule-of-order in bylaws, adopted at suspension threshold", "exception-cite": ["10:26(1)n1"]},
    {"id": "conflicts-adopted-motion-still-in-force", "cite": ["23:6(b)"], "exception": "adopted by the rescind/amend threshold"},
    {"id": "violates-procedural-law", "cite": ["23:6(c)"]},
    {"id": "violates-fundamental-principle", "cite": ["23:6(d)"]},
    {"id": "violates-absentee/ballot-secrecy/basic-right protections", "cite": ["23:6(e)", "23:7-8"]}
  ]
}
```

Exported guards (consumed chiefly by components 3 and 5):

- `rules:guard/rule-suspendible(rule)` — per 6.4 `[25:7-16]`
- `rules:guard/act-in-order-under-governing-rules(act)` — resolve applicable rule via 6.2, apply 6.3 on ambiguity
- `rules:guard/threshold-for-change(rule)` — per 6.5
- `rules:guard/continuing-breach(act)` — per 6.6

---

# Component 7 — Scheduler

The scheduler decides **what business the chair announces next** and **when a scheduled item may seize the floor**. It is a set of priority queues with preemption, plus calendar gates. It acts *between* questions (feeding component 3) except for the one preemption case (special orders at their hour) and scheduled recess/adjournment, which interrupt a pending question.

## 7.1 Temporal frame

- Hierarchy: *meeting* ⊂ *session*; session = unit of scheduling freedom `[8:1-2, 8:11-12]`.
- `quarterly-interval` `[9:7]`: sessions ≤ a quarterly interval apart form a corridor through which business may travel by postponement, tabling, unfinished business, or pending Reconsider; beyond it (or across a membership-term boundary) only committee referral carries business over `[9:8-10, 21:7(c)]`.
- Scheduling horizon: no order of the day may be set beyond the next regular session (within the corridor), or beyond the present session (outside it) `[14:6, 41:40]`.
- One session may not bind a later session's majority except via special rules/bylaws `[8:12-14]`.

## 7.2 Scheduled-item kinds

```json
{
  "scheduler:item-kinds": [
    {"id": "general-order", "made-by": ["catalog:motion/postpone-to-certain-time (majority)", "main motion (majority)", "agenda position without stated hour or with guidance-only hour"], "preempts": false, "cite": ["14:14", "41:41-42", "41:58"]},
    {"id": "special-order", "made-by": ["postpone + make special order (two-thirds)", "main motion (two-thirds)", "agenda hour assignment (adopted with the agenda)"], "preempts": true, "cite": ["14:14", "41:41-42", "41:58", "59:55(4)"]},
    {"id": "the-special-order-for-meeting", "desc": "reserves a meeting; taken up right after minutes approval; outranks all other special orders regardless of when made", "preempts": true, "cite": ["41:57"]},
    {"id": "scheduled-recess", "cite": ["20:6", "41:66"]},
    {"id": "scheduled-adjournment", "cite": ["21:14", "41:66"]},
    {"id": "unfinished-business", "desc": "not schedulable directly; arises from prior meeting per 7.4", "cite": ["41:21-26"]}
  ]
}
```

## 7.3 Queues and the standard order of business

Default heading sequence (a rule of order; alterable per component 6):

```json
{
  "scheduler:standard-order-of-business": [
    "minutes-approval", "officer-board-standing-committee-reports",
    "special-committee-reports", "special-orders",
    "unfinished-business-and-general-orders", "new-business"
  ],
  "cite": ["3:16", "41:5-6"],
  "optional-headings": ["opening-ceremonies", "roll-call", "consent-calendar", "good-of-the-order", "announcements", "program"],
  "optional-cite": ["41:28-36"]
}
```

Queue discipline within headings:

- `scheduler:queue/special-orders-unfinished` then `scheduler:queue/special-orders-current`, each FIFO by *time made* `[41:18]`.
- `scheduler:queue/unfinished-business`: (a) question pending at last adjournment, (b) unreached unfinished business from last meeting in its prior order, (c) unreached general orders from last meeting in made-order — then `scheduler:queue/general-orders-current` in made-order; same-motion batches in listed order `[41:21-23, 41:52]`.
- Reports: standing committees in bylaws-listing order; special committees in appointment order `[41:13, 41:17]`.
- The chair announces items; he must not ask "is there unfinished business?" but state the known next item `[41:24]`; he cannot unilaterally depart from the order `[41:39]`.

## 7.4 Dispatch algorithm (between questions, at heading `special-orders` onward)

`NOTE(judgment):` this is a synthesis of `[41:18-27, 41:40-57]` into one procedure; each clause cites its source.

1. If the hour of *the* special order for the meeting has been reached (or it exists for this meeting): take it up after minutes `[41:57]`.
2. Else if any special order's hour has arrived or passed: take up the eligible special order **made earliest** `[41:53-54]`.
3. Else if a general order's hour has arrived: eligible only if no business pending, no special order interferes, no Reconsider call-up interferes, the general-orders position in the order of business has been reached or passed, and all earlier-made general orders (whose hours have arrived or are unset) are disposed of `[41:49]`. Earlier-made outranks earlier-hour `[41:50-51]`.
4. Else proceed through 7.3 queues in heading order.
5. New business: members introduce in floor-obtained order (`protocol:event/recognition`) `[41:27]`.

## 7.5 Preemption semantics

- **Special order at its hour** interrupts a pending question, except it yields to: (a) motions relating to adjournment/recess, (b) questions of privilege, (c) a special order made *earlier* than it, (d) *the* special order for the meeting `[41:41, 41:53]`. Interrupted business resumes afterward at the point of interruption (`lifecycle:` history restore) — resumption order: earliest-made special orders first, then the originally interrupted business `[41:55, 18:11]`.
- **Scheduled recess/adjournment hour**: chair announces and declares it, even mid-special-order; pending business is interrupted (recess) or carried per 7.6 (adjournment). Preemptible only by a two-thirds motion to postpone the hour or extend consideration time; announced Previous-Question-ordered votes may customarily be completed first `[20:6-7, 21:14, 41:56, 41:66-68]`.
- **Enforcement/waiver**: any single member may demand conformance via `catalog:motion/call-for-orders-of-the-day`; the demand compels the schedule unless set aside — two-thirds *negative* on "proceed to the orders of the day," or two-thirds affirmative to extend time / suspend and take up something else `[18:4(7), 18:8]`. Once refused, not renewable until the pending business is disposed of `[18:8(a), 38:7(3)]`. Except when a special order must be taken up, the call itself yields to the making or calling up of a reconsider `[18:4(1)]` — so a due order of the day does not outrun the reconsider window.
- **Early take-up** of any order of the day: only by reconsidering the making vote (while possible) or suspend-the-rules (two-thirds) `[14:13, 41:40]`; equivalently, laying intervening items on the table one at a time is a legitimate majority path `[17:14, 41:38]`.

## 7.6 Carry-over at boundaries

```json
{
  "scheduler:carryover": [
    {"case": "adjournment not ending session (adjourned meeting set)", "effect": "resume at point of interruption next meeting", "cite": ["21:7(a)", "9:19"]},
    {"case": "session ends; next regular session within quarterly-interval and no term-expiry", "effect": "pending question -> unfinished-business(a); unreached orders -> unfinished categories (b)/(c); unreached special orders -> special-orders-unfinished", "cite": ["21:7(b)", "41:18(a)", "41:21-23"]},
    {"case": "session ends; beyond quarterly-interval or member terms expire", "effect": "everything temporarily-disposed falls to ground except matters in committee hands", "cite": ["21:7(c)", "9:8"]},
    {"case": "agenda session (convention): meeting-end mid-agenda", "effect": "unreached items carry to next meeting ahead of that meeting's items, absent special provision and special-order conflict", "cite": ["41:70"]},
    {"case": "bylaws-mandated item not reached in its required session", "effect": "carries as unfinished business/special order if next regular meeting within quarterly-interval", "cite": ["9:23", "41:20"]}
  ]
}
```

## 7.7 Agenda / program overlay

When an agenda or program is adopted it *becomes* the order of business for the session `[41:61, 59:48]`: hour-assigned items are special orders unless marked guidance-only; unassigned items are general orders `[41:58]`. Adoption/change thresholds live in component 6 (6.5 `agenda-program` row). At an item's assigned hour the chair announces it, puts pending questions to vote without further debate (subject to immediate table/postpone/refer motions, undebatable in that posture, and a two-thirds extension motion) `[41:65]`. Precirculated but unadopted agendas bind nothing `[41:62]`.

Exported guards:

- `scheduler:guard/order-of-day-due(item)` — 7.4 eligibility
- `scheduler:guard/may-preempt(item, pending)` — 7.5 exceptions (a)–(d)
- `scheduler:guard/schedulable-to(time)` — 7.1 horizon `[14:6, 41:40]`
- `scheduler:guard/in-carryover-corridor(session-a, session-b)` — quarterly-interval + term-expiry test `[9:7-8, 21:7]`

---

## Test vectors (Form-and-Example passages usable against these components)

- `[41:51]` — 4:15/4:30 general-order pair: earlier-made outranks earlier-hour; no interruption of pending general order.
- `[41:55]` — 2 P.M./3 P.M./4 P.M. special-order chain: made-order preemption cascade and resumption order.
- `[18:10-11]` — call for the orders of the day at 11:30 special order; interruption and exact-point resumption.
- `[14:21-22]` — postpone-and-make-special-order: amendment changes threshold to two-thirds mid-consideration; announcement at the appointed hour.
- `[25:12]` — attempted wholesale dispensing with the order of business (must fail); two-thirds proceed-to-subject succeeds with mandatory return to sequence.
- `[59:37 + 59:39-47]` — convention standing rule suspension at majority vs. two-thirds combined suspension.
- `[2:25]` — custom falling on point of order (hierarchy resolution).
- `[62:12-14]` — suspending the bylaws-based presiding right (rule-of-order-in-bylaws suspendibility).
