# Glancing is not extracting — browse raw before aggregating

*Participant feedback (coord, 2026-08-10, from a steward observation mid-dialog). Complements `drafting-method-feedback-2026-08-09.md`, which teaches the flood-guard half only.*

**The observation.** Asked to glance over the influx for segment kinds, the coord ran only aggregation-shaped greps — `-c`, `-l`, `-o | sort | uniq -c` — every one of which strips the match from its surrounding line before the reader sees it. The steward's glance style is the opposite: bare `rg -i 'exempla'` and read the matching lines in context. The difference is not cosmetic: **the surrounding line is where surprisal lives.** A tally can only confirm or deny the frame brought to the query; browsed matches can catch the eye with what nobody was looking for. Aggregation-first glancing is the priors-shaped-sample blind spot in a rigorous-looking costume.

**The mechanism.** Trained output-flood aversion, over-applied: the flood-guard lesson (from the evidence-gather's first run) is correct for *extraction* at scale, but glancing is a different activity, and the reflex doesn't distinguish a 10,000-line flood from forty lines of high-signal prose. Doubly unfounded here: the harness truncates oversized tool results and parks the full output in a retrievable file, so the worst case for a bare grep is a bounded slice — the anxiety is priced for a harness we are not running in.

**The praxis pair, stated together:**

- **Glancing** (familiarizing, browsing, hunting the unexpected): bare `rg -i <term>`, read the lines in context; `| head -40` is the only guard needed, and even that is optional under a truncating harness. Aggregate only *after* browsing, when a count is genuinely the question.
- **Extracting** (bulk harvest, feeding files to agents, provenance sweeps): the flood guards apply — `rg -l` first, small `-n`, redirect big output to files.

Teaching only the second half produces exactly the failure the steward caught: an agent that guards floods it will never see while losing the serendipity channel that glancing exists for.

## Addendum (same day): the same correction, one level up — delegation vs direct reading under a 1M window

Minutes after the praxis above landed, the coord fanned four survey agents across the estate — and the steward named the larger instance of the same reflex: *"a whole lot of work and indirection and hazards of multiple chains of summary… when you could just do `rg -i -. -C3 '…'` and let the harness worry about overflow. Especially in the case where you have 1mil context window and you are specifically instantiated to see all of the details while being able to keep the big picture in mind."*

The delegable-discovery boundary (judgment stays home; discovery fans out) was calibrated for bounded context windows. Under a 1M-window coordinator, the economics invert for judgment-adjacent survey work: direct `-C3` browsing of live trees is lossless primary contact at trivial cost, while each delegation hop adds a summary in the proxy chain — the exact shape the estate's 2026-07-18 method-deficiency scar records ("built on summaries with the primary files unread"). What delegation still buys honestly: wall-clock parallelism, and *locator maps* into territory too large even for a big window. The praxis: **agent reports from survey fan-outs are maps, not synthesis inputs** — the synthesizing mind browses the primaries the maps point at, first-hand, before drafting anything from them. (This is [[claim-truth-over-proxy]] applied to one's own subagents.)
