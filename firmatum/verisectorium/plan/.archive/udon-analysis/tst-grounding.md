<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: MOVED/udon/v2/theory/to-integrate/primary/tst-grounding.md (Appendix A: corrections to 2026-07-23 generalization note)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/theory/to-integrate/primary/tst-grounding.md
  Do not edit here expecting to update the live original.
-->

# TST grounding — what the theory actually carries for the 2026-07-28 design questions

**Status:** exploration report, 2026-07-28. Commissioned by Joseph after a day of design work
that invoked TST as its grounding without anyone opening it. This is a *reading* of the
theory against four named questions, not a ruling and not a spike result. Nothing here is
ratified; where I synthesize, I say so.

**One correction to the commission's premise, found during the work.** "Nobody opened the
theory" is true of the design questions (the O-thoughts, the generalization note, today's
seeds) but **not** of the estate as a whole: `udon-needs/02-tooling-needs/reports/theory-of-agentic-tooling.md`
did a genuine survey pass over the live TST segment set. The real situation is more specific
and more fixable — see §8, with counts.

**Register.** *Evidenced* = a TST/AAT segment says this, cited, with the segment's own
epistemic tier reported (ASF's tiers: exact / conditional / robust-qualitative / empirical /
discussion-grade — I do not upgrade them). *Proposed* = my synthesis, generated here,
decides nothing. *Open* = named absence. Where a claim is mine I say "I read this as" rather
than asserting it in the theory's voice.

**Coverage.** I read all 29 current TST segments in `~/src/arch/asf/02-tst-core/src/` whole
(the 43 `old-tst-*` archaeology files I did not read), plus the TST `OUTLINE.md`, plus AAT's
`#def-adaptive-tempo`, `#der-multi-timescale-stability`, `#der-tempo-composition`, and
`#der-temporal-nesting`. A parallel reader surveyed the `udon-needs/02-tooling-needs/` corpus
(§8); I verified its load-bearing counts and quotes against the files myself, and marked what
remains second-hand. The single-author caution applies throughout and is why I have tried
to report *where the mathematics does and does not carry the load*, which is the only thing a
fresh reader can add that agreement cannot.

---

## 0. Conclusions

| Question | Verdict |
|---|---|
| **Q1 — payment-time / the four whens** | TST has **no schema, constraint, or check apparatus at all** (verified by grep across all current segments; the sole mention is `#def-atomic-changeset` listing schemas as part of "codebase"). But it carries three pieces that compose into a real answer, and one of the four whens — *when the check runs* — is already formalized elsewhere as an observation channel with a rate and a noise level. **The four-whens decomposition is not wrong; it is unbuilt, and TST supplies a sharper asymmetry the adage misses.** §3. |
| **Q2 — contraction and expand-contract** | O4's first half (renaming toward ubiquitous vocabulary is principled practice) is **evidenced, near-verbatim** — `#hyp-conceptual-alignment` carries a *Realignment as Feature* corollary and its worked example is Joseph's example. But the segment is **`discussion-grade`**, so "from first principles" overstates the tier. O4's second half (contraction's absence is tooling failure) is **not stated in TST but is derivable from it in three steps**, and I give the derivation. §4. |
| **Q3 — strata on different clocks** | **The apparatus is real, exact, and better than the Kelvin–Helmholtz picture — and it is one named premise short of transferring.** `#der-multi-timescale-stability` is `status: exact`, gives a closed-form separation threshold, and *inverts* Part 5's framing: rate difference is what makes stacking stable, not what drives mixing. Its own Working Notes name the gap that blocks document strata — discrete/jump dynamics. So Part 5 is neither free assembly nor metaphor: it is **assembly modulo a premise AAT has already put on its own open list**. This is the most valuable finding here. §5. |
| **Q4 — co-location and coupling** | Part 4's chain holds, and `#der-change-proximity-principle` *is* the missing correction — it grades the layout menu as a proximity ordering. But the fuller corpus complicates the chain in two ways nobody has noticed: the coupling estimate that justifies the layout is **confounded by the layout** (TST names this confounder class by name), and **"the file is the atomicity unit" inverts TST's unit** — TST's atom is the changeset, and the file is a proximity container measured *against* it. §6. |
| **Q5 — honest inventory** | The largest finding is a **frame nobody ran**: TST self-describes as a *calibration laboratory* whose results transfer only when the analyst names which of six epistemic properties the target domain has. Documents hold four of the six. **The two they lack (P2, P3) are exactly the ones a schema checker manufactures** — which reframes the udon schema work from "applying TST" to "repairing TST's identification conditions in the document sub-domain." §2, §7. |
| **The corpus, as TST-elaboration-in-waiting** | Verified count: `reports/theory-of-agentic-tooling.md` cites each TST segment **exactly once** (a real survey pass); the 30 bridge chapters headed for ASF cite **zero** TST slugs, while restating TST's formulas correctly in prose. Mechanical loss on compression, cheap to repair before landing. Two things cut against the 02-TST destination. §8. |

**The three findings I would most want Joseph to see first:**

1. **The strata question resolves, and the resolution is sharper than the question.** AAT's multi-timescale theorem says the two document-layer pathologies people describe as "the same failure" are the theorem's **two conditions violated separately** — canon churning at spike tempo is a (C1) violation; changelog-voice and diff-voice leaking into canon is a (C2) violation. Kelvin–Helmholtz cannot make that distinction because turbulent mixing is symmetric. Neither is symmetric. §5.2.
2. **Late schema payment is not merely later, it is multiplied.** `#der-dual-optimization` establishes that comprehension cost compounds **per reader** while implementation cost does not. Classify authoring-time schema payment as a `$t_0$`-class cost and read-time normalization as a `$t_{comp}$`-class cost, and the four whens stop being symmetric alternatives: only the late ones get multiplied by the turnover multiplier `$k$`. §3.2.
3. **TST already answered the schema-timing question, for tests.** `#hyp-conceptual-alignment`'s *Strategic test timing* discussion says: write thorough tests **when** refactoring for domain alignment, not **before** the domain model has stabilized, because tests written early lock in the wrong model. Substitute "schema" for "tests" and that is the entire O1/O5 tension with a stated resolution. §3.3.

4. **The corpus is sitting on a measurement for a TST open question and neither side knows.**
   `counter-register.md` row 1 (structured notation vs prose; 100% vs 60% comprehension, failing
   to reproduce on 1 of 4 model families) bears directly on `#hyp-conceptual-alignment`'s own
   Working Note that "the direction is empirically uncertain" for AI agents. That is a
   contribution *back* to TST, and it already exists, honestly tiered. §8.2.

**What I did not find and will not pretend to:** TST supplies nothing on removal safety, nothing on reader observability, nothing on constraint authorship, and nothing on documents-as-a-kind. Four of the five questions get partial grounding; the schema question gets a hard negative on apparatus and a soft positive on structure. §7.

---

## 1. What TST actually is, as an object

Worth stating plainly because the invocations have been treating it as larger and more settled
than it is.

TST is **29 live segments** (plus 43 `old-tst-*` archaeology files), ~2,100 lines, in four
chapters. Its stage field says `draft` on all but two. Its epistemic tiers, counted off the
frontmatter:

| Tier | Count | Which |
|---|---|---|
| `axiomatic` | 12 | the definitions, scopes, and the postulate |
| `conditional` | 6 | dual-optimization, change-investment, proximity, principled-decision-integration, code-quality-as-observation-infrastructure, specification-bound |
| `discussion-grade` | **8** | conceptual-alignment, exponential-cognitive-load, causal-discovery-from-git, software-epistemic-properties, and the four `impl-*` chapter discussions |
| `empirical` | 2 | changeset-size, coherence-coupling measurement |
| `exact` | **1** | [[der-change-expectation-baseline| #asf/tst/der-change-expectation-baseline]] — the Jeffreys-prior Lindy baseline |

Worth sitting with: **more of TST is `discussion-grade` than is `conditional`, and exactly one
segment is `exact`.** The axiomatic dozen are definitions, which carry no truth-claim. So when a
design document says "TST grounds this," the load is almost always being carried by a
`conditional` decision rule or a `discussion-grade` hypothesis — which is a real position, but
not the one the phrase implies.

The mathematical weight sits in **AAT**, not TST. TST's own preface says so: "The dependency is
one-directional: TST depends on AAT." When a udon claim says "TST grounds this," the honest
question is almost always *which AAT machinery does the TST segment route to*, because the TST
segment itself is usually a definition or a conditional decision rule.

Two definitional facts that matter more than anything else here, and that I had not seen cited
anywhere in the udon work:

**Documents are in scope by construction, not by analogy.** [[def-feature| #asf/tst/def-feature]]
(`axiomatic`) lists under **Included**: "Documentation changes affecting stakeholder
understanding" and "Refactoring: changes that alter future implementation time while preserving
external behavior." [[def-atomic-changeset| #asf/tst/def-atomic-changeset]] (`axiomatic`) says
"Codebase" crosses architectural boundaries — source code, **schemas**, configuration,
infrastructure-as-code, tests, API contracts…" So the generalization note's Part 4 reading
("documentation records therefore appear to be in scope by construction rather than by analogy")
is correct, and stronger than it claimed: *schema* is named too.

**But scope is not the same as calibration regime**, and that distinction is the frame the whole
day's work has been missing. §2.

---

## 2. The frame nobody ran: TST's own transfer discipline

[[obs-software-epistemic-properties| #asf/tst/obs-software-epistemic-properties]]
(`discussion-grade`; its P5 clauses at empirical tier; the calibration-lab framing labelled
*formulation*) is the segment that governs every other TST invocation, and I do not believe it
has been read in this thread.

Its claim: software is AAT's **privileged high-identifiability calibration laboratory** — not the
"best" domain, but the one where each identification condition is cleanly satisfied — via six
properties. And it states an obligation on anyone exporting a TST result (*evidenced*, verbatim):

> Transfer to other domains requires the analyst to name which property (P1–P6) the target
> domain shares, which it approximates, and which it lacks, and to accept the corresponding
> strengthening or weakening of the operational conclusion.

It also names three overclaim patterns the framing exists to prevent, the first being "domain
generalization by default."

**Running that table for documents-as-artifacts** (*proposed* — this is my assessment, not the
segment's):

| Property | Software | Documents in a git repo | Verdict |
|---|---|---|---|
| **P1** codebase inspectability (`$U_o$` bounded by agent bandwidth, not environment opacity) | holds | holds — a document tree is fully inspectable in principle | **holds** |
| **P2** executable counterfactuals (literal Pearl Level 3 on deterministic outcomes) | holds, code-internal | **fails** — there is no deterministic outcome function on a document. `git checkout` an alternative wording and there is nothing to re-run | **absent** |
| **P3** genuine interventions with characterized `$(\nu, U_o)$` — the type-checker→canary channel ladder | holds | **largely fails** — no compiler, no test suite. A human or agent read is the only channel, and it is the *high*-`$U_o$` end of the ladder | **largely absent** |
| **P4** partially explicit causal structure (imports, types, contracts declare it) | holds | **partial** — wikilinks and `depends:` frontmatter declare some; most co-change structure is undeclared | **weak** |
| **P5** exact recording of the committed subset | holds | holds — same git | **holds** |
| **P6** agent-controlled observation quality (quality *is* channel quality) | holds | holds, arguably more strongly — a document's only job is to be a channel | **holds** |

I think this table is the most useful single artifact this reading produces, for three reasons.

**(a) It explains the estate's actual pain.** Every hand-policed discipline in `asf/FORMAT.md`
and `vivarium/FORMAT.md` — Gate 4, the Working-Notes rules, the vanity-changelog exclusion,
present-truth enforcement — is a **substitute for the missing P3 channel**. Code gets a
type-checker for free; documents get a human reading a convention document. That is not a
cultural failing, it is a named identification shortfall.

**(b) It reframes what a schema checker is.** *Proposed, and I think this is the strongest
claim in this report:* a schema/constraint mechanism for UDON documents is not an application of
TST — it is the **manufacture of the missing P3 channel** for the document sub-domain. In
`#obs-software-epistemic-properties`'s own table vocabulary, a checker supplies an intervention
with a deterministic outcome and low `$U_o$` at instant `$\nu$` — i.e. it moves documents from
the "no Level-2 access" column into the same column code occupies. Everything TST says about
tests then becomes sayable about schema checks, and *not before*. This is why I think Joseph's
instinct that schema is a *when* question is right but under-described: it is also a *whether
the channel exists at all* question.

**(c) It bounds the borrowing.** Any udon claim of the form "TST says X about our documents"
where X routes through `#hyp-causal-discovery-from-git`, `#meas-coherence-coupling`, or the test
channel in `#der-code-quality-as-observation-infrastructure` is exporting a result across a
P2/P3 gap without stating the transfer assumption. §7 lists the specific instances I found.

---

## 3. Q1 — payment-time and the four whens

### 3.1 The hard negative, stated plainly

I grepped every current TST segment for `schema|constraint|validat|invariant|contract|type
system|lint`. The result is a clean negative: **TST contains no apparatus for schemas, for
constraints, or for checking.** The mentions are `#def-atomic-changeset`'s inclusion of schemas
in "codebase," a passing "modify the schema" in a strategy-DAG example in
`#scope-developer-agent`, and hits on "validating a mental model" which is a different sense of
the word.

So: **the four-whens decomposition (constraint authored / check runs / debt due / schema itself
changes) is not something TST formalizes, and nothing in TST says it is wrong.** It is a
decomposition of a territory TST has not entered. Anyone citing TST *for* the decomposition is
borrowing authority. That is the honest answer to Q1 as literally asked.

What follows is what TST does supply once you stop asking it for apparatus it does not have.

### 3.2 The asymmetry the adage misses — the turnover multiplier

Joseph's adage (O1): *"the data will always want a schema — you can use a technology that forces
a schema prematurely up front, before document consumption, or you trade that check and end up
doing all of the battle of normalizing and consolidating in retrospect at read-time or
processing time."*

The structure of that sentence is exactly
[[der-dual-optimization| #asf/tst/der-dual-optimization]] (`conditional`), whose objective is

$$C^* = \operatorname{argmin}_{C}\left[\,t_0(C) + \hat{n}_{\text{future}} \cdot \big(t_{\text{comp}}(F_{\text{typical}} \mid C) + t_{\text{impl}}(F_{\text{typical}} \mid C)\big)\right]$$

Schema-up-front raises `$t_0$` and lowers the future term; schemaless lowers `$t_0$` and raises
it. So far the adage and the formula agree, and it is a plain trade.

But the segment carries a further move that the adage does not, and it breaks the symmetry
(*evidenced*, the segment's own sentence): **"Comprehension cost compounds per-reader;
implementation cost does not."** The turnover multiplier is `$k = (1+r)\cdot s$`, and the
segment states that "with 100% AI context turnover, `$k$` equals the number of sessions that
touch the relevant code."

*Proposed* — the classification is mine, the multiplier is the segment's: the four whens do not
sit at the same place in this formula.

- **Constraint authored** and **schema itself changes** are `$t_0$`-class — paid once, by one
  author, at one time.
- **Check runs** is a `$\nu$`-class quantity, not a cost at all — it is a channel rate. §3.4.
- **Debt due** — the retrospective normalizing, consolidating, re-inferring — is `$t_{comp}$`-class.
  It is paid *per reader, per feature*, and is therefore multiplied by `$\hat n_{\text{future}}
  \times k$`.

If that classification holds, the consequence is not "pay early or pay late, your choice." It is:
**the late payment is the same work multiplied by the number of readers who will ever need it**,
and for an agent-read corpus `$k$` is the session count, which is large and growing. TST's own
gloss for the code case (`#impl-developer-agent`, *evidenced*) is that "the 100% AI
context-turnover case is not a limit case but the *normal* case."

This is a genuinely stronger statement than O1's, and it is derived rather than asserted —
*given* the classification, which is the part someone should push on. The obvious attack: is
read-time normalization really per-reader, or does the first reader's normalization persist for
the next? In code, `#der-dual-optimization`'s own Working Notes concede this ("good documentation
and code structure can amortize comprehension across readers… but it's not formalized yet"). For
documents the answer is *it persists exactly insofar as the reader writes it down* — which is to
say, insofar as the reader authors a schema. That is a nice closure: **amortizing the read-time
payment across readers just is paying it at authoring time**, one reader late.

### 3.3 TST has already answered this question, in the test register

[[hyp-conceptual-alignment| #asf/tst/hyp-conceptual-alignment]] carries a Discussion headed
*Strategic test timing* (*evidenced*, `discussion-grade` like its host segment):

> Tests protect behavior during realignment refactors. But writing tests *before* domain
> understanding stabilizes locks in the wrong model — tests for "posts" and "friends" become
> obstacles when the domain pivots to "documents" and "teammates." The alignment hypothesis
> suggests: write thorough tests *when* refactoring for domain alignment (tests document the
> current understanding), not *before* the domain model has stabilized enough to warrant locking
> in.

Substitute "schema" for "tests" and this is the O1/O5 tension with a stated resolution: **not
early, not late — at the alignment event.** Which is O7's adjudication point, arrived at from a
different direction.

I want to be careful about how much this earns. It is a discussion paragraph in a
discussion-grade segment, and the substitution is mine. But the two artifacts are structurally
the same object under TST's own framing — both are constraints authored to hold behavior fixed
across change — so I think the transfer is better than analogy and worse than derivation. Call
it *proposed, with a same-shape precedent in the theory.*

### 3.4 What TST *does* formalize: "when the check runs" is a channel

Of the four whens, one has real apparatus, in a place nobody has looked.
[[obs-software-epistemic-properties| #asf/tst/obs-software-epistemic-properties]] P3 gives a
channel table (reproduced verbatim in shape):

| Channel | `$\nu$` | `$U_o$` | Coverage |
|---|---|---|---|
| Type checker | Instant | Near-zero | Syntactic/type |
| Linter | Instant | Very low | Style + common errors |
| Unit tests | Seconds–minutes | Low | Tested paths |
| Integration tests | Minutes | Low–medium | Cross-module |
| Staging deploy | Minutes–hours | Medium | Near-production |
| Production canary | Hours | Low (real traffic) | Full |

and states the sequencing consequence: "Causal information yield is concretely estimable per
channel, enabling principled sequencing: fast narrow channels first, slower broader channels when
needed."

*Proposed:* that table **is** the check-timing ladder for schemas, one register over — editor-time
/ save-time / commit-time / build-time / read-time / consumer-time, each with its own rate and its
own noise. And it says something the four-whens framing does not: the whens are not merely
different *times*, they are **different channels with different `$U_o$`**, and TST's ordering
principle (fast-narrow before slow-broad) is a sequencing rule the four-whens decomposition
currently lacks.

The complementary caution, from AAT: [[def-adaptive-tempo| #asf/aat/def-adaptive-tempo]]
(`conditional`) and its `#deriv-tempo-additivity` derivation establish that **channels only sum
when their noises are independent**, with outright *saturation* under shared persistent bias.
Translated: a schema layer, a linter, and a CI check that all read the same declaration and can
all be wrong in the same way buy you roughly one channel, not three. That is a design constraint
on any multi-gate scheme, and it is theorem-grade rather than taste.

### 3.5 Where early payment stops being free

`#der-change-investment`'s *near-zero cost observation* (Discussion, "empirically plausible, not
derived") argues that the principled choice usually costs almost nothing extra, so the threshold
is almost always met and the decision reduces to a *prediction* problem rather than a *tradeoff*
problem. Applied naively that would say: always pay the schema early.

The brake is [[result-specification-bound| #asf/tst/result-specification-bound]] (`conditional`,
premises S1–S2 named): you cannot reliably produce the intended artifact before the distinguishing
information has been transmitted. *Proposed:* paying the schema debt at authoring time requires
that the schema's content **exist to be paid with** — and in the pre-alignment regime it does not.
Early schema payment in an unstabilized domain is not a cheap investment; it is a fabricated
constraint, and the specification bound is why. This is the same brake `#hyp-conceptual-alignment`
applies to early tests, now with a `conditional`-tier result behind it rather than a discussion
paragraph.

**Net for Q1.** TST supplies: the pay-now/pay-later objective (`conditional`); a per-reader
multiplier that makes the whens asymmetric (`conditional`, my classification); a channel model
for the check-timing axis (`discussion-grade`, plus a `conditional` no-free-lunch result on
channel summation from AAT); a same-shape precedent for the timing answer (`discussion-grade`);
and a lower bound on how early you can honestly pay (`conditional`). It supplies **no** apparatus
for the constraint-authorship, debt-due, or schema-evolution axes as such. The decomposition
survives contact with the theory and is not derivable from it.

---

## 4. Q2 — contraction and expand-contract

O4 says: *"TST is our grounding here, from first principles. Refactoring names as the domain
becomes more well-known is a high-level empirically significant practice based on first
principles."* Two claims. They land very differently.

### 4.1 The renaming half: evidenced, near-verbatim, at a weaker tier than claimed

[[hyp-conceptual-alignment| #asf/tst/hyp-conceptual-alignment]] carries an explicit corollary
(*evidenced*):

> **Corollary: Realignment as Feature.** When domain understanding evolves from `$D_0$` to
> `$D_1$`, code written for `$D_0$` accumulates a comprehension cost against `$D_1$`. By the
> `#der-change-investment` threshold, realignment is justified when
> `$T_{\text{align}} \lt \hat{n}_{\text{future}} \times \Delta t_{\text{comp}}$`.
> **Realignment is a feature with measurable ROI, not cleanup.**

Its worked example (*evidenced*): "A codebase still using 'friends' and 'posts' when the product
has pivoted to 'teammates' and 'documents' forces translation on every interaction." That is
structurally identical to Joseph's `site` → `company-primary-url`.

It further gives an explicit **priority ordering** for realignment work, with "Terminology
mismatches in high-traffic code — renaming where `$\hat n_{\text{future}}$` is highest has the
largest ROI" at position 1 of 5.

So O4's first half is not merely consistent with TST; it is a restatement of a corollary TST
already carries, including the example. **The correction is the tier.** The host segment is
`type: hypothesis, status: discussion-grade`, and the corollary's own Epistemic Status says its
status "inherits from both the investment threshold (conditional) and the alignment hypothesis
(discussion-grade); **the weaker link governs.**" And the segment is explicit that "the
*functional form* (inverse proportionality) is not derived from AAT."

**"TST is our grounding here, from first principles" therefore overstates by roughly two tiers.**
The accurate statement, which I think loses nothing that matters: *TST carries this as a named
corollary with a stated ROI threshold; the qualitative claim is near-tautological in the
segment's own words, the functional form is undetermined, and the segment sits at
discussion-grade.* That is still a much better position than "we believe renaming is good
practice."

### 4.2 The contraction half: not stated in TST, but derivable from it in three steps

TST says **nothing** about removal, deprecation, or expand-contract. Searching for it finds
nothing. So O4's second claim — that contraction's absence in document stores is tooling
failure, not domain nature — is not in the theory.

*Proposed* — a three-step derivation, each step evidenced:

1. **A contraction is a feature, and it is subject to the investment threshold.**
   `#def-feature` (`axiomatic`) explicitly includes "Refactoring: changes that alter future
   implementation time while preserving external behavior." So a rename-and-remove is a feature,
   and `#der-change-investment` (`conditional`) governs it:
   `$T_{\text{align}} \lt \hat n_{\text{future}} \times \Delta t_{\text{comp}}$`.

2. **Without an expand-contract mechanism, `$T_{\text{align}}$` is maximal by construction.**
   A flag-day rename must change every consumer in one changeset. By
   `#emp-changeset-size-principle` (`empirical`), `$t_{\text{impl}} \propto
   |\text{changeset}|$` — the flag-day changeset is as large as the consumer set. By
   `#def-discontinuity-distance` (`axiomatic`) that changeset spans the *worst* rung of the
   hierarchy, `$d_{\text{service}}$`, because consumers live in other repos. By
   `#der-change-proximity-principle` (`conditional`) that maximal distance is the maximal
   time penalty at constant size. So the no-mechanism case puts `$T_{\text{align}}$` at
   simultaneously maximal size *and* maximal scatter.

3. **Therefore the observed non-contraction is a threshold artifact, not a domain fact.**
   The threshold is an inequality between a cost and a benefit. If the cost term is inflated by
   the absence of a mechanism, the inequality fails for reasons that have nothing to do with
   whether the contraction is *worth* doing. Observing "contraction never happens" and concluding
   "the domain is monotonic" reads the failure of an inequality as a property of one of its
   sides.

And the positive statement falls out of the same three segments: **expand-contract works by
splitting one changeset that is large-and-scattered into two that are small-and-local, separated
in time.** Under the size principle plus the proximity principle that is a strict improvement in
`$T_{\text{align}}$` on both terms at once, without changing what is achieved. Joseph's sketch
syntax — `|company-primary-url !{was: site}` — is, in these terms, the *expand* half made cheap
enough that the first changeset costs almost nothing.

I would rather Joseph attack this derivation than accept it. The weakest link is step 3's
inference from "the cost is inflated" to "the measurement is uninformative about the domain" —
strictly, a tool-inflated cost and a genuinely-not-worth-it contraction produce the same
observation, so the survey data alone cannot distinguish them. What *would* distinguish them is
measuring the two terms separately, which is what a census (O7) makes possible.

### 4.3 The corroborating detail: TST says distance is tool-relative

There is one sentence in [[der-change-proximity-principle| #asf/tst/der-change-proximity-principle]]
Working Notes that bears directly on O4's "measurement of tool-shaped behavior was being cited as
domain nature" (*evidenced*):

> The principle is about the agent's *experienced* cost, which depends on the agent's tooling and
> navigation capabilities. An IDE with good "jump to definition" reduces effective file distance;
> AI agents that can hold more context may have different distance sensitivities than humans. The
> principle holds qualitatively (boundaries have nonzero cost) but the distance hierarchy's
> quantitative weights are agent-dependent.

TST is stating, in its own voice, that the cost surface these measurements sit on is a property of
the tooling. That is the general form of O4's reinterpretation, one level up from contraction.

---

## 5. Q3 — strata on different clocks

This is the question I found most worth the reading, and the answer differs from the one Part 5
was reaching for.

### 5.1 The apparatus exists, and it is `exact`

[[der-multi-timescale-stability| #asf/aat/der-multi-timescale-stability]] is `type: derived,
status: exact` — "derived by elementary Lyapunov arguments stacked on the sector-persistence
template, independently re-derived and verified in full (spike trail 2026-06-10)." It gives, for
a fast level `$x_1$` and a slow level `$x_2$` with ratio `$\epsilon$`:

- a **closed-form separation threshold** `$\epsilon_{\max} = \Delta\rho_1^\ast / (L_h v_2^{\max})$`
  — in the segment's gloss, "the faster level's *spare adaptive reserve* … divided by the rate at
  which the slower level drags the faster level's target";
- two conditions, **(C1)** `$\epsilon < \epsilon_{\max}$` and **(C2)** `$\alpha_2 R_2 > \rho_2 + L_{21}R_1$`;
- a **warm-start refinement** pricing early engagement;
- an **N-level recursion** for nearest-neighbour coupling.

Three corrections to what Part 5 assumed about it, all small but they are the kind of thing the
"unread" flag exists to catch:

- Part 5 says "AAT reportedly already imports Tikhonov's theorem." It does not, here. The
  segment's derivation "in fact bypasses Tikhonov's theorem entirely via the composite-Lyapunov
  route," and uses its own sector condition to *supply* Tikhonov's unique-root prerequisite. The
  lineage is Saberi–Khalil composite-Lyapunov, and the RG reading is Chen–Goldenfeld–Oono.
- `#der-temporal-nesting` (`robust-qualitative`) is the qualitative rule; `#der-multi-timescale-stability`
  is what turned it into a threshold. Part 5 treated them as one body.
- Part 5's "flux matching at interfaces… already vivarium's formalism" has a closer AAT relative
  it did not name: the **renormalization reading**. Integrating out a level maps the level above
  onto the *same* template with `$\rho_{k+1} \mapsto \rho_{k+1} + L_{k+1,k} r_k$`. The segment
  states this makes AAT's persistence architecture "scale-free" in a precise sense. That is a
  stronger and more usable statement than flux-matching for the document case, because it says
  **every layer presents the identical problem with neighbour-renormalized parameters**.

### 5.2 The framing inversion, which I think is the finding

Part 5 proposed two exchange processes — rectified selective transport (promotion toward canon)
and symmetric unselective contamination (changelog-voice into Working Notes, diff-voice into
canon) — and concluded that Kelvin–Helmholtz fits only the second, with "shear (a difference in
*rate*) is the driver."

**The apparatus says the opposite about rate, and says the two processes are one theorem.**
(*Proposed* reading of an `exact` result.)

In KH, a rate difference across an interface *drives* instability. In
`#der-multi-timescale-stability`, a rate difference is the thing that makes stacking **stable**:
small `$\epsilon$` is the good case. What destabilizes is *insufficient* separation and
*excessive* sensitivity, and the segment names both pathologies (*evidenced*):

> *Micromanagement* is a (C1) violation: the slow level acts at fast tempo, the moving-target
> disturbance exhausts the fast level's reserve, and the fast level thrashes… *Catastrophic
> forgetting* is a (C2) violation: the slow (consolidated) level is too sensitive to fast-level
> transients — `$L_{21}$` too large — and the fast loop's activity overwrites it even under
> perfect timescale separation. The two pathologies several auditors read as "the same failure"
> are precisely the two conditions of one theorem, violated separately.

Mapped onto document strata (*proposed*):

| Theorem object | Document-strata reading |
|---|---|
| fast level `$x_1$` | the spike / audit / working-notes layer |
| slow level `$x_2$` | canon (segments, spec, ratified decisions) |
| `$\epsilon$` | ratio of canon's revision rate to the spike layer's |
| `$L_h$` | how much a canon change moves the spike layer's target |
| `$L_{21}$` | how sensitive canon is to unsettled spike-layer output |
| **(C1) violation** | canon/spec churning at spike tempo → the fast layer thrashes chasing a moving spec |
| **(C2) violation** | **changelog-voice and diff-voice leaking into canon** — canon too sensitive to transients |
| warm-start refinement | promoting *before* the spike has settled: not forbidden, but priced at `$L_{21}(R_1 - r_1)$` extra reserve |

So Part 5's "symmetric contamination" is, under this mapping, **not symmetric** — it is
directional, from fast to slow, and it is (C2). And the reason KH could not model rectified
promotion is not that promotion is a different process; it is that **KH was the wrong physical
picture for a system whose rate difference is a stabilizer rather than a shear**.

Two consequences I would flag as immediately usable:

**The Gate-4 discipline is graded, not binary.** `#der-temporal-nesting`'s qualitative rule was
"a slower process must not act before the faster process beneath it has converged" — which is
`asf/FORMAT.md`'s integrate-only-settled-output discipline. The warm-start refinement (*evidenced*)
grades it: "early action does not void the guarantee — it raises the slow level's required
reserve from settled-residue size to worst-case-transient size." Integrating an unsettled spike
into canon is not a violation; it is a purchase, and the price is named.

**There is a case where slowing canon does not help, and the theory says which.** From the
Tikhonov remark (*evidenced*): "Slowing the slow level helps only against (C1) violations, never
against a fast level that has no settled state to offer." Translated: if the spike layer has no
convergent output — genuinely open questions, cycling positions — then reducing canon's revision
rate buys nothing. That is a checkable prediction about when a moratorium works and when it is
theatre.

### 5.3 The premise that blocks the transfer — and it is already on AAT's own open list

Here is the honest limit, and I think it settles Part 5's "assembly vs new claim" question in a
third way that is better than either.

The theorem's premises are **(S0)** locally Lipschitz vector fields with Carathéodory solutions,
**(S1)** a Lipschitz quasi-steady-state manifold `$h$` with bounded `$Dh$`, **(S2)** per-level
sector conditions, **(S3)** bounded interconnection, **(S4)** bounded disturbance.

Document strata are **discrete**. An edit is a jump. A promotion into canon is a jump. There is
no continuous vector field and no differentiable manifold. So (S0) and (S1) fail as stated.

And the segment's own Working Notes name exactly this, for AAT's own internal case (*evidenced*):

> **Open (which AAT mechanisms satisfy the premises).** The honest gap left by the promotion:
> structural adaptation as actually triggered by `#result-structural-adaptation-necessity` is
> plausibly a *jump process* (discrete model-class moves), outside (S0)/(S1) as stated. Two
> follow-up directions: an impulsive/hybrid-systems extension of the stacking…

**So: Part 5 is neither free assembly of existing apparatus nor metaphor.** It is assembly
*modulo one premise that AAT has already identified as unmet for its own deepest levels, and has
already named the repair for* (impulsive/hybrid singular perturbation). Document strata are not
a special problem — they are an instance of the same open item, arriving from a different domain.

What that means practically (*proposed*):

- The **qualitative** structure transfers now and is usable now: two conditions, two named
  pathologies, the warm-start price, the renormalization/scale-free reading, the no-settled-state
  caveat. None of these needs the differential form to be meaningful.
- The **quantitative** `$\epsilon_{\max}$` does **not** transfer. Anyone writing
  `$\epsilon_{\max} = \Delta\rho^\ast/(L_h v^{\max})$` about document layers would be exporting a
  formula across a failed premise. I would treat that as the specific overclaim to avoid.
- The segment names one construction where the premises hold **by architecture** rather than by
  assumption: Friston's renormalizing generative models, where "one slow tick per block of `$K$`
  fast ticks" makes `$\epsilon_k/\epsilon_{k+1} = 1/K$` true by construction. *Proposed:* the
  document analogue is **batching promotion on a fixed cadence** rather than promoting
  continuously — which converts an assumption into a design guarantee, and is cheap. AAT's own
  `#form-composition-closure` macro-clock `$K_c \gg 1$` is the same move.

I want to be honest that the discrete-vs-continuous gap is not cosmetic. Whether an
impulsive-systems version of this theorem yields the same two conditions with the same
interpretation is genuinely unknown — the hybrid-systems literature has counterexamples where
jump dynamics destabilize systems whose continuous relaxations are stable. So "the qualitative
structure transfers" is my judgment, not a result, and it is exactly where I would want a
mathematician's second opinion.

---

## 6. Q4 — co-location and coupling

Part 4's chain: `coupling(body, Working Notes) ≈ 1` places them in one module by
`#def-system-coupling`'s own terms; `coupling(body, events) ≈ 0` separates them; the proximity
principle, if it says what its neighbours suggest, grades the layout menu.

### 6.1 The proximity principle does say what its neighbours suggested

[[der-change-proximity-principle| #asf/tst/der-change-proximity-principle]] (`derived`,
`conditional`) states: at constant changeset size, `$t_{\text{impl}} \propto
1/\text{proximity}$` where proximity inverts `$\sum_{i,j} d(c_i,c_j)$` over
`#def-discontinuity-distance`'s hierarchy. Its Discussion gives the *size-proximity
decomposition* explicitly: "Size is the first-order term; proximity is the structure-dependent
correction." So yes — Part 4's guess was right, and the layout menu (same file / sibling file /
sibling directory / parallel tree) is graded by the `$d_{\text{lexical}} < d_{\text{file}} <
d_{\text{module}} < d_{\text{service}}$` hierarchy.

Two things Part 4 could not have known without reading it:

- **The quantitative form is a hypothesis, not a derivation.** "The actual relationship between
  distance and cost is not derived — it could be linear, logarithmic, or dependent on the type of
  boundaries crossed." Only the *ordering* is safe to lean on.
- **The weights are agent-dependent** (quoted in §4.3). For agents with large context windows,
  the human hierarchy is not the right cost surface. *Proposed:* "layout follows co-change"
  should be "layout follows co-change weighted by the *reader's* distance costs," and for an
  agent-read corpus those costs are flatter than the code case assumes — which weakens the
  argument for aggressive co-location and strengthens the argument for declared projection.

### 6.2 The complication nobody has noticed: the estimate is confounded by the layout

This is the finding I would most want checked, because if it holds it partially undercuts the
measurement Part 4's open questions propose running.

Part 4's chain estimates `coupling(body, Working Notes)` from co-change and concludes they belong
in one module. But [[def-system-coupling| #asf/tst/def-system-coupling]] and
[[hyp-causal-discovery-from-git| #asf/tst/hyp-causal-discovery-from-git]] name three confounder
classes, and the second is exactly this case (*evidenced*):

> **C2. Convention-driven bundling.** Developers group related changes into single commits for
> organizational reasons (clean git history, atomic deployments), not because the changes are
> causally linked.

And `#meas-coherence-coupling`'s Working Notes: "If developers make large commits bundling
unrelated changes, coupling estimates are inflated."

*Proposed:* body and Working Notes co-change at rate ≈1 **partly because they are in the same
file** — you cannot edit one without touching the file that holds the other, and a single commit
records them together regardless of whether the changes were causally linked. The measurement
that would justify the layout is contaminated by the layout. This is circular in a way the
generalization note's Part 4 did not flag.

TST names the repair, and it is cheap (*evidenced*, `#hyp-causal-discovery-from-git` point 3):
"**Frequency asymmetries carry causal information.** If changes to `$A$` are frequently followed
by changes to `$B$`, but changes to `$B$` are rarely followed by changes to `$A$`, this asymmetry
is evidence of a directed causal link… that survives common-cause confounding." So the
informative measurement is not `$P(\text{WN changes} \mid \text{body changes})$` but the
**asymmetry** between the two directions. Symmetry would be consistent with pure bundling;
asymmetry would not. That is a concrete refinement to the measurement Part 4's open questions
propose, and it costs nothing extra to compute.

### 6.3 "The file is the atomicity unit" inverts TST's unit

Today's schema work extended Part 4 to "the file is the atomicity unit." I think this is
backwards on TST's own terms, and worth correcting before it propagates.

`#def-atomic-changeset` (`axiomatic`) makes **the changeset** the atom, and is emphatic that it
crosses artifact boundaries: "If it must change to deliver the feature, it is part of the
changeset." Files are not the unit; they are where the unit lands.

*Proposed* restatement that keeps what the schema work wanted: **the file is a proximity
container, and its merit is measured by how much of a typical changeset it contains.** A file is
a good boundary exactly when features that touch it touch little else — which is
`#def-system-coherence` (`axiomatic`), "the expected proximity of changes within a module."
That phrasing preserves the design conclusion (co-locate what co-changes) while keeping the
theory's unit, and it makes the claim measurable rather than definitional.

There is one place where the file *does* have privileged status and it is worth naming because it
is not a TST fact: git records commits, and a commit is the observational grain. So the file is
the atomicity unit **of the measurement apparatus**, not of the theory. Conflating those is how
you get the circularity in §6.2.

### 6.4 What TST does not carry for Part 3's canonicity axis

Part 4's own assessment was that "the canonicity half appears to be absent from that cluster."
Having read the full corpus, I can confirm that more strongly: **nothing in TST distinguishes
canon from non-canon, present-truth from historical, or ratified from provisional.** The nearest
relatives are `#def-atomic-changeset`'s authored-vs-generated exclusion (whose own Working Notes
are unsettled about it) and `#def-feature`'s "as perceived by" level-of-description qualifier.

Fable's collision mechanism from Part 7 — that a present-truth claim can collide and an
append-only entry cannot — has **no TST analogue at all**. I looked for one. If that mechanism is
real, it is a genuine extension TST would want rather than something TST already implies, and it
would sit naturally beside `#der-code-quality-as-observation-infrastructure` as a second route by
which artifact structure determines `$U_o$`: a format that *manufactures collisions* lowers the
observation noise on staleness, which is precisely the `$Q \to U_o \to \eta^* \to \mathcal{T}$`
chain applied to a property (staleness) rather than to comprehension. That is the most promising
TST-extension shape I found in this reading, and it is *proposed*.

---

## 7. Q5 — the honest inventory

### 7.1 What TST actually supplies

| For | TST supplies | Tier |
|---|---|---|
| Pay-now vs pay-later structure | `#der-dual-optimization`'s objective | conditional |
| Why late payment is multiplied | the turnover multiplier `$k$`, per-reader compounding | conditional |
| Check-timing as a design axis | `#obs-software-epistemic-properties` P3 channel ladder + CIY sequencing | discussion-grade |
| Why more gates ≠ more assurance | `#def-adaptive-tempo` / `#deriv-tempo-additivity` channel-independence | conditional |
| Renaming toward domain vocabulary | `#hyp-conceptual-alignment`'s Realignment-as-Feature corollary, with ROI threshold and priority ordering | discussion-grade |
| Refactoring is a first-class feature | `#def-feature` inclusion | axiomatic |
| Why flag-day contraction fails | size × proximity × distance composition | empirical + conditional |
| Layout follows co-change | `#def-system-coupling` + `#der-change-proximity-principle` + `#def-system-coherence` | axiomatic + conditional |
| Layered systems on different clocks | `#der-multi-timescale-stability` (qualitative structure only) | exact result, failed premise |
| Documents are in scope | `#def-feature`, `#def-atomic-changeset` | axiomatic |
| The transfer discipline itself | `#obs-software-epistemic-properties` P1–P6 obligation | discussion-grade |

### 7.2 What is genuinely missing from the mathematics

1. **No schema, constraint, or check apparatus.** Grep-verified. The four whens are unbuilt
   territory.
2. **No removal or deprecation apparatus.** `#der-change-investment` covers adding and
   restructuring; contraction has no segment.
3. **No reader-observability notion — but the shape of one is forced.** *Proposed:*
   `#def-system-coupling` is estimated from *observed* co-change. A consumer outside your history
   generates no commits in it, so `coupling(m_i, \text{unknown consumer})` is not merely unknown,
   it is **structurally unestimable**. That is a derivation of O5's "contraction safety is
   decidable only within an adjudicator's observability" from TST's definition of coupling, and I
   think it is the cleanest formal grounding O5 has.
4. **Nothing on canonicity, present-truth, or collision.** §6.4.
5. **The discrete/jump gap** for the stratification apparatus. §5.3. Already an AAT open item.
6. **The functional forms are open almost everywhere.** Alignment's `$1/\text{alignment}$`,
   proximity's inverse form, cognitive load's `$k^d$` — all explicitly undetermined. Any udon
   claim that leans on a *shape* rather than an *ordering* is leaning on nothing.

### 7.3 Where today's design work has been using TST's name for claims the segments don't carry

Stated as findings, not accusations — most of these are one word away from accurate.

- **"TST is our grounding here, from first principles" (O4).** Overstates by ~two tiers. The
  grounding is a discussion-grade corollary with an undetermined functional form. §4.1. The
  cheap fix is naming the corollary instead of the theory.
- **The contraction-is-tooling-failure claim** is presented as TST-grounded and is not in TST. It
  *is* derivable from TST in three steps (§4.2), which is a better position than the citation
  implied — but the derivation had not been done, so the citation was carrying weight the theory
  had not been asked to bear.
- **"The file is the atomicity unit"** inverts `#def-atomic-changeset`. §6.3.
- **Part 4's layout derivation** rests on a coupling estimate confounded by the layout it
  justifies, with the confounder named in the very segments it cites. §6.2.
- **Part 5's Kelvin–Helmholtz framing** has the sign of the rate-difference effect backwards
  relative to the apparatus AAT actually holds, and it split into two processes what the theorem
  treats as one. §5.2.
- **Every TST invocation in the udon work, without exception,** skips the P1–P6 transfer table
  that `#obs-software-epistemic-properties` explicitly requires of exporters. §2. Given that
  documents fail P2 and largely fail P3, results routed through the test channel or through
  git-causal-discovery are the ones most exposed.
- **The tooling corpus restates TST's formulas correctly and cites none of them** — verified
  zero TST slugs across 30 chapters. Not overclaim; under-attribution, and the reverse defect
  from the ones above. §8.1.

### 7.4 The claim I would most like someone to attack

That a document schema-checker **is** the manufacture of TST's missing P3 channel (§2b), and that
this — rather than "schema is a when question" — is the deepest available framing of the schema
work. If it holds, it predicts something specific and testable: introducing a checker should
lower `$U_o$` on the document-reading channel enough to show up in the
`$Q \to U_o \to \eta^* \to \mathcal{T}$` chain, i.e. in observable agent tempo on the corpus —
which is measurable with the same instruments O7's census would need anyway. If it does not hold,
I would want to know whether that is because documents' outcome function is genuinely
non-deterministic (in which case a checker is a linter, not a test, and belongs at the low-`$\nu$`
high-`$U_o$` end of the ladder) or for some reason I have not seen.

---

## 8. The udon-needs tooling corpus, read as TST-elaboration-in-waiting

Joseph's framing was that the 30 chapters in `udon-needs/02-tooling-needs/src/` will likely land
as a new group inside 02-TST, so they should be read as TST elaborations. A parallel reader
surveyed them while I read the theory; I verified the load-bearing counts myself.

### 8.1 The citation number, verified

For each of the 16 most-relevant TST/AAT slugs, occurrences in the deep report vs. in the 30
bridge chapters:

**`reports/theory-of-agentic-tooling.md`: exactly 1 occurrence of each. The 30 `src/` chapters:
zero occurrences of any TST slug, without exception.**

That is a clean, mechanical result and it names the situation precisely. The deep report did a
**survey pass** — one mention per segment, with each segment's own tier reported, including the
two TST OUTLINE `--GAP--` rows (`#def-developer-tempo-channels`,
`#hyp-software-unmaintainability-bifurcation`) correctly identified as unlanded. That is real
reading. But the citations did not survive compression into the chapters, and the chapters are
the material headed for ASF.

The chapters do restate TST's mathematics, correctly, without naming it. Instances the survey
found (each *evidenced* against the chapter text):

- `the-crystallized-process-thesis.md`: "an investment of time pays when it is less than
  (expected future uses) × (comprehension time saved per use) × (number of distinct future
  readers)" — that is `$t_{\text{invest}} < \hat n_{\text{future}} \times \Delta t_{\text{comp}}
  \times k$` from `#der-code-quality-as-observation-infrastructure`, verbatim in structure, cited
  only to a sibling chapter.
- `typing-and-schema-boundary.md`, `machine-first-documents.md`: the transmission bound and
  "shared notation is compression" — `#result-specification-bound`, cited to the dossier section
  number rather than the slug.
- `round-trip-and-span-splice.md`: "the theory's locality results — at equal size, a concentrated
  change costs less than a scattered one" — that is `#der-change-proximity-principle` and
  `#def-discontinuity-distance`, named as "the theory's locality results."

Given the estate's own ratified convention (`[[stem| #asf/tst/stem]]`, in `udon-needs/CLAUDE.md`),
I read this as **mechanical loss during compression, not a judgment call** — the citation exists
two hops upstream. That makes it cheap to repair and worth repairing before the group lands in
ASF, where the slug is the identity.

### 8.2 The corpus is sitting on evidence for a named TST open question and does not know it

The single most interesting thing the survey turned up. `src/counter-register.md` row 1 records a
measured experiment (*evidenced*, verified in the file): structured notation vs. prose,
comprehension measured directly — 100% vs 60% immediate comprehension, but **the effect failed to
reproduce on 1 of 4 model families, and that family processed the structured form more slowly.**

`#hyp-conceptual-alignment`'s Working Notes contain this open question, in the theory's own voice
(*evidenced*):

> For AI agents with 100% context turnover, alignment may matter *more* than for humans, because
> the agent cannot build up a mental mapping over repeated interactions… Or it may matter *less*,
> because AI agents can process misaligned names faster than humans can. **The direction is
> empirically uncertain.**

*Proposed:* the counter-register row is a measurement bearing on exactly that uncertainty, and
the non-reproduction is the more informative half — it is evidence that **alignment sensitivity
is agent-architecture-dependent**, which is the same conclusion
`#der-change-proximity-principle`'s Working Notes reach independently for *distance* sensitivity
(§4.3). Two TST segments have flagged agent-dependence as open; the corpus has one measurement
pointing the same way. Neither chapter nor segment cites the other. This looks to me like the
corpus's best available contribution *back* to TST rather than from it, and it is cheap to make:
the row already exists, honestly tiered, with its confounds named.

### 8.3 Two things that cut against "this corpus is TST elaboration"

Reported as the survey found them, because they bear on the destination decision and are not
mine to settle.

1. **The temporal-nesting machinery is absent from the demand chapters.** `#der-temporal-nesting`
   and the five-forcing-functions F3 argument appear in the deep report's AAT section and nowhere
   in a UDON-facing chapter — no chapter engages document strata changing at different rates,
   which is §5's whole subject. So §5 of this report is **new material for that corpus, not a
   citation fix**, and if the strata question is going into the eventual ASF group it needs a
   chapter written, not a slug added.
2. **The corpus's center of gravity is AAT/03-llm-core, not TST.** What is load-bearing across
   the widest set of chapters is the κ×A bias law, tempo/persistence, `do()`-gating, and the
   observation/law/mutation decomposition — that is agent cognition and belief update, which is
   AAT's and 03-llm-core's domain. TST proper (change economics, coupling/coherence, alignment)
   is rich in the deep report and thin in the chapters. *Open, and Joseph's call:* whether the
   02-TST destination is a mismatch, or whether TST's scope is expected to widen to absorb
   agent-tooling material. I note only that `#obs-software-epistemic-properties`'s calibration-lab
   framing is an argument for the *narrow* reading — TST is deliberately scoped to software under
   P1–P6, and widening it to agent cognition generally is the first of the three overclaim
   patterns that framing exists to prevent.

### 8.4 Reading caveats on this section

The survey read the epistemology notes, the deep report in full, and roughly a quarter of the 30
chapters closely with grep sweeps across the rest; I verified the citation counts and the
counter-register row against the files myself but did not read all 30 chapters. The survey also
flagged that it did not verify byte-fidelity of the deep report's promotion from
`01-ideation/02-provenanced/syntheses/asf-dossier.md`. If any conclusion here turns on the
precise content of a chapter I have not quoted, treat it as second-hand.

---

## Appendix A — corrections to the generalization note

Offered because `~/src/arch/notes/outline-segments-generalization-2026-07-23.md` flagged these
as unverified and asked for exactly this.

| Note's claim | Status after reading |
|---|---|
| "#der-change-proximity-principle, if it says what its neighbours suggest, would grade the layout menu as a proximity ordering" | **Confirmed.** It does, via `#def-discontinuity-distance`. Add: only the ordering is safe; the functional form is an open hypothesis; the weights are agent-dependent. |
| "AAT reportedly already imports Tikhonov's theorem" | **Corrected.** The derivation bypasses Tikhonov via composite-Lyapunov, and uses its own sector condition to supply Tikhonov's unique-root prerequisite. |
| "carries #def-adaptive-tempo, #der-multi-timescale-stability and #der-tempo-composition — all unread, so this is a hypothesis about where the apparatus lives" | **Confirmed as to location.** The apparatus is where the note guessed. Its content differs from the guess: see §5.2 (framing inversion) and §5.3 (premise gap). |
| "Might carry: slow–fast decomposition with a slow manifold" | **Partly.** The decomposition is `exact` and available; the *manifold* premise (S1) is precisely what documents fail. |
| "Probably does not carry: momentum as a conserved quantity… growth rates and Richardson numbers look like costume" | **Confirmed, and the reason is sharper than 'costume':** AAT's version has no conserved quantity either, and its stability parameters (`$\alpha, R, \rho, L_h, L_{21}$`) are the ones that carry meaning. Richardson numbers would be costume *over* an apparatus that already has its own dimensionless group, `$\epsilon/\epsilon_{\max}$`. |
| Part 5's two-process split (rectified transport vs symmetric contamination) | **Complicated.** Under the theorem both are the same theorem's two conditions, and the second is directional rather than symmetric. §5.2. |
| Part 4's four-segment reading | **Confirmed as far as it goes**, with the C2-confounding complication (§6.2) and the unit inversion (§6.3) added. |
| "#meas-coherence-coupling run over the asf tree's own history would estimate co-change coupling between record parts directly" | **Confirmed as a good idea, with a refinement:** run it *asymmetrically*. The symmetric estimate cannot distinguish coupling from same-file bundling; the directional asymmetry can. §6.2. |
| "the harness's claim-atom is unknown" | Not addressed by this reading. TST's atom is the changeset, which is a *change* rather than a *claim* — I note the type mismatch without resolving it. |

## Appendix B — segment inventory read

`02-tst-core/`: OUTLINE.md and all 29 current segments — `post-temporal-optimality`,
`scope-evolving-software`, `scope-developer-agent`, `scope-continuous-operation`,
`obs-software-epistemic-properties`, `def-feature`, `def-comprehension-time`,
`def-implementation-time`, `def-atomic-changeset`, `def-discontinuity-distance`,
`def-system-coupling`, `def-system-coherence`, `def-system-availability`,
`result-specification-bound`, `der-change-expectation-baseline`, `der-dual-optimization`,
`der-change-investment`, `der-change-proximity-principle`, `der-principled-decision-integration`,
`der-code-quality-as-observation-infrastructure`, `emp-changeset-size-principle`,
`meas-coherence-coupling`, `hyp-conceptual-alignment`, `hyp-exponential-cognitive-load`,
`hyp-causal-discovery-from-git`, `impl-foundations-features`, `impl-developer-agent`,
`impl-code-structure`, `impl-system-measures`.

`01-aat-core/`: `def-adaptive-tempo`, `der-multi-timescale-stability`, `der-tempo-composition`,
`der-temporal-nesting`.

Not read: the 43 `old-tst-*` archaeology files, `lit-review/`, `empirical-discontinuity/`, and
AAT beyond the four segments above (the `asf-dossier.md` in `udon-needs` covers AAT broadly and
this report deliberately does not duplicate it).
