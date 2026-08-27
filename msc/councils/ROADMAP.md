# Councils — Roadmap

*The RONR-12 council system: a formally modeled, executable implementation of parliamentary procedure (Robert's Rules of Order Newly Revised, 12th ed.) usable by any mix of human and logogenic members. The model lives in `model/`; the source corpus in `RONR-12/` (local-only, copyrighted — the model expresses all rules in its own words with §:¶ citations). Governing success criterion, from phase 4 on: `available_actions(member, role, state) → [(action, justification)]` — every member, with roles, has a computable list of available actions and their justifications at any time, updated as actions happen. The deterministic layer is sovereign throughout: nothing in later phases adds, removes, or gates an action outside it.*

## Overview
### Completed

1. **Full text.** Source split into `RONR-12/` (front/body/index + summary tables). Whole-read by the coordinating session.

2. **Outline + interface.** Seven-component decomposition (catalog+precedence, stack engine, question lifecycle, floor protocol, rule hierarchy, scheduler) with shared external conventions. Rationale preserved in `model/.priors/RONR-12-MODEL.md`.

3. **Independent component models.** Five parallel fork agents, one per component group; independently audited (~120 citations verified); findings integrated. Superseded files in `model/.priors/`.

4. **Unified executable model.** One namespace, typed state schema, closed s-expression grammar, adjudication sub-machine for judgment predicates (the judgment content arrives as an input event inside a fully formal declare→challenge→appeal→assembly-final flow), `actions.json` as the affordance root, acceptance suite of script replays, `tools/sweep.py` as mechanical lint. Second independent audit integrated (11 findings). Council configuration layer (`council.json`): contexts — small board, society, convention, legislative body, committee, etc. — as points along seven axes of one machine, not separate models; ordinary-society defaults extracted and named; small-board and committee overlays compiled as proof.

### Upcoming

5. **Storage / instantiation / state + artifacts** — see [Phase 5](#phase-5).
6. **Pedagogy and affordances** — see [Phase 6](#phase-6).
7. **Probabilistic state machine** — see [Phase 7](#phase-7).

---

## Details for Upcoming

### Phase 5 — Storage / Instantiation / Instantiated State / All State + Artifacts

The practical layer: not changes to the core engine, but the artifacts, their evolution, and when/how they surface and affect the running system.

**Bootstrap and instantiation.** A council comes into being by charter: rule-stack instantiation, council-type configuration (`ctx.config`), initial credentials/membership roll, role assignments, quorum and meeting/notice provisions, the charter's own amendment rule — and a pinned version hash of the model data, which is formally the council's adoption of its parliamentary authority. Two bootstrap paths: (a) *fiat charter* — instantiated from outside; (b) *self-hosting* — the mass-meeting → organizing-society sequence (§53–54), where the engine itself runs the founding from nothing but a call and attendance.

**The generative event chain.** Charter → notice/call → convened meeting (given quorum) → the in-meeting action space → records reshaping the next interstitial state. The interstitial (between-meetings) action space is small but never empty: issuing or compelling a call, giving previous notice, committee work, board action within delegation, resignations. Lists are time-reactive as well as event-reactive — windows open and close by clock alone.

**Artifact inventory**, in three classes:

- *Normative inputs (gate admissibility):* the rule stack; the call/notice of each meeting (for special meetings it bounds admissible business, 9:15; for a mass meeting it functions as the bylaws, 53:9); the previous-notice registry with scopes; the credentials/membership roll (living, in conventions maintained by the credentials committee).
- *Scheduling state:* the agenda/program with the adopted-vs-live distinction (adopted = normative, change requires 2/3; live = the scheduler queues); the chair's memorandum as a derived projection.
- *Records (projections of the event stream):* the minutes — specified as a projection function over the event log (48:4–5), draft/approved lifecycle with correction machinery; the precedent record (rulings + reasons, feeding back into the normative class as persuasive precedent, 23:10–11); ballots/tally sheets under custody with recount timers (45:41); reports and referred papers with custody rules. Class-3 artifacts are mostly derivable: minutes-generation is nearly free given the event log.
- *Advisory inputs (shape deliberation, gate nothing):* recommendation objects attached to agenda items — source body, proposed disposition, proposed resolution text, optional rationale — pre-circulated and surfaced in the panel. The book's recommendation channels (committee reports cast as ready-to-adopt resolutions, screening/resolutions committees, nominating slates, sponsors' prepared resolutions, consent calendars) all reduce to this object, under the standing discipline: a recommendation is an input that must pass through motion-and-vote; preparation never impairs the assembly's freedom.

**Single-custody invariant.** A question object has exactly one custodian council at any time: while committed, the parent's only reach is discharge or instruction [10:26(5), 36] — the traveling-object identity below depends on this law, or concurrent parent/child mutation becomes representable.

**Pinned-version governance.** Pinning a model-data hash as the council's adopted parliamentary authority creates a question RONR never faced: when a *model bug* is found, is the fix an erratum applied unilaterally, or an amendment to the council's parliamentary authority requiring in-band adoption? Working answer, to be ratified with the charter presets: pin = edition (as bylaws adopt an edition, 2:15-18); bug-fix releases within a pinned line count as errata *iff the council's charter says so* — a clause both bootstrap paths include by default.

**Pre-charter bootstrap state.** The self-hosting path (§53–54) runs before any chair exists, so it needs a small pre-organization machine the main engine presumes past: call to order by a sponsor, election of chair pro tem and secretary pro tem, statement of objectives (protocol.json now carries the chair-pro-tem role). Session identity during founding follows 53:32: fixed periodic meeting rules make each meeting a session; next-meeting-set-each-time or at-the-call-of-the-chair makes the whole series one session — which gates same-question bars and renewal during organization.

**Schema prerequisites (from the stage-4 audits).** Phase 5's artifacts need slots stage 4 doesn't yet carry: Meeting/Session records (kind, call reference, formation rule), Committee identity (which committee holds a committed question), election structure (offices, candidates, ballot rounds, the 46:46 finality rule). See `model/gaps.json` meeting-session-objects / committee-identity / elections-structure.

**Sub-organizations (recursion).** A committee is an artifact of its parent that runs the same engine on a derived configuration. The referral or establishing act *is* a fiat charter (13:8 carries composition, selection, chairmanship, instructions, powers, meeting-mode): parent act → child `council.json` instance, authority axis pointing upward, committee/small-board overlay, roll enumerated, temporal bounds. Lifecycle: created → instructed (initially and subsequently, 13:22) → monitored (partial reports) → dissolved (final report or discharge — for a special committee, presenting the final report is itself the dissolution event). Cross-council object identity: a referred question is the same object traveling down (with pending amendments, under the child's lifted same-question bars) and back up as report-plus-recommendations that re-pend in the parent. Recursion is native (subcommittees, 50:15; executive committee as board-within-board, 49:13); committee of the whole is the degenerate case (child roll = parent roll).

**Typed committee templates (charter presets).** The book defines rule-bearing committee types — nominating (with its president-exclusion and no-vote report), resolutions/reference (with an explicit menu of screening power levels, 59:68–75), the convention organizing trio (credentials/standing-rules/program), auditing (report adoption discharges the treasurer's liability), bylaws, membership, and the disciplinary chain (investigating/trial/discipline, with disjointness constraints). Each becomes a template artifact: preset config + rule deltas + role exclusions + expected artifact kinds + timers. The engine needs only the generic establish/refer machinery; types are affordances. A template becomes normative only by in-band adoption — the vote converts template into charter.

### Phase 6 — Pedagogy and Affordances

The legitimate home for everything the executability criterion banned from the engine: a `pedagogy/` namespace keyed to action-ids, motion-ids, and artifact kinds, which the engine provably never reads — the panel decorates with it.

Content kinds: glosses (what/why); purpose-based decision aids ("to accomplish X, the instrument is Y" — the book itself indexes motions functionally, §6); sample wordings — *original phrasings of ours*, teaching the same moves with §:¶ provenance, never lifted passages (the book's own scripts and forms are copyrighted); common-misuse warnings surfaced on the relevant affordance (e.g., what lay-on-the-table is for versus how it is misused, 17:13–16); panel hierarchy hints (typed presets grouped under their generic actions).

Pedagogy teaches the system in vivo, and feeds forward: what a council repeatedly does on advice can crystallize — in-band — into its standing rules, its custom (2:25, which the model tracks as a rule class), and eventually special rules of order. Hints never decide; adoption converts teaching into law. One acceptance test the crystallization path owes: a crystallized custom must *lose* to any written rule the moment a point of order cites one [2:25] — the fall is automatic, not discretionary.

### Phase 7 — Probabilistic State Machine

Priors and Bayesian update over the action space, to make large affordance lists usable: the deterministic layer answers what *may* happen; this layer estimates what is *likely wanted*.

**Object:** P(action | state features), not marginal frequencies — conditioned on meeting phase, immediately pending question, council type, role. The typed state schema supplies conditioning features directly. Additional conditioning parameters over time might be cross-organization vs organization-specific, and even role or member-specific.

**Priors:** derived from the book's own frequency language (its explicit markings of common/usual vs. seldom/rare practice), its misuse commentary (modeling *reached-for* and *apt* as distinct distributions), and — via a dedicated survey session — the *selection structure* of the simplified Robert's Rules editions: which fraction of the machinery they judged worth teaching is itself a strong commonality prior. (Their shape, not their content — they are copyrighted like the parent volume.)

**Structure:** hierarchical — book-derived global prior → council-type prior (`council.json` levels) → per-council posterior updated from its own phase-5 event log. Dirichlet–multinomial per state cluster suffices to start.

**Two binding constraints, stated as criteria (not hopes):**

1. *Probability never gates; rarity never buries.* RONR deliberately protects rights whose exercise is rare — point of order, appeal, division, the lone objection to unanimous consent, the 62:8–9 self-put against a stonewalling chair are the least-frequent and most load-bearing actions in the system, and frequency-ranking would progressively hide exactly the minority-protective machinery. Rights-critical affordances are pinned — exempt from ranking, always visible when live — and the pin list is **derived from the model, not curated**: everything reachable by a single member's demand (`single-member-demand` / `not-voted` thresholds), the interrupt-capable acts, and the enforcement family (point-of-order, appeal, division, objection, put-ignored-question). A curated list rots silently when a motion is added; a computed one cannot drift.
2. *The layer may reorder and annotate — never add, remove, or gate.* And because surfacing shapes usage shapes the posterior, visibility keeps a floor (an exploration term) so the panel cannot become self-confirming.
3. *Justifications are never probabilistic.* Ranking reorders the list; the justification attached to each entry remains the deterministic layer's, verbatim — no salience weighting inside the why.
4. *Equal visibility is the default.* Per-member conditioning ("even role or member-specific" above) is an equal-visibility hazard in a governance tool: differentially surfacing rights machinery to different members is a de facto inequality in exactly what the pin list protects (1:1's equal-weight principle is the floor). Panel ranking conditions on role and state only, by default; member-specific conditioning requires that member's own opt-in or an in-band adopted rule.

**Uses:** panel ranking/salience; pedagogy targeting (surface phase-6 glosses where the posterior says confusion lives); anomaly salience (a rare-for-this-state action is exactly when the gloss matters most). The layer also feeds the same in-band crystallization path as pedagogy — observed custom → standing rules → special rules. And its own behavior is governable in-band: a council may adopt rules about its panel's ranking. The layer advises; the members decide. Also for us in simulation runs.
