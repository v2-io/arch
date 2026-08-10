# Sweep: Authority, Deontic Force, and Speech-Act Ontologies

*Deepened from the search-pass findings for the "authority / deontic / speech-act" territory. Every system below was fetched from a primary or near-primary source and checked against the search pass's claims; where fetches failed or a claim could not be independently confirmed, that is marked explicitly rather than silently upgraded to "verified." Written for truth, not for convenience to our current taxonomy — see the closing section for what doesn't fit.*

---

## 1. Searle/Austin speech-act taxonomy

**(a) Structure, as the sources state it:**

Austin (*How to Do Things with Words*, 1962) distinguishes three layers of what an utterance does: the **locutionary act** (uttering with sense and reference), the **illocutionary act** (what the utterance *counts as* — asserting, promising, warning), and the **perlocutionary act** (the effect produced in the hearer — persuading, alarming). Austin's own attempt at classifying illocutionary acts (verdictives, exercitives, commissives, behabitives, expositives) was, per SEP, criticized by Searle as "unduly lexicographic" (grounded in English verbs rather than principled distinctions).

Searle's replacement taxonomy (*"A Classification of Illocutionary Acts,"* published 1976 in *Language in Society*, sometimes cited as 1975/1977 depending on venue — I could not pin the exact year with full confidence; see confidence note below) sorts illocutionary acts by three simultaneous dimensions — **illocutionary point**, **direction of fit**, and **expressed psychological state** — into five basic classes:

- **Assertives / Representatives** — represent a state of affairs (stating, claiming, hypothesizing, describing, insisting, suggesting, swearing that). **Direction of fit: word-to-world.** Truth-apt; speaker is committed to the truth of the expressed proposition.
- **Directives** — attempts to get the hearer to do something (ordering, commanding, requesting, daring, challenging). **Direction of fit: world-to-word**, but the fit is achieved via the *hearer's* subsequent action, not the speaker's.
- **Commissives** — commit the speaker to a future course of action (promising, vowing, offering). **Direction of fit: world-to-word**, but the obligation falls on the *speaker*.
- **Expressives** — express a psychological state about a presupposed state of affairs (thanking, congratulating, apologizing, welcoming). **Direction of fit: null** — Austin's SEP-summarized example: thanking has "neither" a word-to-world nor world-to-word direction, because the state of affairs is presupposed true, not asserted or sought.
- **Declarations** — bring about the state of affairs they name, by virtue of the speaker's institutional standing (pronouncing a couple married, declaring war, firing an employee, christening a ship). **Direction of fit: word≡world (both simultaneously)** — the utterance succeeds only if, and precisely because, saying it makes it so.

Austin's **felicity conditions** are the non-truth-valued gap/failure apparatus riding alongside this taxonomy. The SEP entry (fetched) states two failure classes, not the three "preparatory/sincerity/essential" rule-types the search pass asserted as directly quoted — I could **not** verify the preparatory/sincerity/essential three-part rule split from a primary source in this pass (see confidence note):

- **Misfires** — "cases in which the putative speech act fails to be performed at all" (e.g., I "declare you married" but I have no authority to do so — nothing happens, the declarative act simply doesn't occur).
- **Abuses** — the act *is* performed, but "fail[s] to live up to a standard appropriate for speech acts of its kind" (e.g., I promise insincerely — the promise is made, but defectively; I am now on the hook for an obligation I didn't mean to take on).

This misfire/abuse split is itself a strong structural candidate for us: it distinguishes **"this record never came into being as the kind it claims"** (misfire — a Decision entry with no one having standing to decide) from **"this record came into being but is defective in a way that doesn't erase it"** (abuse — an insincere promise is still a promise, still binds).

**(b) What it's trying to be true about:** what an utterance *does*, institutionally and normatively, independent of and prior to whether its propositional content is true. This is the deepest-available answer to "what force does this atom carry," and is very likely the direct ancestor (via decades of downstream borrowing in linguistics, NLP, legal theory, and speech-act-informed software ontologies) of our Assertions/Definitions/Decisions/Norms&Directives split.

**(c) Uncertainty/gap handling:** felicity, not truth-value. A speech act can be perfectly clear and perfectly *unhappy* — this is a genuinely orthogonal axis to true/false, and is likely the cleanest available academic precedent for a "was this atom validly brought into being as its declared kind" check distinct from "is its content accurate."

**(d) Feature our atom-kinds would care about:** the direction-of-fit triple (word-to-world / world-to-word / word≡world / null) is a compact, principled way to distinguish exactly the families we're already trying to distinguish by hand — Assertions (word-to-world), Norms & Directives (world-to-word, hearer-obligated), Decisions (word≡world, if institutionally licensed — a Decision entry is structurally a Declaration), and something like a Commissive gap we may not currently have a family for (records that commit *the author/project* to a future course of action — is that a Practice? a Decision? neither cleanly).

**(e) Provenance + confidence:**
- SEP "Speech Acts" (fetched, primary-adjacent — SEP entries are peer-reviewed academic summaries, not primary Austin/Searle text): https://plato.stanford.edu/entries/speech-acts/ — **verified via fetch** for locutionary/illocutionary/perlocutionary, direction-of-fit definitions, and misfire/abuse.
- Searle's five-category names, definitions, and "illocutionary point / direction of fit / expressed psychological state" organizing triad: **verified via WebSearch synthesis of multiple secondary sources** (Fiveable study guide, UCSD linguistics course notes, an academia.edu PDF, and a hosted copy of the original 1976/77 paper at eclass.uoa.gr) — I did not fetch full text of any one primary source for this half, so treat the five-category *definitions* as well-corroborated-secondary rather than primary-verified. The exact publication year (1975 "Indirect Speech Acts" vs. 1976 "A Classification of Illocutionary Acts" in *Language in Society* vs. sometimes-cited 1977 reprint) is genuinely inconsistently cited across sources I saw — **not resolved**, flagging rather than picking one.
- Original paper (hosted PDF, not independently fetched this pass): https://eclass.uoa.gr/modules/document/file.php/MEDIA221/... (Searle, "A Classification of Illocutionary Acts," *Language in Society* 5(1), 1976).
- Felicity-condition preparatory/sincerity/essential three-part rule structure (asserted by the search pass): **not verified this pass** — the felicity-conditions page (lel.ed.ac.uk, since moved to pragmatics.ppls.ed.ac.uk) redirected and was not re-fetched; SEP's own treatment gave only the misfire/abuse split plus a passing mention that "sincerity" is "a paradigm condition." Treat the fuller three-rule-type breakdown as **recalled from the search pass, not verified by me** — worth a follow-up fetch of the redirected URL (https://pragmatics.ppls.ed.ac.uk/~hannah/plc/speech-acts-3felicity.html) or Austin's *How to Do Things with Words* Lecture II directly if this matters for the final model.

---

## 2. Deontic logic (Standard Deontic Logic, SDL) and the contrary-to-duty paradox

**(a) Structure, as SEP states it (fetched):**

SDL is a modal logic (system KD) built on a single primitive operator, **OB** ("it is obligatory that"), from which the others are defined:

- `PE p` (permissible) `:= ¬OB¬p`
- `IM p` (impermissible) `:= OB¬p`
- `OM p` (omissible) `:= ¬OBp`
- `OP p` (optional) `:= ¬OBp ∧ ¬OB¬p`

Core axioms/rules, per the fetch:
1. **K** — `OB(p→q) → (OBp → OBq)` (obligation distributes over entailment).
2. **NC** (no-conflicts / consistency) — `OBp → ¬OB¬p` (you cannot be obligated to both p and not-p).
3. **Necessitation** — if `p` is a theorem, then `OBp` is a theorem.

This is a genuine modal system with a semantics (standardly, quantification over a set of "ideal" / deontically-perfect worlds relative to the actual one) — not a flat vocabulary. It gives deontic status real *inferential structure*: obligations entail other obligations, permission and obligation are interdefinable, and — crucially — the system is formally required to be **consistent** (NC), which is exactly what breaks under real-world normative conflict.

**The contrary-to-duty (Chisholm) paradox**, verified via fetch: four natural-language statements that sound perfectly consistent —
1. Jones ought to assist his neighbors. (`OB g`)
2. If Jones goes to assist, he ought to tell them he's coming. (`OB(g→t)`)
3. If Jones doesn't go, he ought not tell them he's coming. (`¬g → OB¬t`)
4. Jones doesn't go. (`¬g`)

— formalize under SDL into a derivation of both `OB t` and `OB ¬t` simultaneously, which directly violates axiom NC. The system that is *supposed* to be the formal home of "what ought to happen" cannot coherently represent "what ought to happen, given that a primary obligation was already violated." This is not a fringe curiosity — it's been the central open problem driving 70+ years of successor systems (Åqvist's system E, dyadic/conditional obligation logics, Carmo & Jones's systems for contrary-to-duty structures), because *most real normative situations involve exactly this shape*: a violated primary obligation generating a distinct, still-binding secondary obligation.

**(b) What it's trying to be true about:** the normative status (obligatory/permitted/forbidden) of actions or propositions, and what follows logically from a set of such statuses.

**(c) Uncertainty/gap handling — the load-bearing part:** SDL's own axioms make it formally **incapable** of representing "ought" claims that persist across a violation without contradiction. The field's response has not been to patch SDL with an exception-handling bolt-on but to build **dyadic/conditional** deontic logics from the ground up — obligation is relativized to a context (`OB(t | g)`, roughly "t is obligatory *given* g"), which dissolves the paradox by making the two "oughts" in Chisholm's example conditional on mutually exclusive antecedents rather than both categorical. This is a genuinely different design pattern from "flag the record as degraded" — it's "represent the obligation as always relative to a specified prior state, so a change in prior state changes which obligation is live, without contradiction."

**(d) Feature our atom-kinds would care about:** the contrary-to-duty structure is close to isomorphic to a pattern we plausibly need — "given that norm N was violated (or a Decision was overturned), what is the *next* obligatory state, distinct from what would have been obligatory had N held?" If our Norms & Directives / Decisions families ever need to represent "this is what's now required given the prior thing having already failed," the dyadic-obligation move (index every obligation to the state it's conditional on, not just categorically) is the field's hard-won answer, not an ad hoc status flag.

**(e) Provenance + confidence:**
- SEP "Deontic Logic" — **fetched and verified**: https://plato.stanford.edu/entries/logic-deontic/ for the operator definitions, NC/K/Necessitation axioms, and the Chisholm-paradox formalization and its NC-violation diagnosis.
- Von Wright (1951), "Deontic Logic," *Mind* 60(237) — founding paper, cited by SEP, **not independently fetched** (paywalled academic journal; treat as recalled/secondary, standard attribution).
- Survey of paradoxes (search-pass source, not re-fetched this pass): https://icr.uni.lu/leonvandertorre/papers/normas07b.pdf — flagging as **not verified this pass**, carried forward from search findings.
- Note on the search pass's earlier framing: the search pass named the paradox after Ross and the free-choice-permission problem too; I focused verification effort on Chisholm's contrary-to-duty case specifically because it's the one most structurally relevant to "gap/violation handling." Ross's paradox and free-choice permission are real and well-attested in the deontic-logic literature generally but were not independently re-verified by fetch this pass.

---

## 3. Legal precedent strength: ratio decidendi / obiter dicta, binding vs. persuasive authority

**(a) Structure, as sources state it (fetched, Wikipedia "Obiter dictum," cross-checked against the Georgetown PDF's existence though its text wasn't extractable):**

- **Ratio decidendi** — "the crucial facts and law of the case" — the part of a judicial opinion actually necessary to reach the holding. **Binding** on courts lower in, or bound by, the deciding court within the same hierarchy.
- **Obiter dictum** ("said in passing") — everything else in the opinion: hypotheticals, tangential legal discussion, remarks not necessary to the outcome. **Persuasive only**, never binding, in any jurisdiction — but "persuasive" is itself graded, not flat (the search pass's characterization — "from simple passing remarks... to statements that have been fully argued" — captures a real, commonly-discussed gradient in the secondary literature, though I did not independently re-verify that exact phrasing against a primary source this pass).
- **Wambaugh's Inversion Test**, verified via fetch, is the accepted operational test for classifying a given passage: "ask whether the decision would have been different, had the statement been omitted. If so, the statement is crucial and is ratio; whereas if it is not crucial, it is obiter." This is a genuinely mechanical, falsifiable procedure for a distinction that could otherwise be hand-wavy — worth noting as a rare case in this whole territory where "how do you actually decide which bucket this goes in" has a named, citable test.
- **Court hierarchy is the orthogonal second axis.** Binding-ness of a *ratio* is never absolute — it's binding only *on courts positioned below or level-with the deciding court in the relevant hierarchy* (a UK Court of Appeal ratio doesn't bind the Supreme Court; a US Ninth Circuit ratio doesn't bind the Fifth Circuit; neither binds a foreign jurisdiction at all, where it would be merely persuasive regardless of how central it was to the holding). Confirmed via fetch: in the UK, "obiter dicta are not binding, although in some jurisdictions, such as England and Wales, they can be strongly persuasive" — and separately, US Supreme Court dicta are noted as "particularly influential despite technical non-binding status," i.e., the source-authority axis can partially compensate for content-type even though it can never fully convert dictum into binding holding.

**(b) What it's trying to be true about:** which parts of a past decision constrain future decisions, and how strongly — a genuinely two-axis system: **content-type** (was this necessary to the holding?) × **source-authority** (what is the deciding body's institutional position relative to the one asking?).

**(c) Gap/uncertainty handling:** obiter dicta are not treated as false, defective, or lesser truth-claims — they are explicitly a *different modal category of statement*, persuasive-not-binding, and the system has a working test (Wambaugh's) for sorting content into the two categories rather than leaving it to intuition. This is a clean precedent for a "non-load-bearing but still true and still worth carrying" status distinct from either "false/retracted" or "binding/authoritative."

**(d) Feature our atom-kinds would care about:** the two-axis grid (content-necessity × source-hierarchy-position) is a genuinely different shape from either the speech-act taxonomy (single act-type classification) or deontic logic (a modal-logical status). It directly answers a design question our Accounts/Decisions families likely have implicitly and unresolved: an Account or Decision atom's authority is not just "who said it" (our current Authority axis) but *how necessary was this specific claim to the thing it's embedded in*, and *what's the position of the source relative to the reader's context* — both needed, neither alone sufficient.

**(e) Provenance + confidence:**
- Wikipedia, "Obiter dictum" — **fetched and verified** for definitions, Wambaugh's test, and the UK/US jurisdictional notes: https://en.wikipedia.org/wiki/Obiter_dictum
- Georgetown Law (Schafer), "Federal Law, Federal Courts, and Binding and Persuasive Authority" — **fetch failed** (PDF binary, not text-extractable by the fetch tool; downloaded but unreadable this pass). This was the search pass's primary citation for this system and I could not independently confirm its content beyond what Wikipedia corroborates. Flagging explicitly: this is the single most load-bearing unverified citation in this sweep. URL for a future pass: https://www.law.georgetown.edu/wp-content/uploads/2018/07/Matthew-Schafer-FederalLawFederalCourtsandBindingandPersuasiveAuthority.pdf
- Open University, "Judicial decision making" (search-pass source, not independently re-fetched): https://www.open.edu/openlearn/society-politics-law/law/judicial-decision-making/content-section-9.1

---

## 4. Hohfeld's fundamental legal conceptions

**(a) Structure, as fetched (Wikisource digest of Hohfeld 1913/1917):**

Hohfeld organizes eight legal concepts into two tables describing the *same* underlying jural relations from two different angles:

**Table of Jural Opposites** (contrasting positions of the *same* party):
| Right | No-right |
|---|---|
| Privilege | Duty |
| Power | Disability |
| Immunity | Liability |

**Table of Jural Correlatives** (the reciprocal relation *between two parties*):
| Right | Duty |
|---|---|
| Privilege | No-right |
| Power | Liability |
| Immunity | Disability |

Definitions, as fetched:
- **Right / Duty** — A holds a claim against B; B correlatively bears a duty to A.
- **Privilege / No-right** — A has liberty to act (no duty *not* to); correlatively B has no claim against A doing it.
- **Power / Liability** — A has the capacity to *alter* legal relations (e.g., by contracting, revoking, promulgating); B is correlatively subject (liable) to having their relations so altered.
- **Immunity / Disability** — A is protected from having their legal position altered by B; B correlatively lacks the power to do so.

The fetch also surfaced a genuine internal controversy: **Kocourek's critique** "challenges whether all these pairings function as true logical opposites and questions the practical utility of some categories, particularly the privilege-no-right and immunity-disability distinctions" — i.e., this is not an uncontested, universally-accepted eight-way split even within analytic jurisprudence; it's the standard teaching tool with known soft spots, not an axiom system nobody has ever questioned.

**(b) What it's trying to be true about:** the relational, correlative structure underlying *any* claim of legal right or authority between two parties — and crucially, it factors that structure into two orders: **first-order** relations (right/duty, privilege/no-right — static normative facts about what must/may happen) and **second-order** relations (power/liability, immunity/disability — dynamic facts about *who can change* the first-order facts, and who is protected from having them changed).

**(c) Gap/uncertainty handling:** Hohfeld's system doesn't itself have a stated uncertainty-handling mechanism (it's a static relational ontology, not a logic of inference or a theory of evidence) — its contribution to "gap handling" is indirect: by cleanly separating *having an obligation* from *having the power to create/alter obligations*, it makes visible a distinct failure mode our system might otherwise conflate — a record can be defective because its **content** is wrong (a first-order problem) or because its **author lacked the power** to bring it into being at all (a second-order problem, structurally identical to Austin's "misfire").

**(d) Feature our atom-kinds would care about:** the power/liability layer is the cleanest available formalization of a distinction our Decisions/Norms&Directives boundary is almost certainly missing explicitly — a Decision that **exercises** existing authority (first-order: "given the standing rule, X is decided") versus a Decision that **creates or alters** authority itself (second-order: "this record is itself what makes future decisions of this shape possible/legitimate," e.g. establishing who gets to decide things at all). Our current Authority axis conflates "how firmly is this backed" with "does this record's author have the standing power to have produced this kind of record in the first place" — Hohfeld suggests those may need to be two different questions.

**(e) Provenance + confidence:**
- Wikisource, "The Hohfeld System of Fundamental Legal Concepts" — **fetched and verified**, including the Kocourek critique: https://en.wikisource.org/wiki/The_Hohfeld_System_of_Fundamental_Legal_Concepts
- Original: Hohfeld, "Some Fundamental Legal Conceptions as Applied in Judicial Reasoning," *Yale Law Journal* 23 (1913) and 26 (1917) — **not independently fetched** (the Wikisource page is a digest/summary of the primary text, not the primary text itself; treat table contents as verified-via-reliable-secondary rather than primary-verified).
- SEP "Rights" (search-pass source, situates Hohfeld in broader context) — **not re-fetched this pass**: https://plato.stanford.edu/entries/rights/

---

## 5. Robert's Rules of Order — motion classification and precedence

**(a) Structure, as fetched (Wikipedia, since robertsrules.org returned 403):**

Four classes of motions:
- **Main motions** — introduce new business; lowest precedence (yield to everything else pending).
- **Subsidiary motions** — modify or dispose of the pending main motion: postpone indefinitely, amend, commit/refer, postpone to a certain time, limit/extend limits of debate, previous question (force an immediate vote), lay on the table.
- **Privileged motions** — urgent, unrelated to the pending business, highest precedence, mostly undebatable: call for the orders of the day, raise a question of privilege, recess, adjourn, fix the time to which to adjourn.
- **Incidental motions** — arise directly out of, and must be settled before, the pending question that spawned them: point of order, appeal, suspend the rules, division of a question, and various requests/inquiries.

The book presents (per the fetch) a "natural order of motions (from lowest to highest rank)" and a "Chart for Determining When Each Subsidiary or Privileged Motion Is In Order" — confirming a genuine **precedence ordering**, not just a flat four-way category split. I was **not** able to extract the exact numeric/rank table itself this pass (robertsrules.org 403'd; Wikipedia's summary describes the existence and function of the precedence chart but doesn't reproduce it verbatim) — this is the weakest-verified system in this sweep structurally, even though its existence and basic four-class taxonomy is solid.

**(b) What it's trying to be true about:** the procedural status of a proposal as it moves through a deliberative body — which things can currently be voted on, which things must be resolved first, and what happens to a main motion while other motions are pending on top of it.

**(c) Gap/uncertainty handling:** not really an epistemic-uncertainty system at all — it's a **process-completeness** system. Its analog to "uncertainty about uncertainty" is closer to "what is still open, and in what order must the open things close" — a genuinely different problem than truth-value gaps, closer to dependency-resolution or a call-stack.

**(d) Feature our atom-kinds would care about:** if verified in full, the precedence stack (subsidiary/incidental motions can pile on top of a pending main motion, each of which must resolve, in a specific enforced order, before the ones below it can proceed) is a strong structural candidate for modeling dependency-ordered open Questions or pending Decisions — a genuinely different pattern from either the speech-act taxonomy (act classification) or the ratio/obiter grid (two-axis authority weighting). I'd flag this as promising but **under-verified relative to its apparent value** — worth a follow-up fetch of the actual precedence table before leaning on it structurally (e.g. via the Internet Archive copy of the 1915 Robert's Rules revised edition, which is public domain and should be fetchable even if robertsrules.org blocks scrapers).

**(e) Provenance + confidence:**
- Wikipedia, "Robert's Rules of Order" — **fetched**, gave the four-class breakdown and confirmed the existence of a precedence ordering and a determining chart, but not the chart's contents: https://en.wikipedia.org/wiki/Robert%27s_Rules_of_Order
- robertsrules.org (primary text host) — **fetch blocked, HTTP 403**. Both attempted URLs (rror-02.htm and rulesintro) failed. This is a real gap, not a resolved one.
- Original: Henry M. Robert, *Robert's Rules of Order* (1876; 12th ed. current, Robert's Rules Association) — **not independently fetched**.

---

## 6. RACI / delegation-of-authority matrices (carried forward, not re-verified this pass)

Not re-fetched independently this pass; carried forward from the search findings as **lower-confidence, professional-practice-grade** evidence rather than a load-bearing academic system. The search pass's own framing — that the existence of RACI variants (RASCI, DACI, RAPID, CAIRO) shows the field treats the role-split as negotiable rather than settled — is a reasonable inference from the *existence* of variants, but I did not verify it against a primary management-science source. Recommend treating this system as **illustrative real-world instance of Hohfeldian power/liability** (who has the power to make a decision stick, who is merely liable to be bound by it) rather than as an independent ontology worth grounding our taxonomy in directly.

---

## What surprised me / what we did not ask about

**The felicity-conditions gap is the single most valuable unresolved thread.** I could verify Austin's misfire/abuse *split* but not the fuller preparatory/sincerity/essential *rule-type* structure the search pass had asserted. That fuller structure, if it holds up under a proper fetch of Austin's Lecture II (or the felicity-conditions page once its redirect is followed), is likely to matter a great deal to us: "preparatory conditions" (does the speaker/author have the standing to perform this act at all) map directly onto our Authority axis's *ceiling* concept — you cannot even *attempt* certain kinds of atoms without prior standing, independent of how well-evidenced their content later turns out to be. This deserves a dedicated follow-up fetch before the coordinator finalizes anything that leans on felicity conditions.

**Deontic logic's own internal admission of defeat is more valuable to us than its successes.** The fact that the *field's own textbook problem*, 70+ years running, is "how do you represent an obligation that persists honestly across a violation of a prior obligation, without the system collapsing into contradiction" is closer to a direct answer to Joseph's framing than anything a working, non-paradoxical system would have given us. A system with no paradoxes wouldn't have taught us anything about gap-handling; the paradox itself, and dyadic/conditional-obligation logic's response to it (index every "ought" to what it's conditional on, don't let it float categorically) is the actual transferable idea.

**We never asked about second-order authority (the Hohfeld power/liability axis) at all**, and I now think that's a real hole in the working taxonomy rather than a nice-to-have. Our Authority axis (rank 1: `proposed`... presumably rising through ratified states) appears to measure *how firmly backed* a record's content is — a first-order, Right/Duty-shaped question. It does not appear to separately ask *whether the record's author had the standing power to produce a record of this kind at all* — a second-order, Power/Liability-shaped question, and Austin's "misfire" is the speech-act-theory version of exactly the same gap. Three independent systems (Hohfeld, Austin, and implicitly the ratio/obiter court-hierarchy axis) converge on treating "who has the power to act in this way" as orthogonal to "how strong is the content once acted." That convergence across three unrelated academic traditions (jurisprudence, philosophy of language, common-law doctrine) feels like real signal, not coincidence — three fields independently needed the same second axis.

**We asked "how do established systems handle gaps" and got two genuinely different kinds of answer**, worth keeping distinct rather than collapsing: (1) felicity/misfire-abuse and Hohfeldian power/liability answer *"was this record validly brought into existence as its declared kind at all"* — a **validity-of-origin** question; (2) contrary-to-duty deontic logic and ratio/obiter answer *"given that this record's content is real and valid, how much weight does it carry, and what should carry the weight when the primary thing it depends on fails"* — a **weight-under-degradation** question. Our current single Authority axis may be trying to do both jobs at once.

**Robert's Rules is the weakest link in this sweep** — genuinely promising shape (a real precedence stack, not just a category split) but I hit a wall (403) trying to verify its actual mechanics, and I'm flagging rather than papering over that with search-pass recall. If the coordinator wants the dependency-ordering pattern for open Questions/pending Decisions, this needs one more fetch pass (Internet Archive copy of the public-domain 1915 edition is the likely unblocked route) before being treated as verified.

**Not covered here, and possibly should have been:** contract law's distinction between conditions precedent/subsequent/concurrent (a temporal-gating structure that might bear on our Freshness/Ceiling axes); the philosophy-of-science literature on defeasible reasoning and non-monotonic logic (which is the AI/formal-epistemology-side cousin of the deontic-logic gap-handling problem, and might have a cleaner "uncertainty about uncertainty" formalization than deontic logic does); and canon law / ecclesiastical authority hierarchies (which the steward's own domain vocabulary — "ratified," "steward" — gestures toward, and which historically predates and likely informs both the legal-precedent and deontic-logic traditions surveyed here). None of these were in scope for this pass but each seems like a plausible next sweep.
