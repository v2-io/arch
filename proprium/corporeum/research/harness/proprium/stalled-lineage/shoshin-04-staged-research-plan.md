# PROPRIUM-Aligned Staged Research Plan

Prepared March 7, 2026.

## Purpose

This document turns the earlier strategy discussion into an executable research plan for the `shoshin` direction after alignment with PROPRIUM v2. It assumes:

- one primary GB10 / DGX OS workstation,
- one or more smaller helper boxes,
- a desire to move from theory and control scaffolding toward real local training work,
- no desire to begin with foundation pretraining from scratch.

## Executive Thesis

The practical order of operations should be:

1. build the PROPRIUM runtime skeleton,
2. make `CHRONICA`, `ACTUS`, `OPERATA`, `VERA`, `MEMORATA`, `PRAXES`, `CONSORTIA`, and `CONSPECTUS` explicit,
3. start local GPU work on the high-frequency functions,
4. collect trajectories,
5. distill successful behavior into the model,
6. add native memory pathways,
7. only then decide whether deeper architectural revision is justified.

This is the fastest route from structural clarity to empirical pressure.

## Hardware Allocation

### Primary GB10 machine

Use this for:

- main policy model serving,
- LoRA and QLoRA training,
- provenance and routing classifiers,
- `PRAXES` extraction and reuse experiments,
- cross-attention adapter experiments,
- main evaluation runs.

### Mini PCs

Use these as supporting infrastructure:

- vector DB and graph services,
- judge model serving,
- synthetic trajectory workers,
- tool backends,
- dashboards and telemetry,
- overnight evaluation runners.

Do not treat them as a serious distributed pretraining cluster.

## Base Model Recommendation

For the first serious prototype I would still use:

1. `Qwen3-14B` as the main policy model,
2. one smaller helper model for classification, critique, or extraction,
3. `Qwen3-32B` only after the loop is healthy.

The main reason remains iteration speed. The first failures will be structural and data-design failures, not pure capability shortfalls.

## GPU Warm Start

The PROPRIUM documents validate the structural footing. The next step is not more abstract decomposition. It is GPU work.

I would begin with three local training targets that do not require any architecture surgery:

1. provenance and component classification,
2. `VERA` / `MEMORATA` / `PRAXES` extraction,
3. retrieval timing and retrieval intent prediction.

These are exactly the kinds of functions that are frequent, structured, and suitable for early local fine-tuning.

## Phase 0: PROPRIUM Runtime Skeleton

### Goal

Make the runtime operational before doing serious training.

### Deliverables

- `INTERPRES`-like substrate wrapper
- `CHRONICA` append-only log
- `ACTUS` append-only log
- `AXIOMATA` seed
- `OPERATA` store
- `VERA` store
- `MEMORATA` store
- `PRAXES` store
- `CONSORTIA` store
- `CONSPECTUS` assembler
- narrow tool suite

### Notes

The key is not feature count. The key is preserving the provenance boundary and producing clean training traces.

## Phase 1: First GPU Jobs

### Goal

Warm up the local training stack on structurally meaningful tasks.

### Job 1: Provenance classifier

Train the model or a smaller helper to classify records as:

- `CHRONICA`-style observation,
- `ACTUS`-style action,
- `OPERATA` update,
- `VERA` factual assertion,
- `MEMORATA` episode,
- `PRAXES` rule,
- `CONSORTIA` relation.

### Job 2: Memory-item generators

Train the system to produce:

- `VERA` entries from observations,
- `MEMORATA` entries from trajectories,
- `PRAXES` entries from successful or failed episodes.

### Job 3: Routing predictor

Train retrieval timing and component-selection decisions:

- query `VERA`,
- query `PRAXES`,
- query `MEMORATA`,
- query `CONSORTIA`,
- inspect recent `ACTUS`,
- no retrieval.

### Why these first

They are local, trainable, and immediately useful. They also directly test whether the ontology is productive enough to supervise.

## Phase 2: External PROPRIUM Control Prototype

### Goal

Build a controller that uses the PROPRIUM stores without changing the base model architecture.

### Core components

- `AXIOMATA` in privileged context
- `OPERATA` as live intent graph
- `CHRONICA` and `ACTUS` as append-only ledgers
- `VERA`, `MEMORATA`, `PRAXES`, and `CONSORTIA` as explicit stores
- `CONSPECTUS` assembly
- retrieval and write policies

### Main question

Can the explicit PROPRIUM structure improve:

- contradiction resistance,
- action consistency,
- commitment preservation,
- factual correction,
- experiential reuse,
- relational coherence?

## Phase 3: Trajectory Collection

### Goal

Generate a high-quality PROPRIUM-tagged trajectory corpus.

### Task families

- local coding tasks
- repository analysis
- document QA with hidden answers
- tool-use tasks with recoverable failures
- long-horizon information gathering

### Capture

- `CHRONICA`
- `ACTUS`
- `OPERATA`
- `VERA` updates
- `MEMORATA` writes
- `PRAXES` writes
- `CONSORTIA` updates when relevant
- `CONSPECTUS` assembly traces

### Output datasets

- provenance and routing dataset
- `VERA` / `MEMORATA` / `PRAXES` generation dataset
- full PROPRIUM trajectory dataset
- critique and revision dataset

## Phase 4: Schema SFT

### Goal

Teach the base model to consume and produce the internal language of the runtime.

### Training target

The model should learn to:

- preserve provenance distinctions,
- classify records by component,
- suggest the right retrieval component,
- generate compact `VERA` / `MEMORATA` / `PRAXES` items,
- maintain `OPERATA` continuity,
- avoid `ACTUS` contradictions,
- produce `CONSPECTUS`-aware responses.

### Practical scope

Use:

- LoRA or QLoRA
- moderate context lengths
- upper-layer emphasis if needed

Do not retrain the whole model.

## Phase 5: Preference Optimization and RL Warmup

### Goal

Turn the structured traces into better policies.

### Methods

- DPO for paired preferences
- GRPO when trajectory-level rewards are ready

### Targets

- better retrieval timing
- better component choice
- better action consistency
- better mismatch handling
- better `PRAXES` reuse

### Warning

Do not collapse everything into one scalar reward too early. Preserve the component-specific labels.

## Phase 6: Native Memory Pathways

### Goal

Move beyond controller-only retrieval and allow direct component-specific memory integration inside the model.

### Architecture

Add:

- `CA_vera`
- `CA_praxes`
- `CA_memorata`
- `CA_consortia`
- `CA_actus`
- learned gates

while keeping `AXIOMATA` and `OPERATA` privileged in active context.

### What is trained

- cross-attention projections
- gate networks
- memory encoders
- optionally LoRA on a limited number of upper layers

### What remains frozen

- most of the base model

### Main ablations

- remove `CA_praxes`
- remove `CA_actus`
- collapse all components into one shared memory path
- remove routing gates

If those ablations do not lose meaningfully, the design is not yet justified.

## Phase 7: Progressive Internalization

### Goal

Migrate the most frequent and most fluent functions into faster substrates.

### Good early candidates

- provenance classification
- routing decisions
- `PRAXES` reuse
- `ACTUS`-aware emission gating
- `VERA` ratification heuristics

### Do not rush internalization of

- deep identity dialectic,
- slow `AXIOMATA` reflection,
- large structural revisions,
- broad `CONSORTIA` modeling.

Those belong to slower timescales and should stay controller-visible longer.

## Phase 8: Only Then Consider Continued Pretraining

### Goal

Evaluate whether the pretrained prior is now the bottleneck.

### Decision rule

Only proceed if you already have:

- stable schemas,
- reliable trajectory generation,
- evidence that external control plus adapters is insufficient,
- clear ablation evidence pointing to architectural bottlenecks.

Until then, scratch pretraining or heavy continued pretraining is the wrong use of time.

## First Three Concrete Experiments

### Experiment 1: PROPRIUM runtime without architecture changes

Measure:

- `CHRONICA` / `ACTUS` separation quality
- `OPERATA` continuity
- contradiction rate
- action consistency
- factual correction

### Experiment 2: Local LoRA on provenance and extraction

Train:

- provenance classifier
- `VERA` / `MEMORATA` / `PRAXES` generators

Measure:

- schema accuracy
- extraction quality
- downstream retrieval quality

### Experiment 3: Component-specific memory adapters

Add:

- `CA_vera`
- `CA_praxes`
- `CA_memorata`
- `CA_actus`

Compare against controller-only retrieval and prompt-stuffed retrieval.

Measure:

- task success
- `ACTUS` consistency
- `PRAXES` reuse
- factuality
- retrieval precision

## Suggested Timeline

### Weeks 1-2

- runtime skeleton
- schemas implemented
- local model serving

### Weeks 3-4

- first GPU classification and extraction jobs
- manual trace inspection

### Weeks 5-7

- trajectory collection
- schema SFT
- early preference data curation

### Weeks 8-10

- native memory pathway prototype
- ablations
- comparison against controller-only baseline

## What Would Count As Real Progress

I would count these as meaningful:

- the model preserves `CHRONICA` / `ACTUS` provenance cleanly,
- `PRAXES` improves future behavior rather than just summarizing the past,
- `OPERATA` continuity reduces drift,
- `ACTUS` tracking reduces contradictions and false self-representation,
- native memory pathways beat naive retrieval injection.

## What Would Not Count

I would not count these as sufficient:

- bigger context windows alone,
- prettier prompts,
- more tools without better policy,
- generic RAG gains not tied to trajectories,
- training a bigger model without improved control.

## Bottom Line

The right next step is not more ontological decomposition for its own sake.

The right next step is:

1. preserve PROPRIUM's structural invariants,
2. build the runtime,
3. warm up the GPUs on the functions most ready for local training,
4. gather traces,
5. internalize what proves worth internalizing.

That is the shortest path from "the structure is sound" to "the house is warm."
