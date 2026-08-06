<!--
  Verisectorium theory INFLUX — instrumenta mining, verbatim copy.
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/udon-needs/02-tooling-needs/src/machine-first-documents.md
  Copied: 2026-08-06 (mining agent, first outside reader of the theory corpus).
  This is a copy, not authority — do not edit here; segments draft from live sources.
  NOTE: the MOVED/ tree is mid-reorg; verify the source path before drafting from it.
-->

---
slug: machine-first-documents
type: finding
register: evidenced
support-kind: [design, observational, theoretic]
strength: robust-qualitative   # the same conclusion re-reached across estate contexts and substrates
convergent: [design, observational]   # the estate re-derivations share one author (single-author caveat carried); SUBSTRATE DIVERSITY in the shipped instances is the real independent signal
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: both
depends: [persistence-is-imported, tools-are-observation-infrastructure]
sources:
  - ../../01-ideation/02-provenanced/copies/II2-zoetica-ennaos/praxis-protocol.md  # head read; SKF/llms.txt argument
  - ../../01-ideation/02-provenanced/characterizations/III-vaults-agents-as-documents-lineage.md  # read
  - ../../01-ideation/02-provenanced/characterizations/II1-sapientia-elixir-consciousness-compiler.md  # read
  - ../../01-ideation/02-provenanced/copies/II4-autopax-practica/THE-PATTERN.md  # living-documents section
  - ../../01-ideation/02-provenanced/syntheses/asf-dossier.md  # §3.4 specification bound
  - ../../01-ideation/02-provenanced/syntheses/CONVERGENCES.md  # cluster 11
---

# Machine-first documents: the document is the interface, and sometimes the implementation

**Claim.** In at least four separate bodies of this research programme's design work — and on three different runtime substrates — the same conclusion keeps being reached: **the document is the natural unit of agent-facing knowledge and configuration**, provided it is machine-first (structured, dense, self-describing) rather than prose-first. In its strongest form the document doesn't describe the system; it *is* the system: "agents ARE documentation, not code that implements behavior."

## The evidence

- **The argument from failure.** A 2025 protocol for agent-facing knowledge documents opens with the indictment: prose documentation written for humans is an agent bottleneck. The ecosystem's own remedy attempts prove the point — machine-oriented `llms.txt` files ballooning past 800,000 tokens; the short variants devolving into link lists that push the parsing back onto the agent; scraped versions full of noise. The protocol's solution shape: maximize actionable knowledge per token, in structured sections that separate *what it is* (definitions), *how it works* (interactions), and *how to use it* (patterns), plus an applicability gate — when to select this knowledge, when not to, and what must be true first. "An agent cannot reliably use a tool if it only has an example snippet without its parameters or its error states." That four-way split is a document-format demand: knowledge-modality boundaries want to be *structural*, not stylistic.
- **"Agents are documents," three times, on three substrates.** One design makes an agent a markdown file — typed metadata up top, prose body — hot-reloaded and schema-validated, the running agent "literally read[ing] its own markdown to understand its purpose." One shipping harness carries agent definitions as markdown files with an output-contract template enforced by a 39-kilobyte linter — schema on the agent's *output*, machine-checked. And one shipped Elixir system compiles markdown documents directly into running processes — section headers become chapter boundaries, metadata becomes typed fields, and the transformation is simple enough that "a human can replay it by hand… documents become alive… zero magic." One author throughout — coherence, not corroboration — but three different execution environments, two of them shipped, is genuine re-derivation of one idea against three different walls.
- **The philosophical spine** (reproduced in  
  [[the-pattern| the pattern statement]]): traditional documentation and code drift apart; a *compiled* document cannot — "documentation is always current — it IS the implementation." Its extended-mind framing — the document as cognitive scaffolding, part of the agent's mind rather than external storage — is the [[persistence-is-imported| persistence chapter]]'s reinjection channel, said with feeling.
- **The theory prices it.** Minimum implementation time is bounded below by the time to transmit the distinguishing information, given what sender and receiver already share — so **shared notation is compression**: conventions, schemas, and dense structure reduce what must be transmitted at all. As implementation automates, communication becomes the binding constraint, and a document format that hosts several channels in one artifact — prose, examples, structure, schema — is several sufficient channels in one transmission. One caveat travels with this: specification arriving faster than the receiver can integrate it is *worse* than under-specification.

## What it generates

- **For UDON:** this is the demand its core thesis answers, stated by its future consumers before the language rebooted: structure and prose in one object, typed metadata, schema-checkable, sections that are stable addressable units. The lineage also names the *governed-edit* requirement from the configuration side: the markdown-agent design treats editing the agent's own document as a transaction — validate, back up, swap, roll back — which is the [[schema-guarded-mutation| guarded-mutation chapter]]'s demand arriving by another road.
- **For the harness:** agent definitions, skills, procedures, and output contracts are one document class, and the 2025 protocol's metadata fields are a ready checklist — version, deprecation, dependencies, and *elaboration level*, with links to more- and less-elaborated variants of the same knowledge: progressive disclosure carried in the metadata. The counter-weight to hold: the one measured comprehension experiment  
  ([[counter-register| counter-register]], row 1) failed to reproduce on one model family — density and structure are the evidenced wins; notational elegance is not yet one.

## What this opens (ideas, not designs)

- ✦ **Documents at N altitudes.** The elaboration-level metadata generalizes: one knowledge artifact maintained at several compression levels, each linking up and down, with the *agent choosing by context budget*. The context-economy machinery keeps proving documents get summarized anyway — this would make the summaries authored, versioned, and honest instead of ad hoc.
- ✦ **Applicability gates that execute.** The when-to-use / preconditions section is currently prose an agent reads. Stated as checkable conditions, the gate could *run* — knowledge that declines to apply itself out of scope, the way a well-designed tool refuses out-of-law calls.
- ✦ **The tool whose definition is its document.** This chapter and the [[tool-definition-anatomy| tool-anatomy chapter]] converge from two sides: if a tool's contract is a structured document and documents can compile to behavior, then a tool registry is a directory of documents — authored, diffed, schema-checked, and projected to vendor formats, with nothing hand-maintained twice.
- ✦ **Drift detection for living documents.** A compiled document cannot drift from its own behavior — but its claims about the *world* still can. Assertions in a living document could carry freshness marks (checked-against-what, when), so staleness becomes visible in the artifact rather than discovered in production.

## Honest edges

The breadth here is mostly one author re-deriving one conviction, plus a single outside lineage (the `llms.txt` ecosystem's token-bloat pain). "Documents compile to behavior" has two shipped instances, both small; nothing tests it at scale or under adversarial edits. And the strongest form of the thesis quietly assumes the governed-edit and validation tooling already exists — without it, living documents drift exactly like dead ones, just with more authority.
