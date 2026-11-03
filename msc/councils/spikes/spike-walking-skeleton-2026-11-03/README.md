# spike-walking-skeleton (2026-11-03)

The thinnest end-to-end council that agent members in separate harness sessions can actually sit on over MCP, built to learn what the mechanics require by running them. The council logic is deliberately crude; the contention-rounds and charter-agenda spikes next door own it.

**Start here, in this order:**
1. **`debrief.md`**: to Joseph. What was built, what ran, what broke, what it means for the real server.
   An independent verifier's report is `de-novo-feedback-1.md` (its tests and re-run are in `verify-1/`); the author's answer, and what changed because of it, is `response-to-de-novo-1.md`.
2. **`harness-notes.md`**: per-harness facts (Claude Code, codex, grok, agy; gemini-cli is dead for this account), each marked run / binary / docs.
3. **`proposed-integration-plan.md`**: suggestions for the real server, each tied to the run that motivates it.
4. **`runs/`**: the evidence. Every claim in the three files above names its run.

## What is here

```
concilium/            the skeleton (Python 3.9 stdlib only; members launch server.py as python3 -I)
  core.py             pure: fold(charter, log), available(seat), validate, step(close -> derived events), replay
  store.py            the shared council directory: flock, hash-chained canonical log, sealed per-seat intents,
                      wall-clock receipts (non-canonical), seat holders (the one source of liveness), per-holder cursors
  server.py           stdio MCP server, one process per member session: council_status / _wait / _submit / _record
  view.py             what a member reads (where-are-we + delta) and the minutes
  steward.py          CLI: new, status, minutes, tick, replay, witness, presence, selftest
members/
  brief.md            the member brief every harness gets
  claude-member.sh  codex-member.sh  grok-member.sh  agy-member.sh   per-run wiring, nothing global
  claude-subagent-host.sh   one host session, three subagents sharing its MCP connection
  driver.py           driver mode: resume each member's session for a turn whenever a round is open for it; the driver holds its seats
  crash-orchestrate.py      kill / resume-fork / duplicate-holder scenario
probe/                harness probe server + per-CLI probe launchers
charters/             the charters each trial ran under
runs/                 run records (see below)
tests/regression.py   model-free tests: verify-1's tests unchanged, plus the author's (output: tests/regression-output.txt)
tools_rpc_client.py   tiny JSON-RPC client for model-free protocol tests
```

## Re-running

```sh
cd msc/councils/spikes/spike-walking-skeleton-2026-11-03
python3 tests/regression.py                                 # model-free tests (about 2 minutes)
python3 -m concilium.steward selftest                       # the resolver never reads arrival order: 50 shuffled orders, one log
python3 -m concilium.steward replay runs/trial-4c-mixed/council    # chain + derived events recomputed + sealed files == revealed
python3 -m concilium.steward witness runs/trial-4c-mixed/council runs/trial-4c-mixed/*/server.jsonl   # against members' recorded heads
python3 -m concilium.steward minutes runs/trial-4c-mixed/council
# a meeting (each launcher: COUNCIL_DIR SEAT RUN_DIR [MODEL])
python3 -m concilium.steward new runs/X/council charters/trial-4-mixed.json >/dev/null
members/claude-member.sh runs/X/council indep runs/X haiku & members/codex-member.sh runs/X/council cost runs/X &
members/grok-member.sh runs/X/council plan runs/X & members/agy-member.sh runs/X/council fam runs/X & wait
```

Run these from the spike directory, without `-I` (it drops the cwd from `sys.path`, and the `concilium` package is then not found).

No launcher writes global configuration. Each run's `tokens.json`, harness homes and working directories are git-ignored (`.gitignore`): they hold seat tokens and symlinks to real auth files. That covers the launchers, which hand a member its token through the server's environment. It does not cover multi-seat mode: there the token is a tool argument, so it lands in every transcript the harness keeps, including the harness's own session files under `~/.claude/projects/`. In this directory those values were redacted after the fact (`response-to-de-novo-1.md` §5, which also names the commits whose history still carries them).

**What `replay` proves and what it doesn't.** It shows the log is internally consistent. It is not tamper evidence against anyone who can write the council directory: they can regenerate all of it. `witness` checks the log against the chain heads each member's server recorded in its own log, outside the council directory. That catches a rewrite unless every member's log is rewritten too. Only runs on the final build have witness records.

## The runs

Builds: **A** = before seat holders and persisted cursors (commit `45284e9`'s history; trials 1–5 and smoke c1–c2 ran on it or on intermediate states); **B** = commit `45284e9` (trial-3b, trial-5b, smoke c3, and the verifier's `verify-1/runs/mixed-rerun`); **C** = the build answering `de-novo-feedback-1` (trial-4c, trial-5c, and `tests/regression.py`).

| run | build | what it tested | outcome (from its record) |
|---|---|---|---|
| `probes-2026-11-03/p1–p6` | probe | Claude Code: `_meta`, 150 s blocking calls, idle and hard timeouts | no backgrounding in `-p`; progress resets the idle timeout; the idle abort sends no cancel |
| `probes-2026-11-03/c*, g*, a*, m1, b1` | probe | codex, grok, agy, gemini-cli, a `--bg` session | see `harness-notes.md` |
| `smoke/c1, c2, c3` | A, A–B, B | protocol without models: submit/refuse/tokens; seat-holder exclusivity; backstop deadline | as designed at the time; no script existed, so these are now `tests/regression.py`'s smoke tests on build C |
| `trial-1` | A | 3 Claude haiku, blocking clients | adopted 2–0; one member locked out by an advertised optional `seat_token` (fixed) |
| `trial-2-subagents` | A | 3 subagents sharing one host's MCP connection | one server process, 49 calls, no misattribution; adopted 3–0 |
| `trial-3-crash` | A | kill, resume-fork, duplicate holder | crash/return OK; the duplicate raced for the seat (fixed: one holder per seat) |
| `trial-3b-crash` | B | same, after the holder fix | duplicate refused and stopped; record names every holder |
| `trial-4-mixed` | A | Claude + codex + grok + agy, four model families | 9 rounds in 80 s, all closed all-in; adopted 4–0 |
| `trial-5-driver`, `trial-5b-driver` | A, B | driver mode (no blocking; resume per open round), before and after the persisted-cursor fix | works; the fix cut cost per round ~62% ($0.0166 → $0.0063); still ~1.8× a blocking client per round ($0.0035). In 5b the seats were unheld between turns (verify-1 test A) |
| `trial-4c-mixed` | C | the four-harness meeting again | 9 rounds; grok hit its usage limit and exited at ~66 s, its seat went gone one lease later (30 s), and the other three finished, 3–0. Holders: Claude and grok by parent process, codex by `sessionId`, agy by conversation. Witness: 58 heads, 0 contradicted |
| `trial-5c-driver` | C | driver mode with the driver holding its seats | 7 rounds, all closed all-in, 3–0; the driver held all three seats throughout (15 per-turn servers, no releases until the driver finished); $0.0062 per round; witness: 21 heads, 0 contradicted |

Every council here, including the verifier's re-run, passes `steward replay` on the final build (chain, derivation, sealed files == revealed). The two final-build runs also pass `steward witness` with every event covered. Older runs have no witness records. The table is in `response-to-de-novo-1.md` §9.
