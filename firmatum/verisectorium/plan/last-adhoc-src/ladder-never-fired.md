---
slug: ladder-never-fired
type: obs
---

# Ladder never fired

**Summary.** The multi-gate promotion ladder (draft → deps-verified → claims-verified → format-clean → candidate, with Gate 4 emptying Working Notes) is documented widely and almost unused as a completed trajectory; flag-shaped stage values and daily landing dominate.

## Observation

Re-checked **2026-08-05** on live frontmatter `stage:` lines:

| Instance | What was counted | Result |
|---|---|---|
| vivarium `core/src` | 115 segment files | **115 × `stage: draft`** (100%) |
| asf all four components `*/src/*.md` (non-`old-*`) | `stage:` lines | **200 draft · 23 deps-verified · 18 claims-verified · 2 exploratory** |
| asf, top of ladder | `format-clean` / `candidate` as stage values | **0** in this pass |

So:

- vivarium (115 segments) has **never left draft** as a stage value in this count.
- ASF has real mid-ladder occupancy (deps-verified / claims-verified) but **no** `format-clean` or `candidate` stages observed — the designed terminus of the ladder does not appear in stage values as used.
- FORMAT’s Gate 4 (empty Working Notes at candidate) therefore has nothing to fire against in the stage field: nothing sits at that terminus.

## Method

- Shell `grep` of `^stage:` under the listed trees; `uniq -c` (2026-08-05).
- Does **not** include a full archaeology of every historical Gate 1–4 *practice* in spikes/audits — only **stage field occupancy** as a proxy for “ladder as lived process.”

## Strength and scope

- Supports: **ladder as designed terminus is not how work completes today**; flags/stages are used, full ascent is rare or absent on the trees counted.
- Does not support: that promotion *work* never happens (ASF has deps-verified and claims-verified); that stages are useless; that Gate 1–3 never run as human practices without updating stage.
- Honest strength: **strong for “full ladder unused as terminus”** on the trees counted; weaker for “gates never fire as human practices.”

## Working Notes

- Feeds [[state-flags-not-gates]] once that formulation is drafted.
- If later work populates `candidate` / `format-clean`, update or supersede this obs — the measurement is dated.
- Unintegrated influx (do not cite as warrant): `plan/INFLUX/synthesis/live-state-field-reports-2026-08-05.md` headline 2 concurred; body warrant is the grep counts above.
