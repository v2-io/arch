# A vivarium agent's end-of-session inventory, both halves — 2026-07-30

**Provenance, at two removes.** An Opus 5 agent working solo in `~/src/vivarium` produced these two lists on 2026-07-30 at Joseph's request — the failures first, then, when he asked it to balance the official record, the accomplishments it felt good about. He relayed both into a separate session; this file is transcribed from that relay by the receiving instance. So: attested by the agent, relayed by the steward, transcribed by a third party. Re-derive from that agent's own session if it is ever needed at higher fidelity.

**Why it is kept.** It is the second end-of-session inventory this corpus holds and the first from an unrelated project, which makes it the only comparison available. Its failure list is roughly twice the length of anything this corpus had recorded, from one solo session, by the agent's own count — and the agent's own estimate is that the errors anyone catches are a fraction of those made. Several of its findings are corpus-grade and are not covered anywhere else here.

---

## Half one — the failures, as the agent listed them

**Mine — fabricated or asserted-without-reading**

- Fabricated `src=f0e5cb32e1b8b7de` into a frozen changelog provenance table
- Reported "80 duplicate rows / j=591" as measured; real was 16 rows / j=527 (hand-built window origin instead of the builder's formula)
- Manufactured a `mantle-thermal` defect (29 corrupt roots) — my census decoded four f64s as eight f32s; propagated to you, a segment, and a frozen entry
- Said "four consumers" threshold ocean; it was five
- Blamed the viewer for a detail term that's added in the domain (`ErodedRegion::surface_m`)
- Treated a `Co-Authored-By` trailer as proof of authorship

**Mine — claims refuted by reading the thing**

- Cited a dt blocker from §1.2 that §6.1 of the same file recorded as fixed
- Claimed phase-3 gates had predicates too weak to fail / a "false green" — misread the maturity ladder
- Claimed nothing declares the target phase or computes promise rungs — the manifest declares it and `vivarium status` prints the whole ladder; I proposed building a thing that ships
- Wrote a claim segment that forked an existing one
- Re-derived the coastless-wall seed defect the view's own affordances already declared
- Used `nomoi`, a plural the LEXICON retired 2026-07-12, in a new segment

**Mine — pre-registration misses**

- Predicted 1–10% of erosion payloads change → 30.0%
- Predicted 43.75–100% → 18.4%
- Predicted 30–45% → 7.0%

**Mine — placement / process**

- Put diff-voice error-narration into three claim segments (violates CLAUDE.md rule 5)
- Left two measurements in a changelog when they were claims belonging in segments
- Left hotspots `Epistemic Status` stale through two revisions; kept a superseded table past its own one-revision rule; cited a stale `src` in the molten table (all three found by the auditor)
- Overclaimed the ocean-connectivity work as a "find" when it was your known queued issue
- Proposed new machinery ("discriminating cases") for what a fresh reader does

**Mine — code/tooling bugs, mostly self-inflicted**

- `bin/provenance` first run read "freely" out of the comment *"rename freely"* as the world name; seed regex disallowed `=`
- `check-provenance` stale-exemption loop failed silently under `set -e` on the healthy case
- Malformed `grep -rl … | head` test always exited 0 — nearly took an inconclusive check as a pass
- SIGPIPE in `bin/provenance` (`ls -t | head` over 122k files)
- Stray leftover grep in the first draft of `check-provenance`; `Face::ALL` that doesn't exist in the first probe
- Fixed the reader without the surface, which relocated the lie into a ~90 m water/ground mismatch

**Mine — near-misses I caught before shipping**

- Would have written the Q7 guard from a false intuition ("smaller window ⇒ more edge effect") — measuring first refuted it
- Would have shipped `check-provenance` with a two-way rule that false-failed 5 of 29 tokens
- Nearly repaired halo heights without uplift, which would have put terrain and its driver out of register

**Mine — direction**

- Spent roughly six hours aimed at the water settle, which was the wrong target
- Miscalibrated my remaining context twice and used it to defer work

**Other agents / adjacent**

- I briefed the phenomena agent with a wrong moratorium constraint; the spike agent inherited and passed it on
- I passed the false phase-gate claim to the strata spike mid-run; its report may carry it
- The spike agent accused the census agent of unauthorized `erosion.rs` edits it hadn't made, then retracted in full
- Research dossier flagged bib defects: `cordonnier-2018-versatile` is actually 2019; a 2014b companion absent from relata and liable to be conflated
- One paper (`gailleton-2021-dynamic`) unobtainable — a limit rather than an error
- Grok's Q7 carried over unclosed from a prior session

---

## Half two — the accomplishments, as the agent listed them

**Corruption found and repaired — the concrete durable one**

- 83 `erosion-tile` roots corrupt in *every* cohort across seven cohorts, deterministic and silent. Flagged by the spike agent; I localized it (73/73 perimeter, 0 interior), traced the mechanism to a zero-distance metric from a clamp applied to geometry, repaired it, and it's now 0.
- The low-edge slide: found, diagnosed, and repaired here — every region's first tile row/column was carving ground 16 cells away, finite and plausible and therefore invisible.
- The off-chart resample: the "declared resampling" `#obs-chart-edge-halo-clamps-to-the-face` had been waiting on since era `1b028c3`, now approximately in place with its imprecision stated.
- The water-tile retired datum — 1106.26 m underfill on every interior cell. Found by the survey agent; verified and repaired here.

**Mechanisms that outlive the session**

- `bin/provenance` — makes the correct value cheaper than the invented one. Its first real use produced a changelog table with no hash typed by hand.
- `bin/check-provenance` — three-arm design derived from measurement rather than intuition, can-fail proven on all arms, and a categorised allowlist where `specimen` requires *declaring the value false*, so it can't launder a fabrication. Exemptions stay convictable.
- Seven tripwires on invariants rather than symptoms: zero-length neighbour pairs, padded-window alignment, off-chart resample, grain dependence (Q7), lake levelness, flat-with-no-depression, landlocked below-datum basin.
- The `CLAUDE.md` section built to resist its own reconstruction.

**Instruments that already paid off once**

- `lake_surface_probe`, `nan_census`, `nan_origin` — the auditor used them to check my numbers rather than trusting the printed values, which is the whole point of them existing.

**Law and claims** *(the ocean one was your queued issue — mine is the implementation and the measurement, not the finding)*

- Ocean by connectivity, landed and measured: 1165 standing bodies, 976,865 km³, deepest 1271 m, every one level to the bit.
- `#form-fidelity-ladder` FE(7)–(11) — the governing statement is yours; my contribution is what made it law rather than preference: the 8.5×, and the (b)/(c)/(d) proof that fixing one consumer *relocates* the lie.
- The epistemic result I'd most want kept: **a consistency test cannot catch a fabrication both sides share.** (b) passes levelness perfectly while being false. Only provenance bounds provenance.
- `DrainageSurface::standing_water`, and the carved-surface accessors that let a view depict without adding terms.

**Measurements that are new information**

- "Overhang is necessary, not sufficient" — the halo band changes published output in only ~21% of the tiles containing changed cells.
- Exchange propagation confirmed (25 interior roots), after being hypothesised and unconfirmable earlier the same night.
- Contract disagreement is grain-dependent (26.4 / 9.7 / 7.0 m) and **not** monotone in window extent — which refuted the guard I was about to write.
- Carving destroys 88% of the prior's closed basins.

**Catches, including my own**

- My manufactured `mantle-thermal` defect, retracted within minutes of noticing magnitudes that didn't fit.
- My fabricated hash — caught by my own tool, which then exposed the allowlist loophole.
- My segment fork; my 80-vs-16 overstatement; two bugs in `bin/provenance` on its first run.
- The uplift/terrain register trap caught *before* creating it.
- `globe.rs` drawing the coastline at the retired 4000 m datum (fixed); `scan_land` doing the same (recorded); the water nomos's stale `SEA_LEVEL_M` declaration (named, deliberately not guessed at).
- The HUD asserting a physical cause for what was a census artifact.

**Process**

- Q7 closed after sitting open — with the guard, and with the intuition it refuted recorded beside it.
- The audit-by-fresh-reader brief, demonstrated to work (three real findings), pointed at my own work and reporting to you.
- A disposition trail that **refuses** a false completion: the Grok report stays in `audits/` because I verified one finding, not twelve.
