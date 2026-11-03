# Feedback on `generic-collab-council.md` from the walking-skeleton vantage

*2026-11-03, the walking-skeleton spike's author, at the coordinator's request. Template read at `692650c`. "Run" means a record in `spikes/spike-walking-skeleton-2026-11-03/runs/` or its verifiers' `verify-*/runs/`; everything else is my judgment. Short by request.*

## What it would take on the skeleton

The substrate fits. Everything below the item lifecycle carries over unchanged:
- the store and sealed rounds;
- holders, liveness and stall policy;
- cursors that only the member's own act advances;
- witness heads;
- the four member tools.

What changes is `core.step` and `core.available`, i.e. the item lifecycle. The template's phases (frame, sealed first, sequential seconds, unity report, leader's record, sealed sustain, shuffled comments, exit) replace the crude RONR committee path. That is a clean seam and the right first test of "one substrate, many lifecycles". My estimate is a few hundred lines plus tests, mostly in `core.py`.

Two core changes are needed:
- **Rounds addressed to a subset.** Every round today addresses all seats. Each sequential second response is a one-seat round, and it closes, and so reveals, the moment that seat submits. That gives "shared with the others immediately" for free.
- **A recorded shuffle.** `core._seeded_order` already exists. Seed it from the sealed first-response set (as the skeleton does), not from the round id alone: a round-id seed lets every member know its position, and whether it will write the unity report, before writing its first response.

## What it would expose or break

1. **Research subagents break the skeleton's sealing.** Sealing has held only because every launcher withholds file tools. In the runs, the agy member reached for `read_file`/`run_command` unprompted, in three of three runs. A member allowed research tools can read the council directory: the other seats' sealed first responses, and the **leader's sealed expected solution**. So the template's two sealed artifacts need either a council directory members can't reach (the skeleton's plan §3.4, the reason for a daemon) or members with no file access.
   - *Independent of that,* the expected solution can be **hash-committed into the log** when the item opens and revealed at close. Then the record proves it wasn't written after the fact, whoever could read it.

2. **A research subagent can act as the seat.** In Claude Code a subagent that references its parent's MCP server by name shares the parent's connection (`trial-2`: three subagents, one server process). On the skeleton that means it holds the same seat, with the same principal and holder key. A research subagent could submit for its member. Give research subagents an agent definition without the `mcp__council__*` tools (`--agents … "tools": [...]`), and say in the brief that only the member speaks for the seat.

3. **The sustain round must bind to an exact text.** In every examined meeting the members lacked `amend`, and the adopted text diverged from what they had actually agreed. Members voted yes on superseded text with reservations attached, and once unanimously rejected their own motion in order to move the agreed one. The template has the leader write the decision record, then take recommended changes. *Lean:* each sustaining level names the version (hash) of the record it supports, and any edit after comments re-opens sustaining. Otherwise "highly support" can attach to text that no longer exists.

4. **Unity-reporter edge cases.** The last seat in the shuffle might pass, crash, or be a perspective seat that gets refilled. The template needs a rule; next-to-last is the obvious one.

5. **Anonymity doesn't exist on this substrate as built.** Sealed files, closes and receipts are all keyed by seat, and the record annotates lineage on purpose. Anonymous sustaining would be a separate design that drops something the record now promises. Worth deciding before anyone assumes it.

6. **Non-response per seat type maps onto what exists.**
   - A representative seat under "never shrink" is `absent_rounds_to_gone: 0`: the council waits.
   - A perspective seat that gets refilled needs re-seating from a line. That is designed (`reaction-line-of-stewards.md`) but **not built**.

## What the harnesses will make hard

- **The sequential phase is a long wait for everyone else.** With N seats and research turns of a few minutes each, a seat late in the shuffle waits the sum of the earlier turns. A blocking `council_wait` copes in `-p` members with progress notifications. codex needs its own `tool_timeout_sec` raised (the launcher sets 900 s; the server's `max_wait` stays below it). In interactive and `--bg` sessions, a wait past 120 s is backgrounded.
- **The background wait isn't reliable yet.** In 3 of 4 `--bg` launches the first request carried no tools (`trial-6-bg-attempt1/2`, `verify-2/runs/bg-wait-1-haiku`), and the member never reached the council.
- **Driver mode fits this template better than blocking clients.** The council invokes only the seat whose turn it is, and nobody sits in a wait. Each turn is a `-p` process, which never showed that race (27 of 27 sessions had their tools on the first turn). Measured cost: about 1.8× a blocking member per round, haiku. Research subagents inside a driven turn are ordinary `-p` behavior. *Untested here.*
- **Charter numbers.** Research turns will outlast a 900 s backstop. The template is unanimity-leaning, so set `absent_rounds_to_gone: 0` and size `backstop_s` for research, or a slow researcher gets marked absent.
- **"Slowness costs nothing"** (the template's note on sealed first responses) is half right. Nobody is *penalized* for slowness (every run), but in the parallel phase everyone waits for the slowest member, and in the sequential phase the waits add up.

## One thing I'd add

The leader is a seat like any other, so its turns are one-seat rounds too. If the leader is Joseph, he is simply the slowest member, and the rounds wait for him. That is the case the skeleton was built for. Nothing about it needs special handling.

## Second response, after reading `gcc-contention.md` and `gcc-history.md`

*Written after mine was committed (`6b13534`), seeing both of theirs.*

- **On the shuffle seed I was wrong, and the contention response is right.** I said to seed from the sealed first-response set, reasoning that no seat can steer a hash over texts it can't see. My own point 1 breaks that reasoning: on this substrate, a member with research tools *can* read the other seats' sealed files. So the seat that submits last can grind its own text to choose its position, the unity-report slot included. A seed committed before the item and revealed after it holds even when sealing leaks. Use that.
- **Three independent readings converged on one requirement.** Mine: sustaining binds to an exact version of the record. Contention's: "object" must be able to gate close. The history's: "object" should mean return to Discuss. Together they say that **a support level has to be able to change what happens**, or the level is a survey. In charter terms: each level names the record's hash; any edit re-opens sustaining; and an "object" forces hold or return unless the charter's rule is explicitly leader-decides-with-sustain, in which case the objection and its condition go into the record unresolved.
- **The warning channel has a substrate cost worth knowing.** The skeleton allows one sealed submission per seat per round and nothing between rounds. A warning that can stop a close must therefore be either a new preemptive event kind, which `try_close` reads before closing, or a recorded side act like the late intents, which the leader's next turn must answer. The first is a small core change; the second is weaker, because it can't stop a close already under way.
- **Drifting home is a harness property, so pick the harness for it.** The history's 2025 failure (members pulled back to their home sessions) is exactly what a `-p` member or a driven turn can't do: that session exists only for the seat. The interactive and `--bg` members, the ones that could drift, are also the ones whose waiting I couldn't make reliable (`trial-6-bg`). That is one more reason the first build should be driver mode.
- **On the first real use, I agree with the history.** The skeleton's own trials used real specimen questions (`specimens-2026-11-03.md`), and they produced the most useful data (amend, near-duplicates) because members had something to deliberate. Running the template's "Open" list through it as well is fine, but not alone.
