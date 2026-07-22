# PROPRIUM Alignment For Shoshin

Prepared March 7, 2026.

## Purpose

This note aligns the earlier `shoshin` design work with PROPRIUM v2. The earlier drafts were already on the right footing: start at the structural and control level, make the temporal loop explicit, and only later internalize the loop into faster substrates. PROPRIUM sharpens that position and gives the operative names and invariants.

## Executive Summary

The earlier generic design language should now be interpreted as follows:

| Earlier Shoshin Term | PROPRIUM / TFT-Aligned Term |
|---|---|
| self-memory | split into `AXIOMATA`, `OPERATA`, and `ACTUS` |
| environment/user memory | mostly `VERA`, plus some `CONSORTIA` |
| experiential memory | split into `MEMORATA` and `PRAXES` |
| event ledger | `CHRONICA` |
| act ledger | `ACTUS` |
| working state | `CONSPECTUS` |
| runtime controller | `ANIMA`, especially `INTERPRES` and `CADENTIA` |

That split is materially better than the earlier three-bank shorthand.

## Hard Invariants From PROPRIUM v2

These should be treated as architectural constraints, not stylistic preferences.

### 1. Observation-action provenance is hard

The distinction between what the entity observed and what it deliberately did is not just a convenient label. It is a structural consequence of causal asymmetry. In practice:

- `CHRONICA` and `PERCEPTA` encode what arrived.
- `ACTUS` encodes deliberate external action.
- The system should not treat these as interchangeable text fragments.

### 2. `CHRONICA` and `ACTUS` are append-only

These are records of reality and accountability. They can be summarized, indexed, or compressed for downstream use, but the canonical records themselves should be system-governed and inviolate.

### 3. `AXIOMATA` is the minimum viable self

The entity needs a privileged identity seed that is always available, even across discontinuities. In transformer terms, that argues for a privileged always-on context position rather than an ordinary retrievable memory item.

### 4. Memory components are regimes, not arbitrary bins

`VERA`, `MEMORATA`, `PRAXES`, `CONSORTIA`, and `CONSPECTUS` are best understood as emergent access/update regimes. The architecture should support the dynamics that cause these regimes to emerge and stabilize.

### 5. Progressive internalization is the right migration path

What is frequent enough and fluent enough should migrate toward faster substrates. That validates the current approach: start scaffolded, gather trajectories, use local GPU compute to internalize what deserves it.

## Practical Consequences

### Replace the earlier coarse banks

The earlier `M_env`, `M_exp`, and `M_self` framing was useful as a first pass, but the current working decomposition should be:

- `AXIOMATA` as privileged identity seed
- `VERA` as factual store with epistemic status
- `MEMORATA` as compressed episodes
- `PRAXES` as gain-improving techniques and strategies
- `CONSORTIA` as relational models
- `OPERATA` as active priorities, efforts, and obligations
- `CHRONICA` as canonical event record
- `ACTUS` as canonical action record
- `CONSPECTUS` as assembled active context

### Promote `PRAXES`

The most important upgrade from the earlier framing is that experiential memory should not remain a single blob. PROPRIUM and TFT both point toward a distinction:

- `MEMORATA` preserves episodes and compressed traces.
- `PRAXES` captures what compounds future update quality.

That makes `PRAXES` a first-class training target.

### Encode the provenance boundary in training

The model should not only classify self vs other after the fact. The provenance boundary should shape:

- tagging,
- retrieval key spaces,
- memory pathways,
- losses,
- evaluation.

### Treat local GPU work as the next serious step

The conceptual footing is now strong enough. The next phase should not be more abstract taxonomy. It should be:

- local serving,
- local logging,
- local LoRA training,
- local adapter experiments,
- empirical pressure on the schemas.

## Recommended Interpretation Of The Next Work

`shoshin` should now be treated as:

1. a PROPRIUM runtime prototype,
2. a TFT-informed memory-and-attention research program,
3. a local-substrate migration path,
4. a GPU warm-start effort rather than a purely theoretical notebook.

That is the frame used in the updated documents that follow.
