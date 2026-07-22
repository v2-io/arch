# AI Coding CLI Tools — Source-Level Assessment (July 2026)

*What the **code** actually is, from reading the cloned repositories — not the
marketing. 10 open-source tools were shallow-cloned into `~/src-ext/` and each
assessed by a dedicated explorer agent against the working tree, git history
(`git fetch --deepen`), and GitHub metadata (`gh`) on **2026-07-18/19**. Stars,
contributor counts, and cadence are as-of that pass. Maintainability grades are
the explorers' honest reads (a "medium-high with reasons" is the norm), not a
metric.*

*This complements the capability reference (features) and the sentiment report
(opinion). Where it **corrects** the feature-level matrix, it's flagged.*

---

## Master comparison

| Tool | Lang (core) | ★ | Contrib | Velocity | Maint. | Agent brain | Repo indexing (ground-truth) | Scale | Tests |
|---|---|---|---|---|---|---|---|---|---|
| **opencode** | TS (Effect) | 187k | ~455 | very high | med-high | **local** | **B** — live ripgrep/glob/LSP + git snapshots; *explicitly no index* | 34 pkgs | 623 files |
| **codex** | Rust | 99k | ~474 | very high | **high** | **local** | **B** — file-search (ignore+nucleo) + bm25; no embedding index | 97 crates, ~1.18M LOC | ~11.5k fns |
| **warp** | Rust | 63k | 100+ | very high | med-high | **hosted (Oz, closed)** | **A** — Merkle-tree incremental **embedding RAG** + tree-sitter chunking | ~1.55M LOC | 793 files |
| **aider** | Python | 47.5k | ~170 | **slowing** ⚠️ | med-high | local (*not* tool-calling) | **A** — tree-sitter + networkx **PageRank** repo-map | ~38k LOC | 489 tests |
| **kilocode** | TS (Effect) | 26k | ~407 | very high | med-high | local (**fork of opencode**) | **A** — **LanceDB** vector store + tree-sitter (an index opencode lacks) | ~820k TS + 106k Kotlin | 1,162 files |
| **qwen-code** | TS | 26k | 100s | very high | med-high | local (**fork of Gemini CLI**) | file index for `@`-completion + auto-memory (not semantic repo-map) | ~522k LOC | 1,765 files |
| **grok-build** | Rust | 19k | 1 (sync bot) | opaque (mirror) | med-high | **local** | **A** — real tree-sitter **codebase graph** (scope graph, go-to-def) | ~1.36M LOC, 122 crates | ~25k occ. |
| **mistral-vibe** | Python | 4.7k | 10 | active | med-high | **local** | tree-sitter file index for `@`-completion (*not* semantic repo-map) | ~170k LOC | 442 files |
| **kimi-code** | TS | 3.7k | 34 | active | med-high | **local** | live git-context probes (no persistent index) | ~690k LOC | 1,065 files |
| **minimax-cli** | TS | 2k | 22 | active | med-high | **none — not an agent** | none | ~8k LOC | ~55 files |

★ = GitHub stars. "Contrib" = GitHub contributor count (many inflated by agent bot-fleets, see below).

---

## Cross-cutting findings (the parts worth the ink)

### 1. The ecosystem quietly orbits **opencode** and **Gemini CLI**
Fork/lineage is the biggest surprise reading the source:
- **kilocode's CLI *is* opencode** — a fork pinned at upstream `v1.17.4` (there's a `.opencode-version` file and a whole `packages/opencode` tree), with **2,479 `// kilocode_change` markers across 228 files** annotating every divergence for merge maintenance. Kilo didn't build a terminal agent; it adopted opencode and bolted its own layers on.
- **qwen-code is a fork of Google's Gemini CLI** (v0.8.2) — `@google/genai` types still thread through its turn loop, and Apache headers still read "Copyright 2025 Google LLC."
- **grok-build carries first-party Rust ports of *both* codex's and opencode's tool suites** in-tree, alongside its own `grok_build` family — it A/B-tests rival tool ergonomics.
- **warp delegates to** claude-code / codex / gemini / opencode as swappable "harnesses."

So **opencode is upstream to Kilo, ported by grok, and a harness for warp** — arguably the most structurally central OSS agent in the category, despite Claude Code getting the mindshare.

### 2. Everyone is converging on the **Claude-Code / Anthropic shape**
Independently, the same primitives recur across all the real agents: **MCP** (client, usually + OAuth), **SKILL.md skills** (explicitly Anthropic-skills-shaped in vibe, kimi, qwen), **ACP** (Agent Client Protocol for editor embedding — opencode, grok, qwen, kimi, vibe), **plan/agent modes**, **subagents via a `task` tool**, **hooks** (pre/post-tool), and **git worktrees**. qwen-code's README is an explicit Claude-Code parity table; vibe's skills+hooks are "directly Claude-Code-shaped." The category has a de-facto standard, and it's Anthropic's.

### 3. Repo indexing — ground-truthed, and it **refines the capability matrix**
Reading the source cleanly separates who *actually* builds a persistent index:
- **Genuine (A) persistent index:** **aider** (tree-sitter + PageRank repo-map), **kilocode** (LanceDB vectors + tree-sitter), **grok-build** (tree-sitter scope graph), **warp** (Merkle-tree embedding RAG).
- **Live / no index (B):** **opencode** (ripgrep/glob/LSP + git snapshots — *the code confirms it deliberately has no index*), **codex** (file-search + bm25 keyword, no embeddings), **kimi** (live git probes).
- **Completion index, not semantic (≈C):** **qwen**, **vibe** (tree-sitter file index for `@`-path autocomplete only).

**The killer detail:** kilocode is a fork of opencode, and its single biggest *architectural* divergence is **adding the semantic index (kilo-indexing) that opencode refuses to build.** The fork diverges precisely on this axis. (This confirms and sharpens §4 of the capability doc.)

**Disaggregated — "repo indexing" is really four orthogonal mechanisms**, and a tool can hold any subset. Verified by reading the clones (✓ = confirmed in source; ~ = present but not the primary path; ❌ = absent):

| Tool | Live search (grep/glob) | LSP (exact symbols) | tree-sitter (AST) | Semantic *embedding* index | Net |
|---|---|---|---|---|---|
| **opencode** | ✓ ripgrep | ✓ first-class | ~ (TUI parser only) | ❌ | live + LSP |
| **codex** | ✓ file-search + **bm25** | ❌ | ~ (present, minor) | ❌ | live + keyword (bm25) |
| **aider** | via repo-map (no grep tool) | ❌ | ✓ (repo-map) | ❌ | tree-sitter + **PageRank** |
| **kilocode** | ✓ | ✓ | ✓ | ✓ **LanceDB** | full stack |
| **qwen-code** | ✓ ripgrep | ✓ (experimental) | ✓ (file index) | ❌ | live + LSP + file-index |
| **kimi-code** | ✓ | ❌ | ❌ | ❌ | live only |
| **mistral-vibe** | ✓ | ❌ | ✓ (@-index + bash parse) | ❌ | live + tree-sitter (completion) |
| **grok-build** | ✓ | ✓-like (own scope graph) | ✓ (codebase graph) | ❌ | tree-sitter **scope graph** |
| **warp** | ✓ | ✓ | ✓ (arborium fork) | ✓ **Merkle RAG** | tree-sitter + embedding RAG (hosted) |
| **minimax-cli** | — (not an agent) | — | — | — | n/a |

> **Why this needed source-reading, not a dependency grep:** a naive grep over the clones *mis*-credits three tools with a "semantic index" and misses one. **opencode**'s manifest hit is the word *"embedding"* in a TUI build-script line — it has **zero** vector dependency. **codex**'s is **bm25** (lexical keyword retrieval, not vectors). **grok**'s is its tree-sitter **scope graph** (structural go-to-def/references, not embeddings). And **warp**'s real Merkle-tree embedding RAG is invisible to the grep because it rides a renamed tree-sitter fork (`arborium`) and a custom index crate. Net: only **kilocode (LanceDB)** and **warp (Merkle RAG)** build a true *embedding/vector* index; **aider** is tree-sitter + PageRank (structural); **grok/qwen/vibe** use tree-sitter for structural parsing without embeddings; **opencode/codex/kimi** deliberately run index-free (LSP and/or keyword + live search).

### 4. Local agent brain vs **hosted/closed** brain
Eight are **local**: you can read the whole agent loop (opencode `session/prompt.ts`, codex `core/client.rs`, aider `base_coder.py`, grok `xai-grok-sampler`, qwen `turn.ts`, kimi `agent-core/loop`, vibe `_loop.py`). **warp is the exception** — the flagship "Oz" agent is a thin protobuf-over-SSE **client to a proprietary hosted backend**; the tool enum and RAG index are open, but the *agent loop, prompts, and model routing are server-side and absent from the repo*. And **minimax-cli isn't an agent at all** (see §6).

### 5. Velocity & health — and the pioneer is the one slowing
- **Blazing (300–600 commits/wk):** opencode, codex, kilocode, qwen, warp.
- **Active (~100/wk):** kimi, vibe, minimax.
- **Slowing ⚠️:** **aider** — the tool everyone benchmarks against and forks ideas from — last push ~May 2026, near-zero recent cadence, ~2-month gap. Worth watching if you'd depend on it.
- **Opaque:** grok-build is a one-way squashed mirror of xAI's internal monorepo (1 GitHub "contributor" = the sync bot; external PRs refused) — readable and buildable, but you can't track real velocity or contribute.

### 6. These tools are increasingly **built by agents**
The commit graphs give it away: **qwen** runs a "Fleet Shepherd" bot fleet filing/reviewing/merging its own PRs at ~40 commits/day; **warp**'s own Oz agents triage issues and write the 269 `specs/` folders (PRODUCT.md + TECH.md per issue) before implementation; **minimax** merges `codex/*` and `agent/*` branches; **kimi** scrubs agent co-author attribution by policy. High contributor counts are partly bot-inflated. The category is bootstrapping on itself.

### 7. Convergent innovation: **"model writes code to orchestrate tools"**
Two independent implementations of the same idea — instead of one JSON tool-call per step, let the model author a small program that calls host tools in a sandbox: **opencode's CodeMode** (acorn-parsed JS sandbox, schema-described tools only) and **codex's "code mode"** (embedded **V8**). Worth watching as a possible next paradigm past one-tool-call-per-turn.

### 8. Other standouts worth a look in source
- **codex** — 3 bespoke OS sandboxes incl. an unusually deep **Windows** one (WFP network filtering, ACL, DPAPI, ConPTY); hard bet on the Responses API (Chat Completions wire **removed**).
- **grok-build** — server-side **`doom_loop`** detector (loop mitigation pushed into the sampling protocol); **`xai-fast-worktree`** (CoW + BTRFS O(1) snapshots — worktree creation as a *performance* feature); explicit **Claude Code import** path.
- **qwen-code** — **Agent Arena** (run multiple models head-to-head on one task) and **Agent Teams** (mailbox + leader-permission bridge) — orchestration beyond plain subagents.
- **kimi-code** — autonomous **goal-mode state machine** (active/paused/blocked/complete, self-continuing) and **AgentSwarm** (128 parallel subagents from a template); ships its own embedded DB + TUI; single-binary via Node SEA.
- **kimi / qwen / vibe** — all three are **genuinely multi-provider despite vendor branding** (in-house or adapter layers that drive Claude/GPT/Gemini, not just their own model).

---

## Per-tool capsules

- **opencode** — Effect-native TS platform (not just a CLI): client/server, MCP+OAuth, worktrees, background subagents, CodeMode sandbox. No repo index by design. *Watch:* steep Effect ramp; sprawling product surface (desktop/web/enterprise); a 1.6k-LOC agent-loop hot file.
- **codex** — the most maintainable of the lot ("high"): 97 clean Rust crates, deepest sandboxing, MCP both ways, multi-agent built into the tool layer. *Watch:* a "god core-crate"; heavy Cargo+Bazel+Nix build; tight OpenAI/Responses coupling.
- **warp** — a huge Rust terminal/GUI whose agent is a **hosted** black box; also a **meta-orchestrator** of rival CLIs; genuine embedding RAG index. *Watch:* you can't run/audit the agent brain from source; hard dependency on Warp's backend.
- **aider** — the elegant outlier: **not** tool-calling, **no** MCP/subagents/worktrees — a send→parse-diffs-from-markdown→apply→git-commit→lint/test loop, with the standout **PageRank repo-map** and the two-model architect pattern. *Watch:* slowing cadence; 2.4k-LOC `base_coder` god-class; format-brittleness of text-parsed edits.
- **kilocode** — opencode fork + real bolt-ons (LanceDB index, 500+-model gateway, project memory) across VS Code + JetBrains + CLI. *Watch:* 2,479-marker fork-maintenance burden coupled to an upstream they don't control; bleeding-edge deps.
- **qwen-code** — Gemini-CLI fork turned multi-protocol framework; Arena + Teams + Auto-Memory + IM bots. *Watch:* enormous surface, many `--experimental-*`, inherited Gemini substrate not fully abstracted.
- **grok-build** — polished 122-crate Rust with leader/daemon architecture, real codebase graph, doom-loop detection, fast-worktree. *Watch:* read-only monorepo mirror (no real history/contribution); heavy hermetic build.
- **mistral-vibe** — Textual-TUI Python agent, multi-provider, subagents, worktrees, **real voice mode**, ACP. More test code than source. *Watch:* 2.5k-LOC AgentLoop god-object; feature sprawl; squash-only public history.
- **kimi-code** — professionally over-engineered TS: two agent engines (v1 + VS Code-DI v2), all-in-house infra (LLM abstraction, DB, TUI), goal-mode + 128-swarm, single-binary. *Watch:* dual-engine mid-migration duplication; heavy reinvention.
- **minimax-cli** — **category mismatch:** an API client for media generation (text/image/video/speech/music/search) that's *built to be a tool an agent calls* (installable skill; exports tool schemas). No agent loop. Clean and well-tested, but not comparable head-to-head with the agents above.

---

## How this feeds back into the other reports

- **Capability §4 (repo indexing):** confirmed and sharpened. Among OSS agents, genuine persistent index = **aider, kilocode, grok-build, warp**; **opencode/codex are deliberately index-free (B)**; qwen/vibe are completion-index only. The kilo-vs-opencode fork diverges *exactly* on this axis.
- **Capability roster:** **minimax** should be marked as **not an agentic coding CLI** (it's a media-generation API client / agent tool) — a category correction beyond the "no official deepseek CLI" one.
- **Capability worktree/agentic cells:** now source-verified for the OSS set (opencode/grok/kimi/vibe/qwen all have real `task`-tool subagents + worktrees; codex has spawn/wait multi-agent; aider deliberately has neither).
- **Sentiment report:** the "harness matters independent of model" thesis is visible in source — these are 8k-to-1.5M-LOC engines with very different loops, sandboxes, and context strategies wrapping the same handful of models.

---

## Caveats

- Grades and cadence are the explorer agents' reads from a single 2026-07-18/19 pass over **shallow** clones (deepened per-repo for cadence); star/contributor numbers are point-in-time.
- **warp** and **grok-build** are only *partially* open (hosted brain / read-only mirror respectively) — source conclusions about their agent loop are bounded by what's actually published.
- Closed tools (cursor, kiro, devin, agy, zcode) aren't here — no readable source. Claude Code and gh-copilot are shims over proprietary binaries (not cloned).
- Everything here is *readable-source* evidence; runtime behavior (actual prompt quality, model routing) may differ, especially where prompts live server-side.

*Method: 10 parallel `general-purpose` explorer agents over `~/src-ext/` clones + `git`/`gh`, 2026-07-18/19. Repos: opencode, kilocode, aider, codex, grok-build, qwen-code, kimi-code, minimax-cli, mistral-vibe, warp.*
