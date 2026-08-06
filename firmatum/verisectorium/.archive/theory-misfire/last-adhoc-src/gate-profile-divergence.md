---
slug: gate-profile-divergence
type: obs
depends: []
---

# One store design, two enforcement regimes — the refs pair

*The full account of two bibliography stores of a single documented lineage running the same layout and the same tooling at opposite verification disciplines — the observation grounding [[verification-provenance]]'s claim that whether a check gates belongs to the consuming deployment, not to the field or the schema.*

## The pair

The estate's bibliography stores descend in a documented line: a per-entry YAML layout with an atomicity contract, append-only verification events in sibling directories, a lint, and an emitter that generates the citable artifact. Two members of that line sit in projects with different consumers.

One serves a machine-learning conference umbrella. Its lint warns. The other serves the academic-philosophy portfolio, where papers are prepared for journal submission — and there its lint **gates**, because it is the anonymization check run before submission. The adaptation note on the second is explicit that the data layout, the atomicity contract and the CLI verbs transferred verbatim while the build pipeline did not.

The reason for the divergence is stated by the steward (2026-07-23) as a general rule: whether a check gates tracks the consuming deployment's **stakes and reversibility**. A polish level is revisable at any time, so gating on it costs more than it protects. A name that should have been anonymized and was not cannot be un-submitted.

## The counts

| Store | Entries | Verification directories | Share |
|---|---|---|---|
| conference-umbrella refs | 188 | 13 | ~7% |
| philosophy-portfolio refs | 97 | 70 | ~72% |

As reported by the live-instance survey of 2026-08-05, at that survey's standing — the counts were not independently re-derived when this observation was written.

## Method & scope

Directory counts over the two stores' `entries/` and verification trees, taken by a survey agent walking the live trees on 2026-08-05. Two stores, one lineage, one day.

The comparison is unusually clean because the machinery is shared: this is not two teams with different habits but one design deployed twice, so the ten-fold difference in verification coverage cannot be attributed to tooling. It can, however, be attributed to more than stakes alone — sample size, corpus age, and how close each project was to a submission deadline on the day of counting are all uncontrolled. What the specimen shows is that identical machinery sustains both regimes; the stakes-and-reversibility account of *why* is the steward's, offered as the reason rather than demonstrated by these counts.

## Working Notes

- Cheap strengthening if wanted: re-run the counts, and add the date of each store's last submission event. If coverage in the gating store spikes near submissions and decays after, that is the mechanism visible in time series rather than in a single snapshot.
- The second, independent axis these counts say nothing about: whether the system may act *without a human*, which tracks evidence grade rather than stakes. Both stores run at the same setting on that axis as far as this observation can tell.
