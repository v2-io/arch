# *Volume* SOP — Standard Operating Procedures

**Canon view** over `sop/src/`. This store is a verisectorium one level deep, terminating at self-governance via its own influx (`sop/influx/`). It carries two aspects:

1. **Praxes**   (meta — how work happens here) and
2. **Doctrina** (domain — what must be known to work here)

In addition to feedback from agents here, `sop/influx/` will capture Verisectorium general upgrades, after which they are adopted / adapted / declined-with-record locally.


## *Chapter* Orientation
| State         | Type (or)<br>Expected | Designator                   | Description                                                 | Epistemic<br>Status | Max             |
|---------------|-----------------------|------------------------------|-------------------------------------------------------------|---------------------|-----------------|
| override      | directive             | [[dir-orient]]               | Orientation index & instructions-- Link target of ORIENT.md | proposed            | ruled           |
| template      | directive             | [[dir-disposition]]          | Agentic instructions on disposition/attitude generally      | proposed            | ratified        |
| proposed      | directive             | [[dir-domain-disposition]]   | Additional disposition for this specific project            |                     | ruled           |
| proposed      | directive             | [[dir-orient-src]]           | The initial *doctrina* needed: crucial def & src segments   |                     | ruled-current   |
| proposed      | directive             | [[dir-orient-docs]]          | Other core domain documents to read in prescribed way (required reading) |            | ruled-current   |
| template      | reference             | [[ref-verisectorium-tools]]  | The tools Verisectorium ships / makes available — maintained upstream as they evolve; built vs planned split honestly | proposed | current |
| proposed      | directive             | [[dir-essential-tools]]      | This instance's tools with normative usage that needs to be followed |            | ruled-current   |
| override      | reference             | [[ref-hazards]]              | Named hazards, so nobody re-learns them (seeded empty; every instance accrues its own) | proposed | current |
| proposed      | reference             | [[ref-available-tools]]      | Other tools to be used situationally or for convenience     |                     | current         |
| proposed      | reference             | [[ref-resources]]            | Contextual primaries: read situationally on a trigger — whole, at the moment of use, keyed to `ref/` |  | current         |
| **--GAP--**   | discussion |          | --Register/epistemology discipline for agents working here (which label system, how strength is held, how the ladders reconcile) — parked on the steward's open epistemology decision (theory OUTLINE.md working notes); not filled by the segments below, which cover corpus/naming/flux discipline instead-- |  |  |


## *Chapter* Segment & Corpus Discipline

*How an atom is built and named — what every agent editing `sop/src/` (or any store in this instance) needs before touching a record.*

| State         | Type (or)<br>Expected | Designator                   | Description                                                 | Epistemic<br>Status | Max             |
|---------------|------------------------|-------------------------------|--------------------------------------------------------------|----------------------|-----------------|
| template      | definition             | [[def-atom]]                  | What counts as one record here: one atom per file, identity is the filename-slug — never a position, number, or path. Read before splitting or merging anything in a store. | axiomatic | axiomatic |
| template      | definition             | [[def-atom-cluster]]          | An atom is body + working notes + events + companions, each with different write rules — mixing them (history in the body, status claims with no event behind them) is the recurring local failure this segment names. | axiomatic | axiomatic |
| template      | formulation            | [[form-slug-form-kinds]]      | Why filenames here carry a `def-`/`claim-`/`form-`/… prefix and what it may and may never encode: the prefix names the segment's speech-act and must never be renamed just because a claim's evidential position moved. | discussion-grade | decided |
| template      | formulation            | [[form-state-flags-not-gates]] | The `State`/check columns in this and every outline are independent, resettable flags, not a promotion ladder — editing a verified atom and resetting its flags is advancement, not a demotion to fix or apologize for. | discussion-grade | decided |

## *Chapter* Naming Discipline

*What makes a name good, and why it is worth arguing about before committing one.*

| State         | Type (or)<br>Expected | Designator                   | Description                                                 | Epistemic<br>Status | Max             |
|---------------|------------------------|-------------------------------|--------------------------------------------------------------|----------------------|-----------------|
| template      | postulate              | [[post-names-are-interface]]  | Why naming is not a side concern here: every access into this store — outline scan, grep, `depends:`, a delegation brief — meets the name before the content, so a bad name taxes every future reader, forever. | axiomatic | axiomatic |
| template      | claim                  | [[claim-naming-criteria]]     | The working checklist for a non-obvious naming call in this store (citability, scope honesty, the renamed-from-now test, and the four repair moves when a name fails) — apply before committing a new slug or reopening an old one. | heuristic | heuristic |

## *Chapter* Flux & Integration Discipline

*How material crosses into this store's population, and what "integrated" is allowed to mean once it has.*

| State         | Type (or)<br>Expected | Designator                   | Description                                                 | Epistemic<br>Status | Max             |
|---------------|------------------------|-------------------------------|--------------------------------------------------------------|----------------------|-----------------|
| template      | formulation            | [[form-influx-membrane]]      | Nothing lands in this store's population directly: incoming material waits on an influx surface until adjudicated, and the crossing resolves to one of *rejected* / *needs-review* / *skipped* / *landed* — conflating "the submitter erred" with "we're honestly unsure" is the small dishonesty this segment exists to block. | discussion-grade | decided |
| template      | definition             | [[def-integration-replacement]] | The delete-test that governs every claim of "integrated" here: assume the item vanishes — is everything it carried either landed in the population or truly disposable? A TODO entry *about* the remainder is not the remainder landed. Read before moving anything to `.integrated/` or `.archive/`. | axiomatic | axiomatic |
| template      | claim                  | [[claim-dispatch-compounds]]  | Why the delete-test is worth enforcing strictly rather than leniently: residue left on a live surface is scanned by every future session, and a lenient dispatch can tip a queue from a compounding-cheap clean state into a widening, false-nugget-generating mess. | heuristic | robust-qualitative |


## *Working Notes (outline-level)*

- **Column semantics (steward, 2026-08-09):** State, Type, Epistemic Status, and Max are view-local denorms of segment frontmatter; the Designator is the segment's identity surfacing (implicit by filename); Description is the only view-native content column — this view's *perspective* on the segment, not a copy of its summary ( [[claim-outline-as-view]]). Epistemic Status is filled only for documents that are actually written.
- **Process-state enum (one MECE axis):** `proposed` — placeholder row the deployed project drafts, not the template · `template` — owned and maintained upstream by verisectorium; copied at deploy; an upstream change temporarily un-ratifies the copy until re-adopted · `override` — seeded from the template as exemplar and (where needed, e.g. orientation) bootstrap, with the intent that the deployment quickly modifies and owns it; once overridden it moves to `drafted` (or `override-drafted`) and onward like any segment. Symlinked theory segments in `template/sop/src/` deploy as copies, not links.
- **Authority register for directives (steward-ratified 2026-08-09):** `proposed / ratified / ruled`, with `current` (and the `-current` suffix on Max) as a freshness qualifier — a claim of currency that decays and resets when the underlying tools/docs change. Directives and references are not truth-apt, so their "Epistemic Status" column carries an **authority** status — *who stands behind this text, how firmly* (kin to DECISIONS' decided-by axis) — not an evidence tier; the column is shared, the register per row follows the row's form-kind (truth-apt rows: evidence tiers; directive/reference rows: authority). This resolves the earlier open question (status vs process state): authority is a third axis, named honestly, and the process-state enum stays pure ownership.
- The **--GAP--** is verisectorium's to fill (override/template rows as theory segments draft), not the deployment's.
