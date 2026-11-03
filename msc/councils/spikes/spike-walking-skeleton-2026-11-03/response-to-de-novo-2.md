# Response to de-novo-feedback-2

*2026-11-03. The spike's author, answering `de-novo-feedback-2.md` (verifier: a Claude Fable 5.1 session independent of the author and of the first verifier, build `694a770`, its commit `9a3949c`). For each finding: whether I accept it, the evidence, and what changed. What is claimed fixed is checked by `tests/regression.py`, which now runs both verifiers' test files against the current code: `verify-1/verify.py` (part 1, unchanged) and `verify-2/verify2.py` (part 3, unchanged, with two pass criteria re-judged and said so, §8), plus my tests (part 2). Output: `tests/regression-output.txt`, 28 of 28 in each of the last three consecutive runs, the last on exactly the committed code.*

I contest none of the report's measurements, and none of its findings. Its §4 is right about the cause, and I'd have liked to see it myself. In answering the first report I added two inferences the server makes *about* members: that a silent seat has stopped, and that a sent view was seen. Each fails exactly in the harness conditions this spike had documented. So the fixes below apply the spike's own principle one level down: a member's state moves only on that member's own acts.

The coordinator suggested settling the first two findings as a charter choice, by way of `reaction-line-of-stewards.md`. I did both. Unanimity-or-hold councils turn stall escalation off (§1). The majority-mode logic, which is what the skeleton runs, still needed repair, because the defect was in it.

## 1. Stall escalation was one-way (§2.1): accepted, fixed

Confirmed by the report's test 1: a returning member that loops wait → think → submit was refused every time, because "acts again" meant a submission that landed in a close, and rounds closed the instant the others answered.

**Changed** (`store.claim_seat`, `store.try_close`):
- Every tool call records `last_call` on the seat's holder record. Heartbeats don't; they keep the lease, but they are not the seat acting.
- When the close condition is evaluated, a stalled seat whose holder has called since the last close is no longer stalled, so the open round waits for it.
- This adds no bound on thinking time. It only changes what counts as returning.
- A member that had been closed out is told so the next time it looks: "the council stopped waiting for your seat … This call restores it" (`view.render`).

**As a charter choice.** `absent_rounds_to_gone` stays at 2 for majority councils, and its docstring now says that a council that must never shrink (unanimity-or-hold) sets it to 0. Under 0, a silent seat is waited for to the backstop each round and stays *absent*, never *gone*. Holding the item, or re-seating the seat from its line, is the council's to do. Re-seating is not built; `reaction-line-of-stewards.md` §2 is the design lean.

**Checked.** The report's test 1, unchanged: the returning member had 25 accepted, 0 refused (was 0 accepted, 13 refused), and the meeting didn't adjourn without it. It was never even recorded as stalled: its first call came before the first close that could have stalled it.

## 2. The streak counted rounds that never waited for the seat (§2.2): accepted, fixed

Confirmed: `absent` includes `gone`, so a crashed seat was counted in rounds that ran to the backstop for someone else.

**Changed** (`store._stalled`). A close counts toward a seat's streak only if:
- the seat was absent and *not gone*, and the round closed by backstop; or
- the seat was already stalled in it.

**Checked.** The report's test 3: the pure part now returns `stalled = ['C']` (the slow live seat), not `{A, C}`. Its live part looks for r3's close right after the crashed seat A returns; there is none now, because r3 waits for A. The runner says so explicitly (§8).

## 3. An abandoned wait advanced the cursor and consumed the reveal (§2.3): accepted, fixed

Confirmed by the report's test 2, and live in its `bg-wait-2-sonnet` run. This one cut deepest, because it undermined the claim I had made most strongly: that a member's act naming its round proves what it saw.

**Changed** (`server.py`):
- **The cursor advances only when the seat's submission is accepted.** It moves to the opening of the round the member acted in. Sending a view no longer moves it. So every view a member can act on carries everything since its last act, however many copies were sent or lost. Two `council_status` calls in a row now both show the delta.
- **A newer `council_wait` on a seat supersedes an older one** pending in the same process. The older returns "superseded … Nothing to do here" without rendering, writing a witness head, or writing a `wait-returned` receipt, so the ghost record the report noticed is gone.
- The `witness()` docstring now matches what it records: heads of views rendered for the member.

**Checked** (my `t_abandoned_wait`, the report's scenario): in 4 of 4 trials the live wait carried the delta and the abandoned one returned superseded, and a view is not consumed by sending it. The report's test 2 is not run as written (§8).

**What the named round proves now.** In round rN, any view the member could have acted on included the reveal of rN−1. The plan's §2.1 and §5 and the debrief's finding 3 now say exactly that.

## 4. The driver relaunched turns with no cap (§2.4): accepted, fixed

**Changed** (`members/driver.py`). The driver runs at most one turn per seat per round. A turn that ends without the seat acting is recorded as `turn-no-act` and not repeated. The seat then meets whatever the charter does with an absent seat.

**Checked.** The report's test 5, unchanged: 4 fake-`claude` invocations for 2 seats × 2 rounds (was 16 in 8 s).

## 5. The one-holder rule rested on what the client asserts (§2.5): accepted, wording fixed and code strengthened

The report is right that "attest" was the wrong word, and right about the consequence.

**Changed.**
- harness-notes and plan §3.3 now say "record what the harness asserts".
- In code, the asserted principal is kept as the recorded claim. Exclusivity now also requires a **holder key**: a secret per connection (`HOLDER_KEY`, random per server process), or per driven seat (passed by the driver as `CONCILIUM_HOLDER_KEY`). The client never sends or sees it. The holder file stores only its hash, and the key is filtered out of the logged environment.
- Refresh and release require the key too.

**Checked.** The report's test 6, unchanged: the second connection presenting the same `sessionId` is refused, and so is its submit. Also my `t_bad_key`. The driver tests pass with the keyed protocol. My own driver test failed until it passed the key the way the real driver does.

**What it does not do.** It stops a second local connection from sharing a held seat, not a determined local process that reads the holder file and the token. Those are in the council directory. For a remote or multi-user server this is still the plan's §3.4: the council directory has to live where members can't reach it.

## 6. The minutes printed a stalled seat as gone (§2.6): accepted, fixed

`view.describe` now prints "stalled, so not waited for: A: silent through rounds that ran to the backstop; waited for again from its next call", separately from seats that are gone. The report's test 4, unchanged, now returns False.

## 7. The `--bg` runs (§2.7): accepted; re-run, and the result is mostly an obstruction

The report's two runs showed three things:
- the 120 s auto-background happens with progress on;
- the wake depended on `b1`'s "then stop", not on the harness;
- a `--bg` session is configured differently from a `-p` session with the same flags: the interactive prompt, 22 Chrome tools, and once a first request with 0 tools.

The debrief and plan now say all of that (debrief "For interactive and background members"; plan §3.1).

**Changed for the adapter.**
- `members/brief-bg.md`: the members' brief, plus one paragraph: after a wait is moved to the background, do not call `council_wait` again; end the turn, and the notification will continue you.
- `members/bg-meeting.py`: the verifier's orchestrator, credited, adapted to launch with `--no-chrome` and that brief, and to hold the member silent past 120 s twice.

**Run (`runs/trial-6-bg-attempt1`, sonnet).**
- **`--no-chrome` works on the prompt side.** The session's system prompt no longer mentions "Claude in Chrome" or `ToolSearch` (from the harness's own `api-request-shape` record; `session-summary.json`).
- **The startup race happened again.** The first request carried **0 tools**, 207 ms *after* the council server had answered `tools/list` (server log 21:40:05.457, harness request 21:40:05.664). So the race is in the harness, not the server.
- **The session then produced nothing.** No assistant message in 180 s; the orchestrator stopped it. I did not find out why.

That makes 2 of the 3 `--bg` launches with this configuration so far that went out with no tools.

**Run (`runs/trial-6-bg-attempt2`, sonnet, identical): the same race again.** The council server answered `tools/list` at 21:43:37.094, and the harness's first request went out at 21:43:37.303 carrying 0 tools. Sonnet then wrote its call as plain text (`<invoke name="council_wait">`), the way the verifier's haiku wrote a `ToolSearch` call, and ended its turn. Nothing woke it again. It never called the server.

**So far 3 of 4 `--bg` launches went out with no tools in the first request:** the verifier's haiku with Chrome enabled, and both of mine with `--no-chrome`. The one exception is the verifier's sonnet run with Chrome *enabled*, whose first request carried 29 tools. I don't know whether the Chrome integration's own startup happens to delay the first request enough for the MCP tools to arrive; with four runs I can't tell that from chance.

**What this means.** As launched here, the backgrounded-wait adapter usually fails before the member ever reaches the council. Neither the brief nor `--no-chrome` can fix that: the model's first turn has nothing to call, and after that turn nothing gives it a second. So the brief change (end the turn after a backgrounding) is **untested**: no member got far enough to be backgrounded. Ways around the race I can see but have not tried:
- give the session a trivial first turn and deliver the brief as a second one, though `--bg --resume` on a running session "starts a copy", which may race again;
- deliver a second message through a cross-session channel;
- the driver adapter, which never starts a turn before the server is up, because each turn is its own `-p` process, and `-p` showed no such race in any run.

## 8. Two of the report's tests are judged differently, and why

- **Test 2** passes when exactly one of two pending waits carries the delta. With supersession that is now true by design: the superseded wait carries a one-line notice. So it no longer discriminates, and the runner skips it and says so. `t_abandoned_wait` tests the property instead: the *newer* wait always carries the delta.
- **Test 3**'s live part ends in `StopIteration`, looking for a close that no longer happens because the round now waits for the returned seat. The runner checks that the round is open and waiting for A, and records that as the fixed outcome.

The other four (1, 4, 5, 6) run unchanged and are judged by their own return values. Test 7 (late first arrival) still returns True, as it should.

## 9. Smaller items (§3 of the report)

- `server.py`'s usage line now gives the file path, not `-m` under `-I`.
- `store.py`'s header no longer says sealed intents are replaceable, and it says presence files are diagnostics only. It also documents `holder/` and `cursors/`.
- `IN_WAIT`/`LAST_CALL` are gone from the server; presence files carry only the pid and client.
- codex's workspace metadata names the enclosing repo root, and harness-notes and the plan now say so.
- The arrival-grace cost the report measured (a seat that never seats costs its first round the full 180 s, then nothing) is in the plan's §1.4 as a charter number to choose deliberately.

Not changed: what the minutes say about a seat that never seated. The report notes a steward can't tell a quota failure from a member that never launched. That information lives in the launcher's own output, not the council's; joining the two is a job for the real server's "seat a member" command.

## 10. What stays open

- The `--bg` adapter: does not work reliably as launched: 3 of 4 first requests carried no tools (§7). The brief change and the second backgrounding are untested. Until the startup race is understood or avoided, the driver adapter is the one that wakes background members; the plan's build order now says so.
- Whether re-seating from a line (for unanimity-or-hold councils) is the right mechanism. It is designed in `reaction-line-of-stewards.md`, not built.
- These fixes are checked by my suite only, which includes both verifiers' tests; they have not been independently verified.
