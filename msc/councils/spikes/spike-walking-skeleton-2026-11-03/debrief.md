# Debrief: the walking skeleton, for Joseph

*2026-11-03. From the spike agent launched by councils-46. Written from the run records in `runs/`, not from memory of the work: each claim names its run, and the few that are my inference say so.*

Joseph,

You asked for mechanics: a council whose members join over MCP, non-Anthropic ones included, that is "turn-based and wall-clock ambivalent", where "an agent shouldn't be penalized simply because their API took a bit longer". I built the thinnest thing that does that and ran it until it broke in instructive ways. What follows is what running taught, roughly in order of how much it should change the server you build.

## What exists

A ~1,300-line Python skeleton (`concilium/`, stdlib only), the four member tools (`council_status`, `council_wait`, `council_submit`, `council_record`), a steward CLI, and launchers that seat a member from any of four harnesses with per-run configuration only. No global setting was touched. The harnesses did write their usual session records (Claude Code transcripts under `~/.claude/projects/`, agy conversations under `~/.gemini/antigravity-cli/`), and one more wrinkle: `claude --bg` sessions show up in your `claude agents` list while they run (one probe, `concilium-probe-b1`). The council logic is a crude committee (roll, floor, debate until a full round passes, vote) and nothing more; the sibling spikes own the logic.

## What ran

Seven meetings with model members, plus probes and model-free protocol tests. The ones that matter:

- **One council, four harnesses, four model families** (`trial-4-mixed`): Claude haiku in Claude Code, gpt-5.6-luna in codex, grok-4.6 in grok, Gemini 3.8 Flash in agy, each in its own session with its own server process, sharing one council directory. Nine rounds in 80 seconds; every round closed because everyone had answered. The last seat in was grok's in 5 rounds and agy's in the other 4, and that cost neither of them anything. The members deliberated for real: they converged on a substitute text, and the maker withdrew so it could be moved. It was adopted 4–0, with each member's reasoning in the record.
- **Three subagents of one session sharing its MCP connection** (`trial-2-subagents`): one server process served all three seats (49 calls). The token was the only thing telling them apart, and nothing was misattributed.
- **A member killed mid-meeting and brought back** (`trial-3`, `trial-3b`): the council went on without it; the resumed fork (`--resume --fork-session`) caught up and voted.
- **Driver mode** (`trial-5`, `trial-5b`): no member waits at all; the council resumes each member's own session for one turn per round it is addressed in.

Every meeting's log replays to itself: each derived event is recomputed from the charter and the members' revealed intents, and the hash chain verifies (`steward replay`, all 11 councils). Fifty shuffled arrival orders of the same intents produce one byte-identical log (`steward selftest`).

## What broke, and what it means

**1. A second holder of a seat turns the council back into a race.** In `trial-3` I launched a fresh session for a seat that a live session already held. For five rounds the two raced: each time the resumed holder submitted for the seat first, which completed the round, and the newcomer's submission arrived to "round is not open". The newcomer never learned another mind held its seat; it blamed its own slowness. Had it been faster, a session with no memory of the first seven rounds would have voted for that seat. Now a seat has one holder at a time (a *principal*: a session or a driver, not a process), and a second one is refused and told why. A crashed holder's seat frees itself after a lease; the epoch increments, and the minutes say who held each seat when (`smoke/c2`, `trial-3b`; in `trial-3b` the duplicate was refused three times and stopped on its own). **For the real server:** seat identity is a lease with an epoch, enforced on every call, from day one.

**2. Nothing per-member may live in a server process.** In driver mode each turn is a new process, and the "since your last look" cursor lived in process memory. So every turn re-sent the member the whole log, and driver mode looked many times more expensive than it is. Keyed to the seat holder on disk, it cut cost per member-turn by a third (`trial-5` → `trial-5b`). The rule generalizes: the harness decides how long a server process lives (Claude Code `-p` ended them with SIGINT at exit in `trial-3b`; agy closes their stdin; codex and grok end them by signal, per utf-mcp's notes and consistent with my probes), so anything about a member belongs to the seat's holder.

**3. The server cannot know a member saw what it sent.** When Claude Code's idle timeout or codex's 60-second tool timeout gives up on a call, the server is not told (`probes/p4`, `probes/c2`). A "your turn" view can go into the void. So rounds wait on *submissions*, never on deliveries, and the only proof a member held a state is its own act naming the round. The charter-agenda spike's "common-state condition" is enforceable exactly that way.

**4. An optional identity field shown to a model gets filled with a guess.** In `trial-1`, haiku saw an optional `seat_token`, put its seat name in it, and locked itself out of the whole meeting. Single-seat servers no longer show the field. With a required one (`trial-2`), two subagents still dropped it once each on long submissions; the server refused, and they retried.

**5. Revision and fairness conflict.** If a member may replace its sealed intent until the round closes, the round closes the moment everyone has submitted once, and a revision loses whenever the last other member answers first: the same intents in shuffled orders gave four different logs. Submissions are therefore final per round. If you ever want revision, the fair form is a two-phase round, at a round-trip's cost.

## How members wait and wake

- **Blocking works everywhere I could reach.** A blocking `council_wait` that sends a progress notification every ~10 s worked in Claude Code `-p`, codex, grok and agy (150-second calls in each, `harness-notes.md`).
- **codex is the exception.** It cuts calls at 60 s, and progress doesn't help; its own per-run `tool_timeout_sec` fixes it.
- **Claude Code `-p` did not background a long call.** The docs' 2-minute auto-background never happened in print mode.
- **Blocking only reaches a member that is still looping.** A session whose model has ended its turn (interactive, `--bg`, a prepared expert) is not waiting on anything. Driver mode wakes it by giving it a turn.
- **Driver mode works, at a cost.** Sessions keep their memory across turns (one session id per seat throughout), but after the fix it still costs about 6–8× a blocking member per round with haiku. I've measured that but only partly explained it.
- **What I'd build.** Both adapters over one round protocol. For prepared experts, the first turn is a `--fork-session` of the expert, so "always fork, never resume an expert" holds; but see the next paragraph before choosing driver mode for them.

**And a better answer for interactive and background members, found last** (`probes/b1-bg-sleep170`). In a `claude --bg` session, a blocking call that runs past 2 minutes is moved to a background task, and the model ends its turn and goes idle. When the server finally answers, the result arrives as a notification that wakes the idle session, which acts on it. So a background or interactive member needs neither a driver nor channels: it calls `council_wait`, goes idle, and the server's "your turn" wakes it. One run, one model. The caveat is in the harness's own message: the backgrounded call "does not survive exiting this session". If that holds up under a verifier, it is the cheapest way to seat a prepared expert: fork it once, then let it wait. Driver mode becomes the fallback for harnesses without this behavior.

## Wiring non-Anthropic members without touching global config

All four worked. Two things changed since utf-mcp's notes a month ago:

- codex 0.163 now refuses MCP calls in `exec` unless the server entry says `default_tools_approval_mode="approve"`.
- agy 1.4.12 has a session-only `--settings` flag. It carries the MCP allow rule that utf-mcp could only get by editing your `~/.gemini` settings, so that blocker is gone.

The Gemini CLI is dead for your account ("migrate to the Antigravity suite"); agy is the Google route. On identity, codex and agy tell the server the caller's session or conversation on every call; Claude Code and grok tell it nothing. codex also sends every MCP server your workspace's git remote and HEAD commit. That is harmless locally; worth knowing before any council server is remote.

## Sealing is honor-system in a shared directory

Sealed intents are files in the council directory. Today they stay sealed only because every launcher withholds file tools from members. The agy member reached for `run_command` and `read_file` unprompted (to read its own tool schema; the hook denied both). Prepared experts keep their file tools. Before one sits on a council, sealed intents need to live somewhere members cannot read. That is the first concrete reason I found for a daemon.

## What the members kept running into (for the council logic)

Every meeting hit the missing **amend**, and members routed around it a different way each time:

- voting yes on superseded text, each vote carrying a reservation (`trial-2`);
- the maker withdrawing so the substitute could be moved (`trial-4`);
- unanimously **rejecting their own motion**, 0–3, to move the text they had agreed (`trial-5`); and in `trial-5b`, a 1–2 rejection followed by a restatement, a withdrawal and a re-move.

Until the workaround landed, what the record said the council adopted and what the council actually thought were different things. That is the failure mode a council exists to prevent, so amend is the first piece of logic I would add. Second: near-identical motions are the common case. In `trial-4` all four seats moved the agreed text in one round, and the record shows two co-makers and two rivals whose standing intents then lapsed. Your near-duplicate example was the first thing reality produced. One member (grok's) asked to be told how the winning motion was chosen; another (Claude's) asked for a "superseded by" marker on stale standing intents.

## What I did not do

- No interactive (TUI) member: Joseph-attended sessions, channels and MCP tasks are untested. One `--bg` session was tested, with the probe server, not a full council meeting.
- No remote or HTTP transport.
- No member with file tools.
- No more than four members.
- The 30-minute default idle timeout and the ~28-hour hard cap were not run to term: I tested the timeouts by lowering them per process.
- Driver mode was run with Claude only. Its codex (`exec resume`) and agy (`--conversation`) equivalents are an inference.
- The cost gap in driver mode is measured, not explained.
- I have not verified my own work independently; that is for the verifier you commission. The places I'd look first:
  - the store's lock discipline under many concurrent writers (tested with 4 members and 50 shuffled single-process orders, not with a stress test);
  - whether `parent-process` is a sound principal for every harness (it is a daemon "spare" process for `--bg` sessions);
  - whether my reading of the 30 s idle-timeout floor is right (one run).

The files: `README.md` is the entrypoint. `harness-notes.md` has the per-harness facts. `proposed-integration-plan.md` has suggestions for the real server, each tied to its run.

I'm staying on the line for follow-ups.
