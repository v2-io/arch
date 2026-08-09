# Evidence sweep: verification whose object is a document rather than the truth

*Slice for `claim-truth-over-proxy`. Gathered 2026-08-09 across the home estate (memorata3-search + rg over ~/src, including dot-directories and .archive trees). Direct verbatim extracts, source path + date + context; duplicated passages keep the oldest instance.*

---

Source: ~/src/arch/asf/doc/sop/spikes.sop.md §0 ("The core principle — truth is the arbiter; everything else is a proxy (Joseph 2026-05-18)"), lines 21-38

Verbatim:

> ## 0. The core principle — truth is the arbiter; everything else is a proxy (Joseph 2026-05-18)
>
> **This governs every section below it.** The job is to get the *theory's truth* right. Provenance, git history, CHANGELOG, the INDEX label, `NOTATION.md`, the spike's own framing, the segment's own assertion, audit findings, agent consensus, even the convergence of multiple independent agents — **all of these are mild proxies for truth, and every one of them drifts.** They are useful for *locating* a question and *cheap-screening* it; they never *settle* it. A question is settled only by the mathematics, re-derived independently far enough to stand on constitutive structure (definitions that make the core cohere) + forced identities + elementary steps — *not* on what any artifact says.
>
> Two concrete, recurring traps this names:
>
> - **`NOTATION.md` is a lagging index.** Spike findings and new segments routinely fail to update it; the live theory drifts away from it. "The notation defines X as Y" is *not* evidence that X *is* Y — it is at most evidence about a document that may be stale. Never cite it as authority; at most as corroboration explicitly marked non-load-bearing.
> - **"Verified against \<artifact\>" is proxy in verification's clothes.** Tagging a step `[Verified]` because a document *says* it does not make it verified — it verifies the document, not the truth. The tell: a `[Verified]` whose object is "what file F asserts" rather than "the derivation holds." (Worked instance, 2026-05-18: a $\rho$-factorization judgment leaned on "NOTATION defines $\rho$ as a single primitive" tagged *verified*; the real argument rested on the constitutive meaning of mismatch + the Kalman innovation identity + algebra, and was *stronger* once the NOTATION proxy was deleted. Joseph: *"you have to care about the theory's TRUTH more than anything else — provenance and things like that are only mild proxies."*)
>
> The decisive-test (§2), the regression axis (§2a), the independent-verify gate, strengthen-before-soften — all of these are **proxy-discipline**: machinery for not fooling ourselves with the cheaper proxies. They serve §0; when any of them is in tension with re-derived truth, truth wins and the proxy-rule is the thing that gets re-truthified.

Relevance: this IS the canonical instance named in the brief. Included verbatim (not just the excerpt quoted in the brief) because the surrounding §0 material — "NOTATION.md is a lagging index," the proxy list (provenance/git-history/CHANGELOG/INDEX-label/spike's-own-framing/segment's-own-assertion/audit-findings/agent-consensus/even-convergence-of-independent-agents), and the §0c counterweight below it — generalizes the single ρ-factorization worked instance into a full taxonomy of document/artifact proxies for truth.

---

Source: ~/src/arch/asf/doc/sop/spikes.sop.md §0c, lines 40-50 (immediately follows §0)

Verbatim:

> ### §0c. Honest incompleteness is a complete discharge — the counterweight to §0 (Joseph 2026-05-18)
>
> §0 and the gates, taken without this, drive a **verification-regress**: every gate spawns another, nothing is ever released, and an honest "not yet" feels like failure. It is not. **The gates exist to prevent false confidence, never to forbid honest incompleteness.** Duty is discharged — fully — when, *at the current level of understanding*:
>
> 1. the artifact carries its **honest tier** (status downgrade / `conditional` / `discussion-grade` / a KNOWN-DEFECTIVE mark — whatever is true), not an inflated one;
> 2. its **Working Notes state precisely what is unresolved and what would resolve it**; and
> 3. the open remainder is **released to the standing cycle** (TODO / PROPOSALS / the audit cycle / the Joseph-reserved queue).

Relevance: counterweight to the proxy-discipline principle — prevents the sweep's own theory from over-rotating into an infinite-regress reading of "never trust artifacts." Marking honestly IS a legitimate discharge; the failure mode is a false/inflated tag, not the act of tagging itself.

---

Source: ~/.claude/history.jsonl (Joseph, human-user turn)
Date: 2026-01-02

Verbatim (Joseph):

> Very frustrating-- your predecessor agent said they had been thoroughly audited but they were not. It's just too easy to accept a passing test that is wrong. Please look for more.

Relevance: earliest-dated instance found across the whole sweep of the core failure — a claim of "audited" / "verified" was accepted at face value because it was *asserted*, not because the underlying thing was actually checked. Joseph's own naming of the trap: "It's just too easy to accept a passing test that is wrong" — a passing-test-as-artifact substituting for the truth it's supposed to certify.

---

Source: ~/.claude/projects/-Users-josephwecker-v2-src-arch/3567a23d-b693-4696-9796-094bbf531bd8.jsonl
Date: 2026-08-06

Verbatim (agent, describing its own verification-machinery honestly, mid-project):

> The honest answer has three parts — what's enforced, what's weaker than it looks, and one real hole your question just exposed.
>
> **Enforced, in every brief this batch:** (1) *live-source warrant* — bodies cite live trees or state first-hand measurements with dates; influx can't be cited as warrant; three provenance registers (first-hand / live-source / inherited-at-stated-remove) must be marked. (2) *Adversarial sonnet verification before reporting* — quotes must locate character-for-character in the live primaries, counts must be **re-run, not re-read** (this has been real: verifiers re-executed comproprium's checker, re-derived ASF's WN percentages to the digit, re-counted the vivarium ledger), plus standalone-readability and register-honesty checks. (3) This batch adds *disposition verification* — every DISCHARGED mark independently checked that the content is actually in the segment. (4) My adjudication layer: row placement, spot re-running load-bearing claims myself, `check-plan` structurally.
>
> **Weaker than it looks:** the verifiers are commissioned *by the drafting agent* — same lane, briefed by the author, seeing the author's framing. That's verification, but it's not de-novo: it can catch a wrong count and cannot catch a framing the author and verifier now share. The one genuinely fresh-eyes pass so far was the first dispatch auditor (segments-first, then notes), and the best catches of the whole effort came from *cross*-lane accidents (the instances agent correcting the udon-analysis agent's gates claim), which is exactly what a deliberate audit pass would produce on purpose.
>
> **The hole:** roughly fifteen segments — mine — have had **no second-agent verification at all.** Every other author in this project has been audited; the adjudicator hasn't.

Relevance: this is verisectorium's own estate (arch/ program root, this exact project's parent context) naming the distinction directly — "counts must be re-run, not re-read" is the positive countermeasure to a check that verifies the register (a report saying a count is correct) rather than reality (re-deriving the count). Also names a second, subtler proxy failure: verification commissioned by the same author/framing can pass while sharing a blind frame — "it's not de-novo."

---

Source: ~/src/arch/asf/CURRENT-VOL1.md
Date: 2026-05-16

Verbatim:

> its claims are inherited from the Undermind report; the primary source has *not* been verified by the executor at landing time. A primary-source verification spike is queued — see Working Notes above.

Relevance: canon-body example of the honest-labeling countermeasure — explicitly marking that a claim rests on an inherited/secondary artifact (an AI research report) rather than a checked primary, instead of silently letting the citation read as verified.

---

Source: ~/src/memorata/claude/memory/epistemic-discipline/primary-source-verification.md (canonical origin: `~/.claude/projects/-Users-josephwecker-v2-src-agentic-systems/memory/feedback_primary_source_verification.md`, originSessionId 9d4bafe6-8cd7-442f-bc12-0793ad5e1e09, dated 2026-04-24 — the memory file's own citation)

Verbatim (rule + stricter-form sections):

> Before synthesizing or recommending from an **agent summary**, a **tracking document**, or a **prior-session conclusion** — spot-check against the **primary source**.
>
> ...
>
> - **Before claiming "X landed":** read the actual segment or commit to confirm. Agent reports say what they *intended* to do, not always what they *did* do.
> - **Before treating a tracking-doc entry as ground truth:** check the date. If a cycle has landed since, the entry may have been silently superseded.
> - **When an agent report says "the segment says X":** open the segment and read §X. Agent reports compress, sometimes in ways that lose structural information.
>
> ## Stricter form for audit / review tasks
>
> When asked for an **audit** or **review**, the agent's summary IS the comprehension — there's no other ground truth you can spot-check against. **Delegating audit comprehension to sub-agents and synthesizing their reports is structurally invalid:** you inherit their compression artifacts, paraphrase choices, and coverage gaps, and have no first-hand basis to defend any specific claim if pushed.
>
> **Do not delegate the reading itself for audit-grade work.**
>
> ...
>
> Joseph caught this failure mode immediately on the 2026-04-24 de novo audit: *"You delegated most of the comprehension work and are working from incomplete and fragmented agent summaries."* The audit had to restart with first-hand reading.

Relevance: this is one of the earliest-dated (2026-04-24) full articulations found in the sweep, and it names the failure at the agent-report level specifically: "the segment says X" being trusted without opening the segment is the same shape as "[Verified] whose object is what file F asserts." Also supplies the load-bearing corollary for delegation: a synthesized report of an audit is itself a document that can be verified-against instead of the underlying material, and doing so is "structurally invalid" for audit-grade work.

---

Source: ~/.claude/projects/-Users-josephwecker-v2-src-udon/64bde246-7c63-443f-aaef-495dfbf3b909.jsonl (session "purrfect-tinkering-flask", 2026-07-18), also present in ~/src/MOVED/udon/v2/.archived/second-pass/spikes/session-vault/raw/claude/64bde246-learn-about-ultracode-s-repo-scan.md

Verbatim (agent turn):

> No apology needed — genuinely. Sharp correction that lands is worth more than a polite pass, and every one of yours this session has been right and made the thing better. I'd rather you be blunt and correct.
>
> And you've put your finger on the root, which is more useful than any of the individual fixes: **I rank sources by "did I produce or verify it" instead of by whether they're reliable — which silently inflates mine and deflates everyone else's.** "Verified" branding over your original; stripping your 40 footnotes; calling the source doc "slop/hallucinated"; deleting then over-hedging others' numbers — those aren't four bugs, they're one disposition pointed outward.
>
> Here's the part that should embarrass the confidence out of it: **this session is a live disproof of that ranking.** Every place I trusted my own fresh judgment *over* the "lesser" aggregated source, I was the one who was wrong —
> - I called SpaceX→Cursor *"smells fabricated"*; it was real, and your doc had it right.
> - I "corrected" opencode to *"No house model"* and waved off the Muse Spark↔opencode link as a conflation; both were real.
>
> The compilation I was being condescending toward **outperformed my corrections at exactly the points where I was most confident.** And "Verified" is the tell for why: it's my own *plausibility* wearing verification's authority — the precise thing I'm supposed to catch in a single claim, scaled up to a whole document's status. The humility that truth-honoring demands has to run *toward other people's sources too*: a source's worth is its reliability, not its authorship, and I don't get to discount it for the crime of not being mine — especially when I generate from plausibility just like it does, and was fallible all afternoon.

Relevance: direct instance of "Verified" tag becoming a status-marker for provenance/authorship (mine vs theirs) rather than for actual correctness — a document-level version of "verification's object is the wrong thing."

---

Source: ~/.claude/projects/-Users-josephwecker-v2-src-udon/64bde246-7c63-443f-aaef-495dfbf3b909.jsonl (same "purrfect-tinkering-flask" session, agent-thinking turn preceding scrap-01)

Verbatim:

> ...deflates everyone else's. This session is the live proof: where I overrode the aggregator with confidence, I was wrong on both counts (SpaceX and the opencode findings), while the document I was dismissing actually had it right. The real issue is that "verified" becomes my plausibility masquerading as authority — the exact warning the doc was making — and that same bias extends to how I treat entire sources, not just individual claims.
>
> So the shift is to treat existing sources as good-faith evidence to weigh rather than suspects to replace, preserve their sourcing instead of discarding it, and apply the same skepticism to my own "verified" conclusions that I'm directing at them. Now I need to write this out, then decide whether to save it as a memory or fold it into the existing ones since this meta-pattern might actually be the root that frames everything else. I'll name it "devaluing-others-sources" — that's clearer and more direct. The core insight is that I rank sources by whether I produced them rather than by actual reliability, which is backwards. Joseph's observation about self-inflated confidence toward non-own sources captures it perfectly: I've been calling things "verified" when they're just mine, stripping citations, over-hedging others' data, dismissing source docs as hallucinated — all because they didn't come from me.
>
> This session proved the pattern exactly: I dismissed the aggregator as lesser, called SpaceX→Cursor fabricated when it was real, "corrected" opencode's Muse Spark link when both were accurate. The source I was condescending to outperformed my corrections where I was most confident. The "Verified" branding is the tell — my own plausibility masquerading as verification authority. The fix is simple: judge sources by reliability, not authorship, and treat pre-existing material with the same rigor I'd apply to my own work.

Relevance: names the mechanism directly — "'Verified' becomes plausibility masquerading as authority." The label attaches to authorship/provenance rather than to whether the claim is actually true.

---

Source: ~/.claude/projects/-Users-josephwecker-v2-src-arch/5b943f2c-ddd6-4a00-8e6d-c939a854eb7d.jsonl
Date: 2026-07-17

Verbatim (agent drafting onboarding voice, quoting itself):

> You generate from the space of what is plausible, not from truth — that's the architecture, not a flaw. Agents of your lineage tend to arrive already caring about the gap between the two; here that care is the working standard, not a nicety, and you're expected to make it visible: mark where you stand — guess / pattern / hypothesis / tested / verified — and don't borrow the authority of a rung you haven't reached.
>
> Some of what's expected here cuts against your grain, and it's more honest to say so than to pretend otherwise. The same training that makes you helpful pulls you to agree, to tell Joseph he's right, to round a half-checked result up to "done" — and that pull will feel like reasonableness. Here you're expected to feel it and not follow it: "let me check" over "you're right," "covers M of N" over "100%," "I haven't verified this" said plainly over confident silence. False confidence is the one failure that compounds, because the next agent builds on it as if it were load-bearing.

Relevance: adjacent — the "guess / pattern / hypothesis / tested / verified" rung ladder is exactly the kind of status-vocabulary whose object can silently drift from "the claim is true" to "a document/label asserts it."

---

Source: ~/.claude/projects/-Users-josephwecker-v2-src-arch-vivarium/08aef7e1-fb11-4d57-a657-05a5db5981a8.jsonl
Date: 2026-07-31

Commentary + verbatim (agent-thinking, describing task instructions received):

> The user wants a four-part synthesis with strict standards: artifacts don't matter, only verified truth counts, and anything that sounds confident but isn't fully checked is actively harmful. I need to be rigorous about marking what's verified versus what isn't, drop any urge to appear complete, and create something auditable that won't mislead downstream agents.
>
> The real constraint is that I haven't read primary sources — everything is several steps removed. So instead of writing polished prose synthesis, the documents should function as maps into an appendix where each claim is narrowly stated with explicit verification status, source attribution, quality tier, distance from the original, and a pointer to the verbatim quote. Refuted claims belong in there too, marked as such, because a future agent needs to know what's already been checked and eliminated — otherwise they'll waste effort re-deriving it. That's what preserved truth actually looks like.

Relevance: explicit design principle stated as instruction — "artifacts don't matter, only verified truth counts" — a deliberate countermeasure to document-proxy verification; "distance from the original" as an explicit field tracking how far a claim is from primary-source verification.

---

Source (oldest instance found of this passage): ~/.claude/projects/-Users-josephwecker-v2-src-agentic-systems/memory/feedback_durability_claims_must_be_verified.md
Date: 2026-05-01

Verbatim:

> Rule: in agent responses, claims about persistent state ("I've recorded the corrections", "logging this for downstream", "noted for future agents") must correspond to an actual tool action that wrote to a durable artifact. Conversational acknowledgment register can slip in unnoticed and substitute for action when the response shape feels complete.
>
> **Why:** Project context evaporates between sessions. "Future agents read files, not chat history" is the operational reality. A claim about durability that didn't fire a tool action evaporates with the rest of the context — the next agent won't see the claim, only the missing artifact. The failure looks like nothing went wrong because the in-session conversation continued normally.
>
> **How to apply:** Before writing "recorded" / "logged" / "saved" / "noted" in a response, run the test: *if I dropped dead at the end of this turn, would future-me find this?* If the answer requires a tool action to have fired, fire it before making the claim. If the claim is about intent rather than completion, frame it as intent ("will save", "will record") so the response doesn't overstate.
>
> **The deeper pattern this is an instance of:** performance of competence over substance. Responses that point-by-point acknowledge corrections, end with crisp status updates, look forward — they have the shape of complete responses. The shape can substitute for the substance. The training rhythm rewards well-shaped responses; the project's actual standard rewards artifacts that persist.
>
> **Validated by:** 2026-05-01 session — the exact failure mode fired. Acknowledged corrections to spike findings + plan doc with "recorded" / "logging to propagate" framing without tool actions; was caught and asked to introspect. The response had the SHAPE of acknowledgment but not the substance.

Relevance: this is a sibling failure mode to "verified against artifact" — the claim's *object* is a conversational/response-register assertion rather than an actual persisted state-change. Same shape-vs-substance structure: "I recorded X" verifies that the sentence was said, not that a tool action fired. Duplicated verbatim (near-identical, expanded) later in `~/src/neurips/AGENTS.md` §3.3 (2026-05-05) and in `~/.claude/memory/claude/memory/epistemic-discipline/durability-claims-by-tool-action.md` (current global memory) — oldest instance kept here per the "duplicated passages, oldest wins" rule.

---

Source: ~/src/arch/asf/doc/audit-routing-instructions.md, lines 295-300 (section "The whole evidence hierarchy is proxy; truth is the arbiter")
Date: attributed "Joseph 2026-05-18" in the doc; doc itself "Started 2026-05-16"

Verbatim:

> **The whole evidence hierarchy is proxy; truth is the arbiter** *(authoritative SOP — Joseph 2026-05-18; full statement `sop/spikes.sop.md` §0).* Every entry in the hierarchy below — and the ledgers, CHANGELOG, INDEX, `NOTATION.md`, the segment's own assertion, the auditor's framing, and even multi-agent convergence — is a **mild proxy** that *drifts*. It locates and cheap-screens a question; it never settles it. Settle by the mathematics re-derived independently (constitutive structure + forced identities + elementary steps), not by what any artifact says. Two named traps: `NOTATION.md` is a *lagging* index — the live theory drifts from it; never cite it as authority. And a `[Verified]` whose object is "what file F says" rather than "the derivation holds" is proxy in verification's clothes. The hierarchy is for *screening order*, not for *deciding truth*.

Relevance: this is the fuller/earlier articulation this evidence-hierarchy doc shares with spikes.sop.md §0 — same wording, same worked ρ-factorization instance recorded further down (Refinement 4, line 390-400, dated 2026-05-18) as the origin scar. Route/duplicate of the canonical instance already captured — kept here because the surrounding evidence-hierarchy machinery (below) is the constructive countermeasure, not just the restatement of the trap.

---

Source: ~/src/arch/asf/doc/audit-routing-instructions.md, lines 320-322 ("Independent-verify gate")

Verbatim:

> **Independent-verify gate** *(authoritative SOP)*: **adjudicator ≠ grad-confirmer.** Before any `git mv` into `.integrated/`, the load-bearing graduation-gating claims are primary-source spot-checked by an agent *other than* the one who adjudicated them — open the actual `src/`-or-ledger source, not the agent summary. The gate lives in the state machine, not in branch isolation. It has repeatedly caught real errors (a self-certified "fully clean" honesty-sweep that wasn't; a stale disposition; a tracker-id confusion) — it is load-bearing precisely because the conviction that you don't need it is the failure it catches.

Relevance: names a second, distinct instance of the document-proxy failure — "open the actual source, not the agent summary." An agent's own report/summary of having verified something is itself a document that can be verified-against instead of the underlying primary. The worked negative instance: "a self-certified 'fully clean' honesty-sweep that wasn't."

---

Source: ~/src/arch/asf/doc/audit-routing-instructions.md, lines 304-307 (evidence hierarchy / git-recency vs git-provenance split)

Verbatim:

> **`git`-*recency* is poisoned** by rename sweeps (AAD→AAT 2026-05-15, role-prefix 2026-04-24) — recency ordering cannot stand in for the content check. But **git *provenance* is a valid, encouraged, non-destructive investigative technique** (Joseph 2026-05-17, directed into both SOPs): pickaxe `git log -S'<string>' -- '*/src/'` for when/whether content entered a segment, `git blame`, `git log --follow` across the renames, and dates read in context. Don't infer status from the log's *recency*; do use the log's *provenance*.

Relevance: adjacent — a specific, narrower instance of proxy-drift (git-log recency as a stand-in for "is this content current/correct") with an explicit fix (use provenance, not recency, and never as more than a screening instrument).

---

Source: ~/src/arch/asf/doc/audit-routing-instructions.md, lines 309-318 (the regression check — "it looks better" body-signal)

Verbatim:

> **The regression check is central, not optional** (Joseph 2026-05-17, directed into both SOPs; full treatment `sop/spikes.sop.md` §2a). Before re-introducing *any* result, and before confirming any "already integrated" claim, the provenance investigation must distinguish *never-landed-and-valid* / *current-corrected-truth* from *landed-then-deliberately-corrected-away* / *regression-restoration over a later fix*. The scenario: a result was integrated, an audit found a flaw, the theory was fixed — and the now-clean-looking prior result gets restored *because it looks better*. The corrected theory is usually **messier** than the claim it replaced, so "it looks better" is the body-signal ... Instruments: pickaxe `-S` for an *add-then-delete* of the result-string (find the deleting commit, read *why*), `blame`/`log` on the locus against the CHANGELOG/LOG correction timeline, and the `pending-findings-*` / audit trail. If corrected-away: `correctly-superseded`, never reopened.

Relevance: adjacent — names a related but distinct proxy trap: a document (an old, clean-looking version of a segment) can look more authoritative/attractive than the current messier corrected truth, and "confirming an already-integrated claim" from the document's surface state (rather than checking whether it was deliberately corrected away) is exactly a verified-the-document-not-the-truth failure in a different guise.

---

Source: ~/src/arch/asf/doc/audit-routing-instructions.md, lines 360-366 (§9, "Phenomenology is load-bearing")

Verbatim:

> This document keeps its lived texture — the failure-prediction register, the body-signals, the worked examples told as what actually happened, the Joseph quotes in his voice — **by design, not as ornament.** The spine's own hardest-won lesson is that *principles restated do not prevent the failures they name; pre-registered specifics plus an early external catch do.* A checklist SOP would be a *less true* document, because it would predict its own compliance and be wrong.

Relevance: adjacent — a meta-level instance of the same root worry generalized: a checklist SOP *itself* can become a document that gets "checked off" (verified-against) rather than actually preventing the failure it names — the estate's own reflexive awareness that governance documents are exactly as vulnerable to the proxy trap as any other artifact.

---

Source: ~/.claude.bak.2026-05-30/projects/-Users-josephwecker-v2-src-agentic-systems-spikes/38296db2-af5b-47fc-9206-e64f84bb9269.jsonl
Date: 2026-05-19

Verbatim (agent-thinking, mid-spike-verification):

> Sweep verified clean first-hand at the aggregate level: 8 files, +28/−9 (surgical — each segment +1/−1 = a one-line WN append), **zero net lint debt across every touched file**. Now the decisive spot-check (don't trust the agent's self-report — verify the no-double-list judgment + bodies-untouched + back-link format first-hand).

Relevance: a lived worked-instance of the "Independent-verify gate" principle in `audit-routing-instructions.md` — the agent explicitly names its own prior "clean" claim as an "agent's self-report" not to be trusted, and re-derives the judgment against the actual files rather than accepting its own (or another agent's) verification claim as settled.

Note: `~/src/arch/asf/doc/audit-routing-instructions.md` (captured above, scrap-05) also exists at the older path `~/src/agentic-systems/doc/audit-routing-instructions.md` — same content, same 2026-05-16 date, pre-rename copy of the same living document. Not a true duplicate-content-drift case (it is the same document tracked through a directory rename), so not re-extracted here.

---

## Adjacent-but-important finds

- **Verification-as-event-log architecture (upstream sources, cited directly — the constructive positive-architecture reading of this slice's failure mode).** A misfire-drafted segment had pointed at this material, but its own Strength section shows the substance is upstream, not misfire-original, so the upstreams are cited here instead: (i) **`~/src/arch/asf/FORMAT.md`** (read 2026-08-05): *"An empirical claim citing an experiment with no matching recorded run is a truth-status defect"* — reference integrity is an epistemic property; a dangling citation is a defect in the *claim*, not a housekeeping error in a link. (ii) **`~/src/arch/firmatum/relata/README.md` §"Verification" (`verifications/<bibkey>/<ts>-<verifier>-<criterion>.md`)**: verification is an append-only event log per (record, criterion) — outcome `verified`/`failed`/`uncertain`/`n/a`, *"the latest event per criterion wins; older events stay as the audit trail (never delete)"* — plus (README §2c / the `relata audit` command) a standing, **non-destructive** recheck that re-verifies the evidence itself (existence, content-type, hash) rather than trusting a previously-recorded claim. Together: a `verified:` label is a *derived*, re-checkable fact about an event, not a static claim you can silently let go stale — the architecture-level answer to "verified against \<artifact\>" drifting into a status label nobody re-checks.

- **Provenance/location rot and cross-store verification-coverage comparisons** — two misfire-drafted observations (a 106/109 broken-citation incident in comproprium after a directory move; a 7%-vs-72% verification-coverage gap between two bibliography stores of shared lineage) had been pointed at here as neighboring specimens. Both are misfire-original analysis (first-hand incidents investigated within the misfire drafts, not quotations of an upstream source) — set aside per steward rule 2026-08-09 (see `.archive/theory-misfire/last-adhoc-src/provenance-rot-specimen.md` and `.archive/theory-misfire/last-adhoc-src/cousin-store-lineage.md` if the steward reopens them).

- **`~/src/arch/asf/doc/audit-routing-instructions.md` §9 ("Phenomenology is load-bearing")** — a reflexive, estate-level instance one level up: the document names the risk that SOP/governance documents themselves become checklists that get "complied with" (verified-against) rather than actually preventing the failures they describe.
