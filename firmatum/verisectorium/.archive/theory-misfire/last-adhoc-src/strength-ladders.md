---
slug: strength-ladders
type: survey
depends: []
---

# The estate's strength vocabularies, side by side

*At least five incompatible ways of saying "how much should you believe this" are live in the family; none is a superset, each encodes something the others drop, and the adjudication of what a shared kit offers is owed and unmade.*

## The inventory

**1. ASF's single `status:` scalar** (8 values: axiomatic · exact · robust-qualitative · heuristic · conditional · empirical · discussion-grade · sketch). One word carries the whole epistemic position. Strengths: compact, enforceable, deeply exercised (243 live segments as of 2026-08-06, excluding old-* archaeology); the vocabulary is itself stored as adjudicated terminology entries with recorded rationale. What it drops: *how* the claim is supported and *what kind* of evidence — a `heuristic` from lived practice and a `heuristic` from one derivation-sketch read identically.

**2. udon-needs' three axes + convergence** (`register` × `support-kind` × `strength`, plus `convergent:` listing which independent evidence kinds agree, plus an embedded dated `verified:` event log). Strengths: the failure-mode-independence idea made machine-readable — agreement counts only across kinds that fail differently; support and strength can move independently, which is how the axes were discovered to be distinct. Costs: heavy frontmatter; free-text drift observed in its own `stage` values; 30 segments of exercise, one corpus.

**3. neurips-adjudicated's rung tags** (`[PROVED]`, `[TESTED] 3×`, `[JUDGMENT]`, `[PDF]`, `[HYPOTHESIS]`). Strengths: the densest form per character; a replication count *inside* the token; born under adversarial pressure (review responses) where every rung must be defensible line-by-line. What it drops: everything else — no process state, no support kind beyond the rung.

**4. comproprium's per-directory ladders** (practices: fired · fired-once · proposed · failed-to-fire · retired; accounts: attested · reconstructed · secondhand). Strengths: the ladder is about the *right property per type* — demonstrated firing for a practice, testimonial distance for an account — the clearest estate evidence that one ladder cannot serve heterogeneous atom types. Plus **`:max-attainable`**: a declared *ceiling* separate from current status, distinguishing not-yet-verified from not-verifiable-by-this-kind-of-claim — a column no other vocabulary has.

**5. The equation-grade inline tags** (ASF's `*[Derived]*`, `*[Hypothesis]*`, `*[Empirical Claim]*` at equation level, and the udon theory corpus's per-clause tagging). Strengths: strength attaches to the *clause*, not the file — a segment whose (A) is definitional and (B) is heuristic says so per part. This is finer grain than any frontmatter field can carry.

## What the comparison establishes

- **Grain varies by an order of magnitude** (clause → segment → directory-type), and the fine grains are doing real work where they exist — collapsing to per-segment loses the (A)-vs-(B) distinctions the best segments rely on.
- **Three genuinely non-redundant dimensions recur**: strength (how settled), support-kind/register (settled *by what*), and ceiling (settleable *how far*). Every vocabulary carries the first; only 2 carries the second structurally; only 4 carries the third at all.
- **Compactness and honesty trade off visibly**: 3's tags survive adversarial review precisely because they claim little; 2's vector is the most honest and the most expensive to maintain.
- Any shared kit therefore faces a real choice, not a merge: per-deployment vocabularies over a small shared *dimension set* (strength / support / ceiling, each optional) is the shape this survey's evidence points at — but that is a reading, and the adjudication is Joseph's and the joint session's, not this survey's.

## Method & scope

All five read first-hand 2026-08-05 in their live homes (asf FORMAT + terminology; udon-needs `02-tooling-needs/src/` frontmatter; neurips `*/adjudicated/`; comproprium FORMAT D4 + segments; equation tags in asf NOTATION/practice and udon theory segments). Counts are as-of that date. This is an inventory with a stated lean, not the adjudication itself.

## Working Notes

- Feeds [[epistemic-axes]] and the ch. 8 calibration cluster; [[verbal-label-calibration]] adds the orthogonal finding that whatever words are chosen, their read-meaning is uncalibrated.
- TODO entry R-T7's strand-A frontmatter (confidence + decay-rate + derivation DAG, 2025) is a sixth vocabulary, unexamined — its numbers were demo values, but its *dimension choice* (temporal decay) appears in no live ladder and bears on [[temporal-truth]].
