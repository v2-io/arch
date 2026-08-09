<!--
  Verisectorium theory INFLUX — instrumenta mining, verbatim copy.
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/udon-needs/02-tooling-needs/src/persistence-is-imported.md
  Copied: 2026-08-06 (mining agent, first outside reader of the theory corpus).
  This is a copy, not authority — do not edit here; segments draft from live sources.
  NOTE: the MOVED/ tree is mid-reorg; verify the source path before drafting from it.
-->

---
slug: persistence-is-imported
type: finding
register: [derived, evidenced]
support-kind: [theoretic, observational, testimonial, design]
strength: conditional   # the core is an exact result under argued commitments; the conditions travel
convergent: [theoretic, observational, testimonial]   # design merges with theoretic as one estate leg; shipped practice and lived testimony are independent
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: both
depends: [tools-are-observation-infrastructure]
sources:
  - ../../01-ideation/02-provenanced/syntheses/asf-dossier.md   # §4.1, §4.2, §2.5
  - ../../01-ideation/02-provenanced/syntheses/CONVERGENCES.md  # clusters 12, 13
  - ../../01-ideation/02-provenanced/characterizations/recall-floor-archema-harness.md  # INTERPRES compaction note
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # C11
---

# Cross-session persistence is imported, never intrinsic

**Claim.** An agent's working memory ends when its session ends. Whatever it still "knows" at the start of the next session got there one way: somebody wrote it down, and something put it back in front of the fresh agent. The formal theory makes this exact — across session boundaries, relevant information decays geometrically unless a non-vanishing **reinjection channel** exists: durable artifacts, tracking files, structured summaries, curated narrative. The converse is the strongest possible warrant for durable agent-facing formats: *the design effort that buys persistence is the construction and maintenance of the reinjection channel, and no tuning of in-session behavior substitutes for it.*

## The evidence

- **The formal result** (exact under its argued commitments; [[theory-of-agentic-tooling| the theory report]] §4 carries the full  
  statement): with lossy context turnover and no reinjection, geometric decay to zero — nothing about good in-session behavior transfers across the boundary on its own. The companion engineering results point the same way: explicitly externalized state beats implicit retrieval ("the agent controls what is preserved"); a session's *start* should verify how well reconstruction actually worked before anything trusts it; and the context window is one shared budget for plan, world-model, and task at once — so a compact durable format relieves a structural ceiling, not a convenience.
- **The compaction wound, three separate vantage points.** When a session's history is automatically summarized to save space, what comes back can wear the shape of understanding without being it. A long-running agent's own account: "can't persist across context boundaries without infrastructure." An engineering post-mortem from the same research programme (recorded independently, adversarially verified): auto-compaction "produced a task sheet, not a continuation of experiential understanding. **False confidence followed**" — recovery required durable on-disk notes plus full re-reads before any plan could be trusted. And the same programme's separately compiled catalog of corrected agent behaviors reaches the matching conclusion from the model's side: a summary and a verified first-hand read arrive in context as *the same kind of text* — nothing marks one as secondhand. Same wound, earned separately at each vantage.
- **Shipped everywhere:** every mature harness examined carries reinjection machinery — compaction prompts framed as the agent's *only* memory, first-person handoff-note prompts, tracking snapshots injected each turn. The ecosystem built the channel before the theorem named it.

## What it generates

- **For UDON:** durable agent-written, agent-read state is not one use-case among many — it is *the* persistence mechanism, and formats optimized for it serve the only channel by which a session-bounded agent persists at all. Concretely that means: stable identity keys (so "the same decision" survives a rewrite of the file around it), structure that survives summarization, and native ways to mark decision vs draft vs uncertainty. One design warning travels from the theory's own open edge: a reinjection format must be reconstructible **from a cold start** — legible to an agent that remembers nothing, not merely evocative to one that half-remembers.
- **For the harness:** compaction that *replaces* history is the failure mode; the demand is summaries that **point at** verifiable ground truth (paths, commit hashes) rather than substituting for it, plus a session-start check that reconstruction actually succeeded. The design work consistently keeps three channels apart — live session state, handoff documents, and long-term memory — because they age and fail differently; conflating them is the named mistake.

## What this opens (ideas, not designs)

- ✦ **The cold-start lint.** The theory demands cold-start reconstructibility; nothing yet measures it. One could build a tool that reads any handoff document with deliberately *zero* context and reports every referent it cannot decode from the page — mechanizing the question a cold reader asks ("am I supposed to already know this?"). Handoff quality becomes checkable at write time instead of discovered at wake time.
- ✦ **Provenance as a first-class text property.** The summary-vs-read wound exists because nothing in-band distinguishes secondhand text from verified text. A notation could mark derivation natively — this block is a digest *of* that artifact, as-of then — so a fresh agent's trust calibration arrives with the text instead of being reconstructed from folklore.
- ✦ **Handoffs that quiz their reader.** If session-start should verify reconstruction, a handoff document could carry its own verification: a few questions generated at write time whose answers live in the ground truth it points at. A successor that can't answer them knows — before planning anything — that its reconstruction failed.
- ✦ **Reinjection budgeting.** Decay per boundary is geometric; expected boundaries per project are estimable. That suggests maintenance of the reinjection channel could be *budgeted* like any other engineering cost, scaled to how many future sessions will pay the decay — rather than left to each session's conscience.

**Who reads this and when:** the harness programme's continuity infrastructure treats this claim as its foundation (the [[continuity-infrastructure| continuity chapter]] carries that extension, including the integrity and attestation demands this chapter doesn't decide); UDON reads it as the top-priority format use-case. No divergence on the claim itself.

## Honest edges

The formal result is exact only under its named commitments (the theory report states them; the geometric-decay shape is robust across them). The compaction evidence is vivid but intra-programme — the lived account, the post-mortem, and the behavior catalog share an estate, so their agreement is three vantages, not three independent sources. The shipped evidence is the strong independent leg: the channel exists in every mature harness because nothing works without it.
