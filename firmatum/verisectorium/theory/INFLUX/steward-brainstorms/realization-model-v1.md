<!--
  Verisectorium theory INFLUX — working draft, iterated IN PLACE (unlike the
  verbatim baseline beside it, which stays frozen).
  Sources: instrumenta-and-realization-2026-08-06.md (steward baseline, verbatim)
  + steward responses 1-9 (in-chat, 2026-08-06) + coord refinements.
  Register: still pre-validation brainstorm-grade; nothing here is ratified.
  Attribution marks: [J] steward-originated · [C] coord-proposed · [J+C] co-shaped.
  Destination: rows/segments in theory/OUTLINE.md once the shape settles.
-->

# Realization model — v1 (working)

How verisectoria get launched, tooled, and managed. The organs (OUTLINE Part II) say what an instance *is*; this says what a *realized* instance is made of and how the pieces are classified.

## 1. Instrumenta — the cognition axis (I)

*[J structure, C labels + grounding]*

| Level | Label | Definition | Examples |
|---|---|---|---|
| I₀ | **substrate** | OS/system deterministic & enforced, incl. de-facto substrate (git, filesystem) | git object store, chmod seals |
| I₁ | **deterministic** | Fully deterministic scripts/CLI; target: ~70% of inner-loop work *(hypothesis-grade target, [J])* | `bin/term`, `lint-outline`, `fmt-md` core |
| I₂ | **gated-generative** | Deterministic shell with a bounded logogenic kernel; badly under-used currently [J] | `fmt-md --math` (needs a new name) |
| I₃ | **scaffolded-agent** | Logogenic agent under scaffold: discrete role, task+result shapes, verification mechanism, bounded cognition-goals, parameterization | workflow-launched verify agents, schema'd extractors |

**The I₂ design law [C]: logogenic proposes, deterministic disposes.** The embedded cognition only generates candidates; a deterministic gate (render-equality, per-line acceptance, schema validation) decides what lands. This is what separates I₂ from "a script that calls an LLM" — without the gate that's just I₃ with worse ergonomics. It's also why I₂ is safe enough to deserve heavy use: its failure mode is a rejected candidate, not a landed error.

**ASF class note [C].** I₂/I₃ are instances of the wrapping construction ( [[der-class-coercion-via-wrapping]]): a Class 3 (Coupled) logogenic component coerced toward Class 1 behavior by an external deterministic scaffold. I₂'s deterministic gate is a W₁-flavored structural bound (the scaffold controls what crosses); I₃'s task+result shapes and verification mechanism are W₂-flavored behavioral bounds. The Brooks's-Law tempo cost of wrapping (more calls per macro-step) is the honest price, paid for leakage control. Casual labels here deliberately do not claim asf's formal scopes [J]; this note is the bridge for whoever formalizes it.

**Tool/actor boundary test [C]:** a *tool* is invoked with its *how* fully (or almost fully) specified; an *actor* receives intent and owns its how. Orchestration fails the tool test — correctly excluded [J]. The I₃ ↔ autonomous-agent gradient [J] has an exact coordinate: **what fraction of Σ (the strategy — the how) is supplied versus self-formed.** A de-novo auditor has I₃-like shapes on its *outputs* (working-dir protocol, FINAL spec) while owning its *process* — output-scaffolded, process-autonomous.

## 2. Instrumenta — invocation × mutation (replacing the single T axis)

*[J first pass; C split after the non-MECE flag was confirmed]*

The original T₂ (stateful) is a property a tool *has*; T₁/T₃/T₄/T₅ are ways it is *invoked*. Two orthogonal axes:

**Trigger axis** — how it runs:

| | Label | Examples |
|---|---|---|
| G₁ | **invoked** | CLI default; agent tool-call |
| G₂ | **state-triggered** | file/dir watch, post-tool-use hook, pre-commit |
| G₃ | **time-triggered** | cron, `/schedule`, `/loop` — the natural home of out-of-band audit/consistency instruments [J] |
| G₄ | **continual** | daemon, server, orchestrator — usually offering G₁–G₃ interfaces into itself [J] |

**Mutation axis** — what invoking it does:

| | Label | Examples |
|---|---|---|
| M₀ | **observing** | read-only: `ls-segments`, lint --check, pending queries |
| M₁ | **idempotent-mutating** | converges on re-run: `fmt-md`, `align-slug` |
| M₂ | **event-mutating** | non-idempotent, appends/advances state: `relata decide`, `git commit`, `term decide` |

The mutation axis wires directly into Organ V: M₂ instruments are exactly the ones that want write-membrane treatment and event records; M₀ instruments are exactly the ones safe to hand any agent without prose constraints (the enforceable form of "analysis only" — constrain by tool-set, never by prose).

## 3. Actors and the promotion lattice

*[J roles + promotability; C valve grounding]*

| Actor | Definition | Defining property |
|---|---|---|
| A_steward | Ultimately responsible and accountable; not always present | Holds the valve; source of fiat (marked as fiat when exercised) |
| A_coord | One or more concurrent logogenic agents in active session | **Valve access** — the direct steward line (Organ VIII); can surface decisions in real time |
| A_delegate | Self-actuated non-tool actors (general-purpose agents, de-novo auditors) | Owns its how; no direct steward line — communicates through artifacts and the coord |

**Promotion is a ladder of deliberation-space grants [J+C]:** I₃ → A_delegate (grant ownership of the *how*) → A_coord (grant the direct line). The harness supports the second grant natively (steward switches over and starts talking directly [J]). The classification is therefore positional, not essential — entities move between positions, and the same underlying agent can occupy different positions in one day.

## 4. The SOP store — norms as a verisectorium

*[J diagnosis + serving-forms list; C population/serving framing]*

The observed serving forms of norms (SOP files, agent roles/system prompts, task templates, CLAUDE.md fanouts, memories, decision records, TODO-embedded directives, in-chat steward directives, accidental prescriptive phrases, top-level orientation files, example-implication) are **views over a norm population that currently has no population store**. That is the diagnosis behind every observed symptom: the same rule living in five places at different staleness; "accidental prescriptive phrases, impossible to distinguish" being norm-atoms with no home and therefore no epistemic state; asf's SOP fanout consolidating once and then stalling — the predicted behavior of a corpus with no influx membrane and no feedback loop.

So: **SOPs are a verisectorium — one level deep, no further [J].** The regress guard holds because the recursion terminates by construction: the SOP store's own meta-process is already fully specified by [[post-self-governance]] — *participant feedback is the only source/sink of the meta-process* — and that feedback channel is realized concretely as **the SOP store's influx** (§5). No third level exists to need governing: the meta-meta-process is one sentence, and its amendment channel is the same influx. The three scattered thoughts (self-governance postulate ↔ one-level recursion limit ↔ SOP-influx-as-feedback-channel) are one mechanism seen from three sides.

A serving-form obligation follows: the many serving surfaces (memories, CLAUDE fanouts, prompts) become *projections* of the store — generated or pointer-formed where possible — rather than independent authorities. The lookup-tool gap [J: "without some kind of specialized sop lookup tool it already gets unwieldy"] is then an Organ VII corpus-verbs item: `sop <topic>` beats symlink archaeology.

## 5. Store anatomy — the triplet with designated influxes

*[J, superseding the earlier quartet]*

```
canon    + influx⁺   primary influx designated "base material" — the
                     pre-verisectorium substrate the instance is founded on
                     (gathers, notes, prior corpora), plus ordinary influxes
                     (spike→canon, audit→canon) as needed
lexicon  + influx*   terms / lexicon entries; influx optional
SOP      + influx⁺   primary influx designated the meta-feedback channel
                     (post-self-governance realized), and — possibly as a
                     second designated influx — the upgrade channel (§7)
```

**Lexicon is every store's embedded foundation [J — the DDD realization].** Corpus and vocabulary never work independently; they co-evolve or drift apart, so every instance embeds its lexicon as a first-class store with its own physical layout — the "ubiquitous bounded shared vocabulary" of DDD made structural. This dissolves the outline's carve-tension (b): vocabulary is *both* an organ of the theory *and* an embedded instance in every deployment.

**Term marking [J+C, settled for now]:** `[[term/scaffolded-agent|scaffolded agent]]` locally; `#vivarium/term/scaffolded-agent` cross-member — riding the existing `#asf/term/<slug>` namespace scheme rather than minting a `term:` dialect. Open sub-question [J]: how the lexicon develops *via* the outline, and how first-definitions are lexically indicated in segments.

**A direction asymmetry worth keeping visible [J flag]:** canon has an *authored* outline over atoms; lexicon has a *generated* view over atoms (segments→outline directioned). The triplet thus contains both view-directions Organ VI distinguishes — a completeness check, and a reminder that "outline" names two different acts.

## 6. Frontdoor — two orientations and the attestation

*[J correction + previously-unwritten intention; C labels offered]*

Orientation is two distinct prerequisites with different failure modes, plus a third element that is neither:

| Element | Offered label | What it establishes | Failure mode it prevents |
|---|---|---|---|
| Domain orientation | **doctrina** *(what the corpus teaches)* | The agent knows what the canon already knows — the deep domain-specific knowledge in the segments | Confidently writing format-correct segments in ignorance of landed knowledge — the *actual* reason for the vivarium gate [J: agents imitate segment form fine once examples exist; what they miss is the doctrine] |
| Process orientation | **praxis** *(how we work here)* | The agent knows the meta: formats, SOPs, instrumenta usage, write semantics | Meta-errors: wrong cadence, wrong membrane, hand-editing generated views |
| Attestation | **sponsio** *(Roman law: the solemn, binding promise)* | A genuine speech-act moment: the agent attests, without requiring steward dialog, that it will conform to named standards and behaviors to the best of its ability | The gap between having-read and having-committed — the orientation that leaves a *record of undertaking*, not just a quiz pass |

The vivarium gate as implemented covers doctrina only; the unstated intention [J, previously only in handwritten notes — first durable capture here] is to evolve it to cover praxis **and** sponsio. Sponsio's design constraints: it must be performable solo (no steward round-trip), it must name the specific standards being undertaken (not a blanket pledge), and it should leave a durable record the way the orient-token does — a per-session speech-act with the same context-lifetime scoping as the quiz pass. The workflow-restatement gate is recognizable as praxis + proto-sponsio for briefs; the naming-cycle's restatement question 5 ("what level of effort… are you expected to exhibit") is proto-sponsio verbatim.

Plain-word fallbacks if the Latin register doesn't take: domain-orientation / process-orientation / undertaking. *(Labels are discussion-grade candidates; Organ II's naming criteria apply before any of them canonicalize.)*

**Evidence specimen (2026-08-06, single-estate, near-controlled).** The theory-misfire and the current founding were the same task on the same estate with the same model class; the operative difference was orientation depth — the misfire's lead read partially and by selection (its own disclosure records the SOPs never opened, theory met through summaries), the refounding was preceded by steward-directed full first-hand reading. The misfire's own agent, reviewing the refounded outline, attributed the categorical quality gap to "truly orienting instead of 'sufficient' orientation." Same-estate evidence, and the reviewer is not independent of the compared work — but it is the sharpest specimen yet for the doctrina claim, and for "sufficient-feeling orientation" being the trap the gate exists to catch.

## 7. Upgrade propagation — instances subscribe

*[C mechanism, J-endorsed, + J's direct-line caveat]*

The kit is itself a living collection and instances *subscribe*: a new kit capability arrives at each existing instance as an **influx item on the instance's own membrane** (the SOP store's upgrade channel, §5), adjudicated locally — adopt, adapt, or decline-with-record. Migration offers are payloads, never pushes; instance sovereignty and a decision record either way.

**The direct-line exception [J]:** some changes can't wait for passive influx adjudication — compatibility-critical ones, especially standard-tooling usage changes (a shared instrument renames a verb, changes a flag, alters output shape that automation parses). Two grades:

- **advisory** — ordinary influx payload; adjudicated whenever the instance next drains its queue.
- **compat-critical** — surfaced at the *frontdoor* (orientation-time, where every session already passes) and flagged in the instance's config as un-dispositioned, so no agent works a session without seeing it; still locally adjudicated, but the surfacing is eager, not queued.

The grade is declared by the upgrade's publisher and contestable by the instance — mis-grading advisory-as-critical is attention spam and gets fed back through the same channel.

## 8. Default replacements — the affordance law

*[J principle + example; C normative statement]*

**For every high-frequency manual file operation in an instance, ship the affordance that makes the right thing the easiest thing.** Generic read/write against text files is the fallback, not the interface: relata instead of hand-editing bib entries; `term` instead of hand-editing lexicon files; and exploration verbs replacing `ls`/`head` archaeology — [J's worked example] `vera ls-segments`: canonical-outline order with orphans shown *after* (instrument honesty — absence displayed, never hidden), mtimes, frontmatter metadata, line/token counts, possibly per-section.

Delivery shape: **global, pwd-aware/root-aware instruments** à la git and relata (`vsect` / `vera` / `term` / `sop`), parameterizable by named verisectorium — not per-project script trees. The relata adoption evidence is the argument: agents pick up corpus verbs instantly and never learn layouts; bowl verbs beat better file tools.

## 9. Launch/bootstrap (held at baseline shape, pending steward continuation)

From the baseline, not yet reworked here: essential template · question/answer discovery [J has ideas queued] · tentative-vs-strong choice marking · core vocabulary mapping / basic config · udon-configured triggers, gates, fluxes · evolving bespoke schemas · upstream theory links (`[[vsect/form-slug-form-kinds]]`-style, scattered through template SOPs) · primary tracking stores · telos/ethos inclusion.

## 10. Open slots

- **Exemplars' home** — the baseline's bigger-picture list names exemplars (commits, segments, epistemology, delegation) and the corpus-teaches-by-example force argues they're a first-class store or designated canon subset, not an afterthought. Unplaced.
- **Telos / ethos** — appeared in the baseline's final line before truncation; placement undecided (candidate: frontdoor's doctrina layer, or a fourth small store).
- **"There also may be…"** — the steward's interrupted continuation never arrived in-band; the baseline moved to `.integrated/` 2026-08-06 with this thread still open. Any continuation now arrives as a fresh brainstorm file in this directory and folds here.
- ~~**`fmt-md --math` rename** — flagged [J]~~ *resolved 2026-08-06: the tool is now `md-press` (global memory updated; `fmt-md` on PATH is a deprecation shim), and `--math` now operates on parser-identified prose sites with per-chunk gates — structurally out of reach of markdown/code, i.e. the I₂ "gated-generative" law implemented more deeply. `md-press --math` is the I₂ exemplar to cite in segments.*
- **~70% I₁ inner-loop target** — hypothesis-grade; worth an eventual observation-store measurement rather than adoption as norm. *(The udon-needs 60/30/6/4 crystallized-process thesis is its lineage, not its corroboration — same author throughout; see `../instrumenta/REGISTER-RULING.md`, which governs all label inheritance from that gather.)*
