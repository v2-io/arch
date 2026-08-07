---
slug: claim-dispatch-compounds
form: claim
type-expected: derived
status: heuristic
max: robust-qualitative
state: [drafted]
depends: [def-integration-replacement, form-influx-membrane]
---

# Claim: True Dispatch Compounds

Each item truly adjudicated and dispatched from a live surface permanently reduces every future agent's search space there — and because the drain rate itself depends on surface cleanliness, the dynamic bifurcates into compounding speedup or widening loops that multiply chaos.

## Formal Expression

*[Claim (linear half — the per-reader tax)]*

Let a live surface (an influx queue, a working directory, a tracker) hold $n$ items, of which $r$ are residue: breadcrumbed, half-dispatched, or integrated-but-still-present. Every future session that searches the surface scans past the residue, so the residue's cost is paid **per reader**: over $k$ future sessions, roughly $k \cdot r \cdot c_{\text{scan}}$, against a dispatch cost paid **once**. This is [[claim-comprehension-economics]]'s turnover multiplier applied to queues — the same asymmetry (per-reader recurring vs per-act once) that makes comprehension investment dominate under total turnover makes true dispatch dominate on live surfaces. A truly dispatched item, by contrast, *permanently* shrinks the search space: every subsequent search over the surface is faster, and the saving also compounds per reader.

*[Hypothesis (dynamical half — the bifurcation)]*

The drain *rate* is not independent of the surface state. Search cost rises with residue, so:

- **Clean attractor:** clean surface → searches terminate fast → adjudication and dispatch are cheap → the surface stays clean and each dispatch makes the next cheaper — compounding speedup.
- **Residue attractor:** residual surface → searches widen → agents find nothing, re-find nuggets that already landed, or find **false nuggets** — half-dispatched items that read as live work → re-adjudication and re-billing of settled questions multiply the residue → wider loops still.

False nuggets are the sharpest mechanism: they are worse than noise because they *reward* the widened search — the agent that dug through residue and "found something" is reinforced toward digging, even though the find was settled or spurious. Noise merely wastes a search; a false nugget trains the loop.

Structurally this is the TST code-quality bistability ( [[der-code-quality-as-observation-infrastructure]]'s vicious/virtuous cycles) with the live surface playing the role of the drain agent's *observation infrastructure* and residue playing the observation noise $U_o$: quality of the surface sets the gain on every future look at it, and near the threshold the system is unstable — small perturbations tip it toward one attractor or the other.

## Epistemic Status

Two halves at two tiers, deliberately not averaged. The **linear half** is *derived, conditional* on [[claim-comprehension-economics]] (itself conditional on its TST grounding): granted that surfaces are scanned per-reader under total turnover, the per-reader tax and the once-paid dispatch follow by the same argument as comprehension-cost dominance. The **bifurcation half** is *hypothesis-grade*: structurally motivated by the TST twin and consistent with the estate's lived specimens (below), but the feedback loop — that residue measurably slows drain, which measurably grows residue — has not been instrumented anywhere. Max attainable: `robust-qualitative` (the quantitative forms are unmeasured and likely deployment-dependent; the qualitative bifurcation is the claim worth defending). The falsifiable structural prediction it inherits from its TST twin: surfaces near the threshold should be *unstable* — observed corpora should cluster at the attractors, with few long-lived in-between states. Same-estate evidence throughout; the specimens share one steward and are coherence, not independent corroboration.

## Discussion

**Why this makes the delete-test economics, not tidiness.** [[def-integration-replacement]]'s delete-test is the *criterion* for true dispatch; this claim is the *argument* for enforcing it strictly. A lenient reading ("mostly landed, breadcrumb the rest") looks like a small local saving and is, under the per-reader tax, a recurring charge against every future session — and under the bifurcation, a step toward the residue attractor. The archive escape is what makes strictness affordable: `.integrated/` retention preserves the reasoning trail *off* the live surface, so the search-space reduction costs no information. The triple — delete-test as criterion, dispatch-compounds as economics, dot-directory archive as the preservation escape — is one complete argument.

**Both attractors have estate specimens.** The residue attractor's clearest specimen is the reverted batch that ratified the delete-test: files declared integrated on the strength of TODO entries describing their remainders — a false-nugget factory in the making, since every future drain agent would have re-found "integrated" items whose information was not landed, and the finds would have read as live work. It was caught by steward inspection before the loop widened, and the state was reverted. The clean attractor's specimen is this corpus's own founding influx: payloads adjudicated and moved to `.integrated/` as their content landed, the index stating at each moment what remains — and each dispatch visibly shrinking what the next session must consider. Single-estate, steward-shared, offered as illustration rather than proof.

**Consequences for drain-agent design.** If the bifurcation holds, the highest-leverage act on a residual surface is not "process items faster" but *restore the surface to the clean basin* — a bounded task-force clearing, after which steady-state healing (drain-one-extra per [[claim-clocked-drains]]) holds the clean attractor cheaply. This matches the estate's independently-arrived process prescription (task-forces only for tangled backlogs; healing everywhere else) and gives it a mechanism: healing is cheap *because* the clean attractor makes each dispatch cheap; task-forces are needed *because* the residue attractor makes incremental drains net-lose against the widening loop.

**Scope honesty.** The claim concerns *live surfaces agents must search* — influx queues, working directories, trackers. It does not argue against history layers, archives, or event trails, which are off the search path by construction; and it does not quantify the threshold, which plausibly varies with surface size, search tooling ( [[form-corpus-verbs]] shift it favorably), and agent context budgets.

## Working Notes

- Steward-primary reinforcement located after drafting (DISCUSSION-THOUGHTS.udon O16, 2026-07-29): body-breadcrumbs named as collision-surface contamination and the instinct behind them as *pre-paid capture energy mis-routed* — [[form-crumb-routing]] is the constructive complement to this claim's prohibition (the crumb is routed to the event trail, not suppressed).

- Frontmatter schema provisional pending the epistemology decision.
- Evidence-action (per [[form-max-attainable]]): the observation store ( [[form-observation-store]]) could instrument this cheaply — per-session time-to-first-relevant-find on influx surfaces, residue counts over time, and re-billing events (a settled question re-adjudicated) as the false-nugget signature. A measured drain-rate-vs-residue curve on one instance would raise the bifurcation half to `empirical`.
- Open: whether "false nugget" deserves its own named atom under Organ IV (it is an adjudication hazard, not only a flux one) or stays a mechanism inside this claim. Watch whether other segments need to cite it independently.
- Regression guard: do not soften the delete-test's strictness on the strength of the linear half alone ("the tax is small for small $r$") — the bifurcation half is precisely about small residue growing, and the founding specimen was small when caught.
