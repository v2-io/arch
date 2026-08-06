---
slug: pipeline-seams
type: form
depends:
  - multiple-views
---

# Publication is a pipeline that meets the corpus at seams

*A paper leaving a corpus, and everything a paper brings back, cross a small number of identifiable boundaries — and what crosses each one is an adjudicated package, not a file copy.*

## The claim

A publishing pipeline is usually pictured as a straight line out of the corpus: segments in, artifact out. In every instance examined for this segment it is instead a loop with several crossings, and each crossing is a place where something must be *decided* rather than merely transformed. Naming the crossings is what makes the loop maintainable, because it is at the crossings that material silently changes epistemic status.

**Outbound, the seam is a rendering decision that is not a content decision.** The same atoms feed a blind-review artifact and a named one; the difference is a property of the *build target*, not of any segment. Putting the decision in the segment (an author block in the source, a hand-anonymized citation) means the corpus now carries the artifact's politics, and a second artifact cannot be made without editing content. Putting it at the seam keeps the atoms audience-neutral. The seam then owes its own honesty check: it is a place where a mistake is unrecoverable — a name that should have been removed cannot be un-submitted — which is why the strongest enforcement in the estate sits exactly here rather than on the segments.

**Inbound is the seam that gets forgotten.** Reviews, rebuttals, and the corrections they force arrive as prose addressed to an artifact, not to the corpus. They cannot be applied to segments as-is, because a reviewer's point is entangled with the paper's framing, is sometimes wrong, and is sometimes right about something other than what it says. The move that works is to give the inbound material its own verisectorium: adjudicate the reviews into claim records with their own verification rungs, keep a **residue store** of formulations that were refuted so nobody resurrects them, and keep the discussion material separate from the claims. What crosses back into the theory corpus is then a package of adjudicated claims, not a pile of review comments.

**The backport is its own seam, and it needs a queue that is not a landing.** Material adjudicated in the paper corpus is often *more* careful than the canon it came from — a proof tightened under review pressure, a quantifier corrected. That correction has to travel upstream, and it crosses a boundary between two corpora with different rules and different clocks. A queue at that seam, which records what is owed to canon and stays honest that nothing has landed until it has, is the thing that keeps the upstream debt from evaporating when the submission deadline passes.

**Publication is therefore not an exit.** The artifact is one projection among several ([[multiple-views]]); the corpus keeps living, and each round of external contact adds a crossing rather than closing the loop.

## Strength & grounds

**A formulation drawn from live practice in one estate, 2026-08-05.** The three seams above are read from working trees, not proposed from theory:

- **Outbound rendering.** `~/src/behavioral-floor/bin/build` forks on `--camera-ready`: the default run rewrites every cited work whose bibliography entry carries `applicable_anonymity: true` to render as `Anonymous`, and the real author block lives in the build script rather than in `src/00-meta.md` — deliberately, so the source's empty-author fallback keeps blind review the safe default. The anonymization gate on the bibliography side is `logos/refs`'s lint before submission.
- **Inbound adjudication.** `~/src/neurips/01-tragedy-confident-agent/adjudicated/` holds twelve claim records with rung tags, a `discussion/` directory, a changelog-style `OUTLINE.md`, and a `residue.md` whose header states the discipline plainly: refuted formulations stay there as the do-not-use list, *"not softened ghosts in the claims tree."* All three papers have such a tree.
- **The backport queue.** `~/src/neurips/adjudicated-common/asf-backport-queue.md` carries per-item corrections owed to ASF canon, each with why / action / status, under a header stating **"None landed in ASF canon yet"** and *"This file is the queue, not the landing."*

What this is not: a measured claim that pipelines with named seams produce better artifacts, or a survey of pipelines outside this estate. The three specimens share authorship and one publication program, and two of the three seams are young — the adjudication trees date from mid-2026 and the backport queue had, at the time of writing, moved nothing upstream. That last fact cuts both ways: it is honest evidence that the seam is real and hard, and it is not evidence that the queue works.

The design record for the inbound seam is `~/src/neurips-reviews-responses/processing-flow.md`, which names the intended shape (per paper: adjudicated claim segments + residue + discussion segments + a changelog-style index, then a package for backport) before the trees existed — so the live trees are partial confirmation of a stated plan rather than a pattern discovered after the fact.

## Working Notes

- Adjacent and not restated here: what assembly forces a corpus to decide ([[build-forced-commitments]]) and the selection/projection split inside a build ([[selection-and-projection]]). This segment is about the *boundaries*, not the machinery on either side.
- Open: whether the inbound adjudication tree should be a separate verisectorium or a region of the paper's own — currently it is a sibling directory with its own index, and nothing states which is right.
- Open: what a backport package should carry so the receiving corpus can adjudicate it without re-reading the reviews. The queue file above is per-item prose; that is probably the pre-schema form of something.
- The outbound twin of the backport queue exists and is worth folding in when this drafts further: `~/src/behavioral-floor/CLAUDE.md` carries a load-bearing-claims table mapping each claim to its substrate source and its status *in this artifact*, including one row whose substrate-source cell reads **"Not in substrate as a standalone derivation."** and then says the paper supplies it. That is an artifact declaring, at write time, what it owes upstream.
- Worth a later check: whether the backport queue ever discharges. If it does not, the seam has a destination in name only ([[gates-need-destinations]]).
