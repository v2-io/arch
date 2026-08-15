# Typed edges — relational state, derived from its consumers

*Register: **proposed**. The model's operations already run over edges (`depends:`, cascade on refutation, descent tracing, referent-resets, supersession) — currently untyped, which means every consumer guesses. This file types them. The adjudication method is the model's own individuation rule applied to edges: **an edge kind earns its place exactly when some consuming operation routes it differently** — so the set below is derived from the consumers, not chosen between the surveyed poles (a rich vocabulary vs a minimal three). It stays open the same way the dimension-set does: a new consumer that routes some relation differently admits a new kind.*

## The derived edge kinds

| Edge | Meaning | Consumed by | Composability *(declared per kind — the compound-error lesson)* |
|---|---|---|---|
| **supports** (`depends:`) | B is premise or ground for A | re-pricing; the lock (legs are supports with independent failure modes); the de-novo verifier path | strength **never** propagates transitively (support chains attenuate; mechanical chaining compounds error); *reachability* is transitive for notification only — a change deep below still signals upward |
| **rebuts** | R attacks A's *conclusion* — a counterexample, a refuting result | landing (collision surface; blocks "done"; the no-go protocol) | not transitive; not symmetric (R rebutting A says nothing about A rebutting R's premises) |
| **undercuts** | R attacks the *support* of A-through-B — the premise weakened or the inference doubted, A's conclusion possibly still true | re-pricing's soft path (below) | not transitive |
| **supersedes** — typed: *revised-by · invalidated-by · alternate-of*, each whole or partial | which record answers now, and why the old one stopped | serving (current-answer resolution); history | *revised-by* chains resolve transitively to the current record; *invalidated-by* and *alternate-of* do not chain |
| **seeded-by** | A's framing, design, or approach descends from S | the lock's independence check | **transitive — the one edge where propagation is the point**: descent genuinely flows (seeded by X, X seeded by Y ⇒ shares Y's failure modes), and truncating the chain is how fake independence gets manufactured |
| **narrates** | teaching or pointer material speaks about A | Freshness (referent-coupled reset); Comprehension's bounds | not transitive (narrating a narration is narrating *that record*, not its referents) |
| **restates** | same claim, different carrier | identity grain (linked, never deduped on sentiment); collision checking across carriers | *exact*-restates compose as an equivalence; *near*-restates deliberately do **not** chain — near-plus-near drifts (the mapping-relations lesson: the relation kind *is* the confidence statement, and transitivity is granted only where meaning survives it) |

Each edge is itself a small record: declarer, date, and — where contested — a minted question; an edge nobody stands behind is a guess wearing graph syntax. But edges deliberately do *not* get the full dimension apparatus — an edge's "state" is its type plus its declaration event, and over-building here would recreate the mixed-composite pathology one level down.

## Dependency re-pricing, made precise

The model's hardest open problem — *when a premise changes, how do dependents re-price?* — largely dissolves under two things this model already committed to, plus the rebut/undercut split:

1. **Because every status is a computed projection, re-pricing is not a mutation sweep.** Nothing stored on A needs editing when B moves; A's projections simply compute differently on next read. What re-pricing actually requires is **notification** (a signal that something upstream moved, so consumers don't serve stale computations) and **threshold detection** (did the recomputation cross a line that demands attention?) — both of which are MAINTAIN's existing machinery: a standing drain reads new events and mints signals.

2. **The two change-classes route differently — this is why rebut and undercut earn separate kinds:**
   - **B refuted** → every `supports` edge from B carries a **rebut-class cascade**: the securings of A that *used* B reset; if B was a necessary premise, A is marked known-broken *in the record, immediately, visibly* — a reader mid-repair must never be misled — and every dependent is found and cascade-marked or re-derived before the finding is routed onward.
   - **B weakened** (status dropped without refutation — a scope narrowed, a ceiling lowered, an era expired) → an **undercut**: A's conclusion is not attacked; A's projections that counted B at the old strength now compute lower, and where the drop crosses a serving threshold, a re-secure question mints. Soft, targeted, no false alarm — and no silent inheritance of strength that no longer exists.

3. **What remains genuinely open** — the honest residue, not dissolved: the *threshold* question (which recomputed drops warrant a minted question vs. silent lower serving) is a per-deployment economics call, kin to the gap-ordering in 07; and partial dependence (A uses one clause of B) requires grain on the edge — `supports` at clause grain — which the object model's grain parameter covers but no instantiation has exercised.

## What this file deliberately does not adopt

- **The full argumentation apparatus** (acceptance as a graph-wide fixpoint; preference edges): powerful, and unneeded until some consumer computes acceptance globally — no current operation does; admitting the machinery without a consumer would be formalism capture in edge clothing.
- **The minimal-three pole** (supports / contradicts / synthesizes): its third member is not an edge here — synthesis is the *reify* closure shape (mint a new record that `supports`-depends on its parents); collapsing supersession, descent, and narration into "related" would put every consumer back to guessing, which is the failure typing exists to end.

## Working Notes

- Provenance: rebut/undercut is adopted argumentation-theory vocabulary (the one distinction there whose consumers already exist here); the transitivity-per-kind discipline is the mapping-relations lesson; the supersession types are the provenance-vocabulary triple with the whole/partial grain from standards practice; descent-transitivity is this model's own commitment (the lock's look-through, extended along chains). Per-element trace lands in 90.
- Open: whether `seeded-by` wants a *strength* (heavily-seeded vs a passing glance) or stays binary with the lock erring conservative. Binary until a real lock evaluation demands otherwise.
- Open: edge direction conventions at the syntax level (who declares — the dependent declares `supports`; the refuter declares `rebuts`; the narrator declares `narrates`) — a 09-language surface question, noted here so the language inherits an answer rather than a gap.
- The `restates` exact/near split needs its discriminator stated when first exercised: candidate — exact iff a collision on one carrier *is* a collision on the other.
