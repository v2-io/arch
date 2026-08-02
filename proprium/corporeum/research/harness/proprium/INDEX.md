# PROPRIUM materials — collected under Archema harness

*Gathered 2026-07-20 from scattered pre-ASF/AAT sources into this workshop’s `proprium/` intake so restart has one home. Copies, not live links — upstream trees remain authoritative until a deliberate rework lands. Workshop lives at `proprium/corporeum/research/harness/` (2026-08-01); this intake folder is **not** the programme `proprium/` seat (`comproprium` + `corporeum`).*

## Status (honest)

PROPRIUM is a **pre-ASF / early-TFT** ontology and architecture for ELI personhood infrastructure (PRINCIPIA stores, ANIMA loop, INTERPRES, CHRONICA vs MEMORATA, etc.). It was more principled than commodity agent shells, and pieces still appear in ASF (`#def-proprium-mapping`, terminology). It has **not** had a serious rework against mature AAT (sector/persistence, directed separation, constructive-impossibility floors, value-functional grounding, etc.).

**Harness work (autopax, nexum, sapientia OPERATA, shoshin) largely stalled** once ASF took legs — thankfully, so the math could solidify first. It is now time for harness + training/algorithm legs of Archema again; this tree is the **intake substrate**, not the finished design.

The **design of record** for a next harness is already here:

| File | Role |
|---|---|
| [`proprium-harness-design.md`](proprium-harness-design.md) | Independent Rust spine; Claude+Grok dual-pass synthesis (2026-07-19) |
| [`proprium-harness-grok-input.md`](proprium-harness-grok-input.md) | Grok pass input into that synthesis |
| [`CHRONICA-PORT-SPEC.md`](CHRONICA-PORT-SPEC.md) | AAT obligations × Autopax/Nexum/sapientia; **revised after full chronica/identity segment reads** — integrity spine + $S_{\mathrm{id}}$ / compensation-channel honesty (2026-07-20) |
| [`AGENTIC-LOOP-PORT-SPEC.md`](AGENTIC-LOOP-PORT-SPEC.md) | ASF event-driven cycle, DS, orient cascade, coupled logogenic update; invent/port for process spine (pairs with CHRONICA) (2026-07-20) |
| [`MVP-VERTICAL-SLICE.md`](MVP-VERTICAL-SLICE.md) | Unified first spine: acceptance tests A–F, build order, non-claims |
| [`INTERPRES-COMPACTION-NOTE.md`](INTERPRES-COMPACTION-NOTE.md) | No-gaslighting / TRACTUS≠C≠CONSPECTUS; lived compaction failure |

One level up (this harness workshop root):

| File | Role |
|---|---|
| [`../CURRENT-THOUGHTS.md`](../CURRENT-THOUGHTS.md) | Descriptive consensus / field chasms snapshot |
| [`../STEWARD-JUDGMENT-2026-07-20.md`](../STEWARD-JUDGMENT-2026-07-20.md) | Joseph’s program-level judgments (Rust, cousins, continuity goal) |

Sibling landscape work: `../ai-cli-tools-*.md`. System-prompt research: `../msc/system/`.

---

## Layout

```
proprium/
  canonical/          # authoritative PROPRIUM v2 (firmatum, Mar 2026)
  archaeology/        # pre-split PROPRIUM.md + v1 ontology/architecture (Feb 2026)
  bridges/            # AAT/shoshin mappings (thin; needs rework)
  stalled-lineage/    # autopax / sapientia / nexum / shoshin plans that froze
  proprium-harness-*.md
  INDEX.md            # this file
```

### `canonical/` — start here for the ontology

| File | Source | ~date |
|---|---|---|
| `PROPRIUM-ONTOLOGY-v2.md` | `~/src/firmatum/` | 2026-03-02 |
| `PROPRIUM-ARCHITECTURE-v2.md` | `~/src/firmatum/` | 2026-03-02 |

Ontology = long-lived principles (entity types, store meanings, sovereignty). Architecture = implementation (layers, context, heartbeat, cost). Split deliberately (see archaeology `PROPRIUM.md` banner, 2026-02-23).

### `archaeology/` — history only

| File | Note |
|---|---|
| `PROPRIUM.md` | Pre-split monolith; points at ontology/architecture split |
| `PROPRIUM-ONTOLOGY-v1.md` | Pre-v2 ontology |
| `PROPRIUM-ARCHITECTURE-v1.md` | Pre-v2 architecture |

### `bridges/` — AAT / runtime alignment (stale relative to full AAT)

| File | Source |
|---|---|
| `def-proprium-mapping.md` | `asf/04-eli-core` — $M_t$/$\mathcal{C}_t$ ↔ PRINCIPIA/ANIMA (draft; thin vs full AAT) |
| `terminology-proprium-mapping.md` | `asf/terminology/entries/` |
| `shoshin-00-proprium-alignment.md` | `shoshin/` Mar 2026 — local-runtime term map + invariants |
| `05-proprium-and-what-act-is-really-for.md` | asf msc reflection |
| `project_proprium_canonical_source.md` | memorata research-xref — points at firmatum as canonical |
| `snippet__shoshin-CLAUDE__proprium-runtime-and-cycle-phases.md` | memorata snippet — runtime/cycle phases |

### `stalled-lineage/` — implementations & plans that paused for ASF

| File | Lineage |
|---|---|
| `autopax-*` | Consolidation attempt (Ruby); OPERATA, README, HANDOFF, synthesis, quick-awakening, operata principles |
| `sapientia-*` | Early OPERATA + AI conversation system requirements (2025-09/10) |
| `nexum-OPERATA.md` | Ruby nexum OPERATA |
| `shoshin-04-staged-research-plan.md` | Local-substrate staged plan |
| **`SURVEY-sapientia-zoetica-ennaos-nexum.md`** | **2026-07-20 survey** of the only fully functional ELI harness (`minimal-sapientia`) + Zoetica/Ennaos/Nexum; top liftables |

Also still live (not copied wholesale): `~/src/autopax/lib/autopax/{curatoria,chronica}/` (Ruby port targets); **`~/src/_core/sapientia/bin/minimal-sapientia`** (~4.5k LOC workhorse — do not duplicate; read in place); `_core/{zoetica,ennaos,nexum}/`.

---

## Version timeline (rough)

| When | What |
|---|---|
| 2025-09–12 | sapientia / early OPERATA; synaptic context designs; ennaos/nexum loops |
| 2025-11–12 | autopax synthesis, instrumenta plans, TUI, quick-awakening |
| 2026-02-23 | PROPRIUM split ontology/architecture (v1) |
| 2026-03-02 | PROPRIUM **v2** ontology + architecture (canonical) |
| 2026-03-07 | shoshin PROPRIUM alignment |
| 2026 spring– | ASF/AAT dominates; ELI `#def-proprium-mapping` thin bridge; harness largely frozen |
| 2026-07-18/19 | CLI landscape + dual-pass **proprium-harness-design** (independent Rust, not product fork) |
| 2026-07-20 | Collected under Archema harness workshop |
| 2026-08-01 | Workshop moved to `proprium/corporeum/research/harness/` |

---

## Open rework (when harness / training legs restart)

Not done in this gather pass:

1. **AAT re-mapping** — full correspondence of PRINCIPIA/ANIMA to AAT quantities after Parts I–III maturity (persistence, DS, sat-gap/regret, floors, wrapping, value-functional grounding). `bridges/def-proprium-mapping.md` is a start, not the rework.
2. **Compaction / INTERPRES honesty** — PROPRIUM “no context gaslighting” vs commodity summarization; design doc already takes rip-and-replace + TRACTUS/CHRONICA separation.
3. **Memorata / ASM vs commodity memory** — including how existing coding harnesses approach the split (see note below).
4. **Training / algorithm leg** — Joseph: separate Archema leg later; not in this tree yet.

---

## Note: commodity harnesses vs PROPRIUM shape

The dual-pass design already ranks OSS coding CLIs and argues **independent Rust**, not a product fork. Separately: among *shipping* agent CLIs, some (e.g. grok-build’s session layout under `~/.grok/`) begin to **mirror** PROPRIUM-ish separations — raw session/transcript layers vs higher memory/dream-style stores — closer to CHRONICA-vs-MEMORATA / ASM than pure chat blobs. That is **observation for the rework**, not a claim that any commodity product *is* PROPRIUM. Use as existence proof and port hints (see `proprium-harness-design.md`: doom_loop → VIGILIAE, leader/reattach later, etc.).

---

## What was *not* copied

- Audit-working duplicates of `def-proprium-mapping`
- asf worktrees / mono build chunks
- Entire autopax `lib/` or `spec/` (too large; port from source when building)
- zoetica (thin / moved; sapientia tools still reference it as dangling)
- Full eli homes under `~/src/eli/*` (lived register, not harness design intake)

Upstream originals remain in `firmatum/`, `autopax/`, `_core/*`, `shoshin/`, `asf/`. Prefer editing those if a source is still “live”; treat this tree as the **Archema harness workshop intake**.
