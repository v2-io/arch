<!--
  Verisectorium theory INFLUX — instrumenta mining, verbatim copy.
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/udon-needs/02-tooling-needs/src/tracking-snapshots-as-perception.md
  Copied: 2026-08-06 (mining agent, first outside reader of the theory corpus).
  This is a copy, not authority — do not edit here; segments draft from live sources.
  NOTE: the MOVED/ tree is mid-reorg; verify the source path before drafting from it.
-->

---
slug: tracking-snapshots-as-perception
type: finding
register: evidenced
support-kind: [design, observational, theoretic]
strength: robust-qualitative   # designed perception; the theory later derived why it is structural
convergent: [design, observational]   # 'built 2025 / designed 2025 / theorized' are THREE FACETS OF ONE ESTATE LEG (same author), not three independent arrivals; the shipped-practice leg is the only independent failure mode
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: harness-primary
depends: [persistence-is-imported, context-economy]
sources:
  - ../../01-ideation/02-provenanced/copies/II2-zoetica-ennaos/tracking-snapshot-spec.md  # head read (schema + fields)
  - ../../01-ideation/02-provenanced/syntheses/CONVERGENCES.md  # cluster 12
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # C11 (state-snapshot family)
  - ../../01-ideation/02-provenanced/syntheses/asf-dossier.md  # §4.2 duality; temporal markers
---

# Structured context-injection is perception design

**Claim.** The recurring "tracking snapshot" — a structured frame injected into an agent's context carrying elapsed time, environment state, queue visibility, and budget — is not user-interface convenience leaking into a prompt. It is **designed perception**: the deliberate construction of the observation channels an embodied creature gets for free and a language-borne agent otherwise simply lacks. It was built in 2025, specified the same year, and the formal theory later derived why it is structural.

## The evidence

- **Built, then specified (2025).** A minimal agent runtime injected a structured tracking snapshot every turn; its design successor specifies the schema, and states the mission in one sentence: the programme's persistent agents "must experience passage of time, environmental context, and pending user input **as part of their reality, not hidden UI state**." The schema reads as a perception inventory: time-passage with elapsed duration rendered both as a number and as logarithmic glyphs (elapsed time as something *perceived*, not computed); repository status (environmental grounding); working directory (spatial awareness); context usage (budget awareness); pending messages (arrivals visible before they are attended); and an audit binding — session, turn, commit — so compressed old snapshots stay expandable from history. Old frames compressed, the latest kept whole: a fast channel and a slow archive of it, in miniature.
- **The ecosystem's parallel, grown for a different reason.** The compaction family's state snapshots ("this snapshot is the agent's *only* memory") and the system-reminder catalogs shipped across harnesses carry much of the same content — elapsed context, repo state, queues — built independently, for token budgets rather than time perception. The same mechanism arriving three ways — built, designed, derived — is what gives this claim its weight.
- **Why the theory calls it structural.** Three results: out-of-band temporal markers are a *prerequisite* for an agent computing its own pace — a suspension gap is invisible in the turn sequence and violent in reality; aligning an agent's subjective event-rate with the world's requires deliberately provisioned, multi-channel, asynchronous input — a coordination requirement, not a luxury; and quick-glance frames versus durable stores are two different design objects (the snapshot is the high-frequency ephemeral leg — sent up, not persisted — with compression as its slow projection). One caveat travels with the enthusiasm: correlated channels drawing on one source overcount — a snapshot earns its tokens through *independent* signals (time, repository, queue), not by restating the conversation.

## What it generates

- **For the harness:** treat the snapshot schema as a perception budget to design deliberately — which independent channels, at what frequencies, ephemeral or persisted per field, with an audit pointer so ephemeral frames stay reconstructible. The 2025 specification is a working draft of exactly this, ready to build from.
- **For UDON:** snapshots are a document class with unusual physics — emitted constantly, read once, compressed aggressively, never hand-authored — the purest ephemeral-instrument case in the ephemeral/durable split. (And the time-glyphs are a standing reminder that perceptual density sometimes beats structural purity — a design sensibility worth keeping near any pure-structure instinct.)

## What this opens (ideas, not designs)

> [!capability] A declared perception budget **What:** the snapshot's channel inventory made explicit configuration — which signals, at which cadences, with per-channel independence stated — rather than an accreted prompt block. **Principles that apply:** perception is designed, not inherited; independent channels are what add. **Hypothesized impact:** adaptive tempo is a sum over channels weighted by their independence — the theory's additivity result says correlated channels saturate while independent ones genuinely add — so an explicit inventory lets a harness *audit* its perception for redundancy instead of discovering it as wasted window. **In tension with:** the context budget (every channel is tokens, every turn); simplicity of one hardcoded frame. **Potential downsides:** configurable perception invites per-project drift into incomparable agent experiences; some redundancy is honest robustness, and an aggressive de-duplication pass could cut it.

> [!capability] Perceptual encodings beyond time **What:** the logarithmic time-glyph move generalized — glyph scales for context-budget pressure, staleness of key beliefs, queue depth: quantities an agent should *feel* at a glance rather than parse and compute. **Principles that apply:** perceptual density; observation design over volume. **Hypothesized impact:** cuts the per-turn attention cost of always-on quantities (a glyph is read without arithmetic), in the same way the glyphs made elapsed time a perceptum — plausibly raising effective sampling frequency on signals agents currently check only when reminded. **In tension with:** machine-legibility (glyphs are for the reading mind; pair them with the numeric field, never replace it). **Potential downsides:** invented pictographs are a learned vocabulary — unfamiliar encodings could cost more than they save until they are as trained-in as progress bars.

> [!capability] The pacing experiment **What:** the untested loop closed — run matched sessions with and without time-passage visibility and measure behavior deltas: deliberation length, re-planning frequency, staleness-checking. **Principles that apply:** claim-or-kill; perception claims deserve perception measurements. **Hypothesized impact:** none claimed — this card exists because the whole chapter rests on a derivation plus a conviction, and one afternoon of A/B sessions would tell both consumers whether designed time-perception actually changes agent pacing. **In tension with:** nothing; it is cheap. **Potential downsides:** a null result would be quietly embarrassing to a beloved design — which is precisely why it should run.

## Honest edges

The built and designed legs share an author; the independent shipped leg converged on *content* (repository status, elapsed context) but not on the temporal-coherence *rationale* — the ecosystem built it for token budgets, so the perceptual framing rests on the design work and the theory. No measurements exist of snapshot-driven behavior change (the third card above is the honest response).

## Working Notes

**"Built 2025 / designed 2025 / theorized" is one estate leg, not three.** All three facets — the running implementation, the written specification, and the later formal derivation — come from the same author, so they corroborate each other the way a person's notebook corroborates their memoir. The convergence that *does* count here is with shipped practice elsewhere (context-injection frames appearing in harnesses built independently), which is the second and only independent failure mode.

This is a tightening, not a doubt about the content: designed-perception is one of the more compelling ideas in this report, and the theory derivation genuinely adds explanatory depth. It just is not third-party confirmation. If someone can point at an independently-invented tracking-snapshot surface (or agent testimony about missing one), that is the leg that would make this properly multi-kind.
