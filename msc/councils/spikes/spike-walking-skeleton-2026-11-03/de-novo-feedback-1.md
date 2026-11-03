# De novo feedback 1: adversarial pass on the walking skeleton

*2026-11-03. Verifier: a Claude (Fable 5.1) session independent of the spiker. Scope: everything in this directory, read whole, plus the sibling spikes and the councils-level docs the spike cites. Method: re-derive and re-run rather than trust. Everything I ran is in `verify-1/` (`verify.py`, its output `verify-output.txt`, and `runs/mixed-rerun/`, a fresh four-harness meeting on the committed code). Where a finding is a judgment rather than a measurement, it says so. Treat this report's finding list, not its silence, as its content: I did not audit the RONR citations in `core.py`, the member transcripts' deliberative quality, or anything about the `--bg` claim beyond the one recorded session.*

## 0. The short version

The skeleton does what the debrief says it does, and the record supports nearly every number in it. I re-ran the selftest, replayed all ten councils, cross-checked every `round-closed` against its `sealed/` files (0 mismatches in 10 councils), recomputed the cost figures, the trial-4 timing, the identity-stability claims for codex and agy, and the probe timelines; and I ran a fresh four-harness meeting (Claude + codex + grok + agy) on the committed code, which the spiker never did: all four seated, ten rounds, every round closed all-in, replay OK (`verify-1/runs/mixed-rerun`).

Against that, five things the next reader should know before building on this:

1. **Driver mode leaves every seat unheld between turns** (`verify.py` test A). The holder is released when each per-turn server exits, so a stranger with the token takes the seat and the driver is then refused its own seat. The debrief's "seat identity … enforced on every call, from day one" is true of the client adapter only. `trial-5b`'s own receipts show it (23 `seat-released`, 3 `seat-held`).
2. **The committed code was run with model members only in `trial-3b` and `trial-5b`** (Claude haiku). Every other trial, including the four-harness showcase `trial-4` and the `smoke/c2` holder test the plan cites, ran on an earlier build (file mtimes 13:07 local vs runs at 12:55–13:05). My re-run closes that gap for the client adapter; nothing closes it for driver mode beyond `trial-5b`.
3. **Members are told twice they may replace a sealed intent, and the server refuses** (test D5): the `council_submit` tool description and the status view both say "You may replace it until the round closes"; every charter in the record has `replaceable: false`. `trial-5b` shows the bite (rec's "not open" refusals in r3 and r4 are a replacement attempt that got a misleading error).
4. **Seat tokens are in the committed record.** `runs/trial-2-subagents/host/{server,stream}.jsonl` and `runs/smoke/server-{A,B}.jsonl` contain the live token values of those councils; the README says tokens are git-ignored. Dead councils, so no harm now, but it is the design speaking: a token passed as a tool argument lands in every transcript the harness keeps.
5. **"Every log replays to itself" is a derivation check, not integrity** (test E): with directory access you can flip every vote in `trial-4`'s r8, regenerate, delete `sealed/`, and `steward replay` says OK. The spike names sealing as honor-system; the log is too, for the same reason.

Plus one place where the spike's own numbers disagree with each other (§3.4): the 6–8× driver-mode cost ratio comes from the harness's `total_cost_usd`; the same transcripts' token counts put the two meetings within ~1.3× of each other. The plan ranks the adapters by this number.

## 1. What I did

| Step | Where | Result |
|---|---|---|
| Read every file in the spike (code, launchers, charters, docs), the three sibling/council docs it cites, `contention.md`, charter-agenda `01`/`04`, `mechanics.md`, `initial-thoughts.md`, `specimens-2026-11-03.md` | — | — |
| `steward selftest`, `steward replay` on all 10 council dirs | `verify-1/verify-output.txt` is the model-free part; replay output in this report §2 | 1 log / 4 logs as claimed; all 10 replay and chain OK |
| Cross-check of every council's receipts against its events and `sealed/` (close reasons, waits, refusals, holders, sealed==logged, calls per tool) | scratch script, results folded into §2–§3 | 0 sealed-vs-logged mismatches anywhere |
| Recompute: trial-4 timing and last-seat-in, trial-2 call count and token drops, trial-3/3b refusals, driver-turn and per-round costs, codex/agy identity stability, probe timelines p2/p4/p5/p6/b1/c2, agy hook denials, codex workspace metadata | §2 | all hold, with the corrections in §3 |
| Nine model-free tests against the committed code: driver gap, SIGKILL lapse, 20-writer concurrency stress, debate-limit reset, all-gone adjourn, stuck-live member, late submission, replace promise, replay-as-integrity | `verify-1/verify.py` | §3, §4 |
| A fresh four-harness meeting on the committed code (`charters/trial-4-mixed.json`, the four launchers as the README gives them, no global config touched) | `verify-1/runs/mixed-rerun/` | §2.1 |

Not done: no driver-mode re-run with models; no `--bg` re-run (the recorded session `probes/b1` is consistent with the claim, timestamps in §2.3, but it is one run and I did not add a second); no audit of RONR cites; no reading of the member transcripts for deliberative quality beyond what the minutes show.

## 2. Claims that hold, as re-derived

### 2.1 The four-harness meeting, re-run on the committed code

`verify-1/runs/mixed-rerun`: council created 20:27:10Z, all four members exited by 20:29:18Z. Ten rounds in 112.8 s, every one closed `all-submitted`, no refusals, replay and chain OK. Holders, from the receipts:

```
indep  parent-process:…:69591                           (Claude Code: no identity in _meta, as documented)
plan   parent-process:…:69601                           (grok: same)
cost   codex-session:01a1a318-24a2-7cb1-bfd5-02fe6c3bc94e   (codex _meta.sessionId)
fam    agy-conversation:3e2de62e-9a75-486b-bcd5-ef756ec06c2a (agy root_conversation_id)
```

So the `principal_for` branches for codex and agy, which no prior run exercised on this build, work. The last seat in was grok's in 7 of 10 rounds and it cost it nothing. Outcome: adopted 3–1, then a second "reaffirm and clarify" motion adopted 4–0; the members again routed around the missing `amend`, this time by adopting a second motion on the same item.

How each harness ended its server, now recorded because the committed server logs signals: Claude Code SIGINT, codex SIGTERM, agy closes stdin, **grok sends nothing the server can catch** (no `signal`, no `stdin-closed`; the process is simply gone two seconds before grok exits, no orphan left). So a grok seat is never released; it lapses after `lease_s`. That sharpens the plan's "codex and grok end them by signal" (plan §1 / harness-notes): codex yes, grok by an uncatchable one.

### 2.2 Numbers in the debrief, plan and README

| Claim | Re-derived | Holds? |
|---|---|---|
| selftest: replaceable=false → 1 log; true → >1 | 1 and 4 (50 orders each) | yes |
| every run replays and chains | all 10 council dirs OK | yes ("11 councils" in the debrief: there are 10) |
| trial-4: 9 rounds in 80 s, all closed all-in, adopted 4–0, last seat in grok ×5 / agy ×4 | 9 rounds, 80.0 s first-open→last-close, 9× `all-submitted`, 4–0 | yes |
| trial-2: one server process, 49 calls, no misattribution, adopted 3–0; two seat_token drops | 49 `tools/call` in `host/server.jsonl`, 2 "no seat" errors, 3–0 | yes |
| trial-3: duplicate raced for five rounds | 5 `submit-refused` for ses, r8–r12, "not open" | yes |
| trial-3b: duplicate refused three times and stopped | 3 `seat-claim-refused`, duplicate session 4 turns, $0.0009 | yes |
| trial-1: sec locked out by `seat_token` | sec `gone` in all 6 rounds, 1 `council_record` call, exited at 12 s | yes |
| driver cost $0.0207 → $0.0138 per driver-turn | 0.0207, 0.0138 | yes |
| driver ~6–8× a blocking client per round ($0.026 vs $0.0035–0.0045) | per round, 3 seats: $0.0264 (5b) vs $0.0038 (3b) → 7× | yes on the same measure; see §3.4 |
| codex session id and agy root conversation id stable across a session's calls | trial-4: cost 19 calls one id; fam 15 calls one id | yes |
| codex sends workspace remote + HEAD | `x-codex-turn-metadata.workspaces` carries `origin` URL, commit hash, dirty flag | yes |
| agy member reached for `run_command`/`read_file`, denied | hook log: 1 deny each, 15 `call_mcp_tool` allow | yes |
| p4: idle abort ~32 s, no cancel reaches server; p5: progress completes at 60 s; p6: hard timeout cancels at 30 s; p2: 150 s completes in `-p` | timelines match to the second | yes |
| b1: backgrounded at 120 s, model ends turn, wake on result at ~172 s | transcript: tool use 20:10:16, background notice 20:12:16, turn ends 20:12:19, result + task-notification 20:13:06, acted 20:13:07 | yes, one run |
| the record names the holder of every seat | yes in trial-3b/5b and the re-run; **not** in trials 1–5, which predate holders | partly |

### 2.3 Properties I tested that the spike claims or relies on

- **SIGKILL of a holder → lease lapses → takeover with epoch 2, previous marked `released: false`, lapsed_for_s recorded** (test B). The model runs only exercised SIGTERM, which is the clean-release path; the crash path works too.
- **Lock discipline under 20 concurrent writer processes + 2 tickers, 8 meetings** (test C): chain OK, replay OK, seq contiguous, each round closed once, every accepted submission's sha present in its round's close, `sealed/` == logged, no receipt corruption. The debrief put this first among "places I'd look"; it held.
- **Single-seat servers do not advertise `seat_token`** (test D5).

## 3. Where the record and the text disagree, or the text overreaches

### 3.1 Driver mode: the seat is unheld between turns (defect)

`server.py` releases every seat it holds on SIGINT/SIGTERM/SIGHUP and on stdin close (`_release_all`), and in driver mode every turn is a process that ends one of those ways. The driver (`members/driver.py`) heartbeats *presence* for its seats but never claims or refreshes the *holder*. So between turns `holder/<seat>.json` has `ts: 0`.

Test A, on the committed code: driver turn ends → `holder ts = 0` → a second principal with the token calls `council_status` → **held, epoch 2** → the driver's next turn → **refused** ("already held by another live connection"). The driver has no code path for a refusal: reading `driver.py`'s loop (not run), a refused turn ends, `BUSY` clears, and the seat is re-launched a second later, until the round closes without it.

`trial-5b`'s receipts show the shape: 23 `server-exit`, 23 `seat-released`, 3 `seat-held` (the same principal re-claims without an epoch change, so the record looks continuous). The plan's §2.2 ("one live holder per seat … enforced on every call") and §3.1 ("The driver holds presence for the seats it drives and passes one principal to all its per-turn server processes") describe presence, not holding. The fix is small (the driver claims and refreshes holders; or servers under `CONCILIUM_PRINCIPAL` do not release on exit), but as written the exclusivity property is an adapter-specific property, and the plan says to port `store` "as it now stands".

### 3.2 The committed code vs the evidence

Code mtimes (local): `core.py`/`steward.py` 12:54; `server.py`/`store.py`/`view.py` 13:07; `driver.py` 13:07. Runs (local): smoke/c1 12:55, trial-1 12:55, trial-2 12:59, trial-3 13:01, trial-4 13:02, trial-5 13:03, **smoke/c2 13:06**, trial-5b 13:07, trial-3b 13:07, smoke/c3 13:09. So:

- Trials 1–5 and smoke/c1–c2 ran on code without seat holders and persisted cursors (their receipts have no `seat-held`/`seat-released` kinds at all).
- `smoke/c2`, cited by the plan as "tested by RPC" for holders, ran on an intermediate build: its `seat-held` receipt after a release carries `lapsed_for_s: 1793736395.8` (now minus zero) and no `released` key, which the committed `claim_seat` would not write. The minutes for c2 therefore print "whose hold had lapsed 1793736395.8s".
- There is no script for the smoke tests in the repo (`tools_rpc_client.py` is the client; nothing drives c1/c2/c3), so "protocol without models … all as designed" is not reproducible from the commit. `verify.py` tests A, B, D3 and D5 cover the holder and backstop paths by RPC on the committed code (D4 and the concurrency test go through the store API).
- The README's re-run recipe does not run as written: `python3 -I -m concilium.steward …` fails (`-I` keeps the cwd off `sys.path`, so the package is not found); `python3 -m concilium.steward` or `python3 -I concilium/steward.py` both work. The "run as python3 -I" note is right for `server.py` (which inserts its own path) and wrong for the `-m` lines.

None of this changes what the runs showed about harnesses; it changes what the *committed* code has been shown to do with model members: `trial-3b`, `trial-5b`, and now the re-run.

### 3.3 The replace promise (defect, small code; real cost in the runs)

`TOOLS_ALL[council_submit].description`: "You may replace it until the round closes; only your last counts." `view.render`: "You have submitted (sealed): … You may replace it until the round closes." `members/brief.md`: "a submission is final for that round". Every charter: `replaceable: false`. `store.submit` refuses with "submissions are final". So each member ran under a contradictory contract, and the refusal they actually see is usually the *wrong* one: because a member's own first submission typically completes the round, the second arrives to "round rN is not open (open round: rN+1)". `trial-5b` rec r3 and r4 are exactly this (speak, then pass, in the same turn). The debrief's finding 5 ("Submissions are therefore final per round") landed in the store and not in the two places the member reads.

### 3.4 The driver-mode cost gap: two harness measures disagree

The 6–8× figure (and "about $0.001 per member per round" for the client) comes from `total_cost_usd` in each `claude -p` process's `result` message. I summed the per-assistant-message `usage` blocks in the same transcripts (same caveat for both: stream-json may repeat a message's usage per content block, so absolute sums are inflated, but equally so):

```
trial-3b blocking, 3 seats:  cache_creation 330k   cache_read 2.65M   (104 assistant msgs)
trial-5b driver,  3 seats:   cache_creation 355k   cache_read 3.43M   (102 assistant msgs)
```

That is ~1.1–1.3×, not 6–8×. Per driver turn: ~8.8k cache creation, ~95k cache read, ~1.2k output (matches the debrief's "~8.8k tokens of prompt cache" per resumed turn). I cannot say which measure is right without the API bill; what I can say is that the plan's adapter ranking (§3.1, "Cheapest: … client") rests on the one of two measures that is not the token count, and the debrief already marks the gap "measured but only partly explained". Worth settling before cost drives the adapter choice for prepared experts.

Related: "one turn per round" is not what happened in `trial-5b`. A driven turn's `council_submit` reply carries the *next* round's view (the submit closes the round and the reply renders after `try_close`), so haiku kept going: rec's turn t2 covered r3–r6 (four rounds, six submits, two refused); sec's turns covered two rounds each. Driver turns per seat were 7–9 for 12 rounds. "Per member-turn" in the plan is therefore per ~1.3–1.7 rounds; the per-round totals I recomputed are the comparable figures, and they do match the plan's.

### 3.5 Smaller corrections

- "`trial-3b` rec, $0.0148 over 13 rounds" (plan §3.1): $0.0148/12 rounds is `trial-3` rec; `trial-3b` rec is $0.0164/13. The per-round figure is unchanged.
- "all 11 councils" (debrief): 10.
- `harness-notes.md` row for Claude Code identity cites `probes/p1-echo`; fine. But the server's own `attest()` records `CC_AGENT` from the server's env, which (as the plan §3.3 says) never reaches an MCP server; the field is always null in the receipts. Harmless, and worth deleting so it does not read as an identity source.
- `server.py` carries dead state (`CURSORS`, `SEATS_USED`) from before the persisted-cursor fix.
- `trial-4` was run before the server logged signals, so its record cannot support the harness-notes line "Claude Code, codex and grok end the server by signal" for codex/grok; the re-run now does (§2.1), with the grok correction.

## 4. Properties of the design worth knowing before porting it

These are not claims the spike gets wrong; they are things the committed design does that the next builder inherits. Each is demonstrated in `verify.py`.

1. **A live-but-silent member stalls every round for the full backstop, forever** (test D3). Presence is the server process's heartbeat, so a member whose model has stopped looping (ended its turn, hit an idle abort and did not retry, wandered off) is "live and absent" every round: each round waits `backstop_s` (900 s in the trial charters), closes `backstop-deadline`, and the next round waits again. Nothing escalates "absent N rounds running" to "gone". `trial-1`'s locked-out sec would have done this to the meeting had its process not exited at 12 s. The presence files already record `in_wait` and `last_call` per process; the close condition does not read them. That is the cheapest fix I can see: a seat whose server is alive but has had no call in flight for longer than some bound is not "slow", it is "not looping".
2. **A late submission is refused and its content is not recorded** (test D4): the `submit-refused` receipt carries seat, round and reason, not the intent or its sha. `contention.md` F4 says "a late response is logged as supplementary, never silently dropped." The two spikes disagree here and the plan does not notice; the charter-agenda spike's common-state condition is fine with dropping, so this is a cross-spike inconsistency to settle, not necessarily a bug.
3. **`max_inquorate_rounds` is counted in closes, and an all-gone round closes on every tick** (test D2): a council whose members all crashed adjourns two ticks after the leases lapse (the servers tick every 5 s; the driver every 5 s; `steward tick` whenever). "Two consecutive inquorate rounds" has no duration.
4. **The debate-round limit resets on a partial withdrawal** (test D1): a co-maker withdrawing in debate re-emits `motion-pending`, and `apply()` zeroes `debate_rounds_this_motion` on every `motion-pending`. With `max_debate_rounds: 3`, one co-maker withdrawing in round 2 put the question after round 5. Charter policy silently extended by a procedural act.
5. **Gone-detection reads presence files, not holders**: after a clean release (`ts: 0`) the presence file keeps the seat "live" for up to `lease_s`; after SIGKILL both lapse together. Harmless inconsistency; worth unifying when holders become the one source of truth.
6. **The log, the holders and the leases live in the same directory as the sealed intents.** Test E: flip every r8 vote in a copy of `trial-4`, regenerate the derived events, rewrite the chain, delete `sealed/`; `steward replay` reports chain OK and replay OK, and the minutes now say REJECTED 0–4. The hash chain has no external anchor and `replay` does not read `sealed/`. The plan's §3.4 says sealed intents are "the first concrete reason for a daemon"; the record is the second, and it is the same reason. The plan's §6 step 4 should read "decide where the council directory lives", not "where sealed intents live".
7. **Token-as-argument** (the multi-seat design the plan keeps in §2.4) puts the credential in every transcript the harness writes: `trial-2`'s three tokens are in its committed `stream.jsonl` and `server.jsonl`, and (checked by grep) in the host session's transcript and its subagent transcripts under `~/.claude/projects/…`. Single-seat servers take the token from env and the committed server log filters `*TOKEN*` env, so those are clean (checked: no token value from any `tokens.json` appears in any single-seat run file, committed or in my re-run).
8. **Schema is not enforced server-side.** `trial-5b` sec r6 submitted without `action` (Claude Code passed it through despite `required`); the server's own validation caught it. Fine; just do not rely on the harness.
9. **The selftest is true by construction.** With `lease_s`/`arrival_grace_s` 9999 and every seat submitting, the only thing entering the log is the seat-sorted submitted set, so one log is guaranteed by the data model; the test guards the resolver against a regression that reads order. The wall-clock facts that make real logs differ (`gone`, `absent`, `closed_by`) are outside it by design, which the plan states correctly and the README's "latency invariance: 50 shuffled arrival orders" states more strongly than it tests.

## 5. On the spike's judgment, not its facts

Two places where I think the spike's framing undersells its own evidence:

- **"The council logic is deliberately crude"** is true, and the runs still produced the most useful council-logic data in the three spikes: four distinct workarounds for a missing `amend` across five meetings (now six, with the re-run's "reaffirm and clarify" second motion), and near-duplicate motions on the first opportunity in every multi-seat open round. The charter-agenda spike already leans on this (`04-machinery-survey.md`, `debrief.md`). That evidence is a property of *running real members*, and it argues for keeping a crude-but-live skeleton as the test bench for whatever resolver the siblings settle on, not for retiring it once the real server exists.
- **The `--bg` wake** (debrief, "found last") is the one mechanism that seats an interactive or expert session at client cost with no driver. It is one run with the probe server, and the author says so. Given §3.1 and §3.4 (driver mode is both the adapter with the holder gap and the one whose cost measure is in doubt), the `--bg` path deserves to be the *first* thing the next cycle runs, as the plan suggests, and I would move it above driver mode in the build order (§6 step 3).

## 6. What I would change in the three documents, concretely

1. Debrief §1 and plan §2.2: qualify "enforced on every call" with "in the client adapter; driver mode releases the holder at every turn end (verify-1 test A)" until `driver.py` holds the seats it drives.
2. Debrief "What ran" / README table: say which runs are on the committed build (3b, 5b, and `verify-1/runs/mixed-rerun`) and which on earlier ones.
3. `server.py` tool description, `view.py` line 84: make them say what the charter says; or make `view.render` read `replaceable`.
4. README: fix the `-m`/`-I` recipe; drop the claim that tokens are git-ignored or purge `trial-2`'s host logs and `smoke/server-{A,B}.jsonl` (the committed values are dead, so this is about the claim, not the exposure).
5. Plan §3.1: mark the 6–8× as "harness `total_cost_usd`; token counts in the same transcripts are within ~1.3×; unresolved".
6. Plan §3.4 / §6.4: widen "where sealed intents live" to "where the council directory lives".
7. Plan §1.4: add the stalled-live-member case and the `in_wait`/`last_call` data that already exists to address it.
8. `core.py` debate branch: do not re-emit `motion-pending` on a partial withdrawal, or do not reset the counter when it is re-emitted.

## 7. Scope honesty

Verified by re-running: §2 entire, §3.1, §3.3, §4.1–4.6, §4.8–4.9. Verified by reading the record only: §2.2's b1 row, §2.1's grok exit (no signal logged; process gone; no orphan). Judgment: §3.4's conclusion that the ratio is unresolved (the measurement is real, the pricing is not mine to settle), §5, §6. Not examined: whether `--resume --fork-session` members carry the right context (I read that they voted, not what they knew); the RONR citations in `core.py`; whether `parent-process` is a sound principal for `--bg` sessions (the author's own open question); anything about remote transport.

Files I wrote: this report; `verify-1/verify.py`, `verify-1/verify-output.txt`, `verify-1/verify-run.log`, `verify-1/.gitignore` (same exclusions as the spike's), and `verify-1/runs/mixed-rerun/` (its `tokens.json`, harness homes and `cwd/` are ignored; the committed files contain no token value, checked). No file outside `verify-1/` and this report was changed; no global configuration was touched; the four launchers were run exactly as the README gives them.
