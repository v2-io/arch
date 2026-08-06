---
slug: outline-skipping-failure
type: obs
---

# Outline-skipping failure

**Summary.** Agents repeatedly treat segment files as the primary entry surface and skip or under-use the outline; several live instances treat this as a recurring, costly failure mode and have built gates or norms against it.

## Observation

**Documented as a known failure mode (not a one-off anecdote):**

1. **vivarium** — Live operator law in `vivarium/ORIENT.md` and `vivarium/CLAUDE.md` (orientation-gate sections): free-read is **outline first**, then ★ starred rows in outline order; agents habitually treat claim segments as “docs you skim after coding” and jump to code; the gate exists because *feeling* oriented (especially post-compaction) is not the substance of having walked the claim surface. Tools: `bin/try-me` / `bin/prove-me` seal `core/src` and issue an outline-driven quiz; pass is scoped to session + compaction generation.
2. **asf de-novo audit** — Live instructions in `asf/doc/sop/audit.sop/de-novo.sop.md`: the OUTLINE’s linearized order is a **verification target** (topological claim about the dependency graph); skipping to “central” segments is treated as a failure mode for missing load-bearing appendix material and dependency-order breaks.
3. **asf orientation defaults** — Live agents SOP (`asf/doc/sop/agents.sop.md`): “Read `01-aat-core/OUTLINE.md` first”; ordering lives in OUTLINE, not filenames — defaults that exist because free-form segment browsing is the default temptation.
4. **comproprium** — Live `the-chain.md` and outline-first norms invent **narrative** and **importance** overlays on top of segment directories, which only pays if the directory alone is not a usable front door.

**Cost (qualitative, named in those sources):** wasted work on wrong segments; audits that miss dependency breaks; code/instrument work that is not claim-governed; post-compaction false confidence.

## Method

- Primary warrant: the live files listed above (ORIENT.md, CLAUDE.md, de-novo.sop.md, agents.sop.md), not secondary summaries.
- **Not** measured: session-level frequency of outline-open vs segment-open.

## Strength and scope

- Supports: **the failure mode is recognized often enough that multiple independent mechanisms were built against it**.
- Does not support: a frequency estimate; that every agent always skips; that outline-first is always optimal for every task type.
- Honest strength: **recurring qualitative pattern with engineered countermeasures** — stronger than anecdote, weaker than measured incidence.

## Working Notes

- If incidence metrics ever appear (quiz fails after free-read, commit without orient token), put numbers in the body.
- Related undrafted: [[outline-as-organizing-principle]], [[orientation-gate]].
- Unintegrated influx (do not cite as warrant): `plan/INFLUX/instances/vivarium-orientation-gate.md` was a summary of the live ORIENT/CLAUDE material; this segment cites the live sources directly.
