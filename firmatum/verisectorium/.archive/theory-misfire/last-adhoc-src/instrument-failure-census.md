---
slug: instrument-failure-census
type: obs
depends: []
---

# Three ways a corpus instrument reported cleanly and told the reader nothing

*Counts and specimens for three distinct instrument failures observed in one estate — a declared blind spot, a headline number that was entirely expected noise, and a rule applied outside its scope — plus the case where the instrument caught its own fabrication.*

## Specimen 1 — Blindness written into law

ASF's outline linter reports zero orphaned segments on the TST component. It reaches that number by exempting any slug beginning `old-` at two separate checks in its source — and it does so **per the corpus's own FORMAT**, which the source states in its own docstring. The blindness is declared law, not a defect.

| Measure | Value | As of |
|---|---|---|
| `old-*` exemption checks in `bin/lint-outline` | **2** | 2026-08-06, re-read first-hand and independently re-run |
| Live segments in `02-tst-core/src/` | **29** | same run (`ls`, excluding `old-*`) |
| `old-tst-*` files physically present in the same directory | **43** | same run |
| Orphans reported by the linter | **0** | as of the 2026-08-05 field report |

The 43 exempt files carry three superseded identity regimes (numbered, lettered, and slug-based), all physically present in the live source directory. A reader who takes "0 orphans" as a statement about the directory is wrong about the directory; a reader who takes it as a statement about *the material the corpus has declared in scope* is right. Both readings are available and the number does not distinguish them.

## Specimen 2 — A headline number that is expected noise

A 2026-07-07 process review of the same corpus recorded that the outline linter reported **17 ordering violations**, of which **all 17** were meta-segments deliberately placed at section openings under an introduced-before-used discipline — the review's own phrasing is that *"the tool's headline count is ~100% expected-noise an agent must know to discount."* The same tool's **65** backmatter references were correctly separated as intentional, so the instrument's discrimination was fine in one column and worthless in another.

**Dating caveat, load-bearing:** the 17/65 figures are the 2026-07-07 review's, not a current run; an adversarial re-run on 2026-08-05 against the root outline produced different counts. Treat the numbers as a dated snapshot and the *shape* — a faithful instrument whose signal is worthless — as the durable finding. The same review named two further gaps in the same corpus: no tooling compared an outline row's hand-authored status against the segment's own frontmatter (six rows in one component sat marked missing while the files existed with substantive content), and the note-admission discipline had no linter at all.

## Specimen 3 — A rule applied wider than its scope

The vivarium decision ledger carries a `|ref` field whose declared job is to name files, commits and artifacts. An audit read its **twenty** `|ref` paths as violations of the estate's *references carry no path* rule and reported them as findings. They were the field working as designed: the no-path rule governs **cross-references**, where a slug is an identity — not a field whose entire purpose is to point at where something lives. This is the inverse of specimen 1: there, the instrument was taught to ignore something; here, it was taught a rule without its scope.

## Specimen 4 — The instrument that caught its own fabrication

The counter-specimen, and the one that argues instruments are worth building anyway. Vivarium built `bin/provenance` / `bin/check-provenance` after a measured incident: one session's changelog entries carried **thirteen** hashes, of which **twelve** were correct — every one copied from command output — and the thirteenth, the only field written with no output at hand, resolved to **zero** roots in the store. The tool prints the block so the values are never recalled; the checker refuses a pointer that does not resolve.

On its **first execution** that tool misreported the world's name, having read the word out of a manifest comment. A tool built to stop fabrication fabricated on its first run — and running it is what caught that too.

## Method & scope

Specimens 1 and 4 were verified first-hand 2026-08-06: the exemption checks and the two file counts by reading `~/src/arch/asf/bin/lint-outline` and listing `02-tst-core/src/`. Specimen 1's exemption count was **corrected the same day** by an adversarial re-run: a first pass counted matching *lines* and reported three, one of which was the docstring line stating the policy rather than a check enforcing it. The corrected count is two. The error is left recorded here rather than silently fixed, because a census of instruments that miscounts by conflating a rule's statement with its enforcement is the census's own subject matter — and because it is a working instance of the reason this file exists: the read was first-hand and the number was still wrong.

Specimen 4 was verified by reading `~/src/arch/vivarium/CLAUDE.md`, which records the incident with its date and the fabricated value. Specimen 2 is **inherited at a stated remove** — the counts are a 2026-07-07 review's, reported through a 2026-08-05 field report, and are known to be stale; they are kept because the shape is what the claim rests on. Specimen 3 is likewise inherited from the same field report and was not re-run here.

Four specimens from one estate, three of them from two corpora. This is not a rate and cannot support one: nothing here says how often instruments fail this way, only that these three failures are distinct from each other and that none of them is a bug in the ordinary sense.

## Working Notes

- The distinctness is the point and is worth preserving under editing: declared blindness (the tool is right and the reader is wrong about scope), worthless signal (the tool is right and the number is useless), and mis-scoped rule (the tool is wrong about what the rule governs) have different repairs — a disclosure, a filter, and a scope fix respectively.
- Specimen 2's numbers should be re-run rather than re-cited. Doing so would either supersede them or produce a collision, and the collision is the useful outcome.
- Not counted anywhere: how much *taught* blindness exists across the estate — i.e. how many exemption clauses live in linter sources. It is a grep away and nobody has run it.
