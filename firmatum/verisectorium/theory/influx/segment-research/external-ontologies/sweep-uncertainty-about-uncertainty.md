# Sweep: uncertainty-about-uncertainty — established systems for grading confidence, gaps, and confidence-in-confidence

*Deep-research pass, 2026-08-10. Domain: systems that formalize not just "is this true" but "how sure are we, along what dimensions, and how do we know when we don't know." Every entry below was checked against a primary or near-primary source via WebFetch except where marked otherwise; where a fetch failed to yield the load-bearing detail, that is stated rather than papered over.*

---

## 1. NUSAP (Numeral, Unit, Spread, Assessment, Pedigree)

**(a) Structure**, per the Wikipedia summary (fetched; the primary van der Sluijs PDF was also fetched but its extracted text did not surface the matrix — noted below as a verification gap):

- **Numeral** — "will usually be an ordinary number" (the value itself; can also be a linguistic descriptor for very qualitative statements).
- **Unit** — the units the Numeral is expressed in.
- **Spread** — "an assessment of the error in the value of the Numeral" (statistical spread — variance, confidence interval, or a qualitative range).
- **Assessment** — a summary judgment of the information's quality: statistical significance and/or qualitative descriptors ("conservative," "optimistic"); for model outputs, includes sensitivity-analysis results.
- **Pedigree** — "an evaluative description of the mode of production and of anticipated use of the information." Expressed as a **matrix**: columns are the phases/aspects of the number's production (search turned up, across applications: *proxy* [how well does the measured quantity represent the target quantity], *empirical basis*, *method* (methodological rigor), *validation*, and sometimes a fifth, *theoretical understanding*); each column scored 0–4 with linguistic anchors per cell. A worked example (via search, not directly fetched — flagged as recalled/secondary): score 4 = "exact measure / large sample direct measurements / well-established theory / best available practice," descending to 1 = "weak correlation / educated guesses," with 0 typically "not applicable / unknown." A real application reported average pedigree scores per criterion across parameters (e.g. proxy 2.4, method 1.8, empirical 1.8, theory 2.0, validation 1.1, all 0–4) — i.e., pedigree is *aggregated descriptively across a corpus*, not just per-atom.

**(b) True about**: quantitative claims/parameters embedded in policy-facing models — especially claims where the *number's provenance quality* is itself contestable and needs to travel with the number rather than being asserted once and forgotten.

**(c) Uncertainty/gap handling**: NUSAP's entire reason for existing is to make "confidence in the number" *itself* a structured, multi-dimensional, inspectable object rather than a single scalar. It refuses to collapse "how good is this number" into the number's own error bars (Spread handles that); Pedigree is explicitly a *second, independent* axis about the *production process*, not the value. This is the cleanest primary-literature precedent found anywhere in this sweep for "confidence about confidence" as a structural (not narrative) feature.

**(d) Feature our atom-kinds would care about**: the Pedigree matrix is a template for a *named, multi-dimension provenance-quality score* that could sit alongside our Evidence axis — separate from "is the claim itself well-supported" and answering instead "was the process that produced this claim itself trustworthy." It also demonstrates that pedigree scores are *comparable and aggregable* across a corpus (a possible instrument-family feature: a manifest could report mean pedigree by dimension across all Assertions).

**(e) Provenance/confidence**: Silvio Funtowicz & Jerome Ravetz, *Uncertainty and Quality in Science for Policy* (Kluwer, 1990); institutionalized at Utrecht/JRC (van der Sluijs et al.). Battle-tested within EU/Dutch environmental-policy science (RIVM, PBL); not mainstream outside that community. **Confidence: MEDIUM.** The five-component top-level structure (Numeral/Unit/Spread/Assessment/Pedigree) is verified against Wikipedia's own summary of the primary literature. The pedigree matrix's exact scoring anchors and dimension names were only surfaced via secondary search snippets (the primary PDF fetch did not extract usable text) — treat the specific dimension list and 0–4 anchors as *plausible-and-consistent-across-sources* rather than *directly read off the primary*.
- https://en.wikipedia.org/wiki/NUSAP
- https://en.wikipedia.org/wiki/Uncertainty_and_Quality_in_Science_for_Policy
- http://www.andreasaltelli.eu/file/repository/08_vdSluijs_et_al2005.pdf (fetched; text extraction did not surface the matrix directly)
- https://www.nusap.net/sections.php?op=viewarticle&artid=12 (pedigree matrix page — found via search, not fetched)
- https://www.nusap.net/spe/NUSAPCases2011.pdf (worked application examples — found via search, not fetched)

---

## 2. IPCC calibrated uncertainty language — confidence (evidence × agreement) vs. likelihood

**(a) Structure**, verified via fetch of the Climate-ADAPT summary (the primary AR5 Guidance Note PDF itself 403'd on fetch — noted as a gap):

- **Confidence** is a qualitative synthesis built from two named inputs: **evidence** (rated Limited / Medium / Robust — a joint function of the amount, quality, and consistency of underlying evidence) crossed with **agreement** (rated Low / Medium / High — agreement among experts/studies). These two are laid out as a 2-D grid ("evidence" one axis, "agreement" the other) that authors are told is NOT to be read as strictly ordinal in either direction alone; overall **confidence** is then summarized on a five-step scale: **Very Low → Low → Medium → High → Very High**, with confidence increasing toward the high-evidence/high-agreement corner of the grid.
- **Likelihood** is a *separate* scale, used only when a probabilistic/quantitative estimate is actually available, with specific calibrated terms mapped to probability ranges (as fetched from Climate-ADAPT):

  | Term | Probability |
  |---|---|
  | Virtually certain | 99–100% |
  | Extremely likely | 95–100% |
  | Very likely | 90–100% |
  | Likely | 66–100% |
  | More likely than not | >50–100% |
  | About as likely as not | 33–66% |
  | Unlikely | 0–33% |
  | Very unlikely | 0–10% |
  | Extremely unlikely | 0–5% |
  | Exceptionally unlikely | 0–1% (from the search-findings pass; not independently re-verified in this fetch) |

  Note the asymmetry in this particular published table: "unlikely," "very unlikely," and "extremely unlikely" are all given as ranges starting at 0 (i.e. nested upper bounds, not a partition) — a reminder that even a canonical calibrated-language table is not always a clean partition and needs to be read carefully rather than assumed regular.

- The guidance explicitly instructs: use confidence language and/or likelihood language, but **do not conflate them** — confidence is about the *state and quality of understanding*, likelihood is about the *probability of a specific outcome*, and a finding can have high confidence in a qualitative statement without any probabilistic likelihood being assignable at all (and vice versa, rarely).

**(b) True about**: scientific findings synthesized into policy-relevant statements, specifically where the *degree of scientific agreement and evidentiary depth* needs to be separable from *the probability of a specific future/physical outcome*.

**(c) Uncertainty/gap handling**: this is the strongest primary-literature precedent for treating "how sure are we this claim is even right" (confidence, from evidence×agreement — an *epistemic-state* judgment) as structurally distinct from "what's the probability of the thing happening" (likelihood — an *outcome-probability* judgment). It does not have an explicit terminal/gap state beyond "very low confidence," and does not appear to formally define what happens when evidence and agreement point in different directions (e.g. robust evidence but low agreement) beyond noting "misleading" combinations are possible and should be handled with care in prose — this is a soft spot in the framework, not a hard rule.

**(d) Feature our atom-kinds would care about**: the clean two-axis separation (evidentiary-confidence vs. outcome-likelihood) is a direct structural precedent for keeping an atom's "how well-supported is this claim" axis distinct from any "what's the probability this is true / this will hold" axis — exactly the conflation risk our organic ladders are prone to. Also notable: authors are *required* to choose which register applies per-finding rather than free-mixing, i.e. the discipline is enforced at the point of assertion, not cleaned up after.

**(e) Provenance/confidence**: Mastrandrea et al., IPCC Guidance Note on Consistent Treatment of Uncertainties (2010); applied and refined AR5→AR6. IPCC is the most heavily scrutinized, most-cited calibrated-uncertainty framework in existence. **Confidence: MEDIUM-HIGH.** The confidence-grid structure and likelihood table were fetched from a secondary-but-close summary (Climate-ADAPT, an EU knowledge platform that reproduces the IPCC table directly) because the primary PDF 403'd; the table shape and five-step confidence scale match what is widely and consistently reported elsewhere, but I did not personally read the primary Guidance Note's exact prose in this pass.
- https://www.ipcc.ch/site/assets/uploads/2017/08/AR5_Uncertainty_Guidance_Note.pdf (403 on fetch — could not verify directly)
- https://climate-adapt.eea.europa.eu/en/knowledge/tools/uncertainty-guidance/topic2 (fetched successfully; used for the table above)
- https://link.springer.com/article/10.1007/s10584-011-0178-6 (Mastrandrea et al., journal version — not fetched this pass)

---

## 3. GRADE (Grading of Recommendations Assessment, Development and Evaluation)

**(a) Structure**, verified via fetch of Wikipedia (GRADE article) plus a search cross-check against the CDC ACIP GRADE Handbook and Cochrane materials for the downgrade/upgrade domains:

- **Certainty of evidence** — four ordinal rungs, each defined not as "amount of evidence" but as *confidence that the effect estimate is adequate to support a decision*:
  - **High** — "high confidence that the true value of the estimate of interest is at one side of a threshold of interest or within a specific range" (fetched, near-verbatim from Wikipedia's paraphrase of the GRADE definition).
  - **Moderate**, **Low**, **Very Low** — same construction, decreasing confidence.
- **Five domains that can downgrade** certainty (confirmed via search cross-check against CDC/Cochrane summaries, consistent across sources): **risk of bias**, **inconsistency** (heterogeneity of effects across studies), **indirectness** (studies don't directly address the question asked), **imprecision** (confidence interval wide enough that the clinical/practical conclusion would change), **publication bias**.
- **Three domains that can upgrade** (for observational evidence, which normally starts lower than RCT evidence): **large magnitude of effect** (e.g., relative risk > 5, unlikely to be explained by bias alone), **dose-response gradient**, and **plausible confounding that would work against the observed effect** (i.e., if confounding is present it would only shrink the effect, so the true effect is probably at least as large as observed).
- **Strength of recommendation** — a *separate, downstream* axis: **Strong** vs. **Conditional (weak)**. Importantly, per the Wikipedia fetch, this is explicitly NOT a symmetric second ladder derived mechanically from certainty — it synthesizes certainty of evidence together with problem importance, values/preferences variability, benefit-vs-harm balance, resource implications, equity, acceptability, and feasibility. High certainty does not automatically produce a strong recommendation, and low certainty does not preclude one (rare, but explicitly allowed by GRADE for high-stakes decisions).

**(b) True about**: the reliability of an *effect estimate* drawn from a body of empirical (usually clinical) studies, as a foundation for a *practical recommendation* — GRADE is explicit that certainty-of-evidence and strength-of-recommendation are two different questions answered by two different (related but non-identical) processes.

**(c) Uncertainty/gap handling**: this is the most fully **decomposed, named-factor** ladder found in this sweep — every rung-transition (High→Moderate→Low→Very Low) is required to be attributable to one or more of the eight named domains, not a vibe or a holistic impression. This is a strong precedent for "every status change is a projection of a named event/reason," which is close to language already in our own taxonomy (Assertions family: "settled only by re-derivation against primaries"). GRADE does not have an explicit "unknown unknowns" or ignorance-terminal state — its floor is "very low confidence," which is still a confidence-in-a-known-claim state, not a "we don't know what we don't know" state. That gap is filled better by Walker/Harremoës (entry 5) and Knight/Ellsberg (entry 7).

**(d) Feature our atom-kinds would care about**: the clean separation of **evidence-certainty** (an epistemic-state property of a claim) from **recommendation-strength** (a decision/action property built partly, but not entirely, on top of it) is a close structural analog to the separation our taxonomy is already reaching for between an Assertion's epistemic status and a Decision's status — and GRADE's explicit statement that strength does not mechanically follow from certainty (a "high evidence, weak recommendation" or "low evidence, strong recommendation" cell can both be legitimate) is a useful caution against assuming our own Decision status should just be a function of upstream Assertion confidence.

**(e) Provenance/confidence**: GRADE Working Group, ~2000-present; adopted by 110+ organizations (Cochrane, WHO, NICE, CDC/ACIP). The single most institutionally dominant evidence-certainty system in medicine. **Confidence: MEDIUM-HIGH.** The four-rung definitions were fetched directly from Wikipedia's paraphrase of GRADE's own construct-clarification paper; the eight named domains (five down / three up) were cross-checked via search against CDC's ACIP GRADE Handbook and Cochrane materials and are consistent across all sources found, but I did not fetch the GRADE Handbook or the ScienceDirect "clarifies the construct" paper directly in this pass — treat the domain *list* as verified-by-convergence rather than verified-against-a-single-primary-fetch.
- https://en.wikipedia.org/wiki/The_Grading_of_Recommendations_Assessment,_Development_and_Evaluation_(GRADE)_approach (fetched)
- https://www.cdc.gov/acip-grade-handbook/hcp/chapter-7-grade-criteria-determining-certainty-of-evidence/index.html (found via search, not fetched)
- https://www.cdc.gov/acip-grade-handbook/hcp/chapter-8-domains-decreasing-certainty-in-the-evidence/index.html (found via search, not fetched)
- https://www.sciencedirect.com/science/article/pii/S089543561630703X (GRADE's own construct-clarification paper — not fetched)
- https://www.gradeworkinggroup.org/

---

## 4. Admiralty Code / NATO source-reliability × information-credibility

**(a) Structure**, verified via direct Wikipedia fetch, which reproduces the scale definitions in near-full:

- **Source Reliability** (A–F, a property of the *originator*, assumed to change slowly):
  - A — Completely reliable: no doubt of authenticity, trustworthiness, or competency; history of complete reliability.
  - B — Usually reliable: minor doubt; history of valid information most of the time.
  - C — Fairly reliable: doubt, but has provided valid information in the past.
  - D — Not usually reliable: significant doubt, but has provided valid information in the past.
  - E — Unreliable: lacking authenticity/trustworthiness/competency; history of invalid information.
  - F — Reliability cannot be judged: no basis exists for evaluation.
- **Information Credibility** (1–6, a property of *this specific report*):
  - 1 — Confirmed by other independent sources; logical in itself; consistent with other information.
  - 2 — Probably true: not confirmed, logical, consistent with other information.
  - 3 — Possibly true: not confirmed, reasonably logical, agrees with *some* other information.
  - 4 — Doubtful: not confirmed, possible but not logical, no other information on the subject.
  - 5 — Improbable: not confirmed, not logical, contradicted by other information.
  - 6 — Truth cannot be judged: no basis exists for evaluating validity.

  The two are combined into a code like "B2" — reliable source, probably-true-but-unconfirmed report.

**(b) True about**: individual intelligence/reconnaissance reports, where the trustworthiness of *who is telling you this* and the plausibility/corroboration of *this specific claim* are genuinely separable and need independent tracking.

**(c) Uncertainty/gap handling**: both scales include an explicit **"cannot be judged" terminal/null state** (F and 6) distinct from the low-confidence end of the scale (E and 5) — i.e., the system distinguishes "we've judged this to be bad" from "we have no basis to judge this at all," which is exactly the known-unknown-vs-can't-even-assess distinction our uncertainty-about-uncertainty question is chasing. Neither scale has a numeric probability attached (unlike IPCC likelihood) — it stays qualitative/ordinal by design.

**(d) Feature our atom-kinds would care about**: the two axes are deliberately **orthogonal by construction** — a normally-A-rated source can file an unconfirmable report (A6), and a normally-unreliable source can happen to be corroborated (F2) — this is the cleanest illustration in the sweep of "who/what said it" as a genuinely separate axis from "does this specific claim check out," directly relevant to how our References/Accounts families might separate source-trust from claim-corroboration. The Wikipedia fetch of this article did **not** surface the documented critiques (halo effect between the two axes, false-precision concerns) that the initial search findings claimed exist — those critiques were reported in the original search pass citing a Blockint article, which was not independently re-fetched here; treat "documented halo-effect critique exists" as **plausible but not directly re-verified** in this pass.

**(e) Provenance/confidence**: British Naval Intelligence, 1939; formalized in NATO STANAG/AJP-2.1. 85+ years of continuous military/intelligence use; adopted into cyber-threat-intel (SANS) and OSINT practice. **Confidence: HIGH** on the scale definitions themselves (directly fetched from Wikipedia, which itself is a reasonably faithful secondary rendering of the widely-reproduced NATO table); **MEDIUM** on the critique material (not independently re-verified this pass).
- https://en.wikipedia.org/wiki/Admiralty_code (fetched)
- https://www.researchgate.net/figure/NATO-AJP-21-Source-Reliability-and-Information-Credibility-Scales_tbl1_328858953 (not fetched)
- https://www.sans.org/blog/enhance-your-cyber-threat-intelligence-with-the-admiralty-system (not fetched)
- https://www.blockint.nl/intel-analysis/critical-review-of-the-admiralty-code/ (critique source — not independently re-fetched this pass; carried over from search findings)

---

## 5. Walker / Harremoës uncertainty taxonomy (Location × Level × Nature)

**(a) Structure**, verified via WebSearch cross-checking multiple secondary sources (the primary PBL/DOI paper's abstract page was fetched but did not surface the taxonomy content itself — a real verification gap, flagged):

Three orthogonal dimensions:

- **Location** — *where* in a model/analysis the uncertainty resides: context, model structure, model technical implementation (code), model inputs, parameters, and model outcome are the named locations (per secondary-source convergence).
- **Level** — an explicit **ordered spectrum**, from full determinism to total ignorance, in five named steps:
  1. **Determinism** (no uncertainty acknowledged/present).
  2. **Statistical uncertainty** — "imprecision, or sampling error, that can be described adequately in statistical terms" (a quantifiable distribution exists).
  3. **Scenario uncertainty** — "a range of possible outcomes, but the probability of a particular outcome occurring cannot be formulated" (the outcome-space is known; the distribution over it is not).
  4. **Recognized ignorance** — "more fundamental uncertainty, where we do not even know the range of possible outcomes" — but critically, **we know that we don't know** (the *fact* of the gap is recognized, its *content* is not).
  5. **Total ignorance** — "a deeper level of uncertainty, to the extent that it is unknown what is unknown" — the true unknown-unknown terminal state.
- **Nature** — orthogonal to both of the above: **epistemic** uncertainty (reducible in principle, from imperfect knowledge — more study could close it) vs. **variability/stochastic/ontological** uncertainty (irreducible, inherent randomness in the system itself — no amount of further study eliminates it).

A related refinement (Harremoës, cited alongside Walker) further splits one flavor of gap into **practical indeterminacy** (too many functional relationships to resolve, but resolvable in principle with enough effort) vs. **theoretical indeterminacy** (the relationships are inherently undefinable — not a resource problem, a structural one).

**(b) True about**: uncertainty embedded in model-based decision-support systems (environmental, water-resource, integrated-assessment/climate modeling) — the taxonomy is meant to let an analyst locate *where* a model's uncertainty lives, *how bad* (structurally) it is, and *whether it is even the kind of thing more research could fix*.

**(c) Uncertainty/gap handling**: this is the **best-formed gap-taxonomy found in the entire sweep**. The Level dimension gives explicit, named, ordered rungs between "I have a quantified distribution" and "I don't even know what I don't know" — with **recognized ignorance** as a formally named intermediate state distinguishing *known-gap-unquantifiable-content* from *total-ignorance* (unknown-that-there-is-a-gap-at-all). This directly answers the brief's "uncertainty about uncertainty" question with a real, citable, three-decade-old academic vocabulary rather than an invented one. The Nature axis additionally answers "is this gap closable in principle" — a genuinely distinct question from "how bad is the gap right now," which our own axis design conflates unless kept separate deliberately.

**(d) Feature our atom-kinds would care about**: (i) **recognized ignorance** as a named, citable rung is a strong candidate vocabulary term for an atom that explicitly declares "we know we don't know X, but can't characterize the gap's shape" — distinct from both "we have a spread/error bar" (statistical uncertainty) and "we haven't noticed this gap exists" (total ignorance, which by definition can't be declared by the atom itself — only inferred externally, a structurally interesting asymmetry: *total ignorance is the one state a well-formed atom system can never self-report*). (ii) The epistemic/aleatory split answers our "ceiling" concept directly: epistemic uncertainty has no principled ceiling (more work can always in principle close it), aleatory/variability uncertainty has a hard ceiling (no amount of work closes it — it is a property of the system, not the knowledge).

**(e) Provenance/confidence**: Walker, Harremoës, Rotmans, van der Sluijs, van Asselt, Janssen, Krayer von Krauss, *Integrated Assessment* 4(1), 2003; RAND Europe / DTU / Utrecht / PBL collaboration. Standard reference in integrated-assessment/climate-adaptation uncertainty literature since 2003; PBL Netherlands continues to apply it. **Confidence: MEDIUM.** The core structure (three dimensions, five-step Level ladder, epistemic/aleatory Nature split) converged consistently across multiple independent secondary sources found via search (a PharmacoEconomics paper, a ScienceDirect uncertainty-frameworks review, a Deltares white paper, and a personal-blog close-reading of the original paper), which gives reasonable confidence in the shape — but I was not able to fetch the primary 2003 paper's actual text (DOI-gated / PBL summary page only carried metadata), so the exact wording of the five Level definitions above is a **near-verbatim reconstruction from convergent secondary paraphrase, not a direct primary quote** — flagged honestly rather than presented with false verbatim authority.
- https://www.pbl.nl/en/publications/defining-uncertainty-a-conceptual-basis-for-uncertainty-management-in-model-based-decision-support (fetched; metadata only, no taxonomy content)
- https://link.springer.com/article/10.1007/s40273-014-0201-7 (secondary source used for convergence check, not fetched directly)
- https://www.sciencedirect.com/science/article/pii/S0016328722000180 (secondary source used for convergence check, not fetched directly)
- https://djmarsay.wordpress.com/decisions/guidance/walkers-defining-uncertainty/ (a close, apparently careful secondary reading of the original — not independently fetched)
- DOI: 10.1076/iaij.4.1.5.16466 (primary; not accessed)

---

## 6. ASME V&V / VVUQ (Verification, Validation, Uncertainty Quantification)

**(a) Structure**, verified via direct fetch of ASME's own page:

- **Verification** — "performed to determine if the computational model fits the mathematical description" (near-verbatim, fetched). A purely internal, code-correctness question: did we solve the equations right.
- **Validation** — "implemented to determine if the model accurately represents the real world application" (near-verbatim, fetched). An external, empirical question: did we solve the right equations — does the model match physical reality via comparison to measurement.
- **Uncertainty Quantification** — "conducted to determine how variations in the numerical and physical parameters affect simulation outcomes" (near-verbatim, fetched). This is explicitly downstream of, and dependent on, V&V having already been done — UQ answers "given the model is right and correctly solved, how much does the answer move under parameter/input variation."

**(b) True about**: computational/simulation models (originally engineering/physics simulation; now used across aerospace, DoD, DOE, FAA, automotive, bioengineering, and NRC/NASA regulatory contexts) — specifically the credibility of a *model's output* as a stand-in for reality.

**(c) Uncertainty/gap handling**: this triad is less about grading confidence on an ordinal scale and more about **decomposing "is this model trustworthy" into three independent, sequentially-dependent questions**, each with its own failure mode: a model can be perfectly *verified* (bug-free implementation of its equations) yet completely *invalid* (the equations themselves don't describe reality) — these are genuinely different failures requiring genuinely different fixes (debugging vs. re-theorizing), and UQ only means anything once both are established.

**(d) Feature our atom-kinds would care about**: the Verification/Validation split maps unusually cleanly onto a distinction our own taxonomy may be under-differentiating — "is this internally consistent / correctly derived from its own stated premises" (verification, an internal-coherence check) vs. "does this correspond to the actual world / actual usage" (validation, an external-correspondence check). Our Instruments family's "settled by re-derivation against the population (proxy discipline)" language is closer to *verification*; our Assertions family's "settled only by re-derivation against primaries" is closer to *validation* — but neither family currently names the distinction explicitly, and this triad suggests it might be worth doing so, since a claim can fail either way for genuinely different reasons.

**(e) Provenance/confidence**: ASME V&V Standards Committee (formed 2001); formal ASME/ANSI standards (V&V 10, V&V 20, VVUQ 1-2022); cross-industry adoption including NASA/AIAA companion standards and NRC regulatory use. **Confidence: HIGH** — definitions fetched directly and near-verbatim from ASME's own page.
- https://www.asme.org/codes-standards/publications-information/verification-validation-uncertainty (fetched)
- https://www.asme.org/codes-standards/vvuq-standards (not fetched, listed by ASME as the standards hub)
- https://nvlpubs.nist.gov/nistpubs/ir/2020/NIST.IR.8298.pdf (NIST industrial survey — not fetched this pass)

---

## 7. Knightian risk vs. uncertainty / Ellsberg ambiguity / Klibanoff-Marinacci-Mukerji smooth ambiguity

*(Not independently re-fetched this pass — carried forward from the search-findings pass with confidence marked accordingly, because it is the only entry in the sweep that gives a fully rigorous mathematical form of "confidence in confidence," and dropping it for lack of a fresh fetch would leave a real gap.)*

**(a) Structure** (as reported, not re-verified against primary this pass):
- **Knight (1921)**: **risk** = outcomes with a *known* probability distribution; **(Knightian) uncertainty** = the distribution itself is unknown or unknowable — a foundational distinction between "we don't know which outcome" and "we don't even know the odds."
- **Ellsberg (1961)**: formalized **ambiguity** experimentally — the Ellsberg paradox shows people systematically prefer known-odds bets (a 50/50 urn) over unknown-composition-odds bets (an urn of unknown mixture) even when the *expected value* is identical, violating the Savage axioms that underpin classical (first-order) subjective probability. This is empirical proof that "uncertainty about the probability" is treated by real reasoners as a *distinct, aversive* category from "uncertainty about the outcome given a known probability."
- **Klibanoff, Marinacci, Mukerji (2005)**: the **smooth ambiguity model** — a rigorous decision-theoretic formalization: a **second-order probability distribution over candidate first-order probability models**, combined with a separate function capturing the decision-maker's *attitude toward ambiguity itself* (distinct from ordinary risk-aversion). This is, structurally, the actual mathematics of "a probability distribution over which probability distribution is correct" — i.e., confidence-in-confidence given fully formal treatment.

**(b) True about**: rational belief and choice under conditions where the first-order probability model itself is contested, unknown, or actively multiple — squarely inside the brief's "uncertainty about uncertainty" territory, and unlike every other entry in this sweep, this is a **formal mathematical apparatus**, not a grading rubric or checklist.

**(c) Uncertainty/gap handling**: this family is the theoretical ceiling of the whole sweep — it doesn't grade or checklist uncertainty, it *models* second-order uncertainty as a mathematical object with its own probability measure. Worth naming clearly to Joseph/the coordinator: everything else in this sweep is a practical instrument that approximates or operationalizes some corner of what this formalism describes exactly.

**(d) Feature our atom-kinds would care about**: if our taxonomy ever wants a rigorous *name* for "we are not just uncertain about the claim, we are uncertain about how confident we should be in our own uncertainty-assessment process," this is the field (ambiguity theory) that already owns that concept formally, with 65+ years of foundational and applied work behind it — a strong candidate for citation-grounding an "ambiguity" or "meta-uncertainty" concept rather than inventing our own vocabulary for it.

**(e) Provenance/confidence**: Frank Knight (1921, foundational economics); Daniel Ellsberg (1961, one of the most-replicated results in behavioral economics); Klibanoff-Marinacci-Mukerji (*Econometrica* 2005, standard reference in decision theory). **Confidence: LOW-MEDIUM (not re-verified this pass)** — this entire entry is carried forward from the initial search-findings pass without independent WebFetch verification against a primary source in this deepening pass; the concepts themselves (Knightian uncertainty, Ellsberg paradox, KMM smooth ambiguity) are extremely well-established and would very likely confirm cleanly on a fetch, but that fetch was not actually performed here — flagging honestly rather than borrowing verification's authority for material only recalled/cross-checked via the original search pass.
- https://en.wikipedia.org/wiki/Ambiguity_aversion (not fetched this pass)
- https://faculty.wcas.northwestern.edu/msi661/Ambiguity-06-29-2013.pdf (Machina & Siniscalchi survey — not fetched this pass)

---

## 8. Reporting-standards family (PRISMA / CONSORT / STROBE / GRADE-extension) — mandatory gap-declaration as checklist

*(Also not independently re-fetched this pass; carried forward with confidence marked down accordingly — included because it answers a different question than the confidence-ladders above: not "how do we grade uncertainty" but "how do we force disclosure of it to actually happen.")*

**(a) Structure** (as reported): a family of **itemized reporting checklists** coordinated by the EQUATOR Network — PRISMA (systematic reviews), CONSORT (randomized trials), STROBE (observational studies), STARD (diagnostic accuracy), MOOSE (observational meta-analyses), SPIRIT (trial protocols). Not confidence ladders — checklists of *required disclosure items*, including a mandatory, itemized "Limitations" section as a checklist line item rather than optional discursive prose. PRISMA has a formal extension tying review-reporting explicitly to GRADE certainty ratings.

**(b) True about**: the *reporting* of empirical research, specifically forcing authors to disclose gaps/limitations as a structural requirement of publication rather than leaving disclosure to individual authorial honesty/discretion.

**(c) Uncertainty/gap handling**: the load-bearing point here isn't a taxonomy at all — it's the demonstration that **"declare your gaps" can be operationalized as an audited checklist item**, and that adherence to such checklists is itself measured and found *imperfect* even under a formal mandate. That is a sobering, honest data point for our own effort: even with a named required field and institutional enforcement, gap-declaration compliance in the wild is incomplete. Worth carrying into the design conversation as a caution against assuming a "declare your gap" field alone solves the honesty problem — enforcement/verification is a separate, additional problem.

**(d) Feature our atom-kinds would care about**: precedent for treating "did you declare your limitations" as itself an auditable, checklist-style property of a record (closer to our Process-state "vector of resettable check-flags" idea than to an ordinal ladder) — and the empirical finding that such checklists are imperfectly followed even when institutionally mandated is a genuine caution for any instrument we design to police atom-level gap-declaration.

**(e) Provenance/confidence**: EQUATOR Network (coordinating body); PRISMA/CONSORT/STROBE are each individually very well-established and widely mandated by journals. **Confidence: LOW (not re-verified this pass)** — carried forward from the search-findings pass without independent fetch.
- https://www.equator-network.org/library/reporting-guidelines-under-development/reporting-guidelines-under-development-for-systematic-reviews/ (not fetched this pass)

---

## 9. Known-Unknowns / Johari Window (primitive, not a rigorous citation)

Retained from the search-findings pass as a **conceptual primitive** only — the 2×2 known/unknown structure underlying "recognized ignorance" and "unknown unknowns" as formal concepts, traceable to Luft & Ingham's 1955 Johari Window (interpersonal psychology, not epistemology) and popularized in the Rumsfeld sense in 2002 (predates him in military/NASA usage). Lower rigor than every other entry here; useful as the ancestor concept behind Walker/Harremoës's "recognized ignorance" and "total ignorance" rungs (entry 5), not as an instrument to adopt on its own. Not independently re-verified this pass.

---

## What surprised me / what we did not ask about

**Surprised me:**

1. **"Total ignorance" may be structurally un-self-reportable.** Walker/Harremoës's Level ladder puts "recognized ignorance" (we know we don't know, can't characterize it) as a formal rung *below* "total ignorance" (we don't know that we don't know). But an atom system can only ever *declare* recognized ignorance — by definition, nothing in the system can self-declare total ignorance, because declaring it requires already recognizing the gap, which converts it into recognized ignorance. This means "total ignorance" can only ever be an *externally inferred* state (a later reader/auditor notices a gap the original atom never flagged), never a status an atom itself can carry. That's a real structural constraint worth naming explicitly if our taxonomy wants a "we don't even know what we don't know" concept — it cannot live as a settable field on the atom that's missing the knowledge; at most it can live as an *externally-assigned* audit finding about the atom's neighborhood.

2. **GRADE's strength-of-recommendation is explicitly NOT a mechanical function of certainty-of-evidence.** I expected (and the initial search findings implied) something closer to a clean two-tier system where strength just follows from certainty. It doesn't — GRADE explicitly allows strong recommendations on low-certainty evidence (high-stakes, low-cost, no-good-alternative situations) and weak recommendations on high-certainty evidence (small/marginal benefit, high cost/burden). This is a genuinely important caution against assuming our own Decision-status axis should derive mechanically from upstream Assertion-confidence — GRADE, the most battle-tested system in this sweep for exactly that kind of two-layer design, deliberately keeps the layers non-mechanically coupled.

3. **The IPCC likelihood table, even in its official canonical form, is not a clean partition.** "Unlikely" (0–33%), "very unlikely" (0–10%), and "extremely unlikely" (0–5%) are nested, overlapping ranges sharing the same floor, not a partition of probability space into disjoint bins. Even the most scrutinized calibrated-language system in existence has this irregularity, apparently by design (each term anchors an upper bound, and stronger terms are meant to be used when applicable rather than the weaker enclosing term) — worth remembering as a caution against assuming our own eventual ladder needs to be (or even can cleanly be) a strict partition.

4. **NUSAP's Pedigree scores are treated as an aggregable corpus-level statistic**, not just a per-atom label — an actual application reported *mean* pedigree scores per dimension across all parameters in a model. That's a genuinely useful idea for an Instruments-family manifest: not just "does this atom have a pedigree score" but "what's the corpus's mean pedigree profile, and where is it weakest" — a corpus-level health metric derived the same way a de-novo auditor would want to ask "how good is our evidence, in aggregate, and along which dimension is it worst."

5. **Verification-vs-Validation (ASME) doesn't map onto anything explicit in our current taxonomy**, despite feeling like it should. We distinguish "fails by being false" (Assertions) from "fails by lying about the status of what it tracks" (Instruments), but nowhere do we currently separate "is this internally consistent with its own stated premises" from "does this correspond to the actual/external world" as two independently-failing checks on the same claim. A claim can be perfectly derived from false premises (verified, not validated) or can happen to be true despite a broken derivation (validated, not verified) — these are different failure modes needing different fixes, and I don't see either of our current axis lists (Evidence, Ceiling, Grain, Salience, Authority, Freshness...) cleanly carrying this distinction.

**What we did not ask about, that this sweep suggests is worth a follow-up pass:**

- **Software/engineering "Technical Debt" and "Definition of Done" frameworks** — these are established, battle-tested industry vocabularies for "known incompleteness that is explicitly tracked rather than hidden," adjacent to our Questions/Practices families, and not covered at all in this sweep (which stayed mostly in science-policy/medicine/military/decision-theory).
- **Actuarial/insurance "credibility theory"** — a formal statistical framework (Bühlmann credibility, etc.) for exactly "how much should we trust this specific dataset's estimate vs. a broader prior," which is arguably the closest existing rigorous mathematical machinery to "confidence in confidence, quantified," and was not touched at all in either search pass.
- **Legal standards of proof** (preponderance of evidence / clear and convincing / beyond reasonable doubt) as an ordinal confidence ladder with centuries of case law refining exactly how gaps and ambiguity get handled procedurally — a completely different institutional lineage from science/medicine/military that might have useful structural lessons (e.g., burden-of-proof shifting, which has no analog anywhere in this sweep) and wasn't explored.
- **Bayesian model comparison / model uncertainty (Bayesian model averaging)** — the formal statistical machinery for "uncertainty over which model is correct," a close statistical cousin of the KMM smooth-ambiguity entry but from mainstream Bayesian statistics rather than decision theory, not investigated here.
- We did not investigate how any of these systems handle **retraction or downgrade of a previously-high-confidence claim** — every system surveyed describes how to *arrive at* a confidence rating, but not explicitly how to *revise it downward later* when new evidence undermines an old high-confidence claim (versus simply re-running the process fresh). This is close to our own "integration-is-replacement" concern (see steward's CLAUDE.md) and might be worth a dedicated follow-up: do any of these established systems have an explicit retraction/versioning protocol, or is that left implicit everywhere?

**A verification-honesty note on this file itself**: entries 1, 4, and 5 (NUSAP, Admiralty, Walker) each carry at least one piece of structure that could only be confirmed via secondary-source convergence rather than a clean primary fetch, and entries 7–9 were not independently re-fetched at all in this deepening pass — carried forward from the initial search findings with confidence explicitly marked down. Per this file's own subject matter: those gaps are recorded here rather than smoothed over, because a file about uncertainty-about-uncertainty is a bad place to quietly overclaim verification.
