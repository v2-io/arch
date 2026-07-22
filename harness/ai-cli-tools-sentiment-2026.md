# AI Coding CLI Tools — End-User Sentiment & Migration Flows (mid-2026 snapshot)

*A qualitative read of how developers who have actually used **more than one** of
these tools feel about the **harness** — the tool/wrapper — as distinct from the
**model** behind it. This is opinion signal, not capability fact (capability
lives in the verified reference). It is a **snapshot that decays fast** — all
evidence clusters **April–June 2026**, referencing Opus 4.7/4.8-era pricing; a
single pricing or limit change could invert the headline within weeks.*

**Overall confidence: MODERATE.** Read the biases below *before* the findings —
they cap how hard you can lean on any of this.

---

## Biases & limitations (up front, on purpose)

1. **Genre skew.** Nearly all evidence is **single-author comparison / "I
   switched" blog posts** (composio, danilchenko, inetanel, jock.pl, sanj.dev,
   unicodeveloper, firecrawl) plus **one** 500+-dev Reddit aggregation and **one**
   developer survey. This *is* the high-signal switch-narrative genre the study
   targeted — but it is **not** high-upvote raw forum sentiment at volume. Don't
   read it as aggregated community opinion.
2. **Vendor-venue bias.** composio.dev (a devtools company) supplied ~4 of the
   load-bearing claims. It frames fairly (calls OpenCode a tie, concedes Opus is
   the better model) — but weight it as an interested party.
3. **The confound is only partly separable.** The cleanest model-controlled seam
   — *the same Claude model in Claude Code vs OpenCode* — is **imperfect**,
   because Anthropic RL-tunes Claude to Claude Code's **native tool schemas**. So
   "same model, different harness" still slightly favors Claude Code for
   model-side reasons, not pure harness quality. Every "CC's loop is smoother"
   claim carries this asterisk.
4. **The top exit driver is economics, not orchestration.** Claude Code's
   dominant outflow cause is **usage limits / cost** — partly an Anthropic
   *pricing/plan* attribute, not harness UX. It's a clean **non-model-quality**
   driver (switchers *prefer* CC's model and leave anyway), which is exactly the
   isolated signal we wanted — but "harness-level" is a loose label for it.
5. **Tribe skew unaddressed.** No native r/cursor or r/ClaudeAI thread sampling
   surfaced, so per-community over-indexing couldn't be discounted directly.
6. **Volume-thin directionality.** The migration flows are *directionally*
   credible but drawn from a handful of blogs + one aggregation — not a broad
   multi-venue corpus.

---

## Headline

- **The harness genuinely differentiates — independent of the model.** Holding
  the model constant, the wrapper contributes materially (~10–20 benchmark points
  in cited same-model tests, and different real-world outcomes). The premise that
  "tool sentiment ≠ model sentiment" holds. **(confidence: high)**
- **The flow is convergent, as hypothesized** — *many different origins → a few
  destinations.* Net **sinks: Codex CLI and OpenCode.** Net **source: Cursor
  Agent.** **Claude Code is both** — it *absorbs* Cursor leavers but *sheds* on
  cost. **(confidence: medium)**
- **The signature case:** users leave Claude Code **despite preferring its model
  quality** — a textbook harness/economics-over-model revealed preference.
  **(confidence: high)**

---

## Migration / directionality map (the primary metric)

Directed switches observed, with dates and the *kind* of reason:

| From → To | Driver (harness / economics / model) | Evidence & date |
|---|---|---|
| **Claude Code → Codex CLI** *(dominant)* | Economics (usage limits) + harness (delegate-review, session reliability); model reason ("code worse") is confounded | 500-Reddit-dev aggregation; browser-use founder Gregor Zunic's CC→Codex return (May 31 2026); totalum "what drove most of the 2026 migration" |
| **Claude Code → OpenCode** *(secondary, routine tasks)* | Economics (BYOK cheapest) + harness (TUI, plan/build) | inetanel CTO piece: "Start new projects on Claude Code… 4–8 weeks in, migrate routine tasks to OpenCode… I do this on every project now" |
| **Cursor Agent → Claude Code + Codex** | Harness/consolidation | composio (Jun 4 2026): "I was a huge Cursor Agent fan; unfortunately, my usage has declined to 0" |

**Net direction:**

| Tool | Role | Inflow diversity | Note |
|---|---|---|---|
| **Codex CLI** | net **SINK** | receives from Claude Code (and Cursor via CC) | No dominant reverse Codex→CC flow was found (only "keep both installed" nuance) |
| **OpenCode** | net **SINK** | receives from Claude Code | Convergent destination for cost-sensitive/routine work |
| **Claude Code** | **both** | absorbs Cursor leavers | Sheds on cost/limits despite model preference |
| **Cursor Agent** | net **SOURCE** | — | People leave; little inflow in this corpus |

*Convergence caveat:* the "former is variable, latter convergent" pattern is
**present but volume-thin** — it rests on a handful of comparison blogs plus one
Reddit aggregation, not a broad corpus. Directionally credible; not
quantitatively firm.

---

## Per-tool sentiment (harness-attributable only)

### Claude Code — *most-deployed, most-admired-on-ergonomics, leaks on cost* · confidence: HIGH
- **Praised (harness):** uniquely built for **unattended / overnight autonomous
  runs** ("the only tool built, end to end, for *'can I leave this running while
  I sleep?'*" — jock.pl, Apr 15 2026); tightest **plan-edit-test loop** (in a
  same-model refactor it one-shot where OpenCode "stumbled on the test-fix loop,
  needing manual intervention twice" — danilchenko, Jun 9 2026); mature plugin
  ecosystem; **editor** presence, not just terminal; frontier models day one.
- **Complained (harness/economics):** **cost & usage-limit friction is the
  #1 non-model exit driver** — *"I used it 8 hours a day. Kept hitting usage
  limits so I bought two $200/month accounts. Canceled both immediately"*
  (500-dev aggregation). Plus **model-lock** to Claude.
- **Adoption anchor:** among developers running multiple agents daily, Claude
  Code is the most-used harness at **~70%** (Stack Overflow survey, ~1,100 pros,
  May 27 2026) — *deployment*, not sentiment.
- ⚠️ *Confound:* the "smoother loop" partly reflects Anthropic tuning Claude to
  CC's own tool schemas (see bias #3).

### Codex CLI — *net sink, "actually usable"* · confidence: HIGH
- **Praised (harness):** **usable throughput with no rate-gating on cheap plans**
  ("I've never hit my $20 plan limit"), clean **delegate-and-review** workflow,
  Codex Cloud execution, `/review`, cross-session reliability.
- **The tell:** *"Opus is the better model when it's having a good day. I just
  can't make it have a good day on demand"* (composio, Jun 4 2026) — prefers the
  harness while **conceding the rival's model is better**. In a 500-dev
  aggregation, **67% blind-quality win for Claude Code yet 65.3% prefer Codex.**
- **Confound note:** part of the switch reason ("code worse") is model-side; the
  usability/limits reason is clean harness/economics signal.

### OpenCode — *net sink, the TUI darling* · confidence: HIGH
- **Praised (harness):** **best-in-class TUI** ("best-looking, best-engineered
  terminal in the category," 60fps+ OpenTUI/Zig — composio, Jun 11 2026);
  **UI-enforced read-only Plan mode** (Tab to Build); **no IDE lock-in**;
  **provider-agnostic BYOK** (runs Claude on your own key); **lowest effective
  cost**.
- **Complained (harness):** more **manual setup / overhead** than Claude Code or
  Codex ("overhead that Claude Code and Codex eliminate by design").
- **The model-controlled recommendation:** several authors land on **"OpenCode +
  Claude BYOK"** as best cost/performance for most developers — *"You get Claude
  Code-quality output at $10–80/month"* (unicodeveloper, May 22 2026) — reserving
  Claude Code for architecture/automation.

### Cursor Agent — *net source, thin signal* · confidence: LOW
- Comparative signal on Cursor's **CLI/agent** is genuinely sparse; what exists
  leans **negative** (at least one former "huge fan" → zero usage). Notably, the
  two strongest *pro*-Cursor items **failed verification** (a 77%-vs-93%
  Terminal-Bench "harness credit" framing, and a CC+Cursor two-tool-stack rec) —
  so this is reported as **low-signal, not padded**.

### aider · GitHub Copilot CLI · Grok Build · Kiro CLI · Warp — *no comparative signal*
- The corpus returned **essentially zero cross-tool comparative sentiment** for
  these in-scope tools. Either genuinely low-discourse or under-sampled — flagged
  honestly rather than invented. (This is itself a finding: the multi-tool
  conversation in mid-2026 is overwhelmingly **Claude Code ↔ Codex ↔ OpenCode**,
  with Cursor as the fading incumbent.)

---

## Cross-tool synthesis — with the model controlled for

Among multi-tool users, **holding the model constant**:
- **Wins on ergonomics/reliability:** **Claude Code** (loop, autonomy) and
  **OpenCode** (TUI, control, cost) — for *different* reasons.
- **Wins on usable-throughput-per-dollar:** **Codex CLI** and **OpenCode**.
- **"Tolerated for its model" rather than loved as a harness:** **Claude Code**
  is the clearest case — people rate its *model* highest yet leave the *harness*
  over cost. That gap **is** the answer to "harness vs model enthusiasm": CC's
  enthusiasm is disproportionately model-driven; Codex's and OpenCode's is
  disproportionately harness/economics-driven.

---

## Open questions (what would sharpen this)

- **Reverse-flow magnitude unknown.** Are the CC→Codex/OpenCode moves durable, or
  do people drift back? Only "keep both installed" nuance surfaced.
- **Individual vs team.** Every switch narrative is an N=1 individual; the
  usage-limit driver may not bind where orgs pay per-seat.
- **The missing five.** aider, Copilot CLI, Grok Build, Kiro, Warp are absent
  from the flow topology — genuinely low-discourse, or under-sampled?
- **Pure orchestration, model-neutral.** No source ran a *non-Claude* (e.g.
  GLM/Qwen) harness-vs-harness test — the one design that would strip out
  Anthropic's native-schema tuning advantage and isolate pure harness quality.

---

## Sources

The corpus this synthesis actually drew on — the evidence, not a curated
highlight reel. As the biases section says, nearly all are single-author
comparison/switch posts (plus one survey), which is why confidence caps at
moderate.

*Rigor note (honest about the difference vs. the capability reference): these
were gathered in the 2026-07-18 research pass and each ran through 3-vote
adversarial verification during that run (19 of 25 claims survived, 6 refuted).
But unlike the capability reference's ledger, the individual sentiment quotes
were **not** re-fetched live afterward — the inline source + post-date on each
quote is your re-check pointer. Quotes are reproduced as the run captured them.*

- Stack Overflow "Agents on a leash" survey — May 27 2026 (the one primary dev survey): https://stackoverflow.blog/2026/05/27/agents-on-a-leash-agentic-ai-remains-mostly-monitored-at-work/
- "Claude Code vs Codex — what 500 Reddit developers really think": https://dev.to/_46ea277e677b888e0cd13/claude-code-vs-codex-2026-what-500-reddit-developers-really-think-31pb
- "I tested Codex, OpenCode, Claude Code and Cursor together" — Jun 2 2026: https://dev.to/jovan_chan_9500711396d4e6/i-tested-codex-opencode-claude-code-and-cursor-together-in-2026-a-practical-multi-tool-1493
- composio.dev — CC vs OpenCode (Jun 11 2026): https://composio.dev/content/claude-code-vs-open-code · CC vs Codex (Jun 4 2026): https://composio.dev/content/claude-code-vs-openai-codex
- danilchenko.dev — same-model CC vs OpenCode test (Jun 9 2026): https://www.danilchenko.dev/posts/opencode-vs-claude-code/
- inetanel.com — CC vs OpenCode, a CTO decision: https://inetanel.com/articles/claude-code-vs-opencode-cto-decision
- thoughts.jock.pl — AI coding harness agents (Apr 15 2026): https://thoughts.jock.pl/p/ai-coding-harness-agents-2026
- unicodeveloper (Medium) — three-way comparison (May 22 2026): https://medium.com/@unicodeveloper/claude-code-vs-codex-vs-opencode-which-ai-coding-agent-is-actually-the-best-in-2026-baa9f6fd5374
- firecrawl.dev — best AI coding agents (browser-use founder switch narrative): https://www.firecrawl.dev/blog/best-ai-coding-agents
- sanj.dev — comparing AI CLI coding assistants: https://sanj.dev/post/comparing-ai-cli-coding-assistants/

*Method: fan-out search across the switch-narrative genre → claim extraction →
3-vote adversarial verification (19 confirmed / 6 refuted of 25). Harness-vs-model
tagging and migration-directionality treated as the primary axes. No numeric score
is assigned — the evidence base is qualitative and decays quickly.*
