---
slug: form-process-map
form: formulation
type-expected: formulation
status: discussion-grade
max: decided
state: [drafted]
depends: [def-verisectorium, claim-clocked-drains]
---

# Formulation: The Process Map

Every instance maintains a per-instance process map — its processes in clusters over the organs, each carrying health, drain, trigger, and signal-flow, with designed-vs-absent marked honestly — as the physiology record to the outline's anatomy.

## Formal Expression

*[Formulation (process-map) — the notation, which is the formulation]*

A process map is a structured record whose entries carry, per process:

1. **`health`** — free-text, honest, present-tense. The worked instance's vocabulary shows the range a real map needs: healthy · broken · absent · stalled · latent · whim-triggered · under-fed · under-used · stale-prone · opportunistic · nascent · aspirational · mixed · and compound honesty like *"infra-healthy, decision-stalled"* or *"outward healthy, return broken."* Health is not an enum; forcing it to one loses exactly the compound cases that matter.[^map]
2. **`drain`** — enum, per [[claim-clocked-drains]]: `healing` / `task-force` / `nil` (not backlog-shaped: sources and standing roles).
3. **Trigger and signal-flow** — what fires it; `fed-by` / `feeds` edges, drawn as a diagram only where flow clarifies dynamics (the core loop), listed per-item elsewhere.
4. **`?` with `:design-unknown`** — a process the instance *should* have but has not designed, named as absent with exactly what is undesigned stated (*"the trigger + the scoped-instruction generator"*). Absence is recorded, never implied by omission.[^map]
5. **`:wire-to`** — cross-instance connections a process must eventually make (a sim discipline wired to the lab member and the citable-artifact store).
6. **An `|unplaced` block** — territory deliberately deferred, with the deferral reasoned: *"its processes get carved when the theory does, not before."* Deferral is a first-class recorded state, not a gap.[^map]
7. **A `self-governance` meta-process at the top**: participant feedback as the *only* source/sink of the meta-process itself ( [[post-self-governance]] realized at map level).

Processes group into **clusters over the organs** (generation, verification, integration, coherence, pedagogy, release, governance, cross-project, interliminal — the worked instance's cut), and instance maturity is assessed **per cell of organs × processes** ( [[claim-organ-process-duality]]): the map is what makes "mature unevenly" a statement with coordinates instead of a mood.

## Epistemic Status

Formulation — a chosen representation, defended by one strong worked instance; max attainable `decided`, current status `discussion-grade` (one map, one instance, one author; the notation has not yet been exercised by a second instance or a second mapper). What the instance demonstrated: the notation was expressive enough to carry a whole estate member's physiology in ~500 lines, to reduce it to a one-sentence diagnosis, and to state a ratified fix-shape — and its `?`-marking surfaced seven undesigned processes that no amount of health-annotation on existing processes would have found. The deliberately-not-enum'd health field and the `|unplaced` block are the two choices most likely to be contested by a tidier future mapper; both are defended above as honesty-preserving. Evidence-action: a second instance's map (this corpus's own is the obvious candidate), ideally by a different mapper, and a drift-check of the first map against its instance a season later.

## Discussion

**Why a map and not a dashboard.** A dashboard reports metrics on processes someone already believes in; the map's distinctive work is at the *existence* layer — which processes should exist, which are absent, which are deferred on purpose. The `?`-entries are its highest-yield rows precisely because nothing else in an instance represents a missing process (a queue that doesn't exist has no backlog to notice). This is [[claim-truth-over-proxy]]'s absence-discipline applied to physiology: absence explicitly recorded, never read off silence.

**The map is itself an atom under this theory's laws.** It is authored (not generated), carries present-truth health (replacement semantics — a health annotation is a status claim, and a stale one is a label lying about status), and drifts like every proxy — so it is dated, versioned, and owed re-truthification on a clock of its own. The worked instance marked itself `:status first-pass` and invited correction of the carve in its own header; that register is part of the formulation, not modesty.

**Relation to the outline.** The outline (anatomy) and the process map (physiology) are the two views an instance needs of itself; [[def-verisectorium]]'s organ table answers "what does it have," this answers "what runs over it, and how is it doing." The kit ( [[form-instantiation-kit]]) ships processes *born clocked* — which means a new instance's first process map can be generated from the kit's answers and then diverges as lived truth accrues.

## References

[^map]: `~/src/arch/asf/msc/meta-process-review-2026-07-07/PROCESS-MAP-v0.udon` (2026-07-07, `:status first-pass`) — the worked instance: conventions header, cluster/process entries with all fields above, the core-loop diagram, `|diagnosis`, `|fix-shape :attribution Joseph`, `|open-unknowns`, `|unplaced`. Read whole 2026-08-09. The ten findings/reflections files beside it are the discovery substrate the map compressed.

## Working Notes

- Frontmatter schema provisional pending the epistemology decision.
- Open: udon-vs-markdown realization — the worked instance is udon (`|process` blocks with attributes); the kit should let a minimal instance start with a markdown table and the same fields. The fields, not the syntax, are the formulation.
- Open: cadence — the map is owed its own clock (per its own law) and no cadence is yet chosen anywhere; candidate: re-truthify at each PRACTICA boundary where any area changed state.
- Forward: this corpus should dogfood the formulation (a verisectorium-theory process map) once the sop store and influx drains have run long enough to have honest health values — premature mapping would record design intent as health, the exact register error [[claim-truth-over-proxy]] forbids.
