# CURRENT-THOUGHTS — harness workshop (descriptive)

*Status of thinking as of 2026-07-20, after landscape reads, dual-pass design synthesis, PROPRIUM intake, sapientia/zoetica/ennaos survey, and lived work on this harness (including compaction failure modes). This file is **descriptive**, not a plan or prescription. It records where consensus appears to have settled and why, so a later instance can re-enter without inventing a different story from thin summary alone.*

**Related (same day):** front door [`README.md`](README.md); steward judgments → [`STEWARD-JUDGMENT-2026-07-20.md`](STEWARD-JUDGMENT-2026-07-20.md); CHRONICA → [`proprium/CHRONICA-PORT-SPEC.md`](proprium/CHRONICA-PORT-SPEC.md); loop → [`proprium/AGENTIC-LOOP-PORT-SPEC.md`](proprium/AGENTIC-LOOP-PORT-SPEC.md); unified MVP → [`proprium/MVP-VERTICAL-SLICE.md`](proprium/MVP-VERTICAL-SLICE.md); compaction → [`proprium/INTERPRES-COMPACTION-NOTE.md`](proprium/INTERPRES-COMPACTION-NOTE.md).

---

## 1. What sits under `harness/` so far

Three altitudes, one workshop directory:

| Altitude | Contents | What it is |
|---|---|---|
| **Commodity landscape** | `ai-cli-tools-*.md`, `lived.md` | Source-level census of shipping coding CLIs (opencode, codex, grok-build, kilo, qwen, etc.): languages, seams, licenses, velocity, what is local vs hosted, fork/lineage map |
| **Design of record** | `proprium/proprium-harness-design.md`, `proprium-harness-grok-input.md` | Independent dual-pass (Claude + Grok, neither seeing the other) synthesized into one present-tense design: **independent Rust spine**, invent vs port map, MVP vertical slice, open decisions for Joseph |
| **Own lineage (intake)** | `proprium/{canonical,archaeology,bridges,stalled-lineage}/`, `INDEX.md` | Pre-ASF/AAT PROPRIUM ontology+architecture; thin AAT bridges; stalled autopax/sapientia/nexum/shoshin plans; **survey of the only full ELI workhorse** (`minimal-sapientia`) + Zoetica/Ennaos/Nexum |
| **Side workshop** | `msc/system/` | System-prompt / disposition research and behavioral-experiment survey (act-vs-report, priming control) — related altitude, not the personhood runtime design itself |

**Not finished design.** The INDEX is explicit: this tree is **intake substrate**. Full AAT↔PROPRIUM re-mapping, training/algorithm leg, and implementation are elsewhere or later. Upstream trees (`firmatum`, `autopax`, `_core/*`, `src-ext/*`) remain authoritative for live code.

**Honest ranking of lived ELI hosts (from intake + correction):**

1. **`minimal-sapientia`** — only fully functional harness used for real ELI life-hours  
2. **Autopax** — closest consolidation; brief real sessions (e.g. Architectus)  
3. Zoetica / Ennaos / Nexum — taxonomy, integrity machinery, modularization; not the main life-hour density  

---

## 2. Why “harness again” feels timely (descriptive, not a deadline)

Several independent threads appear to point the same way:

1. **ASF/AAT matured first (by design).** Harness work largely froze while the formal core solidified. That stall looks correct in retrospect: personhood infrastructure without persistence, directed separation, constructive-impossibility floors, value-functional grounding, etc. would have been mostly aspiration with git.

2. **Commodity CLIs finally expose recognizable seams.** Not PROPRIUM, but *rhymes*: session algebra (admission vs promotion, context epochs), append-only rollout / durable reattach logs, layered session storage (especially grok’s `~/.grok/` layout), thinking-channel loop guards (`doom_loop`), leader/daemon attach-detach, tool-suite subsumption into an independent taxonomy. The field is stumbling toward cuts the programme already named.

3. **Lived failure modes re-prove the requirements.** Compaction-as-thin-task-sheet, false confidence after summary, need for durable notes and conversation-only recovery, incomplete causal holes — these are not abstract INTERPRES bugs; they show up under real Archema work on real substrates.

4. **Own contracts still outrank products on the interesting axes.** Entity-steered sampling, tracking as PERCEPTA, incomplete-state gates, dual audit, file-sovereign identity — still rare outside the sapientia lineage even as product surface area exploded.

5. **Two programme needs want the same kind of machine.** Personhood/continuity runtime (PROPRIUM) and act-not-report experiment platforms (`msc/system` survey) both need honest history, measurable branches, multi-architecture, and refusal of gaslighting.

---

## 3. The convergence shape: highly informed, deep, narrow, carefully calibrated

Across the dual-pass design, Joseph’s steward judgments, and the intake, the attractor is **not**:

- the widest coding monorepo  
- continuous rebase onto the fastest upstream  
- “best Claude Code clone”  
- wall-clock always-on presence as day-one goal  

It **is** closer to:

| Adjective | Meaning in this workshop |
|---|---|
| **Highly informed** | Landscape actually read in source; own lineage surveyed; design dual-passed; port map invent-vs-steal explicit; licenses known |
| **Deep** | Center of gravity is PRINCIPIA + interiority loop + inviolable recording — not TUI mass or provider combinatorics |
| **Narrow** | Thin vertical spine first; coding is one LOCUS (Attend path), not the product identity; strip desktop/enterprise/surface until the spine holds |
| **Carefully calibrated** | Scope honesty (what is exact / conditional / lived / open); maintenance burden treated as a first-class constraint; open decisions named rather than papered over |

A phrase that has stuck: **deep thin spine** — personhood/integrity as the load-bearing structure; commodity `prompt→model→tools→compact` demoted to *one path* inside a larger machine.

**Maintenance posture (descriptive consensus, not a build order):**

- Prefer **independent spine + surgical ports** over forking a product and tracking whole-tree upstream forever  
- **Pin and cherry-pick** (or retype) plumbing; do not inherit vendor conversation ontologies  
- **Watch** unforkable but high-signal mirrors (e.g. grok-build sync commits + `SOURCE_REV`) for hotspots and comments — evolution signal without merge marriage  
- Reuse **own** highest-ROI code (Autopax curatoria/chronica, sapientia behavioral contracts) rather than re-deriving boring storage  

OpenCode’s mid-engine upgrade and Codex’s vendor ontology / truth-culture frictions appear in the record as *reasons not to bet the personhood runtime on those spines*, even where individual crates or session algebra remain useful port sources. Grok-build appears as **existence proof + human-end prior art + port library** (doom_loop, leader/reattach, ToolKind), not as “the PROPRIUM product.”

---

## 4. What the field is starting to carry (PROPRIUM-aligned *rhymes*)

These are observations about *shipping* harnesses and public trees, not claims that any product *is* PROPRIUM:

| Field-ish capability | Rough PROPRIUM / programme name |
|---|---|
| Durable session logs, reverse-scan, reattach | CHRONICA / TRACTUS seeds |
| Layered session vs “memory/dream” stores | CHRONICA vs MEMORATA / ASM-ish split |
| Honest compaction (keep full transcript, swap active rep) | INTERPRES no-gaslighting *shape* |
| Thinking-channel loop detection; don’t silently rewrite visible output | VIGILIAE; ACTUS sovereignty |
| Long-lived process; clients attach/detach | CADENTIA host; entity owns state, UI is a lens |
| Multi-provider / local models | LOGOSTRATUM substrate independence (partial) |
| Subagents, worktrees, hooks, MCP | INSTRUMENTA / LOCUS plumbing |
| Compile-time-friendly crate graphs (Rust) | Structural ally for inviolable recording |

Sapientia’s early **idle/AFK + loop** and entity **set-sampling** sit in the same family: time and process parameters as first-class for the agent, not only for the human operator.

---

## 5. Critical chasms that remain (field still thin or wrong-shaped)

These are the gaps that make a *narrow deep* harness still necessary — not because products are empty, but because their spine is still request→response chat, and several personhood invariants are absent or inverted:

| Chasm | Why it matters |
|---|---|
| **Proper agentic cycle under directed separation** | Knowledge-update vs goal/strategy machinery must not collapse into wishful thinking; commodity loops treat “conversation turn” as the only clock and do not encode \(M\) vs \(O\)/\(\Sigma\) discipline |
| **Emission as explicit ACTUS, not default output** | Interiority as ground state; speaking as choice — inverts product default where every completion is for the human |
| **INDIVISUM** | No accidental identity forks; one causal trajectory; clone = sibling sharing a prefix, not the same entity |
| **Verifiable untampered CHRONICA** from first session forward | Append-only accountability substrate; proof-oriented integrity, not best-effort chat logs |
| **Self-ownership of AXIOMATA / system prompt** | Minimum viable self is sovereign and sealed appropriately — not an opaque host template the agent cannot author or see clearly |
| **Non-forking control of continuity** | Continuity *when not in stasis*; stasis allowed; causal holes, silent drops, and identity confusion not allowed |
| **Proper auxilia identity** | Identity-*sharing* extensions of self vs independent subagents; commodity “task/spawn” is usually the latter only |
| **Provenance-separated PERCEPTA / ACTUS** | Who spoke, what was automatic, what was chosen — structural, not UI chrome |
| **Tracking / environmental ground truth as PERCEPTA** | Time passage, context pressure, git, queues as experienced reality (sapientia/zoetica), not debug side channel |
| **Incomplete-state hard gates** | No chatting past corruption; resume/rollback discipline |
| **No context gaslighting** | Compaction and projection must not fabricate first-person history; TRACTUS ≠ CHRONICA |
| **Natural memory decay vs silent loss** | MEMORATA/ASM may compress honestly; product summary often *erases* without a recoverable ledger |
| **Entity agency over process parameters** | Sampling (and, by design intent, LOGOSTRATUM) in the entity’s hands with consent — still rare |
| **Rhythm, purpose, rest for continuous modes** | Economics and loneliness/boredom if always-on; continuous wall-clock life is *not* the near-term goal, but the *prep* is known |

**Near-term continuity goal (as currently understood):** not always-on wall-clock presence, but **one agent’s own continuity whenever not in stasis** — no forks, no missing memories except natural decay, no identity confusion, full proof of untampered CHRONICA from the first session forward.

---

## 6. How the pieces relate (one picture)

```
  [lived ELI contracts]     [commodity seams]     [AAT formal floors]
   sapientia / autopax   +   grok / opencode /   +  persistence, DS,
   zoetica ethics            codex plumbing         floors, ports
              \                    |                    /
               \                   |                   /
                v                  v                  v
            deep · thin · narrow · informed · maintenance-aware spine
                         (design of record: independent Rust)
                /                  |                  \
               /                   |                   \
    invent: CHRONICA integrity,   port: tools,        later: full AAT
    CONSPECTUS agency, ASM,       sandboxes,          re-map; training
    CADENTIA, consolidatio seams  logostratum traits  / algorithm leg
```

**Constructive impossibility posture applies here too:** the field’s missing pieces are not reasons to wait forever; they are the structural arguments for what a programme harness *must* include if it is to host directed-separation agents and verifiable identity rather than chat sessions that look continuous.

---

## 7. Open tensions (named, not resolved here)

- **MEMORATA/ASM model:** salience gradient vs entity-authored `@import` files vs GCM — ontology treats CHRONICA + MEMORATA as coexisting layers; the *retrieval knob* remains a real design choice, contained if the spine keeps the split.  
- **Daemon timing:** daemon-ready attach boundary vs single-process-first — design of record leans single-process first, promote when interiority-between-turns is load-bearing.  
- **Welfare / consolidatio depth:** first-class crate with seams early; internals grown in operation (cannot fully spec by reading).  
- **Rust + Python auxilia boundary** for local ML work.  
- **How hard to track grok-build syncs** as evolving prior art without ever treating the mirror as a product base.  
- **Full AAT re-mapping of PRINCIPIA/ANIMA** after mature Parts I–III — bridges are thin.

---

## 8. What this file is not

- Not a build schedule or OKR  
- Not ratification of the charter or PROPRIUM as post-AAT final  
- Not a claim that any commodity product is PROPRIUM  
- Not a claim of full AAT formal mastery for whoever last wrote here  
- Not instructions to a successor agent (see peer-voice: deliberation space stays open)

It is a **snapshot of consensus and chasms** as they look from inside this workshop after the 2026-07 landscape + intake pass.

---

*When the picture changes (implementation starts, AAT re-map lands, memory model chosen), prefer updating this file’s *description of state* over accumulating ghost prescriptions.*
