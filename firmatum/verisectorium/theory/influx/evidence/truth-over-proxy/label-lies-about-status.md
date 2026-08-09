# Evidence sweep: label lying about status

*Collected 2026-08-09 for theory segment `claim-truth-over-proxy`. Sources: `memorata3-search` over the home estate + `rg --hidden -g '!.git*'` from `~/src/`. Verbatim extracts unless marked commentary. Duplicated passages: oldest instance kept as primary, later copies noted.*

## Gaps / notes on method

- One early memorata3-search call (batched, 5 queries in one background command) either hung or returned an 8MB dump on a sibling sweep's account per the coordinator's warning; subsequent searches were run foreground with small `-n` and a longer tool timeout, which resolved it.
- Two of the six memorata3-search queries used ("INDEX label wrong both directions", "hand-set status token no event behind it") returned largely off-target hits (generic UI/diagram 'label' hits, or API-docs 'status' hits) rather than estate-relevant material — the load-bearing versions of those two topics were instead found via direct grep of `spikes.sop.md` (see below), so nothing is believed missing, but the search itself was low-yield on those two phrasings.
- Did not exhaustively search every project directory individually; relied on memorata3-search's cross-project index plus targeted rg on grep hits it surfaced. Projects not otherwise touched by this task (e.g. neurips, ops) were not separately swept beyond what memorata surfaced.

## Verbatim — the core invariant (routing.sop.md §4.1)

Source: `~/src/arch/asf/doc/sop/audit.sop/routing.sop.md` lines ~110-135 (asf repo, formerly agentic-systems; file dated 2026-05-16 per git-file-origin in memorata).

> ## 4. The no-go protocol (C) (evolved ops)
>
> A no-go collapses **two** things at once, and missing the second is the error this section exists to prevent:
>
> 1. The segment's claim (e.g. `exact`-shows-`abc`) is now **false**.
> 2. **The auditor's suggested fix is *also* false.** Their proposed downgrade (`exact` → `hypothesis`) presumed a weaker-but-true residue. The no-go says there is none *in that form*. The probability wave has collapsed to zero. **There is no downgrade option.**
>
> The steps, in order. Steps 4.1–4.4 are committed **before** the audit finding is routed (4.5); routing a no-go-bearing finding while the segment still lies about itself is the cardinal sin.
>
> **4.1 — The invariant, stated once and absolute.** *A segment must never lie about its own status, even transiently.* Tracking files carry the change, the why, the inner process. **Segments carry only present truth with correct bounds and accurate epistemology, at all times.** Whether a claim is true, whether the model is useful, within the degrees we have defined — the canon is *always honest about itself*. This is **authoritative SOP**; everything below serves it.
>
> **4.2 — Mark the segment immediately if the fix is not obvious; gate the fast-path if it is.** If the correct new form is genuinely obvious and a few targeted edits away, you may skip the `FALSE`-marking *interval* and the brainstorm-plan and move toward the replacement — **but not unsupervised.** "Obvious" is itself the failure-conviction word here: a softened ghost is a *wording* failure... Otherwise — *before figuring out next steps takes any time at all* — set the segment's epistemic status to **`FALSE`** (or the cascade-appropriate `unlikely in current form`), **in the segment**, with a link to the spike / discussion, and a visible `TODO: FIXME`. Not a quiet note in tracking. In the segment. A reader encountering it mid-repair must see that it is known-broken, not be misled for the duration of our uncertainty.
>
> **The falsified claim does not lie alone (cascade closure).** Directed separation means a no-go upstream propagates: *every dependent of the falsified claim must be found and either cascade-marked (`unlikely in current form`) or re-derived.* This is an obligation with a verification step, not the old parenthetical "the cascade matters" — an un-marked dependent of a falsified claim is the same "segment lies about itself" failure, one hop away.

Commentary: this is the strongest, most-cited formulation — "a segment must never lie about its own status, even transiently" is quoted verbatim (with attribution back to §4.1) throughout the corpus, including in udon (see below) and in agent transcripts working the AAT canon.

---

## Independent restatement — udon FORMAT.md (verbatim, generic-vocabulary rendering)

Source: `~/src/udon/v2/theory/FORMAT.md`, dated 2026-07-29 (this is a udon-project rendering of the same principle, so it postdates the asf original above and is a downstream/parallel articulation, not the origin).

> ### The no-go protocol
>
> A no-go collapses **two** things, and missing the second is the error this exists to prevent: the claim is now false, *and* the suggested downgrade is also false — it presumed a weaker-but-true residue, and the no-go says there is none in that form. **There is no downgrade option.**
>
> **The invariant, absolute: a segment must never lie about its own status, even transiently.** The history layer carries the change and the reasoning. Segments carry present truth with correct bounds at all times.
>
> So: mark the segment immediately if the correct new form is not obvious — before...

---

## Verbatim — spikes.sop.md, "INDEX label is convenience record, not ground truth"

Source: `~/src/arch/asf/doc/sop/spikes.sop.md`, dated 2026-06-01 (git-file-origin per memorata; content documents events from 2026-05-17/18/19).

> The failure: **math or a no-go that is real and true but lives only in the spike** — sometimes referenced from a segment's Working Notes, sometimes not referenced at all. *A reference is not integration.* Per `~/.claude/memory/...feedback_math_lives_in_segments.md` and audit-routing §4: good math derived in a spike never resides only in the spike; it lands in a segment or (more often) a new appendix. A no-go is present-tense canonical truth (audit-routing §6), not archaeology.
>
> **The decisive test for "integrated":** the load-bearing content appears in a `src/` segment or appendix, **verified first-hand** — not the INDEX label, not a Working-Notes pointer from a segment, not an agent summary.

> ### 2-bis. "Fully integrated" is a three-part completion criterion (Joseph 2026-05-18)
>
> A spike is not `integrated` (and must not be filed to `.integrated/`) until **all three** hold — partial satisfaction is the looks-done-but-isn't trap:
>
> 1. **Content present in canon**, verified first-hand...
> 2. **Nothing *needs* to reference the spike anywhere.** ...
> 3. **The navigator is reconciled.** `TODO.md` / `PROPOSALS.md` / `PRACTICA.md` **and the spike-corpus navigators `spikes/INDEX.md` and `spikes/PROPOSED.md`** — items that the integration resolves are **closed at cycle-commit time, not deferred** (triage-is-the-answer); items it *advances but doesn't close* are updated with the new disposition + spike-routing cross-ref; **a navigator entry that still says "partially landed in #X" when #X is now `status: false`/superseded is a navigator-level §4.1 lie** and is corrected with the same urgency as a segment one. "Find more info" cuts both ways — a navigator item may *resolve* the spike or *reopen* it.
>
> ...Two disciplines bind, both about keeping what *is* there trustworthy rather than exhaustive: **(freshness)** what is there stays true — a resolving spike sets its row to a terminal status + canon link at cycle-commit time (**a stale "open" row for a landed direction is a navigator-level §4.1 lie**), landed rows kept-not-retired as the audit trail...

Later in the same file, §7 (git-provenance-as-proxy discussion):

> An `open`/`blocked`/`IN PROGRESS` label is enough to keep a spike out of `.integrated/`; a `LANDED`/`PROMOTED`/`VERDICT` label is a **hypothesis to verify**, never sufficient on its own... **The INDEX label is the *convenience record*, not ground truth** — exactly as the audit cycle learned its audit-id→ledger mapping was unreliable and had to be primary-source-verified.

## Verbatim — the "wrong in both directions" incident (Refinement 1)

Same file, the SOP's own scar-log at the bottom (dated 2026-05-17, diagnostic pilot 023198):

> *Refinement 1 (2026-05-17, diagnostic pilot 023198 — `spike-operator-sector-unification` + `spike-c2-star-to-integrate`). Three frame defects caught before any fan-out, folded above: (1) no clean cell for cross-repo / externally-blocked spikes → §3 decision rule (`spike-c2-star` is `live-or-open` because its proof's source-of-truth is `~/src/behavioral-floor/`'s in-review paper, not because it is incomplete); (2) the Joseph-batch gate keyed on file-vs-dir when the load-bearing axis is reserved-decision-type → §6 criterion (safe-direction, pending ratification); (3) tightly-coupled sibling spikes are mis-routable when split across fan-out slices — adjudicating `spike-operator-sector-unification` forced a check of `spike-update-operator-sector` (a genuine `orphaned`-suspect, not in slice), which an agent trusting "the operator family is done" would mis-route → partition keeps sibling clusters in one slice, siblings surfaced from outside a slice are flag-don't-route. Confirmed, no change: the first-hand decisive-test read is non-optional — **the INDEX label was wrong in both pilot cases, in opposite directions (understated for spike 1, accurate only by encoding an external block for spike 2).** The transferable scar: the convenience-label is unreliable in both directions, not only the optimistic one; the first-hand read is budgeted as mandatory every fan-out slice, not as a spot-check.*

## Verbatim — Refinement 10, the PROPOSED.md drift incident (2026-05-19, Joseph-caught)

Same file, dated 2026-05-19:

> *Refinement 10 (2026-05-19, Joseph-caught — a navigator that drifted because the SOP did not name it). `spikes/PROPOSED.md` (the spike-proposals catalog) had silently drifted: last content-curated 2026-04-25, with at least four entries (Causal-IB LMI, message-passing credit-assignment, FEP-as-suboptimal, Landauer/thermodynamic-cost) still presented as open candidates "requiring repair" when their repairs had landed in canon — **the exact navigator-level §4.1 lie §2-bis(3) exists to catch**, but §2-bis(3) named only `TODO.md`/`PROPOSALS.md`/`PRACTICA.md`, not the spike-corpus navigators, so the obligation never attached to it. Caught not by the SOP but by Joseph's question "are we still tracking a suggested-spikes / proposed or something?" — and his follow-up directive: "be sure, and add a constant update to it in your SOPs." ... Meta-scar for the next steward: **a reconciliation instinct that is correct for a *lying* navigator (fix it with teeth) becomes wrong when applied to a *low-friction convenience* (don't systematize it into a duty); the tell is when "keep it honest" silently becomes "keep it exhaustive."***

## Related — §D.9 navigator-level §4.1 lie (second citation)

Same file, line ~332 (context: a spike-routing cross-reference document, PROPOSALS §D.9):

> ...*"#internal-external-decomposition"* (PROPOSALS §D.9, TODO Group I) when that segment is now `status: false` — a **navigator-level §4.1 lie**,

---

## De-novo-audit instructions — status-label verification as a named audit emphasis

Source: `~/src/agentic-systems/doc/de-novo-audit-instructions.md` (path predates the archema-io/arch rename; original per memorata dated 2026-04-25 — this is the oldest instance of this passage found).

> ### 5.4. Status-label verification (an emphasis available to you)
>
> Each segment carries `status:` in frontmatter (`exact`, `robust-qualitative`, `conditional`, `discussion-grade`, `sketch`, etc.) and equation-level tags (`*[Derived]*`, `*[Formulation]*`, `*[Hypothesis]*`, etc.). For each substantive claim, ask whether the label matches. **A `status:exact` segment with mostly-conditional content is a finding.** A claim tagged `*[Derived]*` whose own Epistemic Status admits "discussion-grade" is a finding. This emphasis is high-yield when the framework has many recent revisions or when status labels appear inconsistent at first glance.

A near-identical passage recurs, generalized to project-agnostic vocabulary, in `~/src/MOVED/udon/v2/theory/to-integrate/refine-more/de-novo-audit.md` (dated 2026-07-30) and copied again into this very project's own archive at `~/src/arch/firmatum/verisectorium/.archive/theory-misfire/.archive/udon-analysis/de-novo-audit-generic.md` (dated 2026-08-05) — a template that has been generalized and carried forward at least twice. Per "oldest wins," the agentic-systems original above is the canonical source; the other two are downstream copies, noted for completeness only.

---

## Caught instances of status-label mismatch (audit findings)

### AUDIT-WORKING-471203 — segment 15/16, self-correcting example

Source: `~/src/arch/asf/audits/AUDIT-WORKING-471203/.integrated/15-der-recursive-update.md` (dated 2026-04-29):

> ## Status-label observation
>
> **The YAML status is `conditional`. The Epistemic Status text begins "Exact, with a partly definitional character."** A reader scanning YAML sees `conditional`; a reader reading prose sees "Exact." The two should match more cleanly. Either:
> - The frontmatter status should be `exact` (with the conditional-on-C1+C2+C3 noted in prose), or
> - The prose should lead with "Conditional (on definitional completeness of $M_t$): the result follows exactly from C1+C2+C3, where C3 is definitional..."
>
> This is a **candidate light finding** — scope/status label mismatch. Severity...

Companion file `16-deriv-recursive-update.md` (same audit, same date) — the finding is later withdrawn on closer reading, itself a good specimen of the discipline (don't assume mismatch = lie; verify what each label actually asserts):

> ## Status-label cross-check
>
> The calling segment `#der-recursive-update` has status `conditional`; this appendix derivation has status `exact`. **My earlier candidate finding (segment 15) about status-label mismatch in the calling segment may be incorrect:**
>
> The two statuses are describing different things:
> - Appendix `exact`: the derivation is rigorous given the three constraints.
> - Body `conditional`: the result holds conditionally on the constraints (specifically on C3 as analytical commitment) being accepted.
>
> These are two different angles on the same derivation. The layering is honest... **Withdrawing the candidate finding from segment 15.** This is a clean honest layering, not a status-label mismatch. Lesson learned: status fields can carry different meanings at different layers of the dependency chain, and the apparent mismatch may be deliberate.

### AUDIT-WORKING-584721 — honest labeling, contrast case (not a violation, cited for calibration)

Source: `~/src/arch/asf/audits/AUDIT-WORKING-584721/.integrated/09-result-mismatch-decomposition.md` (dated 2026-04-25):

> ## Status-label / discipline
>
> `status: exact` defended carefully — the Epistemic Status names GA-1 as the load-bearing assumption and notes the decomposition is "a mathematical identity (bias-variance applied to prediction)." Honest.
>
> `stage: claims-verified` — high stage; consistent with the math-correctness check.

And `14-der-deliberation-cost.md` (same audit):

> ## Status-label / discipline
>
> `status: conditional` (on the local deliberation-drift assumption). The Epistemic Status carefully notes the assumption is local and validated by consistency with global mismatch dynamics. Tier-stratification within tags is honest: `*[Assumption]*` for the drift; `*[Derived (Conditional on ...)]*` for the threshold and FOC.
>
> `stage: claims-verified` — appropriate given the careful Epistemic Status.

### audit-471203-SUPPLEMENT-phase-2 — a caught mismatch, resolved by a structural fix

Source: `~/src/arch/asf/audits/.integrated/audit-471203-SUPPLEMENT-phase-2.md` (dated 2026-04-28):

> ### Finding 2 (status-label / type / Epistemic Status mismatch)
> **Disposition (revised):** **new instance of known pattern**. The exact `disc-ciy-unified-objective.md` discussion-grade-vs-Exact mismatch is new. Related theme in `audit-742613-FINAL-2026-04-25.md:220` ("scope/status mismatch" type). **Now fixed by §H.1 above (the layered-status framing).**

---

## Directory-label honesty — `.integrated/` vs `.archived/` collapse (transcript, not primary doc)

Source: `~/.claude.bak.2026-05-18/projects/-Users-josephwecker-v2-src-agentic-systems/80bd5ec8-eb35-4f6a-814a-fe6063f1bcc9.jsonl` (agent-thinking, dated 2026-05-17T22:13, this is a raw transcript — marked as such, not a canonical doc):

> ...y, the directory-label honesty, and the canonical failure that's specific here: *math left only in the spike, sometimes referenced, sometimes not*). The voice/scar discipline (§9) carries over intact.
>
> **2. The disposition taxonomy I see — five states, one of which is the real work:**
> - *Integrated & correctly filed* (in `.integrated/`) — verify a sample, don't trust the label.
> - *Integrated but mis-filed* — content **is** in canon, spike never moved. Safe-mechanical: spot-check + `git mv`. (Your "fully accounted for but didn't get moved" bucket.)
> - *Orphaned result — success **or** no-go* — c[truncated by search snippet]

The brief's framing ("collapsing `.integrated/` with `.archived/` is a label lying about status at the directory level") was not found verbatim in this sweep — the closest primary hit is the "verify a sample, don't trust the label" framing above, applied specifically to the `.integrated/` directory as a status-claim. Flagging as a possible paraphrase/compression in the brief rather than a literal quote found in the corpus; worth the coordinator double-checking against spikes.sop.md §2 more closely if the exact phrase matters.

---

## Transient status: FALSE marker practice (agent transcript)

Source: `~/.claude.bak.2026-05-18/projects/-Users-josephwecker-v2-src-agentic-systems/fc734ed5-1a1a-4887-8538-b94855040cba.jsonl` (agent-thinking, dated 2026-05-16T15:06:14):

> ...ement wearing unnecessary defensive cruft. That's cosmetic, categorically not the dangerous failure mode (canon asserting false epistemology). I flattened two different things into one severity scale.
>
> And the process steps I never carried at all — **7.2 transient `status: FALSE` in the segment with a link; 7.3 a `spikes/<name>-spike-integration-plan.md`; 7.4 a PRACTICA active-todo reference; 7.5 routing only after those are committed** — were simply absent from the inherited tracking file, which is itself your point: the filter didn't know these existed, so neither did I. The hard invariant I...

Same-day companion transcript, same file family (dated 2026-05-18T01:05:20):

> ...variant is strong ("a segment must never lie about itself, even transiently"). But we *also* just had the searing lesson that even strong independent spikes overclaim and need the gate. The resolution from the audit-routing §4.2 / §0 discipline: the lead agent verifies the *decisive* claim first-hand before marking; for the *checkable canon-internal* part, I can and should verify now...

---

## Navigator freshness incident — MANIFEST-2026-05-31.md (now-false narrative tails corrected)

Source: `~/src/arch/asf/spikes/.integrated/MANIFEST-2026-05-31.md` (dated 2026-05-31):

> ## Navigator reconciliations done (same batch)
>
> - **`spikes/INDEX.md`** — the four cycle-section rows (the 2026-05-31 value-object, 2026-05-30 objective-functional, 2026-05-30 continuity-stance, and 2026-05-30 causal-access entries) reconciled from "COMPLETE — awaiting external-eye review / Proposes no canon edits" to terminal **INTEGRATED + FILED 2026-05-31** status: Location columns point at the `.integrated/<spike>` paths, Status columns name the per-spike canon landings + commit hashes and point here. **The now-false "No canon edits; per-segment integration drafted ... external-eye review gate" tails in the four section-narrative paragraphs were updated to record the 2026-05-31 integration.**

Companion transcript (subagent), `~/.claude.bak.2026-06-02/projects/-Users-josephwecker-v2-src-agentic-systems/ee54600f-8135-44b5-aa94-2e8dd50d3110/subagents/agent-a3effca6cb397893c.jsonl` (dated 2026-05-31T00:36:01):

> ...anon edits DID land). Let me fix those trailing clauses in the four narratives to avoid leaving false statements, matching the integration-is-replacement discipline. Let me read those exact sentences.
>
> The four narratives end with these now-false tails:
> - Line 12: "No canon edits; per-segment integration drafted inside the spike for an external-eye review gate."
> - Line 20: "No canon edits; per-segment integration drafted inside the spike for an external-eye review gate."
> - Line 28: "No canon edits; per-segment integration (touching ...) drafted inside the spike for an external-eye gate. Resolv[truncated]

Another subagent transcript, same incident window, `~/.claude.bak.2026-06-02/projects/-Users-josephwecker-v2-src-agentic-systems/ee54600f-8135-44b5-aa94-2e8dd50d3110/subagents/agent-a4a359714933953fb.jsonl` (dated 2026-05-31T00:28:09):

> The navigator scan is thorough. Findings:
>
> 1. **spikes/INDEX.md** — all four spikes are present with full cycle-header entries and table rows, accurately stating verdicts (B/B+no-go). These rows still read "**COMPLETE — awaiting external-eye review**" and "Proposes no canon edits." **That is now stale** — the integration *has* happened (the external-eye review landed via commits 52d85b2 / 3da7e32 / 9fb7c12 / 0d57da2). Per §2-bis(3) freshness discipline, when these spikes move to `.integrated/`, the INDEX rows should be updated to a terminal "integrated / filed" status with the canon-link. This...

---

## Interim honesty banner as a mitigation pattern (rather than silent staleness)

Source: `~/.claude.bak.2026-05-18/projects/-Users-josephwecker-v2-src-agentic-systems/80bd5ec8-eb35-4f6a-814a-fe6063f1bcc9.jsonl` (agent-thinking, dated 2026-05-18T02:07:53):

> ...s committed (durability gap closed). Canon segments lint-clean. `spikes/` top-level is scannable again.
>
> ## Honestly pending — *not* claiming done (the §0c discharge: tiered + queued in durable homes)
>
> - **`spikes/INDEX.md` row reconciliation** — ~12–15 stale rows. I did **not** cram it at the tail of this turn (rushed-error risk, the exact failure this session is about). Instead: an **interim honesty banner** at INDEX top now flags the cycle-touched rows as known-stale and points to `ROUTING.md` as authoritative — **so it no longer silently misleads** — and `ROUTING.md §4` marks the full pass...

This is a distinct, adjacent discipline worth flagging for the theory segment: when a full reconciliation can't be done immediately, the honest move is a visible "known-stale" banner at the label site itself, not silence.

---

# Adjacent finds

## Adjacent finds (marked, at the end per brief)

### P5-status prediction and worked instance (audit-findings-542891.md, asf)
> **P5-status.** "Status Label Mismatches: I expect to find some segments labeled `exact` that should be `conditional`, or `derived` claims that rely on `hypothesis`-level assumptions in the appendices."
> ...the 471203 cycle's F2 (`#disc-ciy-unified-objective` status-label mismatch) is a worked example of a real instance that *did* get caught.

### Cross-file confidence mismatch (udon, README vs OUTLINE) — same failure shape, different surface
Source: `~/.claude/projects/-Users-josephwecker-v2-src-udon/fc191a72-ba09-4ee9-b7de-a35dd0e1d57e.jsonl` (agent-thinking, 2026-07-29):
> The real issue emerges when I look at how those same claims appear across files: the README states them in confident prose with no hedging, while the OUTLINE marks them with `:status` labels showing they're conditional or discussion-grade. The README uses result-voice ("turns out to be," "stop being four territories and become four directions") for claims I've flagged as uncertain in the OUTLINE

Not a directory/frontmatter label per se, but the same species: a surface asserting a stronger epistemic state than the authoritative label elsewhere in the same project.

### Stale classification labels in result files (causal-language, code/data context, not theory canon)
Source: `~/src/causal-language/audits/.integrated/2026-05-14-opus-audit-code-methodology-math.md`:
> This means the per-script result text files have **stale classifications** — the script in the repo does ANOMALOUS-SIGN-FLIP, but the result text files show "C1-dominant". ... the cross-model aggregator is the source of truth for classification — but a reader looking at result files alone gets a stale label.

Different domain (empirical result classification, not epistemic-status canon) but same shape of finding: a materialized label drifts from the generating source of truth.

### Max attainable status (format.sop.md) — related discipline, not a violation-finding
Source: `~/src/arch/asf/doc/sop/format.sop.md`, "Max attainable status" section — every segment has a ceiling status it can ever honestly reach; noting the ceiling in the Epistemic Status paragraph prevents wasted effort. Adjacent because it's about calibrating what a label *could* honestly say, not about a label lying — included for completeness since it surfaced in the same search cluster.

### Historical-name is not a "stale label" (msc/naming/README.md, asf) — the inverse case
Source: `~/src/arch/asf/msc/naming/README.md`:
> Sweeping those would make them falsely read "ACT → AAT", an event that never happened. Their "AAD" is the *object the document is about*, not a stale label. Leave them exactly as-is.

Worth flagging because it's the discipline's own guard against over-application: correctly-preserved historical naming inside frozen archaeology is not itself a label-lying-about-status instance — updating it would be the actual lie (asserting an event happened that didn't).

### Note on a phrase not found verbatim
The brief's phrasing "collapsing `.integrated/` with `.archived/` is a label lying about status at the directory level" (attributed to spikes.sop §5) was not found verbatim in `spikes.sop.md` itself under that heading — spikes.sop.md's closest material is the "verify a sample, don't trust the label" framing (§02-index-label-not-ground-truth.md above) and the `.integrated`/three-part completion criterion (§2-bis). **The exact phrase does exist, verbatim, in this project's own canon**: `~/src/arch/firmatum/verisectorium/theory/src/def-integration-replacement.md` line 35 — "Collapsing the two is a label lying about status at the directory level" — under Definition (delete-test), describing `.integrated/` vs `.archive/` as truth-claims. This appears to be verisectorium's own prior codification of the principle (possibly what the brief is quoting from, or drawing toward), not an independent external estate source — worth the coordinator's attention since it may mean this segment's claim is partly citing itself.

---

**Coordinator annotation (2026-08-09):** per this slice's flag — the exact phrase "collapsing the two is a label lying about status at the directory level" is our own `theory/src/def-integration-replacement.md` wording; the underlying concept is spikes.sop.md §5's ("collapsing them is the directory-level form of the label-lies-about-status error"). The drafted segment should cite spikes.sop §5 for the concept and must not cite our paraphrase as an external source.
