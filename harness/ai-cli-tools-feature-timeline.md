# AI Coding CLI Tools — Feature Diffusion Timeline (2024–2026)

*Who shipped each key feature **first**, and how it propagated across the
ecosystem. Traced by 8 parallel agents against **primary sources** — local git
history + CHANGELOG/HISTORY files in the 10 cloned repos, npm publish
timestamps, `gh` PR/commit dates, and official changelogs/docs for the
proprietary tools (Claude Code, Cursor, Copilot CLI, Kiro, Gemini CLI/Antigravity,
Warp). Assessment pass: **2026-07-18/19**.*

> **Read-this-first caveats.** (1) This session's **web-search budget was
> exhausted**, so proprietary-tool dates come from *fetched* canonical
> changelogs/docs, not search — and a few (Cursor, Copilot CLI, Kiro, Antigravity)
> have **unresolved or low-confidence** first-ship dates, flagged inline. (2)
> Mirror/shallow clones (grok, kimi, warp) give **upper-bound** dates (the feature
> was present *by* the clone date; true intro is earlier). (3) **"Originator" =
> first within this coding-CLI cohort**, not inventor of the concept — three
> features (MCP, ACP, code-mode) have their real origin *outside* any CLI.

---

## Headline: who originates

| Feature | In-cohort originator | Date | The tell |
|---|---|---|---|
| **Subagents** (Task/dispatch tool) | **Claude Code** | 2025-02-24 (GA) | opencode copied the tool's help-text near-verbatim ~1 month later |
| **MCP** | **Claude Code** *(Anthropic's protocol)* | ~2025-03-05 | Anthropic's own standard (announced 2024-11-25); **aider is the lone holdout** |
| **Plan mode** (read-only, Shift+Tab) | **Claude Code** | ~2025-06 | `Shift+Tab` + `/plan` + `enter/exit_plan_mode` recur near-verbatim everywhere |
| **Hooks** | **opencode** (strict) / **Claude Code** (template) | 2025-06-26 / 2025-06-30 | 4 days apart, independent; **CC's** `PreToolUse`/`PostToolUse` vocab is what spread |
| **ACP** (editor protocol) | **Gemini CLI** *(Zed's protocol)* | 2025-07-17 | Zed co-developed Gemini CLI as its reference agent; **Claude Code is *not* native** (Zed adapter) |
| **Skills** (`SKILL.md`) | **Claude Code** | 2025-10-16 | `SKILL.md` / "Agent Skills" copied verbatim; `~/.agents/skills` became an interop convention |
| **Worktrees** (managed isolation) | **opencode** | 2026-01-02 | CC (fast-follower, Feb 19) coined the `EnterWorktree`/`.claude/worktrees/` vocab that spread |
| **Code-mode** (model writes tool-calling code) | **codex** | 2026-02-11 | the one feature led by *neither* Anthropic nor opencode; pattern from Anthropic-API + Cloudflare |

**Bottom line:** **Anthropic/Claude Code is the ecosystem's pacemaker** — it
originated or co-originated **5 of 8** and its *naming* propagates verbatim even
where others re-implement independently. **opencode is the open-source pacemaker**
(originated worktrees, co-originated hooks, upstream to Kilo, usually ~1 month
behind Claude Code and sometimes ahead). But **not everything is Anthropic**: ACP
is Zed's, code-mode is codex's, worktrees are opencode's.

---

## How each feature propagated

**Subagents** — Claude Code shipped the delegating `dispatch_agent`/Task tool at
its **2025-02-24** GA. **opencode** cloned the pattern **2025-03-27** (its
`agent-tool.go` help text is nearly identical). Then Warp (Jun 20, "Agent
Management Panel"), **Gemini CLI** (Aug 7, "Foundational Subagent Architecture") →
its fork **qwen-code** (Sep 12); **codex** arrived via `/review` delegation
(Oct 2025) maturing into full `spawn_agent` parallelism (Feb 2026); mistral-vibe
(Jan 27), kimi ("agent swarm", May–Jun), grok, Kiro ("introspect subagent", Jul).
Fork chains: **kilo←opencode**, **qwen←Gemini CLI**. **Absent: aider.**

**MCP** — Anthropic's cross-vendor protocol (2024-11-25). In-cohort first =
**Claude Code** (~2025-03-05, likely at its Feb launch). An unusually fast
~4-month cluster: Cursor (Feb, low-conf) · codex (Apr 17; +tool-as-server May 2) ·
Amazon Q (Apr 29) · opencode (May 18) · Gemini CLI (Jun 25) · Kiro (Jul) ·
**Copilot CLI (Sep 25, ships GitHub's MCP server by default)**. Newer tools
(mistral, kimi, grok, warp) ship it from day one. **Lone holdout: aider** (zero
MCP anywhere; its maintainer was publicly MCP-skeptical).

**Plan mode** — **Claude Code** originated the read-only `Shift+Tab` mode
(~Jun 2025). aider's *architect mode* (2024-09-26) is a **conceptual precursor but
distinct** (two-model split that still edits). Then opencode ("modes concept",
Jul 9) · **qwen-code (Sep 24 — ~6 months *before* its own Gemini-CLI upstream)** ·
mistral (Dec) · **codex** ("collaboration modes" Plan/Execute, Jan 2026) ·
Gemini CLI (finally, ~Mar 2026) · warp/kilo (Apr) · kimi (May) · Cursor/grok. The
verbatim `Shift+Tab`+`/plan`+`enter/exit_plan_mode` convention is the clearest
Claude-Code fingerprint in the dataset. Different paradigm: **Kiro** (spec-driven).

**Hooks** — **two independent June-2025 origins**: opencode (commit 2025-06-26,
strict-earliest) and Claude Code (v1.0.38, 2025-06-30). **Claude Code's** richer
event vocabulary (`PreToolUse`/`PostToolUse`/`SessionStart`/`Stop`/…) is what
every follower copied — opencode's `file_edited`/`session_completed` naming did
*not* spread. Kiro (Jul 2025) → a dense 2026 wave: **codex + qwen the same day
(Mar 9)**, mistral (Apr 28), kimi (May 22), Cursor (cloud, Jul). **Absent: Gemini
CLI upstream** (its fork qwen built hooks independently, citing "CC 2.1.168
parity"), **aider** (only git-hooks bypass), Warp (no *agent* hooks found).

**ACP** — **not Anthropic's**: it's **Zed Industries'** LSP-inspired protocol
(announced 2025-08-27). In-cohort originator = **Gemini CLI** (Zed's co-developed
reference agent, commit 2025-07-17). Crucially, **Claude Code is *not* native** —
it's drivable only through Zed's external `@zed-industries/claude-code-acp`
adapter. opencode (Oct 20) → **kilo inherits it wholesale** (same commit) ·
mistral (ACP-first — dedicated `vibe-acp` binary at its Dec 9 launch) · kimi
(`kimi acp`, Jun 2026) · Copilot CLI (native, undated) · grok. **Absent: codex,
aider.** Warp only *plans* to (can launch *others'* ACP agents as a client).

**Skills** — **Claude Code** originated `SKILL.md`/"Agent Skills" (v2.0.20,
**2025-10-16**). A tight **December-2025** fast-follower cluster: codex (Dec 1,
`skills.md`) · qwen-code (Dec 10) · opencode (Dec 21, "add Agent Skills support") ·
**kilocode + Gemini CLI both Dec 30**. A cross-tool `~/.agents/skills` shared
directory emerged as a **de-facto interop standard**. 2026 wave: mistral
(Mar/Apr) · warp (Apr 28) · kimi (May 22, *baked in from inception*) · Cursor
(Jun 22) · Copilot CLI (~Jul 1) · grok (Jul 16). **Absent: aider, Kiro, Amazon Q.**
Inversion: **qwen (a Gemini fork) shipped skills ~2.5 weeks *before* its own
upstream.**

**Worktrees** — **opencode originated** managed git-worktree isolation
(commit #6674, **2026-01-02**), ~7 weeks ahead. **Claude Code** was the
fast-follower (`--worktree`/`-w` + declarative subagent `isolation:"worktree"`,
v2.1.49, **2026-02-19**) and **coined the `EnterWorktree`/`ExitWorktree` tool +
`.claude/worktrees/` convention** that then spread. Gemini CLI (Apr 1) · Warp
(Apr 28, adopts `.claude/worktrees/`) · **kilo inherits opencode's** · qwen
(May 21 — re-implemented using *Claude Code's* `EnterWorktree` names, a naming
cross-pollination into the Gemini line) · mistral (Jul 3) · grok (~Jun).
**Absent: codex** (has "worktree" only in its *own* CI tooling), **aider, Cursor,
Copilot CLI, Kiro.**

**Code-mode** — the outlier: led by **neither Anthropic nor opencode**. Pattern
lineage is *upstream* of the CLIs — the **CodeAct** research pattern (2024) →
**Anthropic's Programmatic Tool Calling** *API* (`code_execution_20250825`, ~Aug
2025 — **not** in the Claude Code CLI) → **Cloudflare** coins the name "Code Mode"
(2025-09-26). Among CLIs, **codex is the clear originator and far ahead**:
`js_repl` (Feb 11 2026) → renamed `code_mode` (Mar 9) → in-process **V8** crate
(Mar 20) → hosted-by-default (Jul 8). **opencode** followed ~5 months later
(`packages/codemode`, confined execution, Jul 2–3 2026 — an independent SST
implementation). **No fork inheritance yet** (kilo hadn't pulled it). **Claude
Code CLI: absent** (exists only as an Anthropic *API* feature). Everyone else:
absent.

---

## Cross-cutting patterns

1. **Naming is the fingerprint of single-source diffusion.** `EnterWorktree`,
   `PreToolUse`/`PostToolUse`, `SKILL.md`/"Agent Skills", `Shift+Tab` plan,
   `dispatch_agent` — these travel *verbatim*, even into tools that implemented
   the feature independently. That's how you can tell Claude Code set the de-facto
   standard, not just shipped early.

2. **Two pacemakers, different lanes.** Anthropic/Claude Code paces the
   *proprietary + concept* frontier (subagents, MCP-in-cohort, plan-mode, skills,
   hooks); **opencode** paces the *open-source* frontier (worktrees, hooks,
   consistently ~1 month behind CC, sometimes ahead) and is the upstream Kilo
   forks from.

3. **Forks inherit — but sometimes *out-run* their parent.** kilo←opencode and
   qwen←Gemini-CLI carry features wholesale, yet **qwen shipped skills, plan-mode,
   and hooks *before* its own upstream**, implementing them independently against
   the Claude-Code template. A fork lapping its source is the dataset's most
   surprising motif.

4. **Diffusion is fast and accelerating.** Core-cluster lag is ~1–3 months; by
   2026 new entrants (kimi, grok, warp, mistral) ship the *entire* feature set
   "baked in from their first public commit" — these capabilities became
   **table-stakes**, not differentiators.

5. **aider is the principled holdout.** Absent from MCP, skills, ACP, worktrees,
   subagents, hooks, and code-mode. It's a different philosophy (text-diff editing,
   no tool-calling, no MCP) — and, per the source assessment, its cadence is
   slowing. It contributed *precursor concepts* (architect mode → the plan/edit
   idea) but joined none of the modern agentic conventions.

6. **The genuinely non-Anthropic firsts are worth remembering:** **ACP → Zed
   (via Gemini CLI)**, **code-mode → codex** (with Cloudflare naming), **worktrees
   → opencode**, and **hooks** a near-tie opencode/Claude Code. The ecosystem
   isn't *only* Anthropic-led — but Anthropic's *vocabulary* wins even where its
   code doesn't ship first.

---

## Confidence & method notes

- **High-confidence dates** (primary, dated): subagents, MCP, skills, hooks,
  worktrees, code-mode originators; the git/CHANGELOG/npm-grounded cluster dates.
- **Bounded/upper-bound** (feature present *by* this date; true intro earlier):
  grok-build (2026-07-16 mirror), warp (2026-04-28 open-sourcing), kimi
  (2026-05-22 inception) — mirror/shallow clones hide earlier history.
- **Low-confidence / unresolved** (web budget exhausted): Cursor's first-dates
  for MCP/plan-mode/subagents/hooks; Copilot CLI's ACP/hooks/skills ship dates;
  Kiro's MCP/hooks launch dates; **Antigravity** (JS-rendered pages returned no
  content — omitted rather than guessed).
- **"Originator" = first-in-cohort**, not inventor: MCP is Anthropic's *protocol*
  (2024-11); ACP is Zed's; code-mode's pattern is CodeAct/Anthropic-API/Cloudflare.

*Method: 8 parallel `general-purpose` tracer agents over `~/src-ext/` clones +
`gh` + WebFetch of canonical changelogs, 2026-07-18/19. Every date carries a
cited source in the run journal; absent-feature and unknown-date were recorded
honestly rather than guessed.*
