---
slug: streaming-and-partial-documents
type: demand
register: [evidenced, decided]
support-kind: [design, observational, theoretic]
strength: robust-qualitative   # partial-is-normal holds across kinds; product shapes deliberately open
convergent: [design, observational]   # theoretic merges with design as one estate leg
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: both (udon-primary on format; harness on transport)
depends: [tools-are-observation-infrastructure]
sources:
  - ../../01-ideation/02-provenanced/commentary/spikes/agent-utility-NOTES.md  # §1, §5, P-A/P-B
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # C10
  - ../../01-ideation/02-provenanced/syntheses/asf-dossier.md  # §2.5 obs-context-turnover; §4.2
  - ../../../DECISIONS.md  # rows: R2, C6, W0, W1d
---

# Partial documents are the normal case

**Claim.** Agents emit bytes in long runs without visual feedback, stop mid-document, resume after tool calls, and routinely produce almost-valid structure. So for an agent-facing format, honesty under *incompleteness* is a first-class property, not an error path. The demand has three parts: feedback available mid-generation (the agent equivalent of syntax highlighting); truncation surfaced as a **verdict** — a clean, machine-branchable "this input ended incomplete" — rather than a malformed tree or a silent loss; and products shaped to their stage (a stream of recognition events for streaming consumers, an assembled tree for structural operations) rather than one mandatory parse result for every purpose.

## The evidence

- **The transport layer pays a standing tax, everywhere.** Streaming tool-call *reassembly* is a real, nontrivial, repeatedly solved problem: arguments arrive fragmented and out of order; harnesses buffer, reassemble, and tolerate malformed partial JSON; one auto-repairs unclosed strings; vendor documentation warns outright that consumers "may get partial/invalid JSON and must guard the parse." Five separate teams built five separate versions of this machinery — one of the few convergences in the shipping ecosystem that survives the copying-vs-invention accounting as genuinely independent, because each was forced into it by the same external reality. All of it is the price of formats whose partial prefixes are illegible. A format whose *every prefix* parses to an honest partial state removes the tax at the source. UDON's recognition posture is already exactly this — bounded lookahead, keep everything, unfinished constructs closed with a warning that cites where they opened — and the evidence here says that property is load-bearing, not incidental.
- **What agents would do with an honest partial state** (explored in depth in [[agent-utility-exploration| the agent-utility exploration]]): ask "where am  
  I?" mid-parse — the open-element stack, the attribute currently being written; surface anomalies early, before five hundred more tokens compound a mistake; validate a prefix against a schema or an enum while there is still time to change course; generate with the grammar enforcing validity (the [[structured-output-two-mechanisms| structured-output chapter]]'s strong mechanism); and  
  hand an interrupted generation to a successor as a partial tree plus verdict, instead of as a puzzle.
- **UDON has already decided the substrate pieces.** Three standing decisions in the [[DECISIONS.md|design ledger]] carry directly: incompleteness is a *verdict on the document*, not an event in the stream — a consumer reads it as a result, and no amount of clever stream-processing recovers it if the format doesn't say it; each stage's product must suffice for the next without reaching back to earlier bytes; and a value's extent must be explicit on the wire, never inferable-only. Stated here as the decisions they are; this chapter's evidence is *why they were worth deciding*.
- **The theory's frame:** what arrives and what gets processed are different events, and a session that begins from a poorly reconstructed state produces unreliable judgments about everything downstream. Honest partial states are what make mid-stream diagnostics mean anything at all.

## The named tension (design input, not resolved)

Generation wants **soft recovery** mid-stream — keep everything, warn, continue, because half a document is worth more than none. Careful writes want **hard, mutation-free refusal** — the [[schema-guarded-mutation| guarded-mutation chapter]]'s territory, where half an edit is worse than none. Same language, opposite postures, selected by stage and stakes. The tooling mistake would be letting either posture colonize the other.

## What this opens (ideas, not designs)

- ✦ **A validity heartbeat.** Nothing today tells a generating agent "your last 400 tokens still parse; one construct open; no anomalies." A sidecar that renders the partial-parse verdict at intervals during generation would be the syntax-highlighting equivalent the demand names — cheap with a format whose prefixes parse, impossible without.
- ✦ **Prefix-honesty as a declared class.** Streaming surfaces could declare what their payload format guarantees mid-stream: parses-at-every-prefix, or guard-the-parse. Consumers would pick buffering strategy mechanically instead of by folklore. (The [[counter-register| counter-register]]'s transport dissent stands unchanged: this classifies *payloads*; framing, sequencing, and cancellation remain the transport's job.)
- ✦ **Interruption as a document.** If a partial tree plus its verdict is honest, an interrupted generation becomes a *handoff artifact* — one model stops mid-emission, another (or the same one, a session later) resumes from the partial state rather than regenerating. Nothing shipped treats mid-generation state as transferable; the format property above is what would make it thinkable.
- ✦ **Pricing the tax.** "Tokens wasted after the first uncaught anomaly" is measurable per format: generate until an induced error, count how long the error survives undetected under each format's feedback affordances. The reassembly machinery's cost has never been priced; this would price its complement.

**Who reads this and when:** UDON reads it as the case for making recognition-layer products *public agent surfaces* with the verdict channel; the harness reads the reassembly evidence as transport reality and the verdict shape as what its tool results should expose for long-running generation — it benefits even for non-UDON payloads.

## Honest edges

The multi-line question in UDON's spec deliberately stays open — and this chapter's scenarios are exactly the demand evidence that should eventually force it: concrete pain in stream-and-repair cases, not deliberation in the abstract. Grammar-constrained generation from UDON's own grammar is technique-known but has never been wired into a harness; until someone runs it, it is a direction, not a result.
