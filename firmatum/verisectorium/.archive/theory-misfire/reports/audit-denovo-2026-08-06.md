# De-novo audit — verisectorium plan corpus, 2026-08-06

*Auditor: a fresh instance, no prior contact with this project. Priming discipline held: `CHANGELOG.un`, `HANDOFF.md`, and everything under `plan/reports/` were unread when every finding below was written. Findings are ranked by consequence. Each carries its epistemic rung (Guess → Pattern → Hypothesis → Tested) per the ASF de-novo SOP §1.1.*

**Corpus is moving under the audit.** Snapshot taken 2026-08-06 00:47 at 79 segments; a re-check at the end of the session found **91**. Counts below are pinned to the snapshot unless marked otherwise. A concurrent writer is active in `last-adhoc-src/`.

---

## Process confession, first — because it is evidence about a segment you are drafting

I read the corpus **batched**: seven concatenated chunks, ~79 segments, four tool calls. I did this before being pointed at `~/src/arch/asf/doc/sop/audit.sop/de-novo.sop.md`, whose §4.4 names batch reading as the protocol's primary self-check failure and whose §3.1 aside records the *same* failure by the agent that co-wrote the rule.

So this audit is a third specimen for [[role-activation]] — and a sharper one than the two it carries, because I had the corpus's own [[experiential-reading]] segment **in my context** when I batched. Reading the claim did not fire. What fired, later, was a message that interrupted my tool cadence. That is precisely [[role-activation]]'s thesis (guidance naming an act survives; guidance naming a stance does not) confirmed against a reader who had *already read the stance-shaped version*.

The honest consequence for what follows: **prediction-delta findings are unavailable to me.** I cannot report causal gaps between segment N and N+1, because I never held segment N's model alone. Everything below is either cross-segment consistency (which batching preserves, and arguably favours) or first-hand source verification. Treat the absence of exposition-gap findings in this report as a coverage hole, not as evidence the corpus has none.

*Suggestion, offered as a finding about the corpus and not about me:* [[experiential-reading]]'s Working Notes ask whether reflections are corpus members or private scratch. The more useful open question this session surfaced is upstream of that — **the discipline has no admission point in a delegated brief.** A brief that says "audit this corpus" hands the receiving agent a task, and the reading cadence is chosen inside the agent's default rhythm before any instruction about reflection format is reached. If ch. 5's roles gap is worked, the de-novo-auditor role's activation moment is *the first Read call*, not the brief's prose.

---

## F1 · The bodies are not honest without the section a view would drop — Rung: **Tested** (structural, checkable against the corpus's own text)

**This is the finding I would most want acted on.**

Two of the corpus's own principles collide, and the collision is currently invisible:

- [[selection-and-projection]] establishes that projection over parts is what makes an atom evergreen, and states the authoring discipline it requires: *"keep each layer self-contained enough to be read without the layers a view drops."*
- [[appendix-placement]] states the same test for the claim/appendix split: *"the claim is understandable without the appendix."*

But this corpus systematically puts the **confident assertion in the body** and the **honesty in `## Strength & grounds`**. A view that projects out `Strength & grounds` — which is exactly the projection its own exemplar performs (`--public` strips Working Notes in the ASF monograph pipeline; the digest builder selects sections by normalized key, per [[build-forced-commitments]]) — converts sixty-odd hedged formulations into flat assertions.

Worked instances, all from the snapshot:

| Segment | Body says | `Strength & grounds` says |
|---|---|---|
| `multiple-views` | "The asymmetry is large — roughly a file against a re-verification cascade — and it is **stable**" | "The cost asymmetry itself **has not been measured** — no one has timed a view against the rewrite it replaced" |
| `atom-grain-parallelism` | "Halve the grain and the number of independently assignable work items roughly doubles; the practical ceiling on parallel work in a corpus is closer to its record count than to anything about the agents" | "(B) and (C) are argued rather than measured" — and the lived evidence offered for (A) is five agents over one batch, which tests no ceiling |
| `layer-speeds` | Five confident mechanism claims (A)–(E), stated as law | "the theorem's premises assume continuous dynamics and document edits are jumps… only the qualitative structure transfers" |
| `pedagogy-layering` | "*scaffolding that overclaims is worse than none*" — stated as a rule | "Heuristic, inherited from ASF's working discipline at a stated remove" |
| `working-note-lifecycle` | "Occasions that plausibly do arrive: any substantive edit… a scheduled sweep…" | "The positive claims about which occasions work are **untested**" |

This is not a request to hedge the bodies into mush. It is that the corpus has an unstated **layer contract** — body = assertion, Strength = qualifier — and its own view theory says that contract must not exist. Two repairs are available and they are different decisions:

1. **Declare `Strength & grounds` non-droppable** (a projection constraint, recorded where notation law is recorded — the same move [[multi-renderer-constraints]] argues for), or
2. **Move the qualifier into the body** at the clause it qualifies, which is what [[strength-ladders]] item 5 (clause-grain tagging) already identifies as the finest live grain in the estate.

The corpus currently does neither and has not noticed the question. I found no row, gap, or Working Note anywhere naming it.

*Why I think the authors cannot see this:* every one of them reads segments whole. The defect only exists for a reader who receives a projection, and no projection of this corpus has ever been built.

---

## F2 · The corpus surveying five incompatible strength vocabularies is running an undeclared sixth — Rung: **Tested** · **PARTIALLY KNOWN** (see Phase-2)

[[strength-ladders]] is a survey whose finding is that five incompatible strength vocabularies live in the family and that *"the adjudication is owed, not made here."* [[verbal-label-calibration]] adds that no ladder in the estate has ever been calibrated for read-spread. [[epistemic-axes]] names strength / support-kind / ceiling as separable.

Meanwhile **this corpus marks strength in free-running prose with no declared vocabulary at all.** No segment carries a `status:` or `strength:` frontmatter field; the type field is the only machine-readable epistemic marker, and it carries kind, not strength.

Counting the opening declaration of each `## Strength & grounds` section across the 79-segment snapshot, the words actually in use include at minimum: *Heuristic · Heuristic/engineering · Held at heuristic strength · Formulation · A design formulation · A practice claim · A proposal · Stipulative · Testimonial · Empirical (for the specimen) · Discussion · Lived/steward-experiential · robust qualitative · methodological observation.* Fourteen-plus surface forms, with compound variants (`**Heuristic; (A) and (E) carry lived evidence…**`, `**Empirical for the specimen, heuristic for the mechanism**`, `**Heuristic, mixed register**`) that no instrument can parse and no reader can rank. Roughly a third of segments open that section with no strength word at all.

This is the exact defect [[strength-ladders]] itself charges against udon-needs — *"free-text drift observed in its own `stage` values"* — reproduced by the segment's own corpus, one grain finer.

**The honest counter-argument, which I attempted before writing this:** ONTOLOGY §`segment[expected-shape]` says *"Epistemic marking style (section vs frontmatter vs inline) is deliberately open — see the ch. 1 gap."* So the *placement* is knowingly unsettled. But placement being open does not license the **vocabulary** being unbounded, and nothing in ONTOLOGY, the OUTLINE, or any segment declares the word-set. The ch. 1 gap row names "machine-readable qualifier placement" — not "which words mean what." The two are separable and only one is marked. Under [[absence-as-structure]], the unmarked half is a hole nobody can see.

*Consequence, concretely:* ch. 11's generator is supposed to ask a deployment "what epistemic fields do you need" and derive the answer from [[epistemic-axes]] + [[strength-ladders]] ([[epistemic-axes]] Working Notes says exactly this). The generator's own reference instance cannot answer the question about itself.

---

## F3 · Nine segments are outside the corpus by the corpus's own definition of membership, and their dependents are illegal — Rung: **Tested** · **KNOWN SINCE 2026-08-05, AND GROWING** (see Phase-2)

ONTOLOGY §`outline[columns]`/`column[tag]`: the Tag column *"is the identity column and the assembly manifest — build/assembly walks this column in outline order."* Membership is the row, not the file.

At snapshot, **nine files had no row**; at session end, ten (`ls plan/last-adhoc-src`, cross-checked against the Tag column of `plan/OUTLINE.md`):

```
basename-manifestation-survey   identity-regime-archaeology   instrument-failure-census
navigation-relocation-specimen  rename-survival-and-families  routing-sop-anatomy
sidecar-ubiquity-census         terminology-store-anatomy     terminology-substore
tracking-layer-census
```

Three consequences, in increasing severity:

1. **They will not assemble.** An outline-order walk emits none of them.
2. **They are load-bearing appendix specimens.** `identity-regime-archaeology` is a declared `depends:` of `slug-identity`; `instrument-failure-census` of `corpus-instruments`; `sidecar-ubiquity-census` and `terminology-store-anatomy` of drafted chapter claims. So drafted claims stand on material the manifest does not contain.
3. **ONTOLOGY calls this an error in terms**, not a warning: *"a dep on a slug with no row anywhere is an error."* `check-plan` reports it under the softer label "orphan," which routes the reader to a housekeeping frame rather than to the corpus's own error condition. That is the mislabelling [[influx-queues]] (B) describes — *"a linter that reports 'lint failed' for both you wrote invalid frontmatter and nothing here can tell whether this dependency edge is genuine."*

Note the shape: [[absence-as-structure]] anticipates *rows-with-no-file* and calls it "the degenerate form" that "break[s] dependency resolution silently." The corpus has the **dual** — files-with-no-row — which the norm does not name and which breaks *assembly* rather than resolution. Worth a clause in that norm.

**Proposed repair (not applied):** rows for all ten, in the Appendices part, before the next batch. The four newest are `survey`/`obs` grade and belong there by [[appendix-placement]].

---

## F4 · Five dangling wikilinks, in a segment that also violates the appendix-naming law it was drafted under — Rung: **Tested**

`plan/last-adhoc-src/routing-sop-anatomy.md`, line 9 (its own summary line), names five consumers:

> *…as the material behind [[strengthen-before-routing]], [[findings-are-not-a-worklist]], [[adjudicator-not-confirmer]], [[honest-incompleteness-discharge]] and [[rule-grounds-and-posture]].*

**None of the five exists as a file, and none has an outline row.** Re-verified twice from clean shells (`grep -oh '\[\[...\]\]' last-adhoc-src/*.md` differenced against filenames ∪ Tag column). By ONTOLOGY these are five errors in one line.

This is the more interesting half: the same segment breaks **ONTOLOGY §`parallel-batch-law`** bullet 3 — *"An appendix segment names its subject, never its consumers — the citation direction lives in the citing claims and in the outline rows."* The law was earned 2026-08-05; this segment was written 2026-08-06 and inverts it in its first sentence, naming five consumers that do not exist.

That conjunction is itself a datum for [[proxy-discipline]] and for ONTOLOGY's own `readable-claims-tripwire` note (*"names don't fire; tripwires do"*). The estate now has a second specimen of a norm violated by an agent that had it available — and this one is stronger than the first, because the violation is not a lapse of attention but a *forward-citation habit* that reads as helpful.

Sixth dangling reference at snapshot: `terminology-store-anatomy` → `[[terminology-substore]]`, since resolved to a file by the concurrent writer, and thereby converted into an F3 orphan rather than fixed.

---

## F5 · The corpus's own delete-test reversion broke three provenance pointers, silently, exactly as its flagship specimen predicts — Rung: **Tested** · **CLASS KNOWN, THESE INSTANCES NEW** (see Phase-2)

ONTOLOGY §`segment[integrated-is-disposable]` records that `.integrated/` was *"emptied back to honest state"* on 2026-08-06 after the steward's delete-test failed the batch. Verified: `find plan/.integrated -type f` returns **0**; only empty directories remain.

But three segments' Working Notes still point into it as though material were there:

- `name-collision-across-stores.md:59` — *"Intake artifact behind this segment, **now at** `plan/.integrated/vera/00-INDEX.md`"*
- `substrate-independence.md:36` — *"The gather inventory **has moved to** `plan/.integrated/`"*
- `basename-manifestation-survey.md:36` — cites `plan/.integrated/` as a live specimen of the sidecar form

The first two are dangling. The third is worse in kind: it offers an empty directory as evidence in a survey's table of live instances.

**Why this matters more than three broken links.** [[provenance-rot-specimen]] is this corpus's strongest empirical leg — 106/109 path-anchored spans broken by one mundane commit, 18/18 slug references surviving. [[identities-over-locations]] converts it into law: *"References carry identities, not locations… no paths in references."* ONTOLOGY repeats it: *"[[slug]] everywhere — outline, segments, sidecars. No paths in references."*

The corpus then wrote intra-corpus paths into segment bodies, performed a mundane reorganization, and broke them silently in under 24 hours. This is a **first-party replication of its own headline specimen**, obtained for free, and it is stronger evidence than the original in one respect: the authors here had read and written the norm.

I checked whether the norm's own escape clause covers it. [[identities-over-locations]] Working Notes: *"sometimes a path is the honest anchor (external trees this corpus doesn't govern, one-off scratch). The norm governs references the corpus owns and expects to survive its own housekeeping."* `plan/.integrated/` is inside the corpus and was moved by the corpus's own housekeeping. **The escape clause does not reach it.** These are exactly the references the norm governs.

*Proposed repair:* the pattern the specimen's own unchosen repair names — drop the path, name the disposition (*"the intake artifact behind this segment has been archived; its content is superseded by the table above"*). No path is needed for any of the three.

*Second-order value:* this converts [[identities-over-locations]]'s Working-Note wish (*"Second specimens are cheap if wanted"*) from an aspiration into a landed second specimen, and gives [[substrate-independence]] the in-estate replication it asks for. I would rank writing it up above fixing the links.

---

## F6 · Declared-and-unexercised law is now this corpus's own dominant pattern — Rung: **Hypothesis** (three specimens, mechanism argued, frequency unmeasured) · **ESTATE LEG KNOWN (TODO R4); IN-CORPUS SPECIMENS AND THE MINTING-WINDOW HYPOTHESIS NEW**

[[absence-as-structure]] Working Notes: *"Whether declared-but-unexercised law generally (gap laws with no gaps, ladders that never fire) is one phenomenon deserving its own segment: two specimens exist; a third would earn the row."*

**The third, fourth, and fifth are in this corpus.**

1. **`checked` has never fired.** ONTOLOGY declares the state vocabulary *proposed · drafted · checked*. Across 87 outline rows: 61 `drafted`, 26 `proposed`, **0 `checked`** (`grep -cE '^\| *checked' OUTLINE.md` → 0). [[ladder-never-fired]] measured exactly this shape in vivarium and ASF — *"the terminus stages that Gate 4 was written for never appear"* — and [[state-flags-not-gates]] draws the design lesson from it. The corpus that carries both has reproduced the occupancy profile within one day of the flag vocabulary being minted. Its own dogfooding note (`state-flags-not-gates`, Working Notes: *"Outline `state` column on this plan instance is itself a flag vocabulary… dogfooding"*) claims the dogfood without reporting the result.

2. **[[priming-discipline]] is law here and has no instrument here.** The norm requires that a corpus wanting independent judgment *"must build the surface"* — an audit-safe entry point — and names ASF's generated `README-auditor.md` as the shipped form. This corpus declares no auditor-safe surface, no priming-heavy markings, and no read order. My own priming protection came from a human instruction in a brief, which dies with the session — the failure mode [[orientation-gate]] names (*"a pass scoped to a context lifetime"*) applied to priming instead of orientation. Nothing in the corpus would have stopped me opening `CHANGELOG.un` first.

3. **[[observable-crossings]] is drafted and no crossing has been recorded.** Its own Working Notes propose the in-house trial. Between drafting and this audit, the corpus performed the single largest crossing in its history — the `.integrated/` reversion — and recorded it in ONTOLOGY prose rather than as an event.

The pattern I would name, if the row is earned: **a norm minted from a specimen is at its lowest enforcement probability in the days right after minting**, because the specimen supplies the *understanding* and no act supplies the *trigger*. That is [[role-activation]]'s mechanism applied to law rather than to guidance, and it predicts something checkable: violations should cluster in the window immediately following a norm's authoring. Four of the five violations in this report (F3, F4, F5, and item 1 here) are within 24 hours of their norm's minting. That is a real, cheap, unrun measurement — `git log` supplies both dates.

---

## F7 · The telos names an instrument that under-reports by ~31% — Rung: **Tested**

`README.md` §Telos: *"The corpus itself reports how done it is (`plan/bin/check-plan`; the proposed-row count; `plan/TODO.md`'s undischarged entries) — read those numbers instead of narrating past them."*

The proposed-row count is **26**. Of those, **8 rows have drafted segment files sitting on disk** — `check-plan` itself reports them (`⚠ [[atom]] has a segment file but row still 'proposed'`, and seven more). So the number the telos designates as ground truth against narrative is wrong by 31% in the *deflationary* direction, while the orphan count (F3) is wrong by ten in the inflationary direction on a different axis.

I want to be careful about the register here, because this estate treats deflation as a failure mode too: **a number that under-reports doneness is not "safely conservative."** [[proxy-discipline]]'s degenerate trap is *"trusting a corpus's shape… as if shape were state"*, and it cuts both ways — an under-full-looking outline invites redundant re-drafting exactly as an over-full one invites false completion. Two agents in the last batch nearly collided on new-slug creation (ONTOLOGY §`parallel-batch-law`); a `proposed` row whose file already exists is the same hazard one step earlier.

Relatedly, [[corpus-instruments]] asserts as a live gap: *"no checker compares a view's hand-authored state column against the record's own declaration."* In this corpus **`check-plan` does exactly that** — the eight warnings above are its output. The claim is scoped to the estate and sourced to a 2026-07-07 ASF review, so it is not false as written, but it is stale in the one corpus the reader is standing in, and the segment does not say so. Cheap repair; the more interesting version is that the checker exists and its output is not being read, which is [[stage-denorm-zero-drift]]'s own closing question (*"A check nobody reads and a check that always passes look identical from the outside"*) landing on the corpus that wrote it.

---

## F8 · An unmigrated first-generation cohort, invisible to every instrument — Rung: **Tested**

Nine to ten segments predate a format shift and were never brought forward. They are identifiable by four independent markers that co-occur:

| Marker | Count (snapshot) | Files |
|---|---|---|
| `**Summary.**` bold lead instead of `*italic summary*` | 4 | `coupling-confounding`, `cousin-store-lineage`, `type-vocabulary-locality`, `view-genres` |
| `## Strength and scope` instead of `## Strength & grounds` | 3 | `coupling-confounding`, `type-vocabulary-locality`, `view-genres` |
| Curly quotes `’ “` | 8 | `ladder-never-fired`, `lost-update-hazard`, `orientation-gate`, `outline-skipping-failure`, `state-flags-not-gates`, `type-vocabulary-locality`, `write-safety`, `view-genres` |
| No `depends:` at all (ONTOLOGY requires explicit `[]`) | 9 | the `check-plan` warning list |

`coupling-confounding` additionally uses `\[ … \]` and `\( … \)` math delimiters. The Archema tree's own cascading law (`asf/CLAUDE.md` §6) is unambiguous that this applies to *every* markdown file written to disk, memos and READMEs included — *"the instant text goes to a file it is false"* that Unicode/non-`$` math is acceptable. This corpus inherits that law by cascade and has one live violation.

The finding is not the typography. It is that **four correlated markers of an unmigrated cohort exist and no instrument sees any of them**, while `check-plan` reports the fifth (missing `depends:`) as nine unrelated warnings rather than as one cohort. [[corpus-instruments]]' second failure mode — *"faithful reporting of worthless signal… it trains readers to discount instruments generally"* — is live: nine identical warnings read as noise, and the cohort they identify is the actual finding.

---

## F9 · Gate-2 probes: where the prose sounds derived and is not — Rung: **Pattern**

The corpus passes this probe unusually well; [[discussion-probes]] is clearly operating on its authors. Three residues:

- **[[atom-grain-parallelism]] (A), second clause.** *"the practical ceiling on parallel work in a corpus is closer to its record count than to anything about the agents."* Probe 1 (does it follow from the laid foundation?) — no; nothing upstream bounds parallelism. Probe 2 (hypothesis with falsifier?) — the Working Notes offer a test for the *doubling* claim, not for the ceiling claim. Probe 3 — it is a quantitative-shaped assertion whose evidence (five agents, no body collisions, one batch) is consistent with almost any ceiling. Repair per the segment's own three options: label it a hypothesis and name what would falsify it (a corpus where parallel work saturated well below record count), or cut the clause. The doubling claim survives on its own.

- **[[atom]], internal tension.** *"an arrangement missing any one of them is doing something else"* (§The definition) versus *"the four properties are stated as separable so that a deployment can be described honestly as having three of them… it fails if every instance turns out to be either all four or none"* (§Strength & grounds). These are contradictory: the first excludes three-property arrangements from the category, the second makes describing them the definition's reason for existing. A `def`-type segment carrying an internal contradiction about its own extension is the highest-leverage repair in ch. 1, because everything downstream imports it.

- **[[name-collision-across-stores]] versus the tripwire.** ONTOLOGY §`readable-claims-tripwire`, minted 2026-08-05 with the explicit reason that *"names don't fire; tripwires do"*, sets a mechanical rule: *"any table, and any number derived from counting more than two things, is appendix-grade."* This segment carries two tables and a four-referent count in a chapter-level `emp` claim, and argues its way out in Working Notes (*"a companion segment carrying that would be ceremony rather than structure"*). I think the argument is **correct on the merits** — the split would be ceremony here. But note what happened: the estate's first mechanical tripwire was converted back into a judgment call by the first segment to meet it, by an author who cited the tripwire's purpose against its text. If that is acceptable, the tripwire is a heuristic and should say so; if it is not, this segment is a violation. The corpus should pick, because the *reason the tripwire exists* is that judgment calls did not hold.

---

## F10 · Grounding specimens under-declared in `depends:` — Rung: **Tested**

ONTOLOGY defines `depends:` as *"definitional imports, logical antecedents, **grounding specimens**."* At snapshot, **eleven segments cite an appendix specimen inside their own `## Strength & grounds` section — i.e. as the thing the claim stands on — without declaring it**:

| Segment | Cited as grounds | Declared `depends:` |
|---|---|---|
| `state-flags-not-gates` | `ladder-never-fired` | *none* |
| `write-safety` | `lost-update-hazard` | *none* |
| `orientation-gate` | `outline-skipping-failure` | *none* |
| `atom` | `cousin-store-lineage`, `type-vocabulary-locality` | *none* |
| `derivation-not-templates` | `strength-ladders` | *none* |
| `name-collision-across-stores` | `provenance-rot-specimen` | `collision-staleness-detection` |
| `substrate-independence` | `provenance-rot-specimen` | `collision-staleness-detection`, `slug-identity` |
| `orientation-scaffold` | `outline-skipping-failure` | `orientation-gate`, `priming-discipline` |
| `working-notes-sidecar` | `working-notes-deluge` | `integration-metabolism`, `sidecar-ubiquity-census` |
| `outline-as-organizing-principle` | `build-forced-commitments`, `view-genres` | `atom`, `navigation-relocation-specimen` |
| `history-layer` | `tracking-layer-census` | (three others) |

The direction is consistent and diagnostic: **claim→claim edges are declared; claim→specimen edges are dropped.** That is the exact edge class [[appendix-placement]] says the DAG must record — *"The dependency DAG still records the truth (the claim `depends` on the appendix segment)"* — and it is the edge class [[dependency-order-tension]]'s whole three-part contract operates on. With these edges missing, the accepted-inversion machinery that segment recommends would have nothing to accept: the appendix rows would not register as inversions at all, because the corpus does not know the claims depend on them.

`check-plan` cannot see this (it validates that declared deps resolve, not that real deps are declared). A cheap instrument exists: flag any segment whose `## Strength & grounds` cites a `[[slug]]` absent from its `depends:`. That is the query that produced this table.

---

## What I attempted to strengthen and could not

Per the estate's strengthen-before-soften discipline, two candidate findings I tried to make stick and could not:

- **"The corpus over-claims first-hand verification."** I re-ran the load-bearing counts against live sources and the opposite is true — see below. I withdraw the finding rather than soften it into a hedge.
- **"[[verbal-label-calibration]] over-reaches in transferring probability-word spread to invented tier words."** The segment already states this as its own leg-3 weakness, names it *"inference from shape"*, calls it *"the leg to attack"*, and records the method failure that nearly shipped a wrong version. There is nothing left to find; the segment audited itself harder than I could. Registering it here as **checked and clean** so a later auditor does not re-spend on it.

---

## Verification of the corpus's numbers (brief, because it is not where an auditor is irreplaceable)

I re-derived the load-bearing counts from live trees before being told this ground was already covered. Reporting the result compactly, since a null result here is itself useful calibration on the drafting-side verifiers:

**Exact matches, re-run first-hand 2026-08-06:** ASF stage occupancy 200/23/18/2/0/0 ([[ladder-never-fired]]); working-notes volume 175,122 / 465,953 words = 27.3%, gold-block words 101,440 = 57.9%, `def-chronica` 106 lines / 67 Working Notes ([[working-notes-deluge]]); vivarium 115/115 `draft` and 115/115 sidecar; `DECISIONS.decision-log.udon` 1,697 lines / 160 blocks / 556 KB (the two segments citing lines and KB are consistent, not in conflict); comproprium 57 segments = 12+10+35, harvest 51 = 43+8, 13 loose, `.integrated/` empty, 87 `:from` fields of which 79 dangle, full type distribution including the stray `:type pattern`, and `bin/check-corpus` re-run returning **3/109 located · 106 fail · 18/18 outline slug refs** ([[provenance-rot-specimen]], [[queue-typing-specimen]], [[write-semantics-declaration]]); refs stores 188/13 and 97/70; relata 2,277 entries / 219 verifications / 15 calibrations / 300 spool / `quarantine/` present; terminology 176 entries / 149 dirs / 160 events / `LEXICON.md` 384 lines; vivarium LEXICON 106 terms = 74+9+23, `SUPERSEDED.md` 94 lines; 02-tst-core 72 files / 43 `old-tst-*` / 29 live / 25 `label:` / 15 `older-tag:`; tracking-layer line counts 132/537/442/27/1,417/221/44; routing SOP line counts 407/355/702/74/56 and tag frequencies 11/10/8/4; `asf/terminology/entries/proprium-mapping.md` still points at `~/src/firmatum/`, which does not exist.

**Three small drifts, all low-consequence, listed for completeness:**

1. `strength-ladders` §1 says ASF's `status:` scalar is *"deeply exercised (~285 live segments)"*. Live segments carrying `status:` = **243**. 287 is the file count *including* `old-*` archaeology, which the same corpus's `identity-regime-archaeology` and `instrument-failure-census` correctly treat as not-live. The figure appears to have been taken from an unfiltered listing.
2. `working-notes-deluge` reports **122** segments carrying an `### Incidental audit gold` block; the live count is **121** as a heading (a 122nd file mentions the phrase in prose). Notable only because every *word* count in that table re-derives to the digit while this one does not — consistent with 122 having been inherited from the cited 2026-07-07 review rather than re-run, inside a table headed "counted first-hand."
3. `navigation-relocation-specimen` §Method says *"171 files in `01-aat-core/src/`"*. There are 170 files and one directory (`img/`). Two sibling segments say 170. An intra-corpus collision of the kind [[collision-staleness-detection]] exists to surface — and it did not surface, because nothing compares counts across segments.

`instrument-failure-census` reports **3** `old-*` exemption sites in `bin/lint-outline`; I find two executable sites (lines 236, 260) plus one docstring (line 250). Whether a docstring is a "site" is a fair reading either way; noting it only so a later reader is not surprised.

---

## Phase-2 triangulation — what the corpus already knew

*Read after every finding above was written: `HANDOFF.md`, `CHANGELOG.un`, `plan/TODO.md`, `plan/reports/`. Per the brief, already-known findings are marked, not deleted — a finding the corpus knows and has not fixed is a different fact from a finding it has not seen, and both are useful.*

| Finding | Status after triangulation |
|---|---|
| **F1** layer contract | **New.** The nearest prior is TODO **A11** (SP-5's proposed unqualified reader's-path preamble, whose self-named risk is *"pressure to write a tight Reader's Path may push authors toward overclaiming-for-concision"*) — but A11 is *prospective*, about a layer not yet built, and is marked discharged into `plain-language-briefs`. F1 is the *already-realized* form in the existing two layers. **R16** covers the adjacent projection-declaration point for Working Notes, not for the honesty layer. Nothing names the body/`Strength & grounds` contract. |
| **F2** strength vocabulary | **Partially known.** TODO **R14** records the lead session's own fork-friction report — *"Strength & grounds smears evidence-kind and strength"* — as a fourth instance of axis-collapse, undischarged. That is the *axis* half. The half I found and R14 does not carry: the **word-set itself is unbounded free text** (14+ surface forms, ~⅓ of segments declaring no strength word at all), which is a vocabulary defect rather than an axis defect and has a different repair. Worth adding to R14 rather than opening a new row. |
| **F3** orphan segments | **Known since 2026-08-05 — and it has grown.** `plan/reports/verify-tribunal-readability-2026-08-05.md` flagged `tribunal-strand-survey` and `rationale-capture-survey` as *"Ship, but **has no outline row**"*. Both now have rows; meanwhile the population went from 2 to **10**. This is the most actionable item in the report precisely *because* it was known: a flagged defect that a verifier caught, that was fixed for the two named instances, and that then recurred five-fold in the next drafting pass. That is a process finding, not a housekeeping one — the repair fixed the instances and not the generator. |
| **F4** five dangling refs | **New.** No mention anywhere. |
| **F5** `.integrated/` pointer rot | **Class known; these three instances new.** `CHANGELOG.un` §2026-08-05 already records, as a live pattern result, *"a path-rot recurrence during the session documenting path rot."* The three pointers I found were broken *later*, by the 2026-08-06 delete-test reversion, so they are a second recurrence — which strengthens the class claim rather than duplicating it. My recommendation stands and sharpens: this is now **two independent first-party replications in two days**, and writing it up is worth more than the link fixes. |
| **F6** unexercised law | **Estate leg known.** TODO **R4** states it well — *"the estate writes absence law it does not run"* — and calls it *"[[ladder-never-fired]]'s shape a second time,"* flagged as a candidate obs row. New here: the three *in-corpus* specimens (`checked` at zero, no auditor-safe surface, no crossing events), and the **minting-window hypothesis** with its `git log` test. If the sibling obs segment R4 proposes gets drafted, these are its cheapest additional specimens. |
| **F7** proposed-row count | **New**, and it bites a surface written the same day: the telos block naming that count as ground truth is the 2026-08-06 reset pass (`CHANGELOG.un` §`reset-worthiness-pass`). Note also that `HANDOFF.md` reports *"52 segments drafted, 34 rows proposed"* while `check-plan` reports 61 and 26 — the handoff surface is stale against the instrument it tells you to read instead of it. |
| **F8** unmigrated cohort | **New.** |
| **F9** Gate-2 residues | **New.** |
| **F10** undeclared specimen deps | **New.** `HANDOFF.md` notes *"Grok owes nine `depends:` declarations"*, which is the *missing-field* half (my F8 marker 4). The *wrong-content* half — declared deps that omit the segment's actual grounding specimen — is not tracked. |
| Three count drifts (285 / 122 / 171) | **New**; no mention in TODO or any verify report. |

**One thing the triangulation changed in my ranking.** F3 moves up. I wrote it as a structural defect; it is better read as evidence that this corpus's repair loop fixes *instances* and not *causes* — the same shape as F6's minting-window and as [[asked-and-answered]]'s repeat-billing argument, seen from the repair side rather than the flag side. The corpus has a segment for the flag half and none for this half.

**And one caution about my own report.** Everything above was found by a reader who had the whole corpus in context at once. That is exactly the condition [[experiential-reading]] argues produces *unfalsifiable* judgments — I could not have been surprised by any of it, because the later material was already present when I formed the earlier reads. Cross-segment findings are the class batching is supposed to favour, and nine of my ten are cross-segment, which is consistent. But it is also what you would expect if batching simply *only* produces this class. A serial reader auditing the same corpus would be the control this corpus has never run, and running it would settle [[experiential-reading]]'s own stated open question far better than any argument in the segment.

---

## Coverage statement

**Audited deeply (read whole, judged, cross-checked):** all 79 segments in the 00:47 snapshot, plus `README.md`, `plan/OUTLINE.md`, `plan/ONTOLOGY.un` (whole), `plan/bin/check-plan` output, and the ASF de-novo SOP §§1–5.5.

**Verified first-hand against live sources:** every numeric claim listed in the verification section — roughly 60 distinct counts across `asf/`, `vivarium/`, `comproprium/`, `neurips/refs`, `logos/refs`, `relata`, and `02-tst-core`. Two checkers re-run (`comproprium/bin/check-corpus`, `plan/bin/check-plan`).

**Sampled, not exhausted:** quotation verification. I located the ASF `FORMAT.md` and neurips `AUTHORING.md` quotations that several segments turn on, but did not character-verify every quoted span in `build-forced-commitments`, `routing-sop-anatomy`, `rationale-capture-survey`, or `tribunal-strand-survey`. `rationale-capture-survey`'s external bibliography (SEURAT, MADR, ISO 42010, Buckingham Shum) I did **not** touch at all — the segment marks it inherited-at-a-remove with sweep-verified/training-recall registers preserved, which is the right posture, but it means an unverified external layer sits under `decision-records` and `tribunal-record`.

**Not touched:** `PRACTICA.un`; `plan/TODO.md` (so every "Discharges TODO entry Rnn/Nnn" claim in the segments is **unverified** — that is a real hole, and it is the cheapest remaining audit surface); `plan/INFLUX/` (16 files); `plan/.archive/`; the twelve segments written during the session; the `depends:` DAG's acyclicity beyond `check-plan`'s own assertion; and — per the priming constraint — `CHANGELOG.un`, `HANDOFF.md`, and all prior reports.

**Structurally unavailable to me:** prediction-delta and causal-gap findings, for the reason given in the process confession. If exposition gaps matter, this corpus still needs a serial reader.

---

*Standing by for follow-up. The two findings I would most want to discuss are F1 (the layer contract) and F6 (the minting-window hypothesis) — F1 because a repair decision is genuinely owed and I do not think it is mine to make, and F6 because it is one `git log` away from being measured rather than hypothesized.*
