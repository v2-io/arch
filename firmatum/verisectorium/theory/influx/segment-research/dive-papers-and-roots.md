# Dive — paper corpora + the estate-wide root-file population (2026-08-10)

*One of four parallel survey dives. Patch: paper-shaped corpora (vivarium core, neurips, behavioral-floor, causal-language, logos) + the population of verisectorium-important project-root files across `~/src/`. Everything below was read live this session (paths absolute); the influx's dangling "Notes copy" column in `instances/deployments.md` was verified dangling (`cousin-stores/`, `paper-projects/`, `planned/` no longer exist in the influx — `ls influx/` 2026-08-10) and all specimens here come from live trees, not gather copies. asf-lineage, comproprium, and udon-tree depth is left to the peers on those patches; where my census touched their ground I record location + one-line only.*

**Reading ledger (whole or head-read live):** vivarium `core/OUTLINE.md`, `FORMAT.md` §§0–1, `core/src/def-nomos.md` + `disc-check-the-ladder.md` (whole), `ETHICS.md` head, `DECISIONS.decision-log.udon` header block; neurips 01 `src/` listing + `04-main-results.md` head, `adjudicated/{OUTLINE.md,residue.md,claims/convexity-gap…,discussion/implications…}`, `refs/{entries,verifications}` specimens, root `LOG.md`, `OUTLINE-STRATEGY.md`; `neurips-reviews-responses/processing-flow.md`; causal-language `EPISTEMIC.md` (§§0–1 whole) + `LOG.md` head; behavioral-floor `src/` + `00-meta.md`, `03-two-distinctions.md` heads; logos tree + paper-01 `SUBMISSION.md`, `DRAFT-GUIDE.md`, `FEEDBACK.md` heads; root files: asf `PRACTICA.md`, `JOSEPH-TODO.md`, `NOTATION.md`, `CHANGELOG.md`/`LOG.md` heads, `msc/meta-process-review-2026-07-07/DECISIONS-CENSUS.md`; ops `STATUS.md`, `PRACTICA.md`; `_self/PRACTICA.md`; `arch/notes/NORMS.md`. Census sweep: `find` over ~/src depth 3 + arch/neurips/papers depth 5 for DECISIONS/PRACTICA/STATUS/CHANGELOG/LOG/NOTATION/NORMS/FORMAT/EPISTEMIC/TODO/PROCESS-MAP/OUTLINE/ETHICS.

---

## Part I — The misfits (lead with the yield)

Things that do **not** fit the epistemic map's current carve (truth-apt / choices / directives / references / decisions), or that break an assumption the carve silently makes.

### M1. Epistemic status is per-clause in the wild, not per-segment

`~/src/arch/vivarium/core/src/disc-check-the-ladder.md` — one segment, frontmatter `status: robust-qualitative`, but its Epistemic Status section splits by clause:

> **Max attainable: robust-qualitative** for the general method (FE §1); **exact** for the water-world instance (FE §2). Authority split: FE §2 is `DECISIONS[water-world-is-the-promise-not-the-bug]` (`:by joseph`, decided). The general ladder-before-modern-Earth guard is `DECISIONS[check-the-ladder-not-modern-earth]` (`:by claude`, decided) — a codified generalisation of Joseph's correction, not a Joseph-tagged method stamp.

So: status, max, **and decider** all vary *within* a segment, clause by clause, carried in prose. The frontmatter field is a projection (apparently the min). Any taxonomy that types the segment as the unit of epistemic state needs a story for sub-segment granularity — the wild already does it.

### M2. The negative canon: residue / do-not-use as a first-class kind

`~/src/neurips/01-tragedy-confident-agent/adjudicated/residue.md` — a ledger of **refuted formulations**, each with Status (REFUTED / "overclaimed relative to disk provenance"), Where (which red-team check, which test, which file), Why, and **Replace with:** (pointer into `claims/`). Header, verbatim:

> *Refuted formulations + why + where refuted. Do not resurrect in claims, strategy, or response text. Integration-is-replacement: these stay here as the do-not-use list; they are not softened ghosts in the claims tree.*

This is truth-apt content whose *function* is prohibition — the history layer made load-bearing and forward-facing. It rhymes with ISO term-status **forbidden** (terminology-survey) but for claims. The carve has nowhere to put "true statement that X is false, kept specifically so nobody reuses X." One entry even carries a graded allowance: *"Allowed use: [TESTED] kill-direction + mechanism as [HYPOTHESIS] for the full law; not claim-grade 'iff.'"* — a per-formulation **usage license**, which is neither status nor authority as the map has them.

### M3. The adjudicated rung dialect: composable, multiplied, directional

`adjudicated/OUTLINE.md` (paper 01) rungs, verbatim from its claims table: `[PROVED]`, `[TESTED]`, `[PROVED]+[TESTED]`, `[PROVED]+[TESTED] 3×`, `[TESTED]+[JUDGMENT]`, `[PROVED]/[TESTED]`, and in a claim body: **"Rung: [TESTED, rigorous in the negative direction] (3× verified)"**. Three axes the epistemic map lacks:

- **Composability** — a claim's support is a *bag* of independent support-kinds, not one rung (this is the trio-ratified support-kind axis surfacing in the wild, uncoordinated).
- **Multiplicity** — `3×` verified; verification count as first-class.
- **Directionality** — rigor can hold in one direction of a biconditional/bound and not the other (residue entry 3 is exactly a kill-direction-only finding).

Each claim file also carries a one-line **"Verification chain:"** (spike §7 exp4 · recon re-run · red-team R8 · TRUTHIFIED.md cleared) — provenance-as-chain, the transmission-lock idea, live in a paper corpus.

### M4. Epistemic verdicts in *filenames*

`~/src/neurips/01-tragedy-confident-agent/src/`: appendix files `C-establishes.md`, `C-not-establishes.md`, `C-not-run.md` — the identity slot itself carries the epistemic verdict (what the experiments establish / don't / weren't run). Also `05-not-shown.md`. The map treats identity (slug) and status as separate; here they've fused. (deployments.md already flags `NN-` prefixes as violating slug-purity; verdict-bearing basenames are a stronger version of the same tension.)

### M5. Audit-priming safety as a document axis

asf `PRACTICA.md` (and echoed in its file-org): *"PRACTICA itself is **auditor-safe** — readable during de-novo audits — but linked entries into TODO / PROPOSALS / CHANGELOG are priming-heavy and should be skipped…"*. Documents carry a **read-safety / priming classification** orthogonal to truth, authority, and process — it's about what reading the file does to a *reader's* epistemics (de-novo audit contamination). Nothing in the current carve or its orthogonal axes holds this. (Global memory has `audit-safe-vs-priming` as method; here it is a standing per-file property.)

### M6. Explicit non-authority: references that disclaim themselves

asf `NOTATION.md` frontmatter `type: reference`, then immediately:

> **This file is a lagging index, not the arbiter (drift caveat).** … **It is not authoritative.** When this index and a segment's own definition/derivation disagree, the [segment wins]

The ref-kind's `current`-freshness axis is here **self-declared negatively** — a standing arbitration rule ("on conflict, I lose") rather than a status value. Same move at vivarium `ETHICS.md`, opposite polarity: *"where the body and Appendix A disagree, **Appendix A wins and the body is the bug**."* → Candidate axis: **conflict-precedence declaration** (who wins on disagreement), attested at both poles, distinct from authority-rung.

### M7. Authority-by-reference (imported bindingness)

vivarium `ETHICS.md` Standing Moratorium Imperative: *"the Archema charter makes it binding across the program, **so its authority does not derive from this document**."* A directive whose ruled-ness is *hosted* locally but *minted* elsewhere. The template-maintenance axis (verisectorium/other-project/target-maintained) is adjacent but not identical — this is about where the authority lives, not who edits the file.

### M8. Confidence about the concern, not the claim

logos paper-01 `FEEDBACK.md` reading convention: each item has *Concern, Why it matters, Possible resolutions, **My confidence** (high/medium/low — where the concern itself lands), Status (open / in-progress / addressed / deferred-to-B-F1-or-similar)*. Second-order epistemics — graded credence attached to an objection — plus a status vocabulary (addressed/deferred-to-X) that is neither the evidence ladder nor the decided ladder. The FEEDBACK kind (working-TODO-cum-weakness-ledger) recurs across all logos papers.

### M9. Process-level epistemics as a document (confirmed, richer than billed)

`~/src/causal-language/EPISTEMIC.md` is not merely "epistemics at process level" — it is a genre: **standing adversarial self-review**, *"the strongest honest critique … from a perspective that would try to break the project"*, ordered most-damning-first, each item flagged testable-or-not + what-would-address-it + status, with **dated resolution narratives** appended in place (§1 runs open → spike-executed → "Resolution (2026-05-15) — this critique is met…", including a historical note where a verify-before-archive pass first flagged wrongly and was reversed). It also carries **read-order directives** ("If you're a reviewer…read this file before any of the others"). One file spanning truth-apt content, directives, and decisions-with-history. The carve can describe its *contents* but not the *kind*.

### M10. Attention/orientation marks on outline rows

vivarium `core/OUTLINE.md`: ★ rows = "current high-importance set (~top 25 by recent/structural rank; refresh with `bin/orient-rank --mark-outline`) … **Most quiz items are drawn from these** … Stars move when importance moves." A tool-refreshed **salience axis** on rows — not status, not stage, not authority; it drives an *orientation gate with a quiz*. Also: outline sections partition by **subject**, with epistemic weight explicitly banned from placement (norm backed by two named 2026-07-24 thrash specimens) — except one weight-defined section, the **Appendix — retired paths** (the negative canon again, M2's cousin, resolved by an is-about vs how-held distinction).

---

## Part II — Paper-corpus specimens

### II.1 vivarium `core/` (~115 segments, `~/src/arch/vivarium/core/src/`)

- **Kinds by prefix observed:** `def-`, `detail-` (18!), `disc-`, `form-`, `scope-`, `post-`, `norm-`, `sketch-`, `obs-`… `detail-` is the bulk kind — "extended operational or technical material supporting other claims."
- **Frontmatter skeleton** (def-nomos.md): `slug / type / status / stage / depends:` — exactly asf's, and FORMAT.md §0 says so with a maintenance stance worth quoting for the template-maintenance axis: the vocabularies are *"recapitulated here from asf/doc/sop/format.sop.md (read 2026-07-13) … **the currently adopted set, not a settled schema**"* — adoption with a read-date stamp, plus coining rules ("same concept → ASF's word"; test = "a paper drawing on both has to read coherently").
- **Body anatomy** (consistent across both segments read): `# Title` → one-line thesis → **Formal Expression** (numbered clauses) → **Epistemic Status** (Max attainable + authority citations + stage restated + *"Not claimed here:"* negative-scope block) → **Discussion** → **Working Notes** (incl. "Ice once this lands" dedup obligations). The **"Not claimed here"** block is a standing anti-claim register — scope honesty structural, per segment.
- **Authority citations inline:** `` `DECISIONS[slug]`, `:by joseph`, decided `` — segments cite the decision ledger by key with decider; also `#lexicon/term/X` cites with the term's own status (*"still `:status open` (do not treat that entry as settled law)"* — a segment warning readers about the epistemic state of a *reference it makes*).
- FORMAT.md scope split is itself a datum: §§1–4 bind segments; **§§5–6 bind every file in the repo** ("'It is only a working document' is not an exemption") — rule-scope as an axis (segment-law vs repo-law).

### II.2 neurips (3 papers + adjudicated overlay + refs + reviews flow)

- **Base paper corpus** (`01…/src/`): flat files, `NN-topic.md` main sections (with sub-splits like `02-agent-state` / `02-mismatch-dynamics` sharing a number), `A-…`–`E-…` appendices, `references.md`, `checklist.tex`. **No frontmatter, no ladder** — Obsidian `^anchor` block-refs + wikilinks carry cross-reference identity; theorem-environment callouts (`> [!theorem] … ^thm-lmi-sufficient`) carry claim-grain identity *inside* sections. So the base papers have claim-grain addressing without claim-grain files. Papers 02/03 additionally have `src/re/` (restructure layer); 01's restructure happened in place.
- **Adjudicated overlay** (`adjudicated/`): `OUTLINE.md` (self-described *"Changelog-style index … Not the paper outline — the post-verification home map"*), `claims/` (12 slug files), `residue.md` (M2), `discussion/`, `strategy.md`, `REVISION-OUTLINE.md`. Claim-file skeleton: `# Claim: <thesis>` → header line `**Paper:** · **Home:** · **Rung:** · **Verification chain:**` → Statement / Mechanism / Relation to the paper / Sources. Note **no YAML frontmatter** — the metadata is a bolded header line. Discussion files carry relocation provenance (*"Relocated from umbrella draft … source file left in place as pointer target until coordinator retires it; **this copy is the home**"* — explicit home-vs-pointer during migration).
- **refs** (`~/src/neurips/refs/`): `entries/<bibkey>.yml` (plain bib fields, no status) + **`verifications/<bibkey>/<timestamp>-<verifier>-<criterion>.md`** with frontmatter `key / criterion (claim-supported) / verifier / outcome (verified) / timestamp` + a prose paragraph tying the theorem to the paper's invocation. **The event-layer pattern is live here** — the epistemic map's note that the event layer "shipped only in relata so far" is wrong; neurips/refs (the *first* generation of the lineage) already stores per-criterion verification events as append-only files, with status projected at emit time. Also `deny-list.yml` (anonymization enforcement — a directive artifact in a reference store).
- **Reviews processing-flow** (`~/src/neurips-reviews-responses/processing-flow.md`): opens *"Here it is, as you typed it:"* followed by Joseph's verbatim checklist in a code fence — a process map whose **provenance form is verbatim-preservation of the steward's own typing**, then presumably interpreted around it. Notable content: reviewers get *implicit profiles* (expertise / surprisal-area / noise-area / disposition / register) — epistemic modeling of an external audience as a deliverable kind.
- **Root LOG.md**: *"Append-only. Reverse-chronological … Never edit prior entries — LOG is the permanent record."* Its 2026-07-29 entry is a specimen of **artifact pinning**: frozen `submitted-neurips-2026.pdf` under *"a stem no manifest owns, so `bin/build` cannot overwrite it"*, after verifying builds are not byte-reproducible — the frozen-copy-vs-regenerable distinction as an explicit epistemic act ("The frozen copy is the artifact; the tag is how you know what it came from").
- `OUTLINE-STRATEGY.md`: a **method document keyed to external exemplars** (two Jin papers vendored under `spikes/paper_structure/`) — guidance whose authority is "modeled after canonical papers," a fourth authority flavor (neither ruled, ratified, nor proposed: *emulated*).

### II.3 behavioral-floor & causal-language (paper-lite)

- **behavioral-floor** (`~/src/behavioral-floor/src/`): `00-meta.md` … `09-conclusion.md`. Section files have **no frontmatter at all** (03-two-distinctions.md starts at `# Two Distinctions`); all document-level metadata is **concentrated in `00-meta.md`'s frontmatter** (title, subtitle, `submission: "AIES 2026 — Submission 1465 …"`, topics). Pattern: *meta-section-file as the corpus's single metadata carrier*. No status ladder anywhere in src/ (looked at file heads + census). Root: `TODO.md` only — minimal root population.
- **causal-language** (`~/src/causal-language/`): sections `paper/src/NN-*`; root population is the richest of the paper-lites: `EPISTEMIC.md` (M9), `LOG.md` (*"frozen record of finished work"* with an explicit live/frozen doc partition naming README / PROGRESS / OPEN-WORK / HEADLINE-FINDINGS / EPISTEMIC as the live set), OPEN-WORK, HEADLINE-FINDINGS. Segment-grain epistemics: none; document-ecology epistemics: strong. The *division of epistemic labor across root files* (findings vs adversarial review vs open work vs frozen history) is itself a taxonomy datum — the ladder exists at the level of the *file system*, not the section.

### II.4 logos (`~/src/arch/logos/`)

Per-paper venue scaffold, consistent across papers 01/03: `CFP.md` · `STRATEGY.md` (repo-level too) · `DRAFT-GUIDE.md` (*"section-by-section argumentative moves … reviewer-objection table … named-concept canonization plan"* — includes a **Tier 1/Tier 2 canonization** scheme: which coined terms this paper mints as the canonical citation) · `SUBMISSION.md` (venue mechanics, anonymization protocol with verbatim publisher guidance) · `FEEDBACK.md` (M8) · `LOG.md` · `meta.<stem>.md` (build-stem metadata) · `OUT.<stem>.md` (manifest) · anon/deanon artifact pairs. The scaffold is a **role-differentiated document suite** where each file's charter names its siblings and its layer ("For the why-this-paper-this-way layer see STRATEGY; for mechanics see SUBMISSION…") — mutual-charter cross-headers as the navigation mechanism. Kinds here not in the carve: CFP (external constraint mirror), canonization plan (naming-authority strategy), anon/deanon **artifact-pair discipline** with deny-list lint.

## Part III — The root-file population (census)

Sweep: `find ~/src` depth 3 + arch/papers depth 5, name-matched. `.archive`, `_ref`, node_modules pruned. Udon/comproprium/asf-interior detail left to those patches.

| Class | Live specimens (owner-project) | Dialect / vocab notes |
|---|---|---|
| **DECISIONS ledger** | vivarium `DECISIONS.decision-log.udon`; udon `v2/DECISIONS.md`, `v2/paths/DECISIONS.ud`; verisectorium `template/DECISIONS.ud`; asf: **none at root** (gap named in epistemic-map ⚠ and confirmed — asf routes decisions via JOSEPH-TODO + msc briefs instead) | vivarium's is the anatomy specimen: udon comment-header charter; fast-append interleave-free (one atomic write, no wrapping parent); `:supersedes [a b]` on the header line so `grep '^|decision\['` yields supersession chains; **editing allowed** — *"WORKING DOCUMENT, not an immutable ledger"*; an explicitly **undecided retention policy** recorded in the header with a dated Joseph status ("status quo until it actually hurts") |
| **PRACTICA navigator** | asf `PRACTICA.md`; ops `PRACTICA.md`; `_self/PRACTICA.md`; verisectorium `PRACTICA.ud`, template copy; firmatum `practica/` (a whole member project) | Three *very* different dialects for one name: asf = rich prose navigator ("top levels of the strategy DAG") with 🌟/⭐ priority marks, cycle history inline, auditor-safety note (M5); ops = bare Obsidian outline with `:luc_refresh_cw:` icon vocabulary; `_self` = personal todo with ‼/⭑ marks. Name convergence, schema divergence — the population is *nominal*, not structural |
| **STATUS** | ops `STATUS.md`; md-press `STATUS.md`; asf `audits/STATUS.md`; `_gems/dry-rbs/STATUS.md`; neurips spike STATUS | ops charter: *"Live operational view … Refresh in place … **do not turn into a planning artifact**"* + `**As of:** 2026-08-02 (paper-decision delta only — full STATUS refresh still owed)` — **partial-refresh honesty stamps**; interior headers still say 2026-05-28 (visible staleness gradient within one live doc) |
| **CHANGELOG / LOG split** | asf (CHANGELOG forward from 2026-04-24, LOG frozen at that date — both carry the identical GUC-renumbering warning banner keyed to a git tag); neurips LOG (append-only, never-edit); causal-language LOG (frozen record, live set enumerated); logos LOG per-paper + root; `_core/nexum/LOG.md`; rowan/gems CHANGELOGs (conventional software dialect) | The **epoch-freeze** pattern (one file frozen at a date, successor forward-going) and the **tag-anchored translation banner** (old-numbering table + `pre-guc-rename-2026-05-09`) are history-layer devices the taxonomy should name |
| **NOTATION** | asf `NOTATION.md` (only one estate-wide) | `type: reference` + self-disclaimed non-authority (M6) |
| **NORMS** | `arch/notes/NORMS.md` (only one) | Draft programme-wide conventions (BASENAME manifestation table incl. deprecated/transitional/compliant forms — a *migration-status vocabulary on file-forms themselves*; sidecar convention `.BASENAME-side`; explicitly interim: "much of this will eventually be part of UDON proper") |
| **Valve (JOSEPH-TODO-shaped)** | asf `JOSEPH-TODO.md`; asf `msc/decision-briefs-2026-07-15.md`; ops STATUS partially serves this | Admission criteria stated as law: only **irreversible / publication-or-authoring-voice / cross-project blast-radius / "did this actually come from Joseph?"**; everything else default-and-proceed. Queue rendered as one-line-per-decision tiers (quick nods / policy / theory & naming / strategic). The 2026-07-07 `DECISIONS-CENSUS.md` is the meta-specimen: decision-routing diagnosed as *"not one process beside the others, it is the **parent**"*, with a Part B "free wins" section — the decided/undecided boundary audited as a population |
| **ETHICS** | vivarium (binding, charter-derived, precedence rule M7); `arch/ETHICS.md`; ops `ETHICS.md`; `_core/nexum/ETHICS.md` | The one root-file class carrying **ruled** authority in the wild |
| **EPISTEMIC (process)** | causal-language only (M9); embeddings has `docs/epistemic-*.md` essays (content, not process registers) | |
| **FORMAT** | vivarium `FORMAT.md`; comproprium `FORMAT.md`; udon `v2/theory/FORMAT.md`; asf's lives as `doc/sop/format.sop.md` (sop-store dialect); ops `papers/format.md` | Same kind, four homes/dialects; vivarium's names its own adoption provenance + expected divergence |
| **Process maps** | asf `msc/…/PROCESS-MAP-v0.udon`; `neurips-reviews-responses/processing-flow.md` (verbatim-Joseph form) | |
| **FEEDBACK** | logos per-paper (M8); asf-side equivalents live as audits/spikes instead | |
| **Navigator-of-navigators** | asf PRACTICA's sister-file paragraph; logos scaffold cross-headers; causal-language LOG's live-set enumeration | Root files increasingly carry **charters naming their siblings' roles** — the ecology is self-describing at the file level, nowhere formalized |

**Population-level observations.** (1) *Nominal, not structural, convergence* — PRACTICA/STATUS/LOG names travel between projects while schema, register, and even medium (md vs udon vs Obsidian-icon outline) diverge; the taxonomy should expect name-classes with per-project dialects, exactly like the `status:` zoo. (2) The **maintenance-stance sentence** is the most consistent structural element across all classes: nearly every specimen opens with a self-charter ("append-only, never edit" / "refresh in place, do not become planning" / "working document, not immutable ledger" / "lagging index, not arbiter" / "short list, admission criteria are…"). That sentence encodes, per file: mutability policy, refresh cadence, authority stance, and anti-drift warnings — it is the root-file analog of frontmatter, and probably the thing a verisectorium template should make first-class. (3) Absences: no DECISIONS at asf root, no NOTATION outside asf, no NORMS outside arch/notes, no ETHICS in any pure paper project, behavioral-floor nearly bare — root-file richness tracks *multi-agent traffic density*, not project importance.

## Part IV — Axis candidates the map doesn't have yet (summary)

1. **Granularity of epistemic state** — segment vs clause (M1); who projects the frontmatter value from the clauses.
2. **Support composition** — bag of support-kinds + multiplicity + direction (M3); the trio-ratified support-kind axis is *already convergently live* in neurips-adjudicated, in a different surface syntax. Strong evidence for that axis; also evidence the *strength ladder* is the part nobody reinvents.
3. **Usage license / prohibition** — do-not-use, allowed-use-at-lower-rung, forbidden (M2 + terminology's *forbidden*); negative canon as a kind (residue.md, vivarium's retired-paths appendix).
4. **Conflict precedence** — on disagreement, who wins (M6, both polarities).
5. **Authority locus vs authority host** — imported bindingness (M7); adoption-with-read-date (vivarium FORMAT).
6. **Reader-effect class** — auditor-safe vs priming-heavy (M5).
7. **Salience/orientation** — ★ marks, tool-refreshed, quiz-coupled (M10); asf's 🌟/⭐ priority dialect is the same axis hand-maintained.
8. **Artifact fixity** — frozen-copy vs regenerable vs pinned-by-unowned-stem (neurips LOG); epoch-freeze + translation-banner history devices.
9. **Maintenance-stance charter** as the root-file frontmatter-equivalent (Part III obs. 2).
10. **Second-order confidence** — credence on concerns/objections, reviewer profiles (M8, reviews flow).
11. **Emulated authority** — guidance normed to external exemplars (OUTLINE-STRATEGY).

## Adjacent finds & brief feedback

- **Two errors in current influx substrate:** (a) epistemic-map's "event layer … shipped only in relata so far" — falsified; `neurips/refs/verifications/` is a live per-criterion event store, and it's the lineage *root*, so the pattern is four generations old, not new. (b) deployments.md's dangling Notes-copy column confirmed dangling (dirs absent from influx); its row for neurips claims `src/re/` for all papers — paper 01 has no `src/re/` (restructure was in place); 02/03 do.
- **ops/STATUS.md still says `~/src/archema-io/asf/`** — a stale-path specimen of the 2026-07-22 rename left to heal on demand (also a live illustration of ref-freshness decay for the taxonomy).
- The **base neurips papers show claim-grain addressing without claim-grain files** (`^thm-…` anchors inside sections) — a middle form between paper-lite and full verisectorium that the instance spectrum doesn't currently name.
- Brief feedback: the assignment's carve ("paper corpora + roots") turned out to have a productive seam — the paper-lites' epistemics live *in the root population* (causal-language), so the two threads are one phenomenon at different densities. Worth preserving that framing in synthesis.

*— dive agent, 2026-08-10. Staying on the line for synthesis follow-ups.*
