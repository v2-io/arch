# External input — a grok agent's independent analysis (narrower-but-deeper Rust harness)

*Provided by Joseph 2026-07-19: an independent analysis from a grok agent that
reviewed the same corpus (OpenCode, Codex/codex-rs, Grok tooling, Claude Code,
Gemini CLI + PROPRIUM/Autopax/Nexum). Saved verbatim-in-substance for our agent's
convergence/divergence cross-check against its own independent pass. Do NOT treat
as ground truth — it's a plausible-reasoning artifact; verify its codebase-specific
mappings (esp. Claude/Gemini) against source where load-bearing.*

## Verdict
Yes — enough intuition for architecture + a first capable slice; NOT enough to skip
hard experiments in the distinct PROPRIUM layers. Reframe: you're not building "best
coding agent product" — you're building **INTERPRES + PRINCIPIA + ANIMA for
entities, with coding as one LOCUS among many.** Independent Rust harness is a
*better fit than a private OpenCode layer* if the center of gravity is
PRINCIPIA/identity/continuity. OpenCode-as-host stays valid *later*; it should not
be the home of identity.

## Subtleties commodity agents already paid for (steal, don't rediscover)
- **Turn atomicity** — one provider request = one causal unit; tool settlement before next turn (OpenCode runner; Claude query engine; Gemini turn+scheduler)
- **Admission vs promotion** — input durable before model-visible (OpenCode V2 inbox)
- **Safe context-mutation points** — mutate privileged context only at boundaries (OpenCode Context Epochs)
- **Projection vs wire** — session history ≠ raw payload; TRACTUS ≠ CHRONICA (OpenCode projections; Autopax split)
- **Tool identity over call IDs** — provider call IDs collide; ownership must be durable
- **Output bounding** — model-visible truncation ≠ success semantics
- **Permission as UX not sandbox** — real isolation is OS/container; approvals separate (Codex sandbox crates)
- **Compaction philosophy** — lossy summary vs rehydratable memory (products pick lossy; ASM is the alternative)
- **Provider-combinatorics tax** + **dual UI/runtime explosion** — the bulk of every monorepo

## Subtleties that are YOURS and still under-proven in code
Sovereign CONSPECTUS (entity decides focus, ANIMA executes) · gradient memory / ASM /
rehydration · CHRONICA as crypto-inviolate H_t (Autopax hash-chain + Nexum DID partial)
· interiority-default / emission-as-ACTUS · CADENTIA (PULSUS/VIGILIAE) · multi-timescale
nesting (TFT) · INDIVISUM/SIGNUM identity · CURATORIA phenomenology ("memory grades") ·
AUXILIA as non-sovereign self-extensions. **These resolve only in operation, not more reading.**

## Why Rust fits THIS program
CHRONICA/SIGNUM/INDIVISUM want types + crypto + append-only integrity (codex-rs proves
a full agent lives here); INTERPRES benefits from async/streaming/process-isolation;
narrow surface stays honest in Rust vs a 600k-LOC TS monorepo; local logostrata as
first-class crates, not TS FFI.

## Narrow (don't build) vs Expansive (must own)
- **Narrow:** multi-surface product UI · provider-marketplace polish · third-party plugin economy · "works for every team" packaging · contribution automation.
- **Expansive:** PRINCIPIA store (AXIOMATA, CHRONICA, MEMORATA, OPERATA, CONSORTIA, VERA, PRAXES, SIGNUM) · CONSPECTUS + ASM · INTERPRES epistemic integrity · CADENTIA/interior loop · LOCUS first-class · provenance/continuity proofs.
- *"Commodity agents are wide shallow shells. You want a deep thin spine."*

## "If grok can subsume, so can we"
Treat other systems as (1) LOGOSTRATA (model APIs), (2) INSTRUMENTA backends (optionally
shell out to opencode/codex/claude for coding), (3) pattern libraries (steal turn/tool/
permission shapes, not codebases). Need OpenCode-class turn discipline in a PROPRIUM
identity machine, not to out-feature OpenCode.

## Proposed Rust workspace (crates/)
- **Wire & contracts:** `proprium-schema` (serde types/IDs/events/brands), `proprium-protocol` (JSON-RPC/HTTP/SSE, optional early)
- **PRINCIPIA (durable entity state):** `principia` (home layout), `signum` (identity card+verify), `chronica` (append-only hash-chain +PQ-sign later), `memorata` (gradient chunks/compression/rehydrate), `operata` (hierarchical intents), `consortia` (models of others), `vera` (qualified claims — start markdown), `praxes` (techniques/SOPs), `axiomata` (load/validate minimum viable self)
- **ANIMA (runtime):** `anima` (orchestrator PERCEIVE→CONTEXTUALIZE→CHOOSE→EFFECT), `conspectus` (assemble model-visible context), `asm` (active salience policy: rank/compress/order), `cadentia` (PULSUS timers + VIGILIAE watches), `indivisum` (single-instance/no-fork lock), `commentaria` (working-notes store)
- **INTERPRES (logostratum mediation):** `interpres` (turn boundary/retries/coherent stream), `tractus` (raw transcript, git/content-addressed), `logostratum` (provider trait+catalog) + `-openai`/`-anthropic`/`-xai`/`-local`
- **INSTRUMENTA/LOCUS:** `instrumenta` (tool trait/registry/settlement/bounding) + `-fs`/`-shell`/`-web`/`-mcp`, `locus` (workspace abstraction), `actus` (external-action log)
- **Policy/safety:** `permission`, `execpolicy`, `sandbox` (optional later)
- **AUXILIA:** `auxilia` (non-sovereign child sharing an AXIOMATA slice)
- **Curation/migration:** `curatoria` (dialog/full extractors — port from Autopax), `archeologia` (import claude/codex/opencode jsonl)
- **Surfaces (thin):** `cli`, `tui` (ratatui, presenter-only), `daemon` (optional long-lived CADENTIA host)
- **Support:** `crypto` (ML-DSA/age/DID from Nexum ADR), `util`, `test-support`

**Hard layer rule:** `schema → principia/* → anima/conspectus/asm ↘ interpres/logostratum ↘ instrumenta/locus`; `cli/tui/daemon → anima only`. **No crate above INTERPRES may fabricate TRACTUS; no crate may rewrite CHRONICA** — encodes ontology authority in the module graph.

## MVP vertical slice (a spine, not full PROPRIUM)
1. `axiomata` load entity → 2. `chronica` append+verify → 3. `interpres` + one OpenAI-compatible logostratum → 4. `instrumenta-fs` + `-shell` + `permission` → 5. `conspectus` = AXIOMATA + recent CHRONICA window + MEMORATA stubs → 6. `cli` interactive turn loop → 7. `curatoria` import one prior Claude/Autopax session into MEMORATA. *"Already beats Autopax's end-state in structure, narrower than any commodity agent."*

## Map: steal / adapt / invent
- **logostratum+interpres** ← OpenCode llm+runner / Codex codex-api / Claude API / Gemini contentGenerator (adapt: one trait, one stream-event enum, one turn)
- **tractus** ← OpenCode http-recorder / Autopax TRACTUS / Claude VCR (adapt)
- **instrumenta+tools** ← all (subsume; implement 6–8 tools; MCP later)
- **permission/execpolicy** ← Codex execpolicy/linux-sandbox / OpenCode permission / Gemini policy-engine (adapt; don't invent policy languages first)
- **protocol/schema** ← OpenCode schema/protocol / Codex protocol (adapt, smaller surface)
- **cli/tui** ← Codex cli / OpenCode tui / Gemini packages/cli (thin; ratatui)
- **locus** ← OpenCode Location/Project / Codex home/workspace (adapt)
- **chronica** ← Autopax Chronica::Log / Codex rollout (softer) — **invent integrity semantics, borrow append-only storage**
- **memorata+asm** ← Sapientia vision / Claude SessionMemory (shallow) / Codex memories — **invent (commodity "memory" ≠ ASM)**
- **conspectus** ← OpenCode System-Context / Gemini prompt-assembly / Claude context — **shape-adapt, agency-invert (sovereignty is yours)**
- **cadentia** ← Claude ScheduleCron/DreamTask hints (nothing real elsewhere) — **invent**
- **signum/indivisum/crypto** ← Nexum DID / Codex agent-identity (lighter) — **invent from your ADRs**
- **curatoria** ← Autopax curatoria / eli-migration-prep — **port Ruby→Rust (highest ROI from your own code)**
- **auxilia** ← Claude AgentTool / OpenCode subagents / Codex ext/agent — **adapt isolation, different identity model**
- **operata** ← todos everywhere (thin) — **invent hierarchy later; todos stub first**

**codex-rs = closest STRUCTURAL cousin** (many small crates, protocol boundary, core loop, exec+policy, memories-as-crates, agent-identity, rollout) — engineering taste to emulate.
**OpenCode-v2 = closest SESSION-ALGEBRA cousin** (durable admission, context epochs, one-stream-per-turn, projections) — port the algebra, not the dual-stack.
**Claude Code = product-completeness cousin** — a *negative* template (how fat the surface gets; refuse to grow it until PRINCIPIA works).
**Gemini CLI = cleanest core/cli split** — reminder the first cut can be that simple.

## Runtime sketch
`CADENTIA (pulses/watches) → PERCEPTA queue ← human/tools/auxilia → ANIMA.triage(Note|Integrate|Attend) → {CHRONICA always · CONSPECTUS (ASM assemble) · CHOOSE (act|wait|interior)} → INTERPRES {TRACTUS · stream · retries} → LOGOSTRATUM → INSTRUMENTA → LOCUS/ACTUS → results back to PERCEPTA.`
*"Commodity agents collapse this to prompt→model→tools→compact. You keep that as the **Attend path only**, inside a larger machine."*

## What NOT to do
1. Don't fork OpenCode/Codex (rebase death; wrong ontology). 2. Don't chase multi-provider perfection (one OpenAI-compat + one Anthropic is enough). 3. Don't build desktop/web (CLI/TUI until CHRONICA+ASM are real). 4. Don't do full PQ-crypto day-1 (hash-chain first; ML-DSA when CHRONICA is load-bearing). 5. Don't let TUI own state (UI-gravity is a trap; TUI is a lens). 6. **Don't wait for OpenCode V2 launch — their cutover problem is not your problem; their algebra already taught you enough.**

## North star
> A single-process entity runtime that can awaken from AXIOMATA, think with a logostratum
> under INTERPRES, act through INSTRUMENTA on a LOCUS, remember through MEMORATA/ASM, and
> prove continuity through CHRONICA — without a product monorepo or a frontier-only substrate.
