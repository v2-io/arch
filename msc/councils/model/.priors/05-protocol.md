# Component 5 — Floor & Dialogue Protocol

*Stage-3 model per `INTERFACE.md`. Covers: roles, the motion-handling dialogue cycle, obtaining/assigning the floor, preference in recognition, interruption rights, debate rights and per-member speech counters, decorum constraints on speech acts, and the voting/announcement exchange. Structured data appears as fenced JSON blocks inline. This component is the turn-taking wrapper around `engine` (component 3): it decides **who may speak and what speech act is admissible now**; whether the *content* of a motion is in order is `catalog`/`engine`/`rules` business.*

## 5.1 Roles

```json
{
  "roles": [
    {"id": "chair", "desc": "presiding officer; grants recognition, states and puts questions, announces results, rules on order", "cite": "[3:6][47:7]"},
    {"id": "member", "desc": "full participant: attend, make motions, debate, vote", "cite": "[1:4]"},
    {"id": "secretary", "desc": "recording officer; reads papers on direction; records", "cite": "[3:6][47:32-33]"},
    {"id": "maker", "desc": "contextual: member who made the pending motion", "cite": "[4:6]"},
    {"id": "seconder", "desc": "contextual: member who seconded", "cite": "[4:9]"},
    {"id": "reporting-member", "desc": "contextual: presents a board/committee report; motions on behalf of committee need no second", "cite": "[51:8-11]"},
    {"id": "nonmember", "desc": "no participation rights; presence at assembly's sufferance", "cite": "[61:7][61:19]"}
  ]
}
```

`NOTE(judgment):` *maker/seconder/reporting-member* are modeled as roles bound per-question, not global states.

## 5.2 The dialogue cycle for a motion

Six steps — three to bring before the assembly, three to consider `[4:2][4:25]`. Emitted core events in parentheses.

1. **Make** (`made`) — member with the floor states the motion `[4:4]`. Before `stated`, the motion is the maker's property: may modify or withdraw freely (`withdrawn`) `[4:19][33:12]`; informal suggestions to modify/withdraw admissible without floor `[4:20-23]`.
2. **Second** (`seconded`) — any member, no floor needed, without rising in small assemblies `[4:9]`. Not required when made by direction of a board/committee `[4:11]`. Absence of second becomes immaterial once debate begins or any member votes `[4:13]`.
3. **State** (`stated`) — only the chair places the question before the assembly `[4:3]`; must state it promptly unless ruling it out of order or requiring clearer/written form (`ruled-out-of-order` alternative) `[4:16-18]`. After `stated`, ownership passes to the assembly `[4:19]`; withdrawal/modification now needs consent — see 5.7.
4. **Debate** (`debate-opened`) — if debatable (`catalog:motion/*` attribute). Maker has first-speech preference `[4:27][42:9(1)]`.
5. **Put** (`put`) — chair puts the question, exact wording definitive `[4:34]`; affirmative called first, **negative always called** (courtesy-motion relaxation noted; irrelevant-negative exception) `[4:35]`. Abstentions never called for `[4:35]`.
6. **Announce** (`adopted` | `rejected`) — report of voting, declaration, effect/execution statement, then next business `[4:43-47]`.

Unanimous-consent shortcut: chair may take action or adopt absent objection — "if there is no objection…"; a single member's objection forces the regular cycle; a timely objection overrides an already-made announcement `[4:58-59]`. `NOTE(judgment):` modeled as a compressed cycle emitting `stated`+`put`+`adopted` atomically, abortable by event `protocol:event/objected`.

## 5.3 Obtaining the floor

- A member must be recognized before making a motion or debating `[3:30][42:2]`; exceptions are the interruption-capable acts of 5.5 and Table `[t44-t45]` (owned by `catalog`).
- Claim: rise after the floor is yielded, address the chair by title; identification in large assemblies `[3:31][42:2]`. Rising before the floor is yielded establishes no claim `[3:32][42:6]`.
- Chair must recognize a member entitled to the floor `[3:30][42:2]`; assignment (`protocol:event/recognized`) by announcing name/identification, or a nod in small meetings `[42:2]`.
- Yield: resuming seat (`protocol:event/floor-yielded`) `[3:31]`. Debate rights not transferable; unexpired time not reservable `[43:10]`.
- Recognition of the wrong claimant: remediable by `catalog:motion/point-of-order`; appealable except in mass meetings `[42:15]`. Assignment may be put to a vote when chair is in doubt (plurality of votes wins the floor) `[42:14]`.
- Large-assembly adaptations (microphone queues etc.) are permitted variations, chair-directed pending rules `[42:16-17]`.

## 5.4 Preference in recognition

Applies only among members who rose before anyone was recognized `[42:7]`.

```json
{
  "preference-rules": [
    {"when": "debatable question immediately pending", "order": [
      {"rule": "maker of the immediately pending motion, if not yet spoken on it", "variants": "reporting-member for committee report; mover of take-from-table; mover (not caller-up) of reconsider", "cite": "[42:9(1)]"},
      {"rule": "member who has not spoken on this question today outranks one who has", "cite": "[42:9(2)]"},
      {"rule": "alternate between opposing sides when known", "cite": "[42:9(3)]"}
    ]},
    {"when": "appeal or point-of-order submitted to assembly, debatable", "order": [
      {"rule": "chair may speak first in preference to all, and again to close", "cite": "[24:3(5)][23:19][42:10]"}
    ]},
    {"when": "undebatable question immediately pending", "order": [
      {"rule": "no maker preference", "exception": "maker of reconsider moved for announced purpose of amending, upon reconsideration", "cite": "[42:12][42:11]"}
    ]},
    {"when": "vote result just reported", "order": [
      {"rule": "member rising to move retake of the vote by another method", "cite": "[42:8]"}
    ]},
    {"when": "no question pending", "order": [
      {"rule": "member assigned to offer the called/prearranged main motion", "cite": "[42:13(1)]"},
      {"rule": "member continuing a series of motions toward one object (e.g. after table/suspend-rules adopted for that purpose)", "cite": "[42:13(2)]"},
      {"rule": "member whose announced alternative motion follows defeat of the prior motion", "cite": "[42:13(3)][10:30(5)]"},
      {"rule": "purpose-priority ladder, in order: (a) reconsider-enter-minutes, (b) reconsider, (c) call up reconsider, (d) give previous notice, (e) take-from-table, (f) motion with previous notice given", "cite": "[42:13(4)]"}
    ]}
  ]
}
```

Guards this component owns:

- `floor-open`: no member currently has the floor `[3:30]`.
- `preference-claim-timely`: claim asserted before the chair has recognized another `[42:7][3:34]`.
- `series-in-progress`: a multi-motion sequence toward one object is mid-flight; blocks unrelated intervening motions (e.g. take-from-table cannot interrupt it) `[34:5]`.

## 5.5 Interrupting an assigned speaker

Only for listed purposes, urgency justifying `[42:18-19]`; interrupted speaker retains the floor and resumes after disposal (`interrupted` → `resumed`, chair re-assigns) `[42:20]`. Handing a report to the secretary to read does not yield the floor `[42:21]`.

```json
{
  "may-interrupt-speaker": [
    {"act": "catalog:motion/call-for-orders-of-the-day", "cite": "[42:18(a)]"},
    {"act": "catalog:motion/raise-question-of-privilege", "cite": "[42:18(b)]"},
    {"act": "catalog:motion/point-of-order (incl. calling member to order)", "cite": "[42:18(c)]"},
    {"act": "demand separate vote on independent series / divisible amendments", "cite": "[42:18(d)]"},
    {"act": "request/inquiry requiring immediate response", "cite": "[42:18(e)]"},
    {"act": "catalog:motion/appeal; objection-to-consideration; division-of-assembly — special circumstances", "cite": "[42:18(f-h)]"}
  ],
  "after-assignment-before-speech-also": [
    {"act": "give previous notice", "cite": "[42:19(a)]"},
    {"act": "make (not take up) reconsider / reconsider-enter-minutes", "cite": "[42:19(b)]"}
  ]
}
```

Full per-motion interrupt/second data is `catalog` ownership (`[t44-t45]`); this component consumes it as guard `catalog:guard/in-order-when-other-has-floor`.

## 5.6 Debate rights and counters

Per-member, per-question, per-day counter; and a per-speech clock.

```json
{
  "speech-counters": {
    "scope": "member × debatable-question × calendar-day",
    "max-speeches": 2,
    "max-minutes-per-speech": 10,
    "modifiable-by": "rules:class/special-rules-of-order, or session/question orders via catalog:motion/limit-extend-debate",
    "cite": "[43:8][43:12][43:15-17]"
  },
  "counter-rules": [
    {"rule": "second speech barred while any member seeking floor has not spoken once on that question", "cite": "[4:28][43:13]"},
    {"rule": "each debatable motion in a series is a distinct question for counters", "cite": "[43:13]"},
    {"rule": "exhausted = 2 speeches that day on that question", "cite": "[4:28]"},
    {"rule": "rights renew entirely on a later day", "cite": "[43:12]"},
    {"rule": "asking a question, brief suggestion, or making a secondary motion without comment: not counted", "cite": "[43:12]"},
    {"rule": "questions answered by a speaker charge the speaker's time", "cite": "[43:10][33:9]"},
    {"rule": "no transfer or reservation of time", "cite": "[43:10]"},
    {"rule": "appeal: one speech per member, chair twice (close)", "cite": "[24:3(5)][43:12]"},
    {"rule": "committee-of-whole/quasi/informal speeches don't count against same-day assembly rights", "cite": "[13:21][43:18][52:1]"},
    {"rule": "reconsider debate rights separate from original consideration; same-day reconsideration re-imports exhaustion for the reconsidered question", "cite": "[37:18][37:21-22]"}
  ]
}
```

Interaction with lifecycle: postponement or table+resume to a *different day* renews rights; same-day resume preserves exhaustion `[14:19][34:6]` (state carried by `lifecycle`; counters owned here).

## 5.7 Requests, consent exchanges, and withdrawal

- Withdraw/modify after `stated`: request by maker; unanimous-consent attempt first, else vote on granting `[33:11-19]`. Withdrawal admissible until voting begins; adhering motions fall with it `[33:16]`. Withdrawn motion = never made (renewal unrestricted) `[33:18][38:2]`.
- Parliamentary inquiry / request for information: no vote; chair responds or channels to a member; third-person exchange through the chair `[33:3-10]`.
- Read papers / other privileges: consent or majority `[33:20-22]`.

## 5.8 Decorum constraints on speech acts (admissibility filters on debate content)

- Germaneness to the immediately pending question `[43:20]`.
- No personalities; no questioning motives; measure-not-member `[43:21][4:30]`.
- All remarks through the chair; no direct member-to-member exchange `[43:22]`.
- Avoid names; officers by title; chair never addressed as "you", speaks of self in third person `[3:10-13][43:23]`.
- No adverse comment on a prior act not pending unless concluding with a motion to reconsider/rescind/amend it or notice thereof `[43:24]`.
- Maker may not speak against own motion (may vote against) `[43:25]`.
- Chair does not debate while presiding; must relinquish chair to do so, not returning until the main question is disposed of; exception: appeals and submitted points of order `[43:29-30]`.
- Enforcement path on breach: call to order → permission-to-continue vote → naming, escalating to `rules`/disciplinary territory `[61:10-18]`.

## 5.9 Voting exchange details

- Interruption of voting barred from first vote cast until polls presumed complete `[45:6]`; no explanation of votes during voting `[45:7]`.
- Vote changeable until result announced; afterward only by unanimous consent, requested immediately `[45:8]`.
- Challenge window: point-of-order on the conduct of a vote, division demand, retake motion, recapitulation, vote change — all only until debate or business intervenes (`protocol:event/challenge-window-closed`), except continuing breaches `[45:9][23:6]`.
- Division of the assembly: single member's demand, from the moment the negative is cast until immediately after announcement, compels a rising retake `[29:4][4:52]`. Dilatory-use filter `[29:7]`.
- Inconclusive-vote ladder (chair-driven): voice → "seem to have it" pause → rising → counted rising `[4:50-51]`.

## 5.10 Events added by this component

`protocol:event/recognized`, `floor-yielded`, `interrupted`, `objected`, `division-demanded`, `challenge-window-closed`, `consent-requested`, `consent-granted`, `consent-denied`.

## 5.11 Test vectors (Form-and-Example passages exercising this component)

- `[10:38-42]` full six-step cycle, maker's first-speech preference, announcement with effect.
- `[4:61]` unanimous-consent extension exchange (three phrasings).
- `[16:26-27]` competing previous-question forms via limited recognition mid-put ("For what purpose does the member rise?").
- `[17:23]` claiming limited recognition while lay-on-table pending.
- `[34:9-10]` preference claim to take from the table against a new main motion, incl. after the new motion is made but before stated.
- `[23:12-16]` point-of-order interruption; `[24:9-13]` appeal exchange and chair's two speeches.
- `[33:4][33:7-8]` inquiry/information exchanges; `[33:12-15]` withdrawal before/after stating.
- `[12:83-89]` chair's restatement discipline across a substitute sequence (stating/putting forms).
- `[19:13][19:16]` question-of-privilege interruptions of a speaker.
- `[61:14]` naming declaration (second-person exception).
