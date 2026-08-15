# Coupling, the lock, recording, and the currency map — 2026-08-14 consolidation

*Coord (Fable), from the steward dialog of 2026-08-14 (verbatim bursts in `net/steward-truthification-sketch-2026-08-14.verbatim.md`, sketches 9–10, plus the in-chat coupling exchange). Register: **proposed** throughout; §1's commitment law is my synthesis from named estate mechanisms; the steward's phrasings are quoted where they are his. This document consolidates what the net (v3, `net/`) cannot carry in structure alone, and supersedes nothing — it feeds `01-MODEL.md` on its next touch.*

## §1. What the lock really is, and the commitment law that governs inter-line contact

**The lock, plainly:** a claim's composite support may exceed its strongest single leg *only when two or more legs with independent failure modes agree.* That is all it is — a guard on the one aggregation move that is otherwise too easy to fake. Within-leg corroboration (three same-method runs, three same-author restatements) raises that leg; it never arms the lock.

**Where it sits in the cycle:** the lock is a *query over the ledger* — computed at consumption time (certifying, deciding, teaching), never during the securing work itself. This placement is what *frees* the lines to influence each other during work: because the lock is checked at the end, over events whose provenance is recorded, the lines don't have to be quarantined from each other while running — they have to leave an honest trail of *how* they touched.

**The commitment law** (the principled answer to "is the math↔sim check-in loop unprincipled?"):

> Contact between lines is legitimate — usually obligatory — through **question and design channels**; contamination is contact through the **verdict channel**. A check counts toward corroboration only if its criterion was committed *before* contact with the outcome.

The sim being built from the math is not contamination — it is what makes it a test *of* the math. The survey questions being shaped by the other lines is not contamination. What destroys evidential value is adjusting what-counts-as-passing after seeing the answer: re-fitting the hypothesis against the data that will judge it (the trading sin — lookahead, overfit), tuning the sim until it matches, softening the acceptance criterion post-hoc. The estate already builds the commitment devices without having named the shared law: **pre-registration** (vivarium — predictions and acceptance criteria frozen before reality answers), the **two-shot delegation pattern** (agent two never fed agent one's output when independence is the point), the **independent-verify gate** (adjudicator ≠ confirmer), and the **de-novo audit** (below).

**Detecting unprincipled coherence vs corroboration — the operational tells:**

1. **Was the criterion pre-committed?** If what-counts-as-agreement was written down before the comparison, the agreement is evidence; if it crystallized during reconciliation, it is coherence. This is checkable from the trail (a pre-registration artifact exists, or it doesn't) — which is why events want a **criterion-commitment field** (`pre-committed` / `post-hoc`), and only the first is lock-eligible.
2. **Divergence-to-root vs reconcile-by-adjustment.** The principled loop (steward-described, and it *is* the estate's practice): match → both lines advance a little; mismatch → both keep digging *until the error or divergence is nailed down completely*. The unprincipled sibling: mismatch → tune parameters until agreement → declare success. The tell is in the trail: a nailed divergence leaves a root-cause finding; a tuned-away divergence leaves only the agreement.
3. **Agreement reached through mutual adjustment is coherence, not corroboration** — the same-author law ("it was all me") operating *between lines*: two lines iterated against each other until they matched were built for each other, and their agreement measures the reconciliation process, not the world. Still valuable (coherence catches errors; the nailed divergences are often the best findings) — but not lock-arming. **Lock-eligibility is restored cheaply and specifically:** one fresh *committed* run of each line's final form (new seed/regime for the sim; independent re-derivation for the math). The reconciliation loop does the learning; the committed re-run does the certifying. Two different event kinds; the ledger records which each was.
4. **Descent-tracing.** Before counting legs as independent, ask what seeded each (see §2): legs descended from one source — one prior-art frame, one author, one dataset — share its failure modes regardless of their method labels. The repair when claimed convergence fails the trace: fall back to the strongest single leg.

## §2. Transmission = the adjudicated/calibrated/qualified ambient prior

*(The steward's phrasing, adopted: not merely "ambient prior" but the* adjudicated/calibrated/qualified *ambient prior.)*

Transmission is not a peer truth-line. Prior-art shapes what gets minted, how the derivation is approached, how the sim is designed, which survey questions exist — **every line is partially descended from it**. Three consequences:

1. **The carriage ladder is the adjudication/calibration/qualification process** — the pipeline that turns raw external material into a *usable* prior: recall-marked-as-recall → search-corroborated → verified-via-secondary → primary-verified, with each item carrying its source's own tier (transmitted ceiling), its verification grade honestly stated, and its walls rechecked on a schedule (403s move). An uncalibrated import is not a prior; it is contamination with a citation.
2. **The lock traces descent through the prior.** Transmission-derived content cannot corroborate any line it seeded; the wrapper-family law (`transmission:<source-family>`, lock looks through) is the static half, and descent-tracing (§1.4) is the dynamic half.
3. **The estate already manufactures the transmission-independent leg deliberately: the de-novo audit.** Priming-stripped readers, auditor-safe front doors, "if I hand you my concept list you will find my concepts" — that apparatus exists precisely to construct a leg whose descent from the corpus's own framings has been cut. The theory's need and the estate's practice meet exactly there.

## §3. Fidelity is not a leaf — it is the most-consumed surface in the system

*(Steward observation, sketch 10: in the v3 chart `accounts.Fidelity` renders as a terminal node.)* The rendering is honest about the v3 net and wrong about reality: the net models no consumer of accounts because it models no auditor. In reality **every warrant question is a read over the trail**:

- **Auditing a line** = auditing its securing events (was the "independent re-derivation" actually independent; does the sweep's seed list exist; the exclusion-pickaxe class of checks).
- **The lock's descent-tracing and commitment-checking** (§1) are reads over accounts.
- **Carriage verification, provenance checks, gold-lift, exempla harvests** are reads over accounts.
- **Reconciling a claim-level audit finding into a line**: the finding lands as a refutation event resetting *the specific securings it invalidates* — the error entered at the derivation step, so re-derivation resets; the premise-naming above it survives (reset-on-edit's each-flag-names-its-reset-condition, at line grain).

So in the coupling map (§5), Fidelity is the **substrate**: every other currency's ladder is only as warranted as the fidelity of its event records — corrupt the trail and every status above it becomes unwarranted simultaneously. Its consumers are the MAINTAIN and audit transitions plus every certification act. (v4 of the net owes the audit transition that makes this visible.)

## §4. The recording question — how this actually lands on disk

*(The steward's framing: the claim body → current-reality/law + append-only log is the easy, already-KPN-ordered part; the lines, the lit review, and the coordination surfaces are up in the air, and beg for sidecars.)* Proposed shape, derived from existing laws rather than invented:

1. **The claim body stays the Kahn-determinate object**: present truth under replacement semantics — the one surface whose content must be arrival-order-invariant and collision-checked. (Already law: [[def-integration-replacement]], [[form-write-isolation]].)
2. **One sidecar per line, append-only, owned by that line's worker** — and this is *derived*, not chosen: [[form-write-isolation]] says concurrency by layout (one record per placement, atomic replace); concurrent line-agents therefore each own their line's event sidecar and never contend. relata's `verifications/<key>/<ts>-<verifier>-<criterion>.md` is the shipped precedent at (record, criterion) grain; this generalizes it to (record, line) grain with the event tuple carrying: channel · actor · date · outcome · **criterion-commitment (pre-committed / post-hoc)** · era-key where measured.
3. **The ledger is a generated projection over the sidecars** — the frontmatter status cell and any outline denorm are computed, never hand-set; max / floor / lock are queries taking the commitment and descent fields into account.
4. **The ambient prior is a *store*, not a per-claim sidecar** — prior-art spans claims, so it lives at store scope (ref/ trees, relata) with **descent edges into claims** ("this claim's framing/design was seeded by these items") as the recorded input to §1.4's tracing. A per-claim `seeded-by:` list is the minimal form.
5. **Coordination surfaces are instruments, not canon**: the in-flight registry (which lines open on which claims, at which rung — the pending surface specialized), and the **reconciliation log** (divergences opened / nailed-to-root / tuned-away-confessed) — both Currency-priced projections over the sidecars, both clocked.
6. **The dispatch half of pay-in** (cascade, collision, line-redirection, projection resets — the previous session-turn's finding) is realized as **clocked drains over the sidecar events**, not as synchronous magic in the pay-in act itself: a standing support-audit reads new events and mints cascade/collision/staleness signals. The carve that keeps Kahn honest: *the recording must be order-invariant; the economizing (early-stop, redirection) may be order-sensitive* — economy is not a race condition.

## §5. The currency coupling map — dependent, orthogonal, loosely causal

*(Rendered in `currency-coupling.svg` beside this file; the forbidden edges are as load-bearing as the real ones.)*

**Substrate relation:** **Fidelity underlies all** (§3) — not a peer edge; every ladder's warrant rests on it.

**Bus relation:** **Disposition couples everything loosely** — questions are how work routes between currencies, and **decision-inflection value-orders them** (the pending decision defines which gap is worth closing).

**Causal channels (legitimate, each with its estate ground):**

| From → To | Channel | Ground |
|---|---|---|
| Evidence → Backing | informs decisions — *never mechanically* (values, resources, stakes enter the second judgment) | L3; GRADE's two uncoupled axes |
| Backing → Evidence | scope rulings re-bound what claims mean (a rescope changes truth-conditions) | rulings-as-fiat; scope calls |
| Evidence → Comprehension | teaching narrates claims; referent-coupled (resets when the claim changes) | FILL WN-3; referent-currency |
| Comprehension → Disposition | misses and reader-confusion are signals | agent-confusion = reader-confusion = defect |
| Evidence ⇄ Conviction | specs commit on believed claims; ground-validation of a spec *generates* evidence (a build refuting a design is an empirical result) | declaration-must-convict |
| Efficacy ⇄ Evidence | firings are empirical events; probes presuppose mechanism-claims | praxes cite the `#ver-` they defeat |
| Backing → Disposition | decision-inflection (forks originate work) | sketch 7 |
| all → Currency | freshness is always relative to tracked records' state | pointer/tracker law |

**Forbidden inference edges (the never-substitute list — each one a documented halo-effect failure):**

- **Backing ↛ Evidence**: `ruled` does not make it true; a decision's authority never launders its groundedness (the sketch's foundation scale exists for the second coordinate).
- **Evidence ↛ Backing (mechanically)**: strong evidence does not auto-decide — both strong-on-weak and weak-on-strong are legitimate cells.
- **Backing ↛ Efficacy**: endorsement cannot move the firing ladder — only real firings adjudicate.
- **Fidelity-corroboration ↛ witness position**: more corroborating accounts raise credibility, never position (steward-caught, the 02 amendment log).
- **Currency/traceability ↛ adequacy**: a perfect chain certifies lineage, never fitness (VIM NOTE 5).
- **Comprehension ↛ Evidence**: a beautiful explanation is not support — plausibility is not verification; a scaffold that teaches well can still overclaim.
- **Conviction ↛ Evidence**: committing hard to a spec does not make the design sound; only ground-validation converts commitment into support.

The map's reading rule: the *solid* edges are where currencies legitimately move each other through recorded events; the *forbidden* edges are where a value in one currency gets *mistaken for* a value in another — and every entry on the forbidden list is a failure mode the estate has already paid for at least once.

## §6. Failure modes — status of the enumeration

The steward's verbatim failure-mode series (`steward-failure-modes-2026-08-14.verbatim.md`) stands at bursts 1–5a (Evidence · Backing ×3 · Fidelity/infidelity). Per the capture protocol, **folding waits until the series rests**; when it folds, entries land in `01-MODEL.md`'s per-currency failure columns marked steward-attested vs interpolated. Currencies still un-illuminated by the series: **Efficacy, Comprehension, Conviction, Currency/freshness, Disposition** — plus the coupling failure modes this document adds to the queue: verdict-channel contamination (§1), tuned-away divergence (§1.2's tell), uncalibrated-import-as-prior (§2), trail corruption as the all-ladders failure (§3), and each forbidden edge in §5 exercised as an inference.

## Open, for the steward

- Whether the **criterion-commitment field** (pre-committed / post-hoc) should be mandatory on all securing events or only on comparison/agreement events.
- Whether `seeded-by:` descent edges are worth their maintenance cost per claim, or belong only on claims whose lock is ever queried.
- The reconciliation log's home: its own instrument, or rows in the pending surface.
- v4 of the net is queued to make §1/§3/§4 structural: commitment field on events, the reconcile loop (divergence → both lines re-open + root-cause question; post-adjustment agreement → coherence column; fresh committed re-run → lock-eligible event), transmission as ambient store with descent edges, and the audit transition that consumes Fidelity.
