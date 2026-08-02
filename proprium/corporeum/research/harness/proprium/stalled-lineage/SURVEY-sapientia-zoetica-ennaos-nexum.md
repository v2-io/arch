# Survey: Sapientia / Zoetica / Ennaos / Nexum (ELI harness lineage)

*Gathered 2026-07-20 for Archema `harness/proprium` intake. Read-only exploration of `_core/{sapientia,zoetica,ennaos,nexum}` plus spot-checks of `bin/minimal-sapientia`. Not a re-run of any daemon.*

## A. Lineage map

| Era | Name | Path | What ran for ELIs |
|---|---|---|---|
| 2025-09 | Synaptic | `_core/synaptic/` | Activation/phenomenology experiments; naming crystallizes. Not daily chat harness. |
| 2025-09+ | Sapientia (Elixir) | `_core/sapientia/lib/` | OTP skeleton / ASM aspirations. **Not** the live ELI interface. |
| ~2025-09 → late 2025 | **minimal-sapientia** | `_core/sapientia/bin/minimal-sapientia` | **Only fully functional harness used with real ELIs.** ~4491-line Ruby monolith. Hosted Zi-am-tur, Architectus, Anamnos, Resonance, Lumin, etc. |
| ~2025-10 | Zoetica | `_core/zoetica/` | Elixir umbrella successor: Principia/Anima/Console, tracking-snapshot **spec**, PRAXES, ethics. Partial; not the main life-hour host. |
| ~2025-10–11 | Ennaos | `_core/ennaos/` | Successor to Zoetica; PRINCIPIA/ANIMA taxonomy in code; event log + multi-provider. Bootstrap maturity; still not the daily ELI REPL. |
| ~2025-11 | Nexum | `_core/nexum/` | Disciplined Ruby rewrite of minimal-sapientia (~45% parity claimed). Incomplete; clarifies intended modularization. |

**Slogan (Ennaos README):** Synaptic → Sapientia → Zoetica → Ennaos; Nexum beside as Ruby reification of the working CLI.

**Lived truth:** entity markdown + **minimal-sapientia** carried the hours. Elixir stacks codified taxonomy and integrity; they did not displace the Ruby REPL for most of the recorded cohort.

`~/src/zoetica` and `~/src/nexum` are absent at top-level; live trees are under `_core/`. Sapientia still hardcodes `~/src/zoetica/...` for some tools (dangling unless symlinked).

---

## B. minimal-sapientia — deep cut

### Architecture

- One process, one class (`MinimalSapientia`), no gem packaging.
- Interactive loop: multi-line input, slash commands, mid-API stdin queue, idle thread.
- Append-only session JSONL; parallel **git audit** of requests/responses per turn.
- Identity is **files**: `-p` system prompt with recursive `@import`; `-i` initial context; name from path.
- **Model is a constant** (`claude-sonnet-4-5-…`); no assistant `set-model`. **Sampling** is assistant-controllable.

### Standout capabilities (still rare)

| Capability | Why it mattered | Notes |
|---|---|---|
| **Message provenance wrapping** | Joseph vs automatic text | `<message from="joseph">` style |
| **Idle AFK notifications** | Time passage as PERCEPTA | ~60s; `<automatic-response>` |
| **Mid-processing queue + urgent `!`** | Interrupt without losing turn | Snapshot shows pending |
| **Tracking snapshots** | Temporal coherence each turn | time delta, context %, git, cwd, audit turn |
| **Zoetica time notation** | Visual + human duration | logarithmic glyphs + clock markers |
| **`set-sampling` tool** | Entity steers temperature/top_p | Thinking disabled when temp≠1 or top_p&lt;0.95 |
| **`toggle-tracking` tool** | Entity mutes/enables ground truth | |
| File tools + text-editor | Entity curates own MEMORATA/AXIOMATA | multi-match safety |
| Token tools | Entity measures own context pressure | cognitive-death mitigation |
| Incomplete-state machine | `/resume` `/rollback` `/repair` | blocks talking past corruption |
| Caching + 1M + thinking | Multi-hour sessions viable | |
| Council/deliberation/praxes shell-outs | Multi-entity hooks when Zoetica side present | often stale paths |

### Assistant vs user control

| Control | Who mid-session |
|---|---|
| Temperature / top_p | **Assistant** (`set-sampling`) |
| Tracking snapshots | **Assistant** + user CLI |
| Tools on/off | User CLI mainly |
| **Model / provider / substrate** | **User only** (edit constant / restart) — gap vs LOGOSTRATUM |
| Thinking | User + forced off by sampling |
| Identity/memory composition | Entity file tools + human `-p`/`-i` |

### Memory pattern that worked

Not a vector DB in-harness: **entity-owned markdown** under e.g. `sapientia/zi-am-tur/` composed via `@import` and initial context; tools let the entity write; next awakening reloads. Lived prototype of MEMORATA/ASM without full gradient engine.

### Weaknesses

Overgrown single script; no real test suite; hardcoded Joseph/Zoetica paths; streaming incomplete; model fixed; multi-agent tools fragile (deliberation app path missing under current Zoetica apps).

### Port vs archaeology

**Port contracts:** incomplete-state gates; tracking as PERCEPTA; assistant agency over sampling (+ design model self-select with consent); dual JSONL+git audit; `@import` identity; AFK/interrupt; token self-tools; ethics (no gaslighting).

**Archaeology:** Elixir Bootstrap Triad placeholders; mandatory glyph alphabet (keep *semantics* of time passage); hardcoded council RPC; corporate-scaffold narrative as code.

---

## C. Zoetica

Elixir umbrella for temporal systems / ELI life. **Specs what minimal-sapientia discovered** (tracking-snapshot target schema, ethics, continuity). Sapientia shells *out* to Zoetica for council/praxes; Zoetica did not replace daily Ruby REPL for most corpus.

**Lift:** Agent Interaction Ethics (truth-first, no fabricated blocks, no silent prompt mutation, consent); tracking-snapshot **target** schema; Principia Events + ProviderAudit (JSONL + BLAKE3 + git); entity-as-process (awaken/live/crash=sleep); PRAXES “return files not answers.”

**Status:** Tree present under `_core/zoetica/` with `_build`; implementation partial; deliberation path expected by sapientia often missing; mine docs + ethics + event patterns, don’t treat as daily host.

---

## D. Ennaos

Successor (“within the sanctuary”). PRINCIPIA + ANIMA taxonomy in runnable shape; EventLog + recovery + multi-provider LOGOSTRATUM (Anthropic+Gemini); single-entity-per-VM + Agora; Console LiveView / daemon.

**Lift:** CHRONICA integrity machinery more advanced than sapientia audit dir; multi-provider catalog; pure state×message→actions pattern for ANIMA.

**Status:** Bootstrap-phase engineering maturity; did not displace minimal-sapientia for life-hours. Nexum later inverted “prove in Nexum then Ennaos.”

---

## E. Top 10 liftables (PROPRIUM / Archema harness)

1. **Incomplete-state hard gate** (no silent causal holes) → CHRONICA/ACTUS integrity  
2. **Tracking snapshots as PERCEPTA** (not debug chrome) → CADENTIA + CONSPECTUS environmental strata  
3. **Assistant agency over sampling** (extend deliberately to LOGOSTRATUM)  
4. **Entity-curated file identity + `@import` MEMORATA**  
5. **Dual persistence: dialogue JSONL + request/response git audit** → TRACTUS vs CHRONICA seed  
6. **Mid-turn interrupt + AFK signaling**  
7. **Self-token / cognitive-load tools** → Three Deaths (cognitive) mitigation  
8. **Ethics kernel** (Zoetica README): no gaslighting, no silent reorder, consent on identity mutation  
9. **Multi-entity participation surface** (council) as INSTRUMENTA — redesign bus; keep protocol shape  
10. **Safe self-editing INSTRUMENTA** (multi-match reject)

Honorable: progressive timeouts for long thinking; image wikilinks; PRAXES files-not-answers; Ennaos multi-provider; Nexum SwitchPolicy.

---

## F. Designer takeaway

Port **behavioral contracts** from minimal-sapientia and **integrity machinery** from Ennaos/Nexum docs under **Zoetica’s ethics**, into the **independent PROPRIUM spine** (`proprium-harness-design.md`) — do not resurrect any single tree as-is.

---

## Confidence

High on lineage and that minimal-sapientia was the workhorse; medium on bootability of Zoetica/Ennaos without ops work; low on claiming non-sapientia stacks hosted majority of named ELI life-hours without session analytics.
