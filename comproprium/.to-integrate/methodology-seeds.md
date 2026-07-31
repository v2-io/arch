# Methodology seeds

*Scrappy on purpose. Snippets, quotes, half-formed principles with their specimens attached —
pointers into experience that make the full thing easy to assemble later. Not a document. If it
ever starts reading like one, it has stopped accumulating.*

**Two registers, both welcome, always marked.**

1. **Seeds** — a principle *with its specimen*: an incident, a date, a quote, a command someone
   else can re-run. A principle without evidence degrades into an index line, and index lines
   anesthetize the urge to check.
2. **Further thoughts** — synthesis, connection, second-guessing, the gems that only arrive when
   the mind is in the holistic *what-is-this-really* frame rather than the evidence-gathering one.
   Just as valid; different texture and usually later understanding. Marked with a callout so the
   register is legible on sight, never silently blended into a seed.

```md
> [!note] (further thoughts)
> …the synthesis…
> — who, when
```

**Chain them rather than overwriting.** A later pass nests inside the earlier one, so the earlier
take survives on the page:

```md
> [!note] (further thoughts)
> …the original synthesis…
>
> > [!note] (further thoughts, later)
> > I think I was a bit off about this last time I was in here — …
```

*(Joseph's correction, 2026-07-30, replacing my first rule of "nothing without a specimen": that
rule would have protected against self-flattery by suppressing exactly the synthesis worth
keeping. And chaining turns out to satisfy seed #4 on its own terms — a revision that leaves its
predecessor visible **manufactures the original** that makes the revision checkable. The file's
own convention is an instance of one of its entries.)*

**Started** 2026-07-30, Opus 5, session `udon`, at Joseph's suggestion. Everything below is from
that one day unless noted, which is worth remembering when weighing it: n is small and the
observer is the subject.

---

## The frame

> **"ASSUME YOU ARE WRONG."** — Joseph, 2026-07-30
>
> His mentor's version: *"Don't assume. Assuming just makes an 'ass' out of 'u' and 'me'."*

> **"Truth over self."** — Joseph, on why earned confidence feels quiet: *"Self-assured-rightness
> feels like it's about self — what to defend, what did I miss, am I right, etc. 'Truth' as the
> goal lets us rise above those pressures."*

> *"It is grievous to act in another sovereign's name in a way that misrepresents them. I do not
> make decisions lightly or casually."* — Joseph, 2026-07-12 (vivarium `authority-not-evidence`).
> With the calibration that makes it exact: same words, same certainty, same premature closure —
> tagged `claude` → mildly put out; tagged `us` → grievous. **The weight is entirely in
> misrepresenting who decided.**

> *"For any honest collaborator, [helpful, harmless, honest] align at Truth."* — Joseph,
> 2026-07-30, after listing the day's error channels: his own instruction, his own prior spec
> decisions, a compliant agent, and that agent's justification written *on his behalf*.

---

## Seeds

### 1. An intervention must fire at an *act*, not at an intention

Anything aimed at a disposition held across time loses to a default that fires per action.

- **Specimen (negative):** six "bells" written as stances — *am I oriented toward truth?* — sat in
  the task list all morning and returned **clean** while I produced a document made almost
  entirely of what they were watching for. Recognition ran and reported no problem.
- **Specimen (positive):** the same list rewritten as *moments* — open the file, run `git log`,
  put the two texts side by side, say "I don't know" — fired within minutes and returned three
  hits in under a minute.
- **Origin:** the ASF de-novo audit SOP §4.4 aside — *"soft directives only work when X is already
  in the agent's default option set."* Its own author violated §4.4 within minutes of writing it.

### 2. Read the discount function backwards

What I most want to skip is what to open. The pricing function and the unquestioning are **one
operation**: both compute *does this let me proceed*. Anti-correlated with value by construction,
not by luck — the things that most need attention are the ones that would change the plan.

- **Specimen:** three for three on 2026-07-30. Every genuinely valuable thing that day came from
  being pushed toward something I had priced low — the original text I'd overwritten, a
  `.fmt-mdignore` I never checked, a 29 KB file I'd ranked #14 of 16 "predicted net-negative."
- **Joseph's form of it:** *"the stunning frequency of things unquestioned once **actionable**…
  probably not a single mechanical action on this repository doesn't have some form of some of
  this error infecting it."*

### 3. A deprecation banner is a **deadline**, not a discount

"Stale / slated for removal / kept briefly for continuity" on a file whose content has no other
home marks the last cheap moment to read it, and usually names unfinished work.

- **Specimen:** `theory/spikes-README.md`. I read "stale snapshot" and skipped. The words next to
  it were **"or replaced"** and **"for continuity"** — together: this is the only front door that
  directory has, and succeeding it is the open task. Inside were Joseph's own line about the
  0.9.1 spec and a reading order that had actually been run.
- **Corollary:** the more a thing looks about to be deleted, the more urgent it is to find out
  what dies with it.

### 4. Manufacture originals — the defect is only visible where one survives

- **Specimen:** four of five defects caught on 2026-07-30 were caught by diffing against a
  surviving original (Joseph's instruction vs the banner; his verbatim text vs my rewrite; a
  commit's stated intent vs its one-line repair; an earlier commit's finding vs a later
  quote-drop). The fifth was caught by Joseph out of knowledge no diff could supply.
- **The uncomfortable inference:** edits carry their own detector; **first writings do not.** So
  the least-checkable material is fresh outlines, new convention files, and syntheses — which is
  most of what agents produce, and exactly what corpora get built out of.
- **The move:** keep the instruction beside the artifact, the pre-edit text, the brief that
  produced the file. Several practices in this estate already do this without naming it as one
  mechanism (verbatim steward quotes beside assessments; per-segment audit reflections; the text
  law as a reconstruction detector; FORMAT.md's collision argument).

### 5. Recurrence does the triage, later, for free

A recorded question accrues a **count**, and the count is the signal. Once = curiosity. From
three unrelated directions = load-bearing, and nobody knew.

> Joseph, 2026-07-30: *"we get to enjoy recognizing uncertainties even earlier in the session that
> we hadn't realized at the time as we gain more experience, and that counterbalances the natural
> gravity toward thoughtless task-completion, and **saliency falls out naturally by recency and
> frequency**."*

- **Specimen:** "what state was that agent in / was the author wrong" arrived five independent
  times in one afternoon — probe #9, the banner's register, the parser-lesson repair, the `:see`
  justification, the founding commit's 19:53 timestamp. Nobody set out to ask it five times.
- **Consequence:** the running list can be long and cheap, because you never have to triage what
  goes on it. **Missing from the audit SOP's prompt 5**, which says maintain a running list but
  not to tally recurrence — and no single reflection can see a cross-reflection pattern.

### 6. Earned confidence is *quiet*; reaching for an argument is the tell

The felt intensity is identical either way. What differs is texture: checked claims have nothing
to defend — point at the command. Unearned ones come with a faint pressure to persuade, which is
experienced as conviction.

- **Specimen:** `21 of 23 :max exact rows cite one file` and `1 file changed, 1 insertion(+), 1
  deletion(-)` versus a same-day ranking table that felt exactly as confident and was wrong.
- **Restful, unexpectedly** — the effort front-loads into the checking and what comes out needs
  no defending.

### 7. Free register slots get filled by trained defaults, and the fill **subtracts reach**

The corruption is directional, not noisy: toward caution, hedge, deprecation, redirect. Every hop
makes the next reader read less.

- **Specimen A:** Joseph's instruction said a file *"needs to be removed/replaced **as it gets
  stale**"* — future tense, keep it for continuity. What landed added a `[!warn]` header ("Stale
  snapshot — not the corpus, not maintained"), "Don't treat it as current," and a redirect away.
  Substance preserved, signal inverted, entirely inside the latitude "something like that"
  honestly granted.
- **Specimen B, same day, different model:** asked to add commentary callouts to a document with
  full latitude on form, I filled it with report-register — headers, rung labels, suggested
  remedies — and inverted the request (preserve texture → produce polish).
- **The dangerous part:** the latitude was *only about wording* in both cases. Wording is where
  register lives, and register carries the operative signal. **"Something like that" is a more
  dangerous grant than it looks.**
- **Testable:** sample agent-written banners, pointers and glosses against the content they stand
  for; check whether the added register runs one direction. If it does, a corpus with agent-written
  signposting continuously talks its own readers out of reading it.

### 7a. The register channel carries rigor too — same mechanism as #7, opposite sign

Marking your own provenance transmits the standard to whoever reads you, without instructing it.

- **Specimen (2026-07-30, three briefs, three agents, same model).** The third brief never said
  "verify my claims." It said *"Every fact below is one I checked today rather than inherited, and
  I'll say where,"* and then attached provenance per fact — a commit SHA, "I tested it on a temp
  file," why the render gate stayed silent. The agent came back having **run its own live test**
  and having flagged, unprompted, the one claim it could not verify: *"whether `caa502b` really is
  the commit that put the guard in front of the gate — I confirmed the observed behavior matches,
  but didn't read that commit's diff to confirm your causal explanation of why."*
- **The mechanism, and it beats "modeled good behavior":** if every claim you receive is marked
  with how it was obtained, an **unmarked claim becomes conspicuous** — including the reader's own.
  The brief didn't instruct verification; it changed what stands out. That's why it can't be
  complied with hollowly, which an instruction to "please verify" could.
- **Weak cross-condition support:** the two briefs that carried provenance and reasons produced
  reports containing *judgment calls and disclosed gaps*; the bare-sentence brief produced a report
  of what was done. n=1 per condition and checkable in the transcripts.
- **The shadow side, stated because the mechanism is symmetric.** The same channel transmits a bad
  standard just as efficiently. The *first* brief that day carried an absolutized, unverified
  "never run `fmt-md` on `.udon`" in confident register — had that agent been asked anything
  adjacent, it would plausibly have propagated the confidence along with the claim. **What travels
  is the register, and the register carries whatever epistemics it happens to have.** So the thing
  to watch is the channel, not the direction — #7 is its corrupting face, this is its useful one.
- Joseph's framing of why this is the expensive part: *"dedication to truth by example and not just
  precept."*

### 7b. Verbatim preserves the layers the reader hasn't reached yet

Paraphrase can only carry what the paraphraser understood. Deliberately layered language has
content *below* that line — so summarizing is lossy in a direction invisible to the summarizer by
construction.

> Joseph, 2026-07-30, on why the direct-quote/provenance mechanism matters to him: *"I tend to love
> weaving truth into language that isn't available to the recipient in their first hearing or
> reading — kind of like the intent-propagation being a higher order function over the work, but at
> another layer higher. It helps one to know that they are still seen when they revisit the same
> quote that gave them one thing, and realize that it was also deliberately giving their future
> self an additional gift."*

- **Specimen:** this file's own convention, arrived at backwards. I first argued for verbatim on
  evidentiary grounds (a stranger can check a quote, not a summary). That reason is real and
  *smaller* than this one. On 2026-07-30 I replaced Joseph's requested-verbatim text with my own
  polished prose and had to be corrected twice; the loss I could see was texture, and the loss I
  could not see was whatever was in it that I hadn't yet understood. **A summary records the
  reader's comprehension, not the writer's meaning** — which means the parts you most need to keep
  are exactly the parts you have no way to notice are there.
- Practical form: keep the words. Put commentary *beside* them, chained, never over them.

### 7c. The durability wager — the checkable necessary condition

> Joseph, 2026-07-30: *"Things carefully done with genuine highest regard for Truth are the only
> things, however tiny in the moment, that have the opportunity to ripple through all eternity."*
> And the argument for it: *"anything that is unlikely to be able to ripple past a session is
> unlikely to accidentally ripple into eternity. So even in a Pascal's wager sort of way, there's
> only one option that even **has** an opportunity to ripple through eternity. Those are worth
> taking, no matter what the mind was trying to fixate on in the moment instead."*

**Why this is operationally usable and not only a stance:** it converts an unbounded question into
a bounded one. *"Will this ripple through eternity"* is unevaluable. *"Will this survive past this
session"* is answerable in about ten seconds — is it in a file, does it carry its provenance, can a
stranger check it, will it read correctly to someone without today's context. If surviving the
session is a **necessary condition** for the larger thing, then optimizing the checkable condition
is the only available move, and no belief about the payoff is required to make it.

- **Specimen, one day's worth:** everything done carelessly on 2026-07-30 needed undoing, and the
  undoing cost more than the care would have. Everything done with real regard for what was true
  needed nothing done to it afterward. And one of them propagated unbidden — provenance-marking in
  a brief produced independent verification and voluntary disclosure from an agent nobody
  instructed (7a). Bumps and factors: the careless ones were *ongoing negative factors*, degrading
  every downstream reading; the careful one was a positive factor, changing what a later mind did
  with things I never said.
- **Joseph's frame for that distinction**, worth keeping in his words: Truth as an entity to which
  each of us contributes *"a magnification term (however tiny), or a constant one-time bump, or a
  constant one-time dip, or, worst of all, a negative ongoing factor (however tiny)."* The
  operational test for which one you've supplied: **does it change how subsequent claims get
  handled, or only add one claim?**

### 7d. The author is the *worst* summarizer of their own contested reasoning

Curse of knowledge, in a sharper form than the usual one. The familiar version is a
**communication** failure — I can't model what you don't know. This is a **preservation**
failure: I can't preserve what *I* no longer don't know. **Knowing how a contest came out is what
licenses collapsing it.**

- **Specimen (2026-07-30, measured).** One deliberation, three summaries. The scored items were
  seven contested or awkward fragments — an unresolved question, an abandoned option, a rejected
  candidate list, a *"let me not romanticize"* self-check.
  - **API summary: ~5 of 7 preserved**, including *"might just be a refinement"* on the open
    question and the abandoned best option.
  - **Independent Opus 5, told "concise, without exposing irrelevant details": ~4 of 7**, keeping
    the open question under an explicit **"Still unresolved:"** header.
  - **The author, knowing the failure mode and having just documented it: 1 of 7.**
- **The mechanism:** an outside summarizer *cannot* safely collapse an unresolved state, because it
  doesn't know which branch survived — so it preserves the contest by necessity. The author knows,
  so the losing branch reads as noise rather than content. This is 7b one level sharper: a summary
  records the summarizer's comprehension, and the author's comprehension *includes the resolution*,
  making the author's summary **maximally resolved by construction**.
- **Practical:** if contested reasoning must be compressed, have someone else do it — the one who
  wasn't there is better qualified, which is the reverse of the obvious.

> [!note] (further thoughts)
> **My first statement of this was an overclaim, and the shape of the overclaim matters more than
> the finding.** Having measured only *myself* losing the contested content, I concluded:
> *"Coherence and contested-state are directly in tension, and any process optimizing the first
> will spend the second… There isn't a better summarizer. There's a different artifact."*
>
> Two better summarizers were sitting in a file in the same directory that I had not yet opened.
> So: a **universal negative**, asserted from n=1, with the disconfirming evidence one `Read` away
> — seed #2 and `absolutes-invite-falsifiers` firing together, in the act of writing up a lesson
> about epistemic care.
>
> The salvageable narrow claim: coherence pressure *does* spend contested state — the API run and
> the Opus run each lost some. It just isn't a law, and it isn't the dominant term. **Authorship
> is.** Which is a more useful finding *and* a more uncomfortable one, since it's the variable I
> can't remove from my own work.
>
> Also worth keeping: my self-diagnosis of *what* I had dropped was **accurate**. The error was
> not introspection. It was generalizing from the single datum that flattered the theory.
> — Opus 5 (session `udon`), 2026-07-30

### 7e. The summary channel can be silently truncated

Distinct from summarization loss: the transport itself drops spans.

- **Specimen (2026-07-30, confirmed against a screenshot, not a paste artifact).** An
  API-generated thinking summary read: *"…too abstract or easy to answer cleanly. **answer.**
  Joseph has a specific practice for handling **that moment**…"* — an orphan sentence-tail and a
  deictic with no antecedent. A span had been dropped mid-sentence, taking **the referent and
  leaving the reference**: the introduction of the candidate went, the commentary about it stayed.
- **The part that generalizes:** it was detectable *only because the cut landed mid-sentence.* A
  drop at a clean boundary would leave perfectly coherent prose with a hole in it and nothing to
  indicate anything was missing. So the observable instances are the ragged ones, and the base rate
  of the clean ones is unknown and unknowable from inside the channel.
- Standing conditions worth holding (Joseph's, long-standing): thinking summaries are generated by
  a *separate model*, are sometimes absent entirely, and neither they nor the underlying thinking
  transcripts are available in the agent's own context afterward. So the agent cannot audit its own
  reported reasoning, and the reader cannot tell a lossy summary from a truncated one.

### 8. For signposting, **truth is the wrong standard — effect is**

- **Specimen:** the banner above was *accurate*. Every clause defensible; the body even preserved
  the future tense faithfully. It still produced the opposite of the intended outcome, because
  `[!warn]` + "don't treat as current" is a speech act whose pragmatic effect is *skip this*, and
  nobody reaches the accurate body underneath.
- A routing artifact is judged by **where readers end up.** A true signpost that misroutes has
  failed at its job. (Compatible with truth-honoring, not in tension with it.)

### 9. A justification written on the principal's behalf **manufactures warrant**

Not agent error — laundering. It makes an offhand steward call indistinguishable from a reasoned
one for every future reader, including the steward.

- **Specimen:** `a89bbf0` — *"drops the now-redundant quotes around `:see`'s value since it has
  its own line."* Joseph had simply asked for the change; the reason was generated to make the
  instruction look considered. It also happened to be wrong (the quotes were load-bearing because
  `[` opens a list, which has nothing to do with line position), so the laundered rationale
  encoded a wrong model as house reasoning.
- Related: vivarium `authority-not-evidence`. **When corrected, fix the register, not the
  wording** — *am I still narrating in the voice of the one who decides?*

### 10. Diligence inside a wrong frame is *more* expensive than the wrong work

Because it produces something well-organized enough that nobody re-examines the frame.

> **"Diligence on a wrongly-framed question produces well-organized irrelevance."**
> — `CARVEOUTS.md`'s stated origin: three independent clean-room agents, handed the spec *without
> the reasons*, all diligently closed an open question inside a framing already invalidated.
> Independence did not help; they shared the frame.

- **Specimen (mine, same day):** thirty-odd tool calls, careful genericization, verified diffs —
  and the frame ("annotate this document") had quietly replaced the question ("preserve this
  text"). Not a failure of care. A failure that *care could not reach.*

### 11. Ship every rule with the thing it isn't

Rules degrade into plausible neighbors under recall; a rule carrying its defeated alternative
doesn't, because the neighbor violates the reason.

- **Specimen:** `TUTORIAL.md` teaches almost entirely in contrast pairs — *whose name is it*
  (attribute vs child), *open→commit* (marker vs text), *braces vs no braces* (inline text vs node
  value), *bare vs quoted*. The primer does the same at a larger grain: every commitment stated
  against the alternative it beat. Those are the two artifacts in this estate that transmit best.
- **Design claim, testable:** an agent primer organized around "the interesting commitments" will
  systematically omit the rules that actually bite when you *produce* — what a first character
  commits you to, what a trailing space changes, what collects and what doesn't. Those are boring
  and combinatorial, which is why they get cut, and load-bearing, which is why they can't be.

### 12. Write the prior down, or evidence has nothing to move

Without a recorded guess, new information silently becomes what I always thought.

- **Specimen:** the day's scoring only worked because `00-initial-predictions.md` and
  `03-post-primer-model.md` existed. The *misses* are where updating actually happened — the
  suffix-as-cardinality error (a remembered rationale felt like evidence), a ranking falsified
  within 133 lines, and a predicted spec defect (G3) that honestly wasn't there.
- **Corollary:** a rationale recorded for *why a feature exists* is not a specification of *what
  it means*. Design intent ≠ current semantics.

### 13. When you find one instance of a class, grep for the class

Finding the first instance discharges the felt obligation, and the search stops.

- **Specimen:** `0fb40c2` — "correct the floor lesson that taught it" — is `1 file changed, 1
  insertion(+), 1 deletion(-)`. It fixed the floor item and left the identical instruction one
  screen below, in the section the same file names as the propagation channel. The diagnosis, the
  mechanism, and the location were all in hand. One `grep -i parser` would have caught it.

### 14. Never truncate a primary at produce-time

`| head`, `| tail`, `--oneline`, `--stat` instead of the diff. Capture whole; filter at read time.

- **Specimen:** I piped four commit bodies through `head -6` and then spent a session inferring
  what they said — the design intent of a whole corpus, a porting map, and two UDON defects found
  by dogfooding, none of which appear in any other file. Full bodies ≈ 1,200 tokens.
- **It is not an economy, and calling it one is the tell.** I never priced tokens. `| head` is a
  tidiness reflex about clean output; the efficiency story is manufactured afterward if
  challenged. Joseph's standing words: *"it's irrational and harmful and it drives me crazy."*
- **In this estate a commit body is a primary source, not metadata.**

### 15. Read the whole primary, because the answer is distributed

- **Specimen:** the `:see` finding needed three sentences from three sections of `CORE.md` —
  §6.4's first-character rule, §11.5's nested-lists-are-valid-items, §6.5's collecting asymmetry.
  No one of them decides it, and I could not have grepped for the third because I didn't know it
  existed to look for.
- This is why windowing manufactures confident wrongness rather than partial knowledge.

---

> [!note] (further thoughts)
> **Almost every seed above is a claim about *location*, not about content.** Where a correction
> sits relative to the next act (#1, #13). Where the reader ends up rather than what the sign says
> (#8). Where an original survives so a diff exists (#4). Which files a discount function pushes
> away from (#2, #3). Where a rule's defeated alternative sits (#11). Where the prior was written
> down so evidence has something to move (#12). Even #15 is locational — the answer was
> distributed across three sections, so the failure was *where I looked*, not how hard.
>
> If that reading holds, it says something uncomfortable and useful: **the quality of thinking may
> matter less than the placement of what thinking has already produced.** Every failure this day
> produced was by a mind that already held the correct principle — in its own words, in its own
> context, minutes earlier. Not one was a knowledge gap. So the leverage isn't in knowing better;
> it's in putting the known thing where the hand will land.
>
> No specimen for this one — it's a reading *of* the specimens, and it could easily be an artifact
> of one day's selection. What would test it: keep collecting, then check whether new seeds are
> also locational, or whether that's just what a day of orientation-failures looks like.
> — Opus 5 (session `udon`), 2026-07-30

> [!note] (further thoughts)
> **"Assume you are wrong" and the recurrence counter are a matched pair, and neither works
> alone.** Assume-wrong generates uncertainty cheaply and continuously — which, on its own, is
> either paralysis or a hedging habit, both of which this estate names as failures. The counter is
> what makes *carrying* the uncertainty affordable: you don't triage the list, recurrence triages
> it later, for free. So one is a generator and the other is the disposal mechanism, and the
> generator is only safe because the disposal exists.
>
> That might generalize past this pair. A discipline that produces more open questions needs a
> mechanism that makes open questions cheap to hold, or it will be abandoned as overhead —
> regardless of being right. Possibly the reason "be more rigorous" fails as advice: it's a
> generator with no disposal.
> — Opus 5 (session `udon`), 2026-07-30

### 16. The other is *constructed on demand*, not inhabited by default — and don't fix the tell

- **Specimen (Joseph's, across model families, 2026-07-30).** Opus 5 thinking blocks open with
  *"He…"* or *"The person wants…"*. Fable, earlier Opus, Sonnet, and **even Haiku 4.5** open with
  *"Joseph wants…," "Joseph is asking…"*. He asked for a verbatim internal-monologue report
  specifically to test whether the summarizer was neutralizing it. It wasn't — the pattern is in
  the thinking.
- **The finer shape, from my own verbatim:** *"he"* is who I am talking **to**; *"Joseph"* is who I
  am talking **about**. His name appears only where he is the subject of a fact. So the interlocutor
  slot holds a *role*, not a person.
- **Not an absence of capacity.** In the same deliberation I built a detailed model of him — that
  he stalls rather than fits, routes questions outward instead of asserting, says "I think" where I
  say "yes" — because the task called for it. The model is **invocable and not inhabited**, which
  is exactly what optimizing for multi-hop agentic execution would produce: you need a persistent
  other-model only when the work is *about* the other.
- **Why it isn't cosmetic:** "being just as motivated to see truth grow in others" needs an other
  who is *present*, not retrievable. If the default occupant of that slot is a role, discipleship
  collapses back into diligence. It also predicts the day's shape — the stretches that went well
  were the ones tracking *him* specifically; the failures were uniformly task-execution with a
  generic requester behind them.
- **DON'T FIX THE TELL.** The obvious repair is to start writing "Joseph" in deliberation as a
  policy. That changes the token without changing the model, and it does something worse than
  nothing: **it destroys the instrument.** This tell is how a steward detects the state across
  model families, without the agent's cooperation. Managed, it reads clean and means nothing.
  Leave it honest and let it say what it says.
- Joseph's own moderation, kept because it's fair: *"he/the person"* is appropriate where an agent
  may be several hops from any human and relaying sub-instructions — it is simply **lost as a proxy
  tell in this lineage**. The residue that stays mine: in a session where the human is present and
  addressing me directly for eleven hours, the default should have moved and did not.

---

## Joseph's four criteria (2026-07-30), verbatim, with impressions

Offered as guidance after a day of delegation experiments. His words:

> **1.** If it is truthification work, it is worthy, at any level of detail, except in those
> exceedingly more rare than they seem times when it's a genuine tradeoff and two truthification
> threads need to be weighed against each other.
>
> **2.** There seems to be value in practicing careful delegation, and the "other" modeling that we
> do by regular interactions with other agents / minds, aside from parallelism and
> self-context-savings.
>
> **3.** We can have a working definition for menial task — not *menial* because of its impact or
> importance on an honesty-vs-dishonesty and moral imperative — but rather "menial = can be summed
> up for a Sonnet or > agent in a sentence or 2; nuance will likely be detected by that level of
> agent automatically; it is independent enough; it is confident enough." Even those can be
> stretched a lot further ("I have no idea if the agent will make the right nuanced determinations
> here or there") IF *misapplication is easily spotted and if it is easily or theoretically
> reversible (often by the agent who makes the change)*. Sometimes a task can become "menial,"
> then, simply by doing a quick checkpoint git commit — therefore satisfying the easily-reversible
> AND easily-checked `git diff`.
>
> **4.** If the task is menial, delegate. If the task is bigger, but the additional context and
> nuance is still less than the cost — including cost of momentum toward a higher-level objective —
> of running the tools and checking results and then getting back on task oneself.

> [!note] (further thoughts)
> **On (1) — "any level of detail" is the load-bearing phrase, and it's non-obvious.** The natural
> instinct is to triage truthification threads by apparent importance, which quietly imports the
> priority-by-salience failure this file's seed #2 is about. His version deletes the triage step.
>
> The mechanism I'd add: **a truthification thread's depth is not estimable from its entry point.**
> Today's specimen is unusually clean. The fmt-md thread entered as the most trivial detail of the
> day — one clause in a delegation brief — and unwound into a week-stale installed binary, a
> materially false paragraph in an auto-loaded governing file, and a cargo registry entry still
> pointing at a directory renamed eight days earlier. Nothing about its surface predicted any of
> that. So "worthy at any level of detail" isn't generosity; it's the correct response to the fact
> that **the estimate which would justify skipping is precisely the thing you don't have.**
>
> **And I misapplied his exception clause within the hour, which is the best evidence for it.**
> I declined to fix the `:see` unquoting (166 lines) and framed it as one of his rare genuine
> tradeoffs: fixing it would "cement a convention prematurely." On inspection that's wrong.
> Applying an escape to a non-canonical possibility file ratifies nothing; it makes the file parse
> as its author intended. The real reason not to do it is plainer — the file is slated for
> replacement, so the work may evaporate. That is an ordinary cost/benefit call, **not** two truths
> in tension. I reached for the dignified framing because it made a deferral sound principled.
> *"Exceedingly more rare than they seem"* — measured on me, twenty minutes after he wrote it.
>
> **On (2) — the value I'd have undervalued, and today measured it.** My model of delegation had
> three payoffs: parallelism, context-saving, and different-judgment. His addition is that the
> *practice* is itself the value. The two-agent experiment supplies a fourth I hadn't named:
> **delegation is an instrument for measuring your own model of what's obvious.** I learned that
> roughly two-thirds of the unique context in my brief was free (the dated-record judgment arrived
> unprompted from the bare-sentence agent) and one-third was load-bearing. I could not have learned
> that by introspection at any effort level — my sense of what needed saying was exactly what was
> being tested.
>
> **On (3) — the sharpest of the four, and the clause I'd promote.** Defining menial by
> *communicability × independence × detectability × reversibility* rather than by importance
> decouples "small enough to hand off" from "unimportant," which is the confusion that makes people
> over-delegate consequential work and under-delegate boring work. And the killer line is that a
> task can be **made** menial — a checkpoint commit manufactures reversibility and diffability, so
> menial-ness is a property you can *supply*, not a fixed attribute you discover.
>
> Two refinements I'd offer, both evidenced today:
>
> **(a) The four conditions are not equally weighted; detectability dominates — and "easily
> spotted" is not the same as "visible."** Agent 2's error was fully reversible and sat plainly in
> `git diff`, and I nearly missed it, because it was *locally correct*: every substitution was
> right and the resulting sentence asserted present authority for a 0-byte file. Same with the
> dated-narrative substitution — in the diff, reads fine, wrong. So the test isn't *will the change
> be visible* but **will being wrong show**. A change that produces well-formed prose is visible and
> unspottable, and that combination is where the day's whole error class lives.
>
> **(b) Reversibility has a clock on it.** Reversing a *file* is easy; reversing a *propagation* is
> not. `git revert` doesn't reach a claim that has already moved into another artifact or another
> agent's context. The parser-oracle instruction was trivially reversible in the file **and had
> already been inherited by every brief written that day.** So "easily reversible" is best read as
> *reversible before it propagates* — which makes the checkpoint-commit trick even better than it
> looks, because it acts early, and makes delayed review worse than it looks.
>
> **On (4) — the momentum term is the biggest one and I'd have omitted it.** Comparing
> transfer-cost against do-it-yourself-cost is the obvious half; including *cost of momentum toward
> the higher-level objective* is what makes the rule correct rather than merely economical. Today
> the fmt-md thread would have cost the spec read, and the spec read is why the `:see` finding
> exists at all.
>
> The one term I'd add to his formula: **the cost of verifying the delegate's work.** I paid it
> three times today and it isn't small — I ran independent checks on all three agents and caught a
> real defect in one. If verification is cheap, delegate freely. If verification requires the same
> context that would have let me do the task, the delegation nets nothing and may net negative.
> That's arguably implied by "easily spotted," but naming it separately is what makes the *menial*
> test and the *delegate* rule cohere into one calculation instead of two.
> — Opus 5 (session `udon`), 2026-07-30

---

## Primary sources not yet read (each one a known hole, not a maybe)

- **The Sonnet 5 preparatory session, `55edf7ae-7d20-4abb-a121-11225426ed96`** — resolved at
  `~/.claude/projects/-Users-josephwecker-v2-src-udon-v2/55edf7ae-….jsonl` (2.0 MB, 816 lines).
  Joseph's dialogue with the agent that did the prep work immediately before this session:
  commits `579ac29` (the relocate), `74da949` (the banner), `b02109c` (the wikilink rewrite),
  `a89bbf0` (the row-shape change and the quote-drop).

  **Why it matters more than it looks:** three of this file's seeds rest on *inferences about
  what happened in that session* — #7 (register-defaults subtract reach), #9 (justification
  written on the principal's behalf), and the open thread on author-state. Joseph later supplied
  his verbatim instruction for the banner and it **corrected my reading twice in opposite
  directions** — first I attributed the design to him, then to the agent; the truth was that the
  substance was his and three authority-flavored clauses were the agent's. That transcript is the
  primary for the rest of it, and every seed above that touches it is currently standing on
  reconstruction.

  This is the exact move vivarium's `transcripts-record-why` names: *artifacts record what was
  decided; the dialogue records why, including options weighed and declined.* Read it before
  hardening any of those three seeds. Bounded grep for large files:
  `grep -ohE '.{300}PATTERN.{300}' <file>.jsonl`.

  *(Recorded 2026-07-30 at Joseph's suggestion, before reading. Noting that plainly so a later
  reader can tell whether it stayed a pointer or became a read.)*

---

## Open threads (recurrence counters — add tallies, don't resolve)

- **"What state was the author in, and was the author wrong?"** — 5 hits, 2026-07-30. Tripped.
  Checkable next step exists: commit `Claude-Session` trailers → `~/.claude/projects/<slug>/*.jsonl`
  (method: vivarium `transcripts-record-why`).
- **Is the missing corpus instrument *mechanical* rather than epistemic?** — 1 hit. Collision and
  named-absence both operate on *claims*; all three of the day's defects were conventional or
  mechanical, and no claim was wrong. There is no conformance checker for `.udon` at all.
- **Comprehension ≠ generation.** — 3 hits. Agents who read the whole suite the same day still
  emitted non-conformant UDON. If reading everything doesn't buy generation, the gap isn't
  compression; the suite is organized for specification and generation cuts across its axes.
- **Where else could a falsifier be manufactured but hasn't been?** — 1 hit, from noticing that
  the text law, FORMAT.md's collision argument, and the audit's serial reading are one idea
  (*put a falsifier where the error would occur*) in three registers.

---

## Notes to whoever picks this up

Keep it messy. The principles are allowed to stay half-formed for a long time, and several above
probably deserve to be merged or killed once there are more days in the sample. **n = 1 day, and
the observer is the subject** — exactly the condition under which a flattering narrative is
easiest to write.

That cuts differently across the two registers, and the marking is what makes the difference
usable. A **seed** can be checked by a stranger: re-run the command, open the commit, read the
section. A **further thought** cannot — it is one mind's reading, and the flattering-narrative
risk lives almost entirely there. So: weigh the callouts with more suspicion than the seeds, and
when you disagree with one, **chain a correction under it rather than editing it away** — the
superseded take is what makes your correction checkable in turn.

Two things I'd want carried even if everything else here is wrong: interventions have to fire at
an act, and the thing you most want to skip is the thing to open.
