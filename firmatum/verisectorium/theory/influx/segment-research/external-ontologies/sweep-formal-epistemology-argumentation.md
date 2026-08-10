# Sweep: Formal Epistemology & Argumentation Theory

*Domain: the academic/professional ontologies underlying "how sure are we, about what, and how do we say so honestly when we aren't." Covers analysis-of-knowledge epistemology, defeasible reasoning, formal argumentation theory, and the uncertainty calculi (probabilistic and non-probabilistic) that sit under confidence-grading.*

*Verification pass: primary/SEP sources fetched and checked against the search-pass draft for §3 (Pollock defeaters), §5 (Dung's AAF extensions), §8 (Walley imprecise probability). Those three entries below are **verified against fetched text**, quoted or near-quoted from the fetch. The remaining entries (§1, 2, 4, 6, 7, 9, 10) are carried from the search pass at **recalled** confidence — flagged per-entry below; I did not have budget to re-fetch every primary text and did not want to silently upgrade their confidence label by omission.*

---

## 1. JTB and the post-Gettier "analysis of knowledge" tradition

**(a) Structure.** Not a ladder — a **conjunctive gate**. Classical JTB: knowledge = justified + true + belief, three individually-necessary, jointly-(claimed)-sufficient conditions. Gettier (1963) showed the gate is satisfiable by luck without producing knowledge. Post-Gettier moves: add a fourth no-defeater/defeasibility condition, or replace "justification" with causal/tracking/sensitivity/safety conditions (Nozick, Sosa).

**(b) What it's true about.** The conditions under which a *belief* — not an assertion, not a document, not a record — counts as *knowledge* held by an epistemic subject.

**(c) Gap/uncertainty handling.** The entire post-Gettier literature is arguably nothing but gap-handling: it exists to diagnose the specific failure mode where a belief looks warranted (satisfies every visible criterion) but isn't, because the warrant-looking-satisfied and the belief-being-true came apart by luck rather than by the justification actually tracking the truth. This is a precise formal precedent for "confidently-held but not actually grounded" — useful vocabulary for us even though the tradition targets *belief-in-a-mind*, not *atoms-in-a-collection*.

**(d) Feature our atom-kinds would care about.** The gate is conjunctive and non-additive — you don't get "70% knowledge" by scoring 2 of 3 conditions; failing any one collapses the whole status to non-knowledge. That's a genuinely different shape from a ladder/rung system, and worth having as a contrast case: not every epistemic-state system decomposes into ordered rungs.

**(e) Provenance / confidence.** SEP "The Analysis of Knowledge" (https://plato.stanford.edu/entries/knowledge-analysis/), IEP "Epistemic Justification" (https://iep.utm.edu/epi-just/). Both are peer-reviewed/editorially-maintained reference encyclopedias — high provenance confidence. **Recalled, not re-fetched this pass** — the JTB/Gettier structure is extremely stable, low-risk content, but flagging per policy.

---

## 2. Internalism vs. externalism about justification (evidentialism vs. reliabilism)

**(a) Structure.** A two-family fork, not a ladder:
- **Evidentialism** (Conee & Feldman) — internalist: justification is entirely a function of the subject's *possessed evidence* / accessible mental states.
- **Reliabilism** (Goldman) — externalist: justification is a function of whether the belief-forming *process* is reliable, whether or not the subject can introspect why.

**(b) What it's true about.** *Where justification lives* — in the believer's own accessible evidence-state, versus in facts about the reliability of the process/mechanism that produced the belief, which may be entirely opaque to the believer.

**(c) Gap/uncertainty handling.** Not itself a gap-calculus — it's upstream of one. It determines *what kind of thing* would even count as closing a gap: more accessible evidence (internalist repair) vs. a more reliable-in-fact process/pipeline (externalist repair), and these can diverge — a belief can be internally well-evidenced but externally unreliable, or vice versa.

**(d) Feature our atom-kinds would care about.** This is a genuine design fork for an atom's justification field: does an atom carry its *own* evidence inline (internalist design — the atom is self-justifying, auditable without leaving it), or does it inherit trust from *provenance/process metadata* (externalist design — trust the atom because of who/what produced it and how, even if the atom itself doesn't restate the case)? Most real systems are hybrids, but naming the fork explicitly is useful before deciding which parts of our schema are internalist-shaped (inline grounds) vs. externalist-shaped (source/method/pipeline-reliability fields).

**(e) Provenance / confidence.** SEP "Internalism vs. Externalism about Justification" (https://plato.stanford.edu/entries/justep-intext/), IEP "Reliabilism" (https://iep.utm.edu/reliabilism/). High-provenance reference sources. **Recalled, not re-fetched.**

---

## 3. Pollock's defeasible reasoning — rebutting vs. undercutting defeaters ✅ verified

**(a) Structure — fetched directly from SEP "Defeasible Reasoning" (2026-08 fetch):**
- **Rebutting defeater**: "themselves prima facie reasons for believing the negation of the conclusion" — i.e., it attacks the *conclusion* by directly supporting its opposite.
- **Undercutting defeater**: "provide[s] a reason for doubting that q provides any support, in the actual circumstances, for r" — i.e., it attacks the *inference link* between premise and conclusion, not the conclusion itself. (Pollock's canonical example, from the search pass, not independently re-verified this fetch: a red light makes an object look red, so "looks red" no longer supports "is red" in that lighting — the premise itself may still be true, but it stops licensing the inference.)
- **Warrant**: "A belief is ultimately warranted in relation to a data set (or epistemic basis) just in case it is supported by some ultimately undefeated argument proceeding from that epistemic basis." Per the fetched SEP text, warrant is defined relative to remaining *undefeated* — the fetch did not surface an explicit statement that defeaters-of-defeaters are a named recursive category in Pollock's own vocabulary (the search-pass framed "defeaters can themselves be defeated" as near-verbatim Pollock; the fetch supports the *undefeated-relative-to-evidence* framing but I could not confirm the recursive-defeat vocabulary from the fetched excerpt specifically — flagging as **not fully confirmed**, treat that specific sub-claim as inferred-from-shape until reread against Pollock 1987 directly).

**(b) What it's true about.** How a *prima facie* justified conclusion in a defeasible-reasoning system can be legitimately withdrawn without the original inference having been invalid — i.e., the logic of *retracting* a currently-held conclusion in light of new evidence, as distinct from having reasoned badly in the first place.

**(c) Gap/uncertainty handling.** This system's entire subject matter IS gap/defeat handling. Two structurally distinct *kinds* of "this claim is now in question": (i) direct counter-evidence (rebutting) vs. (ii) evidence that the reasoning step itself doesn't apply here (undercutting) — even though the premise may remain true. This distinction (attack-the-conclusion vs. attack-the-inference-link) is sharper than most working epistemic vocabularies and doesn't collapse into a single "confidence went down" scalar.

**(d) Feature our atom-kinds would care about.** The rebut/undercut split is directly portable as a **two-kind defeat-relation taxonomy** for how one atom can compromise another — most vocabularies we've organically grown probably don't distinguish "this is wrong" from "the reasoning that got you here doesn't actually hold in this case," and Pollock's is the canonical, citable source for exactly that distinction. Also useful: defeat/warrant is explicitly **relational to an evidence set**, not an intrinsic per-atom property — a claim is warranted *relative to* a data set, which is compatible with the same claim being warranted for one collection-state and not another.

**(e) Provenance.** SEP "Defeasible Reasoning" (https://plato.stanford.edu/entries/reasoning-defeasible/) — canonical, actively maintained, peer-reviewed encyclopedia entry. Primary source: Pollock, "Defeasible Reasoning," *Cognitive Science* 11 (1987) (http://www.horty.umiacs.io/courses/readings/pollock-1987-defreasoning.pdf) — not fetched this pass, cited but unverified directly. **Confidence: the two defeater definitions and the warrant formula are fetch-verified (quoted above); the defeat-of-defeaters recursive claim is unverified this pass.**

---

## 4. Toulmin's model of argument

**(a) Structure.** Six-part, two-tier: three essential elements — **Claim** (the conclusion being argued for), **Data/Grounds** (the evidence offered), **Warrant** (the inference-licensing principle connecting grounds to claim) — plus three supporting elements — **Backing** (why the warrant itself should be trusted), **Qualifier** (an explicit strength-marker on the claim, e.g. "presumably," "necessarily," "probably"), **Rebuttal** (explicitly stated conditions under which the claim does *not* hold, i.e. named exception-conditions).

**(b) What it's true about.** The anatomy of a *single argument's* justificatory bundle — not a system for adjudicating between competing arguments (that's Dung, below), but a template for what a well-formed individual argument, together with its own stated confidence and stated exceptions, actually contains.

**(c) Gap/uncertainty handling.** The Qualifier is a **first-class, named slot** — arguments in this model are expected to *declare* their own strength rather than leaving it implicit in tone or omission. The Rebuttal slot is likewise first-class — it is where an arguer is expected to name, up front, the conditions under which their own claim would not hold, rather than leaving that discoverable only by attack from outside.

**(d) Feature our atom-kinds would care about.** This is the best template found this sweep for *what fields a single atom's epistemic-state record should contain*, independent of any global acceptance calculus: grounds (evidence), warrant (why the grounds support this conclusion), backing (why the warrant is itself trustworthy), a declared qualifier (self-reported strength), and a declared rebuttal-scope (self-reported known exceptions). Notably this is a *self-reporting* schema, not an externally-computed status — closer to internalist/evidentialist in spirit (§2) than to Dung's externally-computed acceptance.

**(e) Provenance.** Stephen Toulmin, *The Uses of Argumentation* (1958) — foundational text in informal logic/rhetoric, not a mathematically formalized system, but with deep uptake in composition pedagogy, legal reasoning, and argumentation-mining NLP. Secondary: Hitchcock (McMaster), scholarly treatment of warrant/backing (https://www.humanities.mcmaster.ca/~hitchckd/Toulminswarrants.pdf); SEP "Argument and Argumentation" situates it within the field (https://plato.stanford.edu/entries/argument/). **Recalled, not re-fetched** this pass — worth a direct primary-text check before treating the six-part decomposition as gospel, since secondary summaries of Toulmin vary in exactly how they carve the six parts.

---

## 5. Dung's Abstract Argumentation Frameworks (AAF) ✅ verified

**(a) Structure — fetched and confirmed (Wikipedia's rendering of Dung 1995, cross-checked against SEP's framing of the attack relation):**
- **Argumentation Framework**: a pair ⟨A, R⟩ — a set of abstract arguments A, and a binary attack relation R ⊆ A×A. Arguments are treated as atomic/opaque; only the attack structure matters.
- **Conflict-free set**: a set E ⊆ A such that no argument in E attacks another argument in E.
- **Acceptable / defended**: an argument a is acceptable with respect to E ("E defends a") iff for every attacker b of a, some member of E attacks b.
- **Admissible set**: conflict-free, and every member is acceptable w.r.t. the set itself.
- **Complete extension**: an admissible set that contains *every* argument it defends (no acceptable argument is left out).
- **Grounded extension**: the smallest (w.r.t. set inclusion) complete extension. **Unique**, always exists — the maximally skeptical/conservative acceptance policy.
- **Preferred extension**: a maximal (w.r.t. set inclusion) admissible set. May be **multiple**, may disagree with each other — the credulous policy.
- **Stable extension**: a conflict-free set that attacks *every* argument outside itself. Strongest condition; does not always exist.

**(b) What it's true about.** Given only a structure of who-attacks-whom (content of the arguments abstracted away entirely), which *subsets* of arguments can be jointly, coherently accepted together.

**(c) Gap/uncertainty handling.** Handles disagreement, not just single-atom confidence: under preferred/stable semantics, there can be **multiple simultaneously valid extensions** — i.e., more than one internally-coherent "accepted worldview" over the same attack-graph, with no further fact of the matter forcing a choice between them at the framework's own level of abstraction. Grounded semantics gives the unique maximally-cautious answer when you want a single skeptical status instead.

**(d) Feature our atom-kinds would care about — the deepest structural find of this sweep.** Acceptance status here is **not intrinsic to an individual argument/atom** — it is a *global fixpoint property of the whole graph's current state*. The same argument can be "in" the grounded extension, "in" one preferred extension and "out" of another, etc. This is a fundamentally different design axis from every per-atom status ladder we've been assuming: if our atoms can attack/defeat each other, Dung's semantics is the mathematically battle-tested (30 years, entire subfield: COMMA conference, *Argument & Computation* journal) way to define "currently accepted" at the *collection* level — including a principled way to represent genuine, unresolved disagreement between coherent sub-communities of belief (multiple preferred extensions) rather than forcing a single global truth-value where none is warranted.

**(e) Provenance.** Dung, "On the acceptability of arguments and its fundamental role in nonmonotonic reasoning, logic programming and n-person games," *Artificial Intelligence* 77(2), 1995 — the founding, most-cited paper in computational argumentation (not fetched directly this pass — paywalled at ScienceDirect, https://www.sciencedirect.com/science/article/pii/000437029400041X). Fetch-verified this pass against Wikipedia's "Argumentation framework" article (https://en.wikipedia.org/wiki/Argumentation_framework), which is not itself a primary/canonical source but reproduces the standard formal definitions accurately enough to confirm the search-pass draft's structure was correctly recalled (I attempted the more authoritative redalyc.org survey PDF and SEP's argumentation entry directly; the PDF fetch returned unreadable binary and SEP's argument-and-argumentation entry's fetched excerpt cut off before the extension definitions — noted as a gap, not papered over). **Confidence: structure verified against a secondary source with correct technical content; the primary 1995 paper itself remains unfetched.**

---

## 6. Dempster–Shafer Theory of Evidence

**(a) Structure.** Three-function system over the full *power set* of the possibility space (not just individual outcomes):
- **Mass function m**: assigns belief-mass to *subsets* A of the frame of discernment, m(∅)=0, Σm(A)=1 over all A. Mass on a non-singleton subset represents evidence that doesn't discriminate among its members — genuine "don't know which" rather than "known to be 50/50."
- **Belief (Bel(A))**: sum of mass on all subsets of A — a lower bound, "definitely supported."
- **Plausibility (Pl(A))**: sum of mass on all subsets that merely intersect A — an upper bound, "not ruled out."
- Dempster's rule of combination merges independent evidence sources (has known pathologies under highly conflicting evidence — a documented open problem in the literature, not mentioned in the search-pass draft).

**(b) What it's true about.** Representing evidence about *sets of possibilities* without forcing it down into per-outcome point probabilities, explicitly so that "I don't know which of several options" can be represented differently from "I know it's evenly split."

**(c) Gap/uncertainty handling.** Pl(A) − Bel(A) is a literal, quantified **ignorance measure** — the width of the belief/plausibility gap is itself meaningful, not just the point values. This is the cleanest formal precedent found this sweep for "uncertainty about uncertainty" expressed as a *quantity* rather than a qualitative flag.

**(d) Feature our atom-kinds would care about.** A confidence field that is honestly an **interval** [Bel, Pl] rather than a point estimate, where the interval's width is itself a signal (wide = genuinely under-evidenced; narrow = well-evidenced, converging toward ordinary probability as Bel→Pl).

**(e) Provenance.** Dempster (1967), Shafer, *A Mathematical Theory of Evidence* (1976) — canonical originating text. Later extended by Smets (Transferable Belief Model). Sources cited in search pass: lecture notes citing Halpern & Voorbraak (http://www.blutner.de/uncert/Dempster-Shafer.pdf), ScienceDirect topic overview (secondary). **Recalled, not re-fetched this pass** — the mass/Bel/Pl formal shape is extremely standard and low-risk, but flagging per policy; I did not independently confirm against Shafer's 1976 text or a peer-reviewed encyclopedia entry.

---

## 7. Possibility Theory (Zadeh / Dubois–Prade)

**(a) Structure.** A dual-measure pair, non-additive (does *not* generalize probability's P + (1−P) = 1 constraint):
- **Possibility measure Π**: max-decomposable over unions, Π(A∪B) = max(Π(A), Π(B)) — "how unsurprising is A."
- **Necessity measure N**: min-decomposable over intersections, dual to Π via N(A) = 1 − Π(¬A) — "how certain are we A holds."

**(b) What it's true about.** Graded plausibility under *qualitative/linguistic* imprecision (vagueness) rather than frequency-based randomness — framed by its own proponents as a genuine sibling to probability theory, not a special case of it.

**(c) Gap/uncertainty handling.** Π and N can **both be low simultaneously** — "possible but not necessary" and "not impossible but not certain" can coexist, unlike probability where P and 1−P are forced to sum to 1. This gives a native representation of total ignorance ("we genuinely don't know") as structurally distinct from "50/50," using a max/min (ordinal) calculus rather than Dempster-Shafer's additive one — the search pass frames this as more apt when the underlying uncertainty is linguistic/vagueness-flavored rather than frequency-flavored, though I did not independently verify that framing-of-applicability claim against a primary source this pass.

**(d) Feature our atom-kinds would care about.** A second, calculus-distinct precedent (alongside D-S) for "possible ≠ certain, and the gap between them is meaningful" — useful if we ever need to justify *why* an interval/dual-measure design is principled rather than ad hoc, or if we want a max/min-flavored alternative to D-S's additive one for cases where the underlying uncertainty is about vagueness/definitional fuzziness rather than missing evidence.

**(e) Provenance.** Zadeh (1978, coining paper via fuzzy set theory); developed into the mature theory by Dubois & Prade (IRIT, Toulouse). Sources cited in search pass: Dubois & Prade's own retrospective survey (https://www.irit.fr/~Didier.Dubois/Papers1208/possibility-EUSFLAT-Mag.pdf — primary/authoritative, author-written), Scholarpedia entry (peer-reviewed encyclopedia, http://www.scholarpedia.org/article/Possibility_theory). **Recalled, not re-fetched this pass.**

---

## 8. Imprecise Probability (Walley) ✅ verified

**(a) Structure — fetched from SEP "Imprecise Probabilities" (2026-08 fetch):**
- **Lower and upper envelopes**: lower probability = inf P(X) over a set of candidate probability functions; upper probability = sup P(X); conjugate pair satisfying lower(X) = 1 − upper(¬X).
- **Credal set** ("representor"): the agent's belief state is represented not by a single probability function but by a *set* **P** of probability functions, each individually satisfying the standard probability axioms; the set collectively captures uncertainty about which precise probability is correct.
- **Risk vs. ambiguity distinction, confirmed via the fetched worked example**: a fair coin observed many times yields a **precise** P(H)={0.5} (evidential weight is high, so the credal set collapses to a single point); a coin of *unknown* bias yields P(H)=[0,1] (complete ignorance — the credal set is maximally wide). Both cases can have the *same point estimate* (0.5, if you had to name one number) under classical point-probability, yet the imprecise-probability model distinguishes them — captured, per the fetched text, in the line: "evidential precision begets attitudinal precision; evidential imprecision begets attitudinal imprecision."

**(b) What it's true about.** Generalizing single-number (point) probability to a *set* of live probability distributions when the evidence genuinely underdetermines a unique prior — the formal home of Knightian uncertainty (known-unknowns vs. the classical-probability assumption that every uncertainty reduces to a single number).

**(c) Gap/uncertainty handling.** The width of the [lower, upper] interval (equivalently, the size/spread of the credal set) is *itself* the ignorance signal — structurally the same move as Dempster-Shafer's Bel/Pl gap (and in fact Bel/Pl is a known special case of coherent lower/upper probability), but developed independently and with its own coherence axioms (avoiding sure loss, and its extension) playing the role Kolmogorov's axioms play for point probability.

**(d) Feature our atom-kinds would care about.** The **evidential-weight-vs-point-estimate distinction is the single cleanest, most citable formal case found this sweep** for why two atoms can share an identical "best guess" confidence number and still be honestly different in epistemic state — one backed by substantial converging evidence, one backed by none. If our schema ever collapses to a single confidence scalar, this system is the canonical formal argument for why that collapse loses real information, and the fair-coin/unknown-coin pair is a ready-made illustrative example.

**(e) Provenance.** Peter Walley, *Statistical Reasoning with Imprecise Probabilities* (1991) — canonical text, active research society (SIPTA). Fetch-verified this pass against SEP "Imprecise Probabilities" (https://plato.stanford.edu/entries/imprecise-probabilities/) — peer-reviewed reference encyclopedia, the formal-apparatus supplement is linked from the main entry. SIPTA's own summer-school intro (https://school18.sipta.org/Material/intro-ip.pdf) cited but not fetched. **Confidence: high — structure and the risk/ambiguity distinction are fetch-verified with a directly quoted worked example.**

---

## 9. Bayesian Epistemology

**(a) Structure.** Credence function c: propositions → [0,1] satisfying Kolmogorov's probability axioms; **Conditionalization** as the normative update rule (new credence = old credence conditioned on new evidence via Bayes' theorem); justified two ways in the literature — Dutch Book arguments (incoherent/non-probabilistic credences are provably exploitable by a clever bettor) and accuracy-based/epistemic-utility arguments (probabilistic credences are provably closer to the truth in expectation than non-probabilistic ones).

**(b) What it's true about.** The norms governing *graded* belief (a degree of confidence, not a binary belief-or-not) and how those degrees should rationally change in light of evidence. The dominant paradigm in formal epistemology for "how sure am I," and the implicit baseline that D-S / possibility theory / imprecise probability are all explicitly generalizing *away from*.

**(c) Gap/uncertainty handling.** Notably weak on this front by design — a point credence collapses "well-evidenced 0.5" and "totally unevidenced 0.5" into the same number (this is precisely the gap that motivated Walley's imprecise probability and Dempster-Shafer, above). Bayesian epistemology's own answer to "uncertainty about uncertainty" tends to be higher-order credences (a credence about one's own credence), which the literature treats as coherent in principle but has known regress/psychologizing worries.

**(d) Feature our atom-kinds would care about.** Worth holding as the **named baseline**: it's the single-point-estimate, always-conditionalize, everything-collapses-to-one-number design that the other uncertainty calculi in this sweep (D-S, possibility theory, imprecise probability) are each a documented, motivated departure from — useful for explaining *why* a richer confidence representation might be worth the complexity cost, by naming exactly what it buys over the Bayesian default.

**(e) Provenance.** SEP "Bayesian Epistemology" (https://plato.stanford.edu/entries/epistemology-bayesian/); Titelbaum, *Fundamentals of Bayesian Epistemology* (Oxford, 2022) — canonical modern textbook (https://academic.oup.com/book/41943, likely paywalled). **Recalled, not re-fetched this pass** — the credence/conditionalization/Dutch-Book shape is extremely standard, low novelty-risk.

---

## 10. AGM Belief Revision

**(a) Structure.** Three named operations on a logically-closed belief set K: **expansion** (K+A: add A, no consistency check, may become inconsistent), **contraction** (K−A: remove A, output stays logically closed, guided by a minimal-change principle — remove as little else as possible), **revision** (K*A: add A while removing just enough of K to preserve consistency, typically defined via the Levi identity as contract-then-expand). Governed by 6–8 rationality postulates (varies by presentation) organized around three principles: consistency-preservation, minimal change (informational economy), and priority to new information.

**(b) What it's true about.** How a *whole logically-closed set of beliefs* should rationally change when new, possibly-conflicting information arrives — not per-claim confidence grading, but wholesale set-level update/retraction with logical-closure and minimal-change constraints.

**(c) Gap/uncertainty handling.** Doesn't grade uncertainty at all — it's binary (in K or not in K) but handles the *cascading* consequence of retraction: because K is logically closed, removing one belief may force removing others that depended on it, and the minimal-change postulates exist specifically to constrain how much collateral retraction is rationally required versus gratuitous.

**(d) Feature our atom-kinds would care about.** Relevant if our atoms have logical/inferential dependencies on each other and a retraction needs to cascade in a principled (minimal-change) rather than ad hoc way — a genuinely different concern from every confidence-calculus above, closer to a "what else has to give" consistency-maintenance system than an "how sure are we" system.

**(e) Provenance.** Alchourrón, Gärdenfors & Makinson, "On the Logic of Theory Change," 1985 — the founding paper, standard model in the belief-revision/AI (KR, NMR) literature. Secondary: Franz Huber (U. Toronto), widely-used graduate exposition (https://huber.artsci.utoronto.ca/wp-content/uploads/2013/07/Belief-Revision-I-The-AGM-Theory.pdf). **Recalled, not re-fetched this pass.**

---

## Which look most load-bearing for verisectorium (carried from search pass, unchanged after verification)

1. **Pollock's rebutting/undercutting defeaters** (§3, verified) — cleanest, most directly portable two-kind defeat-relation taxonomy, already in exactly our vocabulary (defeasible, prima facie, undefeated-relative-to-evidence).
2. **Dung's AAF grounded/preferred/stable semantics** (§5, verified) — the deepest structural find: acceptance as a global graph-fixpoint property, not an intrinsic per-atom field, with a principled way to represent multiple simultaneously-coherent worldviews.
3. **Toulmin's model** (§4, recalled) — best template for what fields a single atom's epistemic-state record should contain, with Qualifier and Rebuttal as first-class self-reported slots.
4. **Dempster-Shafer's Bel/Pl gap** (§6, recalled) — cleanest quantified answer to "uncertainty about uncertainty" as an interval rather than a point.
5. **Internalism/externalism** (§2, recalled) — the design fork of where an atom's justification lives (inline evidence vs. provenance/process trust).

**Newly load-bearing after this verification pass:** Walley's **imprecise probability risk-vs-ambiguity distinction** (§8, verified, with the fair-coin/unknown-coin worked example) deserves to sit alongside Dempster-Shafer on the "final five" list — it's the most directly citable, fetch-confirmed formal argument for why a single confidence scalar can silently discard real epistemic information (same point estimate, different evidential weight), and it comes with SEP's own ready-made illustrative example rather than requiring us to construct one.

---

## What surprised me / what we did not ask about

- **The Dung fetch attempt itself was a small, honest failure worth recording rather than hiding.** I tried the two most-authoritative-looking sources first — the redalyc.org open survey PDF (returned unreadable binary/encoded garbage) and SEP's own "Argument and Argumentation" entry (the fetched excerpt introduced Dung's attack relation but cut off *before* reaching the extension definitions). Only the Wikipedia fallback actually surfaced the formal definitions, and Wikipedia is not a canonical source even though its content checked out correctly against my recollection. This is exactly the "distinguish canonical from blog-grade" instruction cutting against convenience — the most cited foundational paper in an entire subfield (30 years, its own conference) was the hardest of the ten to get primary-verified text for in this session, and I'm flagging that gap rather than quietly upgrading Wikipedia to "verified against primary."

- **Nobody in this list is actually in the business of grading a single atom's confidence in isolation once you get past Bayesian credence.** Every richer system found — Dung, AGM, Toulmin's Rebuttal slot, even Pollock's defeaters — is fundamentally about **relations between claims** (attacks, defeats, dependencies, licensed inferences), not properties of one claim taken alone. If verisectorium's atoms are going to carry rich epistemic state, the state that matters most in the literature surveyed here is edge-shaped (this-atom-attacks/undercuts/depends-on-that-atom), not node-shaped (this-atom-has-confidence-0.7). That's a genuinely different design center of gravity than "status ladder per atom," and it wasn't something the original prompt's framing (ladders/rungs/axes, singular) obviously pointed toward — I noticed it only by laying the ten systems side by side.

- **Dung's semantics gives us formal permission for the collection to honestly disagree with itself.** Multiple preferred extensions isn't a bug or an unresolved-TODO state in AAF theory — it's the framework saying, correctly, that the attack-graph as given *does not determine* a single answer, and picking one anyway would be adding information that isn't there. If verisectorium ever wants "the collection currently holds two live, mutually-incompatible-but-individually-coherent positions on X, and that is not an error" as a first-class, non-degenerate state (rather than something to be flagged for resolution), Dung's preferred semantics is the closest thing to off-the-shelf formal legitimacy for that.

- **The risk-vs-ambiguity coin example (Walley/imprecise probability, §8) is unexpectedly good raw material for explaining the whole "uncertainty about uncertainty" idea to a skeptical reader**, because it produces two states that are numerically identical (P(H)=0.5-ish either way) under the naive model and yet obviously different in how much you'd trust them — that's a two-line illustration, not an abstract argument, and might be worth stealing directly for whatever exposition eventually explains why our own confidence field can't just be a single number.

- **What we did not ask about, and probably should:** none of the ten systems here is a *record-keeping* or *documentation* ontology — they are all reasoning/belief calculi (about a mind's or a system's internal epistemic state), not about how a *document or archival record* declares and preserves its own provenance and confidence over time for later readers who weren't there when it was made. That's a different literature (archival science, records management, provenance standards like PROV-O, diplomatics) and if verisectorium's atoms are meant to be durable artifacts read by future strangers rather than live beliefs held by a current reasoner, that gap may matter more than any single system inside this sweep. Flagging it rather than reaching outside my assigned domain to fill it.

- **Also unasked: nothing here addresses multi-agent/social epistemology** — whose testimony counts, how trust in a *source* (as opposed to a claim) is formally modeled, peer disagreement, epistemic communities. Evidentialism/reliabilism (§2) gestures at "process reliability" but stops short of a worked social-epistemology system. Given verisectorium's explicit premise of 100% agent turnover and provenance-carrying atoms, "how much do we trust the *agent* who asserted this, independent of the claim's own content" is squarely relevant and squarely outside this domain's sweep as scoped.
