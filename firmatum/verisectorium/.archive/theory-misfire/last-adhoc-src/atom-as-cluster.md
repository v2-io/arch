---
slug: atom-as-cluster
type: form
depends:
  - atom
---

# An atom is a cluster of parts on different clocks

*What a slug names is not one block of prose but a small cluster — the present-truth body, forward-looking working notes, a history of events, and companion material — and the parts change at different rates, under different rules, for different readers.*

## The claim

**(A) The cluster.** In every live instance of this pattern, the thing addressed by one identity has parts: a **body** stating present truth; **working notes** carrying forward residue; **events** recording what was decided or checked and when; and **companions** (specimens, sources, generated views) that the body stands on or feeds. Some instances put all four in one file (asf and vivarium segments carry body + Working Notes in the markdown, events in a changelog elsewhere); some split them across the filesystem (the terminology store keeps `entries/<slug>.md` beside `decisions/<slug>/`; relata keeps entries beside `verifications/<key>/`); one puts an event log inside the record's own frontmatter (udon-needs' `verified:` list). Same cluster, three layouts.

**(B) Layout and visibility are two questions, not one.** *Layout* — same file, sibling file, child directory — follows from whether a part changes in the same atomic changeset as the body. *Visibility* — canon, live-but-not-canon, historical, derived — is a separate property that determines what a build or a reader may see. They are routinely conflated, and the conflation is what makes build behaviour hard to change: the working-notes-stripping that publication needs is a projection over *parts*, while selecting which rows appear in a view is a filter over *records*, and both are typically hardcoded in one renderer rather than declared. The corrected reading: **working notes living in the same file as the body is not the defect; the hardcoding of their removal is.**

**(C) Events are forced twice over.** The event part earns its separateness from two independent directions — it has its own write clock (append-only, many writers, no read-modify-write of the body), and it is where crossings between layers get recorded so that backlog is countable rather than felt. A structure forced by two independent constraints has, in this estate's experience, proven more durable than one justified by a single argument.

**(D) What this does not settle.** Which layout any given deployment should choose; whether per-part write rules can be expressed at all on substrates without declared record boundaries; and the grain question for events (dispositions are discrete, absorption of a note into a body is gradual — the honest event grain is the disposition, not the fragment).

## Strength & grounds

**Heuristic**, on breadth of live expression rather than on a derivation. The three layouts in (A) were read first-hand on 2026-08-05 in `~/src/arch/asf/` (segment + `terminology/`), `~/src/arch/vivarium/`, `~/src/arch/firmatum/relata/` (`lib/relata/paths.rb` names `entries`, `verifications`, `calibrations` as sibling trees), and the udon-needs frontmatter schema. (B) and (C) are formulations first stated in the udon design correspondence and restated here; the estate is one authorship, so the agreement across instances is coherence, not corroboration. What would raise it: a differently-authored corpus arriving at the same four-part cluster, or a deployment that declares layout and visibility separately and reports whether the build actually became easier to change.

## Working Notes

- (B)'s two build operations — filter over records vs projection over parts — arrived independently in the same drafting round and now has its own fuller treatment at [[selection-and-projection]]; this segment states only the half that bears on why a cluster's parts need declaring, and should defer to that one on the build question. The cheapest concrete test of either: generalize one existing hardcoded stripper behind a declaration and see whether the per-audience variants fall out.
- The write-rule half of the cluster is [[write-semantics-declaration]]; the concurrency half is [[write-safety]] and [[partition-isolation]]; the crossing-record half is [[observable-crossings]].
- Unintegrated influx behind this segment (do not cite as warrant; this segment replaces those pointers): `plan/INFLUX/udon-analysis/underlying-logical-model.md` and `plan/INFLUX/udon-analysis/living-documents-seed.md`. The live originals are under `~/src/arch/firmatum/udon/v2/theory/to-integrate/`. Note a near-duplicate twin tree at `~/src/MOVED/udon/`: the theory files cited here are byte-identical between the two, but the trees are separate repositories and do differ (one directory collapsed to a file), with firmatum strictly ahead as of 2026-08-05 — firmatum is the live one.
