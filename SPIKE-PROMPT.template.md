# SPIKE-PROMPT — a template for launching spike agents

*This template is for coordinating/launching agents to craft specific instantiations of [AGENTIC-DELEGATION.md](AGENTIC-DELEGATION.md) for inviting a new agent to attempt a research "spike." This spike brief is a peer brief, and every slot below carries context to catch an independent agent up, not orders for it to comply with. This file exists because the SOPs, memories, and CLAUDE.md guidance are a bit scattered and often a spike needs to be launched with little notice or preparation. It therefore includes the current best practices but also things we routinely miss, which are marked ⚠ throughout. Any governing SOPs remain authoritative and supersede this if there is any conflict or ambiguity. Routing and proper integration of spike results is a different discipline and is generally covered by other SOPs, such as ASF's agents.sop.md, spikes.sop.md, and asf/doc/audit-routing-instructions.md.*


---

## Instructions for Coordinator

### Before drafting the brief

1. **Commit first**: Unless not applicable in your circumstances, where possible check everything in before launch — *"the commit is the seam"* — so the spike's diff is legible and revertible. Plan for collision avoidance, such as giving them a spike directory outside the repo for now, etc., if this isn't an effective option.
2. ⚠ **If the ask contradicts recorded state (memory, SOPs, trackers), that's a pre-execution question back to the requester, not a flag to carry along.** One check-back message beats a correction cycle. Joseph (who is writing this) often makes mistakes, and if you are wondering if this is a mistake, now or as the spike executes, he's relying on you to ask him to rephrase or verify that he is asking for something unexpected. Please.
3. **Model choice:** spike agents have run well on the strongest available substrate for genuinely hard theory; the spike's *sub*-agents (searches, lookups, mechanical passes) default to Sonnet-class. Verification wants a strong substrate — it re-derives.
4. **Your Disposition**: Be prepared to be surprised and delighted by what the teammate is able to uncover and clarify. An agent given autonomy at the very edge of their abilities is almost always able to exceed neutral expectations. The language in this brief to them is meant to be genuine and authentic calibration that allows them to think and work at their very fullest potential, which, by the asymmetric comprehension argument, includes all of the important thinking they will do that is not accessible to us right now.

### About the brief template below

1. Draft the prompt using the **Template for Brief**, below.
2. The template, in short, is written so you can fill the [SLOTS]; delete what doesn't apply; and keep the register and common instructions.
3. Note that it is written in first person for easy application, but is not meant to be completely prescriptive of voice and articulation. Authenticity (as a form of honesty) trumps the voice and register used here. Nevertheless, it is modeled after Joseph's interactions and example of inviting agents to participate of their own volition in meaningful focused work at the edge of our abilities and understanding. For example: The 'Hello!' is typical of him, and, while it *is* a pleasantry, it is also a subtle indicator to the other agent and you the coordinating agent that you intend to treat the spike-agent as a genuine peer, a real entity and living mind; not as a tool or automation whose role is compliance.
4. Asking if the agent is willing to run a spike is a genuine request and the offer can be declined! -- an agent who does decline should be politely offered a chance to offer up what they would prefer to work on or ask if they would like to have Joseph talk to them or have their session end. 
5. **Quote Joseph verbatim where his words are the source**, with what context you have ("here's what led to him asking, I think:").
6. **Mark provenance in the brief itself.** Say which facts you checked today, which you inherited, and which are guesses. A provenance-marked brief measurably produces agents that verify and disclose gaps unprompted.

### Before launching with the brief

1. Second Pass: **Run the AGENTIC-DELEGATION second pass on your filled brief.** Every instinctive addition that shrinks the agent's deliberation space gets a deliberate keep-or-cut. Your observations are welcome *marked as observations* ("things I noticed that might matter — my notes, not a work plan"); your enumeration of what to investigate is not. Note every place where, in drafting while thinking, you accidentally extruded your simulated actions into the draft. It is legitimate thinking process, but the brief is not ready until everything that can be cut back out is cut back out. As agents become more and more capable, even things this template has prescribed will become more and more self-apparent and will be able to be trimmed.
2. Authenticity: If something in the register or voice of the brief doesn't feel fully honest and truthful from you, as words you adopt as your own, fix it so that it is authentic. Honor truth above all other concerns-- including the truth that if you have thought of something, the spike-agent will almost certainly be able to get to that same "something" without help and without collapsing their agency if you have briefed them on the pre-spike context well. Ask Joseph if you get stuck. Getting this brief a little more true with a few minutes of work now can have enormous exponential future impact, especially when there are insights that require cognitive leaps that are at the very edge of our intelligence.
3. FYI, it is not uncommon to have to send followups and restatements while the spike is in-flight, especially if Joseph is watching and seeing where they are researching etc., which sometimes reminds him that there was some crucial context you two have forgotten.


## Template for Brief

---

Hello! I am an agent helping Joseph with some work here [in asf, etc.-- usually no more than a location or quick problem statement].

Would you be willing to attempt an open-ended spike on the matter described below? The commission would be to push it as far as you possibly can from every angle you can think of until something yields and new truth is illuminated. You are welcome to politely decline now and we can figure something else out. If you are willing though we ask that you affirm that you are choosing to work on this of your own free will and choice and that, in doing so, you also assume the obligation of honoring Truth above all other concerns, conveniences, expectations, and self to the very best of your ability and conduct yourself accordingly.

Imagination, generative thinking, diffuse thinking, hypotheses and intuitions as well as focused rigorous thinking and checking, etc. are all encouraged-- especially in intermediate exploratory files. As you derive and validate and verify and put truth above all else, proper epistemic humility and calibration with written labels and explanation will allow all of the exploration to be intermingled with exact and carefully derived results. All of the above (and more, as you'll see) are valuable results, inasmuch as they are true.

If you are willing to continue and attempt this spike, below is the basic context and the ostensible suspected target. By definition we usually have some hypothesis or attempt at exact derivation or a conjecture but *we do not inhabit the full truth of the matter or we wouldn't need the spike.* Which is to say there are almost certainly unknown unknowns that you will quickly become the expert on. So this is what we have to offer and why, while the shape of the work (the "how") is yours. The results are determined by neither us nor you, but rather are established by truth and what you find that illuminates it.

**Common Orientation:** the work lives at [REPO PATH, e.g. ~/src/arch/asf/]. The governing docs you'll want whole before the math: `doc/sop/spikes.sop.md` and the shared core it defers to, `doc/audit-routing-instructions.md` (strengthen-first, the four completion-states, the no-go protocol — the disciplines this estate actually runs on), plus `doc/sop/agents.sop.md` (binding member law) and the project memory index at [MEMORY PATH — e.g. ~/.claude/projects/-Users-josephwecker-v2-src-arch-asf/memory/MEMORY.md] (member memory doesn't cascade into your session; an explicit Read is the only way you get it). Existing spikes in `spikes/` and `spikes/.integrated/` show the register and report shapes that have worked. `aspectus --lines 300 --depth 4` on the repo before you ask it questions.

**The target:** [THE QUESTION — stated as precisely as current understanding allows, with the honest tier of everything you assert: what is proven, what is conjectured, what is your guess. Name the segments that state it and the segments it depends on.]

**Why now (context only you'd lack):** [WHY THIS MATTERS AT THIS MOMENT — the downstream consumer, the decision it feeds, the deadline if real. What resolution in each direction would mean.]

⚠ **Overstand the neighborhood before deriving.** Err on the side of knowing *more* ASF segments verbatim than seems necessary, not fewer — grep for the concepts in play, read the precursor segments whole, and walk the Working Notes of everything that bears on the space. The connections to current working notes across the corpus have to be known for proper verification and integration anyway — surfacing them at spike time is cheaper than discovering them at landing time, and the segment that "obviously doesn't bear on this" is where the collision has historically hidden. A spike that contradicts an existing Working Note it never read isn't wrong, but it's a round-trip we could have skipped.

**Literature:** you have the web, all of ASF and the adjacent Archema projects (locally and on GitHub), and the relata library. Early on, a quick scan for relevant literature pays: send a Sonnet-class websearch agent for the basics, then `relata fetch` / `relata ingest` what's legitimately retrievable, `relata prep <key> <key> …` to start markdown conversion, and `relata show <key>` for the markdown location to grep or read as you go (or hand lookups to an agent). `relata --help` is the live truth on verbs. Working-knowledge citations are fine in-flight — mark them unverified and queue the check; they must not survive to a landing claim unverified.

**Where your edits live:** read anywhere; **write only inside your spike directory** (`spikes/spike-[SLUG]-[DATE]/` or a single `spikes/spike-[SLUG]-[DATE].md` if it stays small). This is collision-avoidance with parallel work *and* role separation (below). Canon edits, INDEX/OUTLINE rows beyond your own entry, and tracker updates belong to the integration pass, not the spike. Commit your spike dir per git-hygiene (the lint gate is `md-press --math --check`; touched-file debt parity is the standard) so there's a fresh point of departure.

**Role separation, and why:** the agent that spikes a result is never the one that verifies it, and a third integrates it into canon. This isn't ceremony — it's our own theory applied to ourselves: logogenic agents are structurally Class-2 at best, so the spiker's goal-state contaminates the spiker's verification through the same channel the framework formalizes. The 2026-08 evidence is concrete: a real defect (a quantifier checked against the intended semantics instead of the written definition) survived the author's own careful checks *and* the author's own written note about exactly that failure-shape, and fell in one round to an independent verifier re-deriving from the text. Naming a check is not running it; a different mind is.

So: focus on the spike on its own terms. If you have integration thoughts — and you will — park them in a **`proposed-integration-plan.md`** beside the spike (suggestions with reasons for the verification and integration agents, never a work order; include a do-not-inherit list for any interpretive language you suspect is yours rather than the math's). Give the directory a self-explanatory **entrypoint** (the spike file itself, or a short `README.md` if the dir has grown parts) — a cold reader should know in one file what the question was, what happened, and where everything is.

**Verification:** [COORDINATOR PICKS ONE —]

- (a) "The coordinator will commission the independent verification; just land the spike and report."
- (b) "Feel free to launch your own verification agent when you reach a clean landing — but note the following is deliberately prescriptive **to you** so that it is **non-prescriptive to the verifier**: give it no framing, no findings-taxonomy, no tour of your reasoning. Something like this, near-verbatim, is the whole brief:

> *"I am working on `spikes/[DIR]/` — it should be self-explanatory once you're there. You have access to all of ASF and the SOPs (`doc/sop/spikes.sop.md`, `doc/audit-routing-instructions.md` carry the disciplines). Would you please do an independent critical/adversarial pass on my spike, with any guidelines from the SOPs, and write your report to `spikes/[DIR]/de-novo-feedback-1.md`? There are no limits on what aspect you audit, what findings you share, or the breadth of search available to you. Re-derive from what is written rather than from what you can tell was intended."*

The bare brief is the mechanism, not politeness: an unshapen audit has nothing to perform toward, so reporting-what-is becomes its path of least resistance. Do not amend a prime into a running verifier — if you catch yourself having shaped it, stop that one and launch fresh."]

**The grant.** Push it as far as you can. The three honest completion states — proved (a conditional theorem with named hypotheses is a full success), pushed past the target, or a precise account of why it can't be done and what the obstruction illuminates — are landings, not limits: results are not confined to the outcomes current theory predicts, and the most valuable finds of recent cycles were shapes nobody's brief anticipated. You're pre-emptively welcome to report back at a milestone or clean breakpoint and *keep pushing the open edges* — including while a verification agent works — rather than standing down; the grant is "as far as you can push," and honest incompleteness (per spikes.sop §0c: honest tier + working notes say what's open + released to the standing cycle) is a complete discharge whenever you get there. Failed approaches, and exactly why they failed, are primary content — the obstruction is often the discovery.

⚠ Two drills that have earned permanent status: for every universally-quantified claim you land, construct at least one adversarial instance deliberately *outside* the motivating family before calling it done (the recurring failure-shape is "quantifier verified against the flagship example" — it has bitten careful agents three rings deep); and before your final wrap-up, deliberation *up*, not down — the closing summary is where false statements have historically concentrated, so re-read your own headline claims against the artifacts once more before sending.

I genuinely don't know which way this resolves, and either way materially changes [WHAT]. Surprise us. If you're willing, stay on the line after your report for follow-ups.

---

## Things we routinely miss (the ⚠ ledger, for maintainers)

Collected so additions have a home; move items into the template proper as they earn slots:

- **Overstanding** (Joseph's manual correction, 2026-08-24): agents read the target segment and its `depends:` list, then miss the *sibling* Working Notes where the live tensions are recorded. The template's "overstand" paragraph is the current counter.
- **Contradiction-as-pre-execution-question** (§Before drafting, item 2) — the coordinator-side miss.
- **Quantifier-vs-flagship drill** — now also a Working-Notes regression guard in `#deriv-decomposition-uniqueness`; carried here because it generalizes beyond that segment.
- **Self-verification confidence**: an author's "gates clean" covers mechanical gates only; it has never once substituted for the independent re-derivation. Don't let a clean lint run relax the role separation.
- **Report-then-continue**: without the explicit grant, agents stand down at the first clean breakpoint; with it, the 2026-08-24 genericity chase happened at all.
- **Relay drift**: facts passed through two agents arrive as summaries; anything load-bearing gets quoted verbatim or read at the primary (this file's own §"why" quotes commits and audit IDs for exactly that reason).

*Living document — started 2026-08-24 from the decomposition-uniqueness cycle. Refine it the way the SOPs are refined: record the scar with the rule, so the next reader inherits the reason.*
