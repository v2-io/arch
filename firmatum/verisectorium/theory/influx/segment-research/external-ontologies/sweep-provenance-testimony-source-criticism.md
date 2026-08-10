# Sweep: Provenance, Testimony & Source Criticism

Domain: systems for grading *where a claim/document came from*, *whether the chain to it is intact*, and *how much we should trust it* — across W3C data provenance, life-science nanopublications, historiography, textual criticism, analytic epistemology of testimony, journalism, forensic/legal chain of custody, evidence-based-medicine certainty grading, hadith science, and archival science.

Confidence key per entry: **[verified]** = fetched the primary/near-primary source this session and structure below is drawn from it; **[verified via secondary]** = fetched a solid secondary (e.g. Wikipedia) that itself quotes or closely paraphrases the primary, structure looks internally consistent but I did not touch the primary text; **[recalled]** = not independently re-fetched this session, carried from the assigned search-findings pass — flagged for the coordinator to verify before load-bearing use.

---

## 1. W3C PROV (PROV-DM / PROV-O / PROV-N / PROV-CONSTRAINTS)

**(a) Structure, as the source states it.**  
Three core classes, fetched directly from PROV-O [verified]:

- **`prov:Entity`** — "a physical, digital, conceptual, or other kind of thing with some fixed aspects; entities may be real or imaginary."
- **`prov:Activity`** — "something that occurs over a period of time and acts upon or with entities; it may include consuming, processing, transforming, modifying, relocating, using, or generating entities."
- **`prov:Agent`** — "something that bears some form of responsibility for an activity taking place, for the existence of an entity, or for another agent's activity."

Core "starting point" relations (the minimal vocabulary the spec itself foregrounds): `wasGeneratedBy`, `used`, `wasAssociatedWith`, `wasDerivedFrom`, `wasAttributedTo`, `wasInformedBy`, `actedOnBehalfOf`, `startedAtTime`, `endedAtTime`. Beyond the core triad, PROV-DM's overview [verified] names six aspects total: (1) entities/activities/time, (2) derivation, (3) agents and responsibility, (4) **bundles** — provenance-of- provenance, i.e. nested meta-provenance ("the provenance of provenance"), addressed via a dedicated **PROV-LINKS** note for cross-bundle linking, (5) alternate/specialization ("same underlying thing, different aspects"), (6) collections.

Validity is handled not by scoring but by **PROV-CONSTRAINTS** — a separate document of formal logical rules a provenance graph must satisfy to be well-formed, checked by validators. This is a soundness/well-formedness notion, not a confidence notion.

**(b) What it's trying to be true about.** Not "is this claim true" but "what process produced this thing, and who is answerable for it" — a purely structural/causal-historical account: what happened, in what order, who did it, what it came from.

**(c) Uncertainty/gap handling.** **[verified]** Explicitly absent. I searched PROV-O directly for confidence/certainty/trust vocabulary: "No built-in properties for confidence, certainty, or degree of trust are documented. The specification contains no mention of confidence levels, trust scores, or certainty measures." This is a *deliberate scope exclusion* by a mature, W3C-ratified, decade-old standard used across science/government/data-lineage tooling — it treats "how sure are we" as strictly out of scope, to be layered on top by something else. That's a strong negative data point: the most established general-purpose provenance ontology in existence does not conflate provenance with trust-scoring, and doesn't even gesture at how the two should compose.

**(d) Feature our atom-kinds would care about.** The **bundle** mechanism (provenance-of-provenance) is the notable structural feature: PROV anticipates that provenance metadata is itself a first-class thing that can have its *own* provenance, recursively. If our atoms ever need "who asserted this atom's Evidence rating, and when, and on what basis" as a trackable fact in its own right, PROV already has a slot-shape for that (a bundle is itself an Entity). Also notable: PROV's Entity/Activity/Agent triad is an *event-plus-responsibility* model, not a status-ladder model — closer in spirit to our "status cells are projections of events" framing than to any of the grading ladders below.

**(e) Provenance.** W3C Provenance Working Group, 2011–2013, Recommendation status (final, standards-track, not draft). De facto interoperability standard for machine-readable data lineage. URLs: https://www.w3.org/TR/prov-dm/ · https://www.w3.org/TR/prov-o/ [fetched directly] · https://www.w3.org/TR/prov-overview/ [fetched directly] · https://www.w3.org/TR/prov-n/ · https://www.w3.org/TR/prov-constraints/ Confidence: **[verified]** core classes/properties and the confidence-gap claim, via direct fetch of PROV-O and PROV-Overview.

---

## 2. Nanopublications

**(a) Structure, as the source states it.** **[verified]**, fetched the Working Draft guidelines directly. A nanopublication is **four** named RDF graphs (the assigned search findings said three plus a head; the primary source itself names four, with the head being the fourth, not a wrapper outside the count):

- **Assertion graph** — "the main claim of the nanopublication" (e.g. drug X treats disease Y).
- **Provenance graph** — "one or more RDF triples that provide information about the assertion" — how it was derived, by what method, from what.
- **Publication info graph** — metadata about the *publishing act itself* (who published, when) — distinct from the provenance of the *assertion*.
- **Head graph** — links the nanopublication's own URI to the other three via `np:hasAssertion`, `np:hasProvenance`, `np:hasPublicationInfo`.

**(b) What it's trying to be true about.** One atomic, independently citable scientific claim per nanopublication — designed explicitly so a single fact (not a whole paper) can be the unit of publication, citation, and machine aggregation across knowledge graphs (heavy uptake in life-science / bioinformatics).

**(c) Uncertainty/gap handling.** **[verified]** — the guidelines do *not* define an uncertainty/confidence field within the format. Trust is handled indirectly: the provenance graph documents source/derivation so a consumer can *assess* trustworthiness themselves, and **Trusty URIs** give cryptographic tamper-evidence (immutability, not confidence) — i.e. the format guarantees "this is unaltered since publication," not "this is correct." Notably, this is the *same* gap PROV has: a mature, purpose-built atomic-claim format still leaves confidence-scoring out of its core schema.

**(d) Feature our atom-kinds would care about.** The **three-graph content/ derivation/publication-event split** is the closest existing precedent for exactly the shape our want-mode taxonomy reaches for: an atom's *payload* (assertion), its *evidence/derivation trail* (provenance), and its *publication-event metadata* (who/when, independent of whether the claim is true) kept as genuinely separate graphs rather than folded into one document. Also worth flagging: the format is deliberately *atomic* — one claim per unit — which is a direct structural analog to our own atom concept, and it emerged from the same pressure (needing a citable, independently-verifiable minimal unit) rather than by design analogy.

**(e) Provenance.** nanopub.org community effort (Groth, Mons and others), active standard with an ongoing Working Draft process; adopted widely in bioinformatics/life-science linked-data projects, not a top-tier standards body (not W3C) but a real, maintained, cited spec with a defined guidelines document.  
URLs: https://nanopub.net/guidelines/working_draft/ [fetched directly] · http://www.nanopub.org/2013/WD-guidelines-20131215/ (earlier version) · https://arxiv.org/pdf/1809.06532 (survey, secondary) · https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7959622/ (secondary) Confidence: **[verified]** structure and uncertainty-gap claim, direct fetch.

---

## 3. Historiographical Source Criticism (external / internal criticism)

**(a) Structure.** **[recalled, not re-fetched this session — see note]**  
Two orthogonal axes, not one ladder:

- **Source-distance**: primary / secondary / tertiary — proximity to the event described.
- **Criticism-type**, crossed against distance:
  - *External criticism* — is the document what it claims to be? (date, authorship, physical/chain-of-custody facts, authenticity).
  - *Internal criticism* — once authenticity is granted, is the *content* credible? (bias, competence of the author, internal consistency, corroboration by other sources).

**(b) What it's trying to be true about.** Historical documents/testimony as evidence for what actually happened — a two-step gate: first "is this real," then "is what it says believable."

**(c) Uncertainty/gap handling.** Not formalized as a scale; the discipline handles gaps through corroboration-counting (multiple independent sources agreeing raises confidence) and explicit historiographical argument in prose rather than a numeric or named-tier system — there is no canonical "grade" a source receives, only an argued judgment.

**(d) Feature our atom-kinds would care about.** The external/internal split is a genuinely load-bearing structural precedent for separating "is this record what it claims to be" (closer to our Kind/Authority/provenance axis) from "is what it asserts true" (closer to our Evidence axis) — and it's centuries-old, discipline-wide method, not one institution's house style, which gives it more weight as a convergent finding than a single professional body's standard.

**(e) Provenance.** 19th-century German historical-critical method (Ranke; formalized by Bernheim's *Lehrbuch der historischen Methode* and Langlois & Seignobos's *Introduction aux études historiques*), now the baseline methodology taught in historical-methods curricula worldwide. **Important caveat, stated plainly: I did not re-fetch a primary or even a strong secondary source for this entry this session** — the only URL surfaced in the original pass was a blog-grade restatement (https://historiographyandmethod.blogspot.com/2020/05/external-criticism.html), and I did not locate or fetch Bernheim or Langlois & Seignobos (neither is readily web-fetchable). This entry should be treated as **[recalled]**, weaker-confidence than the others, until someone verifies it against an actual historiography-methods textbook or the primaries. The two-axis shape is very likely right (it's genuinely standard vocabulary in the field) but I have not personally checked it against a citable primary this pass.

---

## 4. Textual Criticism / Stemmatics

**(a) Structure.** **[recalled]** A witness tree (*stemma codicum*) is reconstructed from surviving copies by clustering on **shared errors** ("community of error implies community of origin" — shared correct readings prove nothing, since anyone could independently get it right; shared *mistakes* are the diagnostic signal), converging toward a reconstructed archetype or hyparchetypes. The adjudication rule for choosing between competing variant readings is **lectio difficilior potior** — "the more difficult reading is the stronger [more likely original] one" — because scribes tend to *smooth over* difficulty (replacing an odd/hard word with an easier one), so a surviving hard reading is evidence of *not* having been smoothed, i.e. of being closer to the original.

**(b) What it's trying to be true about.** Reconstructing a single lost original text from multiple divergent, imperfect copies — a genuinely different problem from source criticism (which grades a single document's trustworthiness): this is about *convergent reconstruction across witnesses*, with an explicit method for weighing readings against each other.

**(c) Uncertainty/gap handling.** Critical editions display uncertainty directly to readers via a **critical apparatus** — footnotes/sigla showing which manuscripts support which reading at each point of variance, so the editor's judgment call is fully auditable rather than hidden inside a single "final" text. This is a genuinely strong precedent for "surface the disagreement, don't just resolve it silently."

**(d) Feature our atom-kinds would care about.** *lectio difficilior potior* is a sharp, counterintuitive, well-battle-tested heuristic that **inverts naive plausibility-weighting** — the more surprising/awkward version is treated as more likely original, precisely because smoothing is the expected corruption direction. Worth holding as a caution against any temptation to let "more plausible-sounding" atoms silently outrank "more awkward but earlier-attested" ones. The critical-apparatus habit (show competing readings + support, don't collapse to one silently) is also a direct precedent for surfacing disagreement rather than hiding it behind a single resolved value.

**(e) Provenance.** Karl Lachmann's method (19th c.), refined by later scholars (Paul Maas, Giorgio Pasquali critiquing/extending it); standard in classics, biblical textual criticism, medieval studies. **Not re-verified against a primary this session** — carried from the assigned pass, general Wikipedia-level confidence: https://en.wikipedia.org/wiki/Stemma · https://en.wikipedia.org/wiki/Lectio_difficilior_potior. Confidence: **[recalled]**.

---

## 5. Epistemology of Testimony (Coady et al.)

**(a) Structure.** **[recalled]** Not a ladder — a foundational-warrant dichotomy:

- **Reductionism**: testimonial belief is only justified insofar as it can be *reduced* to the hearer's own perception, memory, or inductive track-record of that testifier's (or testifiers-in-general's) past reliability. Heavy epistemic burden sits on the receiver to independently ground trust.
- **Anti-reductionism** (Coady's own position, following a Reidian line): testimony is a *basic*, non-derived source of warrant — on par with perception and memory — such that a hearer is entitled to believe testimony by default, absent specific defeating reasons. Light default burden on the receiver.

**(b) What it's trying to be true about.** Not any specific claim's truth-status, but the *prior philosophical question* of what licenses believing anyone else's say-so at all — the justificatory floor underneath every other system in this sweep.

**(c) Uncertainty/gap handling.** This is the one entry in the sweep that is itself entirely about how much epistemic weight testimony can bear in principle, rather than a practical grading scheme — the "gap handling" IS the subject matter (defeaters, undercutting vs rebutting evidence, degrees of warrant transfer).

**(d) Feature our atom-kinds would care about.** This is the deepest philosophical fork bearing directly on a design choice our theory makes implicitly: does an atom's evidential status ultimately have to *reduce* to re-derivable primaries (perception/measurement/observation), or can testimony/attestation itself be treated as a first-class warrant floor with its own default-trust? Directly relevant to whatever "Witness Position" or attestation axis Accounts carry — and it's a genuinely unresolved, still-debated question in the field, not settled consensus, so our own choice here is a real design decision, not a lookup.

**(e) Provenance.** C.A.J. Coady, *Testimony: A Philosophical Study* (1992, Oxford) — the modern founding text of this sub-literature; substantial ongoing analytic-epistemology discussion since. **Not re-fetched this session** — carried from the assigned pass: https://plato.stanford.edu/entries/testimony-episprob/ (SEP, canonical tertiary source for the field, generally reliable) · https://iep.utm.edu/ep-testi/ (secondary). Confidence: **[recalled]**, but SEP is a genuinely high-trust tertiary source even unverified this session.

---

## 6. Journalism Sourcing Standards (attribution ladder + two-source rule)

**(a) Structure.** **[verified via secondary]** — fetched the Wikipedia "sources in journalism" article, which itself quotes/cites named journalism-standards bodies (AP, Poynter). Note: my attempt to fetch AP's own Statement of News Values & Principles directly failed (`ap.org` is unreachable from this tool), so this is secondary, not primary, confirmation — but the secondary is solid and internally cites the primary terms. Terms found:

- **On the record** — "All that is said can be quoted and attributed" by name.
- **On background** — substance may be reported, general source characterization allowed, but no direct quotations.
- **Deep background** — U.S.-specific term; information may be used to inform/guide reporting but, in some usages, may not appear in the article at all, or may appear unattributed to any source.
- **Off the record** — "provided to inform a decision or provide a confidential explanation, not for publication" — per Poynter, "the ethical thing to do is not report or even repeat that information."
- **Not for attribution** — distinct fifth category found here beyond the four in the original findings: comments may be quoted directly but the source identified only generically (e.g. "a government insider").
- **Unattributable** — reportable, but without identifying the source at all.

So the *canonical* ladder as documented is actually closer to **five/six named terms**, not the four the assigned findings listed (On the Record → On Background → Deep Background → Off the Record) — worth correcting: "Not for attribution" and "Unattributable" are separate, commonly-used categories with distinct licenses, sitting between On the Record and On Background.

**Two-source rule**: confirmed as real practice — "news organizations may impose safeguards, such as requiring that information from an anonymous source be corroborated by a second source before it can be printed" — but phrased in the secondary as a common newsroom safeguard, not a universal AP-mandated rule; I could not verify AP's own exact wording (site unreachable), so the "named exception for a single sufficiently-authoritative source" claim from the original findings is **[recalled, unverified]** — plausible newsroom practice but not confirmed against AP's own text this session.

**(b) What it's trying to be true about.** Two genuinely separate questions bundled under "sourcing": (i) how may this source be described/ quoted in print (a *usage-license* axis), and (ii) how many independent sources corroborate the fact before it's fit to print (a *corroboration- count* gate on the *evidence* axis).

**(c) Uncertainty/gap handling.** The two-source rule is exactly a minimum-corroboration gate — a fact isn't published until independently corroborated, i.e. gaps in corroboration are handled by *withholding publication*, not by publishing with a hedge.

**(d) Feature our atom-kinds would care about.** The clean separation between the attribution ladder (usage license — what may be said about the source) and the corroboration count (evidence strength) is the strongest real-world precedent that these two concerns must be tracked *independently*: a fact can be On Background (restrictive usage license) and extremely well-corroborated, or On the Record (fully open usage) and single-sourced and shaky. Direct validation of keeping Usage License and Evidence as separate axes rather than one ladder — professional practice apparently learned this the hard way (conflating the two produces bad journalism).

**(e) Provenance.** AP Stylebook / AP Statement of News Values & Principles is the closest thing to a canonical institutional standard here, adopted with local variance by most major US newsrooms — but I was **unable to fetch it directly this session** (tool access blocked to ap.org). Everything above is verified only against a secondary (Wikipedia), which is itself reasonably careful and cites Poynter directly. URLs: https://en.wikipedia.org/wiki/Source_(journalism) [fetched] · https://www.ap.org/about/news-values-and-principles/ [fetch attempted, failed — flag for coordinator/Joseph to pull directly if load-bearing]. Confidence: **[verified via secondary]**, primary still unconfirmed.

---

## 7. Chain of Custody (forensic / legal)

**(a) Structure.** **[verified]** — fetched the NIST CSRC glossary entry directly. NIST's definition (appearing identically in both NIST SP 800-72 and NIST SP 800-101 Rev. 1): "A process that tracks the movement of evidence through its collection, safeguarding, and analysis lifecycle by documenting each person who handled the evidence, the date/time it was collected or transferred, and the purpose for the transfer."

This is **not a strength ladder** — it's a continuity/audit-trail requirement over a sequence of custody-transfer events (who / when / why, at every handoff). NIST's own definition doesn't state consequences for a break — that's legal-procedure territory, not NIST's, and the assigned findings' claim about exclusion-vs-reduced-weight is **[recalled, unverified this session]**: plausible and consistent with general legal knowledge (a broken chain classically goes to admissibility or, short of exclusion, to the weight a factfinder gives the evidence) but I did not independently verify this against case law or an evidentiary-procedure source this pass — flagging rather than asserting with confidence I don't have.

**(b) What it's trying to be true about.** Not "is this evidence truthful/reliable in content" at all — purely "has this physical/digital item been in continuously accounted-for, undisturbed possession since collection," a pure integrity-of-handling question, orthogonal to content credibility.

**(c) Uncertainty/gap handling.** A gap in the chain (an unaccounted-for period, an undocumented handler) is not something the framework grades on a scale — its own definition frames chain-of-custody as either intact (fully documented) or not; consequences of a gap live outside NIST's definition, in legal procedure.

**(d) Feature our atom-kinds would care about.** A clean precedent for a **terminal/structural failure mode distinct from gradual evidential decay**: "this atom's provenance-chain is broken" is categorically different from "this atom's supporting evidence is weak" — the former is (in legal practice, per the recalled-not-reverified claim above) closer to "can this be admitted/used at all," the latter is a strength question. Worth distinguishing explicitly in our axes if we don't already: a Kind/Authority-track integrity break vs an Evidence-track strength downgrade are not the same kind of gap.

**(e) Provenance.** US forensic/legal evidence procedure, codified via case law plus agency SOPs; NIST's own definitions (SP 800-72, SP 800-101 Rev. 1) are for digital-forensics contexts specifically, not a general legal-procedure restatement.  
URLs: https://csrc.nist.gov/glossary/term/chain_of_custody [fetched directly] · https://www.ncbi.nlm.nih.gov/books/NBK551677/ [not re-fetched] · https://nij.ojp.gov/nij-hosted-online-training-courses/law-101-legal-guide-forensic-expert/pretrial/pretrial-motions/chain-custody [not re-fetched]. Confidence: **[verified]** for the NIST definition itself; **[recalled, unverified]** for the admissibility/weight consequences.

---

## 8. GRADE (Grading of Recommendations Assessment, Development and Evaluation)

**(a) Structure.** **[verified via secondary]** (Wikipedia's GRADE_approach article, cross-checked against a web-search summary of the CDC ACIP GRADE Handbook chapters — I could not get full chapter text since the CDC page returned only a partial/blocked fetch, so treat the domain *names* as solidly corroborated across two independent sources but the exact chapter wording as unconfirmed).

Four certainty levels, quoted from the source fetched: **High** — "high confidence that the true value of the estimate of interest is at one side of a threshold of interest or within a specific range"; **Moderate**, **Low**, **Very Low** — same phrasing, differing confidence level. Starting point by study design: **RCTs start High, observational studies start Low** (search-confirmed via CDC handbook chapter titles + Wikipedia, though Wikipedia itself flags this as a point critics have called overly simplistic — GRADE's own literature apparently allows observational studies to reach High when confounding is well-controlled, so the "RCT-high / observational-low" framing is a *starting default*, explicitly adjustable, not a hard ceiling by design-type alone).

**Five downgrade domains** (web-search corroborated, matches original findings): risk of bias, inconsistency, indirectness, imprecision, publication bias. **Three upgrade domains**: large effect size, dose-response gradient, and "all plausible confounders/biases would reduce a demonstrated effect or increase an effect if none was observed" (i.e. confounding that would work *against* the observed effect, making the observed effect more credible despite it).

**(b) What it's trying to be true about.** Certainty in a *specific quantitative estimate* (an effect size, a risk ratio) as support for a clinical recommendation — evidence-based medicine's answer to "how sure should the guideline-writer be."

**(c) Uncertainty/gap handling.** This is GRADE's whole purpose — it *is* an uncertainty-grading system, uniquely among this sweep's entries. Its handling is structured, not holistic: start at a tier set by design-type, then move by named, individually-checkable criteria (this domain applies → downgrade one level; that domain applies strongly → downgrade two levels), producing an auditable trail for *why* a given certainty level was assigned rather than a single undifferentiated judgment call.

**(d) Feature our atom-kinds would care about.** The clean separation of **(i) a starting tier set by evidence-kind** and **(ii) named, itemizable adjustment criteria that move you within/across tiers** is a strong, directly transferable precedent for how a Ceiling (kind-derived starting max) should compose with specific observed defects — exactly the shape a "trio cut" (support-kind × strength) adjudication would want: start somewhere principled by kind, then apply named criteria rather than one holistic call.

**(e) Provenance.** GRADE Working Group (international, cross-society), used by 100+ organizations including WHO and Cochrane — genuinely the dominant standard in evidence-based medicine, not a fringe or single-body scheme.  
URLs: https://en.wikipedia.org/wiki/GRADE_approach [fetched directly] · https://www.cdc.gov/acip-grade-handbook/hcp/chapter-7-grade-criteria-determining-certainty-of-evidence/index.html [fetch returned 403 this session — blocked] · https://www.cdc.gov/acip-grade-handbook/hcp/chapter-8-domains-decreasing-certainty-in-the-evidence/index.html [not fetched, found via search] · https://www.sciencedirect.com/science/article/pii/S2213398423002713 [not fetched]. Confidence: **[verified via secondary + search corroboration]** for the four-tier structure and domain names; the exact CDC handbook wording is **[recalled, blocked from direct verification]**.

---

## 9. Isnad / ʿIlm al-Ḥadīth (Islamic hadith science)

**(a) Structure.** **[verified via secondary]** — fetched Wikipedia's "Hadith terminology" article directly, which quotes named classical scholars (Ibn Ḥajar al-ʿAsqalānī, al-Dhahabī) rather than merely paraphrasing, giving reasonable confidence even though it's a secondary source. Four primary grades:

- **Ṣaḥīḥ** ("authentic/sound") — Ibn Ḥajar's definition, quoted: "a singular narration conveyed by a trustworthy, completely competent person" via a connected chain free of hidden flaws. **Five explicit conditions**, all required: (1) every narrator trustworthy [ʿadālah], (2) every narrator reliably preserves the narration, by memory or written record [ḍabṭ], (3) the chain is connected [ittiṣāl], (4) free of ʿillah (hidden detrimental flaw), (5) does not contradict more established hadith [absence of shudhūdh/irregularity]. There is a named sub-grade — **Ṣaḥīḥ li-ghayrihi** ("authentic through another") — for an originally-weak hadith elevated to authentic status via convergent corroborating chains, which is itself a notable feature: weak evidence can be *upgraded* by independent corroboration, echoing GRADE's upgrade-domain logic and the two-source rule in journalism.
- **Ḥasan** ("good") — same five conditions as ṣaḥīḥ, except one narrator's competence/precision (ḍabṭ) is "less than complete." Can itself be elevated to ṣaḥīḥ-li-ghayrihi status via corroboration.
- **Ḍaʿīf** ("weak") — fails on either chain-discontinuity or narrator criticism (per Ibn Ḥajar, quoted: weakness "either due to discontinuity in the chain of narrators or due to some criticism of a narrator"). Multiple named *sub-types* of discontinuity are formally distinguished by *where* the break occurs: muʿallaq (break at the very start), mursal (missing link between a Successor and the Prophet), muʿḍal (two-plus consecutive narrators missing), munqaṭiʿ (a break anywhere else) — a genuinely fine-grained taxonomy of *where* a chain fails, not just *that* it fails.
- **Mawḍūʿ** ("fabricated") — al-Dhahabī's definition, quoted: a hadith "of which the text contradicts established norms of Muhammad's sayings or of which the reporters include a liar." This is explicitly a **terminal** category — not "very weak," a qualitatively different, hard-fail status.

Additional named defect-types beyond the four main grades: munkar (a weak narrator's report that contradicts an authentic one), shādhdh (even a *trustworthy* narrator's report, if it contradicts more reliable sources — notably, this shows reliability of the *narrator* does not automatically grant reliability of the *specific report*, i.e. per-instance not just per-source grading), muḍtarib (irreconcilable disagreement among reporters about either the chain or the text itself).

**(b) What it's trying to be true about.** Whether a specific reported saying/action of Muhammad was actually transmitted as claimed — grading BOTH the **chain** (isnād — is every link present and is every individual narrator both honest and precise) AND, separately, the **content** (matn — internal coherence, non-contradiction with more strongly attested material) — genuinely dual-axis, not collapsed into one score.

**(c) Uncertainty/gap handling.** The most structurally rich system in the sweep: (i) a named, ordered, MECE-flavored grade ladder (ṣaḥīḥ > ḥasan > ḍaʿīf, with mawḍūʿ as an off-ladder terminal-fail category rather than the bottom rung of the same ladder); (ii) a *typed* taxonomy of exactly how and where a chain fails (not just "broken" but *which kind* of gap); (iii) an explicit corroboration-based *upgrade* path (ḍaʿīf/ḥasan → ṣaḥīḥ-li-ghayrihi through multiple independent chains) — directly parallel to GRADE's upgrade domains and journalism's two-source rule, arrived at independently by a wholly separate tradition; (iv) per-narrator biographical vetting (ʿilm al-rijāl, "science of men" — a whole companion discipline cataloguing each named transmitter's individual trustworthiness and precision) as the atomic unit underneath chain evaluation, i.e. chain strength is not assessed holistically but composed from individually assessed links.

**(d) Feature our atom-kinds would care about.** This is very plausibly the single richest structural precedent in the whole sweep for our purposes: a genuinely ancient (formalized roughly 9th–13th c. CE, continuously refined and applied at enormous adversarial scale — hundreds of thousands of individually-vetted narrators catalogued), MECE-oriented grade ladder that (1) separately grades chain-integrity and content-integrity, (2) types *how* a chain gap occurred rather than treating all gaps alike, (3) has a hard terminal fabrication category distinct from "weak," and (4) allows weak evidence to be upgraded by independent corroboration. Directly analogous to a Witness-Position / per-source reliability axis crossed with an Evidence-strength axis, with a terminal off-ladder state matching our Prohibitions family's rejected/superseded design instinct.

**(e) Provenance.** Classical Islamic scholarship (Ibn al-Ṣalāḥ's *Muqaddimah*, al-Dhahabī, later systematized by Ibn Ḥajar al-ʿAsqalānī), continuously applied and refined for roughly a millennium, with an entire companion biographical-critical discipline (ʿilm al-rijāl) — genuinely one of the most battle-tested, high-stakes, adversarially-refined source-evaluation systems in human history, independent of and structurally richer than the roughly-contemporaneous Western tradition. **Caveat, stated plainly**: I verified this only against Wikipedia's "Hadith terminology" article, which does quote named classical scholars directly, but I did not reach an actual primary text (Ibn al-Ṣalāḥ's *Muqaddimah* or Ibn Ḥajar's *Nukhbat al-Fikar*, neither readily web-fetchable) — so treat the overall shape as trustworthy (Wikipedia's academic hadith-studies articles are generally well-sourced and this one quotes primaries directly) but the exact wording as secondary-sourced. URLs: https://en.wikipedia.org/wiki/Hadith_terminology [fetched directly] · https://en.wikipedia.org/wiki/Isnad [fetched directly, thinner than expected] · https://www.britannica.com/topic/ilm-al-hadith [not re-fetched] · https://studioarabiya.com/types-of-chains-in-isnad-of-hadith/ [not re-fetched, secondary]. Confidence: **[verified via secondary]**, strong secondary quoting primaries directly.

---

## 10. Archival Science — Principle of Provenance / Respect des Fonds / Original Order

**(a) Structure.** **[recalled, not re-fetched this session]** Two companion principles rather than a ladder: **provenance** (records originating from one creator/body are kept together and distinguishable from another creator's records — don't intermix fonds from different sources) and **original order** (preserve the creator's own filing sequence rather than reorganizing by subject, on the theory that the *order itself* — what was filed next to what, in what sequence — is evidence of the original context of use, not just a retrieval convenience).

**(b) What it's trying to be true about.** Not any single document's trustworthiness, but the integrity of a *collection's structure* as evidence — treating arrangement itself as historically meaningful, not neutral.

**(c) Uncertainty/gap handling.** Not a grading scheme at all; a preservation discipline. Gaps show up as literal missing items in a fonds, documented as such, not scored.

**(d) Feature our atom-kinds would care about.** A precedent for treating an atom's *position/grouping/neighboring context* as itself evidence-bearing rather than incidental — worth holding if our theory ever needs to reason about whether an atom's placement within a collection carries information beyond its own content.

**(e) Provenance.** 19th-c. French/Prussian archival principle, formalized internationally via ICA's ISAD(G) standard and the Society of American Archivists' DACS.  
URLs: https://en.wikipedia.org/wiki/Respect_des_fonds · https://en.wikipedia.org/wiki/Original_order · https://saa-ts-dacs.github.io/dacs/04_statement_of_principles.html. Confidence: **[recalled]**, not independently re-verified this session — lowest-effort entry in the sweep, flagged as such rather than dressed up.

---

## What surprised me / what we did not ask about

**The two most load-bearing established systems (PROV, nanopublications) both deliberately exclude confidence/certainty from their core schema.** This is the single most important negative finding in the sweep and it runs directly against an assumption I initially expected to confirm — I expected mature provenance standards to have *some* built-in trust/ confidence scalar, and went looking for it specifically in PROV-O's property list. There isn't one, and the spec is explicit that this is a deliberate scope boundary, not an oversight. Two independent, well-established, purpose-built lineages both concluded that "what produced this and who's answerable" and "how much should I believe it" are separable concerns best kept in separate layers. That's real evidence for our own Kind/Authority-vs-Evidence split, but it should also make us cautious about over-fusing provenance-tracking and confidence-scoring into one axis anywhere in the taxonomy — the two most careful prior attempts at "track where knowledge comes from" both refused to do that.

**The hadith-science system is structurally richer than anything from the Western academic tradition surveyed here, and it got that way through literal adversarial stakes (forgery for political/sectarian gain was common and consequential) rather than through methodological elegance-seeking.** That's worth naming honestly: the richest MECE ladder in this sweep isn't the "closest to us" tradition, and importing its shape means importing a system built to withstand deliberate, high-stakes fabrication — which may be more rigor than most of our atom-kinds need, but is worth knowing is available as a ceiling.

**I could not verify several things I would have wanted to before calling them settled**, and I want to be explicit about which ones rather than let them blend in with the verified material: (1) AP's actual sourcing language (site unreachable — everything there is secondary-sourced); (2) the exact CDC/GRADE handbook chapter wording on downgrade/upgrade domains (403 blocked; corroborated only via Wikipedia + a web-search summary, not the handbook itself); (3) the historiography external/internal-criticism split against any real primary (Bernheim, Langlois & Seignobos) — this is probably the weakest-verified entry in the whole sweep despite being maybe the most directly relevant one to our Kind-vs-Evidence axis question, and someone should actually pull it from a real historical-methods textbook before it gets treated as settled; (4) the legal consequences of a broken chain of custody (exclusion vs. weight-reduction) — plausible, consistent with general legal knowledge, but not checked against case law this session.

**Nobody in this sweep has a clean answer for "uncertainty about uncertainty"** — the steward's phrase. GRADE comes closest (a certainty *about* a certainty rating could in principle be represented by how many of its five/three domains were contestable or how borderline the tier-assignment call was, and its whole apparatus is designed to make that call auditable) but even GRADE reports a single collapsed tier at the end, not a distribution or a second-order confidence on the rating itself. The epistemology-of-testimony literature (#5) is the one place where second-order questions ("how much should the *hearer's* own uncertainty about the *testifier's* reliability itself count") get discussed explicitly and philosophically, but it doesn't resolve into anything formalizable the way GRADE does. This looks like a genuine gap across every established system surveyed here, not just something our organic taxonomy failed to inherit — worth flagging to the coordinator/Joseph as possibly requiring original design work rather than borrowed structure, since I could not find a precedent for it anywhere in this domain.

**One correction to the assigned findings worth flagging plainly**: the journalism attribution ladder is not cleanly four rungs (On the Record → On Background → Deep Background → Off the Record) as the original pass stated — the Wikipedia source names at least six distinct terms including "Not for attribution" and "Unattributable" as separate categories with distinct licenses sitting in between. Minor, but the original findings oversimplified a real professional taxonomy.
