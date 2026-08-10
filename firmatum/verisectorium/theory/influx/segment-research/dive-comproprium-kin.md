# Dive: comproprium and kin — segment kinds in the wild (2026-08-10)

*Dive agent, comproprium neighborhood, one of four parallel survey dives. Everything below was read at the live tree (`~/src/arch/proprium/comproprium/`, git parent `~/src/arch/proprium`, last touched 2026-08-01 "Reorganize various ingest queues" a40825f). Specimen-grain: kind as its corpus names it, verbatim anatomy, register vocabulary actually used, maintainer, and fit/misfit against the epistemic map's Axis-0 carve (truth-apt / choices / directives / references / decisions). I read README.md, FORMAT.md, GATHERING.md, the-chain.md, by-trigger.outline.udon, bin/check-corpus, .gitignore whole, plus one full specimen per kind and directory listings of all 109 .udon files. The influx `vera/` ennaos strands are noted for existence only (reserved for the Joseph+lead joint session).*

## 0. Orientation corrections to the influx picture

Two facts the influx catalog has slightly wrong:

- **"precept / practice / exemplum / verum" is a three-kind system, not four.** `vera/` *holds* the precepts — a verum IS a precept ("what is true, in the truest form currently reachable," README table). The deployments.md row "precept / practice / exemplum" and the glimpse's "precepts, practices, exempla, vera" both double-count.
- **The corpus's own docs dangle like deployments.md's "Notes copy" column does.** README/FORMAT/segments all reference `.to-integrate/` (and README documents `.integrated/` as "sources whose content now lives in segments"), but on disk the staging dir is now `INGEST/` (renamed in a40825f, 2026-08-01) and `.integrated/` is **empty**. Every segment's `:from .to-integrate/...` locator is therefore path-dangling — the checker's quote-fidelity pass presumably fails or was not re-run since the rename. Same class of drift, worth a taxonomy note: *provenance locators are references with freshness, and nothing currently re-verifies them on reorganization.*

## 1. The kinds, at specimen grain

The organizing law (README, "Three source directories, because they have three different failure modes") is itself the most taxonomy-relevant find in the tree: **kinds are separated by failure mode and adjudication instrument, not by content-type.**

| dir | holds | fails by | adjudicated against |
|---|---|---|---|
| `vera/` | precepts | being false / overclaimed | evidence and derivation |
| `praxes/` | practices | **not firing** | whether it demonstrably fired, in use |
| `exempla/` | true accounts | inaccuracy to what happened | the primary record |

All segments are `.udon` (UDON dialect), filename = slug, slug carries type prefix (`ver-`/`prx-`/`exm-`) so `#slug` refs self-locate. Maintainer throughout: Joseph (steward, ruler of names — "Ruled `exempla` by Joseph, 2026-07-30") + harvesting agent instances; nothing ratified — README banner: *"proof of concept with aspirations to be disciplined in a way worthy of becoming critical lived/living infrastructure."*

### 1a. verum (precept) — `vera/*.udon`, 12 files

Specimen: `vera/ver-earned-confidence-is-quiet.udon`, header verbatim:

```
|segment[ver-earned-confidence-is-quiet] :type observation :status discussion-grade :stage draft
  :max-attainable empirical
  :tags [confidence verification register]
  :from .to-integrate/methodology-seeds-2.md
```

Sections: `|title |summary |formal-expression |epistemic-status |discussion |working-notes` — ASF's claim cadence, ported unchanged (FORMAT D3). Register vocabulary: ASF-style ladder (`discussion-grade`, `:max-attainable empirical`), `:type` from a local set `principle · mechanism · observation · definition · hypothesis · discussion` (FORMAT D2 — with the travel rule: "where a claim would be well-typed in ASF's vocabulary, use ASF's word instead, so it can travel"). **Fit:** cleanly truth-apt; the evidence ladder applies. Notable refinement the map lacks: FORMAT D2's *mechanism vs principle* split — "a mechanism claims how something works; a principle says what follows for conduct" — a truth-apt kind whose content is conduct-implying, sitting between truth-apt and directive.

### 1b. praxis (practice) — `praxes/*.udon`, 10 files

Specimen: `praxes/prx-untruncated-primary.udon`:

```
|segment[prx-untruncated-primary] :type probe :status fired-once :stage draft
  :depends [ver-discount-is-unquestioning]
  :from .to-integrate/methodology-seeds-2.md#appendix-a-14
```

Sections: `|title |when |act |why |evidence |working-notes` (FORMAT D3: `|when` "the moment it fires, stated as a moment, not a disposition"; `|why` cites the `#ver-` mechanism it defeats). `:type` set: `probe · mitigation · practice · protocol`. **Status ladder is its own, and it is not an evidence ladder:** `fired · fired-once · proposed · failed-to-fire · retired` (FORMAT D4) — "about *demonstrated firing*, not about whether the practice is a good idea. A practice that is obviously correct and has never fired is `proposed`."

**Misfit #1 for the carve.** A praxis is directive-shaped but its epistemic status is neither truth-tier nor authority-rung: it is a **fired-in-use ladder** — closest cousin in the estate is the method-evidence-tiers "support-kind defined by its repair," but here the whole status axis is *transmission evidence*. The map's ③ authority register (proposed/ratified/ruled) does not describe it: nobody ratifies a probe; it earns status by firing. And FORMAT's own open list sharpens it further: **fired-for-its-author vs fired-for-an-inheritor are different events** ("a probe firing for its author is evidence it is well-aimed; a probe that fires for an inheritor is evidence it *transmits* — and only the second is what a corpus is for"), currently unrepresentable — `prx-untruncated-primary` has one of each and is forced to `fired-once`. A candidate sixth Axis-0 column, or a third register: **efficacy** (adjudicated-by-firing), distinct from evidence and authority.

### 1c. exemplum (account) — `exempla/*.udon`, 35 integrated + 51 staged in `INGEST/harvest-a|b`

Five sub-kinds (FORMAT D2): `account · exchange · testimony · quotation · demonstration`. Quotation is "the smallest exemplum, and the highest-volume kind." Specimen (`exempla/exm-truth-over-self.udon`):

```
|segment[exm-truth-over-self] :type quotation :status attested :stage draft
  :speaker joseph
  :said 2026-07-30T12:54
  :family truth-over-self
  :tags [truth-over-self confidence register]
  :from .to-integrate/methodology-seeds-dialog-quotes.md#12:54

  |title … |quote … |occasion … |about … |should-come-to-mind-when … |see …
```

Status ladder (FORMAT D4): **`attested · reconstructed · secondhand`** — a *provenance-fidelity* ladder. "Nothing higher than `reconstructed` is available to an instance working from a transcript, however complete. A `quotation` extracted from a primary is `attested` — the speaker attests it, not the extractor." Harvest-b specimens carry `:date-confidence weak` explicitly (e.g. `exm-call-me-dad-not-papa.udon`, `:said 2025-09-20 :date-confidence weak`) — a date-epistemics field the map has nowhere to put.

**Misfit #2 — the central one, as the brief predicted.** An exemplum is a record-of-what-happened: not truth-apt in the claim sense (its adjudicator is *the primary record*, not evidence-for-a-proposition), not a choice, directive, reference, or decision. Its ladder measures **witness position** (who stands between the reader and the event), not evidence strength. And its lifecycle inverts the theory's own law: **FORMAT D5 explicitly diverges from integration-is-replacement** — "an `exempla/` segment is **not revised** when understanding improves. Corrections and later readings chain beneath it… a claim's job is to be presently true, an account's job is to be what happened." (One exception: factual errors about the record — wrong timestamp, misquote — corrected in place, "because the account was never attesting to them.") The carve needs a column something like **attestations / records** with its own ladder (witness position × date-confidence) and its own lifecycle (append-only-with-chained-commentary).

**Misfit #2a — `demonstration` as epistemically load-bearing positivity.** FORMAT D2: a demonstration exists because "knowing a thing **has been done** by someone at one's own capability level changes what is in the action space, which an instruction cannot do" (cites `#ver-demonstrated-is-in-the-action-space`); carries `:demonstrates <virtue>`. An account kind whose *function* is directive-adjacent (expands action space) while its *adjudication* is exemplum-standard (named actor, locatable primary). Kinds can straddle the carve on function-vs-adjudication; the carve should probably key on adjudication instrument, per this corpus's example.

**Misfit #2b — restatement families.** `:family` links restatements of one idea across occasions; "two segments are duplicates only when they resolve to the same span in the same primary" (FORMAT D4). Identity semantics unlike any claim corpus: same-proposition ≠ same-segment; a renderer may "pick one, rotate, or synthesize." No map axis carries within-family multiplicity.

### 1d. Trigger fields — a retrieval register, not an epistemic one

The load-bearing exemplum field is `|should-come-to-mind-when`, since split (FORMAT D3a) into **`|before-acting`** (pending-act pattern) and **`|on-reading`** (input pattern — noticed to be "a different mechanism," arguably more reliable: "they match incoming context in the same pass that reads it"). Rules: triggers must be *semantic*, observer-adjudicable ("could an observer disagree about whether it fired?"); **empty is a finding, not a hole** ("an empty pair means: this belongs in an onboarding view, not in a system prompt"; "the weakest lines in the first harvest are the fabricated ones"). This is a whole segment-state dimension orthogonal to everything on the map: *when should this segment arrive in a mind* — deployment-salience metadata, adjudicated by firing like a praxis.

### 1e. Views — outline as generated projection, plus a narrative view

- `by-trigger.outline.udon` — header verbatim: `|outline[by-trigger] :view triggers :orders exempla / :generated-from exempla/*.udon / :note This file orders and references. It introduces nothing`. **Generated, regenerated rather than edited** — further along than most estate outlines (matches the theory's outline-as-view claim, already realized here). Its `|entry` rows re-carry `|trigger` lines — projection of segment fields into the view.
- `the-chain.md` — a second view kind the taxonomy should not miss: a **derivation-ordered narrative view**. Self-description: "It orders and it references; it introduces nothing… Atomizing it keeps every conclusion and destroys the derivation, which is the same operation as summarizing and has the same loss." Connective tissue is explicitly non-authoritative: "the italic line before each span is connective tissue and carries no claim of its own; if it disagrees with the span, the span is right." A view whose *ordering itself is the content* (the click-order of a day's derivation) — outlines-only-order holds, but "order" here carries meaning beyond pedagogy.

### 1f. Root-file population (comproprium's contribution to dive-target 4)

`README.md` (front door + the failure-mode law + priorities-as-steward-context), `FORMAT.md` (**divergences-only format file** — "ported by reference, not restated… restating it here would fork it"; chain: comproprium ← `udon/v2/theory/FORMAT.md` ← `asf/FORMAT.md`), `GATHERING.md` (a **standing harvest brief** — a root-file kind I've not seen named elsewhere: a durable peer-voice delegation brief with tool-epistemics inside it, including the anti-priming move "What I am deliberately not giving you: a list of concepts to search for"), `bin/check-corpus` (the falsifier; its docstring is register-relevant: "What it deliberately does not check: whether a claim is true, whether a status is honest, or whether a practice ever fired. Those are not mechanical and pretending otherwise would make this a proxy wearing verification's clothes"), `.gitignore` as **publication membrane**: deliberately-untracked-but-real corpus members ("the distinction is publication, not validity"; religious-imagery constraint carried with its reason; "bare filenames match at any depth, so an entry survives a segment moving"). **Publication-status (tracked/untracked-by-design) is a segment state no ladder on the map carries**, and README §"What is on disk but not in git" exists precisely so a completeness audit doesn't misread the absence.

## 2. Kin, briefly

- **`proprium/` parent** (README): comproprium = communal half of the *lived* register seat; register adjectives derived/argued/tried/**lived**. The lived register is where the exempla misfit comes from — worth noting that Axis-0 was carved from derived-register corpora.
- **`proprium/corporeum/`** — implementations ("code that makes it happen"), `research/harness/`; INGEST only, no segment corpus yet. `proprium/INGEST/{msc-from-harness, old-firmatum}` — staging, pre-pattern.
- **Tribunal strand:** influx `tribunal/` holds notes only (`discovery-internal-architecture-2025-09-21.md`, `zoetica-tribunal-xml-template.md`, `synthesis-scatter-2026-08-05.md`); living home per deployments.md §F is `_ref/epistemic_tribunal` product code — not surveyed (read-only cousin; the XML-template + bounded-context YAML registers are already flagged in terminology-survey).
- **influx `vera/` (ennaos strands):** exists — `00-INDEX.md`, `ennaos-gemini-chat.md`, `ennaos-vera-architecture-final-specification.md`, `synthesis-scatter-2026-08-05.md`. **Not read**; reserved for the Joseph+lead joint session. Note only: an "ennaos vera architecture" spec implies a second, possibly divergent, formalization of the verum kind — the joint session may want the comproprium D2/D4 vocabulary above in hand for comparison.
- **Citation cross-check:** the theory's `norm-truth-above-self.md` cites ~25 comproprium paths; spot-checked several (`exempla/exm-truth-over-self.udon`, `vera/ver-earned-confidence-is-quiet.udon`, `exempla/exm-hedge-licensed-unchecked-claim.udon`, harvest members) — all resolve at the live tree. One footnote cites `exempla/exm-opus-5-root-weakness.udon` — present. The `INGEST/harvest-a` members it cites via other segments were not exhaustively verified.

## 3. Axes the epistemic map does not yet have (the yield, condensed)

1. **Adjudication instrument as the kind-separator** — the corpus's own law, and a better first principle for Axis 0 than content-shape: kinds separate when they *fail differently and route to different repairs* (FORMAT D1 explicitly invokes source §9's two-repairs test).
2. **Efficacy register (fired-ladder)** for practices: `fired/fired-once/proposed/failed-to-fire/retired`, with the open author-vs-inheritor split (aim vs *transmission*). Neither evidence nor authority.
3. **Attestation register (witness-position ladder)** for records: `attested/reconstructed/secondhand`, + `:date-confidence`, + append-only-with-chained-commentary lifecycle (an explicit, argued divergence from integration-is-replacement).
4. **Retrieval/salience state**: `|before-acting` / `|on-reading` triggers, legitimately-empty semantics — *when should this arrive in a mind*, adjudicated by observable firing.
5. **Publication membrane**: on-disk-but-untracked as a deliberate state, distinct from validity and from process-state flags.
6. **Restatement-family identity** (`:family`, `:speaker`, `:said`): duplicates = same-span-same-primary only; families rendered stochastically/synthetically.
7. **Divergence-only rule files**: FORMAT.md-as-delta with a named upstream chain — a reference kind whose freshness axis is "does the upstream still say what I diverge from."
8. **Verbatim-fidelity mechanics as epistemics**: transport artifacts preserved because normalization breaks the checker's ability to locate spans (FORMAT D6) — verifiability constraining formatting.

## 4. Adjacent finds / feedback

- **The `.to-integrate/`→`INGEST/` rename left the corpus's docs and every segment's `:from` locator dangling** (§0). Cheap fix, but also a live specimen for the taxonomy: locator freshness is an unowned state.
- **`.integrated/` is empty while README describes it as populated-by-process** — either integration never moved sources there or the rename ate it; a completeness audit would misread this without the git history (a40825f).
- **README's "Priorities" section carries its own incident**: an earlier version presented Joseph's priorities as *the* ordering — "That was the corpus's own #9 — a steward's shared thinking rewritten in canon voice — committed while writing the segment about it." A ready-made specimen for any authority-register segment: steward-context vs steward-ruling as distinct speech-acts.
- **GATHERING.md is a genre find**: a durable, versioned, peer-voice harvest brief with embedded tool-epistemics and an explicit refusal to prime ("if I hand you my concept list you will find my concepts"). If root-type files get a taxonomy row for "standing briefs," this is the type specimen.
- Brief feedback: the pointer "precepts, practices/praxes, exempla, vera, plus newer kinds" mildly misled (vera *are* the precepts; the genuinely newer kinds are the *sub*-kinds — quotation, demonstration, testimony — and the two-field trigger split). Otherwise the brief's prediction was exactly right: comproprium is where the carve breaks, and it breaks at exempla.
