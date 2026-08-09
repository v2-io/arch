# Evidence sweep: "proxies for truth" idea family

Sweep run 2026-08-09 for the `claim-truth-over-proxy` segment draft. Canonical statement is asf `doc/sop/spikes.sop.md` §0, confirmed via `git log -S` to have been introduced by commit `b4742de` on 2026-05-18 ("Spike-routing §0: truth is the arbiter; all artifacts (incl. NOTATION) are drifting proxies"). No genuine pre-2026-05-18 precursor was found in the memorata index or via ripgrep across `~/src/`; all hits found are either the canonical statement itself, verbatim/near-verbatim copies of it (asf/agentic-systems/fmt-md fixture snapshots), or later independent restatements in sibling projects (udon, verisectorium's own archived prior attempt). This is itself evidence worth reporting to the drafter: the idea appears to originate cleanly at this one point rather than accreting from older material — contrary to the brief's expectation that it likely predates its canonical statement.

## Source: `~/src/udon/v2/theory/FORMAT.md` §0
Date (git-blame-min): 2026-07-29T19:53:37-06:00 (LATER than the canonical asf `spikes.sop.md` §0, which is dated 2026-05-18 per the brief — so this is a restatement/parallel articulation in a sibling project, not a precursor. Worth keeping as a close-echo instance in a different project's own words.)

Duplicate copies found (same text, later dates — oldest wins, this file is the oldest occurrence found):
- `~/src/MOVED/udon/v2/theory/FORMAT.md` (2026-07-29T19:53:37-06:00, same date — likely the MOVED relocation of the same repo)
- `~/src/arch/firmatum/verisectorium/.archive/theory-misfire/.archive/udon-theory/theory-format.md` (2026-08-05T22:02:32-06:00 — a later copy already inside verisectorium's own archive)

Verbatim text:

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

Commentary (mine): this is essentially a sibling-project rephrasing of the same idea family as asf `spikes.sop.md` §0, independently titled ("Truth is the arbiter; everything else is a proxy") — worth flagging to the drafter as convergent evidence the idea is estate-wide, not localized to asf. Already partially imported into verisectorium's own `.archive` (theory-misfire), meaning this idea may have already influenced verisectorium theory once before under a different framing pass.

---

## Source: `~/src/arch/firmatum/udon/v2/theory/src/norm-proxy-discipline.udon`

Upstream source, verified to exist and read directly (per steward ruling 2026-08-09: verisectorium's own `.archive/theory-misfire/last-adhoc-src/proxy-discipline.md` explicitly names this `.udon` segment as what it restates — "The norm restates, in this corpus's voice, the udon theory corpus's `norm-proxy-discipline`" — so this upstream is quoted directly instead of the misfire file; the misfire file's own local enforcement-incident content is not upstream-sourced and is set aside below).

Verbatim:

```
|segment[norm-proxy-discipline] :type normative :status decided :stage draft
  :max-attainable decided
  :depends [form-present-truth-collision norm-gap-as-discontinuity]
  :from theory/FORMAT.md§0

  |title Proxies locate; they do not settle
  |summary
    Provenance, git history, changelogs, outline rows, spike framings, agent consensus, and convergence are useful for locating a question and cheap-screening it — none of them settles a claim.
  |formal-expression
    **Norm (proxy discipline), decided for this corpus:**

    1. Every artifact that is not a re-derived (or otherwise warranted) claim is a **proxy** for truth: outline rows, `:see` pointers, status labels assigned without opening a primary, multi-agent agreement, "the spike concluded," "verified against file F."
    2. Proxies may **locate** work and **cheap-screen** it. They may not be cited as the warrant for a present-truth statement.
    3. A claim is settled only by constitutive structure, forced identities, and elementary steps held on the page — or by an honest lower tier with a stated strengthen-action (FORMAT §0a), which is discharge, not settlement at a higher tier.
    4. Two traps this norm names:
       - **Index as authority** — "the outline says X" is evidence about a navigator document, not that X holds.
       - **Verified-against-artifact** — checking that a file asserts $P$ verifies the file; it does not verify $P$.

    This is a **decision** about how this corpus is written and audited, not a theorem about knowledge in general.
  |epistemic-status
    **Status: `decided`.** Normative for udon-theory authoring; inherits FORMAT §0. Ceiling is `decided` (FORMAT §4: normative slots are not truth-apt). Overturn by steward if wrong — do not re-label as discussion-grade to avoid defending the norm, and do not dress the decision as a derivation.
  |discussion
    Honest incompleteness is the companion discharge (FORMAT §0a): an honestly lower tier with Working Notes stating what would raise it, and remainder released as a bare gap or spike, is complete work. Proxy discipline without that discharge produces verification regress.

    Agent-handover reading: "outline complete through Part IX" is a proxy if Part IX is unwritten slots. "Part I has N segments at stated statuses; two bare gaps; here are the slugs" is pointer-shaped reinjection.
  |working-notes
    Keep FORMAT §0 as the full prose home; this segment is the citable kernel. If FORMAT and this segment diverge, that is a collision — fix one.
```

Set aside: the misfire file's own enforcement-record paragraph and Working Notes (the "delete-test failed the whole batch" local incident, the "disposition-verifier role" idea, the Relata cross-ref) were original to `proxy-discipline.md`, not restated from this upstream — misfire-origin material set aside per steward rule 2026-08-09 (see `.archive/theory-misfire/last-adhoc-src/proxy-discipline.md` if the steward reopens it).

---

## Source: `~/src/arch/asf/doc/sop/audit.sop/routing.sop.md` (also mirrored at `~/src/arch/firmatum/verisectorium/theory/influx/asf-sops/audit.sop/routing.sop.md`)

This is not an independent articulation — it explicitly cites `sop/spikes.sop.md` §0 as canonical and restates it inline, dated to the same 2026-05-18 event. Included because it adds dated "Refinement" framing showing HOW the idea entered this second SOP corpus, and because it shows the idea being treated as supreme/foundational ("Refinement 4 ... the foundational stance, made explicit and supreme over all the proxy-mechanics in this doc").

Verbatim (lines ~303-308):

> **The whole evidence hierarchy is proxy; truth is the arbiter**
> *(authoritative SOP — Joseph 2026-05-18; full statement `sop/spikes.sop.md` §0).* Every entry in the hierarchy below — and the ledgers, CHANGELOG, INDEX, `NOTATION.md`, the segment's own assertion, the auditor's framing, and even multi-agent convergence — is a **mild proxy** that *drifts*. It locates and cheap-screens a question; it never settles it. Settle by the mathematics re-derived independently (constitutive structure + forced identities + elementary steps), not by what any artifact says. Two named traps: `NOTATION.md` is a *lagging* index — the live theory drifts from it; never cite it as authority. And a `[Verified]` whose object is "what file F says" rather than "the derivation holds" is proxy in verification's clothes. The hierarchy is for *screening order*, not for *deciding truth*.

Verbatim, "Refinement 4" (dated inline 2026-05-18, lines ~398-408) — this is the passage explicitly marking the idea as supreme over the doc's other mechanics, and gives the worked incident (ρ-factorization / NOTATION.md) also seen in spikes.sop.md:

> *Refinement 4 (2026-05-18, Joseph-directed — the foundational stance, made explicit and supreme over all the proxy-mechanics in this doc). Care about the theory's **truth** above everything; provenance, ledgers, CHANGELOG, INDEX, `NOTATION.md`, segment/spike assertions, audit findings, and multi-agent convergence are *mild proxies that drift* — screening, never settling. Settle by re-derived mathematics. Two named traps folded into the evidence-hierarchy preamble above: `NOTATION.md` lags the live theory (never authority); a `[Verified]` tag whose object is "what a file says" is proxy wearing verification's clothes. Caught on a live lead-agent slip (a $\rho$-factorization judgment that cited NOTATION as a verified pillar — the real argument was stronger without it). Canonical statement: `sop/spikes.sop.md` §0. This Refinement sits above Refinements 1–3: they are proxy-discipline; §0 is what proxy-discipline is *for*.*

And "Refinement 5" — the honest-incompleteness counterweight, also present verbatim in spikes.sop.md §0c (already captured in the main canonical extract) — confirms the two ideas (proxy-discipline + honest-incompleteness-is-discharge) travel together everywhere they appear in the estate.

Commentary (mine): the "Refinement N, dated, Joseph-directed" convention in this SOP is itself worth noting to the drafter as a structural pattern — it's how this estate tracks provenance of *decisions* inline, which is itself an instance of provenance-as-proxy-with-explicit-dating (the SOP practices what §0 preaches).

---

## Adjacent misfire-dir material — set aside

`epistemic-axes.md` and `integration-metabolism.md` (both `~/src/arch/firmatum/verisectorium/.archive/theory-misfire/last-adhoc-src/`) were originally excerpted here for their sharper "multi-agent agreement" and "delete-test" treatments. Both files' own "Strength & grounds" sections describe themselves as *this corpus's synthesis* ("the synthesis is this corpus's" / "the norm half is estate law restated" with only a same-corpus incident as the worked specimen) rather than a verbatim restatement of a specific quotable upstream segment — no single upstream file/passage could be identified and verified to stand behind their content the way `norm-proxy-discipline.udon` stands behind `proxy-discipline.md` above.

misfire-origin material set aside per steward rule 2026-08-09 (see `.archive/theory-misfire/last-adhoc-src/epistemic-axes.md` and `.archive/theory-misfire/last-adhoc-src/integration-metabolism.md` if the steward reopens it).

**Steward-gated, not sweep-recommended:** the wider `last-adhoc-src/` directory (~90 files) that these two lived in appears to be a substantial prior draft of verisectorium's own segment corpus, archived as a "misfire" per commit 23dd5cc (2026-08-06) — "failed to orient itself as instructed and so the work is far, far inferior." Per the 2026-08-09 steward ruling, this directory is not being recommended here as sweep/reference material; any future use of it (including as a source to trace upstream citations from, the way `proxy-discipline.md` led to `norm-proxy-discipline.udon` above) is a call for the steward to make, not a default for a drafter to act on unprompted.
