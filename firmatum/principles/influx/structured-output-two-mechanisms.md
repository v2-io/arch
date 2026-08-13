---
slug: structured-output-two-mechanisms
type: finding
register: [derived, evidenced]
support-kind: [observational, measured]
strength: robust-qualitative   # the two-mechanism distinction is derived from how each works; the sources themselves flag it
convergent: [observational, measured]   # shipped practice + external benchmark data, independent failure modes
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: "harness-primary (udon: payload/schema design input)"
depends: [tools-are-observation-infrastructure]
sources:
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # C9, Part D.3
  - ../../01-ideation/02-provenanced/syntheses/external-landscape-2026-07.md  # findings 3, 4
---

# "Structured output" names two different guarantees

**Claim.** Most shipping harnesses offer some machine-readable-answer mode, and the phrase "structured output" covers all of them — but it names two different mechanisms with two different guarantees.  
**Constrained decoding** enforces a grammar or schema *while the model generates*: malformed output cannot exist, because the sampler is not allowed to produce it. **After-the-fact serialization** takes whatever the model produced internally and renders it as well-formed JSON: the *transport* is guaranteed, the content inside it is as unconstrained as ever. Both are marketed under one name; they are different guarantees, and a design decision that conflates them inherits the wrong one.

## The evidence

- **Shipped, both kinds, sometimes knowingly:** of the fourteen harnesses examined at source, eight offer a structured-output mode. The examination catalogs true constrained decoding (Anthropic's strict mode, codex's output-schema flag, grammar-constrained patch emission) alongside formatter-class serialization — and at least one harness's own engineering log draws the distinction itself, honestly, about its own feature.
- **Structure changes the error profile; it does not remove error.** The Berkeley function-calling benchmark (published at ICML 2025; the finding survived two of our three adversarial checks, so it is carried at medium confidence) found structured calling modes produced substantially *more* incorrect calls than free-text prompting in its multiple-call category — counted among successfully decoded responses, in that one category. A companion finding, carried at high confidence: small-model failures split roughly 68% *omission* (a required field simply absent) to 32% malformation, attributed to insufficient schema grounding — large models showed nearly none of either in the same study. And when a required parameter is missing from what the caller gave, models tend to **fabricate a plausible value rather than ask**.
- **The edit-side echo:** one prominent tool tried routing file edits through structured tool-call arguments and abandoned the mechanism after models kept mangling the arguments (the [[edit-representation-landscape| edit-landscape chapter]] tells that story, and the [[counter-register| counter-register]] scopes it by ecosystem). Same lesson: pushing content *into* structure moved the failure; it didn't eliminate it.

## What it generates

- **For the harness:** state which guarantee each surface actually provides. Use constrained decoding where malformation is costly and the capability exists. Then design for the failure modes structure demonstrably does not touch: omission (the dominant small-model failure), fabricated parameters (a cheap structured way to ask is the evidence-backed answer), and wrong-but-well-formed content — the validation-passes-but-it's-false failure the [[counter-register| counter-register]] documents.
- **For UDON:** two implications, pulling in different directions. The case the [[tools-are-observation-infrastructure| observation chapter]] makes for sharp formats is about *reading* — observation design — and survives this chapter untouched. But any claim that structured **emission** improves reliability must be scoped by the benchmark result above: the honest pitch is verifiability-of-output, not error-elimination. The strong-guarantee path exists, though: UDON has a machine-readable grammar (the same one its parser is generated from), and constrained decoding against that grammar would make emission correctness *structural* — exactly the tier of guarantee the weak mechanism lacks. Worth an experiment before a claim.

## What this opens (ideas, not designs)

- ✦ **Guarantee provenance on every output.** A structured result could *declare which mechanism produced it* — constrained-decode versus serialized — so a consumer knows whether it holds a proof or a formatting promise. Today that distinction lives in vendor docs; nothing carries it with the data.
- ✦ **Omission-first validation.** If absence outweighs malformation two to one, validators have the emphasis backwards: the first-class question is "what's missing," not "what's malformed." A checker whose primary output is a filled-in skeleton of absent required material would match the measured failure distribution.
- ✦ **Asks generated from the schema.** The fabrication finding and the ask-the-user affordance combine naturally: when a required field is missing, the *schema itself* has everything needed to generate the structured question (field, type, allowed values, why it matters). The clarification loop becomes mechanical instead of hoped-for.
- ✦ **Emission-constrained streaming.** If UDON emission were grammar-constrained, a stream would be valid *at every prefix* — which is the [[streaming-and-partial-documents| streaming chapter]]'s territory arriving from the generation side. Whether the two mechanisms compose (constrained decode + incremental consumption) is an experiment nobody has run, in any notation.

## Honest edges

The benchmark figure is one category of one benchmark, carried at medium confidence; the omission split is small-model-only. Neither licenses "structure is bad" — they license precision about *which* structure buys *what*.
