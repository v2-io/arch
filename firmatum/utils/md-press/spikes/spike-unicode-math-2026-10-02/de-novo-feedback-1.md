# De novo feedback 1: the unicode-math spike

*[2026-10-03, spiker: a few short quotations from non-public sources were replaced with short descriptions, at Joseph's request; see README §Local-only data. Nothing else was changed.]*

*Independent critical pass, 2026-10-02, by an agent that had no part in the spike. Scope was open. I read `README.md`, `debrief.md`, `proposed-integration-plan.md`, `results.md`, `notes/LOG.md`, all four labeler briefs, `py/atoms.py`, `py/evaluate_c.py`, the v1→v2 converter diff, the md-press gate code in `src/math.rs`, and both SOPs (`spikes.sop.md`, `audit-routing-instructions.md`). I also read the session transcripts of the spiker and of the 20 labelers. I re-ran the scoring and ran four experiments of my own, described under each finding. I did not read `umath.py` line by line, audit the Rust source, re-run `properties.py` or the KaTeX/MathJax validator, or re-check the agent-dialect, lexicon, or forest numbers.*

## Summary

The central claim holds, and held-out evidence the spike didn't collect makes it stronger. I ran llama3.2:3b through md-press's own piece splitter and real gates on the 200 held-out D lines. It scores **38.0%** exact-or-equivalent; the frozen v4 converter scores **86.0%**. Head to head, the converter is right where llama is wrong on 100 lines, and the reverse happens on 4. The spike's own comparison with the model used only v1 on set B, so this is the first comparison on the held-out unit md-press would actually use.

The claims around that headline need correcting:

1. **"Fresh held-out set" is accurate for D and overstated for B and C.** For B and C, the labelers' final reports were in the spiker's context before the freeze, and those reports named held-out items. For C it measurably matters: 14 of the 15 untriggered items that v2 fixed relative to v1 belong to classes the C reports had named, 11 of them one file's `f_0xxx.xhtml` filenames. "Labeled after both were frozen" is also literally false for D, though D is clean in substance. (F1)
2. **The equivalence ladder can't see one real converter bug.** It turns a word hyphen next to a named operator into a minus sign (`$n$-dim` → `$n - \dim$`, `$\gamma$-sign` → `$\gamma - \operatorname{sign}$`), and the ladder scores that "equivalent". This happens at 25 estate sites. A by-eye re-score of all 49 D-line "equivalent" verdicts suggests a band of roughly 80–85% correct and 4–8% wrong+over, rather than 86.0% / 3.5%. (F2)
3. **The per-write error rate equals llama's; it is not lower.** On D lines the rates are 3.7% and 3.8%. Because the converter writes about 2.3× as many lines, it lands about twice as many wrong edits in absolute terms. The debrief frames only the rate. (F3)
4. **"The residue is convention, not language" is overstated.** About a third of the D untriggered errors can be decided from the line itself: a timestamp next to `t=45`, a unit after a noun, a sentence saying "n_past lacks LaTeX". The spike's own proposed rules for these cases are in-line rules. (F4)
5. **Two plan items conflict with the evidence or the code.**
   - Plan step 4(b), adding Unicode script digits to the trigger, targets the untriggered subpopulation with the *highest* held-out error rate: 31%, against 17% for the rest. (F5)
   - Plan item 3 keeps `edits_confined` and drops `operands_survive`, but `edits_confined` calls `operands_survive`. (F6)
6. **Smaller items.** The documents disagree with each other (F7). Two incidental md-press findings: one is already fixed upstream; the other is new, a single-`*` emphasis hole in the gates (F8).

Things I checked and found sound are listed in their own section near the end, along with how to reproduce my runs.

---

## F1. Held-out sequencing: what the transcripts and file times show

The README says: "Three times the converter was frozen and then scored once on a fresh labeled set: B after v1, C after v2, D after v3 and v4". It also says "D (400): held out for v3 and v4. Labeled after both were frozen", and the results table annotates C with "v2 (frozen before C was looked at)". The timeline comes from the spiker's transcript (`agent-a1af5992…jsonl`), the labeler task-notifications, and the file mtimes. Times are local (−0600).

| set | labelers launched | label files written | labeler reports reach the spiker | freeze |
|---|---|---|---|---|
| A+B | 19:24–19:25 | by 19:36 | 19:32–19:36 | v1 at 19:54 |
| C | 20:02–20:03 | 20:08–20:12 | 20:09–20:13 | v2 at 20:21:14 |
| D | 20:30–20:31 | 20:37:27–20:39:25 | 20:37:56–20:39:54 | v3 at 20:27:25, v4 at 20:37:02 |

- **D is clean in substance.** v3 was frozen before D was even selected (D was selected with v3). v4 was frozen 25 seconds before the first D label file existed, and 54 seconds before the first D report arrived. The transcript shows the `cp … umath_v4.py` call at 20:37:02. I found no way information from D labels could have reached v4. But "labeled after both were frozen" is false: labeling ran from 20:30 to 20:39, and v4 was built during it. "Frozen before any D label or report existed" is what's true, and it's enough.
- **C is not clean for the untriggered items.** All six C reports were in the spiker's context 8–12 minutes before the v2 freeze. They name items and classes:
  - p1b0: "Filenames take up 5 of the 150 items (`f_NNNN.xhtml` and similar, all from one one private index file)", and "CU033 is a glyph-perception survey";
  - p2b1 and p1b1: "`3 <= 0?` is a step in a code trace";
  - p1b2 and p2b2: CL058 YAML; and more.

  LOG §12 records the spiker's decision not to act on C-specific content. The v1→v2 diff, though, adds a filename rule at 20:17:31 (`mness = 'broken'  # f_NNNN.xhtml: a file name`), eight minutes after the report naming that class. The transcript shows an independent path to it as well: an estate drill printed `f_NNNN.xhtml` at 20:16:44. So the cause is ambiguous, but the effect isn't.

  Re-scoring v1 and v2 on C, the untriggered "changed" items go from 31 wrong + 4 over to 19 wrong + 1 over. Of the 15 untriggered items that moved from error to correct:
  - 11 are `f_0xxx.xhtml` filenames from that one file;
  - 2 are the named code-trace class (CU111 `3 <= 0?`, CU018 `2 <= 5?`);
  - 1 is the named glyph survey (CU033);
  - 1 is a transcript bra-ket (CU096), the only one whose class no report named.

  C's 80.0% / 13.3% for v2 on untriggered items is therefore not a held-out number. The C line and C piece numbers are mostly unaffected: their two flips (CL016, CL049) trace to the B-driven `x_t+1` removal and the `|delta|` rule.
- **B has the same channel, at smaller scale.** LOG §4 (19:33) records labeler reports quoting B items (B252, B053, B158/B269, B245, B078) 21 minutes before v1 froze. At 19:35:58 the spiker also scored all of A+B gold and printed the malformed B078. The currency-`$` hazard rule (B078's class) came in before the freeze, with an independent path recorded through the math-free drill in §8. I don't think this moves the B numbers much, but "fresh" overstates it.
- **Sampling is clustered.** Separately, C's untriggered "changed" sample takes 11 of its 150 items from a single file. The sample isn't stratified by file, so one rule moves seven points on that row.

What this means: the D numbers carry the held-out weight, and they reproduce exactly (below). The C numbers should be labeled "labeled before freeze; labeler reports read before freeze". For C's untriggered row, I'd label it "not held out".

Process lesson for `SPIKE-PROMPT.template.md`: D got the order right. Freeze first, then launch the labelers, then read their reports. A+B and C were labeled while the converter was still changing, and the labelers' final messages, which are the most legible text they produce, arrived during development. The spiker's "I won't act on it" was sincere and, as far as I can tell, mostly kept. But a pledge doesn't stop context from steering. Freezing first does.

## F2. The equivalence ladder: two blind spots, one hiding a real bug class

The ladder's rules live in `py/atoms.py`. `font_relation` returns `'same'` for any glyph that isn't a letter, and for any letter whose two fonts both fall in {`tx`, `mu`}. That means prose text and upright math are indistinguishable, and so are a capital Greek letter in prose and its typeset form, `\text{…}`, `\operatorname{…}` and `\max`. `normalize_atoms` also folds `−` and `–` into `-`, and `piece_atoms` drops whitespace. Two consequences:

**(a) A real converter bug is invisible to the ladder.** When an existing span or a symbol is followed by a hyphen and a word that happens to be a named operator, the converter absorbs the compound and emits a subtraction:

```
$n$-dim, $m$-dim → $n - \dim$                 (×4 estate sites, asf audits/spikes)
$(2K+1)$-dim     → $(2K+1) - \dim$            (×9; plus $(2^m - 1)$-dim ×1)
$\gamma$-sign    → $\gamma - \operatorname{sign}$   (and $\mu$-sign)
$\Phi$-max       → $\Phi - \max$
arg-max $…$      → $\arg - \max\arg\max_k …$  (and arg-sup)
log-det/λ_max    → $\log - \det/\lambda_{\max}$
ΔMAE-cos         → $\Delta\text{MAE} - \cos$  (×4)
Δ(near-boundary − elsewhere) → $\Delta(\text{near} - \text{boundary} - \text{elsewhere})$
```

v6 does this at 25 estate sites (8 triggered, 17 untriggered). Each one changes the meaning: "n-dimensional" becomes "n minus dim". Two gold items, CU002 and CU003, are exactly this. Both score **equivalent** against both labelers, because the hyphen and the minus normalize to the same atom and the operator name is "upright", the same as prose. The integration plan proposes to stop running the gates that currently catch some of these (`ΔMAE-cos` and `$n$-dim` are refused as "altered prose"), on evidence from A+B that couldn't see this class. I haven't attempted a fix. The pattern looks narrow: a glued hyphen between an operand and a run of letters that only happens to match an operator name.

**(b) A by-eye re-score of all 49 D-line "equivalent" verdicts** (the README's verifier request 1). Most are honest render-equivalents: spacing, braces, `\le`/`\leq`, comma spacing inside math, `\text` vs italic subscripts, `\phi`/`\varphi`. The exceptions:

| class | items | my reading |
|---|---|---|
| meaning changed | DL072: `$b = 2/b = 3/2/b \to …$` vs gold `$b=2$ / $b=3/2$ / $b\to…$` | wrong; it reads as b = 2/b |
| slash list merged into one span | DL074, DL097, DL120, DL136, DL192 (`$\alpha/\beta$` vs gold `$\alpha$/$\beta$`) | judgment call. LOG §6 notes canon prefers separate spans 15:1, and the labelers chose them deliberately ("the slash means 'and', not division", C p2b1). In math, the merged form reads as division. |
| a name over-typeset, invisible as capital Greek | DL016, DL146 (`Φ(fight)` → `$\Phi(\text{fight})$`, a project name), DL057 (`Δ category`) | over |
| an all-upright expression missed, scored as equivalent | DL096 (`∪`), DL108 (`(5 < 20 × 0.25)`), DL184 (`Fr > 1.5`) | degraded (safe) |

Counting DL072 and the names as errors and the slash merges as equivalent gives 82.5% correct and 5.5% wrong+over. The strict reading, with slash merges counted as wrong, gives 80.0% and 8.0%. The README's "exact-only 62% is the floor" is a floor on correctness, but it says nothing about the error ceiling, and this band is more informative.

The "degraded = safe, incomplete" label also covers two different things. About 9 of the 21 degraded D lines are fragmentations in the middle of an expression rather than whole-expression misses: DL084, DL103, DL141, DL147, DL148, DL155, DL190, DL193, DL198. For example:

```
impact$(t) = \Sigma$(context_factor × w)
ā/$(1 - \beta)…$
$m \in M$\{time}
time($C_1$) > time($C_2$)
```

These render a single formula in mixed fonts. Whether that's better than leaving the line untouched is a judgment, but "safe" is too generous. They are also edits the converter writes.

These blind spots apply to the llama scoring too, so the head-to-head comparison stays fair. The converter writes more, so it's more exposed to them.

## F3. Per-write error rate: parity, with about twice the wrong writes in absolute terms

Held-out D lines, both systems through the same scorer (my run; details under "How to reproduce"):

| system | correct | wrong+over | lines changed | wrong+over among changed |
|---|---|---|---|---|
| llama3.2:3b via md-press pieces and gates | 38.0% | 1.5% (3) | 80 | 3.8% |
| converter v4, whole line | 86.0% | 3.5% (7) | 187 | 3.7% |

The spike's own comparisons (LOG §22) are 3.9% vs 4.9% (B, v1 vs llama) and 0.9% vs 2.9% (A, dev set, vs Muse). Those are cross-set, with 4–11 errors each. Nothing here separates the systems on rate. The honest phrasing is "about the same per write, around 4%, with 2.3× the useful work". For a tool that edits files in place, the absolute count matters too: about 7 vs 3 wrong lines per 200 triggered lines. That's still a clear net win (100 lines fixed against 4 lost), but the debrief's "Error per write: 3.9% vs llama's 4.9%" reads as a precision advantage the data doesn't support.

## F4. "Convention, not language understanding" is half right

From the README: "Labelers resolved those cases by reading information outside the line. A line-level LLM would lack that information too." I read all 30 wrong or over verdicts on D's untriggered items along with both labelers' notes:

- **Needs outside information (about 16):**
  - H_D1/H_D3 (6);
  - W₂ and W₁ᶜ regime labels (6);
  - S₁ (1);
  - footnote markers E³/X⁸ alone in a table cell (2);
  - a code-chart cell (1).
- **Decidable from the line itself (about 11):**
  - `t=45` beside a timestamp `11-14 12:41` (4);
  - units: "drainage area (m²)", "m/s, kg/m³" (2);
  - mention vs use: a sentence saying a variable lacks LaTeX (DU030), a quoted code comment (DU092), a terminal diff `+-` (DU056);
  - run metadata `o(640,5376)` (DU093);
  - "the W axis" (DU136).
- **Other:** `⟹` between case labels, `\(…\)`, a flattened paste.

The plan's own next rules are in-line rules: "`key=N` fields in log-shaped lines (a timestamp … earlier on the line)", and SI units. So the boundary is about half convention and half genre or mention judgment that the line does carry. That second half is the case for the experiment the plan already lists as untried: a model prompted for exactly the converter's residue. It doesn't change the recommendation for triggered sites.

## F5. Plan step 4(b) would widen scope into the worst-measured subpopulation

Plan step 4(b) says to "add Unicode sub/superscript digits (`W₁`, `R²`, `10⁻⁶`) to the trigger, since they are real math the trigger misses". On held-out D untriggered items, the 36 sites that carry Unicode script characters score 31% wrong+over (11 of 36):

```
W₂ ×5, W₁ᶜ, S₁, E³, X⁸, m², kg/m³
```

The other 114 score 17%. These are exactly the label and unit cases item 6 says need a convention mechanism first. The sample is small, but the direction is clear. Step (b) as written precedes the mechanism that would make it safe, which contradicts the plan's own "expand only on evidence". Reordering it after item 6, or limiting it to scripts on Greek bases, would fit the data better.

Also, "Those sites are most of what it changes estate-wide (16k sites vs 8.6k triggered)" mixes denominators. From `conv-v6`, v6 changes 7,826 triggered and 8,309 untriggered sites, so untriggered is about half, not "most". The 8.6k is all triggered sites, not the triggered sites it changes.

## F6. Plan item 3 can't be done as written, and the gates do catch some real errors

- **`edits_confined` includes `operands_survive`.** Plan item 3 says that if defense-in-depth gates are wanted, "`edits_confined` (the alignment gate) is the one that still makes sense", while `operands_survive` and the others need Unicode-aware updates. But `edits_confined` in `src/math.rs` calls `operands_survive` on every region, along with `region_reads_as_math`, a prose-weak-glyph check, and `**`/backtick checks. It isn't the pure alignment check that `edit_pairs` is. Keeping it keeps `operands_survive`.
- **On the estate, today's gates refuse 22% of the converter's triggered whole-line outputs.** I fed v6's output for all 7,826 changed triggered estate sites through `rs/probe` `judge`:

  | outcome | sites |
  |---|---|
  | accepted | 6,087 |
  | refused: "altered prose" | 986 |
  | refused: "invented math content" | 342 |
  | refused: `edits_confined` | 296 |
  | refused: residual math | 115 |
- **Most refusals look wrong, but not all.** I read a random 30 of the refusals. About 25 look like over-refusals of good conversions: `±ρ`, `π*`, a rate with a unit, `n≥3, k=2`. About 5 catch dubious output:
  - flattened PDF subscripts (`ℳθ,ϕ` → `\mathcal{M}\theta,\phi`);
  - APA statistics fragmented inconsistently;
  - the hyphen-operator class from F2;
  - `pilot-A · κ×A` → `pilot-$A \cdot \kappa \times A$` (a label hyphen plus the house prose `·`).

  So the spike's direction holds: the gates over-refuse for this converter. But the A+B accounting ("passed 2 of its 4 wrong outputs") undercounts what the gates catch, because the ladder can't see some of what they catch.
- **The edit-shape check diverges from md-press's on 3 estate sites.** These have a `$` inside inline code (`` `$RELATA_DATA_DIR/…` ``, `` `$DOCUMENT` ``). The spike's `properties.py` masks code before checking shape, while md-press's own `edit_pairs` doesn't. So "0 shape violations" holds for the spike's check, and 3 sites would fail md-press's. This bears on verifier request 3. A cheap fix is to make the converter abstain on sites with a `$` inside code, or to make md-press's `edit_pairs` aware of code.

## F7. Places where the documents disagree or are stale

- `debrief.md`: "The recommended v5 adds two malformed-input guards". The plan and `rs/umath` default to v6.
- Plan item 3: "Over 4,551 distinct emitted spans". That's the v3 count; README and debrief say 4,837 (v6).
- README: "So every v4 number below is also v5's and v6's". The numbers are above that sentence. The claim itself checks out: v4 = v5 on all 1,390 gold items, and v4 ≠ v6 on exactly 1.
- README results table: "C lines | v2 (frozen before C was looked at)". Per F1, C was labeled, and its labelers' reports read, before v2 froze.
- Plan, "Incidental md-press findings", first bullet (YAML after an HTML comment): md-press's working-tree `STATUS.md` now records this as reproduced and fixed the same day. The working-tree binary no longer joins that file. The plan reads as if it's still open.
- More generally, `rs/probe` was built against md-press's *working tree* during the spike. That tree has since gained more uncommitted changes to `src/math.rs` from another session. The trigger and the gates the integrator meets may differ from the ones measured. It's worth re-running `judge` and `trig` against whatever tree the integration lands on.

## F8. Incidental: a single `*` emphasis delimiter can be swallowed under today's gates

While running llama on D, two of its outputs that passed the gates deleted markdown emphasis:
- DL160: `For *p ≤ p_crit*:` became `For $p \leq p_{crit}$:`, losing both asterisks.
- DL172: the closing `*` of an italic line was removed, which leaves the italic unclosed.

Both were accepted by `judge`. `edits_confined`'s doc comment promises "no markdown emphasis or code delimiters", but the code checks `r.contains("**") || r.contains('`')`. A single `*` passes, and the current working-tree source still has only that check. This is independent of the converter. It applies to the LLM path that is live today, and to any converter kept behind `edits_confined`. As far as I can tell, the converter itself never swallows emphasis: none of its estate spans contains `**`, a backtick, link syntax, or an opening `*`/`_`.

---

## Checked and sound

- **The D numbers reproduce exactly** with a scorer of my own over the spike's modules, writing nothing: D lines v4 86.0% / 3.5%; D untriggered 71.3% / 20.0%; D untriggered-unchanged 50/50. Scoring against the *worse* labeler instead of the better one moves the lines only to 84.5% / 4.0%, and the untriggered items to 65.3% / 26.0%. "Match either labeler" is not doing much work on lines.
- **The labelers were blind, as far as their transcripts show.** The A+B and C briefs didn't forbid opening `py/` (the D brief does). I scanned all 20 labeler transcripts. No labeler read the converter, the model outputs, or the other pass for its items. Two A+B labelers ran `ls` on `py/` and nothing more. The documented batch-0 scratchpad collision is the only cross-pass exposure.
- **Frozen files are immutable.** Each `py/frozen/umath_v{1..6}.py` was committed once and never modified, and the hashes match the README. `py/umath.py` is identical to v6.
- **Rust/Python parity.** In an independent spot check, `rs/umath` `--v6` and `umath_v6.py` agree on 45,000 random estate sites, 5,404 of them changed: 0 differences and 0 raises.
- **The converter beats the model on held-out lines** (F3, with the caveats there).
- **Scope expansion is not safe as-is.** Confirmed, and somewhat worse under the worse labeler (26%).
- **Markdown-syntax safety on the estate.** Across all v6 spans, no replaced region contains `**`, a backtick, link syntax, or an opening `*`/`_`. The 18 regions matching `<` + letter are math relations (`E_{<i}`, `K<<N`), not HTML.

## How to reproduce my runs

My scripts and outputs are in this session's scratchpad (`…/scratchpad/verifier-denovo1/`), which may not survive the session. Each run is short to redo:
- **Re-scoring:** import `atoms` and the frozen modules, and score each item against `P1`, `P2`, best and worst, without writing `verdicts.json`. (`py/evaluate_c.py` overwrites `data/gold/{C,D}/verdicts.json` on every run, which is worth knowing before a verifier runs it.)
- **llama on D lines:**
  1. Run `rs/probe` `pieces_of` on the 200 DL texts, giving 494 pieces.
  2. For each piece, call llama3.2:3b with `py/propose_llama.py`'s exact request; the median was 0.24 s per call.
  3. Run `rs/probe` `judge` on each piece's proposal, using the already-built binaries.
  4. Re-join the pieces with their separators and score against the D gold.
- **Gates on the converter:** run `judge` on `{text: body, proposal: out}` for every changed triggered row of `data/bulk/conv-v6.jsonl`.
- **The hyphen-operator class:** for each new span in `conv-v6` (via `score.new_spans`), check whether its source region contains a non-space character, then `-`, then three or more letters.

One correction about my own conduct: my first scoring script inherited a `chdir` into the spike directory and wrote two scratch files there (`rows-D-umath_v4.json`, `rows-D-umath_v6.json`). I moved them to the scratchpad within a minute, and `git status` on the spike directory was clean afterwards. This report is the only file I've left in the tree.

## Feedback on the brief and the SOPs

- **The brief worked.** "Re-derive from what is written rather than from what you can tell was intended" did real work: it's why I compared the timestamps against "labeled after both were frozen" instead of accepting the obvious intent. The coordination fact about other sessions' uncommitted changes was useful, and so was the open scope.
- **The README's "For the independent verifier" section was the most useful thing in the spike for this pass.** It named the scorer as the spiker's least-checkable artifact, which pointed straight at F2. That deserves a place in the template.
- **The SOPs don't name the main freeze risk.** `spikes.sop.md` and the audit-routing doc carry the strengthen-first, proxy and independent-verify disciplines, and those mapped well onto this spike: the ladder and the labeler consensus are both proxies here. What they don't name is the held-out-set ordering hazard from F1: an agent's final report is the most legible text it produces, and it lands mid-development unless the freeze comes first. One line in the spike template would cover it: freeze, then launch labelers, then read their reports.

I'm available for follow-ups. I'd most like to know whether the spiker reads CU018/CU111 differently from me (F1), and whether the hyphen-operator class is already known.
