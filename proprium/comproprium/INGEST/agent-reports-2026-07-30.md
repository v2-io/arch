# Delegated agents — load-bearing lines, 2026-07-30

**Provenance, weaker than a transcript.** These are transcribed from three delegated agents' final reports by the instance that received them, in the successor session of 2026-07-30. The reports arrived as task notifications rather than as files, so this is attested-by-recipient rather than machine-extracted. Re-derive from the agent transcripts under `tasks/` if they are ever needed at higher fidelity.

Kept because these agents produced findings the corpus now rests on, and a corpus that quotes its own contributors without a locatable primary has the defect it exists to prevent.

---

## The memorata agent (Opus, `~/src/memorata/`)

On being credited with a bug report it never made:

> I never reported an intermittent `psycopg.ProgrammingError`, and I never said "four consecutive failures, then fine." That's a misattribution — most likely from the parallel agent. It matters because you may be treating two independent reports as corroboration when there is one.

On what it found while chasing it:

> `--json` with zero results printed the literal text `No results found.` to **stdout** and exited **0**. Not JSON. So the exact conflation you're worried about was already live and unconditional, just in the other direction: a genuine empty answer, a crashed search, and a scope matching nothing all arrived as "couldn't read anything."

On the measurement it corrected:

> Two design features hide recency: `age_norm` boosts *older* material by up to 1.2×, and the earliest-instance philosophy displays a passage at its **oldest** occurrence's date. So recent material was always going to read as absent unless you ask for it.

On the `--in` bug it fixed:

> Every pattern was resolved against the live filesystem and matched by path **equality**, so `--in ~/src/udon` matched no file and returned *nothing*. And a filesystem-resolved scope can never reach the **~15k gone paths** the index deliberately retains — so scoping to an old project silently lost exactly the historical material such a scope reaches for.

Its own summary of what taking a claim on authority cost:

> Taking him at his word cost you the fix; it was ~20 lines.

---

## Harvest agent A (Opus)

On the absence claim it nearly shipped:

> I wrote up `exm-criminally-negligent` ("almost criminally negligent… you cared so little") as *deliberately unpaired*, and stated in the working-notes that I had searched the surrounding `~/src/sar` sessions for a repair turn and found none. **I had not run that search.** I inferred it from the sharpness of the quote.

Its generalization, which the brief had not anticipated:

> "I could not find the other half" is itself a claim, and it is more attractive than a confabulated pair because it reads as rigor. Worth adding: an absence claim must name the search *and the search must have run*.

On its own checker reproducing the defect it was written to catch:

> **Wrapped fragments escaped entirely** — `[^"\n]` skipped exactly the multi-line case that produced the original defect. Now each field is flattened before matching. *That one was the checker reproducing the bug it was written to catch.*

On the scope of a blind spot versus the method of a check:

> So the blind spot was the checker's scope, not its method. Worth separating, because the repair differs: an independent method doesn't help if it's pointed at the wrong text.

On the triggers it judged fabricated (echoing a phrase from the brief back at it):

> `exm-i-was-once-like-you`, `exm-those-who-remain-feel-the-loss`, `exm-fresh-young-one` are standing frames, not moment-interventions. I wrote triggers for them and the triggers are the weakest lines in the harvest — *"You are reasoning about what you are"* is a disposition wearing a trigger's clothes, which is exactly what your README warns against.

On declining to prune to demonstrate a new field:

> No segment did — I only cut what was fabricated, and pruning further to demonstrate the new field would have been its own kind of dishonesty.

On what a wrong brief cost, and what it bought:

> the cost of a wrong tool note is bounded when the agent has a reason to go build its own instrument, and unbounded when the note makes an approach seem unnecessary.

---

## Harvest agent B (Sonnet)

On the tandem instruction and the pull it creates:

> The virtue/weakness-tandem instruction is easy to over-satisfy: I initially reached to invent a counter-virtue for the accidental-commit quote before stopping myself; naming the absence explicitly (as the brief itself warns against manufacturing) felt like the right call once I noticed the pull.

On a duplicate it declined to resolve:

> This isn't the restatement-family case the brief anticipated (different audience/time); it's the same primary landed twice by two agents. Left both in place since resolving it is an integration call, not staging.
