---
slug: stage-denorm-zero-drift
type: obs
depends: []
---

# The checked-but-untrusted duplicate field, measured

*The full account of ASF's deliberately duplicated `stage` field and what its drift actually measures — the observation grounding [[view-edge-metadata]]'s claim that a copied edge attribute is a manageable risk rather than an inevitable rot.*

## The setup

ASF's four theory components each hold a flat directory of segment files plus an `OUTLINE.md` whose rows name them. The process field `stage` is recorded in **both** places: in each segment's YAML frontmatter, and in the Stage column of the outline row that names that segment. That is a denormalization — the same fact stored twice, with nothing forcing the two copies to agree.

The corpus knows this and has a written policy for it. `FORMAT.md` states that `bin/lint-outline` verifies consistency between the two — mismatches, missing or off-vocabulary values, `missing`-versus-file-exists — **"as warnings only, never gate failures"**, giving the reason: the stage layer *"is known to go stale quickly under rearranging, pedagogical reorientation, and continued refinement, and is currently ignored in practice."* The same sentence warns readers not to read a low stage value as low epistemic strength — that is what the separate `status` field is for.

So the field is permitted, checked, and explicitly not trusted.

## The measurement

| Measure | Value | As of |
|---|---|---|
| Segment files in `01-aat-core/src/` | 170 | 2026-08-05 |
| OUTLINE rows carrying a slug | 169 | same run |
| Rows compared against an existing file | 168 | same run |
| Rows whose file does not exist | 1 | same run |
| **Stage mismatches between row and frontmatter** | **0** | same run |

## Method & scope

Every line of `~/src/arch/asf/01-aat-core/OUTLINE.md` beginning with a table pipe was scanned for a slug-bearing link of the form `[#slug]`; the row's last cell was taken as its Stage value and compared, exactly, against the `stage:` field parsed from the YAML frontmatter of `src/<slug>.md`. Rows with no matching file were counted separately rather than as mismatches. One component, one corpus, one day.

This does **not** show that denormalized fields are safe in general, and it does not measure how quickly the value would drift under different working conditions — the same format document asserts, as a claim about the field, that it goes stale quickly. What it shows is that in the one live instance where the duplication is checked continuously and trusted by nobody, the measured divergence is zero. The negative result is the point: the anomaly this layout is theoretically exposed to has not materialized here.

## Working Notes

- **A second, independent count the same day returned 170 slug-bearing rows and 169 compared.** Adjudicated by a third pass: the extra row is a `--GAP--` row carrying no slug, and there are 169 distinct `[#slug]` links with no duplicates. The counts above stand, but the disagreement is recorded rather than silently resolved, because a row-counting method that disagrees by one is exactly the kind of thing that makes a later re-run look like drift when it is not. Running the corpus's own `bin/lint-outline` would be the deciding third method and was not run.
- The corpus holds 170 segment files against 169 rows and 1 row without a file, so exactly one file is off the outline entirely — an orphan, and a different defect from the one this observation is about.
- Keep the counts dated. A re-run supersedes them, and if a later run shows drift, the interesting quantity is how long the warnings had been printing unattended.
- The one row without a file belongs to the estate's absence-handling problem rather than to this one; it is the shape that `--GAP--` rows exist to express.
- Not measured here and worth measuring: whether anyone ever acts on a `lint-outline` stage warning. A check nobody reads and a check that always passes look identical from the outside.
