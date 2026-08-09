<!--
  Verisectorium notes — live-instance field reports, 2026-08-05 (evening pass).
  Two survey agents walked the live trees (which the original gather deliberately
  did not copy); the lead session verified the load-bearing claims first-hand
  where marked ✔. Scout-reported-only claims are marked (scout). Non-canon.
  This is HEALTH data to sit beside the gather's SHAPE data — the gather
  captured what each instance is; this captures what state it is actually in.
-->

# Live-state field reports — health of the instances

## Headline findings

1. ✔ **comproprium's falsifiability check is red across the board, silently.**
   `bin/check-corpus`: 57 segments, **3/109 quoted spans locate in a primary,
   106 fail**. Cause: the 2026-08-01 "Reorganize various ingest queues" commit
   (a40825f) moved `.to-integrate/` → `.integrated/` + `INGEST/harvest-a|b/`
   without rewriting `:from` paths or teaching the checker. The mechanism built
   to keep verbatim spans verifiable (FORMAT §D6's own argument) was disabled
   by a directory rename. Meanwhile ✔ all 18 outline `#slug` refs survived the
   same move. **Natural experiment: slug-identity is move-proof;
   path-as-provenance rots.**
2. ✔ **The promotion ladder has never run anywhere.** vivarium: 115/115
   segments `stage: draft` (verified by grep) despite being the most actively
   worked corpus (all files touched since Jul 1). Nothing estate-wide has
   reached `format-clean` or `candidate`; Gate 4 has never fired. (scout) The
   most-promoted AAT segments therefore carry the heaviest Working-Notes load —
   `def-chronica` 63% WN — because nothing has ever been swept.
3. ✔ **02-tst-core is a half-migration wearing a clean bill of health**: 29
   live segments vs 43 `old-tst-*` files (verified counts) carrying two prior
   identity regimes (FP-numbers → T-labels → slugs, all physically present).
   (scout) `bin/lint-outline` reports 0 orphans because it was taught to
   ignore `old-*` — the instrument's blindness is configured, not discovered.
4. ✔ **udon-needs runs a generation-ahead schema** (verified against
   `addressing-is-the-long-pole.md`): status split into three axes
   (`strength` × `support-kind` × `convergent`), append-only `verified:` event
   log in frontmatter, per-line-commented `sources:`, `opens:`/
   `handoff-routing:`. (scout) **vivarium's FORMAT open-questions ask in
   writing for nearly these exact fields** (`sources:`, a mechanism field) —
   live cross-pollination gap; nobody told vivarium. And the two instances
   *disagree on principle*: vivarium's law says qualifier-in-frontmatter is
   the failure mode ("the claim travels and the qualifier does not");
   udon-needs made the qualifier machine-readable in frontmatter.

## Instance health table

| Instance | State (scout unless ✔) |
|---|---|
| asf 01-aat-core | Mature: 170 segments, near-clean lint, real promotion history (128 draft / 22 deps-verified / 18 claims-verified). 9 `--GAP--` rows, 5 of them empty-Discussion placeholders. One real orphan pair + one dangling row the linter misses (`example-L1`, `worked-example-cam`). |
| asf 02-tst-core | ✔ Half-migrated (29 live / 43 `old-tst-*`, three identity regimes). Stale since 07-21. |
| asf 03-llm / 04-eli | Outline-first skeletons (30 rows/23 files; 35/22). 20 dangling slugs + 5 real dependency breaks, expressed as rows-with-no-file rather than `--GAP--` (zero GAP rows) — the immature absence-dialect silently breaks `depends:`. Stale 3 weeks. |
| vivarium core | ✔ Structurally cleanest (115/115 files↔rows, 100% frontmatter/WN/ES) and ✔ 100% draft, ✔ zero `--GAP--` rows despite writing the estate's strongest GAP law. Fork-under-protest FORMAT (own law, borrows asf's linter cross-repo). |
| udon-needs | ✔ Most evolved schema (above); 30 bridges over a 7-report anthology + 322-file provenance layer. Location unstable (`MOVED/`, reorg mid-flight) — do not hardcode paths. |
| comproprium | ✔ Provenance red (above). Young (born ~07-30), lopsided: 69 quotations vs 8 probes / 4 principles / 1 practice; ~51 harvest segments unintegrated (≈half the corpus outside the store). `by-trigger` outline healthy but partial (18/35 exempla). One type-vocab drift (`:type pattern`, undeclared). |
| neurips adjudicated | Real but one pass old (born 08-02, delegated agent, spot-checked): 01 full (19 files incl. residue + REVISION-OUTLINE), 02/03 thin. Exact OUTLINE↔claims agreement. One index/reality mismatch (02 discussion "empty" but has 1 file). Umbrella submodule pointers behind. |
| terminology | Most mature cousin: 176 entries, 149 decision dirs, permissive-extractor/strict-linter split. Frozen 3 weeks; LEXICON lives at asf root, consistent with sources. |
| neurips/refs vs logos/refs | Same schema, opposite discipline: 188 entries/7% verified vs 97/72% verified. |
| relata | **Not a peer cousin — the generalized successor**: 2,277 entries + 219 verification dirs at `~/.local/share/relata`, actively used, with quarantine / deny-list / calibrations machinery nothing else has. neurips/refs and logos/refs are unmigrated satellites (migration TODOs exist on both sides). |

## Cross-cutting corrections to the gather's framing

- **What rots is not what the design conversation worried about.** Generated
  views: every one checked was consistent with sources. Stage denorm rot:
  measured ≈0 across ASF (the duplicated-field policy is holding). What
  actually rots: **provenance paths** (106 failures), **absence handling**
  (unknown-stage buckets, unexercised GAP laws, rows-without-files), and
  **verification backlog** (7–10% event coverage outside logos).
- **The instances disagree on principle, not just drift**: epistemic axis
  (1 scalar vs 3 axes vs neurips' `[PROVED]/[TESTED n×]/[JUDGMENT]` rung tags —
  ≥3 incompatible strength ladders live); write semantics (integration-is-
  replacement vs comproprium D5 append-only accounts — write-semantics must be
  a per-directory *declaration*); absence dialects (typed `--GAP--` vs bare
  missing rows); Epistemic-Status-as-section vs as-frontmatter.
- **Four outline-table dialects** (01/02 six-col; 03/04 five-col; vivarium
  five-col+importance-tier, no §; udon-needs wikilink refs, ragged widths) —
  any shared lint/build must eat these as reality.
- **Family members no roster listed**: `residue.md` (do-not-resurrect
  named-absence store), `REVISION-OUTLINE.md` (outline over segments that
  don't exist yet), `adjudicated-common/` (cross-instance shared layer),
  comproprium's `the-chain.md` (narrative-order outline with non-authoritative
  connective prose — counter-case to outline-as-mere-ordering) and
  `GATHERING.md` (harvester brief genre), relata's `quarantine/` +
  `deny-list.yml` + `calibrations/`, vivarium's orientation gate
  (`../instances/vivarium-orientation-gate.md`), vivarium
  `.super-archive/.../core/OUTLINE.md` (dead prior generation). False
  positive: `~/src/_gems/devex/OUTLINE.md` is not an instance.

## Candidate near-term repairs surfaced (not adjudicated)

- comproprium provenance repair — and the *design* decision it forces:
  repoint paths vs teach the checker layouts vs make provenance
  layout-independent (slug/anchor-shaped, like the orientation quiz's
  slug+section+word-anchor addressing).
- Cross-pollinate udon-needs' schema answers into vivarium's open questions
  (or adjudicate the principled disagreement rather than letting it stand
  unnoticed).
- 03/04's 20 dangling rows → typed `--GAP--` or files (one absence dialect).
- neurips umbrella submodule pointer bump; 02 discussion index mismatch.
