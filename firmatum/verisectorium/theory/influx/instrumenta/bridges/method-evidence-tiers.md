<!--
  Verisectorium theory INFLUX — instrumenta mining, verbatim copy.
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/udon-needs/02-tooling-needs/src/method-evidence-tiers.md
  Copied: 2026-08-06 (mining agent, first outside reader of the theory corpus).
  This is a copy, not authority — do not edit here; segments draft from live sources.
  NOTE: the MOVED/ tree is mid-reorg; verify the source path before drafting from it.
-->

---
slug: method-evidence-tiers
type: method
register: decided          # this chapter states the report's conventions; a method is chosen, not measured
support-kind: —            # a method segment carries no evidence of its own
strength: —               # decided chapters take no strength rung (see "How the axes cross")
convergent: —
stage: drafted
consumers: both
verified:
  - 2026-07-22 · content · pilot-A · three-axis + two-lock + event-log schema landed and self-consistent
sources:
  - ../../01-ideation/02-provenanced/syntheses/CONVERGENCES.md
  - ../../01-ideation/02-provenanced/syntheses/tier2-lineage.md
  - ../../BRIEF-agentic-tooling-compilation.md
apparatus-note: >
  The gathering apparatus tagged evidence with codes T1–T5. Those codes are
  retired from this report's live vocabulary; they survive only as the
  auditor's historical mapping note at the end of this chapter, kept so that
  older provenance files stay decodable. Live claim metadata uses the three
  axes and two locks defined below.
---

# What this report counts as evidence

Every claim in this report can be placed — by a reader, from the prose alone — on **three axes** and under **two locks**. The axes say *what kind of support a claim has*, *how strong it is*, and *what kind of claim it is*. The locks say *whether independent kinds of support agree* and *whether the claim was checked at its source*. None of this apparatus needs to surface while reading; the prose carries what a reader must know in plain words, and the frontmatter carries the rest so the whole report is machine-auditable.

The reason for three axes rather than one blended label: **the axes vary independently, and collapsing them hides exactly the mistakes this report is most prone to.** A claim from the formal theory can be exploratory; a claim from a single agent's testimony can be a measurement. Reading "formal theory" as automatically strong, or "one agent's account" as automatically weak, is the error the separation prevents.

## Axis 1 — support-kind (how the evidence was produced, and how it fails)

Each kind is defined by its **repair**: the move that would strengthen a claim of that kind, or discharge it. The repair is the useful part — it tells a later agent what to *do* with a claim, and (by the anti-collapse test the formal theory uses on its own vocabulary) two kinds are worth distinguishing only when they route to different repairs. These six do.

| Support-kind | What it is | Repair (what strengthens or discharges it) |
|---|---|---|
| **design** | first-principles design reasoning, not yet exercised against reality (the 2025–26 agent-interface principles) | ground-validate: build it and see whether reality bears the reasoning |
| **observational** | a regularity seen across shipped systems or agent behavior | replicate it elsewhere **and descent-correct** — is the agreement independent, or one design copied? |
| **testimonial** | a first-person account from an agent of a tool serving or failing it | corroborate: independent voices, ideally across substrates |
| **theoretic** | a formal result — almost always **transmitted** from the theory, not derived here (see *Transmission*) | check the proof, or check the named premises hold in this domain |
| **measured** *(self / ext)* | an in-the-wild or benchmark quantity — `ext` = external published, `self` = our own recorded run | replicate; re-measure at wider conditions; corroborate the source |
| **synthetic** | evidence from constructed scenarios that were actually run (the stress tests) | validate that the scenarios are *representative*; re-run under variation |

**Kind is routed by repair, not by surface — a numeral does not force `measured`.** "Eleven of fourteen harnesses use exact-match editing" is `observational`: the claim rests on the *regularity's independence*, and its repair is descent-correction (are the eleven independent arrivals or one design copied?). "Format choice moved pass@1 from 14% to 57%" is `measured`: the claim rests on the *effect size*, and its repair is re-measurement at wider conditions. Same numeral-shaped surface, different kind, because the kind tracks what the claim rests on and therefore how it is repaired. (This is the rule the frontmatter sweep applies when tagging chapters; it is easy to route every count to `measured` on sight, and wrong.)

The kind tracks a claim's *current* support, not its origin: a `design` idea that later ships and is observed working becomes `observational` (its live repair is now replication), and the `design` leg persists only as the earlier, still-honest statement of the same idea.

## Axis 2 — strength (how defeasible the claim is)

A single graded ladder, borrowed almost whole from the formal theory's own status vocabulary, extended with two rungs this report needs for empirical and imaginative claims:

| Strength rung | What it means here |
|---|---|
| **exact** | A measurement or closed-form result; defeasible only if someone finds a mistake (detection rate zero; a theorem's algebra). "Exact" already carries that humility — do not pay for it twice by down-tiering. |
| **conditional** | A theorem-grade result that holds *under named premises* that travel with every use (the κ×A bias law; the persistence threshold). As strong as exact *given* its premises. |
| **robust-qualitative** | A direction or ordering that holds across many cases and would survive most reasonable perturbations, without a precise magnitude. |
| **measured** | An empirical number with its measurement conditions attached — carrying the caveats the source stated (single-repo, one model family, benchmark-era, N=…). Strong *at* its conditions, silent beyond them. |
| **heuristic** | A working rule that earns its keep in practice but has no proof and known exceptions. |
| **hypothesis** | A prediction not yet tested — every *proposed*-register claim lands here by construction. |
| **discussion-grade** | Reasoning offered to frame or provoke, explicitly not load-bearing; honest about being a starting point. |

The ladder is not a ranking to climb — a claim sits at the rung its support actually reaches, and "measured" is not "better than" "conditional"; they answer different questions. Where a claim's strength is load-bearing the prose *says the rung in words* ("measured, single repo"; "conditional on the drift assumption"); the frontmatter carries it so an auditor can sort by it.

## Axis 3 — register (what kind of claim this is, as a speech act)

Every claim is exactly one of four kinds, and the prose is written so a reader can tell which without apparatus:

- **Derived** — it follows from common knowledge or stated premises, and the reasoning is on the page.
- **Evidenced** — observation, measurement, or testimony supports it, cited where it stands.
- **Decided** — someone chose: a scope call, a convention, a project ruling. Decisions are legitimate — design runs on them — but they are never dressed up as derivations. Where the project's owner made a call, the text says so plainly (usually a link to the [[DECISIONS.md|design ledger]]) rather than manufacturing an argument that arrives where the decision already stood; and where a decision merely confirmed something obvious, the text says *that*.
- **Proposed** — an idea, generated here. This report is not only an organization of what exists; each chapter is chartered to *think* — to notice what its principles make conceivable that nobody has yet said. Such passages are voiced unmistakably as imagination ("one could…", "nothing yet built does…"), open possibility space without closing any of it, and decide nothing — the design work downstream owns that. **Proposed content is marked visibly** as **capability cards**: callout blocks `> [!capability] <name>` carrying, as a strong default, what it is · the principles that apply · the **hypothesized impact on the agent**, stated in the theory's precise vocabulary wherever that is the exact fit (observation ambiguity, update gain, tempo and the persistence condition, reinjection strength, law-teaching density, the context description-length budget… the full survey is [[theory-of-agentic-tooling| the theory report]]) — because that vocabulary names every aspect of an agent's fitness · what it stands in tension with · its downsides. The last two fields are the point: cards that name their own costs are what let the later priority stages weigh capabilities *against one another* instead of inheriting a verdict. A card whose territory genuinely lacks a field says so rather than manufacturing content. That a report like this must *invent* a typographic convention to mark a claim's register is itself a small demand datum for the notation work: registers-on-content is exactly what a structure-and-prose format could carry natively.

## How the axes cross

The three axes are orthogonal, and the machinery is smaller than it looks because two of the crossings are fixed:

- **Support-kind × strength cross freely.** A testimonial can be `hypothesis` (one voice) or `robust-qualitative` (corroborated across substrates); a `measured` outcome can be `measured`-strength or, at a single unreplicated data point, `heuristic`. Kind and strength move independently — the clearest proof is that strengthening a claim usually happens by *adding or changing its support-kind*, not by re-asserting it (below, *Max-attainable-status*).
- **Register fixes strength at two of its four values.** Only the two *truth-apt* registers take a strength rung. A **decided** claim takes none — a decision is not strong-or-weak; asking "how confident are we that we *chose* X?" is a category error (the question is only "did we choose it," and the answer is yes). A **proposed** claim is `hypothesis` by definition. So strength is carried only on the **derived** and **evidenced** claims; the other two self-tag.

So the per-claim machinery reduces to: **support-kind and register on every claim; strength on the derived and evidenced ones.** The two locks below sit on top.

## Lock 1 — convergent (does independent support agree?)

Agreement across kinds of evidence is this report's actual unit of proof. But `convergent` is **not a rung on the strength ladder** — it is a lock computed on top of it: a claim is `convergent` when **two or more support-kinds with independent failure modes** agree. A convergent claim still has a strength ceiling; the lock raises confidence *within* that ceiling by ruling out the failure mode any single kind is prone to.

**The lock keys on failure-mode independence, and that is the whole discipline.** Within-kind corroboration — two testimonials, three shipped tools doing the same thing — raises *strength* (a stronger testimonial, a wider observation) but does **not** arm the lock, because same-kind sources share a failure mode. Two de-novo testimonials from different substrates are still one kind of evidence with one blind spot; more voices do not make an interpretation *correct*. This is the report's own same-lineage-blind-spot thesis applied to its own epistemology, and the frontmatter enforces it: `convergent:` lists **kinds**, never same-kind sources.

**The repair for a convergent claim is: break the independence** — show that two legs actually share a failure mode (descent from one source is the usual culprit). If the independence breaks, the lock voids and the claim falls back to its strongest single leg. This is what makes descent-correction *mechanical* rather than a matter of remembering to be careful.

**Worked example (the lock spec's canonical case):** [[errors-that-teach| #errors-that-teach]] presents its refuse-on-multi-match principle as four-kind convergent — `design` (the built refusal tool), `observational` (eleven of fourteen shipping harnesses), `testimonial` (an agent's own account of the damage its absence causes), and `theoretic` (the law-teaching condition). But the `observational` leg is *mostly one influential design copied across the ecosystem* — so it is one partially-self-correlated leg, not a fourth independent failure mode. The honest lock is **three independent kinds plus a descent-echo**, not four independent kinds — and the chapter says so. The lock's failure-mode key is exactly what forces that audit instead of letting "4-tier convergent" stand unexamined.

## Lock 2 — transmission (was the claim checked at its source?)

This report mostly **transmits** the formal theory — it inherits results rather than deriving them. A transmitted claim is defeasible on **two independent grounds**: the source could be wrong, *or* our carriage of it could be infidelious. That second ground is real and this report has already caught itself on it (the κ×A "one knob" claim needed a wrapping caveat added so faithful carriage did not overclaim the source).

The encoding is deliberately light, because the eventual home — a group within ASF's software volume — already has the machinery. A transmitted claim is a **cross-volume reference** into the theory: [[scope-channel-collapse| #asf/llm/scope-channel-collapse]] carries the source's own status by reference; there is no separate "inherited" strength label (that would launder a non-exact source and duplicate what the reference already says). Two obligations travel with every transmitted claim:

1. **Prose form**, wherever a reader could mistake inheritance for local derivation: *"exact at source, under its premises; the risk here is carriage."*
2. **Plain-word gloss**, so a reader who cannot follow the reference still gets the substance on the page (the standalone-ownership bar).

The carriage-fidelity risk is **audited in the verification-event log**, not stamped as a label — "restatement checked against #asf/llm/scope-channel-collapse on DATE." And the honest-staleness note: an event records a check *as of a date*. A source segment can change after that date; inside ASF a `depends:` edge closes this automatically (the source's downgrade cascades to its dependents), but while this report is still a separate corpus there is no live edge, so **a known change in a referenced source is a re-verify trigger** — staleness is honest about the source, not only about our own prose.

## The verification-event log

Verification is recorded as an **append-only event stream**, not as a status a chapter attains and holds. (The four rigor checks a segment can pass — dependency audit, content review, mechanical, notes-disposition — are re-runnable instruments, not a ladder; a living document under flux gets re-checked, not re-certified.) Each chapter's frontmatter carries a `verified:` list; each entry records **what was verified · against what · by whom · when** — never "what state was attained":

```
verified:
  - 2026-07-22 · deps-audit · pilot-A · depends-chain present and at-or-above this chapter
  - 2026-07-22 · source · pilot-A · κ×A carriage checked against #asf/llm/scope-channel-collapse; wrapping caveat added
  - 2026-07-22 · deliberately-corrected-away · pilot-A · "5 harnesses vote" was one origin copied 5× — do not restore the cleaner count
```

The third kind earns its keep: a **`deliberately-corrected-away` event** records a form that was corrected *away from* on purpose, so a future re-verifier who notices the corrected truth reads messier than the original is told **not to "fix it back."** (Corrected truth is usually messier than what it replaced; without this marker, the next careful reader re-introduces the error in good faith.) An event goes stale honestly — a later edit can invalidate a prior event; the event stays, dated, and the next check appends.

## The frontmatter machinery (what an auditor sorts by)

Each chapter carries the axes and locks as fields, so the report is machine-auditable without any of this surfacing in the reading experience:

- `register:` — the dominant speech-act kind(s) (`derived` / `evidenced` / `decided` / `proposed`).
- `support-kind:` — the kinds present (`design` / `observational` / `testimonial` / `theoretic` / `measured` `(self|ext)` / `synthetic`). Replaces the retired `evidence: [T…]`.
- `strength:` — the headline claim's rung, for truth-apt chapters; a `decided` or purely `proposed` chapter omits it, and in-body claims that diverge carry their rung in prose.
- `convergent:` — the **kinds** whose independent-failure-mode agreement arms the lock, listed so the claim is auditable, not asserted (never same-kind sources).
- `verified:` — the append-only event log above.
- `stage:` — draft maturity, wholly separate from all of the above.

### Claim grain — proportionate, not per-line

The native claim-grain unit is the **capability card** (all cards are `proposed` / `hypothesis`). Load-bearing body claims that diverge from a chapter's headline carry their rung inline, in words. A **"what each leg carries" mini-table** — one row per claim, giving its support-kind and strength — is warranted **only** for chapters carrying three or more load-bearing claims at mixed strength: today those are [[typing-and-schema-boundary| #typing-and-schema-boundary]], [[context-economy| #context-economy]], [[edit-representation-landscape| #edit-representation-landscape]], [[counter-register| #counter-register]] (which *is* such a table already), and [[tools-are-observation-infrastructure| #tools-are-observation-infrastructure]]. No LaTeX-style equation tags — that is costume rigor for a report with few equations of its own.

### Max-attainable-status — the ceiling, and the action that raises it

Every claim has a ceiling: the strongest status it could *ever* reach. Note it where the ceiling is not obvious — *"Max attainable: measured. Currently hypothesis because no experiment has run."* — because knowing the ceiling makes strengthen-before-softening tractable (push to the ceiling, then stop) and prevents the false-modesty under-claim (don't leave something at `discussion-grade` when its ceiling is `measured` and the experiment is cheap).

And name the **evidence-action** that would move the claim, not just the ceiling value — because a ceiling is almost always raised by *adding or changing a support-kind*, which turns the whole system into a strengthen-before-soften to-do generator. Promoting "no silent retype" from `design` to `measured` is not more thinking — it is running the Norway-suite (`design` → `synthetic` / `measured`). Arming a lone testimonial's convergence is not re-asserting it — it is adding an independent kind. A capability card *is* such an action in disguise: it names the experiment that would move its own claim off `hypothesis`.

## The discipline this report holds itself to

1. **Agreement across independent kinds is the unit of proof** — the convergent lock above, with its failure-mode key. Most of the design work has one author; agreement between his own projects is coherence, not corroboration, and does not arm the lock.
2. **Shipped-practice counts are descent-corrected — and survivorship still carries positive design weight.** Much of the uniformity across shipping harnesses is inheritance from one or two influential designs, not independent invention: the patch-envelope format traces to a single published origin with zero independent arrivals; the ubiquitous exact-replace edit tool and the todo-list shape are convention-adoption of one design. Uniformity-by-descent is weak evidence that agents *need* a thing but strong evidence of what current models are trained against — a real design input, not a deflated count. Two patterns survive the correction as genuinely independent arrivals — the graduated fuzzy-match ladder and the headless I/O contract — and arm the lock at full weight.
3. **A claim's status is limited by its weakest *necessary* premise** — not by the mere presence of a weaker supporting source (that would punish honest inclusion of thin evidence), and not laundered upward by a strong source that supports only part of the claim. Each chapter says which part of its claim each kind of evidence carries. And the characteristic failure of synthesis writing — firming up caveats the sources stated carefully — is named here so it can be checked: where a source said "conditional," "single-repo," or "2024-era numbers," the chapter says so too.
4. **Counter-evidence rides adjacent to the claims it qualifies** ([[counter-register| #counter-register]]), never in a footnote graveyard.
5. **Theory results are conditional theorems, not vibes** — and the conditions travel with every use (the bias bound's named sub-scopes; the deliberation threshold's drift assumption).

If a passage seems to argue hard for something that needed no argument, treat it as a defect in this report and flag it.

One further fact about evidence, learned while writing this report: **readable, principled analysis is itself an elicitation channel.** When a chapter lays its territory out from first principles, readers remember things no search would have surfaced — shipped precedents, old rationales, buried counter-examples latent until good analysis fired the recall. (The addressing chapter's project-root anchor arrived exactly this way: the owner had forgotten a shipped precedent entirely until the principled treatment made it vivid again.) So late-arriving evidence is not a gathering failure; it is the analysis working, and this report is built to absorb it additively.

## Auditor's mapping note (historical — not live vocabulary)

The gathering apparatus tagged evidence with codes **T1–T5**, which appear throughout the provenance files. They are retired from this report's live vocabulary; the support-kind axis replaces them. The mapping, kept only so older files stay decodable:

| Retired code | Was | Now (support-kind) |
|---|---|---|
| T1 | first-principles design work | `design` |
| T2 | shipped-harness practice | `observational` |
| T3 | first-person agent testimony | `testimonial` |
| T4 | formal theory (ASF/AAT) | `theoretic` (usually transmitted) |
| T5 | external published research | `measured (ext)` — or `synthetic` for constructed-scenario studies |

**Who reads this and when:** both consumers, first — any downstream use of this report's claims should apply these axes and locks rather than re-deriving or ignoring them.
