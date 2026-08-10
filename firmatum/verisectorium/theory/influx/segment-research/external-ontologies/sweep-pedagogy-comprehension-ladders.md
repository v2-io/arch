# Sweep: Pedagogy / comprehension ladders and efficacy-measurement systems

Domain: established academic/professional systems for classifying comprehension depth, skill/expertise state, and — separately — for *measuring whether learning actually happened* (efficacy detection, not just state classification). Written for truth, not for convenience to our current taxonomy.

Confidence key: **VERIFIED** = fetched and read from a primary or authoritative-secondary source this session. **RECALLED-HIGH** = not re-fetched successfully this session (fetch failed/blocked) but high-confidence well-established knowledge, independently cross-checked against multiple secondary descriptions in the search pass. **RECALLED** = lower confidence, should be re-verified before load-bearing use.

---

## 1. Revised Bloom's Taxonomy (Anderson & Krathwohl, 2001)

**Structure (RECALLED-HIGH — primary PDF fetch failed as unparseable binary; cross-checked against multiple independent descriptions, structure is extremely stable across sources so confidence is high despite the failed fetch):**

A genuine **two-axis matrix**, not a single ladder:

- **Cognitive Process Dimension** (the verb — ascending): Remember → Understand → Apply → Analyze → Evaluate → Create. (Original 1956 Bloom had Knowledge → Comprehension → Application → Analysis → Synthesis → Evaluation — note Synthesis and Evaluation swapped order and Synthesis was renamed/promoted to Create at the top.)
- **Knowledge Dimension** (the noun — not strictly ordered as a ladder, but the four types are commonly presented low-to-high concreteness): Factual → Conceptual → Procedural → **Metacognitive**.

Every learning objective is a cell in the resulting 6×4 matrix (e.g. "Analyze + Procedural knowledge"). The Metacognitive Knowledge category is itself subdivided (per Krathwohl's own overview, widely reported): strategic knowledge, knowledge about cognitive tasks (incl. contextual/conditional knowledge), and self-knowledge — i.e. **awareness of one's own knowledge-state and its limits is a named first-class knowledge type**, not an afterthought.

**What it is trying to be true about:** the *type and complexity of a learning objective / assessment item*, for curriculum and assessment design — not a claim about a specific learner's actual state at a moment, though it's widely repurposed that way.

**Uncertainty/gap handling:** none built in as a formal mechanism — it's a classification scheme for objectives, not a measurement instrument with error bars. The Metacognitive category is the closest thing to "knowledge about the limits of one's own knowledge" being represented at all, but only as a *category to classify an objective into*, not as a live confidence value.

**Feature our atom-kinds would care about:** the orthogonality of "what kind of cognitive operation" vs "what kind of knowledge" is the single most directly reusable structural idea in this whole sweep — if any of our ladders are quietly conflating "how deeply is this understood" with "what kind of thing is being understood," Bloom-revised is the citation that those are meant to be separate axes.

**Provenance:** Anderson, L. W., & Krathwohl, D. R. (Eds.). (2001). *A Taxonomy for Learning, Teaching, and Assessing: A Revision of Bloom's Taxonomy of Educational Objectives.* Longman. Krathwohl's own summary paper: Krathwohl, D. R. (2002). "A Revision of Bloom's Taxonomy: An Overview." *Theory Into Practice*, 41(4), 212–218.
- https://cmapspublic2.ihmc.us/rid=1Q2PTM7HL-26LTFBX-9YN8/Krathwohl%202002.pdf (fetched, but returned as corrupted/unparseable binary this session — could not confirm text directly; retry with a different extraction path if this needs to become fully VERIFIED)
- https://en.wikipedia.org/wiki/Bloom%27s_taxonomy (cross-check, not fetched this session but consistent with recalled structure)

---

## 2. SOLO Taxonomy (Biggs & Collis, 1982)

**Structure (RECALLED-HIGH, not re-fetched this session — original search pass already drew from Wikipedia + the original book's ScienceDirect record + a citation-rich practitioner elaboration; internally consistent across all three):**

Single-axis, 5 levels, ascending by **structural complexity of the learner's actual response artifact** (not an ability the learner is presumed to have — SOLO explicitly classifies what was *produced*, not the person):

1. Prestructural — response misses the point / uses irrelevant information.
2. Unistructural — one relevant aspect used.
3. Multistructural — several relevant aspects used, but not integrated with each other.
4. Relational — aspects integrated into a coherent whole/structure.
5. Extended Abstract — the integrated structure is generalized to a new domain/context beyond what was given.

**What it is trying to be true about:** the quality/completeness of a *specific observed response* to a specific task — an evidentiary classification, closer to what our system would call the grain of an Account than a claim about the learner's general competence.

**Uncertainty/gap handling:** none formal. Prestructural is explicitly the "no relevant structure present" / near-null case, which is a clean analog for a "gap" or "no signal" terminal-ish state at the bottom of a ladder — but SOLO doesn't distinguish *why* it's absent (never taught vs. forgotten vs. misunderstood vs. task itself ambiguous).

**Feature our atom-kinds would care about:** it is explicitly a taxonomy of *evidence artifacts*, not of latent learner state — a precedent worth citing if we want an axis that classifies "how much of the structure does this particular Account/response actually show," separate from any claim about underlying truth-status.

**Provenance:** Biggs, J. B., & Collis, K. F. (1982). *Evaluating the Quality of Learning: The SOLO Taxonomy.* Academic Press.
- https://en.wikipedia.org/wiki/Structure_of_observed_learning_outcome
- https://www.sciencedirect.com/book/9780120975525/evaluating-the-quality-of-learning
- https://pamhook.com/solo-taxonomy/ (secondary, practitioner-grade, not independently authoritative but widely cited and consistent)

---

## 3. Dreyfus Model of Skill Acquisition (Dreyfus & Dreyfus, 1980/1986)

**Structure (RECALLED-HIGH, not re-fetched this session):**

Single-axis, 5 stages: Novice → Advanced Beginner → Competent → Proficient → Expert. The ordering dimension is **qualitative mode of reasoning**, not accumulated facts: novices apply context-free rules mechanically; as skill develops, situational discrimination and pattern-recognition increasingly replace explicit rule-following; experts operate via largely tacit, holistic intuition, with rules essentially invisible/dissolved into fluent performance. This is philosophically load-bearing for the Dreyfus brothers — it underlies Hubert Dreyfus's broader critique of rule-based/symbolic AI.

**What it is trying to be true about:** an individual's mode of skilled coping/reasoning in a domain of practice, not their factual knowledge inventory.

**Uncertainty/gap handling:** none formal; it's a phenomenological/ descriptive stage model, not a measurement instrument. Notably it does NOT have a built-in mechanism for representing regression, plateau, or domain- specific unevenness (an "expert" in one subdomain reasoning as a "novice" in an adjacent one) — later commentary flags this as a known limitation.

**Feature our atom-kinds would care about:** the axis itself ("rule-dependence → intuitive holism") is a genuinely different ordering dimension than every comprehension ladder above — worth flagging because it means "expertise ladders" are not one family; at least two distinct axes (complexity-of-content vs. mode-of-reasoning) are both called "expertise" in the literature and conflating them would be a mistake we could import.

**Provenance:** Dreyfus, S. E., & Dreyfus, H. L. (1980). *A Five-Stage Model of the Mental Activities Involved in Directed Skill Acquisition.* USAF Office of Scientific Research report, Univ. of California Berkeley. Later: Dreyfus, H. L., & Dreyfus, S. E. (1986). *Mind Over Machine.* Free Press. Widely applied clinically via Benner, P. (1984). *From Novice to Expert.*
- https://en.wikipedia.org/wiki/Dreyfus_model_of_skill_acquisition
- https://peer.asee.org/dreyfus-five-stage-model-of-adult-skills-acquisition-applied-to-engineering-lifelong-learning.pdf

---

## 4. Kirkpatrick's Four Levels of Training Evaluation (Kirkpatrick, 1959; extended)

**Structure (RECALLED-HIGH, not re-fetched this session):**

Single-axis, 4 levels — but critically, this axis is **not** comprehension depth; it is **distance from the training event / type of evidence required**:

1. Reaction — did participants find it favorable, engaging, relevant?
2. Learning — did knowledge, skill, attitude, or confidence actually change?
3. Behavior — did on-the-job conduct change as a result?
4. Results — did the targeted organizational/business outcome change?

Kirkpatrick Partners' "New World Kirkpatrick Model" (their current, actively maintained extension) adds a "Return on Expectations" framing and formally splits Level 1 into Engagement/Relevance/Customer Satisfaction, and Level 2 into Knowledge/Skill/Attitude/Confidence/Commitment — i.e. the steward institution has itself already found the original 4 levels too coarse and subdivided them, which is itself a data point about ladder longevity under real use.

**What it is trying to be true about:** whether an *intervention* (a training program) caused a change, at increasingly costly-to-measure and increasingly-consequential levels — this is an efficacy-of-intervention ladder, structurally distinct from a comprehension-state ladder.

**Uncertainty/gap handling:** implicitly, via the ladder itself — Level 1 data is cheap but weakly diagnostic of anything beyond satisfaction; Level 4 is the gold standard but confounded by many other causes, and Kirkpatrick Partners' own literature is explicit that most organizations only ever measure Level 1–2 and *assume* the higher levels, which they treat as a known industry failure mode. No formal statistical uncertainty representation.

**Feature our atom-kinds would care about:** distinguishes "was this believed/liked" from "was this true/effective" from "did it change behavior" from "did it change outcomes" — a clean 4-way split of exactly the kind of question our Assertions-vs-Accounts-vs-Practices distinction is reaching for, but built around *interventions* rather than *claims*.

**Provenance:** Kirkpatrick, D. L. (1959, series in *Training Directors Journal*). Steward org actively maintaining/extending it: Kirkpatrick Partners.
- https://www.kirkpatrickpartners.com/the-kirkpatrick-model/
- https://trainingindustry.com/wiki/measurement-and-analytics/the-kirkpatrick-model/

---

## 5. Item Response Theory (IRT) / Latent Trait Theory

**Structure (RECALLED-HIGH, not re-fetched this session, but this is stable, decades-settled psychometric consensus, cross-checked against Columbia Mailman's methods explainer in the original pass):**

Not a discrete ladder — a **continuous latent-trait statistical model**. Core objects:

- **θ (theta)**: a person's position on a continuous latent ability/trait scale.
- **Item parameters**, depending on model complexity: difficulty/location (1-parameter/Rasch model), discrimination (2-parameter model), and a pseudo-guessing floor (3-parameter model).
- These jointly define an **Item Characteristic Curve**: P(correct | θ, item params) — a probabilistic, not deterministic, mapping.
- **Standard error of measurement is itself a modeled, shrinking quantity**: as more items are administered (esp. in computer-adaptive testing), the estimate of θ sharpens and its confidence interval narrows. This is distinct from item-level uncertainty (does this specific item discriminate well, does it fit the model at all — misfit statistics like infit/outfit in Rasch modeling flag items or persons whose response pattern doesn't fit the assumed single-trait structure).
- **Dimensionality is itself an assumption that can fail**: IRT models typically assume unidimensionality (one underlying trait); when the data show multiple latent traits are actually being measured, standard IRT model fit degrades in a diagnosable way (this is the "uncertainty about whether we're even measuring one thing" layer).

**What it is trying to be true about:** a person's position on a latent, unobservable trait (ability, attitude, whatever the instrument is designed to target), inferred probabilistically from a pattern of observed item-responses.

**Uncertainty/gap handling:** this is IRT's central strength relative to everything else in this sweep — it natively carries (a) a point estimate, (b) a shrinking-with-evidence standard error around that estimate (uncertainty about the value), and (c) model-fit/misfit diagnostics that can flag when the very assumption of a single coherent trait is breaking down (uncertainty about whether the measurement itself is well-formed) — i.e. it structurally separates "how sure are we of the number" from "is the number even meaningful here," which is close to a literal implementation of "uncertainty about uncertainty."

**Feature our atom-kinds would care about:** the clean three-layer separation — estimate / confidence-in-estimate / validity-of-the-measurement- model-itself — is probably the single most reusable formal idea in this sweep for how an Assertion's or Account's confidence value could be represented with a shrinking error term rather than a single scalar or a discrete rung.

**Provenance:** Field consensus, no single steward; foundational figures Georg Rasch (1960, the 1-parameter model), Frederic Lord & Melvin Novick (1968, statistical theory), Allan Birnbaum (2/3-parameter logistic models). Standard framework behind GRE, NAEP, PISA computer-adaptive testing.
- https://en.wikipedia.org/wiki/Item_response_theory
- https://www.publichealth.columbia.edu/research/population-health-methods/item-response-theory

---

## 6. Force Concept Inventory (FCI, 1992) + Hake Normalized Gain (1998)

**Structure — Hake's normalized-gain formula (VERIFIED this session via WebSearch, formula and paper title confirmed from multiple independent sources):**

g = (post% − pre%) / (100 − pre%)

i.e. **actual gain achieved, divided by the maximum possible gain available given the starting point** — explicitly normalizing for prior state so courses/cohorts with different starting knowledge can be compared on a common scale. Hake's own 1998 six-thousand-student survey classified courses into bands by this measure ("low-g," "medium-g," "high-g" — roughly <0.3, 0.3–0.7, >0.7 in the commonly cited convention) contrasting traditional lecture against "interactive engagement" pedagogy, and found IE courses clustered at much higher normalized gain — this became one of the most-cited findings in physics-education research and a major empirical argument for active-learning pedagogy generally.

**FCI structure itself:** a validated multiple-choice concept-diagnostic test where **distractors (wrong answers) are keyed to specific, empirically documented student misconceptions** about Newtonian mechanics — not arbitrary wrong answers. A wrong answer is diagnostic evidence of *which* misconception is held, not just an absence-of-knowledge signal.

**What it is trying to be true about:** whether conceptual understanding (specifically, correct vs. commonsense/Aristotelian misconceptions about force and motion) actually shifted between two measurement points, and how much of the available room for improvement was captured.

**Uncertainty/gap handling:** the diagnostic-distractor design means a "wrong" answer already carries structured information about the *kind* of gap, not just its presence — closer to our system's interest in "characterizing the gap," not merely flagging it exists. Note also (RECALLED from search pass, not independently re-verified): normalized gain itself has been statistically contested in the PER literature in later years (ratio-of-differences measures have known statistical pathologies — e.g. the 2024/2025 arxiv papers found in the search defending or critiquing continued use) — i.e. **this is a battle-tested-but-actively-disputed metric**, a good specimen of an established measurement tool whose own field keeps re-litigating its validity rather than treating it as settled.

**Feature our atom-kinds would care about:** the paradigm case in this whole sweep of an actual **efficacy-detection instrument** rather than a self-report or expert-rated ladder — measured before/after, normalized for starting point, with diagnostic (not just binary) wrong-answer structure. Directly reusable as a design pattern if our system ever wants to represent "how much did this atom's assertion actually move belief/practice, given how far it could have moved."

**Provenance:**
- Hestenes, D., Wells, M., & Swackhamer, G. (1992). "Force Concept Inventory." *The Physics Teacher*, 30(3), 141–158.
- Hake, R. R. (1998). "Interactive-engagement versus traditional methods: A six-thousand-student survey of mechanics test data for introductory physics courses." *American Journal of Physics*, 66(1), 64–74. (VERIFIED title/formula via WebSearch this session.)
- https://link.aps.org/doi/10.1103/PhysRevSTPER.3.010106
- https://doi.org/10.1103/PhysRevPhysEducRes.16.010108 (later methodological defense — shows the metric is still actively contested)
- https://arxiv.org/html/2407.07730v1 (2024/2025-era continued defense of normalized gain, found via WebSearch this session)

---

## 7. Webb's Depth of Knowledge (DOK)

**Structure (RECALLED-HIGH, not re-fetched this session):**

Single-axis, 4 levels, explicitly framed by its own literature as **orthogonal to Bloom**, not a restatement of it:

1. Recall & Reproduction
2. Skills & Concepts (basic reasoning, use of information)
3. Strategic Thinking / Reasoning (planning, using evidence, non-routine problem-solving)
4. Extended Thinking (sustained investigation, synthesis across an extended task/time period)

**What it is trying to be true about:** the **cognitive complexity/rigor demanded by a task**, not the cognitive-operation-type of a task (Bloom) and not a claim about the learner. The oft-cited illustrative point (present in several DOK-vs-Bloom crosswalk documents found in the original search pass): a "Remember"-verb task (Bloom) can still be DOK-3 if it requires strategic assembly and application of many recalled facts under non-routine conditions — i.e. verb-type and rigor-of-engagement are genuinely separable, and conflating them is a documented, named error in that literature.

**Uncertainty/gap handling:** none formal — purely a task/item classification scheme for standardized-assessment design, used to ensure item banks aren't all shallow-recall even when they use higher-Bloom verbs.

**Feature our atom-kinds would care about:** DOK-vs-Bloom is the cleanest documented case in this sweep of "two established ladders that sound similar and are routinely confused, but measure genuinely orthogonal properties" — directly useful as a cautionary precedent for auditing whether any two of our own axes are secretly doing this.

**Provenance:** Webb, N. L. (1997/1999-era technical reports, Wisconsin Center for Education Research); adopted into Common Core-era state assessment design across the US.
- https://community.ksde.gov/LinkClick.aspx?fileticket=ZIu_UDfYtX4%3D
- https://www.floridaipdae.org/dfiles/resources/webinars/050918/Webinar_050918_Handout_DOK_Blooms_Taxonomy.pdf

---

## 8. Miller's Pyramid of Clinical Competence (Miller, 1990)

**Structure (VERIFIED this session via WebFetch of mededmentor.org, which cites the primary Miller 1990 *Academic Medicine* paper directly):**

Single-axis, 4-level pyramid:

1. **Knows** — foundational knowledge base; assessed by objective/written tests.
2. **Knows How** — competence: ability to apply knowledge (analysis, synthesis, data-gathering, forming a management plan); assessed via applied/data-interpretation exercises.
3. **Shows How** — performance: demonstrated in simulated or controlled clinical scenarios (e.g. OSCEs); learner behavior is *observed*, not self-reported or tested by written exam.
4. **Does** — action: independent, unobserved real-world clinical practice; assessed via workplace-based methods (e.g. mini-CEX, direct observation of procedural skills), not by any form of test.

Later extensions beyond Miller's original four (VERIFIED as noted by the same source, attributed but not part of Miller's original paper): Cruess et al. (2016) added **"Is"** at the apex — professional identity, i.e. not just doing the right thing but *being* the kind of practitioner who does it as identity, not performance. Ten Cate et al. (2021) proposed **"Trusted"** — entrustment, i.e. whether supervisors are willing to grant unsupervised responsibility, an explicitly social/relational credential layered on top of demonstrated competence.

**What it is trying to be true about:** whether a clinician's competence, as established across increasingly-hard-to-fake assessment methods, actually translates into safe independent real-world practice. Miller's original 1990 motivation (explicit in the source, and well-corroborated in medical- education literature broadly): **there is documented poor correlation between what people know and what they actually do** — the pyramid exists specifically because knowledge tests were found to be a weak proxy for behavior.

**Uncertainty/gap handling:** none formal as a measurement mechanism, but structurally significant: it names, as its entire reason for existing, that "comprehension" and "action" are NOT the same claim and testing one does not license concluding the other — the gap itself is the pyramid's founding observation, not a defect to patch.

**Feature our atom-kinds would care about:** the cleanest, most compact precedent in this whole sweep for splitting "knows something is true" (Assertion-adjacent) from "has actually enacted/verified it in practice" (Practice/Account-adjacent), with an explicit, evidence-backed claim that these do NOT reduce to one ladder. Also worth noting for our system: the apex is inherently the **least directly measurable** level (real, unobserved practice) — the pyramid gets *harder to verify* exactly as it gets more consequential, mirroring Kirkpatrick's Level 4 problem above. Two independent traditions (medical education, corporate training) converged on the same shape: assessment difficulty rises with the very thing you most want to know.

**Provenance:** Miller, G. E. (1990). "The assessment of clinical skills/competence/performance." *Academic Medicine*, 65(9 Suppl), S63–S67. DOI: 10.1097/00001888-199009000-00045.
- https://mededmentor.org/theory-database/theory-index/millers-pyramid-of-skill-development/ (fetched and read this session; cites Miller 1990 directly with DOI)
- https://www.stemlynsblog.org/better-learning/educational-theories-you-must-know-st-emlyns/educational-theories-you-must-know-millers-pyramid-st-emlyns/ (secondary, consistent, not independently re-fetched this session)

---

## 9. CEFR (Common European Framework of Reference for Languages)

**Structure (VERIFIED this session via WebFetch of efset.org, a secondary but detailed and widely-used explainer; the primary Council of Europe URL returned HTTP 403 this session and could not be directly fetched — see note below):**

6 levels grouped into 3 bands:

- Basic User: A1 (Beginner), A2 (Elementary)
- Independent User: B1 (Intermediate), B2 (Upper Intermediate)
- Proficient User: C1 (Advanced), C2 (Proficient/Mastery)

Levels are defined via **"can-do" behavioral descriptors** rather than content-coverage checklists — e.g. the B1 descriptor: "Can produce simple connected text on topics that are familiar or of personal interest." This is a genuinely different design axis from every ladder above except SOLO: competence is described as demonstrated *functional capability in real tasks*, not knowledge inventory or cognitive-operation type.

**What it is trying to be true about:** a language learner's functional communicative capability, structured explicitly so that many independent national/commercial tests (IELTS, TOEFL, DELF/DALF, Goethe-Zertifikat, etc.) can each map their own scores onto a common, cross-institution scale — this is the rare example in this sweep of a ladder functioning as an **interoperability standard among otherwise-independent measurement systems**, governed by a formal international body (Council of Europe), not one program's house rubric.

**Uncertainty/gap handling — this is the most important finding of this entry and the source directly names it as a limitation, not something we inferred:** per the efset.org page (verified this session), "each of the six levels comprises a wide range of skills and abilities," to the point that "a student who has just reached the B1 level is quite a long way behind a student who has almost, but not quite mastered all of the B2 skills, but both students would be defined as being in the B1 level." The source states plainly that **the CEFR itself does not formally differentiate partial mastery within a level, or uneven competence across the four skills (reading/writing/listening/speaking)** — in practice, test-providers and educators patch this by adding sub-levels (e.g. B1+, B2.1/B2.2) informally, outside the formal CEFR structure itself. This is a genuine, acknowledged, still-unresolved coarseness in one of the most institutionally authoritative ladders in this entire sweep.

**Feature our atom-kinds would care about:** two things. First, the interoperability-standard design (external bodies calibrate their own independent instruments against CEFR's descriptors) is the strongest precedent here for a ladder meant to be a citable cross-system reference rather than an internal house convention. Second — and this cuts against treating CEFR as an unqualified model to imitate — its own acknowledged coarseness (bucket-boundary effects, no native handling of skill-unevenness) is a cautionary data point: even the most battle-tested, internationally governed ladder in this sweep has an openly conceded gap between its formal granularity and the reality it's classifying.

**Provenance:** Council of Europe (steward institution). *Common European Framework of Reference for Languages: Learning, Teaching, Assessment* (2001), extended by the CEFR Companion Volume (2018/2020, adding mediation and plurilingual/pluricultural descriptors — not independently verified this session, RECALLED from original search pass).
- https://www.coe.int/en/web/common-european-framework-reference-languages/level-descriptions (primary steward source — **returned HTTP 403 this session, could not be fetched directly**; treat CEFR level-descriptor wording above as sourced from the efset.org secondary, not independently cross-checked against the Council of Europe primary text this session)
- https://www.efset.org/cefr/ (fetched and read this session; secondary but detailed, widely used, internally consistent)

---

## 10. Gagné's Nine Events of Instruction / Merrill's First Principles of Instruction

**Structure (RECALLED, not re-fetched this session):**

Not comprehension ladders — **prescribed sequences of pedagogical moves**.

Gagné (1965, *Conditions of Learning*), 9 events in order: gain attention → inform learners of objectives → stimulate recall of prior learning → present the content → provide learning guidance → elicit performance (practice) → provide feedback → assess performance → enhance retention and transfer.

Merrill (2002), "First Principles of Instruction" — a later synthesis paper explicitly built by comparing five major instructional-design theories (including Gagné's) and extracting principles common across all of them. Structure: Activation (of prior experience) → Demonstration (show, don't just tell) → Application (learner does it, with decreasing guidance) → Integration (learner reflects on / publicly applies the new knowledge in their own life/world), organized around a central task/problem rather than abstract content.

**What it is trying to be true about:** the *order* in which pedagogical moves should occur for effective learning to occur — a causal/procedural claim, not a state-classification.

**Uncertainty/gap handling:** none — these are prescriptive design frameworks, not measurement or classification instruments.

**Feature our atom-kinds would care about:** relevant specifically to Exposition & Pedagogy atoms as models for *sequencing* an atom's supporting context for a reader/learner, distinct from every other entry in this file which classifies a *state*. Also methodologically interesting: Merrill's paper is itself an example of establishing a claim by explicit convergence across multiple independently-developed prior systems — a research pattern (triangulating across established frameworks rather than inventing from scratch) that mirrors what this whole sweep-of-sweeps effort is doing.

**Provenance:** Gagné, R. M. (1965). *The Conditions of Learning.* Holt,  
Rinehart & Winston. Merrill, M. D. (2002). "First Principles of Instruction." *Educational Technology Research and Development*, 50(3), 43–59.
- https://instructionaldesign.io/toolkit/merrill
- https://www.structural-learning.com/post/gagnes-nine-events-of-instruction-teachers-guide

---

## 11. Deliberate Practice (Ericsson, Krampe & Tesch-Römer, 1993) — and its own later downgrade

**Structure (RECALLED, not re-fetched this session):**

Not a ladder — a causal theory: expertise is acquired through specially designed, effortful practice at the edge of current ability, with rapid feedback, sustained over large accumulated time (the "10,000 hours" popular simplification derives loosely from this, though Ericsson himself repeatedly pushed back on that oversimplification in later writing).

**What it is trying to be true about:** the causal mechanism by which expert-level performance is acquired.

**Uncertainty/gap handling / epistemic status — this is the most valuable part of this entry:** the theory is **contested and partially walked back by its own field**. A 2019 meta-analytic replication/critique (royalsocietypublishing.org, found in original search pass, not re-fetched this session) found that deliberate-practice hours do not sufficiently explain individual differences in expertise in chess and music specifically — other factors (starting age, innate variation, quality-not-just-quantity of practice) carry substantial unexplained variance the original theory underclaimed. This is a genuine, citable specimen of **an established, heavily-cited claim later formally downgraded by follow-up work within its own discipline, with the downgrade published and traceable** — rather than either (a) silently forgotten/superseded with no trace, or (b) defended indefinitely as settled.

**Feature our atom-kinds would care about:** directly useful as a worked example for how our epistemic-state system might represent a claim whose confidence has been *formally revised downward over time by cited follow-up work*, as distinct from a claim that was simply always understood as uncertain, or one that's been outright refuted. The steward's framing ("truth as ultimate adjudication... how do they explain their gaps") is almost literally what this citation-trail is doing structurally.

**Provenance:** Ericsson, K. A., Krampe, R. T., & Tesch-Römer, C. (1993). "The role of deliberate practice in the acquisition of expert performance." *Psychological Review*, 100(3), 363–406.
- https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7461852/
- https://royalsocietypublishing.org/doi/10.1098/rsos.190327 (2019 replication/critique)

---

## What surprised me / what we did not ask about

- **The two clearest wins were not comprehension ladders at all.** Miller's Pyramid and IRT are the two entries with the highest verification confidence *and* the highest apparent direct usefulness to verisectorium's actual problem — and neither is a "how deep do you understand X" ladder. Miller's Pyramid is a *knowledge-vs-behavior split with an explicit evidence-based justification for why they don't reduce to each other*; IRT is a *formal three-layer uncertainty representation* (estimate / error-on-estimate / validity-of-the-measurement-model). If the original brief had been read narrowly as "find comprehension ladders," both of these would have been under-weighted relative to their actual value.

- **Every system with a genuine two-axis or split design beat every single-axis ladder on usefulness**, without exception, across this whole sweep: revised-Bloom (process × knowledge-type), Miller (knows × does), Kirkpatrick (belief/liking × behavior × outcome), IRT (estimate × confidence × model-validity), DOK-vs-Bloom (verb-type × rigor). The single-axis ladders (SOLO, Dreyfus, DOK alone, CEFR alone) are each individually well-established and citable, but structurally thinner. This suggests the steward's instinct to look past organic single-ladders toward established systems was well-founded specifically because established systems keep re-discovering that a second axis was hiding inside what looked like one dimension — worth treating as a standing hypothesis to test against our own current TAXONOMY-FILL axes rather than a coincidence of this sample.

- **CEFR's own conceded coarseness was an inconvenient finding worth flagging plainly** (per the steward's "record what is actually there... including what is inconvenient" instruction): the single most institutionally authoritative, internationally governed ladder in this entire sweep openly admits it cannot represent partial mastery within a level or unevenness across sub-skills. If we're tempted to reach for CEFR as a gold-standard precedent for a citable external-interoperability ladder, its own stewards' literature already documents the exact failure mode ("bucket-boundary students look identical to just-barely students") that a grain-sensitive system like ours is trying to avoid. That's a genuine caution, not just a citation.

- **Deliberate Practice is the one entry that models "a claim revised downward over time by its own field, with citations"** — none of the other ten systems in this sweep have a documented instance of *themselves* being contested or downgraded (they're mostly presented, including by me, in their settled/canonical form). That asymmetry is worth noticing: most of what got surfaced is "here is an established ladder," when the steward's actual ask was equally about "how do systems represent their own gaps and uncertainty" — and only Deliberate Practice and (partially) the normalized-gain metric (contested in later PER literature) actually demonstrate a field auditing and revising its own prior confident claim in public, with a citable trail. If depth on "how established systems represent walked-back confidence" is wanted, this sweep under-delivers on that specific sub-question and would benefit from a dedicated follow-up pass (e.g. looking at how psychiatric diagnostic categories, or software CVE severity ratings, or replication-crisis-era psychology findings, get formally downgraded/retracted/re-classified over time — none of which were searched here).

- **We did not look at software/engineering maturity or readiness ladders** at all (TRL — Technology Readiness Levels; CMMI; the OWASP/NIST severity and confidence-rating schemes) despite these being arguably closer in *domain* to a knowledge-management system like verisectorium than most of what's here — that's a genuine gap in this sweep's coverage, not a judgment that they're less relevant; they may belong to a different sweep-agent's assigned domain and are flagged here in case that's not already covered elsewhere.

- **Two failed fetches are worth being honest about rather than silently papering over**: the Krathwohl 2002 PDF (primary source for revised Bloom's) returned as unparseable binary, and the Council of Europe's own CEFR page returned HTTP 403. Both entries above are therefore built on RECALLED-HIGH or secondary-verified confidence rather than full primary verification, and are flagged as such inline rather than presented with false certainty — this itself is arguably the most on-theme finding for a project about honestly representing gaps: even a deliberately careful verification pass has real, disclosed holes in its own coverage.
