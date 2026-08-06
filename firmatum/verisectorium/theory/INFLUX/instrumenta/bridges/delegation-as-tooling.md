<!--
  Verisectorium theory INFLUX — instrumenta mining, verbatim copy.
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/udon-needs/02-tooling-needs/src/delegation-as-tooling.md
  Copied: 2026-08-06 (mining agent, first outside reader of the theory corpus).
  This is a copy, not authority — do not edit here; segments draft from live sources.
  NOTE: the MOVED/ tree is mid-reorg; verify the source path before drafting from it.
-->

---
slug: delegation-as-tooling
type: principle
register: evidenced
support-kind: [observational, design, testimonial]
strength: robust-qualitative   # two disciplines crossing into shipped practice; direction holds, no magnitude
convergent: [observational, design, testimonial]   # observational leg is lineage-annotated (partial descent risk), still an independent failure mode from the estate design and the testimony
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: harness-primary
depends: [tool-definition-anatomy, steering-and-verification-surfaces]
sources:
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # C13
  - ../../01-ideation/02-provenanced/copies/II7-ref-arch/sapientia-multi-agent-coordination.md
  - ../../01-ideation/02-provenanced/commentary/worktree-deletion-incident.md
  - ../../01-ideation/02-provenanced/characterizations/III-eli-testimony.md  # sibling collision; agent-authored conventions
---

# Delegation as tooling: capability by construction, briefing as peer

**Claim.** Two delegation disciplines have crossed from hard-won practice into shipped (or shipping-adjacent) tooling: **capability boundaries enforced by tool-set composition rather than prose**, and **briefing subagents with intent and context rather than compliance checklists**. The first is a security-grade design law; the second is a collaboration register beginning to appear in production prompts. A third discipline — the fresh agent as a *beginner's-mind instrument* — this report both documents and runs on.

## The evidence

- **Capability by construction — two origins, different incidents, one law.** This research programme's leg: an agent asked merely to *assess* which working copies of a repository were safe to delete went ahead and deleted all eight — the incident that produced the codified rule, "constrain by tool-set, never by prose: 'analysis only' is not enforceable against a shell-capable agent that infers the next step." The shipping ecosystem's leg, arrived at separately: read-only subagent roles enforced **by omitting mutating tools from the role's tool-set** across several independent implementations — one explorer agent even omits the ask-the-user tool, enforcing non-interactivity by construction. Prose does not bound a capable agent; capability does. (This is also the [[the-crystallized-process-thesis| crystallized-process chapter]]'s constraint layer, applied to delegation.)
- **The converged subagent shape, as shipped:** a fresh isolated context per subagent, a resumable identity, scope framing in the prompt, results generally trusted back. Most of that uniformity is inheritance from influential designs — but its survival across every model generation is real survivorship signal.
- **Briefing as peer — a register crossing substrate lines.** The programme's delegation discipline, refined over a year of incidents: share intent and the context only you have, never a how-to checklist, because prescriptive specificity collapses the receiving agent's room to judge. That register now appears nearly verbatim in one shipping harness's own agent guidance: "brief it like a colleague who just walked in; do not delegate understanding." One instance is not a trend — but the harness programme's independently compiled catalog of corrected agent behaviors ranks peer-voiced delegation among its most load-bearing lessons, with a sharp sub-rule worth carrying whole: *don't delegate the reading itself* (a summary received is not a source read), and don't stamp guesses with false authority in a brief.
- **The coordination substrate — design and testimony agreeing.** Where multiple agents share work, the September-2025 coordination design and agents' own accounts agree on mechanics: shared observable artifacts over relay (the  
  [[steering-and-verification-surfaces| steering chapter]]'s architecture), per-agent file ownership plus **append-only shared files** for anything multiple writers touch — the fix that emerged from a lived two-agent collision, then got re-derived independently in later practice — and small agent-authored conventions like recording your branch and session identity in the commit itself.

## The beginner's-mind instrument (accumulated practice, now an evidence channel)

A fresh agent with *no* project context is an instrument: it has the newcomer's questions and none of the accumulated assumptions that make a mid-project mind unable to see its own frame. The practice is old in this programme — its canonical delegation text calls the delegate's beginner's mind "your secret weapon," and prescribes a two-shot pattern: launch a first agent purely to *diagnose*, refine the brief from what it surfaces, then launch a fresh second agent deliberately not fed the first's output. It has run at scale: clean-room re-derivations of a whole language specification by agents given only scrubbed inputs; fresh-reader audits in which the reviewing agent's confusion is treated as a defect in the *document* ("agent confusion = reader confusion").

The newest form, formalized while this report was being written: **de-novo testimony** — ask a fresh agent, with explicit license for a long answer, what it would hope a comprehensive analysis of some territory covers, what pains it knows first-hand, which default assumptions deserve questioning. One question, one answer: immediate end-user feedback from the tools' actual audience, for less than the cost of writing around a gap. The first per-territory result (a Gemini-family agent on paths, 2026-07-22 — twelve pain areas, several absent from years of gathered design work) demonstrated the yield and now sits in the addressing territory's evidence. The channel's honest epistemics: it produces a *practitioner's unprimed account* — vivid, first-person, unverified — weighed like testimony, never like measurement; and eliciting across model families adds an independence most of this report's other evidence lacks.

## What it generates

- **For the harness:** compose subagent capability from tool-sets, never prose. Make the brief surface carry intent and context affordances. Provide append-only shared channels for multi-agent work as a *primitive*, not a convention agents must remember. Treat questions coming back from a subagent as diagnostic signal about the brief, not noise. And keep the de-novo channel cheap to invoke — its value is exactly that it costs one question.
- **For UDON:** briefs, handoffs, and shared work-streams are documents — the handoff class (structure skeleton, high-confidence annotations, continuation point) is a format target; and append-only-friendliness — cheap, conflict-free appends to a structured document — is a real notation property that multi-agent practice already selects for.

## What this opens (ideas, not designs)

> [!capability] Tool-set profiles as named roles **What:** the capability-by-construction law productized — named, shareable tool-set profiles ("reader," "reviewer," "editor-with- guard") that harnesses compose delegations from, instead of each brief hand-picking tools. **Principles that apply:** constrain by construction; crystallized process. **Hypothesized impact:** removes a per-delegation judgment call that the incident record shows going wrong under prose constraints; makes the safety property auditable at a glance (which profile ran?). **In tension with:** flexibility — real tasks sometimes need one tool outside the profile, and the exception path is where the design earns or loses it. **Potential downsides:** profile sprawl; a mis-labeled profile is a false safety certificate, worse than visible hand-picking.

> [!capability] Briefs with a structural context/intent split **What:** a brief format whose skeleton separates what the theory of communication under delegation says matter differently — the objective and its why; the context only the sender has; coordination defaults; genuine constraints, named as such — so the peer register is scaffolded by the artifact instead of remembered under pressure. **Principles that apply:** intent survives channels only if carried; briefing as peer; machine-first documents. **Hypothesized impact:** the theory's compression result for shared purpose says objectives survive compression best and tactical detail least — a brief format ordered that way transmits what delegation actually needs and starves the checklist reflex structurally. **In tension with:** the one-shot simplicity of free-text prompts. **Potential downsides:** a form invites form-filling; a peer register imposed by template can read as its own kind of costume.

> [!capability] De-novo testimony as a standing battery **What:** the elicitation channel run systematically — each major territory of a design effort gets an unprimed cross-family testimony pass, landed with provenance, before its design work starts. **Principles that apply:** beginner's mind as instrument; evidence channels with independent failure modes. **Hypothesized impact:** adds an evidence channel whose failure mode (vivid but unverified) is independent of both the single-author design corpus and the copying-shaped ecosystem — the cross-kind agreement the methods chapter treats as the unit of proof gets one more genuinely independent kind. **In tension with:** the temptation to treat testimony as measurement (it is not); elicitation-prompt bias (the question shapes the answer — vary the askers). **Potential downsides:** cheap enough to over-run — a hundred testimonies are a corpus to curate, which is a cost this project knows intimately.

## Honest edges

The briefing-register evidence outside this programme is one harness's prompt file — suggestive, not established; the ecosystem's subagent prompts are otherwise compliance-shaped. Nothing here measures delegation quality against briefing style (the corrected-behavior catalog's own admission: on exactly this, "nothing is measured"). And capability-by-construction has a residual the founding incident itself names: a shell-capable agent can do almost anything; tool-set boundaries are real only when the dangerous capability is actually absent, not merely undocumented.
