---
slug: form-agentic-eyes
form: form
type: formulation
max: decided
state: influx
decided-by: supported
---

# Formulation: the design of agentic eyes

*The master design for agent-facing glance surfaces — the channels ↔ affordances ↔ concerns ↔ options picture. A foundation doc for every look-shaped tool in the suite (first consumer: aspectus; the principles generalize to any surface whose reader is a token-stream mind). Synthesized 2026-08-14 from the day's decided material; provenance per section; Joseph's framing: "a critical upgrade for the 'eyes' of agents, which, biologically speaking, is no small thing."*

## What a glance surface is

A glance surface is **observation infrastructure**, and its quality is existential, not ergonomic (AAT: bias is bounded by κ×A, and observation ambiguity A is the one designer-controllable knob; you cannot out-iterate a bad channel). A retina, biologically, is not a passive sensor — it hands the mind edges, motion, and contrast *pre-computed, before attention spends anything*. That is the design target: the look does the pre-processing a cold mind would otherwise spend its context reconstructing badly. (The phylogeny is Joseph's image, and it grades the existing tools exactly: *"lower life-forms that can't see other than light-intensity, and just grope around blindly and with vague shapes and shadows"* — `ls` is the light-patch, `tree` the cup-eye; the leap that makes a scene out of an image is **motion** — the aliveness channels — which is what eyes were for all along.)

**The governing principle:** minimum surprise from the tool; maximum surprisal per glyph in the look. Its two inseparable duties (Joseph): *turn unknown-unknowns into unknowns* — mass, censuses, denial marks, so the reader's sense of what it has NOT seen is calibrated — *and show the place as a living, evolving system* — recency, heat, movement, so the glance answers "where is this alive" in the same pass.

**And the mode of use the duties imply** (Joseph, on the field praxis that formed): the glance is **perceptual, not interrogative**. Using it to check a specific claim is derivative — even a falsified absence-claim is still "a perceived known" being verified. *"The bigger thing that aspectus gives is 'where are your unknowns' — which means being used not as a check for something specific, but a 'look at the landscape before asking it a question' — hence 'anywhere you go; and if not, why are you choosing to put your hand in someplace without looking first?'"* The look precedes the question the way perception precedes reach; a question formed before looking is shaped by priors, and the landscape reshapes which questions exist. This is the eyes-not-oracle distinction at the level of *when*: the query-for-files law says what the tool returns; this says the glance comes before you know what you'd query for.

## The channels — the physical media a text look owns

A token-reading mind has a retina made of regularity rather than pixels — Joseph measured agents' spatial awareness in one of his earliest LLM experiments, and it shaped udon, aspectus, and tablecop (2026-08-22 correction of this sentence, which had said "no retina": a deflationary absence-claim, wrong in the way that matters; aligned delimiters and consistent whitespace runs are *read* structurally, ragged ones are *parsed*). A text surface has independent information channels, each with its own carrying capacity, cost, and natural tenants. Spending them deliberately is the whole craft:

| Channel | Carries best | Cost | Evidence / prior art |
|---|---|---|---|
| **Vertical position** (sort order) | one ranked register — the default is *recency*, so re-calling the look shows movement with zero state | free | git-heat's order-by-last-edit; the recency-default decision |
| **Horizontal position** (placement classes) | *which fact* a value is — decoration / far-left / near-left / near-right / far-right / glyph-block | free once aligned | shorthand design; `ls -l`'s block |
| **The aligned edge** | separability at a glance — and *per-kind* tab-stops make the silhouette itself carry node kind ("how much of this screen is directories") | free | Joseph's empirical: "all modern agents LOVE column alignment"; git-heat's pulled-in file column |
| **Glyph density** | magnitude, perceived not computed — logarithmic | ~5–20 glyphs | SIGNA; TicToc's finding that raw timestamps fail (~65% even with them) |
| **Glyph identity** | category and grade (★★★ vs ✫ vs ✭); position-in-block = fact, glyph = value | 1 glyph/fact | permissions blocks; the glyph-block office |
| **Presence/absence** (quiet) | surprisal itself — a fact that appears *is* the message | zero when silent | quiet office; "644 prints nothing" |
| **Aggregation** (censuses, mass) | the size and kind of the unseen | 1 line or cluster | dir/leaf census, deep-agg; the "300 segments" fix |
| **Pairing/clustering** | related facts as one glance-stop (`score · age`) | shared width | git-heat's cluster |
| **Color** (TTY only) | a redundant magnitude/kind overlay — never the sole carrier | free, evaporates in pipes | git-heat's ramp; `--color=auto` law |
| **Plain number/text** | precision, when the reader will actually compute | width | the fallback, not the default |

Two registers per fact is normal and *good* — position for one reading, value for another (git-heat: position = most-recently-changed, score = has-been-hot; an anti-collapse pair — never fuse them into one "activity" number).

## The concerns — laws that bound every option

1. **Determinism.** Every channel is a pure function of (tree content, caller state). Never locale, readdir order, or the place's own preferences. Two runs are byte-identical; two agents with one config see one look. *(Corrected 2026-08-22 — the original also said "never terminal width"; that was agent-authored overreach, never Joseph's: a human caller's terminal width **is** caller state — "precisely the thing an agent/human should set about his/her view/eyes" — so it may be the default for a `width` setting; what the law forbids is the *place* tuning the channel, not the caller's own eyes.)*
2. **Only the caller tunes the channel** ([[norm-caller-tunes-the-channel]]). The observed may speak into the look as *content*; it never sets columns, budgets, thresholds, or formats.
3. **Never lie by omission.** Truncation, denial, bounds, and hiding all confess — typed censuses, `≥`, `[denied]`, `[walk bound]`, kind-marks for hidden furniture. A look that ran out of anything says so where a reader will see it.
4. **Absent, never faked.** An unobtainable fact (no btime, outside git, unreadable) prints nothing and claims nothing. A `0` that means "not a thing that has this fact" is a lie.
5. **Glyphs are interfaces.** A learned glyph or placement is expensive to change — vocabulary is taught in `--help` (the law channel), ratified before it ossifies, stable across releases.
6. **Machine formats get facts, not affordances.** JSON carries fields (bytes, booleans, ISO); glyphs, alignment, SIGNA, and color exist only for the perceptual look. Same look, two renderings — machines never get a *different* look, only a different encoding.
7. **Default glyphs are earned.** Every default-ON fact pays rent in surprisal for a cold reader; everything else is QUIET (speaks when surprising) or compose/config. Flag museums refused.
8. **Attention is the scarce resource, lines are its budget.** The allocator spends every line it can on the highest-value tenants (capacity-aware redistribution) and names what it couldn't spend.
9. **Data carries its units and labels in the look itself.** Joseph (2026-08-14, after hitting unglossed `767`/`1.01` cold — the finding's third independent arrival): *"Data without units or labels or context is just another area ripe for confabulation and misunderstanding — potentially correct, but when wrong it's wrong in the most dangerous way."* An unlabeled number is a maximally ambiguous observation: the reader's priors fill the gap, the fill is phenomenologically indistinguishable from reading, and a wrong fill raises no error. Column headings, unit glyphs, and first-contact glosses are not decoration for novices — they are A-reduction on the channel's most confabulation-prone fact class. Familiarity never repeals this (*"the day will come when agents are so familiar… that the header will seem superfluous, but not for a long time"* — and even then, the cold reader is always one turnover away).

## The affordance ↔ concern map (how options get generated)

When a new fact wants into the look, walk this, in order:

1. **Which duty does it serve** — calibrating the unseen, or showing aliveness? (Neither → it likely wants compose-only.)
2. **Which register(s)** — magnitude, category, recency, identity, anomaly? Registers pick channels: magnitude → density/far-right cluster; category → glyph identity/decoration; recency → vertical position first; anomaly → presence/absence.
3. **Which channel has spare capacity at the default?** Position and silence are free; glyphs cost; lines cost most. Prefer the free channels; a fact that only works as a wide column probably waits for an ask.
4. **What does its absence mean?** Design the nothing-case first (quiet, omit, absent) — the something-case then carries surprisal by contrast.
5. **What is its honest degraded form** under bounds, denial, cache-staleness? (`≥`, `≈`, absent — decided before it ships.)
6. **Does it aggregate?** If the fact is meaningful summed/maxed over a subtree, claim its census/deep-agg office in the same design — aggregates are the mission.

## Provenance

Decided-by `supported` under Joseph's explicit ask for this document (2026-08-14); every load-bearing claim above descends from same-day steward decisions or shipped prior art: the aspectus lattice rework and its Reasoning, [[../../utils/aspectus/design/shorthand.md|shorthand]], [[../../utils/aspectus/design/heat.md|heat]] (git-heat affordances), [[form-signa-notation]] / [[norm-elapsed-time-is-perceived]], [[norm-caller-tunes-the-channel]], the walk/denied/census honesty family, and AAT's observation-ambiguity law (`#scope-observation-ambiguity-modulation`) as the formal ground. Corrections welcome at the section level — the channel inventory and the walk-order are the parts most likely to grow.
