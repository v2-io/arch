# RONR-12 — Proposed Model Decomposition

*Drafted 2026-08-27 from a whole read of `RONR-12/body.md` (RONR 12th ed. body text; front matter and index split to `RONR-12/front.md` and `RONR-12/index.md`). Status: proposed architecture, pre-implementation. The core claim: no single formalism covers parliamentary procedure without distortion — the decomposition itself is the model, and it should follow RONR's own seams.*

## The seven components

### 1. Motion catalog — relational / attribute model (decision tables)

The Standard Descriptive Characteristics (§7) are already a relational table: motion × {precedence, applicability, interrupts?, requires second?, debatable?, amendable?, vote threshold, reconsiderable?}. RONR's own Table II (t6–t33) is this table, hand-maintained by the authors. Static data, not dynamics: typed records the engine consults. Ballot-counting rules (Table VIII, 45:31–36) are likewise a pure decision table.

### 2. Precedence — a partial order with guards, not a total order

The 13 ranked motions (main + 7 subsidiary + 5 privileged) form a chain (5:10–13), but incidental motions explicitly "have no rank among themselves" (5:12, 6:19) and slot in via condition-guards ("legitimately incidental to…", 6:18). So: a rank lattice plus guard predicates, not a numeric priority scale.

### 3. Pending-question core — a guarded stack machine (pushdown), not a flat FSM

The load-bearing choice. Motions become pending LIFO and are voted in reverse order (10:33–34); a flat state machine cannot represent an arbitrary pending series (main + postpone-indefinitely + amendment + commit + postpone + ballot-motion + table + recess) as a state — it is a stack of typed frames whose push-legality is computed from components 1, 2, and 6.

Key observation: *Lay on the Table*, *Postpone*, and *Commit* are stack-segment serialization — the adhering bundle (10:35) is popped as a unit into an object and later restored intact (17:5–7, 34:6). A pushdown system with checkpointing.

### 4. Question lifecycle — per-object statecharts (Harel), with timers

Each question is an object moving through pending / committed / tabled / postponed / adopted / rejected / within-reconsideration-window / dead, with clocks: the reconsider deadline (same or next business day, 37:10(b)), table expiry (end of next session within a quarterly interval, 17:8, 34:3), exhaustion of debate-limit and Previous Question orders at session end (15:18, 16:11).

Statecharts specifically earn their keep via **history states**: RONR is full of "consideration resumes at exactly the point where it was interrupted" (recess, orders of the day, questions of privilege — 8:7, 18:7, 19:10), which is literally the H* mechanism.

### 5. Floor and dialogue — interaction protocol (multi-role protocol state machines)

The make–second–state–debate–put–announce cycle (§4), recognition and preference in recognition (§42), interruption rights (t44–t45), per-member speech counters (twice per question per day, 43:12) — a multi-role protocol (chair, member, secretary) with debate rights as per-member resource counters. Descriptive glue around component 3, not the engine itself; RONR's Form-and-Example scripts are essentially its sequence diagrams.

### 6. Rule hierarchy — defeasibility / priority model

Which rule governs is its own system: charter > bylaws > special rules of order > parliamentary authority > standing rules > custom (§2), plus "specific yields over general" (3:2, 56:68(3)), plus suspension — temporarily disabling a rule by 2/3, with unsuspendable classes (25:7–13: fundamental principles, absentee protections, basic individual rights). Legal-informatics territory: defeasible rules with priorities, not automata at all.

### 7. Scheduler — orders of the day / agenda as priority queues with preemption

General orders vs. special orders (§14, §41): special orders preempt pending business at their hour (with the made-first-outranks rules of 41:53–55); general orders never preempt and drain in made-order; *the* special order for a meeting is a distinct top priority (41:57); unfinished business carries over by fixed rules (21:7, 41:21–26); session boundaries and the quarterly time interval (9:7) gate what may be scheduled at all. A priority-queue-with-preemption model plus calendar logic.

## Why the obvious single-formalism candidates fail

- **Petri nets — poor fit.** They shine at concurrency and resource flow; RONR's whole architecture exists to *forbid* concurrency — one question at a time (5:4), everything serialized through the floor. Using a concurrency formalism to model a mutex. Where tokens feel natural (quorum, vote counts, speech rights), counters/guards are simpler and truer.
- **Flat state machines / automata** — right instinct, wrong power class: no stack (component 3), no hierarchy/history (component 4).
- **Flowcharts / protocol diagrams** — fine as *views* (Chart I is a view of components 1–2; Form-and-Example is a view of 5), but they don't compose and can't carry guards.

## The architecture in one sentence

A guarded pushdown engine (3) whose transition legality is computed from decision tables (1) + a precedence partial order (2) + a defeasible rule hierarchy (6), with each question as a timed statechart object (4), wrapped in a turn-taking protocol (5), and driven between questions by a preemptive scheduler (7).

If forced to stretch one formalism furthest: hierarchical statecharts with an explicit stack and rule-table-driven guards. But the honest answer is the decomposition — RONR is coherent precisely because it separates these layers.
