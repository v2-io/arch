---
slug: headless-io-contract
type: finding
register: evidenced
support-kind: [observational, design]
strength: robust-qualitative   # a converged contract shape; direction firm, no magnitude
convergent: [observational, design]   # one of only two patterns certified as GENUINE INDEPENDENT ARRIVAL (survives the descent correction) - the observational leg is at full weight here
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: both
depends: [method-evidence-tiers]
sources:
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # C16
  - ../../01-ideation/02-provenanced/syntheses/tier2-lineage.md        # C16 survivor
  - ../../01-ideation/02-provenanced/syntheses/CONVERGENCES.md          # cluster 18
---

# The headless I/O contract — the machine caller's bill of rights

**Claim.** When the caller of a command-line tool is a program or an agent rather than a person at a terminal, the tool's contract converges on a specific shape: **standard output carries data and nothing else; diagnostics go to standard error; a flag family selects machine formats, with a streaming newline-delimited-JSON variant; failures exit non-zero carrying a structured error object; there is a single-shot prompt-and-exit mode and a dry-run mode; and the tool detects for itself whether a human or a machine is calling** (terminal and CI environment signals), so the machine path doesn't depend on the machine knowing the flag. This is one of only two patterns in the whole shipping ecosystem that the [[method-evidence-tiers| methods chapter]]'s copying-vs-invention accounting certifies as *genuine independent convergence*: essentially every harness built it separately, forced by the same hard external constraint — a machine on the other end. That is the strongest evidence shape shipped practice can produce.

## The evidence

- **Shipped, independently, everywhere:** effectively all fourteen harnesses examined carry the contract (the one exception is a genuine absence, recorded as such). The cleanest standalone statement of the detection heuristic is one CLI's pair of checks — "am I attached to an interactive terminal?" and "am I running under CI?" — with everything downstream keyed to the answer.
- **Anticipated from first principles (2025):** the tooling conventions reproduced in [[quick-tooling-conventions| the quick-tooling report]] specified the same contract — data on stdout, diagnostics on stderr, standard exit-code vocabulary, a JSON format flag, terminal detection — before most of these harnesses existed. The clearest case in this report of design thinking anticipating practice. (Same-author caveat applies to the anticipation; the shipped convergence carries the evidential weight.)
- **A related one-off worth its sentence:** one CLI exports *its own tool schemas* — describing itself as an agent-callable tool, inferring JSON types from its flag definitions. A single occurrence, but it points somewhere (see below).

## What it generates

- **For the harness — and for every tool either consumer ships:** this is the floor contract; deviations need reasons. The auto-detection half matters as much as the flag half: agents should not need tribal knowledge to get the machine path.
- **For UDON:** two distinct pulls. (a) Every UDON command-line tool — the formatter, the validators, the eventual edit tool — inherits the contract verbatim, including that *diagnostics* (anomalies, verdicts) belong on the structured channel with stable codes; UDON's own severity discipline (a warning means content was kept, an error means something was lost) slots directly into the structured-error object. (b) The contract points at a payload-shaped question, stated carefully: newline-delimited JSON is the converged streaming answer *because JSON has no honest partial form*. A format whose prefixes parse (the [[streaming-and-partial-documents| streaming chapter]]) solves the payload-validation half of that problem — but the newline framing is also doing *transport* work (framing, sequencing, one-record-one-event) that prefix-parseability does not provide. The two are complements until a protocol experiment shows otherwise — a dissent from a reviewer outside this model family, adopted as this report's working frame ([[counter-register| counter-register]], row 10).

## What this opens (ideas, not designs)

- ✦ **An agent-identification convention.** Terminal-detection infers *that* a machine is calling; nothing today lets the caller *say who it is and what it can consume* — an environment convention through which an agent declares itself (and, say, its context budget or format preferences) would let tools adapt output richness deliberately instead of guessing from TTY absence.
- ✦ **A shared error-code registry per tool suite.** Structured errors with stable codes, held in common across a whole suite, would make the [[errors-that-teach| refusal chapter]]'s law-accumulation idea practical: laws learned from one tool's refusals transfer to its siblings because the vocabulary is shared.
- ✦ **Dry-run as machine-readable plan.** The dry-run flag exists everywhere; its output is usually prose. A dry run that emitted the *structured would-do* — same schema as the real run's result, marked hypothetical — would give agents the predict-failure-before-execution affordance the 2025 conventions asked for, as data rather than as reassurance.
- ✦ **Self-describing tools, universalized.** The schema-export one-off generalizes: if every CLI shipped its own agent-callable tool definition, harnesses would *discover* their toolsets rather than hand-author them — and the [[tool-definition-anatomy| tool-anatomy chapter]]'s single-source-contract idea  
  gets its distribution mechanism for free.

## Honest edges

The contract is about *transport*, not content quality — it says nothing about whether what's inside the JSON is right (the [[counter-register| counter-register]]'s validation-doesn't-catch- plausible-wrongness row bounds that hope). And "near-universal" is still a coding-harness sample; agent tools outside the coding world are unsampled here.
