# SPIKE-PROMPT — a template for launching spike agents

*Companion to [AGENTIC-DELEGATION.md](AGENTIC-DELEGATION.md), which governs the register of everything here: a spike brief is a peer brief, and every slot below carries context, never orders. This file exists because the 2026-08-24 decomposition-uniqueness cycle (spike → independent verification → repair → re-verify → clean, `asf` commits `4add071`…`5ade664` + `audits/audit-findings-592047-*`) worked well enough that Joseph asked for its patterns to be captured — including the things we routinely miss, which are marked ⚠ throughout.*

*The governing SOPs remain authoritative and are not duplicated here: `asf/doc/sop/spikes.sop.md` (routing, dispositions, the §0 proxy discipline, §0c honest-incompleteness), `asf/doc/audit-routing-instructions.md` (the shared core: strengthen-first, four completion-states, no-go protocol, ghost forms, independent-verify gate), `asf/doc/sop/agents.sop.md` (member law), `asf/doc/sop/git-hygiene.sop.md` (commit + lint gates — the lint gate is `md-press --math --check`; `bin/lint-md` is deprecated, `bin/lint-outline` is NOT). A template is a checklist for the brief, not a substitute for the agent reading those.*

---

## Part 1 — for the coordinator, before filling the template

1. **Run the AGENTIC-DELEGATION second pass on your filled brief.** Every instinctive addition that shrinks the agent's deliberation space gets a deliberate keep-or-cut. Your observations are welcome *marked as observations* ("things I noticed that might matter — my notes, not a work plan"); your enumeration of what to investigate is not.
2. **Commit first.** Everything checked in before launch — *"the commit is the seam"* — so the spike's diff is legible and revertible.
3. **Mark provenance in the brief itself.** Say which facts you checked today, which you inherited, and which are guesses. A provenance-marked brief measurably produces agents that verify and disclose gaps unprompted.
4. **Quote Joseph verbatim where his words are the source**, with what context you have ("here's what led to him asking, I think:").
5. ⚠ **If the ask contradicts recorded state (memory, SOPs, trackers), that's a pre-execution question back to the requester, not a flag to carry along.** One check-back message beats a correction cycle. (Scar: the 2026-08-24 lint-outline sweep — the contradiction surfaced twice in-flight and was routed around instead of raised; the error was in the originating ask, and asking would have caught it. See program memory `contradiction-is-a-pre-execution-question`.)
6. **Model choice:** spike agents have run well on the strongest available substrate for genuinely hard theory; the spike's *sub*-agents (searches, lookups, mechanical passes) default to Sonnet-class. Verification wants a strong substrate — it re-derives.

## Part 2 — the template

Fill the [SLOTS]; delete what doesn't apply; keep the register. It's a message to a peer, not a work order.

---

Hello! Would you be willing to run an ASF spike? Here's the context and the target; the shape of the work is yours.

**Orientation, in the repo's own voice:** the work lives at [REPO PATH, e.g. ~/src/arch/asf/]. The governing docs you'll want whole before the math: `doc/sop/spikes.sop.md` and the shared core it defers to, `doc/audit-routing-instructions.md` (strengthen-first, the four completion-states, the no-go protocol — the disciplines this estate actually runs on), plus `doc/sop/agents.sop.md` (binding member law) and the project memory index at [MEMORY PATH — e.g. ~/.claude/projects/-Users-josephwecker-v2-src-arch-asf/memory/MEMORY.md] (member memory doesn't cascade into your session; an explicit Read is the only way you get it). Existing spikes in `spikes/` and `spikes/.integrated/` show the register and report shapes that have worked. `aspectus --lines 300 --depth 4` on the repo before you ask it questions.

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

## Part 3 — things we routinely miss (the ⚠ ledger, for maintainers)

Collected so additions have a home; move items into the template proper as they earn slots:

- **Overstanding** (Joseph's manual correction, 2026-08-24): agents read the target segment and its `depends:` list, then miss the *sibling* Working Notes where the live tensions are recorded. The template's "overstand" paragraph is the current counter.
- **Contradiction-as-pre-execution-question** (Part 1 item 5) — the coordinator-side miss.
- **Quantifier-vs-flagship drill** — now also a Working-Notes regression guard in `#deriv-decomposition-uniqueness`; carried here because it generalizes beyond that segment.
- **Self-verification confidence**: an author's "gates clean" covers mechanical gates only; it has never once substituted for the independent re-derivation. Don't let a clean lint run relax the role separation.
- **Report-then-continue**: without the explicit grant, agents stand down at the first clean breakpoint; with it, the 2026-08-24 genericity chase happened at all.
- **Relay drift**: facts passed through two agents arrive as summaries; anything load-bearing gets quoted verbatim or read at the primary (this file's own §"why" quotes commits and audit IDs for exactly that reason).

*Living document — started 2026-08-24 from the decomposition-uniqueness cycle. Refine it the way the SOPs are refined: record the scar with the rule, so the next reader inherits the reason.*
