# Harness notes: what each CLI does with a council member's MCP server

*2026-11-03, spike-walking-skeleton. Every row says how it is known. **run** = a run record under `runs/` (named); **binary** = read from the CLI's own binary strings or its error text; **docs** = Claude Code docs as relayed by the coordinator's docs-reading agent, not tested here. Versions: Claude Code 2.1.317, codex-cli 0.163.0, grok 1.0.70, agy 1.4.12, gemini-cli 0.50.0.*

## Wiring a member with no global configuration

| CLI | how the council server is attached, per run | how MCP calls are approved, per run | other tools left | evidence |
|---|---|---|---|---|
| `claude -p` | `--mcp-config JSON --strict-mcp-config`, `--setting-sources ""`, `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` | `--allowedTools mcp__council__…` (four names) | none (`--tools ""`) | run: `trial-1`, all probes `p1`–`p6` |
| `codex exec` | isolated `CODEX_HOME` (auth symlinked), `-c mcp_servers.council.*` | **new in 0.163:** without approval, every MCP call fails with *"MCP tool call requires approval, but approval policy is never"*. Fix: `-c mcp_servers.council.default_tools_approval_mode="approve"` (variants `auto`, `prompt`, `writes`, `approve`, from codex's own config error) | shell/exec disabled with `--disable …`; read-only sandbox | run: `probes/c1-codex-echo` (failed first, then passed) |
| `grok -p` | isolated `HOME` whose `config.toml` declares only this server | `--allow 'mcp__council__*'` | `search_tool`/`use_tool` (grok's only route to MCP tools) | run: `probes/g1-grok-echo` (grok-4.6 by default) |
| `agy -p` | workspace plugin `.agents/plugins/council/` (`plugin.json`, `mcp_config.json`) | **new in 1.4.12:** `--settings '{"permissions":{"allow":["mcp(council_council/*)"]}}'` — "settings for this session only, never saved". utf-mcp found 1.2.17 blocked without editing `~/.gemini/antigravity-cli/settings.json`; this flag removes that blocker. | utf-mcp's `agy-hook --allow-server council_council` denies every other tool and logs each decision | run: `probes/a1-agy-echo` (gemini-3.8-flash-low) |
| `gemini -p` | project `.gemini/settings.json` in the run cwd | `"trust": true` per server | — | run: `probes/m1-gemini-echo`: **dead for this account** — *"IneligibleTierError: This client is no longer supported for Gemini Code Assist for individuals. … migrate to the Antigravity suite"*. agy is the Google route. |

Model names have moved since utf-mcp's notes (2026-10-05): codex refuses `gpt-5.5` on a ChatGPT account (*"model is not supported when using Codex with a ChatGPT account"*); the cache lists `gpt-reserve, gpt-6-luna, gpt-5.6-terra, gpt-5.6-luna, codex-auto-review`. agy's list is now `gemini-3.8-flash-{high,medium,low}`, `claude-sonnet-4-6`, `claude-opus-4-6-thinking`, `gpt-oss-120b-medium`; `gemini-3.6-flash-low` is refused.

## What reaches the server about the caller (identity)

| CLI | `initialize` clientInfo | per-call `_meta` | caller identity reaches the server? | evidence |
|---|---|---|---|---|
| Claude Code | `claude-code` 2.1.317, protocol 2025-11-25, capabilities `roots`, `elicitation {form, url}` | `claudecode/toolUseId`, `progressToken` (an integer equal to the JSON-RPC id) | **no**: nothing names the session or agent | run: `probes/p1-echo` |
| codex | `codex-mcp-client` 0.163.0, capabilities `elicitation`, `experimental.codex/auth-change` | `callId`, `threadId`, `sessionId`, `windowId`, `itemId`, `progressToken`, and `x-codex-turn-metadata` carrying session/thread/turn ids, **model**, reasoning effort, sandbox mode, codex version, and **the workspace's git remote URL, HEAD commit and dirty flag** | **yes**: session, thread and model on every call | run: `probes/c1-codex-echo` |
| grok | `grok-shell-probe` 1.0.70, capabilities `elicitation` (with schemaValidation), MCP-apps UI extension | `progressToken` only | **no** | run: `probes/g1-grok-echo` |
| agy | `antigravity-client` v1.0.0, capabilities `elicitation`, `roots` | `antigravity.google/conversation_id`, `…/root_conversation_id`, `…/artifacts_dir`, `progressToken` (string) | **yes**: conversation, and the root conversation (so a sub-conversation is distinguishable from its root) | run: `probes/a1-agy-echo` |

So the coordinator's docs fact ("no per-session identity reaches the server") holds for Claude Code and grok and fails for codex and agy. From codex and agy a server can *record what the caller asserts* about its identity, taken from `_meta`. These are strings the client sends, so any client can send the same ones (the second verifier showed two connections presenting one codex `sessionId` being treated as one holder, `de-novo-feedback-2` §2.5). For Claude Code and grok a server knows only the seat it was launched for, or a token it was handed. The skeleton now keys seat exclusivity to the asserted principal *plus* a holder key the client never sees.

Privacy note: codex sends the workspace's remote URL and commit to every MCP server it calls. It walks up to the enclosing git repository: a member whose cwd was under `runs/` reported `/Users/josephwecker-v2/src/arch`, its `origin` URL, HEAD and dirty flag (second verifier, `trial-4c`). Harmless for a local council server; worth knowing before a council server is ever remote.

## How members wait and wake (blocking calls)

### Claude Code (`-p`), run: `runs/probes-2026-11-03/p2`…`p6`

| case | result |
|---|---|
| 150 s call, defaults (`p2`) | completed normally at 150 s. **Not backgrounded in `-p` mode.** The docs' 2-minute auto-background did not occur in print mode. |
| 150 s call, `CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS=0` (`p3`) | same: completed at 150 s |
| idle timeout set to 20 s, no progress (`p4`) | aborted at ~32 s with the text *"MCP server "probe" tool "probe_sleep" sent no response or progress for 30s; aborting. If this server is configured in your MCP settings, set a per-server "timeout" (ms) … otherwise set CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT (ms) globally (0 disables)."* The message says 30 s though 20000 was set: a **30 s floor** is my reading (inferred, one run). **No `notifications/cancelled` reached the server**: it kept "sleeping" until the process was killed. |
| same, progress every 5 s (`p5`) | **completed at 60 s**: progress notifications reset the idle timeout |
| hard timeout `MCP_TOOL_TIMEOUT=30000`, progress every 5 s (`p6`) | aborted at 30 s, *"timed out after 30s"*; the server **did** receive `notifications/cancelled` (*"SdkError: Request timed out"*). Progress does not extend the hard limit (matches the binary's own help text). |

Consequence for council design (derived): **a result the server sends is not a result the member saw.** After an idle-timeout abort the server is never told, and would hand a "your turn" view into the void. Delivery can only be acknowledged by the member's next act (a submit, or a later status/wait). The skeleton therefore never treats "wait returned" as "member informed"; a round waits on submissions.

The per-server `"timeout"` field in an MCP server entry exists (binary help text: *"Per-server tool-call timeout in milliseconds. Overrides the MCP_TOOL_TIMEOUT environment variable for this server. Hard wall-clock limit per call; progress notifications do not extend it."*), so a council member config can set its own limits without touching global settings.

### Claude Code `--bg` (background) session, run: `runs/probes-2026-11-03/b1-bg-sleep170` (`session-extract.jsonl` is the session's own transcript, trimmed)

One 170 s blocking call, no progress, haiku, cwd outside any git repo (so `--bg` made no worktree):

| t | what happened |
|---|---|
| 2.0 s | the model calls `probe_sleep` |
| 122.0 s | the call is **moved to the background**; the model receives, as the tool result: *"MCP tool "probe/probe_sleep" is still running after 120s. It was moved to the background as task kgbs5pyy9 and keeps running; you'll receive a notification with the result when it completes. You can keep working in the meantime. … Note: it does not survive exiting this session."* |
| 125–127 s | the model says it will wait and **ends its turn**; the session goes idle |
| 172.0 s | the server returns the result; a `<task-notification>` is queued into the session |
| 172.1–173.8 s | the **idle session wakes** on that notification and acts on the result |

So the docs' 2-minute auto-background is real in a main (here background) conversation, absent in `-p` (`p2`, `p3`). The second verifier showed it also happens with the real server sending progress every 10 s (`verify-2/runs/bg-wait-2-sonnet`). A notification arriving at an idle session wakes it. **But whether the session is idle when it arrives depends on the brief, not the harness.** This probe's prompt said "then stop". Under `members/brief.md` ("if a tool call … times out, call council_wait again") the verifier's sonnet member opened a second wait instead of going idle, so two waits were pending for one seat. The wake can work only if the member is told to end its turn after a backgrounding. `members/brief-bg.md` says so; it is untested, because both of my `--bg` runs failed earlier, at the startup race below. The backgrounded call "does not survive exiting this session".

**A `--bg` session does not get what a `-p` session gets from the same flags** (second verifier, one session each, from the harness's own `api-request-shape` records):
- *Prompt.* A `--bg` session gets Claude Code's full interactive system prompt, including a built-in "Claude in Chrome" section that tells the model to load deferred tools with `ToolSearch`. `--setting-sources ""` and `--strict-mcp-config` govern settings and MCP configuration, not built-in integrations. A haiku member, with `ToolSearch` removed by `--tools ""`, wrote out a fake `ToolSearch` call as text and went idle (`bg-wait-1-haiku`).
- *Tools.* The sonnet member's first API request carried 29 tools: the four council tools, `EndConversation`, `advisor`, `DeferredToolPlaceholder`, and **22 `mcp__claude-in-chrome__*` browser tools**. A `-p` member's `init` lists one server and four tools. `--no-chrome` ("Disable Claude in Chrome integration") exists and the launchers did not pass it. With it, the `--bg` system prompt no longer mentions Claude in Chrome or `ToolSearch` (`runs/trial-6-bg-attempt1`, `-attempt2`, `session-summary.json`).
- *Startup race.* A `--bg` session's first API request often carries **0 tools**, although the council server has already answered `tools/list`: about 200 ms earlier in all three cases measured (21:40:05.457 → .664 and 21:43:37.094 → .303 in my two runs; 0.2 s in the verifier's). It happened in 3 of 4 launches: the verifier's haiku (Chrome on) and both of mine (sonnet, `--no-chrome`). The exception was the verifier's sonnet with Chrome on (29 tools). After a tool-less first turn the model either said nothing (my attempt 1) or wrote a fake call as text (`<invoke name="council_wait">`; haiku's `ToolSearch`) and ended its turn. Nothing woke it. Not explained; the binary has first-turn MCP wait logic (`CLAUDE_CODE_MCP_PREWAIT_SERVERS`), which I did not work out. **Not seen in `-p`:** all 27 `claude -p` sessions in the runs (mine and both verifiers') listed their tools in `init`, and the first thing each did was a real tool call. The session ran in permission mode `auto` (the transcript's `permission-mode` record) although `--setting-sources ""` was passed; the MCP tool was pre-allowed, so this didn't matter here. The server's parent process was `claude bg-spare --bg-spare …/cc-daemon-505/…claim.sock`: background sessions run in pre-spawned daemon "spare" processes, which is what the skeleton's `parent-process` principal sees for them.

### codex, grok, agy (runs: `runs/probes-2026-11-03/c2,c3,g2,a2,a3`, one 150 s blocking call each)

| CLI | case | result |
|---|---|---|
| codex 0.163 (gpt-5.6-luna) | defaults, progress every 5 s (`c2`) | **failed at 60 s**: *"tool call error: tool call failed for `probe/probe_sleep` … timed out awaiting tools/call after 60s"*. Progress does **not** extend codex's limit. No `notifications/cancelled` reached the server (its log simply stops). |
| codex | `-c mcp_servers.probe.tool_timeout_sec=600`, progress every 5 s (`c3`) | completed at 150 s |
| grok 1.0.70 (grok-4.6) | progress every 5 s (`g2`) | completed at 150 s |
| agy 1.4.12 (gemini-3.8-flash-low) | progress every 5 s (`a2`) | completed at 150 s |
| agy | no progress (`a3`) | completed at 150 s |

So the one universal waiting mechanism, a blocking `council_wait`, works in all four harnesses, with two per-harness adjustments, both per-run: codex needs `tool_timeout_sec` raised in its own `-c` config (or the server must cap `council_wait` under 60 s), and Claude Code needs progress notifications under ~30 s apart if anyone lowers its idle timeout (its default idle timeout is 30 min on stdio, per docs). The server should always send progress; it costs nothing and is what keeps Claude Code's idle timer alive.

How each harness ends a member's server, from servers that log signals (the verifier's `verify-1/runs/mixed-rerun`, and `runs/trial-4c-mixed` receipts): Claude Code sends SIGINT; codex SIGTERM; agy closes stdin (as utf-mcp observed); grok sends nothing a server can catch (the process is simply gone). So a grok seat never releases and only lapses after the lease, and a server cannot count on an orderly exit from any harness. Liveness has to be a lease, not a goodbye.

Quota is a crash like any other. In `trial-4c` grok (now grok-4.7) stopped mid-meeting: *"You've reached your free Grok Build usage limit for now"*, exit 1, about 66 s in. Its seat went gone one lease later, and the meeting finished without it.
