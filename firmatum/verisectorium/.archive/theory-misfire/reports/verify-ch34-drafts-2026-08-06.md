# Verification pass — 16 segments, 2026-08-06

Scope: adversarial check of the 16 segments listed in the brief, focused on
re-running the counted claims against live trees, spot-checking quotations
verbatim, and checking `depends:` validity. Not a full read-for-restatement
pass against the eight named neighbours — flagged as a gap below, not done.

## Headline

The batch is unusually solid on its numbers. Every multi-source count I
re-ran in `sidecar-ubiquity-census`, `tracking-layer-census`, and
`terminology-store-anatomy` matched exactly. `depends:` lists all resolve to
real files and none is obviously circular. The defects found are small:
one likely off-by-a-few count, and two verbatim-quote fidelity slips (a
capitalization change and a word-form change inside quotation marks). I
did not find anything that should be deleted.

## Verified exact (re-run 2026-08-06, this pass)

- `sidecar-ubiquity-census`: asf `01-aat-core/src/` 170 files / 164 with
  `## Working Notes` — exact. vivarium `core/src/` 115/115 — exact.
  `comproprium/vera/` 12 files / 12 with `|working-notes` — exact.
- `tracking-layer-census`: PRACTICA.md 132, TODO.md 537, PROPOSALS.md 442,
  JOSEPH-TODO.md 27, CHANGELOG.md 1417 (segment says "1,417" — matches),
  LOG.md 221, audits/STATUS.md 44 — all exact.
- `terminology-store-anatomy`: asf `terminology/entries/` 176 files,
  `terminology/decisions/` 149 dirs / 160 event files, `LEXICON.md` 384
  lines — all exact. vivarium `LEXICON.udon` 106 `|term[…]` elements — exact.
  `.archive/SUPERSEDED.md` 94 lines — exact.
- `routing-sop-anatomy`: `routing.sop.md` 407 lines / 5 named Refinements,
  `spikes.sop.md` 355 lines / 10 named Refinements (15 total) — exact, once
  matched against the actual `*Refinement N (...)` markers rather than a
  naive grep (my first grep pattern over/under-counted; the segment's
  numbers are the correct ones).
- `tracking-layer-census` / `tracking-altitudes` audience markings: 6 files
  auditor-hidden (TODO, PROPOSALS, JOSEPH-TODO, TODO-big-picture,
  TST-IDEAS, HISTORICAL-CONTEXT), 3 auditor-safe (PRACTICA + the two
  generated README variants) — matches the segment's 6/3 claim, confirmed
  by direct read of `CLAUDE.md`'s file-organization list rather than a
  grep on a literal string (the marks are prose annotations, not a
  consistent grep target — worth knowing if anyone tries to re-derive this
  mechanically later).
- `depends:` fields on all 16 segments resolve to files that exist in
  `plan/last-adhoc-src/` (checked against the directory listing) or are
  declared `[]`. No dangling deps, no obvious cycles among the 16.

## Defects found

1. **`basename-manifestation-survey`: "ASF's root alone carries 25 such
   files" is off.** `find asf -maxdepth 1 -type f -name '*.md'` returns 20;
   `ls asf/*.md | wc -l` returns 24 (the discrepancy between those two is
   itself worth knowing — likely a symlink or hidden-file artifact, not
   re-diagnosed here). Neither matches 25. Recommend re-running the count
   with the exact method used originally and either fixing the number or
   noting the method that produces 25 (e.g. if the original count included
   a file since removed, or counted `.udon`/`.un` alongside `.md`).

2. **Two verbatim-quote fidelity slips in `routing-sop-anatomy`**, both
   against `provenance-registers`' own discipline (a segment that marks a
   quote as verbatim should be exact):
   - Segment: *"Check the new filter externally before building on it —
     the conviction that the correction is clean is the same conviction the
     doc says fails."* Source (`routing.sop.md:378`): "check the new filter
     externally before building on it — the conviction that the correction
     is clean is the same conviction the doc says fails." — capitalization
     of the opening word changed (minor, but it is inside quotation marks
     rendered as direct speech).
   - Segment: *"forking a hard-won protocol so two copies could drift"*
     Source (`spikes.sop.md:62`): "fork a hard-won protocol so two copies
     could drift" — "forking" vs "fork". This one changes the grammatical
     role of the word inside a quotation; small, but it is a paraphrase
     wearing a quote's punctuation.

   Neither changes the meaning. Both are the kind of thing
   `readable-claims-appendix-evidence`'s spirit (and the ONTOLOGY's own
   "locate each verbatim" self-assessment) asks to be exact about. Fix is
   a two-second edit each.

## Not done / explicitly out of scope for this pass

- I did not re-check `gate-profile-divergence`'s 70/97 and 13/188 pair,
  `working-notes-deluge`'s 27.3%/57.9% figures, or the udon-needs 20-file /
  six-spelling breakdown in `sidecar-ubiquity-census` — those ground
  *other* segments (cited, not asserted, by the ones I was asked to check)
  and re-running them was outside what I had budget for this pass. Given
  the batch's track record above, I'd bet on them, but that's a bet, not a
  check — flag them for whoever verifies those segments directly.
- I did not do a full restate-and-compare against the eight named
  neighbours (`integration-metabolism`, `observable-crossings`,
  `influx-queues`, `layer-speeds`, `working-note-lifecycle`,
  `decision-records`, `warrant-over-authority`, `identities-over-locations`)
  to hunt for restatement. On a skim while reading the 16, nothing jumped
  out as duplicating a neighbour rather than complementing it — the
  cross-references between segments (e.g. `strengthen-before-routing` (E)
  explicitly correcting `integration-metabolism`'s replacement discipline
  against itself) read as genuine extension, not restatement. But I want
  to be honest that "read the neighbour list while reading the 16" is a
  much weaker check than the dedicated restatement pass the brief asked
  for, and I did not do the latter.
- `honest-incompleteness-discharge`'s self-named tension (its stopping rule
  being disposition-shaped per `gates-need-destinations` (B)'s own
  prediction) — I read it, and I think naming it honestly is the right
  call *for this draft*; I don't think it needs more than naming right now,
  because the segment already states the falsifiable next step (attach the
  self-check to the act of proposing a further gate) rather than leaving it
  as an unexamined stance. That's a judgment call, not a finding — flagged
  as requested, not resolved.
- No near-collisions noticed with the parallel batch's files as of this
  read; I didn't re-scan the directory at the end of this pass to check for
  new arrivals since I started.

## Register check on NORMS.md citations

Spot-checked: `basename-manifestation-survey` and `sidecar-conventions`
both cite `~/src/arch/notes/NORMS.md` with the "provisional, unratified
brainstorm, cited as a proposal to measure against" framing stated
explicitly and up front, and both immediately follow with a measurement
that is *not* flattering to adoption (one compliant instance out of seven
trees). That's the register you were worried about, and it reads right —
the citations don't smuggle NORMS.md in as law anywhere I found.
