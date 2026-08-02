---
slug: machine-first-documents
type: finding
evidence: [T1, T2, T4]
status: re-derived across estate contexts + substrate-diverse shipped instances (same-author caveat carried; substrate diversity is the real signal)
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

**Claim.** Across at least four distinct estate contexts — and three
different runtime substrates — the same conclusion keeps being reached:
**the document is the natural unit of agent-facing knowledge and
configuration**, provided it is machine-first (structured, dense,
self-describing) rather than prose-first. In its strongest form the
document doesn't describe the system, it *is* the system: "agents ARE
documentation, not code that implements behavior."

## The evidence

- **The argument from failure:** the PRAXIS protocol opens with the
  indictment — human-centric prose documentation is an agent bottleneck
  (llms.txt files exceeding 800K tokens; short variants devolving to link
  lists that offload the parsing back onto the agent; scraped versions
  full of noise). Its solution shape: maximize actionable knowledge per
  token, in structured sections that separate *what it is* (definitions),
  *how it works* (interactions), *how to use it* (patterns), plus an
  applicability gate (when to select it, when not, preconditions) — "an
  agent cannot reliably use a tool if it only has an example snippet
  without its parameters or its error states." That quadripartite split is
  a document-format demand statement: knowledge modality boundaries want
  to be structural, not stylistic.
- **The "agents ARE documents" lineage (three substrates):** the
  MACH design (agent = YAML frontmatter + markdown body, hot-reloaded,
  schema-validated, the running agent "literally reads its own markdown to
  understand its purpose"); gemini's *shipped* `agents/*.md` with an
  output-contract template enforced by a 39KB linter — schema on agent
  *output*, machine-checked; and the sapientia Elixir consciousness
  compiler, where `## ` headers are the segment boundaries, frontmatter is
  the typed metadata, and the parsed document compiles to a running
  GenServer *by a transformation a human can replay by hand* ("documents
  become alive… zero magic"). Same author throughout — coherence, not
  corroboration — but the substrate diversity (Elixir OTP, gemini
  markdown-runtime, Ruby-era design) is genuine re-derivation of one idea
  against three different execution environments, and two of the three
  shipped.
- **The philosophical spine ([the pattern statement](../reports/the-pattern.md)):** the pattern-language work's
  living-documents
  section: traditional docs and code drift; a compiled document *cannot*
  drift ("documentation is always current — it IS the implementation"),
  with the extended-mind framing: the document is cognitive scaffolding,
  part of the agent's mind, not external storage — which is
  #persistence-is-imported's reinjection channel said with feeling.
- **The theory (the price argument):** the specification bound — minimum
  implementation time is bounded below by the time to transmit
  distinguishing information given shared context; **shared notation is
  compression** (DSLs, conventions, schemas reduce the residual entropy);
  as implementation automates, communication becomes the binding
  constraint. A document format that hosts several channels at once
  (prose, examples, structure, schema — UDON's structure-plus-prose
  thesis) is multiple sufficient channels in one transmission. With the
  over-transmission caveat carried: specification arriving faster than the
  receiver can integrate is *worse* than under-specification.

## What it generates

- **For UDON:** this is the demand its core thesis answers, stated by its
  future consumers before it rebooted: structure and prose in one object,
  typed metadata, schema-checkable, sections that are stable addressable
  units. The lineage also names the *governed-edit* requirement: MACH's
  hot-reload pipeline (validate → backup → swap → rollback) treats editing
  the agent-document as a transaction — #schema-guarded-mutation again,
  arrived at from the config side.
- **For the harness:** agent definitions, skills, praxes, and output
  contracts are one document class; the PRAXIS metadata fields
  (version, deprecation, dependencies, elaboration-level with
  more/less-elaborated links — progressive disclosure *in the metadata*)
  are a ready checklist for whatever the harness standardizes. The
  counter-weight to hold: sar2's alignment-speed non-reproduction
  (#counter-register row 1) cautions against assuming machine-first
  *formatting* alone buys comprehension — density and structure are the
  measured wins; notational elegance is not yet one.

## Honest edges

The breadth here is mostly same-author re-derivation plus one external
lineage (llms.txt/llm-min.txt); the only outside-the-estate corroboration
is that ecosystem's own token-bloat pain. "Documents compile to behavior"
has two shipped instances, both small; nothing here tests it at scale or
under adversarial edits. And the strongest form of the thesis quietly
assumes the governed-edit and validation tooling exists — without it,
living documents drift exactly like dead ones, just with more authority.
