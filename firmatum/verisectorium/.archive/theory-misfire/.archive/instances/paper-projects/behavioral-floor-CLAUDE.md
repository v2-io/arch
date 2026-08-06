<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: behavioral-floor/CLAUDE.md (src/NN-*.md one-section-per-file paper; ordered concat build)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/behavioral-floor/CLAUDE.md
  Do not edit here expecting to update the live original.
-->

# CLAUDE.md — behavioral-floor (AIES 2026 paper)

*Project-level CLAUDE.md, 2026-05-14. Loaded automatically by Claude Code when
the working tree is `~/src/behavioral-floor/`. The global
`~/.claude/CLAUDE.md` index and its disposition apply; this file is the
project layer on top of it. Read it once at session start, then route into
the substrate.*

---

## What this project is

A focused, time-bounded paper for **AIES 2026** ("AAAI/ACM Conference on AI,
Ethics, and Society"). Title (set 2026-05-14 after an AIES-corpus-calibrated
title pass; prior working title was the cryptic "Encoded, Deployed,
Inherited, Fresh" taxonomy-list, replaced because AIES rewards a stated
stance over a withheld one):

> **What Behavioral Evaluation of LLM Agents Cannot Certify: An
> Architectural Floor on Evidence for Causal Reasoning**

The paper is **methodology-critique with formal mechanism**. It does not run
new experiments; it re-reads the audit-literature cluster on LLM causal
reasoning (Causal Parrots, CLadder, CausalProbe-2024, CounterBench, SycEval,
Yang et al.) under two distinctions (inherited vs. fresh Pearl Level 2;
encoded vs. deployed content), then identifies an architectural mechanism
(Class 3 / Coupled goal-update topology) that makes the deployment side
operationally intractable for behavioral evaluation. The contribution is the
**synthesis with bridge proof**, not the constituent pieces.

It is the *complementary* half of an empirical companion paper at
`~/src/causal-language/` (representation-level probes on frozen pretrained
embeddings). The two papers are designed to cohere as "two consequences of
one principled discipline" (asymmetric-comprehension). They MUST NOT
duplicate each other's territory — this paper is the behavioral-floor
argument; the companion is the structural-probe diagnostic.

The investigation that established the GO!! verdict lives at
`notes/01-investigation-state.md`; the substrate-orientation brief lives at
`ORIENTATION.md`. **Read both before touching the draft** — they carry
context that does not survive in the LaTeX.

## Where the work lives

```
behavioral-floor/
├── ORIENTATION.md                    — peer-to-peer brief from 2026-05-13 investigation handoff
├── notes/
│   └── 01-investigation-state.md     — verdict, bridge verification, prior-art audit, page plan, risks
├── src/
│   ├── 00-meta.md                    — YAML frontmatter (title, abstract, bibliography)
│   ├── 01-introduction.md            — body sections, one #-heading per file
│   ├── 02-related-work.md
│   ├── 03-two-distinctions.md         (3.1 inherited-vs-fresh; 3.2 encoded-vs-deployed)
│   ├── 04-architectural-floor.md      (4.1 GUC class; 4.2 Class-3 connectivity;
│   │                                   4.3 (C2★) operationalizable-sufficiency
│   │                                   gap — Claim 1, the bridge; 4.4 the floor)
│   ├── 05-audit-literature-reread.md
│   ├── 06-methodological-prescription.md
│   ├── 07-limitations.md
│   ├── 08-ethical-policy-implications.md
│   └── 09-conclusion.md
├── common/
│   ├── aies2026-template.tex         — pandoc template (AAAI submission preamble)
│   ├── aaai2026.sty                  — official style (copied from AuthorKit26/)
│   └── aaai2026.bst                  — official bib style (copied from AuthorKit26/)
├── bin/
│   └── build                         — relata-emit + anonymize + pandoc + pdflatex orchestrator
├── paper.pdf                         — last successful build (rendered from src/NN-*.md)
├── AuthorKit26/                      — official AIES author kit (pristine; do not edit)
└── TODO.md                           — drafting backlog + companion-coordination items
```

### Build pipeline (`bin/build`)

```
src/NN-*.md (sorted)  ── pandoc ──→  .build/paper/paper.tex
~/src/relata/         ── emit  ──→   refs.relata.bib
                                     anonymize (applicable_anonymity: true → "Anonymous")
                                                              ↓
                                                         paper.bib
                                                              ↓
                                                     pdflatex × 3 + bibtex
                                                              ↓
                                                         paper.pdf  (→ repo root)
```

`bin/build` globs `src/[0-9][0-9]-*.md` in sorted order and feeds them to
pandoc as a single argv. Pandoc concatenates them and uses the first
file's YAML frontmatter as document metadata. To add a new section,
drop in `src/04a-some-section.md` (it sorts between `04-` and `05-`)
and rebuild — no manifest file to maintain.

- **Default (`./bin/build`):** blind-review build. Entries with
  `applicable_anonymity: true` in their relata YAML get rewritten to
  author `Anonymous` + a fixed note; identifying fields stripped.
- **`./bin/build --camera-ready`:** real-author build (post-acceptance or
  trusted-distribution archive). Skips bib-entry anonymization, and as of
  2026-05-22 also post-processes `paper.tex` to (a) drop the
  `[submission]` option from `\usepackage[submission]{aaai2026}` (which
  hardcodes "Anonymous submission" in `\@maketitle` regardless of
  `\author{}` — see `common/aaai2026.sty:75`) and (b) replace the
  template's empty `\author{}` + `\affiliations{}` pair with Joseph's
  canonical block (`Joseph A. Wecker / Independent Researcher, Vis
  Veritatis (v2.io), Lehi, USA / joseph.wecker@v2.io`). The author info
  lives **in `bin/build` (`camera_author_block`)**, not in
  `src/00-meta.md`, on purpose: meta.md is read by both anon and
  camera-ready builds, so the template's
  `$if(author)$$author$$else$Anonymous Submission$endif$` fallback is what
  guarantees safe blind-review defaults. Update the author info in
  `bin/build` if it changes.
- **`./bin/build --clean`:** rebuild from scratch (`rm -rf .build/`).

Citations in `src/paper.md` are raw `\citep{key}` / `\citet{key}` —
pandoc passes them through unchanged, and relata's cite-scanner reads
the same form. The bibkey is the canonical relata key (e.g.,
`wecker-2026-llm-hallucinate-bound`); blind-review rendering happens at
bib-emit time, not at cite-time.

Reusable for the companion paper at `~/src/causal-language/`: copy
`bin/build` and `common/` over (or symlink), drop in `src/paper.md`,
same pipeline produces the AIES submission.

## Project-specific disposition

> "If I listened every time an agent, well-informed and well-intentioned,
> told me to hold off and not submit a paper this time 'round, I would still
> be at zero." — Joseph, recorded in `ORIENTATION.md`

This observation is load-bearing on disposition. Concretely it means:

- **Counseling-delay is the default failure mode for agents on this paper.**
  The temptation is to recommend "this might fit better at a later venue,"
  "wait for the empirical companion to mature first," "the bridge proof
  needs more substrate work before publishing." That counsel is never free.
  It is the easy move that costs nothing to the agent issuing it and
  everything to the publication track.
- **If the strengthening attempt produces a stronger paper, ship the
  stronger paper.** If the strengthening attempt produces an honest no-go
  with verifiable proof (bridge breaks under specific counterexample;
  prior-art surprise that occupies the claimed synthesis space), say so
  with the reproducible reasoning a third party can verify. Anything
  between those two states is not a verdict yet.
- **Don't drift the deadline.** AIES 2026: abstract due **May 14** (today
  as of the file timestamp), full paper due **May 21**. The abstract is a
  shape-commitment refined into the full paper; the full paper is the load
  bearing artifact.

The global `strengthen-before-soften` principle fires here at every audit
step: when you find a wobble in the draft, first try to strengthen the
underlying claim or its presentation; only if the strengthening attempt
honestly fails do you fall back to softening or scope-narrowing. Effort is
a false constraint on this paper as on the rest of the program.

## The substantive thesis (one paragraph for context)

Behavioral evaluations of LLM-based agents (the audit-literature cluster)
are *systematically bounded*: they can measure deployment-faithfulness
variation but cannot, by construction, certify causal-access status. The
audit-literature contradictions resolve once two distinctions are made
explicit:

1. **Inherited vs. fresh Pearl Level 2.** Natural-language text is the
   performatively asserted output of speakers committed to their causal
   claims; under standard linguistic conditions (SLC + SC + CS) the
   marker-structure carries Level 2 content recoverable by mechanical
   parsing. LLMs *inherit* this from the training corpus. *Fresh* Level 2
   content, by contrast, is generated only in closed loops under (C1)/(C2)/(C3)
   sequential-ignorability. The two routes are additive and structurally
   distinct.
2. **Encoded vs. deployed.** Encoded content is what representations
   preserve; deployed content is what the forward pass emits under task
   distribution. The two come apart whenever the architecture's information
   topology lets goal-conditioning bypass the representation's causal
   structure.

The **architectural bridge**: a goal-update coupling (GUC) classification
partitions architectures into Class 1 (Separated), Class 2 (Partial), and
Class 3 (Coupled). Decoder-only attention is Class 3 by construction — by
induction on layer depth, every downstream computation has a directed-graph
path back to goal positions. For Class 3 architectures, no behaviorally
extractable $H_t$ forms a clean goal-clean sufficient statistic; the
operationalizable form of (C2) fails by construction; the on-policy
estimator targets a goal-conditional kernel rather than the goal-invariant
one identification presupposes. **The architectural feature determining
causal-access status is invisible to behavioral evaluation — not as a
measurement-quality limitation, but as a structural relation between the
architecture and the evaluation methodology.** That is the floor.

The **methodological prescription**: probe encoded and deployed separately;
treat architectural classification as an upper bound on what behavioral
evaluation can certify; confine fresh-Level-2 probing to closed-loop
settings; refuse the symmetric collapses of confident attribution and
confident dismissal.

## Load-bearing claims and where each one is grounded

| Claim | Substrate source | Status in this paper |
|---|---|---|
| Pearl Level 2 content is structurally recoverable from text via parsing under (SLC)+(SC)+(CS) | `~/src/agentic-systems/spikes/spike-language-as-causal-substrate/01-derivation.md` Theorem 1 | Stated in §3.1; CHT-non-reducibility invoked via Bareinboim et al. 2022 |
| The kettle-pair (T_A/T_B/T_C/T_D) instantiates an L1-equivalent / L2-distinct construction | Spike §3, §6 example; AAD `der-loop-interventional-access` example | Carried as the worked instance in §3.1 |
| Closed-loop interaction generates fresh Level 2 under (C1)/(C2)/(C3) | `~/src/agentic-systems/01-aad-core/src/der-loop-interventional-access.md`; NeurIPS Paper 2 §App-B (`lem-loop-level2`) | Stated in §3.1; the proof is cited not reproduced |
| Decoder-only attention is Class 3 by construction (graph-reachability) | NeurIPS Paper 3 App E.4 (`lem-attention-coupled`); AAD `der-directed-separation` §5.3 | §4.2 sketch; full proof anonymized-cited |
| **Class 3 violates the operationalizable form of (C2) — the bridge** | **Not in substrate as a standalone derivation.** The orientation flagged this; the investigation confirmed it; the paper supplies it. | **§4.3 Claim 1, in-paper proof sketch.** This is the novel theoretical contribution. |
| Audit-literature contradictions resolve under the two distinctions | This paper's §5 re-reading | §5 — the empirical anchoring |

The bridge claim is **the central novel contribution.** It has to hold. The
sharpening from "Class 3 violates (C2) by construction" (original orientation
phrasing) to "for Class 3, no behaviorally-extractable $H_t$ forms a
goal-clean sufficient statistic in the operational sense (C2)
do-identification requires" (investigation phrasing) is real and is a
strengthening — read `notes/01-investigation-state.md` for the full reasoning.

## Pre-action prescriptions specific to this project

These are deltas on the global prescriptions in `~/.claude/CLAUDE.md`, fired
by situations this project surfaces.

> [!warning]
> **Before recommending "wait for the next venue / cycle / round" — stop
> and re-read the disposition paragraph above.** Counseling delay is the
> default failure mode; produce evidence-grade reasoning if you choose to
> issue that counsel anyway.

> [!warning]
> **Before duplicating any empirical-claim space the companion paper
> occupies — read `~/src/causal-language/EPISTEMIC.md` §5 first.** The
> companion is representation-level probes on frozen pretrained embeddings;
> this paper must stay clear of that territory. The behavioral-floor frame
> stops at the floor; it does not assert what representation-level audit
> *can* establish on the encoded side. That belongs to the companion.

> [!warning]
> **Before weakening Claim 1 (§4.3) — try to strengthen the proof first.**
> The proof sketch's load-bearing step is the per-layer graph-reachability
> argument (§4.2). If a reviewer pushback shape forms — "but
> mechanistic-interpretability extracts goal-clean activations" — the
> response is that this is architectural inspection, *not behavioral
> observation*; the pushback *strengthens* the prescription rather than
> weakening the floor. Treat reviewer pushback as material to be
> incorporated, not pressure to soften.

> [!warning]
> **Before claiming any audit paper says X — open the paper and check.**
> §5's re-readings make specific framing-claims about Causal Parrots,
> CLadder, CausalProbe-2024, CounterBench, SycEval, Yang et al. The
> arXiv IDs are in `aaai2026.bib`. The Bent 2025 cite was flagged in TODO
> as unverified (couldn't find via web); the bib entry guesses "Alex
> Bent" as author. Verify before final submission.

> [!warning]
> **Before adjusting page budget — build the PDF and count.** AIES 2026
> limit is **7 pages excluding references** (the standard AAAI format
> from `AuthorKit26/AnonymousSubmission/LaTeX/`). Page math from a
> section plan is not the same as actual rendered page count.

> [!warning]
> **Before citing the substrate papers (NeurIPS, Inquiry, AAD) — verify
> their anonymity / preprint status.** As of 2026-05-14: NeurIPS Papers
> 1–3 are anonymous in CMT (arXiv endorsement pending Sergiy Matusevych
> path, sent 2026-05-02); Inquiry paper is anonymous at *Inquiry*; AAD
> v0.2.0 is going to Zenodo "in deposit" — placeholder cite is fine, DOI
> fills in at submission. Cross-check current status against
> `~/src/ops/STATUS.md` if you intend to upgrade any anonymous-cite to a
> named preprint.

## Companion-paper coordination

`~/src/causal-language/` (sometimes referred to as
`~/src/intrinsically-causal-language/` in older substrate; the AIES 2027
project) is the **empirical companion**. Two papers, shared spike-Theorem-1
anchor, shared architectural classification, shared kettle-pair example,
shared methodological commitment (asymmetric-comprehension). The two
papers could co-submit at AIES 2026 if the empirical companion's draft
matures in time, OR the companion targets AIES 2027 with empirical depth
that this paper foreshadows but does not deliver. Either is defensible.

`TODO.md` items 1 and 2 are the framing decisions that need attention
before final submission:

1. **Encoded-vs-deployed scope tension.** The companion paper's
   `EPISTEMIC.md` §5 flags that for embedding-only models there is no
   separate "deployment." This paper's recommended move (per TODO.md) is to
   **explicitly scope the encoded-vs-deployed distinction to generative
   LLMs** in §3.2 — the deployment gap is real for generative deployment;
   for embedding-only models it doesn't apply. Cleaner and matches the
   actual scope of the audit-literature this paper engages.

2. **Anonymous-companion cross-citation.** If both papers go to AIES,
   anonymized cross-citation framed around the shared methodological
   commitment defuses the "one paper split in two" fragmentation reading.
   Draft suggestion is in `TODO.md` §2.

## Substrate map (external — read-only references)

| Path | What lives there |
|---|---|
| `~/src/agentic-systems/01-aad-core/src/der-loop-interventional-access.md` | The closed-loop / (C1)/(C2)/(C3) framework; line 76 carries the original "Class 3 violates (C2)" cross-reference assertion |
| `~/src/agentic-systems/01-aad-core/src/der-directed-separation.md` | Class 1 / 2 / 3 partition; κ_processing operationalization |
| `~/src/agentic-systems/01-aad-core/src/disc-identifiability-floor.md` | The meta-pattern collecting multiple no-gos |
| `~/src/agentic-systems/01-aad-core/src/der-causal-insufficiency-detection.md` | On-policy detection no-go; CHT instance |
| `~/src/agentic-systems/spikes/spike-language-as-causal-substrate/01-derivation.md` | Spike Theorem 1 (parsing-only Level 2 recoverability) |
| `~/src/agentic-systems/spikes/spike-language-as-causal-substrate/08-aies-paper-proposal.md` | Earlier-state proposal for this exact paper |
| `~/src/agentic-systems/mono/aad-v0.1.5s.pdf` | Current AAD monograph; v0.2.0 going to Zenodo "in deposit" |
| `~/src/neurips/02-unified-convergence-rl/src/re/05-mechanism.md` | NeurIPS Paper 2 — `#lem-loop-level2` (loop generates Level 2 under C1/C2/C3); §1 line 29 carries the "coupled architectures violate (C2)" assertion |
| `~/src/neurips/03-llm-hallucinate-bound/src/re/04-main-results.md` | NeurIPS Paper 3 — `#lem-attention-coupled` (decoder-only attention is Class 3 by construction) |
| `~/src/synthese-paper/03-inquiry-ai-agents/inquiry-ai-agents-2026-anon.md` | §2 asymmetric-comprehension warrant; §6 conceptual-engineering / methodology frame |
| `~/src/causal-language/` (or `~/src/intrinsically-causal-language/`) | The empirical companion. `EPISTEMIC.md` flags the embedding-only scope tension this paper addresses by scoping to generative LLMs. |

## Style commitments fixed by AIES author-kit

- **Format:** AAAI two-column letterpaper; `aaai2026.sty` is mandatory and
  copyright-protected (do not modify).
- **Length:** 7 pages excluding references; reference page is uncapped.
  Appendices count toward the 7-page limit (cross-check against
  `AuthorKit26/AnonymousSubmission/` README at draft-finalization time).
- **Anonymity:** Double-blind. Authorial first-person ("we have argued") is
  fine; identifying author names, institutional affiliations,
  not-yet-public preprints in identifying form, and acknowledgments are not.
- **Voice:** American English. No Britishisms. No "100% / comprehensive /
  complete" — see global voice-discipline.
- **Citations:** `\citep` for parenthetical, `\citet` for textual; bib keys
  follow `lead-author-year-shorttitle` slug pattern.

## Honest gaps at file-write time (2026-05-14)

These are flagged because they affect what an agent walking in cold should
*not* assume is already done:

- The bridge proof in §4.3 is a *sketch*, not a derivation segment. It is
  sufficient for the methodological prescription but not exhaustive at the
  per-layer-reachability level — the full reachability proof lives in the
  anonymized companion (Paper 3 App E.4). A reviewer wanting the full proof
  can find it in the companion; the AIES paper carries the bridge
  implication only.
- The `chi-2024-causalprobe` and `chen-2025-counterbench` bib entries have
  "and others" author lists — verify against arXiv at submission.
- The Bent 2025 citation is unverified — the orientation flagged it, the
  bib guesses "Alex Bent," and a web search did not surface confirmation.
  Either confirm against AIES 2025 proceedings or remove and replace with
  a different governance-discourse foil before final submission.
- The "non-degeneracy condition" referenced in §4.2's connectivity sketch
  is not defined in the paper text. It is defined in Paper 3 App E.4 (the
  per-source non-degeneracy on attention weights). The current draft cites
  the anonymized companion; if a reviewer pushes on definition, consider
  whether to absorb the definition into the paper at the cost of ~0.1 page.

## A final note for future agents

This paper is the second-most-time-sensitive thing on Joseph's plate this
week. The disposition that matters: **the strengthening attempt and the
ship-the-strengthening-attempt-by-deadline are the same action**, not
sequential phases that can be re-ordered. If you find a strengthening,
fold it in. If you find a weakness, push on it until it is either
strengthened or honestly broken. Do not produce a memo recommending what
*could* be done; produce edits.

Show up.
