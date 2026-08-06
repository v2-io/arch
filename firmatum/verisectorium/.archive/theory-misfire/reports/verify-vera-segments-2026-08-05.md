# Adversarial check — verbal-label-calibration.md + name-collision-across-stores.md

*2026-08-05. Independent re-verification, not a review of prose quality. I re-ran every check myself rather than trusting the author's report of having run it — where that changed the answer, I say so plainly.*

## Bottom line first

Both segments are substantially solid — the quotes and numbers I could locate in primaries check out almost everywhere, and the two first-hand "verified" claims about file existence and cross-references are true as stated. But I found two things that matter:

1. **The Leg-3 grep claim in `verbal-label-calibration.md` is false as literally written**, in a way that isn't just pedantic — it misses a hit that bears directly on the claim it's supporting.
2. **The segment's own self-diagnosis in section (D) is correct, but section (B) doesn't honor it** — the prose overclaims past its own caveat, exactly as the author suspected and asked me to check.

Neither is fatal. Both are fixable in a few sentences. Detail below.

---

## 1. Quotes and numbers — verbal-label-calibration.md

| Claim | Verdict | Detail |
|---|---|---|
| `"possible"` median ≈38%, IQR >40 points, bimodal | **Solid** | `vox-vera/verbal-probability-calibration.md:74`: "Possible": Median 38.5%, IQR 42.7% → NO consensus (bimodal!)". Matches. |
| `"always"` IQR under half a point | **Solid** | Same file, line 72 / CSV: `Always,99.6,99.7,99.8,0.3` — IQR 0.3. Matches. |
| Mosteller & Youtz 1990, n=238 | **Solid** | CSV and calibration guide both confirm n=238, "science writers." |
| Budescu IPCC gap: "very likely" intended >90%, public reads ~67% | **Solid** | `ipcc_likelihood_interpretation.csv`: Very likely, >90%, Public_Mean 67.0. Matches exactly, including the "~28 point gap" figure used in Working Notes. |
| Vogel 2022, 21 studies, 1967–2018, temporally stable | **Solid** | `verbal-probability-calibration.md:90-93`: "Meta-analysis of 21 studies (1967-2018)... Confirms temporal stability." Matches. |
| **"54 verbal probability expressions"** | **Wrong — off by one.** | `mosteller_youtz_1990_full.csv` has 53 data rows (54 lines including header). The embeddings project's own `CLAUDE.md` gets this right: "53 verbal probability expressions." Small, but it's the kind of number that gets repeated as fact once it's written down — worth a one-character fix (54→53). |
| Mosteller→Vogel ρ=0.991, Mosteller→Wintle ρ=0.967 | **Solid, exact** | `embeddings/CLAUDE.md` Key Findings, verbatim numbers. |
| "transfers zero-shot across 8 typologically diverse languages" | **Solid** | Matches `CLAUDE.md` exactly. |
| **"recovered from 5 architecturally diverse models"** | **Imprecise, and the primary itself disambiguates this.** | The embeddings paper's own abstract says "six embedding architectures" / "six models spanning five architectural families" (`paper.md:9,17`). `CLAUDE.md`'s Key Findings also says "6 models." "5 models" conflates model-count with family-count — the correct claim is either "6 models" or "5 architectural families," not "5 models." (This isn't the segment inventing a number — it looks like it was carried over from `asf/03-llm-core/OUTLINE.md:192`, which makes the same "5 architecturally diverse model families" phrasing — but the segment drops "families" and keeps "5," which turns an accurate claim into an inaccurate one.) |
| "submitted paper, TACL 11139" | **Solid, but not from the source cited.** | `embeddings/CLAUDE.md` is stale on this point — it still says "Preparing paper for submission." The actual confirmation is `embeddings/TODO.md:5`: "Submitted to TACL as `11139-epistemic-hedging.pdf`." The claim is true; just flagging that if anyone re-checks it against CLAUDE.md alone (as I initially did) they'll get a false negative. |

## 2. Quotes — name-collision-across-stores.md

| Claim | Verdict | Detail |
|---|---|---|
| Comproprium README quote: "collision with symbolic PROPRIUM / firmatum ontology remains a naming caution, not a layout blocker" | **Solid, verbatim.** | `comproprium/README.md` §Open, item 2. Character-for-character match. |
| VERA (PROPRIUM) description: "a component name with a brief mechanism sketch, not a designed subsystem" | **Solid, fair paraphrase.** | Confirmed against `PROPRIUM-ONTOLOGY-v2.md` §"VERA: Qualified Truths" (lines 637–651) — one paragraph, two acquisition paths, no uncertainty-representation design, no retraction protocol. The characterization is accurate; it's an interpretive judgment rather than a quote, and it's a fair one. |
| VERA architecture (ennaos): "Nov 2025... substantial, and abandoned in place" | **Solid.** | `vera-architecture-final-specification.md` is dated "November 2025" in its own header, is 1249 lines (substantial), and I found no implementation in ennaos's `lib/`/`apps/` referencing it and no git activity on the file after Nov 5 2025 — "abandoned in place" holds up as far as I could check. |
| Comproprium `vera/` runs the ported `robust-qualitative`/`discussion-grade` ladder | **Solid, directly confirmed.** (This one is actually load-bearing for the *other* segment, verbal-label-calibration.md, which makes the same claim — both check out.) | `grep` over `comproprium/vera/*.udon` shows `:status robust-qualitative` (2 files) and `:status discussion-grade` (9 files) in frontmatter. |

## 3. The three specifically-flagged ✔ claims — independently re-run

**Claim: `~/src/firmatum/` does not exist, and the two paths in `proprium-mapping.md` are dead.**

**Confirmed true**, both halves. `ls ~/src/firmatum` → no such file or directory. `proprium-mapping.md`'s closing line points at `~/src/firmatum/PROPRIUM-ONTOLOGY-v2.md` and `~/src/firmatum/PROPRIUM-ARCHITECTURE-v2.md`; neither can exist since the parent directory doesn't. The actual files live at `~/src/arch/proprium/INGEST/msc-from-harness/canonical/PROPRIUM-{ONTOLOGY,ARCHITECTURE}-v2.md`. Good find, correctly reported.

**Claim: no cross-reference exists between the PROPRIUM v2 documents and the comproprium precepts, in either direction.**

**Confirmed true.** `grep -i comproprium` over both v2 documents: zero hits. `grep` over all 12 files in `comproprium/vera/` for any PROPRIUM-v2 path or filename reference: zero hits. Genuinely independent, unconnected.

**Claim: a grep for `mosteller|budescu|verbal probabilit` across `~/src/arch/` and `~/src/_core/` finds no hit in any FORMAT file, terminology entry, or segment — every hit is either inside the calibration material itself, inside this project's own influx copies, or in the embeddings project.**

**The grep itself checks out (no hits in FORMAT/terminology-entry/segment), but the summary sentence describing what the *other* hits are is false, and the false part matters.**

Running the same grep myself, after excluding the calibration material (`vox-vera/`), this project's own influx/segments, and the embeddings tree, there are real hits left over — and they are not transcripts or noise:

- **`~/src/arch/asf/03-llm-core/OUTLINE.md:192`** — a substantial paragraph connecting the Mosteller-calibrated embeddings result directly to "logogenic agents [leveraging] language-geometric encodings of epistemic states for $U_M$ / $U_o$ / hedging-projection estimates" — i.e., ASF's own canonical outline already discusses using this calibration research for an agent's internal epistemic-state estimates. That's not "epistemic-status label calibration" in the exact sense Leg 3 means (labels a *reader* interprets vs. an *agent's* internal uncertainty variable), but it is close enough that a claim of "nobody in the estate has pointed this instrument at anything of this kind" needed to name and distinguish it, not silently omit it.
- **`~/src/_core/ennaos/docs/research/vera/deprecated-hierarchical-knowledge-representation-in-graphs.md:2210`** — lists "Verbal probability calibration (Mosteller & Youtz 1990)" as one of the "Statistically Grounded" foundations for VERA's confidence architecture. This is under `~/src/_core/`, squarely in the grep's stated scope, and it is neither the calibration material itself, an influx copy, nor the embeddings project — it's a fourth location the claim's own wording says shouldn't exist. (On inspection this usage is about grounding *numeric* Beta-distribution confidence encoding, not calibrating verbal *status labels* like `robust-qualitative` — so it doesn't refute Leg 3's substantive point once you read it. But the segment's sentence "every hit is either X, Y, or Z" is still factually wrong, and a reader who trusts that sentence and doesn't re-run the grep would never find this.)
- Several ASF spike files (`spike-language-as-causal-substrate/03-minimum-scaffold.md`, `08-aies-paper-proposal.md`) and `msc/` files also reference Mosteller in the context of extending the embeddings methodology to new domains — again, not FORMAT/terminology-entry/segment, so technically outside the letter of the claim, but inside "the estate," which is the word doing the rhetorical work in "the estate has this instrument and has never pointed it at itself."

**Net assessment:** the *specific* claim (no FORMAT/terminology/segment hit) is true. The *summary characterization* of what else the grep returns is false — it undercounts by at least five files across two projects that are not any of the three named buckets. I'd fix this by either narrowing the claimed scope of the grep in the prose to match exactly what was searched (FORMAT/terminology/segment only, full stop, no claim about "every other hit"), or by acknowledging the ennaos and asf-outline hits and explaining why they don't count as calibrating *reader-facing status labels* (which is a defensible distinction, but it has to be made, not assumed).

## 4. Leg 3's self-flag vs. the rest of the prose — does the caveat cover the claim?

You asked me to judge this independently rather than take the author's own tiering at face value. My reading: **section (D) is honest and section (B) is not consistent with it.**

(D) says, correctly and with real hedging: *"The measured spreads are for probability phrases... `robust-qualitative` is not one of them... The transfer is an argument from the structure of the situation... not a finding. It could fail."* That's exactly the right epistemic posture for Leg 3, and "Strength & grounds" correctly tiers Leg 3 as "discussion-grade, and honestly so."

But (B) — which comes *before* the caveat and is not itself flagged — asserts, unhedged: *"the labels in a ladder are not interchangeable in quality: some carry their meaning intact and some do not, and which is which is an empirical fact about the word, discoverable in advance, and currently unknown for every ladder in this estate."* Read on its own, this sentence is a flat claim about epistemic-status ladders (not probability phrases) — it says outright that per-label spread is a real, measurable, currently-unknown property of *every ladder in the estate*, which is precisely the un-established transfer (D) later disclaims. A reader who stops at (B), or who skims the doc for its load-bearing sentence, walks away believing something the author doesn't actually know.

The title and dek make this worse, not better: the H1 is *"A strength label is a word, and words are read with measurable spread"* and the italic dek under it is *"Epistemic-status ladders are made of English words that readers interpret with a spread the corpus has never measured — and in the one domain where stipulated meaning and read meaning have been compared, they differ by a lot."* Both are the most rhetorically prominent sentences in the document (title + dek, read before anything else), and both assert the transfer as settled fact, with zero hedge, well before section (D) walks it back. "The corpus has never measured" (dek) presupposes the spread *exists to be measured* — that's the very thing not established.

So: the self-flag is real and honestly worded, but it doesn't reach far enough back into the document. This is not a case of "flag exists, ignore it" — it's a case of the flag being placed *after* the load-bearing claims it should be qualifying, so it reads as a footnote to an already-landed point rather than the frame the whole argument should be read through. I'd fix this by either softening the title/dek/B to explicitly name the transfer as inference (e.g., "...and if the same pattern holds for epistemic-status words — untested — currently unknown for every ladder in this estate"), or by moving a compressed version of (D)'s caveat up before (B).

## 5. Strand separation (the four "vera"s)

Checked every occurrence of "vera" (case-insensitive, word-boundary and substring) in both segments. **No conflation found.** `verbal-label-calibration.md` only ever invokes comproprium's `vera/` (correctly) and never touches PROPRIUM-VERA or ennaos-VERA. `name-collision-across-stores.md` is *about* keeping the four apart and does so correctly and consistently, including in the illustrative "arriving from a third project" aside — that's a deliberately ambiguous hypothetical, not an actual conflation. This is the one segment whose entire job is not to make this mistake, and it doesn't.

## 6. The convergence claim (Leg 2, embeddings vs. Mosteller-style elicitation)

The claim that the two are "genuinely independent support-kinds with genuinely different failure modes" holds up, with one nuance worth naming that the segment doesn't. The embeddings axis is not merely *compared against* Mosteller after the fact — Mosteller's medians are the **training signal** (`CLAUDE.md`: "Hardcode Mosteller training expressions with median probabilities inline"; ridge regression trains the axis on them). Vogel and Wintle are the genuinely held-out sets, and it's the strong correlation to *those* (ρ=0.991, ρ=0.967) that constitutes the independent confirmation — not the Mosteller correlation itself, which is closer to a fit-quality check on the training data. The segment's framing ("cross-validated across three psychometric datasets") is technically accurate but reads as if all three plays are symmetric evidence, when one of the three is the training target. This doesn't break the convergence argument — a linear structure trained on one dataset's medians generalizing to two unseen datasets at ρ>0.96 is still a real and independent confirmation of the *psychometric* result, from a *distributional* method with a different failure surface (embedding artifacts, tokenization, corpus-frequency confounds vs. survey response bias, framing effects, question-order effects). I'd just make the training/held-out distinction explicit rather than let "cross-validated across three" imply three-way symmetry.

## 7. Ontology-rule compliance

**`influx-not-warrant`**: both segments comply. Neither leans on `plan/INFLUX/` as a citation; both name the influx paths in Working Notes only, marked do-not-cite, with live-source alternatives named as what the body actually stands on. Clean.

**`readable-claims-appendix-evidence` / `placement-is-outline-only`**: neither segment states its outline position — clean on the letter of the rule. One structural observation, offered as a judgment call rather than a violation: `name-collision-across-stores.md` explicitly holds up the `identities-over-locations` (norm) + `provenance-rot-specimen` (obs) split as "the worked exemplar pair (readable claim in chapter · full specimen in appendix)" — but it doesn't apply that split to itself. Its own four-row specimen table, "verified first-hand" methodology note, and dated verification details read like appendix-grade `obs` material embedded directly inside a chapter-level `emp` segment, rather than split into a companion appendix the way its own cited exemplar does. The rule says "usually," not "always," so this isn't a clean violation — but given the segment cites the split-pattern approvingly in its own prose, the inconsistency is worth the author's attention, not just mine.

**Neither segment is currently referenced anywhere in `OUTLINE.md`.** I checked by grepping both slugs against the outline file and got zero hits for either. This is presumably just pending integration (both are freshly drafted, `last-adhoc-src/` is explicitly a landing zone), not a rule violation — but since placement is outline-only and the outline is the assembly manifest, right now these two segments have no address in the structure they're meant to belong to. Worth a follow-up row addition, not a fix to the segment bodies.

## 8. Holistic judgment — worth having as standalone segments?

**`verbal-label-calibration.md`: yes, clearly worth it**, modulo the two fixes above (the 53/54 count, the "5 models" imprecision) and the title/dek/§B overclaim relative to §D's own caveat. The three-leg structure is a genuinely useful piece of reasoning — it correctly separates "external calibration facts" (rock solid) from "convergent second instrument" (solid, with the training-vs-holdout nuance) from "transfer to this estate's ladders" (honestly speculative, once the caveat is moved where it belongs). This is real, load-bearing thinking, not padding.

**`name-collision-across-stores.md`: worth keeping, but it is thin, and it says so about itself** ("This segment is deliberately narrow. ... should not be pre-empted from one specimen."). The core mechanism argument (collision detection is blind between stores because identity, not content, is what makes contradiction visible) is a real and non-obvious structural point, and the four-way "vera" specimen is a clean, verified illustration of it. The incidental finding (dead `~/src/firmatum/` paths in `proprium-mapping.md`) is a genuinely useful catch that stands on its own. I don't think it's too thin to justify existing — one well-verified specimen plus a correct general mechanism claim is a legitimate `emp` segment — but it is exactly the kind of single-instance piece that should be watched for whether it merges or gets superseded once more collision specimens turn up, which the segment itself anticipates.

## Summary of concrete fixes

1. `verbal-label-calibration.md`: "54" → "53" verbal probability expressions.
2. `verbal-label-calibration.md`: "5 architecturally diverse models" → either "6 models" or "5 architectural families" (the paper's own phrase) — not "5 models," which the primary directly contradicts.
3. `verbal-label-calibration.md`: the Leg-3 grep sentence needs either a narrower scope claim (say exactly "FORMAT/terminology/segment," nothing broader) or an acknowledgment of the ASF-outline and ennaos hits and why they don't count as label calibration.
4. `verbal-label-calibration.md`: move (or echo) some of (D)'s caveat up into the title/dek/§B, so the most-read sentences in the document don't overclaim what the Strength & Grounds section correctly downgrades.
5. `verbal-label-calibration.md`: worth a sentence distinguishing Mosteller-as-training-signal from Vogel/Wintle-as-holdout in the Leg 2 "cross-validated across three" framing.
6. `name-collision-across-stores.md`: no correction needed, but consider whether its own specimen material belongs in a companion appendix segment, per the split it cites approvingly from `identities-over-locations`/`provenance-rot-specimen`.
7. Both: add outline rows — neither segment currently has a home in `OUTLINE.md`.
