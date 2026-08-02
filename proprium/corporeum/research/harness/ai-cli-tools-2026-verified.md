# AI Coding CLI Tools — Capability Reference (July 2026)

*A capability-focused comparison of terminal-native AI coding agents, **labeled
per claim** for how it was verified (not blanket-"verified"). Emphasis is on
**what each tool structurally is and does** — ownership, architecture, agentic
model, and especially how each actually "understands" a repository.*

*Prices, model-version strings, star counts, and benchmarks are **carried as
reported, with as-of dates and provenance** — volatile and (benchmarks
especially) self-reported, kept and dated rather than dropped, since magnitude
and ordering still carry information; how much to trust them is the reader's
call.*

*Every non-local claim carries a footnote marker linking to §13, which is a
**re-verification ledger**: verdict · verbatim source quote · URL · source date ·
the date I last checked it. A future reader (or agent) can walk each footnote,
re-fetch the source, and confirm-or-mark-stale. The web pass behind it ran
**2026-07-18**.*

## How to read the confidence markers

| Marker | Meaning |
|---|---|
| ✓ **Verified** | Primary source — the tool's own `--help` run locally, or a source quote confirmed this pass |
| 🌐 **Corroborated** | Multiple independent web sources agree |
| ⚠️ **Reported** | Single/secondary source, or a claim the cited page did **not** actually support; plausible, not confirmed |
| ❓ **Unverified** | No supporting source found; likely marketing or error |

Tools marked **[installed]** were probed directly on this machine (`--help`),
2026-07-18. The rest are official docs + web verification.

---

## 1. Tool roster

| Command | App / Project | Source (repo / download)[^repos] | Maker | Model families (approx., fast-moving) | Notes |
|---|---|---|---|---|---|
| `claude` **[installed]** | claude-code | [anthropics/claude-code](https://github.com/anthropics/claude-code) · *shim* | Anthropic | Haiku 4.5, Sonnet 5, Opus 4.8, Fable 5 | Fable 5 is a real model (✓, launched Jun 9 2026)[^fable5] |
| `agy` **[installed]** | antigravity | [antigravity.google](https://antigravity.google) · *closed* | Google | Gemini 3.x Flash/Pro | Replaced the retired `gemini` CLI (✓ Jun 18 2026 for consumer tiers; Enterprise retains access)[^gemini]; `gemini` binary now a deprecated shim → `agy` |
| `codex` **[installed]** | codex | [openai/codex](https://github.com/openai/codex) · Apache-2.0 · **cloned** | OpenAI | GPT-5.x, o-series | Rust CLI; native sandbox; **native local-model path** (`--oss`, lmstudio/ollama) ✓[^local] |
| `grok` / `agent` **[installed]** | grok-build | [xai-org/grok-build](https://github.com/xai-org/grok-build) · Apache-2.0 (read-only mirror) · **cloned** | xAI | grok-build, Composer 2.x | Ships an `agent` alias too — on a box with Cursor, PATH order decides which `agent` wins ✓[^local] |
| `opencode` **[installed]** | opencode | [anomalyco/opencode](https://github.com/anomalyco/opencode) · MIT · **cloned** | **Anomaly** | Model-agnostic — any provider incl. local (Meta Muse Spark, Llama, etc. selectable)[^oc-model]; also ships **OpenCode Zen**, a free first-party model gateway (`opencode/` provider) | MIT-licensed (🌐 deepwiki; not stated on opencode.ai). Maker is **Anomaly, not Meta** — Meta lists opencode (and Claude Code) as compatible harnesses for Muse Spark[^meta] |
| `gh copilot` **[installed: `gh`]** | github-copilot-cli | [github/copilot-cli](https://github.com/github/copilot-cli) · *shim* | GitHub / Microsoft | Copilot, MAI-Code, Gemini | Real `/fleet` parallel-agent orchestrator ✓[^fleet] |
| `aider` **[installed]** | aider (v0.86.2) | [Aider-AI/aider](https://github.com/Aider-AI/aider) · Apache-2.0 · **cloned** | Open source (Paul Gauthier) | Any model via LiteLLM (100+ providers) | Apache-2.0; **the reference tool for genuine repo indexing** — dedicated repo-map subsystem (`--map-tokens`, `--map-refresh`, `--show-repo-map`) ✓[^local] |
| `agent` / `cursor-agent` **[installed]** | cursor-cli | [cursor.com/cli](https://cursor.com/cli) · *closed* | Cursor / Anysphere | Cursor Origin, Claude, GPT, Gemini | Entrypoint `agent`, aliased `cursor-agent` — on this box `agent`→grok, so invoked as `cursor-agent` ✓[^cursor]; **being acquired by SpaceX for $60B** (Anysphere → wholly-owned subsidiary; Jun 16 2026 — page does *not* confirm "all-stock")[^spacex] |
| `kilo` | kilo-cli / Kilo Code | [Kilo-Org/kilocode](https://github.com/Kilo-Org/kilocode) · MIT · **cloned** | **Kilo Code Inc.** (Brendan O'Leary, JP Posma) | Multi-provider (BYOK) | MIT; the **CLI is a fork of OpenCode**; the broader Kilo project is a fork of Roo→Cline — *not* Meta[^kilo] |
| `q` / `kiro-cli` | kiro-cli | [kiro.dev/cli](https://kiro.dev/cli/) · *closed* (pred. OSS: [aws/amazon-q-developer-cli](https://github.com/aws/amazon-q-developer-cli), unmaintained) | AWS / Amazon | Claude frontier models | Amazon Q Developer CLI → **Kiro CLI**; `q`/`q chat` remain as aliases ✓[^kiro-rebrand]; Q IDE plugins EOL Apr 30 2027[^amazon-eol] |
| `vibe` | mistral-vibe | [mistralai/mistral-vibe](https://github.com/mistralai/mistral-vibe) · Apache-2.0 · **cloned** | Mistral | Devstral 2, Mistral Medium | "Le Chat" → **"Vibe"** (unified agent, Jun 5 2026; modes Vibe Work/Code/Chat) ✓[^mistral] |
| `devin` / `windsurf` | windsurf-cli / Devin Desktop | [devin.ai/download](https://devin.ai/download/) · *closed* ⚠️ *a fake "official Devin" GitHub org exists — phishing; do not clone it*[^repos] | Cognition (acq. Codeium/Windsurf) | SWE-1.5, SWE-1, BYOK | Windsurf → Devin Desktop (Jun 2026) ⚠️ *(rebrand source now 404s)*; ships **Codemaps** ✓[^codemaps] |
| `warp ai` | warp | [warpdotdev/warp](https://github.com/warpdotdev/warp) · AGPL-3.0 · **cloned** | Warp | GPT, Claude, Gemini, GLM | Terminal itself; **open-sourced under AGPL-3.0** (client mostly AGPLv3, UI crates MIT; Apr 2026)[^warp] |
| `kimi` | kimi-code | [MoonshotAI/kimi-code](https://github.com/MoonshotAI/kimi-code) · MIT · **cloned** | Moonshot AI | K2.x Code | **Agent Swarm** — up to 300 sub-agents / 4,000 steps 🌐[^kimi] |
| `qwen` | qwen-code | [QwenLM/qwen-code](https://github.com/QwenLM/qwen-code) · Apache-2.0 · **cloned** | Alibaba | Qwen 3.x Plus | 1M context; forked from Gemini CLI, now independent; free tier ended Apr 2026 ⚠️ |
| `zcode` | zcode-cli / ZCode IDE | [zcode.z.ai](https://zcode.z.ai/en) · *closed* | Z.ai (Zhipu) | GLM-5.x | Goal Mode; BYOK ⚠️ |
| `mmx` | mmx-cli | [MiniMax-AI/cli](https://github.com/MiniMax-AI/cli) · MIT · **cloned** | MiniMax | MiniMax M3 | Multimodal-first, not coding-agent-first ⚠️ |
| `deepseek` | *(no official CLI)* | — **none** — every `deepseek-cli` is third-party[^repos] | DeepSeek | V3, R1 | **No official first-party CLI exists** ⚠️ — DeepSeek ships models/integrations, not a CLI |
| `mentat`, `gpt-engineer` | (OSS orchestrators) | [AbanteAI/…mentat](https://github.com/AbanteAI/archive-old-cli-mentat) · Apache-2.0 · *archived* · [gpt-engineer-org/gpt-engineer](https://github.com/gpt-engineer-org/gpt-engineer) · MIT · *archived* | Open source | Multi-model (BYOK) | Both archived/read-only; gpt-engineer's README now points to aider ⚠️ |

> **Provider caveat:** the model-family column is the most volatile part of this
> table. Where a specific version string matters, check the vendor's page —
> don't trust any static list (this one included).

---

## 2. What reviewers compare on

Agentic delegation (subagents / teams / parallel orchestration) · CLI/TUI
completeness · native git-worktree isolation · **repo indexing / scanning** ·
hook / event system · spec/plan-driven workflow · MCP support · headless/CI
mode · context window & persistent memory · pricing · open-source status ·
benchmark scores · sandboxing & permissions · local-model / air-gapped support.

---

## 3. Feature matrix

Capability cells for **[installed]** tools are ✓ (probed via `--help`); others
are docs/web. Repo-Indexing uses the A/B/C/D taxonomy in §4; the interface/
surface dimension is broken out in §3a. The **worktree** column is one axis —
*native worktree command · none (git-aware only) · external tooling* — with each
cell tagged by how it was checked.

| Tool | Agentic / delegation | Native git worktree | Repo indexing (§4) | Hooks / events |
|---|---|---|---|---|
| **claude** ✓ | Subagents, agent teams, dynamic workflows, `/batch` | **Native** `/worktree` | **B** — on-demand grep/glob/read + optional LSP + CLAUDE.md prose | Rich hook system (PreToolUse/PostToolUse/…) |
| **codex** ✓ | Subagents; `review`; Codex Cloud | **None native** — no `worktree` in `--help`; run inside a manual worktree ✓ | **C** — AGENTS.md context; no index | Hooks (hook-trust model) |
| **grok** ✓ | Subagents (inline JSON), "run N ways pick best", `--no-subagents` | **Native** `worktree` subcommand + `--worktree`/`--ref` ✓ | **C** — AGENTS.md context; no index | Hooks, skills, plugins |
| **opencode** ✓ | Agent management, ACP server, GitHub agent, `pr` | **None native** (git-aware via ACP/PR) | **B/C** — LSP + context; no semantic index | Plugins; MCP-centric |
| **agy** ✓ | Agents, plan mode | **None native** ✓ | **C** — workspace `--add-dir` context; no index | Plugins |
| **gh copilot** ✓(`gh`) | **`/fleet`** orchestrator → parallel subtasks[^fleet] | **None native** (GitHub PR/issue-aware) | **D** — "Project Polaris" indexing ❓ | GitHub Actions |
| **aider** | Architect+Editor split; no multi-agent delegation | **None native** (community tooling, e.g. Pane) | **A** ✓ — tree-sitter repo map + graph-rank[^aider-map][^aider-ts] | Watch-mode AI comments; no full hook system |
| **cursor-agent** ✓ | **Ask/Plan/Agent modes** (`--mode plan\|ask`) ✓; cloud agents; `--resume` sessions[^cursor] | **Native** `--worktree` (isolated worktrees under `~/.cursor/worktrees`) ✓[^cursor] | **A** — embeddings-based codebase index ⚠️ (server-side; not in `--help`) | Hooks, rules, skills, MCP |
| **kiro-cli** (`q`) | Spec-driven agents, parallel subagents, orchestration | **None first-class** (git-aware) | **A** ✓ — built-in codebase index[^kiro-index] | Hooks on file/commit/tool events |
| **devin/windsurf** | Devin integration, parallel agents | **None native** (git-aware) | **A** ✓ — **Codemaps**[^codemaps] | Devin triggers |
| **grok/kimi/qwen/warp/vibe/zcode/deepseek** | varies (kimi: ~300-agent swarm 🌐[^kimi]) | mostly none native (git-aware) | **C/D** — context-files or unclear; no evidenced index | mostly limited |

---

## 3a. Interface & surfaces

Six independent axes (the old "CLI-complete" column mashed these together).
**Switching** = can you move between surfaces *within one session* — e.g. Claude
Code's CLI↔web handoff.

Legend: ✓ verified locally via `--help` · ✓ per vendor docs (not locally
checked) · ✗ not offered · ~ partial/uncertain · ? unknown. For **[inst]**alled
tools, CLI/Headless/TUI and any surface exposed as a subcommand are ✓;
App/Web/Switching that ride on external products (desktop apps, cloud) are ✓/~
from docs.

| Tool                    | CLI     | Headless           | TUI                      | App / IDE                     | Web                       | Switching                                                  |
| ----------------------- | ------- | ------------------ | ------------------------ | ----------------------------- | ------------------------- | ---------------------------------------------------------- |
| **claude** [inst]       | ✓       | ✓ `-p`             | ✓                        | ✓ desktop + VS Code/JetBrains | ✓ claude.ai/code          | ✓ CLI↔web, same session                                    |
| **codex** [inst]        | ✓       | ✓ `exec`           | ✓                        | ✓ `app` desktop + IDE ext     | ✓ `cloud` (Codex Cloud)   | ~ `--remote`/`remote-control` to a remote app server       |
| **grok** [inst]         | ✓       | ✓ (json/streaming) | ✓                        | ✗                             | ✗                         | ✗ (terminal-only)                                          |
| **opencode** [inst]     | ✓ `run` | ✓ `serve`          | ✓                        | ✗                             | ✓ `web`                   | ✓ `attach` to a running server                             |
| **agy** [inst]          | ✓       | ✓ `--print`        | ✓                        | ~ Antigravity IDE platform    | ?                         | ?                                                          |
| **gh copilot** [inst]   | ✓       | ✓                  | ~ not full-screen        | ✓ VS Code/JetBrains           | ✓ github.com / Workspaces | ~                                                          |
| **aider** [inst]        | ✓ REPL  | ✓ `-m`             | ✗ line-oriented          | ✗                             | ✓ `--gui`/`--browser`     | ✗                                                          |
| **cursor-agent** [inst] | ✓       | ✓ `-p`             | ✓                        | ✓ Cursor IDE                  | ~ cloud agents            | ~ (`--resume`/`--continue`)                                |
| **kiro** (`q`)          | ✓       | ✓                  | ✓                        | ✓ Kiro IDE                    | ✓                         | ✓ one "unified agent harness" → IDE/CLI/Web[^kiro-surface] |
| **devin/windsurf**      | ✓       | ~                  | ~                        | ✓ Devin Desktop               | ✓                         | ~                                                          |
| **warp**                | ✓       | ~                  | ✓ (is itself a terminal) | ✓ desktop terminal app        | ~                         | ~                                                          |

*Tail (mmx, deepseek, vibe, zcode, qwen, kimi, mentat, gpt-engineer): mostly CLI,
some headless; TUI varies; App/Web/Switching generally ✗ or unknown — thin
surface presence, not individually verified.*

---

## 4. Repo indexing / scanning — the axis that gets overstated

This is the fuzziest column in every comparison, so it gets its own taxonomy.
"Repo indexing" conflates four genuinely different architectures:

| Class | What it means |
|---|---|
| **A — genuine index artifact** | Builds a persistent structural/semantic index: embeddings, a tree-sitter repo map, a call/symbol graph, or an on-disk cache queried across sessions |
| **B — live on-demand retrieval** | No stored index; the agent navigates each session with grep/glob/read (+ optional LSP for symbols). Full-fidelity, never stale, but re-derived every time |
| **C — context files only** | Loads author-written context (AGENTS.md / CLAUDE.md / rules). Not an index at all |
| **D — unclear / marketing** | Claim exists but no verifiable indexing mechanism |

**The core insight:** these are *different architectures, not a single quality
scale.* Pre-indexed retrieval (A) trades staleness/lossiness for fast whole-repo
recall; agentic on-demand search (B) trades that recall for fidelity and
freshness. Grading a (B) tool as "behind" the (A) tools misreads the design.
Most tools marketed as "repo indexing" are actually (B)+(C).

| Tool | Class | Evidence |
|---|---|---|
| **aider** | **A** ✓🌐 | tree-sitter builds the map[^aider-ts]; symbols ranked by a graph algorithm (NetworkX PageRank, per secondary docs)[^aider-map][^aider-pr] → token-budgeted repo map. Locally confirmed (v0.86.2): dedicated `--map-tokens`, `--map-refresh`, `--show-repo-map`.[^local] The axis's reference implementation. |
| **cursor-cli** | **A** ⚠️ | Cursor's well-known embeddings-based codebase index — a **server-side** feature, **not surfaced in `cursor-agent --help`** (installed + probed 2026-07-18; the index isn't a CLI-visible artifact, so it stays reported; the "Cursor-Small" label is unconfirmed). |
| **kiro-cli** | **A** ✓ | Built-in codebase index: auto-indexes on project open, updates on file change, manually rebuildable[^kiro-index]. Note: **"KiroGraph" is an independent community project** (davide-desio-eleva, MIT) — *not* a Kiro built-in. |
| **Windsurf / Devin** | **A** ✓ | **Codemaps** — a real Cognition feature generating **AI-annotated structured** maps of a codebase[^codemaps]. |
| **claude-code** | **B** ✓ | grep/glob/read + optional LSP "code intelligence" plugins[^claude-mech] + prose CLAUDE.md. *No persistent index* is an inference from the documented tool set — the docs neither assert nor deny one. ("ultracode scan" / "Project knowledge" are **not** Claude Code features — the latter is a claude.ai *chat* concept.) |
| **codex** | **C** ✓ | AGENTS.md context; on-demand file ops. No index. |
| **grok** | **C** ✓ | AGENTS.md context. No index. |
| **opencode** | **B/C** ✓ | LSP + context; ACP/MCP. No semantic index. |
| **agy** | **C** ✓ | Workspace dirs + plan mode. No index. |
| **gh copilot** | **D** ❓ | `/fleet` is real orchestration (not indexing)[^fleet]; "Project Polaris deep indexing" has no verifiable source. |
| **qwen / kimi / zcode / warp / vibe / deepseek** | **C/D** ⚠️ | Context-file or unclear; no evidenced persistent index. |

**What a genuine (A) index actually produces** — aider's real `--show-repo-map`
output on a two-file sample repo (verified locally, v0.86.2). tree-sitter
extracts the definitions across files; `⋮` marks elided bodies, `│` the
retained signature lines, budgeted to the token limit:

```text
Repo-map: using 1024 tokens, auto refresh

main.py:
⋮
│def run():
⋮

shapes.py:
│class Shape:
│    def area(self): raise NotImplementedError
│
│class Circle(Shape):
│    def __init__(self, r): self.r = r
│    def area(self): return 3.14159 * self.r * self.r
│
│def total_area(shapes):
⋮
```

That structural, cross-file symbol summary — not raw file contents, not a grep
hit — is what distinguishes (A) from (B)/(C).

---

## 5. Agentic delegation & parallelism (the axis these tools actually compete on)

- **claude** — subagents, agent teams, dynamic multi-agent workflows, `/batch`; native worktree isolation. ✓
- **grok** — subagents from inline JSON, "run the task N ways in parallel and pick the best" (headless), first-class git-worktree management. ✓ (Secondary sources say up to **8 parallel sub-agents in isolated worktrees**; the mechanism is verified locally, the exact "8/default" is ⚠️.)
- **kimi** — "Agent Swarm" scaling to **300 sub-agents** across 4,000 coordinated steps (up from K2.5's 100/1,500). 🌐[^kimi]
- **kiro** — spec-driven agents (requirements → design → tasks → execution), parallel subagents with isolated context. ✓ docs
- **gh copilot** — `/fleet`: an orchestrator agent decomposes an implementation plan into independent subtasks run in parallel *by subagents*. ✓[^fleet]
- **codex** — subagents, `review`, Codex Cloud tasks. ✓
- **aider** — deliberately *not* multi-agent: a two-model Architect+Editor split. ✓🌐

---

## 6. Pricing & free tier *(as reported, ~mid-2026)*

| Tool | Free tier | Paid entry | Model |
|---|---|---|---|
| Claude Code | No (free claude.ai excluded) | ~$17–20/mo; Max from ~$100 | Subscription + usage caps |
| Codex CLI | No CLI free tier | ~$20/mo (ChatGPT Plus) | Subscription |
| Antigravity (agy) | Generous free daily quota | ~$20/mo | Free quota or sub/API |
| OpenCode | Tool is free | BYOK or existing sub | Pay provider / local |
| Aider | Tool is free | BYOK | Pay provider / local (Ollama) |
| Copilot CLI | Free (limited) | ~$10/mo Pro | Usage credits |
| Kiro CLI | Preview tier | ~$100/mo Pro Max | Subscription |
| Grok Build | Via SuperGrok / X Premium+ | ~$30–300/mo | Subscription-gated |
| Warp | Freemium + BYOK | usage credits | Usage-based |

---

## 7. Open-source status, license & GitHub stars

| Tool | License | Open source? |
|---|---|---|
| OpenCode | MIT (Anomaly; 🌐 deepwiki) | ✓ — most-starred OSS coding agent |
| Antigravity / (ex-Gemini CLI) | Apache-2.0 | ✓ |
| Codex CLI | Apache-2.0 | ✓ (proprietary model backend) |
| Aider | Apache-2.0 | ✓ |
| Cline / Kilo Code | Apache-2.0 / MIT | ✓ (Kilo = Roo→Cline fork)[^kilo] |
| Goose | Apache-2.0 | ✓ |
| Warp | AGPL-3.0 client (UI crates MIT) | ✓ (cloud "Oz" stays proprietary)[^warp] |
| Qwen Code / Trae | Apache-2.0 / MIT | ✓ self-hostable |
| **Claude Code** | Proprietary | ❌ (repo is issues/docs only; most-starred *repo* overall) |
| Kiro / Cursor / Copilot / Grok Build | Proprietary | ❌ |

**GitHub stars** *(~June 2026; noisy but the ordering is informative)*[^stars] —
*re-verification on 2026-07-18 was blocked (source returned 429); figures below
are as originally reported ~Jun 2026, not re-confirmed today:*

| Project | Stars (~Jun 2026) |
|---|---|
| OpenCode | 172,198 — most-starred *open-source* coding agent |
| Claude Code | 131,380 — most-starred *repo* overall, but the repo is issues/docs only (the tool itself is closed) |
| Gemini CLI (→ Antigravity) | 105,104 |
| Codex CLI | 89,991 |
| Cline | 62,996 |
| Warp | ~56,000 |
| Goose | 48,542 |
| Aider | 45,945 |
| Kilo Code | 19,968 |
| Qwen Code, Trae Agent | not reported |
| Kiro, Cursor CLI, Copilot CLI, Grok Build | closed-source — n/a |

---

## 8. Benchmarks *(as reported, ~mid-2026)*

These come from different harnesses and evaluators, so they **aren't comparable
across rows** — each figure is meaningful only against others from the same
source (how-to-read guidance, not distrust). Some are self-reported, as noted.
*Re-verification on 2026-07-18 was blocked (source 429); figures are as
originally reported, not re-confirmed today.*[^stars]

| Tool (model) | Terminal-Bench 2.1 | SWE-bench Pro / Verified |
|---|---|---|
| Codex CLI (GPT-5.5) | **83.4%** (±2.2) — #1 | 59.1% Pro |
| Claude Code (Opus 4.8) | 78.9% (±2.5) — #2 | 51.9% Pro (agent-reported); 69.2% Pro / 88.6% Verified (model-only, self-reported) |
| Antigravity / Gemini CLI (Gemini 3.1 Pro) | 70.7% (±2.9) | 43.3% Pro |
| Qwen Code (Qwen 3.6 Plus) | — | **78.8% Verified** (highest Verified in the lineup) |
| Kimi Code (K2.6 → K3) | K3: **88.3%** | 58.6% Pro (K2.6) |
| Copilot CLI (Sonnet 4.6 + GPT-5.5) | — | ~70% |
| Amazon Q / Kiro (Claude Sonnet) | — | ~58% |
| Trae Agent | — | ~55% |
| Meta Muse Spark 1.1 | 80.0% (Meta's own eval) | — |

---

## 9. Sandboxing & permissions

| Tool | Model |
|---|---|
| **claude** | Per-tool permission prompts; "Safe Mode" for sensitive ops |
| **codex** | Native sandbox (`--sandbox read-only/workspace-write/danger-full-access`); approval policies; full-trajectory logs ✓ |
| **grok** | Local execution (air-gap compatible); Plan-Review-Approve gate |
| **agy** | `--sandbox` terminal restrictions ✓ |
| General CLI agents | Run under the OS user's own permissions; sandbox via containers/restricted shells |

---

## 10. Local-model / air-gapped support

| Tool | Local support |
|---|---|
| Aider | `--model ollama/<name>`; fully offline once pulled; git-native |
| OpenCode | Ollama, LM Studio, llama.cpp + 75+ hosted providers |
| **Codex** | ✓ **native** — `--oss` + `--local-provider lmstudio\|ollama` (verified locally; *corrects the common "no local path" claim*)[^local] |
| Cline / Goose | Ollama, LM Studio, OpenAI-compatible endpoints |
| Grok Build | Local / air-gap compatible mode ⚠️ |
| Claude Code, Copilot CLI, Kiro, Cursor CLI | No native local/self-hosted model path (hosted account/API required) |

> **Hosted-model routing — functional, but "unsupported" ≠ "blessed."** Claude
> Code isn't tied to Anthropic-*hosted* models; it's tied to the Anthropic
> **Messages API**. The `ANTHROPIC_BASE_URL` + `ANTHROPIC_AUTH_TOKEN` gateway
> path is officially documented (Bedrock, Vertex, corporate proxies,
> LiteLLM-as-transport) — **but** Claude Code's docs *explicitly* state Anthropic
> "doesn't support routing Claude Code to non-Claude models through any
> gateway."[^claude-gateway] So it *works* only insofar as the upstream
> implements the full Messages contract, and features degrade on a
> non-`api.anthropic.com` host: MCP tool-search is off by default (re-enable via
> `ENABLE_TOOL_SEARCH=true`)[^claude-envvars], Remote Control is disabled
> (v2.1.196+), and adaptive reasoning / context management / structured outputs
> can `400` unless the upstream supports them. **Example:** Meta's Model API
> ships a Claude Code snippet pointing `ANTHROPIC_BASE_URL=https://api.meta.ai`
> and remapping every tier (`ANTHROPIC_MODEL`,
> `ANTHROPIC_DEFAULT_{OPUS,SONNET,HAIKU}_MODEL`, `CLAUDE_CODE_SUBAGENT_MODEL`) to
> `muse-spark-1.1` — plus `ENABLE_TOOL_SEARCH=true` for exactly the reason above
> (snippet pasted by the report's user from the Meta dashboard)[^meta]. The
> tier-remap is required: otherwise Claude Code's internal Haiku/Opus routing
> calls model names the foreign endpoint doesn't serve. This is *hosted* routing,
> functional-but-unsupported; the local/air-gapped column above still holds.

---

## 11. Bottom line

For maximum agentic sophistication in a pure-CLI package with native
git/worktree isolation:

1. **`claude`** — broadest agentic surface (subagents, agent teams, dynamic
   workflows, `/batch`), native worktrees, rich hooks, Safe Mode. Repo
   understanding is **on-demand retrieval (B)**, not a persistent index — a
   deliberate architecture, not a gap.
2. **`grok`** — subagents + "run N ways pick best" + first-class worktree
   isolation, air-gap-capable local execution.
3. **`kiro-cli` (`q`)** — spec-driven workflow (requirements → design → tasks),
   parallel subagents, and a **genuine built-in codebase index (A)**.
4. **`aider`** — best-in-class **genuine repo indexing (A)**: tree-sitter repo
   map + graph-rank symbol scoring; atomic git commits; fully local via Ollama.
   No multi-agent delegation.
5. **`codex`** — top-tier benchmarks, native sandbox + audit logs, **and** a
   native local-model path; Apache-2.0.
6. **`cursor` (`cursor-agent`)** — Plan/Ask/Agent modes + native `--worktree`
   isolation (both verified locally) and a semantic index; ownership in flux
   (SpaceX acquisition, Q3 2026).

If open-source / self-hostable is the priority: **OpenCode** (MIT, Anomaly,
model-agnostic) and **Aider** (Apache-2.0, git-native, Ollama-friendly) are the
strongest; **Antigravity** offers the most generous no-card free quota.

**On "repo indexing" specifically:** only **aider, cursor, Kiro's built-in
index, and Windsurf/Devin Codemaps** are genuine index artifacts (A).
Claude/codex/grok/opencode/agy do **on-demand retrieval + context files (B/C)** —
a different architecture, not an inferior one. Treat any table cell that grades
these on one scale with suspicion.

---

## 12. Verification summary

- **Verified locally** (`--help`, this machine, 2026-07-18): claude, agy, codex,
  grok, opencode, gh, aider, ollama capabilities; codex native local-model path
  and *absence* of a worktree subcommand; grok subagents + worktree; opencode
  model-agnosticism + Anomaly footer; aider repo-map subsystem; no semantic-index
  surface in any other installed tool.
- **Re-verified against live sources** (web pass, 2026-07-18): 12 claims
  confirmed with verbatim quotes; 6 "partly" (wording corrected to match source —
  see the ⚠️ notes on SpaceX "all-stock", Codemaps "structured" not
  "hierarchical", Kiro "unified agent harness", Claude "no index" as inference,
  opencode MIT; cursor modes/worktree were ⚠️ from the forum page, then
  **confirmed locally** via `cursor-agent --help`); 3 sources unreachable this pass (Meta
  blog SPA, Devin-rebrand blog 404, morphllm 429 — noted inline).
- **Not re-confirmable today:** star counts / benchmarks (source rate-limited);
  the Windsurf→Devin-Desktop rebrand (source 404); Meta's "harness compatibility"
  wording (blog didn't render — grounded instead by the local install script +
  the user's Meta dashboard).

---

## 13. Sources & re-verification ledger

Each footnote: **verdict** · verbatim source quote · URL · source date ·
`checked` date. To re-verify, open the URL and confirm the quote still stands (or
mark it stale). Local `--help` claims cite the machine, not a URL.

[^local]: **verified (local).** `--help` on this machine, 2026-07-18 — aider
v0.86.2 (`--map-tokens`/`--map-refresh`/`--show-repo-map`, `--gui`/`--browser`,
`-m`); codex (`exec`, `app`, `cloud`, `review`, `--oss`/`--local-provider`,
`--sandbox`, *no* `worktree`); grok (`worktree`, `--worktree`/`--ref`, headless
json); opencode (`run`/`serve`/`web`/`attach`, agent mgmt); agy (`--print`,
`--sandbox`). `checked` 2026-07-18.

[^repos]: **repo-verification pass (2026-07-18).** One agent per tool confirmed the official source repo (or, for closed tools, the official download page) resolves to the *right* project — filtering forks, mirrors, and same-named squats. Open-source repos with real (non-shim) source were shallow-cloned into `~/src-ext/`: opencode, kilocode, aider, codex, grok-build, qwen-code, kimi-code, minimax-cli, mistral-vibe, warp. **Shims** (public repo wrapping a proprietary binary — limited source value): anthropics/claude-code, github/copilot-cli. **No official CLI:** deepseek (every `deepseek-cli` is third-party). **Archived/read-only:** AbanteAI mentat, gpt-engineer-org/gpt-engineer. ⚠️ **Phishing:** a fake "official Devin" GitHub org (`DevinAI-agent/devin-AI`, ~1 star) impersonates Cognition — the real download is devin.ai; do not clone the impostor. Kiro's predecessor `aws/amazon-q-developer-cli` is OSS but unmaintained (security fixes only).

[^claude-gateway]: **confirmed.** "Anthropic doesn't endorse, maintain, or audit third-party gateway products, and doesn't support routing Claude Code to non-Claude models through any gateway." — https://code.claude.com/docs/en/other-llm-gateways.md (redirects to .../llm-gateway) · date unknown · `checked` 2026-07-18.

[^claude-envvars]: **confirmed.** "When set to a non-first-party host, MCP tool search is disabled by default. Set `ENABLE_TOOL_SEARCH=true` if your proxy forwards `tool_reference` blocks." — https://code.claude.com/docs/en/env-vars.md · date unknown · `checked` 2026-07-18.

[^claude-mech]: **partly.** Grounds the mechanism (search + optional code-intelligence plugins): "See type errors and warnings after edits, jump to definitions, find references (requires code intelligence plugins)." The page never asserts *or denies* a persistent index — "no index" is inference from the tool set. — https://code.claude.com/docs/en/how-claude-code-works.md · date unknown · `checked` 2026-07-18.

[^oc-model]: **confirmed (model-agnostic).** "Free models included or connect any model from any provider, including Claude, GPT, Gemini and more." Footer: "©2026 Anomaly". Slug is `anomalyco/opencode`; MIT not stated on this page (see deepwiki). — https://opencode.ai/ · date unknown · `checked` 2026-07-18.

[^meta]: **partly / page-unavailable.** The Meta blog is a JS SPA that returned no body this pass, so the "drop-in compatible with … OpenCode and Claude Code" line couldn't be re-quoted. Grounded instead by primary session evidence: the installer at https://dev.meta.ai/cli/install-opencode.sh fetches mainline opencode (opencode.ai/install; github.com/anomalyco/opencode) and only adds a Meta-provider config; and the user's Meta Model API dashboard lists OpenCode / Claude Code / Codex as install tabs. URLs: https://developer.meta.com/ai/resources/blog/build-with-muse-spark/ · https://ai.meta.com/blog/introducing-muse-spark-meta-model-api/ · `checked` 2026-07-18.

[^kilo]: **confirmed.** "A month ago, we forked Roo Code, which itself was a fork of Cline—the most popular AI coding agent" — https://blog.kilo.ai/p/roo-or-cline-were-building-a-superset · Apr 10 2025 · `checked` 2026-07-18.

[^kiro-rebrand]: **confirmed.** "You can still continue using the `q` and `q chat` entry points." (Q Developer CLI auto-updates to Kiro CLI.) — https://kiro.dev/docs/cli/migrating-from-q/ · Jul 1 2026 · `checked` 2026-07-18.

[^amazon-eol]: **confirmed.** "Amazon Q Developer IDE plugins and paid Subscriptions will reach end of support on April 30, 2027, giving customers 12 months to transition to Kiro." (Q in the AWS Console is *not* part of this sunset.) — https://aws.amazon.com/blogs/devops/amazon-q-developer-end-of-support-announcement/ · Apr 30 2026 · `checked` 2026-07-18.

[^gemini]: **confirmed.** "On June 18, 2026, Gemini CLI and Gemini Code Assist IDE extensions will stop serving requests for Google AI Pro and Ultra, as well as those using it free of charge." (Enterprise licensees retain access.) — https://developers.googleblog.com/an-important-update-transitioning-gemini-cli-to-antigravity-cli/ · May 19 2026 · `checked` 2026-07-18.

[^mistral]: **confirmed.** "Le Chat has become Vibe, Mistral's unified agent for professional productivity and coding tasks." (Modes: Vibe Work / Vibe Code / Vibe Chat.) — https://help.mistral.ai/en/articles/682992-le-chat-is-now-vibe · Jun 5 2026 · `checked` 2026-07-18. Also https://mistral.ai/news/devstral-2-vibe-cli/.

[^spacex]: **partly.** "SpaceX said it will acquire Cursor's parent company, Anysphere, Inc., which will become a wholly owned subsidiary of the rocket company." Acquirer/target/value ($60B)/date all match; the page does **not** support "all-stock" (no cash/stock breakdown; framed as an outright purchase, close ~Q3 2026). — https://www.forbes.com/sites/siladityaray/2026/06/16/spacex-will-buy-ai-coding-firm-cursor-for-60-billion/ · Jun 16 2026 · `checked` 2026-07-18. Corroborated: cnbc.com/2026/06/16/spacex-spcx-cursor-acquisition-ipo.html · devops.com.

[^aider-map]: **partly.** "It does this by analyzing the full repo map using a graph ranking algorithm, computed on a graph where each source file is a node and edges connect files which have dependencies." (Confirms graph-ranking; this page does *not* say "tree-sitter" or "PageRank".) — https://aider.chat/docs/repomap.html · date unknown · `checked` 2026-07-18.

[^aider-ts]: **confirmed (secondary).** "Aider uses tree sitter to build the map. It specifically uses the py-tree-sitter-languages python module." — https://aider.chat/2023/10/22/repomap.html · Oct 22 2023 · `checked` (earlier research pass).

[^aider-pr]: **secondary.** "Files are then ranked using NetworkX's PageRank algorithm with personalization based on chat context." — deepwiki (Aider-AI/aider repository-mapping) · `checked` (earlier research pass).

[^kiro-index]: **confirmed.** "When you first open a project in Kiro, it automatically begins indexing all files in your workspace." (New/changed files re-indexed automatically.) — https://kiro.dev/docs/editor/codebase-indexing/ · Nov 16 2025 · `checked` 2026-07-18.

[^kiro-surface]: **partly.** "CLI 3.0 is built on the same unified agent harness that powers the Kiro IDE and Kiro Web." (Substance — one engine → IDE/CLI/Web — confirmed; the term "Agent Server" does not appear.) — https://kiro.dev/docs/cli/v3/ · Jun 17 2026 · `checked` 2026-07-18.

[^codemaps]: **partly.** "first-of-its-kind AI-annotated structured maps of your code, powered by SWE-1.5 and Claude Sonnet 4.5." (Confirms Codemaps as a Cognition/Windsurf codebase-map feature; the page says "structured", not "hierarchical".) — https://cognition.com/blog/codemaps · Nov 4 2025 · `checked` 2026-07-18. Also https://docs.windsurf.com/windsurf/codemaps (307 → docs.devin.ai).

[^fleet]: **confirmed.** "The `/fleet` slash command in Copilot CLI is designed to take an implementation plan and break it down into smaller, independent tasks that can be executed in parallel by subagents." (Orchestrator coordinates; subagents execute.) — https://docs.github.com/en/copilot/concepts/agents/copilot-cli/fleet · date unknown · `checked` 2026-07-18.

[^fable5]: **confirmed.** "Today we're launching Claude Fable 5: a Mythos-class model that we've made safe for general use." — https://www.anthropic.com/news/claude-fable-5-mythos-5 · Jun 9 2026 · `checked` 2026-07-18.

[^warp]: **confirmed.** "parts of Warp that aren't part of the now open source Warp client — such as Warp Drive — are designed into Oz and remain proprietary." (Client "mostly AGPLv3"; UI components MIT.) — https://fossforce.com/2026/05/after-years-of-teasing-warp-finally-goes-open-source/ · May 4 2026 · `checked` 2026-07-18.

[^kimi]: **confirmed.** "The architecture scales horizontally to 300 sub-agents executing across 4,000 coordinated steps simultaneously." (Up from K2.5's 100 / 1,500.) — https://www.marktechpost.com/2026/04/20/moonshot-ai-releases-kimi-k2-6-with-long-horizon-coding-agent-swarm-scaling-to-300-sub-agents-and-4000-coordinated-steps/ · Apr 20 2026 · `checked` 2026-07-18.

[^cursor]: **confirmed (local `--help`, supersedes the forum page).** `cursor-agent --help` (v2026.07.16, this machine, 2026-07-18) confirms what the forum post omitted: `--mode plan|ask` ("plan: read-only/planning… ask: Q&A style… read-only") plus the default agent mode; and `-w, --worktree [name]` → "Start in an isolated git worktree at `~/.cursor/worktrees/<reponame>/<name>`" (+ `--worktree-base`, `--skip-worktree-setup`); headless `-p` / `--output-format json|stream-json`; `mcp` subcommand; `--sandbox`. On this box the binary's own `agent` name is shadowed by grok, so it's invoked as `cursor-agent`. Forum page grounds the entrypoint only: "The new primary entrypoint is `agent` (`cursor-agent` still works as an alias)." — https://forum.cursor.com/t/cursor-cli-jan-8-2026-new-commands-and-performance-improvement/148372 · Jan 8 2026 · `checked` 2026-07-18.

[^stars]: **page-unavailable (this pass).** Star counts + benchmark figures could not be re-fetched — the source returned HTTP 429 across retries on 2026-07-18. Figures in §7/§8 are as originally reported (~Jun 9 2026) and were **not** re-confirmed today. — https://www.morphllm.com/best-ai-cli-tools-2026 · https://www.morphllm.com/ai-coding-assistant-open-source · also hackmd.io/@Ali-G/rJ2vZYtbGx (the broader comparison this table descends from) · stackoverflow.blog/2026/05/27/agents-on-a-leash-agentic-ai-remains-mostly-monitored-at-work (survey).

**Still unverified (❓):** Copilot "Project Polaris" deep indexing (no source
found); the specific "Cursor-Small semantic index" label (Cursor *does* index via
embeddings, but that label wasn't confirmed).
