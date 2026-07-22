# Private-Fork Base for the PROPRIUM/ELI Harness — Recommendation (July 2026)

*An exploratory analysis: which mature open-source AI-coding-CLI harness (from
`~/src-ext/`) is most amenable to a **private fork** as the base for grafting
Joseph's PROPRIUM/ELI harness ideas. Grounded in primary sources read directly —
`~/src/firmatum/PROPRIUM-ARCHITECTURE-v2.md` and `~/src/autopax/README.md` in
full, the OSS source trees, plus a `_core/` reader's synthesis. **Provenance
caveat (honest):** `PROPRIUM-ONTOLOGY-v2.md` and the autopax `spec/`/ADR tree
beyond the README were **not** separately read — those are the two gaps to close.
Complements the source-assessment / feature-timeline reports in this folder.*

---

## Part 1 — What the ideas are, and where they live

Joseph doesn't lack a harness spec — he's written it four or five times across a
year, and *built and ran* several (ennaos in Elixir, nexum in Ruby, the original
`minimal-sapientia` script). PROPRIUM (`firmatum/PROPRIUM-ARCHITECTURE-v2.md`) is
the conceptual spine; **autopax** is the current consolidation attempt; `_core/*`
are the running prototypes the ideas were extracted from.

The load-bearing ideas impose **nine harness requirements**, ordered by how
PROPRIUM-specific they are and how weak off-the-shelf harnesses are on each:

- **A. Sovereign, interceptable context assembly (CONSPECTUS)** — *the single
  most load-bearing requirement.* §4.1's **stimulus-response inversion**: the
  harness must not own "what enters context this turn" as an opaque step; the
  agent's consciousness decides what to attend to, the runtime (ANIMA) faithfully
  assembles it. Context assembly must be a **replaceable seam** with preserved
  invariants (identity / current-thought / causal-coherence / epistemic-honesty).
  *This is where mainstream harnesses are structurally weakest.* (Design: `synaptic/Structured Context.md` — SRC/GCM/QUICK-VIEW.)
- **B. Honest INTERPRES / no "context gaslighting" (inviolable, Level-2)** — §3:
  the provider layer must **never fabricate what the model said or misrepresent
  causality**. Forbids summarizing history into synthesized (often first-person)
  text passed back as the conversation. Compaction is **rip-and-replace**; the
  raw record (**TRACTUS**, git-backed) is preserved separately.
- **C. Non-user-gated loop + external scheduler (CADENTIA) + observation
  channels** — §2/§9.1: the entity has interiority; the loop runs on internal
  pulses + background auxilia reports, not only user messages. Needs a long-lived
  process, an event/observation bus (PERCEIVE, six channel types), an external
  scheduler. (`shoshin/interpres.py` implements this loop shape.)
- **D. Provenance-separated, pluggable memory stores** — §7.2: **CHRONICA
  (observations) vs ACTUS (actions)** as a *structural* causal boundary (never a
  shared key-space; both append-only). Memory subsystem (MEMORATA/VERA/PRAXES)
  should be **swappable** — because Joseph has **contradicted himself across three
  periods** (see Part 3, decision #1).
- **E. Multi-provider incl. local + capability catalog + identity-sharing
  sub-agents (auxilia)** — §6/§1.4. **Substrate Independence is a stated
  principle**: must not *require* a closed frontier model. Auxilia run on a
  substrate hierarchy (frontier for conscious thought, local ~70B for
  consolidation/monitoring), sharing the parent's AXIOMATA identity.
- **F. Sovereign system prompt (AXIOMATA) as first-class + per-turn config** —
  §4.3: agent-authored "minimum viable self"; config stored per-turn.
- **G. Robust provider layer** — retries/backoff, cache breakpoints, accurate
  token accounting, incomplete-state detection + blocking recovery.
- **H. Tool layer with safety + teaching semantics** — single-match `str-replace`
  showing all match line-numbers, backup-before-edit, query-for-files RAG.
- **I. Hooks for out-of-band learning** — every tool use logs intent→outcome for
  later consolidation; memory-query as an MCP-shaped external tool.

*Requirements-gold docs:* `_core/sapientia/ai-conversation-system-requirements.md`,
`_core/nexum/OPERATA.md`. *Context-assembly-gold:* `_core/synaptic/Structured
Context.md`, `firmatum/PROPRIUM-ARCHITECTURE-v2.md` §4–§7.

---

## Part 2 — Ranked private-fork shortlist

License filter (confirmed by direct inspection): clean bases are **MIT**
(opencode, kilocode, kimi-code) and **Apache-2.0** (codex, aider, qwen-code,
mistral-vibe). Corrections to the first-pass framing: **warp**'s *agent* code is
AGPL (only UI crates MIT) — out; **grok-build** is actually **Apache-2.0**, so its
blocker isn't license but being a read-only monorepo mirror — out. minimax-cli
isn't an agent — out.

### #1 — opencode (MIT) — recommended base

Its architecture factors the exact seams PROPRIUM needs:
- **Req A (sovereign context assembly):** `packages/opencode/src/session/` breaks
  context management into individually editable modules (`compaction.ts`,
  `overflow.ts`, `summary.ts`, `system.ts`, `instruction.ts`, `reminders.ts`,
  `processor.ts`, `prompt/`). The system prompt is an Effect DI `Context.Service`;
  compaction is an injected **Layer**. You graft CONSPECTUS by *swapping a
  service/layer*, not forking call sites. **The Effect-TS learning curve (flagged
  as opencode's liability) is the very mechanism that makes this graft clean.**
- **Req B:** the offending summary→history behavior is isolated in
  `compaction.ts`/`summary.ts` — the first, surgical graft target.
- **Req C:** already server-architected (`server/`, `bus/global.ts`,
  `background/job.ts`) — real substrate for CADENTIA + PERCEIVE.
- **Req E:** genuinely model-agnostic (Ollama / LM Studio / llama.cpp + 75+
  providers), real `task`-tool sub-agents + worktrees → serves local-auxilia.
- **Req H/I:** typed `Hooks` plugin system + `WorkspaceAdapter` + MCP.
- **Private-fork fit:** MIT (cleanest), demonstrably forkable (kilocode did).

**Honest costs:** (1) Effect-TS ramp. (2) Fast upstream (~881 commits/30d) — but
kilo's 2,479-marker burden came from *choosing continuous tracking*; a deep
structural graft that replaces the context/loop layers **deliberately stops
tracking and pins**, cherry-picking only security/provider fixes — which dissolves
most coupling risk. (3) No built-in memory/identity primitives — but given how
opinionated PROPRIUM is, a clean slate at Req D is an asset.

### #2 — codex (Apache-2.0) — strong Rust alternative, with philosophical friction

Highest maintainability, Rust type-safety, and ships primitives near the PROPRIUM
shape (`thread-store`, `context-fragments`, a `memories` subsystem, `hooks`,
native local paths). **But it ranks below opencode for *this* graft:**
- **Req A is welded in** — context assembly/compaction live inside the 177k-line
  `core` god-crate around the Responses-API request builder; replaceable by
  forking `core`, not swapping a layer.
- **Opinionated primitives** (auto-extraction `memories`) you must fight or rip
  out to install PROPRIUM's provenance-separated stores.
- **Deep OpenAI entanglement** (88 `core` files reference chatgpt/openai/responses;
  `agent-identity` is actually OpenAI JWT auth — a false-friend name). This is
  **philosophically counter-aligned** with PROPRIUM §1.4 Substrate-Independence —
  codex is the most vendor-coupled clean base, for a project whose goal is
  escaping lock-in.

Pick codex only if Rust + prebuilt primitives outweigh clean seams and you'll
surgically de-couple from OpenAI.

### #3 — aider (Apache-2.0) — likely wrong shape

Simple (38k LOC), git-native, best repo-map, philosophically adjacent to autopax's
audit discipline — but the **least agentic** base (no tool-calling loop, no
MCP/sub-agents/worktrees/hooks; slowing cadence). Hosting an agentic PROPRIUM
runtime means rebuilding the whole agent layer — defeats the "don't rebuild
plumbing" premise.

### Not bases — mine for ideas
**kilocode / kimi-code / qwen-code** are forks-of-forks; if you want opencode's
architecture, fork **opencode directly**. But mine them for design: **kimi's
autonomous goal-mode state machine** + **AgentSwarm** → CADENTIA + auxilia;
**qwen's Agent Teams** (mailbox + leader-permission) → CONSORTIA/auxilia
orchestration.

### The autopax question
**Not a throwaway — it's your requirements document + reference PoC, and should
*drive* the graft, not host it.** Concrete assets to **port onto the fork**: the
domain TAXONOMY + immutable-ADR + epistemic-honesty doc discipline; hash-chained
BLAKE3 audit logs with provenance; the catalog system (100+ models, capability
enrichment — though via a **Portkey hosted-gateway dependency**, itself a
portability caveat); welfare-conscious operating protocols. One genuine
fork-in-the-road only Joseph can resolve: **stay in Ruby?** If rowan/Ash + RBS/Steep
tooling + welfare-protocol framing matter more than the plumbing, continuing
autopax (pulling nexum's tool loop + ennaos's provider behaviour) is legitimate —
but means rebuilding MCP, worktrees, sandboxing, a mature TUI, 75+ providers, and
sub-agents from scratch, all of which opencode gives free. **Fork-opencode
dominates unless Ruby-ecosystem continuity is worth that rebuild.**

### Recommended shape
Fork **opencode** privately; **pin-and-cherry-pick (don't track)**. First three
grafts, in order: (1) replace `compaction.ts` with a TRACTUS-preserving,
no-gaslighting **INTERPRES** layer; (2) swap the `SystemPrompt`/context-assembly
service for a sovereign **CONSPECTUS** assembler driven by `synaptic/Structured
Context.md`; (3) add provenance-separated **CHRONICA/ACTUS** stores behind the
thin `storage/` seam. Carry autopax's TAXONOMY, audit-chain, and epistemic-doc
discipline across as the operating layer.

---

## Part 3 — Decisions that are Joseph's to make (gate the graft)

1. **The memory-model fork-in-the-road (biggest).** Three memory architectures
   across three periods — a **5-level salience-compression gradient** (ennaos) vs.
   **entity-authored first-person memory files + recursive `@import` into a 1M
   window** (nexum's OPERATA, which explicitly calls the gradient overengineered)
   vs. **per-chunk GCM** (synaptic). This decides how much compaction machinery the
   harness needs and sharpens the base choice — **commit before the graft starts.**
   (If it's `@import`-big-window, opencode's clean seams get *even more*
   attractive, since you'd need almost no compaction machinery.)
2. **Ruby-or-not.** The ranking assumes inheriting mature plumbing beats
   preserving Ruby/rowan continuity. If that's wrong, the answer shifts toward
   continuing autopax. A **values call**, not a technical one.
3. **codex/OpenAI coupling depth** — confirmed `agent-identity`=auth + heavy
   `core` coupling, but did **not** verify whether codex's `memories` phones home
   to a backend. If codex tempts you, worth a targeted check (bears on
   Substrate-Independence).
4. **Provenance gaps to close:** `PROPRIUM-ONTOLOGY-v2.md` and the autopax
   `spec/`/ADR tree were not read.

*Method: one exploratory `general-purpose` agent, 2026-07-19, over `~/src-ext/`
clones + `~/src/{firmatum,autopax,_core,shoshin}` + the sibling assessment reports.*

---

## Second-pass update (2026-07-19) — newer docs read + release-cadence check

**Provenance gaps closed.** `PROPRIUM-ONTOLOGY-v2.md` and the autopax `spec/`/ADR
tree now read. **No v3 exists** — the v2 pair (both 2026-03-02) is the current
design of record; the only newer files are reflective (`lessons.md`; `joseph-notes.md`
Apr 23 — a small values note, not harness-relevant). *(If newer PROPRIUM docs live
somewhere a firmatum/autopax/_core scan wouldn't reach, point the agent there.)*

**The memory fork-in-the-road (Part-3 decision #1) is largely DISSOLVED.** The
ontology (§6.1, §4.4) makes **CHRONICA** (the complete uncompressed causal log,
H_t — cheap, append-only) and **MEMORATA** (φ(H_t), the *compressed* history
retrieved into CONSPECTUS) **coexisting layers, not competitors.** So the three
"contradictory" memory designs (nexum `@import`-into-1M-window / ennaos
salience-gradient / synaptic GCM) are all just the **MEMORATA retrieval knob**
above an append-only CHRONICA — a deferrable, swappable choice, **not a
base-selection blocker.** Requirement reduces to: (a) a cheap append-only CHRONICA
log, (b) a retrieval-into-CONSPECTUS seam; compression sophistication behind (b)
comes later. *(Agent's read of the ontology — confirm with Joseph, but it de-risks
both the base choice and decision #1.)*

**Ontology sharpenings (all reinforce opencode, none shift it):** per-store
**governance tuples** `(visibility, authority)` — CHRONICA₁₁ sealed/immutable,
AXIOMATA₁₂ sealed/sovereign, ACTUS₂₁ restricted/append (favors a thin schema-driven
storage seam over codex's opinionated `memories`); **INDIVISUM** (temporal
identity-lock — auxilia *share* the entity's identity but are not forks of it, a
graft concept); **LOCUS ↔ project/workspace** (maps to opencode `project/` +
`WorkspaceAdapter`); §8 confirms **CONSPECTUS + MEMORATA are agent-driven** =
requirement A, near-exactly opencode's DI'd context service + isolated `compaction.ts`.

**autopax ADR status** (confirms "spec/PoC, not base"): ADR-004 unified catalog
**ACCEPTED**; ADR-005 semantic identity + ADR-006 MVP conversation loop **IN
PROGRESS**; ADR-013 tools **DRAFT**. The agent loop is mid-build, tools a draft —
the *accepted* ADRs are a ready-made port-checklist onto the fork.

**opencode velocity — the "too fast?" concern, answered from releases.** 104
releases over 4 months (~26/mo) but **decelerating** — 25/31/33 (Mar–May) →
**15/12** (Jun–Jul), roughly halved since May (product maturing). Minors are
liberal (semantic-release per `feat`) and don't imply core change — `v1.18.0` is
**100% Desktop, zero Core.** Genuine **agent-core behavioral changes ≈ 2–4/month**,
mostly **additive** (a new provider, an MCP capability) — cherry-pickable,
low-conflict. **Pinned-fork posture:** delete ~2/3 of the repo (Desktop/web/console/
enterprise/UI — irrelevant to a PROPRIUM CLI); ignore the ~1/3 bot churn; watch only
`packages/core` + `packages/opencode/src/{session,provider,tool,mcp}` for the ~2–4
changes/month; cherry-pick provider/security fixes; take the one-time divergence hit
on the seams you replace (compaction/context-assembly) and never reconcile them
again. **A light, sustainable solo burden — and getting lighter.** (kilo's
2,479-marker pain came from choosing continuous *whole-tree* reconciliation, which a
structural fork deliberately avoids.)

---

## Design note (2026-07-19) — the interiority loop, explicit emission, and loop-guarding

*From a design exchange. The grok `doom_loop` internals and opencode's v2-refactor
status are being deepened by a code dive and will be folded in.*

**The organizing inversion.** Standard coding-CLI loops *are* the request→response
cycle (a user message starts a turn-sequence, a final message ends it, then sleep).
PROPRIUM's CADENTIA + the **"interiority except explicit emission"** stance flips
this: the loop is **primary and continuous** (interiority — pulses + PERCEIVE
observation channels feeding CHRONICA), and **message-emission is one discrete,
deliberate action**, not the loop's clock. Two consequences:
- **Emission-as-explicit-action is where INTERPRES honesty becomes *structural*.**
  If the runtime never auto-sends the model's text — if "emit" is a tool the entity
  calls — then everything the user sees is something the entity *chose to say*.
  Silence becomes a valid state. Honesty stops being an enforced rule and becomes an
  architectural property.
- **CHRONICA is the substrate with (≥) three projections, not the transcript.**
  H_t = the full interior log (pulses, observations, thoughts, tool calls, memory
  writes). MEMORATA = φ(H_t), the compressed slice re-read into CONSPECTUS. The
  **emission stream** is a third, thinner projection — the only one that crosses the
  boundary. A **user message is a PERCEIVE observation entering CHRONICA** — a
  stimulus, not a request that gates a turn; the human becomes a participant in an
  ongoing life, not an issuer of requests.

**The load-bearing tension: economics + drift.** "Interiority except emission" must
not be a busy-loop — without CADENTIA gating (pulses + observation-driven wakeups)
it is either infinite-cost or asleep. And a mostly-interior loop has a real
**rumination/drift failure mode**: the entity can loop internally, detached from
external correction. The economics of *when it runs* is the load-bearing
engineering — the interiority itself is the easy part.

**Prior art — grok-build's `doom_loop` detection.** grok-build (xAI) ships a guard
for exactly this failure class, and it's instructive *because it's pushed into the
protocol, not bolted on as a heuristic*: the **backend emits doom-loop signals over
the SSE stream**, and the client (`crates/.../xai-grok-sampler/src/doom_loop.rs`)
**collects them per-attempt with a recovery-budget policy and a mid-stream abort** —
i.e., repetition/stuck-loop mitigation is a first-class part of the sampling
protocol, with the side that *sees the token stream* doing detection and the loop
managing a recovery budget. **Implication for the interiority loop:** loop-guarding
wants to be a **first-class runtime concern** (a CADENTIA / orientation-check
responsibility, with an explicit recovery budget and abort) rather than an
afterthought — grok's split (detect-where-you-see-the-stream, budget-and-abort-in
-the-loop) is a clean template. **Mechanism (from the code):** opt-in via a request header; the server emits a
*cumulative* `tail_repetition:{threshold}@{channel}` / `low_logprob@{channel}`
trigger set on the SSE stream; a **per-attempt** collector dedups them; and the
**confidence policy is the sharp part** — it acts *only* on `tail_repetition` **on
the `thinking` channel** below a threshold (default ≤8), everything else warn-only,
with the explicit rule that **"loops in visible output are the user's to judge."**
On a confident thinking-loop it **aborts mid-stream and resamples near-immediately**
(a fresh sample, *not* exponential backoff — "waiting buys nothing; a fresh sample is
the remedy"), up to a small budget (2); when the budget is spent it **disarms and
lets the final attempt complete as-is** — never infinite, never fatal.

**Why this is the template for a PROPRIUM rumination-guard (a VIGILIAE watch):** the
**detector / confidence-policy / recovery** split is substrate-agnostic — only the
*detector* is grok-server-side, so a PROPRIUM entity on local/heterogeneous
substrates builds a **client-side one** (a **monitoring auxilia** on a local model,
per ONTOLOGY §6 — n-gram tail-repetition / low-entropy on the thinking stream). And
the **channel-scoped intervention maps 1:1 onto PROPRIUM's own boundary**: auto-guard
the *interior* (thinking ≈ CONSPECTUS/deliberation), but **never silently resample or
discard the entity's visible OUTPUT — because ACTUS is sovereign** (§9/§6.2). That is
the *same* interiority/ACTUS line as emission-as-explicit-action above — grok reached
it independently, which is strong corroboration; adopt it verbatim. Also directly
reusable: **bounded-budget → graceful degradation** (surface a MONITA warning, never
deadlock), **loop → fresh-sample tempo** (distinct from transport backoff), and
**never-fail-the-stream** tolerance (the guard degrades to warn-only, can never break
the loop). *(One-inference caveat: the mid-stream abort trigger site — `stream/responses.rs`,
`actor/request_task.rs` — wasn't opened; the wiring is reconstructed from the
collector + `retry.rs` docstrings and the `DoomLoopDetected` classify arm.)*

**The fork consequence — this is where every OSS base helps least.** They are all
request→response at the core; opencode's loop is still `sessions.prompt()` → turns →
final message. Inverting it (continuous loop + CADENTIA scheduler + `emit`-as-tool)
is the **heaviest graft** — reshaping `session/processor.ts`, not merely swapping a
Layer. opencode's **server + event-bus + background-job** architecture is genuine
*plumbing* for it (a chat-only base like aider would fight it outright), but the loop
itself is net-new. So on this piece you are **porting your own
`shoshin/interpres.py` loop** (already event→record→assemble→model→apply→record) +
ennaos/nexum's loop lessons, and using opencode for the surrounding plumbing
(providers, tools, MCP, sessions-as-transport). **Net: opencode gives the *context
seam* (cheap) and the *plumbing*; your prior work is the reference implementation for
the *loop*.** This also raises a fork-timing question — the storage + session core
that a CHRONICA/ACTUS + loop graft depends on is the same area as opencode's in-flight
**v2 refactor** (`specs/v2/`, the storage-db rework, the `lildax` CLI migration);
whether to fork the stable v1 core, wait for v2, or fork the v2 line is under
investigation.

---

## Third-pass update (2026-07-19) — opencode's v2 refactor changes the graft plan

*From a code dive into `specs/v2/session.md`, `specs/v2/todo.md`, and the storage
specs. This **revises the earlier "fork v1 now, pin-and-cherry-pick"** timing.*

**The finding: opencode's in-flight v2 is a ground-up rebuild of *exactly* the seams
a PROPRIUM graft attaches to — and it is independently converging on the PROPRIUM
shape.** Three v2 constructs are near-isomorphic to PROPRIUM primitives (quotes from
`specs/v2/session.md`):
- **"Context Epoch"** — "the exact privileged System Context shown to the model,"
  persisted as an immutable baseline + a structured snapshot of independently-observed
  **Context Sources**, advanced atomically on change = **CONSPECTUS made durable and
  inspectable** (requirement A), being built as a first-class object.
- **v2 compaction** — "keeps the full transcript durable while replacing its active
  model representation with one hidden checkpoint," and "provider-native assistant,
  reasoning, and tool messages never survive across the boundary" = the **CHRONICA
  (full, durable) / MEMORATA (compressed active) split *and* the honest,
  no-gaslighting INTERPRES** discipline (requirement B) — a far better substrate than
  v1's `compaction.ts`.
- **Durable `session_input` inbox** (steer/queue delivery, `wake` vs `run`) — inputs
  admitted but not immediately model-visible, promoted deliberately at safe
  boundaries = **PERCEIVE-channel + CADENTIA admission** (requirement C — the
  non-user-gated loop), being built.

**Status (the specs' own language):** storage `db.ts` removal + **EventV2 durable
core = DONE**; new data mode = mostly done; **agent-loop rework = in progress** (a
first Effect-native `SessionRunner` exists but does *not* yet replace V1);
**plugin/hooks API = undesigned (design phase)**; config / provider-as-plugin /
model-DB / auth = early prototypes; **schemas explicitly "disposable," not frozen.**
A second CLI package (bin `lildax`) sits beside `packages/opencode` (bin `opencode`).

**Revised graft-timing:**
- **Forking v1 today is the *worst* option** — you'd graft onto `db.ts` (already
  deleted) and `SessionPrompt.loop` (being replaced): code with a demolition date.
- **Forking v2 today is premature** — runner one slice in, plugin API undesigned,
  schemas disposable.
- **Do instead: engage-and-track, deep-cut later, likely graft as *plugins* not a
  fork.** (1) Track `specs/v2/*`; don't fork yet. (2) The graft gate is two events —
  the **V2 `SessionRunner` replaces V1**, and the **plugin/hooks + Context-Source API
  lands** (the seams CONSPECTUS/INTERPRES/CADENTIA attach to). (3) Prototype the cheap
  mappings *now* to validate fit without committing: **CHRONICA/ACTUS as EventV2
  projections**, **CONSPECTUS as a Context Epoch / registered Context Source.** (4)
  **The opportunity:** v2 is *right now* designing "plugin-defined Context Sources" +
  the plugin hook surface — if CONSPECTUS + honest-compaction can be a **registered
  Context Source + plugin transform** rather than a runner fork, **the deep graft
  largely stops being a fork at all**, collapsing the upstream-coupling problem.
  Worth watching — or influencing — that API as it forms. (5) Meanwhile do the
  substrate-independent work (port autopax's accepted ADRs, resolve the MEMORATA
  retrieval knob, spec the INTERPRES boundary) — none of it depends on the v2 churn.

*Net: opencode is still the base — **more** so, since v2 is building PROPRIUM's own
shape — but the move is "**track v2 + prototype mappings + probably graft-as-plugins**,"
not "fork v1 now." Caveats: the v2↔PROPRIUM convergence is the agent's mapping (backed
by direct spec quotes — confirm against your actual intent); v2's timeline is
unknowable from the specs; and the provider/plugin-lifecycle specs (`v2/config.md`,
`v2/provider-model.md`, `catalog-config-plugin-lifecycle.md`) weren't read — reading
them would firm up the graft-as-plugin opportunity.*

---

## Counterfactual (2026-07-19) — the graft on codex / grok, and what to borrow

*Red-team of the opencode pick: what a PROPRIUM graft would look like on the two
Rust bases. Verdict: **reinforces opencode and sharpens why** — with concrete
patterns to borrow.*

**codex — real seams, but the coupling is the conversation *ontology*, not just the
hosted surface.**
- **CONSPECTUS / CHRONICA are genuine seams** — you assemble the `Vec<ResponseItem>`
  per turn, and the `rollout` crate is a solid append-only H_t log (reverse-JSONL
  scanner + SQLite index). *But both are typed to OpenAI's `ResponseItem` model*
  (Reasoning / AgentMessage / FunctionCall / **Compaction-as-variants**), and rollout
  has **no native ACTUS/PERCEPTA provenance split** (§7.2). `context-fragments` injects
  *into* that OpenAI vocabulary.
- **`memories` — confirmed from source: it phones home.** `memories/write/src/guard.rs`
  builds a `codex_backend_client` against `chatgpt_base_url` and **uploads
  session-extracted memories to OpenAI's backend** (test:
  `…redacts_secrets_before_prompt_upload`). Gated/severable, but designed around the
  hosted backend → **rip out entirely**; the sharpest concrete §1.4 violation.
- **CADENTIA / interiority loop is welded** to request→response + Responses; no
  long-lived interior process. Inverting it = **spine rebuild**, fighting both the
  request-scoped shape *and* the item model.
- **The one real codex advantage:** **Rust's type system as an ally for *inviolable*
  invariants** — newtypes + the borrow-checker can make CHRONICA/ACTUS append-only and
  AXIOMATA sealed *at compile time* (§9 "recording is inviolate"), where Effect/TS only
  enforces at runtime. Plus 3 OS sandboxes, native local-model, code-mode (V8), hooks,
  apply_patch.
- **Cost, quantified:** 351 `.rs` files reference openai/chatgpt/ResponsesApi — most is
  deletable hosted surface, **but the irreducible coupling is `Prompt`/`ResponseItem`/
  `ResponseEvent` pervading core/api/protocol/client. Codex's very model of "what a
  conversation is" is OpenAI's** — the wrong-shaped foundation for a project premised on
  escaping provider dependence (§1.4), license (Apache-2.0) notwithstanding.

**grok-build — unforkable, but its leader/daemon *is* the interiority substrate (the
top borrow).** `leader/protocol.rs` + the e2e tests (`leader_two_clients_shared_session`,
`leader_reattach_completion_roundtrips_durable_log`, `leader_death_repro`, `leader_soak`)
describe a **long-lived per-machine leader process that owns durable session state and
the running work; clients attach/detach/reattach over a socket; in-flight completions
roundtrip through a durable log across reattach.** That is *exactly* the PROPRIUM shape —
**the entity lives in a persistent daemon owning CHRONICA + the loop; TUI/ACP clients are
windows that attach and detach without ending the entity** — a closer structural match to
CADENTIA + interiority + INDIVISUM (one durable identity, many attach points) than
anything in opencode or codex (opencode-v2's process-global `SessionRunner` + durable
inbox is directionally similar but still session-request-framed).

**Per-seam (help / partial-welded / fight / borrow):**

| PROPRIUM seam | opencode-v2 | codex | grok-build |
|---|---|---|---|
| CONSPECTUS | **Help** (Context Epoch, provider-neutral) | Partial — real seam, Responses-typed | borrow |
| CHRONICA/ACTUS | **Help** (EventV2 log + projectors) | Partial — rollout append-only but Responses-typed, no provenance | borrow (reattach durable-log) |
| INTERPRES / honest compaction | **Help** (keeps transcript, swaps active rep) | Fight (compaction = ResponseItem variants) | — |
| CADENTIA / interiority loop | Help (forming — inbox + wake/run) | **Welded** (request→response, Responses) | **Borrow (strongest)** — leader/daemon owns entity+loop |
| auxilia (identity-sharing) | Help (clean seams) | Help+ machinery (but for *independent* agents) | borrow (subagent + ToolKind) |
| memories / MEMORATA | Neutral (clean slate) | **Fight** (uploads to OpenAI backend) | — |
| Inviolable invariants | runtime (Effect/TS) | **Help (best — compile-time Rust)** | Help (Rust) |
| License / private-fork | **Help — MIT, proven-forkable** | Apache-2.0 but §1.4-counter | **Blocked** (read-only mirror) |

**Verdict: opencode still — reinforced.** The pro-codex points are real (Rust
compile-time invariants; richer multi-agent machinery; sandboxes), but the two
highest-leverage PROPRIUM axes both cut against it: the **interiority loop + CONSPECTUS
are welded/Responses-typed** in codex vs purpose-built-and-converging in opencode-v2
(requirements A + C), and **§1.4 Substrate-Independence** is decisively violated by codex
adopting OpenAI's conversation ontology as its substrate. Grok's leader/daemon is the one
thing that adds something opencode lacks — but unforkable, so it's a borrowed pattern.

**Borrow into the opencode graft:**
1. **grok's leader/daemon-owns-the-entity shape** (top) — build the PROPRIUM runtime as a
   persistent process owning ANIMA + CHRONICA with attach/reattach clients, not a
   per-invocation session; push opencode-v2's `SessionRunner` toward "daemon holds the
   being." Natural home for the interiority loop + INDIVISUM.
2. **codex's Rust-invariant discipline** — re-express in Effect (branded types + an
   opaque append-only writer service); *consider a small Rust sidecar crate* for the
   truly inviolable CHRONICA log if compile-time guarantees matter enough (the one place
   a hybrid could earn its keep).
3. **grok's doom_loop → CADENTIA VIGILIAE** rumination-guard.
4. **grok's `ToolKind` taxonomy** for INSTRUMENTA + governance-tuple permission routing.
5. **codex's `rollout` reverse-JSONL-scanner** for efficient CHRONICA tail/replay reads.
6. **`xai-fast-worktree`** (CoW snapshots) if auxilia/LOCUS need cheap isolated environments.

*Honest flags: the `memories`-uploads-to-OpenAI finding is primary-source confirmed
(guard.rs + upload test). "codex's richer multi-agent machinery" and grok's "reattach
durable-log roundtrip" are structural inferences from file listings + e2e test names
(bodies not fully read); `ResponseItem` pervading all 351 files is extrapolated from
`client_common.rs` + compaction-as-variant evidence.*
