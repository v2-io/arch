# Evidence sweep — "proxy-discipline" as a named concept, and locates-vs-settles

*Slice of the estate-wide `claim-truth-over-proxy` evidence gather. Method: `memorata3-search` (several phrasings, `--sort oldest`) plus `rg --hidden -g '!.git*'` over `~/src`. Ordered oldest-primary-source first, per the "duplicated passages, oldest wins" rule — later restatements are kept because they show the lineage/porting chain, not because they're independently load-bearing.*

---

## 1. PRIMARY ORIGIN — `spikes.sop.md` §0 (earliest authoritative statement found; coins "proxy-discipline")

**Source:** `~/src/arch/asf/doc/sop/spikes.sop.md` (lines 21-38) — also present verbatim at `~/src/arch/firmatum/verisectorium/theory/influx/asf-sops/spikes.sop.md` (already staged as influx inside this project)
**Dated in-text:** "Joseph 2026-05-18"

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

Companion, same file/day, immediately follows §0 — the discharge counterweight (relevant because `claim-truth-over-proxy` will likely need to pair with it, as `spikes.sop.md` itself does):

> ### §0c. Honest incompleteness is a complete discharge — the counterweight to §0 (Joseph 2026-05-18)
>
> §0 and the gates, taken without this, drive a **verification-regress**: every gate spawns another, nothing is ever released, and an honest "not yet" feels like failure. It is not. **The gates exist to prevent false confidence, never to forbid honest incompleteness.**

---

## 2. Same-corpus restatement — `routing.sop.md` (cites spikes.sop.md as its source; adds "screening order, not deciding truth")

**Source:** `~/src/arch/asf/doc/sop/audit.sop/routing.sop.md` lines ~295-303 (also at `~/src/arch/firmatum/verisectorium/theory/influx/asf-sops/audit.sop/routing.sop.md`)

> **The whole evidence hierarchy is proxy; truth is the arbiter** *(authoritative SOP — Joseph 2026-05-18; full statement `sop/spikes.sop.md` §0).* Every entry in the hierarchy below — and the ledgers, CHANGELOG, INDEX, `NOTATION.md`, the segment's own assertion, the auditor's framing, and even multi-agent convergence — is a **mild proxy** that *drifts*. It locates and cheap-screens a question; it never settles it. Settle by the mathematics re-derived independently (constitutive structure + forced identities + elementary steps), not by what any artifact says. Two named traps: `NOTATION.md` is a *lagging* index — the live theory drifts from it; never cite it as authority. And a `[Verified]` whose object is "what file F says" rather than "the derivation holds" is proxy in verification's clothes. The hierarchy is for *screening order*, not for *deciding truth*.

The same file's dated changelog ("Refinement 4," 2026-05-18) retrospectively marks §0 as **supreme** over all the proxy-mechanics in the document, and "Refinement 5" (same day) as its counterweight — matching §0/§0c in spikes.sop.md:

> *Refinement 4 (2026-05-18, Joseph-directed — the foundational stance, made explicit and supreme over all the proxy-mechanics in this doc).* Care about the theory's **truth** above everything; provenance, ledgers, CHANGELOG, INDEX, `NOTATION.md`, segment/spike assertions, audit findings, and multi-agent convergence are *mild proxies that drift* — screening, never settling. Settle by re-derived mathematics. […] Canonical statement: `sop/spikes.sop.md` §0. This Refinement sits above Refinements 1–3: they are proxy-discipline; §0 is what proxy-discipline is *for*.

A subagent transcript (`~/.claude/projects/-Users-josephwecker-v2-src-arch/5b943f2c-ddd6-4a00-8e6d-c939a854eb7d/subagents/agent-a6698a2638d43f489.jsonl:42`, 2026-07-18) independently quotes this exact `routing.sop.md:295` passage and glosses it for a different (measurement-platform) context: *"don't let a label (even a consensus label) substitute for the underlying observation."* — evidence the principle has already migrated by citation into at least one other project's reasoning, not just by re-drafting.

---

## 3. The explicit porting chain into udon-theory, then into verisectorium (found by following in-file citations)

**Source:** `~/src/arch/firmatum/udon/v2/theory/FORMAT.md` §0 (lines 9-20), titled "Truth is the arbiter; everything else is a proxy" — file header states its own provenance:

> Ported from [`asf/FORMAT.md`](../../../arch/asf/FORMAT.md) and the two routing SOPs it depends on ([`audit.sop/routing.sop.md`](../../../arch/asf/doc/sop/audit.sop/routing.sop.md), [`spikes.sop.md`](../../../arch/asf/doc/sop/spikes.sop.md)), trimmed to what a foundation corpus needs. Where a rule reads differently from its source, the divergence is stated with its reason. **Nothing in this document has authority beyond being true.** A rule found false is corrected here, not obeyed.

§0 verbatim (condensed prose form of the same cut):

> ## 0. Truth is the arbiter; everything else is a proxy
>
> This governs every section below it.
>
> The job is to get the claims right. Provenance, git history, a CHANGELOG, an OUTLINE row, a spike's own framing, a segment's own assertion, an audit finding, agent consensus, even the convergence of several independent agents — **all of these are proxies for truth, and every one of them drifts.** They are useful for *locating* a question and for *cheap-screening* it. None of them settles it. A question is settled by re-deriving far enough to stand on constitutive structure, forced identities, and elementary steps — not on what any artifact says.
>
> Two recurring traps this names, both observed:
>
> - **An index is a lagging artifact.** "The outline says X" is not evidence that X holds; it is at most evidence about a document that may be stale. Cite it as location, never as authority.
> - **"Verified against \<artifact\>" is proxy wearing verification's clothes.** Marking a step verified because a document says so verifies the document. The tell: a verification whose object is *what file F asserts* rather than *the claim holds*.
>
> Every gate below is proxy-discipline — machinery for not being fooled by the cheaper proxies. When a gate is in tension with re-derived truth, truth wins and the gate is what gets corrected.

**Next link — the udon-theory corpus's citable segment form**, `~/src/arch/firmatum/udon/v2/theory/src/norm-proxy-discipline.udon` (whole file, 27 lines; `:type normative :status decided :stage draft`, `:from theory/FORMAT.md§0`; also under the pre-move path `~/src/MOVED/udon/v2/theory/src/norm-proxy-discipline.udon`, identical):

> |title Proxies locate; they do not settle
> |summary
>   Provenance, git history, changelogs, outline rows, spike framings, agent consensus, and convergence are useful for locating a question and cheap-screening it — none of them settles a claim.
> |formal-expression
>   **Norm (proxy discipline), decided for this corpus:**
>
>   1. Every artifact that is not a re-derived (or otherwise warranted) claim is a **proxy** for truth: outline rows, `:see` pointers, status labels assigned without opening a primary, multi-agent agreement, "the spike concluded," "verified against file F."
>   2. Proxies may **locate** work and **cheap-screen** it. They may not be cited as the warrant for a present-truth statement.
>   3. A claim is settled only by constitutive structure, forced identities, and elementary steps held on the page — or by an honest lower tier with a stated strengthen-action (FORMAT §0a), which is discharge, not settlement at a higher tier.
>   4. Two traps this norm names:
>      - **Index as authority** — "the outline says X" is evidence about a navigator document, not that X holds.
>      - **Verified-against-artifact** — checking that a file asserts $P$ verifies the file; it does not verify $P$.
>
>   This is a **decision** about how this corpus is written and audited, not a theorem about knowledge in general.
> |epistemic-status
>   **Status: `decided`.** Normative for udon-theory authoring; inherits FORMAT §0. Ceiling is `decided` (FORMAT §4: normative slots are not truth-apt). Overturn by steward if wrong — do not re-label as discussion-grade to avoid defending the norm, and do not dress the decision as a derivation.
> |discussion
>   Honest incompleteness is the companion discharge (FORMAT §0a): an honestly lower tier with Working Notes stating what would raise it, and remainder released as a bare gap or spike, is complete work. Proxy discipline without that discharge produces verification regress.
>
>   Agent-handover reading: "outline complete through Part IX" is a proxy if Part IX is unwritten slots. "Part I has N segments at stated statuses; two bare gaps; here are the slugs" is pointer-shaped reinjection.
> |working-notes
>   Keep FORMAT §0 as the full prose home; this segment is the citable kernel. If FORMAT and this segment diverge, that is a collision — fix one.

This title — **"Proxies locate; they do not settle"** — is the cleanest one-line phrasing of the locate/settle cut found anywhere in the estate.

---

## 4. [SET ASIDE per steward rule 2026-08-09] Verisectorium's own earlier draft under `.archive/theory-misfire/`

*Misfire-origin material set aside per steward rule 2026-08-09 (see `.archive/theory-misfire/last-adhoc-src/proxy-discipline.md` if the steward reopens it). Ruling: `.archive/theory-misfire/` material may not be used as a reference or quoted when the misfire file is the ORIGIN of the content; where it cites an upstream, cite that upstream directly.*

That draft self-identified as a restatement of §3's udon `norm-proxy-discipline.udon` ("shared authorship, deliberately not counted as a second arrival") — so for citation purposes the lineage **collapses cleanly to §3 above** as the citable source; nothing here supersedes it.

One piece of that draft's content was not a restatement but appeared misfire-original: a lived local incident description (a register-entry-as-integrated-proxy failure, tied to `[[integration-metabolism]]`) and a Working Note proposing a "disposition-verifier role." Per the ruling, that content is set aside rather than reported here — flagged only as *existing*, not quoted, pending the steward's decision on whether to reopen it.

**Withdrawn:** my earlier recommendation to "read the archived draft first" before drafting `claim-truth-over-proxy` is withdrawn. Draft from §1–§3 (the asf/udon upstream chain) directly.

The two adjacent files I'd flagged as "worth opening when drafting" (`adjudicator-not-confirmer.md`, `honest-incompleteness-discharge.md`) are reframed the same way: **steward-gated, not to-chase** — they sit in the same `.archive/theory-misfire/` cluster and are subject to the same ruling until the steward rules on them individually.

---

## 5. Current verisectorium statement (what the brief quoted; included for completeness of the chain)

**Source:** `~/src/arch/firmatum/verisectorium/README.md:7-10`, dated ~2026-08-05

> Two laws govern every disposition: **the delete-test** (nothing is integrated unless all its information is landed or truly disposable — assume it disappears) and **proxy-discipline** (a register, an index, or an agent's report *locates* truth; it never settles it).

---

## Full lineage summary (oldest → newest)

1. `asf/doc/sop/spikes.sop.md` §0 (2026-05-18) — coins "proxy-discipline," states locate/cheap-screen-vs-settle, pairs with §0c honest-incompleteness counterweight.
2. `asf/doc/sop/audit.sop/routing.sop.md` (same day/corpus) — restates as "the whole evidence hierarchy is proxy; truth is the arbiter," adds "screening order, not deciding truth."
3. `udon/v2/theory/FORMAT.md` §0 — explicit port, condensed prose, retitled "Truth is the arbiter; everything else is a proxy."
4. `udon/v2/theory/src/norm-proxy-discipline.udon` — udon corpus's citable-segment form, titled "Proxies locate; they do not settle," `:from theory/FORMAT.md§0`. **This is the last citable link** — the next step in the actual on-disk history (verisectorium's own earlier draft under `.archive/theory-misfire/`) is set aside per the 2026-08-09 steward ruling on misfire-origin material (see §4 note above), since it self-identified as a restatement of *this* file rather than adding new citable content.
5. `verisectorium/README.md:7-10` (current) — terse restatement as one of the project's two governing laws.

**"Proxy-discipline" is not novel to verisectorium** — it is a six-month-old, twice-ported piece of ASF methodology. Citable upstream for drafting `claim-truth-over-proxy`: §1–§3 above (asf → udon). The corpus's *prior attempt at this exact segment* exists on disk under `.archive/theory-misfire/` but is steward-gated as of 2026-08-09 and should not be read as a drafting reference unless the steward reopens it.

---

## Adjacent-but-important finds (marked, not fully chased)

- The subagent citation of `routing.sop.md:295` in a *different* project's context (`~/.claude/projects/-Users-josephwecker-v2-src-arch/.../agent-a6698a2638d43f489.jsonl`, 2026-07-18) shows the principle already migrating estate-wide by reasoning-citation, not only by re-drafting — possibly relevant if the new segment wants an example of proxy-discipline being applied outside its birth-corpus.
- **Steward-gated, not to-chase:** `~/src/arch/firmatum/verisectorium/.archive/theory-misfire/last-adhoc-src/{adjudicator-not-confirmer,honest-incompleteness-discharge,warrant-over-authority,rule-grounds-and-posture,epistemic-axes,proxy-discipline}.md` all live in the same archived cluster. Per the 2026-08-09 steward ruling, misfire-origin content in this directory is not to be used as a reference or quoted; where any of these cite an upstream, cite that upstream directly instead. Not opened further in this sweep — noted as existing, not as a drafting source, pending steward disposition.
- No pre-2026-05-18 origin was found anywhere in the estate (including the oldest sapientia/zi-am-tur corpus from Sept 2025) — the "truthification"/"meaningful vs true" vocabulary from that era is a different, earlier concept (general epistemic humility about LLM generation) and does **not** use "proxy" or locate/settle language; it reads as a plausible philosophical ancestor in spirit but not in vocabulary. Worth a note if the new segment wants to credit that older lineage, but it should not be cited as the source of "proxy-discipline" specifically.
- `~/src/arch/firmatum/utils/md-press/fixtures/asf-history/{before,after}/{audit-routing-instructions.md,spike-routing.md}` are test fixtures containing copies of the same text at two points in time — useful only as a diff-artifact if someone wants to see exactly how the wording evolved, not independent evidence.

If any part of this brief's framing seems off: the request specifically asked for "proxy-discipline as a named concept, and locates-vs-settles" — I found the concept is thoroughly cross-corpus already-canonical rather than something needing fresh grounding, and the single highest-value find is arguably not a *new* piece of evidence but the fact that verisectorium already tried and archived its own version of this exact segment. Worth flagging to whoever integrates this slice with the other nine.
