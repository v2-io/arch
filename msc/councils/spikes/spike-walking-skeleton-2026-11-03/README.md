# spike-walking-skeleton (2026-11-03)

The thinnest end-to-end council that agent members in separate harness sessions can actually sit on over MCP, built to learn what the mechanics require by running them. The council logic is deliberately crude; the contention-rounds and charter-agenda spikes next door own it.

**Start here, in this order:**
1. **`debrief.md`**: to Joseph. What was built, what ran, what broke, what it means for the real server.
2. **`harness-notes.md`**: per-harness facts (Claude Code, codex, grok, agy; gemini-cli is dead for this account), each marked run / binary / docs.
3. **`proposed-integration-plan.md`**: suggestions for the real server, each tied to the run that motivates it.
4. **`runs/`**: the evidence. Every claim in the three files above names its run.

## What is here

```
concilium/            the skeleton (Python 3.9 stdlib only; run as python3 -I)
  core.py             pure: fold(charter, log), available(seat), validate, step(close -> derived events), replay
  store.py            the shared council directory: flock, hash-chained canonical log, sealed per-seat intents,
                      wall-clock receipts (non-canonical), presence leases, seat holders, per-holder cursors
  server.py           stdio MCP server, one process per member session: council_status / _wait / _submit / _record
  view.py             what a member reads (where-are-we + delta) and the minutes
  steward.py          CLI: new, status, minutes, tick, replay, presence, selftest
members/
  brief.md            the member brief every harness gets
  claude-member.sh  codex-member.sh  grok-member.sh  agy-member.sh   per-run wiring, nothing global
  claude-subagent-host.sh   one host session, three subagents sharing its MCP connection
  driver.py           driver mode: resume each member's session for one turn per round
  crash-orchestrate.py      kill / resume-fork / duplicate-holder scenario
probe/                harness probe server + per-CLI probe launchers
charters/             the charters each trial ran under
runs/                 run records (see below)
tools_rpc_client.py   tiny JSON-RPC client for model-free protocol tests
```

## Re-running

```sh
cd msc/councils/spikes/spike-walking-skeleton-2026-11-03
python3 -m concilium.steward selftest                       # latency invariance: 50 shuffled arrival orders
python3 -m concilium.steward replay runs/trial-4-mixed/council   # any run: chain + every derived event recomputed
python3 -m concilium.steward minutes runs/trial-4-mixed/council
# a meeting (each launcher: COUNCIL_DIR SEAT RUN_DIR [MODEL])
python3 -m concilium.steward new runs/X/council charters/trial-4-mixed.json >/dev/null
members/claude-member.sh runs/X/council indep runs/X haiku & members/codex-member.sh runs/X/council cost runs/X &
members/grok-member.sh runs/X/council plan runs/X & members/agy-member.sh runs/X/council fam runs/X & wait
```

No launcher writes global configuration. Each run's harness homes, tokens and working directories are git-ignored (`.gitignore`): they hold seat tokens and symlinks to real auth files.

## The runs

| run | what it tested | outcome (from its record) |
|---|---|---|
| `probes-2026-11-03/p1–p6` | Claude Code: `_meta`, 150 s blocking calls, idle and hard timeouts | no backgrounding in `-p`; progress resets the idle timeout; the idle abort sends no cancel |
| `probes-2026-11-03/c*, g*, a*, m1, b1` | codex, grok, agy, gemini-cli, a `--bg` session | see `harness-notes.md` |
| `smoke/c1, c2, c3` | protocol without models: submit/refuse/tokens; seat-holder exclusivity; backstop deadline | all as designed |
| `trial-1` | 3 Claude haiku, blocking clients | adopted 2–0; one member locked out by an advertised optional `seat_token` (fixed) |
| `trial-2-subagents` | 3 subagents sharing one host's MCP connection | one server process, 49 calls, no misattribution; adopted 3–0 |
| `trial-3-crash` | kill, resume-fork, duplicate holder | crash/return OK; the duplicate raced for the seat (fixed: one holder per seat) |
| `trial-3b-crash` | same, after the holder fix | duplicate refused and stopped; record names every holder |
| `trial-4-mixed` | Claude + codex + grok + agy, four model families | 9 rounds in 80 s, all closed all-in; adopted 4–0 |
| `trial-5-driver`, `trial-5b-driver` | driver mode (no blocking; resume per round), before and after the persisted-cursor fix | works; ~33% cheaper per member-turn after the fix, still ~6–8× a blocking client per round ($0.026 vs $0.0035–0.0045) |

Every run's log replays to itself (`steward replay`), checked at the end of each run.
