---
slug: claim-clocked-drains
form: claim
type-expected: derived
status: heuristic
max: conditional
state: [drafted]
depends: [post-living-collection, claim-dispatch-compounds]
---

# Claim: Every Queue Gets a Clock

Un-clocked queues driven by a single human clock are the observed systemic failure mode of a living collection's process layer; the repair shape is per-process: a trigger (one queued item is work), a healing drain woven into steady state (drain one extra, so backlog goes to zero by construction), and task-forces reserved for tangled one-offs.

## Formal Expression

*[Claim (the diagnosis)]*

The estate's one full process-layer review reduced its findings to a single sentence: the processes are *"un-clocked queues driven by a single human clock"* — sources (spikes, proposals, audits) fire on the steward's whim, flooding verification and integration, whose *drains* (graduation, routing-to-steward) are also steward-gated and whim-clocked — *"so everything backs up between his sessions."*[^map] The failure is structural, not diligence-shaped: no amount of agent effort inside a queue drains a queue whose drain waits on an absent clock.

*[Claim (the fix-shape — steward fiat, marked as such)]*

The repair, attributed in the source to the steward as fiat:[^map]

1. **Trigger**: every process fires on its own condition — ≥1 queued item *is* work, not a backlog awaiting a session that decides to care.
2. **Healing drain**: woven into steady state — each pass drains its own arrivals *plus one extra*, so any finite backlog reaches zero by construction, without a cleanup ever being scheduled.
3. **Task-forces only where a backlog is tangled or one-off** (the review named its own: the 22 gold dirs, the PROPOSALS knot, the working-note pool, the tracker design, the repo cleanup) — everywhere else, healing beats a task-force, and a steady rhythm mis-fits a tangle exactly as a task-force mis-fits a steady flow.
4. **Reduce the human-clocked part to genuine domain decisions**, each arriving as a real brief ( [[claim-decision-surfacing]]) — the steward stays the valve for what is genuinely his and stops being the clock for everything else.

Not every process takes a drain: sources and standing roles are `drain: nil` — a spike lifecycle is a source, not a backlog; a coherence role is a stance, not a queue.[^map]

## Epistemic Status

Heuristic, with `conditional` attainable and as ceiling: the diagnosis is one estate's lived process review (a ten-cluster discovery pass over asf's whole process layer, findings verified first-hand by its agents), and the fix-shape is steward design ratified by that diagnosis — neither is derived from the theory's postulates without conditions, and the load-bearing condition is visible: the claim presumes agent capacity is *available between steward sessions* (triggers only help if something can fire on them). Where that holds, the backlog dynamics follow from arithmetic (drain-one-extra is a strictly decreasing backlog) plus [[claim-dispatch-compounds]]'s surface-economics; where it does not, clocking changes nothing. Single-estate evidence, one steward. Evidence-action: instrument queue depths and time-in-queue before/after clocking on one instance ( [[form-observation-store]]).

## Discussion

**Why this is an organ-level law and not ops advice.** Under [[post-living-collection]] no queue ever empties *for good* — flux is permanent, so a process without a clock is not "occasionally behind," it is structurally divergent, and the divergence is invisible from inside any one session (each session sees a backlog, not a trend). The clocking discipline is what makes a living collection's physiology self-sustaining rather than steward-metabolized; it is also the precondition for [[claim-dispatch-compounds]]'s clean attractor — a healing drain is precisely the mechanism that holds a surface in the clean basin cheaply, and a task-force is the bounded restoration that gets a residual surface back there.

**The coupling caution, carried from the misfire feedback.** A drain can be defined and still never fire because its *trigger condition is a stage no item reaches* — the estate's working-note pool grew unbounded partly because its drain waited on a promotion gate that never fired ( [[form-state-flags-not-gates]]'s 115/115 finding is the same data seen from the other axis). Drains must be clocked against conditions that actually occur; a drain keyed to a ladder is a drain keyed to nothing.[^coupling]

**The deepest backup is the steward-gated drain.** The review's spine finding — every stall reduces to a decision that never reached the steward in actionable form — means the clocking program and the decision-surfacing program are one system: clock the mechanical drains, and what remains queued is genuinely-reserved calls, which drain through briefs, not through effort. A living collection that clocks everything *except* its valve has moved the single human clock, not removed it.

## References

[^map]: `~/src/arch/asf/msc/meta-process-review-2026-07-07/PROCESS-MAP-v0.udon` — the `|diagnosis` block (verbatim), the `|fix-shape :attribution Joseph` block, the per-process `drain` vocabulary (healing / task-force / nil) with the named task-force set, and the health annotations. Read whole 2026-08-09.
[^coupling]: Misfire-feedback drafting caution (2026-08-06, outline Working Notes): "keep the working-note-drain ↔ state-flag coupling (drains that never fire because a stage is never reached)"; the 115/115 first-rung measurement is in [[form-state-flags-not-gates]]'s sources.

## Working Notes

- Frontmatter schema provisional pending the epistemology decision.
- Open: the trigger vocabulary is thinner than the drain vocabulary — the process map distinguishes invoked/state-triggered/time-triggered/continual only in the instrumenta stratum ( [[form-instrumenta-invocation]]); whether process triggers and instrument triggers are one vocabulary is a kit question.
- Open (condition sharpening): "agent capacity available between steward sessions" is doing quiet load-bearing work — under harness limitations (no standing scheduler) the trigger reduces to "next session's orientation surfaces the queue," which is a weaker clock. The honest current form of clocking in this estate *is* orientation-surfaced queues (PRACTICA areas, pending surfaces); name that as the realization until real schedulers exist.
- Evidence-action logged above; the first measurable is time-in-queue for this very corpus's influx.
