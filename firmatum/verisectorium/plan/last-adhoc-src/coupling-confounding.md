---
slug: coupling-confounding
type: obs
---

# Coupling confounding (placement confounds co-change)

**Summary.** If you estimate “these two pieces of the corpus must belong together” from how often they change in the same commits, the estimate is confounded by *where you already put them on disk*. Shared files and folders force co-change; that co-change then looks like evidence for the placement you already chose.

## Terms (this segment)

**Coupling (co-change sense).** Following ASF’s `#def-system-coupling` (live theory, not an influx note): for parts \(m_i, m_j\),

\[
\text{coupling}(m_i, m_j) = P(\text{change}(m_j) \mid \text{change}(m_i))
\]

estimated from observed changesets (e.g. git history). High coupling means: when one part is touched, the other is often touched too.

**Layout (placement sense).** Here **layout** means the *filesystem placement* of corpus parts — which content lives in the same file, which lives in sibling files, which lives under a shared directory policy — **not** CSS/layout, outline order, or pedagogical sequencing. Examples:

| Placement | What co-moves by construction |
|---|---|
| Two sections in **one segment file** (e.g. Formal Expression and Working Notes) | Almost every edit of the file touches “both” in git’s eyes if the commit is file-grained |
| Many records in **one multi-record log file** | Appends and rewrites often rewrite the whole file |
| One record per **slug file** in a directory | Co-change of two slugs is *not* forced by sharing a file; it has to come from real joint work (or bundling habit) |

**Indirection.** Putting parts in **separate** files (or a child directory of events) so that a change to one need not rewrite the other. Low co-change between those paths is then at least *possible*; high co-change would mean something real (or a bad commit habit), not “they share a file.”

## Observation

**The circularity.** Suppose you want evidence that “body and Working Notes should stay in one file.” You measure coupling between “body edits” and “Working Notes edits.” If they already share a file, ordinary commits that touch the segment will count as co-change **because of placement**, even when the reason for the edit was only one of the two roles. Using that high coupling score to *justify* co-location is circular: the layout produced the statistic that was supposed to license the layout.

The same circle applies to any “should these subsection roles share a container?” decision that is scored by file-level co-change while they already share the container.

**What is *not* circular.** If two **separately placed** parts still co-change often (two slug files always edited together for the same features), that co-change is not explained by “they are one file.” It may still be confounded by other things (shared requirements, developers always bundling two features in one commit — ASF names these classes under `#hyp-causal-discovery-from-git` and the Discussion of `#def-system-coupling`), but it is not confounded by **shared-file placement**.

**Asymmetry.** ASF’s coupling definition is directed: \(P(j \mid i)\) need not equal \(P(i \mid j)\). Symmetric “they always change together” is exactly what common-cause bundling and shared-file packaging tend to produce. Strong **asymmetry** (changing A often forces B, but not the reverse) is harder for pure packaging to fake, and is what the theory treats as better evidence of directed dependence — still not a free lunch, but a different standard than “high mutual co-change.”

**Applied to claim corpora (qualitative, not a measured matrix).**

| Pair of roles | Typical placement | What co-change would mean if naively read |
|---|---|---|
| Segment body ↔ Working Notes | Same file (usual ASF/vivarium shape) | High file-level co-change is **expected from packaging**; does not by itself prove they *must* share a file |
| Segment body ↔ append-only events in a **sibling directory** (one file per event) | Separate paths | High co-change would mean real joint work (or bundling), not shared-file artifact |
| Two independent claim slugs | Separate files | Same |

So: **co-location of section roles inside one atom file is consistent with high coupling *and* with the confounder.** Separation into different paths is the placement that *could* falsify “they only move together because of the container.”

## Method

- Substance of the coupling definition and confounder classes: ASF segments `#def-system-coupling`, Discussion there and `#hyp-causal-discovery-from-git` (regime where co-change ≈ causal vs descriptive only).
- Application to segment parts / document placement: this segment’s framing (estate practice of co-locating body and Working Notes, separating event logs).
- **Not done:** a fresh co-change matrix over asf or vivarium trees. Atomic-commit preconditions for good estimates are often unmet; that is a separate measurement problem.

## Strength and scope

- Supports: **naive “high co-change ⇒ keep them in one file” is invalid when they already share a file**; placement is a confounder for co-change-based layout arguments.
- Does not support: that body and Working Notes *should* be split; that measured coupling is useless; a numerical coupling study of this estate.
- Honest strength: **methodological observation grounded in named live theory**, applied qualitatively to segment structure — not a new empirical co-change study.

## Working Notes

- **Unintegrated influx.** Earlier drafts pointed at `plan/INFLUX/` notes (`tst-grounding.md`, `underlying-logical-model.md`, 2026-07-23 generalization) as if they were citations. Those are **intake, not canon** — they will be archived after adjudication. Do not treat them as warrant. Anything still only in INFLUX on this topic should be re-derived into segments (this one is the start) or discarded.
- If a real co-change measurement is ever run on segment *sections* (not whole files) under atomic commits, attach results here or supersede with a measurement obs.
- Related future segments: [[atom-as-cluster]] (what parts an atom has), [[write-safety]] / [[partition-isolation]] (when multi-record files force whole-file rewrites).
- **2026-08-05 dispatch (fresh-eyes pass).** Two items from the archived generalization note, registered as R16 in `plan/TODO.md` (cite the live original at `~/src/arch/notes/`): the Joseph-endorsed deconfliction that **layout** (same file / sibling / child table) and **rules-and-visibility** (canon / non-canon / historical / derived) are independent questions — concluding that Working Notes being in-file is not the defect, the hardcoding of their removal is; and, measured this pass, a bearing negative — `stage` is denormalized across OUTLINE rows and frontmatter with **0 mismatches in 168 comparisons** on 01-aat-core (R1), so the co-location worry has an empirical answer here.
