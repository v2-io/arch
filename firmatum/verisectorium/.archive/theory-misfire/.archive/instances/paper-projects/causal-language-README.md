<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: causal-language/README.md (companion paper; paper/src/NN-*.md)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/causal-language/README.md
  Do not edit here expecting to update the live original.
-->

# Intrinsically Causal Language

*An empirical and methodological investigation of whether Pearl-hierarchy
causal content — and more broadly, directional/categorial structural content
of natural language — is preserved in frozen pretrained sentence embeddings,
recoverable via structural probes that use formal semantics as oracle, and
**decomposable by mechanism** (syntactic-position vs vocabulary-Reichenbachian)
in a way that says exactly what such measurement can and cannot certify.*

*The originating theoretical work (Theorem 1: explicit causal markers encode
Pearl-Level-2 content non-reducibly to the event-variable Level-1 distribution,
by the Bareinboim-Correa-Ibeling-Icard PCH theorem, under three named
linguistic postulates) is at [`spikes/originating-spike/`](spikes/originating-spike/)
— a snapshot of the 2026-05-13 spike, canonical at
`~/src/agentic-systems/spikes/spike-language-as-causal-substrate/`.*

> **Freshness: 2026-05-15.** History of completed cycles lives in
> [`LOG.md`](LOG.md) (frozen, newest-first). This README is current
> orientation + status only. The substantive docs (`HEADLINE-FINDINGS.md`,
> `EPISTEMIC.md`, `OPEN-WORK.md`, `STRENGTHENING-AND-SYNTHESIS.md`) carry the
> 2026-05-15 D1 correction and are current; their empirical numbers are
> explicitly marked PENDING the running V2.1.1 panel.

---

## Where the work is right now

The project has moved through three phases: (1) originating theoretical spike
→ exploratory empirical battery (E1–E13); (2) parallel-spike convergence on
the C1/C2 framework → V2 publication-quality synthesis (E14–E19); (3) three
integrated audits + a theory-push that made the cross-paper relationship
exact. It is now in the **V2.1.1-panel + paper-drafting** phase.

- **V2.1.1 panel: running.** All six strengthened experiments (E14–E19) ×
  12-model capacity-diverse panel, on a cleared `results/`. No canonical
  numbers yet; `results/cross_model_aggregation.{md,json}` is regenerated
  when it lands. The numbers write-back into `HEADLINE-FINDINGS.md` /
  `EPISTEMIC.md` is the next tracked task.
- **Audits: all integrated.** Joint opus+Codex audit and the 2026-05-15
  follow-up audit are in `audits/.integrated/` with full per-finding
  disposition in `INTEGRATION-LOG.md`. Strengthen-before-soften was applied
  throughout (e.g. F5/F6 strengthened rather than relabeled).
- **Cross-paper relationship: exact.** The relationship to
  `~/src/behavioral-floor/` is **Proposition D1** (modality-switch handoff
  at the encoded/deployed seam), not "constructive dual / complement" — that
  framing was pushed to a proven categorial no-go and corrected. See
  `spikes/spike-constructive-dual-of-behavioral-floor/`.
- **Paper: working draft assembled.** `paper/` builds a 9-page blind-review
  PDF. Number-independent sections are first-draft; empirical sections are
  honest PENDING scaffolds wired to the numbers write-back. Nothing in the
  paper reads as a result that isn't one.

## Suggested reading order

1. This section, for orientation.
2. [`docs/initial-spike-dialog-2026-05-13-and-14.md`](docs/initial-spike-dialog-2026-05-13-and-14.md)
   — the dialog that produced the theoretical work (load-bearing framing).
3. [`spikes/originating-spike/01-derivation.md`](spikes/originating-spike/01-derivation.md)
   — Theorem 1; [`02-related-angles.md`](spikes/originating-spike/02-related-angles.md)
   (C1/C2/C3/C4 framework); [`06-cht-application-audit.md`](spikes/originating-spike/06-cht-application-audit.md)
   (the load-bearing non-reduction step, audited).
4. [`HEADLINE-FINDINGS.md`](HEADLINE-FINDINGS.md) **with**
   [`EPISTEMIC.md`](EPISTEMIC.md) — the synthesis and its honest adversarial
   companion. Read the top-of-file callouts: the V1 numbers are pilot
   evidence; the V2.1.1 panel re-measures; EPISTEMIC §16 + §8.5 carry the
   C1-vs-C2-vs-Theorem-1 scope discipline.
5. [`spikes/spike-constructive-dual-of-behavioral-floor/`](spikes/spike-constructive-dual-of-behavioral-floor/)
   (README → `03` → `04` → `05`) — the exact cross-paper statement (D1/D2).
6. [`docs/diagnostic-methodology.md`](docs/diagnostic-methodology.md) — the
   methodological-class contribution. [`docs/prior-art-assessment.md`](docs/prior-art-assessment.md)
   — novelty audit. [`docs/algorithmic-information-unification-2026-05-14.md`](docs/algorithmic-information-unification-2026-05-14.md)
   — the theoretical-spike layer.
7. [`STRENGTHENING-AND-SYNTHESIS.md`](STRENGTHENING-AND-SYNTHESIS.md) — the
   V2 blueprint + conceptual scaffold (incl. the D1 cross-paper section).
8. [`paper/src/`](paper/src/) — the working draft. [`OPEN-WORK.md`](OPEN-WORK.md)
   — forward work. [`LOG.md`](LOG.md) — what's already closed.

## If you are auditing

The phase where the whole project was offered for external audit is closed
(three audits integrated). Engage `EPISTEMIC.md` directly — it openly
catalogues where the work is weak, what would falsify it, and what survives.
The strongest single honesty boundary: only **C1-dominant** classifications
test Theorem 1's non-reducibility property; C2-dominant/MIXED are
Pearl-content-*aligned* signal whose mechanism a Level-1 corpus summary could
in principle reproduce. The paper splits results accordingly and does not let
a C2-dominant finding borrow Theorem-1's authority.

---

## What this project is (one paragraph)

Three coupled outputs:

1. **A mechanism-decomposing substrate-side diagnostic.** A class of probes
   using formal semantics itself as oracle (no annotation/crowd-sourcing)
   that does not merely detect but *decomposes* a directional/categorial
   signal into a C1 syntactic-position route (vocabulary-identical;
   shuffle-null; CHT-non-reducible — the route Theorem 1 licenses) and a C2
   Reichenbachian vocabulary-co-occurrence route (shuffle-surviving), with an
   H_D3 control isolating a world-knowledge-direction prior. Substrate-
   agnostic (incl. decoder pooled hidden states), self-correcting, and
   content-general — it applies unchanged to evidential register, so it is a
   frozen-substrate inventory tool, not a Pearl-specific probe.

2. **A precise statement of what it can and cannot certify (D1).** Where it
   returns C1-dominant it discharges the *encoded-presence precondition* that
   behavioral evaluation provably cannot even lower-bound (companion
   no-go); by the encoded/deployed seam it certifies presence in the
   representation, explicitly *not* deployed use. The papers compose by
   handoff across the seam, not by duality.

3. **The theoretical anchor.** Theorem 1 as an audited lower-bound
   conditional result; the C1/C2/C3/C4 framework; the asymmetric-comprehension
   principle (`~/src/synthese-paper/01-synthese-asymmetric-comprehension/`);
   the four-arrows directionality unification (`docs/`).

**What it does not claim.** Not that LLMs reason causally — it refutes one
specific empirical ground for *confidently denying* the content is present,
and is careful which side of the substrate/channel line each result speaks
to. It does not assert the strong CHT-non-reducible route is the dominant
finding; that breadth is a measured quantity (PENDING the V2.1.1 panel), and
the methodological contribution — *separating* present-in-substrate from
deployed-in-channel from vocabulary-recoverable — stands independent of it.

---

## Project layout

```
intrinsically-causal-language/
├── README.md                      — orientation + current status (this file)
├── LOG.md                         — frozen record of closed cycles ★ HISTORY ★
├── HEADLINE-FINDINGS.md           — empirical synthesis (V1 pilot; V2.1.1 numbers PENDING)
├── EPISTEMIC.md                   — honest adversarial review (D1-corrected §16)
├── OPEN-WORK.md                   — forward work + strengthening ideas
├── PROGRESS.md                    — experiment trail
├── STRENGTHENING-AND-SYNTHESIS.md — V2 blueprint + conceptual scaffold (D1 section)
├── CLAUDE.md                      — project-specific agent guidance
│
├── paper/                         — working draft (pandoc+relata+pdflatex; builds)
│   ├── src/00-meta.md             — title + abstract (empirical slot bracketed)
│   ├── src/01..10-*.md            — first-draft safe sections + PENDING scaffolds
│   └── bin/build, common/         — pipeline (mirrors ~/src/behavioral-floor/)
│
├── spikes/
│   ├── originating-spike/                          ★ THEORETICAL ANCHOR ★
│   ├── spike-constructive-dual-of-behavioral-floor/ ★ D1 — cross-paper exact ★
│   └── llm-centric/                                — H_D3 + decoder-hidden-state
│
│   (2026-05-15: the two 2026-05-14 parallel spikes — situation_independent,
│    phrasing-vs-situation — were consolidated into experiments/E14–E19 per
│    the V2 blueprint and archived to .archive/spikes_2026-05-14_*/, each with
│    a provenance README. Superseded; cite E14–E19, not the spikes.)
│
├── audits/.integrated/            — 3 integrated audits + INTEGRATION-LOG.md
│
├── docs/                          — framing/context (diagnostic-methodology,
│                                    prior-art, algorithmic-information-unification,
│                                    initial-spike-dialog, reflections, …)
│
├── experiments/                   — E1–E13 (stable pilot) + E14–E19 (V2.1.1)
│                                    + aggregate_cross_model.py + NOTES.md
├── results/                       — cleared; V2.1.1 panel repopulating
│   └── .archive/                  — pre-V2.1.1 + legacy results
├── data/                          — model-cards, external-generated, (pdtb placeholder)
├── figures/                       — empty until paper figures
└── .archive/                      — historical / superseded planning docs
```

---

## Cross-references to other projects

- **`~/src/agentic-systems/`** — framework substrate (AAD); canonical home of
  the originating spike. The relationship surfaces as a candidate adjacent
  pair in the `#disc-identifiability-floor` cascade (see the D1 spike).
- **`~/src/embeddings/`** — methodology source (difference-vector /
  null-control / concept-erasure machinery).
- **`~/src/synthese-paper/01-synthese-asymmetric-comprehension/`** —
  philosophical companion; asymmetric-comprehension is load-bearing in both.
- **`~/src/behavioral-floor/`** — companion. The exact relationship is **D1**
  (modality-switch handoff at the encoded/deployed seam): behavioral
  evaluation provably cannot lower-bound the encoded-presence precondition
  this diagnostic discharges; the encoded/deployed seam is why presence does
  not upgrade to deployed use. **Not** a duality (corrected 2026-05-15). The
  genuine dual pair is GC2 ↔ H_D3 (Proposition D2, role-homology). Mirrored
  cross-reference into behavioral-floor is a coordinated, not-yet-done move
  (OPEN-WORK §C; do not edit behavioral-floor src unilaterally).

---

## What's NOT here

- **No final results** — the V2.1.1 panel is running; V1 numbers are pilot
  evidence, explicitly so flagged.
- **No corpora committed** — PDTB-3 / RST-DT / GUM are licensed; references
  in `docs/` only.
- **No model weights** — pulled via Ollama at runtime.
- **No remote** — local-only.
- **No license file** — TBD (AIES camera-ready, if that venue, requires ACM
  rights agreement; until then this is private working material).
