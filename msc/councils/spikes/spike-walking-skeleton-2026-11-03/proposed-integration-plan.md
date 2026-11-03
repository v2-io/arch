# Proposed integration plan: what the walking skeleton suggests for the real server

*2026-11-03, spike-walking-skeleton. These are suggestions with reasons, for whoever builds the council server (Joseph's "server and channels/mcp adapters for a working council system"). Each cites the run record that motivates it. Where a suggestion is my inference rather than something a run showed, it says so. The council *logic* (contention, charter, agenda) is the sibling spikes' ground; I touch it only where a mechanic forced it.*

## 1. Keep: the shape that survived contact

1. **Pure core + any-process store, no daemon.** State = fold(charter, canonical log); one function turns a closed round's intents into derived events; any process holding the council directory can close a round whose condition holds, under an flock. Five model-member meetings across four harnesses closed every round exactly once, and every log replays to itself (`steward replay` on each `runs/trial-*/council`). vsect's choice (shared state under a common directory, no server) works here too. *Inference:* a daemon becomes worth it only when members are remote or the sealed store must be unreadable to members (§3.4).
2. **Sealed intents outside the log; only the close enters it, sorted by seat.** That is what makes the canonical log latency-invariant byte for byte: 50 shuffled arrival orders → 1 log (`steward selftest`). Wall-clock receipts go in a separate, non-canonical journal.
3. **Submissions are final within a round.** With replacement allowed, the same intents in shuffled orders gave 4 distinct logs: a revision races the "everyone has submitted" close. Either submissions are final (as now), or close waits for a deadline (and wall-clock order decides again). A member who wants to change course does it next round. If revision is ever wanted, the latency-invariant form is an explicit *two-phase* round (submit, then a separate "seal" act), and that costs a round-trip per round.
4. **Close = everyone submitted, or gone; the wall clock only as a long backstop.** Measured: once the last member answered, the round closed at once, so a round lasted as long as its slowest member (2–18 s after opening in `trial-4`). The last seat in was grok's in 5 rounds and agy's in 4, and nobody was penalized. A member whose process died stopped holding the round up one lease later (~20–30 s: `trial-1`, `trial-3`). Slow thinking is never absence, because presence is the *server process's* heartbeat, not the model's.
5. **One blocking `council_wait` that sends progress every ~10 s** is the universal member loop. It worked in Claude Code `-p`, codex, grok and agy (`harness-notes.md`).

## 2. Change: what running showed was wrong in the first design

1. **A returned result is not a delivered result.** Claude Code's idle-timeout abort and codex's 60 s timeout send the server no cancellation (`probes/p4`, `probes/c2`). So nothing in the server may treat "my wait call returned" as "the member saw it". Rounds wait on *submissions*. Any "member was informed" fact the record needs (e.g. for the charter-agenda spike's common-state condition) must come from the member's own act naming the round it answers, which `council_submit(round=…)` already requires.
2. **Seat identity: one live holder per seat, keyed to a principal, with an epoch.** In `trial-3` a fresh session holding an already-held seat lost every round only because it was slower: the other holder's submission completed the round, and the newcomer's arrived to "round is not open". It never learned another process held its seat, and had it been faster, a mind with no memory of rounds 1–7 would have voted. Now (tested by RPC, `runs/smoke/c2`; by model, `trial-3b`) a seat has one holder: a principal (a launcher-declared id such as a driver; else codex `sessionId` or agy `root_conversation_id` from `_meta`; else the harness process). A second principal is refused, and told why, while the holder's lease is live. A clean exit releases at once; a crash lapses after the lease. The epoch increments, and the minutes show who held each seat when.
3. **Every piece of per-member state belongs to the seat holder, persisted, never to the process.** The delivery cursor lived in process memory, so in driver mode every turn re-sent the whole log (rec's transcript: ~573k characters of user-side content, of which the driver's prompts were ~75k; the rest was tool results re-sending the log). The cursor fix cut cost per member-turn from $0.0207 (`trial-5`) to $0.0138 (`trial-5b`); driver mode is still ~6–8× a blocking client per round ($0.026 vs $0.0035–0.0045 in `trial-3`/`trial-3b`), and the remainder is not fully diagnosed: each resumed turn re-creates ~8.8k tokens of prompt cache, and per-call context runs larger than a client's. The cursor is now keyed to (seat, principal) on disk. A new holder starts from nothing and gets a full catch-up, which is right for a returning member.
4. **Never advertise an optional identity field to a model.** Haiku filled `seat_token` with its seat name and locked itself out (`trial-1`). Single-seat servers no longer show the field; multi-seat (shared-connection) servers require it. Even required, two subagents dropped it once on a long submit (`trial-2`); the server refused, and they retried. Errors must say exactly what happened (*"this connection already holds seat X: call again without seat_token"*).
5. **Presence counts only the holder.** A refused duplicate's heartbeat must not make a dead holder look alive (found while fixing 2).

## 3. Add: what the real server needs that the skeleton lacks

1. **Two member adapters, one round protocol** (both run here):
   - *client*: the member loops on `council_wait` (`trial-1`…`trial-4`). Right for headless `-p`/`exec` members whose whole job is the meeting. Cheapest: one session, one growing context (about $0.001 per member per round with haiku: trial-3b rec, $0.0148 over 13 rounds).
   - *driver*: the council resumes each member's own session for one turn per round it is addressed in (`claude -p --resume`; `trial-5`, `trial-5b`). Costs ~6–8× a client per round (measured, haiku). Right for members that must not sit blocked: prepared experts (first turn `--resume EXPERT --fork-session`, honoring "always fork, never resume an expert"), interactive or `--bg` sessions, and harnesses whose tool timeouts are short. The driver holds presence for the seats it drives and passes one principal to all its per-turn server processes. Other harnesses have the same hook: codex `exec resume`, agy `--conversation`. *Inference, untested here.*
   - *backgrounded wait* (Claude Code main/background conversations only; `probes/b1-bg-sleep170`, one run): a `council_wait` still running at 120 s becomes a background task. The member's session goes idle, and the server's answer arrives as a notification that wakes it. This seats an interactive or `--bg` member, a prepared expert's fork included, with no driver and at client cost. Untested in a full meeting. The harness says the backgrounded call does not survive the session exiting. *Suggestion:* the first thing to run against the real server is a meeting of `--bg` forks waiting this way.
   - channels / MCP tasks (Claude Code only) were not tested; the backgrounded wait already covers the waking need they were meant for.
2. **Per-harness launch recipes as data, not prose.** Each harness needed one non-obvious per-run setting, none of them global:
   - codex: `default_tools_approval_mode="approve"` and `tool_timeout_sec`;
   - agy: the `--settings` allow rule plus the deny hook;
   - grok: an isolated `HOME` and `--allow`;
   - Claude: `--strict-mcp-config --setting-sources ""`.

   `members/*-member.sh` are working recipes. The real server's "seat a member" command should own them, so Joseph adds a non-Anthropic member by naming harness + model, not by editing scripts.
3. **Attest caller identity wherever the harness gives it.** codex and agy put session/conversation ids in `_meta`; record them on every act (the skeleton does, in receipts). For Claude Code, `CC_AGENT` never reaches an MCP server (it is a Bash-prefix hook). The launcher can pass the session it started as the principal. *Suggestion:* the agent-identity hook could also export into MCP server env, but MCP servers start before the first Bash call, so it would have to be a different mechanism. Out of scope here.
4. **Sealing must be enforced by where the sealed store lives, not by etiquette.** Sealed intents sit in the council directory, readable by any member with file tools. The agy member reached for `run_command`/`read_file` unprompted (to read its own tool schema, `trial-4/fam/agy-hook.jsonl`; the hook denied both). Today sealing holds only because every launcher withholds file access (`--tools ""`, codex `--disable shell_tool`, grok `--tools search_tool,use_tool`, agy deny hook). For members that keep their file tools (prepared experts do), sealed intents must live where they can't read them: a daemon's memory, a directory outside their sandbox, or encrypted to the closer. That is the first concrete reason for a daemon.
5. **codex leaks the workspace's git remote and HEAD to every MCP server** (`x-codex-turn-metadata.workspaces`). Fine locally; know it before any council server is remote.

## 4. For the council-logic spikes (observed, not designed here)

Every meeting with model members hit these. They are offered as data:

1. **Members need `amend`.** Without it they routed around the gap three ways:
   - voting yes on text they had all superseded, with dissent notes attached to each vote (`trial-2`);
   - the maker withdrawing so the substitute could be moved (`trial-4`);
   - unanimously *rejecting* their own pending motion (0–3) so they could move the agreed text (`trial-5`).

   In each, the adopted text and the members' actual consensus diverged until the workaround landed.
2. **Near-duplicate motions are the common case, not the edge.** In `trial-4` all four seats moved the agreed substitute in one round. Two texts were byte-identical (co-made); two differed in wording and were recorded as rival standing intents, which then lapsed. The record says two members moved it and two were overruled; in fact all four moved the same thing. `initial-thoughts.md` §1 predicted this; it occurs at the first opportunity.
3. **The view must explain resolution.** Members asked how the winning motion was chosen ("The view did not say how that one was chosen", grok seat). The ladder is in the event (`ladder:`) but not in what the member reads.
4. **Standing intents need a "superseded by" path.** Members were asked to re-affirm intents the group had moved past (claude seat, trial-4).
5. **Debate ended by the round limit, not exhaustion, when one member kept making procedural notes** (`trial-2`). A full round of passes ended debate in every other case.

## 5. How the sibling spikes meet this one

- **charter-agenda's common-state condition** (*every act applied at s_k was formed by a member who held s_k*) is enforceable exactly as §2.1 says: by the act naming its round, never by the server believing it delivered. Its result "rounds → majority valid; polling → unanimity only" [51:2] applies to driver mode unchanged: driver mode is still rounds.
- **contention-rounds:** the arrival-order invariance property holds in the store as built; whatever resolver that spike settles on can be dropped into `core.step` and checked with `steward selftest`'s shuffle harness. The replacement-race result (§1.3) is a constraint on any resolver design that allows revision.

## 6. Suggested order for building the real server

*(My judgment about sequencing; not something a run showed.)*

1. Port `core` + `store` as they now stand (holders, persisted cursors, final submissions), with the selftest, replay and holder RPC tests as acceptance.
2. The client adapter and launch recipes for the four harnesses (§3.2).
3. The driver adapter, with an experts fork as the first real driven member.
4. Decide where sealed intents live (§3.4) before seating any member with file tools.
5. Then the council logic from the sibling spikes, amend first.
