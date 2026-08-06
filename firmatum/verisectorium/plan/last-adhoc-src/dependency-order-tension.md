---
slug: dependency-order-tension
type: form
depends:
  - identities-over-locations
---

# Outline order and dependency order diverge legitimately — so the divergence must be checkable

*A view's reading order serves exposition; the atoms' `depends:` DAG records logical priority. They will disagree, the disagreement is often correct, and the difference between a chosen inversion and a mistake is whether it is declared.*

## The claim

Two orderings live over the same atoms. The **DAG** states what stands on what — it changes when the argument changes. The **outline** states what a reader meets first — it changes when pedagogy improves. Forcing the outline to be a topological sort of the DAG makes exposition hostage to logic (every foundational lemma dragged to the front); ignoring the DAG entirely lets genuine errors — a claim used before anything establishes it — hide inside pedagogical license.

The resolution the estate has converged on is a three-part contract:

1. **The check runs**: a linter validates the view's order against the DAG, so every inversion is *seen*.
2. **Intended inversions are records, keyed by the relation**: an accepted-violations store whose rows are (segment, depends-on) pairs — surviving row moves, unlike position-keyed exceptions — each carrying an acceptance date and a reason grounded in a citable record, not convenience. Accepted rows are still printed, so the exception stays visible; the check stays green only while every inversion is either accepted or fixed.
3. **Staleness self-reports**: when a slug renames or a violation resolves, the exception row goes stale detectably and gets pruned — dead exceptions do not accumulate.

Appendix placement ( [[appendix-placement]] ) is the highest-volume instance — supporting material upstream in the DAG, downstream in the exposition — and meta-segments introduced-before-used are the second. Both are the *same* declared inversion, at different scales.

## Strength & grounds

**Heuristic; the contract is shipped and exercised in one corpus.** ASF's `bin/lint-outline` + per-component `OUTLINE-accepted.md` implement all three parts (relation-keyed rows, reason-with-citable-record required, stale-row reporting, accepted-but-printed) and run across four components with 16 accepted inversions and one open violation as of 2026-08-05; the design properties quoted here were read in the live store's own header. Single implementation, single estate — what would raise it: a second corpus adopting the contract and reporting the stale-row half actually firing, which is the part that distinguishes it from an ordinary lint whitelist.

## Working Notes

- This corpus declares order-divergence legal in ONTOLOGY but has no lint of its own yet — `check-plan` validates dep *resolution*, not order-vs-DAG; adding it needs the accepted-store first or every appendix row reports as a violation.
- Open (from [[appendix-placement]]): whether accepted-inversion reasons should distinguish *pedagogical* from *structural* license; the estate's store does not.
