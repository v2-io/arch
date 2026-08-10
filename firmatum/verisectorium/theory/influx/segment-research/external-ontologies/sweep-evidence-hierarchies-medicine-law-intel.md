# Sweep: evidence/confidence hierarchies — medicine, intelligence analysis, law

Domain scope: institutionalized systems for grading how much to believe a claim and what to do about gaps/uncertainty, drawn from clinical evidence-based medicine, intelligence analysis tradecraft, and Anglo-American legal proof standards. This deepens an initial search pass; entries below distinguish what I verified against a primary/near-primary source this session (fetched or directly quoted from official text) from what is search-synthesized (secondary sources agreeing, primary source blocked/paywalled) versus recalled from training. Confidence is stated per entry.

---

## 1. GRADE (Grading of Recommendations Assessment, Development and Evaluation)

**(a) Structure.** Confirmed via search of GRADE Working Group guideline papers (Guyatt et al., "GRADE guidelines" series in *J Clin Epidemiol*, 2011–), though the full-text primary articles were blocked by paywall/fetch 403 this session — the structural claims below are corroborated across multiple independent secondary descriptions (search engine, PubMed abstracts) rather than pulled verbatim from the article body, so treat the exact domain-boundary wording as search-synthesized, not primary-quoted.

- Starting point: study *design* sets a default — randomized trials start **high**, observational studies start **low**.
- Then the rating is adjusted using eight independently-assessed domains:
  - **Rate DOWN** (up to −2 each, occasionally −3 cumulative): risk of bias, inconsistency (unexplained heterogeneity across studies), indirectness (population/intervention/outcome doesn't match the question), imprecision (wide confidence intervals / small sample), publication bias.
  - **Rate UP** (observational evidence only, in practice): large effect magnitude (≥2-fold → +1, ≥5-fold → +2, per the guideline-9 rating-up paper), dose-response gradient, and "all plausible confounding/bias would reduce the observed effect" (i.e., the true effect is probably even bigger than what was measured, or a null finding survives confounding that would have created an association).
- **Terminal output: four ordinal levels** — **High / Moderate / Low / Very low** certainty. Definitional shape (near-canonical wording used across GRADE literature): High = very confident the true effect lies close to the estimate; Moderate = moderately confident, true effect likely close but possibly substantially different; Low = confidence is limited, true effect may be substantially different; Very low = very little confidence, true effect likely substantially different.
- **Structural feature our atom-kinds would care about most: certainty is assigned per OUTCOME (per claim), not per study and not even per body of literature-as-a-whole.** A single systematic review can carry "High" for mortality and "Low" for a secondary quality-of-life outcome simultaneously. This directly answers a question our epistemic-map hasn't cleanly settled: the certainty rating is a property of the *claim being supported*, and one underlying evidence base can support several claims at different certainty levels at once.
- GRADE also carries a **second, separate axis**: strength of recommendation (Strong / Weak, sometimes "conditional"), which is gated by (i) certainty of evidence, (ii) balance of desirable/undesirable effects, (iii) values/preferences variability, (iv) resource use. This is a genuine evidence-confidence × decision-strength two-tier design — confidence in a claim is necessary but not sufficient for a strong action-recommendation; low-certainty evidence can still license a strong recommendation if the effect is large enough or there's no reasonable alternative (GRADE explicitly flags these as exceptions worth explaining, not silent overrides).

**(b) What it's trying to be true about.** How close an *effect estimate* (from a body of evidence, for one specific outcome) is likely to be to the true effect — i.e., calibrated confidence in a quantitative or directional claim, used to license a clinical/policy action.

**(c) Uncertainty/gap handling.** Explicit, itemized, directional (each domain both diagnoses a *specific kind* of gap and states which way it moves confidence and how far). Nothing is a single scalar "quality" score; the domains are individually documented in the GRADE evidence profile / summary-of-findings table so a reader can see *why* an outcome sits at "Moderate" rather than "High" — this transparency-of-reasons is arguably as important structurally as the four-level ladder itself.

**(d) Structural feature for our atom-kinds.** (1) Outcome-bound (not source-bound) certainty — worth importing directly. (2) The down/up asymmetry itself — five reasons to distrust, three (narrower, harder-to-meet) reasons to actively trust *more* than the design baseline — models real epistemic movement in both directions with different evidentiary bars for each direction. (3) The two-tier confidence × required-strength split (see law, §6, for the closest complementary case).

**(e) Provenance + confidence.** GRADE Working Group (est. ~2000, Guyatt, Oxman, Schünemann, et al.), used by WHO, Cochrane, UpToDate, NICE, US CDC ACIP, and reportedly 100+ organizations — canonical and extremely battle-tested. **Confidence I verified this session:** MODERATE. The CDC ACIP worked-example page and the gradepro.org handbook were not independently re-fetched this pass (search corroborates but I did not load their raw text); the *J Clin Epidemiol* primary articles 403'd. The four-level names and the per-outcome binding are stated consistently enough across independent secondary sources (PubMed abstract text, search-engine synthesis of the guideline-9 rate-up paper) that I trust them at "verified-by-corroboration" rather than "verified against primary text." Recommend the coordinator or a follow-up pass fetch https://gradepro.org/handbook/ directly (not attempted this session) if exact domain-boundary wording becomes load-bearing.
- https://gradepro.org/handbook/
- https://www.jclinepi.com/article/S0895-4356(10)00332-X/fulltext (blocked, 403, this session)
- https://www.jclinepi.com/article/S0895-4356(11)00184-3/fulltext (rate-up guideline; not re-fetched, content taken from search synthesis)
- https://www.cdc.gov/acip-grade-handbook/hcp/chapter-7-grade-criteria-determining-certainty-of-evidence/index.html

---

## 2. Oxford CEBM Levels of Evidence (2011, "2.1")

**(a) Structure.** Partially verified: the CEBM landing page (cebm.ox.ac.uk) fetched successfully but does not itself contain the table (it explicitly tells readers "do not read the table separately... read the Introductory Document and Table together" and links out to the PDF). The PDF fetch returned raw compressed binary (not machine-extractable by the fetch tool this session) — table contents below are search-synthesized from multiple descriptions of the 2011 revision, not primary-quoted.

- Ordinal levels **1 → 5** (finer subdivisions within some levels, e.g. 1a, 1b, 1c existed in earlier versions; the 2011 revision folded most of these toward integer levels with descriptive notes rather than lettered sub-tiers, per the CEBM's own framing).
- Level 1 ≈ systematic review of randomized trials / all-or-none case series (e.g., "all patients died before treatment was introduced, some now survive on it" is explicitly one of the CEBM's own worked Level-1 examples for a *harms* or *therapy* question — a striking case of a level populated by a study design that isn't a trial at all, because the effect is dramatic enough to make confounding implausible; conceptually close to GRADE's "rate up for large effect").  
  Level 5 ≈ mechanism-based reasoning / expert opinion without explicit critical appraisal.
- **The structural feature that makes CEBM worth citing distinctly from GRADE: the ladder is a MATRIX, not a single column.** Separate columns (question types) exist for at minimum: Therapy/Prevention/Etiology/Harm, Prognosis, Diagnosis, Differential diagnosis/symptom prevalence, and Economic/decision analysis — and the study design that earns "Level 1" in one column (e.g. a systematic review of RCTs, for therapy) is a structurally different kind of study than what earns "Level 1" in another column (e.g. a systematic review of cross-sectional studies with consistently applied reference standard, for diagnosis). **The rung number is comparable across columns (a 2 is "less certain than a 1" regardless of column) but the rung's REFERENT — what kind of study satisfies it — is gated by claim-type first.** This is the same design insight the IPCC system (§7 below) reaches independently from a completely different institutional lineage.
- CEBM is explicit that levels are "a shortcut for people who want to know quickly" and are not a substitute for full critical appraisal — it names its own ceiling.

**(b) What it's trying to be true about.** Which study DESIGN was used to answer a given clinical question — a mechanical/architectural marker (what kind of study is this) rather than GRADE's domain-based appraisal of a specific body of evidence's actual quality. CEBM predates and is less granular than GRADE; GRADE was explicitly developed partly in response to the "two RCTs can differ wildly in quality yet both count as Level 1" critique of pure design-based hierarchies like CEBM's.

**(c) Uncertainty/gap handling.** Weaker than GRADE — CEBM levels answer "how strong a design is this" not "how much should I actually trust this specific finding given its execution." CEBM's own explanatory document reportedly acknowledges this limitation (background/foreground question framing, levels-as-guide-not-gospel) — I was not able to verify the exact wording of that caveat this session (blocked PDF).

**(d) Structural feature for our atom-kinds.** The **claim-type-gated rung referent** — same rung number, different definitional content depending on what kind of question the atom is answering. Directly transferable design question for verisectorium: does an atom's "confidence rung" mean the same thing across all Assertion sub-kinds, or does the *kind of claim* change what counts as rung-3 evidence?

**(e) Provenance + confidence.** Centre for Evidence-Based Medicine, Oxford (Howick, Chalmers, Glasziou, et al.); the 2011 revision is the current citable version; historical lineage to 1998. Canonical and widely taught in medical schools, though explicitly less used in *current* institutional guideline-writing than GRADE (which largely supplanted it for guideline bodies while CEBM remains a teaching/quick-reference tool). **Confidence I verified this session: LOW-MODERATE.** I could not get either the raw table or the primary explanatory document to render this session (landing page fetched but table-free by design; PDF fetch returned binary; a second PDF fetch of the introductory document was not attempted). The five-question-type-column structure and its claim-type-gating are consistent across three independent search results and match my training recollection, but I did NOT see the verbatim table this session — flag this one for re-verification if precise level definitions become load-bearing.
- https://www.cebm.ox.ac.uk/resources/levels-of-evidence/ocebm-levels-of-evidence
- https://www.cebm.ox.ac.uk/resources/levels-of-evidence/explanation-of-the-2011-ocebm-levels-of-evidence (not fetched this session)
- https://www.cebm.net/wp-content/uploads/2014/06/CEBM-Levels-of-Evidence-2.1.pdf (fetched, returned binary/unreadable by the fetch tool)

---

## 3. Admiralty Code / NATO STANAG 2511 (source reliability × information credibility)

**(a) Structure.** Verified via search corroboration across the ETURWG working-group page and multiple restatements (direct WebFetch of primary NATO STANAG text was not attempted — STANAGs are often not freely hosted; the DTIC academic paper on it was fetched by search snippet, not directly by WebFetch, this session). Two **independent** axes:

- **Source Reliability**, lettered A–F:
  - A — Completely reliable ("a tried and trusted source which can be depended upon with confidence")
  - B — Usually reliable ("successful in the past but... some element of doubt in a particular case")
  - C — Fairly reliable ("occasionally been used in the past and upon which some degree of confidence can be based")
  - D — Not usually reliable ("used in the past but has proved more often than not unreliable")
  - E — Unreliable ("used in the past and has proved unworthy of any confidence")
  - F — Reliability cannot be judged ("has not been used in the past")
- **Information Credibility**, numbered 1–6:
  - 1 — Confirmed by other sources
  - 2 — Probably true
  - 3 — Possibly true
  - 4 — Doubtful
  - 5 — Improbable
  - 6 — Truth cannot be judged
- **The axes are explicitly declared independent** — a rating of "A5" (a completely trusted source reporting something judged improbable) or "F1" (a source of unknown/unjudged reliability reporting something independently confirmed by others) are both valid, meaningful combinations. This is the cleanest primary-source-adjacent example in the sweep of a genuinely orthogonal two-axis rating where neither axis is derived from or gates the other.
- **Both axes carry an explicit terminal "unknown/unjudgeable" state** (F and 6) that is NOT the worst-possible rating (that's E and 5) — it is a distinct, structurally separate "we have no basis to rate this at all" state, sitting outside the ordinal scale proper. This is a formalized uncertainty-about-uncertainty vocabulary item, not a bolt-on caveat.

**(b) What it's trying to be true about.** Two logically separable questions about a piece of reported information: (i) has THIS SOURCE been right before (a track-record/reputational judgment, independent of the current claim's content), and (ii) is THIS CLAIM corroborated/plausible regardless of who's saying it. The system's whole value proposition is that these can diverge and analysts need both numbers, not a blended one.

**(c) Uncertainty/gap handling.** The F/6 terminal states are the mechanism: "we don't know" is a first-class, nameable state on both axes, distinct from "we know it's bad" (E/5). The dissection paper I found (Dezert et al., DTIC ADA615607, on URREF reliability-vs-credibility) reportedly argues the Admiralty Code's own informal definitions get fuzzy at the boundary between reliability and credibility in practice (e.g., analysts sometimes let source-reliability leak into their credibility judgment, defeating the independence the two-axis design is supposed to buy) — this is a genuine, citable critique of the system's real-world fidelity to its own design, not just a design description. I did not fetch this PDF directly this session (search-snippet only); flagging as worth a direct read if the coordinator wants the sharpest theoretical grounding.

**(d) Structural feature for our atom-kinds.** The orthogonal who-said-it vs. is-it-corroborated split, PLUS the "cannot be judged" as a structurally distinct (not just "lowest") terminal state on each axis separately. Our epistemic-map, if it currently has a single confidence ladder, is missing at minimum the who/what split this system treats as foundational.

**(e) Provenance + confidence.** Originated British Royal Navy early 20th century; standardized as NATO STANAG 2511 / AJP-2.1 doctrine; widely adopted in OSINT and cyber threat intelligence (CTI) fusion centers well beyond military use. **Confidence I verified this session: MODERATE.** The A–F/1–6 table and definitions are corroborated across the search result directly (which itself synthesized from the ETURWG page and others) — the letter/ number definitions above are close to verbatim from that search synthesis, not from a WebFetch of the NATO primary text (I could not locate a freely hosted STANAG 2511 primary document this session; STANAGs are frequently access-controlled). Treat the definitions as high-confidence secondary-sourced, not primary-verified.
- https://eturwg.c4i.gmu.edu/?q=node%2F128
- https://apps.dtic.mil/sti/pdfs/ADA615607.pdf (not directly fetched this session — search snippet only)
- https://www.researchgate.net/figure/NATO-AJP-21-Source-Reliability-and-Information-Credibility-Scales_tbl1_328858953

---

## 4. Kent's Words of Estimative Probability → ICD-203 Probability Yardstick + Confidence Level

**(a) Structure — Kent's original (1964), verified via Wikipedia fetch this session** (Wikipedia's synthesis, not Kent's original essay text directly — but includes what appear to be direct quotes and a clean table):

| Term | Probability | Margin |
|---|---|---|
| Certain | 100% | ±0% |
| Almost certain | 93% | ±6% |
| Probable | 75% | ±12% |
| Chances about even | 50% | ±10% |
| Probably not | 30% | ±10% |
| Almost certainly not | 7% | ±5% |
| Impossible | 0% | ±0% |

Kent's stated goal (direct quote per the fetch): to "set forth the community's findings in such a way as to make clear to the reader what is certain knowledge and what is reasoned judgment." Kent explicitly criticized "expressions of avoidance ... which convey a definite meaning but at the same time either absolve[] us completely of the responsibility" — i.e., he identified hedge-language-as-liability-shield as a specific epistemic failure mode, not just vagueness in the abstract. **Notably, per the same source, Kent's own proposal was NOT adopted in his lifetime** despite being "well received" — a genuine historical non-institutionalization, worth holding against any temptation to treat "battle-tested" as automatic for intelligence-community systems.

**(b) ICD-203 (current binding standard, 2007 orig., revised 2015) — NOT independently re-verified against primary text this session** (both dni.gov and the FAS mirror either 403'd or returned empty content to the fetch tool). Structure below is search-corroborated across two independent search passes, consistent with training recollection:

- **Probability yardstick**, 7 standardized terms bucketed to ranges: almost-no-chance (1–5%), very-unlikely (5–20%), unlikely/improbable (20–45%), roughly-even-chance (45–55%), likely/probably (55–80%), very-likely (80–95%), almost-certain (95–99%).
- **Separately, a "Level of Confidence" axis** (High / Moderate / Low), rating the analyst's confidence in the JUDGMENT itself — based on quality/quantity of source reporting and soundness of analytic reasoning, NOT on the probability estimate's number. Per one search result, ICD-203 explicitly instructs analysts NOT to combine a confidence level and a likelihood term in the same sentence (e.g., not "we have high confidence this is very likely") — i.e., the two axes are meant to stay legible as separate judgments, an explicit guard against exactly the axis-conflation risk the Admiralty-code critique (§3c) flags for its own two axes.
- This is a second genuine probability-of-claim × confidence-in-judgment two-axis design, structurally different from Admiralty's source-trust × content-corroboration split: ICD-203's second axis is about the analyst's OWN epistemic position (how good was my process/inputs), not about the claim's content or its source.

**(c) Uncertainty/gap handling.** The Confidence axis (High/Mod/Low) IS the formal uncertainty-about-uncertainty layer — a declared meta-judgment about how much to trust the probability judgment itself, kept structurally separate from that judgment. Per the search synthesis, ICD-203 (2015) explicitly requires products to "note the causes of uncertainty and explain how those uncertainties affect the analysis" — i.e., the standard mandates prose explanation of gaps, not just a numeric/verbal tag.

**(d) Structural feature for our atom-kinds.** The mandated non-conflation rule (don't combine probability-language and confidence-language in one sentence) is a genuinely transferable authoring/rendering constraint, not just a taxonomy shape — worth flagging to the coordinator as something our atom *rendering* (not just data model) might want to enforce.

**(e) Provenance + confidence.** Sherman Kent, CIA Office of National Estimates (1964 essay, *Studies in Intelligence*) → institutionalized government-wide as Intelligence Community Directive 203 (ODNI, 2007, rev. 2015) — the CURRENT binding US IC standard. **Confidence I verified this session: Kent's original table — MODERATE-HIGH** (direct Wikipedia fetch, includes what read as direct Kent quotes, though Wikipedia is a tertiary source for the primary 1964 essay). **ICD-203's yardstick and confidence-axis structure — LOW-MODERATE, search-corroborated only.** I was unable to load either dni.gov or the FAS mirror this session (403 and empty-content respectively) — this is the weakest-verified load-bearing system in the sweep and should be re-fetched directly (try https://irp.fas.org/dni/icd/icd-203.pdf via a different fetch path, or the GitHub mirror at wesinator/ICD203-intel-analysis which appears to host the text in a more parseable form) before the coordinator treats exact range boundaries as settled.
- https://www.dni.gov/files/documents/ICD/ICD-203.pdf (403 this session)
- https://irp.fas.org/dni/icd/icd-203.pdf (fetched, returned empty content to the tool — worth a retry or alternate extraction method)
- https://github.com/wesinator/ICD203-intel-analysis (not fetched this session, surfaced by search, likely more parseable)
- https://en.wikipedia.org/wiki/Words_of_estimative_probability (fetched successfully this session)

---

## 5. Analysis of Competing Hypotheses (ACH) — process, not a ladder

**(a) Structure.** Not independently re-fetched this session (carried forward from the initial pass; not re-verified). A matrix method: hypotheses as columns, evidence/reasoning items as rows, each cell scored consistent / inconsistent / not-applicable with that hypothesis. The diagnostic move is scoring evidence by how well it DISCRIMINATES between hypotheses (evidence consistent with all hypotheses is non-diagnostic and gets less weight), and the recommended output is not "the hypothesis with the most supporting evidence" but "the hypothesis with the LEAST disconfirming evidence" — an explicit falsification-first design, self-aware about confirmation bias.

**(b) What it's trying to be true about.** Not a confidence-in-a-single-claim system at all — it's a comparative-hypothesis-set discipline for reaching a confidence judgment across several competing claims at once, which is a different shape than every other entry in this sweep (all of which rate ONE claim/body-of-evidence). Possibly most relevant to a verisectorium "Questions" atom-kind where multiple candidate Assertions are being tracked against each other, rather than to a single Assertion's own confidence field.

**(c)/(d)/(e)** — carried forward unverified from the initial pass; flagging for a dedicated follow-up read of https://pherson.org and the Dhami et al. 2019 peer-reviewed empirical evaluation (https://onlinelibrary.wiley.com/doi/full/10.1002/acp.3550) if ACH becomes load-bearing for the Questions family design. **Confidence: LOW (not re-verified this session, carried from search-pass findings only).**

---

## 6. Legal standards of proof + Daubert/FRE 702 admissibility gate

**(a) Structure — verified partially.** Not independently re-fetched this session (Cornell LII and Wikipedia URLs from the initial pass were not re-hit); the standard-of-proof ladder is high-confidence general legal knowledge, cross-checked against the initial pass's citation list rather than re-fetched:

- **Standard of proof** (an ordinal REQUIRED-confidence ladder, gating what a fact-finder must believe before ruling for a party): preponderance of the evidence (>50%, "more likely than not") → clear and convincing evidence (~75%, "highly probable") → beyond a reasonable doubt (~95%+, "no other reasonable explanation," the criminal-conviction standard).
- **Admissibility gate** (Daubert v. Merrell Dow Pharmaceuticals, 1993, and its progeny Kumho Tire, codified into Federal Rule of Evidence 702): a threshold test a trial judge applies BEFORE a jury weighs anything — is this expert testimony/evidence admissible AT ALL. Factors: whether the theory/technique can be (and has been) tested; whether it's been subjected to peer review and publication; its known or potential error rate; the existence and maintenance of standards controlling its application; and general acceptance in the relevant scientific community (the last being the sole factor under the older *Frye* standard that Daubert superseded federally).

**(b) What it's trying to be true about.** Two genuinely different questions: standard-of-proof is about how confident a decision-maker must BE before acting (a required-confidence-to-decide threshold — directly comparable to GRADE's strength-of-recommendation axis, §1a); Daubert/702 is about whether a piece of evidence is even a legitimate INPUT to the confidence-forming process at all — a source/method-qualification gate that sits logically upstream of any confidence rating, closer to a References/ Instruments admissibility check than to an Assertions confidence field.

**(c) Uncertainty/gap handling.** The standard-of-proof ladder doesn't itself explain gaps — it sets the bar; a case can simply fail to meet the bar and the claim is treated as not-proven (a term the legal system takes seriously as its own state — NOT proven is explicitly not the same claim as proven-false; this not-proven/proven-false distinction is arguably the single most directly reusable idea in this whole entry for verisectorium's prohibition/decision atom-kinds, since "insufficient evidence to act" and "evidence against" are kept rigorously separate). Daubert handles gaps by exclusion — inadmissible evidence isn't weighted-down, it's removed from the record entirely, a much harder gate than any of the graded-confidence systems above.

**(d) Structural feature for our atom-kinds.** (1) required-confidence (standard of proof) vs. actual-confidence as a clean distinct-axis pair, mirroring GRADE's split. (2) not-proven ≠ disproven as a first-class terminal-state distinction. (3) an admissibility GATE (binary, upstream, not gradable) as a structurally different kind of epistemic control than anything else in this sweep — worth a dedicated look for our References/Instruments families specifically, since "is this citation even usable" is a different question from "how much do I trust what it says."

**(e) Provenance + confidence.** Anglo-American common law; Daubert is US federal law (FRE 702, as amended most recently 2023) plus binding SCOTUS precedent; standards of proof vary by jurisdiction but the three-tier US civil/criminal ladder is uncontroversially standard black-letter law. **Confidence: MODERATE-HIGH on substance** (this is well-established legal doctrine I'd expect to be stable and low-risk-of-hallucination even from training alone), **but LOW on session-verification** — none of the four legal URLs from the initial search pass were re-fetched this session. Flag for direct verification if exact percentage anchors ("~75%," "~95%+" for clear-and-convincing / beyond-reasonable-doubt) become load-bearing — these percentages are illustrative glosses courts themselves are often reluctant to assign a number to; SCOTUS in *Colorado v. New Mexico* used qualitative language ("highly probable") for clear-and-convincing specifically because numeric quantification is contested doctrine, which the initial pass's own findings note but I have not re-confirmed against Cornell LII's exact page this session.
- https://www.law.cornell.edu/rules/fre/rule_702 (not re-fetched this session)
- https://en.wikipedia.org/wiki/Daubert_standard (not re-fetched this session)
- https://www.law.cornell.edu/wex/clear_and_convincing_evidence (not re-fetched this session)

---

## 7. IPCC Uncertainty Guidance (Mastrandrea et al. 2010, applied through AR5/AR6)

**(a) Structure.** Not independently re-fetched this session (carried forward from the initial search pass). Two DELIBERATELY separate axes, gated by claim-type rather than always both applying:

- **Confidence** (qualitative: very low → low → medium → high → very high), derived from a stated 2-D input (type/amount/quality/consistency of evidence, crossed with degree of expert agreement) collapsed to a 1-D output.
- **Likelihood** (quantitative, calibrated probability language: virtually certain ≥99%, extremely likely ≥95%, ... exceptionally unlikely ≤1%), used ONLY where a finding can genuinely be expressed probabilistically.
- The explicit institutional rule (per the initial pass's findings, not re-verified against the primary guidance note this session): a finding can carry high CONFIDENCE with no LIKELIHOOD assigned at all, if the finding isn't the kind of claim probability meaningfully applies to (e.g., a mechanistic/qualitative finding). This is the same claim-type-gates-which-axis-applies design CEBM reaches independently (§2a) — two unrelated institutional lineages (climate science assessment, clinical epidemiology) converging on "not every claim gets every axis; the claim's TYPE decides which epistemic vocabulary is even meaningful for it."

**(b)/(c)/(d)/(e)** — carried forward from the initial pass, **not re-verified this session**. Confidence: LOW on session-verification, though the underlying description is consistent with widely-cited climate-science methodology literature. Flag for direct fetch of https://www.ipcc.ch/site/assets/uploads/2017/08/AR5_Uncertainty_Guidance_Note.pdf if this becomes load-bearing — it did not get a WebFetch attempt this session at all (deprioritized in favor of the four systems judged most load-bearing per the initial pass's own ranking).

---

## What surprised me

1. **How much of this session's fetch attempts failed or were blocked** (dni.gov 403, FAS mirror empty, jclinepi 403, CEBM PDF unreadable-binary). This is itself a finding worth surfacing to the coordinator: several of the MOST load-bearing primary sources for this exact research question (the actual ICD-203 text, the actual GRADE guideline papers, the actual CEBM table) are genuinely hard to access programmatically — paywalled, access-controlled, or PDF-locked. If verisectorium's own methodology cares about primary-source-verification as a first-class epistemic state (and per the steward's framing it clearly does), this sweep is itself a small case study: most of what's above is at "corroborated by multiple independent secondary descriptions" rather than "verified against primary text," and I want that gap named rather than smoothed over, per this task's own instruction to write for truth including what's inconvenient.

2. **Kent's 1964 proposal was not adopted in his own lifetime**, despite being "well received" — and the search result described this as "a pattern repeated when later CIA directors attempted similar reforms." That's a genuinely different shape of "established system" than GRADE or Admiralty: it's an *authored, canonical, frequently-cited-as-if-adopted* framework whose actual institutional uptake was contested and delayed for decades before ICD-203 finally codified something related (and even ICD-203's own critics — per the initial pass's Cipher Brief citation — argue the ambiguity problem Kent identified in 1964 is still not fully solved today). Worth holding against any temptation to treat "there's a named, citable framework" as equivalent to "this is how practitioners actually behave." A framework's textbook status and its lived-practice status can diverge for 60 years.

3. **The not-proven / disproven distinction in law** (§6c) feels like the single most directly portable idea I didn't expect going in — it's not in the "findings" bullet list from the initial search pass at all (that pass focused on the ladders and axes), but reading the standard-of-proof material closely, the legal system treats "the plaintiff failed to meet their burden" as rigorously NOT the same claim as "the defendant's position was affirmatively established" — those are different epistemic states with different downstream consequences (case dismissed vs. counter-claim proven), and I don't think our current epistemic-map ladders (which I have not re-read this session, per the task framing that context is "worth reading first" — I did not in fact load TAXONOMY-FILL-v1.md or epistemic-map.md this session, which I should flag rather than pretend otherwise) necessarily preserve that distinction as sharply.

4. **What we did not ask about, that this domain clearly has more of:** forensic-science-specific reliability frameworks (the 2009 NAS report "Strengthening Forensic Science," PCAST 2016 — both are well-known, heavily-cited critiques of pattern-matching forensic disciplines' epistemic overclaiming, and neither appears in either search pass); the Bradford Hill criteria for causal inference in epidemiology (a named, canonical 9-point checklist, older than GRADE, for a DIFFERENT question — not "how much evidence" but "is this an association or a cause," which is a structurally distinct axis from everything above and might matter for how verisectorium atoms distinguish correlational from causal Assertions); Cochrane's Risk of Bias tool (RoB 2) which is the specific instrument GRADE's "risk of bias" domain leans on and has its own detailed sub-structure (five bias domains at the individual-study level, feeding UP into GRADE's outcome-level judgment — i.e., there may be a THIRD layer of granularity below GRADE's four domains that this sweep never reached); and the replication-crisis-era statistical-significance critiques (p-hacking, pre-registration, the "many labs" replication projects) which are a completely different epistemic-gap vocabulary (about whether a SINGLE study's finding is even real) sitting logically prior to any of the "how much do I trust this body of evidence" systems above. None of these were searched this session; flagging as plausible-next-territory rather than claiming coverage.

5. **A meta-observation about my own process this session:** I did not independently re-fetch or re-verify entries 5, 6, and 7 (ACH, legal standards, IPCC) — I carried them forward from the initial search pass essentially as-received, only re-organizing and confidence-labeling them. That is a real limitation of this deliverable and I want it named plainly rather than let the uniform per-entry formatting below imply uniform verification depth across all seven entries. Entries 1–4 got at least one direct fetch attempt each this session (even where those fetches failed); entries 5–7 got none.
