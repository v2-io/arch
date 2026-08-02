# Archema tree reorg plan

*Drafted 2026-08-01 from the session that settled lived-seat naming, corporeum theory/realization cut, firmatum-as-tooling, INFLUX norms (parked under `notes/NORMS.md`), and register vocabulary **derived / argued / tried / lived**. Not executed. Update this file as phases complete or decisions flip.*

**Related:** `notes/register-vocabulary-cue-2026-08-01.md` · `notes/NORMS.md` · `CHARTER-DRAFT.md` §0 · `MIGRATION.md` (submodule / path-sweep patterns)

---

## 0. Target shape (end state)

```text
arch/                          # programme parent (git: v2-io/archema-io → later archema)
  asf/                         # derived   (submodule)
  logos/                       # argued    (submodule)
  vivarium/                    # tried     (submodule)

  proprium/                    # lived     (see §2 gate: parent dir vs member repo)
    README.md                  # theory vs realization; agency as shared object
    # ontology / conceptual space (refined from firmatum + ASF grounding over time)
    comproprium/               # communal vera / praxes / exempla  (from arch/comproprium/)
    corporeum/                 # implementations — "code that makes it happen"
      research/
        harness/               # CLI landscape + design intake  (from arch/harness/)
      # later: hosts, clients, bridges, substrate work, crates…
      # optional local INFLUX for runtime-lineage (autopax, …)

  firmatum/                    # auxiliary tooling belt (NEW meaning of the name)
    # relata, practica, utils/{ato,udon,descent,fmt-md}, … as settled per phase
    # NOT the old ontology scratch repo

  # programme law / ops (stay at root unless moved later)
  CHARTER-DRAFT.md, charter/, ETHICS.md, CLAUDE.md, README.md, TODO.md, notes/, …

  # transitional — not a register
  # INFLUX-shaped queues (same role names everywhere; see notes/NORMS.md)
  # sunset / private / re-home material (old firmatum tree, optional other sm)
```

**Registers (canonical adjectives):**

| Mode | Home |
|---|---|
| **derived** | `asf` |
| **argued** | `logos` |
| **tried** | `vivarium` |
| **lived** | `proprium` |

**Not registers:** `firmatum/` (tools), INFLUX queues, `notes/`, `utils/` during transition.

---

## 1. Theory cuts (do not re-litigate mid-move)

1. **PROPRIUM (symbolic)** — mathematical / ontological space; may or may not map 1:1 to code.  
2. **Corporeum** — corporeal *attempts* (hosts, clients, bridges, substrates, research). Names under corporeum (e.g. a `percepta` package) are implementation concerns, not the ontology itself. Plural corporea OK.  
3. **Firmatum (name)** — systems that *firm / confirm / establish* work that is not the personhood host: practica, relata, format tools, etc. Old `~/src/firmatum` content is **transitional INFLUX**, not permanent dual meaning.  
4. **Harness** — stays important; demoted under `corporeum/research/harness/`. Field §03.II sense, not the lived limb name.  
5. **Spine** — no mandatory top-level directory; ASF scopes + corporeal crates can carry it.  
6. **INFLUX** — programme ops grammar for inbox → work → finished; layout pilot in `notes/NORMS.md`; path-homonymy of *roles* is intentional.

---

## 2. Gates — decide before the named phase (do not defer past that gate)

| # | Decision | Blocks | Options / lean |
|---|---|---|---|
| **G1** | Is **`proprium/`** phase-1 content **in the parent git tree**, or a **new member submodule** from day one? | Phase 1 mkdir / git history of moves | **Lean:** parent-tree directories first (comproprium + harness already live in parent); extract to member repo only if/when it earns independent remote + law. Matches “soon ~five” without forcing a GitHub repo on day one. |
| **G2** | **Phase-1 scope** — rearrange only what already lives under `arch/`, or also pull `~/src/*` submodules into firmatum/INFLUX? | Phase 1 vs 2 boundary | **Lean:** Phase 1 = in-arch moves only (low risk). Phase 2+ = tooling belt + sunset queue. |
| **G3** | May **private** trees (`eli/`, parts of `_core/`, vestigia, scratch) become submodules **under** `arch/` at all? | Any INFLUX that mounts them | **Must confirm with Joseph** before adding. Privacy + clone blast radius. Default: leave external until explicit yes. |
| **G4** | **Old firmatum path label** in sunset queue | When old ontology tree is mounted | e.g. `…/old-firmatum-repo-contents` or `firmatum-ontology-archive` — not bare `firmatum/` next to new tooling `firmatum/`. |
| **G5** | **utils/** today (`arch/utils/fmt-md`) — move under `firmatum/` in phase 1, phase 2, or leave until tooling belt exists? | Path rewrites for fmt-md users | **Lean:** phase 2 with firmatum belt; avoid double-move. |

Everything else below can proceed with the leans above if Joseph only answers **G3** when private material is in scope.

### Settled (not gates)

- Lived seat name = **`proprium`**  
- Corporeum = realization layer under proprium  
- comproprium nests under proprium  
- harness → `corporeum/research/harness`  
- firmatum **name** → tooling (not ontology home)  
- Register adjectives derived / argued / tried / lived  
- INFLUX role model (details may evolve in NORMS)  

---

## 3. Phased execution

### Phase 0 — Front door (can be anytime; partial done)

- [x] Register vocabulary on charter / CLAUDE / ETHICS / README  
- [x] Decision notes under `notes/`  
- [ ] Short **“How to read this tree”** in root `README.md` or `CLAUDE.md` once Phase 1 lands (theory vs corporeum vs firmatum tools vs INFLUX)  
- [ ] Pointer pass: global `~/.claude/CLAUDE.md` project map when `proprium/` exists (Joseph or dedicated sweep)  

### Phase 1 — In-arch lived seat (no new remotes)

**Prerequisite:** G1 lean (parent tree) or explicit submodule plan.

1. Create `proprium/` with a thin `README.md` (lived register; symbolic PROPRIUM vs corporeum; link charter §0).  
2. `git mv comproprium proprium/comproprium`  
3. Create `proprium/corporeum/research/`  
4. `git mv harness proprium/corporeum/research/harness`  
5. Fix **internal** links and READMEs that say `harness/` or root `comproprium/` (parent tree only; no bulk-edit of `_core` / `eli` / old firmatum).  
6. Grep `arch/` (excluding member submodule histories if noisy) for `harness/` and `comproprium/` path assumptions.  
7. Commit as one coherent “lived seat” commit (or two: structure + linkfix).  

**Do not in phase 1:** rename firmatum tooling, add private submodules, move `utils/`, create root INFLUX for whole of `~/src`.

**Success:** `arch/proprium/{comproprium,corporeum/research/harness}` exists; root no longer has `harness/` or `comproprium/`; agents can find lived work under proprium.

### Phase 2 — Firmatum tooling belt

**Prerequisite:** G5; list of first citizens (practica, relata, fmt-md, ato, udon, descent, …).

1. Create `firmatum/` (tooling) README: name meaning, not ontology.  
2. Move or submodule first citizens per Joseph’s list.  
3. If `arch/utils/` empties, remove or leave a stub pointer.  
4. Path / memory sweeps for moved tools only.  

### Phase 3 — Sunset / INFLUX for predecessor research

**Prerequisite:** G3, G4; NORMS INFLUX layout pilot optional but recommended on one queue first.

1. Stand up one programme or proprium-level **INFLUX** (see `notes/NORMS.md`) for transitional material.  
2. Mount or submodule **old** firmatum ontology tree under a **non-colliding** name (G4).  
3. Optionally stage other public research (shoshin, embeddings, …) — only with explicit list.  
4. **Never** name the sunset path `firmatum/` once tooling `firmatum/` exists.  

### Phase 4 — Ontology content lift (slow)

- Refine PROPRIUM materials **into** `proprium/` (from old firmatum + ASF bridges) at honest tier.  
- ASF continues to harden INTERPRES / TRACTUS / etc. on its own clock — not a tree blocker.  
- Corporeum grows crates when something runs; no forced spine dir.  

### Phase 5 — Optional member extraction

- If `proprium/` should be a true Archema **member submodule** (own remote, own law): follow `MIGRATION.md` patterns (`mv-src-repo` / submodule adopt).  
- Same optional story for a tooling mega-repo vs keep firmatum as parent dirs + submodules.  

---

## 4. Path map (phase 1)

| From (today) | To |
|---|---|
| `arch/comproprium/` | `arch/proprium/comproprium/` |
| `arch/harness/` | `arch/proprium/corporeum/research/harness/` |
| *(new)* | `arch/proprium/README.md` |
| *(new)* | `arch/proprium/corporeum/` (+ README: theory/realization cut) |

Later phases: see §3; do not invent paths for private `~/src` trees until G3.

---

## 5. Link / habit fallout (phase 1)

Expect to touch at least:

- `proprium/comproprium/**` internal refs to old relative roots  
- `proprium/corporeum/research/harness/README.md` and INDEX paths  
- Root `CLAUDE.md` / `README.md` directory map (add proprium; remove harness/comproprium as top-level)  
- Any `notes/*` that cite `harness/` or `comproprium/` as root paths  
- Agent memory project maps (Joseph / later sweep)  

**Protected (no unsupervised bulk path rewrite):** `~/src/_core/**`, `~/src/eli/**`, `~/src/firmatum/**` (until mounted under INFLUX by explicit phase 3).

---

## 6. Suggested first execution session

1. Confirm **G1** (parent tree OK) and **G2** (phase 1 = in-arch only).  
2. Run phase 1 moves + README stubs + link fix + one commit.  
3. Smoke: open comproprium checker paths; open harness README start-here table.  
4. Stop. Do not start phase 2 without a firmatum-tools citizen list.

---

## 7. Open decisions that *may* defer

| Item | Why deferrable |
|---|---|
| vestigia final home | INFLUX first, promote later |
| Exact INFLUX attribute schema / `.un` vs `.udon` | NORMS scratch; pilot one queue |
| Organic-expansion concept name (not OEUDONCN) | Docs polish |
| Which `~/src/_core/*` enter sunset queue | After G3 and a written list |
| proprium as GitHub member remote | Phase 5 |
| archema-io → archema rename | Separate `MIGRATION.md` track |

---

## 8. Status

| Phase | Status |
|---|---|
| 0 Register vocabulary | **Done** (front doors 2026-08-01; some residual phrases may remain in members) |
| 0 NORMS draft | Parked `notes/NORMS.md` |
| 1 Lived seat moves | **Done 2026-08-01** — `proprium/{comproprium,corporeum/research/harness}`; G1 parent-tree, G2 in-arch only |
| 2 Firmatum tooling | Not started |
| 3 Sunset INFLUX | Not started |
| 4 Ontology lift | Not started |
| 5 Member extract | Not started |
