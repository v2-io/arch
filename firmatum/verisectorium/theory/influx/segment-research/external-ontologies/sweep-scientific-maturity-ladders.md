# Sweep: scientific/engineering maturity & corroboration ladders

*Domain: readiness ladders (TRL/XRL family), evidence-certainty grading (GRADE), evidence hierarchies (Oxford CEBM), mathematical statement-kind vocabulary, proof-assistant trust mechanisms (Lean/Coq), and the philosophy-of-science ancestors (Popper/Lakatos). Gathered 2026-08-10 for verisectorium's epistemic-state work. Written to record what is actually there, including what's inconvenient or unresolved for our working taxonomy — not to flatter it.*

*Verification note up front, honestly: several primary PDFs/tables resisted WebFetch (CDC page 403'd; the NASA and Oxford CEBM PDFs came back as raw compressed PDF stream data the fetch tool couldn't decode; the BMJ GRADE paper 403'd). Where I could not get primary-source text through, I fell back to a secondary source (Wikipedia, mirrors) and say so explicitly per entry. Nothing below is asserted as verified when it was actually recalled — see the confidence line on each entry.*

---

## 1. NASA Technology Readiness Levels (TRL) / ISO 16290 / the XRL genre

**(a) Structure**, per Wikipedia's rendering of NASA's own list (I could not get text out of the primary NASA PDF — see confidence note):

1. Basic principles observed and reported
2. Technology concept and/or application formulated
3. Analytical and experimental critical function and/or characteristic proof-of-concept
4. Component and/or breadboard validation in laboratory environment
5. Component and/or breadboard validation in relevant environment
6. System/subsystem model or prototype demonstration in a relevant environment
7. System prototype demonstration in a space (operational) environment
8. Actual system completed and "flight qualified" through test and demonstration
9. Actual system "flight proven" through successful mission operations

Single ordinal axis, ascending along two entangled dimensions that the definitions never fully separate: (i) **environment fidelity** (lab → relevant → operational/space) and (ii) **integration scope** (component/breadboard → subsystem/prototype → full system). ISO 16290:2013 formalizes this with explicit per-level assessment criteria (I did not verify the ISO text itself — only that the standard exists and canonizes the scale, per secondary sources). The EU/Horizon variant reportedly diverges slightly (TRL 9 alternative = "competitive manufacturing," not just flight) — again, secondary-sourced, not verified against the EU primary document.

**(b) What it's trying to be true about**: not a truth-claim at all — a **readiness-to-deploy** claim about a *technology/system*, not a proposition. Category-different from the rest of this sweep, which matters: TRL answers "how much has this actually been tried, at what integration scope, in what environment," not "how likely is this to be correct."

**(c) Uncertainty/gap handling**: none explicit, and this is a real gap in the system itself — there's no vocabulary in TRL for "we think this would work but haven't tried it yet" versus "we tried it and it failed" versus "we haven't decided if it's worth trying." The ladder only records *how far up the demonstration chain you've climbed*, not *why you haven't climbed further* or *what would go wrong if you tried*. The terminal state (TRL 9) is the interesting exception: it is **empirically unforgeable** — you cannot claim it without the actual flight/mission event having happened. That's a real design feature: the top rung is event-gated, not asserted.

**(d) Structural feature our atom-kinds would care about**: a single-axis ordinal ladder with a **hard, event-gated terminal state** and **no native gap-explanation vocabulary**. Useful as a *contrast case* — it shows what a maturity ladder looks like when it optimizes for "how far along" and gives up entirely on "why isn't it further" or "what's the confidence given where it is." Also notable: TRL is a *genre* now — Manufacturing Readiness Levels, AI Readiness Levels, Commercial Readiness Index, etc. are all forks, evidence that the pattern generalizes past its origin domain but the fork proliferation is itself a symptom of the original scale's definitional looseness once it left aerospace ("sophistication gradually diminished," per Wikipedia's synthesis, which is itself worth being skeptical of as a claim rather than adopting wholesale).

**(e) Provenance + confidence**:
- NASA TRL page: https://www.nasa.gov/directorates/somd/space-communications-navigation-program/technology-readiness-levels/ — not fetched this session.
- NASA definitional PDF: https://esto.nasa.gov/files/trl_definitions.pdf — fetched, but the tool returned undecoded PDF stream data, not readable text. **The 9-level list above is from Wikipedia's rendering, not confirmed against this primary**, though the wording is the well-known standard NASA phrasing and I'd expect it matches; flagging as recalled/secondary, not verified this session.
- ISO 16290:2013: https://www.iso.org/standard/56064.html — not fetched (paywalled standard); existence and role reported secondhand.
- https://en.wikipedia.org/wiki/Technology_readiness_level — fetched this session, used as the actual source of the list above and the EU/DoD divergence notes.

---

## 2. GRADE (Grading of Recommendations Assessment, Development and Evaluation)

**(a) Structure** (per Wikipedia's GRADE article, fetched this session — primary CDC and BMJ sources 403'd; see confidence note):

- **Starting point**: certainty of evidence starts from **study design** (the Wikipedia fetch didn't give me the exact RCT-starts-high / observational-starts-low framing in so many words this pass, though that is GRADE's well-known starting convention — flagging this specific claim as recalled, not re-verified against primary this session).
- **Four certainty categories** (confirmed via fetch): **High / Moderate / Low / Very Low** — each has an actual definition about how far "the true value" might deviate from the estimate, not just a bare label:
  - High: confidence the true value lies within a specific range/side of a threshold.
  - Moderate: true value "may deviate slightly" from the estimate.
  - Low: true value "may deviate" from the estimate.
  - Very Low: true value "may deviate significantly."
- **Downgrade/upgrade domains**: the article confirms these exist as named, itemized domains ("comprehensive criteria for downgrading and upgrading") but the fetch did not return the five/three domain names verbatim this session. From general knowledge (NOT verified this session — flagging explicitly): the five canonical downgrade domains are risk of bias, inconsistency, indirectness, imprecision, and publication bias; the three upgrade domains are large magnitude of effect, dose-response gradient, and "plausible confounding would reduce the observed effect." **This paragraph is recalled, not confirmed against a primary source in this pass** — the coordinator should treat the domain *names* as needing a fresh independent check before they're load-bearing in the final model, even though their *existence and role* is confirmed.
- **Two-axis design, confirmed**: GRADE explicitly separates **certainty of evidence** from **strength of recommendation** (Strong/Weak) — the Wikipedia article states this plainly: "The GRADE approach separates recommendations following from an evaluation of the evidence as strong or weak," independent from the certainty rating. This is a real, confirmed two-axis system.

**(b) What it's trying to be true about**: not "is this claim true" in the abstract — specifically, **how much should a specific effect-size estimate be trusted as a basis for a clinical/policy recommendation**. It's explicitly decision-oriented: certainty of evidence feeds into, but is analytically separate from, whether to actually recommend an action.

**(c) Uncertainty/gap handling**: this is the system's whole reason for being, and structurally it is the most mature "explain your uncertainty about uncertainty" system in the sweep — instead of one confidence number, an assessor works through a fixed, named checklist, and every downgrade or upgrade has an inspectable *reason* attached (not just a magnitude). This is a directly reusable pattern: confidence-with-attached-itemized-reasons beats confidence-as-a-bare-number, and it beats it specifically because a reader auditing the rating can check each domain against the actual study population rather than trusting an unglossed number.

**(d) Structural feature our atom-kinds would care about**: (i) starting-point-by-provenance-kind (a claim's *kind of backing* sets its floor before anything else is assessed) — resonant with our Evidence axis's "support-kind × strength" cut; (ii) certainty-of-evidence and strength-of-recommendation as genuinely separate axes — a strong recommendation can rest on low-certainty evidence (e.g., when the downside of inaction is severe) and this is treated as a legitimate, expected combination, not a contradiction. That's a sharp, useful precedent for keeping our Assertions (evidence/truth) and Norms/Directives (authority/binding-ness) families cleanly separated rather than conflating "how sure are we" with "how hard should you follow this."

**(e) Provenance + confidence**:
- CDC ACIP GRADE handbook ch. 7: https://www.cdc.gov/acip-grade-handbook/hcp/chapter-7-grade-criteria-determining-certainty-of-evidence/index.html — attempted fetch, **403 Forbidden**, not read this session.
- BMJ 2008 GRADE paper (Guyatt et al., "GRADE: an emerging consensus…"): https://www.bmj.com/content/336/7650/924 — attempted fetch, **403 Forbidden**, not read this session.
- https://en.wikipedia.org/wiki/GRADE_approach — fetched this session; source of the confirmed structure above (four categories, two-axis separation). **The specific five/three domain names are recalled from training, not re-confirmed against any source this session** — flag this explicitly to the coordinator.
- gradeworkinggroup.org — canonical home, not fetched.
- Confidence summary: **medium-high** on overall shape (two-axis design, four categories, domain-based reasoning as a genre) since two independent passes (this session's search-findings pass + this fetch) agree; **low-medium** on the exact domain *names* and the exact study-design starting-point wording, since I could not get a primary source to render this session.

---

## 3. Oxford Centre for Evidence-Based Medicine (OCEBM) Levels of Evidence, 2011 revision

**(a) Structure**: I was **not able to verify the exact table** this session — both the CEBM webpage and the CEBM PDF fetches returned unusable content (the webpage explicitly told the fetch tool not to read the table without its companion documents; the PDF came back as undecoded binary stream data). What I can report with real confidence: the 2011 revision is **explicitly organized around clinical *question type*** — the CEBM's own webpage text (which did render, just not the table itself) instructs readers that "the table is intended to be used alongside the Introductory Document and Background Document" and warns against reading the table in isolation — i.e., the framework itself insists it is not a bare ladder, it's a ladder-plus-interpretive-apparatus. Wikipedia's `Levels of evidence` article confirms the general redesign happened in 2011 "to make it more understandable and to take into account recent developments in evidence ranking schemes" but likewise didn't render the actual per-question-type rows through this fetch pass.

From the search-findings pass (not independently re-verified by fetch this session, so treat as recalled/secondary): question types include diagnosis, prognosis, treatment benefits, treatment harms, and screening, each with its own 5-level ladder ordered by study-design rigor within that question type (e.g., for treatment benefit: systematic review of RCTs > individual RCT > cohort study > case-control/case-series > mechanism-based reasoning). One genuinely interesting nuance the CEBM page *did* render and I can report with confidence: there is a documented ambiguity even within the canonical table about how to read "Level 1" for treatment benefits — whether it means "N-of-1 randomized trials OR systematic reviews of RCTs" — and the CEBM felt the need to publish a clarifying note about their own table's ambiguity. That's a live admission from the standard-setters that even a canonical ladder can be read two ways by competent users.

**(b) What it's trying to be true about**: the same general territory as GRADE (how much to trust a piece of clinical evidence) but forked earlier and structured by **question type first, rigor-ordering second** — i.e., its primary axis-choice is "what kind of claim is this" (diagnostic accuracy? prognosis? treatment effect? harm?) before "how strong is the study design," rather than GRADE's single certainty scale applied after evidence synthesis.

**(c) Uncertainty/gap handling**: less mature than GRADE — it's fundamentally a study-design ranking (a proxy for likely rigor) rather than a checklist of actual, itemized threats to validity for the specific evidence at hand. It has been substantially superseded by GRADE for guideline-writing for exactly this reason (a lower-tier study design doesn't tell you *why* it's less trustworthy, whereas GRADE forces the assessor to name the actual problem).

**(d) Structural feature our atom-kinds would care about**: the fork-by-question-type move. This is the single most directly resonant structural finding for the steward's framing — "what they need to be true about" is treated by OCEBM as **the primary fork that has to happen before any ladder gets applied at all**, not as an afterthought. A single universal maturity ladder was tried, found wanting, and deliberately abandoned in favor of question-relative ladders that still each internally rank by rigor. Worth weighing seriously against any temptation in our system to build one universal ladder across all of Assertions/Definitions/Decisions/etc.

**(e) Provenance + confidence**: **low on the exact table**, since neither primary rendered this session.
- https://www.cebm.ox.ac.uk/resources/levels-of-evidence/ocebm-levels-of-evidence — fetched, webpage text rendered but not the table; used for the "read together, not in isolation" quote and the Level-1-ambiguity note, both of which I'm confident are accurate (they're quoted/paraphrased from actually-rendered text).
- https://www.cebm.net/wp-content/uploads/2014/06/CEBM-Levels-of-Evidence-Introduction-2.1.pdf — fetched, returned undecoded PDF binary, **not read**.
- https://en.wikipedia.org/wiki/Levels_of_evidence — fetched, confirmed the 2011-redesign fact and general shape, did not render the per-question-type table.
- The specific question-type list (diagnosis/prognosis/treatment-benefit/treatment-harm/screening) and the worked rigor-ordering example are **carried over from the search-findings pass, not independently confirmed by fetch this session** — flagged for the coordinator as recalled, moderate confidence (this is genuinely well-known material, but "well-known" is not "verified" per this task's own standard).

---

## 4. Mathematical statement-kind vocabulary (axiom, postulate, definition, conjecture, lemma, theorem, corollary, proposition)

**Not independently re-fetched this session** — carried forward from the search-findings pass as-is, since it's a craft convention rather than a citable institutional standard and the two sources listed (a mathematician's blog post and Wikipedia's parallel-postulate article) are the right register for what it is. Reporting it here anyway because it's the sharpest **kind-vs-degree separation** exemplar in the whole sweep and genuinely load-bearing for the coordinator's synthesis:

**(a) Structure**: no formal ladder — these are **role labels inside an argument's structure**, not degrees of certainty:
- axiom / postulate = accepted without proof, foundational (historically: axiom = self-evident-to-all, postulate = domain-specific starting assumption; the distinction has mostly collapsed in modern usage)
- definition = stipulative — not truth-apt the same way an assertion is
- **conjecture = the one genuine epistemic-status word in the set**: proposed, believed plausible, explicitly *not yet proven*
- theorem / lemma / corollary / proposition = all **proved**, differing only by *rhetorical role* (main result / stepping-stone / immediate consequence / lesser-but-standalone result) — confirmed convergently across sources that there is **no formal distinction** in rigor or certainty between these four; it's authorial signaling about importance and narrative structure, not an epistemic ladder.

**(b) What it's trying to be true about**: the *role* a statement plays in a deductive structure, not how sure anyone is that it's true (except conjecture, which is the exception that proves the rule).

**(c) Uncertainty/gap handling**: essentially none within the proved tier (theorem/lemma/corollary are all "fully proved" as a hard binary, no gradation) — mathematics offloads the actual uncertainty-handling to a different, much stricter mechanism: proof itself, and (in the mechanized case) the proof-assistant kernel described in #5 below. The one soft spot in this whole vocabulary — the Parallel Postulate — is instructive precisely because it broke the "accepted without proof, foundational, therefore settled" assumption: it turned out to be a *choice* (Euclidean vs. non-Euclidean geometry), not a fact, and this was only discovered centuries after being treated as bedrock. That's a genuine cautionary case for any permanent ontology that wants to treat some class of atoms as foundational-and-therefore-lower-scrutiny.

**(d) Structural feature our atom-kinds would care about**: this is a clean real-world case of "same terminal certainty, different kind/role axis" — exactly the kind of conflation our system should avoid (a record's *kind* — is it a definition, a derived result, a stepping-stone — should not silently double as its *epistemic strength*). Also useful precedent for treating Definitions as a genuinely distinct family from Assertions (stipulative vs. truth-apt), which our current taxonomy already does.

**(e) Provenance + confidence**: not fetched this session, carried from the initial search pass, so this whole section is **recalled**, not verified this pass. URLs (unfetched): https://divisbyzero.com/2008/09/22/what-is-the-difference-between-a-theorem-a-lemma-and-a-corollary/, https://en.wikipedia.org/wiki/Parallel_postulate, https://www.mathsisfun.com/algebra/theorems-lemmas.html (explicitly pedagogical, not authoritative).

---

## 5. Proof-assistant trust hierarchies (Lean / Coq-Rocq), LCF-style kernels

**Not independently re-fetched this session** — carried forward from the search-findings pass. This is the structurally most interesting finding in the sweep and deserves the coordinator's attention even unverified-this-pass, because it's a genuinely different *kind* of system from all the ladders above.

**(a) Structure**: not an ordinal scale at all — a **dependency/provenance graph rooted in a minimal trusted kernel** (the LCF architecture, Milner, Edinburgh, 1970s; both Lean and Coq/Rocq follow it). A proof's trust status is determined by two things: (i) was it checked by the small trusted kernel (binary: yes/no), and (ii) which **axioms** does it transitively depend on (an enumerated, not ranked, set — some axioms like classical choice or propositional extensionality are standard/widely-accepted, others are nonstandard, and an incomplete proof leaves an explicit, trackable placeholder — `sorry` in Lean, `admit`/`Admitted` in Coq — rather than silently failing or silently succeeding).

**(b) What it's trying to be true about**: whether a specific formal claim is *actually proved*, mechanically, with zero trust extended to the human or the elaborate tactic machinery that constructed the proof — only the tiny kernel needs to be trusted.

**(c) Uncertainty/gap handling**: the strongest in the whole sweep, structurally. A gap (an unproved `sorry`) is not scored or graded — it's **enumerated and automatically propagated** to every downstream theorem that depends on it, so a consumer can always ask "what does this actually rest on" and get a complete, machine-generated answer, not a human-asserted confidence level. No system in this sweep handles "uncertainty about uncertainty" as rigorously — because it refuses to grade uncertainty at all, converting it instead into an enumerable, inspectable, unforgeable list of exactly what's assumed or missing.

**(d) Structural feature our atom-kinds would care about**: two-axis, but neither axis is a confidence gradient — (1) kernel-checked-or-not (binary, event-based — an actual verification event either happened or didn't), (2) enumerated axiom/gap dependency set (unranked, but complete and propagated). This is a hard-edged, adversarially robust alternative to every graded-confidence system above, worth holding as a genuine contrast/limit case: **rather than asking "how confident are we," it asks "exactly what would have to be true, and has that actually been checked."** If any part of our system wants a truly unforgeable "this atom's truth rests on exactly these N unproven/assumed things" property (as opposed to a vibes-based confidence number), this is the working precedent, not GRADE.

**(e) Provenance + confidence**: not fetched this session — carried from search pass, **recalled not verified this pass**. URLs (unfetched): https://lean-lang.org/faq/, https://cs.ru.nl/~freek/courses/mfocs-2024/slides/rutger.pdf. The de Bruijn criterion (N.G. de Bruijn, Automath project, 1970s) is the deeper primary lineage and was flagged as a follow-up, not pursued.

---

## 6. Popper (corroboration) and Lakatos (research programmes: hard core / protective belt / progressive vs. degenerating)

**Not independently re-fetched this session** — carried from the search pass as the conceptual ancestor layer, not an operational system. Reported because the trajectory-vs-snapshot distinction is structurally important and not covered by any of the operational systems above.

**(a) Structure**: Lakatos (1970, responding to and revising Popper) replaces single-theory falsification with: a **hard core** (the programme's non-negotiable central commitments, protected from direct refutation), a **protective belt** of auxiliary hypotheses (the part that actually absorbs anomalies and gets revised), a **negative heuristic** (rules for what must not be touched — protect the hard core) and a **positive heuristic** (rules for how the programme should be extended/developed). A programme's overall status is judged as **progressive** (it predicts genuinely novel facts, and some are subsequently confirmed) or **degenerating** (it only patches the belt after the fact to accommodate anomalies, generating no novel predictions) — and critically, this is a **trajectory judgment over time**, not a static rung assigned at a moment.

**(b) What it's trying to be true about**: not the truth of a single claim, but the **health of a whole research programme** — is confidence in this cluster of interlocking claims trending up (progressive) or down (degenerating) over successive rounds of anomaly-encounter and patching.

**(c) Uncertainty/gap handling**: distinctive in kind — Lakatos doesn't grade a claim's certainty at a point in time at all; he grades the **behavior of the community's response to anomalies** over time. A degenerating programme is diagnosed not by any single claim being wrong but by the *pattern* of only ever patching, never predicting.

**(d) Structural feature our atom-kinds would care about**: the only source in the sweep offering a genuine **trajectory axis** (rising/falling confidence over time) distinct from a static tier, plus a principled account of **why some claims resist revision and others absorb it** (hard-core protection vs. protective-belt flexibility) — directly suggestive for any "centrality" dimension our atoms might want (is this a load-bearing/protected claim vs. a peripheral one that's expected to flex under new evidence).

**(e) Provenance + confidence**: not fetched this session, **recalled not verified this pass**. URLs (unfetched): https://aeon.co/essays/imre-lakatos-and-the-philosophy-of-bad-science (secondary but Aeon commissions from academics), https://pmc.ncbi.nlm.nih.gov/articles/PMC9643948/ (operational worked example, "progressive and degenerative journals"). Primary citation, not fetched: Lakatos, I. (1970), "Falsification and the Methodology of Scientific Research Programmes," in Lakatos & Musgrave (eds.), *Criticism and the Growth of Knowledge*, Cambridge University Press — this is a canonical, load-bearing philosophy-of-science text; if the coordinator wants to lean on Lakatos's account structurally (not just cite it), the primary chapter should be read directly rather than trusted secondhand, given how much interpretive weight rides on the hard-core/protective-belt distinction.

---

## What surprised me / what we did not ask about

- **The fetch pass mostly failed on primaries, and that's itself informative.** Four of six primary URLs (CDC, BMJ, NASA PDF, CEBM PDF) came back either 403'd or as undecoded binary. This is a real, boring, structural fact about how "authoritative" these sources are in practice: the most load-bearing primary documents for GRADE and TRL are sitting behind paywalls, JS-rendered pages, or non-text PDF encodings that make them hard for even a diligent reader to actually verify quickly — meaning most people citing "GRADE says X" or "TRL is defined as Y" are, like this sweep partially had to be, working from secondary renderings. If verisectorium ever wants to model "how do real external epistemic communities actually propagate verified-vs-recalled status," the friction I just hit is itself a small case study: even well-resourced, well-cited standards are frequently *transmitted* through unverified secondary paraphrase, not re-checked against primary text at each hop. That's an argument for taking our own "verified vs. recalled" distinction very seriously as a first-class atom property, because the ambient practice around us — even in rigorous fields — visibly does NOT reliably do this.

- **None of the six systems have a native "uncertain about our own uncertainty" (meta-uncertainty) rung**, despite the steward's framing explicitly asking for it. GRADE comes closest structurally (a "Very Low" certainty rating combined with a note about *why* — e.g., "downgraded twice for imprecision and once for risk of bias" — functions as meta-uncertainty in practice, since the reasons themselves can be contested), and the Lean/Coq axiom-enumeration is arguably the actual closest fit (an axiom dependency you're unsure whether to trust is meta-uncertainty made structural and enumerable) — but none of them name "uncertainty about the uncertainty rating itself" as a first-class thing you can assert. This might be a genuine gap in the external landscape, not just in our organic ladders — worth being honest that we may be inventing something without much precedent here rather than "discovering" it in a mature source, if the final model wants this concept explicitly.

- **TRL's near-total silence on gap-explanation, next to GRADE's near-total focus on it, is a useful polarity** — they sit at opposite ends of a spectrum (readiness-of-a-thing vs. certainty-of-a-claim) and the field hasn't produced much in between, i.e. no widely-adopted system that grades *both* "how far along is this" *and* "why isn't it further, itemized." That combination — which sounds like exactly what a living, mid-maturity Assertion in our system needs — doesn't appear to exist as an established external pattern. Worth flagging plainly to the coordinator: this part of the final model may need to be genuinely novel synthesis, not adaptation, and should be built and labeled with that honesty rather than presented as "grounded in X."

- **The OCEBM's own live self-correcting behavior** (publishing a clarifying note about ambiguity in their own canonical table) is a small but real data point that even a "settled, canonical" ladder from a serious institution needs living maintenance and public erratum-style correction — resonant with verisectorium's whole premise about atoms needing honest, current epistemic state rather than a one-time-stamped label.

- **What we did not ask about, and probably should in a follow-up pass**: (1) how *engineering* safety-case / assurance-case frameworks (e.g., Goal Structuring Notation, ISO 26262 ASIL levels) handle multi-factor claims-with-evidence-with-gaps — these are argument-structure ontologies (claim/argument/evidence trees) that are much closer in shape to "an atom with supporting/undermining links" than any single-axis ladder above, and were not investigated this pass at all; (2) how legal/evidentiary standards (preponderance / clear-and-convincing / beyond-reasonable-doubt, plus the Federal Rules of Evidence's treatment of hearsay/provenance) handle witness-position and provenance-of-claim, which is directly adjacent to our Accounts family's Witness Position axis and wasn't touched here; (3) IPCC's own confidence/likelihood framework (which explicitly separates "confidence in a finding" from "likelihood of an outcome" as two named, defined, differently-scaled axes, and has a well-known calibrated-language guide) — this may be the single closest external analogue to what the steward is asking for and it is a real gap that this sweep didn't reach it; worth a dedicated follow-up sweep.
