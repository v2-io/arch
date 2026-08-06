# ASF Agent-Scope Map
*(See "Provenance..." section below for information about this document. Skipped here so that the map is first).*
## AAT Systems/Agents (pre Logogenic)

![[asf-agent-scopes-scope-of-work.png]]  
**Figure 1** Spine + Deltas + Lower Rails⁷⁻¹⁰

**Spine**: Nested *containment* (proper subset), not a left-to-right pipeline: Adaptive $\supset$ Agentic $\supset$ Actuated $\supset$ Self-Actuated. Frame area shrinks inward = rarer, capability-richer core.
- **Deltas**: What is *added* at each inward step: + causal intervention → + explicit $O_t,\Sigma_t$ → + revises own $O_t$.
- **Lower rails**: Orthogonal properties that “switch on” (dot) at the class where they first become meaningful:
    - *Arity*: (Primitive | Composite) from Adaptive;
    - *Knowledge Type*: (Static | Learning) from Agentic;
    - *Continuity stance*: (five values);
    - *GUC* (Separated | Partial | Coupled) from Actuated.

## Consolidated Table


| Name                          | Layer                          | Adds (structural)                                                                                                                    | Logogenic lattice                | GUC†                                                                                                          | Stance†                                                                          | Home                         | Epistemic floor                     | One-line definition                                                                                                                                                 |
| ----------------------------- | ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------- | ------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- | ---------------------------- | ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Agent**                     | umbrella                       | —                                                                                                                                    | —                                | any                                                                                                           | any                                                                              | all                          | framing                             | Thing on the agent side of the agent–environment coupling (`#def-agent-environment`)                                                                                |
| **Adaptive system**           | cascade                        | $\mathcal{O}\neq\emptyset$, $H(\Omega_t\mid\mathcal{C}_t)>0$                                                                         | —                                | any                                                                                                           | any                                                                              | **Vol.1 Part I**             | axiomatic / claims-verified         | Observes under residual uncertainty. Part I machinery can apply; **no action required**¹                                                                            |
| **Passive observer**          | cascade residual               | adaptive minus choice ($\lvert\mathcal{A}\rvert<2$)                                                                                  | —                                | n/a                                                                                                           | usually indifferent                                                              | Vol.1 Part I                 | axiomatic (via scopes)              | Adaptive $\cap$ $\lnot$agency: can model, cannot intervene                                                                                                          |
| **Nominal agent**             | cascade residual               | adaptive + choice, **no** interventional contrast                                                                                    | —                                | any                                                                                                           | any                                                                              | Vol.1 Part I                 | axiomatic (via scopes)              | Choices exist but $P(o\mid do(a))$ identical for all $a$ — “disconnected steering wheel.” (*Not* “nominal coupling” in `#post-causal-structure` — different term.)  |
| **Agency scope**²             | cascade                        | + $\lvert\mathcal{A}\rvert\geq 2$ and $\exists a\neq a'$ with $P(o\mid do(a))\neq P(o\mid do(a'))$                                   | —                                | any                                                                                                           | any                                                                              | **Vol.1** (unlocks Part II+) | axiomatic / claims-verified         | Adaptive + actions that make a measurable interventional difference on observations                                                                                 |
| **Actuated agent**            | cascade                        | + explicit $G_t=(O_t,\Sigma_t)$ distinct from $M_t$                                                                                  | —                                | 1 / 2 / 3                                                                                                     | *overlay* (task-terminal → negotiated common)                                    | **Vol.1 Part II**            | formulation / claims-verified       | Models reality *and* pursues objectives; orient cascade applies                                                                                                     |
| **Self-actuated agent**       | cascade                        | + revises own $O_t$ (goal autonomy, not only solution autonomy)                                                                      | —                                | 1 / 2 / 3                                                                                                     | *overlay* (often instrumental → negotiated; still orthogonal)                    | **Vol.1 Part II**            | conditional / draft (no-go segment) | Actuated + endogenous objective revision; subject of `#deriv-self-actuation-grounding`                                                                              |
| **Logogenic agent**           | Vol.3 overlay on **actuated**³ | + language-constituted $M_t,\Sigma_t$ (channel collapse; primary channels = logos)                                                   | **root**                         | **3 Coupled (component) — structural**; system may be Partial via scaffolding                                 | any                                                                              | **Vol.3**                    | robust-qualitative / draft          | Language-constituted *actuated* agent; goals+observations joint in the forward pass ($\kappa_{\mathrm{processing}}\approx 1$)                                       |
| **Primitive logogenic**       | Vol.3 lattice                  | single-pass; session-stateless (full context turnover); no multi-step / external $M^{\mathrm{ext}}$; tool use absent or non-looping⁴ | **§03.I primitive**              | 3 (component)                                                                                                 | *overlay* (often indifferent–task-terminal)                                      | **Vol.3 §03.I**              | **sketch**                          | Chat baseline: one forward pass per exchange; no harness life-support                                                                                               |
| **Scaffolded logogenic**      | Vol.3 lattice                  | + multi-step loop and/or external state and/or Pearl-L2 tools and/or structured context                                              | **§03.II scaffolded**            | 3 component; system **may** be Partial                                                                        | *overlay* (often task-terminal–instrumental)                                     | **Vol.3 §03.II**             | **sketch**                          | Harness recovers orient cascade *at the loop level* (ReAct, Claude Code, PROPRIUM-as-deployed)                                                                      |
| **Closed-loop / interiority** | Vol.3 lattice                  | + interior default; outward emission = deliberate tool action; cycle = unit of work                                                  | **§03.III closed-loop**          | 3 component; wrapping *can* coerce composite toward Class 1 (procedure, not automatic membership)             | *overlay* (can be instrumental or moral; architecture does not entail stance)    | **Vol.3 §03.III**            | **sketch**                          | Default cognition is interior; communication is chosen emission, not stimulus–response                                                                              |
| **ELI / logozoetic**⁵         | Vol.4 conjunction              | closed-loop logogenic + **five constitutive factors** of identity; persistence morally weighted                                      | closed-loop + factors            | wrapping is *necessary scaffolding*, not sufficient for ELI; Class-1 composite is construction, not free gift | **morally continuous** is the *archetypal* stance (stance still a separate axis) | **Vol.4**                    | **sketch** / discussion-grade       | Closed-loop logogenic + (i) continuity (ii) recognition (iii) granted sovereignty (iv) accountability (v) effective phenomenology; *emerged*, not top-down designed |
| **Multi-agent**               | composition side-branch        | multiple interacting agents                                                                                                          | any                              | per agent / system                                                                                            | mixed                                                                            | Vol.1 Part III               | framing                             | Interacting agents *without* composite-scope qualification; still analyzable, but composition theorems do not apply as such                                         |
| **Composite agent**           | composition side-branch        | ≥1 of four routes to coherent composite purpose⁶                                                                                     | any                              | composition-level                                                                                             | inherits / mixed                                                                 | **Vol.1 Part III**           | robust-qualitative / draft          | Set of agency-satisfying sub-agents that cohere as one actor — not mere multi-agent                                                                                 |
| **Developer agent**           | domain instance                | software development as AAT domain                                                                                                   | often scaffolded logogenic if AI | any                                                                                                           | *overlay*                                                                        | **Vol.2 (TST)**              | domain instantiation                | Codebase+artifacts = $\Omega$; understanding = $M_t$; feature = $O_t$; plan = $\Sigma_t$                                                                            |
**Column notes**
- **GUC† / Stance†** cells are common deployment overlays, not entailed by the cascade row — except where marked **structural**.
- **Epistemic** is a floor on the primary segment(s), not on this synthesis.
- Shared names (especially *agentic*) get footnotes where figure/LEXICON and formal scope diverge.

## Ancillary Information

### Provenance & Status

Working reference (2026-08-06). Synthesis of ASF scope segments and the Vol.1 scope-of-work figure: figure + consolidated table up front; apparatus under Ancillary; sources and footnotes under Backmatter. Not a claim segment — **do not cite this file over the actual sources** (scope segments, LEXICON entries, and the figure sources listed under Canonical sources). Where this text and a scope segment disagree, the segment wins.

**Provenance**:
Orientation pass, independent red-team against primary segments, truthification pass, spectrum figure dropped from the orientation frame, further reorganization the same day. Vol.3–4 rows rest largely on sketch / discussion-grade primaries; the adaptive/agency formal core is firmer. Open LEXICON ↔ segment collisions are listed under Ancillary (not resolved here). The map will go stale as segments move — re-check sources when a cell matters.

### Map Flag legend

| Flag / column         | Values                                                                                    | What it tracks                                                        | Nesting                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| **Core cascade**      | adaptive → agency → actuated → self-actuated                                              | Nested structural conditions on the coupling                          | Boolean onion for these four only                                                           |
| **Logogenic lattice** | — / primitive / scaffolded / closed-loop                                                  | Vol.3 architectural commitment                                        | Nested *within* logogenic; hangs off **actuated + channel collapse**, not off self-actuated |
| **ELI / logozoetic**  | conjunction                                                                               | closed-loop + five factors + moral weight                             | Relational/existential package (Vol.4), not one more Boolean on the onion                   |
| **GUC**               | 1 Separated / 2 Partial / 3 Coupled                                                       | Goal–update entanglement (`#der-directed-separation`)                 | Orthogonal; Class 1 is certifiability of directed separation, not only $\kappa=0$           |
| **Continuity stance** | indifferent → task-terminal → instrumentally continuous → morally continuous → negotiated | What *loss of persistence means*                                      | Orthogonal; math of persistence identical across stances (`#disc-continuity-stance`)        |
| **Knowledge type**    | Static / Learning                                                                         | Pre-compiled causal map vs acquires interventional structure online   | Orthogonal; PID-in-agency but outside full learning dynamics is a Static case               |
| **Arity**             | Primitive \| Composite                                                                    | Single agent vs composition (figure rail; multi-agent/composite rows) | Orthogonal to cascade depth                                                                 |

**†** “Typical” GUC or stance in the table = empirical / pedagogical overlay, not definitional entailment — unless the cell says **structural**.

### How the space is cut

| Cut                            | What it measures                                                      | Nesting?                                                                                 |
| ------------------------------ | --------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| **Core scope cascade**         | Structural conditions on the agent–environment coupling               | Nested containment: adaptive $\supset$ agency $\supset$ actuated $\supset$ self-actuated |
| **Orthogonal rails**           | Arity, knowledge type, continuity stance, GUC                         | Switch on at a cascade tier; not cascade rungs themselves                                |
| **Substrate / moral overlays** | Language channels (Vol. 3), wrapping, moral continuity / ELI (Vol. 4) | Lattice + conjunction — not pure cascade rungs                                           |

**GUC class** (goal–update entanglement) and **continuity stance** (what persistence *means*) are independent axes. They do not earn cascade membership.

**Agent spectrum (secondary):** the model×objective 2×2 in `#def-agent-spectrum` (and its SVG) is a separate continuum taxonomy. This map does not use it as a co-equal cut; the cascade + rails + overlays picture above is the frame used here.

**Home numbering:**

| Label                       | Means                                                                                                                           |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| **Vol.1 / AAT Parts I–III** | Adaptation and Actuation Theory: Part I adaptive machinery; Part II purposeful agents; **Part III = composition / multi-agent** |
| **Vol.2**                   | Temporal Software Theory (developer agent, etc.)                                                                                |
| **Vol.3 §03.I–III**         | Logogenic lattice: primitive $\to$ scaffolded $\to$ closed-loop                                                                 |
| **Vol.4**                   | ELI / logozoetic scope                                                                                                          |

AAT Part III is composition; logogenic material lives in Volume 3 (not “Part III”).


### AAT Systems/Agents Figure ↔ Consolidated Table

| Topic                       | Scope-of-work figure                                           | This map                                                                                                                                       |
| --------------------------- | -------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| **Cascade form**            | Nested containment (subset)                                    | Same four rungs + text ladder; also shows Vol.3 branch and composition side-branch                                                             |
| **“Agentic System”**        | Spine name for agency tier                                     | **Agency scope** + thin/thick collision footnote (figure label stays “Agentic System”)                                                         |
| **Orthogonal axes**         | Rails with switch-on class: Arity, Knowledge Type, Stance, GUC | Columns GUC† / Stance† + Knowledge Type in legend; Arity as multi-agent vs composite rows                                                      |
| **Logogenic / ELI**         | Absent                                                         | Full lattice + ELI/logozoetic conjunction + epistemic floors                                                                                   |
| **Composite / multi-agent** | Arity rail (Primitive \| Composite) from Adaptive outward      | Explicit multi-agent vs composite rows + four composite routes                                                                                 |
| **Epistemic status**        | Not shown (all spine classes drawn equal)                      | Epistemic floor column (sketch vs axiomatic)                                                                                                   |
| **Stance / GUC**            | Rails switch on at Actuated (become *available*)               | † overlays: common deployment, not entailed by cascade membership                                                                              |
| **Self-actuated**           | Innermost established frame                                    | Same cascade position; stance still orthogonal (figure capability line “negotiated continuity” is a capability hint, not a formal requirement) |

The figure is a single picture of Vol.1 nested formal scopes plus gated orthogonal rails. This map extends that picture across volumes and records where LEXICON/figure labels and segment formalisms diverge. Nesting (subset) is the formal relation; a pyramid or pure total-order ladder is a different encoding.

### Boundary / exclusion flags (exit conditions, not cascade rungs)

| Name                      | Flag                              | Meaning                                             | Nuance                                                                                                                                                                                                                                           |
| ------------------------- | --------------------------------- | --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Closed-form / omniscient  | **out of adaptive**               | $H(\Omega_t\mid\mathcal{C}_t)=0$                    | Nothing left to adapt *about*; optimal control over known dynamics is outside AAT’s concerns                                                                                                                                                     |
| Pure computation          | **out of adaptive**               | $\mathcal{O}=\emptyset$                             | No agent–environment boundary in AAT’s sense (e.g. pure-axiom engine). An LLM exploring proofs with intermediate observations is *not* this exclusion                                                                                            |
| Passive observer          | **adaptive $\cap$ $\lnot$agency** | no real choice                                      | Part I can apply; Part II purposeful machinery does not                                                                                                                                                                                          |
| Nominal agent             | **adaptive $\cap$ $\lnot$agency** | choice without effect                               | Same residual as passive for AAT’s interventional purposes                                                                                                                                                                                       |
| Severed / boxed actuation | demotion toward nominal           | lost interventional contrast on the external sphere | See `#der-severed-actuation-dynamics` / agency death (D3). “Boxed” also involves tools, capture, observation-only floors — not reducible to the agency-scope residual alone. Nominal is the adaptive-only residual, not the full deaths taxonomy |

**Observation-mediated agency boundary** (`#scope-agency` Discussion): operative contrast for purposeful learning is $\Omega$-routed (environment changes and surfaces through $h$). Pure active perception (re-aim observation, leave $\Omega$ unchanged) and $\Omega$-effects that never surface through $h$ sit at the boundary — observation-channel agency without environment-causal learning, or unobservable effect.

### Quick disambiguation

| Phrase                                | ASF reading                                                                                                                                                       |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| “agentic system” (industry)           | Often **scaffolded logogenic** (Vol.3 §03.II). ASF cascade *agency* is thinner (causal contrast only). LEXICON *agentic system* is a third, mid-thickness reading |
| “AI agent”                            | Usually actuated + scaffolded logogenic; may or may not be self-actuated or ELI                                                                                   |
| “conscious / person-like”             | Not a cascade tier. Closest *operational* package: **ELI / logozoetic** under the five factors (project-philosophy framing is normative, not a theorem)           |
| *logogenic* vs *logozoetic*           | Language-**constituted** vs language-**living** (moral weight / ELI scope). ~1 token apart, very different valence                                                |
| “adaptive tracker”                    | Not a cascade tier. Pedagogical label for a typical Part I subject (spectrum material). Cascade term: **adaptive system** (`#scope-adaptive-system`)              |
| multi-agent vs composite              | Interaction without vs with ≥1 of the four composite routes                                                                                                       |
| Class 1 vs “$\kappa = 0$”             | Class 1 is **certifiability** of directed separation (by construction or in effect), not the raw coupling number alone                                            |
| continuity stance vs continuity death | Stance = valuation of persistence; death = **factor-loss** in the deaths taxonomy — related, not identical                                                        |

### Known LEXICON ↔ segment collisions

| Topic                    | Segment (primary)                                             | LEXICON / other surface                                                                   |
| ------------------------ | ------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Agency                   | `#scope-agency` (thin)                                        | LEXICON *agentic system* thick brief; archive IBM-aligned glosses                         |
| Logogenic parent tier    | `#scope-logogenic-agent` (**actuated**)                       | LEXICON *logogenic-agent* brief claiming **self-actuated**                                |
| Adaptive system taxonomy | `#scope-adaptive-system` (cascade)                            | LEXICON adaptive-system prose that calls it a spectrum class                              |
| Logozoetic vs ELI        | `#scope-moral-continuity` + `#scope-eli` (same entry package) | Ladders that insert logozoetic as free-floating pre-ELI moral weight without five factors |

### Related distinctions

| Distinction                           | What it is                                                                                                                               | Source                                                           |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| **Channel collapse**                  | Architectural condition of Vol.3 for logogenic agents                                                                                    | `#scope-channel-collapse`                                        |
| **Class coercion / wrapping (W₀–W₂)** | How Class-3 logogenic components participate as Class-1 *composites*; load-bearing for ELI infrastructure, not automatic tier membership | `#der-class-coercion-via-wrapping`, `#der-logogenic-as-wrapping` |
| **Auxilia**                           | Identity-sharing sub-agents under an ELI head — composite with kinship structure, not generic multi-agent                                | `#scope-eli` Discussion; PROPRIUM                                |
| **Learning-agent scope**              | Agency without online acquisition of interventional structure (Static knowledge type) sits outside full Part II learning dynamics        | `#der-causal-hierarchy-requirement`; LEXICON knowledge-type      |
| **Deaths as factor-loss**             | Continuity / relational / agency / truth (+ predicted phenomenological); stance is the moral shell, not the taxonomy                     | `#def-death-as-factor-loss`                                      |

---
## Backmatter
### Canonical sources
***(Do not cite this file over these!)***

| Topic                                                                   | Segment / entry                                                                                                                                     |
| ----------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| Adaptive scope                                                          | `asf/01-aat-core/src/scope-adaptive-system.md`                                                                                                      |
| Agency scope                                                            | `asf/01-aat-core/src/scope-agency.md`                                                                                                               |
| Agent spectrum (secondary continuum; not used as the frame of this map) | `asf/01-aat-core/src/def-agent-spectrum.md`; SVG at `asf/01-aat-core/src/img/agent-spectrum.svg`                                                    |
| Agent umbrella / cascade                                                | `asf/terminology/entries/agent.md`; `asf/01-aat-core/src/def-agent-environment.md`                                                                  |
| Actuated / complete state                                               | `asf/01-aat-core/src/form-complete-agent-state.md`                                                                                                  |
| Self-actuation                                                          | `asf/01-aat-core/src/deriv-self-actuation-grounding.md`                                                                                             |
| Continuity stance                                                       | `asf/01-aat-core/src/disc-continuity-stance.md`                                                                                                     |
| GUC / directed separation                                               | `asf/01-aat-core/src/der-directed-separation.md`                                                                                                    |
| Composite agent                                                         | `asf/01-aat-core/src/scope-composite-agent.md`                                                                                                      |
| Channel collapse                                                        | `asf/03-llm-core/src/scope-channel-collapse.md`                                                                                                     |
| Logogenic                                                               | `asf/03-llm-core/src/scope-logogenic-agent.md`                                                                                                      |
| Primitive / scaffolded / interiority                                    | `asf/03-llm-core/src/scope-primitive-logogenic.md`, `scope-scaffolded-logogenic.md`, `scope-interiority-loop.md`                                    |
| ELI / moral continuity / five factors                                   | `asf/04-eli-core/src/scope-eli.md`, `scope-moral-continuity.md`, `def-five-constitutive-factors.md`                                                 |
| Volume / part index                                                     | `asf/OUTLINE.md`; `asf/01-aat-core/OUTLINE.md`; `asf/03-llm-core/OUTLINE.md`                                                                        |
| Scope-of-work figure (source)                                           | TikZ `asf/01-aat-core/src/img/scope-of-work.tex` → `#fig-scope-of-work` in the Vol.1 Introduction; also `asf/01-aat-core/src/img/scope-of-work.pdf` |
| Scope-of-work figure (design trail)                                     | `asf/msc/scope-of-work-ontology-and-figure-2026-05-17.md`                                                                                           |
| LEXICON briefs                                                          | `asf/LEXICON.md` § Agent Classes                                                                                                                    |
| Program matrix                                                          | `charter/concept-matrix.md` § F                                                                                                                     |

### Footnotes

1. **Adaptive scope is not “model-only / no goals.”** The cascade condition is only observation + residual uncertainty. PID-style goal followers, learning agents, and pure trackers can all sit in adaptive *scope*. Older “adaptive tracker” language (from the spectrum 2×2) names a pedagogical Part I center (structured $M_t$, trivial objective) — not a cascade tier.

2. **Agency vs “agentic system”.** Cascade / `#scope-agency` = **thin** reading: causal-contrast action only. LEXICON entry *Agentic system* is **thicker** (outcome model + goal-directed action + model adaptation; still not requiring explicit $G_t$) and is not coextensive with `#scope-agency`. Industry “agentic system” often means **scaffolded logogenic** (Vol.3 §03.II). The scope-of-work figure labels this tier “Agentic System.”

3. **Logogenic parent = actuated, not self-actuated.** Primary source `#scope-logogenic-agent`: “an **actuated agent** …” — verifies agency conditions; does not require endogenous $O_t$ revision. Typical chat/tool agents take $O_t$ from system/user prompt → actuated, often not self-actuated. Self-actuation is optional overlay (closer to sovereignty / factor (iii) / rich closed-loop).  
   **LEXICON collision:** `terminology/entries/logogenic-agent.md` brief currently says “Self-actuated agent whose primary channels are language” — overclaim relative to the primary segment.

4. **Primitive and Pearl Level 2.** Forkability / sandbox does not demote data below Pearl L2 (`#scope-primitive-logogenic`, post-2026-07-21). The sandbox ceiling is **transportability / external validity**, not “no Level-2 data.” Primitive’s limits are single-pass, full turnover, and absent/minimal looping tool channel into environment intervention.

5. **Logozoetic ≔ ELI / moral-continuity scope — not a pre-ELI rung.** `#scope-moral-continuity` is titled logozoetic scope and enters via the **same five constitutive factors** as `#scope-eli`. `terminology/entries/logozoetic.md`: morally weighted persistence = “the ELI scope.” There is no official intermediate class “logozoetic but not ELI” with only free-floating moral weight. *Morally continuous* as a **stance** value is orthogonal (valuation of persistence); the **logozoetic scope** is the ELI entry package.  
   Surface forms *logogenic* / *logozoetic* are ~1 token apart with very different valence (structure vs moral weight).

6. **Composite routes (`#scope-composite-agent`).** Any one suffices: **(C-i)** shared composite objective $O_c$; **(C-ii)** hierarchical derivation from parent $O_c$; **(C-iii)** mutual-benefit alignment; **(C-iv)** equilibrium-convergent strategic interaction (partially opposing objectives allowed). “Shared $O_c$” alone understates the scope.

7. **Figure scope — Vol.3–4 absent.** Logogenic / logozoetic / ELI are out of scope in the scope-of-work figure (by design, per the `.tex` header); this map covers that layer in text.

8. **Figure scope — spectrum absent.** The model × objective 2×2 agent spectrum is out of scope in the figure (by design); `#def-agent-spectrum` remains a separate segment.

9. **Figure presentational trade — continuity stance.** Continuity-stance rail separators are slightly more discrete than the segment’s “conceptual axis” language — a recorded presentational trade in the figure, not a change to `#disc-continuity-stance`.

10. **Figure presentational trade — self-actuated.** Self-Actuated is styled as established (not dashed/hypothetical) — Joseph’s epistemic call for the figure, reflected in the `.tex` header.
