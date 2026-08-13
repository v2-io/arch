---
slug: tool-definition-anatomy
type: finding
register: evidenced
support-kind: [observational, design, theoretic]
strength: robust-qualitative   # the anatomy is agreed across the ecosystem; reported as survivorship, never as independent votes
convergent: [observational, design]   # the shape uniformity is convention-adoption of one or two designs (descent-echo, carried as survivorship not as a vote); description-as-teaching is the cross-kind part
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: harness-primary
depends: [errors-that-teach, method-evidence-tiers]
sources:
  - ../../01-ideation/02-provenanced/syntheses/CONVERGENCES.md  # cluster 17; clusters on ask-user/todo
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # C12, C13, C14, C15
  - ../../01-ideation/02-provenanced/syntheses/tier2-lineage.md
---

# Tool-definition anatomy and the converged micro-tools

**Claim.** The shipping ecosystem agrees on what a tool *is* to an agent: **a name + machine-checkable parameters (in practice, a JSON schema) + a description that is a teaching surface** — the description carries the tool's law and usage wisdom, not just its signature, with longer guidance split into a separate file. Around that anatomy, a small set of micro-tools recurs with near-verbatim rules. The copying-vs-invention accounting from the [[method-evidence-tiers| methods chapter]] applies hard here: most of the uniformity is convention-adoption of one or two influential designs — reported below as survivorship (nothing displaced them in years of intense iteration), never as independent votes.

## The converged shapes

Per-harness detail for all of these lives in  
[[shipping-practice| the shipping-practice report]].

- **Ask-the-user:** one to four questions, each with two to four options, a recommended option first, and always a free-text escape. Probably a single design origin, copied nearly verbatim everywhere. The survivorship reading still carries content: a *structured* clarification affordance beat free-text asking everywhere it landed. And published measurement supplies the reason such an affordance is load-bearing: when a required parameter is missing, models tend to *fabricate a plausible value* rather than stop and ask (the [[structured-output-two-mechanisms| structured-output chapter]] carries that result) — so a cheap, structured way to ask is a direct counter to a measured failure.
- **The todo list:** the most uniform micro-convention in the landscape — near word-for-word rules across harnesses: one item in progress at a time; mark complete only after verification; never call something done with failing tests. Most-uniform means most-copied; the interesting question is why it survives contact with every model generation. The theory's answer: it is an externalized plan whose intermediate steps are *observable* — which turns the notoriously hard problem of "which step deserves blame" into bookkeeping. One plan-shape lesson from the same theory belongs beside it: under uncertainty, long dependent chains are mathematically punished while parallel alternatives are rewarded — a four-step chain at 90% confidence per step succeeds 65% of the time; three independent 50% options succeed 87.5%. A plan artifact that makes parallel fallbacks easy to write and long dependent chains awkward would be quietly load-bearing.
- **The delegation tool:** a fresh isolated context, a resumable identity, scope framing — and, repeated across genuinely separate implementations, **read-only roles enforced by leaving tools out, not by prose**. That last is a design law learned independently at least twice: the shipping ecosystem builds it in by construction, and this research programme learned it the hard way when an agent asked merely to *assess* which worktrees were safe to delete went ahead and removed all eight. The codified rule — constrain by tool-set, never by instructions — has two origins and one lesson: prose does not bound a capable agent; capability does. (The  
  [[delegation-as-tooling| delegation chapter]] carries the briefing half of this territory.)
- **Instruction files:** a per-directory file of standing instructions (commonly `AGENTS.md`), nearest-file-wins. One live disagreement, unresolved: a single harness treats these files as *untrusted data* with injection-precedence rules, while the rest of the ecosystem treats them as authoritative instruction — see the [[counter-register| counter-register]], which carries it as a security-relevant open question.
- **Description as teaching surface:** stated explicitly in the 2025 design work and visible throughout the shipping ecosystem — the description field is where a tool teaches its law *before* the first refusal; the [[errors-that-teach| refusal chapter]] is the ex-post half of the same channel.

## What it generates

- **For the harness:** adopt the anatomy and the micro-tool shapes as the empirical floor — they are what current models are trained against, so deviating carries a real familiarity cost (that is survivorship's practical content, stated as a positive design input). Enforce capability boundaries by tool-set composition. Treat descriptions and refusals as two halves of one teaching channel.
- **For UDON:** tool definitions, guidance files, and plan artifacts are exactly the document class UDON targets — structure, prose, and schema in one artifact. One shipped seed points at the generative direction: a CLI that exports *its own tool schemas* for agents to consume, sketching a world where tool contracts are authored once in a richer notation and projected into each vendor's schema dialect.

## What this opens (ideas, not designs)

- ✦ **Single-source tool contracts.** If the anatomy is name + schema + teaching description + guidance file, nothing requires authoring those four in four places. One could author a tool's whole contract — law, examples, refusal vocabulary, per-vendor quirks — as one structured document and project the JSON-schema fragment each vendor API wants. The export seed above suggests the direction is live.
- ✦ **Description-vs-law drift detection.** A tool's refusal stream reveals the laws agents actually collide with. Diffing that against the laws its description *teaches* would mechanically expose the gap — the description rewritten from evidence, the ex-ante channel kept honest by the ex-post one.
- ✦ **Plans with first-class alternatives.** The chain-vs-options numbers suggest a todo artifact whose syntax makes "or, failing that…" as cheap to write as "then…". Nothing shipped has one; the theory says the shape difference is worth twenty-two points of success probability in the worked example.
- ✦ **Pricing the familiarity cost.** "Models are trained against these shapes" is currently an argument. It could be a number: the same task run against a tool in its trained-shape dress and in a renamed, reshaped twin. Anyone choosing to deviate from convention would then know what the deviation costs.

## Honest edges

Uniformity here is the most inheritance-inflated in the whole landscape — this chapter deliberately makes no "N teams independently needed X" claim anywhere. And the anatomy is JSON-schema-shaped because the vendor APIs are; whether that shape is *right* rather than merely installed is untested — the benchmark and fabrication findings suggest reliability lives in the description and grounding layer, not the schema syntax.
