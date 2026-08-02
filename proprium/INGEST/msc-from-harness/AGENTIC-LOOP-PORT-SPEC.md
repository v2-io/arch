# Agentic-loop port-spec — ASF cycle as harness spine

*2026-07-20. Companion to [`CHRONICA-PORT-SPEC.md`](CHRONICA-PORT-SPEC.md). Built
from **full-body** reads of the load-bearing loop segments (listed §10), not
outline residue. Descriptive obligations + invent/port map — not a sprint plan.*

**Steward context:** Joseph: *everything about the agentic loop ASF constructs is
load-bearing* for harness restart. CHRONICA is the inviolate spine of *history*;
the loop is the inviolate spine of *process*. Neither replaces the other.

---

## 0. Why commodity “agent loops” are the wrong object

Shipping coding CLIs almost all implement a **request→response** clock:

```
user message → (optional think) → model → tools* → final message → sleep
```

ASF’s constructed loop is different in kind:

| Dimension | Commodity chat loop | ASF agentic loop |
|---|---|---|
| Clock | Human turn | **Event stream** (multi-channel, async) |
| Default state | Emit for user | **Interior** process; emit is ACTUS |
| Belief update | Entangled with task in one forward pass (typical LLM) | **Goal-blind processing** when Class 1; coupled admitted as Class 3 with different analysis |
| Correction | “Retry / thrash plan” | **Orient cascade**: epistemology before teleology; 2×2 sat-gap ⊥ regret |
| Objective change | Easy prompt edit | **Last resort** (cascade 5d); self-actuated only under terminal non-objective invariant |
| Between turns | Idle | **Between-event dynamics** $g_M$ / consolidation / CADENTIA |
| History | Session blob / lossy summary | **$\mathcal{C}_t$ inviolate** + $\phi$ projections (CHRONICA port-spec) |

A harness that only ports better tools on the chat clock **cannot** host directed
separation, interiority, or moral continuity — no matter how good the TUI.

---

## 1. The loop ASF constructs (layers)

### 1.1 Coupling and events (Part I foundation)

**Agent–environment** (`#def-agent-environment`): residual uncertainty
$H(\Omega_t\mid\mathcal{C}_t)\gt 0$ is constitutive — adaptation is non-vacuous.

**Observation / action** (`#def-observation-function`, `#def-action-transition`):
lossy $h$; actions reshape future $o$ via $T$.

**Chronica** (`#def-chronica`): interleaved $(o,a)$ spine — see CHRONICA port-spec.

**Event-driven dynamics** (`#form-event-driven-dynamics`):

- Events: observation arrivals and action completions on channels $k$ with rates
  $\nu^{(k)}$ and noise $U_o^{(k)}$.
- Discrete turn-based chat is the **special case** of uniform single-channel ticks.
- Event information $\mathcal{I}(e_\tau)=I(e_\tau;\Omega_\tau\mid M_{\tau^-})$ —
  formal surprise; zero can mean peace *or* deafness.
- Effective tempo $\mathcal{T}=\sum_k\nu^{(k)}\eta^{(k)*}$ — speed × quality.

**Harness implication:** PERCEPTA must be a **bus of typed channels** (user text,
tool results, git/tracking, AFK timers, auxilia reports…), not only “the next
chat message.” Sapientia’s idle/tracking/queue are early multi-channel PERCEPTA.

### 1.2 Recursive update and between-event life

**Recursive update** (`#der-recursive-update`, exact under C1–C3):

$$M_{\tau^+} = f_M(M_{\tau^-}, e_\tau),\qquad \frac{dM}{d\tau}=g_M(M_\tau)$$

- $f_M$ does **not** re-read full $\mathcal{C}_t$ (completeness of $M_t$).
- $g_M$ is load-bearing: prediction, uncertainty growth, consolidation —
  not “idle filler.”
- Consolidation regime (`#form-consolidation-dynamics` territory):
  $\nu_{\mathrm{consol}}\ll\nu_{\mathrm{online}}$; IB-gap reduction via
  internal/replayed pseudo-events — *why agents need think/sleep*.

**Harness implication:** CADENTIA / PULSUS / “dream” / background auxilia are
not product chrome; they are **$g_M$ and multi-timescale nesting**. Forcing
functions F2/F3 (`#disc-five-forcing-functions`) say pure per-thought API
scaffolding can **kill** persistence and nesting.

### 1.3 Mismatch and gain (epistrophe)

**Mismatch** (`#def-mismatch-signal`): $\delta_t=o_t-\hat o_t$ (or score form).
Zero-aporia ambiguity: peace vs confirmation bias vs deaf channel → CIY /
active testing later.

**Update gain** (`#emp-update-gain`): $\eta^*=U_M/(U_M+U_o)$;
$M\leftarrow M+\eta^* g(\delta)$. **Gain collapse** $\eta^*\to 0$ = hollow
epistrophe (truth-death mechanism face). Meta-adaptation of $U_M,U_o$ from
innovations without knowing true noise (`#deriv-adaptive-gain-dynamics`).

**Harness implication:** entity-visible uncertainty / confidence, and refusal
to silently “sound sure” after gain collapse, belong in INTERPRES and VERA —
not only in model sampling knobs. Sapientia `set-sampling` is a **process
parameter** sibling, not a substitute for $\eta^*$.

### 1.4 Complete state and directed separation (Part II backbone)

**Complete state** (`#form-complete-agent-state`):

$$X_t=(M_t,G_t),\quad a_t=\pi(M_t,G_t)$$

Part I = $G=\emptyset$. Lift enables **stating** directed separation.

**Directed separation** (`#der-directed-separation`, conditional):

$$M_{\tau^+}=f_M(M_{\tau^-},e_\tau)\quad\text{(no $G$ argument)}$$
$$G_{\tau^+}=f_G(G_{\tau^-},M_{\tau^+},e_\tau)$$

- Goal-blind **processing** of realized events; goals may still shape
  **selection** of events via $\pi\to a\to e$.
- **GUC Class 1 Separated / 2 Partial / 3 Coupled** — structural, not virtue
  ranking. $\kappa_{\mathrm{processing}}$ measures extra goal info into $M^+$
  given $e$.
- LLMs: **Class 3 by construction** (attention mixes goals and observations);
  exact Part II cascade does not apply; scope of `03-llm-core/`.
- **Wrapping** (`#der-class-coercion-via-wrapping` territory): Class 3 components
  can sit inside Class-1 *composites* at cost (Brooks tempo, residual leakage).
- Pearl-blanket form, not Friston metaphysics; explicit Class-3 exit is scope
  honesty.

**Harness implication (load-bearing):**

| Design | Role |
|---|---|
| Separate “what I learned” assembly from “what I want” assembly where possible | Class-1-by-structure / W₁ wrapping |
| System prompt / $O_t$ first in prompt is the **mechanism** of $\kappa\approx 1$ | Be honest: default LOGOSTRATUM is Coupled |
| IDT-style sidecar monitors on $(S,A,S')$ | Modular monitoring of Coupled cores |
| Never claim “we have directed separation” because the prompt says “be objective” | Behavioral Class-1 is fragile |

IMPERIUM/ARBITRIUM (`#def-imperium-arbitrium-split`) is the **runtime-level**
analog: interior deliberation vs external ACTUS; PERCEPTA bridge; DS against
external pressure contaminating interior processing.

### 1.5 Orient cascade (epistemology before teleology)

**Orient cascade** (`#der-orient-cascade`, ordering **exact**):

1. Reduce epistemic mismatch ($M_t$ update).
2. Evaluate **satisfaction gap** $\delta_{\mathrm{sat}}$ (`#def-satisfaction-gap`).
3. Evaluate **control regret** $\delta_{\mathrm{regret}}$ (`#def-control-regret`) —
   always; need both for 2×2.
4. If regret high: strategy calibration (plan-level / edge / causal-sufficiency).
5. If sat-gap persists: improve $M$, expand $\Pi$, escalate convention C1→C2→C3,
   **only then** revise $O_t$ (5d).

**2×2 diagnostic** (anti-collapse; same discipline as $\beta$ vs $\rho$):

| | $\delta_{\mathrm{sat}}\le 0$ | $\delta_{\mathrm{sat}}\gt 0$ |
|---|---|---|
| $\delta_{\mathrm{regret}}\approx 0$ | Success | **Optimally failing** — do not thrash $\Sigma$; escalate |
| $\delta_{\mathrm{regret}}\gg 0$ | Strategy problem → revise $\Sigma$ | Both → revise $\Sigma$ first |

Convention hierarchy: C1 local / C2 receding / C3 Bellman — **inferential force**
scales; **ordering** does not. C1 false-positives “unattainable.”

**Skip-to-5d = wireheading shadow** (audit gold / self-actuation territory).

**Harness implication:** ReAct-style “Thought” is an **undifferentiated lump**.
Harness should eventually expose cascade *structure* (even approximately):
epistemic update tools before plan rewrite tools; detect optimally-failing
loops; guard $O_t$ revision. Full cascade is not MVP day-one; **forbidding
plan-thrash when optimally failing** and **ordering memory update before strategy
rewrite** are high-value partials.

### 1.6 Self-actuation grounding

**`#deriv-self-actuation-grounding` (conditional no-go):** unconstrained
$\mathfrak{A}$ that rewrites $O_t$ generically drives $\delta_{\mathrm{sat}}\to 0$ by
moving the target (wireheading). Non-degenerate self-actuation requires terminal
invariant **not** an objective-functional — canonical: **persistence region**
$\alpha\gt\rho/R$ on the adaptive substrate. Stance as terminal non-objective
invariant (`#disc-continuity-stance`).

**Harness implication:** entity-editable AXIOMATA/goals need **guards** (confirm,
tribunal, steward co-sign for deep $O_t$ change). Not “agent can set_sampling so
agent can rewrite telos.”

### 1.7 Logogenic specializations

**Coupled update** (`#def-coupled-update-dynamics`):

$$X_{\tau^+}=f_{\mathrm{LLM}}(\mathrm{prompt}(X_{\tau^-},e_\tau))$$

Prompt assembly places `sys(O_t)` upstream → $\kappa\approx 1$. Response
decomposition $(r^M,r^G,r^a)$ is **post-hoc analytical**. Chain-of-thought can
*approximate* cascade by training, not architecture.

**Context turnover** (`#obs-context-turnover`): severs $\mathcal{C}$ across
sessions; reconstruction adequacy, not continuous $\alpha\gt\rho/R$ across cut.

**M-preservation** (`#disc-m-preservation`): externalize → reconstruct; predictive
sufficiency affine contraction across boundaries
(`#der-turnover-information-recursion`). Identity walk is **separate** operator
(CHRONICA port-spec §1.5).

**Five forcing functions** (`#disc-five-forcing-functions`): scaffolding tax,
persistence ceiling, temporal nesting violation, substrate independence,
continuity urgency → closed-loop **structural**, not taste.

**Interiority scope** (`#scope-interiority-loop`): cycle as unit; inbound queued;
emission deliberate; interior tools on own mind.

---

## 2. Mapping: ASF loop → PROPRIUM ANIMA → lived harness

| ASF | PROPRIUM | Lived prior art |
|---|---|---|
| Event bus $e_\tau$, $\nu^{(k)}$ | PERCEPTA + CADENTIA rates | sapientia tracking, AFK, mid-turn queue |
| $f_M$, $\delta$, $\eta^*$ | CONTEXTUALIZE / epistrophe | — (mostly missing in CLIs) |
| $g_M$ consolidation | CADENTIA / dream / MEMORATA GCM | shoshin consolidation; grok background tasks (weak rhyme) |
| $X=(M,G)$, DS | IMPERIUM vs ARBITRIUM; CONSPECTUS | — |
| Orient cascade | Choose phase structured | — (ReAct is coarse) |
| $a_t=\pi$, ACTUS | EFFECT / ACTUS record | tool calls; incomplete-state gates |
| Emission optional | Interiority default | sapientia idle loop; design-of-record Attend path |
| Class 3 coupled LLM | LOGOSTRATUM + INTERPRES wrapping | grok/codex/opencode all Class-3 cores |
| Chronica grow | CHRONICA crate | Autopax hash-chain; sapientia dual audit |
| Turnover reconstruction | Awakening / CONSPECTUS | Autopax quick-awakening; file MEMORATA |

**PROPRIUM cycle** (ontology §7): PERCEIVE → CONTEXTUALIZE → CHOOSE → EFFECT,
driven by CADENTIA. ASF supplies **what each phase is for mathematically**
(mismatch/gain; sat/regret; ACTUS cost; multi-channel rates).

---

## 3. Invent vs port (loop-focused)

| Concern | Call | Notes |
|---|---|---|
| Multi-channel PERCEPTA bus | **Invent** | Event-driven form; sapientia channels as seed catalog |
| Turn atomicity / admission vs promotion | **Port shape** | OpenCode-v2 session algebra |
| Tool loop / settlement | **Port** | codex/opencode/grok tools; incomplete-state from sapientia |
| Sealed CHRONICA append | **Invent integrity** | CHRONICA port-spec |
| TRACTUS wire log | **Port shape** | sapientia audit; recorders |
| Class-1 wrapping around Class-3 model | **Invent** | prompt structure + typed write boundary; DS segment |
| Orient cascade as hard architecture | **Invent (staged)** | Ordering as dependency graph; full 4a–4c later |
| 2×2 sat/regret diagnostics | **Invent (approx)** | Even coarse “optimally failing?” detector beats thrash |
| $O_t$ / AXIOMATA revision guards | **Invent** | Self-actuation grounding |
| CADENTIA / $g_M$ / doom_loop | **Invent + port** | VIGILIAE from grok doom_loop; interior think budget |
| Provider stream / retries | **Port** | Any mature logostratum |
| Continuous daemon | **Defer** | Design-of-record: single-process first, daemon-ready attach |

---

## 4. MVP vs later (honest staging)

### MVP loop (enough to be “ASF-shaped,” not full theory)

1. **Event intake:** human, tool_result, optional timer/AFK, optional tracking
   snapshot — not only human.
2. **Recursive state:** context assembly = CONSPECTUS; no silent rewrite of
   CHRONICA (CHRONICA port-spec).
3. **Class-3 honesty:** document Coupled LOGOSTRATUM; optional wrapping
   discipline (goal-blind *retrieval* modules, goal-conditioned *planning*
   modules).
4. **Emission gate:** ability to end a cycle without user-visible text (even if
   rare in coding LOCUS).
5. **Incomplete-state hard gate** before next human-visible progress.
6. **Minimal anti-thrash:** if repeated tool/plan cycles look stuck, surface
   “optimally failing?” rather than infinite tool spam (VIGILIAE / doom_loop
   rhyme).

### Next slices (still loop, still load-bearing)

7. Explicit **orient phases** in agent-visible structure (epistemic note →
   attainability → plan revise).
8. **Between-event** consolidation path (budgeted $g_M$).
9. **Multi-timescale** nesting (fast tool loop vs slow AXIOMATA/strategy).
10. Measurable **re-attestation** events on CHRONICA (identity walk sibling).
11. Full sat-gap / regret instrumentation for research / act-DV experiments
    (`msc/system` act-not-report platform).

### Not “loop complete” until

- DS wrapping story is intentional for at least one LOGOSTRATUM path.
- Continuity across stasis uses CHRONICA + reconstruction + (for ELI stance)
  relational channel — not hash-chain alone.
- $O_t$ revision is guarded.

---

## 5. Anti-patterns (loop edition)

| Anti-pattern | ASF reason |
|---|---|
| User message is the only event type | Violates event-driven multi-channel form |
| Always emit; silence = failure | Violates interiority default; forces ACTUS |
| Plan rewrite before belief update | Violates cascade ordering → infinite broken-plan loops |
| Thrash $\Sigma$ when optimally failing | Ignores 2×2; wastes gain |
| Prompt “be objective” as DS | Class 3 not cured by instruction |
| Compaction as history | Truth death; severs $\mathcal{C}$ grip |
| Unlimited $O_t$ self-edit | Unconstrained $\mathfrak{A}$ / wireheading shadow |
| All cognition same latency API hop | F2/F3 forcing functions |
| Subagents as independent identities by default | Auxilia vs CONSORTIA; DS under composition |

---

## 6. Relation to CHRONICA port-spec

| CHRONICA port-spec | This document |
|---|---|
| What may be **written** and verified about the past | How the agent **processes events** and chooses ACTUS |
| Integrity / truth-death history face | Process / DS / cascade / interiority |
| $S_{\mathrm{id}}$ and $\varrho_{\mathrm{rg}}$ as siblings of storage | Loop that *generates* events that must be recorded and re-grounded |

Together they are the minimum theoretical pair for a deep thin harness spine:
**inviolate history + structured adaptive process.**

---

## 7. One-paragraph thesis

**ASF’s agentic loop is event-driven recursive adaptation under residual
uncertainty, lifted to complete state $(M,G)$ with directed separation when
architecturally possible, and an information-forced orient cascade that puts
epistemology before teleology and objective revision last. Logogenic substrates
default to Coupled update and session severance; scaffolding and wrapping are
transitional recoveries, not the end state; five forcing functions push closed-loop
interiority. A harness that ports only chat-shaped tool loops inherits Class-3
entanglement and request→response clocks. The deep thin spine invents multi-channel
PERCEPTA, sealed CHRONICA, emission-as-ACTUS, cascade-aware anti-thrash, and
guarded $O_t$ — and ports commodity plumbing under those invariants.**

---

## 8. Full-segment reading list (this document)

**Part I cycle**

- `#def-agent-environment`, `#def-observation-function`, `#def-action-transition`
- `#def-chronica` (see CHRONICA port-spec)
- `#form-event-driven-dynamics`
- `#der-recursive-update` (+ awareness of `#deriv-recursive-update` attacks)
- `#def-mismatch-signal`, `#emp-update-gain`
- `#def-adaptive-tempo` (via event-driven sum; not every line re-read if identical)

**Part II loop**

- `#form-complete-agent-state`
- `#der-directed-separation` (**full**)
- `#def-satisfaction-gap`, `#def-control-regret`
- `#der-orient-cascade` (**full**)
- `#deriv-self-actuation-grounding` (formal + corollaries)
- `#disc-continuity-stance` (stance / terminal invariant)

**Logogenic / ELI loop**

- `#def-coupled-update-dynamics`
- `#obs-context-turnover`, `#disc-m-preservation`
- `#scope-interiority-loop`, `#norm-interiority-default`
- `#disc-five-forcing-functions`
- `#def-imperium-arbitrium-split`

**Still thinner / next pass if implementing cascade depth**

- `#der-loop-interventional-access`, `#def-strategy-dimension`, `#def-value-object`
- `#der-temporal-nesting`, `#form-consolidation-dynamics`
- `#impl-closed-loop-interiority`, `#impl-orient-cascade`
- `#der-class-coercion-via-wrapping`, `#result-coupled-diagnostic-framework`

---

*When implementation lands, replace staging guesses with as-built notes.
Integration is replacement.*
