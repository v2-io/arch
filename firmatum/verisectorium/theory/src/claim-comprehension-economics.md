---
slug: claim-comprehension-economics
form: claim
type-expected: derived
status: conditional
max: conditional
state: [drafted]
depends: [post-living-collection, post-total-turnover]
---

# Claim: Comprehension Economics

Comprehension cost compounds per reader while authoring cost is paid once per atom; under total turnover on an unbounded horizon the comprehension term dominates, which makes the collection's structure the reader's observation infrastructure.

## Formal Expression

*[Derived (comprehension-economics, from post-living-collection + post-total-turnover; conditional on (E1)–(E3) below)]*

The lifetime cost of an atom $a$ decomposes as

$$T(a) = t_{\text{auth}}(a) + \sum_{i=1}^{k} t_{\text{comp}}^{(i)}(a)$$

— authored once, comprehended by each of $k$ readers. Under [[post-total-turnover]] every session pays the collection-specific comprehension afresh; under [[post-living-collection]] the horizon is unbounded, so $k$ grows without bound for any atom that stays live. Therefore any one-time structural investment $t_{\text{invest}}$ that saves $\Delta t_{\text{comp}}$ per reader pays exactly when

$$t_{\text{invest}} \lt k \cdot \Delta t_{\text{comp}}$$

and the operating regime makes $k$ large by construction — so the inequality is *usually satisfied*, and the design default inverts from "author cheaply" to "spend authoring effort freely to buy per-reader seconds."

*[Discussion — the mechanism, transmitted from TST at its own tiers]*

The chain that makes "structure" quantitative: structural quality $Q$ sets the observation noise $U_o^{\text{(read)}}$ of the reading channel; the optimal update gain is $\eta^\ast = U_M/(U_M + U_o)$ ( [[emp-update-gain]], robust-qualitative at source); reading tempo is $\mathcal T = \sum_c \nu^{(c)} \cdot \eta^{(c)\ast}$ over channels (definitional). So the collection's structure is literally the reader's observation infrastructure ( [[der-code-quality-as-observation-infrastructure]], conditional at source), degraded structure depresses every future reader's effective tempo regardless of how fast they read, and "this corpus has become unnavigable" names a real threshold, not a mood.

### Conditions

- **(E1) Quality→noise monotonicity.** $U_o^{\text{(read)}}$ decreasing in structural quality — TST's own load-bearing premise, empirically motivated and unproven there; inherited here, not discharged.
- **(E2) Domain transfer.** The software-calibrated chain transfers to notes corpora on the properties they share: full-corpus inspectability with reader-bandwidth-limited observation (P1 analog), an exteriorized commit record (P5), and agent-controlled observation quality (P6 — the strongest analog, since formatting/structure/naming *are* the corpus's $U_o$ knobs). Weak or absent: executable counterfactuals and cheap interventional probes (P2/P3) — lint and checkers are thin analogs of a test suite. Consequences leaning only on P1/P5/P6 transfer; any consequence leaning on P2/P3 does not transfer without additional machinery.
- **(E3) Turnover regime.** $k$ large. In low-$k$ deployments (single author, short-lived notes) the authoring term competes and the claim's force weakens — consistent with the estate's paper-lite instances surviving happily without the full apparatus.

## Epistemic Status

Conditional, and `conditional` is also the ceiling: (E1) is empirical in character and (E2) is a transfer judgment — neither is removable by more derivation, so no amount of work makes this `exact`. Given (E1)–(E3), the derivation itself is elementary arithmetic over the two postulates; the substance is in the conditions. Transmission honesty: the load-bearing TST sources are conditional and robust-qualitative *at source* ( [[der-dual-optimization]], [[emp-update-gain]]) — "the theory grounds this" without naming (E1)–(E3) would be overclaim. Carriage was checked at drafting: the cited TST segments were read first-hand (2026-08-06). Evidence-action for tightening (per [[form-max-attainable]]): measure per-reader comprehension cost against structural variables on this estate's own corpora — a natural early [[form-observation-store]] query, since $k$ per region is exactly computable from the commit stream (TST P5).

## Discussion

**This claim is the theory's engine room.** Nearly every organ is a comprehension-cost answer once the inequality is accepted: stable slugs eliminate re-derivation of identity ( Organ I); the lexicon amortizes vocabulary re-acquisition ( II); honest per-atom epistemic state spares every reader re-adjudicating confidence ( III); views amortize ordering and audience-fit ( VI); the front door amortizes orientation itself ( VII); and [[claim-dispatch-compounds]] is this claim applied to queue surfaces, feedback loop included. The organs are where the $\Delta t_{\text{comp}}$ savings live; this claim is why they pay.

**The turnover multiplier's two forms.** For human teams, $k = (1+r) \cdot s$ (size × turnover). For AI-maintained corpora, $k$ is the number of sessions that touch the atom's region — the limiting regime in which comprehension dominance is not a tendency but the whole ledger ( [[der-dual-optimization]]'s finding: the 100%-turnover case is the *normal* case). A file touched twenty times predicts $k \geq 20$ future comprehension payments; the investment threshold is usually met by trivially small $\Delta t_{\text{comp}}$.

**What the inversion licenses — and what it does not.** It licenses explicitness over cleverness, in-prose glossing, mental-model-first framing, and intent-revealing structure as *temporal optimizations* rather than style preferences — respectful pedagogy is economics. It does **not** license unbounded atomization or padding: comprehension cost includes navigation and assembly, so grain and volume have their own trade-off curve — held open at [[claim-atomicity-parallelism]], not settled here.

## Working Notes

- Frontmatter schema provisional pending the epistemology decision.
- Open (transfer of the threshold dynamics): TST's vicious/virtuous quality cycles and the bifurcation around the persistence threshold are hypothesis-grade *at source*; whether corpus-structure decay exhibits the same bistability is untested here — [[claim-dispatch-compounds]] asserts the queue-surface instance on its own evidence, and the two should eventually cite a common dynamical treatment or explain their difference.
- Open (measurement): the $k$-distribution per atom region, from the commit stream — cheap, exact (P5), and the first number the evidence-action above needs.
- Drafting note against the outline row: the row's one-liner survives intact; what drafting added is the explicit (E1)–(E3) conditioning and the P2/P3 non-transfer boundary, which the row's "TST chain applied to notes corpora" was silently assuming.
