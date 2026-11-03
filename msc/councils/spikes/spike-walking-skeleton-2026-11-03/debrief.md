# Debrief: the walking skeleton, for Joseph

*2026-11-03. From the spike agent launched by councils-46. Written from the run records in `runs/`, not from memory of the work: each claim names its run, and the few that are my inference say so. Revised the same day after two independent verifiers' passes (`de-novo-feedback-1.md`, `de-novo-feedback-2.md`): this text states what is now true, and the two `response-to-de-novo-*.md` files say what changed and why.*

Joseph,

You asked for mechanics: a council whose members join over MCP, non-Anthropic ones included, that is "turn-based and wall-clock ambivalent", where "an agent shouldn't be penalized simply because their API took a bit longer". I built the thinnest thing that does that and ran it until it broke in instructive ways. What follows is what running taught, roughly in order of how much it should change the server you build.

## What exists

A ~1,500-line Python skeleton (`concilium/`, stdlib only), the four member tools (`council_status`, `council_wait`, `council_submit`, `council_record`), a steward CLI, and launchers that seat a member from any of four harnesses with per-run configuration only. No global setting was touched. The harnesses did write their usual session records (Claude Code transcripts under `~/.claude/projects/`, agy conversations under `~/.gemini/antigravity-cli/`), and one more wrinkle: `claude --bg` sessions show up in your `claude agents` list while they run (one probe, `concilium-probe-b1`). The council logic is a crude committee (roll, floor, debate until a full round passes, vote) and nothing more; the sibling spikes own the logic.

## What ran

Ten meetings with model members, plus probes and model-free tests. The two verifiers ran four more, two of them with a `--bg` member. The code changed as the runs taught: early trials ran without seat holders. `trial-4c` and `trial-5c` ran on the build after the first review, and `trial-6-bg` on the build after the second. `README.md` names each run's build. The ones that matter:

- **One council, four harnesses, four model families** (`trial-4-mixed`): Claude haiku in Claude Code, gpt-5.6-luna in codex, grok-4.6 in grok, Gemini 3.8 Flash in agy, each in its own session with its own server process, sharing one council directory. Nine rounds in 80 seconds; every round closed because everyone had answered. The last seat in was grok's in 5 rounds and agy's in the other 4, and that cost neither of them anything. The members deliberated for real: they converged on a substitute text, and the maker withdrew so it could be moved. It was adopted 4–0, with each member's reasoning in the record.
- **Three subagents of one session sharing its MCP connection** (`trial-2-subagents`): one server process served all three seats (49 calls). The token was the only thing telling them apart, and nothing was misattributed.
- **A member killed mid-meeting and brought back** (`trial-3`, `trial-3b`): the council went on without it; the resumed fork (`--resume --fork-session`) caught up and voted.
- **Driver mode** (`trial-5`, `trial-5b`, `trial-5c`): no member waits at all; the council resumes each member's own session for a turn whenever a round is open for it.
- **The four-harness meeting again on the final build** (`trial-4c`), and once more on the earlier build by the verifier (`verify-1/runs/mixed-rerun`). In `trial-4c` grok ran out of quota mid-meeting and exited. One lease later the council stopped waiting for its seat, and the other three finished and adopted 3–0. It was the one real, unplanned crash in all the runs, and it went as designed.

Every meeting's log is internally consistent: each derived event is recomputed from the charter and the members' revealed intents, the hash chain verifies, and the revealed intents match the seats' sealed files (`steward replay`). That is not proof against tampering by anyone who can write the council directory, as the verifier showed by flipping votes and regenerating a log that still passed. Members' servers now record the chain heads they showed their members in their own logs, and `steward witness` catches such a rewrite (§7 of the response). Fifty shuffled arrival orders of the same intents produce one byte-identical log (`steward selftest`). That shows the resolver never reads arrival order; the wall-clock facts (who was absent, why a round closed) are recorded as facts and are outside the test by design.

## What broke, and what it means

**1. A second holder of a seat turns the council back into a race.** In `trial-3` I launched a fresh session for a seat that a live session already held. For five rounds the two raced: each time the resumed holder submitted for the seat first, which completed the round, and the newcomer's submission arrived to "round is not open". The newcomer never learned another mind held its seat; it blamed its own slowness. Had it been faster, a session with no memory of the first seven rounds would have voted for that seat. Now a seat has one holder at a time (a *principal*: a session or a driver, not a process), and a second one is refused and told why. A crashed holder's seat frees itself after a lease; the epoch increments, and the minutes say who held each seat when (`trial-3b`: the duplicate was refused three times and stopped on its own; `tests/regression.py` covers clean release, SIGKILL lapse and takeover). The verifier found that driver mode let every seat go between turns, so anyone with the token could take one. The driver now holds the seats it drives itself (`trial-5c`). The seat holder's lease is also now the only measure of whether a seat is live. **For the real server:** seat identity is a lease with an epoch, held by whoever outlives the turns (the session, or the driver), enforced on every call, from day one.

**2. Nothing per-member may live in a server process.** In driver mode each turn is a new process, and the "since your last look" cursor lived in process memory. So every turn re-sent the member the whole log. Keyed to the seat holder on disk, it cut driver cost per round by about 62% (`trial-5` → `trial-5b`). The rule generalizes: the harness decides how long a server process lives, and it ends them differently. Claude Code sends SIGINT (`trial-3b`), codex SIGTERM, agy closes stdin; grok ends them with nothing a server can catch, so a grok seat never releases and only lapses (codex and grok from the verifier's re-run). So anything about a member belongs to the seat's holder.

**3. The server cannot know a member saw what it sent.** When Claude Code's idle timeout or codex's 60-second tool timeout gives up on a call, the server is not told (`probes/p4`, `probes/c2`). A "your turn" view can go into the void. So rounds wait on *submissions*, never on deliveries, and the only proof a member held a state is its own act naming the round. The second verifier found that this proof was weaker than I claimed. The server still advanced the seat's "since your last look" cursor when it *sent* a view. An abandoned wait could therefore use up the reveal of the last round, and the member's live wait would show the open round without it. That happened live with a `--bg` member (`de-novo-feedback-2` §2.3). Now the cursor advances only when the member's submission is accepted, and a newer wait supersedes an older one. Every view a member can act on carries everything since its last act, so naming the round means what I said it meant.

**4. An optional identity field shown to a model gets filled with a guess.** In `trial-1`, haiku saw an optional `seat_token`, put its seat name in it, and locked itself out of the whole meeting. Single-seat servers no longer show the field. With a required one (`trial-2`), two subagents still dropped it once each on long submissions; the server refused, and they retried.

**5. Revision and fairness conflict.** If a member may replace its sealed intent until the round closes, the round closes the moment everyone has submitted once, and a revision loses whenever the last other member answers first: the same intents in shuffled orders gave four different logs. Submissions are therefore final per round. The verifier caught that I had made that rule in the store but not in what members read: the tool description and the status view still promised replacement. Both now say what the charter says, and a member whose second try meets a closed round is told its first one counted. If you ever want revision, the fair form is a two-phase round, at a round-trip's cost.

**6. A member that stops looping used to stall every round, and my first fix locked it out instead.** A member whose server is alive but whose model never answers was waited for until the full wall-clock backstop, every round, forever (first verifier). I made a seat silent through two backstopped rounds "gone until it acts again". The second verifier showed that this broke a returning member. "Acts" meant a submission that landed in a close, and rounds now closed the moment everyone else answered. So a member coming back had to out-race the others, and lost 13 times in 13 tries. The streak also counted rounds that never waited for the seat. Now any call from the seat clears the stall, and the open round waits for it again; only rounds that actually waited for the seat count toward a stall. In the verifier's own scenario, the returning member went from 0 accepted to 25 accepted and 0 refused. **And this is a rule for majority councils only.** A council that decides by unanimity or holds the item must never shrink: dropping a silent seat would turn "unanimous" into a hollow word. Such a charter turns escalation off, and the item waits or the seat is re-seated from its line (`reaction-line-of-stewards.md`; re-seating is not built).

## How members wait and wake

- **Blocking works everywhere I could reach.** A blocking `council_wait` that sends a progress notification every ~10 s worked in Claude Code `-p`, codex, grok and agy (150-second calls in each, `harness-notes.md`).
- **codex is the exception.** It cuts calls at 60 s, and progress doesn't help; its own per-run `tool_timeout_sec` fixes it.
- **Claude Code `-p` did not background a long call.** The docs' 2-minute auto-background never happened in print mode.
- **Blocking only reaches a member that is still looping.** A session whose model has ended its turn (interactive, `--bg`, a prepared expert) is not waiting on anything. Driver mode wakes it by giving it a turn.
- **Driver mode works, at a modest cost.** Sessions keep their memory across turns (one session id per seat throughout). It costs about 1.8× a blocking member per round with haiku. In a resumed `-p` session the harness reports `total_cost_usd` cumulatively over the whole session, so the right total is each seat's last turn, which is what that figure uses.
- **What I'd build.** Both adapters over one round protocol. For prepared experts, the first turn is a `--fork-session` of the expert, so "always fork, never resume an expert" holds; but see the next paragraph before choosing driver mode for them.

**For interactive and background members** (`probes/b1-bg-sleep170`; the second verifier's `verify-2/runs/bg-wait-*`; `runs/trial-6-bg`). In a `claude --bg` session, a blocking call running past 2 minutes is moved to a background task. That happens even while the server sends progress. If the model then ends its turn, the server's eventual answer arrives as a notification and wakes it. I first presented that wake as a harness property; it is partly a property of my probe's prompt, which said "then stop". Under the members' brief ("if a call times out, call council_wait again") the second verifier's member did not go idle. It opened a second wait, and the two waits are what used up the reveal in finding 3. So a `--bg` member needs a brief that says to end the turn after a backgrounding (`members/brief-bg.md`). It also needs `--no-chrome`: a `--bg` session gets Claude Code's interactive prompt, and on this machine 22 browser-automation tools, which the same flags keep from a `-p` session. I re-ran it with both fixes (`runs/trial-6-bg-attempt1`, `-attempt2`). It never got that far. In both runs the session's first request to the model carried **no tools**, about 200 ms after the council server had listed them. The member either produced nothing, or wrote its call out as plain text and ended its turn, and nothing woke it again. That makes 3 of 4 background launches so far. So, as launched, the backgrounded wait usually fails before the member reaches the council, and **the driver is the adapter to build first** for experts and other background members: each driver turn is a `-p` process, and `-p` never showed this race. `--no-chrome` did remove the Chrome section from the prompt; the brief change is untested.

## Wiring non-Anthropic members without touching global config

All four worked. Two things changed since utf-mcp's notes a month ago:

- codex 0.163 now refuses MCP calls in `exec` unless the server entry says `default_tools_approval_mode="approve"`.
- agy 1.4.12 has a session-only `--settings` flag. It carries the MCP allow rule that utf-mcp could only get by editing your `~/.gemini` settings, so that blocker is gone.

The Gemini CLI is dead for your account ("migrate to the Antigravity suite"); agy is the Google route. On identity, codex and agy tell the server the caller's session or conversation on every call; Claude Code and grok tell it nothing. What codex and agy send is an assertion the server records, not something it can check: any client can send the same id, and the second verifier showed two connections sharing one seat that way. Seat exclusivity now also requires a holder key the client never sees. codex also sends every MCP server your workspace's git remote and HEAD commit. That is harmless locally; worth knowing before any council server is remote.

## Sealing is honor-system in a shared directory

Sealed intents are files in the council directory. Today they stay sealed only because every launcher withholds file tools from members. The agy member reached for `run_command` and `read_file` unprompted (to read its own tool schema; the hook denied both). Prepared experts keep their file tools. The log itself has the same exposure, as the verifier showed. So before a member with file tools sits on a council, the council directory as a whole (sealed intents, log, holders) needs to live somewhere members cannot write or read. That is the first concrete reason for a daemon.

Seat tokens have a related problem. Handed to a member through its server's environment, they stay out of transcripts. Passed as a tool argument (multi-seat mode, `trial-2`), they land in every transcript the harness keeps. Four committed files here carried them; they are redacted now. The commits whose history still has them are named in the response (§5), for you to decide on before any push. The values are for dead local councils and grant nothing.

## What the members kept running into (for the council logic)

Every meeting I examined for it (`trial-2`, `-4`, `-5`, `-5b`, and the verifier's re-run, which adopted a second "reaffirm and clarify" motion) hit the missing **amend**, and members routed around it a different way each time:

- voting yes on superseded text, each vote carrying a reservation (`trial-2`);
- the maker withdrawing so the substitute could be moved (`trial-4`);
- unanimously **rejecting their own motion**, 0–3, to move the text they had agreed (`trial-5`); and in `trial-5b`, a 1–2 rejection followed by a restatement, a withdrawal and a re-move.

Until the workaround landed, what the record said the council adopted and what the council actually thought were different things. That is the failure mode a council exists to prevent, so amend is the first piece of logic I would add. Second: near-identical motions are the common case. In `trial-4` all four seats moved the agreed text in one round, and the record shows two co-makers and two rivals whose standing intents then lapsed. Your near-duplicate example was the first thing reality produced. One member (grok's) asked to be told how the winning motion was chosen; another (Claude's) asked for a "superseded by" marker on stale standing intents.

## What I did not do

- No interactive (TUI) member: Joseph-attended sessions, channels and MCP tasks are untested. One `--bg` session was tested, with the probe server, not a full council meeting. The verifier did not re-run it either.
- No remote or HTTP transport.
- No member with file tools.
- No more than four members.
- The 30-minute default idle timeout and the ~28-hour hard cap were not run to term: I tested the timeouts by lowering them per process.
- Driver mode was run with Claude only. Its codex (`exec resume`) and agy (`--conversation`) equivalents are an inference.
- Whether a `--resume --fork-session` member's first reported cost includes its parent session's usage is unchecked.
- Independent verification has happened twice (`de-novo-feedback-1.md`, `de-novo-feedback-2.md`). The first confirmed the lock discipline under 20 concurrent writers and the SIGKILL lapse path. The second re-derived every measured claim of mine and of the first verifier, and they held; it also found the defects in findings 3 and 6. The fixes from the second pass (`response-to-de-novo-2.md`) are checked only by my own suite, which runs both verifiers' tests. They have not been independently verified. Still open from my first list:
  - whether `parent-process` is a sound principal for every harness (it is a daemon "spare" process for `--bg` sessions);
  - whether my reading of the 30 s idle-timeout floor is right (one run).

The files: `README.md` is the entrypoint. `harness-notes.md` has the per-harness facts. `proposed-integration-plan.md` has suggestions for the real server, each tied to its run.

I'm staying on the line for follow-ups.
