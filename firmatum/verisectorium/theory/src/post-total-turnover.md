---
slug: post-total-turnover
form: postulate
type-expected: postulate
status: axiomatic
max: axiomatic
state: [drafted]
depends: []
---

# Postulate: Total Reader Turnover

Every reader arrives with total turnover of collection-specific context: nothing a session understood about the collection survives the session boundary except what was exteriorized into the collection itself.

## Formal Expression

*[Postulate (total-turnover)]*

A reader's state at session start decomposes into two components with opposite fates:

1. **Substrate** — pretrained weights and general competence (language, code, reasoning patterns, estate-independent knowledge). Survives every boundary; arrives for free with every reader.
2. **Collection-specific model** — what this collection currently claims, has decided, has named, has retired, and how it works here. Does **not** survive: it is reconstructed each session, and the only reconstruction sources are the collection and its durable surrounding surfaces.

The postulate: **the collection-specific component of any reader's understanding crosses a session boundary only through exteriorization.** Equivalently: the collection is the sole carrier of collection-specific knowledge across sessions.

*[Discussion — the refinement that keeps the postulate honest]*

Treating session start as "model ≈ zero" is wrong, and TST says so explicitly ( [[obs-context-turnover]], [[scope-developer-agent]]): effective state at session start is reconstructed from external memory plus the session prompt plus the pretrained weights, so uncertainty starts *elevated*, not blank-slate. The postulate is deliberately scoped to the component weights cannot carry — everything minted, decided, or corrected after the weights froze, and everything deployment-specific regardless of age. That component's turnover is total; the substrate's is zero; conflating the two produces both failure modes (over-orientation that re-teaches what weights already know, and under-orientation that mistakes substrate fluency for collection knowledge).

*[Discussion — regime]*

Human readers instantiate the postulate at rate $r \lt 1$ per interval (forgetting is turnover on a slower clock); AI sessions instantiate the limiting case $r = 1$ per session. Per [[der-dual-optimization]], the limiting case is the *normal operating regime* for these collections, not an edge case.

## Epistemic Status

Postulate — near-definitional of the operating regime, and anchored in substrate fact rather than assumption: context windows are bounded and session-scoped, and the weights predate the collection's present truth by construction. The decomposition and the elevated-not-blank refinement are transmitted from TST at that segment's own tier; the postulate's own content is the exteriorization-only survival claim, which is not currently falsifiable so much as escapable — a substrate change (persistent cross-session memory, continual learning) would shrink the collection-specific-loss component, and the postulate would then scope to whatever remains non-persistent. That is a named future weakening, recorded here so it is a scoping event when it arrives rather than a surprise.

## Discussion

**The two-postulate engine.** With [[post-living-collection]] this forms the theory's engine: the collection outlives every reader, and every reader loses collection-specific state — therefore everything load-bearing must be exteriorized, and the nine organs ( [[def-verisectorium]]) are the load-bearing answers to *what*, exactly. Every downstream economic claim ( [[claim-comprehension-economics]], [[claim-dispatch-compounds]]) quantifies a consequence of this pair.

**What the substrate carries is why exemplars govern.** Because general pattern-competence survives every boundary, a corpus teaches by example more forcefully than by rule — new readers pattern-match existing atoms against structures the weights already hold. The exteriorization burden is correspondingly specific: identity, present truth, epistemic state, decisions with reasons, norms with scars, orientation. Style takes care of itself; doctrine does not.

**Fluency masquerades as orientation.** The substrate's zero-turnover half makes agents *fluent* immediately — forms flow, vocabulary lands — while the collection-specific half is still absent. The feeling of orientation therefore systematically overstates the substance, which is the failure the orientation gate ( [[form-orientation-gate]]) exists to catch and the reason its doctrina half targets collection knowledge rather than format competence. The theory-misfire/refounding comparison is the estate's sharpest specimen: same substrate fluency, categorically different collection-specific orientation, categorically different outcome.

**The steward turns over too.** On a longer clock, human memory decay makes the steward a slow-turnover reader of their own estate — the unread-corpus surface of [[form-steward-model]] is this postulate applied to Organ VIII.

## Working Notes

- Frontmatter schema provisional pending the epistemology decision (see OUTLINE working notes).
- Open: whether a formal AAT statement should be cited rather than gestured — [[scope-developer-agent]]'s $X_{\tau_{k+1}} = f_{\text{init}}(\mathcal E_{\text{ext}}, p_{k+1}, M_0^{\text{weights}})$ already formalizes the reconstruction; a transmitted citation with the carriage note would be cheap and sufficient.
- Open (boundary question for [[def-verisectorium]]): "durable surrounding surfaces" (global memory, harness config, CLAUDE fanouts) are read here as the collection *by extension* for survival purposes — whether they are inside the deployment boundary proper is an Organ IX front-door/instantiation question, not this postulate's to settle.
