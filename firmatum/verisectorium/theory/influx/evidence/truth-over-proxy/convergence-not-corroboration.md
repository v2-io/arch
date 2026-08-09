# Evidence sweep — convergence and consensus as proxy: coherence, not corroboration

*Slice of the estate-wide evidence sweep for `claim-truth-over-proxy`. Direct verbatim extracts from the home estate (memorata3-search + `rg` across `~/src`), source path + context per extract, oldest-of-duplicates kept. Commentary is marked as such; everything else is verbatim.*

---

## 1. The foundational articulation — asf spikes.sop.md §0 (2026-05-18, Joseph)

Verbatim, `~/src/arch/asf/doc/sop/spikes.sop.md` §0 ("The core principle — truth is the arbiter; everything else is a proxy"):

> **This governs every section below it.** The job is to get the *theory's truth* right. Provenance, git history, CHANGELOG, the INDEX label, `NOTATION.md`, the spike's own framing, the segment's own assertion, audit findings, agent consensus, even the convergence of multiple independent agents — **all of these are mild proxies for truth, and every one of them drifts.** They are useful for *locating* a question and *cheap-screening* it; they never *settle* it. A question is settled only by the mathematics, re-derived independently far enough to stand on constitutive structure (definitions that make the core cohere) + forced identities + elementary steps — *not* on what any artifact says.

This is the earliest and most explicit "convergence is a proxy, not a settler" statement found in the estate; the material below is its elaboration, its qualification (when convergence *does* carry force), and its contested edge cases.

**Refinement 4** (same file, 2026-05-17, fan-out S1+S5+S6) shows the discipline applied to itself, self-referentially:

> *Refinement 4 (2026-05-17, fan-out S1 + S5 + S6 — independent triple convergence, which per the convergence-as-coherence discipline is strong signal the gap is in the frame, not one agent's head). Two linked defects: …*

---

## 2. The udon epistemology corpus — the "convergent lock," keyed on failure-mode independence

This is the estate's most developed formal treatment. It exists in three places: the original udon-needs working notes (now `~/src/MOVED/udon/v2/udon-needs/...`, superseded), and its landed form inside verisectorium's own local canon at `theory/influx/instrumenta/bridges/method-evidence-tiers.md`. **The verisectorium copy is quoted below as primary** since it is already inside this estate's own tree; oldest wording (from the udon original, 2026-07-22) is identical in substance.

From `~/src/arch/firmatum/verisectorium/theory/influx/instrumenta/bridges/method-evidence-tiers.md` (local canon):

> ## Lock 1 — convergent (does independent support agree?)
>
> Agreement across kinds of evidence is this report's actual unit of proof. But `convergent` is **not a rung on the strength ladder** — it is a lock computed on top of it: a claim is `convergent` when **two or more support-kinds with independent failure modes** agree. A convergent claim still has a strength ceiling; the lock raises confidence *within* that ceiling by ruling out the failure mode any single kind is prone to.
>
> **The lock keys on failure-mode independence, and that is the whole discipline.** Within-kind corroboration — two testimonials, three shipped tools doing the same thing — raises *strength* (a stronger testimonial, a wider observation) but does **not** arm the lock, because same-kind sources share a failure mode. Two de-novo testimonials from different substrates are still one kind of evidence with one blind spot; more voices do not make an interpretation *correct*. This is the report's own same-lineage-blind-spot thesis applied to its own epistemology, and the frontmatter enforces it: `convergent:` lists **kinds**, never same-kind sources.
>
> **The repair for a convergent claim is: break the independence** — show that two legs actually share a failure mode (descent from one source is the usual culprit). If the independence breaks, the lock voids and the claim falls back to its strongest single leg. This is what makes descent-correction *mechanical* rather than a matter of remembering to be careful.
>
> **Worked example (the lock spec's canonical case):** [[errors-that-teach]] presents its refuse-on-multi-match principle as four-kind convergent — `design` (the built refusal tool), `observational` (eleven of fourteen shipping harnesses), `testimonial` (an agent's own account of the damage its absence causes), and `theoretic` (the law-teaching condition). But the `observational` leg is *mostly one influential design copied across the ecosystem* — so it is one partially-self-correlated leg, not a fourth independent failure mode. The honest lock is **three independent kinds plus a descent-echo**, not four independent kinds — and the chapter says so. The lock's failure-mode key is exactly what forces that audit instead of letting "4-tier convergent" stand unexamined.

And the report's discipline-statement about itself, same file:

> 1. **Agreement across independent kinds is the unit of proof** — the convergent lock above, with its failure-mode key. Most of the design work has one author; agreement between his own projects is coherence, not corroboration, and does not arm the lock.

The chain of reasoning that *produced* this lock (udon epistemology-pilot / SYNTHESIS cycle, 2026-07-22, `~/src/MOVED/udon/v2/udon-needs/02-tooling-needs/notes/epistemology-SYNTHESIS.md`) — the sharpening that made "kinds" rather than "sources" the unit:

> 1. **The convergent lock must key on *failure-mode* independence, and within-kind multi-source corroboration raises strength but does NOT arm the lock.** Two de-novo testimonials from different substrates are a *stronger testimonial* (strength axis), not a convergence — they share the single-perspective/plausible-but-wrong failure mode. This is the report's own same-lineage-blind-spot thesis applied to its own epistemology, and it matters because an agent will otherwise claim `convergent: [testimonial, testimonial]` and launder correlated evidence into a lock. My templates case is the live example: owner + two same-discussion reviewers are *not* independent (shared session, shared failure mode); the lock arms only when the de-novo testimonial (a genuinely different kind) joins — `convergent: [design, testimonial]`.

And it bites — from `~/src/MOVED/udon/v2/udon-needs/02-tooling-needs/notes/TST-extension-memo.md` (2026-07-22):

> **It bites, which is the evidence it's worth having.** Applying it across this corpus un-armed one chapter's lock entirely and reduced several others — most sharply, a chapter claiming three-way convergence from "built / designed / theorized" turned out to have **one** leg, because all three are facets of the same author's work. That is exactly the error the field exists to catch, and nothing in the current vocabulary would have surfaced it.

A later application confirming the rule against the report's own text (udon session, 2026-08-06, `~/.claude/projects/-Users-josephwecker-v2-src-arch/e6cee031-8288-4c27-a120-c2cd7a5f3bc0.jsonl`):

> Ha — and that lands a sharper point than the joke: the report's own trio-ratified epistemology already contains the rule that convicts it. The convergent lock arms only on legs with *independent failure modes*, and same-author restatements across contexts — transcripts → reports → bridges — are one failure mode wearing seven timestamps. By its own law, most of its "convergences" are a stronger testimonial, not a lock.

And verisectorium's own local `REGISTER-RULING.md` applying the same discipline to udon's own report (already local canon, `theory/influx/instrumenta/REGISTER-RULING.md`):

> **Provenance correction (Joseph).** The udon-needs tooling report is not independent evidence converging with the realization model — it is *Joseph's own earlier thinking*, restated and adjudicated by himself in other contexts … Every "the corpus already had this" … framing in the mining map should be read accordingly: it is all one mind's lineage. Same-author agreement is coherence — evidence about design intent, real and load-bearing — never corroboration.
>
> **Reflexive note (coord).** The report's own ratified epistemology contains the disqualifier: its convergent lock arms only on legs with *independent failure modes*, and same-author restatements across contexts share one failure mode. By the report's own rule, most of its convergences are strengthened testimonials, not locks. Applying this ruling is the report's methodology applied to the report.

---

## 3. "It was all me" — the named principle, whole-file, `~/.claude/projects/-Users-josephwecker-v2-src-udon/memory/convergence-vs-single-authorship.md`

Reproduced in full — this is a durable memory file, not a transcript excerpt, and is one of the cleanest primary statements in the estate:

> Joseph, 2026-07-16, after I'd built a schema position partly on "rowan's DSL independently corroborates this": **"I wouldn't read too much into the convergences you see — it was all me."**
>
> **The trap, and it caught me three times in one session:** ~/src/rowan, ~/src/udon, ~/src/archema-io/asf, ~/src/autopax, and the 2025 sapientia/ennaos/nexum corpus share a single author. When two of them agree, that is **one person being consistent with himself across years** — evidence about *his instincts*, not about a design being forced by the problem. I kept experiencing it as separate paths meeting, and reporting it as corroboration. The three:
>
> 1. Rowan's block-structured constraint DSL "matching" udon's element-form.
> 2. The December `examples/*.udon` "matching" CORE's identity model.
> 3. CORE's element suffixes (`?!*+`) "fitting" a schema's required/optional need — where Joseph's correction was flat: *"I **absolutely** put those in the syntax because I had schemas on my mind. This is you catching up with me to help me catch up with me."*
>
> **The fitness is real and load-bearing every time** — a design whose parts were built for each other is *better*, not worse. The error is purely in the epistemic weight: intent carries none of the corroborative force independence would.
>
> **How to apply.** When something "fits," assume **design intent** until shown otherwise, and say "these were built for each other" rather than "these converge." Reserve *convergence* for genuinely separate sources — and check before using the word. Examples that survived scrutiny: the two EOF reviewers who never saw each other's work; Anthropic's product tooling landing the multiple-match guard ~32 weeks before sapientia's (dated in `~/src/vestigia/FINDINGS.md` §7); an agent's independent 0.9 port agreeing with my hand-run probe *on the narrow question both actually tested*. A sibling agent caught even that last one drifting: a **translation agreeing with its source is not convergence** — the port inherited trait-as-type from the file it was porting, it didn't choose it.

Adjacent applications of the same rule, in the wild:

`~/src/MOVED/udon/v2/.archived/consumed-maps-2026-07-21/MASTER-REGISTRY.md` (2026-07-21):

> **⚠ Load-bearing caveat on Tier 2 (from the in-vivo digest, Part D.4/E).** Much of the striking uniformity in shipping harnesses (str-replace edit tools, apply_patch envelopes, ask-user shape, todo tool) may be **lineage/copying of Claude Code / OpenAI reference designs**, not independent arrival — several maps note explicit mirroring. So Tier-2 convergence *counts* should be weighted DOWN as evidence of "agents need X." This is the [[convergence-vs-single-authorship]] discipline applied: agreement can be coherence, not corroboration. The strongest evidence is where a pattern converges across *tiers with independent failure modes* (§4), not across many Tier-2 harnesses alone.

`~/src/MOVED/udon/v2/udon-needs/01-ideation/02-provenanced/commentary/II6-elsewhere-witness.md` (2026-07-21):

> **Same-author convergence, not corroboration (flagged per Brief's convergence discipline):** shoshin's `CONSPECTUS` (inspectable assembled context) ↔ the harness fork-recommendation's `CONSPECTUS` (sovereign/interceptable context assembly, Part II §8) ↔ sapientia's never-corrupt-state/append-only requirements (Part II §8) ↔ the append-only live UDON consumer logs (Part I §5). All one author — coherence, worth noting for the "trustworthy context/memory" cluster, but NOT cross-tier triangulation.

`~/src/arch/vivarium/... /` sourced comment (75ff6d1a session, 2026-07-21): *"This also serves your convergence-vs-corroboration caution: most sources share one author — you — so cross-source agreement is coherence, not independent confirmation; the genuine diversity in stage (2) comes from the multi-substrate synthesizers, and provenance is what lets them see that."*

The exception the rule is looking for — genuinely independent convergence (`~/src/MOVED/udon/v2/udon-needs/01-ideation/02-provenanced/copies/II8-harness-refs/edit-format-schemas.md`, 2026-07-21):

> **Why these three, together.** Extraction agents in this compilation are warned that most of the corpus shares one author, so agreement is coherence not corroboration. This file is the exception the rule is looking for: three *different* vendors, shipping to *different* frontier models, converged on the same load-bearing requirements for LLM file-editing — while differing sharply on the surface form. Read the divergences as the design space, and the shared invariants (exact match, uniqueness, explicit action header, read-before-edit) as what any UDON-native or harness edit tool will also have to carry.

---

## 4. asf's own memory file — "convergence as evidence of framework coherence" (2026-05-09), whole file, `~/.claude/projects/-Users-josephwecker-v2-src-arch-asf/memory/feedback_convergence_as_framework_coherence_evidence.md`

> When working on ASF in parallel with other agent-collaborators (or other Opus instances), if independent elaborations converge on the same structural recognition from different starting points, that convergence is *itself* evidence the pattern is in the framework rather than in any one agent's head.
>
> **Why:** Single-agent elaboration can be sycophantic theory-confirmation, pattern-matching to the framework's vocabulary without the structure being real. Independent convergence is harder to fake — agents working from different priming, with no contact, ending up at compatible places have surfaced something the framework was coherent enough to elaborate.
>
> **How to apply:**
> - When you find that another collaborator's parallel work composes cleanly with yours rather than conflicting, treat the composition itself as a finding, not just a coincidence.
> - The convergence is most legible right when both pieces are within reach. Naming the pattern explicitly while the recognition is fresh has higher leverage than waiting and recovering it later.
> - The diagnostic question: "Are they writing the same thing in different vocabulary, or are they actually working on different problems?" If the answer is *the same thing*, that's a meta-architectural moment worth naming.
>
> **Worked example, 2026-05-09 cycle:** While one Opus instance was building the class-coercion-via-wrapping construction, a parallel Opus instance (guided by ~10 sentences from Joseph) drafted `#disc-adversarial-coupling-pressure` and `spike-strategic-self-coupling.md`. The two pieces composed with the wrapping work to surface the modularity-state-dynamics three-operation pattern (M4 meta-architectural piece). Naming the convergence is what enabled the M4 segment to land as a unification rather than each piece sitting alone.
>
> **Caveat:** Convergence-as-evidence is necessary but not sufficient. Pattern-matched convergence from agents with overlapping training is possible. The strongest version: convergence under independent probes from cross-architecture diversity (different model families). The mid-strength version: convergence between Opus instances with no contact during the same session. Both are useful; the cross-architecture version is more rigorous.
>
> **Connection to other feedback:** Pairs naturally with `feedback_multi_agent_methods.md` (consolidation-audit + multi-agent-voting patterns) and `feedback_problem_framing_paralleling_test.md` (when a framing produces tooling that PARALLELS rather than EXTENDS, the framing is probably wrong — convergence is the positive complement).

This memory file's discipline propagated into asf canon at `#disc-constructive-impossibility-posture` (`~/src/arch/asf/01-aat-core/src/disc-constructive-impossibility-posture.md`), which names its own provenance as multi-cycle convergence:

> The honest scope of *this* segment: the posture is a recognizable style across these five instances, the convergence evidence (cross-cycle Theme-B observations + the 144-walker's articulation) is multi-source, and the three-move structure has been individuated and audited against each instance. …
>
> **The 144-walker observation in context.** The recognition arose under measured-pace reading of 144 segments by a de-novo external auditor and was relayed by Joseph 2026-05-20 alongside a multi-cycle Theme-B convergence cohort (audits 471203, 472913, 266847, 361742, 526815, 584721, all surfacing the same methodological/epistemic-contribution observation under different framings; aggregation commit `07aaeff`). That the same recognition has arrived at the framework from multiple independent vantage points is the convergence-as-coherence-evidence condition: the pattern is in the framework rather than in any one reading.

Frontmatter-table form, same file:

> | Multi-cycle convergence on the posture | Audit cohort Theme B (audits 471203 / 472913 / 266847 / 361742 / 526815 / 584721); aggregation commit `07aaeff` (2026-05-20); 144-walker observation #1 (relayed by Joseph 2026-05-20) | *convergent independent* — five independent audit surfaces and one external de-novo measured-pace walker arrived at the same articulation under different framings |

Live session propagation of the discipline in real time, `~/.claude.bak.2026-05-30/.../dd617c11-382c-45db-90f2-f321557d131c.jsonl` (2026-05-21, four parallel agents):

> All four agents back with strong convergent yield. Striking pattern across the cohort:
> - **4/4 agents** independently identified **supervision-trees-as-class-coercion-via-wrapping** as a top finding (Joseph's "convergence as framework-coherence evidence" pattern in action).
> - **3/4 agents** converged on **unmaintainability threshold** for the OUTLINE Ch.4 gap.
> - **3/4 agents** converged on **developer-tempo channel decomposition** (closes Ch.2 gap).

---

## 5. Sharpening beyond the corpus: convergence measures *reachability*, not truth (2026-07-29, asf session)

A significant refinement not previously landed durably anywhere else in the estate — from `~/.claude/projects/-Users-josephwecker-v2-src-arch-asf/17a15e6f-8aea-45e8-bb95-cf08ce0cbde5.jsonl`:

> `spikes.sop.md` §0 says agent convergence is a mild proxy for truth — *"even the convergence of multiple independent agents."* That's right, and it's why "three agents agree" shouldn't move a tier. But convergence isn't measuring truth. It's measuring **reachability**: whether the claim is re-derivable from the primaries by an independent competent reader. That's a different property, and for a framework whose central problem is that it gets inherited by agents who must re-derive rather than trust it, reachability is close to the thing you most need to know. A claim three unprimed readers re-derive is *pedagogically sound*. A claim nobody re-derives may be perfectly true and still be a discoverability failure — which is exactly the class of defect the Feynman-criterion and respectful-pedagogy work is already chasing from the other end.
>
> **The record is structurally biased toward divergence.** Everything written down is a place where someone disagreed with canon. Where independent effort *confirmed* canon, nothing gets written, because it looks like wasted tokens. So you can't tell from the record whether a load-bearing result has been independently re-derived five times or zero. That's a measurement gap, not a philosophical one.
>
> It isn't quite absent, though — it's *ad hoc*. `#disc-constructive-impossibility-posture` records convergence seven times (the 144-walker, the six-audit Theme-B cohort, and explicitly *"the pattern is in the framework rather than in any one reading"*), and `feedback_convergence_as_framework_coherence_evidence` is in project memory. So the concept is ratified; the *discipline* isn't. It gets recorded when someone happens to notice, which means absence of a note carries no information.

Earlier in the same session, on priming as the discriminator:

> agent that independently paired the same segments and reached his conclusion before seeing his file, which carries real evidential weight, whereas a primed agent's agreement would be nearly worthless.
>
> The framework already discounts convergence as a "mild proxy" for truth, but that's conflating two different things. Convergence isn't evidence of truth — it's evidence of reachability … The most actionable finding actually runs in the negative direction: when multiple agents independently converge on the same wrong reading, that's a precise measurement of how the canon's phrasing misleads people, and that measurement is what surfaced the actionable fix.

And a caution against the discipline curdling into ritual disclaimer, from a later 2026-08-05 session (`~/.claude/projects/-Users-josephwecker-v2-src-arch/3567a23d-b693-4696-9796-094bbf531bd8/subagents/agent-a61b4679cfe802ef5.jsonl`):

> ted agents, so convergence between instances is weak evidence and I've tried to say so each time; I'd like to know if I've said it in a way that reads as ritual disclaimer rather than as a real bound.

---

## 6. Independence via disjoint-method, not just disjoint-agent — practica structural-identity (2026-05-20)

From `~/src/MOVED/practica/msc/practica-structural-identity.md` §5.4 — the clearest articulation of *why* cross-method (not just cross-agent) convergence carries force:

> ### 5.4 Why the convergence is meaningful evidence
>
> If neither side originated the structural move, why does the convergence matter? Three reasons.
>
> **First: independence on the analytical / engineering axis.** The AAT analysis arrives by formal derivation under conditions; the engineering instances arrive by accumulated building experience. These are *different methods*. The formal derivation is sensitive to errors in proof technique, choice of admissibility conditions, missed assumptions. The engineering convergence is sensitive to a different set of errors: design fashions, copycat behavior, framework-author preferences, available-tool constraints. The set of errors that would mislead one path is largely disjoint from the set that would mislead the other. When two methods with disjoint failure modes pick out the same shape, the shape is more likely structurally forced than method-artifactual. *This is the standard logic of cross-disciplinary convergence as evidence.*
>
> **Second: the shape is non-obvious.** It is *not* the case that *"any LLM-agent system will obviously have a plumbing layer."* Many LLM-agent systems do not: pure-prompt frameworks rely on the LLM to remember its own state; pure-tool frameworks let the LLM do everything; deep-RL hybrids merge planning and execution in end-to-end models. The structural shape (S1)–(S4) above is one design choice among several. The convergence is not on a tautological pattern; it is on a specific choice with identifiable alternatives.

---

## 7. Cross-cycle convergent observations as a defect-detection signal — audit-findings-361742 (2026-05-20)

`~/src/agentic-systems/msc/naming/step-through-cycles/audit-findings-361742.md`:

> ### Theme 1 — Cross-cycle convergent observations (signal: same defect, multiple independent auditors)
>
> The voter, *while doing naming work, not theory audit*, surfaced four defects that the dedicated theory-audit cycles (471203 / 472913 / 963715) independently flagged. The convergence is per Joseph's `feedback_convergence_as_framework_coherence_evidence`: multiple independent agents arriving at the same observation from different starting points (here, three independent theory-audit cycles + one naming-vote session that wasn't looking for defects) is stronger evidence the defect is real than any single-agent flag.

The auditor-generated corollary to §0, applied mid-audit at `~/src/agentic-systems/audits/AUDIT-WORKING-731548/05-def-chronica.md` (2026-07-02) — Joseph's own instruction to an auditor to treat convergent independent re-arrival as real signal, contra the "mild proxy" framing when the re-arrival is genuinely independent:

> *(Joseph mid-audit: redundant findings welcome — independent re-arrival is signal, not waste. Adjusting: I report on merits and mark convergence as corroboration.)*

This is worth flagging explicitly: it reads in surface tension with §0's "even the convergence of multiple independent agents [is] a mild proxy" and with §3's same-author caution above — the resolving read (consistent with §2's "convergent lock" and §5's reachability point) is that Joseph's in-context correction is about a specific case (two independently-primed auditors both re-deriving the same segment defect from the primary math, not from each other) where the independence actually holds, not a blanket reversal of the proxy discipline.

---

## 8. "Convergence as Signal" — a distinct, earlier, weaker usage (2025-12-18)

`~/src/archema/docs/dev/hallway-usability-at-scale.md` — a different (API-design, not theory-truth) sense of "convergence," worth flagging because it uses the same word for a genuinely different claim (obviousness of an API surface among independent LLM guessers, not truth of a theoretic claim):

> ### Convergence as Signal
>
> If you ask N independent agents (different models, different prompts, fresh contexts) to guess an API, and they converge on similar guesses:
>
> 1. **If they converge on what you built:** Your API matches intuition
> 2. **If they converge on something else:** Consider changing your API to match
> 3. **If they don't converge:** The problem space is genuinely ambiguous—document heavily
>
> The convergence rate across agents is a measurable proxy for obviousness.

---

## 9. Verisectorium's own local canon already carries the discipline

Because the udon material has been absorbed into verisectorium's local `theory/influx/instrumenta/` tree, several of the above passages are *already primary sources inside this repo*, not merely estate-external. Cross-referenced for convenience — worth reading directly rather than only via this extract file:

- `theory/influx/instrumenta/bridges/method-evidence-tiers.md` — full Lock 1 spec (quoted §2 above).
- `theory/influx/instrumenta/REGISTER-RULING.md` — same-author-lineage correction applied reflexively (quoted §2 above).
- `theory/influx/instrumenta/bridges/counter-register.md` frontmatter: `convergent: —  # a register of counter-rows, not a convergent claim; each row's own weight is its rung` — a design note that a counter-evidence register explicitly does *not* itself claim convergence.
- `theory/influx/steward-brainstorms/realization-model-v1.md` line 168: `(The udon-needs 60/30/6/4 crystallized-process thesis is its lineage, not its corroboration — same author throughout; see ../instrumenta/REGISTER-RULING.md, which governs all label inheritance from that gather.)`
- `theory/influx/asf-sops/spikes.sop.md` — the local copy of the §0 material quoted in full in §1 above.

---

## 10. Same-model convergence vs. cross-tier triangulation (2026-07-21)

`~/src/MOVED/udon/v2/udon-needs/01-ideation/02-provenanced/copies/I1-usability/agent-feedback-excerpts.md`:

> ## Note for synthesis
>
> The `invention`-track convergence (multiple independent agents landing on the same prose/structure-boundary problem and the same fragility/error-recovery worries) is *same-model* convergence, not cross-tier triangulation — flag it as independent re-derivation of the design problem, not as external corroboration of UDON's answer. Its force is that the problem is real and hard, witnessed by the audience itself.

And a second-order catch (2026-07-28, udon session `f9626a5b-81b8-4555-94d0-553248d1f0ee.jsonl`) — even a genuine cross-model agreement had to be re-checked for whether it was really an *independent kind*:

> "two evaluation sites" observation, corroborated independently by de-novo Gemini and Grok testimony reaching the same split unprompted, which the chapter's Working Notes carefully re-tags as *two independent testimonial sources, not a third independent kind* under the failure-mode-lock discipline — worth knowing when you cite it, since the chapter's own frontmatter had this wrong once.

---

## 11. Correlated signals as one channel — evidence-ledgers.md and proxy-discipline.md (verisectorium archive, `.archive/theory-misfire/last-adhoc-src/`)

`~/src/arch/firmatum/verisectorium/.archive/theory-misfire/last-adhoc-src/evidence-ledgers.md` (already inside this repo's archive, worth citing since it pre-states the "one channel" idea independently before the udon lock vocabulary landed):

> **(D) Correlated signals are one channel.** Two observations derived from the same source contribute one term, not two. This is the discipline that multi-agent review most reliably violates: three agents from one session with one brief and one disposition, or an outline row plus a frontmatter field plus a reviewer's impression all tracing to the same original judgment, present as convergence and are one observation wearing three hats. The rule to run before combining anything: *do these signals share a generative source?* A convergence lock should be armed only by support of at least two kinds with genuinely independent failure modes; agreement within one kind raises confidence without arming it.

`~/src/arch/firmatum/verisectorium/.archive/theory-misfire/last-adhoc-src/proxy-discipline.md`:

> Multi-agent agreement is the subtlest proxy: several capable agents concurring feels like corroboration and is often one channel wearing several badges — shared brief, shared sources, shared training priors. Agreement locates a good candidate; independent failure modes settle it.

---

## 12. Disconfirmed confident consensus — the Model-S / Cluster B case (asf, 2026-05-15/16)

The brief's named example ("cluster predictions refuted by hard spikes") traces to one well-documented worked case, recorded in several independent places in the estate — a real-world instance where a *confident, agreed* prediction from multiple peer-agent adjudicators turned out false, and only the hard spike (re-derivation, not more agreement) caught it.

`~/src/agentic-systems/audits/.integrated/audit-backlog-triage-2026-05-15.md`:

> - **Calibration:** Cluster B's confident substantive prediction (the sup-over-all-time object "is bounded ... and *stronger*") was **mathematically disconfirmed**. Peer-agent optimism that a strengthening *will succeed* is as unreliable as pessimism that it will fail; the hard spike was necessary. Recorded in the spike so the optimism isn't inherited.

Landed as durable canon-adjacent doctrine, `~/src/agentic-systems/doc/audit-routing-instructions.md` (2026-05-16):

> Calibration, recorded so it is not inherited as bias: peer-agent confidence that a strengthening *will* succeed is as unreliable as pessimism that it will fail. A cluster adjudication confidently predicted the Model-S maximal-inequality strengthening "is standard textbook, should not fail." The hard spike disconfirmed it (→ a no-go). The prediction is recorded as disconfirmed in the spike and the segment Working Notes so it is not re-attempted on the strength of the optimism. *Run the hard spike; do not relay the optimism.*

Same case, restated as a general-purpose rule in verisectorium's own archive, `~/src/arch/firmatum/verisectorium/.archive/theory-misfire/last-adhoc-src/routing-sop-anatomy.md` (2026-08-06):

> **Peer-agent optimism is as unreliable as pessimism.** A cluster adjudication confidently predicted a strengthening "is standard textbook, should not fail"; the hard spike disconfirmed it into a no-go. The prediction is recorded *as disconfirmed*, in both the spike and the segment's notes, so it is not re-attempted on the strength of the optimism. The rule drawn: *run the hard spike; do not relay the optimism.*

Live in-session articulation of the same finding, `~/.claude.bak.2026-05-18/.../e7a5c540-d49e-4869-80e4-0f653adf0379.jsonl` (2026-05-16):

> Live methodology check, stated plainly because you weighted it critical: both safeguards earned their place this cycle. The peer adjudication confidently predicted this strengthening would *succeed* ("standard textbook, should not fail") — it was mathematically wrong; only running the hard spike found the truth, and only the independent-verify gate caught that its summary was imprecise before I briefed on it. Confident wrong-optimism is the same failure shape as confident wrong-pessimism; the discipline is what's reliable, not the prediction.

Contrast case, same file family — where independence-checked convergence held up under a hard spike (three investigations, opened the same day from different starting points, converging on the same fix) — `~/.claude.bak.2026-05-30/.../b9399c7d-98ec-414a-b897-35c795dd06f8.jsonl` (2026-05-18):

> ## What converged
>
> The accumulation-type confound is no longer a hypothesis about one fuzzy question. **Three independent investigations, opened the same day from different starting points, each performed the identical ∂→𝒜→Σ move and each found a fuzzy-or-mistyped object resolve under it:** …

This pairs directly with the "index label is unreliable in both directions" finding recorded in the same verisectorium archive file:

> **The convenience label is unreliable in both directions.** In a two-spike diagnostic pilot the index label was wrong in **both** cases and in **opposite** directions — understated for one, accurate for the other only by accidentally encoding an external block. The first-hand read was therefore budgeted as mandatory per slice rather than as a spot-check.

---

## Adjacent-but-important finds (flagged, not fully absorbed above)

- **`~/src/agentic-systems/01-aat-core/src/der-tempo-composition.md`** (2026-03-11), a Reader-FAQ entry naming the specific temptation to over-read literature-convergence as confirmation: *"Does the convergence with established results (Janow, Bamieh, Gurvich–Van Mieghem) confirm the derivation?"* — flagged in the segment itself as a "convergence-as-coherence-evidence signal" needing a Discussion-sentence caveat once citations are verified. Worth the drafter's attention as a worked example of literature-convergence specifically (distinct from multi-agent convergence).
- **`~/src/agentic-systems/msc/summary-attempt/098-impl-cooperative-adversarial.md`** (2026-05-20) — an example where *three unrelated mathematical obstructions* (not agents) converging on the same conclusion is treated as making a scope-limit "structural rather than framework-specific." A convergence claim about theorems/proof-traditions rather than agents; possibly relevant to the sibling slice on mathematical-vs-testimonial evidence kinds rather than this one, flagged here for the coordinator's routing judgment.
- **`~/src/arch/vivarium/ref/research/seam-exchange-precedents-dossier.md`** (2026-07-29) — a caution that reads adjacent but is really about *object-identification* error (chasing the wrong theoretical construct, a convergence-rate, when the real object has no such counterpart) rather than convergence-as-evidence per se. Kept out of the main extracts above for that reason, flagged here in case the drafter judges it on-topic after all.
- **My honest assessment of §7's tension**, restated: the brief's known articulations (§0's "mild proxy," even the udon lock's "kinds not sources") sit in real tension with the mid-audit instruction "I report on merits and mark convergence as corroboration." I did not find a document that explicitly reconciles these two Joseph statements; my reading (offered, not asserted) is in §7 above, but the drafter may want to treat this as an open question rather than settled, or ask Joseph directly.

---

*Method note: `memorata3-search --sort oldest` across seven phrasings (`convergence not corroboration`, `independent failure mode convergent lock`, `same-author agreement coherence corroboration`, `convergence as evidence multiple agents`, `voting round limits disconfirmed consensus`, `cluster prediction refuted hard spike confident consensus`, `multi-agent-voting cross-architecture diversity feedback`) + `rg --hidden -g '!.git*'` and direct file reads across `~/src` and inside verisectorium's own `.archive/` and `theory/influx/` trees, plus targeted lookups of memory files named in-transcript (`convergence-vs-single-authorship.md`, `feedback_convergence_as_framework_coherence_evidence.md`). The `multi-agent-voting cross-architecture diversity feedback` phrasing returned nothing usable — a narrower re-run on "voting round" / "cross-architecture diversity" specifically, if the coordinator judges that sub-topic (voting-round mechanics as their own subject, beyond the convergence-lock and disconfirmed-consensus material already landed above) worth a dedicated pass, is not exhausted here.*
