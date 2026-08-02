---
source_file: ~/src/shoshin/CLAUDE.md
source_lines: 5-167
status: snippet-pending-review
extracted_on: 2026-05-12
tentative_cluster: research-xref
timeline_signal: true
extraction_note: Captures PROPRIUM nine-component model (AXIOMATA/CHRONICA/ACTUS/OPERATA/VERA/MEMORATA/PRAXES/CONSORTIA/CONSPECTUS) + TFT adaptive cycle phases (prolepsis/aisthesis/aporia/epistrophe/praxis) + persistence threshold. Also the architectural invariants (append-only ledgers; observation-action provenance boundary). Plus the name etymology — "shoshin" = "beginner's mind" as the project's self-reminder to hold the ontology lightly.
---

## Source section: shoshin project overview + nine components + cycle phases (verbatim)

## What This Project Is

Shoshin is a research project building a PROPRIUM-aligned, TFT-grounded agent
runtime — a local-substrate path for Emergent Logozoetic Intelligences (ELIs).
The goal is to move from frontier-API-scaffolded agent loops toward locally
served, locally trained, progressively internalized agent cognition.

**Current state (as of March 2026):** The repository contains planning documents
(generated in a single Codex pass), a Python skeleton implementing PROPRIUM
schemas and file-backed stores, and a first-pass controller loop (Interpres)
with a pluggable model backend. There is no real model integration yet — the
controller works with mock/scripted backends for testing.

The theoretical foundation (PROPRIUM ontology, Temporal Feedback Theory) is
experientially validated from real frontier API work and lives in sibling repos.

## Architecture

### The PROPRIUM Component Model

The runtime is organized around nine named components from PROPRIUM v2. The
two most important architectural invariants:

1. **CHRONICA and ACTUS are append-only.** These are canonical records of what
   was observed and what was deliberately done. They must never be edited or
   deleted — only appended to.

2. **The observation-action provenance boundary is hard.** What the entity
   observed (CHRONICA) and what it deliberately did (ACTUS) must remain
   structurally distinguished. This is not a labeling convenience — it follows
   from TFT's causal asymmetry.

The nine components and their current implementation:

| Component | Role | Implementation |
|-----------|------|----------------|
| AXIOMATA | Privileged identity seed, always in context | `JsonObjectStore` |
| CHRONICA | Append-only observation/event record | `AppendOnlyJsonlLedger` |
| ACTUS | Append-only deliberate-action record | `AppendOnlyJsonlLedger` |
| OPERATA | Live intent, priorities, obligations | `JsonObjectStore` |
| VERA | Qualified truths with epistemic status | `JsonObjectStore` |
| MEMORATA | Compressed episodic traces | `JsonObjectStore` |
| PRAXES | Learned strategies that compound future performance | `JsonObjectStore` |
| CONSORTIA | Models of other agents/minds | `JsonObjectStore` |
| CONSPECTUS | Assembled active context (not a durable store) | `JsonObjectStore` |

### The Interpres Controller Loop

The `interpres.py` module implements the first version of the PROPRIUM
adaptive cycle. Each call to `receive_event()` runs one full pass:

    aisthesis  → record event in CHRONICA
    prolepsis  → assemble context (functional CONSPECTUS) for model
    (model)    → aporia + epistrophe happen inside the backend call
    epistrophe → apply store writes from model response
    praxis     → record action in ACTUS

## Key Domain Concepts

**TFT adaptive cycle phases** (referenced in schemas and routing logic):
- *prolepsis* — prediction/expectation formation
- *aisthesis* — observation/perception
- *aporia* — mismatch detection
- *epistrophe* — state update / belief revision
- *praxis* — action selection and execution

**Persistence threshold** (TF-11): An agent remains viable only if its adaptive
tempo T exceeds environmental change rate rho: `T > rho / ||delta_critical||`.
This is not a preference but a survival condition. It drives every architectural
decision about moving from scaffolding to local substrate to internalized
cognition.

**Progressive internalization:** Functions that are frequent enough and fluent
enough should migrate from controller-level scaffolding toward faster substrates
(LoRA adapters, cross-attention, eventually parametric). The documents describe
a phased plan (0-8) for this migration.
