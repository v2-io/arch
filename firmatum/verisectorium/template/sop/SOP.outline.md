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
| **--GAP--**   | discussion |          | --Various **Verisectorium** theory segments that are relevant to all who work on these--<br>  e.g., segment & def & register discipline, breadcrumb-free canon, feedback, epistemology, etc. etc. |  |  |
| proposed      | directive             | [[dir-orient-src]]           | The initial *doctrina* needed: crucial def & src segments   |                     | ruled-current   |
| proposed      | directive             | [[dir-orient-docs]]          | Other core domain documents to read in prescribed way (required reading) |            | ruled-current   |
| template      | reference             | [[ref-verisectorium-tools]]  | The tools Verisectorium ships / makes available — maintained upstream as they evolve |  | current |
| proposed      | directive             | [[dir-essential-tools]]      | This instance's tools with normative usage that needs to be followed |            | ruled-current   |
| override      | reference             | [[ref-hazards]]              | Named hazards, so nobody re-learns them (seeded empty; every instance accrues its own) |  | current |
| proposed      | reference             | [[ref-available-tools]]      | Other tools to be used situationally or for convenience     |                     | current         |
| proposed      | reference             | [[ref-resources]]            | Contextual primaries: read situationally on a trigger — whole, at the moment of use, keyed to `ref/` |  | current         |


## *Working Notes (outline-level)*

- **Column semantics (steward, 2026-08-09):** State, Type, Epistemic Status, and Max are view-local denorms of segment frontmatter; the Designator is the segment's identity surfacing (implicit by filename); Description is the only view-native content column — this view's *perspective* on the segment, not a copy of its summary ( [[claim-outline-as-view]]). Epistemic Status is filled only for documents that are actually written.
- **Process-state enum (one MECE axis):** `proposed` — placeholder row the deployed project drafts, not the template · `template` — owned and maintained upstream by verisectorium; copied at deploy; an upstream change temporarily un-ratifies the copy until re-adopted · `override` — seeded from the template as exemplar and (where needed, e.g. orientation) bootstrap, with the intent that the deployment quickly modifies and owns it; once overridden it moves to `drafted` (or `override-drafted`) and onward like any segment. Symlinked theory segments in `template/sop/src/` deploy as copies, not links.
- **Epistemic-status register for directives (held, with an open question):** `proposed / ratified / ruled`, with `current` (and the `-current` suffix on Max) as a freshness qualifier — a claim of currency that decays and resets when the underlying tools/docs change. Open (steward, 2026-08-09): whether ratified/ruled are epistemic statuses for normatives and directives or belong to process state instead; held as epistemic status for now.
- The **--GAP--** is verisectorium's to fill (override/template rows as theory segments draft), not the deployment's.
