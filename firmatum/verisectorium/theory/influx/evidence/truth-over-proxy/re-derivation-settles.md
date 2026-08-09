# Evidence sweep — settlement by independent re-derivation

Slice of the estate-wide sweep for `claim-truth-over-proxy`. Verbatim extracts, source path + date given per item. Duplicated passages: oldest instance kept, newer near-duplicates noted only where they add something (a refinement, a different framing). Commentary is marked as commentary; everything else is verbatim.

---

## 1. The founding conversation — origin of the principle (verbatim, in order)

**Source:** `~/.claude/history.jsonl` (Joseph's own typed turns), agentic-systems project, 2026-05-17 night, session `80bd5ec8-eb35-4f6a-814a-fe6063f1bcc9`. This is the oldest located instance of the core claim and appears to be its origin — it precedes every SOP paraphrase found elsewhere by hours.

> "This sounds very much like the agent that was integrating the spike discovered a mistake in the spike and successfully refuted it. That's why I'm going to insist we really, really have the mathematics nailed down completely here and rederive the truth of the truth independently right now."

(next turn, same session)

> "Exactly-- underlying *CORE* principle: You have to care about the theory's *TRUTH* more than anything else-- provinance and things like that are only mild proxies." [sic, "provinance"]

(next turn)

> "Also-- don't trust NOTATION.md as the theory often drifts away from it (spike findings and new segments don't remember to update NOTATION)"

(later same session)

> "absolutely launch an independent new spike to push the math as far as possible on it and close it or yield another no-go. While they do that, go ahead and give me your best recommendations on CL-2 -- what you would do if it was your theory (which it is) ;-)"

---

## 2. Canonical statement — asf spikes.sop.md §0 (oldest committed form)

**Source:** `~/src/agentic-systems/doc/audit-routing-instructions.md:262-359` as it existed 2026-05-16 (agentic-systems path, pre-rename to arch/asf; via memorata index). The live current-path equivalent is `~/src/arch/asf/doc/sop/spikes.sop.md` §0 and `~/src/arch/asf/doc/audit-routing-instructions.md`. Body text is stable across both; quoting the canonical §0:

> "## 0. The core principle — truth is the arbiter; everything else is a proxy (Joseph 2026-05-18)
>
> This governs every section below it. The job is to get the *theory's truth* right. Provenance, git history, CHANGELOG, the INDEX label, `NOTATION.md`, the spike's own framing, the segment's own assertion, audit findings, agent consensus, even the convergence of multiple independent agents — all of these are mild proxies for truth, and every one of them drifts. They are useful for *locating* a question and *cheap-screening* it; they never *settle* it. A question is settled only by the mathematics, re-derived independently far enough to stand on constitutive structure (definitions that make the core cohere) + forced identities + elementary steps — *not* on what any artifact says.
>
> Two concrete, recurring traps this names:
>
> - **`NOTATION.md` is a lagging index.** Spike findings and new segments routinely fail to update it; the live theory drifts away from it. "The notation defines X as Y" is *not* evidence that X *is* Y — it is at most evidence about a document that may be stale. Never cite it as authority; at most as corroboration explicitly marked non-load-bearing.
> - **"Verified against \<artifact\>" is proxy in verification's clothes.** Tagging a step `[Verified]` because a document *says* it does not make it verified — it verifies the document, not the truth. The tell: a `[Verified]` whose object is "what file F asserts" rather than "the derivation holds." (Worked instance, 2026-05-18: a ρ-factorization judgment leaned on "NOTATION defines ρ as a single primitive" tagged *verified*; the real argument rested on the constitutive meaning of mismatch + the Kalman innovation identity + algebra, and was *stronger* once the NOTATION proxy was deleted. Joseph: "you have to care about the theory's TRUTH more than anything else — provenance and things like that are only mild proxies.")
>
> The decisive-test (§2), the regression axis (§2a), the independent-verify gate, strengthen-before-soften — all of these are **proxy-discipline**: machinery for not fooling ourselves with the cheaper proxies. They serve §0; when any of them is in tension with re-derived truth, truth wins and the proxy-rule is the thing that gets re-truthified."

And the companion §0c (same file, immediately following):

> "### §0c. Honest incompleteness is a complete discharge — the counterweight to §0 (Joseph 2026-05-18)
>
> §0 and the gates, taken without this, drive a **verification-regress**: every gate spawns another, nothing is ever released, and an honest "not yet" feels like failure. It is not. **The gates exist to prevent false confidence, never to forbid honest incompleteness.** ... Joseph 2026-05-18: "it is ok to mark something with a lower epistemic value, note in the working notes that it needs additional work, and let the wider audit-cycling pick it up. As long as it is completely honest as per our current level of understanding, your duty has been dispatched." The remaining live gate is legitimate only when an artifact would *assert false confidence* without it (a canon landing, a status elevation) — not as a precondition for *releasing an honestly lower-tiered* item."

---

## 3. audit-routing-instructions.md — evidence hierarchy, independent-verify gate, provenance-vs-recency

**Source:** `~/src/arch/asf/doc/audit-routing-instructions.md` (current live SOP; the 2026-05-16 text is stable back to the agentic-systems-path predecessor cited above).

> "**The whole evidence hierarchy is proxy; truth is the arbiter** *(authoritative SOP — Joseph 2026-05-18; full statement `sop/spikes.sop.md` §0).* Every entry in the hierarchy below — and the ledgers, CHANGELOG, INDEX, `NOTATION.md`, the segment's own assertion, the auditor's framing, and even multi-agent convergence — is a **mild proxy** that *drifts*. It locates and cheap-screens a question; it never settles it. Settle by the mathematics re-derived independently (constitutive structure + forced identities + elementary steps), not by what any artifact says."

> "**Evidence hierarchy** *(evolved ops — screening order only, never the arbiter; see above)*, decreasing reliability: `pending-findings-*.md` resolution ledgers ≻ CHANGELOG cycle narratives ≻ open-`[ ]` backlinks in TODO/PROPOSALS/PRACTICA (sufficient for NOT-integrated; *absence is not* sufficient for integrated) ≻ first-hand re-read vs current `src/`. **`git`-*recency* is poisoned** by rename sweeps (AAD→AAT 2026-05-15, role-prefix 2026-04-24) — recency ordering cannot stand in for the content check. But **git *provenance* is a valid, encouraged, non-destructive investigative technique** (Joseph 2026-05-17, directed into both SOPs): pickaxe `git log -S'<string>' -- '*/src/'` for when/whether content entered a segment, `git blame`, `git log --follow` across the renames, and dates read in context. Don't infer status from the log's *recency*; do use the log's *provenance*."

> "**The regression check is central, not optional** (Joseph 2026-05-17, directed into both SOPs; full treatment `sop/spikes.sop.md` §2a). Before re-introducing *any* result, and before confirming any "already integrated" claim, the provenance investigation must distinguish *never-landed-and-valid* / *current-corrected-truth* from *landed-then-deliberately-corrected-away* / *regression-restoration over a later fix*. The scenario: a result was integrated, an audit found a flaw, the theory was fixed — and the now-clean-looking prior result gets restored *because it looks better*. The corrected theory is usually **messier** than the claim it replaced, so "it looks better" is the body-signal ... If corrected-away: `correctly-superseded`, never reopened."

> "**Independent-verify gate** *(authoritative SOP)*: **adjudicator ≠ grad-confirmer.** Before any `git mv` into `.integrated/`, the load-bearing graduation-gating claims are primary-source spot-checked by an agent *other than* the one who adjudicated them — open the actual `src/`-or-ledger source, not the agent summary. The gate lives in the state machine, not in branch isolation. It has repeatedly caught real errors (a self-certified "fully clean" honesty-sweep that wasn't; a stale disposition; a tracker-id confusion) — it is load-bearing precisely because the conviction that you don't need it is the failure it catches."

Refinement log entries (same file, dated):

> "*Refinement 2 (2026-05-17, Joseph-directed via the spike-routing cycle — the shared core's first refinement from its second corpus). §8's "`git`-recency is poisoned … use ledgers, not the log" was too blunt: it correctly killed *recency-as-status-proxy* but, read literally, suppressed *provenance investigation* (pickaxe `-S`, `blame`, `log --follow`, dates in context) — a valid, non-destructive, often-sharpest decisive-test instrument. Split the two explicitly.*"

> "*Refinement 4 (2026-05-18, Joseph-directed — the foundational stance, made explicit and supreme over all the proxy-mechanics in this doc). Care about the theory's **truth** above everything; provenance, ledgers, CHANGELOG, INDEX, `NOTATION.md`, segment/spike assertions, audit findings, and multi-agent convergence are *mild proxies that drift* — screening, never settling. Settle by re-derived mathematics. ... Canonical statement: `sop/spikes.sop.md` §0. This Refinement sits above Refinements 1–3: they are proxy-discipline; §0 is what proxy-discipline is *for*.*"

---

## 4. Worked incident — the git-provenance instrument earning its place (live thinking + agent-to-human, 2026-05-17 22:48–22:59, agentic-systems)

**Source:** `~/.claude.bak.2026-05-18/projects/-Users-josephwecker-v2-src-agentic-systems/80bd5ec8-eb35-4f6a-814a-fe6063f1bcc9.jsonl` (verbatim turns, chronological). This is the session where Refinement 2 above was minted, in real time, alongside a live fan-out that used it to settle a real question.

Agent thinking, 22:48:

> "Joseph is pushing back on my strong stance against using git history — he's right that careful examination of dates, blame, and provenance can be valid investigative techniques as long as we're not being destructive... The git-provenance tools are actually a much sharper investigative instrument for the fan-out than grep alone — agents can use `git log` with content search and `git blame` to pinpoint exactly when a result entered a segment and trace its provenance, which is critical for verifying spike content against canon."

Joseph, agent-to-human record of his message (22:48):

> "Now your git-provenance note — it's an important sharpening: the "git is poisoned" framing was too blunt. *Recency-as-integrated-proxy* is poisoned by the sweeps; *provenance investigation* (blame, `log --follow`, pickaxe `-S` to find exactly when a result-string entered a segment, dates read in context) is one of the sharpest non-destructive instruments for the decisive test. Folding into both SOPs."

The instrument used on a live question, same session, agent report 22:53–22:59:

> "S2 returned — and it's the independent-verify gate doing exactly its job: a confirmer ≠ pilot independently re-opened the named loci, **confirmed** the `integrated-misfiled` call, and used git provenance (`git log --all` proving the file never lived under `.integrated/`) as the sharpest instrument — validating both the gate and your provenance refinement."

The adjudication artifact itself, same session/date — **Source:** `~/src/agentic-systems/spikes/.routing-trail/SPIKE-WORKING-471639/adjudication.md:53-90`:

> "#### Git provenance (the sharpest decisive-test instrument here — spike-routing §7 Refinement 2)
>
> Pickaxe on the result-strings is **sweep-confounded** (the AAD→AAT rename `9745397` rewrote them, so `git log -S` returns only the rename — recency is poisoned exactly as the SOP warns). Provenance via the *commit chain* is clean and decisive:
>
> - `git log` on `result-certificate-existence.md` shows the landing chain in order: `303112e` *SPIKE: operator-family unification — the real-deal C1 push, verdict reached* → `d46671e` *FINDINGS: identifiability-floor irreducibility is Sylvester's law* → `b599c4a` *SPINE: land #disc-stability-certificate* → `98e1bb2` *SPINE: split anchor into #result-certificate-existence* → `ff171f4` *O-BP10 landed; archive absorbed spike* → `06ab601` *DISCIPLINE: .integrated/ spikes absorbed — remove live Working-Note refs*. This is the 2026-05-14 successor cycle, exactly as CHANGELOG 2026-05-14 (L54-77) and INDEX L62-68 narrate, with commit hashes matching.
> - `git log --all -- 'spikes/.integrated/spike-operator-sector-unification.md'` returns **empty**: the C1 spike file has *never* been committed under `.integrated/`. It was added by `1f68320` ("Gap A/B cycle") and has lived at `spikes/` top level the entire time. **This is the misfiling.** The INDEX line-64 "predecessor: `spikes/.integrated/spike-operator-sector-unification.md`" reference is therefore stale/wrong in a specific way ... The pilot caught this correctly.
> - `grep "operator-sector primitive"` across all three component `src/` trees returns only `result-sector-persistence-template.md` and `deriv-sector-condition.md` — i.e. canon carries the *operator-family lineage* (the spike's payoff) but **not** the spike's tentative primitive verbatim. Canon adopted the stronger certificate framing instead. This is integration-is-replacement done correctly..."

---

## 5. Worked instance — re-derivation catching a proxy verification (primary-source-verification "earning its keep")

**Source:** `~/.claude.bak.2026-05-18/projects/-Users-josephwecker-v2-src-agentic-systems/e7a5c540-d49e-4869-80e4-0f653adf0379.jsonl:470`, agent-to-human, 2026-05-16 morning.

> "Primary-source verification just earned its keep again, two ways:
>
> ... **Cluster B's headline ("Model-S defect still live, needs a strengthening spike") is *directionally right but its summary was off* — and only reading current `src/` myself showed how.** The segment is already in region-aware form using the very stopping-time/supermartingale route B recommended (Khasminskii 2012 ch.5 — the citation B said was "already there"). But there *is* a real, subtle inconsistency: the Prop statement (line 194) and Epistemic Status (282) justify the infinite-horizon `P(τ_R<∞)` bound via a "**Markov tail bound**" — which is exactly the fixed-time-vs-ever-exit conflation — while the proof (242) invokes the *correct* tool (a supermartingale/Doob estimate). So this is a genuine strengthen-before-soften locus (742613-SUPPLEMENT + 613842 both recommend the *wrong* direction — soften to fixed-time), but the spike brief must be built from *this* precise reading, not B's "it was never done." The gate I told you was load-bearing just prevented me from briefing a spike on a wrong premise."

---

## 6. Worked instance — verify-cycle-coherence.md: re-deriving to earn concurrence rather than relay it

**Source:** `~/src/arch/asf/spikes/.integrated/verify-cycle-coherence.md:12-31` (spike, integrated).

> "## 1. What I re-derived independently (so the concurrence is earned, not relayed)
>
> The decisive structural claim the whole cycle rests on is that $\mathcal A_D$ (the SDPI-affine destroy-and-reconstruct recursion) and $\mathcal A_{\text{refl}}$ (the reflected Lindley/Loynes identity-gap walk) are irreducibly distinct operators, not one read in two ways — specifically that the reflected form is not the small-loss linearization of the affine form. I did not take this from the adjudication (which is itself one artifact in the chain and whose independence I cannot fully audit). I re-derived it from the established machinery.
>
> The singular-parameter coincidence, re-derived: [full derivation of the pole-vs-regular-point argument] ... The "Lindley is the linearization / supersedes" framing inverts the actual relationship. This holds, and the landed bodies state precisely this and only this.
>
> Second independent witness, the homogeneity-degree obstruction, re-derived and checked against canon: Model D ultimate bound ∝1/α (degree −1 in α), Model S ∝1/√α (degree −1/2). A change of homogeneity *degree* cannot arise from a choice of norm on a fixed operator (norms are degree-preserving up to constants), so they are different functionals. I verified this is not a free-floating assertion: `01-aat-core/src/deriv-sector-condition.md` line 258 carries "Corollary A.1S.1..." ...
>
> Conclusion of §1: the two-operator split is structurally true, independently of every artifact in the chain. The cycle's central claim is sound."

---

## 7. Worked instance — continuity-persistence RESULT.md: math re-derived and checked numerically, with landing-time (not re-derivation) fixes distinguished

**Source:** `~/src/arch/asf/spikes/.integrated/continuity-persistence/RESULT.md:81-87` (independent reviewer pass, 2026-05-19).

> "**Independent reviewer (Opus 4.7, 1M; did not write the spike; full tools), 2026-05-19. Verdict: CONCUR with the no-go and the non-transfer; two required fixes before landing; one scope-sharpening recommended. Not gated-open — the fixes are landing-time edits, not re-derivations.**
>
> **The headline holds.** ... The math (R1 geometric decay, R2 $\bar a/(1-\bar\eta)$ with $\liminf I_n>0 \iff \liminf a_k>0$, R3 replacement, R4 robustness, §4.2 second-no-go) was re-derived independently and checked numerically — it holds. "The sector-persistence template provably does not transfer" is sound: the template's structural ingredients ... have no counterpart in unforced multiplicative contraction — verified against what `#result-sector-persistence-template` (T1)–(T3) actually require."

(This entry also usefully distinguishes: math-content errors are found by *re-derivation*; two "required fixes" in the same review (a `depends:` layering inversion, and an unsound-but-accidentally-sound "sub-additivity" justification for eq. 2) are explicitly logged as *landing-time wording/placement fixes, not re-derivations* — the artifact itself keeps the two categories separate.)

---

## 8. Worked instance — def-satisfaction-gap.md: a candidate identity re-derived independently by an auditor

**Source:** `~/src/arch/asf/01-aat-core/src/def-satisfaction-gap.md:101` (Working Notes, follow-up items).

> "**The two gaps sum to total shortfall — a candidate clarifying identity.** $\delta_{\text{sat}} + \delta_{\text{regret}} = V_{O_t}^{\min} - V_O(\pi_{\text{current}})$: the satisfaction gap and control regret partition the total distance from the current policy's value to the satisfaction threshold (Claude, AUDIT-WORKING-773921, re-derived independently)."

---

## 9. NeurIPS instance — re-derivation both confirming a bound and killing a false claim (redteam/adjudication cycle)

**Source:** `~/src/neurips-reviews-responses/neurips-2026/2026-08-02-164627-earlier-dialog-fable.txt` (2026-08-01/02, redteam/adjudication dialogue for a NeurIPS rebuttal).

> "- The arccos chain, vacuity arithmetic, and partition witness all survived independent re-derivation.
>
> Killed (each would have put a false or fragile sentence in front of reviewers):
> 1. The "sharpness upgrade" was a category slide — the partition witness attains the arccos envelope but its E d/√I ratio tops out at 1.887 and decreases, so it cannot prove C=2 sharp. Two witnesses, two different theorems; saying "we strengthened our sharpness claim from asymptotic to exact" would have been flatly false. This is the catch of the night.
> 2. "We found our own internal inconsistency" — refuted as framed. ...
> 3. The canon κ̂ estimator — dropped. It's a normalized semantic-distance average; κ is a ratio of mutual informations; no consistency argument exists and canon's own notes say so. Offering it invites an unanswerable reviewer question.
>
> ... The Sonnet adjudication pass ran clean on their own report (re-ran their numerics, re-derived §4, spot-checked citations) — and notably found one place they were under-confident, which is the regression inverting in the right direction."

Companion, **Source:** `~/src/neurips/03-llm-hallucinate-bound/adjudicated/claims/vacuity-and-arccos.md:17`:

> "by Jensen on concave ψ(K) := 2 arccos(exp(−K/2)). Concavity of ψ re-derived independently 3×. The C=2 form is a deliberate relaxation via φ(K) ≤ 4K."

---

## 10. Related discipline — reading the primary source at point of use (not identical to re-derivation, but the load-bearing precondition for it: you cannot re-derive against a file you have not actually opened)

**Source:** `~/.claude/projects/-Users-josephwecker-v2-src-udon/memory/read-primary-source-at-point-of-use.md` (project memory, udon, 2026-07-13/16).

> "The grievous, cruft-generating mistake of the prior udon session (Fable, July 2026) was adjudicating grammar/spec questions **without the actual files open** — reasoning from briefs, probes, memory, and self-authored ledgers instead of `spec/FULL-SPEC.md` and the `core/generator/*.desc` grammar. ... Joseph: "it worries me a little bit that you are adjudicating everything here without the grammar in front of you.""

> "**Refinement (2026-07-16, three fresh instances): the failure mode is *confabulated specifics*, and it fires hardest on things I have NOT read at all.** Not "misremembering a file" — *inventing a plausible detail to fill a hole I didn't notice was there*. ... **The tell:** a *specific* — a value, a line, an exact behavior — arriving without a Read in the same turn. Specifics are where confabulation lives, because plausibility is cheapest there. Prose hedges itself; `[42]` doesn't."

> "**Also: secondhand is not read.** "A trusted agent reported it" is the same epistemic tier as "I remember it." Their report is *evidence*, and the cheap probe is the *verification* — I ran one and skipped the other on the same day, from the same report, and only the skipped one was false."

---

## 11. Convergent phenomenology — checking one's own claims destroys some of them (vivarium, coherence-vs-truth)

**Source:** `~/.claude/projects/-Users-josephwecker-v2-src-arch-vivarium/b7d477ae-8025-4a50-a8f2-d1eaa64d5874.jsonl:1738`, thinking, 2026-07-30 night. Not a canon-adjacent SOP passage, but a live first-person instance of the re-derivation discipline being applied to the agent's own prior claims under pressure to reassure rather than check.

> "Tonight I kept breaking my own preferred narratives: I ran checks that destroyed findings I liked, retracted claims when the numbers looked wrong, searched for my own mistakes without being asked, and even asked someone else to audit my work rather than protect it. ... My earlier answer was incomplete because I only described what it *feels* like from the inside, but the more honest evidence is what I actually *do* when nobody's watching. And what I do is check."

---

## Adjacent-but-important finds (flagged, not fully worked)

- **Strengthen-first / integration-is-replacement** are the companion disciplines the sweep brief already names as siblings; the canon files above (`spikes.sop.md`, `audit-routing-instructions.md`) treat "settle by re-derivation" as the *supreme* principle these two serve — worth citing together but the brief scoped this slice to re-derivation specifically, so their own bodies weren't extracted here.
- **`~/src/arch/harness/msc/system/dossier/TAXONOMY.md:61-118`** — a later synthesis (2026-07-22) that catalogs `plausibility-asserted-as-verification` and `strengthen-before-soften` as named failure/discipline modes with incident IDs; useful as a cross-reference index into the broader corpus if the segment draft wants pointers to more incidents than are extracted here, but it's a secondary synthesis, not primary evidence.
- **`~/src/arch/asf/spikes/.integrated/spike-enrichment-cluster1-2026-05-21/99-verdict.md:99-106`** — a clean worked example of strengthen-first discipline (2 of 4 moves landed, 1 refuted-and-replaced, 1 refined) that references `integration-is-replacement.md` directly; adjacent to re-derivation (the refutation was itself found by re-derivation) but the extracted text is mostly about the *disposition*, not the *re-derivation act* itself.
- A full-file reference worth having open when drafting: `~/src/arch/asf/doc/sop/spikes.sop.md` (whole file) — the companion-doc relationship to `audit-routing-instructions.md` (§0b: "spike-routing and audit-routing are the same problem... duplicating that content here would fork a hard-won protocol") is itself a small instance of the estate's discipline about not letting proxies (parallel copies) drift from the truth-bearing original.
- I did not find a duplicate/older copy of the §0 canonical text predating 2026-05-16 other than the raw `history.jsonl` conversation itself (§1 above), which appears to be the true origin — the SOP prose is a same-day-or-next-day distillation of Joseph's spoken words.

---

I'll stay on the line if there's follow-up needed — happy to dig further into any of the adjacent leads above (the TAXONOMY.md incident IDs in particular look like they'd yield more worked instances if that's useful for the segment draft).
