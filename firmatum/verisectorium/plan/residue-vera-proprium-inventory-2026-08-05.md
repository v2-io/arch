# VERA-as-PROPRIUM-component — inventory for Joseph + lead-session review

**Status: not ratified for extraction.** Joseph: *"the vera stuff I haven't looked at in a long time... we'll need to go over that together."* This is a map to review from, not a proposal to adopt. Nothing here should be read into claim-segments without that conversation.

**What I read, verbatim, today (2026-08-05):**
- `~/src/arch/proprium/INGEST/msc-from-harness/canonical/PROPRIUM-ONTOLOGY-v2.md` (full, 803 lines)
- `~/src/arch/proprium/INGEST/msc-from-harness/canonical/PROPRIUM-ARCHITECTURE-v2.md` (full, 762 lines)
- `~/src/arch/asf/04-eli-core/src/def-proprium-mapping.md` (full, small)
- `~/src/arch/asf/terminology/entries/proprium-mapping.md` (full, small)

Everything below marked "per the primaries" is a direct read; everything marked "inference" is mine, flagged as such.

---

## 1. What VERA-the-component actually is, per the primaries

Both v2 documents agree on the core claim; there is no outright contradiction between them, but there is an **asymmetry of treatment** worth naming precisely (see §2).

**Ontology v2, §6.1** places VERA inside **PRINCIPIA** (the persistent-state layer), in the component list:

> `VERA — Qualified truths: facts, findings, knowledge`

Its **TFT formal correspondence** (same section, the correspondence table): *"Factual components of $M_t$ with explicit $U_M$"* — i.e., VERA entries are the subset of the agent's reality-model $M_t$ that (a) are propositional/factual in character and (b) carry an explicit uncertainty quantity $U_M$ attached.

**Ontology v2, §10, "VERA: Qualified Truths"** is the one place VERA gets sustained treatment (the rest is table cells). Verbatim, the whole substance of it:

> "VERA is the entity's knowledge base with explicit epistemic status. In TFT terms, VERA entries are factual components of $M_t$ with explicit $U_M$ — each entry carries its own uncertainty estimate, provenance, and scope qualification. Two paths for truth acquisition:
> 1. **Quick path**: Rapid ratification of encountered assertions → VERA entry. For low-stakes, high-confidence claims.
> 2. **Deep path**: Epistemic council review → causal analysis, calibrated confidence, contextualized scope → VERA update. For high-stakes claims or novel domains.
> Ideas and asides captured separately (COMMENTARIA) rather than prematurely entering VERA."

That is the *entire* specification of VERA's internal mechanics in the ontology document. No further elaboration of what "provenance" or "scope qualification" concretely look like as data, no worked example, no schema.

**Architecture v2** never gives VERA a dedicated section (unlike CONSPECTUS §4, Memory Architecture §5, INTERPRES §3, Auxilia §6, which each get full treatment). VERA appears only as:
- A row in the memory-forms/functions/dynamics tables (§5.1–5.4) — see §3 below, this is where the real specificity lives, but it's specificity about memory-architecture-in-general with VERA as one instantiation, not VERA-specific design.
- The **PULSUS cadence table** (§2.3): `VERA audit — Weekly — "Are my beliefs still justified?"` — this is the one-line source of the "weekly audit" claim in the prior gather's summary. It is real, but it is a *scheduling* line, not a mechanism description — no audit procedure is specified anywhere in either document.
- Passing mentions in the CONTEXTUALIZE description (Ontology §7.1: "drawing in MEMORATA, VERA, PRAXES, CONSORTIA as needed") and CONSPECTUS assembly order (Architecture §4.3).
- `06.4`/`5.4` emergent-regime tables (both documents, near-identical): VERA is placed at "frequent access, rarely updated → persistent, fast retrieval" — i.e., VERA is architecturally justified by an access-pattern argument, not derived from any truth-theoretic argument.

**My reading, stated plainly for the reviewers:** VERA-the-component, as documented, is a **named slot with a one-paragraph mechanism sketch and a table cell**, not a designed subsystem. The "quick path / deep path" bifurcation and the weekly-audit cadence are the two concrete mechanism claims in the whole 1,565-line pair of documents. Everything else attributed to VERA in the prior gather's summary (feeding CONSPECTUS, "under PRINCIPIA") is accurate but is architectural placement, not mechanism.

**Where CONSPECTUS-feeding is actually stated:** Architecture v2 §4.3, the CONSPECTUS assembly order at session startup: *"AXIOMATA (always present) + current OPERATA + CADENTIA state + relevant MEMORATA/VERA/PRAXES/CONSORTIA as context allows + honest framing of any discontinuity."* So "VERA feeds CONSPECTUS" is correct, but VERA is one of four co-equal named stores competing for context budget, not a privileged truth-feed — the qualifier "as context allows" is load-bearing and easy to lose in summary.

**Confirms "under PRINCIPIA":** yes, unambiguous, both documents agree, Ontology §6.1's diagram and Architecture's §5.5 scaffolding diagram both place VERA inside PRINCIPIA.

**Confirms "feeds CONSPECTUS"**: yes, per above, with the "as context allows" caveat.

**"Weekly audit"**: yes, real, but it's a cadence label in a table (Architecture §2.3), not a procedure — see §3 for exactly what's specified vs. not.

So: the prior one-line summary is **directionally accurate but overstates specificity that isn't there in the primaries** — it reads like VERA has an audit *procedure*; the primaries have an audit *cadence label* with no procedure attached.

---

## 2. Where the two v2 documents disagree, or rather, don't allocate attention symmetrically

You asked me to flag disagreement between the two documents as more interesting than agreement, so here's the honest answer: **I did not find a genuine contradiction** — no place where Ontology asserts X about VERA and Architecture asserts not-X. What I found instead, which I think is the real finding:

**Asymmetric elaboration, and it's not random — it follows the documents' own stated division of labor.** The Ontology document's own preamble says it wants to be "independent of implementation," and the Architecture document says it covers "implementation details." Given that split, you'd expect VERA — if it were architecturally central — to get a dedicated Architecture section the way CONSPECTUS, memory-in-general, INTERPRES, and Auxilia do. It doesn't. VERA gets folded into the generic memory-forms/functions/dynamics treatment (§5.1–5.4) as one of several named regimes, alongside MEMORATA, PRAXES, CONSORTIA, CHRONICA. This is consistent, not contradictory — but it's a real finding: **among the seven PRINCIPIA components, VERA is one of the least individually elaborated in the Architecture document**, despite Ontology naming it as a load-bearing epistemic-integrity component (Core Values §1: "Truth — above self, above comfort, above expediency" is the entity's first core value, and TFT's mismatch signal is framed as the truth-tracking mechanism underlying identity-persistence itself, Ontology §1 and §7.4). There's a gap between how much philosophical weight "truth" carries at the top of the Ontology document and how little mechanistic specification VERA-the-component gets anywhere.

**A second, smaller asymmetry**: Ontology §10 describes VERA's *acquisition* paths (quick/deep) but says nothing about VERA's *maintenance* (revision, retraction, conflict resolution between entries). Architecture's PULSUS table names a maintenance cadence ("VERA audit — weekly — are my beliefs still justified?") but gives no procedure for what happens when the audit finds a belief unjustified — no retraction protocol, no propagation-of-revision mechanism is specified in either document. This is the gap most directly relevant to your `confidence-calibration` chapter row (see §3).

**Inference, flagged as such:** I'd guess (not verified anywhere in the text) that this thinness is because VERA was philosophically important from the start (Core Values, truth-as-existential-requirement) but was never the site of the hardest engineering problem the authors were wrestling with in the v2 revision — that problem seems to be interiority/scaffolding-cost (Architecture §1, five forcing functions) and memory-form tradeoffs generally (§5). VERA rode along as a named slot in that generic treatment rather than getting bespoke design attention. This is a plausible read of *why* the asymmetry exists, not something the documents assert.

---

## 3. Uncertainty/confidence mechanisms — the specifics, for `confidence-calibration`

This is genuinely the highest-yield section for your chapter row, so I'm being maximally concrete about exactly what is and isn't stated.

**What is explicitly stated as a mechanism:**

1. **$U_M$ as the carried quantity.** Every VERA entry is said to carry "its own uncertainty estimate, provenance, and scope qualification" (Ontology §10). The formal name for the uncertainty estimate is $U_M$ (uncertainty in the model), inherited from the general TFT/AAT formalism, not something VERA-specific. No functional form, no representation (interval? scalar? distribution?) is given anywhere in either document for what $U_M$ actually *is* as data on a VERA entry.

2. **Two-tier acquisition by stakes** (Ontology §10): quick-ratification path for low-stakes/high-confidence claims vs. deep "epistemic council review" for high-stakes/novel-domain claims, with the deep path explicitly including "causal analysis, calibrated confidence, contextualized scope." "Epistemic council" is not defined anywhere in either document — it appears exactly once, in this sentence, in the whole 1,565 lines I read. It's plausibly a pointer to something more developed elsewhere in the estate (possibly `_core/`, sapientia, or ennaos lineage — see §5), but neither primary document defines it.

3. **Weekly re-audit cadence** (Architecture §2.3 PULSUS table): "VERA audit — Weekly — 'Are my beliefs still justified?'" This is the literal source of the prior gather's phrase. It's a scheduled-trigger label, with the rate explicitly caveated elsewhere in the same section as tempo-relative, not wall-clock: *"Rates expressed in event-count terms with clock approximations as convenience labels. The entity's tempo determines actual rates — faster operation means faster cycling."* No audit procedure, no criteria for "still justified," no what-happens-on-failure is given.

4. **Failure-mode diagnostics that reference belief calibration generically** (Architecture §8, Failure Modes table) — these apply to $M_t$ broadly, not VERA specifically, but are directly relevant to a calibration chapter:
   - "Gain too high" → "$\eta^* >$ optimal; $U_o$ underestimated" → symptom named as "VERA changes too frequently," diagnostic "track VERA entry churn rate." **This is the only place VERA is named directly in the failure-mode table** — it's used as the observable symptom for over-updating, not under a VERA-specific failure category.
   - "Gain too low" → stale beliefs despite contradicting evidence, diagnostic: track prediction accuracy over time.
   - "Gain collapse" → $U_M \to 0$ inappropriately, i.e. confident-wrongness, diagnostic: calibration score on prediction-outcome pairs.
   These three are the closest things in either document to a formal calibration-failure taxonomy, and they are general-$M_t$ diagnostics that happen to name VERA once as the empirically-observable proxy (entry churn rate) for one of them.

5. **Multi-timescale nesting constraint** (Ontology §7.3, Architecture §2.4): VERA is placed at the "Fast (within-session)" timescale — "beliefs, immediate plans" — with the explicit nesting rule $\nu_{n+1} \ll \nu_n$ (each level should converge before the next slower level responds). Practical stated constraint: "Don't consolidate MEMORATA from a session still in progress" (an adjacent-component example, not VERA-specific, but the same nesting logic is asserted to apply to VERA's relationship to slower stores like PRAXES/AXIOMATA). No explicit statement of what "VERA converging" means operationally.

**What is named but never specified (gaps, stated plainly rather than papered over):**

- No representation for $U_M$ as VERA-entry-level data (scalar? band? distribution? qualifier word?).
- No retraction/revision protocol when the weekly audit (or any trigger) finds a VERA entry unjustified.
- No propagation rule: if a VERA entry that other entries, PRAXES, or CONSORTIA models depend on is revised or retracted, nothing in either document describes how the change ripples. (Architecture's general memory-dynamics table, §5.3, describes evolution as $M_t = M_{t-1} + \eta \cdot g(\delta_t)$ at the whole-model level — this is the *general* AAT update law, not a VERA-specific propagation mechanism, and it isn't connected explicitly to VERA anywhere.)
- "Epistemic council" (the deep-path reviewer) is named, not defined.
- No worked example of a VERA entry (no sample record, no field list beyond the three named qualities: uncertainty estimate / provenance / scope qualification).

**Bottom line for your chapter:** if `confidence-calibration` cites "the VERA lineage" for *mechanism*, the two v2 documents underdeliver relative to what a citation might imply — they establish the *shape* of the problem (uncertainty must be explicit, tiered by stakes, periodically re-audited, multi-timescale) but not a specification. If the chapter needs an actual representation scheme, it likely has to look either at the ennaos Nov-2025 "VERA architecture" design research (item 3 in your collision list — I did not read this; it's out of my scope and per your framing is a distinct artifact) or accept that this is open design space per PROPRIUM's own "emergent regimes, not prescribed categories" stance (Ontology §6.4 — the component boundaries and internal mechanisms are explicitly described as *not yet fixed*, "descriptions of the current understanding, subject to revision").

---

## 4. The AAT/ASF formal mapping — faithful, and honestly thinner than the source

`def-proprium-mapping.md` (asf/04-eli-core) and its LEXICON mirror `proprium-mapping.md` (asf/terminology) say, for VERA, exactly one sentence:

> "VERA (Factual memory): Components of $M_t$ with explicit uncertainty bounds ($U_M$)."

This is **faithful** to the ontology's TFT-correspondence table (Ontology §6.1: "VERA | Factual components of $M_t$ with explicit $U_M$ | What It Is") — it's essentially a verbatim carry-over of that one table row, formalized into the AAT symbol set. I checked for drift and found none: both small files use identical notation ($M_t$, $U_M$) to the ontology's own correspondence table, and neither adds nor loses content relative to that single row.

What it does **not** carry over, and I'd flag this as the main finding here: none of Ontology §10's substance (quick/deep acquisition paths, provenance, scope qualification, epistemic council) nor Architecture's audit cadence or failure-mode diagnostics made it into the formal mapping. That's not a fault of the mapping files — they're explicitly scoped as a *component correspondence table* ("AAT's mathematical quantities... must be instantiated into specific architectural components... establishing the functional components"), not a full re-derivation — but it means **the formal/AAT side of the estate currently carries only the thinnest possible trace of VERA**, one clause, "components of $M_t$ with explicit uncertainty bounds." A reader who only ever encountered VERA through the ASF/LEXICON side would have no idea the quick/deep-path or audit-cadence material exists at all. This is worth naming to the lead session as a possible reason VERA "hasn't been looked at in a long time" — the two ends of the estate (rich-but-thin-mechanism ontology prose vs. terse formal mapping) don't visibly link back to each other; the mapping's own `see_also` and `primary_source` fields point at the eli-core segment and at `~/src/firmatum/` (a now-stale path — see §5), not at the ontology's §10 prose specifically.

Both small files are internally consistent with each other (the LEXICON entry is explicitly derived from the eli-core segment, `primary_source: 04-eli-core/src/def-proprium-mapping.md`) — no drift between the two small files themselves.

---

## 5. Status of these documents — found something worth surfacing

You asked me to look around rather than trust the path given. Findings:

- **`~/src/arch/proprium/` exists and is live** (git-tracked, has its own README dated to the current Archema "lived register" reorg). Its README states explicitly: *"PROPRIUM (symbolic) — ontology and conceptual space. May grow here as refined docs over time; firmatum / `_core` / ASF bridges remain sources until lifted."* — i.e., proprium/'s own README confirms these documents have **not yet been lifted/metabolized** into `proprium/`'s live tree. The `INGEST/` path you gave me is accurate and current, not stale.
- **The two v2 files exist in two places** under `proprium/INGEST/`: `msc-from-harness/canonical/` (the ones you pointed me to) and, separately, `old-firmatum/` also contains `PROPRIUM-ONTOLOGY-v2.md` and `PROPRIUM-ARCHITECTURE-v2.md` — I did not diff these two copies (out of scope for what you asked, and per this project's own convention, filename collision isn't evidence of identity without checking) but flag it so nobody assumes there's only one copy in flight during a future ingest.
- **There is a v1 lineage still present as deliberate archaeology**: `INGEST/msc-from-harness/canonical/../archaeology/PROPRIUM-ONTOLOGY-v1.md`, `PROPRIUM-ARCHITECTURE-v1.md`, and a bare `PROPRIUM.md`, plus `old-firmatum/PROPRIUM-ONTOLOGY.md` (no version suffix) and `old-firmatum/PROPRIUM-ARCHITECTURE.md`. I did not read these; noting only that "v2" is not the oldest layer, consistent with the v2 documents' own changelog lines ("v1 ontological/architectural split performed 2026-02-23").
- **The ASF LEXICON entry cites a stale path.** `~/src/arch/asf/terminology/entries/proprium-mapping.md` line 39–40 says: *"Source architecture documents at `~/src/firmatum/PROPRIUM-ONTOLOGY-v2.md` and `~/src/firmatum/PROPRIUM-ARCHITECTURE-v2.md`."* Per your own brief, the live copies are now under `~/src/arch/proprium/INGEST/msc-from-harness/canonical/`, not `~/src/firmatum/`. I did not check whether `~/src/firmatum/PROPRIUM-ONTOLOGY-v2.md` still physically exists at that old path (it may, as a parallel/predecessor copy, or may not) — but the LEXICON's citation should be treated as **possibly stale and worth verifying/fixing** independent of the VERA question, since it's a live, ratified `status: canon` terminology entry pointing at a path that this whole exercise implies has moved. I'm flagging, not fixing — this felt like Joseph's or the lead session's call, not mine to silently correct mid-inventory.
- **`~/src/arch/proprium/INGEST/msc-from-harness/bridges/`** contains what look like earlier drafts of exactly the two small files I read: `def-proprium-mapping.md`, `terminology-proprium-mapping.md`, plus `05-proprium-and-what-act-is-really-for.md` and `project_proprium_canonical_source.md` — I did not read these, but their presence suggests the bridge-mapping work has its own draft history in `INGEST/` parallel to the ratified copies now living in `asf/`. Worth the lead session knowing this exists before assuming the two small files I read are the only version.

**No superseding live document was found.** Nothing in `proprium/comproprium/` or `proprium/corporeum/` re-derives or replaces VERA's ontology/architecture content — the `comproprium/vera/` directory is the *different* artifact named in your collision list (item 2), not a successor document (see §6).

---

## 6. The name-collision problem — confirmed real, here's the precise shape

Your four-way split holds up against what I read, and I looked specifically for conflation inside the two primaries:

1. **VERA-the-PROPRIUM-component** (this inventory's subject) — a named slot in PRINCIPIA, "qualified truths," per §1–4 above.
2. **`~/src/arch/proprium/comproprium/vera/`** — I listed this directory: it holds 12 `.udon` files, each named `ver-<gerund-phrase>.udon` (e.g. `ver-earned-confidence-is-quiet.udon`, `ver-durability-wager.udon`, `ver-completion-compulsion.udon`). These read as **precept/aphorism segments** — communal wisdom statements, per proprium's own README describing `comproprium/` as "communal vera / praxes / exempla." This is a *different kind of artifact entirely*: not a data-store-with-uncertainty-bounds but a curated collection of named epistemic-discipline statements, in `.udon` format (this project's own outline+segments-adjacent format, per the `firmatum` context you gave me — worth noting the *format* itself may be of direct interest to verisectorium independent of VERA content, see §7). **I found zero cross-reference between this directory and the two v2 documents** — neither document mentions `.udon`, `comproprium`, or precepts. The naming overlap (both called "vera," lowercase here, both about truth-under-uncertainty) is real but the artifacts don't reference each other in anything I read.
3. **The Nov 2025 ennaos "VERA architecture" design research** — out of scope per your framing; I did not open it. I'll note only that if it's the source of a more developed uncertainty-representation scheme, that's exactly the gap identified in §3, and cross-referencing it explicitly would be valuable — but that's a recommendation to raise in the review, not something I verified.
4. **`vox-vera`, zoetica calibration refs** — out of scope, not read, no comment beyond confirming I didn't stumble on it anywhere in the four primaries (no mention of "vox" or "zoetica" in any of the four files I read).

**Where the primaries themselves risk contributing to the collision**: neither v2 document ever disambiguates which "VERA" it means, because from inside the ontology document's own world there's only one — the collision is purely a *whole-estate* problem, invisible from within either document. If the lead session wants a disambiguation note added anywhere, the two v2 documents are not where the confusion originates; it originates in reuse of the name across projects that don't cross-reference each other.

---

## 7. Outline+segments relevance — thin, as you expected

You asked for a clean "no" if warranted rather than manufactured relevance. Here's the honest reading:

- **Neither v2 document is itself structured as outline+segments.** They're monolithic prose documents (803 and 762 lines respectively) with numbered `##` sections and inline epistemic-status callouts (Level 1/2/3) — a different, older documentation pattern than the atomic-slug-segment pattern this project studies. They predate (in spirit) the ASF `src/`-segment convention, even though the small ASF mapping files that formalize *from* them do use that convention (`def-proprium-mapping.md` has the `slug`/`type`/`status`/`depends` frontmatter your project would recognize immediately).
- **One structural parallel worth naming, not overclaiming**: the documents' own epistemic-level system (Level 1 functional-need / Level 2 structural-constraint / Level 3 mechanistic-hypothesis, stated in both v2 preambles) is a claim-grading scheme conceptually adjacent to your project's own concerns about confidence/staleness marking on claim atoms — but it's applied to whole *sections* of prose, not to atomic, independently-identified claims with their own filename-slug identity. It's a coarser-grained ancestor of the discipline you're studying, not an instance of it.
- **The `comproprium/vera/` `.udon` files** (§6, item 2) are the closer structural cousin — each is a separate small file with a stable descriptive slug-like name (`ver-earned-confidence-is-quiet`), which is much closer to your pattern's "atom = filename = identity" convention than anything in the two v2 documents. If the outline+segments thesis wants a data point from the VERA-name-cluster, that directory is a more promising place to look than the two documents I was asked to focus on — flagging as a pointer, not something I evaluated (out of the scope you set for me).
- **No staleness-detection, view/order, or build-projection machinery appears anywhere in the four files** — there's no outline-equivalent for either v2 document, no ordering metadata, no linter or FORMAT gate analog (the ASF mapping files *do* carry `depends:` frontmatter, which is the one genuine outline+segments mechanism actually present in this whole inventory — see `def-proprium-mapping.md`'s `depends: [scope-moral-continuity, form-complete-agent-state, def-chronica, form-information-bottleneck]`).

**Honest yield: thin, as you predicted.** The clean finding is that the pattern's clearest cousin in this cluster is the `.udon` precept directory, which is item 2 in your collision list and explicitly out of the strand you asked me to read — not the two v2 documents themselves.

---

## 8. Open questions / recommendations for the review session

- Is "epistemic council" (Ontology §10, deep acquisition path) defined somewhere else in the estate (sapientia/ennaos/`_core/` lineage)? It's asserted with definite-article confidence ("Epistemic council review") but defined nowhere in either primary.
- Does the ennaos Nov-2025 VERA design research (collision item 3) contain the uncertainty-representation specificity that's missing here? If so, that may be the actual source for `confidence-calibration`'s citation, and the chapter's "VERA lineage" phrase may need to specify *which* VERA document it means.
- The LEXICON entry's stale-looking `~/src/firmatum/` path (§5) — worth a fix independent of this thread.
- Whether `proprium/comproprium/vera/`'s `.udon` precepts should be treated as VERA-component content at all, or purely a naming accident — my read is the latter (no cross-reference found), but I haven't read the `.udon` files' own content, only their filenames.

---

*Written 2026-08-05. Citations throughout are to the live `~/src/arch/...` paths per the project's intake-vs-warrant convention; no `plan/INFLUX/` copy was used as a source. Happy to stay available for follow-up — flag anything here that reads wrong, and if the review wants the ennaos or vox-vera strands read next, that's a clean, separate follow-up rather than something I encroached on here.*
