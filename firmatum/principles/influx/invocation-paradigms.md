---
slug: invocation-paradigms
type: finding
register: evidenced
support-kind: [observational, measured]
strength: robust-qualitative   # three coexisting paradigms; the choice is conditional on model and workload, not settled
convergent: [observational, measured]   # shipped practice + external measurement; shapes are lineage-annotated
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: harness-primary
depends: [tool-definition-anatomy, context-economy, structured-output-two-mechanisms]
sources:
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # C7, C10, Part C code-mode
  - ../../01-ideation/02-provenanced/copies/harness-workshop/ai-cli-tools-source-assessment.md  # §7 convergent innovation
  - ../../01-ideation/02-provenanced/syntheses/external-landscape-2026-07.md  # finding 3
---

# Invocation paradigms: per-call JSON, code mode, and constrained freeform

**Claim.** Three ways for a model to invoke tools now coexist in shipped practice, and they distribute the reliability burden to different places. The choice among them — like the choice of edit format — is turning out to be conditional on model and workload, not a solved default.

1. **One JSON call per tool per turn — the installed base.** Every loaded schema costs context (the [[context-economy| context chapter]]'s deferred-loading machinery exists to blunt this); every call is a round-trip; and the arguments arrive as streamed JSON fragments the harness must reassemble and guard — the standing tax the [[streaming-and-partial-documents| streaming chapter]] documents. The reliability burden sits on the model's schema adherence, which is exactly where external measurement located the error profile: omitted fields and fabricated parameters (the [[structured-output-two-mechanisms| structured-output chapter]]).
2. **Code mode — tools as a callable API in a sandbox.** The model writes a small program that calls host tools as functions. Two harnesses built it independently (one parsing the model's JavaScript before running it, one embedding a JavaScript engine), and Anthropic then shipped an official third — a source-level survey of the shipping CLIs, conducted inside this research programme, flags it as *convergent innovation*, the newest paradigm in the landscape. What it buys: orchestration logic — loops, conditionals, intermediate variables — collapses from N round-trips of call-and-wait into one artifact, and bulky intermediate results never transit the context window at all. What it costs: the reliability burden moves to *program correctness* inside a sandbox, and per-step observability has to be rebuilt — the teaching refusal that fired once per tool call (the [[errors-that-teach| refusal chapter]]) now fires inside a program run, where nobody is watching unless the sandbox makes them able to.
3. **Grammar-constrained freeform.** In one harness, the input to the edit tool is not JSON at all but a small domain language, and the decoder is constrained by its grammar — malformed input is not detected but *impossible*. The strongest guarantee tier available, currently applied only where the payoff is highest: edits.

Deferred tool loading — register tools by name only, fetch a schema when first needed — cuts across all three paradigms; it is what makes large tool catalogs viable at all. (Most of the ecosystem inherited it from one origin, with two or three teams plausibly arriving independently — the [[context-economy| context chapter]] tells that history.)

## What it generates

- **For the harness:** treat the paradigm as a *routing decision*, not an identity. One harness already ships rival tool suites side by side to compare them in production — the honest posture toward an unsettled question. The evidence-backed default: per-call JSON for sparse tool use; code mode for orchestration-heavy work with bulky intermediates (its context payoff is the pruning family's, without the extra model); constrained freeform wherever a domain grammar exists and malformation is expensive.
- **For UDON:** two openings. In code mode, tool *results* are values in a program rather than text in a context window — a format with an honest programmatic surface (typed values, stable addresses) becomes more valuable there, not less. And constrained freeform is the shipped precedent for grammar-constrained *UDON emission* — the same guarantee tier applied to the notation itself; the [[structured-output-two-mechanisms| structured-output chapter]] names the experiment.

## What this opens (ideas, not designs)

- ✦ **The program run as a document.** Code mode's observability regression has a document-shaped answer: the sandbox could emit the whole run — each in-program tool call, its arguments, its result or teaching refusal — as one structured trace artifact. The per-call learning channel comes back, and the run becomes auditable and handoff-able rather than ephemeral.
- ✦ **Composed guarantees.** Nothing prevents a code-mode program from emitting constrained-freeform payloads for its riskiest operations — orchestration in code, mutation under grammar. The paradigms are presented as rivals; they compose, and no one has tried the composition.
- ✦ **A measured router.** The rival-suites experiment generalizes: a harness could route invocation paradigm *per task class* from its own measured outcomes rather than by fashion — the same move the [[the-crystallized-process-thesis| crystallized-process chapter]] proposes for model-tier routing, applied one level down.

## Honest edges

Code mode is new enough that its failure modes are folklore, not data — no benchmark like the function-calling one exists for it yet, and the observability regression above is this report's inference from its structure, not a measured finding. The three-way convergence includes one vendor whose designs the ecosystem demonstrably copies, so expect the instance count to inflate without adding evidence.
