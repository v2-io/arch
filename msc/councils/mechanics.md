# Agentic councils — mechanics, draft 0

> [!note] **Superseded (2026-11-03, the same day).** This was the coordinator's design before any spike ran. The spikes and their verifications corrected much of it, and Joseph's minimal loop replaced its procedure. Read `generic-collab-council.md` for the procedure, `councils-system-model.md` for how councils fit the system, and the walking-skeleton spike for the substrate. The spikes cite this file, so it stays as history. Nothing in it is current design.

*2026-11-03. Claude (Opus 5.5). This is the mechanical design for the first working council tool. Joseph's direction (2026-11-03): put the mechanisms in place. He will choose members with "just enough diversity to push the system". Member disposition, independence, herding and "sufficient wisdom" are out of scope for now. Members join through MCP so that non-Anthropic members can be added later. Background and leans are in `initial-thoughts.md`. This file is where those leans become choices. Each choice is marked as mine, and Joseph or the build can overturn any of them.*

## What it is

A council is a committee: a few members, a charter, an agenda of questions, and a parent who receives its report and may veto it. Joseph is the parent, and he acts outside the tool. Members are agents, each connected through an MCP server.

The tool has four parts:
- **Store:** one append-only event log per council.
- **Core:** pure functions over the charter and the log.
  - state = fold(charter, events)
  - what each member may do now
  - how a closed round resolves into new events
- **Adapters:** an MCP server for members, a CLI for the steward and for debugging, and later an HTTP transport and a driver that launches members itself.
- **Report:** a projection of the log. It is what goes to the parent.

The core never reads a clock or the order in which messages arrive, and it never needs a mind. Everything that needs a mind arrives as a member's submitted intent.

## The two criteria

1. **Available actions.** At every state, each member has a computable list of what it may submit, each entry with a reason. This is the RONR model's `available_actions`, narrowed to the charter.
2. **Latency invariance.** The log is the same under any permutation of the order in which submissions arrive within a round. It is tested by property: replay with every submission order and compare the logs.

The one place wall-clock time enters is a round's deadline. Who submitted before the deadline is a fact about time, and it is recorded as that fact: a member who did not submit is `absent` for that round, which is not the same as passing. Everything after that is determined by which members submitted and what they submitted.

## The charter

The charter is the council's machine-executable constitution. It is fixed when the council convenes, and changed only by recorded events: co-opting a member, an amendment the grant allows. Its fields:

- **seats:** id, declared vantage (what the seat is *for*), model and harness (recorded, not enforced), and a credential.
- **grant:** what the council may do with effect (spawn a research subcommittee, co-opt a member within stated bounds) and what it may not. Its output is a report to the parent.
- **quorum:** a number of seats.
- **actions:** a whitelist, in the sense of `model/council.json`'s `restrict_actions`. Ids reuse `model/actions.json` and `model/vocabulary.json` where they mean the same thing; agentic additions are named as such.
- **thresholds per question kind:** from the model's threshold vocabulary (`majority`, `two-thirds`, `unanimous-consent`, …).
- **round policy:** deadline, and what a missing submission means.
- **debate policy:** `parallel` in v0 (below).
- **agenda:** the questions, in order, each with what a decision on it must produce.

*My choice:* YAML for v0, because it is quick to write. The surface can move to udon later, the way the RONR model's surface may. The core reads a parsed structure, so the surface is replaceable.

## Rounds

A round is the unit of contention. Every step of a meeting is one or more rounds.

1. **Open.** The core opens a round addressed to a set of seats (all seats, or a subset), with a kind (below) and the actions each addressed seat may submit. Opening is an event.
2. **Submit.** Each addressed seat submits one sealed intent. Passing is an intent. A seat may replace its own intent until the round closes. Only its last submission counts, and the replacement history is kept. Replacement is ordered within one seat, never across seats, so latency invariance holds. Nobody sees another seat's intent before close.
3. **Close.** A round closes when every addressed seat present has submitted, or when its deadline has passed. Closing is lazy: whichever call first finds the condition true closes the round. Since resolution depends only on the submitted set, it doesn't matter which process closed it.
4. **Resolve.** Resolution is a pure function of state and the set of intents, in canonical order (by seat id). It emits events.
5. **Reveal.** Every submitted intent is in the round-closed event, including those that did not win.

**Round kinds.** v0 has six; the build will likely find more.
- **open:** "anything to bring?" This is where motions, points, questions and passes arrive.
- **debate:** each seat may submit one statement on the pending question, or pass.
- **consent:** consent, or object *with a ground*. A bare objection is not accepted.
- **vote:** yes, no, or abstain, with an optional explanation that is revealed with the tally.
- **targeted:** addressed to a subset, for one question. Examples: a re-affirmation, a merge question to two makers, a member's question to another member.
- **ruling:** a judgment question, put to the assembly. v0 has no presiding mind; see *Chair*.

**Action classes.** These decide how resolution treats intents.

| class | v0 members | resolution |
|---|---|---|
| preemptive | point of order, question to the assembly about procedure | handled before anything else in the round; more than one is handled in canonical order, each opening its own ruling round |
| exclusive | making a motion, amending | one wins; the rest become standing intents (below) |
| first-suffices | second, join | merged into one event listing every seat that did it |
| additive | votes, statements | all recorded |
| pass | pass | recorded; counts toward closure rules |

**Which exclusive intent wins.** *My choice* for v0, as a ladder:
1. A standing intent carried over from an earlier round outranks a new one.
2. Then the seat least recently given the floor.
3. Then a seeded order. The seed is the hash of the round id together with the digest of the submitted set, so replays reproduce it.

Arrival order never appears in this ladder.

**What happens to the intents that didn't win.** They become *standing*. The seat keeps its priority. When its turn comes, it gets a targeted round to re-affirm, revise or drop the intent against the new state. Nothing runs that its author didn't re-decide.

**Merging.**
- Byte-identical motion texts in one round merge automatically into one motion with co-makers. That also counts as seconded.
- Otherwise, every revealed motion is visible, and in the next round any seat may submit `second` or `join` for another seat's motion, or `withdraw-in-favor-of` it.
- A seat that thinks two motions are effectively the same can submit `ask` to the two makers. This opens a targeted round to them, and their answers are revealed.

## A question's path

The default path, in the spirit of truth over interest (Joseph, 2026-11-03):

```
moved (and seconded, or co-made)
  → consent round: consent / object-with-ground
      all consent → adopted (unanimous-consent)
      any grounded objection → debate
  → debate rounds (parallel): statements revealed together
      ends when a full round passes, or the charter's limit on debate rounds is reached
      amendments arrive as exclusive intents in open rounds between debate rounds
  → vote round (sealed), tallied by the charter's threshold
  → adopted / rejected, with every statement and every objection's ground kept for the report
```

Points of order and questions can preempt at any round boundary. A seat with nothing to say passes, and a round in which every seat passes moves the question on. That is how debate ends without a "previous question" motion, which the committee configuration disables [50:25].

## Chair

The engine performs the chair's *duties*: stating, putting, announcing, opening rounds. It does those exactly when the state calls for them.

The chair's *rulings* are judgments: is this in order, is it germane, are these two motions the same. In v0 they go to the assembly as a ruling round, the path RONR gives a chair in doubt [23:18-21], decided by majority.

*My choice:* no presiding seat in v0. It removes one role and one failure mode, and loses nothing a three- to seven-member committee needs. A presiding seat with appealable rulings can be added later as a charter option.

## Effects

An adopted motion can carry an effect the grant allows. The core emits an `effect-requested` event, an adapter carries it out, and its result comes back as an event (`effect-completed`, with the outcome).

v0 effects:
- **record:** the adopted text goes into the report.
- **refer-for-information:** a research task with an adopted brief. The question stays pending. The council may keep debating, or postpone the question until the report event arrives, which is the event-anchored postponement from `initial-thoughts.md`. In v0 the steward, or whoever runs the meeting, launches the task by hand and submits its report through the CLI. A driver can automate this later.
- **co-opt:** add a seat with a stated vantage, within the grant's bounds. The steward issues the credential, and quorum is recomputed.

## Identity

Each seat has a credential, given to that member's MCP server at launch. Every event records the seat.

Whatever the harness reveals about the caller is recorded beside the seat as an attestation, never as the identity: the MCP client's name and version, Claude Code's `_meta["claudecode/toolUseId"]`, codex's `callId`. `firmatum/utils/utf-mcp` already logs exactly these across Claude Code, codex, grok and agy. Its README is the reference for what each harness sends.

## Store

`<council dir>/charter.yaml` plus `events.jsonl`. Each event carries:
- its sequence number;
- the hash of the previous event, so the log is a chain and an edit shows;
- its type, its round, and its seat or `engine`;
- its body;
- for submissions, the wall-clock receipt time, recorded for the record only and never read by the core.

Appends take a file lock, so several member servers (one stdio process per member session) can share one council directory without a daemon. The same choice vsect made for claims, which live under git's common directory and need no server.

## The member's tools (MCP)

- `council_status`: where the meeting stands; any open round addressed to me and what I may submit in it, with reasons; and the events since my last call, so a member reads the change, not the whole log.
- `council_submit {round, intent}`: sealed; replaceable until close.
- `council_wait {after}`: blocks until something is addressed to me or a round I'm in is revealed, sending progress notifications while it waits. See *Open* for how each harness treats a long call.
- `council_record {from?, to?}`: the log or the minutes, for a member arriving late or wanting the full text.

Steward CLI: `new`, `seat`, `convene`, `status`, `log`, `report`, `close-round --deadline-passed`, `effect-complete`.

## Report

The report is the projection the parent reads. It contains:
- each question and its outcome;
- the text as adopted;
- the vote by seat;
- every grounded objection and every debate statement;
- standing intents that never came up;
- effects and their results;
- the membership, with each seat's vantage, model and harness, as Joseph asked.

Later the report can feed DECISIONS ledgers (`deciders: council` under a grant) and vsect, but not in v0.

## v0 scope, and what is left out on purpose

**In v0:**
- one council at a time per directory;
- the six round kinds;
- the question path above;
- `refer-for-information` and `co-opt` as effects with a manual adapter;
- the stdio MCP server;
- the steward CLI;
- the latency-invariance property test;
- a replay test that runs a scripted meeting.

**Left out:**
- the RONR model's expression evaluator. v0 hand-implements its small action set. The evaluator is the RONR model's own stage 5, and the charter's action ids are chosen so it can replace the hand code later.
- floor-mode debate, subsidiary motions beyond amend and refer, reconsideration, and a presiding seat;
- an HTTP transport;
- a driver;
- links to DECISIONS and vsect.

## Open

- **How a member waits.** Claude Code facts, from its docs as checked 2026-11-03 by a docs-reading agent. Docs pages: `mcp.md`, `channels.md`, `channels-reference.md`, `sub-agents.md` at code.claude.com/docs/en/. These were not tested here, and the version numbers are as the docs give them.
  - **Baseline, which should work in any MCP client:** `council_wait` is a blocking call that sends progress notifications every few minutes.
    - Claude Code's idle timeout aborts a silent call after 30 min on stdio and 5 min on HTTP. Progress resets it.
    - The hard cap is about 28 hours by default (`MCP_TOOL_TIMEOUT`), and progress does not extend it.
    - In a main conversation, a call still running after 2 min is moved to the background. The session keeps working, and the result arrives later as a task notification. A council can use that: a member isn't stuck while it waits. Subagent calls are never backgrounded.
  - **Optional accelerators, both Claude Code only, behind an adapter:**
    - MCP tasks (protocol revision 2026-07-28): "If Claude is idle when the result arrives, Claude starts on it unprompted." Subagents can't use them.
    - Channels: a research preview that needs `--channels` plus a development flag or an allowlist, interactive sessions only.
  - **Not yet tested:** what the model sees when a timeout fires. For the other harnesses, utf-mcp's integration notes are the starting point.
- **Seat credential when connections are shared.** Subagents that reference a parent's MCP server by name share the parent's connection, and Claude Code passes no caller identity to MCP servers. So the seat credential can't depend only on the server process's environment.
  - *My choice:* the server's environment sets a default seat, and every tool accepts an optional `seat_token` argument that overrides it.
  - That covers one session hosting a seat and, later, several subagents each holding their own seat through one connection.
  - A subagent defined with an inline `mcpServers` entry gets its own connection, so it can use the environment default.
- **Name and home.** *My lean:* `concilium`, at `firmatum/utils/concilium/`, beside vsect and utf-mcp. It is Latin for council, which the estate's naming criteria favor over a coinage.
- **Language.** *My lean:* Rust, like vsect, aspectus and utf-mcp. utf-mcp is a hand-written stdio MCP loop with only `serde_json`, which shows that's enough.
