# The agent-utility exploration (surfaces demand map)

**How to read this.** The working notes of the companion exploration: what surfaces agents need for generating, streaming, repairing, editing, remembering, and round-tripping documents — the demand side of every tool question that isn't addressing itself. Its harvest list (§9) is the densest single page of agent-facing demand in this book.

> **Provenance.** Promoted to the body of this report 2026-07-22. Refinements: this framing introduction; nothing else touched — the text below is the assembled original (gathered 2026-07-21; original file paths in its own frontmatter, which is auditor apparatus).

---

<!-- auditor apparatus — original gathered frontmatter:
- - -
source: second-pass agent-utility demand spike (Grok, night of 2026-07-20/21); free-form NOTES
gathered: 2026-07-21
status: gathered source material — NOT an authoritative decision document
paths:
  - v2/.archived/second-pass/spikes/agent-utility/NOTES.md
  - v2/.archived/second-pass/spikes/agent-utility/README.md (orient only; not copied)
  - design/udon-agentic.md, design/agentic-ux-principles.md, ux/TODO-AGENT-UX.md (inputs mined into this spike)
categories:
  - agent-edit
  - streaming-partial-doc
  - payload-handoff
  - round-trip-span-splice
  - paths-dependency
  - P-A-through-P-H
why_included: Highest-density in-repo demand residue for agent surfaces (generate/stream/edit/payload/fmt). Not yet present in gathering; needs-map only *cites* it. Older design docs restated here against pipeline vocabulary — keep both this and the design originals when they differ in texture.
- - -
-->


# Agent-utility notes

Spike residue. Free-form. Wrong is fine.

---

## 0. Starting itch

UDON’s primary consumers are agents. PROCESS is agent-primary. Yet most of the deep agent tooling design still lives in Dec-2025 / Jan-2026 design docs, partly superseded by CORE 0.9, with critical path named (paths → schema → edit) but not demanded against the pipeline vocabulary now in flight (recognition / assembly / resolution / evaluation).

This spike asks: **what surfaces must the language + suite expose so agents can generate, stream, repair, address, tool-I/O, remember, and round-trip — without making the steward the bottleneck for agent ergonomics?**

Not “what tools should we ship first” as a product roadmap. More: which **properties of the format and of stage products** agents lean on, and which gaps keep showing up in lived design.

---

## 1. Generation surface (streaming, repair, partial docs)

### What agents do that humans don’t

Agents emit bytes in long runs, often without a visual feedback loop. They stop mid-document. They resume after tool calls. They produce almost-valid structure with wrong indents. They need **semantic feedback while writing**, not colors on a screen.

The Dec-2025 brainstorm called this “the agent equivalent of syntax highlighting” (`UDON-AGENT-TOOLS.md` Tier 1; still open in `TODO-AGENT-UX`).

### Substrate that already exists

- Pushdown / `TreeStream` (2026-07-15): streaming parse, partial tree.
- Same wire from pushdown and one-shot RD (differential property).
- Incomplete-input is **not an event** (R2 / C6 direction): a **recognition verdict**, not something you recover by folding harder.

So generation/repair is not “finish the AST or fail.” It is:

```text
bytes-in-progress
  → recognition (events + recognition-verdict)
  → optional mid-assembly peek (open stack, open attr, text run state)
  → agent continues / repairs / hands off
```

### Affordances agents need here

| Need | Why |
|------|-----|
| **Partial-tree query mid-parse** | Open-element stack, current attr, depth — “where am I?” while generating |
| **Early anomaly surfacing** | Catch illegal geometry / bad attr shape before 500 more tokens |
| **Prefix / candidate validation** | Schema or enum: “`dra` is valid prefix of `draft`” |
| **Grammar-constrained generation** | Derive decoder grammar from descent; guaranteed-valid local models |
| **Interrupt / resume fidelity** | Partial tree + verdict is the handoff substrate when generation stops |
| **Repair without full reparse of intent** | Agent says “fix indent / close open element / complete attr”; tool owns spatial fix |

### Keep-everything meets generation

R11 / keep-everything at recognition: content kept with warnings where coherent; halt/reject is **consumer menu**. That is agent-friendly for streaming partial docs — the recognition product can stay honest while assembly or a careful-profile gate refuses to commit.

**Tension:** generation wants soft recovery mid-stream; careful writes want mutation-free refusal. Same language; different **stage + profile**.

### Multi-line / line-bound (ML in OPEN)

OPEN marks multi-line policy **WAIT-DEMAND**: needs paths + agent edit/stream evidence, not supply-side pin. Generation/repair is exactly the demand side that should force this:

- Can an agent stream a multi-line attr value without guessing R3?
- Does repair of a partial multi-line construct need self-delimiting value extent (W1)?
- Does line-bound policy change mid-generation feedback shape?

Until those hurt in concrete scenarios, do not nail ML.

---

## 2. Tool I/O / ACP-style payloads

### Thesis (still live, recontextualized)

`UDON-AS-ACP-FORMAT.md`: not “invent a new protocol” (predates MCP dominance), but **UDON as payload convention** inside MCP / harnesses / handoffs.

What that buys agents that pure JSON often doesn’t:

- Mixed structure + prose in one object (explanation rides with data)
- Nested semantic layers (structure first, semantics later — streamable)
- Strippable annotations for metacognition
- Same query / diff / merge surface as ordinary documents
- Handoffs that are documents, not ad-hoc blobs

### Tool suite shape (`udon-agentic.md` + principles)

Core read path: **glance → focus** (progressive disclosure; P5 orient cheap).  
Core write path: **propose → apply** (or atomic edit that validates inside the transaction; P4 tempo).

Principles that dominate sketches when they disagree (`agentic-ux-principles`):

1. Tool loop = epistemic organ (deterministic, honest readout)
2. Errors mutation-free + revelation-rich + law-rich (error-as-menu of exact paths)
3. Drive observation ambiguity toward zero (codes, counts, paths — not interpretive prose in the result channel)
4. One-call resolution over edit→check→revert loops
5. Structural vs parametric failure named in the error
6. Paths re-resolve at write time (never trust cache over file)
7. File’s own schema/law governs (declared ≠ theater)
8. Agent works in structure; tool owns indent/escape/spatial render
9. Confidence as first-class output; quiet success

Joseph priority signal (TODO-AGENT-UX): the **principled agentic edit** — atomicity, right indents, conformant with file’s spec. Critical path:

```text
path syntax → schema syntax → serializer/round-trip + spans
  → edit v0 (atomic + indent + syntax-valid)
  → schema conformance v1
```

Staged: v0 can be syntax-only; schema later.

### Payload conventions agents need (regardless of tool names)

| Surface | Agent need |
|---------|------------|
| **Skeleton / path map** | Copy-pasteable paths; multiplicities; attr names without full body |
| **Focused subtree + breadcrumb** | Where am I; siblings summary; refs in/out |
| **Diff structural, not only line** | What changed in tree terms; move vs delete+add |
| **Impact / side-effects** | Broken refs, inherited changes |
| **Validation verdict** | Conforms / violates rule R at path P |
| **Confidence** | high/medium/low + reason |
| **Teaching refusal** | Candidates as ready-to-use exact paths; stale-model hypothesis on zero match |

### Annotation / metacognition layer (open syntax)

Agents want strippable, queryable residue: confidence, source, decision, uncertainty — without polluting content. Dec form `|{@ ...}` is **not** valid under 0.9. Needs convention ruling (named element / reserved trait / new syntax) — currently `*(discuss w/ Joseph)*` in TODO-AGENT-UX. Same family as `;?` / TBD / `.draft` markers from older feedback.

Until then: **convention experiments in house styles are fine; do not invent CORE syntax in a spike.**

### Semantic merge

Only Tier-1 idea from `UDON-AGENT-TOOLS` that `udon-agentic` never absorbed. Multi-agent concurrent edit of the same doc is real in agent workflows. Merge needs structural awareness + annotation accumulation. Depends on paths
+ identity + maybe ornamental policy (what counts as conflict).

---

## 3. Memory / chunking / addressing

### Addressing is load-bearing

Almost every agentic affordance bottoms out on **paths**:

- `at` / `all` (exactly one or error vs explicit plural)
- Type-scoped uniqueness `(element-name, key)` — CORE, not only design docs
- References as subset of path syntax (ruled direction; design still open)
- Skeleton lines that are valid paths
- Error-as-menu of paths
- Span-splicing edits without canonicalizing whole files

`udon-paths.md` is **stale input**, not design of record. Live work: AUX / adjudication packet. This spike does not redesign paths; it records the dependency: **agent-utility is blocked on paths the way edit is.**

### Self-chunking / RAG (unmeasured claim)

README-level claim: element boundaries are intentional chunk boundaries for retrieval. Nobody has measured it (`TODO-AGENT-UX`). Experiments would compare:

- Element-boundary chunks
- Heuristic / fixed-size chunks
- Optional tooling that *emits* partitions as UDON or as payload envelopes

If true, recognition/assembly products should preserve enough structure for chunk emission without re-indent archaeology. If false, kill the claim cleanly.

### Handoff / compaction / memory

Three related but distinct channels (principles doc scopes **in-loop** vs **cross-session** carefully):

| Channel | Need |
|---------|------|
| **In-loop session** | Navigation state, staged edits, undo — performance only; write semantics still fresh-resolve |
| **Handoff document** | Next agent: structure skeleton + high-confidence annotations + continuation note; compressed prose |
| **Persistent memory** | Decisions, uncertainties, todos, project context as UDON (or MD today) that reloads next session |

Live consumers to design against: ASF process maps; this repo’s own STATUS/OPEN/DECISIONS discipline; Grok experimental memory (markdown + index) from sibling spikes.

### What memory wants from the format

- Stable identity (keys) so “the same decision” survives rewrite
- Annotations or traits that mark decision vs draft vs uncertainty
- Structure that survives summarization (keep skeleton, compress prose)
- Extract/query over mixed documents
- Soft/hard interleave (`udon-guarantees`): metrics stay hard; rationale soft

---

## 4. fmt / round-trip / ornamental

### Why agents care (not aesthetics)

Edit tools need **span-splicing**: untouched regions stay byte-identical. That is the opposite of “always pretty-print the whole file.” Minimal changeset cost (TST / proximity) wants:

- Serialize a **subtree** with correct indent relative to insertion site
- Escape correctly without agent thinking about it
- Round-trip enough of the model that re-apply is idempotent under stable paths (lens GetPut/PutGet/PutPut conditional on addressing)

`fmt` as whole-file house style is a **different product** — useful, and M5-shaped — but not a substitute for edit substrate.

### Ornamental criterion (from pipeline discussion)

Joseph’s testable criterion (paraphrased):

```text
original.udon → (drop ornamental) → model → house-style.udon
house-style.udon → (drop house ornamental) → same model → same house-style.udon
```

Ornamental = discretionary geometry (extra blanks, alignment padding, indent width beyond minimum) that changes **look** without changing the assembly product (except trivia namespace for exact byte round-trip). Comments are **not** ornamental (they’re nodes).

### Agent-facing consequences

| Product | Agent use |
|---------|-----------|
| **Byte identity** | Patch/diff against disk; span splice |
| **Recognition identity** | Same events + same recognition-verdict (± trivia) |
| **Assembly model identity** | “Did my edit change structure/content that matters?” |
| **House-style emit** | Optional normalize for human review or CI |

Agents mostly want model-level certainty + local spatial correctness, not global pretty. Humans want fmt. Both are real; conflating them produces bad edit tools.

### Soft/hard guarantees (`udon-guarantees`)

Profiles casual / careful / critical: same notation, different enforcement.  
Agent edit tool is the **careful** gatekeeper for writes that flow through agents. Critical may need store/query later — not generation-surface day one.

---

## 5. Recognition vs assembly products

Pipeline vocabulary (deliberation, not suite law yet):

```text
bytes
  → Recognition  → (event stream, recognition-verdict)
  → Assembly     → document model (ADM-ish) ± trivia
  → Resolution   → keys/refs/mixins/dialects/schema as policy
  → Evaluation   → host artifacts (templates, native types, …)
```

Sufficiency / no-reachback (W0 lean): each stage’s product must suffice for the next without re-reading source. That is also the public-API promise for “bring your own fold/consumer.”

### Which product agents need, when

| Agent activity | Primary product | Why |
|----------------|-----------------|-----|
| Stream generate / mid-write feedback | Recognition events + open stack + verdict | Incomplete docs are normal |
| Grammar-constrained decode | Recognition state machine | Next-token legality |
| Structural glance / focus / path resolve | Assembly model (or streaming assembly of closed subtrees) | Paths address model nodes, not raw events |
| Schema validate / careful apply | Resolution (or assembly + schema pass) | Conformance is a verdict |
| Liquid/template run | Evaluation | Needs host context |
| Semantic diff / merge | Assembly (± resolution for refs) | Tree operations |
| Memory chunk emit | Assembly boundaries | Element extents as chunks |
| Exact patch / span splice | Spans from recognition + model path | Bridge wire↔model |
| Handoff compress | Assembly skeleton + selected annotations | Drop evaluation noise |

### Dual assertion (C5)

Fixtures asserting **events and assembly product** matches agent needs: agents sometimes care about wire honesty (streaming, keep-everything, incomplete), sometimes about “what tree did I get.” Profiles idiomatic / comprehensive / descriptive are harness concerns; agents consume the same products with different strictness knobs.

### Streaming assembly

Not a second wire. Mode of assembly: emit completed root/subtree when it closes. Agent tool responses can stream layers (structure first, semantics later) as in ACP-format examples — that is assembly/evaluation pacing, not new recognition events.

---

## 6. What the design corpus keeps re-deriving

Convergence worth holding (not as proof, as signal):

1. **Fail-on-ambiguity at write boundary** (`at`) with assistive menus
2. **Atomic multi-site edits** (`all` as one transaction)
3. **Validate inside the write**, not post-hoc
4. **Progressive disclosure** for read
5. **Structure-first mental model**; tools own whitespace
6. **Format as protocol payload** for tool and handoff I/O
7. **Annotations as first-class metacognition**, strippable
8. **Soft/hard interleave** without splitting into two formats

Independent legs claimed in principles doc: ELI phenomenology, Anthropic product tooling, UDON spec discipline. Treat agreement carefully when sources share authorship.

---

## 7. Gaps and dead ends noticed

| Gap | Notes |
|-----|-------|
| Paths not designed | Blocks edit, query, error-as-menu, ref subset |
| Annotation syntax invalid under 0.9 | Convention vs CORE ruling needed |
| Semantic merge underspecified | Real multi-agent need, no suite home |
| Self-chunking unmeasured | Claim or kill |
| Cheat-sheets / usability harness stale | After compliance pins behavior |
| Mid-generation affordance layer | TreeStream exists; agent API doesn’t |
| Grammar artifact not derived from descent yet | Technique known; harnessable |
| udon-paths / many Dec examples stale | Read for shape; CORE + TODO for truth |
| Multi-line WAIT-DEMAND | Must not pin from this spike alone |
| Cross-session vs in-loop | Easy to conflate; principles warn |

Dead end to avoid: building a mega-tool suite before paths + round-trip + partial-tree query. The principles’ build order still looks right.

---

## 8. Provisional proposals (not law)

Boundary demands that *appeared* while reading — **proposals only**. Do not treat as DECISIONS or OPEN rows until harvested deliberately.

### P-A. Stage products as public agent surfaces

**Proposal:** Suite vocabulary should name recognition-product, assembly-product, resolution-product, evaluation-product as **consumable contracts**, not only internal parser stages — because agent tools naturally bind to different stages.

**Demand on pipeline:** W0 sufficiency stated per boundary; agent docs say which product each tool reads/writes.

### P-B. Recognition-verdict channel for partial generation

**Proposal:** Incomplete-input and related generation stops surface as  
**verdicts** agents can branch on, not as malformed ASTs or silent truncation.

**Pairs with:** C6 / OPEN C6; mid-generation feedback item.

### P-C. Edit tool binds assembly + optional resolution

**Proposal:** Default careful edit: re-resolve paths against current file → mutate assembly model → syntax-validate → (if schema bound) resolution check → span-splice write. Never mutate from a cached recognition-only guess.

### P-D. Ornamental is out of agent happy-path

**Proposal:** Agent write path preserves non-touched bytes (byte identity locally); does not run house-style fmt unless asked. Ornamental fixpoint is a separate tool/profile so agents don’t pay full-file rewrites.

### P-E. Payload convention > new protocol

**Proposal:** Keep north star: UDON documents as tool results / handoffs / traces **inside** MCP and similar. Do not re-open “build ACP as competitor protocol” as v2 work.

### P-F. Annotation layer as WAIT-DEMAND or steward-touch syntax

**Proposal:** Until syntax is ruled, agents use host conventions (traits, named note elements, out-of-band sidecar) for metacognition; CORE stays silent rather than inventing `|{@`.

### P-G. Multi-line / value extent demand from agent stream+edit

**Proposal:** When paths and mid-generation spikes exist, run scenarios that force multi-line attr values and partial multi-line constructs — let **pain** choose among greenfield strawmen. Until then ML stays WAIT-DEMAND.

### P-H. Memory chunk experiment as harness, not CORE

**Proposal:** Self-chunking evidence lives in UX/eval harness; CORE only needs to keep element extents recoverable from assembly product.

---

## 9. Top agent-facing needs (harvest list)

If this spike is only remembered for one page, remember these:

1. **Stable structural addressing (paths)** — write `at`/`all`, error menus, skeleton, refs, merge, focus.
2. **Principled edit** — atomic; tool-owned indent/escape; mutation-free teaching refusals; apply-time re-resolution; syntax then schema.
3. **Mid-generation / partial-document fidelity** — open stack + recognition verdict + early anomalies (semantic streaming).
4. **Stage-appropriate products** — events for stream/constrain; assembly for tree ops; resolution for careful validate; don’t force one “the AST.”
5. **Progressive read** — glance/skeleton before focus/full; context window as design constraint.
6. **UDON as tool/handoff payload** — structured + prose + optional annotations; streamable layers; same tooling as documents.
7. **Round-trip / span-splice substrate** — edit without full pretty; model identity vs ornamental vs bytes kept distinct.
8. **Metacognition residue** — confidence/decision/uncertainty queryable and strippable (syntax open).
9. **Handoff & compaction** — preserve structure and decisions; compress prose; continuation point explicit.
10. **Soft/hard guarantees dial** — casual explore vs careful agent write vs critical store; same notation.
11. **Grammar-constrained generation** (local models) + harness measure of invalid rates.
12. **Evidence for chunking / multi-line / merge** before pinning those as law.

---

## 10. What not to do from this residue

- Do not implement tools here
- Do not edit DECISIONS or live CORE from these proposals
- Do not treat Dec-2025 syntax as current
- Do not nail multi-line, wire vocab, or annotation syntax supply-side
- Do not require the next spike to fill a template

---

## 11. Possible next explorations (optional, nonprescriptive)

- **Paths spike** — demand scenarios from edit + error-as-menu + ref subset
- **Partial-tree API sketch** against real TreeStream (read-only experiment)
- **Three day-in-the-life scenarios** (schema-guard-before-write, handoff-mid-edit, contested-claim) written as agent transcripts against stage products
- **Chunking micro-experiment** on design/examples or ASF maps
- **Annotation convention sandbox** without CORE change

Any of these can ignore the others. Residue that changes OPEN should cite this NOTES section.

---

*End of free-form residue. 2026-07-21.*

