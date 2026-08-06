---
slug: view-edge-metadata
type: form
depends:
  - outline-as-organizing-principle
  - slug-identity
  - stage-denorm-zero-drift
---

# View-local metadata belongs to the membership edge

*When an outline row carries a gloss, a section number, an importance mark or a rendering directive, that information is not about the record and not about the view — it is about the fact of this record appearing in this view, and treating it as either of the other two is what makes it rot or vanish.*

## The claim

**(A) The third place data can live.** A view is a stored selection of records in an order. Between the view and each record there is a *membership* — and most of what an outline row actually contains attaches there: the gloss the view uses to introduce this record to this audience, its position or § number, an importance or hotness mark that is relative to this reading path, a directive about how to render it here. None of it is true of the record independent of the view (a different view legitimately glosses the same record differently), and none of it is true of the view as a whole. It is an attribute of the edge.

**(B) What goes wrong without the distinction.** Fields with nowhere correct to live get copied onto the record, and a copied field is a denormalization that can drift — the classical update anomaly, wearing frontmatter. Data modelling has a name for the thing being reached for here (an association class; an edge that carries properties of its own) and a worked history of getting it wrong first and correcting, so the shape does not have to be invented.

**(C) But the anomaly is theoretical and the drift, here, is measured at zero.** ASF duplicates `stage` between its outline rows and its segment frontmatter, with an explicit policy: a linter checks the two and reports mismatches as warnings only, never as failures, because the field is known to go stale and is not to be read as epistemic strength. Re-measured first-hand on 2026-08-05 across one component: **zero mismatches** over 168 compared rows (full counts and method: [[stage-denorm-zero-drift]]). So the checked-but-not-trusted duplicate is holding. The honest statement is therefore narrower than "denormalize and you will rot": *an edge attribute copied onto the record is a drift risk that a cheap standing check can keep at zero, and the estate's live instance of it has stayed at zero* — which is a better answer than either normalizing it away or believing it.

**(D) Authored views are canonical about their own fields.** Two kinds of view exist: **generated** (regenerable from the records, clobber-guarded — a lexicon, an emitted bibliography) and **authored** (the outline). An authored view is *referential* about the records it names and *canonical* about the edge data it carries, because that data exists nowhere else and cannot be regenerated. Losing an authored view loses content; losing a generated one loses nothing.

## Strength & grounds

**Heuristic** for (A), (B) and (D); (C) carries a measurement, whose method and full counts are in [[stage-denorm-zero-drift]] and whose governing policy sentence was read first-hand in `~/src/arch/asf/FORMAT.md` on 2026-08-05. (A) and (D) are a formulation from the udon design correspondence, restated here; the reading that outline rows are edges rather than record fields has not been implemented anywhere, so it is a design position with a live diagnostic behind it, not a tested result. What would raise it: an instance that actually stores its per-row metadata on the edge and reports what became easier — in particular whether multiple views over one record set stop competing for the same frontmatter slots.

## Working Notes

- The zero-drift measurement is a counterweight worth keeping attached whenever normalization is argued from theory alone; it is also dated, and a re-run supersedes it.
- Companion pieces not carried here: the ordering constraint a view may declare against the records' own dependency DAG, and the accepted-violation store keyed by the (segment, depends-on) relation so exceptions survive row moves — those belong with [[dependency-order-tension]].
- Unintegrated influx behind this segment (do not cite as warrant): `plan/INFLUX/udon-analysis/underlying-logical-model.md` §5(d) and `plan/INFLUX/udon-analysis/doc-store-report-s12-2-outline-segments.md`. Live originals: `~/src/arch/firmatum/udon/v2/theory/to-integrate/primary/underlying-logical-model.md` and `.../refine-more/doc-store-and-schemas-report.md` §12.2.
