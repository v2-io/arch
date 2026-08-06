---
slug: corpus-instruments
type: form
depends:
  - instrument-failure-census
---

# An instrument reports on the corpus it was told about, and the reader has to know which corpus that is

*Every measurement of a living corpus is taken through a declared scope; the failures that matter are not wrong numbers but true numbers whose scope the reader supplies from imagination.*

## The claim

A corpus that outlives its authors needs instruments — checkers, censuses, health reports, linters — because no agent can hold it. Instruments are cheap to build and their output is trusted far past its warrant, so the design question is not accuracy but **what a clean report licenses a reader to believe**.

Three failures are distinct, and conflating them produces the wrong repair (specimens and counts: [[instrument-failure-census]]).

- **Declared blindness.** An instrument exempts a class by design — superseded material, generated files, an archive — and then reports zero problems. The number is true of the material in scope and says nothing about the material out of it. This is not a bug and should not be repaired as one; the repair is *disclosure at the point of reporting*, so that the exemption travels with the number instead of living in the source.
- **Faithful reporting of worthless signal.** The instrument counts exactly what it says, and the count is dominated by cases everyone already knows are fine. This is the more corrosive failure, because it does not train readers to distrust *this* number — it trains them to discount instruments generally, and that discount then applies to the instrument that was about to say something true.
- **A rule applied outside its scope.** A rule is taught to the checker without the boundary that makes it a rule, and correct usage is reported as violation. The cost is paid by whoever spends a day proving the field was working.

**The general principle behind all three:** an instrument's report is a statement about a scope, and the scope is usually implicit. Making it explicit at the point of reporting — *N problems among M records, excluding class C* — costs one line and converts every one of these failures from a silent miscalibration into a visible one.

**Two design consequences worth taking seriously.**

*Counts in durable prose are pointers, not values.* A number written into a README or a summary is true on its authoring date and drifts silently afterward. The alternative shipped in this estate is to state the **command that computes the figure** and mark any quoted snapshot as orientation-only. This is the same discipline [[collision-staleness-detection]] argues from the other side — a stale number that cannot collide with anything is the worst case, because nothing will ever contradict it.

*Absence of an instrument is not evidence of health.* Classes nothing measures are invisible in exactly the way a clean report is visible. Two live examples: no checker compares a view's hand-authored state column against the record's own declaration, and no checker exists for the discipline governing what may be admitted as a note. Both gaps read as silence, and silence reads as fine.

**And the counter-argument, which is decisive.** All of this is an argument for building instruments carefully, not for building fewer. The strongest single case in this estate for instruments is a tool built to stop a specific fabrication — which fabricated on its own first run, and whose running is what caught that. An instrument that can catch its own author is doing work no discipline replaces; the failures above are reasons to declare scope, not reasons to trust memory instead.

## Strength & grounds

**Heuristic, mixed register.** Two of the three failure modes were verified first-hand 2026-08-06 (the declared-blindness exemption checks and file counts — with one count corrected the same day by adversarial re-run, recorded in the specimen; the fabrication incident and its tool, recorded with its date and the fabricated value); the worthless-signal and mis-scoped-rule specimens are inherited at a stated remove from a 2026-07-07 process review through a 2026-08-05 field report, with the review's counts known to be stale. All counts, dating caveats, and method live in [[instrument-failure-census]].

The taxonomy — that these three are *distinct failures with distinct repairs* — is this segment's, drawn from four specimens in one estate across two corpora. No frequency claim is available or attempted. The counts-are-pointers norm is one instance's practice — `~/src/arch/firmatum/relata`, whose README declines to state its own corpus size as canon, giving instead the commands that compute each figure and marking any quoted snapshot as orientation-only. It is violated freely elsewhere in the same estate, which is evidence about its cost rather than its correctness.

## Working Notes

- Discharges TODO entries A14, R3, N39, and N16's ch. 14 half.
- Registered and not carried: the compression-drops-citations claim (R38) — that a derived surface can restate a body of work correctly while citing none of it, with the loss invisible in the output. It is a genuine corpus-health class with no instrument, and it belongs here or in the ch. 12 gap; left open rather than asserted because the single measurement behind it was not re-run.
- The obvious next instrument is a census of exemptions: how many blind spots exist across the estate's checkers, and whether each is disclosed at its reporting site. One grep, never run.
- Open, and larger than this row: what a *standard* health report should contain. R2 supplies the beginning of an answer from measurement — what actually rots in this estate is path-anchored provenance, absence handling, and verification backlog, while generated views and denormalized state did not — but that is a finding about one estate's rot profile, not a report specification, and the ch. 14 gap row is its home.
