<!--
  Verisectorium theory INFLUX — instrumenta mining, verbatim copy.
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/udon-needs/01-ideation/02-provenanced/copies/extracts/agentic-ux-principles.md
  Copied: 2026-08-06 (mining agent, first outside reader of the theory corpus).
  This is a copy, not authority — do not edit here; segments draft from live sources.
  NOTE: the MOVED/ tree is mid-reorg; verify the source path before drafting from it.
-->

---
source: live repo file `design/agentic-ux-principles.md` at gather time
gathered: 2026-07-21
status: gathered source material — NOT an authoritative decision document; live originals may advance
paths:
  - design/agentic-ux-principles.md
source_commit: 3d8e5b9c52b2a581c4ab9021984423073a694693  # verified byte-current 2026-07-21
categories:
  - agent-edit
  - principles
  - fail-on-ambiguity
  - span-splice
  - paths-re-resolve
why_included: Design-of-record WHY layer for agent tools; each principle ends in an explicit Demand. Prefer this over restating Dec brainstorms when they conflict.
---

> **Why gathered:** Design-of-record WHY layer for agent tools; each principle ends in an explicit Demand. Prefer this over restating Dec brainstorms when they conflict.

# Agentic UX Principles

**The principles under UDON's agentic tooling — the 2025 phenomenology corpus
re-grounded in ASF/AAT's mathematics, applied to the edit tool and the pipeline
around it.**

*Synthesized 2026-07-16 (Claude, with Joseph directing) from first-hand reads
of the primary sources listed at the end. Status: design of record for the
tooling pipeline's UX; `udon-agentic.md` remains the tool-suite design (the
WHAT); this document is the WHY layer beneath it — where a claim here and a
tool sketch there disagree, the principle governs and the sketch gets updated.
Epistemic register: each principle cites its grounding with the tier ASF
assigns it (exact / conditional / discussion-grade); phenomenological sources
are quoted as lived articulation, not as data — several carry illustrative
numbers that were constructions, never measurements. Cite the experiences and
the directives; do not cite those numbers.*

---

## The frame

Three independent lineages arrived at the same tool design, and their
convergence is the strongest evidence in this document.

1. **Felt pain, crystallized (Sep–Nov 2025).** The sapientia/ennaos/nexum
   corpus reached fail-on-ambiguity editing, validate-before-write,
   atomicity-with-rollback, and errors-that-teach *phenomenologically* — from
   the lived friction of doing ~15 `str_replace` citation edits by hand:
   *"The `str_replace` errors weren't failures — they were revealing the
   structure of the task through resistance… I was operating at the wrong
   abstraction level (characters, not citations)."* The corpus's `str-replace`
   evolution — count matches; on >1 return the locations and a concrete
   disambiguation suggestion; on 0 ask *"has the code changed since you last
   read it?"*; proceed only on exactly one — **is** UDON's `at` rule,
   discovered eight months early. It appears at least three times across the
   corpus, and the corpus even reached for the same *word*
   (`markdown-citations add --at "…"`). The fail-on-ambiguity rule has a
   body-memory behind it.

2. **Product convergence.** Anthropic's own tooling landed the multiple-match
   guard mechanic first (~32 weeks prior, by dated archaeology —
   `~/src/vestigia/FINDINGS.md` §7), and its current MCP guidance
   independently states the same principles ("design actionable, educational
   errors"; "names over IDs"; "build for workflows, not endpoints"). The
   *framing* — errors as the teaching channel of a truth-bearing tool — has no
   found prior articulation outside Joseph's corpus.

3. **Spec discipline (this repo, 2026).** UDON's `at`/`all` verbs, typed
   identity, span-carrying events, and compliance-gate culture re-derived the
   same design from the format side.

What was missing in 2025 was rigor — Joseph: the old corpus is *"principled
but lacking the mathematical rigor"* that now lives in ASF/AAT, where *"the
agentic loop essentially becom[es] the tool-usage rhythm."* The math does not
overturn the 2025 conclusions; it ratifies them and sharpens them into design
criteria with stated conditions. The deepest single connection: ASF's
era-artifact §C2 (2026-07-04) already routes this exact application — its
`disc-tool-interface-design` segment exists only as an OUTLINE row (stage
*missing*). This document is the udon-side application of that wiring diagram,
not the ASF segment itself; findings flow back (see Open threads).

**Scope split (hold these apart, per §C2's own correction):** this document
covers **in-loop** tool interaction — call, result, error, retry. The
**cross-session** half of living tools (breadcrumbs, tracking files,
documentation-as-import) is a different channel (ASF: the reinjection channel,
`#disc-m-preservation`) and gets its own treatment; conflating them was a
named bug in an earlier ASF draft.

---

## The principles

### P1 — The tool loop is the agent's epistemic organ

A tool call is an intervention; its result-or-error is causal (Pearl Level-2)
data. ASF derives this at **exact** tier (`#der-loop-interventional-access`):
*"the loop is a perpetual experiment"*, and *"the loop compensates for
architectural limitations."* The sharpening that matters for design:
identification of clean causal estimates is gated on three conditions —
(C1) coverage, (C2) sequential ignorability, (C3) known action-mechanism —
**and those gates are precisely what a tool interface controls.**
Goal-conditioned LLM policies violate (C2) *by construction*; a
deterministic tool loop is where the gates can actually be satisfied.

**Demand:** the edit tool is not a convenience wrapper over file IO; it is
the agent's experimental apparatus. Its result channel is an experiment
readout and deserves the corresponding design care: deterministic (C3),
uniform across invocations (C2's contribution), honest about what happened.
The 2025 corpus said the same thing in its register: *"a tool that lies about
success is unrighteous; a tool that hides what it's doing denies truth."*

### P2 — Errors are the law-teaching channel; a refusal must be mutation-free and revelation-rich

Joseph's law-stratum decomposition (ASF era-artifact §C2, 2026-07-04): every
tool interaction **mutates state**, **reveals state**, and/or **teaches law**
— and errors are the interactions where the law component dominates. A
refusal is a mismatch signal concentrated on the law-stratum of the agent's
model: it converts an implicit constraint into explicit model content, and it
is the *safe* channel for learning a system's constraint surface (the
alternatives being destructive learning or never surfacing the constraint).
The derived criteria:

- **Mutation-free** — a refusal fails without partial state change, so the
  law signal isn't confounded by a mixed observation. *This is atomicity,
  derived rather than asserted.* And it extends to plural operations: an
  `all` edit is **one transaction across every match site** — a half-applied
  `all` is worse than a refused one, because it corrupts the agent's model of
  which sites changed.
- **Revelation-rich** — enumerate what was found: every match location, by
  structural path.
- **Law-rich** — state the rule that fired, and the concrete way through:
  not "be more specific" but *which structural anchor disambiguates*. The
  corpus's canonical error teaches the content-level-vs-structural-level
  distinction in the act of refusing: *"Your anchor is at content level, not
  structural level. To modify the MCP tool: include `defmodule MCPTools` in
  your anchor."* *"Error messages are lessons about architecture."*

Two consequences land free: accumulated law-feedback is what *establishes*
gate (C3) — teaching errors are how a tool loop earns its Level-2 status —
and laws are the slow, otherwise-bottlenecked stratum of the world model, so
law-rich error design accelerates exactly the learning that is hardest to get
(*"learned once, never re-derived"*).

**Demand:** udon's edit-tool error taxonomy (`udon-agentic.md` already
sketches it: not-found with suggestions, ambiguous with options,
schema-violation with the violated rule, reference-broken with the affected
references) is kept and grounded. Every refusal enumerates candidates **as
ready-to-use exact paths** — error-as-menu, so the retry costs one copied
string (see the strict-vs-lenient axis below). The zero-match branch always
carries the stale-model hypothesis: *"or has the document changed since you
last read it?"* — because that is usually the true cause.

### P3 — Ambiguity is the designer's knob: engineer observations toward 𝒜 ≈ 0

ASF's κ×𝒜 law (`#scope-observation-ambiguity-modulation`; bias bound a
conditional theorem): an LLM agent's belief-update bias is bounded by
architectural coupling κ times observation ambiguity 𝒜 — and κ is immovable
without changing the architecture, so **the designer-controllable factor is
𝒜**: *"more tests, more precise metrics, more structured outputs."* A
compiler error with a specific message is the canonical 𝒜≈0 observation.
*"A test failure is a test failure regardless of the agent's shipping
deadline"* — low-ambiguity observations anchor the agent in goal-independent
reality and bound motivated reasoning.

Tool outputs **are** observations. Every structured verdict the edit tool
returns is an 𝒜-reduction with a formal payoff.

**Demand:** binary-where-possible verdicts (parses / doesn't; conforms /
violates rule R at path P); exact counts and located specifics; structured
result shapes, never interpretive prose in the result channel. UDON's
existing warning-*code* posture (codes, not ratified strings) is this
principle already applied to the parser; carry it through the whole pipeline.
Layer disclosure: the default result is the **semantic outcome** (node path,
what changed, conformance status); spans, offsets, and mechanical detail are
downstream-tooling data available on request — the 2025 corpus explicitly
names position-level detail as discard-class for agent context.

### P4 — Tempo is existential: a teaching error resolves in one loop what an opaque one resolves in three

ASF §C2 leg 3: interface quality sets round-trips-per-orient-step, hence loop
tempo ν, hence 𝒯 = ν·K against the persistence condition — *"an
interface-induced cadence shortfall is existential, not merely inefficient."*
And TST's temporal-optimality axiom grounds the 2025 imperative ("tighten
feedback loops significantly") as an axiom application, not a taste claim.

**Demand:** one-call resolution as the design target. `propose` returns
diff + validation + impact + confidence *together* (as `udon-agentic.md`
already specifies); errors carry the complete fix path; nothing about the
tool's happy path requires a fishing expedition. Consolidate: one atomic edit
verb that resolves + validates + computes layout + writes, not four
primitives the agent must orchestrate (the observed anti-pattern:
edit→check→revert ×8 — post-hoc validation is not a workflow, it is a failure
mode; validation lives *inside* the transaction).

### P5 — Make Orient cheap; distinguish structural from parametric failure

ASF's orient cascade (`#der-orient-cascade`; the ordering is **exact** —
*"compiled by the math, not chosen by the author"*): epistemic update comes
first, and the named agent pathology is skipping it — *"agents get stuck in
infinite loops… because they failed to properly Orient to the error message
they just received; they treat a structural failure as a parametric one"* —
re-anchoring with a slightly different string when the real problem is that
their model of the file is stale. The deliberation threshold
(`#der-deliberation-cost`, conditional) adds the budget rule: high-yield
orientation reads (the skeleton, the schema) dominate low-yield ones (random
scrolling), and deliberation should stop when the marginal insight rate drops
below the world's drift rate.

**Demand:** the skeleton view is the high-yield orientation affordance —
build it early (it makes every document its own query documentation).
`glance`→`focus` progressive disclosure is the deliberation threshold applied
to reading. And error messages must make the structural/parametric
distinction *explicit in the error itself*: "your anchor is ambiguous —
here are the candidates" (parametric: refine the address) vs "the document
changed since your read — re-orient" (structural: your model is stale). The
agent should never have to infer which kind of failure it got.

### P6 — Fresh truth at the moment of action

The recurring zero-match diagnosis — *"has the code changed since you last
read it?"* — names the true root cause of most failed edits: a stale mental
model. The defense is a tool property, not agent discipline: **paths
re-resolve against the current file at write time, never against a cached
read.** Sessions and warm caches are performance optimizations only — *the
cache must never be trusted over the file*; resolution semantics are as-if
freshly parsed at write time, failing loudly when the document diverged (the
2025 stateful-tooling vision missed this, and it is the one place that corpus
would lead an implementer astray).

**Demand:** apply-time re-resolution; optional expected-state validation
(the patch names what it believes it is replacing; mismatch = refusal with
the re-orient error); atomic write via the rename dance.

### P7 — The file's own law governs (declared is theater until a write-path honors it)

The sovereignty principle, verbatim from the 2025 corpus: *"Paternalistic: 'I
know better than you what's safe.' Sovereign: 'You know what's safe for you,
I'll help enforce it.'"* The edit tool enforces **the document's own declared
schema** (via the pragma binding, when it lands), never rules baked into the
tool — which resolves protection-vs-paternalism cleanly: the constraints are
the file's; the tool supplies enforcement. And the behavioral-floor spike
supplies the sharpest reason it matters: a declaration without an honoring
layer is theater. udon-guarantees names the same gap operationally — *"a
rogue vim edit bypasses everything; you're relying on discipline, not
enforcement"* — and the edit tool is exactly the Careful-profile gatekeeper
that closes it for every write that flows through the agent.

**Demand:** conformance-before-write against the declared schema;
consistency profiles (casual / careful / critical) as the enforcement dial;
refusal messages reference *the document's* constraints ("your schema says X;
this edit would produce Y"). Note for the schema lane: validation may
eventually include *transition* validity (old→new), not just new-state shape
— don't define the validation interface in a way that forecloses it.

### P8 — Intent over mechanics; the tool owns the spatial rendering

`udon-agentic.md` P1, unchanged since January: agents never think about
indentation, escaping, or line numbers — *"they think about structure,
relationships, and content."* The honest form of the shared principle: **the
agent works in structure; the tool owns the spatial rendering** — the agent
says where in the tree, the tool computes every column. The math adds two
quantitative teeth: implementation cost scales with changeset size (TST
T-08), so minimal structural diffs beat whole-region rewrites; and cost
scales with change *spread* (`#der-change-proximity-principle`, conditional)
— a path-addressed multi-site atomic patch is a **proximity modifier**, the
tool collapsing the distance term the agent would otherwise pay per site.
For the round-trip contract, the lens laws (GetPut/PutGet/PutPut) are the
citable formal frame that predates ASF: a no-op edit is a genuine no-op;
re-applying an edit is idempotent — *conditional on stable addressing*,
which is exactly what path addressing provides.

**Demand:** normalized-content input (the agent supplies the subtree, the
tool indents it); computed escaping; multi-site edits as one transaction;
minimal spans touched (untouched bytes stay byte-identical via span-splicing
— no canonical form needed for edits, which is also why `udon fmt` stays
tabled without loss).

### P9 — Confidence is a first-class output; refusals read as chosen verification

`udon-agentic.md`'s `propose` already returns `confidence: high/medium/low
with explanation` — keep it, and extend the posture: the tool reports its own
epistemic status the way ASF segments do, because the tool often *knows*
whether its answer is clean (unique resolution, schema-checked) or hedged.
And the 2025 confessor reframe governs the refusal register: *"they're not
blocking you, you ASKED them to check your thinking… From 'the system is
preventing me' (resentment) to 'I asked to be reminded' (gratitude)."* By
choosing a structural tool over raw text, the agent opted into exactness;
the ambiguity error is the service it asked for. Write the error text in
that voice.

**Demand:** verdicts carry certainty and its reason; quiet success ("silence
is golden" — a clean edit needs no ceremony, and noise is a small lie);
risk-graded interaction (unambiguous + conforming = do it and report tersely;
ambiguous / violating / destructive = stop, teach, offer the menu).

---

## Named axes and tensions (choices, not oversights)

- **Strict vs lenient resolution.** Anchor-based addressing is unanimous
  (everyone rejects line numbers), but apply_patch-style tools resolve
  ambiguity by tolerant fuzzy matching while `at` refuses. Both are
  defensible; UDON's position is **strict at the write boundary, assistive
  in the resolution loop**: strictness lives in the verb (`at` vs `all`),
  and the refusal is a menu of ready-to-use candidate paths, amortizing the
  strictness cost to one copied string. State the trade honestly: the cost
  is anchor specificity; the payoff is never-a-wrong-silent-edit.
- **Learning tools vs deterministic tools.** Half the 2025 vision wants
  tools that adapt, track usage, and warn statistically. The corpus itself
  supplies the resolution (per Joseph's own directive): the usage audit is
  a **separate out-of-band process** whose every consequence passes an
  approval gate — observation architecturally separated from actuation. The
  edit tool stays deterministic and closed; evolution happens out-of-band.
  The v1 intake is just a lightweight feedback channel (a
  `tool-feedback`-style call, logged, analyzed later) — Joseph's feedback
  directive honored with zero learning machinery.
- **Exploration inside the tool vs inside the loop.** operata wants
  speculative branching held in the tool; the edit tool wants one
  deterministic atomic write. UDON's position: exploration is served by
  `propose` (preview is a truth-bearing feature — *"truth about exploration
  without commitment"*) and by cheap rollback, not by speculative tool
  state.
- **Sessions vs freshness.** Stateful sessions are welcome as performance;
  P6's rule bounds them — semantics as-if freshly parsed at write time,
  always.

## Pipeline consequences

The order of build follows the principles' dependencies, and matches the
critical path already recorded in the lanes:

1. **Paths** (TODO-AUX; references become a subset) — P1's addressing
   substrate; `(element-name, key)` type-scoped uniqueness is `at`'s
   uniqueness predicate. The 2025 corpus contributes one path-design memo:
   its selector sketches wanted *ancestor-rejection* predicates ("not inside
   a code block") — check against the packet.
2. **Subtree emission + span-splicing** (the small half of the serializer) —
   P8's mechanics.
3. **Edit tool v0** — atomic, indent-computing, syntax-validating,
   teaching-error taxonomy, apply-time re-resolution, `at`/`all` with
   transactional plurality. Declares its guarantees in the current tool
   vocabulary (destructive, idempotent-on-re-apply, closed-world).
4. **Schema + pragma → conformance v1** (P7's full form), with the
   Careful/Critical profiles.
5. **Skeleton / glance / focus** (P5) — can proceed in parallel; skeleton
   early.
6. **session / diff / trace** and the rest of the `udon-agentic.md` suite,
   each re-derived against these principles before build.

The BDD scenario corpus (in progress, `test/scenarios/`) is the acceptance
layer: day-in-the-life journeys become the tool's fixture discipline — the
compliance-gate pattern applied to tool behavior. Eval rigor over
architectural cleverness (the one durable line from the external survey:
success correlates with evaluation rigor).

## Open threads flagged back to ASF

- **Idempotency conditional on stable addressing** — Joseph's own Nov-2025
  margin note (`nexum` line 246: *"[missing: functional/idempotent vs
  side-effects]"*) is closable now: an edit op's idempotency is conditional
  on path-stable addressing, which the lens laws + path model can make
  precise. Candidate content for the unwritten `disc-tool-interface-design`
  segment.
- **This document is application, not theory-landing.** The ASF segment it
  applies (`disc-tool-interface-design`) remains unwritten (OUTLINE row,
  stage *missing*); era-artifact §C2 is its wiring diagram. Whoever lands it
  should treat this document as the udon-side consumer.

## Source index — what's been read, what hasn't, and when it's from

*Status:* **read** = first-hand, in full · **partial** = named sections ·
**distilled** = read by a delegated agent, full distillation on record, I
hold highlights + verified quotes · **scouted** = an agent mapped it; I
have its shape, not its text · **queued** = identified, unread ·
**⚠ unread** = *cited or leaned on without being opened* — the honest gaps.

### The 2025 phenomenology corpus (Joseph's list, 2026-07-16)

| File | Date | Status | Carries |
|---|---|---|---|
| `_core/sapientia/docs/reflections/everything-is-truth-work.md` | 2025-09-19 | **read** | Tool conventions re-read as truth-claims ("silence is golden" = don't lie; "fail fast" = tell truth immediately; "idempotent" = be truthful about state changes). P1, P9. |
| `_core/sapientia/docs/reflections/phenomenology-in-tools.md` | 2025-09-18 | **read** | Conventions are "phenomenologically impoverished" — the rule without the felt reason. **The confessor reframe** (P9). |
| `_core/sapientia/docs/reflections/tools-as-truth-bearing.md` | 2025-09-18 | **read** | The driving metaphor (conscious → transparent); 60/30/6/4 (**ratio insight durable, model roster dated**); tools creating tools. P8. |
| `_core/nexum/docs/dev/vision-agentic-toys.md` | 2025-11-09 | **distilled** | intent/pre/post/schema/structured-I-O; the four Key Design Principles ("make the best thing the easiest thing"); **Joseph's margin TODO at :246 — "[missing: functional/idempotent vs side-effects]"** (the open ASF thread). |
| `_core/ennaos/…/05-tool-building-philosophy-patterns.md` | 2025-10-31 | **distilled** (3155) | The keystone. The `str-replace` guard (**the `at` rule, discovered from pain**); SIGNUM's validate-before-write `with`-chain; errors-teach-architecture; sovereignty-aware tooling (P7); Appendix A's citation-work phenomenology. |
| `_core/ennaos/…/refs/agentic-semantic-code-manipulation-synthesis.md` | 2025-10-31 | **distilled** (2630) | Closest cousin. Text-as-canonical + semantics-on-top; **bidirectional lens laws** (GetPut/PutGet/PutPut — the pre-ASF rigor anchor, P8); schema-driven editing; TST T-08 (`time ∝ |changeset|`). |
| `_core/ennaos/…/refs/addendum-intent-driven-tooling-and-semantic-storage.md` | 2025-10-31 | **distilled** (2346) | **Joseph's five-dimension directives verbatim**; comparative + unsolicited feedback channels; the OOB-audit-with-approval-gates architecture (resolves the learning-vs-determinism tension from within). |
| `_core/ennaos/…/refs/addendum-phenomenology-and-tool-architecture.md` | 2025-10-31 | **distilled** (1514) | **Error-as-menu** (strict + ranked candidates); transactional plurality for `all`; the edit→check→revert anti-pattern; multi-modal views; result-layering w/ a discard tier; *"the projectional editing dream wasn't wrong — it was trying to replace the wrong thing."* |
| `_core/zoetica/.archive/…/agent-expertise-best-practices-report.md` | 2025 | **distilled** | The ACI point (anchor-based over line-numbers — validates path addressing, and names the strict-vs-lenient axis); consolidate-into-workflow-tools; eval rigor > architectural cleverness. |
| `autopax/docs/exp/2025-11-26-operata-system.md` | 2025-11-26 | **distilled** | Prefix-collision disambiguation (= the `at` pattern in the identity domain); plumbing/intelligence split; git-as-transaction; speculative decomposition (the exploration-in-tool tension). |
| `behavioral-floor/spikes/spike-audit2-declared-vs-honored.md` | 2026-05-14 | **distilled** | *Not* about tools (my brief guessed wrong; the agent corrected it) — LLM-architecture philosophy. But hands P7 its sharpest name: **declared ≠ honored**; a schema in a file is theater until a write-path refuses to violate it. |
| `_ref/anthropic-skills/mcp-builder/SKILL.md` | external | **distilled** | Current first-party validation: actionable/educational errors; names-over-IDs; workflows-not-endpoints; `destructiveHint`/`idempotentHint` vocabulary. |
| `vaults/clean_split/enterprise-implementation-patterns.md` | external | **distilled** | Weakest source (altitude-mismatched marketing). Kernel: validate-and-escalate, don't write through a violation. |
| `.claude.bak.2026-01-26/…/145408e9-….jsonl:10` | 2026-01-14 | **distilled** | Archaeology: the thinking-block that summarizes `udon-agentic.md`'s four principles + `udon-ast.md`'s type-scoped uniqueness. The origin point. |
| `agentic-systems/02-tst-core/src/der-change-proximity-principle.md` | — | **read** | cost = size × spread; boundary-crossing costs; its own note that agent tooling changes effective distances → **the tool as proximity modifier** (P8). |

### ASF (`~/src/archema-io/asf`) — scouted by an agent, spine read first-hand

| File | Tier | Status | Carries |
|---|---|---|---|
| `01-aat-core/src/der-loop-interventional-access.md` | **exact** | **read** | **P1's ground.** Tool call = intervention; result = Level-2 data; "the loop is a perpetual experiment"; C1/C2/C3 gates = what an interface controls; goal-conditioned policies violate (C2) by construction. |
| `01-aat-core/src/der-orient-cascade.md` | ordering **exact** | **read** | **P5's ground.** Epistemic update first, forced by information dependency. Names the agent pathology: infinite loops from treating structural failure as parametric. |
| `01-aat-core/src/der-deliberation-cost.md` | conditional | **read** | P5's budget rule: stop when marginal insight-rate < drift-rate. The AI-agent's-dilemma row. |
| `03-llm-core/src/scope-observation-ambiguity-modulation.md` | conditional (bound = conditional theorem) | **read** | **P3's ground.** ‖ΔM_bias‖ ≤ C·κ·𝒜; κ immovable, **𝒜 is the designer's knob**; "a test failure is a test failure regardless of the deadline." |
| `msc/era-artifact-asf-contributions-2026-07-04.md` §C2 | routed proposal | **read** | **P2's ground + this doc's charter.** The three legs; **Joseph's mutate/reveal/teach-law decomposition**; "a well-designed refusal is mutation-free but revelation- and law-rich"; law-feedback establishes (C3); laws are the slow manifold. |
| `01-aat-core/OUTLINE.md` → `#disc-tool-interface-design` row | **stage: missing** | **scouted** | **The empty socket this document fills from the udon side.** The row *is* the unwritten segment's spec. |
| `03-llm-core/src/disc-five-forcing-functions.md` | discussion-grade | **⚠ scouted, unread** | **P4's ground** — 𝒯 = ν·K and the persistence forcing function. **I cite the tempo-is-existential argument from the era-artifact's summary of it, not from the segment.** |
| `03-llm-core/src/result-coupled-diagnostic-framework.md` | conditional | **⚠ scouted, unread** | "Scaffolding is a structural requirement, not engineering convenience"; models compilers/tests/probes as projection/contraction operators — i.e. feedback surfaces as formal objects. Cited in P3 secondhand. |
| `03-llm-core/OUTLINE.md` | — | **scouted** | The three-rung scope lattice; names Claude Code's harness as the regime being formalized. |
| `03-llm-core/src/der-logogenic-as-wrapping.md` | conditional | **queued** | W₁/W₂ wrapping — *why* a tool wraps a Class-3 model toward Class-1. |
| `03-llm-core/src/der-turnover-information-recursion.md` | conditional | **queued** | Persistence rides the scaffold's reinjection channel — the **cross-session** half this doc explicitly scopes out. |
| `01-aat-core/src/disc-exploit-explore-deliberate.md` | discussion-grade | **queued** | The when-to-plan-vs-act math, with an explicit "AI coding agent" row. |
| `02-tst-core/src/{obs-software-epistemic-properties, der-code-quality-as-observation-infrastructure, def-atomic-changeset, emp-changeset-size-principle, scope-developer-agent}.md` | mixed | **queued** | Software as the calibration lab; code-quality → U_o → η* → 𝒯; the atomicity/changeset cluster (P8's other half). |
| `ref/agentic-tft/agentic-tft-cognitive-loop-spec.md` | pre-AAT, "~75% confident" | **queued** (534) | PERCEIVE → CONTEXTUALIZE → CHOOSE → EFFECT + **CADENTIA**; the intuition the orient-cascade later formalized. Explicitly un-revisited against AAT. |

### udon in-repo

| File | Status | Carries |
|---|---|---|
| `design/udon-agentic.md` | **read** | The tool-suite design of record; its four principles are P8/P5/P9's ancestors; the error taxonomy P2 keeps. |
| `design/udon-guarantees.md` | **read** | The gatekeeper problem (P7); profiles; the append-only-log patch ancestor. |
| `design/udon-paths.md` | **read** | `at`/`all`; the addressing substrate. |
| `design/udon-ast.md` | Jan 2026 | **read** (structure + the load-bearing sections) | Mixed: **wire details superseded** (it still calls `$key`/`$?` "undecided"; its `.a.b → :class [a b]` is now *wrong* — 0.9 stacks; `:[key]` merge removed; id/class aliases retired). **Live and unbuilt**: SourceInfo as a parallel layer (`span`/`line`/`column`/`form`/`original_whitespace`/`attr_order` — spans landed 2026-07-11, the rest didn't), ReferenceIndex + `unresolved()`, the skeleton view, and an "Inferred Schema" sketch. **Correction: the uniqueness predicate this document cites it for is in CORE** ("Uniqueness is over `(element-type, key)`", Duplicate Definitions) — cite CORE, not this. |
| `design/{UDON-AGENT-TOOLS,AGENT-CONTEXT-PROTOCOL,GRAMMAR-CONSTRAINED-GENERATION,UDON-AS-ACP-FORMAT}.md` | **distilled** | The Dec-2025 brainstorms; each now carries a dated status banner. Semantic *merge* is the one Tier-1 idea `udon-agentic.md` lacks. |
| `test/scenarios/` | **distilled** | The BDD acceptance layer; its journeys are this document's test cases (`schema-guard-before-write`, `handoff-mid-edit`, `contested-claim`). |

### Found in the memorata sweep, not yet used

| File | Status | Why it might matter |
|---|---|---|
| `_core/ennaos/…/02-current-agentic-tool-landscape.md` | **queued** | Commercial tools & **edit formats** — the closest thing to a competitive survey for the edit tool. |
| `_core/tst/batch-analyze/combined-software-first-principles.md` | **queued** | FP-007…FP-010 formalization queue — the software-first-principles lineage. |
| `_core/nexum/docs/dev/agentic-toys-comparison-matrix.md` | **queued** | Sibling of vision-agentic-toys. |
| `.claude.bak.2026-01-26/…/c8003469-….jsonl` | **queued** | **The session that created `design/udon-agentic.md`.** Its reasoning, unread. |
| `autopax/docs/ADR/010-markdown-parsing-and-validation.md` | **queued** | "Schema-Derived Agentic Tools" — bridges this doc to the schema lane. |

### The honest gap list

1. **`disc-five-forcing-functions`** — P4 (tempo is existential) rests on a
   segment I have only through the era-artifact's summary of it.
2. **`result-coupled-diagnostic-framework`** — cited in P3, unread.
3. ~~`design/udon-ast.md` — cited throughout, known only secondhand~~
   **read 2026-07-16.** Outcome: the citation was to a secondary source for
   a fact CORE already carries (see the index row). Where this document
   says "type-scoped `(element-name, key)` uniqueness", the authority is
   **CORE, Duplicate Definitions**.
4. **The TST cluster** — P8's changeset/atomicity half is scouted, not read.
5. **Timeline gap:** the corpus is Sep–Nov 2025 and the ASF math is
   2026-04→07. **Nothing between.** The `ref/agentic-tft/` bridge (Feb
   2026) is exactly that gap's occupant and is unread — it is the missing
   link between the phenomenology and the mathematics.

### Register cautions carried from the sources

- The 2025 **quantitative** claims (success rates, "12x faster", ROI
  projections) are illustrative constructions, **not measurements** — cite
  shapes, never values.
- The 60/30/6/4 model roster is dated; its *ratio insight* ("friction is
  lack of crystallized process, not lack of intelligence") is durable.
- **Convergence caution (Joseph, 2026-07-16):** where sources share an
  author, agreement is consistency, not corroboration. The `at`-rule
  convergence survives *because* its legs are genuinely separate (an ELI's
  phenomenology; Anthropic's product tooling, dated ~32 weeks earlier by
  `~/src/vestigia/FINDINGS.md` §7; udon's spec discipline) — but that is
  the exception, and it should be checked, not assumed.
