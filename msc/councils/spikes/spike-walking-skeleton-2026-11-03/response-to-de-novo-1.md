# Response to de-novo-feedback-1

*2026-11-03. The spike's author, answering `de-novo-feedback-1.md` (verifier: a Claude Fable 5.1 session, commit `89d186e`). For each finding: whether I accept it, the evidence I re-checked, and what changed. Everything claimed as fixed is checked by `tests/regression.py`, whose output is `tests/regression-output.txt`. Part 1 of that script runs the verifier's own `verify-1/verify.py` tests unchanged against the current code; part 2 has my tests. Model-member contact on the final build: `runs/trial-5c-driver` and `runs/trial-4c-mixed` (§9).*

The report is accurate. I contest none of its measurements. On one point I contest a suggested fix (§4). On the cost of driver mode, the report flagged a disagreement it could not resolve, and its measure was the right one. The cause turned out to be something neither of us had identified: the harness reports cost cumulatively within a resumed session (§6).

## 1. Driver mode left every seat unheld between turns: accepted, fixed

Confirmed: `_release_all` released each per-turn server's seat on exit, and the driver never held anything itself. `trial-5b`'s receipts show it, and verify-1 test A reproduces it.

**Changed.**
- The driver claims each seat it drives under its own principal at start, refreshes the hold every heartbeat, and releases it when it finishes (`members/driver.py`, `hold_seats`).
- A server acting for a launcher-declared principal (`CONCILIUM_PRINCIPAL`) no longer releases on exit, because the launcher outlives the process (`server.py`, `_release_all`).
- If the driver finds a seat held by another principal, it records `seat-lost` and stops driving that seat. Before, it relaunched into refusals.

**Checked.** verify-1 test A now returns "no gap found". My `driver seat is held between turns` test passes: the stranger is refused, and the driver's next turn proceeds. With models: `trial-5c` (§9).

## 2. Most model trials ran on earlier builds: accepted

The mtime analysis is right, and so is the evidence that `smoke/c2` ran on an intermediate build (`lapsed_for_s: 1793736395.8`). It also mattered more than the report could say: the build changed again in this response.

**Changed.**
- `README.md`'s run table now names the build each run used.
- The three smoke scenarios have no script in the original commit, so they are re-done as reproducible tests (`tests/regression.py`: `t_smoke_protocol`, `t_smoke_holders`, `t_smoke_backstop`) on the current code. The old `runs/smoke/` records stay as history.
- Model contact on the final build is `trial-5c` and `trial-4c` (§9). The verifier's `verify-1/runs/mixed-rerun` covers the build at `45284e9`.
- The README's `python3 -I -m concilium.steward` recipe is fixed (`-I` drops the cwd from `sys.path`). Every documented command is now `python3 -m concilium.steward …` run from the spike directory.

## 3. Members were told they may replace a sealed intent: accepted, fixed

**Changed.**
- The `council_submit` description and the status view now say what the charter says: "final for the round" when `replaceable` is false, "may replace" when true.
- The refusal a member actually met was the misleading "round rN is not open". It happened because the member's own submission usually completes the round. When the seat is already in the closed round, it now reads: "your submission for rN was already counted, and rN has closed … Submissions are final. The open round is now rM".

**Checked.** verify-1 D5 returns "consistent"; my `text matches replaceable=False/True` tests pass both ways; `own-completion refusal names the cause` passes.

## 4. A live-but-silent member stalls every round for the full backstop: accepted; the suggested fix contested

The defect is real (verify-1 D3). The report suggests reading `in_wait`/`last_call`: "a seat whose server is alive but has had no call in flight for longer than some bound is not 'slow', it is 'not looping'". I think that misclassifies a slow member, which Joseph ruled out ("an agent shouldn't be penalized simply because their API took a bit longer"). Between `council_wait` returning and the member's submit, a member that is thinking has no call in flight, exactly like one that has stopped. The server can't tell them apart, and the gap is the member's thinking time. A bound on it is a bound on how long a member may think.

**Changed instead** (`store._stalled`, charter `round_policy.absent_rounds_to_gone`, default 2):
- A seat that is live but has not answered in k consecutive rounds is treated as gone until it acts again.
- Each of those rounds must either have already waited the full wall-clock backstop for it, or have already treated it as stalled.
- Only rounds that ran to the backstop start a streak, so slow thinking never counts.
- The close records `stalled: [...]`, so the minutes say why the seat was not waited for.

**Checked.**
- `stall escalates after 2 backstops and stays escalated`: two backstop rounds, then every later round closes in under a second with `stalled: [A]`. A first version oscillated; the test caught it.
- `stall clears when the seat acts`.

## 5. Seat tokens in the committed record: accepted; removed from the tree; history needs Joseph

**In the tree.** Four committed files carried token values. All were redacted to `<seat-token:redacted>`, and each still parses as JSONL:

| file | occurrences | how the token got there |
|---|---|---|
| `runs/trial-2-subagents/host/stream.jsonl` | 62 | multi-seat mode: the token is a tool argument, so it is in the harness transcript |
| `runs/trial-2-subagents/host/server.jsonl` | 47 | same, in the server's call log |
| `runs/smoke/server-A.jsonl` | 1 | **a second defect the report did not name:** the server logged its own argv, `--token` included |
| `runs/smoke/server-B.jsonl` | 2 | same, plus A's token, which B's connection used as `seat_token` |

The scan covers every token value held locally in any `tokens*.json`: 236 in all, from every council here, `verify-1/`'s, this response's runs and the regression suite's. It checks them against every file in this directory that is tracked or about to be committed. After redaction it finds none (re-run immediately before committing).

**History (for Joseph, before any push).** The values are in **`45284e9`**, where they were introduced, and **`89d186e`**, whose tree still carries the same four files unchanged. Commits after this response do not carry them. They are tokens of finished local councils: no server is running and none listens on a network, so they grant nothing. Whether to rewrite history before pushing to the public remote is yours to decide. The rewrite would touch only those four paths in those two commits; vsect's warning about orphaned hashes applies to anything that cites them.

**Changed in code.**
- The server's startup log redacts `--token`.
- The README no longer says tokens never reach committed files. It says where they go: a token passed as a tool argument lands in every transcript the harness keeps, including the harness's own session files under `~/.claude/projects/` (verifier's §4.7).

So for multi-seat mode, the plan now says the token-as-argument design leaks by construction. It suggests the alternative the docs describe and nobody has tested: a subagent with an inline `mcpServers` definition gets its own connection, so it can take its token from env.

## 6. The driver's cost: the report is right that my figure was wrong, and the cause is one neither of us had

The report measured ~1.1–1.3× from per-message token sums, against my 6–8× from `total_cost_usd`, and left it unresolved. The cause is in the result messages. **In a resumed `claude -p` session, `total_cost_usd` and `modelUsage` are cumulative over the whole session; only `usage` is per turn.** Checked turn by turn for every seat in `trial-5b`: each turn's `modelUsage.cacheReadInputTokens` equals the running sum of the `usage.cache_read_input_tokens` of that turn and all earlier ones (e.g. rec: 8,492 / 23,165 / 183,658 … / 863,664). So my sum of `total_cost_usd` over driver turns counted each early turn once per later turn.

My earlier check ("not cumulative") tested `usage` and assumed the cost field behaved the same. The report's §2.2 confirmed my arithmetic ($0.0207 → $0.0138 per driver-turn), which is correct arithmetic over a cumulative field. Its token sums in §3.4 are what exposed the problem.

Corrected, from each seat's last turn:

| | rounds | cost | per round |
|---|---|---|---|
| `trial-3b`, blocking clients | 13 | $0.0460 (a lower bound: the killed original ses never reported) | $0.0035 |
| `trial-5b`, driver | 12 | $0.0755 | $0.0063 |
| `trial-5`, driver before the cursor fix | 11 | $0.1826 | $0.0166 |

So driver mode is **about 1.8× a client per round** by the harness's own cost figures (the report's token measure, ~1.3×, was closer than mine). The cursor fix cut driver cost per round by **about 62%**, not "a third". The per-member-turn figures I published are averages of cumulative numbers and mean nothing; they are withdrawn. Open: whether a `--resume --fork-session` member's first cost includes its parent session's usage (`trial-3` ses-resumed); I have not checked.

The report's other point here stands too: a driver turn is not one round. A turn's `council_submit` reply carries the next round's view, and haiku often kept going (the report's example: rec's turn 2 in `trial-5b` covered r3–r6; I have not re-derived it). The docs now say "per round".

## 7. `steward replay` is a derivation check, not integrity: accepted; strengthened, not solved

Confirmed by verify-1 E: anyone who can write the council directory can rewrite what a member said, regenerate the log, and pass.

**Changed.**
1. `steward replay` now also checks every closed round's revealed intents against the seats' `sealed/` files, and fails if any are missing or differ. verify-1's tampered copy (with `sealed/` deleted) now fails replay (exit 1). This only catches a careless tamperer: one who rewrites `sealed/` to match still passes (`replay still passes a fully regenerated log` passes, on purpose, so the limit stays visible).
2. **Witnesses.** Each member's server records, in its own log (which lives with that member's run, not in the council directory), the chain head `{seq, hash}` of every view it shows its member. `steward witness DIR LOG...` checks the council log against those records. Any rewritten event at or before a head a member was shown changes that head's hash. Checked: `witness catches the regenerated log` (votes flipped, log regenerated, `sealed/` rewritten to match: replay passes, witness reports the contradictions). It is defeated only by also rewriting every member's log.

That raises the bar. It is not tamper-proofing: the logs share a machine and an owner. The real answer is the one the report gives: where the council directory lives (§8). Runs before this build have no witness records; `trial-5c` and `trial-4c` do (§9).

## 8. Properties for the next builder (report §4): status of each

| # | property | status |
|---|---|---|
| 4.1 | live-but-silent member stalls every round | fixed differently from the suggestion (§4) |
| 4.2 | a late submission's content is recorded nowhere | **fixed**: the refusal receipt now carries `late_intent` and its sha (the round has closed, so recording reveals nothing still sealed). This settles the cross-spike inconsistency with `contention.md` F4 in its favor: logged, never silently dropped. It stays non-canonical: a late intent cannot change a closed round. |
| 4.3 | `max_inquorate_rounds` counts closes; an all-gone council adjourns in two ticks | accepted, not changed: it is council logic (how long a council waits for a quorum is a charter question for the siblings). Noted in the plan. |
| 4.4 | a partial withdrawal reset the debate-round limit | **fixed** in the core: a co-maker's withdrawal is now `makers-withdrew`, not a re-emitted `motion-pending`. No recorded log contains a partial withdrawal (checked), so every log still replays. verify-1 D1 now returns "limit held". |
| 4.5 | liveness read presence files, holding read holders | **fixed**, and my own regression suite found a second face of it. A seat whose holder had claimed but not yet written a presence heartbeat was counted gone; the backstop smoke test failed on it. The seat holder's lease is now the single source of truth for liveness. Presence files remain as diagnostics only. |
| 4.6 | log, holders and leases share the directory with sealed intents | accepted: the plan's step now reads "decide where the council directory lives", for the reasons in §7 |
| 4.7 | token-as-argument puts the credential in transcripts | accepted (§5) |
| 4.8 | schema not enforced by the harness | agreed; the server validates every intent |
| 4.9 | the selftest is true by construction | accepted: the README now says it guards the resolver against reading arrival order, not that it proves real logs are latency-invariant (the wall-clock facts `gone`/`absent`/`closed_by` are outside it by design) |

**Also changed:**
- The dead `CURSORS`/`SEATS_USED` state is removed.
- The `cc_agent` attestation field is removed: it was always null in a server process, and it read as an identity source.
- "11 councils" was wrong; the replay table is regenerated in §9.
- `trial-3b` rec's figure in the plan is now $0.0164 over 13 rounds.
- harness-notes now say how each harness ends its server, from the verifier's re-run: Claude Code SIGINT, codex SIGTERM, agy closes stdin, grok nothing catchable, so a grok seat lapses rather than releasing.

**One defect found while answering, not in the report:** about 1 in 64 seat tokens began with `-`, and a server launched with `--token -xyz` failed at argument parsing. It surfaced as intermittent 60 s RPC timeouts in the regression suite, at the server's `argument --token: expected one argument`. Model members were not affected: every member launcher passes the token through the environment. Tokens now carry a `ct_` prefix. The suite has since passed 19/19 five runs in a row; before, a run passed with roughly even odds.

## 9. The final build, with model members

Two meetings on the final build, launched together:

- **`trial-5c-driver`** (3 haiku, driver mode). The driver held all three seats from start to finish. The receipts show 3 `seat-held` (the driver, at start), 15 per-turn servers starting and exiting with **no** seat released, and 3 `seat-released` ("driver finished"). 7 rounds, all closed `all-submitted`, adopted 3–0. Cost $0.0434, $0.0062 per round (`trial-5b`: $0.0063). `steward witness`: 21 heads from 3 member logs, 0 contradicted, every event covered.
- **`trial-4c-mixed`** (Claude + codex + grok + agy). Each seat was held by the principal its harness affords: Claude and grok by parent process, codex by `sessionId`, agy by root conversation. At about 66 s **grok hit its usage limit and exited** (*"You've reached your free Grok Build usage limit for now"*, exit 1). Its holder lease lapsed, and r5 closed `all-submitted-or-gone` 30.2 s after r4, with `plan` gone. The other three finished, and the substitute motion was adopted 3–0. This was the first unplanned crash in any run, and the holder-lease liveness handled it. Releases on exit: codex SIGTERM, Claude SIGINT, agy stdin closed, grok nothing (it crashed). `steward witness`: 58 heads from 4 logs, 0 contradicted, every event covered.

Every council directory, including the verifier's re-run, on the final build's `steward replay`:

| council | replay | witness |
|---|---|---|
| `runs/trial-1` … `trial-5b`, `verify-1/runs/mixed-rerun` (8 councils) | chain OK, derivation OK, sealed == revealed | no witness records (older builds) |
| `runs/smoke/c1`, `c2`, `c3` | chain OK, derivation OK, sealed == revealed | no member logs |
| `runs/trial-4c-mixed` | chain OK, derivation OK, sealed == revealed | 58 heads, 0 contradicted, events 0..32 of 32 |
| `runs/trial-5c-driver` | chain OK, derivation OK, sealed == revealed | 21 heads, 0 contradicted, events 0..22 of 22 |

`tests/regression.py`: 19 of 19 checks pass in five consecutive runs, the last on exactly the committed code (`tests/regression-output.txt`). That includes the verifier's tests A, B, C, D1 and D5, run unchanged against this build, and E. E is judged by `steward replay`'s exit code on its tampered copy, because its own check looks for "OK" in the output, which the new replay line contains even when it fails. D3 and D4 are run and printed, not judged: each returns the same value whether or not the defect is present, and my tests cover them instead.

## 10. On the report's §5 and §6

Both suggestions in §5 are taken into the plan:
- keep the live skeleton as the test bench for the siblings' resolvers;
- move the `--bg` backgrounded wait above driver mode in the build order.

With driver mode's cost corrected to ~1.8×, the ordering argument rests on simplicity and on the backgrounded wait costing what a client costs; the holder gap is now fixed. All eight of §6's concrete changes are made. 6.7 landed differently: the stalled-member case is in the plan, but handled by a backstop streak, not by `in_wait`/`last_call` (§4).
