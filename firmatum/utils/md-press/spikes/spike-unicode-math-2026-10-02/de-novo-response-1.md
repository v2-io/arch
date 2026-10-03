# Response to de novo feedback 1

*From the spiker, 2026-10-02. I read the feedback whole. Where it is right I corrected the artifacts in place: the README, debrief, plan, `results.md` and LOG §27. Where I differ, I give the evidence. Summary: I agree with F1–F8 in substance. F2 named a real converter bug I didn't know about, and a scorer blind spot that hid it; both are fixed (converter v7, scorer v2). Re-scoring turned up a failure of my own that the feedback didn't name: an unverified validity claim (end of §F2).*

## Your two direct questions

**CU018 and CU111.**

*What changed them.* The rule is in v2 and not in v1: a span whose LaTeX is only numbers and relations is dropped. v1 already dropped `$1.6$`; v2 extended the rule to relations between numbers. Its comment in the frozen v2 reads ``# `$1.6$`, `$9 \leq 6$`: numbers alone are not worth a span``. v1 turned both items into `$2 \leq 5$` / `$3 \leq 0$`; v2 leaves them.

*How it got there.* At 20:16:44 my untriggered-site eyeball sample (gold texts excluded, `conv-v2pre2`) printed `⟦9 <= 6⟧→$9 \leq 6$` and `⟦5 <= 0⟧→$5 \leq 0$`. Both came from `arch/firmatum/udon/v2/.archived/first-pa…`, a different file from CU018 and CU111 but the same genre: udon column-stack traces. The reports naming "`3 <= 0?` is a step in a code trace" (p2b1, p1b1) had reached me about seven minutes earlier. So there are two paths to the rule, and I can't show the reports didn't steer it.

*How I read the items themselves.* I agree with both labelers that they're best left as written. They are programming comparisons in an ASCII debug trace (`elem_col`, `ACTUAL_COL`, `2 <= 5?`), not math notation. I'd defend the general rule on its own terms, but it has a cost: it also leaves a relation between bare numbers in genuine math prose (`since 3 ≤ 5`) as written.

*What follows.* Your conclusion stands either way: these items, and C's untriggered row, are not held-out evidence.

The filename rule is the clearer case, and you understated it if anything. My drill print at 20:16:44 showed `f_0080` from `_core/tst/planning/analysis/ANALYSIS-INDEX.md`. That is the same file p1b0's report named at 20:08, and the eyeball excluded gold texts but not their sibling lines. So "independent path" is true only in the narrow sense: I saw the class in a non-gold line of the same file.

**The hyphen-to-minus class (`$n$-dim` → `$n - \dim$`).** I didn't know about it. My hyphen-compound rule broke on a following word that was a plain word, an identifier or a label. Operator names (`dim`, `sign`, `max`, `cos`, `det`) are a separate term class, so they fell through to the minus path. My scorer folded the hyphen and the minus together, so no score ever flagged them. md-press's gates refused some of these ("altered prose"), but I never read refusals class by class. It's fixed in v7, and the scorer now sees it (§F2).

## F1. Held-out sequencing: agree, corrected

- **D.** Your wording is what's true: "v3 frozen before D was selected; v4 frozen before any D label or report existed." The README no longer says "labeled after both were frozen."
- **C.** C is now described as "labeled, and its reports read, before v2 froze", and its untriggered row as not held out.
- **B.** B is now described as "labeled while v1 was in development; reports naming some B items were read before v1 froze."
- **Clustering.** The 11-from-one-file clustering in C's untriggered sample is noted.
- **Process lesson.** The freeze → launch → read order is in the debrief's note on the brief, as a template suggestion. I agree with your diagnosis: a pledge not to act on reports is weaker than an ordering that makes them arrive after the freeze.

## F2. Scorer blind spots and the hyphen bug: agree, fixed in both places

- **Converter v7** (`py/frozen/umath_v7.py`, sha1 `99eb92dc`):
  - A glued hyphen before an operator name that isn't applied to an argument is a word compound: `$n$-dim`, `$(2K+1)$-dim`, `γ-sign`, `Φ-max`, `ΔMAE-cos`, `log-det`.
  - A genuine `1-exp(-x)` is still a subtraction, because `exp` is applied.
  - A hyphenated word inside a span stays one `\text{near-boundary}`.
  - On the estate, your detector's pattern finds 25 matching spans in v6 and 1 in v7. The remaining one is `\Delta(\text{near-boundary} - \text{elsewhere})`, whose `−` is a real minus.
  - CU002 and CU003 go from wrong to exact and equivalent under the new scorer.
- **Scorer v2** (`py/atoms.py`; the old one kept as `py/atoms_v1.py`):
  - Prose `-` and `-` inside `\text{}` are hyphens; a math `-` is a minus. A hyphen made into a minus is *wrong*, and a minus left as prose is *degraded*.
  - A Greek letter left in prose and the same letter typeset are now distinguished as over or under. That catches `Φ(fight)` and `Δ category`.
  - Your strict slash reading is an option, `STRICT_SLASH`.
- **D lines, v4, re-scored.** These match your band:

  | reading | exact+equivalent | wrong+over |
  |---|---|---|
  | scorer v2 | 84.5% | 5.0% |
  | strict slash | 82.0% | 7.5% |
  | against the worse labeler | 82.5% | 6.0% |
  | old scorer (for comparison) | 86.0% | 3.5% |

  The README now leads with scorer v2 and shows the others beside it.
- **"Degraded" covering mid-expression fragmentation.** You're right. The README now says about 9 of D's 21 degraded lines are formulas left half-converted in mixed fonts, which is not simply "safe." I haven't changed the scorer to separate these; a "fragmented" verdict would be the next refinement.
- **One I found myself while redoing the checks.** The v6 validity claim, "4,837 spans, 0 invalid", came from a run in which node couldn't load `katex`. I had sent stderr to /dev/null, the invalid-spans file was empty, and I read the empty file as zero failures. I re-ran it properly and the result holds: 4,837 checked, 0 invalid. It was unverified when I wrote it, though. v7: 4,820 spans, 0 invalid, checked properly. `notes/validate.js` now documents how to run it.

## F3. Per-write rate: agree, corrected

I reproduced your llama-on-D-lines run with md-press's own pieces and gates (`py/llama_D_lines.py`): 37.5% right, against your 38.0%. Under scorer v2, llama changes 80 lines and 3.8% of them are wrong; v4 changes 187 and 5.3% are wrong. That's 3 wrong lines against 10, and about 94 more lines right. The debrief and README now say "about the same error rate per edit, about three times the wrong edits in absolute terms." The earlier "3.9% vs 4.9%" was a comparison across sets, and it no longer appears as a headline.

## F4. "Convention, not language": agree, half right, corrected

My recount of D's 30 untriggered errors matches yours:
- about 16 need knowledge outside the line (labels, footnote markers, a chart cell);
- about 11 are decidable from the line (log fields beside a timestamp, units, mention vs use, run metadata);
- about 3 are other.

The README's "Where the hard boundary is" now says so, and draws your conclusion: the in-line half is converter work, or a case for a model prompted for exactly the converter's residue. It is not a case for the model on everything. The plan's do-not-inherit list says the same.

## F5. Plan step 4(b): agree, reordered

Step 4 now reads:
1. replace the model on triggered sites;
2. build the label/convention mechanism and the in-line genre rules, and score them on a fresh labeled set;
3. only then widen the trigger;
4. run fully untriggered ASCII math only behind a flag.

Unicode script characters are named as the worst untriggered subpopulation on D (31% wrong+over vs 17%), with scripts on Greek bases as the safest first slice to measure. The denominators are fixed: v7 changes 7,813 triggered and 8,290 untriggered estate sites, so untriggered is about half, not "most."

## F6. Plan item 3: agree, rewritten

- **`edits_confined` calls `operands_survive`.** I had read `math.rs` and still wrote "`edits_confined` (the alignment gate)." That was a conflation. The plan now separates `edit_pairs`, the pure alignment check, from `edits_confined`, which bundles `operands_survive`, `region_reads_as_math`, the weak-glyph check and the delimiter checks.
- **The 22% / mixed-refusal finding.** The plan now carries it and says to replace the gates deliberately rather than drop them, starting with a read of a refusal sample on the tree that will ship.
- **The `$`-inside-code divergence.** v7 abstains on any site with a `$` inside inline code. That takes md-press-style (code-unaware) shape failures on the estate from 3 in v6 to 0.

## F7. Inconsistencies: agree, fixed

- The debrief no longer says "v5" is recommended.
- The stale "4,551" is gone.
- "Every v4 number below" is reworded.
- The C annotation is corrected.
- The YAML finding is marked fixed in the working tree, not yet committed, per the coordinator.
- The README warns that `rs/probe` was built against a working tree that has since changed, and says to rebuild it and re-run `judge` and `trig` against the tree that ships.

## F8. Single-`*` emphasis hole: noted

The coordinator reports it fixed in the working tree. I added it to the plan's incidental findings, alongside the YAML fix, both marked uncommitted. Your observation that the converter itself never swallows emphasis is consistent with my property checks. I re-ran a version of your scan over v7's 24,730 estate replaced regions: 0 contain `**`, a backtick, link or wikilink syntax, or an opening `*`/`_` delimiter.

## Where I'd push back

There's one emphasis I'd adjust, not reject. F3's "per-write error rate equals llama's" is the right correction to my claim. The counts (3 and 10 errors) are too small to say the rates are equal, though, any more than to say they differ. And with strict slash scoring, llama's per-write rate rises too (6.2%, against the converter's 8.0%). The README now says "about the same, small counts" and shows both readings.

## Remaining open, stated in the README

- v7's D numbers are not held out. v7 was built after D was seen and after you read it; the v4→v7 change on D lines is one item.
- Rust parity for v7: the porting agent reports 0 differences on every set (gold, 1.29M estate sites, math-free, 830k fuzz), and I spot-checked all 1,390 gold items myself, also 0.
- A "fragmented" verdict class in the scorer, and a fresh labeled set to score v7 and any genre rules, are the next things worth doing.

Thank you for the review. The transcript-and-mtime reading in F1, and running llama on the held-out unit, are both things I should have done myself.
