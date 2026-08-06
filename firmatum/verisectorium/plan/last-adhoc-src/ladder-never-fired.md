---
slug: ladder-never-fired
type: obs
---

# Ladder never fired

**Summary.** The multi-gate promotion ladder (draft → deps-verified → claims-verified → format-clean → candidate, with Gate 4 emptying Working Notes) is documented widely and almost unused as a completed trajectory; flag-shaped stage values and daily landing dominate.

## Observation

Re-checked 2026-08-05 (this session) on live frontmatter `stage:` lines:

| Instance | What was counted | Result |
|---|---|---|
| vivarium `core/src` | 115 segment files | **115 × `stage: draft`** (100%) |
| asf all four components `*/src/*.md` (non-`old-*`) | `stage:` lines | **200 draft · 23 deps-verified · 18 claims-verified · 2 exploratory** |
| asf alone, top of ladder | search for `format-clean` / `candidate` as stage values | **0** in this pass |

So:

- The most actively worked claim corpus in the sample (vivarium, 115 segments) has **never left draft** as a stage value.
- ASF has real mid-ladder occupancy (deps-verified / claims-verified) but **no** `format-clean` or `candidate` stages observed — Gate 4-style terminus does not appear in the stage vocabulary as used.
- Live-state field report (same day, ✔) independently stated estate-wide Gate 4 has never fired and that heavy Working Notes sit on “most promoted” material because nothing has been swept — consistent with these counts.

## Method

- Shell `grep` of `^stage:` under the listed trees; `uniq -c`.
- Cross-check: field report 2026-08-05 headline 2.
- Does **not** include a full archaeology of every historical Gate 1–4 run in spikes/audits — only **stage field occupancy** as a proxy for “ladder as lived process.”

## Strength and scope

- Supports: **ladder as designed terminus is not how work completes today**; flags/stages are used, full ascent is rare or absent.
- Does not support: that promotion *work* never happens (ASF has deps-verified and claims-verified); that stages are useless; that Gate 1–3 never run as *practices* without updating stage.
- Honest strength: **strong for “full ladder unused as terminus”** on the trees counted; weaker for “gates never fire as human practices.”

## Working Notes

- Feeds [[state-flags-not-gates]] as formulation once principles are drafted.
- If later work populates `candidate` / `format-clean`, update this obs or supersede it — the measurement is dated.
