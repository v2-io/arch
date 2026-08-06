---
slug: outline-skipping-failure
type: obs
---

# Outline-skipping as a recurring failure mode

*The full account of how multiple instances treat “agents open segments and skip the outline” as a known, costly pattern — the specimen grounding [[orientation-gate]] and feeding [[outline-as-organizing-principle]].*

## What happens

A claim corpus’s outline is the front door: order, gaps, importance, the path a new mind is supposed to walk. Segment files are the rooms. Agents under task pressure open rooms — often the ones named in the prompt or the ones that look central — and treat the outline as optional documentation.

That habit has a structural cost: dependency order and “what the corpus says is missing” live in the outline. Skipping it produces work on the wrong claims, audits that miss load-bearing appendices, and code or tools that never learn they are downstream of claims. Compaction makes it worse: after a long session the *feeling* of orientation remains while the free-read of the outline does not.

## Specimens (live mechanisms, not anecdotes alone)

**1. vivarium — enforced free-read.**  
Live law in `vivarium/ORIENT.md` and `vivarium/CLAUDE.md`: free-read is **outline first**, then ★ starred rows in outline order; only then `bin/try-me` / `bin/prove-me` (seal `core/src`, quiz, commit token). The gate exists *because* agents treat segments as post-hoc docs and jump to code; the pass dies on compaction so false confidence cannot permanently unlock the tree.

**2. asf — audit and orientation defaults.**  
Live `asf/doc/sop/audit.sop/de-novo.sop.md`: the OUTLINE’s linearized order is a **verification target**; skipping to “central” segments is an explicit failure mode for missing dependency breaks and appendix load-bearing material. Live `asf/doc/sop/agents.sop.md`: read `01-aat-core/OUTLINE.md` first — ordering lives in the outline, not in filenames.

**3. comproprium — extra front doors.**  
Live narrative outline (`the-chain.md`) and trigger outlines exist on top of segment directories — effort that only pays if a raw segment directory is not enough of a front door.

## Method & scope

- Warrant: the live files named above (paths under vivarium and asf), read as operator law / SOP.
- **Not** measured: fraction of sessions that open the outline first (no systematic log).

This shows the failure mode is **recognized and counter-engineered** in multiple places. It does not give a frequency rate.

## Working Notes

- Incidence metrics (orient failures, commits without token) would upgrade this obs if they ever exist.
- Unintegrated influx (do not cite as warrant): `plan/INFLUX/instances/vivarium-orientation-gate.md` summarizes ORIENT/CLAUDE; the live files are the sources.
