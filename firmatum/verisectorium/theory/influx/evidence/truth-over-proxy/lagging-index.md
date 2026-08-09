# Evidence sweep — the lagging index / stale registry cited as authority

*Slice of the truth-over-proxy evidence sweep for `claim-truth-over-proxy`. Verbatim extracts unless marked otherwise, with source path and surrounding context. Oldest instance kept where a passage is duplicated across the estate (asf originals over verisectorium influx copies over md-press fixtures).*

---

## Canonical instance — NOTATION.md as lagging index (asf)

**Source: `~/src/arch/asf/NOTATION.md:9-25`** (frontmatter warning box, current live file)

> [!warning]
> **This file is a lagging index, not the arbiter (drift caveat).** It is
> hand-maintained, and the live theory drifts away from it: spike
> findings and new/rewritten segments routinely change a quantity's
> structure without updating this file. **It is not authoritative.** When
> this index and a segment's own definition/derivation disagree, the
> segment is ground truth and this file is the thing that is stale. Use
> it to *locate* a symbol, never to *settle* what it means or how it
> decomposes — settle that from the segment, re-derived. (Worked
> instance: this file glosses ρ as a single "mismatch injection rate"; an
> independent recheck established ρ² decomposes additively via the exact
> `#result-mismatch-decomposition`, and that a multiplicative
> `ρ = ρ_external·f(𝓜)·g(π)` is type-incorrect — the segment, not this
> gloss, is the truth. See `doc/sop/spikes.sop.md` §0 and CHANGELOG
> 2026-05-18.) A structural fix — auto-deriving this file from segment
> definitions so it cannot drift — is queued in `TODO.md`.

Note the last line: the estate's own prescribed remedy for a lagging index is not "update it more diligently" but **make it derived** (auto-generated from source) so it structurally cannot drift.

---

## The governing law that generalizes it — spikes.sop.md §0 (Joseph 2026-05-18)

**Source: `~/src/arch/asf/doc/sop/spikes.sop.md:19-30`** (verbatim, current)

> ## 0. The core principle — truth is the arbiter; everything else is a proxy (Joseph 2026-05-18)
>
> **This governs every section below it.** The job is to get the *theory's truth* right. Provenance, git history, CHANGELOG, the INDEX label, `NOTATION.md`, the spike's own framing, the segment's own assertion, audit findings, agent consensus, even the convergence of multiple independent agents — **all of these are mild proxies for truth, and every one of them drifts.** They are useful for *locating* a question and *cheap-screening* it; they never *settle* it. A question is settled only by the mathematics, re-derived independently far enough to stand on constitutive structure (definitions that make the core cohere) + forced identities + elementary steps — *not* on what any artifact says.
>
> Two concrete, recurring traps this names:
>
> - **`NOTATION.md` is a lagging index.** Spike findings and new segments routinely fail to update it; the live theory drifts away from it. "The notation defines X as Y" is *not* evidence that X *is* Y — it is at most evidence about a document that may be stale. Never cite it as authority; at most as corroboration explicitly marked non-load-bearing.
> - **"Verified against \<artifact\>" is proxy in verification's clothes.** Tagging a step `[Verified]` because a document *says* it does not make it verified — it verifies the document, not the truth. The tell: a `[Verified]` whose object is "what file F asserts" rather than "the derivation holds." (Worked instance, 2026-05-18: a $\rho$-factorization judgment leaned on "NOTATION defines $\rho$ as a single primitive" tagged *verified*; the real argument rested on the constitutive meaning of mismatch + the Kalman innovation identity + algebra, and was *stronger* once the NOTATION proxy was deleted. Joseph: *"you have to care about the theory's TRUTH more than anything else — provenance and things like that are only mild proxies."*)

**This is the same worked incident named twice** — once from NOTATION.md's own vantage (the file admitting it was wrong) and once from the SOP's vantage (the process law legislated in response). The ρ-factorization case is the estate's canonical concrete instance of "a stale index cited as evidence and caught."

**§0c, same file, immediately following** (the counterweight — worth carrying alongside so the claim segment doesn't over-rotate toward endless doubt):

> §0 and the gates, taken without this, drive a **verification-regress**: every gate spawns another, nothing is ever released, and an honest "not yet" feels like failure. It is not. **The gates exist to prevent false confidence, never to forbid honest incompleteness.**

---

## Same law, restated in audit-routing.sop.md (the parallel process document)

**Source: `~/src/arch/asf/doc/sop/audit.sop/routing.sop.md:296-301`** (verbatim, current)

> **The whole evidence hierarchy is proxy; truth is the arbiter** *(authoritative SOP — Joseph 2026-05-18; full statement `sop/spikes.sop.md` §0).* Every entry in the hierarchy below — and the ledgers, CHANGELOG, INDEX, `NOTATION.md`, the segment's own assertion, the auditor's framing, and even multi-agent convergence — is a **mild proxy** that *drifts*. It locates and cheap-screens a question; it never settles it. Settle by the mathematics re-derived independently (constitutive structure + forced identities + elementary steps), not by what any artifact says. Two named traps: `NOTATION.md` is a *lagging* index — the live theory drifts from it; never cite it as authority. And a `[Verified]` whose object is "what file F says" rather than "the derivation holds" is proxy in verification's clothes. The hierarchy is for *screening order*, not for *deciding truth*.

Immediately following, the **git-recency vs git-provenance distinction** is a related and reinforcing pattern (same file, lines ~304-309): "`git`-*recency* is poisoned by rename sweeps... recency ordering cannot stand in for the content check. But git *provenance* is a valid, encouraged, non-destructive investigative technique... Don't infer status from the log's *recency*; do use the log's *provenance*." This is the same failure shape one layer down — an artifact's superficial freshness (git timestamp) can itself become a false proxy for truth, exactly like a stale index's superficial authority.

Provenance note: `spikes.sop.md` §0 and `routing.sop.md`'s parallel passage are dated to the same event (Joseph 2026-05-18); memorata's git-file-origin dates the spikes.sop.md copy to 2026-06-01, but the in-text attribution "Joseph 2026-05-18" is the more reliable date (git blame/rename noise per the SOP's own caveat above). Per "oldest one wins" for duplicated passages, the asf originals (not the verisectorium influx copies, not the md-press fixtures) are the canonical source.

---

## Verisectorium's own prior synthesis of this exact vein

misfire-origin material set aside per steward rule 2026-08-09 (see `.archive/theory-misfire/last-adhoc-src/terminology-substore.md` if the steward reopens it). The underlying upstream this passage was itself drawing on (asf's NOTATION.md/LEXICON.md contrast) is already quoted directly above and below from primary sources.

---

## The derived-vs-authored contrast, independently corroborated from udon

**Source: `~/src/arch/firmatum/udon/v2/udon-needs/02-tooling-needs/reports/theory-of-agentic-tooling.md:329`** and (duplicate/earlier?) `~/src/arch/firmatum/udon/v2/udon-needs/01-ideation/02-provenanced/syntheses/asf-dossier.md:319` — near-identical text, treat asf-dossier.md as upstream source since it's filed under "provenanced" originals and theory-of-agentic-tooling.md is a "report" (downstream synthesis); verify order with `git log` if it matters for citation:

> **Root reference files**: `NOTATION.md` opens with a drift caveat worth quoting for any tooling that might consume it — "a lagging index, not the arbiter … use it to *locate* a symbol, never to *settle* what it means" (with a worked instance where the gloss was type-incorrect against the segment) — and carries the **accumulation-typing convention** (2026-05-19)... That is a live example of a notation being *designed to carry epistemic status* — directly relevant precedent for UDON. `LEXICON.md` is auto-generated from per-term files (`terminology/entries/`, append-only decision events, multi-agent-safe by construction — itself a tooling pattern worth studying), and carries the full segment-type / epistemic-status / promotion-gate / Findings-schema vocabulary this dossier's tier labels come from.

Same estate-wide pattern (asf's own two reference surfaces, one derived/trusted, one hand-maintained/legislated-around) independently rediscovered by a different sub-project (udon) doing tooling research — convergent evidence, not just repetition, since udon's interest is "what tooling pattern should we build," not "what does asf's process law say."

---

## Status columns / navigators as the same failure shape, one level down from a "notation index"

**Source: `~/src/arch/asf/msc/meta-process-review-2026-07-07/01-theory-content-lifecycle-findings.md:45-48,64`** (verbatim, current — an internal process audit of asf's own lifecycle discipline)

> 3. **OUTLINE stage-marker staleness, recurring and unremediated in 04-eli-core.** Verified stale (marked `missing` in the OUTLINE status column, but file exists at `stage: draft`, 84-98 lines of substantive content each): `scope-eli` (row 48), `def-five-constitutive-factors` (50), `def-eli-cohort` (51), `scope-emergence-conditions` (119), `scope-witness-bidirectional` (120), `obs-growth-vs-drift` (124). Plus `scope-observation-ambiguity-modulation` in `03-llm-core`. (Genuinely-missing and correctly-marked: `der-substrate-independent-persistence`, `def-character-aspiration-dialectic`, `form-congruency-selfhood`.) The 2026-06-30 fix did not cover these.
> 4. **No tooling checks OUTLINE-stage vs frontmatter-stage.** `bin/lint-outline` reads `stage:` from frontmatter for *ordering* checks but never compares it to the OUTLINE table's hand-authored status column — which is exactly why #3 persists silently.

And the summary table row (line 64):

> | P4 | **OUTLINE hygiene (stage-marker accuracy + ordering)** | Segment authored / stage changes | Update OUTLINE row status; keep ordering topological | **Stale/recurring.** Confirmed stale rows in 04-eli (6) + 03 (1); point-fixed 2026-06-30, class recurred; no tooling check of status column. |

This is a **second, independent instance** of the exact same failure mechanism as NOTATION.md — a hand-maintained status column drifting from the live truth of the files it describes — caught by direct audit rather than by theory-derivation, and explicitly diagnosed as recurring *because no tooling cross-checks it* (the structural-fix insight the NOTATION.md warning box also reaches for). Worth citing as a second data point that the failure is a class, not a one-off.

---

## The general principle, generalized beyond NOTATION.md to navigators/status-columns/README-priority-lists

misfire-origin material set aside per steward rule 2026-08-09 (see `.archive/theory-misfire/last-adhoc-src/steward-surfaces.md` if the steward reopens it). The one identifiable upstream it named — the comproprium README's priorities section (`~/src/arch/proprium/comproprium/README.md`) as an example of a steward's-view-marked-as-such surface — was not independently re-verified as a load-bearing "lagging index" instance within this pass; if this vein is wanted, it needs a fresh direct read of that README rather than reliance on the misfire's characterization.

---

## Confirmed-stale index caught by an audit, worked prediction of risk

**Source: `~/src/arch/asf/audits/AUDIT-WORKING-384279/00-initial-predictions.md:84`** (verbatim, current)

> 7. **The Layer-0 Appendices preamble paragraph naming five constructive-impossibility instances.** That's a paragraph at the appendix gate. If it has drifted from the canonical text inside the named segments (#disc-constructive-impossibility-posture, the five instance segments), readers will get a stale catalog. High-leverage check.

This is a pre-registered *prediction* (not yet a confirmed finding as extracted) that a specific catalog paragraph may have drifted — worth noting as an instance of the estate now routinely pre-flagging "stale catalog" risk before auditing, i.e. the lesson has become anticipatory discipline, not just after-the-fact correction.

---

## A stale INDEX row caught and scheduled for reconciliation mid-adjudication

**Source: `~/src/arch/asf/spikes/.routing-trail/SPIKE-WORKING-417303/adjudication.md:529`** (verbatim, current — table row from a spike-routing adjudication)

> | fenchel-bregman | `integrated-misfiled` (fully — both halves) | independent-verify the "full reframe landed" claim (`disc-additive-coordinate-forcing.md:14-84`), then `git mv → .integrated/`; **reconcile stale INDEX row 106 to "fully landed"** | n/a (done) |

A concrete operational instance: the adjudicator both independent-verifies the underlying claim in the segment (not trusting the INDEX) *and* separately schedules the stale INDEX row itself for correction — i.e. the discipline in practice: settle by the segment, then fix the proxy afterward as housekeeping, never the reverse order.

---

## Outside precedent — grep beats indexes because indexes go stale (general software engineering, cited approvingly)

**Source: `~/src/arch/firmatum/udon/v2/theory/to-integrate/refine-more/lsp-treesitter-learnings/README.md:90`** (verbatim quote from an external interview, current file)

> "Claude Code's 'agentic search' is really just glob and grep, and it outperformed RAG. The team tried several approaches to make agentic search better: local vector databases, recursive model-based indexing, and other fancy approaches. All had downsides (stale indexes, permission complexity). Plain glob and grep, driven by the model, beat everything."

Same file, a self-correction the estate recorded about its own handling of this exact quote (line 96) — worth including as a meta-instance of "don't trust a derived summary of a source over the source itself":

> **A correction I owe, in the interest of not laundering a claim upward.** My delegated sweep reported this as "they had already built the vector DB and *removed* it," and separately attributed to Cherny a quote about "security, privacy, staleness, and reliability" sourced to an X post I could not fetch. The interview I read supports *tried several approaches, all had downsides, stale indexes and permission complexity* — it does **not** narrate a build-then-remove, and it **does not mention LSP at all**. The stronger framing may well be true; it is not established by the source I actually read. **[relayed, downgraded]**

And the file's own design conclusion drawn from this (line 180) — the idea of an index that **declares its own staleness** instead of silently going wrong, which is the constructive answer to "never cite it as authority":

> That composes with §2 better than I expected, and it is worth stating as the report's one design recommendation. **[inference]** The whole staleness liability in §3 — my own `content modified` failure, Cursor's "futile search loops," Cherny's "stale indexes" — is a liability *because those indexes are load-bearing while stale*. An index that knows it is derived can do what none of them do: **report its own staleness instead of silently under-answering.** That is the fail-loud principle from §2 applied to the exact failure mode §5 says killed the indexed alternatives. It does not make the index free, but it converts the dominant failure from *wrong answer* to *declared uncertainty*, and a `census` that says "I am 40 commits behind, reconstructing" is a categorically different object from one that quietly returns fewer records.

This is genuinely adjacent material rather than the same finding restated — it's software-engineering precedent (external, non-estate-authored) that the estate's own theory-index problem is a specific case of a widely-recognized general failure mode, plus the estate's own constructive proposal (self-declaring staleness) for how to defuse it rather than merely warn about it. Worth flagging to the segment author as a possible "outside world agrees" citation and/or a forward-looking design idea (fail loud > silent drift), distinct from the "never cite as authority" retrospective discipline found elsewhere.

Related, shorter, same theme — an estate session-vault raw transcript (lower-tier source, but corroborating): `~/src/arch/firmatum/udon/v2/.archived/second-pass/spikes/session-vault/raw/claude/64bde246-learn-about-ultracode-s-repo-scan.md:1431`:

> 2. **Staleness.** An embedding index drifts from the code between re-embeddings. For a tool whose whole job is editing live code, a stale index is a real failure mode; grep/LSP are never stale.

---

## Stale registry pointer, named and excluded from a udon scoping pass

**Source: `~/src/arch/firmatum/udon/v2/udon-needs/01-ideation/scratch/use-udon-for-audit.md:41`** (verbatim, current)

> - **Off-topic** — `vivarium/{VIVARIA-DEFINITIONS, scratch/05-architecture}`, `autopax/.archive/…unified_catalog`, `ops/papers/_legacy/06-adjacent-repos.md` (a stale registry pointer to `FULL-SPEC.md`) → not UDON-demand; **Excluded**.

Minor/adjacent — noted mainly as evidence the "stale registry" framing recurs even in casual scoping notes across the estate, not just in formal process law.

---

## A stale index's *gloss* producing false confidence of having read the underlying material (udon, self-diagnosed)

**Source: `~/src/arch/firmatum/udon/v2/msc/read-log-2026-07-30/02-value-predictions.md:47`** (table cell, verbatim, current)

> | 14 | `theory/spikes-README.md` (29 KB) | **Low — predicted net-negative** | `v2/README.md` banners it as a stale 7/28 snapshot slated for removal. Per the hole-marker contract its *glosses* would give me the feeling of having read the material it indexes. Reading a stale index of things I haven't read is the specific trap that produced this project's worst orientation failures. | If it contains the "floor" material (register rules, hazards, the ten claims) in a form that exists nowhere else — in which case it isn't stale, it's *unmigrated*, and that's a finding about the reorg rather than about the file. |

Same phrasing recurs verbatim at `~/src/arch/firmatum/udon/v2/msc/read-log-2026-07-30/04-spikes-README.md:11`:

> material it indexes; reading a stale index of things I haven't read is the specific trap that

This is a distinct sub-species worth flagging separately from "cited as authority": here the danger isn't that the index's *content* is wrong, it's that *reading the index at all* generates the phenomenology of having read the underlying material without actually having done so — a stale index as a confidence-laundering device, independent of whether any individual claim in it is currently true. The file's own escape hatch is sharp: distinguish "stale" (superseded, safe to ignore) from "unmigrated" (the only surviving copy of real content, not actually safe to ignore) — collapsing those two is itself a failure mode worth naming in the segment.

(Note: both of these `MOVED/udon/v2/...` paths also appear as pre-rename duplicates at `arch/firmatum/udon/v2/...` — the `arch/firmatum/udon/...` copies are current live paths per the estate's naming-migration notes; `MOVED/` is the stale/relocated tree. Cite the `arch/firmatum/udon/v2/...` paths.)
