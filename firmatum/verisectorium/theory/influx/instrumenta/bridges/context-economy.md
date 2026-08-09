<!--
  Verisectorium theory INFLUX — instrumenta mining, verbatim copy.
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/udon-needs/02-tooling-needs/src/context-economy.md
  Copied: 2026-08-06 (mining agent, first outside reader of the theory corpus).
  This is a copy, not authority — do not edit here; segments draft from live sources.
  NOTE: the MOVED/ tree is mid-reorg; verify the source path before drafting from it.
-->

---
slug: context-economy
type: finding
register: evidenced   # the four families are observed in shipping practice + external measurement; the budget-is-structural backbone is derived from conditional theory; cards are proposed
support-kind: [observational, theoretic, measured]
strength: robust-qualitative   # the four-distinct-families finding holds across harnesses; the DL-budget backbone is conditional (theory, named premises) and the numeric thresholds/measurements are measured-with-caveats — all marked in prose
convergent: [observational, theoretic, measured]   # external measurement + shipped practice + theory; the 2000-line/50KB thresholds carry descent risk, the >85% and 5K->250K figures are measured-ext
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: both
depends: [tools-are-observation-infrastructure, persistence-is-imported]
sources:
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # C7, C8, C11, SWE-Pruner singleton
  - ../../01-ideation/02-provenanced/syntheses/asf-dossier.md  # obs-context-turnover DL budget; §4.2
  - ../../01-ideation/02-provenanced/syntheses/external-landscape-2026-07.md  # finding 7 (response-size hazard)
---

# Context is a hard budget, and the ecosystem built an economy around it

**Claim.** An agent's context window is one shared budget paying for three things at once — its plan, its picture of the world, and the task in front of it. The formal theory makes that a structural fact rather than a budgeting metaphor: window limits are a ceiling on sustainable plan complexity, and "context-stuffing helps to a point, then degrades" is an information-rate result, not folklore. Shipping practice, without citing any of that, has built a whole economy against the same budget — in **four distinct mechanism families that must not be merged**, because they fail differently and route to different repairs.

**What each leg carries.** This chapter makes several load-bearing claims at different strengths; each stands on its own support, and each has its own way of being moved:

| Claim | Support-kind | Strength | What would move it |
|---|---|---|---|
| The window is one joint budget paying for plan, world-model, and task; window limits ceiling sustainable plan complexity | theoretic (transmitted) | conditional (its named premises) | check the premises hold for real harness workloads |
| "Context-stuffing helps to a point, then degrades" is an information-rate result, not folklore | theoretic (transmitted) | conditional | as above |
| Four mechanism families exist and are distinct — they fail differently and route to different repairs | observational | robust-qualitative | find a harness whose mechanism collapses two families, or show one family's failure is a special case of another |
| Deferred tool loading: one origin plus two-or-three independent rediscoveries | observational | robust-qualitative (descent-corrected — not five votes) | trace the two suspected-independent arrivals to a common ancestor, or confirm their independence at source |
| Deferred loading yields >85% token reduction on tool definitions; accuracy degrades past 30–50 loaded tools | measured (ext) | measured — at the cited conditions | re-measure on other tool populations; the degradation threshold especially |
| A production tool response inflated 5K → 250K tokens | measured (ext) | measured (single published incident) | corroborate with further incidents, or bound how often it happens |
| ~2000 lines / ~50KB are the spill thresholds that recur | observational | heuristic — one ecosystem's shared-lineage defaults, not derived constants | check whether independently-built harnesses land near the same numbers |
| Capture, triage, and processing are separable stages; triage can read signal statistics without content | theoretic (transmitted) | conditional | as above |
| Whether structure-aware pruning beats question-driven small-model pruning | — | open (unmeasured) | the natural experiment, once structured payloads exist in a real harness loop |

## The four families

1. **Don't load it until needed — deferred tool loading.** Tools register by name only; the full schema is fetched on first use. The history, told straight: Claude Code originated the design; qwen-code explicitly mirrors it (and inherits its fork-parent's infrastructure); codex and kimi-code look like genuinely independent arrivals in the same 2026 window. One origin plus two or three rediscoveries — not five votes, but a real signal. The cited payoff: over 85% token reduction on tool definitions, with accuracy degrading once thirty to fifty tools are loaded at once.
2. **Don't let one result flood the window — spill to disk with a preview.** Around two thousand lines or fifty kilobytes — thresholds that recur across harnesses — a tool result stops arriving whole: the full output parks on disk, recoverable by path. External measurement supplies the cautionary tale for this mechanism's absence: a published incident of a tool response inflating from five thousand to a quarter-million tokens, in production.
3. **Shrink each result before it lands — content-aware pruning.** One research system runs a small model over raw tool output per call, skimming it against a declared focus question. A different lever than size-based truncation — content-aware, question-relative — and kept distinct because merging it with family 2 hides both its cost (an extra model in the loop) and its promise (relevance, not just size).
4. **Compact the conversation itself.** Two shapes ship: a structured state snapshot framed as the agent's *only* memory going forward, and first-person handoff notes. This family is where the [[persistence-is-imported| persistence chapter]]'s warning bites hardest — compaction that *replaces* history rather than pointing at it produced the false-confidence failure documented there.

Beneath all four, the theory adds an architecture note: capture, triage, and processing are separable stages — triage can read signal statistics without reading content, which keeps it cheap and honest — and quick-glance instruments differ from durable stores in their whole design physics, the same split the persistence chapter draws.

## What it generates

- **For the harness:** provision all four families deliberately — they are complements, not alternatives; a harness that optimizes only one (ever-more-aggressive compaction, say) inherits the failures the others prevent. Keep quick-glance instruments and durable stores architecturally distinct.
- **For UDON:** compact-but-legible is a theory-priced property — the honest frame for the size-comparison pitch. Spilled and deferred artifacts are documents an agent re-enters *cold*, so self-describing structure and stable addressing (the [[addressing-is-the-long-pole| addressing chapter]]) are what make a parked artifact usable rather than merely stored. And a standardized focused-subtree-plus-breadcrumb payload (the [[progressive-disclosure-read-path| read-path chapter]]) is the notation-side complement of family 3: relevance-shaped delivery without the extra model.

## What this opens (ideas, not designs)

> [!capability] Declared context budget **What:** a caller-declared token budget on every tool call, letting each tool pick preview depth, spill threshold, and pruning aggression per call instead of shipping one global compromise. (The [[headless-io-contract| headless-contract chapter]]'s agent-identification idea supplies the channel.) **Principles that apply:** the joint budget above; crystallized process (the tool absorbs a decision the agent now makes by guessing). **Hypothesized impact:** raises event rate ν (fewer turns burned on re-fetching what a right-sized response would have carried), and makes each family's contribution to the context description-length budget observable and tunable per workload. **In tension with:** tool-contract simplicity (one more field everywhere); the familiarity cost of deviating from installed shapes. **Potential downsides:** budgets become lies if agents cargo-cult a number; per-call variance complicates caching.

> [!capability] Spill with a table of contents **What:** family-2 spills park a *structured* artifact — a skeleton of addresses with the preview attached to the relevant node — so the agent expands precisely instead of re-reading linearly. **Principles that apply:** parked output is a document re-entered cold; addresses beat offsets; progressive disclosure. **Hypothesized impact:** cuts comprehension time on re-entry (the turnover-multiplied cost the theory prices), and lowers observation noise U_o on the parked channel — the artifact stops being a flood and becomes an addressable store. **In tension with:** family 2's simplicity (byte-truncation is format-agnostic; skeletons need a parse). **Potential downsides:** skeleton generation on huge outputs has its own cost; only pays where output has structure to skeletonize.

> [!capability] Focus questions as standard tool input **What:** the pruning family's declared focus question, promoted to an ordinary optional field on any tool — receivers may order, filter, or annotate output toward it, small model or no. **Principles that apply:** intent as parameter (this is intent's read-side twin); observation design over observation volume. **Hypothesized impact:** drives observation ambiguity A downward on the result channel (goal-relevant content forward, interpretive residue reduced) at near-zero mechanism cost — family 3's benefit given a no-extra-model floor. **In tension with:** the one-shot tool constraint (a bad focus question can't be renegotiated mid-call). **Potential downsides:** an over-trusted focus filter hides the unexpected — exactly what an exploring agent sometimes needs to see.

> [!capability] Compaction that emits both layers **What:** a compactor that produces its narrative summary *and* a machine-readable pointer map (claim → source span) in one pass — the summary for reading, the map for verification, neither pretending to be the other. **Principles that apply:** summaries must point at ground truth, not replace it; provenance as first-class text property. **Hypothesized impact:** strengthens the reinjection channel's integrity (the persistence condition's additive term) while keeping update-gain calibration honest after the boundary — the successor's trust in each claim becomes checkable, not vibes. **In tension with:** compaction's whole point (the map costs tokens; though it can live on disk, spilled like any family-2 artifact). **Potential downsides:** a wrong pointer map is worse than none — verification theater if unaudited.

## Honest edges

The numeric thresholds are one ecosystem's folklore (shared lineage, JavaScript-era defaults), not derived constants. Whether structure-aware pruning beats question-driven small-model pruning is unmeasured — a natural experiment once structured payloads exist in a real harness loop. And in-session retrieval machinery genuinely compensates for much of the budget pressure; the hard law lives at the session boundary, not inside it.
