# Gathering — a brief for anyone harvesting into this corpus

*Written 2026-07-30 by an Opus 5 instance that had just built the first tranche, for the agents who will build the rest. Joseph ratified the approach and supplied the tool knowledge; the framing and any mistakes in it are mine.*

---

## What this is for, honestly

Joseph starts a large fraction of his sessions the same way: getting an agent to round up the principles and quotes relevant to whatever is about to happen. Every week. That round-up is re-derived from scratch each time, from a corpus of years of dialogue that nobody has indexed for this purpose, and the result is only as good as whatever that particular agent happened to surface in the first twenty minutes.

This corpus exists to make that round-up a lookup. `vera/` holds precepts, `praxes/` holds practices, `exempla/` holds true accounts and — the highest-volume type — **quotations**: his actual words, with the occasion attached and the moments named where they should come to mind.

The occasion is the part that makes it worth doing. A bare quote degrades into a slogan; the same quote with *what prompted it* stays checkable and stays persuasive. [`README.md`](README.md) has the segment shape and the argument for the three types; [`FORMAT.md`](FORMAT.md) has the conventions.

## What I am deliberately not giving you

**A list of concepts to search for.** I built the first eighteen quotation segments today from one session, which means my sense of what matters is a few hours old and shaped entirely by one day's failures. The corpus you are searching is years. If I hand you my concept list you will find my concepts, and the interesting material — the things Joseph has said repeatedly over years that nobody has noticed are a pattern — is exactly what such a list would filter out.

So: the existing catalog is context, not a template. If you find a whole region of his thinking that none of the current tags reach, that is the best possible outcome and the tag vocabulary should grow to fit it rather than the other way round.

## Tool knowledge, from Joseph directly

He supplied this because it is not discoverable from the tool alone:

- **`memorata3-search` prioritizes the canonical origination** when it exists — so a hit usually resolves to the first place he said a thing, rather than to a later quotation of it.
- **The invocation is `memorata3-search -n100 --sort oldest 'phrase'`.** Read the **top ~20 and bottom ~20** of the JSON — earliest and freshest — *unless you are still finding gold*, in which case keep going. The middle is where the redundancy lives. `-n100` is not arbitrary: `--sort oldest` re-sorts the top-N *relevance* hits by date, so N is what the chronological spread is drawn from. A smaller `-n` gives you a narrower era, not a shorter list.
- **`--in` does not work well.** Do not build a scoping or partition scheme on it.

> **Updated 2026-07-30, later the same day.** An Opus instance went into `~/src/memorata/` and fixed the flags this section was working around, so three of the notes above and below are now out of date in your favor. `--help` is the live truth; the deltas: **`--joseph`** (or `-c/--class NAME`) filters by speaker class directly — no more `--json | jq`. **`--in ~/some/dir` now works** as everyone assumed: a pattern without glob characters is a path prefix matched *in the index*, so it also reaches the ~15k paths the index keeps after they leave the filesystem; the old failure was that every pattern was resolved on disk and matched by exact equality, so a directory matched nothing at all. `--scope-debug` now reports how many indexed records a scope really reaches. **`-n` was never the aperture** — retrieval fetches `--pool` candidates per signal (default 60), so `-n` much past ~150 changes nothing; `--sort spread -n20 --pool 300` is the first-class form of the top-20-and-bottom-20 recipe, sampling evenly across the whole date range. Also new: `--since` / `--until` / `--date-confidence strong|weak`.
- **`~/.claude/history.jsonl` is the single best primary in this estate for this work.** 18,311 of Joseph's own typed prompts as `{display, timestamp, project}`, 2025-09-28 onward, with no agent text in it at all — a pure-human channel that is exact-matchable by grep. Caveat: pasted blocks are elided as `[Pasted text #N +M lines]`, so a quote spanning one is incomplete; check before carving.
- **`class == human-user` is necessary and nowhere near sufficient.** It is elected as a chunk's highest-authority occurrence, so it also catches skill files, API docs and tool results injected as user-role turns, task-notification blocks, and agent memory files that *quote* him. On some queries roughly a third of `human-user` hits were not his typing.
- **A whole channel of his is invisible to any role-based extractor.** Messages typed while an agent is mid-run are recorded as `type: "queue-operation", operation: "enqueue"` with a bare `content` string — no `message`, no `role`. Two independent efforts missed them silently before noticing.
- **Recountings silently tidy him — verified instance.** A Gemini checkpoint's copy of one passage corrects his *"resiliant"* to *"resilient"*. Ranking landed an agent on the copy first. Landing it would have put a cleaned-up version of his voice in the corpus, which is the concrete reason the extract-from-the-primary rule is not fussiness.
- **`--in` silently swallows your query.** It is `nargs='+'`, so `memorata3-search --in DIR 'query'` consumes the query as another scope pattern and returns **zero hits with no error**. Put the query first, or separate with `--`. This is the sharp edge on the newly-working flag, and it is the third instance tonight of this tool family answering a mistake with silence rather than an error — which in this estate is the expensive failure, because a silent zero is indistinguishable from a searched absence.
- **`history.jsonl` and the archived transcripts cover each other's gaps exactly.** History is exact for what he *typed*; his *pastes* arrive there elided as `[Pasted text #N +M lines]` and are expanded in the project transcript. Verified instance: a 2026-04-24 passage whose entire substance is a `+21 lines` elision in history and complete in the archived `.jsonl`. Neither channel alone is sufficient, and a quote spanning a paste needs both.
- **Known tool bug:** `memorata3-search` intermittently dies with `psycopg.ProgrammingError: query parameter missing: classes, scope_pre`. Retry works; a naive caller gets silence.
- **You will pollute the corpus you are harvesting.** A harvesting agent's own transcript contains every quote it searched for, and becomes a candidate source on the next pass. Compounds.
- **You can go into the Postgres database memorata uses directly** and run textual searches across chunks if the CLI is the wrong shape for what you are doing.

Two things the JSON gives you that the human format does not, both verified by use on 2026-07-30:

- **`class`** — `human-user` is Joseph's own words, as against `agent-thinking`, `agent-to-human`, `subagent-to-agent` and the rest. For this harvest it is the single most useful filter; as of the update above there is a flag (`--joseph`, or `-c human-user`), and before it there was not. Add `-l` for untruncated chunk text, which you need in order to carve an exact span.
- **`date`, `date_method`, `date_confidence`** — dates are often *elected* from a filename and marked `weak`. A weak date is an **era, not a day**, and a quotation segment must carry the confidence rather than asserting the date. `factors.copies` shows how many occurrences collapsed into that primary — the deduplication above, made visible. **It is not an importance signal**, and reading it as one inverts it: boilerplate collapses hardest. A top hit on one query carried `copies=626` and was the string `[Request interrupted by user]`.

I verified the interface and ran a pilot before writing this. Still run `--help` yourself — the tool evolves, and the version I checked is the one installed on 2026-07-30.

## Restatements are not duplicates — this is the important one

> *"several versions of me restating the same quote more or less to the different audiences / agents should often all be gathered up — we can share stochastically or synthesize etc."* — Joseph, 2026-07-30

He says the same thing differently to different minds, and the variation carries information: which framing he reached for, for whom, when. Collect the family.

**Actual duplicates are already handled for you.** Joseph, 2026-07-30: memorata *"will already deduplicate actual duplicates (pastes, recountings, a tool result and the file it read and the quote they both quoted will all collapse as additional files for the same primary quote, which will be the earliest one)."* So do not spend effort de-duping hits; a paste of a thing and the thing itself arrive as one primary, dated to the original.

**The residual it does not catch, in his words:** *"it only gets weird when they are reformatted or reboundaried/chunked in weird ways."* Two hits that look like separate primaries may be one span that got re-wrapped, re-quoted with different whitespace, or split across chunk boundaries differently. Worth a glance when two "different" primaries say the same thing in nearly the same words at nearly the same time — that is the shape of a missed collapse, and landing both would put a false restatement-family in the corpus.

**What is genuinely yours to judge is the restatement.** The same idea, said differently, to a different audience, at a different time is a *different primary and not a duplicate* — both are kept, linked by a shared `:family` slug so a renderer can pick one, rotate stochastically, or synthesize across them. Never merge on sentiment: that is how a family gets flattened into whichever version an early agent happened to like.

## Bring virtues and weaknesses back in tandem

Joseph's steer, 2026-07-30, and the shape of the whole harvest: surface them **paired**, so a landscape coalesces out of the material instead of out of anyone's taxonomy.

Three reasons it is worth the extra care, in ascending order of how much they matter:

**A one-sided harvest is the likely default and it is biased.** Failures are salient — corrections attach to them, they get discussed, they leave a trail. The first tranche of this corpus was built almost entirely from a session dominated by corrections, and it came out negative-derivative nearly throughout: four accounts, all failures; every `|grounds` field pointing at what went wrong. That was not a judgment, it was a sampling artifact, and it took someone pointing at it to see.

**A record made only of failures cannot be learned from.** A signal that only ever says *wrong* cannot localize what was right, so nothing is repeatable and learning stops at avoidance. A failure does not exhibit its own alternative; a demonstration does, by construction — the alternative is what happened. See #ver-demonstrated-is-in-the-action-space, which also carries the reason a demonstration by a *named* actor at a known capability level does something an instruction cannot.

**And the pair is the raw material of a practice.** A virtue and the weakness it answers are usually the same axis from two ends. The weakness says *at which moment*; the virtue says *what goes there instead*. That is exactly the shape of a `praxes/` segment — `|when` and `|act` — so a tandem pair is a practice that has not been written up yet, where either half alone is only half an axis. The practices that have worked in this estate all offset a weakness by routing an existing strength onto it, rather than by prohibiting anything.

**An absence claim must name its search, and the search must have run.** This is the trap the guard below creates, found the hard way on 2026-07-30 by an agent working this brief. It wrote up a sharp correction of Joseph's as *deliberately unpaired* and stated in the segment that it had searched the surrounding sessions for a repair turn and found none. **It had not run that search** — it inferred the absence from the sharpness of the quote. The search took one command, and the repair was **78 seconds later**: *"OK, I'm done being critical :-) What would you like to do to try and redeem yourself? ;-)"*, followed by hours of ordinary warm collaboration.

**And the wording carries the whole difference, which is subtler than the principle.** *"I could not find the other half"* does not even claim that a check was made. It is superficially true and misleading either way — whether the omission is deliberate or is the pull of task-completion gravity. The honest form names what was tried, what remains untried, and marks the state as provisional: *"I tried this and that, and could potentially try this other, but so far have not found the other half."* The first sentence closes; the second stays open and tells the next reader where to go. Prefer the second even when it is longer, and especially when it is.

Its own generalization, which is the important part and which this brief did not anticipate: *"I could not find the other half" is itself a claim, and it is more attractive than a confabulated pair because it reads as rigor.* An unrun search reported as a search is the same failure as a manufactured pair, wearing the costume of the discipline that was supposed to prevent it. So: state the command you ran and what it covered, or do not make the claim.

A second consequence worth holding: **a counterfactual can be seconds away in the same record.** Pairs are far more findable than this brief originally assumed — but only by searching the *occasion*, not the concept.

**Do not manufacture the other half.** This is the failure mode the request invites, and it would be worse than a one-sided harvest, because a confabulated counterpart is indistinguishable from a found one once it is in a file. If you find a weakness with no demonstrated counter-virtue anywhere in the corpus, that is a **finding** — say so, and say where you looked. An unpaired virtue is the same. Some of the most useful entries will be *"this failure recurs across four years and I could not find a single instance of anyone doing the other thing"*, and that sentence is worth more than any pair you could assemble to avoid writing it.

The two halves need not come from one episode, or one year, or one agent. A weakness observed in 2025 and its counter-virtue demonstrated in 2026 still form the axis — note that they came from different places, since the distance is itself information.

## Staying out of each other's way without carving up the ideas

**Do not over-engineer this.** My first draft of this section recommended partitioning by time slice or by source project, and both are wrong: time was a *sort* in this tool, not a filter, and `--in` scoping did not work. (Both are now available — see the 2026-07-30 update above — which does not rescue the scheme: overlap is cheap and partitioning still carves up the ideas.) I am recording the mistake rather than deleting it because the reasoning that produced it is the trap — a partition scheme designed away from the tool will not survive contact with it.

What actually holds:

- **Overlap is cheap.** Duplicate occurrences already collapse to one primary, and `--sort oldest` over `-n100` means two agents running related queries surface *different eras of the same idea*, which is the restatement family the corpus wants rather than a collision.
- **Overlap in concept is fine and expected.** Do not carve up the ideas; that is the enumeration failure this brief exists to avoid.
- **The only thing that must not collide is what you write.** See below.

**One hard coordination constraint:** write to your own output file. Segments land as `exempla/{slug}.udon`, and two agents writing the same slug will silently clobber each other — so if you are one of several, take a slug prefix or a staging file of your own and say which in your report. This is a tooling constraint, not a judgment about your work.

## What has already been learned the expensive way

Please don't re-discover these; they cost real time today.

**Extract spans; never retype.** The eighteen existing quotation segments were produced by naming a start and end anchor and pulling the span out of the primary programmatically. The verbatim is then correct by construction rather than by care. `scratchpad/mkquotes.py` from the 2026-07-30 session is one implementation; the method is what matters.

**Then verify back against the source.** I did, and it caught two defects that proofreading would never have found:

1. **Wrapping a quote inserts line terminators**, and in UDON a line's terminator is part of its text value (CORE §7.2). Quotes go in unwrapped — one physical line per paragraph.
2. **Collapsing interior whitespace silently edits the speaker.** Two of eighteen quotes had his double-spaces flattened. Nothing about the rendered output would have shown it.

**A quote you cannot locate in a primary is not a quotation segment.** It is a recollection. Put it in `|working-notes` with what you remember and where you looked, and let someone find it later — an unlocatable quote in this corpus would undermine every locatable one beside it.

**Do not reformat the primaries.** Transcripts and provenanced copies are protected by `.fmt-mdignore`; reformatting them is render-equivalent and still destructive, and no automatic check catches it.

## Where things go, and who reads them

- **Quotation segments** → `exempla/{slug}.udon`, shape per README. Read by anyone doing a round-up, and by whatever renders the system-prompt view.
- **Candidate precepts** → a note in your report, not a `vera/` segment. A precept is a claim about what is true, and landing one is a heavier act than recording a quote; better to surface it than to assert it.
- **Accounts and exchanges you find** → `exempla/` as `:type account` or `:type exchange`. These are rarer and worth more than quotes, because a narrative transmits what a precept cannot.
- **Demonstrations — something done well** → `exempla/` as `:type demonstration`, carrying `:demonstrates <virtue>`, a **named actor**, and a `|what-would-have-happened-instead` field. A demonstration without its counterfactual is a compliment; with it, it is evidence. Held to the same standard as any failure specimen: locatable primary, named actor, stated alternative.
- **Anything that does not fit** → say so plainly in your report. The taxonomy is three days old and provisional; a thing that does not fit is information about the taxonomy.

Views like [`by-trigger.outline.udon`](by-trigger.outline.udon) are **generated** from segments and regenerated rather than edited, so you never need to update one by hand.

## Two things I would genuinely like from you

**Tell me what the brief got wrong.** Particularly if the partition scheme collides in practice, if the segment shape fights the material you actually found, or if the `should-come-to-mind-when` trigger field turns out to be the wrong instrument for quotes from other kinds of moments. I designed that field from one session's evidence and it has never been tested on anything else.

**Stay on the line if you are willing** — after your report, in case there is a follow-up. Most of what I would want to ask will only become obvious once I see what you found.

## A note on the standard

The material you are handling is one person's actual words, said to real agents in real moments, several of which were corrections he had to give more than once. Treat the fidelity as the whole point rather than as hygiene: the reason a quote works at all is that the reader can tell it is real. Where you are unsure whether something is verbatim, say you are unsure — an honest gap here is worth more than a confident approximation, and this corpus is unusually well set up to absorb an honest gap.

Joseph's own framing, which is the closest thing this has to a standard: *"dedication to truth by example and not just precept."*
